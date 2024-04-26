use aeloc::server;
use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser, Clone)]
#[clap(version)]
struct Args {
    #[clap(short, long, env)]
    wss_uri: String,

    #[clap(short, long, env)]
    nominatim_uri: String,

    #[clap(short, long, env)]
    overpass_uri: String,

    #[clap(short, long, env)]
    key: String,

    #[clap(short, long, env)]
    dispatcher: String,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Serve,
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    let args = Args::parse();
    match args.cmd {
        Commands::Serve => {
            server::serve(args.wss_uri, args.dispatcher, args.key).await;
        }
    }
}
