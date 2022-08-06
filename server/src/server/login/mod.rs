mod handler;
mod component;
mod conn;
mod entity;
mod timer;

use crate::{prelude::*, server::login::{entity::EntityKind, conn::MsgToConnSender, handler::packet::ErrorKind}};
//use legion::serialize::WorldDeserializer;
use serde_json::Value;
//use legion::{storage::IntoComponentSource, world::EntryRef};
use tokio::{net::{TcpListener, TcpStream}, sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel}};
use std::{net::SocketAddr, collections::{HashSet}, time::{Duration, Instant}, thread, rc::Rc};
use conn::Conn;
use component::*;

use self::conn::MsgToConn;

const LOGIN_PORT: u16 = 9874;
const FPS: u64 = 20;
const MILLI_PER_FRAME: u64 = 1000 / FPS;

#[derive(Debug)]
pub enum MsgToServer {
    /// A new connection.
    NewConn(TcpStream, SocketAddr),
    /// Received a packet.
    OnPacketReceived(EntityId, PacketReader),
    /// Disconnection.
    OnDisconnected(EntityId),
    /// Inter-server message.
    /// JSON Value / Request port
    InterRequest(Value, UnboundedSender<Value>)
}

pub type MsgToServerSender = UnboundedSender<MsgToServer>;

pub struct Server {
    pub world: World,
    pub db: Connection,
    config: Config,
    msg_rx: UnboundedReceiver<MsgToServer>,
    msg_tx: UnboundedSender<MsgToServer>,
}

impl Server {
    pub fn new(config: Config, db: Connection) -> Self {
        let (tx, rx) = unbounded_channel();
        let mut server = Self {
            world: World::default(),
            config: config,
            db: db,
            msg_rx: rx,
            msg_tx: tx,
        };
        server.add_timers();
        server
    }

    /// Add timers to handle periodic events such as checking disconnections and
    /// game-related stuffs.
    fn add_timers(&mut self) {
        timer::register_timers(&mut self.world);
    }

    async fn recv_loop(conn_tx: UnboundedSender<MsgToServer>) -> anyhow::Result<()> {
        let addr = format!("0.0.0.0:{LOGIN_PORT}");
        let listener = TcpListener::bind(addr).await?;
        println!("Login server listening on {LOGIN_PORT}");
        loop {
            let (stream, sock_addr) = listener.accept().await?;
            let new_conn = MsgToServer::NewConn(stream, sock_addr);
            conn_tx.send(new_conn)?;
        }
    }

    async fn start_recv(&self) {
        tokio::spawn(Self::recv_loop(self.msg_tx.clone()));
    }

    fn register_connection(&mut self, stream: TcpStream, addr: SocketAddr) {
        let world = &mut self.world;
        let entity = entity::session::create(world, addr);
        let entity_id = entity.id();
        let conn = Conn::new(entity_id, stream, self.msg_tx.clone());
        entity.push(conn.get_conn_tx());
        conn.start();

        log!("[{}] A new connection.", entity_id);
    }

    fn handle_packet(&mut self, eid: EntityId, pr: PacketReader) {
        log!("[{}] Packet received.", eid);
        if self.is_entity_offline(&eid) { // It is already disconnected.
            // Ignore the message.
        }
        else if let Err(err) = handler::handle(self, eid, pr) {
            // Disconnect?
            // TODO
            println!("An error should be resolved: {err}");
        } else { // Ok.
            if let Ok(entity) = self.world.get_mut(&eid) {
                entity.push(PongInfo(true));
            }
        }
    }

    /// We handle disconnection here in the end.
    /// (Disconnection) -> (Connection) -> (Server: here)
    fn handle_disconnection(&mut self, entity_id: EntityId) {
        // 1. Check if it is in a room or in-game.
        let state =
            self
                .world
                .get(&entity_id)
                .unwrap()
                .get::<ClientState>()
                .unwrap()
                .to_owned();
        if state == ClientState::OnRoom || state == ClientState::OnGame {
            handler::on_room::handle_exit(self, &entity_id);
        }
        
        // 2. Release every resource attached to the entity.
        self.world.remove(&entity_id);

        log!("[{}] Disconnection.", entity_id);
    }

    fn is_entity_offline(&self, eid: &EntityId) -> bool {
        self
            .world
            .get(eid)
            .is_err()
    }

