use structopt::StructOpt;
use warp::Filter;

#[macro_use]
extern crate log;

mod config;
mod hdhomerun;
mod tvheadend;

#[derive(Debug)]
enum AppError {
    Generic,
    RequestError,
    DecodeError,
    TransformError,
}

impl warp::reject::Reject for AppError {}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let cfg = config::Config::from_args();
    info!("starting with config {:?}", cfg);
    let listen = cfg.clone().listen();
    let with_cfg = warp::any().map(move || cfg.clone());

    let client = reqwest::Client::new();
    let with_client = warp::any().map(move || client.clone());

    let discover = warp::path("discover.json")
        .and(with_cfg.clone())
        .map(|cfg: config::Config| {
            let resp: hdhomerun::Discover = (&cfg).into();
            warp::reply::json(&resp)
        });

    let lineup_status = warp::path("lineup_status.json")
        .map(|| warp::reply::json(&hdhomerun::LineupStatus::default()));

    let lineup = warp::path("lineup.json")
        .and(with_cfg.clone())
        .and(with_client.clone())
        .and_then(|cfg: config::Config, client: reqwest::Client| async move {
            let url = cfg
                .tvheadend_url()
                .join("/api/channel/grid?start=0&limit=999999")
                .map_err(|_| warp::reject::custom(AppError::Generic))?;

            let resp = client
                .get(url)
                .send()
                .await
                .map_err(|_| warp::reject::custom(AppError::RequestError))?
                .json::<tvheadend::ChannelGridResponse>()
                .await
                .map_err(|_| warp::reject::custom(AppError::DecodeError))?;

            let lineup: Result<Vec<hdhomerun::Lineup>, _> = resp
                .entries
                .into_iter()
                .map(|channel| hdhomerun::Lineup::from_channel(channel, cfg.tvheadend_url().clone()))
                .collect();

            let lineup = lineup.map_err(|_| warp::reject::custom(AppError::TransformError))?;

            Ok::<_, warp::reject::Rejection>(warp::reply::json(&lineup))
        });

    let guide = warp::path("guide.xml")
        .and(with_cfg)
        .and(with_client)
        .and_then(|cfg: config::Config, client: reqwest::Client| async move {
            let url = cfg
                .tvheadend_url()
                .join("/xmltv/channels")
                .map_err(|_| warp::reject::custom(AppError::Generic))?;

            let guide = client
                .get(url)
                .send()
                .await
                .map_err(|_| warp::reject::custom(AppError::RequestError))?
                .bytes()
                .await
                .map_err(|_| warp::reject::custom(AppError::DecodeError))?;

            let resp = warp::http::Response::builder().header("Content-Type", "text/xml").body(guide);

            Ok::<_, warp::reject::Rejection>(resp)
        });

    let default = warp::any()
        .map(|| warp::reply::with_status(warp::reply(), warp::http::StatusCode::NOT_FOUND));

    let route = discover.or(lineup_status).or(lineup).or(guide).or(default);
    let route = route.with(warp::log("antennas_rs::web"));

    warp::serve(route).run(listen).await;
}
