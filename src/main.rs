use anyhow::Result;
use axum::{
    Extension,
    routing::{Router, get, post},
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use sqlx::{Connection, SqliteConnection};
use std::sync::Arc;
use std::{fs::File, io::Read};
use tower_http::services::ServeDir;

mod handler;
use handler::*;

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// config file
    #[arg(short, long, default_value = "./config.toml")]
    config: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u32,
    pub db_path: String,
    pub static_dir: String,
    pub ffmpeg_dump_name: String,
}

pub fn read_from_toml(f: &str) -> Result<Config> {
    let mut file = File::open(f)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let config: Config = toml::from_str(&s)?;
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = Arc::new(read_from_toml(&cli.config)?);
    let app = Router::new()
        .nest_service("/static", ServeDir::new(&cfg.static_dir))
        .route("/", get(async || "hello, drone msg data!".to_string()))
        .route("/start", post(start_drone::StartDrone::handle_post))
        .route("/stop", post(stop_drone::StopDrone::handle_post))
        .layer(Extension(Arc::clone(&cfg)));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cfg.port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// 根据启动指令启动dj-mqtt-msg服务，超时关闭，接收指令也可关闭
