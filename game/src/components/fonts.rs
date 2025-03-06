
use std::{ ffi::OsString, fs, io, path::{ Path, PathBuf }, sync::Arc, collections::BTreeMap };
use fyrox:: { asset::manager::ResourceManager, gui::font::{ Font, FontResource }, resource };
use tracing:: { trace, trace_span, debug, debug_span, info, info_span, warn, warn_span, error, error_span, instrument::Instrument, instrument };


#[derive(Debug, Default)]
pub struct Fonts {

    /// Collection of fonts in format `(O, R)` where `O` is a font's file name (not a `Path` or its true location on 
    /// the filesystem) and `R` is the Font Resource kept in the engine.
    pub map:    BTreeMap<OsString, FontResource>,

}

impl Fonts {

    /// Load all fonts in the data directory.
    /// 
    /// *Scans the* `fonts/` *directory.* A Future is returned with an `io::Result<Self>`.
    pub async fn load(data_path_prefix: &Path, resource_manager: ResourceManager) -> io::Result<Self> {
        
        let mut map = BTreeMap::new();
        let data_path_prefix = PathBuf::from(data_path_prefix).join("fonts/");
        
        // Check if the prefix is not a directory or doesn't exist.
        if !data_path_prefix.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                format!("Path is not directory: {:?}", data_path_prefix)
            ))
        }
        else if !data_path_prefix.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Path does not exist: {:?}", data_path_prefix)
            ))
        }

        for each_entry in fs::read_dir(data_path_prefix)? {
            if let Ok(each_entry) = each_entry {
                let each_entry = each_entry.path();
                // Only immediate files are considered in this loop.
                if each_entry.is_file() {
                    if let Some(font) = resource_manager.try_request::<Font>(&each_entry) {
                        map.insert(each_entry.clone().file_name().unwrap().to_owned(), font);
                        info!("Font loaded: {:?}", each_entry);
                    }
                    else {
                        error!("Font file may be malformed: {:?}", each_entry);
                    }
                }
            }
        }

        Ok(Fonts {
            map,
        })

    }

    /// Takes an exact name for a font file and returns its file (if it exists.)
    /// 
    /// Clones the resource from its ownership in Fyrox engine.
    pub fn select_exact(&self, font_name: &OsString) -> Option<FontResource> {
        match self.map.get(font_name) {
            Some(font)  => Some(font.clone()),
            None        => None
        }
    }

}

#[derive(Debug)]
struct FontsEntry {

    /// Path to font file.
    file:       PathBuf,
    /// Key to represent this font within the Font component.
    key:        String,

}

mod tests {
    
    use std::{future::IntoFuture, sync::Arc};
    use fyrox:: { core::futures::{FutureExt, TryFutureExt}, generic_animation::core::task::TaskPool };
    use super::*;


    // #[test]
    // fn test_fonts_load() {

    //     let task_pool = Arc::new(TaskPool::new());
    //     let fonts = Fonts::load(
    //         Path::new("data/"),
    //         ResourceManager::new(task_pool)
    //     );

    //     // todo: this function needs an async context to run within.
    //     // todo: function `select_exact()` is async.
    //     assert!(fonts.select_exact(OsString::from("Ubuntu-Medium.ttf")).is_some())

    // }

    #[test]
    fn test_fonts_default() {
        assert_eq!(
            Fonts::default().map,
            BTreeMap::new()
        );
    }

    // fn test_fonts_artificalinsertion() {
    //     let fonts = Fonts::default();
    //     assert_eq!(
    // 
    //     )
    // }

}
