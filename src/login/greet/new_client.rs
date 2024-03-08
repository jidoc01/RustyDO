use crate::login::*;

pub fn greet_new_client(
    receiver: Receiver<Insert<ClientAddr>, &ClientSessionJobSender>,
    mut sender: Sender<Insert<ClientDisconnecting>>,
    uid_fetcher: Fetcher<&ClientUid>,
) {
    todo!();
    /*
    let used_uids = {
        let mut s = HashSet::new();
        uid_fetcher
            .iter()
            .for_each(|uid| {
                s.insert((*uid).clone());
            });
        s
    };
    receiver
        .iter()
        .for_each(|sender| {
            // try to allocate a client uid (u16).
            let maybe_uid: Option<ClientUid> = (1..=u16::MAX)
                .find_map(|i| {
                    if used_uids.contains_key(&ClientUid(i)) == false {
                        Some(ClientUid(i))
                    }
                    else {
                        None
                    }
                });
            let Some(uid) = maybe_uid else {
                // cannot allocate a new uid.
                // disconnect this client.
                sender.send(ClientSessionJob::Disconnect);
                event_writer.send(DisconnectEvent { entity });
                return;
            };
            commands
                .entity(entity)
                .insert(uid);
            // TODO: send an enc data using enc_data
            let pkt = SetEncData {
            };
            sender.send_packet(pkt);
        });
    */
}