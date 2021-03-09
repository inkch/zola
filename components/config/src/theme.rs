use std::collections::HashMap;
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};
use toml::Value as Toml;

use errors::{bail, Result};
use utils::fs::read_file;

/// Holds the data from a `theme.toml` file.
/// There are other fields than `extra` in it but Zola
/// itself doesn't care about them.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    /// All user params set in [extra] in the theme.toml
    pub extra: HashMap<String, Toml>,
}

impl Theme {
    /// Parses a TOML string to our Theme struct
    pub fn parse(content: &str) -> Result<Theme> {
        let theme = match content.parse::<Toml>() {
            Ok(t) => t,
            Err(e) => bail!(e),
        };

        let mut extra = HashMap::new();
        if let Some(theme_table) = theme.as_table() {
            if let Some(ex) = theme_table.get("extra") {
                if ex.is_table() {
                    extra = ex.clone().try_into().unwrap();
                }
            }
        } else {
            bail!("Expected the `theme.toml` to be a TOML table")
        }

        Ok(Theme { extra })
    }

    /// Parses a theme file from the given path
    pub fn from_file(path: &PathBuf, theme_name: &str) -> Result<Theme> {
        let content = read_file(path)
            .map_err(|e| errors::Error::chain(format!("Failed to load theme {}", theme_name), e))?;
        Theme::parse(&content)
    }
}
