//! A build without `native-code` (gravity_tpde_off): every compile is refused, a program the interpreter accepts runs
//! interpreted instead of failing the statement, and a program it refuses keeps the compile's error. Compiled to nothing
//! in the default build.
#![cfg(not(feature = "native-code"))]

use std::sync::atomic::Ordering;

use gravity_backend::{mapped_cached_all, MAPPING_REFUSED_INTERPRETED};
use gravity_ir::text::parse_program;
use gravity_ir::Func;

#[test]
fn a_program_runs_interpreted_when_the_build_has_no_code_generator() {
    let src = "(program (fn e ((c ptr)) (i64) (block entry ((c ptr)) (k (const i64 7301)) (return k))))";
    let p = parse_program(src, &|_| None).unwrap_or_else(|e| panic!("{e}"));
    let f = Func(p.funcs.iter().position(|f| f.name == "e").unwrap() as u32);
    let before = MAPPING_REFUSED_INTERPRETED.load(Ordering::Relaxed);
    let (codes, _) = mapped_cached_all(&p, &[f]).unwrap();
    assert_eq!(codes[0].kind(), "interp", "interpreted, not mapped");
    // SAFETY: the program never reads its ctx.
    assert_eq!(unsafe { codes[0].call(core::ptr::null_mut()) }, 7301);
    assert!(MAPPING_REFUSED_INTERPRETED.load(Ordering::Relaxed) > before, "the refusal was counted");
}
