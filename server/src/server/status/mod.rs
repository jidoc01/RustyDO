// Copyright 2022 JungHyun Kim
// This file is part of RustyDO.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::sync::Arc;

use tokio::net::UdpSocket;
use tokio::sync::mpsc::unbounded_channel;

use crate::prelude::*;
use crate::server::login::MsgToServer;

use super::login::MsgToServerSender;

const STATUS_PORT: u16 = 9874;

async fn parse_packet(buf: Vec<u8>, n: usize) -> anyhow::Result<Vec<u8>> {
    if n < HEADER_SIZE + TAIL_SIZE {
        bail!("Packet too short");
    }
    let header = &buf[0 .. HEADER_SIZE];
    let header = crypt::decode(header)?;
    let body_size = read_u16(&header, 0) as usize;
    let to_read = body_size + TAIL_SIZE;
    if n < HEADER_SIZE + to_read {
        bail!("The size field is too long");
    }
    let body = &buf[HEADER_SIZE .. HEADER_SIZE + to_read ];
    let body = crypt::decrypt(body);
    Ok(body)
}

async fn send_to(sock: Arc<UdpSocket>, addr: SocketAddr, pkt: &[u8]) {
    let _ = sock.send_to(pkt, addr).await;
}

async fn handle_packet(sock: Arc<UdpSocket>, data: &[u8], addr: SocketAddr) {
    let mut pr = PacketReader::new(data);
    let opcode = pr.opcode();
    let opcode_to_reply = opcode + 1;
    match opcode {
        1 => { // TODO: Support multiple servers.
            let ping = pr.u8();
            let pong = ping;
            let curr = 0;//global::user::get_user_count().await;
            let max = MAX_USERS as u16;
            let avail = (MAX_USERS - curr) as u16;
            let mut pw = PacketWriter::new(opcode_to_reply);
            let pkt = pw
                .u8(pong)
                .u8(1)      // the number of servers
                .u16(401)   // (1) server uid
                .u16(avail) // (2) available
                .u16(max)   // (3) the max number of clients 
                .as_vec();
            send_to(sock, addr, &pkt).await;
        },
        46 => { // Right after being authenticated in the login server.
            let mut pw = PacketWriter::new(opcode_to_reply);
            let ping = pr.u8();
            let pong = ping;
            let is_send_permitted = true; 
            pw
                .u8(pong)
                .u8(if_else(is_send_permitted, 1, 0))
                .u16(0);             // TODO: count
            let pkt = pw.as_vec();
            send_to(sock, addr, &pkt).await;
        }
        _ => {
            println!("Invalid status packet: {opcode}");
        }
    }
}

const INTER_MSG_PREFIX: &str = "!INTER_MSG";
const DELIMITER: u8 = 0xff;

fn generate_inter_packet(pw: String, msg: String) -> Vec<u8> {
    let mut ret = vec!();

    ret.append(&mut INTER_MSG_PREFIX.as_bytes().to_vec());
    ret.push(DELIMITER);
    ret.append(&mut pw.as_bytes().to_vec());
    ret.push(DELIMITER);
    ret.append(&mut msg.as_bytes().to_vec());

    ret
}

// TODO: Do not implement handling of inter messages here.
// Instead, create a new UDP port in the login server.
fn try_get_inter_msg(pw: String, pkt: &Vec<u8>) -> Option<String> {
    /*
        [Inter-message structure]
        {{PREFIX}}
        {{PASSWORD}}
        {{CONTENTS}}

        Note that we use '\n' (0xff) as a delimiter.
    */
    let mut it = pkt.split(|&v| v == DELIMITER);
    let mut f = || -> Option<(String, String, String)> {
        let prefix = match String::from_utf8(it.next()?.to_vec()) {
            Ok(s) => s,
            _ => return None,
        };
        let password = match String::from_utf8(it.next()?.to_vec()) {
            Ok(s) => s,
            _ => return None,
        };
        let contents = match String::from_utf8(it.next()?.to_vec()) {
            Ok(s) => s,
            _ => return None,
        };
        Some((prefix, password, contents))
    };
    if let Some((prefix, password, contents)) = f() {
        if &prefix == INTER_MSG_PREFIX && password == pw {
            Some(contents)
        } else {
            None
        }
    } else {
        None
    }
}

async fn handle(config: Config, login_tx: MsgToServerSender) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{STATUS_PORT}");
    let server_sock = UdpSocket::bind(addr).await?;
    let server_sock = Arc::new(server_sock);
    let mut buf = [0u8; 4096];
    let password = &config.server.password;
    println!("Status server listening on {STATUS_PORT}");
    loop {
        let server_sock = server_sock.clone();
        match server_sock.recv_from(&mut buf).await {
            Ok((n, addr)) => {
                let vec = Vec::from(buf)[..n].to_vec();
                let password = password.clone();
                let login_tx = login_tx.clone();
                tokio::spawn(async move {
                    if let Some(msg) = try_get_inter_msg(password.clone(), &vec) {
                        let json: serde_json::Value = serde_json::from_str(&msg).unwrap();
                        let (msg_tx, mut msg_rx) = unbounded_channel();
                        login_tx
                            .send(MsgToServer::InterRequest(json, msg_tx))
                            .expect("InterRequest failed");
                        let response_msg = match msg_rx.recv().await {
                            Some(response) => response.to_string(),
                            None => "{}".into()
                        };
                        let response_pkt = generate_inter_packet(password, response_msg);
                        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9875); // hacky.
                        send_to(server_sock, addr, &response_pkt).await;
                    }
                    else {
                        match parse_packet(vec, n).await {
                            Ok(data) => {
                                handle_packet(server_sock, &data, addr).await;
                            },
                            _ => {
                                // TODO: Check if it's a DoS attack.
                            }
                        }
                    }
                });
            },
            Err(error) => {
                bail!(error);
            }
        }
    }
}

pub async fn run(config: Config, login_tx: MsgToServerSender) {
    tokio::spawn(handle(config, login_tx));
}