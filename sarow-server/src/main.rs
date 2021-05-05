mod config;
use crate::config::Config;
mod auth;
use crate::auth::BasicAuth;
use bytes::buf::BufMut;
use futures::TryStreamExt;
use humantime::format_duration;
use humantime_serde::Serde;
use log;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{SocketAddr, SocketAddrV4};
use std::ops::{Add, Deref};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::ErrorKind;
use tokio::time::Duration;
use warp::host::Authority;
use warp::http::header::HeaderValue;
use warp::http::HeaderMap;
use warp::path::{FullPath, Tail};
use warp::{
    multipart::{FormData, Part},
    Filter, Rejection, Reply,
};

#[tokio::main]
async fn main() {
    let config: Config = config::Config::new().unwrap();

    env_logger::Builder::new()
        .parse_filters(&config.log_level)
        .init();

    log::debug!("{:?}", config);

    let upload_route = warp::path("upload")
        .and(warp::filters::header::headers_cloned())
        .and(warp::path::full())
        .and(with_work_dir(config.work_dir.clone()))
        .and(warp::host::optional())
        .and(warp::query::<HashMap<String, Serde<Duration>>>())
        .and(warp::post())
        .and(warp::multipart::form().max_length(config.max_size.get_bytes()))
        .and_then(upload);

    let download_route = warp::path("files")
        .and(warp::filters::header::headers_cloned())
        .and_then(download);
    //.and(warp::fs::dir(format!("{}/files", config.work_dir)));

    let router = upload_route.or(download_route);

    let addr: SocketAddrV4 = SocketAddrV4::new(config.ip, config.port);

    tokio::fs::create_dir(format!("{}/files", config.work_dir))
        .await
        .map_err(|error| match error.kind() {
            ErrorKind::AlreadyExists => log::info!("{}", error.to_string()),
            _ => panic!("{}", error.to_string()),
        })
        .unwrap_or_default();

    tokio::spawn(warp::serve(router).run(addr));
    tokio::spawn(clean_task(config.clean_interval, config.work_dir))
        .await
        .unwrap_or_default();
}

async fn clean_task(interval: Duration, work_dir: String) {
    log::info!("Started cleaning task");
    let mut interval = tokio::time::interval(interval);
    loop {
        interval.tick().await;
        let now: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let mut files = tokio::fs::read_dir(format!("{}/files", work_dir))
            .await
            .unwrap();

        while let Some(file) = files.next_entry().await.unwrap() {
            let expiration_string: String = file.file_name().into_string().unwrap();
            let expiration_timestamp: u128 = expiration_string.parse::<u128>().unwrap();
            if expiration_timestamp < now {
                tokio::fs::remove_dir_all(file.path()).await.unwrap();
                log::debug!("Removed file: {:?}", file.file_name());
            }
        }
    }
}

fn with_work_dir(work_dir: String) -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::any().map(move || work_dir.clone())
}

async fn download(addr: HeaderMap) -> Result<impl Reply, Rejection> {
    log::debug!("{:?}", addr);
    Ok("XD")
}

async fn upload(
    addr: HeaderMap,
    path: FullPath,
    work_dir: String,
    host: Option<Authority>,
    query: HashMap<String, Serde<Duration>>,
    form: FormData,
) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        log::error!("form error: {}", e);
        warp::reject::reject()
    })?;

    let expires_in: Duration = query.get("expires_in").unwrap().into_inner();

    let mut response_path = String::new();

    for p in parts {
        if p.name() == "file" {
            let filename = p.filename().unwrap();

            let expiration_timestamp = format!(
                "{}",
                SystemTime::now()
                    .add(expires_in)
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            );

            let path = format!("{}/files/{}", work_dir, expiration_timestamp);
            let full_path = format!("{}/{}", path, filename);
            response_path = format!("/files/{}/{}", expiration_timestamp, filename);

            tokio::fs::create_dir(path).await.map_err(|e| {
                log::error!("{}", e.to_string());
                warp::reject::reject()
            })?;

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    log::error!("reading file error: {}", e);
                    warp::reject::reject()
                })?;

            tokio::fs::write(&full_path, value).await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })?;

            log::info!("created file {}", full_path);
            log::info!("File will expire in {}", format_duration(expires_in))
        }
    }

    let host_xd = host.unwrap();

    log::debug!("{:?}", addr);

    let request = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .post(format!(
            "https://localhost:8443/?msg={}{}",
            host_xd, response_path
        ))
        .send()
        .await;

    log::debug!("{:?}", path.as_str().to_string());

    Ok(format!("http://{}{}", host_xd, response_path))
}
