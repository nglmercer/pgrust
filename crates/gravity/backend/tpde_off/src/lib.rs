//! gravity_tpde without a code generator (see this crate's Cargo.toml): the same names and signatures the backend uses,
//! every compile refused with `MAPPING_REFUSED`, so a program runs interpreted (tier 0) or the statement fails with that
//! error when the interpreter refuses the program too. No `Compiled` value can exist.
use core::convert::Infallible;
use core::marker::PhantomData;
use gravity_ir::{Func, Program};
use mcx::Mcx;
use types_error::{PgError, PgResult};

pub const MAPPING_REFUSED: &str = "gravity_tpde: mapping refused on this platform";

/// This build has no code generator.
pub const NATIVE_CODE: bool = false;

pub mod view {
    pub const FUSION_NAMES: [&str; 7] = ["tpde.cmp_branch", "tpde.imm", "tpde.narrow_op", "tpde.narrow_move", "tpde.addr", "tpde.store_pair", "tpde.flag_reuse"];
}

pub mod adaptor {
    pub const JIT_PROLOGUE: u32 = u32::MAX;
    pub const JIT_EPILOGUE: u32 = u32::MAX - 1;
}

static CLOCK: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

pub fn set_clock(f: fn() -> u64) {
    CLOCK.store(f as usize, core::sync::atomic::Ordering::Relaxed);
}

pub fn now_ns() -> u64 {
    let p = CLOCK.load(core::sync::atomic::Ordering::Relaxed);
    if p == 0 {
        return 0;
    }
    #[allow(unsafe_code)]
    // SAFETY: `p` was stored by `set_clock` from a `fn() -> u64`.
    let f: fn() -> u64 = unsafe { core::mem::transmute::<usize, fn() -> u64>(p) };
    f()
}

/// The unwind payload of a stopped compile (never raised here: nothing compiles).
pub struct Interrupted;

pub fn stop_point() {
    if interrupt_pending() {
        std::panic::resume_unwind(Box::new(Interrupted))
    }
}

pub fn interrupt_pending() -> bool {
    cancelpoll::interrupts::pending()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileTiming {
    pub view_ns: u64,
    pub analysis_ns: u64,
    pub encode_ns: u64,
    pub install_ns: u64,
    pub total_ns: u64,
}

/// The mapped range of a compile (never one here).
pub struct MappedRange;

impl MappedRange {
    pub fn address(&self) -> usize {
        0
    }
    /// # Safety
    /// Nothing is mapped; the slice is empty.
    #[allow(unsafe_code)]
    pub unsafe fn bytes(&self) -> &[u8] {
        &[]
    }
}

pub struct Mapper;

impl Mapper {
    pub fn get_mapped_range(&self) -> Option<MappedRange> {
        None
    }
}

/// Uninhabited: `compile*` never returns one.
pub struct Compiled<'m> {
    never: Infallible,
    _m: PhantomData<&'m ()>,
}

impl<'m> Compiled<'m> {
    pub fn entry(&self, _f: Func) -> usize {
        match self.never {}
    }
    pub fn size(&self, _f: Func) -> usize {
        match self.never {}
    }
    pub fn ranges(&self, _f: Func) -> core::iter::Empty<(u32, u32)> {
        match self.never {}
    }
    pub fn code_bytes(&self) -> usize {
        match self.never {}
    }
    pub fn mapper(&self) -> &Mapper {
        match self.never {}
    }
    pub fn timing(&self) -> CompileTiming {
        match self.never {}
    }
    pub fn fusions(&self) -> u8 {
        match self.never {}
    }
}

#[derive(Default)]
pub struct Scratch;

impl Scratch {
    pub fn new() -> Self {
        Scratch
    }
    pub fn recording() -> Self {
        Scratch
    }
    pub fn owned(_own: impl Fn(&mcx::MemoryContext)) -> Self {
        Scratch
    }
}

fn refused<T>() -> PgResult<T> {
    Err(PgError::error(MAPPING_REFUSED.to_string()).into())
}

pub fn compile<'m>(_mcx: Mcx<'m>, _program: &Program) -> PgResult<Compiled<'m>> {
    refused()
}

pub fn compile_owned<'m>(_mcx: Mcx<'m>, _program: &Program, _own: impl Fn(&mcx::MemoryContext)) -> PgResult<Compiled<'m>> {
    refused()
}

pub fn compile_tagged<'m>(_mcx: Mcx<'m>, _program: &Program, _own: impl Fn(&mcx::MemoryContext), _tags: Option<&gravity_ir::JitTags>) -> PgResult<Compiled<'m>> {
    refused()
}

pub fn compile_recording<'m>(_mcx: Mcx<'m>, _program: &Program) -> PgResult<Compiled<'m>> {
    refused()
}

pub fn compile_with<'m>(_mcx: Mcx<'m>, _program: &Program, _scratch: &mut Scratch) -> PgResult<Compiled<'m>> {
    refused()
}
