use serde_derive::Deserialize;

use super::DisplayRawOption;
use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::util::display::DisplayOption;
use crate::util::sort;

use crate::CONFIG_FILE;

const fn default_true() -> bool {
    true
}
const fn default_scroll_offset() -> usize {
    6
}
const fn default_max_preview_size() -> u64 {
    2 * 1024 * 1024 // 2 MB
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoshutoRawConfig {
    #[serde(default = "default_scroll_offset")]
    scroll_offset: usize,
    #[serde(default = "default_true")]
    use_trash: bool,
    #[serde(default)]
    xdg_open: bool,
    #[serde(default = "default_max_preview_size")]
    max_preview_size: u64,
    #[serde(default, rename = "display")]
    display_options: DisplayRawOption,
}

impl Flattenable<JoshutoConfig> for JoshutoRawConfig {
    fn flatten(self) -> JoshutoConfig {
        JoshutoConfig {
            max_preview_size: self.max_preview_size,
            scroll_offset: self.scroll_offset,
            use_trash: self.use_trash,
            xdg_open: self.xdg_open,
            _display_options: self.display_options.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JoshutoConfig {
    pub max_preview_size: u64,
    pub scroll_offset: usize,
    pub use_trash: bool,
    pub xdg_open: bool,
    _display_options: DisplayOption,
}

impl JoshutoConfig {
    pub fn display_options_ref(&self) -> &DisplayOption {
        &self._display_options
    }
    pub fn display_options_mut(&mut self) -> &mut DisplayOption {
        &mut self._display_options
    }

    pub fn sort_options_ref(&self) -> &sort::SortOption {
        self.display_options_ref().sort_options_ref()
    }
    pub fn sort_options_mut(&mut self) -> &mut sort::SortOption {
        self.display_options_mut().sort_options_mut()
    }
}

impl ConfigStructure for JoshutoConfig {
    fn get_config() -> Self {
        parse_to_config_file::<JoshutoRawConfig, JoshutoConfig>(CONFIG_FILE)
            .unwrap_or_else(Self::default)
    }
}

impl std::default::Default for JoshutoConfig {
    fn default() -> Self {
        Self {
            max_preview_size: default_max_preview_size(),
            scroll_offset: default_scroll_offset(),
            use_trash: true,
            xdg_open: false,
            _display_options: DisplayOption::default(),
        }
    }
}
