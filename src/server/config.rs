use std::fs::File;
use std::io::{BufReader, Read as _, Write as _};
use std::path::PathBuf;
use std::sync::RwLock;

use clap::{ArgAction, Parser};
use fuzion_commons::config::{
  clap_arg_to_log_level, DatabaseConfig, HttpConfigWithPublic, LoggingConfig,
};

const DEFAULT_CONFIG: &'static str = r##"encoding = "utf-8"

[http]
host = "0.0.0.0"
port = 10666

[http.public]
host = "127.0.0.1"
port = 10666

[logging]
log_to_stdout = true
log_level = "info"

[database]
host = "127.0.0.1"
port = 4432
user = "verita"
password = "CHANGEME"
name = "verita"
"##;

lazy_static! {
  pub static ref FUZION_VERITA_CONFIG: RwLock<Option<FuzionVeritaConfig>> = RwLock::new(None);
}

#[derive(Clone, Debug, Deserialize)]
pub struct FuzionVeritaConfig {
  #[serde(skip)]
  pub raw: Option<toml::Value>,

  pub encoding: String,
  #[serde(default)]
  pub non_interactive: bool,
  #[serde(default)]
  pub migrate: bool,

  pub admin: Option<String>,
  pub admin_password: Option<String>,

  pub logging: LoggingConfig,
  pub database: DatabaseConfig,
  pub http: HttpConfigWithPublic,
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct FuzionVeritaArgs {
  /// Config file used to start server.
  #[arg(short, long, value_name = "FILE", default_value = "fuzion-verita.toml")]
  config: PathBuf,
  /// If mismatched version, perform migration.
  #[arg(long, action = ArgAction::SetTrue)]
  migrate: Option<bool>,
  /// If the program is run as a daemon or non-interactive process.
  #[arg(short, long, action = ArgAction::SetTrue)]
  non_interactive: Option<bool>,
  /// The verbosity of logging.
  #[arg(long, short, value_parser = clap_arg_to_log_level)]
  log_level: Option<slog::Level>,
  /// Username for the Verita admin.
  #[arg(long, env)]
  admin: Option<String>,
  /// Password for the Verita admin.
  #[arg(long, env)]
  admin_password: Option<String>,
}

impl FuzionVeritaConfig {
  pub fn set(server_config: FuzionVeritaConfig) {
    let mut lock = FUZION_VERITA_CONFIG
      .write()
      .expect("Could not set FuzionVeritaConfig.");
    *lock = Some(server_config);
  }

  pub fn get() -> FuzionVeritaConfig {
    FUZION_VERITA_CONFIG
      .read()
      .expect("Could not get FuzionVeritaConfig.")
      .to_owned()
      .unwrap()
  }

  pub fn load() -> FuzionVeritaConfig {
    let args = FuzionVeritaArgs::parse();

    let mut config = FuzionVeritaConfig::load_from_file(&args.config);

    // Apply command line arguments.
    config.migrate = args.migrate.unwrap_or(config.migrate);
    config.non_interactive = !args.non_interactive.unwrap_or(config.non_interactive);
    config.logging.log_level = args.log_level.unwrap_or(config.logging.log_level);

    config.admin = args.admin.or(config.admin);
    config.admin_password = args.admin_password.or(config.admin_password);

    config
  }

  pub fn load_from_file(path: &PathBuf) -> FuzionVeritaConfig {
    let toml = match File::open(path) {
      Ok(file) => {
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();

        content.parse::<toml::Value>()
      }
      Err(_) => {
        let mut file = File::create(path).expect("Unable to create config file.");

        let content = DEFAULT_CONFIG.parse::<toml::Value>();
        if content.is_ok() {
          file
            .write_all(DEFAULT_CONFIG.as_bytes())
            .expect("Unable to write config file.");
        }

        content
      }
    };

    FuzionVeritaConfig::load_from_toml(toml.expect("Could not parse toml."))
  }

  pub fn load_from_toml(toml: toml::Value) -> FuzionVeritaConfig {
    let raw = toml.clone();

    let mut config: FuzionVeritaConfig = toml.try_into().unwrap();
    config.raw = Some(raw);

    config
  }
}
