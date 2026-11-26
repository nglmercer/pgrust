//! Gravity ctx ABI: the Ctx header, Site windows, Program block, Out block, the one offset table
//! and the status vocabulary (docs/gravity/abi.md, docs/gravity/abi-v2.md).
#![no_std]

pub mod breaker;
pub mod buildrows;
pub mod ctx;
pub mod hash;
pub mod host;
pub mod key;
pub mod layout;
pub mod limits;
pub mod program;
pub mod site;
pub mod spill;
pub mod status;
pub mod t3;

pub use breaker::{
    hash_at, keys_eq, rec_face, rec_open, table_kind, AccCell, AggSlot, Block, Breaker,
    BreakerKind, Budget, Chain, DenseFront, HashAgg, HashBuild, HashBuildEntry, KeyCell, KeyKind,
    Owned, OwnerFns, OwnerTop, Rec, RecFace, RecOpenFn, RecordLayout, SlotKind, SortRun, Spill,
    TopK, ACC_CELL, BUILD_PARTITION, BUILD_TABLE, DENSE, FLAG_ACC, FLAG_FRONT, HASHAGG_HDR,
    HASHBUILD_ENTRY, HASHBUILD_HDR, HASHED, LAYOUT_IMPLICIT_COUNT, LAYOUT_VALIDITY,
    MAX_RECORD_KEYS, MAX_RECORD_SLOTS, NO_VALIDITY, ORDER_KEY, ORDER_NONE, OWNED_HDR, OWNERTOP_HDR,
    RECORD_HDR, SELECT_MAX, SHARE, SORTRUN_HDR, T1, T1_BYPASS, T1_HDR, TABLE_AGG, TABLE_BUILD,
    TABLE_KIND_OFF, TABLE_OWNED, TABLE_SORT, TABLE_T1, TOPK_TIES, USED_NULL,
};
pub use ctx::lane;
pub use ctx::{
    Answer, Ctx, Datum, Identity, Lane, MorselRange, Out, Partials, ScanPos, ANSWER_MINMAX,
    ANSWER_SIZE, ANSWER_SUM, HEADER_SIZE, IDENTITY_SIZE, LANE_SIZE, OUT_SIZE, PARTIALS_SIZE,
};
pub use layout::{unword, word, PtrWord, Word};
pub use program::{ProgramBlock, PROGRAM_SIZE};
pub use site::{Site, SITE_SIZE};
pub use status::Status;
