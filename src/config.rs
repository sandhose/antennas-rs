use std::net::SocketAddr;
use structopt::StructOpt;
use url::Url;
use uuid::Uuid;

#[derive(StructOpt, Debug, Clone)]
pub struct Config {
    #[structopt(short, long, default_value = "127.0.0.1:8080")]
    listen: SocketAddr,

    #[structopt(short, long)]
    public_url: Option<Url>,

    #[structopt(short, long, default_value)]
    uuid: Uuid,

    #[structopt(short, long, default_value = "http://127.0.0.1:9981")]
    tvheadend_url: Url,
}

impl Config {
    pub fn public_url(&self) -> Url {
        match &self.public_url {
            Some(u) => u.clone(),
            None => {
                let mut u = Url::parse("http://localhost").unwrap();
                u.set_port(Some(self.listen.port())).unwrap();
                u.set_ip_host(self.listen.ip()).unwrap();
                u
            }
        }
    }

    pub fn listen(&self) -> SocketAddr {
        self.listen
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn tvheadend_url(&self) -> &Url {
        &self.tvheadend_url
    }
}
