mod db;
mod keystore;
mod server;

use clap::{Parser, Subcommand};

use db::DbManager;
use ethers::core::types::Address;
use keystore::KeyStore;
use log::info;
use std::env;
use tokio::task::JoinSet;

#[derive(Parser, Clone)]
#[clap(version)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Serve {
        #[clap(short, long, env)]
        wss_uri: String,

        #[clap(
            short,
            long,
            env,
            default_value = "https://nominatim.openstreetmap.org"
        )]
        nominatim_uri: String,

        #[clap(
            short,
            long,
            env,
            default_value = "https://overpass-api.de/api/interpreter"
        )]
        overpass_uri: String,
    },
    Auth {
        #[command(subcommand)]
        cmd: AuthCommands,
    },
    Account {
        #[command(subcommand)]
        cmd: AccountCommands,
    },
}

#[derive(Subcommand, Clone)]
enum AuthCommands {
    List,
    Add {
        #[clap(value_name = "CONTRACT")]
        contract: String,
    },
    Rm {
        #[clap(value_name = "CONTRACT")]
        contract: String,
    },
}

#[derive(Subcommand, Clone)]
enum AccountCommands {
    New,
    List,
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    let args = Args::parse();
    match args.cmd {
        Commands::Serve {
            wss_uri,
            nominatim_uri,
            overpass_uri,
        } => {
            let d = DbManager::new().await.expect("Database connection error");
            let auth = d
                .authorized_contracts()
                .await
                .expect("Authorized contracts error");
            if auth.is_empty() {
                std::process::exit(1);
            }

            let password = rpassword::prompt_password("Key Store Password: ").unwrap();
            let ks = KeyStore::open(&password).expect("Key store error");

            let mut set = JoinSet::new();
            for a in auth {
                set.spawn(server::serve(
                    wss_uri.to_owned(),
                    nominatim_uri.to_owned(),
                    overpass_uri.to_owned(),
                    a.to_owned(),
                    ks.wallet.to_owned(),
                ));
                info!("Spawned task for authorized contract: {}", hex::encode(a));
            }

            while let Some(res) = set.join_next().await {
                res.expect("Task join error").unwrap();
            }
        }
        Commands::Auth { cmd } => match cmd {
            AuthCommands::Add { contract } => {
                let a = contract
                    .parse::<Address>()
                    .expect("Invalid contract address");
                let d = DbManager::new().await.expect("Database connection error");
                d.authorize_contract(a)
                    .await
                    .expect("Contract authorization error");
            }
            AuthCommands::Rm { contract } => {
                let a = contract
                    .parse::<Address>()
                    .expect("Invalid contract address");
                let d = DbManager::new().await.expect("Database connection error");
                d.remove_contract(a).await.expect("Contract removal error");
            }
            AuthCommands::List => {
                let d = DbManager::new().await.expect("Database connection error");
                let auth = d
                    .authorized_contracts()
                    .await
                    .expect("Authorized contracts error");
                for a in auth {
                    println!("{}", hex::encode(a));
                }
            }
        },
        Commands::Account { cmd } => match cmd {
            AccountCommands::New => {
                let password = rpassword::prompt_password("Key Store Password: ").unwrap();
                KeyStore::create(&password).expect("Key store create error");
            }
            AccountCommands::List => {
                let password = rpassword::prompt_password("Key Store Password: ").unwrap();
                KeyStore::open(&password).expect("Key store open error");
            }
        },
    }
}
