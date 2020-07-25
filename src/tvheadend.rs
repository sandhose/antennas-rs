use serde::Deserialize;
use url::{ParseError, Url};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Channel {
    uuid: Uuid,
    enabled: bool,
    autoname: bool,
    name: String,
    number: u16,
    epgauto: bool,
    epggrab: Vec<String>,
    dvr_pre_time: u16,
    dvr_pst_time: u16,
    epg_running: i8,
    services: Vec<Uuid>,
    tags: Vec<Uuid>,
    bouquet: String,
}

impl Channel {
    pub fn guide_number(&self) -> String {
        format!("{:?}", self.number)
    }

    pub fn guide_name(&self) -> String {
        self.name.clone()
    }

    pub fn url(&self, base_url: Url) -> Result<Url, ParseError> {
        base_url.join(format!("/stream/channel/{}", self.uuid.to_simple()).as_str())
    }
}

#[derive(Deserialize, Debug)]
pub struct ChannelGridResponse {
    pub entries: Vec<Channel>,
}
