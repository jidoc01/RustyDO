use crate::login::*;

const MAX_ROOM_MEMBERS: usize = 8;

/*
#[derive(Bundle)]
pub struct RoomBundle {
    pub name: RoomName,
    pub room_id: RoomId,
    pub map_id: RoomMapId,
    pub manager: RoomManager,
    pub members: RoomMembers,
    pub pw: RoomPassword,
}
*/

#[derive(Component)]
pub struct RoomName(String);

#[derive(Component)]
pub struct RoomId(usize);

#[derive(Component)]
pub struct RoomMapId(u8);

#[derive(Component)]
pub struct RoomManager(EntityId);

#[derive(Component)]
pub struct RoomMembers([EntityId; MAX_ROOM_MEMBERS]);

#[derive(Component)]
pub struct RoomPassword(Option<String>);

/*
#[derive(Bundle)]
pub struct ClientOnRoomBundle {
    pub color: ClientOnRoomTeamColor,
    pub char: ClientOnRoomCharacter,
    pub state: ClientOnRoomState
}
*/

#[derive(Component)]
pub struct ClientOnRoomTeamColor(u8);

#[derive(Component)]
pub struct ClientOnRoomCharacter(u8);

#[derive(Component)]
pub enum ClientOnRoomState {
    Idle,
    Ready,
    Shopping,
}