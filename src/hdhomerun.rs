use serde::Serialize;
use url::{ParseError, Url};
use uuid::Uuid;

use super::config::Config;
use super::tvheadend::Channel;

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Lineup {
    guide_number: String,
    guide_name: String,

    #[serde(rename = "URL")]
    url: Url,
}

impl Lineup {
    pub fn from_channel(channel: Channel, base_url: Url) -> Result<Lineup, ParseError> {
        Ok(Lineup {
            guide_number: channel.guide_number(),
            guide_name: channel.guide_name(),
            url: channel.url(base_url)?,
        })
    }
}

#[derive(Serialize)]
enum SourceType {
    Cable,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineupStatus {
    scan_in_progress: u16,
    scan_possible: u16,
    source: SourceType,
    source_list: Vec<SourceType>,
}

impl Default for LineupStatus {
    fn default() -> Self {
        LineupStatus {
            scan_in_progress: 0,
            scan_possible: 1,
            source: SourceType::Cable,
            source_list: vec![SourceType::Cable],
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Discover {
    friendly_name: String,

    manufacturer: String,

    #[serde(rename = "ManufacturerURL")]
    manufacturer_url: Url,

    model_number: String,

    firmware_name: String,

    tuner_count: u16,

    firmware_version: String,

    #[serde(rename = "DeviceID")]
    device_id: Uuid,

    device_auth: String,

    #[serde(rename = "BaseURL")]
    base_url: Url,

    #[serde(rename = "LineupURL")]
    lineup_url: Url,
}

impl From<&Config> for Discover {
    fn from(cfg: &Config) -> Discover {
        let base_url = cfg.public_url();
        let lineup_url = base_url.join("/lineup.json").unwrap();
        Discover {
            friendly_name: String::from("HDHomerun (antennas-rs)"),
            manufacturer: String::from("Silicondust"),
            manufacturer_url: Url::parse("https://github.com/sandhose/antennas-rs").unwrap(),
            model_number: String::from("HDTC-2US"),
            firmware_name: String::from("hdhomeruntc_atsc"),
            tuner_count: 6,
            firmware_version: String::from("20170930"),
            device_id: cfg.uuid().clone(),
            device_auth: String::from("5678"),
            base_url,
            lineup_url,
        }
    }
}
