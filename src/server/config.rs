use std::fs::File;
use std::io::{BufReader, Read as _, Write as _};
use std::path::PathBuf;
use std::sync::RwLock;

use clap::Parser;
use fuzion_commons::config::{DatabaseConfig, HttpConfigWithPublic, LoggingConfig};
use toml;

const DEFAULT_CONFIG: &'static str = r##"encoding = "utf-8"

[http]
host = "0.0.0.0"
port = 10666

[http.public]
host = "fuziondev"
port = 8182
secure = true

[logging]
log_to_stdout = true
log_level = "info"

[database]
host = "127.0.0.1"
port = 8432
user = "fuzion"
password = "CHANGEME"
name = "fuzion"
"##;

lazy_static! {
  pub static ref FUZION_VERITA_CONFIG: RwLock<Option<FuzionVeritaConfig>> = RwLock::new(None);
}

#[derive(Clone, Debug, Deserialize)]
pub struct FuzionVeritaConfig {
  pub raw: Option<toml::Value>,

  pub encoding: String,
  #[serde(default)]
  pub interactive: bool,
  #[serde(default)]
  pub migrate: bool,

  pub logging: LoggingConfig,
  pub database: DatabaseConfig,
  pub http: HttpConfigWithPublic,
}

impl Default for FuzionVeritaConfig {
  fn default() -> Self {
    let config = FuzionVeritaConfig {
      raw: None,

      encoding: "utf8".to_owned(),
      interactive: false,
      migrate: false,

      logging: Default::default(),
      database: Default::default(),
      http: Default::default(),
    };

    config
  }
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct FuzionVeritaArgs {
  /// Config file used to start server.
  #[arg(short, long, value_name = "FILE", default_value = "fuzion-verita.toml")]
  config: PathBuf,
  /// If mismatched version, perform migration.
  #[arg(long)]
  migrate: Option<bool>,
  /// If the program is run as a daemon or non-interactive process.
  #[arg(long)]
  non_interactive: Option<bool>,
  /// The verbosity of logging.
  #[arg(long, short, value_parser = clap_arg_to_log_level)]
  log_level: Option<slog::Level>,
}

pub fn clap_arg_to_log_level(level: &str) -> Result<slog::Level, String> {
  match level {
    "critical" => Ok(slog::Level::Critical),
    "debug" => Ok(slog::Level::Debug),
    "error" => Ok(slog::Level::Error),
    "trace" => Ok(slog::Level::Trace),
    "warning" => Ok(slog::Level::Warning),
    "info" => Ok(slog::Level::Info),
    _ => Err(String::from("Failed to parse log level.")),
  }
}

impl FuzionVeritaConfig {
  pub fn set(server_config: FuzionVeritaConfig) {
    let mut lock = FUZION_VERITA_CONFIG
      .write()
      .expect("Could not set ServerConfig");
    *lock = Some(server_config);
  }

  pub fn get() -> FuzionVeritaConfig {
    let lock = FUZION_VERITA_CONFIG
      .read()
      .expect("Could not get ServerConfig");
    (*lock).as_ref().unwrap().to_owned()
  }

  pub fn load() -> FuzionVeritaConfig {
    let args = FuzionVeritaArgs::parse();

    let mut config = FuzionVeritaConfig::load_from_file(&args.config);

    // Apply command line arguments.
    config.migrate = args.migrate.unwrap_or(false);
    config.interactive = !args.non_interactive.unwrap_or(false);
    config.logging.log_level = args.log_level.unwrap_or(config.logging.log_level);

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
