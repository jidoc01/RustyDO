use super::*;

pub fn create(world: &mut World, addr: SocketAddr) -> &mut Entity {
    let id = world.push();
    let entity = world.get_mut(&id).unwrap();
    entity.push(EntityKind::Client);
    entity.push(addr);
    entity.push(ClientState::BeforeLogin);
    entity
}

pub fn send(entity: &Entity, pkt: Vec<u8>) {
    let _ = entity
        .get::<MsgToConnSender>()
        .unwrap()
        .send(MsgToConn::SendPacket(pkt));
}

pub fn send_by_eid(world: &World, eid: &EntityId, pkt: Vec<u8>) {
    let entity = world
        .get(eid)
        .unwrap();
    send(entity, pkt);
}