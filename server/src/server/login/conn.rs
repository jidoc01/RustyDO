
use crate::prelude::*;
use tokio::{sync::mpsc::{UnboundedSender, unbounded_channel, UnboundedReceiver}, net::{TcpStream, tcp::{OwnedReadHalf, OwnedWriteHalf}}, io::AsyncReadExt};
use super::MsgToServer;

pub enum MsgToConn {
    Shutdown,
    SendPacket(Vec<u8>),
}

pub type MsgToConnSender = UnboundedSender<MsgToConn>;

enum RecvState {
    WaitForHeader,
    WaitForBody,
}

pub struct Conn {
    entity_id: EntityId,
    writer: RefCell<OwnedWriteHalf>,
    reader: RefCell<OwnedReadHalf>,
    server_tx: UnboundedSender<MsgToServer>,
    conn_tx: RefCell<UnboundedSender<MsgToConn>>,
    conn_rx: RefCell<UnboundedReceiver<MsgToConn>>,
    is_running: bool,
}

impl Conn {
    pub fn new(entity: EntityId, stream: TcpStream, tx: UnboundedSender<MsgToServer>) -> Self {
        let (reader, writer) = stream.into_split();
        let (conn_tx, conn_rx) = unbounded_channel();
        Self {
            entity_id: entity,
            writer: RefCell::new(writer),
            reader: RefCell::new(reader),
            server_tx: tx,
            conn_tx: RefCell::new(conn_tx),
            conn_rx: RefCell::new(conn_rx),
            is_running: true,
        }
    }

    /// Explicitly drop itself.
    fn release(self) {
        drop(self);
    }

    /*
    async fn recv_packet(read: &OwnedReadHalf)
        -> anyhow::Result<Vec<u8>> {
        let header = Self::recv_exact(HEADER_SIZE, read.clone()).await?;
        let header = crypt::decode_header(&header);
        if header.is_none() { // Invalid packet. Should be noticed.
            bail!("Could not decode the header");
        }
        let header = header.unwrap();
        let body_size = read_u16(&header, 0) as usize;
        let to_read = body_size + TAIL_SIZE;
        let body = Self::recv_exact(to_read, cancel_rx.clone(), read.clone()).await?;
        let body = crypt::decrypt(&body);
        Ok(body)
    }
    */

    /// It's used to disconnect itself when abnormal behavior was detected internally.
    fn disconnect(&mut self) {
        let _ = self.conn_tx.get_mut().send(MsgToConn::Shutdown);
    }

    fn handle_message(&mut self, msg: &MsgToConn) {
        match msg {
            MsgToConn::Shutdown => {
                self.is_running = false;
            },
            MsgToConn::SendPacket(data) => {
                let disconnect = match self.writer.borrow_mut().try_write(&data) {
                    Ok(n) => {
                        if n != data.len() {
                            todo!("TODO");
                        }
                        false
                    },
                    Err(err) => {
                        println!("{}", err);
                        true
                    }
                };
                if disconnect {
                    self.disconnect();
                }
            }
        }
    }

    fn handle_packet_chunk(&mut self, state: &mut RecvState, buf: &mut Vec<u8>) -> Result<()> {
        match state {
            RecvState::WaitForHeader => {
                let decoded = crypt::decode(&buf)?;
                let body_size = read_u16(&decoded, 0) as usize;
                let total_size = body_size + TAIL_SIZE;
                *buf = vec![0u8; total_size];
                *state = RecvState::WaitForBody;
            },
            RecvState::WaitForBody => {
                let decrypted = crypt::decrypt(&buf);
                let pr = PacketReader::new(&decrypted);
                let entity = self.entity_id;
                self.server_tx.send(MsgToServer::OnPacketReceived(entity, pr))?;
                *buf = vec![0u8; HEADER_SIZE];
                *state = RecvState::WaitForHeader;
            },
        }
        Ok(())
    }

    async fn handle(mut self) -> Result<()> {
        let mut state = RecvState::WaitForHeader;
        let mut buf: Vec<u8> = vec![0u8; HEADER_SIZE]; // Its size varies depending on its state.
        while self.is_running {
            tokio::select! {
                // Receive messages.
                msg = self.conn_rx.get_mut().recv() => {
                    if let Some(msg) = msg {
                        self.handle_message(&msg);
                    } else {
                        bail!("Invalid message on Conn");
                    }
                },
                // Receive packets.
                n = self.reader.get_mut().read_exact(&mut buf) => {
                    if let Ok(_n) = n {
                        if let Err(err) = self.handle_packet_chunk(&mut state, &mut buf) {
                            println!("Abnormal behavior: {}", err);
                            self.disconnect();
                        }
                    } else { // It can occur when the remote shut down its connection.
                        self.disconnect();
                    }
                }
            }
        }
        // Let the server know that it has been disconnected.
        self.server_tx.send(MsgToServer::OnDisconnected(self.entity_id))?;
        self.release();
        Ok(())
    }

    pub fn start(self) {
        tokio::spawn(async move {
            if let Err(err) = self.handle().await {
                panic!("{}", err);
            }
        });
    }

    pub fn get_conn_tx(&self) -> UnboundedSender<MsgToConn> {
        self.conn_tx.borrow().clone()
    }
}