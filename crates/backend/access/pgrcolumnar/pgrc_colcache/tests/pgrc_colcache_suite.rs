//! pgrc_colcache's integration tests (tests/<stem>.rs) that can share one process, as ONE test binary: one module
//! `t_<stem>` per file. As one binary per file, every edit under the package or a crate it uses compiled and linked each
//! of them again on its own. Now these are one link.
//! Run one file with `cargo test -p pgrc_colcache --test pgrc_colcache_suite t_<stem>::`.
//! A plain `cargo test -p pgrc_colcache` runs every module here in ONE process, so every module must pass beside the others.
//! A file stays its own [[test]] in Cargo.toml when it cannot share a process (tests/registered.rs's rule).
//! tests/registered.rs (the module t_registered) checks that every test file is in exactly one of the two places.
#[path = "compacted_keying.rs"]
mod t_compacted_keying;
#[path = "lock_only.rs"]
mod t_lock_only;
#[path = "lock_rows.rs"]
mod t_lock_rows;
#[path = "model.rs"]
mod t_model;
#[path = "registered.rs"]
mod t_registered;
#[path = "view_key.rs"]
mod t_view_key;
#[path = "view_model.rs"]
mod t_view_model;
