use serde::{Deserialize, Serialize};
use std::{error::Error, fs};
use wlral::input::keyboard::KeyboardConfig;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
  pub keyboard_layouts: Vec<KeyboardConfig>,
}

impl Config {
  pub fn load() -> Result<Config, Box<dyn Error>> {
    let config_string =
      fs::read_to_string(shellexpand::tilde("~/.config/cascade/config.yaml").to_string())?;
    let config: Config = serde_yaml::from_str(&config_string)?;

    for (i, a) in config.keyboard_layouts.iter().enumerate() {
      for (j, b) in config.keyboard_layouts.iter().enumerate() {
        if a == b && i != j {
          return Err(
            format!(
              "Duplicated keyboard layout in index {} and {}: {:?}",
              i, j, a
            )
            .into(),
          );
        }
      }
    }

    Ok(config)
  }
}
