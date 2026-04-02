use c2rust_bitfields::BitfieldStruct;
use ::libc;
extern "C" {
    pub type VdbeSorter;
    pub type BtCursor;
    pub type Btree;
    pub type RenameToken;
    pub type TableLock;
    pub type VtabCtx;
    pub type sqlite3_mutex;
    fn sqlite3_libversion() -> *const ::core::ffi::c_char;
    fn sqlite3_sourceid() -> *const ::core::ffi::c_char;
    fn sqlite3_compileoption_used(
        zOptName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_compileoption_get(N: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
    fn sqlite3_changes64(_: *mut sqlite3) -> sqlite3_int64;
    fn sqlite3_total_changes64(_: *mut sqlite3) -> sqlite3_int64;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_vmprintf(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_bytes16(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_numeric_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_encoding(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_subtype(_: *mut sqlite3_value) -> ::core::ffi::c_uint;
    fn sqlite3_value_dup(_: *const sqlite3_value) -> *mut sqlite3_value;
    fn sqlite3_value_free(_: *mut sqlite3_value);
    fn sqlite3_aggregate_context(
        _: *mut sqlite3_context,
        nBytes: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_blob64(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: sqlite3_uint64,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_double(_: *mut sqlite3_context, _: ::core::ffi::c_double);
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_error_toobig(_: *mut sqlite3_context);
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
    fn sqlite3_result_error_code(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_result_null(_: *mut sqlite3_context);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_text64(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: sqlite3_uint64,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        encoding: ::core::ffi::c_uchar,
    );
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value);
    fn sqlite3_result_zeroblob64(
        _: *mut sqlite3_context,
        n: sqlite3_uint64,
    ) -> ::core::ffi::c_int;
    fn sqlite3_load_extension(
        db: *mut sqlite3,
        zFile: *const ::core::ffi::c_char,
        zProc: *const ::core::ffi::c_char,
        pzErrMsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_overload_function(
        _: *mut sqlite3,
        zFuncName: *const ::core::ffi::c_char,
        nArg: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_str_appendf(
        _: *mut sqlite3_str,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3_str_append(
        _: *mut sqlite3_str,
        zIn: *const ::core::ffi::c_char,
        N: ::core::ffi::c_int,
    );
    fn sqlite3_str_appendchar(
        _: *mut sqlite3_str,
        N: ::core::ffi::c_int,
        C: ::core::ffi::c_char,
    );
    fn sqlite3_str_reset(_: *mut sqlite3_str);
    fn sqlite3_str_value(_: *mut sqlite3_str) -> *mut ::core::ffi::c_char;
    fn sqlite3_log(
        iErrCode: ::core::ffi::c_int,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn sqlite3MemCompare(
        _: *const Mem,
        _: *const Mem,
        _: *const CollSeq,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeFuncName(_: *const sqlite3_context) -> *const ::core::ffi::c_char;
    fn sqlite3WindowFunctions();
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3Malloc(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3Realloc(_: *mut ::core::ffi::c_void, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3IsOverflow(_: ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn sqlite3RowSetClear(_: *mut ::core::ffi::c_void);
    fn sqlite3InsertBuiltinFuncs(_: *mut FuncDef, _: ::core::ffi::c_int);
    fn sqlite3FindFunction(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: u8_0,
        _: u8_0,
    ) -> *mut FuncDef;
    fn sqlite3AppendOneUtf8Character(
        _: *mut ::core::ffi::c_char,
        _: u32_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3RegisterDateTimeFunctions();
    fn sqlite3RegisterJsonFunctions();
    fn sqlite3AtoF(
        z: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_double,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3Utf8CharLen(
        pData: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Utf8Read(_: *mut *const u8_0) -> u32_0;
    fn sqlite3HexToInt(h: ::core::ffi::c_int) -> u8_0;
    fn sqlite3AddInt64(_: *mut i64_0, _: i64_0) -> ::core::ffi::c_int;
    fn sqlite3SubInt64(_: *mut i64_0, _: i64_0) -> ::core::ffi::c_int;
    static sqlite3UpperToLower: [::core::ffi::c_uchar; 0];
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    fn sqlite3AlterFunctions();
    fn sqlite3CreateFunc(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
        _: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        _: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        _: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
        _: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
        _: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        pDestructor: *mut FuncDestructor,
    ) -> ::core::ffi::c_int;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3StrAccumInit(
        _: *mut StrAccum,
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3StrAccumEnlarge(_: *mut StrAccum, _: i64_0) -> ::core::ffi::c_int;
    fn sqlite3StrAccumFinish(_: *mut StrAccum) -> *mut ::core::ffi::c_char;
    fn sqlite3StrAccumSetError(_: *mut StrAccum, _: u8_0);
    fn sqlite3ResultStrAccum(_: *mut sqlite3_context, _: *mut StrAccum);
    fn acos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn asin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atan2(
        __y: ::core::ffi::c_double,
        __x: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn tan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cosh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sinh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn tanh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn acosh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn asinh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atanh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log2(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(
        __x: ::core::ffi::c_double,
        __y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fmod(
        __x: ::core::ffi::c_double,
        __y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn trunc(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqlite3VdbeMemCopy(_: *mut Mem, _: *const Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemRelease(p: *mut Mem);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void

}
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3 {
    pub pVfs: *mut sqlite3_vfs,
    pub pVdbe: *mut Vdbe,
    pub pDfltColl: *mut CollSeq,
    pub mutex: *mut sqlite3_mutex,
    pub aDb: *mut Db,
    pub nDb: ::core::ffi::c_int,
    pub mDbFlags: u32_0,
    pub flags: u64_0,
    pub lastRowid: i64_0,
    pub szMmap: i64_0,
    pub nSchemaLock: u32_0,
    pub openFlags: ::core::ffi::c_uint,
    pub errCode: ::core::ffi::c_int,
    pub errByteOffset: ::core::ffi::c_int,
    pub errMask: ::core::ffi::c_int,
    pub iSysErrno: ::core::ffi::c_int,
    pub dbOptFlags: u32_0,
    pub enc: u8_0,
    pub autoCommit: u8_0,
    pub temp_store: u8_0,
    pub mallocFailed: u8_0,
    pub bBenignMalloc: u8_0,
    pub dfltLockMode: u8_0,
    pub nextAutovac: ::core::ffi::c_schar,
    pub suppressErr: u8_0,
    pub vtabOnConflict: u8_0,
    pub isTransactionSavepoint: u8_0,
    pub mTrace: u8_0,
    pub noSharedCache: u8_0,
    pub nSqlExec: u8_0,
    pub eOpenState: u8_0,
    pub nextPagesize: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub nTotalChange: i64_0,
    pub aLimit: [::core::ffi::c_int; 12],
    pub nMaxSorterMmap: ::core::ffi::c_int,
    pub init: sqlite3InitInfo,
    pub nVdbeActive: ::core::ffi::c_int,
    pub nVdbeRead: ::core::ffi::c_int,
    pub nVdbeWrite: ::core::ffi::c_int,
    pub nVdbeExec: ::core::ffi::c_int,
    pub nVDestroy: ::core::ffi::c_int,
    pub nExtension: ::core::ffi::c_int,
    pub aExtension: *mut *mut ::core::ffi::c_void,
    pub trace: C2Rust_Unnamed_25,
    pub pTraceArg: *mut ::core::ffi::c_void,
    pub xProfile: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            u64_0,
        ) -> (),
    >,
    pub pProfileArg: *mut ::core::ffi::c_void,
    pub pCommitArg: *mut ::core::ffi::c_void,
    pub xCommitCallback: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub pRollbackArg: *mut ::core::ffi::c_void,
    pub xRollbackCallback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pUpdateArg: *mut ::core::ffi::c_void,
    pub xUpdateCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            sqlite_int64,
        ) -> (),
    >,
    pub pAutovacPagesArg: *mut ::core::ffi::c_void,
    pub xAutovacDestr: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub xAutovacPages: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            u32_0,
            u32_0,
            u32_0,
        ) -> ::core::ffi::c_uint,
    >,
    pub pParse: *mut Parse,
    pub pPreUpdateArg: *mut ::core::ffi::c_void,
    pub xPreUpdateCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            sqlite3_int64,
            sqlite3_int64,
        ) -> (),
    >,
    pub pPreUpdate: *mut PreUpdate,
    pub xWalCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pWalArg: *mut ::core::ffi::c_void,
    pub xCollNeeded: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub xCollNeeded16: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> (),
    >,
    pub pCollNeededArg: *mut ::core::ffi::c_void,
    pub pErr: *mut sqlite3_value,
    pub u1: C2Rust_Unnamed_22,
    pub lookaside: Lookaside,
    pub xAuth: sqlite3_xauth,
    pub pAuthArg: *mut ::core::ffi::c_void,
    pub xProgress: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub pProgressArg: *mut ::core::ffi::c_void,
    pub nProgressOps: ::core::ffi::c_uint,
    pub nVTrans: ::core::ffi::c_int,
    pub aModule: Hash,
    pub pVtabCtx: *mut VtabCtx,
    pub aVTrans: *mut *mut VTable,
    pub pDisconnect: *mut VTable,
    pub aFunc: Hash,
    pub aCollSeq: Hash,
    pub busyHandler: BusyHandler,
    pub aDbStatic: [Db; 2],
    pub pSavepoint: *mut Savepoint,
    pub nAnalysisLimit: ::core::ffi::c_int,
    pub busyTimeout: ::core::ffi::c_int,
    pub nSavepoint: ::core::ffi::c_int,
    pub nStatement: ::core::ffi::c_int,
    pub nDeferredCons: i64_0,
    pub nDeferredImmCons: i64_0,
    pub pnBytesFreed: *mut ::core::ffi::c_int,
    pub pDbData: *mut DbClientData,
    pub nSpill: u64_0

}
pub type u64_0 = sqlite_uint64;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbClientData {
    pub pNext: *mut DbClientData,
    pub pData: *mut ::core::ffi::c_void,
    pub xDestructor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub zName: [::core::ffi::c_char; 0]

}
pub type i64_0 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Savepoint {
    pub zName: *mut ::core::ffi::c_char,
    pub nDeferredCons: i64_0,
    pub nDeferredImmCons: i64_0,
    pub pNext: *mut Savepoint

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Db {
    pub zDbSName: *mut ::core::ffi::c_char,
    pub pBt: *mut Btree,
    pub safety_level: u8_0,
    pub bSyncSet: u8_0,
    pub pSchema: *mut Schema

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Schema {
    pub schema_cookie: ::core::ffi::c_int,
    pub iGeneration: ::core::ffi::c_int,
    pub tblHash: Hash,
    pub idxHash: Hash,
    pub trigHash: Hash,
    pub fkeyHash: Hash,
    pub pSeqTab: *mut Table,
    pub file_format: u8_0,
    pub enc: u8_0,
    pub schemaFlags: u16_0,
    pub cache_size: ::core::ffi::c_int

}
pub type u16_0 = uint16_t;
pub type uint16_t = __uint16_t;
pub type __uint16_t = u16;
pub type u8_0 = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub zName: *mut ::core::ffi::c_char,
    pub aCol: *mut Column,
    pub pIndex: *mut Index,
    pub zColAff: *mut ::core::ffi::c_char,
    pub pCheck: *mut ExprList,
    pub tnum: Pgno,
    pub nTabRef: u32_0,
    pub tabFlags: u32_0,
    pub iPKey: i16_0,
    pub nCol: i16_0,
    pub nNVCol: i16_0,
    pub nRowLogEst: LogEst,
    pub szTabRow: LogEst,
    pub keyConf: u8_0,
    pub eTabType: u8_0,
    pub u: C2Rust_Unnamed_18,
    pub pTrigger: *mut Trigger,
    pub pSchema: *mut Schema,
    pub aHx: [u8_0; 16]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Trigger {
    pub zName: *mut ::core::ffi::c_char,
    pub table: *mut ::core::ffi::c_char,
    pub op: u8_0,
    pub tr_tm: u8_0,
    pub bReturning: u8_0,
    pub pWhen: *mut Expr,
    pub pColumns: *mut IdList,
    pub pSchema: *mut Schema,
    pub pTabSchema: *mut Schema,
    pub step_list: *mut TriggerStep,
    pub pNext: *mut Trigger

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerStep {
    pub op: u8_0,
    pub orconf: u8_0,
    pub pTrig: *mut Trigger,
    pub pSelect: *mut Select,
    pub zTarget: *mut ::core::ffi::c_char,
    pub pFrom: *mut SrcList,
    pub pWhere: *mut Expr,
    pub pExprList: *mut ExprList,
    pub pIdList: *mut IdList,
    pub pUpsert: *mut Upsert,
    pub zSpan: *mut ::core::ffi::c_char,
    pub pNext: *mut TriggerStep,
    pub pLast: *mut TriggerStep

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upsert {
    pub pUpsertTarget: *mut ExprList,
    pub pUpsertTargetWhere: *mut Expr,
    pub pUpsertSet: *mut ExprList,
    pub pUpsertWhere: *mut Expr,
    pub pNextUpsert: *mut Upsert,
    pub isDoUpdate: u8_0,
    pub isDup: u8_0,
    pub pToFree: *mut ::core::ffi::c_void,
    pub pUpsertIdx: *mut Index,
    pub pUpsertSrc: *mut SrcList,
    pub regData: ::core::ffi::c_int,
    pub iDataCur: ::core::ffi::c_int,
    pub iIdxCur: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SrcList {
    pub nSrc: ::core::ffi::c_int,
    pub nAlloc: u32_0,
    pub a: [SrcItem; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SrcItem {
    pub zName: *mut ::core::ffi::c_char,
    pub zAlias: *mut ::core::ffi::c_char,
    pub pSTab: *mut Table,
    pub fg: C2Rust_Unnamed_17,
    pub iCursor: ::core::ffi::c_int,
    pub colUsed: Bitmask,
    pub u1: C2Rust_Unnamed_16,
    pub u2: C2Rust_Unnamed_15,
    pub u3: C2Rust_Unnamed_14,
    pub u4: C2Rust_Unnamed

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub pSchema: *mut Schema,
    pub zDatabase: *mut ::core::ffi::c_char,
    pub pSubq: *mut Subquery

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Subquery {
    pub pSelect: *mut Select,
    pub addrFillSub: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int,
    pub regResult: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Select {
    pub op: u8_0,
    pub nSelectRow: LogEst,
    pub selFlags: u32_0,
    pub iLimit: ::core::ffi::c_int,
    pub iOffset: ::core::ffi::c_int,
    pub selId: u32_0,
    pub addrOpenEphm: [::core::ffi::c_int; 2],
    pub pEList: *mut ExprList,
    pub pSrc: *mut SrcList,
    pub pWhere: *mut Expr,
    pub pGroupBy: *mut ExprList,
    pub pHaving: *mut Expr,
    pub pOrderBy: *mut ExprList,
    pub pPrior: *mut Select,
    pub pNext: *mut Select,
    pub pLimit: *mut Expr,
    pub pWith: *mut With,
    pub pWin: *mut Window,
    pub pWinDefn: *mut Window

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Window {
    pub zName: *mut ::core::ffi::c_char,
    pub zBase: *mut ::core::ffi::c_char,
    pub pPartition: *mut ExprList,
    pub pOrderBy: *mut ExprList,
    pub eFrmType: u8_0,
    pub eStart: u8_0,
    pub eEnd: u8_0,
    pub bImplicitFrame: u8_0,
    pub eExclude: u8_0,
    pub pStart: *mut Expr,
    pub pEnd: *mut Expr,
    pub ppThis: *mut *mut Window,
    pub pNextWin: *mut Window,
    pub pFilter: *mut Expr,
    pub pWFunc: *mut FuncDef,
    pub iEphCsr: ::core::ffi::c_int,
    pub regAccum: ::core::ffi::c_int,
    pub regResult: ::core::ffi::c_int,
    pub csrApp: ::core::ffi::c_int,
    pub regApp: ::core::ffi::c_int,
    pub regPart: ::core::ffi::c_int,
    pub pOwner: *mut Expr,
    pub nBufferCol: ::core::ffi::c_int,
    pub iArgCol: ::core::ffi::c_int,
    pub regOne: ::core::ffi::c_int,
    pub regStartRowid: ::core::ffi::c_int,
    pub regEndRowid: ::core::ffi::c_int,
    pub bExprArgs: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Expr {
    pub op: u8_0,
    pub affExpr: ::core::ffi::c_char,
    pub op2: u8_0,
    pub flags: u32_0,
    pub u: C2Rust_Unnamed_13,
    pub pLeft: *mut Expr,
    pub pRight: *mut Expr,
    pub x: C2Rust_Unnamed_12,
    pub nHeight: ::core::ffi::c_int,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ynVar,
    pub iAgg: i16_0,
    pub w: C2Rust_Unnamed_11,
    pub pAggInfo: *mut AggInfo,
    pub y: C2Rust_Unnamed_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub pTab: *mut Table,
    pub pWin: *mut Window,
    pub nReg: ::core::ffi::c_int,
    pub sub: C2Rust_Unnamed_1

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub iAddr: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AggInfo {
    pub directMode: u8_0,
    pub useSortingIdx: u8_0,
    pub nSortingColumn: u32_0,
    pub sortingIdx: ::core::ffi::c_int,
    pub sortingIdxPTab: ::core::ffi::c_int,
    pub iFirstReg: ::core::ffi::c_int,
    pub pGroupBy: *mut ExprList,
    pub aCol: *mut AggInfo_col,
    pub nColumn: ::core::ffi::c_int,
    pub nAccumulator: ::core::ffi::c_int,
    pub aFunc: *mut AggInfo_func,
    pub nFunc: ::core::ffi::c_int,
    pub selId: u32_0

}
pub type u32_0 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AggInfo_func {
    pub pFExpr: *mut Expr,
    pub pFunc: *mut FuncDef,
    pub iDistinct: ::core::ffi::c_int,
    pub iDistAddr: ::core::ffi::c_int,
    pub iOBTab: ::core::ffi::c_int,
    pub bOBPayload: u8_0,
    pub bOBUnique: u8_0,
    pub bUseSubtype: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncDef {
    pub nArg: i16_0,
    pub funcFlags: u32_0,
    pub pUserData: *mut ::core::ffi::c_void,
    pub pNext: *mut FuncDef,
    pub xSFunc: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    pub xFinalize: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    pub xValue: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    pub xInverse: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    pub zName: *const ::core::ffi::c_char,
    pub u: C2Rust_Unnamed_2

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub pHash: *mut FuncDef,
    pub pDestructor: *mut FuncDestructor

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncDestructor {
    pub nRef: ::core::ffi::c_int,
    pub xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pUserData: *mut ::core::ffi::c_void

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_value {
    pub u: MemValue,
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub flags: u16_0,
    pub enc: u8_0,
    pub eSubtype: u8_0,
    pub db: *mut sqlite3,
    pub szMalloc: ::core::ffi::c_int,
    pub uTemp: u32_0,
    pub zMalloc: *mut ::core::ffi::c_char,
    pub xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union MemValue {
    pub r: ::core::ffi::c_double,
    pub i: i64_0,
    pub nZero: ::core::ffi::c_int,
    pub zPType: *const ::core::ffi::c_char,
    pub pDef: *mut FuncDef

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_context {
    pub pOut: *mut Mem,
    pub pFunc: *mut FuncDef,
    pub pMem: *mut Mem,
    pub pVdbe: *mut Vdbe,
    pub iOp: ::core::ffi::c_int,
    pub isError: ::core::ffi::c_int,
    pub enc: u8_0,
    pub skipFlag: u8_0,
    pub argc: u16_0,
    pub argv: [*mut sqlite3_value; 0]

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Vdbe {
    pub db: *mut sqlite3,
    pub ppVPrev: *mut *mut Vdbe,
    pub pVNext: *mut Vdbe,
    pub pParse: *mut Parse,
    pub nVar: ynVar,
    pub nMem: ::core::ffi::c_int,
    pub nCursor: ::core::ffi::c_int,
    pub cacheCtr: u32_0,
    pub pc: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub iStatement: ::core::ffi::c_int,
    pub iCurrentTime: i64_0,
    pub nFkConstraint: i64_0,
    pub nStmtDefCons: i64_0,
    pub nStmtDefImmCons: i64_0,
    pub aMem: *mut Mem,
    pub apArg: *mut *mut Mem,
    pub apCsr: *mut *mut VdbeCursor,
    pub aVar: *mut Mem,
    pub aOp: *mut Op,
    pub nOp: ::core::ffi::c_int,
    pub nOpAlloc: ::core::ffi::c_int,
    pub aColName: *mut Mem,
    pub pResultRow: *mut Mem,
    pub zErrMsg: *mut ::core::ffi::c_char,
    pub pVList: *mut VList,
    pub startTime: i64_0,
    pub nResColumn: u16_0,
    pub nResAlloc: u16_0,
    pub errorAction: u8_0,
    pub minWriteFileFormat: u8_0,
    pub prepFlags: u8_0,
    pub eVdbeState: u8_0,
    #[bitfield(name = "expired", ty = "bft", bits = "0..=1")]
    #[bitfield(name = "explain", ty = "bft", bits = "2..=3")]
    #[bitfield(name = "changeCntOn", ty = "bft", bits = "4..=4")]
    #[bitfield(name = "usesStmtJournal", ty = "bft", bits = "5..=5")]
    #[bitfield(name = "readOnly", ty = "bft", bits = "6..=6")]
    #[bitfield(name = "bIsReader", ty = "bft", bits = "7..=7")]
    #[bitfield(name = "haveEqpOps", ty = "bft", bits = "8..=8")]
    pub expired_explain_changeCntOn_usesStmtJournal_readOnly_bIsReader_haveEqpOps: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub btreeMask: yDbMask,
    pub lockMask: yDbMask,
    pub aCounter: [u32_0; 9],
    pub zSql: *mut ::core::ffi::c_char,
    pub pFree: *mut ::core::ffi::c_void,
    pub pFrame: *mut VdbeFrame,
    pub pDelFrame: *mut VdbeFrame,
    pub nFrame: ::core::ffi::c_int,
    pub expmask: u32_0,
    pub pProgram: *mut SubProgram,
    pub pAuxData: *mut AuxData

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AuxData {
    pub iAuxOp: ::core::ffi::c_int,
    pub iAuxArg: ::core::ffi::c_int,
    pub pAux: *mut ::core::ffi::c_void,
    pub xDeleteAux: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pNextAux: *mut AuxData

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubProgram {
    pub aOp: *mut VdbeOp,
    pub nOp: ::core::ffi::c_int,
    pub nMem: ::core::ffi::c_int,
    pub nCsr: ::core::ffi::c_int,
    pub aOnce: *mut u8_0,
    pub token: *mut ::core::ffi::c_void,
    pub pNext: *mut SubProgram

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeOp {
    pub opcode: u8_0,
    pub p4type: ::core::ffi::c_schar,
    pub p5: u16_0,
    pub p1: ::core::ffi::c_int,
    pub p2: ::core::ffi::c_int,
    pub p3: ::core::ffi::c_int,
    pub p4: p4union

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union p4union {
    pub i: ::core::ffi::c_int,
    pub p: *mut ::core::ffi::c_void,
    pub z: *mut ::core::ffi::c_char,
    pub pI64: *mut i64_0,
    pub pReal: *mut ::core::ffi::c_double,
    pub pFunc: *mut FuncDef,
    pub pCtx: *mut sqlite3_context,
    pub pColl: *mut CollSeq,
    pub pMem: *mut Mem,
    pub pVtab: *mut VTable,
    pub pKeyInfo: *mut KeyInfo,
    pub ai: *mut u32_0,
    pub pProgram: *mut SubProgram,
    pub pTab: *mut Table,
    pub pSubrtnSig: *mut SubrtnSig

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubrtnSig {
    pub selId: ::core::ffi::c_int,
    pub bComplete: u8_0,
    pub zAff: *mut ::core::ffi::c_char,
    pub iTable: ::core::ffi::c_int,
    pub iAddr: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyInfo {
    pub nRef: u32_0,
    pub enc: u8_0,
    pub nKeyField: u16_0,
    pub nAllField: u16_0,
    pub db: *mut sqlite3,
    pub aSortFlags: *mut u8_0,
    pub aColl: [*mut CollSeq; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollSeq {
    pub zName: *mut ::core::ffi::c_char,
    pub enc: u8_0,
    pub pUser: *mut ::core::ffi::c_void,
    pub xCmp: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VTable {
    pub db: *mut sqlite3,
    pub pMod: *mut Module,
    pub pVtab: *mut sqlite3_vtab,
    pub nRef: ::core::ffi::c_int,
    pub bConstraint: u8_0,
    pub bAllSchemas: u8_0,
    pub eVtabRisk: u8_0,
    pub iSavepoint: ::core::ffi::c_int,
    pub pNext: *mut VTable

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab {
    pub pModule: *const sqlite3_module,
    pub nRef: ::core::ffi::c_int,
    pub zErrMsg: *mut ::core::ffi::c_char

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_module {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xConnect: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xBestIndex: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut sqlite3_index_info,
        ) -> ::core::ffi::c_int,
    >,
    pub xDisconnect: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut *mut sqlite3_vtab_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xFilter: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub xNext: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xEof: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xColumn: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_context,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRowid: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xUpdate: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xBegin: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xSync: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xCommit: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xRollback: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xFindFunction: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *mut Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xRename: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xSavepoint: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xRelease: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xRollbackTo: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xShadowName: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    pub xIntegrity: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >

}
pub type sqlite3_int64 = sqlite_int64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_info {
    pub nConstraint: ::core::ffi::c_int,
    pub aConstraint: *mut sqlite3_index_constraint,
    pub nOrderBy: ::core::ffi::c_int,
    pub aOrderBy: *mut sqlite3_index_orderby,
    pub aConstraintUsage: *mut sqlite3_index_constraint_usage,
    pub idxNum: ::core::ffi::c_int,
    pub idxStr: *mut ::core::ffi::c_char,
    pub needToFreeIdxStr: ::core::ffi::c_int,
    pub orderByConsumed: ::core::ffi::c_int,
    pub estimatedCost: ::core::ffi::c_double,
    pub estimatedRows: sqlite3_int64,
    pub idxFlags: ::core::ffi::c_int,
    pub colUsed: sqlite3_uint64

}
pub type sqlite3_uint64 = sqlite_uint64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint_usage {
    pub argvIndex: ::core::ffi::c_int,
    pub omit: ::core::ffi::c_uchar

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_orderby {
    pub iColumn: ::core::ffi::c_int,
    pub desc: ::core::ffi::c_uchar

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint {
    pub iColumn: ::core::ffi::c_int,
    pub op: ::core::ffi::c_uchar,
    pub usable: ::core::ffi::c_uchar,
    pub iTermOffset: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Module {
    pub pModule: *const sqlite3_module,
    pub zName: *const ::core::ffi::c_char,
    pub nRefModule: ::core::ffi::c_int,
    pub pAux: *mut ::core::ffi::c_void,
    pub xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pEpoTab: *mut Table

}
pub type Mem = sqlite3_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeFrame {
    pub v: *mut Vdbe,
    pub pParent: *mut VdbeFrame,
    pub aOp: *mut Op,
    pub aMem: *mut Mem,
    pub apCsr: *mut *mut VdbeCursor,
    pub aOnce: *mut u8_0,
    pub token: *mut ::core::ffi::c_void,
    pub lastRowid: i64_0,
    pub pAuxData: *mut AuxData,
    pub nCursor: ::core::ffi::c_int,
    pub pc: ::core::ffi::c_int,
    pub nOp: ::core::ffi::c_int,
    pub nMem: ::core::ffi::c_int,
    pub nChildMem: ::core::ffi::c_int,
    pub nChildCsr: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub nDbChange: i64_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VdbeCursor {
    pub eCurType: u8_0,
    pub iDb: i8_0,
    pub nullRow: u8_0,
    pub deferredMoveto: u8_0,
    pub isTable: u8_0,
    #[bitfield(name = "isEphemeral", ty = "Bool", bits = "0..=0")]
    #[bitfield(name = "useRandomRowid", ty = "Bool", bits = "1..=1")]
    #[bitfield(name = "isOrdered", ty = "Bool", bits = "2..=2")]
    #[bitfield(name = "noReuse", ty = "Bool", bits = "3..=3")]
    #[bitfield(name = "colCache", ty = "Bool", bits = "4..=4")]
    pub isEphemeral_useRandomRowid_isOrdered_noReuse_colCache: [u8; 1],
    pub seekHit: u16_0,
    pub ub: C2Rust_Unnamed_4,
    pub seqCount: i64_0,
    pub cacheStatus: u32_0,
    pub seekResult: ::core::ffi::c_int,
    pub pAltCursor: *mut VdbeCursor,
    pub uc: C2Rust_Unnamed_3,
    pub pKeyInfo: *mut KeyInfo,
    pub iHdrOffset: u32_0,
    pub pgnoRoot: Pgno,
    pub nField: i16_0,
    pub nHdrParsed: u16_0,
    pub movetoTarget: i64_0,
    pub aOffset: *mut u32_0,
    pub aRow: *const u8_0,
    pub payloadSize: u32_0,
    pub szRow: u32_0,
    pub pCache: *mut VdbeTxtBlbCache,
    pub aType: [u32_0; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeTxtBlbCache {
    pub pCValue: *mut ::core::ffi::c_char,
    pub iOffset: i64_0,
    pub iCol: ::core::ffi::c_int,
    pub cacheStatus: u32_0,
    pub colCacheCtr: u32_0

}
pub type i16_0 = int16_t;
pub type int16_t = __int16_t;
pub type __int16_t = i16;
pub type Pgno = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub pCursor: *mut BtCursor,
    pub pVCur: *mut sqlite3_vtab_cursor,
    pub pSorter: *mut VdbeSorter

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub pBtx: *mut Btree,
    pub aAltMap: *mut u32_0

}
pub type Bool = ::core::ffi::c_uint;
pub type i8_0 = int8_t;
pub type int8_t = __int8_t;
pub type __int8_t = i8;
pub type Op = VdbeOp;
pub type yDbMask = ::core::ffi::c_uint;
pub type bft = ::core::ffi::c_uint;
pub type VList = ::core::ffi::c_int;
pub type ynVar = i16_0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Parse {
    pub db: *mut sqlite3,
    pub zErrMsg: *mut ::core::ffi::c_char,
    pub pVdbe: *mut Vdbe,
    pub rc: ::core::ffi::c_int,
    pub nQueryLoop: LogEst,
    pub nested: u8_0,
    pub nTempReg: u8_0,
    pub isMultiWrite: u8_0,
    pub mayAbort: u8_0,
    pub hasCompound: u8_0,
    pub disableLookaside: u8_0,
    pub prepFlags: u8_0,
    pub withinRJSubrtn: u8_0,
    pub bHasExists: u8_0,
    pub mSubrtnSig: u8_0,
    pub eTriggerOp: u8_0,
    pub bReturning: u8_0,
    pub eOrconf: u8_0,
    pub disableTriggers: u8_0,
    #[bitfield(name = "colNamesSet", ty = "bft", bits = "0..=0")]
    #[bitfield(name = "bHasWith", ty = "bft", bits = "1..=1")]
    #[bitfield(name = "okConstFactor", ty = "bft", bits = "2..=2")]
    #[bitfield(name = "checkSchema", ty = "bft", bits = "3..=3")]
    pub colNamesSet_bHasWith_okConstFactor_checkSchema: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub nRangeReg: ::core::ffi::c_int,
    pub iRangeReg: ::core::ffi::c_int,
    pub nErr: ::core::ffi::c_int,
    pub nTab: ::core::ffi::c_int,
    pub nMem: ::core::ffi::c_int,
    pub szOpAlloc: ::core::ffi::c_int,
    pub iSelfTab: ::core::ffi::c_int,
    pub nLabel: ::core::ffi::c_int,
    pub nLabelAlloc: ::core::ffi::c_int,
    pub aLabel: *mut ::core::ffi::c_int,
    pub pConstExpr: *mut ExprList,
    pub pIdxEpr: *mut IndexedExpr,
    pub pIdxPartExpr: *mut IndexedExpr,
    pub writeMask: yDbMask,
    pub cookieMask: yDbMask,
    pub nMaxArg: ::core::ffi::c_int,
    pub nSelect: ::core::ffi::c_int,
    pub nProgressSteps: u32_0,
    pub nTableLock: ::core::ffi::c_int,
    pub aTableLock: *mut TableLock,
    pub pAinc: *mut AutoincInfo,
    pub pToplevel: *mut Parse,
    pub pTriggerTab: *mut Table,
    pub pTriggerPrg: *mut TriggerPrg,
    pub pCleanup: *mut ParseCleanup,
    pub aTempReg: [::core::ffi::c_int; 8],
    pub pOuterParse: *mut Parse,
    pub sNameToken: Token,
    pub oldmask: u32_0,
    pub newmask: u32_0,
    pub u1: C2Rust_Unnamed_8,
    pub sLastToken: Token,
    pub nVar: ynVar,
    pub iPkSortOrder: u8_0,
    pub explain: u8_0,
    pub eParseMode: u8_0,
    pub nVtabLock: ::core::ffi::c_int,
    pub nHeight: ::core::ffi::c_int,
    pub addrExplain: ::core::ffi::c_int,
    pub pVList: *mut VList,
    pub pReprepare: *mut Vdbe,
    pub zTail: *const ::core::ffi::c_char,
    pub pNewTable: *mut Table,
    pub pNewIndex: *mut Index,
    pub pNewTrigger: *mut Trigger,
    pub zAuthContext: *const ::core::ffi::c_char,
    pub sArg: Token,
    pub apVtabLock: *mut *mut Table,
    pub pWith: *mut With,
    pub pRename: *mut RenameToken

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct With {
    pub nCte: ::core::ffi::c_int,
    pub bView: ::core::ffi::c_int,
    pub pOuter: *mut With,
    pub a: [Cte; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cte {
    pub zName: *mut ::core::ffi::c_char,
    pub pCols: *mut ExprList,
    pub pSelect: *mut Select,
    pub zCteErr: *const ::core::ffi::c_char,
    pub pUse: *mut CteUse,
    pub eM10d: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CteUse {
    pub nUse: ::core::ffi::c_int,
    pub addrM9e: ::core::ffi::c_int,
    pub regRtn: ::core::ffi::c_int,
    pub iCur: ::core::ffi::c_int,
    pub nRowEst: LogEst,
    pub eM10d: u8_0

}
pub type LogEst = int16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprList {
    pub nExpr: ::core::ffi::c_int,
    pub nAlloc: ::core::ffi::c_int,
    pub a: [ExprList_item; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprList_item {
    pub pExpr: *mut Expr,
    pub zEName: *mut ::core::ffi::c_char,
    pub fg: C2Rust_Unnamed_7,
    pub u: C2Rust_Unnamed_5

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub x: C2Rust_Unnamed_6,
    pub iConstExprReg: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub iOrderByCol: u16_0,
    pub iAlias: u16_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub sortFlags: u8_0,
    #[bitfield(name = "eEName", ty = "::core::ffi::c_uint", bits = "0..=1")]
    #[bitfield(name = "done", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "reusable", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "bSorterRef", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "bNulls", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "bUsed", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "bUsingTerm", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "bNoExpand", ty = "::core::ffi::c_uint", bits = "8..=8")]
    pub eEName_done_reusable_bSorterRef_bNulls_bUsed_bUsingTerm_bNoExpand: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub z: *const ::core::ffi::c_char,
    pub n: ::core::ffi::c_uint

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Index {
    pub zName: *mut ::core::ffi::c_char,
    pub aiColumn: *mut i16_0,
    pub aiRowLogEst: *mut LogEst,
    pub pTable: *mut Table,
    pub zColAff: *mut ::core::ffi::c_char,
    pub pNext: *mut Index,
    pub pSchema: *mut Schema,
    pub aSortOrder: *mut u8_0,
    pub azColl: *mut *const ::core::ffi::c_char,
    pub pPartIdxWhere: *mut Expr,
    pub aColExpr: *mut ExprList,
    pub tnum: Pgno,
    pub szIdxRow: LogEst,
    pub nKeyCol: u16_0,
    pub nColumn: u16_0,
    pub onError: u8_0,
    #[bitfield(name = "idxType", ty = "::core::ffi::c_uint", bits = "0..=1")]
    #[bitfield(name = "bUnordered", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "uniqNotNull", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "isResized", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "isCovering", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "noSkipScan", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "hasStat1", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "bNoQuery", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "bAscKeyBug", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "bHasVCol", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(name = "bHasExpr", ty = "::core::ffi::c_uint", bits = "11..=11")]
    pub idxType_bUnordered_uniqNotNull_isResized_isCovering_noSkipScan_hasStat1_bNoQuery_bAscKeyBug_bHasVCol_bHasExpr: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub colNotIdxed: Bitmask

}
pub type Bitmask = u64_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub cr: C2Rust_Unnamed_10,
    pub d: C2Rust_Unnamed_9

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub pReturning: *mut Returning

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Returning {
    pub pParse: *mut Parse,
    pub pReturnEL: *mut ExprList,
    pub retTrig: Trigger,
    pub retTStep: TriggerStep,
    pub iRetCur: ::core::ffi::c_int,
    pub nRetCol: ::core::ffi::c_int,
    pub iRetReg: ::core::ffi::c_int,
    pub zName: [::core::ffi::c_char; 40]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub addrCrTab: ::core::ffi::c_int,
    pub regRowid: ::core::ffi::c_int,
    pub regRoot: ::core::ffi::c_int,
    pub constraintName: Token

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseCleanup {
    pub pNext: *mut ParseCleanup,
    pub pPtr: *mut ::core::ffi::c_void,
    pub xCleanup: Option<
        unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> (),
    >

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerPrg {
    pub pTrigger: *mut Trigger,
    pub pNext: *mut TriggerPrg,
    pub pProgram: *mut SubProgram,
    pub orconf: ::core::ffi::c_int,
    pub aColmask: [u32_0; 2]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AutoincInfo {
    pub pNext: *mut AutoincInfo,
    pub pTab: *mut Table,
    pub iDb: ::core::ffi::c_int,
    pub regCtr: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexedExpr {
    pub pExpr: *mut Expr,
    pub iDataCur: ::core::ffi::c_int,
    pub iIdxCur: ::core::ffi::c_int,
    pub iIdxCol: ::core::ffi::c_int,
    pub bMaybeNullRow: u8_0,
    pub aff: u8_0,
    pub pIENext: *mut IndexedExpr

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AggInfo_col {
    pub pTab: *mut Table,
    pub pCExpr: *mut Expr,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ::core::ffi::c_int,
    pub iSorterColumn: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub iJoin: ::core::ffi::c_int,
    pub iOfst: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub pList: *mut ExprList,
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_13 {
    pub zToken: *mut ::core::ffi::c_char,
    pub iValue: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_14 {
    pub pOn: *mut Expr,
    pub pUsing: *mut IdList

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdList {
    pub nId: ::core::ffi::c_int,
    pub a: [IdList_item; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdList_item {
    pub zName: *mut ::core::ffi::c_char

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_15 {
    pub pIBIndex: *mut Index,
    pub pCteUse: *mut CteUse

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_16 {
    pub zIndexedBy: *mut ::core::ffi::c_char,
    pub pFuncArg: *mut ExprList,
    pub nRow: u32_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub jointype: u8_0,
    #[bitfield(name = "notIndexed", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "isIndexedBy", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "isSubquery", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "isTabFunc", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "isCorrelated", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "isMaterialized", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "viaCoroutine", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "isRecursive", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "fromDDL", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "isCte", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "notCte", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(name = "isUsing", ty = "::core::ffi::c_uint", bits = "11..=11")]
    #[bitfield(name = "isOn", ty = "::core::ffi::c_uint", bits = "12..=12")]
    #[bitfield(name = "isSynthUsing", ty = "::core::ffi::c_uint", bits = "13..=13")]
    #[bitfield(name = "isNestedFrom", ty = "::core::ffi::c_uint", bits = "14..=14")]
    #[bitfield(name = "rowidUsed", ty = "::core::ffi::c_uint", bits = "15..=15")]
    #[bitfield(name = "fixedSchema", ty = "::core::ffi::c_uint", bits = "16..=16")]
    #[bitfield(name = "hadSchema", ty = "::core::ffi::c_uint", bits = "17..=17")]
    #[bitfield(name = "fromExists", ty = "::core::ffi::c_uint", bits = "18..=18")]
    pub notIndexed_isIndexedBy_isSubquery_isTabFunc_isCorrelated_isMaterialized_viaCoroutine_isRecursive_fromDDL_isCte_notCte_isUsing_isOn_isSynthUsing_isNestedFrom_rowidUsed_fixedSchema_hadSchema_fromExists: [u8; 3]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_18 {
    pub tab: C2Rust_Unnamed_21,
    pub view: C2Rust_Unnamed_20,
    pub vtab: C2Rust_Unnamed_19

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub nArg: ::core::ffi::c_int,
    pub azArg: *mut *mut ::core::ffi::c_char,
    pub p: *mut VTable

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub addColOffset: ::core::ffi::c_int,
    pub pFKey: *mut FKey,
    pub pDfltList: *mut ExprList

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FKey {
    pub pFrom: *mut Table,
    pub pNextFrom: *mut FKey,
    pub zTo: *mut ::core::ffi::c_char,
    pub pNextTo: *mut FKey,
    pub pPrevTo: *mut FKey,
    pub nCol: ::core::ffi::c_int,
    pub isDeferred: u8_0,
    pub aAction: [u8_0; 2],
    pub apTrigger: [*mut Trigger; 2],
    pub aCol: [sColMap; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sColMap {
    pub iFrom: ::core::ffi::c_int,
    pub zCol: *mut ::core::ffi::c_char

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Column {
    pub zCnName: *mut ::core::ffi::c_char,
    #[bitfield(name = "notNull", ty = "::core::ffi::c_uint", bits = "0..=3")]
    #[bitfield(name = "eCType", ty = "::core::ffi::c_uint", bits = "4..=7")]
    pub notNull_eCType: [u8; 1],
    pub affinity: ::core::ffi::c_char,
    pub szEst: u8_0,
    pub hName: u8_0,
    pub iDflt: u16_0,
    pub colFlags: u16_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hash {
    pub htsize: ::core::ffi::c_uint,
    pub count: ::core::ffi::c_uint,
    pub first: *mut HashElem,
    pub ht: *mut _ht

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ht {
    pub count: ::core::ffi::c_uint,
    pub chain: *mut HashElem

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashElem {
    pub next: *mut HashElem,
    pub prev: *mut HashElem,
    pub data: *mut ::core::ffi::c_void,
    pub pKey: *const ::core::ffi::c_char,
    pub h: ::core::ffi::c_uint

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BusyHandler {
    pub xBusyHandler: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pBusyArg: *mut ::core::ffi::c_void,
    pub nBusy: ::core::ffi::c_int

}
pub type sqlite3_xauth = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lookaside {
    pub bDisable: u32_0,
    pub sz: u16_0,
    pub szTrue: u16_0,
    pub bMalloced: u8_0,
    pub nSlot: u32_0,
    pub anStat: [u32_0; 3],
    pub pInit: *mut LookasideSlot,
    pub pFree: *mut LookasideSlot,
    pub pSmallInit: *mut LookasideSlot,
    pub pSmallFree: *mut LookasideSlot,
    pub pMiddle: *mut ::core::ffi::c_void,
    pub pStart: *mut ::core::ffi::c_void,
    pub pEnd: *mut ::core::ffi::c_void,
    pub pTrueEnd: *mut ::core::ffi::c_void

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookasideSlot {
    pub pNext: *mut LookasideSlot

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_22 {
    pub isInterrupted: ::core::ffi::c_int,
    pub notUsed1: ::core::ffi::c_double

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreUpdate {
    pub v: *mut Vdbe,
    pub pCsr: *mut VdbeCursor,
    pub op: ::core::ffi::c_int,
    pub aRecord: *mut u8_0,
    pub pKeyinfo: *mut KeyInfo,
    pub pUnpacked: *mut UnpackedRecord,
    pub pNewUnpacked: *mut UnpackedRecord,
    pub iNewReg: ::core::ffi::c_int,
    pub iBlobWrite: ::core::ffi::c_int,
    pub iKey1: i64_0,
    pub iKey2: i64_0,
    pub oldipk: Mem,
    pub aNew: *mut Mem,
    pub pTab: *mut Table,
    pub pPk: *mut Index,
    pub apDflt: *mut *mut sqlite3_value,
    pub uKey: C2Rust_Unnamed_23

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub keyinfoSpace: [u8_0; 32]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnpackedRecord {
    pub pKeyInfo: *mut KeyInfo,
    pub aMem: *mut Mem,
    pub u: C2Rust_Unnamed_24,
    pub n: ::core::ffi::c_int,
    pub nField: u16_0,
    pub default_rc: i8_0,
    pub errCode: u8_0,
    pub r1: i8_0,
    pub r2: i8_0,
    pub eqSeen: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_24 {
    pub z: *mut ::core::ffi::c_char,
    pub i: i64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_25 {
    pub xLegacy: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> (),
    >,
    pub xV2: Option<
        unsafe extern "C" fn(
            u32_0,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct sqlite3InitInfo {
    pub newTnum: Pgno,
    pub iDb: u8_0,
    pub busy: u8_0,
    #[bitfield(name = "orphanTrigger", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "imposterTable", ty = "::core::ffi::c_uint", bits = "1..=2")]
    #[bitfield(name = "reopenMemdb", ty = "::core::ffi::c_uint", bits = "3..=3")]
    pub orphanTrigger_imposterTable_reopenMemdb: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub azInit: *mut *const ::core::ffi::c_char

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vfs {
    pub iVersion: ::core::ffi::c_int,
    pub szOsFile: ::core::ffi::c_int,
    pub mxPathname: ::core::ffi::c_int,
    pub pNext: *mut sqlite3_vfs,
    pub zName: *const ::core::ffi::c_char,
    pub pAppData: *mut ::core::ffi::c_void,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            sqlite3_filename,
            *mut sqlite3_file,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xDelete: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xAccess: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xFullPathname: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xDlOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub xDlError: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> (),
    >,
    pub xDlSym: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> Option<unsafe extern "C" fn() -> ()>,
    >,
    pub xDlClose: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
    >,
    pub xRandomness: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xSleep: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xCurrentTime: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *mut ::core::ffi::c_double,
        ) -> ::core::ffi::c_int,
    >,
    pub xGetLastError: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xCurrentTimeInt64: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs, *mut sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xSetSystemCall: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            sqlite3_syscall_ptr,
        ) -> ::core::ffi::c_int,
    >,
    pub xGetSystemCall: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
        ) -> sqlite3_syscall_ptr,
    >,
    pub xNextSystemCall: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
        ) -> *const ::core::ffi::c_char,
    >

}
pub type sqlite3_syscall_ptr = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_file {
    pub pMethods: *const sqlite3_io_methods

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_io_methods {
    pub iVersion: ::core::ffi::c_int,
    pub xClose: Option<unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int>,
    pub xRead: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xWrite: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xTruncate: Option<
        unsafe extern "C" fn(*mut sqlite3_file, sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xSync: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xFileSize: Option<
        unsafe extern "C" fn(*mut sqlite3_file, *mut sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xLock: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xUnlock: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xCheckReservedLock: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xFileControl: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xSectorSize: Option<
        unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
    >,
    pub xDeviceCharacteristics: Option<
        unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
    >,
    pub xShmMap: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xShmLock: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xShmBarrier: Option<unsafe extern "C" fn(*mut sqlite3_file) -> ()>,
    pub xShmUnmap: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xFetch: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            sqlite3_int64,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xUnfetch: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            sqlite3_int64,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >

}
pub type sqlite3_filename = *const ::core::ffi::c_char;
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_str {
    pub db: *mut sqlite3,
    pub zText: *mut ::core::ffi::c_char,
    pub nAlloc: u32_0,
    pub mxAlloc: u32_0,
    pub nChar: u32_0,
    pub accError: u8_0,
    pub printfFlags: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compareInfo {
    pub matchAll: u8_0,
    pub matchOne: u8_0,
    pub matchSet: u8_0,
    pub noCase: u8_0

}
pub type intptr_t = isize;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PrintfArguments {
    pub nArg: ::core::ffi::c_int,
    pub nUsed: ::core::ffi::c_int,
    pub apArg: *mut *mut sqlite3_value

}
pub type StrAccum = sqlite3_str;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Percentile {
    pub nAlloc: u64_0,
    pub nUsed: u64_0,
    pub bSorted: ::core::ffi::c_char,
    pub bKeepSorted: ::core::ffi::c_char,
    pub bPctValid: ::core::ffi::c_char,
    pub rPct: ::core::ffi::c_double,
    pub a: *mut ::core::ffi::c_double

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GroupConcatCtx {
    pub str: StrAccum,
    pub nAccum: ::core::ffi::c_int,
    pub nFirstSepLength: ::core::ffi::c_int,
    pub pnSepLengths: *mut ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CountCtx {
    pub n: i64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SumCtx {
    pub rSum: ::core::ffi::c_double,
    pub rErr: ::core::ffi::c_double,
    pub iSum: i64_0,
    pub cnt: i64_0,
    pub approx: u8_0,
    pub ovrfl: u8_0

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LENGTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LIKE_PATTERN_LENGTH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DIRECTONLY: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const LARGEST_INT64: i64_0 = 0xffffffff as i64_0
    | (0x7fffffff as ::core::ffi::c_int as i64_0) << 32 as ::core::ffi::c_int;
pub const SMALLEST_INT64: i64_0 = -1 as ::core::ffi::c_int as i64_0 - LARGEST_INT64;
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846f64;
pub const SQLITE_LoadExtFunc: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_LIKE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_FUNC_CASE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_FUNC_NEEDCOLL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_FUNC_CONSTANT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_FUNC_SLOCHNG: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_TEST: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_INTERNAL: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_UNSAFE: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_INLINE: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_BUILTIN: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
pub const SQLITE_PRINTF_SQLFUNC: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const TK_STRING: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
unsafe extern "C" fn sqlite3GetFuncCollSeq(
    mut context: *mut sqlite3_context,
) -> *mut CollSeq {
    unsafe {
        let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
        pOp = (*(*context).pVdbe)
            .aOp
            .offset(((*context).iOp - 1 as ::core::ffi::c_int) as isize) as *mut Op
            as *mut VdbeOp;
        return (*pOp).p4.pColl;
    }
}
unsafe extern "C" fn sqlite3SkipAccumulatorLoad(mut context: *mut sqlite3_context) {
    unsafe {
        (*context).isError = -1 as ::core::ffi::c_int;
        (*context).skipFlag = 1 as u8_0;
    }
}
unsafe extern "C" fn minmaxFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut mask: ::core::ffi::c_int = 0;
        let mut iBest: ::core::ffi::c_int = 0;
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        mask = if sqlite3_user_data(context).is_null() {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
        pColl = sqlite3GetFuncCollSeq(context);
        iBest = 0 as ::core::ffi::c_int;
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            == SQLITE_NULL
        {
            return;
        }
        i = 1 as ::core::ffi::c_int;
        while i < argc {
            if sqlite3_value_type(*argv.offset(i as isize)) == SQLITE_NULL {
                return;
            }
            if sqlite3MemCompare(
                *argv.offset(iBest as isize),
                *argv.offset(i as isize),
                pColl,
            ) ^ mask >= 0 as ::core::ffi::c_int
            {
                iBest = i;
            }
            i += 1;
        }
        sqlite3_result_value(context, *argv.offset(iBest as isize));
    }
}
unsafe extern "C" fn typeofFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        static mut azType: [*const ::core::ffi::c_char; 5] = [
            b"integer\0".as_ptr() as *const ::core::ffi::c_char,
            b"real\0".as_ptr() as *const ::core::ffi::c_char,
            b"text\0".as_ptr() as *const ::core::ffi::c_char,
            b"blob\0".as_ptr() as *const ::core::ffi::c_char,
            b"null\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = sqlite3_value_type(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) - 1 as ::core::ffi::c_int;
        sqlite3_result_text(
            context,
            azType[i as usize],
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
}
unsafe extern "C" fn subtypeFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_result_int(
            context,
            sqlite3_value_subtype(*argv.offset(0 as ::core::ffi::c_int as isize))
                as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn lengthFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        match sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize)) {
            SQLITE_BLOB | SQLITE_INTEGER | SQLITE_FLOAT => {
                sqlite3_result_int(
                    context,
                    sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize)),
                );
            }
            SQLITE_TEXT => {
                let mut z: *const ::core::ffi::c_uchar = sqlite3_value_text(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
                let mut z0: *const ::core::ffi::c_uchar = ::core::ptr::null::<
                    ::core::ffi::c_uchar,
                >();
                let mut c: ::core::ffi::c_uchar = 0;
                if z.is_null() {
                    return;
                }
                z0 = z;
                loop {
                    c = *z;
                    if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                        break;
                    }
                    z = z.offset(1);
                    if c as ::core::ffi::c_int >= 0xc0 as ::core::ffi::c_int {
                        while *z as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                            == 0x80 as ::core::ffi::c_int
                        {
                            z = z.offset(1);
                            z0 = z0.offset(1);
                        }
                    }
                }
                sqlite3_result_int(
                    context,
                    z.offset_from(z0) as ::core::ffi::c_long as ::core::ffi::c_int,
                );
            }
            _ => {
                sqlite3_result_null(context);
            }
        };
    }
}
unsafe extern "C" fn bytelengthFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        match sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize)) {
            SQLITE_BLOB => {
                sqlite3_result_int(
                    context,
                    sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize)),
                );
            }
            SQLITE_INTEGER | SQLITE_FLOAT => {
                let mut m: i64_0 = (if (*sqlite3_context_db_handle(context)).enc
                    as ::core::ffi::c_int <= SQLITE_UTF8
                {
                    1 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }) as i64_0;
                sqlite3_result_int64(
                    context,
                    sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize))
                        as sqlite3_int64 * m as sqlite3_int64,
                );
            }
            SQLITE_TEXT => {
                if sqlite3_value_encoding(*argv.offset(0 as ::core::ffi::c_int as isize))
                    <= SQLITE_UTF8
                {
                    sqlite3_result_int(
                        context,
                        sqlite3_value_bytes(
                            *argv.offset(0 as ::core::ffi::c_int as isize),
                        ),
                    );
                } else {
                    sqlite3_result_int(
                        context,
                        sqlite3_value_bytes16(
                            *argv.offset(0 as ::core::ffi::c_int as isize),
                        ),
                    );
                }
            }
            _ => {
                sqlite3_result_null(context);
            }
        };
    }
}
unsafe extern "C" fn absFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        match sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize)) {
            SQLITE_INTEGER => {
                let mut iVal: i64_0 = sqlite3_value_int64(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                ) as i64_0;
                if iVal < 0 as ::core::ffi::c_longlong {
                    if iVal == SMALLEST_INT64 {
                        sqlite3_result_error(
                            context,
                            b"integer overflow\0".as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                        );
                        return;
                    }
                    iVal = -iVal;
                }
                sqlite3_result_int64(context, iVal as sqlite3_int64);
            }
            SQLITE_NULL => {
                sqlite3_result_null(context);
            }
            _ => {
                let mut rVal: ::core::ffi::c_double = sqlite3_value_double(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
                if rVal < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    rVal = -rVal;
                }
                sqlite3_result_double(context, rVal);
            }
        };
    }
}
unsafe extern "C" fn instrFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut zHaystack: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut zNeedle: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut nHaystack: ::core::ffi::c_int = 0;
        let mut nNeedle: ::core::ffi::c_int = 0;
        let mut typeHaystack: ::core::ffi::c_int = 0;
        let mut typeNeedle: ::core::ffi::c_int = 0;
        let mut N: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut isText: ::core::ffi::c_int = 0;
        let mut firstChar: ::core::ffi::c_uchar = 0;
        let mut pC1: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut pC2: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        typeHaystack = sqlite3_value_type(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        typeNeedle = sqlite3_value_type(*argv.offset(1 as ::core::ffi::c_int as isize));
        if typeHaystack == SQLITE_NULL || typeNeedle == SQLITE_NULL {
            return;
        }
        nHaystack = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        nNeedle = sqlite3_value_bytes(*argv.offset(1 as ::core::ffi::c_int as isize));
        if nNeedle > 0 as ::core::ffi::c_int {
            if typeHaystack == SQLITE_BLOB && typeNeedle == SQLITE_BLOB {
                zHaystack = sqlite3_value_blob(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                ) as *const ::core::ffi::c_uchar;
                zNeedle = sqlite3_value_blob(
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                ) as *const ::core::ffi::c_uchar;
                isText = 0 as ::core::ffi::c_int;
                c2rust_current_block = 1109700713171191020;
            } else if typeHaystack != SQLITE_BLOB && typeNeedle != SQLITE_BLOB {
                zHaystack = sqlite3_value_text(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
                zNeedle = sqlite3_value_text(
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                );
                isText = 1 as ::core::ffi::c_int;
                c2rust_current_block = 1109700713171191020;
            } else {
                pC1 = sqlite3_value_dup(*argv.offset(0 as ::core::ffi::c_int as isize));
                zHaystack = sqlite3_value_text(pC1);
                if zHaystack.is_null() {
                    c2rust_current_block = 15652201585744561833;
                } else {
                    nHaystack = sqlite3_value_bytes(pC1);
                    pC2 = sqlite3_value_dup(
                        *argv.offset(1 as ::core::ffi::c_int as isize),
                    );
                    zNeedle = sqlite3_value_text(pC2);
                    if zNeedle.is_null() {
                        c2rust_current_block = 15652201585744561833;
                    } else {
                        nNeedle = sqlite3_value_bytes(pC2);
                        isText = 1 as ::core::ffi::c_int;
                        c2rust_current_block = 1109700713171191020;
                    }
                }
            }
            match c2rust_current_block {
                1109700713171191020 => {
                    if zNeedle.is_null() || nHaystack != 0 && zHaystack.is_null() {
                        c2rust_current_block = 15652201585744561833;
                    } else {
                        firstChar = *zNeedle.offset(0 as ::core::ffi::c_int as isize);
                        while nNeedle <= nHaystack
                            && (*zHaystack.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int != firstChar as ::core::ffi::c_int
                                || memcmp(
                                    zHaystack as *const ::core::ffi::c_void,
                                    zNeedle as *const ::core::ffi::c_void,
                                    nNeedle as size_t,
                                ) != 0 as ::core::ffi::c_int)
                        {
                            N += 1;
                            loop {
                                nHaystack -= 1;
                                zHaystack = zHaystack.offset(1);
                                if !(isText != 0
                                    && *zHaystack.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                                        == 0x80 as ::core::ffi::c_int)
                                {
                                    break;
                                }
                            }
                        }
                        if nNeedle > nHaystack {
                            N = 0 as ::core::ffi::c_int;
                        }
                        c2rust_current_block = 18377268871191777778;
                    }
                }
                _ => {}
            }
            match c2rust_current_block {
                18377268871191777778 => {}
                _ => {
                    sqlite3_result_error_nomem(context);
                    c2rust_current_block = 8390614848291103524;
                }
            }
        } else {
            c2rust_current_block = 18377268871191777778;
        }
        match c2rust_current_block {
            18377268871191777778 => {
                sqlite3_result_int(context, N);
            }
            _ => {}
        }
        sqlite3_value_free(pC1);
        sqlite3_value_free(pC2);
    }
}
unsafe extern "C" fn printfFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut x: PrintfArguments = PrintfArguments {
            nArg: 0,
            nUsed: 0,
            apArg: ::core::ptr::null_mut::<*mut sqlite3_value>()

        };
        let mut str: StrAccum = sqlite3_str {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        let mut zFormat: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut n: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        if argc >= 1 as ::core::ffi::c_int
            && {
                zFormat = sqlite3_value_text(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                ) as *const ::core::ffi::c_char;
                !zFormat.is_null()
            }
        {
            x.nArg = argc - 1 as ::core::ffi::c_int;
            x.nUsed = 0 as ::core::ffi::c_int;
            x.apArg = argv.offset(1 as ::core::ffi::c_int as isize);
            sqlite3StrAccumInit(
                &raw mut str,
                db,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
                (*db).aLimit[SQLITE_LIMIT_LENGTH as usize],
            );
            str.printfFlags = SQLITE_PRINTF_SQLFUNC as u8_0;
            sqlite3_str_appendf(&raw mut str, zFormat, &raw mut x);
            n = str.nChar as ::core::ffi::c_int;
            sqlite3_result_text(
                context,
                sqlite3StrAccumFinish(&raw mut str),
                n,
                Some(
                    sqlite3RowSetClear
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        }
    }
}
unsafe extern "C" fn substrFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut z: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut z2: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut len: ::core::ffi::c_int = 0;
        let mut p0type: ::core::ffi::c_int = 0;
        let mut p1: i64_0 = 0;
        let mut p2: i64_0 = 0;
        p0type = sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize));
        p1 = sqlite3_value_int64(*argv.offset(1 as ::core::ffi::c_int as isize))
            as i64_0;
        if p0type == SQLITE_BLOB {
            len = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
            z = sqlite3_value_blob(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_uchar;
            if z.is_null() {
                return;
            }
        } else {
            z = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
            if z.is_null() {
                return;
            }
            len = 0 as ::core::ffi::c_int;
            if p1 < 0 as ::core::ffi::c_longlong {
                z2 = z;
                while *z2 != 0 {
                    let c2rust_fresh9 = z2;
                    z2 = z2.offset(1);
                    if *c2rust_fresh9 as ::core::ffi::c_int >= 0xc0 as ::core::ffi::c_int
                    {
                        while *z2 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                            == 0x80 as ::core::ffi::c_int
                        {
                            z2 = z2.offset(1);
                        }
                    }
                    len += 1;
                }
            }
        }
        if argc == 3 as ::core::ffi::c_int {
            p2 = sqlite3_value_int64(*argv.offset(2 as ::core::ffi::c_int as isize))
                as i64_0;
            if p2 == 0 as ::core::ffi::c_longlong
                && sqlite3_value_type(*argv.offset(2 as ::core::ffi::c_int as isize))
                    == SQLITE_NULL
            {
                return;
            }
        } else {
            p2 = (*sqlite3_context_db_handle(context))
                .aLimit[SQLITE_LIMIT_LENGTH as usize] as i64_0;
        }
        if p1 == 0 as ::core::ffi::c_longlong {
            if sqlite3_value_type(*argv.offset(1 as ::core::ffi::c_int as isize))
                == SQLITE_NULL
            {
                return;
            }
        }
        if p1 < 0 as ::core::ffi::c_longlong {
            p1 += len as ::core::ffi::c_longlong;
            if p1 < 0 as ::core::ffi::c_longlong {
                if p2 < 0 as ::core::ffi::c_longlong {
                    p2 = 0 as i64_0;
                } else {
                    p2 += p1 as ::core::ffi::c_longlong;
                }
                p1 = 0 as i64_0;
            }
        } else if p1 > 0 as ::core::ffi::c_longlong {
            p1 -= 1;
        } else if p2 > 0 as ::core::ffi::c_longlong {
            p2 -= 1;
        }
        if p2 < 0 as ::core::ffi::c_longlong {
            if p2 < -p1 {
                p2 = p1;
            } else {
                p2 = -p2;
            }
            p1 -= p2 as ::core::ffi::c_longlong;
        }
        if p0type != SQLITE_BLOB {
            while *z as ::core::ffi::c_int != 0 && p1 != 0 {
                let c2rust_fresh10 = z;
                z = z.offset(1);
                if *c2rust_fresh10 as ::core::ffi::c_int >= 0xc0 as ::core::ffi::c_int {
                    while *z as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                        == 0x80 as ::core::ffi::c_int
                    {
                        z = z.offset(1);
                    }
                }
                p1 -= 1;
            }
            z2 = z;
            while *z2 as ::core::ffi::c_int != 0 && p2 != 0 {
                let c2rust_fresh11 = z2;
                z2 = z2.offset(1);
                if *c2rust_fresh11 as ::core::ffi::c_int >= 0xc0 as ::core::ffi::c_int {
                    while *z2 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                        == 0x80 as ::core::ffi::c_int
                    {
                        z2 = z2.offset(1);
                    }
                }
                p2 -= 1;
            }
            sqlite3_result_text64(
                context,
                z as *mut ::core::ffi::c_char,
                z2.offset_from(z) as ::core::ffi::c_long as sqlite3_uint64,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                SQLITE_UTF8 as ::core::ffi::c_uchar,
            );
        } else {
            if p1 >= len as ::core::ffi::c_longlong {
                p2 = 0 as i64_0;
                p1 = p2;
            } else if p2 > len as i64_0 - p1 {
                p2 = len as i64_0 - p1;
            }
            sqlite3_result_blob64(
                context,
                z.offset(p1 as isize) as *const ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                p2 as sqlite3_uint64,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        };
    }
}
unsafe extern "C" fn roundFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut n: i64_0 = 0 as i64_0;
        let mut r: ::core::ffi::c_double = 0.;
        let mut zBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if argc == 2 as ::core::ffi::c_int {
            if SQLITE_NULL
                == sqlite3_value_type(*argv.offset(1 as ::core::ffi::c_int as isize))
            {
                return;
            }
            n = sqlite3_value_int64(*argv.offset(1 as ::core::ffi::c_int as isize))
                as i64_0;
            if n > 30 as ::core::ffi::c_longlong {
                n = 30 as i64_0;
            }
            if n < 0 as ::core::ffi::c_longlong {
                n = 0 as i64_0;
            }
        }
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            == SQLITE_NULL
        {
            return;
        }
        r = sqlite3_value_double(*argv.offset(0 as ::core::ffi::c_int as isize));
        if !(r < -4503599627370496.0f64 || r > 4503599627370496.0f64) {
            if n == 0 as ::core::ffi::c_longlong {
                r = (r
                    + (if r < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        -0.5f64
                    } else {
                        0.5f64
                    })) as sqlite_int64 as ::core::ffi::c_double;
            } else {
                zBuf = sqlite3_mprintf(
                    b"%!.*f\0".as_ptr() as *const ::core::ffi::c_char,
                    n as ::core::ffi::c_int,
                    r,
                );
                if zBuf.is_null() {
                    sqlite3_result_error_nomem(context);
                    return;
                }
                sqlite3AtoF(
                    zBuf,
                    &raw mut r,
                    sqlite3Strlen30(zBuf),
                    SQLITE_UTF8 as u8_0,
                );
                sqlite3_free(zBuf as *mut ::core::ffi::c_void);
            }
        }
        sqlite3_result_double(context, r);
    }
}
unsafe extern "C" fn contextMalloc(
    mut context: *mut sqlite3_context,
    mut nByte: i64_0,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        if nByte > (*db).aLimit[SQLITE_LIMIT_LENGTH as usize] as ::core::ffi::c_longlong
        {
            sqlite3_result_error_toobig(context);
            z = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            z = sqlite3Malloc(nByte as u64_0) as *mut ::core::ffi::c_char;
            if z.is_null() {
                sqlite3_result_error_nomem(context);
            }
        }
        return z as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn upperFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut z1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut z2: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        z2 = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_char;
        n = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        if !z2.is_null() {
            z1 = contextMalloc(context, n as i64_0 + 1 as i64_0)
                as *mut ::core::ffi::c_char;
            if !z1.is_null() {
                i = 0 as ::core::ffi::c_int;
                while i < n {
                    *z1.offset(i as isize) = (*z2.offset(i as isize)
                        as ::core::ffi::c_int
                        & !(*(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *z2.offset(i as isize) as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                    i += 1;
                }
                sqlite3_result_text(
                    context,
                    z1,
                    n,
                    Some(
                        sqlite3_free
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
            }
        }
    }
}
unsafe extern "C" fn lowerFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut z1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut z2: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        z2 = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_char;
        n = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        if !z2.is_null() {
            z1 = contextMalloc(context, n as i64_0 + 1 as i64_0)
                as *mut ::core::ffi::c_char;
            if !z1.is_null() {
                i = 0 as ::core::ffi::c_int;
                while i < n {
                    *z1.offset(i as isize) = *(&raw const sqlite3UpperToLower
                        as *const ::core::ffi::c_uchar)
                        .offset(*z2.offset(i as isize) as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_char;
                    i += 1;
                }
                sqlite3_result_text(
                    context,
                    z1,
                    n,
                    Some(
                        sqlite3_free
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
            }
        }
    }
}
unsafe extern "C" fn randomFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut r: sqlite_int64 = 0;
        sqlite3_randomness(
            ::core::mem::size_of::<sqlite_int64>() as ::core::ffi::c_int,
            &raw mut r as *mut ::core::ffi::c_void,
        );
        if r < 0 as ::core::ffi::c_longlong {
            r = -(r as i64_0 & LARGEST_INT64) as sqlite_int64;
        }
        sqlite3_result_int64(context, r as sqlite3_int64);
    }
}
unsafe extern "C" fn randomBlob(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut n: sqlite3_int64 = 0;
        let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        n = sqlite3_value_int64(*argv.offset(0 as ::core::ffi::c_int as isize));
        if n < 1 as ::core::ffi::c_longlong {
            n = 1 as sqlite3_int64;
        }
        p = contextMalloc(context, n as i64_0) as *mut ::core::ffi::c_uchar;
        if !p.is_null() {
            sqlite3_randomness(n as ::core::ffi::c_int, p as *mut ::core::ffi::c_void);
            sqlite3_result_blob(
                context,
                p as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                n as ::core::ffi::c_int,
                Some(
                    sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        }
    }
}
unsafe extern "C" fn last_insert_rowid(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        sqlite3_result_int64(context, sqlite3_last_insert_rowid(db));
    }
}
unsafe extern "C" fn changes(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        sqlite3_result_int64(context, sqlite3_changes64(db));
    }
}
unsafe extern "C" fn total_changes(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        sqlite3_result_int64(context, sqlite3_total_changes64(db));
    }
}
static mut globInfo: compareInfo = compareInfo {
    matchAll: '*' as i32 as u8_0,
    matchOne: '?' as i32 as u8_0,
    matchSet: '[' as i32 as u8_0,
    noCase: 0 as u8_0

};
static mut likeInfoNorm: compareInfo = compareInfo {
    matchAll: '%' as i32 as u8_0,
    matchOne: '_' as i32 as u8_0,
    matchSet: 0 as u8_0,
    noCase: 1 as u8_0

};
static mut likeInfoAlt: compareInfo = compareInfo {
    matchAll: '%' as i32 as u8_0,
    matchOne: '_' as i32 as u8_0,
    matchSet: 0 as u8_0,
    noCase: 0 as u8_0

};
pub const SQLITE_MATCH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMATCH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOWILDCARDMATCH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn patternCompare(
    mut zPattern: *const u8_0,
    mut zString: *const u8_0,
    mut pInfo: *const compareInfo,
    mut matchOther: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: u32_0 = 0;
        let mut c2: u32_0 = 0;
        let mut matchOne: u32_0 = (*pInfo).matchOne as u32_0;
        let mut matchAll: u32_0 = (*pInfo).matchAll as u32_0;
        let mut noCase: u8_0 = (*pInfo).noCase;
        let mut zEscaped: *const u8_0 = ::core::ptr::null::<u8_0>();
        loop {
            c = (if (*zPattern.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int
            {
                let c2rust_fresh0 = zPattern;
                zPattern = zPattern.offset(1);
                *c2rust_fresh0 as u32_0
            } else {
                sqlite3Utf8Read(&raw mut zPattern)
            });
            if !(c != 0 as u32_0) {
                break;
            }
            if c == matchAll {
                loop {
                    c = (if (*zPattern.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int
                    {
                        let c2rust_fresh1 = zPattern;
                        zPattern = zPattern.offset(1);
                        *c2rust_fresh1 as u32_0
                    } else {
                        sqlite3Utf8Read(&raw mut zPattern)
                    });
                    if !(c == matchAll || c == matchOne && matchOne != 0 as u32_0) {
                        break;
                    }
                    if c == matchOne && sqlite3Utf8Read(&raw mut zString) == 0 as u32_0 {
                        return SQLITE_NOWILDCARDMATCH;
                    }
                }
                if c == 0 as u32_0 {
                    return SQLITE_MATCH
                } else if c == matchOther {
                    if (*pInfo).matchSet as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        c = sqlite3Utf8Read(&raw mut zPattern);
                        if c == 0 as u32_0 {
                            return SQLITE_NOWILDCARDMATCH;
                        }
                    } else {
                        while *zString != 0 {
                            let mut bMatch: ::core::ffi::c_int = patternCompare(
                                zPattern.offset(-1 as ::core::ffi::c_int as isize)
                                    as *const u8_0,
                                zString,
                                pInfo,
                                matchOther,
                            );
                            if bMatch != SQLITE_NOMATCH {
                                return bMatch;
                            }
                            let c2rust_fresh2 = zString;
                            zString = zString.offset(1);
                            if *c2rust_fresh2 as ::core::ffi::c_int
                                >= 0xc0 as ::core::ffi::c_int
                            {
                                while *zString as ::core::ffi::c_int
                                    & 0xc0 as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int
                                {
                                    zString = zString.offset(1);
                                }
                            }
                        }
                        return SQLITE_NOWILDCARDMATCH;
                    }
                }
                if c < 0x80 as u32_0 {
                    let mut zStop: [::core::ffi::c_char; 3] = [0; 3];
                    let mut bMatch_0: ::core::ffi::c_int = 0;
                    if noCase != 0 {
                        zStop[0 as ::core::ffi::c_int as usize] = (c
                            & !(*(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(c as ::core::ffi::c_uchar as isize)
                                as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int)
                                as u32_0) as ::core::ffi::c_char;
                        zStop[1 as ::core::ffi::c_int as usize] = *(&raw const sqlite3UpperToLower
                            as *const ::core::ffi::c_uchar)
                            .offset(c as ::core::ffi::c_uchar as isize)
                            as ::core::ffi::c_char;
                        zStop[2 as ::core::ffi::c_int as usize] = 0
                            as ::core::ffi::c_char;
                    } else {
                        zStop[0 as ::core::ffi::c_int as usize] = c
                            as ::core::ffi::c_char;
                        zStop[1 as ::core::ffi::c_int as usize] = 0
                            as ::core::ffi::c_char;
                    }
                    loop {
                        zString = zString
                            .offset(
                                strcspn(
                                    zString as *const ::core::ffi::c_char,
                                    &raw mut zStop as *mut ::core::ffi::c_char,
                                ) as isize,
                            );
                        if *zString.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        zString = zString.offset(1);
                        bMatch_0 = patternCompare(zPattern, zString, pInfo, matchOther);
                        if bMatch_0 != SQLITE_NOMATCH {
                            return bMatch_0;
                        }
                    }
                } else {
                    let mut bMatch_1: ::core::ffi::c_int = 0;
                    loop {
                        c2 = (if (*zString.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int
                        {
                            let c2rust_fresh3 = zString;
                            zString = zString.offset(1);
                            *c2rust_fresh3 as u32_0
                        } else {
                            sqlite3Utf8Read(&raw mut zString)
                        });
                        if !(c2 != 0 as u32_0) {
                            break;
                        }
                        if c2 != c {
                            continue;
                        }
                        bMatch_1 = patternCompare(zPattern, zString, pInfo, matchOther);
                        if bMatch_1 != SQLITE_NOMATCH {
                            return bMatch_1;
                        }
                    }
                }
                return SQLITE_NOWILDCARDMATCH;
            }
            if c == matchOther {
                if (*pInfo).matchSet as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    c = sqlite3Utf8Read(&raw mut zPattern);
                    if c == 0 as u32_0 {
                        return SQLITE_NOMATCH;
                    }
                    zEscaped = zPattern;
                } else {
                    let mut prior_c: u32_0 = 0 as u32_0;
                    let mut seen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut invert: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    c = sqlite3Utf8Read(&raw mut zString);
                    if c == 0 as u32_0 {
                        return SQLITE_NOMATCH;
                    }
                    c2 = sqlite3Utf8Read(&raw mut zPattern);
                    if c2 == '^' as i32 as u32_0 {
                        invert = 1 as ::core::ffi::c_int;
                        c2 = sqlite3Utf8Read(&raw mut zPattern);
                    }
                    if c2 == ']' as i32 as u32_0 {
                        if c == ']' as i32 as u32_0 {
                            seen = 1 as ::core::ffi::c_int;
                        }
                        c2 = sqlite3Utf8Read(&raw mut zPattern);
                    }
                    while c2 != 0 && c2 != ']' as i32 as u32_0 {
                        if c2 == '-' as i32 as u32_0
                            && *zPattern.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int != ']' as i32
                            && *zPattern.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                            && prior_c > 0 as u32_0
                        {
                            c2 = sqlite3Utf8Read(&raw mut zPattern);
                            if c >= prior_c && c <= c2 {
                                seen = 1 as ::core::ffi::c_int;
                            }
                            prior_c = 0 as u32_0;
                        } else {
                            if c == c2 {
                                seen = 1 as ::core::ffi::c_int;
                            }
                            prior_c = c2;
                        }
                        c2 = sqlite3Utf8Read(&raw mut zPattern);
                    }
                    if c2 == 0 as u32_0 || seen ^ invert == 0 as ::core::ffi::c_int {
                        return SQLITE_NOMATCH;
                    }
                    continue;
                }
            }
            c2 = if (*zString.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int
            {
                let c2rust_fresh4 = zString;
                zString = zString.offset(1);
                *c2rust_fresh4 as u32_0
            } else {
                sqlite3Utf8Read(&raw mut zString)
            };
            if c == c2 {
                continue;
            }
            if noCase as ::core::ffi::c_int != 0
                && *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                    .offset(c as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                    == *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                        .offset(c2 as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int && c < 0x80 as u32_0 && c2 < 0x80 as u32_0
            {
                continue;
            }
            if c == matchOne && zPattern != zEscaped && c2 != 0 as u32_0 {
                continue;
            }
            return SQLITE_NOMATCH;
        }
        return if *zString as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            SQLITE_MATCH
        } else {
            SQLITE_NOMATCH
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_strglob(
    mut zGlobPattern: *const ::core::ffi::c_char,
    mut zString: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if zString.is_null() {
            return !zGlobPattern.is_null() as ::core::ffi::c_int
        } else if zGlobPattern.is_null() {
            return 1 as ::core::ffi::c_int
        } else {
            return patternCompare(
                zGlobPattern as *mut u8_0,
                zString as *mut u8_0,
                &raw const globInfo,
                '[' as i32 as u32_0,
            )
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_strlike(
    mut zPattern: *const ::core::ffi::c_char,
    mut zStr: *const ::core::ffi::c_char,
    mut esc: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        if zStr.is_null() {
            return !zPattern.is_null() as ::core::ffi::c_int
        } else if zPattern.is_null() {
            return 1 as ::core::ffi::c_int
        } else {
            return patternCompare(
                zPattern as *mut u8_0,
                zStr as *mut u8_0,
                &raw const likeInfoNorm,
                esc as u32_0,
            )
        };
    }
}
#[no_mangle]
pub static mut sqlite3_like_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn likeFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zA: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut zB: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut escape: u32_0 = 0;
        let mut nPat: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut pInfo: *mut compareInfo = sqlite3_user_data(context) as *mut compareInfo;
        let mut backupInfo: compareInfo = compareInfo {
            matchAll: 0,
            matchOne: 0,
            matchSet: 0,
            noCase: 0

        };
        nPat = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        if nPat > (*db).aLimit[SQLITE_LIMIT_LIKE_PATTERN_LENGTH as usize] {
            sqlite3_result_error(
                context,
                b"LIKE or GLOB pattern too complex\0".as_ptr()
                    as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            );
            return;
        }
        if argc == 3 as ::core::ffi::c_int {
            let mut zEsc: *const ::core::ffi::c_uchar = sqlite3_value_text(
                *argv.offset(2 as ::core::ffi::c_int as isize),
            );
            if zEsc.is_null() {
                return;
            }
            if sqlite3Utf8CharLen(
                zEsc as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ) != 1 as ::core::ffi::c_int
            {
                sqlite3_result_error(
                    context,
                    b"ESCAPE expression must be a single character\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                return;
            }
            escape = sqlite3Utf8Read(&raw mut zEsc);
            if escape == (*pInfo).matchAll as u32_0
                || escape == (*pInfo).matchOne as u32_0
            {
                memcpy(
                    &raw mut backupInfo as *mut ::core::ffi::c_void,
                    pInfo as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<compareInfo>() as size_t,
                );
                pInfo = &raw mut backupInfo;
                if escape == (*pInfo).matchAll as u32_0 {
                    (*pInfo).matchAll = 0 as u8_0;
                }
                if escape == (*pInfo).matchOne as u32_0 {
                    (*pInfo).matchOne = 0 as u8_0;
                }
            }
        } else {
            escape = (*pInfo).matchSet as u32_0;
        }
        zB = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
        zA = sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize));
        if !zA.is_null() && !zB.is_null() {
            sqlite3_like_count += 1;
            sqlite3_result_int(
                context,
                (patternCompare(zB as *const u8_0, zA as *const u8_0, pInfo, escape)
                    == SQLITE_MATCH) as ::core::ffi::c_int,
            );
        }
    }
}
unsafe extern "C" fn nullifFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pColl: *mut CollSeq = sqlite3GetFuncCollSeq(context);
        if sqlite3MemCompare(
            *argv.offset(0 as ::core::ffi::c_int as isize),
            *argv.offset(1 as ::core::ffi::c_int as isize),
            pColl,
        ) != 0 as ::core::ffi::c_int
        {
            sqlite3_result_value(
                context,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
        }
    }
}
unsafe extern "C" fn versionFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_result_text(
            context,
            sqlite3_libversion(),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
}
unsafe extern "C" fn sourceidFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_result_text(
            context,
            sqlite3_sourceid(),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
}
unsafe extern "C" fn errlogFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_log(
            sqlite3_value_int(*argv.offset(0 as ::core::ffi::c_int as isize)),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize)),
        );
    }
}
unsafe extern "C" fn compileoptionusedFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zOptName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        zOptName = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        if !zOptName.is_null() {
            sqlite3_result_int(context, sqlite3_compileoption_used(zOptName));
        }
    }
}
unsafe extern "C" fn compileoptiongetFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        n = sqlite3_value_int(*argv.offset(0 as ::core::ffi::c_int as isize));
        sqlite3_result_text(
            context,
            sqlite3_compileoption_get(n),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
}
static mut hexdigits: [::core::ffi::c_char; 16] = [
    '0' as i32 as ::core::ffi::c_char,
    '1' as i32 as ::core::ffi::c_char,
    '2' as i32 as ::core::ffi::c_char,
    '3' as i32 as ::core::ffi::c_char,
    '4' as i32 as ::core::ffi::c_char,
    '5' as i32 as ::core::ffi::c_char,
    '6' as i32 as ::core::ffi::c_char,
    '7' as i32 as ::core::ffi::c_char,
    '8' as i32 as ::core::ffi::c_char,
    '9' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn sqlite3QuoteValue(
    mut pStr: *mut StrAccum,
    mut pValue: *mut sqlite3_value,
    mut bEscape: ::core::ffi::c_int,
) {
    unsafe {
        match sqlite3_value_type(pValue) {
            SQLITE_FLOAT => {
                let mut r1: ::core::ffi::c_double = 0.;
                let mut r2: ::core::ffi::c_double = 0.;
                let mut zVal: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                r1 = sqlite3_value_double(pValue);
                sqlite3_str_appendf(
                    pStr as *mut sqlite3_str,
                    b"%!0.15g\0".as_ptr() as *const ::core::ffi::c_char,
                    r1,
                );
                zVal = sqlite3_str_value(pStr as *mut sqlite3_str);
                if !zVal.is_null() {
                    sqlite3AtoF(
                        zVal,
                        &raw mut r2,
                        (*pStr).nChar as ::core::ffi::c_int,
                        SQLITE_UTF8 as u8_0,
                    );
                    if r1 != r2 {
                        sqlite3_str_reset(pStr as *mut sqlite3_str);
                        sqlite3_str_appendf(
                            pStr as *mut sqlite3_str,
                            b"%!0.20e\0".as_ptr() as *const ::core::ffi::c_char,
                            r1,
                        );
                    }
                }
            }
            SQLITE_INTEGER => {
                sqlite3_str_appendf(
                    pStr as *mut sqlite3_str,
                    b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_value_int64(pValue),
                );
            }
            SQLITE_BLOB => {
                let mut zBlob: *const ::core::ffi::c_char = sqlite3_value_blob(pValue)
                    as *const ::core::ffi::c_char;
                let mut nBlob: i64_0 = sqlite3_value_bytes(pValue) as i64_0;
                sqlite3StrAccumEnlarge(pStr, nBlob * 2 as i64_0 + 4 as i64_0);
                if (*pStr).accError as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    let mut zText: *mut ::core::ffi::c_char = (*pStr).zText;
                    let mut i: ::core::ffi::c_int = 0;
                    i = 0 as ::core::ffi::c_int;
                    while (i as ::core::ffi::c_longlong) < nBlob {
                        *zText
                            .offset(
                                (i * 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                    as isize,
                            ) = hexdigits[(*zBlob.offset(i as isize)
                            as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int) as usize];
                        *zText
                            .offset(
                                (i * 2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                    as isize,
                            ) = hexdigits[(*zBlob.offset(i as isize)
                            as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize];
                        i += 1;
                    }
                    *zText
                        .offset(
                            (nBlob as ::core::ffi::c_longlong
                                * 2 as ::core::ffi::c_longlong
                                + 2 as ::core::ffi::c_longlong) as isize,
                        ) = '\'' as i32 as ::core::ffi::c_char;
                    *zText
                        .offset(
                            (nBlob as ::core::ffi::c_longlong
                                * 2 as ::core::ffi::c_longlong
                                + 3 as ::core::ffi::c_longlong) as isize,
                        ) = '\0' as i32 as ::core::ffi::c_char;
                    *zText.offset(0 as ::core::ffi::c_int as isize) = 'X' as i32
                        as ::core::ffi::c_char;
                    *zText.offset(1 as ::core::ffi::c_int as isize) = '\'' as i32
                        as ::core::ffi::c_char;
                    (*pStr).nChar = (nBlob as ::core::ffi::c_longlong
                        * 2 as ::core::ffi::c_longlong + 3 as ::core::ffi::c_longlong)
                        as u32_0;
                }
            }
            SQLITE_TEXT => {
                let mut zArg: *const ::core::ffi::c_uchar = sqlite3_value_text(pValue);
                sqlite3_str_appendf(
                    pStr as *mut sqlite3_str,
                    if bEscape != 0 {
                        b"%#Q\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"%Q\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    zArg,
                );
            }
            _ => {
                sqlite3_str_append(
                    pStr as *mut sqlite3_str,
                    b"NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    4 as ::core::ffi::c_int,
                );
            }
        };
    }
}
unsafe extern "C" fn isNHex(
    mut z: *const ::core::ffi::c_char,
    mut N: ::core::ffi::c_int,
    mut pVal: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut v: u32_0 = 0 as u32_0;
        i = 0 as ::core::ffi::c_int;
        while i < N {
            if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(*z.offset(i as isize) as ::core::ffi::c_uchar as isize)
                as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            v = (v << 4 as ::core::ffi::c_int)
                .wrapping_add(
                    sqlite3HexToInt(*z.offset(i as isize) as ::core::ffi::c_int) as u32_0,
                );
            i += 1;
        }
        *pVal = v;
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn unistrFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut zOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zIn: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nIn: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut v: u32_0 = 0;
        zIn = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        if zIn.is_null() {
            return;
        }
        nIn = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        zOut = sqlite3_malloc64((nIn + 1 as ::core::ffi::c_int) as sqlite3_uint64)
            as *mut ::core::ffi::c_char;
        if zOut.is_null() {
            sqlite3_result_error_nomem(context);
            return;
        }
        j = 0 as ::core::ffi::c_int;
        i = j;
        loop {
            if !(i < nIn) {
                c2rust_current_block = 15597372965620363352;
                break;
            }
            let mut z: *mut ::core::ffi::c_char = strchr(
                zIn.offset(i as isize) as *const ::core::ffi::c_char,
                '\\' as i32,
            );
            if z.is_null() {
                n = nIn - i;
                memmove(
                    zOut.offset(j as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    zIn.offset(i as isize) as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    n as size_t,
                );
                j += n;
                c2rust_current_block = 15597372965620363352;
                break;
            } else {
                n = z.offset_from(zIn.offset(i as isize) as *const ::core::ffi::c_char)
                    as ::core::ffi::c_long as ::core::ffi::c_int;
                if n > 0 as ::core::ffi::c_int {
                    memmove(
                        zOut.offset(j as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        zIn.offset(i as isize) as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        n as size_t,
                    );
                    j += n;
                    i += n;
                }
                if *zIn.offset((i + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int == '\\' as i32
                {
                    i += 2 as ::core::ffi::c_int;
                    let c2rust_fresh13 = j;
                    j = j + 1;
                    *zOut.offset(c2rust_fresh13 as isize) = '\\' as i32
                        as ::core::ffi::c_char;
                } else if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(
                        *zIn.offset((i + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_uchar as isize,
                    ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                {
                    if isNHex(
                        zIn.offset((i + 1 as ::core::ffi::c_int) as isize)
                            as *const ::core::ffi::c_char,
                        4 as ::core::ffi::c_int,
                        &raw mut v,
                    ) == 0
                    {
                        c2rust_current_block = 11352269577411190203;
                        break;
                    }
                    i += 5 as ::core::ffi::c_int;
                    j
                        += sqlite3AppendOneUtf8Character(
                            zOut.offset(j as isize) as *mut ::core::ffi::c_char,
                            v,
                        );
                } else if *zIn.offset((i + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int == '+' as i32
                {
                    if isNHex(
                        zIn.offset((i + 2 as ::core::ffi::c_int) as isize)
                            as *const ::core::ffi::c_char,
                        6 as ::core::ffi::c_int,
                        &raw mut v,
                    ) == 0
                    {
                        c2rust_current_block = 11352269577411190203;
                        break;
                    }
                    i += 8 as ::core::ffi::c_int;
                    j
                        += sqlite3AppendOneUtf8Character(
                            zOut.offset(j as isize) as *mut ::core::ffi::c_char,
                            v,
                        );
                } else if *zIn.offset((i + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int == 'u' as i32
                {
                    if isNHex(
                        zIn.offset((i + 2 as ::core::ffi::c_int) as isize)
                            as *const ::core::ffi::c_char,
                        4 as ::core::ffi::c_int,
                        &raw mut v,
                    ) == 0
                    {
                        c2rust_current_block = 11352269577411190203;
                        break;
                    }
                    i += 6 as ::core::ffi::c_int;
                    j
                        += sqlite3AppendOneUtf8Character(
                            zOut.offset(j as isize) as *mut ::core::ffi::c_char,
                            v,
                        );
                } else {
                    if !(*zIn.offset((i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 'U' as i32)
                    {
                        c2rust_current_block = 11352269577411190203;
                        break;
                    }
                    if isNHex(
                        zIn.offset((i + 2 as ::core::ffi::c_int) as isize)
                            as *const ::core::ffi::c_char,
                        8 as ::core::ffi::c_int,
                        &raw mut v,
                    ) == 0
                    {
                        c2rust_current_block = 11352269577411190203;
                        break;
                    }
                    i += 10 as ::core::ffi::c_int;
                    j
                        += sqlite3AppendOneUtf8Character(
                            zOut.offset(j as isize) as *mut ::core::ffi::c_char,
                            v,
                        );
                }
            }
        }
        match c2rust_current_block {
            11352269577411190203 => {
                sqlite3_free(zOut as *mut ::core::ffi::c_void);
                sqlite3_result_error(
                    context,
                    b"invalid Unicode escape\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                return;
            }
            _ => {
                *zOut.offset(j as isize) = 0 as ::core::ffi::c_char;
                sqlite3_result_text64(
                    context,
                    zOut,
                    j as sqlite3_uint64,
                    Some(
                        sqlite3_free
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                    SQLITE_UTF8 as ::core::ffi::c_uchar,
                );
                return;
            }
        };
    }
}
unsafe extern "C" fn quoteFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut str: sqlite3_str = sqlite3_str {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        sqlite3StrAccumInit(
            &raw mut str,
            db,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            (*db).aLimit[SQLITE_LIMIT_LENGTH as usize],
        );
        sqlite3QuoteValue(
            &raw mut str,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            sqlite3_user_data(context) as intptr_t as ::core::ffi::c_int,
        );
        sqlite3_result_text(
            context,
            sqlite3StrAccumFinish(&raw mut str),
            str.nChar as ::core::ffi::c_int,
            Some(
                sqlite3RowSetClear
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
        if str.accError as ::core::ffi::c_int != SQLITE_OK {
            sqlite3_result_null(context);
            sqlite3_result_error_code(context, str.accError as ::core::ffi::c_int);
        }
    }
}
unsafe extern "C" fn unicodeFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut z: *const ::core::ffi::c_uchar = sqlite3_value_text(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        if !z.is_null()
            && *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        {
            sqlite3_result_int(
                context,
                sqlite3Utf8Read(&raw mut z) as ::core::ffi::c_int,
            );
        }
    }
}
unsafe extern "C" fn charFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut z: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut zOut: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut i: ::core::ffi::c_int = 0;
        z = sqlite3_malloc64(
            (argc * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as sqlite3_uint64,
        ) as *mut ::core::ffi::c_uchar;
        zOut = z;
        if z.is_null() {
            sqlite3_result_error_nomem(context);
            return;
        }
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            let mut x: sqlite3_int64 = 0;
            let mut c: ::core::ffi::c_uint = 0;
            x = sqlite3_value_int64(*argv.offset(i as isize));
            if x < 0 as ::core::ffi::c_longlong
                || x > 0x10ffff as ::core::ffi::c_longlong
            {
                x = 0xfffd as sqlite3_int64;
            }
            c = (x as ::core::ffi::c_longlong & 0x1fffff as ::core::ffi::c_longlong)
                as ::core::ffi::c_uint;
            if c < 0x80 as ::core::ffi::c_uint {
                let c2rust_fresh20 = zOut;
                zOut = zOut.offset(1);
                *c2rust_fresh20 = (c & 0xff as ::core::ffi::c_uint) as u8_0
                    as ::core::ffi::c_uchar;
            } else if c < 0x800 as ::core::ffi::c_uint {
                let c2rust_fresh21 = zOut;
                zOut = zOut.offset(1);
                *c2rust_fresh21 = (0xc0 as ::core::ffi::c_int
                    + (c >> 6 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_uint)
                        as u8_0 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                let c2rust_fresh22 = zOut;
                zOut = zOut.offset(1);
                *c2rust_fresh22 = (0x80 as ::core::ffi::c_int
                    + (c & 0x3f as ::core::ffi::c_uint) as u8_0 as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            } else if c < 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint {
                let c2rust_fresh23 = zOut;
                zOut = zOut.offset(1);
                *c2rust_fresh23 = (0xe0 as ::core::ffi::c_int
                    + (c >> 12 as ::core::ffi::c_int & 0xf as ::core::ffi::c_uint)
                        as u8_0 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                let c2rust_fresh24 = zOut;
                zOut = zOut.offset(1);
                *c2rust_fresh24 = (0x80 as ::core::ffi::c_int
                    + (c >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_uint)
                        as u8_0 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                let c2rust_fresh25 = zOut;
                zOut = zOut.offset(1);
                *c2rust_fresh25 = (0x80 as ::core::ffi::c_int
                    + (c & 0x3f as ::core::ffi::c_uint) as u8_0 as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            } else {
                let c2rust_fresh26 = zOut;
                zOut = zOut.offset(1);
                *c2rust_fresh26 = (0xf0 as ::core::ffi::c_int
                    + (c >> 18 as ::core::ffi::c_int & 0x7 as ::core::ffi::c_uint)
                        as u8_0 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                let c2rust_fresh27 = zOut;
                zOut = zOut.offset(1);
                *c2rust_fresh27 = (0x80 as ::core::ffi::c_int
                    + (c >> 12 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_uint)
                        as u8_0 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                let c2rust_fresh28 = zOut;
                zOut = zOut.offset(1);
                *c2rust_fresh28 = (0x80 as ::core::ffi::c_int
                    + (c >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_uint)
                        as u8_0 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                let c2rust_fresh29 = zOut;
                zOut = zOut.offset(1);
                *c2rust_fresh29 = (0x80 as ::core::ffi::c_int
                    + (c & 0x3f as ::core::ffi::c_uint) as u8_0 as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
            }
            i += 1;
        }
        *zOut = 0 as ::core::ffi::c_uchar;
        sqlite3_result_text64(
            context,
            z as *mut ::core::ffi::c_char,
            zOut.offset_from(z) as ::core::ffi::c_long as sqlite3_uint64,
            Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            SQLITE_UTF8 as ::core::ffi::c_uchar,
        );
    }
}
unsafe extern "C" fn hexFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut pBlob: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut zHex: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        pBlob = sqlite3_value_blob(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_uchar;
        n = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        zHex = contextMalloc(context, n as i64_0 * 2 as i64_0 + 1 as i64_0)
            as *mut ::core::ffi::c_char;
        z = zHex;
        if !zHex.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < n {
                let mut c: ::core::ffi::c_uchar = *pBlob;
                let c2rust_fresh18 = z;
                z = z.offset(1);
                *c2rust_fresh18 = hexdigits[(c as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize];
                let c2rust_fresh19 = z;
                z = z.offset(1);
                *c2rust_fresh19 = hexdigits[(c as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int) as usize];
                i += 1;
                pBlob = pBlob.offset(1);
            }
            *z = 0 as ::core::ffi::c_char;
            sqlite3_result_text64(
                context,
                zHex,
                z.offset_from(zHex) as ::core::ffi::c_long as sqlite3_uint64,
                Some(
                    sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
                SQLITE_UTF8 as ::core::ffi::c_uchar,
            );
        }
    }
}
unsafe extern "C" fn strContainsChar(
    mut zStr: *const u8_0,
    mut nStr: ::core::ffi::c_int,
    mut ch: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zEnd: *const u8_0 = zStr.offset(nStr as isize) as *const u8_0;
        let mut z: *const u8_0 = zStr;
        while z < zEnd {
            let mut tst: u32_0 = if (*z.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int
            {
                let c2rust_fresh17 = z;
                z = z.offset(1);
                *c2rust_fresh17 as u32_0
            } else {
                sqlite3Utf8Read(&raw mut z)
            };
            if tst == ch {
                return 1 as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn unhexFunc(
    mut pCtx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut zPass: *const u8_0 = b"\0".as_ptr() as *const ::core::ffi::c_char
            as *const u8_0;
        let mut nPass: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zHex: *const u8_0 = sqlite3_value_text(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) as *const u8_0;
        let mut nHex: ::core::ffi::c_int = sqlite3_value_bytes(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        let mut pBlob: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut p: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        if argc == 2 as ::core::ffi::c_int {
            zPass = sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize))
                as *const u8_0;
            nPass = sqlite3_value_bytes(*argv.offset(1 as ::core::ffi::c_int as isize));
        }
        if zHex.is_null() || zPass.is_null() {
            return;
        }
        pBlob = contextMalloc(
            pCtx,
            (nHex / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as i64_0,
        ) as *mut u8_0;
        p = pBlob;
        if !pBlob.is_null() {
            let mut c: u8_0 = 0;
            let mut d: u8_0 = 0;
            's_48: loop {
                c = *zHex;
                if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                    c2rust_current_block = 18046725890150364449;
                    break;
                }
                while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(c as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                    & 0x8 as ::core::ffi::c_int == 0
                {
                    let mut ch: u32_0 = if (*zHex
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        < 0x80 as ::core::ffi::c_int
                    {
                        let c2rust_fresh14 = zHex;
                        zHex = zHex.offset(1);
                        *c2rust_fresh14 as u32_0
                    } else {
                        sqlite3Utf8Read(&raw mut zHex)
                    };
                    if strContainsChar(zPass, nPass, ch) == 0 {
                        c2rust_current_block = 11172533437425673027;
                        break 's_48;
                    }
                    c = *zHex;
                    if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        c2rust_current_block = 18046725890150364449;
                        break 's_48;
                    }
                }
                zHex = zHex.offset(1);
                let c2rust_fresh15 = zHex;
                zHex = zHex.offset(1);
                d = *c2rust_fresh15;
                if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(d as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                    & 0x8 as ::core::ffi::c_int == 0
                {
                    c2rust_current_block = 11172533437425673027;
                    break;
                }
                let c2rust_fresh16 = p;
                p = p.offset(1);
                *c2rust_fresh16 = ((sqlite3HexToInt(c as ::core::ffi::c_int)
                    as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | sqlite3HexToInt(d as ::core::ffi::c_int) as ::core::ffi::c_int)
                    as u8_0;
            }
            match c2rust_current_block {
                18046725890150364449 => {}
                _ => {
                    sqlite3_free(pBlob as *mut ::core::ffi::c_void);
                    return;
                }
            }
        }
        sqlite3_result_blob(
            pCtx,
            pBlob as *const ::core::ffi::c_void,
            p.offset_from(pBlob) as ::core::ffi::c_long as ::core::ffi::c_int,
            Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
    }
}
unsafe extern "C" fn zeroblobFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut n: i64_0 = 0;
        let mut rc: ::core::ffi::c_int = 0;
        n = sqlite3_value_int64(*argv.offset(0 as ::core::ffi::c_int as isize)) as i64_0;
        if n < 0 as ::core::ffi::c_longlong {
            n = 0 as i64_0;
        }
        rc = sqlite3_result_zeroblob64(context, n as sqlite3_uint64);
        if rc != 0 {
            sqlite3_result_error_code(context, rc);
        }
    }
}
unsafe extern "C" fn replaceFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zStr: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut zPattern: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut zRep: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut zOut: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut nStr: ::core::ffi::c_int = 0;
        let mut nPattern: ::core::ffi::c_int = 0;
        let mut nRep: ::core::ffi::c_int = 0;
        let mut nOut: i64_0 = 0;
        let mut loopLimit: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut cntExpand: ::core::ffi::c_uint = 0;
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        zStr = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
        if zStr.is_null() {
            return;
        }
        nStr = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        zPattern = sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize));
        if zPattern.is_null() {
            return;
        }
        if *zPattern.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            sqlite3_result_text(
                context,
                zStr as *const ::core::ffi::c_char,
                nStr,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            return;
        }
        nPattern = sqlite3_value_bytes(*argv.offset(1 as ::core::ffi::c_int as isize));
        zRep = sqlite3_value_text(*argv.offset(2 as ::core::ffi::c_int as isize));
        if zRep.is_null() {
            return;
        }
        nRep = sqlite3_value_bytes(*argv.offset(2 as ::core::ffi::c_int as isize));
        nOut = (nStr + 1 as ::core::ffi::c_int) as i64_0;
        zOut = contextMalloc(context, nOut) as *mut ::core::ffi::c_uchar;
        if zOut.is_null() {
            return;
        }
        loopLimit = nStr - nPattern;
        cntExpand = 0 as ::core::ffi::c_uint;
        j = 0 as ::core::ffi::c_int;
        i = j;
        while i <= loopLimit {
            if *zStr.offset(i as isize) as ::core::ffi::c_int
                != *zPattern.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                || memcmp(
                    zStr.offset(i as isize) as *const ::core::ffi::c_uchar
                        as *const ::core::ffi::c_void,
                    zPattern as *const ::core::ffi::c_void,
                    nPattern as size_t,
                ) != 0
            {
                let c2rust_fresh12 = j;
                j = j + 1;
                *zOut.offset(c2rust_fresh12 as isize) = *zStr.offset(i as isize);
            } else {
                if nRep > nPattern {
                    nOut += (nRep - nPattern) as ::core::ffi::c_longlong;
                    if nOut as ::core::ffi::c_longlong - 1 as ::core::ffi::c_longlong
                        > (*db).aLimit[SQLITE_LIMIT_LENGTH as usize]
                            as ::core::ffi::c_longlong
                    {
                        sqlite3_result_error_toobig(context);
                        sqlite3_free(zOut as *mut ::core::ffi::c_void);
                        return;
                    }
                    cntExpand = cntExpand.wrapping_add(1);
                    if cntExpand & cntExpand.wrapping_sub(1 as ::core::ffi::c_uint)
                        == 0 as ::core::ffi::c_uint
                    {
                        let mut zOld: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                        zOld = zOut as *mut u8_0;
                        zOut = sqlite3Realloc(
                            zOut as *mut ::core::ffi::c_void,
                            (nOut as ::core::ffi::c_int as ::core::ffi::c_longlong
                                + (nOut as ::core::ffi::c_longlong
                                    - nStr as ::core::ffi::c_longlong
                                    - 1 as ::core::ffi::c_longlong)) as u64_0,
                        ) as *mut ::core::ffi::c_uchar;
                        if zOut.is_null() {
                            sqlite3_result_error_nomem(context);
                            sqlite3_free(zOld as *mut ::core::ffi::c_void);
                            return;
                        }
                    }
                }
                memcpy(
                    zOut.offset(j as isize) as *mut ::core::ffi::c_uchar
                        as *mut ::core::ffi::c_void,
                    zRep as *const ::core::ffi::c_void,
                    nRep as size_t,
                );
                j += nRep;
                i += nPattern - 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        memcpy(
            zOut.offset(j as isize) as *mut ::core::ffi::c_uchar
                as *mut ::core::ffi::c_void,
            zStr.offset(i as isize) as *const ::core::ffi::c_uchar
                as *const ::core::ffi::c_void,
            (nStr - i) as size_t,
        );
        j += nStr - i;
        *zOut.offset(j as isize) = 0 as ::core::ffi::c_uchar;
        sqlite3_result_text(
            context,
            zOut as *mut ::core::ffi::c_char,
            j,
            Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
    }
}
unsafe extern "C" fn trimFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zIn: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut zCharSet: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut nIn: ::core::ffi::c_uint = 0;
        let mut flags: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut aLen: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<
            ::core::ffi::c_uint,
        >();
        let mut azChar: *mut *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            *mut ::core::ffi::c_uchar,
        >();
        let mut nChar: ::core::ffi::c_int = 0;
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            == SQLITE_NULL
        {
            return;
        }
        zIn = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
        if zIn.is_null() {
            return;
        }
        nIn = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_uint;
        if argc == 1 as ::core::ffi::c_int {
            static mut lenOne: [::core::ffi::c_uint; 1] = [
                1 as ::core::ffi::c_int as ::core::ffi::c_uint,
            ];
            static mut azOne: [*mut ::core::ffi::c_uchar; 1] = [
                b" \0".as_ptr() as *const ::core::ffi::c_char as *mut u8_0,
            ];
            nChar = 1 as ::core::ffi::c_int;
            aLen = &raw const lenOne as *const ::core::ffi::c_uint
                as *mut ::core::ffi::c_uint;
            azChar = &raw const azOne as *const *mut ::core::ffi::c_uchar
                as *mut *mut ::core::ffi::c_uchar;
            zCharSet = ::core::ptr::null::<::core::ffi::c_uchar>();
        } else {
            zCharSet = sqlite3_value_text(
                *argv.offset(1 as ::core::ffi::c_int as isize),
            );
            if zCharSet.is_null() {
                return
            } else {
                let mut z: *const ::core::ffi::c_uchar = ::core::ptr::null::<
                    ::core::ffi::c_uchar,
                >();
                z = zCharSet;
                nChar = 0 as ::core::ffi::c_int;
                while *z != 0 {
                    let c2rust_fresh30 = z;
                    z = z.offset(1);
                    if *c2rust_fresh30 as ::core::ffi::c_int
                        >= 0xc0 as ::core::ffi::c_int
                    {
                        while *z as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                            == 0x80 as ::core::ffi::c_int
                        {
                            z = z.offset(1);
                        }
                    }
                    nChar += 1;
                }
                if nChar > 0 as ::core::ffi::c_int {
                    azChar = contextMalloc(
                        context,
                        (nChar as i64_0 as ::core::ffi::c_ulonglong)
                            .wrapping_mul(
                                (::core::mem::size_of::<*mut ::core::ffi::c_char>()
                                    as usize)
                                    .wrapping_add(
                                        ::core::mem::size_of::<::core::ffi::c_uint>() as usize,
                                    ) as ::core::ffi::c_ulonglong,
                            ) as i64_0,
                    ) as *mut *mut ::core::ffi::c_uchar;
                    if azChar.is_null() {
                        return;
                    }
                    aLen = azChar.offset(nChar as isize)
                        as *mut *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_uint;
                    z = zCharSet;
                    nChar = 0 as ::core::ffi::c_int;
                    while *z != 0 {
                        let ref mut c2rust_fresh31 = *azChar.offset(nChar as isize);
                        *c2rust_fresh31 = z as *mut ::core::ffi::c_uchar;
                        let c2rust_fresh32 = z;
                        z = z.offset(1);
                        if *c2rust_fresh32 as ::core::ffi::c_int
                            >= 0xc0 as ::core::ffi::c_int
                        {
                            while *z as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                                == 0x80 as ::core::ffi::c_int
                            {
                                z = z.offset(1);
                            }
                        }
                        *aLen.offset(nChar as isize) = z
                            .offset_from(*azChar.offset(nChar as isize))
                            as ::core::ffi::c_long as ::core::ffi::c_uint;
                        nChar += 1;
                    }
                }
            }
        }
        if nChar > 0 as ::core::ffi::c_int {
            flags = sqlite3_user_data(context) as intptr_t as ::core::ffi::c_int;
            if flags & 1 as ::core::ffi::c_int != 0 {
                while nIn > 0 as ::core::ffi::c_uint {
                    let mut len: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                    i = 0 as ::core::ffi::c_int;
                    while i < nChar {
                        len = *aLen.offset(i as isize);
                        if len <= nIn
                            && memcmp(
                                zIn as *const ::core::ffi::c_void,
                                *azChar.offset(i as isize) as *const ::core::ffi::c_void,
                                len as size_t,
                            ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        i += 1;
                    }
                    if i >= nChar {
                        break;
                    }
                    zIn = zIn.offset(len as isize);
                    nIn = nIn.wrapping_sub(len);
                }
            }
            if flags & 2 as ::core::ffi::c_int != 0 {
                while nIn > 0 as ::core::ffi::c_uint {
                    let mut len_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                    i = 0 as ::core::ffi::c_int;
                    while i < nChar {
                        len_0 = *aLen.offset(i as isize);
                        if len_0 <= nIn
                            && memcmp(
                                zIn.offset(nIn.wrapping_sub(len_0) as isize)
                                    as *const ::core::ffi::c_uchar
                                    as *const ::core::ffi::c_void,
                                *azChar.offset(i as isize) as *const ::core::ffi::c_void,
                                len_0 as size_t,
                            ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        i += 1;
                    }
                    if i >= nChar {
                        break;
                    }
                    nIn = nIn.wrapping_sub(len_0);
                }
            }
            if !zCharSet.is_null() {
                sqlite3_free(azChar as *mut ::core::ffi::c_void);
            }
        }
        sqlite3_result_text(
            context,
            zIn as *mut ::core::ffi::c_char,
            nIn as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
unsafe extern "C" fn concatFuncCore(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
    mut nSep: ::core::ffi::c_int,
    mut zSep: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut j: i64_0 = 0;
        let mut n: i64_0 = 0 as i64_0;
        let mut i: ::core::ffi::c_int = 0;
        let mut bNotNull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            n
                += sqlite3_value_bytes(*argv.offset(i as isize))
                    as ::core::ffi::c_longlong;
            i += 1;
        }
        n
            += ((argc - 1 as ::core::ffi::c_int) as i64_0 * nSep as i64_0)
                as ::core::ffi::c_longlong;
        z = sqlite3_malloc64(
            (n as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if z.is_null() {
            sqlite3_result_error_nomem(context);
            return;
        }
        j = 0 as i64_0;
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            if sqlite3_value_type(*argv.offset(i as isize)) != SQLITE_NULL {
                let mut k: ::core::ffi::c_int = sqlite3_value_bytes(
                    *argv.offset(i as isize),
                );
                let mut v: *const ::core::ffi::c_char = sqlite3_value_text(
                    *argv.offset(i as isize),
                ) as *const ::core::ffi::c_char;
                if !v.is_null() {
                    if bNotNull != 0 && nSep > 0 as ::core::ffi::c_int {
                        memcpy(
                            z.offset(j as isize) as *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_void,
                            zSep as *const ::core::ffi::c_void,
                            nSep as size_t,
                        );
                        j += nSep as ::core::ffi::c_longlong;
                    }
                    memcpy(
                        z.offset(j as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        v as *const ::core::ffi::c_void,
                        k as size_t,
                    );
                    j += k as ::core::ffi::c_longlong;
                    bNotNull = 1 as ::core::ffi::c_int;
                }
            }
            i += 1;
        }
        *z.offset(j as isize) = 0 as ::core::ffi::c_char;
        sqlite3_result_text64(
            context,
            z,
            j as sqlite3_uint64,
            Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            SQLITE_UTF8 as ::core::ffi::c_uchar,
        );
    }
}
unsafe extern "C" fn concatFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        concatFuncCore(
            context,
            argc,
            argv,
            0 as ::core::ffi::c_int,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn concatwsFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut nSep: ::core::ffi::c_int = sqlite3_value_bytes(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        let mut zSep: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        if zSep.is_null() {
            return;
        }
        concatFuncCore(
            context,
            argc - 1 as ::core::ffi::c_int,
            argv.offset(1 as ::core::ffi::c_int as isize),
            nSep,
            zSep,
        );
    }
}
unsafe extern "C" fn loadExt(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zFile: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut zProc: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut zErrMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if (*db).flags as ::core::ffi::c_ulonglong
            & SQLITE_LoadExtFunc as ::core::ffi::c_ulonglong
            == 0 as ::core::ffi::c_ulonglong
        {
            sqlite3_result_error(
                context,
                b"not authorized\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            );
            return;
        }
        if argc == 2 as ::core::ffi::c_int {
            zProc = sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
        } else {
            zProc = ::core::ptr::null::<::core::ffi::c_char>();
        }
        if !zFile.is_null()
            && sqlite3_load_extension(db, zFile, zProc, &raw mut zErrMsg) != 0
        {
            sqlite3_result_error(context, zErrMsg, -1 as ::core::ffi::c_int);
            sqlite3_free(zErrMsg as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn kahanBabuskaNeumaierStep(
    mut pSum: *mut SumCtx,
    mut r: ::core::ffi::c_double,
) {
    unsafe {
        let mut s: ::core::ffi::c_double = (*pSum).rSum;
        let mut t: ::core::ffi::c_double = s + r;
        if fabs(s) > fabs(r) {
            ::core::ptr::write_volatile(
                &mut (*pSum).rErr as *mut ::core::ffi::c_double,
                ::core::ptr::read_volatile::<
                    ::core::ffi::c_double,
                >(&(*pSum).rErr as *const ::core::ffi::c_double) + (s - t + r),
            );
        } else {
            ::core::ptr::write_volatile(
                &mut (*pSum).rErr as *mut ::core::ffi::c_double,
                ::core::ptr::read_volatile::<
                    ::core::ffi::c_double,
                >(&(*pSum).rErr as *const ::core::ffi::c_double) + (r - t + s),
            );
        }
        ::core::ptr::write_volatile(&mut (*pSum).rSum as *mut ::core::ffi::c_double, t);
    }
}
unsafe extern "C" fn kahanBabuskaNeumaierStepInt64(
    mut pSum: *mut SumCtx,
    mut iVal: i64_0,
) {
    unsafe {
        if iVal <= -4503599627370496 as ::core::ffi::c_longlong
            || iVal >= 4503599627370496 as ::core::ffi::c_longlong
        {
            let mut iBig: i64_0 = 0;
            let mut iSm: i64_0 = 0;
            iSm = (iVal as ::core::ffi::c_longlong % 16384 as ::core::ffi::c_longlong)
                as i64_0;
            iBig = iVal - iSm;
            kahanBabuskaNeumaierStep(pSum, iBig as ::core::ffi::c_double);
            kahanBabuskaNeumaierStep(pSum, iSm as ::core::ffi::c_double);
        } else {
            kahanBabuskaNeumaierStep(pSum, iVal as ::core::ffi::c_double);
        };
    }
}
unsafe extern "C" fn kahanBabuskaNeumaierInit(mut p: *mut SumCtx, mut iVal: i64_0) {
    unsafe {
        if iVal <= -4503599627370496 as ::core::ffi::c_longlong
            || iVal >= 4503599627370496 as ::core::ffi::c_longlong
        {
            let mut iSm: i64_0 = iVal % 16384 as i64_0;
            ::core::ptr::write_volatile(
                &mut (*p).rSum as *mut ::core::ffi::c_double,
                (iVal - iSm) as ::core::ffi::c_double,
            );
            ::core::ptr::write_volatile(
                &mut (*p).rErr as *mut ::core::ffi::c_double,
                iSm as ::core::ffi::c_double,
            );
        } else {
            ::core::ptr::write_volatile(
                &mut (*p).rSum as *mut ::core::ffi::c_double,
                iVal as ::core::ffi::c_double,
            );
            ::core::ptr::write_volatile(
                &mut (*p).rErr as *mut ::core::ffi::c_double,
                0.0f64,
            );
        };
    }
}
unsafe extern "C" fn sumStep(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut SumCtx = ::core::ptr::null_mut::<SumCtx>();
        let mut type_0: ::core::ffi::c_int = 0;
        p = sqlite3_aggregate_context(
            context,
            ::core::mem::size_of::<SumCtx>() as ::core::ffi::c_int,
        ) as *mut SumCtx;
        type_0 = sqlite3_value_numeric_type(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        if !p.is_null() && type_0 != SQLITE_NULL {
            (*p).cnt += 1;
            if (*p).approx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if type_0 != SQLITE_INTEGER {
                    kahanBabuskaNeumaierInit(p as *mut SumCtx, (*p).iSum);
                    (*p).approx = 1 as u8_0;
                    kahanBabuskaNeumaierStep(
                        p as *mut SumCtx,
                        sqlite3_value_double(
                            *argv.offset(0 as ::core::ffi::c_int as isize),
                        ),
                    );
                } else {
                    let mut x: i64_0 = (*p).iSum;
                    if sqlite3AddInt64(
                        &raw mut x,
                        sqlite3_value_int64(
                            *argv.offset(0 as ::core::ffi::c_int as isize),
                        ) as i64_0,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*p).iSum = x;
                    } else {
                        (*p).ovrfl = 1 as u8_0;
                        kahanBabuskaNeumaierInit(p as *mut SumCtx, (*p).iSum);
                        (*p).approx = 1 as u8_0;
                        kahanBabuskaNeumaierStepInt64(
                            p as *mut SumCtx,
                            sqlite3_value_int64(
                                *argv.offset(0 as ::core::ffi::c_int as isize),
                            ) as i64_0,
                        );
                    }
                }
            } else if type_0 == SQLITE_INTEGER {
                kahanBabuskaNeumaierStepInt64(
                    p as *mut SumCtx,
                    sqlite3_value_int64(*argv.offset(0 as ::core::ffi::c_int as isize))
                        as i64_0,
                );
            } else {
                (*p).ovrfl = 0 as u8_0;
                kahanBabuskaNeumaierStep(
                    p as *mut SumCtx,
                    sqlite3_value_double(*argv.offset(0 as ::core::ffi::c_int as isize)),
                );
            }
        }
    }
}
unsafe extern "C" fn sumInverse(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut SumCtx = ::core::ptr::null_mut::<SumCtx>();
        let mut type_0: ::core::ffi::c_int = 0;
        p = sqlite3_aggregate_context(
            context,
            ::core::mem::size_of::<SumCtx>() as ::core::ffi::c_int,
        ) as *mut SumCtx;
        type_0 = sqlite3_value_numeric_type(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        if !p.is_null() && type_0 != SQLITE_NULL {
            (*p).cnt -= 1;
            if (*p).approx == 0 {
                if sqlite3SubInt64(
                    &raw mut (*p).iSum,
                    sqlite3_value_int64(*argv.offset(0 as ::core::ffi::c_int as isize))
                        as i64_0,
                ) != 0
                {
                    (*p).ovrfl = 1 as u8_0;
                    (*p).approx = 1 as u8_0;
                }
            } else if type_0 == SQLITE_INTEGER {
                let mut iVal: i64_0 = sqlite3_value_int64(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                ) as i64_0;
                if iVal != SMALLEST_INT64 {
                    kahanBabuskaNeumaierStepInt64(p as *mut SumCtx, -iVal);
                } else {
                    kahanBabuskaNeumaierStepInt64(p as *mut SumCtx, LARGEST_INT64);
                    kahanBabuskaNeumaierStepInt64(p as *mut SumCtx, 1 as i64_0);
                }
            } else {
                kahanBabuskaNeumaierStep(
                    p as *mut SumCtx,
                    -sqlite3_value_double(*argv.offset(0 as ::core::ffi::c_int as isize)),
                );
            }
        }
    }
}
unsafe extern "C" fn sumFinalize(mut context: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut SumCtx = ::core::ptr::null_mut::<SumCtx>();
        p = sqlite3_aggregate_context(context, 0 as ::core::ffi::c_int) as *mut SumCtx;
        if !p.is_null() && (*p).cnt > 0 as ::core::ffi::c_longlong {
            if (*p).approx != 0 {
                if (*p).ovrfl != 0 {
                    sqlite3_result_error(
                        context,
                        b"integer overflow\0".as_ptr() as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                } else if sqlite3IsOverflow((*p).rErr) == 0 {
                    sqlite3_result_double(context, (*p).rSum + (*p).rErr);
                } else {
                    sqlite3_result_double(context, (*p).rSum);
                }
            } else {
                sqlite3_result_int64(context, (*p).iSum as sqlite3_int64);
            }
        }
    }
}
unsafe extern "C" fn avgFinalize(mut context: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut SumCtx = ::core::ptr::null_mut::<SumCtx>();
        p = sqlite3_aggregate_context(context, 0 as ::core::ffi::c_int) as *mut SumCtx;
        if !p.is_null() && (*p).cnt > 0 as ::core::ffi::c_longlong {
            let mut r: ::core::ffi::c_double = 0.;
            if (*p).approx != 0 {
                r = (*p).rSum;
                if sqlite3IsOverflow((*p).rErr) == 0 {
                    r += (*p).rErr;
                }
            } else {
                r = (*p).iSum as ::core::ffi::c_double;
            }
            sqlite3_result_double(context, r / (*p).cnt as ::core::ffi::c_double);
        }
    }
}
unsafe extern "C" fn totalFinalize(mut context: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut SumCtx = ::core::ptr::null_mut::<SumCtx>();
        let mut r: ::core::ffi::c_double = 0.0f64;
        p = sqlite3_aggregate_context(context, 0 as ::core::ffi::c_int) as *mut SumCtx;
        if !p.is_null() {
            if (*p).approx != 0 {
                r = (*p).rSum;
                if sqlite3IsOverflow((*p).rErr) == 0 {
                    r += (*p).rErr;
                }
            } else {
                r = (*p).iSum as ::core::ffi::c_double;
            }
        }
        sqlite3_result_double(context, r);
    }
}
unsafe extern "C" fn countStep(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut CountCtx = ::core::ptr::null_mut::<CountCtx>();
        p = sqlite3_aggregate_context(
            context,
            ::core::mem::size_of::<CountCtx>() as ::core::ffi::c_int,
        ) as *mut CountCtx;
        if (argc == 0 as ::core::ffi::c_int
            || SQLITE_NULL
                != sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize)))
            && !p.is_null()
        {
            (*p).n += 1;
        }
    }
}
unsafe extern "C" fn countFinalize(mut context: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut CountCtx = ::core::ptr::null_mut::<CountCtx>();
        p = sqlite3_aggregate_context(context, 0 as ::core::ffi::c_int) as *mut CountCtx;
        sqlite3_result_int64(
            context,
            if !p.is_null() { (*p).n as sqlite3_int64 } else { 0 as sqlite3_int64 },
        );
    }
}
unsafe extern "C" fn countInverse(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut CountCtx = ::core::ptr::null_mut::<CountCtx>();
        p = sqlite3_aggregate_context(
            ctx,
            ::core::mem::size_of::<CountCtx>() as ::core::ffi::c_int,
        ) as *mut CountCtx;
        if (argc == 0 as ::core::ffi::c_int
            || SQLITE_NULL
                != sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize)))
            && !p.is_null()
        {
            (*p).n -= 1;
        }
    }
}
unsafe extern "C" fn minmaxStep(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pArg: *mut Mem = *argv.offset(0 as ::core::ffi::c_int as isize)
            as *mut Mem;
        let mut pBest: *mut Mem = ::core::ptr::null_mut::<Mem>();
        pBest = sqlite3_aggregate_context(
            context,
            ::core::mem::size_of::<Mem>() as ::core::ffi::c_int,
        ) as *mut Mem;
        if pBest.is_null() {
            return;
        }
        if sqlite3_value_type(pArg as *mut sqlite3_value) == SQLITE_NULL {
            if (*pBest).flags != 0 {
                sqlite3SkipAccumulatorLoad(context);
            }
        } else if (*pBest).flags != 0 {
            let mut max: ::core::ffi::c_int = 0;
            let mut cmp: ::core::ffi::c_int = 0;
            let mut pColl: *mut CollSeq = sqlite3GetFuncCollSeq(context);
            max = !sqlite3_user_data(context).is_null() as ::core::ffi::c_int;
            cmp = sqlite3MemCompare(pBest, pArg, pColl);
            if max != 0 && cmp < 0 as ::core::ffi::c_int
                || max == 0 && cmp > 0 as ::core::ffi::c_int
            {
                sqlite3VdbeMemCopy(pBest, pArg);
            } else {
                sqlite3SkipAccumulatorLoad(context);
            }
        } else {
            (*pBest).db = sqlite3_context_db_handle(context);
            sqlite3VdbeMemCopy(pBest, pArg);
        };
    }
}
unsafe extern "C" fn minMaxValueFinalize(
    mut context: *mut sqlite3_context,
    mut bValue: ::core::ffi::c_int,
) {
    unsafe {
        let mut pRes: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        pRes = sqlite3_aggregate_context(context, 0 as ::core::ffi::c_int)
            as *mut sqlite3_value;
        if !pRes.is_null() {
            if (*pRes).flags != 0 {
                sqlite3_result_value(context, pRes);
            }
            if bValue == 0 as ::core::ffi::c_int {
                sqlite3VdbeMemRelease(pRes as *mut Mem);
            }
        }
    }
}
unsafe extern "C" fn minMaxValue(mut context: *mut sqlite3_context) {
    unsafe {
        minMaxValueFinalize(context, 1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn minMaxFinalize(mut context: *mut sqlite3_context) {
    unsafe {
        minMaxValueFinalize(context, 0 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn groupConcatStep(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zVal: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pGCC: *mut GroupConcatCtx = ::core::ptr::null_mut::<GroupConcatCtx>();
        let mut zSep: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nVal: ::core::ffi::c_int = 0;
        let mut nSep: ::core::ffi::c_int = 0;
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            == SQLITE_NULL
        {
            return;
        }
        pGCC = sqlite3_aggregate_context(
            context,
            ::core::mem::size_of::<GroupConcatCtx>() as ::core::ffi::c_int,
        ) as *mut GroupConcatCtx;
        if !pGCC.is_null() {
            let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
            let mut firstTerm: ::core::ffi::c_int = ((*pGCC).str.mxAlloc == 0 as u32_0)
                as ::core::ffi::c_int;
            (*pGCC).str.mxAlloc = (*db).aLimit[SQLITE_LIMIT_LENGTH as usize] as u32_0;
            if argc == 1 as ::core::ffi::c_int {
                if firstTerm == 0 {
                    sqlite3_str_appendchar(
                        &raw mut (*pGCC).str,
                        1 as ::core::ffi::c_int,
                        ',' as i32 as ::core::ffi::c_char,
                    );
                } else {
                    (*pGCC).nFirstSepLength = 1 as ::core::ffi::c_int;
                }
            } else if firstTerm == 0 {
                zSep = sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize))
                    as *mut ::core::ffi::c_char;
                nSep = sqlite3_value_bytes(
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                );
                if !zSep.is_null() {
                    sqlite3_str_append(&raw mut (*pGCC).str, zSep, nSep);
                } else {
                    nSep = 0 as ::core::ffi::c_int;
                }
                if nSep != (*pGCC).nFirstSepLength || !(*pGCC).pnSepLengths.is_null() {
                    let mut pnsl: *mut ::core::ffi::c_int = (*pGCC).pnSepLengths;
                    if pnsl.is_null() {
                        pnsl = sqlite3_malloc64(
                            (((*pGCC).nAccum + 1 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                ) as sqlite3_uint64,
                        ) as *mut ::core::ffi::c_int;
                        if !pnsl.is_null() {
                            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            let mut nA: ::core::ffi::c_int = (*pGCC).nAccum
                                - 1 as ::core::ffi::c_int;
                            while i < nA {
                                let c2rust_fresh8 = i;
                                i = i + 1;
                                *pnsl.offset(c2rust_fresh8 as isize) = (*pGCC)
                                    .nFirstSepLength;
                            }
                        }
                    } else {
                        pnsl = sqlite3_realloc64(
                            pnsl as *mut ::core::ffi::c_void,
                            ((*pGCC).nAccum as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                ) as sqlite3_uint64,
                        ) as *mut ::core::ffi::c_int;
                    }
                    if !pnsl.is_null() {
                        if (*pGCC).nAccum > 0 as ::core::ffi::c_int {
                            *pnsl
                                .offset(
                                    ((*pGCC).nAccum - 1 as ::core::ffi::c_int) as isize,
                                ) = nSep;
                        }
                        (*pGCC).pnSepLengths = pnsl;
                    } else {
                        sqlite3StrAccumSetError(
                            &raw mut (*pGCC).str,
                            SQLITE_NOMEM as u8_0,
                        );
                    }
                }
            } else {
                (*pGCC).nFirstSepLength = sqlite3_value_bytes(
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                );
            }
            (*pGCC).nAccum += 1 as ::core::ffi::c_int;
            zVal = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *mut ::core::ffi::c_char;
            nVal = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
            if !zVal.is_null() {
                sqlite3_str_append(&raw mut (*pGCC).str, zVal, nVal);
            }
        }
    }
}
unsafe extern "C" fn groupConcatInverse(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pGCC: *mut GroupConcatCtx = ::core::ptr::null_mut::<GroupConcatCtx>();
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            == SQLITE_NULL
        {
            return;
        }
        pGCC = sqlite3_aggregate_context(
            context,
            ::core::mem::size_of::<GroupConcatCtx>() as ::core::ffi::c_int,
        ) as *mut GroupConcatCtx;
        if !pGCC.is_null() {
            let mut nVS: ::core::ffi::c_int = 0;
            sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
            nVS = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
            (*pGCC).nAccum -= 1 as ::core::ffi::c_int;
            if !(*pGCC).pnSepLengths.is_null() {
                if (*pGCC).nAccum > 0 as ::core::ffi::c_int {
                    nVS += *(*pGCC).pnSepLengths;
                    memmove(
                        (*pGCC).pnSepLengths as *mut ::core::ffi::c_void,
                        (*pGCC).pnSepLengths.offset(1 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        (((*pGCC).nAccum - 1 as ::core::ffi::c_int) as size_t)
                            .wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                            ),
                    );
                }
            } else {
                nVS += (*pGCC).nFirstSepLength;
            }
            if nVS >= (*pGCC).str.nChar as ::core::ffi::c_int {
                (*pGCC).str.nChar = 0 as u32_0;
            } else {
                (*pGCC).str.nChar = ((*pGCC).str.nChar as ::core::ffi::c_uint)
                    .wrapping_sub(nVS as ::core::ffi::c_uint) as u32_0 as u32_0;
                memmove(
                    (*pGCC).str.zText as *mut ::core::ffi::c_void,
                    (*pGCC).str.zText.offset(nVS as isize) as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    (*pGCC).str.nChar as size_t,
                );
            }
            if (*pGCC).str.nChar == 0 as u32_0 {
                (*pGCC).str.mxAlloc = 0 as u32_0;
                sqlite3_free((*pGCC).pnSepLengths as *mut ::core::ffi::c_void);
                (*pGCC).pnSepLengths = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
        }
    }
}
unsafe extern "C" fn groupConcatFinalize(mut context: *mut sqlite3_context) {
    unsafe {
        let mut pGCC: *mut GroupConcatCtx = sqlite3_aggregate_context(
            context,
            0 as ::core::ffi::c_int,
        ) as *mut GroupConcatCtx;
        if !pGCC.is_null() {
            sqlite3ResultStrAccum(context, &raw mut (*pGCC).str);
            sqlite3_free((*pGCC).pnSepLengths as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn groupConcatValue(mut context: *mut sqlite3_context) {
    unsafe {
        let mut pGCC: *mut GroupConcatCtx = sqlite3_aggregate_context(
            context,
            0 as ::core::ffi::c_int,
        ) as *mut GroupConcatCtx;
        if !pGCC.is_null() {
            let mut pAccum: *mut StrAccum = &raw mut (*pGCC).str;
            if (*pAccum).accError as ::core::ffi::c_int == SQLITE_TOOBIG {
                sqlite3_result_error_toobig(context);
            } else if (*pAccum).accError as ::core::ffi::c_int == SQLITE_NOMEM {
                sqlite3_result_error_nomem(context);
            } else if (*pGCC).nAccum > 0 as ::core::ffi::c_int
                && (*pAccum).nChar == 0 as u32_0
            {
                sqlite3_result_text(
                    context,
                    b"\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            } else {
                let mut zText: *const ::core::ffi::c_char = sqlite3_str_value(
                    pAccum as *mut sqlite3_str,
                );
                sqlite3_result_text(
                    context,
                    zText,
                    (*pAccum).nChar as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RegisterPerConnectionBuiltinFunctions(
    mut db: *mut sqlite3,
) {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_overload_function(
            db,
            b"MATCH\0".as_ptr() as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
        );
        if rc == SQLITE_NOMEM {
            sqlite3OomFault(db);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RegisterLikeFunctions(
    mut db: *mut sqlite3,
    mut caseSensitive: ::core::ffi::c_int,
) {
    unsafe {
        let mut pDef: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
        let mut pInfo: *mut compareInfo = ::core::ptr::null_mut::<compareInfo>();
        let mut flags: ::core::ffi::c_int = 0;
        let mut nArg: ::core::ffi::c_int = 0;
        if caseSensitive != 0 {
            pInfo = &raw const likeInfoAlt as *mut compareInfo;
            flags = SQLITE_FUNC_LIKE | SQLITE_FUNC_CASE;
        } else {
            pInfo = &raw const likeInfoNorm as *mut compareInfo;
            flags = SQLITE_FUNC_LIKE;
        }
        nArg = 2 as ::core::ffi::c_int;
        while nArg <= 3 as ::core::ffi::c_int {
            sqlite3CreateFunc(
                db,
                b"like\0".as_ptr() as *const ::core::ffi::c_char,
                nArg,
                SQLITE_UTF8,
                pInfo as *mut ::core::ffi::c_void,
                Some(
                    likeFunc
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                None,
                None,
                None,
                None,
                ::core::ptr::null_mut::<FuncDestructor>(),
            );
            pDef = sqlite3FindFunction(
                db,
                b"like\0".as_ptr() as *const ::core::ffi::c_char,
                nArg,
                SQLITE_UTF8 as u8_0,
                0 as u8_0,
            );
            (*pDef).funcFlags = ((*pDef).funcFlags as ::core::ffi::c_uint
                | flags as ::core::ffi::c_uint) as u32_0;
            (*pDef).funcFlags = ((*pDef).funcFlags as ::core::ffi::c_uint
                & !SQLITE_FUNC_UNSAFE as ::core::ffi::c_uint) as u32_0;
            nArg += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IsLikeFunction(
    mut db: *mut sqlite3,
    mut pExpr: *mut Expr,
    mut pIsNocase: *mut ::core::ffi::c_int,
    mut aWc: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pDef: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
        let mut nExpr: ::core::ffi::c_int = 0;
        if (*pExpr).x.pList.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        nExpr = (*(*pExpr).x.pList).nExpr;
        pDef = sqlite3FindFunction(
            db,
            (*pExpr).u.zToken,
            nExpr,
            SQLITE_UTF8 as u8_0,
            0 as u8_0,
        );
        if pDef.is_null() || (*pDef).funcFlags & SQLITE_FUNC_LIKE as u32_0 == 0 as u32_0
        {
            return 0 as ::core::ffi::c_int;
        }
        memcpy(aWc as *mut ::core::ffi::c_void, (*pDef).pUserData, 3 as size_t);
        if nExpr < 3 as ::core::ffi::c_int {
            *aWc.offset(3 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
        } else {
            let mut pEscape: *mut Expr = (*(&raw mut (*(*pExpr).x.pList).a
                as *mut ExprList_item)
                .offset(2 as ::core::ffi::c_int as isize))
                .pExpr;
            let mut zEscape: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            if (*pEscape).op as ::core::ffi::c_int != TK_STRING {
                return 0 as ::core::ffi::c_int;
            }
            zEscape = (*pEscape).u.zToken;
            if *zEscape.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                || *zEscape.offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            if *zEscape.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == *aWc.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            if *zEscape.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == *aWc.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            *aWc.offset(3 as ::core::ffi::c_int as isize) = *zEscape
                .offset(0 as ::core::ffi::c_int as isize);
        }
        *pIsNocase = ((*pDef).funcFlags & SQLITE_FUNC_CASE as u32_0 == 0 as u32_0)
            as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn ceilingFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        match sqlite3_value_numeric_type(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) {
            SQLITE_INTEGER => {
                sqlite3_result_int64(
                    context,
                    sqlite3_value_int64(*argv.offset(0 as ::core::ffi::c_int as isize)),
                );
            }
            SQLITE_FLOAT => {
                let mut x: Option<
                    unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double
                > = ::core::mem::transmute::<
                    *mut ::core::ffi::c_void,
                    Option<
                        unsafe extern "C" fn(
                            ::core::ffi::c_double,
                        ) -> ::core::ffi::c_double,
                    >,
                >(sqlite3_user_data(context));
                sqlite3_result_double(
                    context,
                    x
                        .expect(
                            "non-null function pointer",
                        )(
                        sqlite3_value_double(
                            *argv.offset(0 as ::core::ffi::c_int as isize),
                        ),
                    ),
                );
            }
            _ => {}
        };
    }
}
unsafe extern "C" fn xCeil(mut x: ::core::ffi::c_double) -> ::core::ffi::c_double {
    unsafe {
        return ceil(x);
    }
}
unsafe extern "C" fn xFloor(mut x: ::core::ffi::c_double) -> ::core::ffi::c_double {
    unsafe {
        return floor(x);
    }
}
unsafe extern "C" fn logFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut x: ::core::ffi::c_double = 0.;
        let mut b: ::core::ffi::c_double = 0.;
        let mut ans: ::core::ffi::c_double = 0.;
        match sqlite3_value_numeric_type(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) {
            SQLITE_INTEGER | SQLITE_FLOAT => {
                x = sqlite3_value_double(*argv.offset(0 as ::core::ffi::c_int as isize));
                if x <= 0.0f64 {
                    return;
                }
            }
            _ => return

        }
        if argc == 2 as ::core::ffi::c_int {
            match sqlite3_value_numeric_type(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            ) {
                SQLITE_INTEGER | SQLITE_FLOAT => {
                    b = log(x);
                    if b <= 0.0f64 {
                        return;
                    }
                    x = sqlite3_value_double(
                        *argv.offset(1 as ::core::ffi::c_int as isize),
                    );
                    if x <= 0.0f64 {
                        return;
                    }
                }
                _ => return

            }
            ans = log(x) / b;
        } else {
            match sqlite3_user_data(context) as intptr_t as ::core::ffi::c_int {
                1 => {
                    ans = log10(x);
                }
                2 => {
                    ans = log2(x);
                }
                _ => {
                    ans = log(x);
                }
            }
        }
        sqlite3_result_double(context, ans);
    }
}
unsafe extern "C" fn degToRad(mut x: ::core::ffi::c_double) -> ::core::ffi::c_double {
    unsafe {
        return x * (M_PI / 180.0f64);
    }
}
unsafe extern "C" fn radToDeg(mut x: ::core::ffi::c_double) -> ::core::ffi::c_double {
    unsafe {
        return x * (180.0f64 / M_PI);
    }
}
unsafe extern "C" fn math1Func(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut type0: ::core::ffi::c_int = 0;
        let mut v0: ::core::ffi::c_double = 0.;
        let mut ans: ::core::ffi::c_double = 0.;
        let mut x: Option<
            unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double
        > = None;
        type0 = sqlite3_value_numeric_type(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        if type0 != SQLITE_INTEGER && type0 != SQLITE_FLOAT {
            return;
        }
        v0 = sqlite3_value_double(*argv.offset(0 as ::core::ffi::c_int as isize));
        x = ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double>,
        >(sqlite3_user_data(context));
        ans = x.expect("non-null function pointer")(v0);
        sqlite3_result_double(context, ans);
    }
}
unsafe extern "C" fn math2Func(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut type0: ::core::ffi::c_int = 0;
        let mut type1: ::core::ffi::c_int = 0;
        let mut v0: ::core::ffi::c_double = 0.;
        let mut v1: ::core::ffi::c_double = 0.;
        let mut ans: ::core::ffi::c_double = 0.;
        let mut x: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_double,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_double
        > = None;
        type0 = sqlite3_value_numeric_type(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        if type0 != SQLITE_INTEGER && type0 != SQLITE_FLOAT {
            return;
        }
        type1 = sqlite3_value_numeric_type(
            *argv.offset(1 as ::core::ffi::c_int as isize),
        );
        if type1 != SQLITE_INTEGER && type1 != SQLITE_FLOAT {
            return;
        }
        v0 = sqlite3_value_double(*argv.offset(0 as ::core::ffi::c_int as isize));
        v1 = sqlite3_value_double(*argv.offset(1 as ::core::ffi::c_int as isize));
        x = ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            >,
        >(sqlite3_user_data(context));
        ans = x.expect("non-null function pointer")(v0, v1);
        sqlite3_result_double(context, ans);
    }
}
unsafe extern "C" fn piFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_result_double(context, M_PI);
    }
}
unsafe extern "C" fn signFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut type0: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_double = 0.;
        type0 = sqlite3_value_numeric_type(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        if type0 != SQLITE_INTEGER && type0 != SQLITE_FLOAT {
            return;
        }
        x = sqlite3_value_double(*argv.offset(0 as ::core::ffi::c_int as isize));
        sqlite3_result_int(
            context,
            if x < 0.0f64 {
                -1 as ::core::ffi::c_int
            } else if x > 0.0f64 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
    }
}
unsafe extern "C" fn percentIsInfinity(
    mut r: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        let mut u: sqlite3_uint64 = 0;
        memcpy(
            &raw mut u as *mut ::core::ffi::c_void,
            &raw mut r as *const ::core::ffi::c_void,
            ::core::mem::size_of::<sqlite3_uint64>() as size_t,
        );
        return (u as ::core::ffi::c_ulonglong >> 52 as ::core::ffi::c_int
            & 0x7ff as ::core::ffi::c_ulonglong == 0x7ff as ::core::ffi::c_ulonglong)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn percentSameValue(
    mut a: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        a -= b;
        return (a >= -0.001f64 && a <= 0.001f64) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn percentBinarySearch(
    mut p: *mut Percentile,
    mut y: ::core::ffi::c_double,
    mut bExact: ::core::ffi::c_int,
) -> i64_0 {
    unsafe {
        let mut iFirst: i64_0 = 0 as i64_0;
        let mut iLast: i64_0 = (*p).nUsed as i64_0 - 1 as i64_0;
        while iLast >= iFirst {
            let mut iMid: i64_0 = (iFirst + iLast) / 2 as i64_0;
            let mut x: ::core::ffi::c_double = *(*p).a.offset(iMid as isize);
            if x < y {
                iFirst = (iMid as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                    as i64_0;
            } else if x > y {
                iLast = (iMid as ::core::ffi::c_longlong - 1 as ::core::ffi::c_longlong)
                    as i64_0;
            } else {
                return iMid
            }
        }
        if bExact != 0 {
            return -1 as ::core::ffi::c_int as i64_0;
        }
        return iFirst;
    }
}
unsafe extern "C" fn percentError(
    mut pCtx: *mut sqlite3_context,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut zMsg1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zMsg2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        
        let mut ap = c2rust_args;
        zMsg1 = sqlite3_vmprintf(zFormat, ap);
        zMsg2 = if !zMsg1.is_null() {
            sqlite3_mprintf(zMsg1, sqlite3VdbeFuncName(pCtx))
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        sqlite3_result_error(pCtx, zMsg2, -1 as ::core::ffi::c_int);
        sqlite3_free(zMsg1 as *mut ::core::ffi::c_void);
        sqlite3_free(zMsg2 as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn percentStep(
    mut pCtx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut Percentile = ::core::ptr::null_mut::<Percentile>();
        let mut rPct: ::core::ffi::c_double = 0.;
        let mut eType: ::core::ffi::c_int = 0;
        let mut y: ::core::ffi::c_double = 0.;
        if argc == 1 as ::core::ffi::c_int {
            rPct = 0.5f64;
        } else {
            let mut mxFrac: ::core::ffi::c_double = if sqlite3_user_data(pCtx)
                as intptr_t as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0
            {
                100.0f64
            } else {
                1.0f64
            };
            eType = sqlite3_value_numeric_type(
                *argv.offset(1 as ::core::ffi::c_int as isize),
            );
            rPct = sqlite3_value_double(*argv.offset(1 as ::core::ffi::c_int as isize))
                / mxFrac;
            if eType != SQLITE_INTEGER && eType != SQLITE_FLOAT || rPct < 0.0f64
                || rPct > 1.0f64
            {
                percentError(
                    pCtx,
                    b"the fraction argument to %%s() is not between 0.0 and %.1f\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    mxFrac,
                );
                return;
            }
        }
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<Percentile>() as ::core::ffi::c_int,
        ) as *mut Percentile;
        if p.is_null() {
            return;
        }
        if (*p).bPctValid == 0 {
            (*p).rPct = rPct;
            (*p).bPctValid = 1 as ::core::ffi::c_char;
        } else if percentSameValue((*p).rPct, rPct) == 0 {
            percentError(
                pCtx,
                b"the fraction argument to %%s() is not the same for all input rows\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        eType = sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize));
        if eType == SQLITE_NULL {
            return;
        }
        if eType != SQLITE_INTEGER && eType != SQLITE_FLOAT {
            percentError(
                pCtx,
                b"input to %%s() is not numeric\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        y = sqlite3_value_double(*argv.offset(0 as ::core::ffi::c_int as isize));
        if percentIsInfinity(y) != 0 {
            percentError(
                pCtx,
                b"Inf input to %%s()\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if (*p).nUsed >= (*p).nAlloc {
            let mut n: u64_0 = (*p)
                .nAlloc
                .wrapping_mul(2 as u64_0)
                .wrapping_add(250 as u64_0);
            let mut a: *mut ::core::ffi::c_double = sqlite3_realloc64(
                (*p).a as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<::core::ffi::c_double>() as sqlite3_uint64)
                    .wrapping_mul(n as sqlite3_uint64),
            ) as *mut ::core::ffi::c_double;
            if a.is_null() {
                sqlite3_free((*p).a as *mut ::core::ffi::c_void);
                memset(
                    p as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<Percentile>() as size_t,
                );
                sqlite3_result_error_nomem(pCtx);
                return;
            }
            (*p).nAlloc = n;
            (*p).a = a;
        }
        if (*p).nUsed == 0 as ::core::ffi::c_ulonglong {
            let c2rust_fresh5 = (*p).nUsed;
            (*p).nUsed = (*p).nUsed.wrapping_add(1);
            *(*p).a.offset(c2rust_fresh5 as isize) = y;
            (*p).bSorted = 1 as ::core::ffi::c_char;
        } else if (*p).bSorted == 0
            || y
                >= *(*p)
                    .a
                    .offset(
                        ((*p).nUsed as ::core::ffi::c_ulonglong)
                            .wrapping_sub(1 as ::core::ffi::c_ulonglong) as isize,
                    )
        {
            let c2rust_fresh6 = (*p).nUsed;
            (*p).nUsed = (*p).nUsed.wrapping_add(1);
            *(*p).a.offset(c2rust_fresh6 as isize) = y;
        } else if (*p).bKeepSorted != 0 {
            let mut i: i64_0 = 0;
            i = percentBinarySearch(p, y, 0 as ::core::ffi::c_int);
            if i < (*p).nUsed as ::core::ffi::c_int as ::core::ffi::c_longlong {
                memmove(
                    (*p)
                        .a
                        .offset(
                            (i as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                                as isize,
                        ) as *mut ::core::ffi::c_double as *mut ::core::ffi::c_void,
                    (*p).a.offset(i as isize) as *mut ::core::ffi::c_double
                        as *const ::core::ffi::c_void,
                    ((*p).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_sub(i as ::core::ffi::c_ulonglong)
                        .wrapping_mul(
                            ::core::mem::size_of::<::core::ffi::c_double>()
                                as ::core::ffi::c_ulonglong,
                        ) as size_t,
                );
            }
            *(*p).a.offset(i as isize) = y;
            (*p).nUsed = (*p).nUsed.wrapping_add(1);
        } else {
            let c2rust_fresh7 = (*p).nUsed;
            (*p).nUsed = (*p).nUsed.wrapping_add(1);
            *(*p).a.offset(c2rust_fresh7 as isize) = y;
            (*p).bSorted = 0 as ::core::ffi::c_char;
        };
    }
}
unsafe extern "C" fn percentSort(
    mut a: *mut ::core::ffi::c_double,
    mut n: ::core::ffi::c_uint,
) {
    unsafe {
        let mut iLt: ::core::ffi::c_int = 0;
        let mut iGt: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut rPivot: ::core::ffi::c_double = 0.;
        if *a.offset(0 as ::core::ffi::c_int as isize)
            > *a.offset(n.wrapping_sub(1 as ::core::ffi::c_uint) as isize)
        {
            let mut ttt: ::core::ffi::c_double = *a
                .offset(0 as ::core::ffi::c_int as isize);
            *a.offset(0 as ::core::ffi::c_int as isize) = *a
                .offset(n.wrapping_sub(1 as ::core::ffi::c_uint) as isize);
            *a.offset(n.wrapping_sub(1 as ::core::ffi::c_uint) as isize) = ttt;
        }
        if n == 2 as ::core::ffi::c_uint {
            return;
        }
        iGt = n.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        i = n.wrapping_div(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        if *a.offset(0 as ::core::ffi::c_int as isize) > *a.offset(i as isize) {
            let mut ttt_0: ::core::ffi::c_double = *a
                .offset(0 as ::core::ffi::c_int as isize);
            *a.offset(0 as ::core::ffi::c_int as isize) = *a.offset(i as isize);
            *a.offset(i as isize) = ttt_0;
        } else if *a.offset(i as isize) > *a.offset(iGt as isize) {
            let mut ttt_1: ::core::ffi::c_double = *a.offset(i as isize);
            *a.offset(i as isize) = *a.offset(iGt as isize);
            *a.offset(iGt as isize) = ttt_1;
        }
        if n == 3 as ::core::ffi::c_uint {
            return;
        }
        rPivot = *a.offset(i as isize);
        i = 1 as ::core::ffi::c_int;
        iLt = i;
        loop {
            if *a.offset(i as isize) < rPivot {
                if i > iLt {
                    let mut ttt_2: ::core::ffi::c_double = *a.offset(i as isize);
                    *a.offset(i as isize) = *a.offset(iLt as isize);
                    *a.offset(iLt as isize) = ttt_2;
                }
                iLt += 1;
                i += 1;
            } else if *a.offset(i as isize) > rPivot {
                loop {
                    iGt -= 1;
                    if !(iGt > i && *a.offset(iGt as isize) > rPivot) {
                        break;
                    }
                }
                let mut ttt_3: ::core::ffi::c_double = *a.offset(i as isize);
                *a.offset(i as isize) = *a.offset(iGt as isize);
                *a.offset(iGt as isize) = ttt_3;
            } else {
                i += 1;
            }
            if !(i < iGt) {
                break;
            }
        }
        if iLt >= 2 as ::core::ffi::c_int {
            percentSort(a, iLt as ::core::ffi::c_uint);
        }
        if n.wrapping_sub(iGt as ::core::ffi::c_uint) >= 2 as ::core::ffi::c_uint {
            percentSort(
                a.offset(iGt as isize),
                n.wrapping_sub(iGt as ::core::ffi::c_uint),
            );
        }
    }
}
unsafe extern "C" fn percentInverse(
    mut pCtx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut Percentile = ::core::ptr::null_mut::<Percentile>();
        let mut eType: ::core::ffi::c_int = 0;
        let mut y: ::core::ffi::c_double = 0.;
        let mut i: i64_0 = 0;
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<Percentile>() as ::core::ffi::c_int,
        ) as *mut Percentile;
        eType = sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize));
        if eType == SQLITE_NULL {
            return;
        }
        if eType != SQLITE_INTEGER && eType != SQLITE_FLOAT {
            return;
        }
        y = sqlite3_value_double(*argv.offset(0 as ::core::ffi::c_int as isize));
        if percentIsInfinity(y) != 0 {
            return;
        }
        if (*p).bSorted as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            percentSort((*p).a, (*p).nUsed as ::core::ffi::c_uint);
            (*p).bSorted = 1 as ::core::ffi::c_char;
        }
        (*p).bKeepSorted = 1 as ::core::ffi::c_char;
        i = percentBinarySearch(p, y, 1 as ::core::ffi::c_int);
        if i >= 0 as ::core::ffi::c_longlong {
            (*p).nUsed = (*p).nUsed.wrapping_sub(1);
            if i < (*p).nUsed as ::core::ffi::c_int as ::core::ffi::c_longlong {
                memmove(
                    (*p).a.offset(i as isize) as *mut ::core::ffi::c_double
                        as *mut ::core::ffi::c_void,
                    (*p)
                        .a
                        .offset(
                            (i as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                                as isize,
                        ) as *mut ::core::ffi::c_double as *const ::core::ffi::c_void,
                    ((*p).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_sub(i as ::core::ffi::c_ulonglong)
                        .wrapping_mul(
                            ::core::mem::size_of::<::core::ffi::c_double>()
                                as ::core::ffi::c_ulonglong,
                        ) as size_t,
                );
            }
        }
    }
}
unsafe extern "C" fn percentCompute(
    mut pCtx: *mut sqlite3_context,
    mut bIsFinal: ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *mut Percentile = ::core::ptr::null_mut::<Percentile>();
        let mut settings: ::core::ffi::c_int = sqlite3_user_data(pCtx) as intptr_t
            as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
        let mut i1: ::core::ffi::c_uint = 0;
        let mut i2: ::core::ffi::c_uint = 0;
        let mut v1: ::core::ffi::c_double = 0.;
        let mut v2: ::core::ffi::c_double = 0.;
        let mut ix: ::core::ffi::c_double = 0.;
        let mut vx: ::core::ffi::c_double = 0.;
        p = sqlite3_aggregate_context(pCtx, 0 as ::core::ffi::c_int) as *mut Percentile;
        if p.is_null() {
            return;
        }
        if (*p).a.is_null() {
            return;
        }
        if (*p).nUsed != 0 {
            if (*p).bSorted as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                percentSort((*p).a, (*p).nUsed as ::core::ffi::c_uint);
                (*p).bSorted = 1 as ::core::ffi::c_char;
            }
            ix = (*p).rPct
                * ((*p).nUsed as ::core::ffi::c_ulonglong)
                    .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                    as ::core::ffi::c_double;
            i1 = ix as ::core::ffi::c_uint;
            if settings & 1 as ::core::ffi::c_int != 0 {
                vx = *(*p).a.offset(i1 as isize);
            } else {
                i2 = if ix == i1 as ::core::ffi::c_double
                    || i1 as ::core::ffi::c_ulonglong
                        == ((*p).nUsed as ::core::ffi::c_ulonglong)
                            .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                {
                    i1
                } else {
                    i1.wrapping_add(1 as ::core::ffi::c_uint)
                };
                v1 = *(*p).a.offset(i1 as isize);
                v2 = *(*p).a.offset(i2 as isize);
                vx = v1 + (v2 - v1) * (ix - i1 as ::core::ffi::c_double);
            }
            sqlite3_result_double(pCtx, vx);
        }
        if bIsFinal != 0 {
            sqlite3_free((*p).a as *mut ::core::ffi::c_void);
            memset(
                p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Percentile>() as size_t,
            );
        } else {
            (*p).bKeepSorted = 1 as ::core::ffi::c_char;
        };
    }
}
unsafe extern "C" fn percentFinal(mut pCtx: *mut sqlite3_context) {
    unsafe {
        percentCompute(pCtx, 1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn percentValue(mut pCtx: *mut sqlite3_context) {
    unsafe {
        percentCompute(pCtx, 0 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RegisterBuiltinFunctions() {
    unsafe {
        static mut aBuiltinFunc: [FuncDef; 108] = unsafe {
            [
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_INTERNAL
                        | SQLITE_FUNC_TEST | SQLITE_FUNC_INLINE | SQLITE_FUNC_CONSTANT
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: 1 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        versionFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"implies_nonnull_row\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_INTERNAL
                        | SQLITE_FUNC_TEST | SQLITE_FUNC_INLINE | SQLITE_FUNC_CONSTANT
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: 3 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        versionFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"expr_compare\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_INTERNAL
                        | SQLITE_FUNC_TEST | SQLITE_FUNC_INLINE | SQLITE_FUNC_CONSTANT
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: 2 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        versionFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"expr_implies_expr\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_INTERNAL
                        | SQLITE_FUNC_TEST | SQLITE_FUNC_INLINE | SQLITE_FUNC_CONSTANT
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: 4 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        versionFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"affinity\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_DIRECTONLY
                        | SQLITE_FUNC_UNSAFE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        loadExt
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"load_extension\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_DIRECTONLY
                        | SQLITE_FUNC_UNSAFE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        loadExt
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"load_extension\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_SLOCHNG | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        compileoptionusedFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sqlite_compileoption_used\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_SLOCHNG | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        compileoptiongetFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sqlite_compileoption_get\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_INLINE
                        | SQLITE_FUNC_CONSTANT | 0x400 as ::core::ffi::c_int) as u32_0,
                    pUserData: 99 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        versionFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"unlikely\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_INLINE
                        | SQLITE_FUNC_CONSTANT | 0x400 as ::core::ffi::c_int) as u32_0,
                    pUserData: 99 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        versionFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"likelihood\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_INLINE
                        | SQLITE_FUNC_CONSTANT | 0x400 as ::core::ffi::c_int) as u32_0,
                    pUserData: 99 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        versionFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"likely\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: 1 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        trimFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"ltrim\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: 1 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        trimFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"ltrim\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: 2 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        trimFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"rtrim\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: 2 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        trimFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"rtrim\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: 3 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        trimFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"trim\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: 3 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        trimFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"trim\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -3 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        minmaxFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"min\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x1000 as ::core::ffi::c_int | 0x8000000 as ::core::ffi::c_int)
                        as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        minmaxStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        minMaxFinalize
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        minMaxValue as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: None,
                    zName: b"min\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -3 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: 1 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        minmaxFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"max\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x1000 as ::core::ffi::c_int | 0x8000000 as ::core::ffi::c_int)
                        as u32_0,
                    pUserData: 1 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        minmaxStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        minMaxFinalize
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        minMaxValue as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: None,
                    zName: b"max\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x80 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        typeofFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"typeof\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x80 as ::core::ffi::c_int | 0x100000 as ::core::ffi::c_int)
                        as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        subtypeFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"subtype\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x40 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        lengthFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"length\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0xc0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        bytelengthFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"octet_length\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        instrFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"instr\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        printfFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"printf\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        printfFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"format\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        unicodeFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"unicode\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        charFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"char\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        absFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"abs\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        roundFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"round\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        roundFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"round\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        upperFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"upper\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        lowerFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"lower\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        hexFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"hex\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        unhexFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"unhex\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        unhexFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"unhex\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -3 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        concatFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"concat\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -4 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        concatwsFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"concat_ws\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_INLINE
                        | SQLITE_FUNC_CONSTANT | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        versionFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"ifnull\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        randomFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"random\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        randomBlob
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"randomblob\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        nullifFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"nullif\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_SLOCHNG | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        versionFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sqlite_version\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_SLOCHNG | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        sourceidFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sqlite_source_id\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        errlogFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sqlite_log\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        unistrFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"unistr\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        quoteFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"quote\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: 1 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        quoteFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"unistr_quote\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        last_insert_rowid
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"last_insert_rowid\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        changes
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"changes\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        total_changes
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"total_changes\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 3 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        replaceFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"replace\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        zeroblobFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"zeroblob\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        substrFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"substr\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 3 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        substrFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"substr\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        substrFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"substring\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 3 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        substrFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"substring\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        sumStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        sumFinalize as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        sumFinalize as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        sumInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"sum\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        sumStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        totalFinalize as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        totalFinalize as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        sumInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"total\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        sumStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        avgFinalize as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        avgFinalize as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        sumInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"avg\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x100 as ::core::ffi::c_int | 0x8000000 as ::core::ffi::c_int)
                        as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        countStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        countFinalize as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        countFinalize as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        countInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"count\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x8000000 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        countStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        countFinalize as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        countFinalize as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        countInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"count\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        groupConcatStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        groupConcatFinalize
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        groupConcatValue
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        groupConcatInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"group_concat\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        groupConcatStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        groupConcatFinalize
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        groupConcatValue
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        groupConcatInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"group_concat\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        groupConcatStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        groupConcatFinalize
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        groupConcatValue
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        groupConcatInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"string_agg\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x200000 as ::core::ffi::c_int
                        | 0x2000000 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        percentStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        percentFinal as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        percentValue as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        percentInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"median\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x200000 as ::core::ffi::c_int
                        | 0x2000000 as ::core::ffi::c_int) as u32_0,
                    pUserData: 0x2 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        percentStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        percentFinal as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        percentValue as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        percentInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"percentile\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x200000 as ::core::ffi::c_int
                        | 0x2000000 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        percentStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        percentFinal as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        percentValue as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        percentInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"percentile_cont\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x200000 as ::core::ffi::c_int
                        | 0x2000000 as ::core::ffi::c_int) as u32_0,
                    pUserData: 0x1 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        percentStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        percentFinal as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        percentValue as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        percentInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"percentile_disc\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0x4 as ::core::ffi::c_int | 0x8 as ::core::ffi::c_int)
                        as u32_0,
                    pUserData: &raw const globInfo as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        likeFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"glob\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0x4 as ::core::ffi::c_int) as u32_0,
                    pUserData: &raw const likeInfoNorm as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        likeFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"like\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 3 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0x4 as ::core::ffi::c_int) as u32_0,
                    pUserData: &raw const likeInfoNorm as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        likeFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"like\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            xCeil
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        ceilingFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"ceil\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            xCeil
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        ceilingFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"ceiling\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            xFloor
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        ceilingFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"floor\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            trunc
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        ceilingFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"trunc\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        logFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"ln\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: 1 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        logFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"log\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: 1 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        logFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"log10\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: 2 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        logFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"log2\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        logFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"log\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            exp
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"exp\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            pow
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math2Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"pow\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            pow
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math2Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"power\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            fmod
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math2Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"mod\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            acos
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"acos\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            asin
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"asin\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            atan
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"atan\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            atan2
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math2Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"atan2\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            cos
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"cos\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            sin
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sin\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            tan
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"tan\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            cosh
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"cosh\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            sinh
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sinh\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            tanh
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"tanh\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            acosh
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"acosh\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            asinh
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"asinh\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            atanh
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"atanh\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            sqrt
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sqrt\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            degToRad
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"radians\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_double,
                            ) -> ::core::ffi::c_double,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            radToDeg
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_double,
                                ) -> ::core::ffi::c_double,
                        ),
                    ),
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        math1Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"degrees\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8)
                        as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        piFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"pi\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        signFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sign\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -4 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_INLINE
                        | SQLITE_FUNC_CONSTANT | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        versionFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"coalesce\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -4 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_INLINE
                        | SQLITE_FUNC_CONSTANT | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: 5 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        versionFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"iif\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -4 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_INLINE
                        | SQLITE_FUNC_CONSTANT | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: 5 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        versionFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"if\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
            ]
        };
        sqlite3AlterFunctions();
        sqlite3WindowFunctions();
        sqlite3RegisterDateTimeFunctions();
        sqlite3RegisterJsonFunctions();
        sqlite3InsertBuiltinFuncs(
            &raw mut aBuiltinFunc as *mut FuncDef,
            (::core::mem::size_of::<[FuncDef; 108]>() as usize)
                .wrapping_div(::core::mem::size_of::<FuncDef>() as usize)
                as ::core::ffi::c_int,
        );
    }
}
