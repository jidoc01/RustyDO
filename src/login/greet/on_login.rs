
use crate::*;
use crate::login::component::*;

use self::{packet::outgoing::{OutPacketBuildable, SetAccountInfo}, storage::account::Account};

pub fn system_greet_client_on_login(
    q: Receiver<Insert<ClientId>, (&ClientAccount, &ClientSessionJobSender)>
) {
    let (ClientAccount(account), sender) = q.query;
    let pkt = build_account_info_packet(&account);
    sender.send_packet(pkt);
}

fn build_account_info_packet(account: &Account) -> impl OutPacketBuildable {
    let id = &account.id;
    let name = &account.name;
    SetAccountInfo
}