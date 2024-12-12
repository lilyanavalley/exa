
use std:: { fs, io, io::Read, path:: { Path, PathBuf } };
use fluent_bundle::*;
use unic_langid_impl;
use unic_langid_macros::*;


/// Cached fluent files.
#[derive(Default)]
pub struct FluentCache {

    bundle:         FluentBundle<FluentResource>,
    errors:         Vec<FluentError>

}

impl FluentCache {

    /// Create a new empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a language layer to the cache.
    /// 
    /// Any language previously added to the cache will be replaced by new additions silently.
    /// 
    /// Takes `lang`: `LanguagesSupported` to specify which language to add;
    /// Takes `prefix`: `PathBuf` a path to the game's **data directory**.
    pub fn add_language(&mut self, lang: LanguagesSupported, prefix: PathBuf) {
        
        let ftl_resource = Self::retrieve_file(prefix, lang)
            .unwrap();
        
        // TODO: Replace .unwrap()
        let ftl_resource = String::from_utf8(ftl_resource).unwrap();

        // TODO: Replace .unwrap()
        let ftl_resource = FluentResource::try_new(ftl_resource).unwrap();
        
        // TODO: Replace .unwrap()
        self.bundle.add_resource(ftl_resource).unwrap();

    }

    fn retrieve_file(mut prefix: PathBuf, lang: LanguagesSupported) -> io::Result<Vec<u8>> {
        prefix.push("languages/");
        let ftl_path = lang.file(Some(&prefix));
        let mut ftl_resource = Vec::new();
        let mut fs = fs::File::open(ftl_path)?;
        fs.read_to_end(&mut ftl_resource)?;
        Ok(ftl_resource)
    }

}

impl std::fmt::Debug for FluentCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FluentCache")
            .field("bundle", &self.bundle.locales)
            .field("errors", &self.errors)
        .finish()
    }
}

// TODO: Document.
#[derive(Debug, PartialEq)]
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
    fn test_fluentcache_debugimpl() {

        let fluentcache = FluentCache::default();

        assert_eq!(
            format!("{:?}", fluentcache),
            "FluentCache { bundle: [LanguageIdentifier { language: Language(None), script: None, region: None, variants: None }], errors: [] }"
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
