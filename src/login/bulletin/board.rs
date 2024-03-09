use crate::login::*;

/*
const BULLETIN_TEXT: &str = "Welcome to the server!";
const TICKER_TEXT: &str = "Welcome to the server!";

#[derive(Resource)]
pub struct ResBulletinBoard {
    bulletin_text: String,
    ticker_text: String,
    bulletin_pkt: Arc<dyn OutPacketBuildable + Send + Sync>,
    ticker_pkt: Arc<dyn OutPacketBuildable + Send + Sync>,
}

impl ResBulletinBoard {
    pub fn bulletin_pkt(&self) -> Arc<dyn OutPacketBuildable + Send + Sync> {
        self.bulletin_pkt.clone()
    }

    pub fn ticker_pkt(&self) -> Arc<dyn OutPacketBuildable + Send + Sync> {
        self.ticker_pkt.clone()
    }

    pub fn update_bulletin_text(&mut self, text: String) {
        self.bulletin_pkt = Arc::new(build_bulletin_info_packet(&text));
        self.bulletin_text = text;
    }

    pub fn update_ticker_text(&mut self, text: String) {
        self.ticker_pkt = Arc::new(build_ticker_message_packet(&text));
        self.ticker_text = text;
    }
}

impl Default for ResBulletinBoard {
    fn default() -> Self {
        Self {
            bulletin_text: BULLETIN_TEXT.to_string(),
            ticker_text: TICKER_TEXT.to_string(),
            bulletin_pkt: Arc::new(build_bulletin_info_packet(BULLETIN_TEXT)),
            ticker_pkt: Arc::new(build_ticker_message_packet(TICKER_TEXT)),
        }
    }
}

fn build_bulletin_info_packet(text: &str) -> impl OutPacketBuildable {
    //todo!();
    SetEncData {}
}

fn build_ticker_message_packet(text: &str) -> impl OutPacketBuildable {
    //todo!();
    SetEncData {}
}
*/