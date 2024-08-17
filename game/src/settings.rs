
use crate::GAME_VERSION;
use std::{ 
    default::Default,
    fmt::{ Display, Write }
};
use fyrox:: {
    gui,
    gui::UiNode,
    asset::manager::ResourceManager,
    core:: { reflect::prelude::*, visitor::prelude::*, pool::Handle },
    raw_window_handle::DisplayHandle
};


const GAMESETTINGS_FILEPATH:        &'static str = "gamesettings.ron";      // Current configuration file.
const GAMESETTINGS_BACKUPFILEPATH:  &'static str = "gamesettings.backup";   // Backup example file or latest working...


#[derive(Debug, Visit, Reflect)]
pub struct Settings {

    /// Display and game window preferences.
    pub display:        DisplaySettings,

    // Version of game at time of last save.
    _write_version:     String,

}

impl Settings {

    pub fn load_or_default(resource_manager: &ResourceManager) -> Self {
        // match resource_manager.try_request(&std::path::Path::new(GAMESETTINGS_FILEPATH)) {
        //     Some(settingsfile)  => {
                
        //     },
        //     None                => Settings::default()
        // }
        Settings::default()
    }

    pub fn version(&self) -> &str {
        &self._write_version
    }

}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            display:            DisplaySettings::default(),
            _write_version:     String::from(GAME_VERSION)
        }
    }
}

#[derive(Debug, Visit, Reflect)]
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
