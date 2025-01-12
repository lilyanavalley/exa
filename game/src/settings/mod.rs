
use crate::GAME_VERSION;
use core::any::Any;
use std::{ 
    default::Default,
    collections::HashMap,
    hash::Hash,
    { io, io::{ Write, Read }, fs }
};
use fyrox:: {
    asset::manager::ResourceManager, 
    core:: { 
        pool::Handle, reflect::prelude::*, visitor::prelude::*
    }, 
    dpi::PhysicalPosition, 
    event::*, 
    gui::UiNode, 
    keyboard::*, 
    plugin
};
use tracing:: { trace, trace_span, info, info_span, warn, warn_span, error, error_span };
use serde::{Deserialize, Serialize};

mod input;

const SETTINGS_SAVENAME: &'static str = "gamesettings.ron";
const SETTINGS_SAVENAME_OLD: &'static str = "gamesettings.ron.old";


// trait SettingsComponent<A, H, V, R>

//     where A: Any, H: Hash, V: Visit, R: Reflect {

//     // fn new_ui_node(&self, context: &mut plugin::PluginContext) -> Handle<UiNode>;
//     fn save(&self, context: &mut plugin::PluginContext) -> VisitResult;
//     fn load(&self, context: &mut plugin::PluginContext) -> VisitResult;

// }


mod tests {

    use super::*;


    #[test]
    fn test_settings() {
    
        let settings = Settings {
            display:        DisplaySettings::default(),
            input:          input::InputSettingsComponent::default(),
            _written_version: String::from(GAME_VERSION)
        };
    
        assert_eq!(settings.version_matches("0.0.0"), false);
        // TODO
    
    }

    #[test]
    fn test_settings_display() {

        // TODO

    }

    #[test]
    fn test_settings_saveload_sanity() {

        let settings_saved = Settings::default()
            .save()
            .unwrap();

        let settings_loaded = Settings::default()
            .load(None)
            .unwrap();

    }

}


#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {

    /// Display and game window preferences.
    pub display:        DisplaySettings,

    /// Keyboard, mouse and controller input settings.
    pub input:          input::InputSettingsComponent,

    // Version of game at time of last save.
    _written_version:   String,

}

impl Settings {

    // pub fn input(&self) -> &input::InputSettingsComponent {
    //     &self.input
    // }

    // pub fn version(&self) -> &str {
    //     &self._write_version
    // }

    /// Write a file with the content of `Settings` serialized in [Rusty Object Notation][1].
    /// This file is meant to persist game settings across runtime sessions for *this machine*; Settings on one machine
    /// should not be transfered to a different machine, as doing so may cause [strange misconfigurations][2].
    /// 
    /// **Important details:**
    /// 1. The location of this file is dependent on the Operating System and where it stores application files.
    //    TODO: Implement this functionality ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// 2. A backup of the last settings configuration is kept along with the [version we're saving][3].
    /// 
    /// [1]: https://docs.rs/ron/0.8.1/ron/index.html
    /// [2]: Serialization of system-specific event codes prevents such transfer from being a *comfortable experience*
    ///  to foreign systems.
    /// [3]: Backup copy is available for restoring previous settings.
    /// 
    pub fn save(&self) -> io::Result<()> {

        if let Err(why) = fs::copy(SETTINGS_SAVENAME, SETTINGS_SAVENAME_OLD) {
            warn!("Copy file '{}' -> '{}' failed: {:?}", SETTINGS_SAVENAME, SETTINGS_SAVENAME_OLD, why);
        }

        let buffer = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .expect("Serialization error!"); // TODO: Replace with something less crashy.

        let mut new_save = fs::File::create(SETTINGS_SAVENAME)?;
        new_save.write_all(buffer.as_bytes())

    }

    /// Load the content of the settings save file previously serialized in [Rusty Object Notation][1].
    /// This file is meant to persist game settings across runtime sessions for *this machine*; Settings on one machine
    /// should not be transfered to a different machine, as doing so may cause [strange misconfigurations][2]. 
    /// 
    /// You may override the `path` of the file to read from by passing `Some(PathBuf)` as an argument, otherwise the 
    /// default location is read from instead.
    /// 
    /// [1]: https://docs.rs/ron/0.8.1/ron/index.html
    /// [2]: Serialization of system-specific event codes prevents such transfer from being a *comfortable experience*
    ///  to foreign systems.
    /// 
    pub fn load(&mut self, path: Option<std::path::PathBuf>) -> io::Result<()> {

        let read_save = fs::File::open(path.unwrap_or(SETTINGS_SAVENAME.into()))?;
        *self = ron::de::from_reader(read_save).unwrap(); // TODO: Replace .unwrap()
        Ok(())

    }

    // Perform equality test of `self.write_version` with provided `other`.
    // 
    fn version_matches(&self, other: &str) -> bool {
        if &self._written_version == other { true }
        else { false }
    }

}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            display:            DisplaySettings::default(),
            input:              input::InputSettingsComponent::default(),
            _written_version:     String::from(GAME_VERSION)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplaySettings {

    /// Fullscreen preference.
    pub fullscreen:         bool,

    pub resolution_width:   u32,
    pub resolution_height:  u32,
    pub scalefactor:        f32,

    /// Monitor preference.
    //? Represents the index # in the collection of monitors provided by Fyrox and points to the desired one.
    //? Default is 0 which equates to the primary monitor.
    pub monitor:            u8

}

impl Default for DisplaySettings {
    fn default() -> Self {
        DisplaySettings {
            fullscreen:         false,
            resolution_width:   960,
            resolution_height:  540,
            scalefactor:        1.0,
            monitor:            0
        }
    }
}
