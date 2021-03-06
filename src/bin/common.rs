use std::path::PathBuf;
use std::env;

use clap::App;
use config::{self, Config};

pub fn configure(app: App) -> Config {
    // Set defaults
    let mut config = Config::default();
    config.set_default::<&str>("cfg_file", concat!(env!("CARGO_PKG_NAME"), ".toml")).unwrap();
    config.set_default::<&str>("log_file", concat!(env!("CARGO_PKG_NAME"), ".log")).unwrap();
    config.set_default::<&str>("Web.Listen.address", "127.0.0.1").unwrap();
    config.set_default("Web.Listen.port", 7878).unwrap();

    // Parse arguments
    let args = app.get_matches();

    // Determine config file
    // TODO: Is there a better way to handle this?
    if let Ok(c) = env::var("AARDWOLF_CONFIG") {
        config.set("cfg_file", c).expect("failed to set cfg_file from the environment variable AARDWOLF_CONFIG");
    }

    if let Some(c) = args.value_of("config") {
        config.set("cfg_file", c).expect("failed to set cfg_file from the --config argument");
    }

    // Merge config file and apply over-rides
    let cfg_file: PathBuf = PathBuf::from(config.get_str("cfg_file").unwrap());
    config.merge(config::File::with_name(cfg_file.to_str().unwrap())).unwrap();

    //  TODO: Is there a better way to handle this?
    if let Ok(l) = env::var("AARDWOLF_LOG") {
        config.set("log_file", l).expect("failed to set log_file from the environment variable AARDWOLF_LOG");
    }

    if let Some(l) = args.value_of("log") {
        config.set("log_file", l).expect("failed to set log_file from the --log argument");
    }

    config
}

pub fn db_conn_str(config: &Config) -> String {
    let scheme = match config.get_str("Database.type").unwrap().to_lowercase().as_str() {
        "postgresql" | "postgres" => "postgres",

        // If we reach this case, it's an error.
        // TODO: Handle it better.
        _ => panic!("Unsupported scheme, only `postgres' and `postgresql` are supported"),
    };

    format!("{scheme}://{username}:{password}@{host}:{port}/{database}",
            scheme=scheme,
            username=config.get_str("Database.username").expect("Missing key Database.username").as_str(),
            password=config.get_str("Database.password").expect("Missing key Database.password").as_str(),
            host=config.get_str("Database.host").expect("Missing key Database.host").as_str(),
            port=config.get_str("Database.port").expect("Missing key Database.port").as_str(),
            database=config.get_str("Database.database").expect("Missing key Database.database").as_str())
}
