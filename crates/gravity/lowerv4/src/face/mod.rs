//! The hook interfaces (plan §3.1): the variant lists (`kinds`, `roads`, closed at M0) and one face per hook family. Its
//! module lines live here, not in the crate root (rule R9; construct bundle core-face-09).
pub mod consts;
pub(crate) mod cosplit;
pub mod end;
pub mod ext;
pub mod fold;
pub mod kinds;
pub(crate) mod partfill;
pub(crate) mod partruns;
pub mod policy;
pub(crate) mod pull;
pub mod rewrite;
pub mod runs;
pub(crate) mod slot_run;
pub(crate) mod slot_window;
pub mod source;
pub mod spill;
pub mod step;
pub mod strategy;
