use datum::Datum;
use types_core::InvalidOid;
use types_error::PgResult;
use types_fmgr::builtin;
use types_fmgr::{varlena_result, FmgrBuiltin, FmgrInfo, FunctionCallInfoBaseData as Fcinfo};

pub fn fc_enum_in(_flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    // SAFETY: catalog arg 0 of enum_in is cstring (typlen -2).
    let name = unsafe { fcinfo.arg_cstring(0) }.to_bytes();
    let [_, typoid] = fcinfo.args_n::<2>();
    // SAFETY: context, if set, rides per the ErrorSaveNode contract.
    let esc = unsafe { fcinfo.soft_error_context() };
    Ok(Datum::from_oid(
        crate::enum_in(name, typoid.value.as_oid(), esc)?.unwrap_or(InvalidOid),
    ))
}

pub fn fc_enum_out(flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    let [a] = fcinfo.args_n::<1>();
    let en = crate::enum_out(a.value.as_oid())?;
    Ok(::types_fmgr::cstring_scratch(
        flinfo,
        "enum_out",
        en.enumlabel.name_str(),
    ))
}

pub fn fc_enum_recv(_flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    // SAFETY: recv arg 0 is the live StringInfo pointer per the recv ABI.
    let buf = unsafe { fcinfo.arg_stringinfo(0) };
    let [_, typoid] = fcinfo.args_n::<2>();
    let mcx = fcinfo.result_mcx();
    let rawbytes = buf.len().saturating_sub(buf.cursor);
    let name = pqformat::pq_getmsgtext(mcx, buf, rawbytes)?;
    // Hard error either way here (no ereturn in the C).
    match crate::enum_in(&name, typoid.value.as_oid(), None)? {
        Some(oid) => Ok(Datum::from_oid(oid)),
        None => unreachable!("enum_in without escontext returned soft error"),
    }
}

pub fn fc_enum_send(_flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    let [a] = fcinfo.args_n::<1>();
    let en = crate::enum_out(a.value.as_oid())?;
    let mcx = fcinfo.result_mcx();
    let mut buf = pqformat::pq_begintypsend(mcx)?;
    pqformat::pq_sendtext(&mut buf, en.enumlabel.name_str())?;
    Ok(varlena_result(pqformat::pq_endtypsend(buf)))
}

pub fn fc_enum_eq(_flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    let [a, b] = fcinfo.args_n::<2>();
    Ok(Datum::from_bool(a.value.as_oid() == b.value.as_oid()))
}

pub fn fc_enum_ne(_flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    let [a, b] = fcinfo.args_n::<2>();
    Ok(Datum::from_bool(a.value.as_oid() != b.value.as_oid()))
}

macro_rules! fc_cmp {
    ($($name:ident: $op:tt $rhs:literal;)+) => {
        $(pub fn $name(flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
            Ok(Datum::from_bool(crate::cmp_via(fcinfo, flinfo)? $op $rhs))
        })+
    };
}

fc_cmp! {
    fc_enum_lt: < 0;
    fc_enum_le: <= 0;
    fc_enum_ge: >= 0;
    fc_enum_gt: > 0;
}

pub fn fc_enum_cmp(flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    Ok(Datum::from_i32(crate::cmp_via(fcinfo, flinfo)?))
}

pub fn fc_enum_smaller(flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    let [a, b] = fcinfo.args_n::<2>();
    let (a, b) = (a.value.as_oid(), b.value.as_oid());
    Ok(Datum::from_oid(if crate::cmp_via(fcinfo, flinfo)? < 0 {
        a
    } else {
        b
    }))
}

pub fn fc_enum_larger(flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    let [a, b] = fcinfo.args_n::<2>();
    let (a, b) = (a.value.as_oid(), b.value.as_oid());
    Ok(Datum::from_oid(if crate::cmp_via(fcinfo, flinfo)? > 0 {
        a
    } else {
        b
    }))
}

pub fn fc_hashenum(_flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    let [a] = fcinfo.args_n::<1>();
    Ok(Datum::from_u32(hashfn::hash_bytes_uint32(a.value.as_oid())))
}

pub fn fc_hashenumextended(_flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    let [a, seed] = fcinfo.args_n::<2>();
    Ok(Datum::from_u64(hashfn::hash_bytes_uint32_extended(
        a.value.as_oid(),
        seed.value.as_u64(),
    )))
}

pub fn fc_enum_first(flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    let mcx = fcinfo.result_mcx();
    Ok(Datum::from_oid(crate::enum_first_last(
        mcx,
        flinfo.as_deref(),
        false,
    )?))
}

pub fn fc_enum_last(flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    let mcx = fcinfo.result_mcx();
    Ok(Datum::from_oid(crate::enum_first_last(
        mcx,
        flinfo.as_deref(),
        true,
    )?))
}

pub fn fc_enum_range_bounds(flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    let lower = if fcinfo.argisnull(0) {
        InvalidOid
    } else {
        fcinfo.arg(0).as_oid()
    };
    let upper = if fcinfo.argisnull(1) {
        InvalidOid
    } else {
        fcinfo.arg(1).as_oid()
    };
    let enumtypoid = crate::enum_range_typoid(flinfo.as_deref())?;
    let mcx = fcinfo.result_mcx();
    crate::enum_range_internal(mcx, enumtypoid, lower, upper)
}

pub fn fc_enum_range_all(flinfo: Option<&mut FmgrInfo>, fcinfo: &mut Fcinfo) -> PgResult<Datum> {
    let enumtypoid = crate::enum_range_typoid(flinfo.as_deref())?;
    let mcx = fcinfo.result_mcx();
    crate::enum_range_internal(mcx, enumtypoid, InvalidOid, InvalidOid)
}

// OIDs verified against pg_proc.dat 18.3.
pub static ENUM_BUILTINS: &[FmgrBuiltin] = &[
    builtin(3506, "enum_in", 2, true, false, fc_enum_in),
    builtin(3507, "enum_out", 1, true, false, fc_enum_out),
    // anyenum_out (pseudotypes.c) is `return enum_out(fcinfo)`.
    builtin(3505, "anyenum_out", 1, true, false, fc_enum_out),
    builtin(3508, "enum_eq", 2, true, false, fc_enum_eq),
    builtin(3509, "enum_ne", 2, true, false, fc_enum_ne),
    builtin(3510, "enum_lt", 2, true, false, fc_enum_lt),
    builtin(3511, "enum_gt", 2, true, false, fc_enum_gt),
    builtin(3512, "enum_le", 2, true, false, fc_enum_le),
    builtin(3513, "enum_ge", 2, true, false, fc_enum_ge),
    builtin(3514, "enum_cmp", 2, true, false, fc_enum_cmp),
    builtin(3515, "hashenum", 1, true, false, fc_hashenum),
    builtin(
        3414,
        "hashenumextended",
        2,
        true,
        false,
        fc_hashenumextended,
    ),
    builtin(3524, "enum_smaller", 2, true, false, fc_enum_smaller),
    builtin(3525, "enum_larger", 2, true, false, fc_enum_larger),
    builtin(3528, "enum_first", 1, false, false, fc_enum_first),
    builtin(3529, "enum_last", 1, false, false, fc_enum_last),
    builtin(
        3530,
        "enum_range_bounds",
        2,
        false,
        false,
        fc_enum_range_bounds,
    ),
    builtin(3531, "enum_range_all", 1, false, false, fc_enum_range_all),
    builtin(3532, "enum_recv", 2, true, false, fc_enum_recv),
    builtin(3533, "enum_send", 1, true, false, fc_enum_send),
];
