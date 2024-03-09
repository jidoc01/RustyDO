use crate::login::*;

pub async fn run_client_session_async(
    mut stream: TcpStream,
    client_tx: UnboundedSender<ClientJob>,
    mut client_session_rx: UnboundedReceiver<ClientSessionJob>
) {
    let mut recv_buf = [0u8; 1024];
    let mut acc = vec!();
    let mut enc_data = None;
    loop {
        select! {
            res = stream.read(&mut recv_buf) => {
                match res {
                    Ok(n) if n > 0 => on_receive(&recv_buf[0..n], &mut acc, &client_tx, &enc_data),
                    _ => { on_disconn(&client_tx); break; }
                }
            },
            msg = client_session_rx.recv() => {
                let Some(msg) = msg else {
                    // TODO
                    continue;
                };
                if !on_msg_async(&mut stream, &mut enc_data, msg).await {
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

fn on_receive(received: &[u8], acc: &mut Vec<u8>, sender: &UnboundedSender<ClientJob>, enc_data: &Option<ClientEncData>) {
    debug!("Client session received {} bytes", received.len());
    acc.extend_from_slice(received);
    match enc_data {
        None => return,
        Some(enc_data) => {
            let Some(required_len) = parse_length_field(&acc, &enc_data) else {
                return;
            };
            if acc.len() < required_len {
                return;
            }
            let body = parse_body(&acc);
            let decrypted_body = decrypt_body(body);
            let pkt = InPacket::parse(&decrypted_body);
            sender.send(ClientJob::OnReceive(pkt)).ignore();
            todo!();
        }
    }
}

fn parse_length_field(acc: &[u8], enc_data: &ClientEncData) -> Option<usize> {
    todo!()
}

fn parse_body(acc: &[u8]) -> &[u8] {
    todo!()
}

fn decrypt_body(body: &[u8]) -> Vec<u8> {
    todo!()
}

async fn on_msg_async(stream: &mut TcpStream, enc_data: &mut Option<ClientEncData>, msg: ClientSessionJob) -> bool {
    match msg {
        ClientSessionJob::SendPacket(pkt) if enc_data.is_none() => {
            todo!()
        },
        ClientSessionJob::SendPacket(pkt) => {
            let enc_data = enc_data.as_ref().unwrap();
            let Some(data) = build_packet(pkt, &enc_data) else {
                // TODO: why?
                return true;
            };
            stream.write(&data).await.ignore();
            true
        },
        ClientSessionJob::SetEncData(enc_data_) => {
            *enc_data = Some(enc_data_);
            true
        },
        ClientSessionJob::Disconnect => {
            false
        }
    }
}

// TODO: use zero-copy buffers.
fn build_packet(pkt: Arc<dyn OutPacketBuildable + Send + Sync>, enc_data: &ClientEncData) -> Option<Vec<u8>> {
    let mut body = Vec::new();
    let mut writer = Writer::from_vec_mut(&mut body);
    let Ok(_) = pkt.try_build(&mut writer) else {
        // TODO: we should inspect this error.
        return None;
    };

    let header = [0u8; 3];
    let body = enc_data.encrypt(&body);
    let tail = [0u8; 3];

    let mut ret = Vec::new();
    ret.extend_from_slice(&header);
    ret.extend_from_slice(&body);
    ret.extend_from_slice(&tail);
    Some(ret)
}