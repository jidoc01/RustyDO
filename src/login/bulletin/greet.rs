use crate::login::*;

/*

pub fn system_greet_client_on_bulletin(
    q: Query<&ClientSessionJobSender, Added<ClientOnBulletinBoard>>,
    board: Res<ResBulletinBoard>
) {
    if q.is_empty() { return }
    let bulletin_info_pkt = board.bulletin_pkt();
    let ticker_pkt = board.ticker_pkt();
    q
        .par_iter()
        .for_each(move |sender| {
            sender.send_shared_packet(bulletin_info_pkt.clone());
            sender.send_shared_packet(ticker_pkt.clone());
        });
}

*/