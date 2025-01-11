
// This file is part of EXA.
// EXA is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as 
// published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// EXA is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with EXA. If not, see 
// <https://www.gnu.org/licenses/>.

use std:: { fs, io, io::Read, path:: { Path, PathBuf }, sync::{ Arc, Mutex, RwLock } };

use fluent_bundle:: { FluentResource, FluentError, bundle::FluentBundle, concurrent::FluentBundle as ConcurrentFluentBundle };
use tracing::trace_span;
use unic_langid_impl;
use unic_langid_macros::*;
use intl_memoizer::concurrent::IntlLangMemoizer;


/// Cached fluent files.
// #[derive(Default)]
pub struct FluentCache {

    pub bundle:     FluentBundle<FluentResource, IntlLangMemoizer>,
    pub errors:     Vec<FluentError>

}

impl FluentCache {

    /// Create a new empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pull the default language file and return a `Future` with data at a later time.
    pub async fn default_later() -> Result<ConcurrentFluentBundle<FluentResource>, io::Error> {
        
        let ftl_resource = FluentCache::retrieve_file(
            PathBuf::from("data/"),
            LanguagesSupported::lang_en_us()
        ).unwrap();

        // TODO: Replace .unwrap()
        let ftl_resource = String::from_utf8(ftl_resource).unwrap();

        // TODO: Replace .unwrap()
        let ftl_resource = fluent_bundle::FluentResource::try_new(ftl_resource).unwrap();

        let mut ftl_bundle = fluent_bundle::concurrent::FluentBundle::new_concurrent(
            vec![unic_langid_macros::langid!("en-US")]
        );
        ftl_bundle.add_resource(ftl_resource).unwrap();

        Ok(ftl_bundle)

    }

    // TODO: Document this whole thing.
    /// Reads a fluent file based on provided path to data folder `prefix` and the language `lang`.
    pub fn retrieve_file(mut prefix: PathBuf, lang: LanguagesSupported) -> io::Result<Vec<u8>> {
        prefix.push("languages/");
        let ftl_path = lang.file(Some(&prefix));
        let mut ftl_resource = Vec::new();
        let mut fs = fs::File::open(ftl_path)?;
        fs.read_to_end(&mut ftl_resource)?;
        Ok(ftl_resource)
    }

    // // TODO: Document this whole thing.
    // pub fn bundle(&self) -> &Option<FluentBundle<FluentResource>> {
    //     &self.bundle
    // }

    // // TODO: Document this whole thing.
    // pub fn bundle_mut(&mut self) -> &mut Option<FluentBundle<FluentResource>> {
    //     &mut self.bundle
    // }

    /// Stringify Fluent errors and collect them.
    pub fn errors(&self) -> Vec<String> {
        let mut vec = Vec::new();
        for each_error in &self.errors {
            vec.push(format!("Fluent error: {}", each_error));
        };
        vec
    }

}

impl std::fmt::Debug for FluentCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Document this whole thing.
        f.debug_struct("FluentCache")
            // TODO: Reintroduce this next line to debug it:
            .field("bundle", &self.bundle.locales)
            .field("errors", &self.errors)
        .finish()
    }
}

impl Default for FluentCache {
    fn default() -> Self {
        FluentCache {
            bundle:     FluentBundle::new_concurrent(Vec::new()),
            errors:     Vec::new()
        }
    }
}

// TODO: Document.
#[derive(Debug, PartialEq, Clone)]
pub enum LanguagesSupported {

    EnglishUS,

    FrançiasFR,

}

impl LanguagesSupported {

    // TODO: Document.
    pub fn lang_en_us() -> Self {
        LanguagesSupported::EnglishUS
    }

    // TODO: Document.
    pub fn lang_fr_fr() -> Self {
        LanguagesSupported::FrançiasFR
    }

    /// Determines the FTL filename of `self`.
    fn file(&self, prefix: Option<&Path>) -> PathBuf {
        let mut pathbuf = PathBuf::new();
        if let Some(with_prefix) = prefix {
            pathbuf.push(with_prefix);
        }
        pathbuf.push(format!("{}.ftl", self.langid().to_string()));
        pathbuf
    }

    /// Detremines the *langid* of self, returning `unic_langid_impl::LanguageIdentifier`.
    fn langid(&self) -> unic_langid_impl::LanguageIdentifier {
        match self {
            Self::EnglishUS     => langid!("en-US"),
            Self::FrançiasFR    => langid!("fr-FR"),
        }
    }

