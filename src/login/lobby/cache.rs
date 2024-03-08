use crate::login::*;

/// This includes users who are in the lobby or room.
#[derive(Default)]
pub struct LobbyCache {
    users: HashSet<EntityId>,
    user_list_packet: Vec<u8>,
}

impl LobbyCache {
    pub fn add_user(&mut self, user: EntityId) {
        self.users.insert(user);
    }

    pub fn remove_user(&mut self, user: EntityId) {
        self.users.remove(&user);
    }

    pub fn get_users(&self) -> Vec<&EntityId> {
        self.users.iter().collect()
    }
}
