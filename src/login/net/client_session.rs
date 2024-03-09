use crate::{login::*, packet::{build_packet, packet_receiver::PacketReceiver}};

pub async fn run_client_session_async(
    mut stream: TcpStream,
    client_tx: UnboundedSender<ClientJob>,
    mut client_session_rx: UnboundedReceiver<ClientSessionJob>
) {
    let mut recv_buf = [0u8; 1024];
    let mut packet_receiver = PacketReceiver::default();

    loop {
        select! {
            res = stream.read(&mut recv_buf) => {
                match res {
                    Ok(n) if n > 0 => on_receive(&recv_buf[0..n], &mut packet_receiver, &client_tx),
                    _ => { on_disconn(&client_tx); break; }
                }
            },
            msg = client_session_rx.recv() => {
                let Some(msg) = msg else {
                    // TODO
                    continue;
                };
                if !on_msg_async(&mut stream, msg).await {
                    break;
                }
            }
        }
    }
    stream.shutdown().await.ignore();
    client_session_rx.close();
}

fn on_disconn(client_tx: &UnboundedSender<ClientJob>) {
    debug!("Client session disconnected");
    client_tx.send(ClientJob::OnDisconnected).ignore();
}

fn on_receive(received: &[u8], packet_receiver: &mut PacketReceiver, sender: &UnboundedSender<ClientJob>) {
    debug!("Client session received {} bytes", received.len());
    packet_receiver.push(received);

    // TODO: no need to allocate a new buffer for the body.
    // Instead we can slice the packet buffer w/ proper lifetime.
    let Ok(Some(body)) = packet_receiver.try_fetch_body() else {
        // TODO: block the IP
        debug!("we should block this IP");
        return;
    };
    debug!("body: {:?}", body);
    let pkt = InPacket::parse(body);
    sender.send(ClientJob::OnReceive(pkt)).ignore();
}

async fn on_msg_async(stream: &mut TcpStream, msg: ClientSessionJob) -> bool {
    match msg {
        ClientSessionJob::SendPacket(pkt) => {
            let Ok(data) = build_packet(pkt.as_ref()) else {
                // TODO: why?
                return true;
            };
            stream.write(&data).await.ignore();
            true
        },
        ClientSessionJob::Disconnect => {
            false
        }
    }
}