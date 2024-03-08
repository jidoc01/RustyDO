use std::collections::HashSet;

use crate::{login::*, packet::outgoing::{LoginMessage, LoginMessageKind}, storage::{account::Account, Storage}};

use evenio::{component::RemoveComponent, rayon::iter::{IntoParallelIterator, IntoParallelRefIterator}};
use pwhash::bcrypt;

use super::LoginEvent;

pub fn handle_login_event(
    r: Receiver<LoginEvent, (&ClientSessionJobSender, Option<&ClientId>)>,
    online_ids: Fetcher<&ClientId>,
    Single(storage): Single<&Storage>,
    mut after_login_adder: Sender<(Insert<ClientId>, Insert<ClientAccount>, Insert<ClientOnBulletinBoard>)>,
) {
    if r.query.1.is_some() {
        return;
    }
    let mut online_id_set = get_online_id_set(&online_ids);
    let LoginEvent { entity, id, pw } = r.event;
    let sender = r.query.0;
    let Some(account) = storage.find_one::<Account>(doc!{ "id": id }) else {
        let pkt = LoginMessage(LoginMessageKind::NoId);
        sender.send_packet(pkt);
        return;
    };
    if account.pw != encrypt_password(pw) {
        let pkt = LoginMessage(LoginMessageKind::InvalidInfo);
        sender.send_packet(pkt);
        return;
    }
    if online_id_set.contains(id) {
        let pkt = LoginMessage(LoginMessageKind::AlreadyOnline);
        sender.send_packet(pkt);
        return;
    }
    online_id_set.insert((*id).clone());
    after_login_adder.insert(*entity, ClientId(id.clone()));
    after_login_adder.insert(*entity, ClientAccount(account.clone()));
    after_login_adder.insert(*entity, ClientOnBulletinBoard);
}

fn get_online_id_set(online_ids: &Fetcher<&ClientId>) -> HashSet<String> {
    let mut set = HashSet::new();
    online_ids
        .iter()
        .for_each(|x| { set.insert(x.0.clone()); });
    set
}

fn encrypt_password(s: &str) -> String {
    match bcrypt::hash(s) {
        Ok(s) => s,
        Err(_) => "".into(),
    }
}