    #[inline]
    fn handle_message(&mut self) {
        while let Ok(msg) = self.msg_rx.try_recv() {
            match msg {
                // A new connection.
                MsgToServer::NewConn(stream, addr) => {
                    self.register_connection(stream, addr);
                },
                // Packet received.
                MsgToServer::OnPacketReceived(eid, pr) => {
                    self.handle_packet(eid, pr);
                },
                // Disconnected.
                MsgToServer::OnDisconnected(entity) => {
                    self.handle_disconnection(entity);
                },
                // This message can be sent from the other server such as a web
                // server.
                MsgToServer::InterRequest(msg, tx) => {
                    self.handle_inter_request(msg, tx);
                }
            }
        }
    }

    #[inline]
    fn handle_timer(&mut self) {
        let timer_list = {
            let mut timer_list = vec!();
            self
                .world
                .select(|entity| {
                    let kind = entity
                        .get::<entity::EntityKind>()
                        .unwrap();
                    *kind == entity::EntityKind::Timer
                })
                .iter()
                .for_each(|timer| {
                    let timer_eid = timer.id();
                    timer_list.push(timer_eid);
                });
            timer_list
        };

        for timer_eid in timer_list.iter() {
            entity::timer::tick(self, timer_eid);
        }
    }

    #[inline]
    fn sleep(&self, tick: &Duration, time: Instant) {
        let duration = time.elapsed();
        if duration >= *tick {
            log!("There is too much traffic right now.");
        } else {
            thread::sleep(tick.saturating_sub(duration));
        }
    }

    /// The main loop.
    /// It never terminates.
    /// TODO: Make priority between callbacks.
    fn handle(&mut self) -> Option<()> {
        let tick = Duration::from_millis(MILLI_PER_FRAME);
        loop {
            let time = std::time::Instant::now();
            self.handle_message();
            self.handle_timer();
            self.sleep(&tick, time);
        }
    }

    pub async fn run(&mut self) {
        self.start_recv().await;
        self.handle();
    }

    pub fn get_server_tx(&self) -> MsgToServerSender {
        self.msg_tx.clone()
    }
    
    fn add_new_account(&self, name: String, id: String, pw: String) -> String {
        let db = &self.db;
        let user_tbl = db.table(USER_TBL).unwrap();
        let config = &self.config;

        // Check the uniqueness of id & name.
        {
            let users: Vec<UserSchema> = user_tbl
                .iter()
                .filter(field("id").eq(&id))
                .data(&db)
                .unwrap();
            if users.is_empty() == false {
                return "id_dup".into();
            }
        }
        {
            let users: Vec<UserSchema> = user_tbl
                .iter()
                .filter(field("name").eq(&name))
                .data(&db)
                .unwrap();
            if users.is_empty() == false {
                return "name_dup".into();
            }
        }

        let user_schema = UserSchema {
            id: id.clone(),
            pw: hex::encode(hash::from_str(&pw)),
            name: name.clone(),
            level: config.user.initial_level,
            is_female: false,
            money: config.user.initial_money,
            items: vec![0; ITEM_COUNT],
            exps: vec![0; EXP_COUNT],
            is_admin: false,
            is_muted: false,
            is_banned: false,
            setting: SettingSchema {
                key_binding: 1,
                bgm_volume: 49,
                bgm_mute: false,
                bgm_echo: true,
                sound_volume: 49,
                sound_mute: false,
                macros: vec!["".into(); MACRO_COUNT]
            }
        };
        user_tbl
            .insert(user_schema, &db)
            .expect("Failed to insert an account into database");

        println!("Added an account: {id} ({name})");
        return "".into();
    }

    fn handle_inter_request(&mut self, msg: Value, tx: UnboundedSender<Value>) {
        //println!("{}", msg.to_string());
        let opcode = msg["opcode"].as_str().unwrap();
        let msg_id = msg["msg_id"].as_u64().unwrap();
        let response_msg = match opcode {
            "ping" => {
                let mut response_msg = Value::default();
                response_msg["msg_id"] = Value::from(msg_id);
                Some(response_msg)
            },
            "new_account" => {
                let id = msg["id"].as_str().unwrap().into();
                let name = msg["name"].as_str().unwrap().into();
                let pw = msg["pw"].as_str().unwrap().into();
                let response = self.add_new_account(name, id, pw);
                let mut response_msg = Value::default();
                response_msg["result"] = Value::from(response);
                response_msg["msg_id"] = Value::from(msg_id);
                Some(response_msg)
            },
            _ => {
                println!("Invalid opcode: {opcode}");
                None
            }
        };
        
        if let Some(msg) = response_msg {
            tx
                .send(msg)
                .expect("Failed to send a response message");
        }
    }
}