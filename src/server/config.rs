use std::fs::File;
use std::io::{BufReader, Read as _, Write as _};
use std::sync::RwLock;

use clap::{Arg, Command};
use fuzion_commons::config::{DatabaseConfig, HttpConfigWithPublic, LoggingConfig};
use fuzion_commons::serde::str_to_log_level;
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
  pub static ref FUZION_VERITAS_CONFIG: RwLock<Option<FuzionVeritasConfig>> = RwLock::new(None);
}

#[derive(Clone, Debug, Deserialize)]
pub struct FuzionVeritasConfig {
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

impl Default for FuzionVeritasConfig {
  fn default() -> Self {
    let config = FuzionVeritasConfig {
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

impl FuzionVeritasConfig {
  pub fn set(server_config: FuzionVeritasConfig) {
    let mut lock = FUZION_VERITAS_CONFIG
      .write()
      .expect("Could not set ServerConfig");
    *lock = Some(server_config);
  }

  pub fn get() -> FuzionVeritasConfig {
    let lock = FUZION_VERITAS_CONFIG
      .read()
      .expect("Could not get ServerConfig");
    (*lock).as_ref().unwrap().to_owned()
  }

  pub fn load() -> FuzionVeritasConfig {
    let matches = Command::new("fuzion-arbiter")
      .version(env!("CARGO_PKG_VERSION"))
      .author("enzi (enzi@braindead.io)")
      .about("Does awesome things.")
      .arg(
        Arg::new("migrate")
          .long("migrate")
          .help("If mismatched version, allow migration."),
      )
      .arg(
        Arg::new("non-interactive")
          .long("non-interactive")
          .help("If the program is run as a daemon or non-interactive process."),
      )
      .arg(
        Arg::new("log-level")
          .long("log-level")
          .help("The verbosity of logging."),
      )
      .arg(
        Arg::new("config")
          .long("config")
          .help("Config file used to start server."),
      )
      .get_matches();

    let config_file = matches
      .get_one::<String>("config")
      .map(|val| val.to_owned())
      .unwrap_or("fuzion-arbiter.toml".to_owned());

    let mut config = FuzionVeritasConfig::load_from_file(&config_file);

    // Apply command line arguments.
    config.migrate = matches.contains_id("migrate");
    config.interactive = !matches.contains_id("non-interactive");
    config.logging.log_level = matches
      .get_one::<String>("verbosity")
      .and_then(|value| str_to_log_level(Some(value as &str)).ok())
      .unwrap_or(config.logging.log_level);

    config
  }

  pub fn load_from_file(path: &str) -> FuzionVeritasConfig {
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

    FuzionVeritasConfig::load_from_toml(toml.expect("Could not parse toml."))
  }

  pub fn load_from_toml(toml: toml::Value) -> FuzionVeritasConfig {
    let raw = toml.clone();

    let mut config: FuzionVeritasConfig = toml.try_into().unwrap();
    config.raw = Some(raw);

    config
  }
}
