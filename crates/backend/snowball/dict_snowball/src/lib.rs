#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

pub mod api;
pub mod builtins;
pub mod dict;
pub mod mem;
pub mod types;
pub mod utilities;

// Generated Snowball stemmer modules (c2rust translation of PostgreSQL 18.3
// `src/backend/snowball/libstemmer/stem_*.c`), linking against this crate's
// runtime (`api`/`utilities`/`types`). The non-english modules keep the raw
// transpiled shape, hence the broad per-module allow list.
#[allow(
    dead_code,
    unused_mut,
    unused_assignments,
    static_mut_refs,
    unused_variables,
    unused_parens,
    unused_unsafe,
    clippy::all
)]
pub mod stemmers {
    #[cfg(feature = "lang-basque")]
    pub mod stem_iso_8859_1_basque;
    #[cfg(feature = "lang-catalan")]
    pub mod stem_iso_8859_1_catalan;
    #[cfg(feature = "lang-danish")]
    pub mod stem_iso_8859_1_danish;
    #[cfg(feature = "lang-dutch")]
    pub mod stem_iso_8859_1_dutch;
    #[cfg(feature = "lang-english")]
    pub mod stem_iso_8859_1_english;
    #[cfg(feature = "lang-finnish")]
    pub mod stem_iso_8859_1_finnish;
    #[cfg(feature = "lang-french")]
    pub mod stem_iso_8859_1_french;
    #[cfg(feature = "lang-german")]
    pub mod stem_iso_8859_1_german;
    #[cfg(feature = "lang-indonesian")]
    pub mod stem_iso_8859_1_indonesian;
    #[cfg(feature = "lang-irish")]
    pub mod stem_iso_8859_1_irish;
    #[cfg(feature = "lang-italian")]
    pub mod stem_iso_8859_1_italian;
    #[cfg(feature = "lang-norwegian")]
    pub mod stem_iso_8859_1_norwegian;
    #[cfg(feature = "lang-porter")]
    pub mod stem_iso_8859_1_porter;
    #[cfg(feature = "lang-portuguese")]
    pub mod stem_iso_8859_1_portuguese;
    #[cfg(feature = "lang-spanish")]
    pub mod stem_iso_8859_1_spanish;
    #[cfg(feature = "lang-swedish")]
    pub mod stem_iso_8859_1_swedish;
    #[cfg(feature = "lang-hungarian")]
    pub mod stem_iso_8859_2_hungarian;
    #[cfg(feature = "lang-russian")]
    pub mod stem_koi8_r_russian;
    #[cfg(feature = "lang-arabic")]
    pub mod stem_utf8_arabic;
    #[cfg(feature = "lang-armenian")]
    pub mod stem_utf8_armenian;
    #[cfg(feature = "lang-basque")]
    pub mod stem_utf8_basque;
    #[cfg(feature = "lang-catalan")]
    pub mod stem_utf8_catalan;
    #[cfg(feature = "lang-danish")]
    pub mod stem_utf8_danish;
    #[cfg(feature = "lang-dutch")]
    pub mod stem_utf8_dutch;
    #[cfg(feature = "lang-english")]
    pub mod stem_utf8_english;
    #[cfg(feature = "lang-estonian")]
    pub mod stem_utf8_estonian;
    #[cfg(feature = "lang-finnish")]
    pub mod stem_utf8_finnish;
    #[cfg(feature = "lang-french")]
    pub mod stem_utf8_french;
    #[cfg(feature = "lang-german")]
    pub mod stem_utf8_german;
    #[cfg(feature = "lang-greek")]
    pub mod stem_utf8_greek;
    #[cfg(feature = "lang-hindi")]
    pub mod stem_utf8_hindi;
    #[cfg(feature = "lang-hungarian")]
    pub mod stem_utf8_hungarian;
    #[cfg(feature = "lang-indonesian")]
    pub mod stem_utf8_indonesian;
    #[cfg(feature = "lang-irish")]
    pub mod stem_utf8_irish;
    #[cfg(feature = "lang-italian")]
    pub mod stem_utf8_italian;
    #[cfg(feature = "lang-lithuanian")]
    pub mod stem_utf8_lithuanian;
    #[cfg(feature = "lang-nepali")]
    pub mod stem_utf8_nepali;
    #[cfg(feature = "lang-norwegian")]
    pub mod stem_utf8_norwegian;
    #[cfg(feature = "lang-porter")]
    pub mod stem_utf8_porter;
    #[cfg(feature = "lang-portuguese")]
    pub mod stem_utf8_portuguese;
    #[cfg(feature = "lang-romanian")]
    pub mod stem_utf8_romanian;
    #[cfg(feature = "lang-russian")]
    pub mod stem_utf8_russian;
    #[cfg(feature = "lang-serbian")]
    pub mod stem_utf8_serbian;
    #[cfg(feature = "lang-spanish")]
    pub mod stem_utf8_spanish;
    #[cfg(feature = "lang-swedish")]
    pub mod stem_utf8_swedish;
    #[cfg(feature = "lang-tamil")]
    pub mod stem_utf8_tamil;
    #[cfg(feature = "lang-turkish")]
    pub mod stem_utf8_turkish;
    #[cfg(feature = "lang-yiddish")]
    pub mod stem_utf8_yiddish;
}

#[cfg(all(test, feature = "all-languages"))]
mod tests;

// dict_snowball.so's entry points (PG_MODULE_MAGIC_EXT, no _PG_init):
// snowball_create.sql declares dsnowball_init/dsnowball_lexize as LANGUAGE C
// AS '$libdir/dict_snowball', so fmgr's C-language leg resolves them through
// the dfmgr library registry (fmgr.c fmgr_info_C_lang -> dfmgr.c
// load_external_function), which records the library's first use in the
// session for pg_get_loaded_modules.
pub fn init_seams() {
    dfmgr::register_builtin_library(dfmgr::BuiltinLibraryEntry {
        name: builtins::SNOWBALL_LIBRARY,
        lookup: |symbol| {
            builtins::SNOWBALL_CLANG
                .iter()
                .find(|(name, _, _)| *name == symbol)
                .map(|&(_, _, func)| func)
        },
        pg_init: None,
    });
}
