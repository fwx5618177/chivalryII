mod config;
mod events;
mod logging;
mod plugins;
mod resources;

use clap::builder::EnumValueParser;
use clap::{Parser, ValueEnum};
use config::{ConfigManager, ConfigType};
use plugins::GamePluginManager;
use std::fmt;

#[derive(Clone, Debug, ValueEnum)]
enum Mode {
    Debug,
    Dev,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Debug => write!(f, "debug"),
            Mode::Dev => write!(f, "dev"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_parser = EnumValueParser::<Mode>::new(), default_value_t = Mode::Dev)]
    mode: Mode,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config_type = match args.mode {
        Mode::Debug => ConfigType::Debug,
        Mode::Dev => ConfigType::Dev,
    };
    let config_manager = ConfigManager::new(config_type)?;
    let settings = config_manager.get_settings();

    GamePluginManager::run(settings);

    Ok(())
}