    // /// Determines the filesystem path pointing to the FTL file for the supported language.
    // /// 
    // /// Takes `data_dir` as a path to the data directory for this game.
    // fn which(&self, data_dir: PathBuf) -> PathBuf {
    //     // data_dir.push("languages/");
    //     // match self {
    //     //     Self::EnglishUS     => { data_dir.push(self.file(None)); data_dir },
    //     //     Self::FrançiasFR    => { data_dir.push(self.file(None)); data_dir },
    //     // }
    //     self.file(Some(&data_dir))
    // }

}

impl Default for LanguagesSupported {
    fn default() -> Self {
        Self::EnglishUS
    }
}

impl ToString for LanguagesSupported {
    fn to_string(&self) -> String {
        match self {
            Self::EnglishUS     => "English (US)".to_string(),
            Self::FrançiasFR    => "Françias (FR)".to_string()
        }
    }
}

// TOOD: Document.
pub enum FluentMessage {

    /// Add language to bundle cache.
    AddLanguage(LanguagesSupported),
    /// Invalidate bundle cache; resets to default.
    CacheInvalidate,
    /// Reload bundle cache to game preference.
    CacheReload

}

mod tests {

    use crate::components::fluent;
    use super::*;


    // #[test]
    // fn test_fluentcache_addlanguage() {
        
    //     let mut fluentcache = FluentCache::default();
    //     fluentcache.bundle

    //     assert_eq!(fluentcache.bundle.has_message("coremenu"), true); // TODO: Write this.

    // }

    #[test]
    fn test_fluentcache() {

        // TODO: Document this whole thing.

        let mut fluentcache = FluentCache::default();

        assert_eq!(fluentcache.bundle.has_message("ggez"), false);
        assert_eq!(fluentcache.bundle.get_message("ggez"), None);

        let frfr= "fr fr!!";
        let stringy = format!("ggez = {frfr}");
        let resource = FluentResource::try_new(stringy).unwrap();
        fluentcache.bundle.add_resource(resource);

        let ggez = fluentcache.bundle.get_message("ggez").unwrap().value().unwrap();

        assert_eq!(fluentcache.bundle.has_message("ggez"), true);
        assert_eq!(
            fluentcache.bundle.format_pattern(ggez, None, &mut vec![]),
            frfr
        );

    }

    #[test]
    fn test_fluentcache_debugimpl() {

        let fluentcache = FluentCache::default();

        assert_eq!(
            format!("{:?}", fluentcache),
            "FluentCache { bundle: [], errors: [] }"
        );
        // TODO: Document.

    }

    #[test]
    fn test_languagessupported_new() {

        let call_ls_en_us = LanguagesSupported::lang_en_us();
        let call_ls_fr_fr = LanguagesSupported::lang_fr_fr();
        let manual_ls_en_us = LanguagesSupported::EnglishUS;
        let manual_ls_fr_fr = LanguagesSupported::FrançiasFR;

        // Function call and manual initalization results must be equal.
        assert_eq!(manual_ls_en_us, call_ls_en_us);
        assert_eq!(manual_ls_fr_fr, call_ls_fr_fr);

        // Supported language variants should never be equal to another variant.
        assert_ne!(manual_ls_en_us, manual_ls_fr_fr);
        assert_ne!(call_ls_en_us, call_ls_fr_fr);

    }

    #[test]
    fn test_languagessupported_langid() {

        let ls_en_us = LanguagesSupported::EnglishUS;
        let ls_fr_fr = LanguagesSupported::FrançiasFR;

        // LangIDs must match the macro equivalent.
        assert_eq!(ls_en_us.langid(), langid!("en-US"));
        assert_eq!(ls_fr_fr.langid(), langid!("fr-FR"));

        // LangIDs across different languages should never be equal.
        assert_ne!(ls_en_us.langid(), ls_fr_fr.langid());

        // TODO: Write test to completion.

    }

    #[test]
    fn test_languagessupported_file() {

        let ls_en_us = LanguagesSupported::EnglishUS;
        let ls_fr_fr = LanguagesSupported::FrançiasFR;

        let ls_en_us_file = ls_en_us.file(None);
        let ls_fr_fr_file = ls_fr_fr.file(None);
        let ls_en_us_file_withprefix = ls_en_us.file(Some(&PathBuf::from("/mydir")));

        // FTL files should equal expected RHS value according to their type.
        assert_eq!(ls_en_us_file, PathBuf::from("en-US.ftl"));
        assert_eq!(ls_fr_fr_file, PathBuf::from("fr-FR.ftl"));
        // A language with a prefix must match the expected RHS value.
        assert_eq!(ls_en_us_file_withprefix, PathBuf::from("/mydir/en-US.ftl"));

        // Language variants should never be equal to another variant.
        assert_ne!(ls_en_us, ls_fr_fr);
        assert_ne!(ls_en_us_file, ls_fr_fr_file);

    }

}
