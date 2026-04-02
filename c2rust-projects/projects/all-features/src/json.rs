use c2rust_bitfields::BitfieldStruct;
use ::libc;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type PreUpdate;
    pub type RenameToken;
    pub type Vdbe;
    pub type TableLock;
    pub type sqlite3_mutex;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_vsnprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_subtype(_: *mut sqlite3_value) -> ::core::ffi::c_uint;
    fn sqlite3_aggregate_context(
        _: *mut sqlite3_context,
        nBytes: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_get_auxdata(
        _: *mut sqlite3_context,
        N: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_set_auxdata(
        _: *mut sqlite3_context,
        N: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_double(_: *mut sqlite3_context, _: ::core::ffi::c_double);
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
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
    fn sqlite3_result_subtype(_: *mut sqlite3_context, _: ::core::ffi::c_uint);
    fn sqlite3_declare_vtab(
        _: *mut sqlite3,
        zSQL: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vtab_config(
        _: *mut sqlite3,
        op: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
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
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strspn(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRaw(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrNDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: u64_0,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbRealloc(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: u64_0,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3IsNaN(_: ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn sqlite3RowSetClear(_: *mut ::core::ffi::c_void);
    fn sqlite3InsertBuiltinFuncs(_: *mut FuncDef, _: ::core::ffi::c_int);
    fn sqlite3AtoF(
        z: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_double,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3Utf8ReadLimited(
        _: *const u8_0,
        _: ::core::ffi::c_int,
        _: *mut u32_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3Atoi64(
        _: *const ::core::ffi::c_char,
        _: *mut i64_0,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3DecOrHexToI64(
        _: *const ::core::ffi::c_char,
        _: *mut i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3HexToInt(h: ::core::ffi::c_int) -> u8_0;
    fn sqlite3ValueIsOfClass(
        _: *const sqlite3_value,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    fn sqlite3RCStrRef(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sqlite3RCStrUnref(_: *mut ::core::ffi::c_void);
    fn sqlite3RCStrNew(_: u64_0) -> *mut ::core::ffi::c_char;
    fn sqlite3RCStrResize(
        _: *mut ::core::ffi::c_char,
        _: u64_0,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3VtabCreateModule(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: *const sqlite3_module,
        _: *mut ::core::ffi::c_void,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> *mut Module;
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
    pub trace: C2Rust_Unnamed_21,
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
    pub u1: C2Rust_Unnamed_17,
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
    pub u: C2Rust_Unnamed_13,
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
    pub fg: C2Rust_Unnamed_12,
    pub iCursor: ::core::ffi::c_int,
    pub colUsed: Bitmask,
    pub u1: C2Rust_Unnamed_11,
    pub u2: C2Rust_Unnamed_10,
    pub u3: C2Rust_Unnamed_9,
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
    pub u: C2Rust_Unnamed_8,
    pub pLeft: *mut Expr,
    pub pRight: *mut Expr,
    pub x: C2Rust_Unnamed_7,
    pub nHeight: ::core::ffi::c_int,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ynVar,
    pub iAgg: i16_0,
    pub w: C2Rust_Unnamed_6,
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
pub type i16_0 = int16_t;
pub type int16_t = __int16_t;
pub type __int16_t = i16;
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
    pub fg: C2Rust_Unnamed_5,
    pub u: C2Rust_Unnamed_3

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub x: C2Rust_Unnamed_4,
    pub iConstExprReg: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub iOrderByCol: u16_0,
    pub iAlias: u16_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
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
pub union C2Rust_Unnamed_6 {
    pub iJoin: ::core::ffi::c_int,
    pub iOfst: ::core::ffi::c_int

}
pub type ynVar = i16_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub pList: *mut ExprList,
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub zToken: *mut ::core::ffi::c_char,
    pub iValue: ::core::ffi::c_int

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
pub union C2Rust_Unnamed_9 {
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
pub union C2Rust_Unnamed_10 {
    pub pIBIndex: *mut Index,
    pub pCteUse: *mut CteUse

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
pub type Pgno = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub zIndexedBy: *mut ::core::ffi::c_char,
    pub pFuncArg: *mut ExprList,
    pub nRow: u32_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
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
pub union C2Rust_Unnamed_13 {
    pub tab: C2Rust_Unnamed_16,
    pub view: C2Rust_Unnamed_15,
    pub vtab: C2Rust_Unnamed_14

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub nArg: ::core::ffi::c_int,
    pub azArg: *mut *mut ::core::ffi::c_char,
    pub p: *mut VTable

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
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
pub union C2Rust_Unnamed_17 {
    pub isInterrupted: ::core::ffi::c_int,
    pub notUsed1: ::core::ffi::c_double

}
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
    pub u1: C2Rust_Unnamed_18,
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
pub struct Token {
    pub z: *const ::core::ffi::c_char,
    pub n: ::core::ffi::c_uint

}
pub type VList = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_18 {
    pub cr: C2Rust_Unnamed_20,
    pub d: C2Rust_Unnamed_19

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
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
pub struct C2Rust_Unnamed_20 {
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
pub type Mem = sqlite3_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AutoincInfo {
    pub pNext: *mut AutoincInfo,
    pub pTab: *mut Table,
    pub iDb: ::core::ffi::c_int,
    pub regCtr: ::core::ffi::c_int

}
pub type yDbMask = ::core::ffi::c_uint;
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
pub type bft = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_21 {
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
pub type intptr_t = isize;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JsonString {
    pub pCtx: *mut sqlite3_context,
    pub zBuf: *mut ::core::ffi::c_char,
    pub nAlloc: u64_0,
    pub nUsed: u64_0,
    pub bStatic: u8_0,
    pub eErr: u8_0,
    pub zSpace: [::core::ffi::c_char; 100]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JsonParse {
    pub aBlob: *mut u8_0,
    pub nBlob: u32_0,
    pub nBlobAlloc: u32_0,
    pub zJson: *mut ::core::ffi::c_char,
    pub db: *mut sqlite3,
    pub nJson: ::core::ffi::c_int,
    pub nJPRef: u32_0,
    pub iErr: u32_0,
    pub iDepth: u16_0,
    pub nErr: u8_0,
    pub oom: u8_0,
    pub bJsonIsRCStr: u8_0,
    pub hasNonstd: u8_0,
    pub bReadOnly: u8_0,
    pub eEdit: u8_0,
    pub delta: ::core::ffi::c_int,
    pub nIns: u32_0,
    pub iLabel: u32_0,
    pub aIns: *mut u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NanInfName {
    pub c1: ::core::ffi::c_char,
    pub c2: ::core::ffi::c_char,
    pub n: ::core::ffi::c_char,
    pub eType: ::core::ffi::c_char,
    pub nRepl: ::core::ffi::c_char,
    pub zMatch: *mut ::core::ffi::c_char,
    pub zRepl: *mut ::core::ffi::c_char

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JsonCache {
    pub db: *mut sqlite3,
    pub nUsed: ::core::ffi::c_int,
    pub a: [*mut JsonParse; 4]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JsonPretty {
    pub pParse: *mut JsonParse,
    pub pOut: *mut JsonString,
    pub zIndent: *const ::core::ffi::c_char,
    pub szIndent: u32_0,
    pub nIndent: u32_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JsonEachCursor {
    pub base: sqlite3_vtab_cursor,
    pub iRowid: u32_0,
    pub i: u32_0,
    pub iEnd: u32_0,
    pub nRoot: u32_0,
    pub eType: u8_0,
    pub bRecursive: u8_0,
    pub eMode: u8_0,
    pub nParent: u32_0,
    pub nParentAlloc: u32_0,
    pub aParent: *mut JsonParent,
    pub db: *mut sqlite3,
    pub path: JsonString,
    pub sParse: JsonParse

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JsonParent {
    pub iHead: u32_0,
    pub iValue: u32_0,
    pub iEnd: u32_0,
    pub nPath: u32_0,
    pub iKey: i64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JsonEachConnection {
    pub base: sqlite3_vtab,
    pub db: *mut sqlite3,
    pub eMode: u8_0,
    pub bRecursive: u8_0

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_SUBTYPE: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const SQLITE_RESULT_SUBTYPE: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_VTAB_INNOCUOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LARGEST_INT64: i64_0 = 0xffffffff as i64_0
    | (0x7fffffff as ::core::ffi::c_int as i64_0) << 32 as ::core::ffi::c_int;
pub const SMALLEST_INT64: i64_0 = -1 as ::core::ffi::c_int as i64_0 - LARGEST_INT64;
pub const JSONB_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSONB_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JSONB_FALSE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const JSONB_INT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const JSONB_INT5: ::core::ffi::c_int = 4;
pub const JSONB_FLOAT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const JSONB_FLOAT5: ::core::ffi::c_int = 6;
pub const JSONB_TEXT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const JSONB_TEXTJ: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const JSONB_TEXT5: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const JSONB_TEXTRAW: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const JSONB_ARRAY: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const JSONB_OBJECT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
static mut jsonbType: [*const ::core::ffi::c_char; 17] = [
    b"null\0".as_ptr() as *const ::core::ffi::c_char,
    b"true\0".as_ptr() as *const ::core::ffi::c_char,
    b"false\0".as_ptr() as *const ::core::ffi::c_char,
    b"integer\0".as_ptr() as *const ::core::ffi::c_char,
    b"integer\0".as_ptr() as *const ::core::ffi::c_char,
    b"real\0".as_ptr() as *const ::core::ffi::c_char,
    b"real\0".as_ptr() as *const ::core::ffi::c_char,
    b"text\0".as_ptr() as *const ::core::ffi::c_char,
    b"text\0".as_ptr() as *const ::core::ffi::c_char,
    b"text\0".as_ptr() as *const ::core::ffi::c_char,
    b"text\0".as_ptr() as *const ::core::ffi::c_char,
    b"array\0".as_ptr() as *const ::core::ffi::c_char,
    b"object\0".as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut jsonIsSpace: [::core::ffi::c_char; 256] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
];
static mut jsonSpaces: [::core::ffi::c_char; 5] = unsafe {
    ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"\t\n\r \0")
};
static mut jsonIsOk: [::core::ffi::c_char; 256] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
];
pub const JSON_CACHE_ID: ::core::ffi::c_int = -429938 as ::core::ffi::c_int;
pub const JSON_CACHE_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const JSON_INVALID_CHAR: ::core::ffi::c_int = 0x99999 as ::core::ffi::c_int;
pub const JSTRING_OOM: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const JSTRING_MALFORMED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const JSTRING_ERR: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const JSON_SUBTYPE: ::core::ffi::c_int = 74 as ::core::ffi::c_int;
pub const JSON_JSON: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const JSON_SQL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const JSON_ABPATH: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
pub const JSON_ISSET: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const JSON_BLOB: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const JEDIT_DEL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JEDIT_REPL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const JEDIT_INS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const JEDIT_SET: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const JSON_MAX_DEPTH: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const JSON_EDITABLE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const JSON_KEEPERROR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
unsafe extern "C" fn jsonCacheDelete(mut p: *mut JsonCache) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nUsed {
            jsonParseFree((*p).a[i as usize]);
            i += 1;
        }
        sqlite3DbFree((*p).db, p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn jsonCacheDeleteGeneric(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        jsonCacheDelete(p as *mut JsonCache);
    }
}
unsafe extern "C" fn jsonCacheInsert(
    mut ctx: *mut sqlite3_context,
    mut pParse: *mut JsonParse,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut JsonCache = ::core::ptr::null_mut::<JsonCache>();
        p = sqlite3_get_auxdata(ctx, JSON_CACHE_ID) as *mut JsonCache;
        if p.is_null() {
            let mut db: *mut sqlite3 = sqlite3_context_db_handle(ctx);
            p = sqlite3DbMallocZero(db, ::core::mem::size_of::<JsonCache>() as u64_0)
                as *mut JsonCache;
            if p.is_null() {
                return SQLITE_NOMEM;
            }
            (*p).db = db;
            sqlite3_set_auxdata(
                ctx,
                JSON_CACHE_ID,
                p as *mut ::core::ffi::c_void,
                Some(
                    jsonCacheDeleteGeneric
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
            p = sqlite3_get_auxdata(ctx, JSON_CACHE_ID) as *mut JsonCache;
            if p.is_null() {
                return SQLITE_NOMEM;
            }
        }
        if (*p).nUsed >= JSON_CACHE_SIZE {
            jsonParseFree((*p).a[0 as ::core::ffi::c_int as usize]);
            memmove(
                &raw mut (*p).a as *mut *mut JsonParse as *mut ::core::ffi::c_void,
                (&raw mut (*p).a as *mut *mut JsonParse)
                    .offset(1 as ::core::ffi::c_int as isize) as *mut *mut JsonParse
                    as *const ::core::ffi::c_void,
                ((JSON_CACHE_SIZE - 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut JsonParse>() as size_t),
            );
            (*p).nUsed = JSON_CACHE_SIZE - 1 as ::core::ffi::c_int;
        }
        (*pParse).eEdit = 0 as u8_0;
        (*pParse).nJPRef = (*pParse).nJPRef.wrapping_add(1);
        (*pParse).bReadOnly = 1 as u8_0;
        (*p).a[(*p).nUsed as usize] = pParse;
        (*p).nUsed += 1;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn jsonCacheSearch(
    mut ctx: *mut sqlite3_context,
    mut pArg: *mut sqlite3_value,
) -> *mut JsonParse {
    unsafe {
        let mut p: *mut JsonCache = ::core::ptr::null_mut::<JsonCache>();
        let mut i: ::core::ffi::c_int = 0;
        let mut zJson: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nJson: ::core::ffi::c_int = 0;
        if sqlite3_value_type(pArg) != SQLITE_TEXT {
            return ::core::ptr::null_mut::<JsonParse>();
        }
        zJson = sqlite3_value_text(pArg) as *const ::core::ffi::c_char;
        if zJson.is_null() {
            return ::core::ptr::null_mut::<JsonParse>();
        }
        nJson = sqlite3_value_bytes(pArg);
        p = sqlite3_get_auxdata(ctx, JSON_CACHE_ID) as *mut JsonCache;
        if p.is_null() {
            return ::core::ptr::null_mut::<JsonParse>();
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nUsed {
            if (*(*p).a[i as usize]).zJson == zJson as *mut ::core::ffi::c_char {
                break;
            }
            i += 1;
        }
        if i >= (*p).nUsed {
            i = 0 as ::core::ffi::c_int;
            while i < (*p).nUsed {
                if !((*(*p).a[i as usize]).nJson != nJson) {
                    if memcmp(
                        (*(*p).a[i as usize]).zJson as *const ::core::ffi::c_void,
                        zJson as *const ::core::ffi::c_void,
                        nJson as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                i += 1;
            }
        }
        if i < (*p).nUsed {
            if i < (*p).nUsed - 1 as ::core::ffi::c_int {
                let mut tmp: *mut JsonParse = (*p).a[i as usize];
                memmove(
                    (&raw mut (*p).a as *mut *mut JsonParse).offset(i as isize)
                        as *mut *mut JsonParse as *mut ::core::ffi::c_void,
                    (&raw mut (*p).a as *mut *mut JsonParse)
                        .offset((i + 1 as ::core::ffi::c_int) as isize)
                        as *mut *mut JsonParse as *const ::core::ffi::c_void,
                    (((*p).nUsed - i - 1 as ::core::ffi::c_int) as size_t)
                        .wrapping_mul(::core::mem::size_of::<*mut JsonParse>() as size_t),
                );
                (*p).a[((*p).nUsed - 1 as ::core::ffi::c_int) as usize] = tmp;
                i = (*p).nUsed - 1 as ::core::ffi::c_int;
            }
            return (*p).a[i as usize];
        } else {
            return ::core::ptr::null_mut::<JsonParse>()
        };
    }
}
unsafe extern "C" fn jsonStringZero(mut p: *mut JsonString) {
    unsafe {
        (*p).zBuf = &raw mut (*p).zSpace as *mut ::core::ffi::c_char;
        (*p).nAlloc = ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as u64_0;
        (*p).nUsed = 0 as u64_0;
        (*p).bStatic = 1 as u8_0;
    }
}
unsafe extern "C" fn jsonStringInit(
    mut p: *mut JsonString,
    mut pCtx: *mut sqlite3_context,
) {
    unsafe {
        (*p).pCtx = pCtx;
        (*p).eErr = 0 as u8_0;
        jsonStringZero(p);
    }
}
unsafe extern "C" fn jsonStringReset(mut p: *mut JsonString) {
    unsafe {
        if (*p).bStatic == 0 {
            sqlite3RCStrUnref((*p).zBuf as *mut ::core::ffi::c_void);
        }
        jsonStringZero(p);
    }
}
unsafe extern "C" fn jsonStringOom(mut p: *mut JsonString) {
    unsafe {
        (*p).eErr = ((*p).eErr as ::core::ffi::c_int | JSTRING_OOM) as u8_0;
        if !(*p).pCtx.is_null() {
            sqlite3_result_error_nomem((*p).pCtx);
        }
        jsonStringReset(p);
    }
}
unsafe extern "C" fn jsonStringGrow(
    mut p: *mut JsonString,
    mut N: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nTotal: u64_0 = if (N as ::core::ffi::c_ulonglong) < (*p).nAlloc {
            (*p).nAlloc.wrapping_mul(2 as u64_0)
        } else {
            (*p).nAlloc.wrapping_add(N as u64_0).wrapping_add(10 as u64_0)
        };
        let mut zNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if (*p).bStatic != 0 {
            if (*p).eErr != 0 {
                return 1 as ::core::ffi::c_int;
            }
            zNew = sqlite3RCStrNew(nTotal);
            if zNew.is_null() {
                jsonStringOom(p);
                return SQLITE_NOMEM;
            }
            memcpy(
                zNew as *mut ::core::ffi::c_void,
                (*p).zBuf as *const ::core::ffi::c_void,
                (*p).nUsed as size_t,
            );
            (*p).zBuf = zNew;
            (*p).bStatic = 0 as u8_0;
        } else {
            (*p).zBuf = sqlite3RCStrResize((*p).zBuf, nTotal);
            if (*p).zBuf.is_null() {
                (*p).eErr = ((*p).eErr as ::core::ffi::c_int | JSTRING_OOM) as u8_0;
                jsonStringZero(p);
                return SQLITE_NOMEM;
            }
        }
        (*p).nAlloc = nTotal;
        return SQLITE_OK;
    }
}
#[inline(never)]
unsafe extern "C" fn jsonStringExpandAndAppend(
    mut p: *mut JsonString,
    mut zIn: *const ::core::ffi::c_char,
    mut N: u32_0,
) {
    unsafe {
        if jsonStringGrow(p, N) != 0 {
            return;
        }
        memcpy(
            (*p).zBuf.offset((*p).nUsed as isize) as *mut ::core::ffi::c_void,
            zIn as *const ::core::ffi::c_void,
            N as size_t,
        );
        (*p).nUsed = ((*p).nUsed as ::core::ffi::c_ulonglong)
            .wrapping_add(N as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
    }
}
unsafe extern "C" fn jsonAppendRaw(
    mut p: *mut JsonString,
    mut zIn: *const ::core::ffi::c_char,
    mut N: u32_0,
) {
    unsafe {
        if N == 0 as u32_0 {
            return;
        }
        if (N as u64_0).wrapping_add((*p).nUsed) >= (*p).nAlloc {
            jsonStringExpandAndAppend(p, zIn, N);
        } else {
            memcpy(
                (*p).zBuf.offset((*p).nUsed as isize) as *mut ::core::ffi::c_void,
                zIn as *const ::core::ffi::c_void,
                N as size_t,
            );
            (*p).nUsed = ((*p).nUsed as ::core::ffi::c_ulonglong)
                .wrapping_add(N as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
        };
    }
}
unsafe extern "C" fn jsonAppendRawNZ(
    mut p: *mut JsonString,
    mut zIn: *const ::core::ffi::c_char,
    mut N: u32_0,
) {
    unsafe {
        if (N as u64_0).wrapping_add((*p).nUsed) >= (*p).nAlloc {
            jsonStringExpandAndAppend(p, zIn, N);
        } else {
            memcpy(
                (*p).zBuf.offset((*p).nUsed as isize) as *mut ::core::ffi::c_void,
                zIn as *const ::core::ffi::c_void,
                N as size_t,
            );
            (*p).nUsed = ((*p).nUsed as ::core::ffi::c_ulonglong)
                .wrapping_add(N as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
        };
    }
}
unsafe extern "C" fn jsonPrintf(
    mut N: ::core::ffi::c_int,
    mut p: *mut JsonString,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        
        if ((*p).nUsed as ::core::ffi::c_ulonglong)
            .wrapping_add(N as ::core::ffi::c_ulonglong) >= (*p).nAlloc
            && jsonStringGrow(p, N as u32_0) != 0
        {
            return;
        }
        let mut ap = c2rust_args;
        sqlite3_vsnprintf(
            N,
            (*p).zBuf.offset((*p).nUsed as isize),
            zFormat,
            ap,
        );
        (*p).nUsed = ((*p).nUsed as ::core::ffi::c_ulonglong)
            .wrapping_add(
                strlen((*p).zBuf.offset((*p).nUsed as isize)) as ::core::ffi::c_int
                    as ::core::ffi::c_ulonglong,
            ) as u64_0 as u64_0;
    }
}
#[inline(never)]
unsafe extern "C" fn jsonAppendCharExpand(
    mut p: *mut JsonString,
    mut c: ::core::ffi::c_char,
) {
    unsafe {
        if jsonStringGrow(p, 1 as u32_0) != 0 {
            return;
        }
        let c2rust_fresh3 = (*p).nUsed;
        (*p).nUsed = (*p).nUsed.wrapping_add(1);
        *(*p).zBuf.offset(c2rust_fresh3 as isize) = c;
    }
}
unsafe extern "C" fn jsonAppendChar(mut p: *mut JsonString, mut c: ::core::ffi::c_char) {
    unsafe {
        if (*p).nUsed >= (*p).nAlloc {
            jsonAppendCharExpand(p, c);
        } else {
            let c2rust_fresh2 = (*p).nUsed;
            (*p).nUsed = (*p).nUsed.wrapping_add(1);
            *(*p).zBuf.offset(c2rust_fresh2 as isize) = c;
        };
    }
}
unsafe extern "C" fn jsonStringTrimOneChar(mut p: *mut JsonString) {
    unsafe {
        if (*p).eErr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*p).nUsed = (*p).nUsed.wrapping_sub(1);
        }
    }
}
unsafe extern "C" fn jsonStringTerminate(mut p: *mut JsonString) -> ::core::ffi::c_int {
    unsafe {
        jsonAppendChar(p, 0 as ::core::ffi::c_char);
        jsonStringTrimOneChar(p);
        return ((*p).eErr as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn jsonAppendSeparator(mut p: *mut JsonString) {
    unsafe {
        let mut c: ::core::ffi::c_char = 0;
        if (*p).nUsed == 0 as ::core::ffi::c_ulonglong {
            return;
        }
        c = *(*p)
            .zBuf
            .offset(
                ((*p).nUsed as ::core::ffi::c_ulonglong)
                    .wrapping_sub(1 as ::core::ffi::c_ulonglong) as isize,
            );
        if c as ::core::ffi::c_int == '[' as i32 || c as ::core::ffi::c_int == '{' as i32
        {
            return;
        }
        jsonAppendChar(p, ',' as i32 as ::core::ffi::c_char);
    }
}
unsafe extern "C" fn jsonAppendControlChar(mut p: *mut JsonString, mut c: u8_0) {
    unsafe {
        static mut aSpecial: [::core::ffi::c_char; 32] = [
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            'b' as i32 as ::core::ffi::c_char,
            't' as i32 as ::core::ffi::c_char,
            'n' as i32 as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            'f' as i32 as ::core::ffi::c_char,
            'r' as i32 as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
        ];
        if aSpecial[c as usize] != 0 {
            *(*p).zBuf.offset((*p).nUsed as isize) = '\\' as i32 as ::core::ffi::c_char;
            *(*p)
                .zBuf
                .offset(
                    ((*p).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_add(1 as ::core::ffi::c_ulonglong) as isize,
                ) = aSpecial[c as usize];
            (*p).nUsed = ((*p).nUsed as ::core::ffi::c_ulonglong)
                .wrapping_add(2 as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
        } else {
            *(*p).zBuf.offset((*p).nUsed as isize) = '\\' as i32 as ::core::ffi::c_char;
            *(*p)
                .zBuf
                .offset(
                    ((*p).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_add(1 as ::core::ffi::c_ulonglong) as isize,
                ) = 'u' as i32 as ::core::ffi::c_char;
            *(*p)
                .zBuf
                .offset(
                    ((*p).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_add(2 as ::core::ffi::c_ulonglong) as isize,
                ) = '0' as i32 as ::core::ffi::c_char;
            *(*p)
                .zBuf
                .offset(
                    ((*p).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_add(3 as ::core::ffi::c_ulonglong) as isize,
                ) = '0' as i32 as ::core::ffi::c_char;
            *(*p)
                .zBuf
                .offset(
                    ((*p).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_add(4 as ::core::ffi::c_ulonglong) as isize,
                ) = ::core::mem::transmute::<
                [u8; 17],
                [::core::ffi::c_char; 17],
            >(
                *b"0123456789abcdef\0",
            )[(c as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as usize];
            *(*p)
                .zBuf
                .offset(
                    ((*p).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_add(5 as ::core::ffi::c_ulonglong) as isize,
                ) = ::core::mem::transmute::<
                [u8; 17],
                [::core::ffi::c_char; 17],
            >(
                *b"0123456789abcdef\0",
            )[(c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize];
            (*p).nUsed = ((*p).nUsed as ::core::ffi::c_ulonglong)
                .wrapping_add(6 as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
        };
    }
}
unsafe extern "C" fn jsonAppendString(
    mut p: *mut JsonString,
    mut zIn: *const ::core::ffi::c_char,
    mut N: u32_0,
) {
    unsafe {
        let mut k: u32_0 = 0;
        let mut c: u8_0 = 0;
        let mut z: *const u8_0 = zIn as *const u8_0;
        if z.is_null() {
            return;
        }
        if (N as ::core::ffi::c_ulonglong)
            .wrapping_add((*p).nUsed as ::core::ffi::c_ulonglong)
            .wrapping_add(2 as ::core::ffi::c_ulonglong) >= (*p).nAlloc
            && jsonStringGrow(p, N.wrapping_add(2 as u32_0)) != 0 as ::core::ffi::c_int
        {
            return;
        }
        let c2rust_fresh5 = (*p).nUsed;
        (*p).nUsed = (*p).nUsed.wrapping_add(1);
        *(*p).zBuf.offset(c2rust_fresh5 as isize) = '"' as i32 as ::core::ffi::c_char;
        loop {
            k = 0 as u32_0;
            loop {
                if k.wrapping_add(3 as u32_0) >= N {
                    while k < N
                        && jsonIsOk[*z.offset(k as isize) as usize] as ::core::ffi::c_int
                            != 0
                    {
                        k = k.wrapping_add(1);
                    }
                    break;
                } else {
                    if jsonIsOk[*z.offset(k as isize) as usize] == 0 {
                        break;
                    }
                    if jsonIsOk[*z.offset(k.wrapping_add(1 as u32_0) as isize) as usize]
                        == 0
                    {
                        k = (k as ::core::ffi::c_uint)
                            .wrapping_add(1 as ::core::ffi::c_uint) as u32_0 as u32_0;
                        break;
                    } else if jsonIsOk[*z.offset(k.wrapping_add(2 as u32_0) as isize)
                        as usize] == 0
                    {
                        k = (k as ::core::ffi::c_uint)
                            .wrapping_add(2 as ::core::ffi::c_uint) as u32_0 as u32_0;
                        break;
                    } else if jsonIsOk[*z.offset(k.wrapping_add(3 as u32_0) as isize)
                        as usize] == 0
                    {
                        k = (k as ::core::ffi::c_uint)
                            .wrapping_add(3 as ::core::ffi::c_uint) as u32_0 as u32_0;
                        break;
                    } else {
                        k = (k as ::core::ffi::c_uint)
                            .wrapping_add(4 as ::core::ffi::c_uint) as u32_0 as u32_0;
                    }
                }
            }
            if k >= N {
                if k > 0 as u32_0 {
                    memcpy(
                        (*p).zBuf.offset((*p).nUsed as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        z as *const ::core::ffi::c_void,
                        k as size_t,
                    );
                    (*p).nUsed = ((*p).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_add(k as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
                }
                break;
            } else {
                if k > 0 as u32_0 {
                    memcpy(
                        (*p).zBuf.offset((*p).nUsed as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        z as *const ::core::ffi::c_void,
                        k as size_t,
                    );
                    (*p).nUsed = ((*p).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_add(k as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
                    z = z.offset(k as isize);
                    N = (N as ::core::ffi::c_uint).wrapping_sub(k as ::core::ffi::c_uint)
                        as u32_0 as u32_0;
                }
                c = *z.offset(0 as ::core::ffi::c_int as isize);
                if c as ::core::ffi::c_int == '"' as i32
                    || c as ::core::ffi::c_int == '\\' as i32
                {
                    if ((*p).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_add(N as ::core::ffi::c_ulonglong)
                        .wrapping_add(3 as ::core::ffi::c_ulonglong) > (*p).nAlloc
                        && jsonStringGrow(p, N.wrapping_add(3 as u32_0))
                            != 0 as ::core::ffi::c_int
                    {
                        return;
                    }
                    let c2rust_fresh6 = (*p).nUsed;
                    (*p).nUsed = (*p).nUsed.wrapping_add(1);
                    *(*p).zBuf.offset(c2rust_fresh6 as isize) = '\\' as i32
                        as ::core::ffi::c_char;
                    let c2rust_fresh7 = (*p).nUsed;
                    (*p).nUsed = (*p).nUsed.wrapping_add(1);
                    *(*p).zBuf.offset(c2rust_fresh7 as isize) = c as ::core::ffi::c_char;
                } else if c as ::core::ffi::c_int == '\'' as i32 {
                    let c2rust_fresh8 = (*p).nUsed;
                    (*p).nUsed = (*p).nUsed.wrapping_add(1);
                    *(*p).zBuf.offset(c2rust_fresh8 as isize) = c as ::core::ffi::c_char;
                } else {
                    if ((*p).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_add(N as ::core::ffi::c_ulonglong)
                        .wrapping_add(7 as ::core::ffi::c_ulonglong) > (*p).nAlloc
                        && jsonStringGrow(p, N.wrapping_add(7 as u32_0))
                            != 0 as ::core::ffi::c_int
                    {
                        return;
                    }
                    jsonAppendControlChar(p, c);
                }
                z = z.offset(1);
                N = N.wrapping_sub(1);
            }
        }
        let c2rust_fresh9 = (*p).nUsed;
        (*p).nUsed = (*p).nUsed.wrapping_add(1);
        *(*p).zBuf.offset(c2rust_fresh9 as isize) = '"' as i32 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn jsonAppendSqlValue(
    mut p: *mut JsonString,
    mut pValue: *mut sqlite3_value,
) {
    unsafe {
        match sqlite3_value_type(pValue) {
            SQLITE_NULL => {
                jsonAppendRawNZ(
                    p,
                    b"null\0".as_ptr() as *const ::core::ffi::c_char,
                    4 as u32_0,
                );
            }
            SQLITE_FLOAT => {
                jsonPrintf(
                    100 as ::core::ffi::c_int,
                    p,
                    b"%!0.15g\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_value_double(pValue),
                );
            }
            SQLITE_INTEGER => {
                let mut z: *const ::core::ffi::c_char = sqlite3_value_text(pValue)
                    as *const ::core::ffi::c_char;
                let mut n: u32_0 = sqlite3_value_bytes(pValue) as u32_0;
                jsonAppendRaw(p, z, n);
            }
            SQLITE_TEXT => {
                let mut z_0: *const ::core::ffi::c_char = sqlite3_value_text(pValue)
                    as *const ::core::ffi::c_char;
                let mut n_0: u32_0 = sqlite3_value_bytes(pValue) as u32_0;
                if sqlite3_value_subtype(pValue) == JSON_SUBTYPE as ::core::ffi::c_uint {
                    jsonAppendRaw(p, z_0, n_0);
                } else {
                    jsonAppendString(p, z_0, n_0);
                }
            }
            _ => {
                let mut px: JsonParse = JsonParse {
                    aBlob: ::core::ptr::null_mut::<u8_0>(),
                    nBlob: 0,
                    nBlobAlloc: 0,
                    zJson: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    db: ::core::ptr::null_mut::<sqlite3>(),
                    nJson: 0,
                    nJPRef: 0,
                    iErr: 0,
                    iDepth: 0,
                    nErr: 0,
                    oom: 0,
                    bJsonIsRCStr: 0,
                    hasNonstd: 0,
                    bReadOnly: 0,
                    eEdit: 0,
                    delta: 0,
                    nIns: 0,
                    iLabel: 0,
                    aIns: ::core::ptr::null_mut::<u8_0>()

                };
                memset(
                    &raw mut px as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<JsonParse>() as size_t,
                );
                if jsonArgIsJsonb(pValue, &raw mut px) != 0 {
                    jsonTranslateBlobToText(&raw mut px, 0 as u32_0, p);
                } else if (*p).eErr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    sqlite3_result_error(
                        (*p).pCtx,
                        b"JSON cannot hold BLOB values\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                    (*p).eErr = JSTRING_ERR as u8_0;
                    jsonStringReset(p);
                }
            }
        };
    }
}
unsafe extern "C" fn jsonReturnString(
    mut p: *mut JsonString,
    mut pParse: *mut JsonParse,
    mut ctx: *mut sqlite3_context,
) {
    unsafe {
        if (*p).eErr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut flags: ::core::ffi::c_int = sqlite3_user_data((*p).pCtx) as intptr_t
                as ::core::ffi::c_int;
            if flags & JSON_BLOB != 0 {
                jsonReturnStringAsBlob(p);
            } else if (*p).bStatic != 0 {
                sqlite3_result_text64(
                    (*p).pCtx,
                    (*p).zBuf,
                    (*p).nUsed as sqlite3_uint64,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    SQLITE_UTF8 as ::core::ffi::c_uchar,
                );
            } else if jsonStringTerminate(p) != 0 {
                if !pParse.is_null()
                    && (*pParse).bJsonIsRCStr as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int && (*pParse).nBlobAlloc > 0 as u32_0
                {
                    let mut rc: ::core::ffi::c_int = 0;
                    (*pParse).zJson = sqlite3RCStrRef((*p).zBuf);
                    (*pParse).nJson = (*p).nUsed as ::core::ffi::c_int;
                    (*pParse).bJsonIsRCStr = 1 as u8_0;
                    rc = jsonCacheInsert(ctx, pParse);
                    if rc == SQLITE_NOMEM {
                        sqlite3_result_error_nomem(ctx);
                        jsonStringReset(p);
                        return;
                    }
                }
                sqlite3_result_text64(
                    (*p).pCtx,
                    sqlite3RCStrRef((*p).zBuf),
                    (*p).nUsed as sqlite3_uint64,
                    Some(
                        sqlite3RCStrUnref
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                    SQLITE_UTF8 as ::core::ffi::c_uchar,
                );
            } else {
                sqlite3_result_error_nomem((*p).pCtx);
            }
        } else if (*p).eErr as ::core::ffi::c_int & JSTRING_OOM != 0 {
            sqlite3_result_error_nomem((*p).pCtx);
        } else if (*p).eErr as ::core::ffi::c_int & JSTRING_MALFORMED != 0 {
            sqlite3_result_error(
                (*p).pCtx,
                b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            );
        }
        jsonStringReset(p);
    }
}
unsafe extern "C" fn jsonParseReset(mut pParse: *mut JsonParse) {
    unsafe {
        if (*pParse).bJsonIsRCStr != 0 {
            sqlite3RCStrUnref((*pParse).zJson as *mut ::core::ffi::c_void);
            (*pParse).zJson = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*pParse).nJson = 0 as ::core::ffi::c_int;
            (*pParse).bJsonIsRCStr = 0 as u8_0;
        }
        if (*pParse).nBlobAlloc != 0 {
            sqlite3DbFree((*pParse).db, (*pParse).aBlob as *mut ::core::ffi::c_void);
            (*pParse).aBlob = ::core::ptr::null_mut::<u8_0>();
            (*pParse).nBlob = 0 as u32_0;
            (*pParse).nBlobAlloc = 0 as u32_0;
        }
    }
}
unsafe extern "C" fn jsonParseFree(mut pParse: *mut JsonParse) {
    unsafe {
        if !pParse.is_null() {
            if (*pParse).nJPRef > 1 as u32_0 {
                (*pParse).nJPRef = (*pParse).nJPRef.wrapping_sub(1);
            } else {
                jsonParseReset(pParse);
                sqlite3DbFree((*pParse).db, pParse as *mut ::core::ffi::c_void);
            }
        }
    }
}
unsafe extern "C" fn jsonHexToInt(mut h: ::core::ffi::c_int) -> u8_0 {
    unsafe {
        h
            += 9 as ::core::ffi::c_int
                * (1 as ::core::ffi::c_int & h >> 6 as ::core::ffi::c_int);
        return (h & 0xf as ::core::ffi::c_int) as u8_0;
    }
}
unsafe extern "C" fn jsonHexToInt4(mut z: *const ::core::ffi::c_char) -> u32_0 {
    unsafe {
        let mut v: u32_0 = 0;
        v = (((jsonHexToInt(
            *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
            + ((jsonHexToInt(
                *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            + ((jsonHexToInt(
                *z.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            + jsonHexToInt(
                *z.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_int) as u32_0;
        return v;
    }
}
unsafe extern "C" fn jsonIs2Hex(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(
                *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                    as isize,
            ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
            && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(
                    *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                        as isize,
                ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn jsonIs4Hex(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (jsonIs2Hex(z) != 0
            && jsonIs2Hex(
                z.offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
            ) != 0) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn json5Whitespace(
    mut zIn: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut z: *const u8_0 = zIn as *mut u8_0;
        's_8: loop {
            match *z.offset(n as isize) as ::core::ffi::c_int {
                9 | 10 | 11 | 12 | 13 | 32 => {
                    n += 1;
                }
                47 => {
                    if *z.offset((n + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == '*' as i32
                        && *z.offset((n + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    {
                        let mut j: ::core::ffi::c_int = 0;
                        j = n + 3 as ::core::ffi::c_int;
                        while *z.offset(j as isize) as ::core::ffi::c_int != '/' as i32
                            || *z.offset((j - 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int != '*' as i32
                        {
                            if *z.offset(j as isize) as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                break 's_8;
                            }
                            j += 1;
                        }
                        n = j + 1 as ::core::ffi::c_int;
                    } else {
                        if !(*z.offset((n + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == '/' as i32)
                        {
                            break;
                        }
                        let mut j_0: ::core::ffi::c_int = 0;
                        let mut c: ::core::ffi::c_char = 0;
                        j_0 = n + 2 as ::core::ffi::c_int;
                        loop {
                            c = *z.offset(j_0 as isize) as ::core::ffi::c_char;
                            if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                                break;
                            }
                            if c as ::core::ffi::c_int == '\n' as i32
                                || c as ::core::ffi::c_int == '\r' as i32
                            {
                                break;
                            }
                            if 0xe2 as ::core::ffi::c_int
                                == c as u8_0 as ::core::ffi::c_int
                                && 0x80 as ::core::ffi::c_int
                                    == *z.offset((j_0 + 1 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int
                                && (0xa8 as ::core::ffi::c_int
                                    == *z.offset((j_0 + 2 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int
                                    || 0xa9 as ::core::ffi::c_int
                                        == *z.offset((j_0 + 2 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int)
                            {
                                j_0 += 2 as ::core::ffi::c_int;
                                break;
                            } else {
                                j_0 += 1;
                            }
                        }
                        n = j_0;
                        if *z.offset(n as isize) != 0 {
                            n += 1;
                        }
                    }
                }
                194 => {
                    if !(*z.offset((n + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 0xa0 as ::core::ffi::c_int)
                    {
                        break;
                    }
                    n += 2 as ::core::ffi::c_int;
                }
                225 => {
                    if !(*z.offset((n + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 0x9a as ::core::ffi::c_int
                        && *z.offset((n + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int)
                    {
                        break;
                    }
                    n += 3 as ::core::ffi::c_int;
                }
                226 => {
                    if *z.offset((n + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int
                    {
                        let mut c_0: u8_0 = *z
                            .offset((n + 2 as ::core::ffi::c_int) as isize);
                        if (c_0 as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
                            break;
                        }
                        if !(c_0 as ::core::ffi::c_int <= 0x8a as ::core::ffi::c_int
                            || c_0 as ::core::ffi::c_int == 0xa8 as ::core::ffi::c_int
                            || c_0 as ::core::ffi::c_int == 0xa9 as ::core::ffi::c_int
                            || c_0 as ::core::ffi::c_int == 0xaf as ::core::ffi::c_int)
                        {
                            break;
                        }
                        n += 3 as ::core::ffi::c_int;
                    } else {
                        if !(*z.offset((n + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == 0x81 as ::core::ffi::c_int
                            && *z.offset((n + 2 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int == 0x9f as ::core::ffi::c_int)
                        {
                            break;
                        }
                        n += 3 as ::core::ffi::c_int;
                    }
                }
                227 => {
                    if !(*z.offset((n + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int
                        && *z.offset((n + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int)
                    {
                        break;
                    }
                    n += 3 as ::core::ffi::c_int;
                }
                239 => {
                    if !(*z.offset((n + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 0xbb as ::core::ffi::c_int
                        && *z.offset((n + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == 0xbf as ::core::ffi::c_int)
                    {
                        break;
                    }
                    n += 3 as ::core::ffi::c_int;
                }
                _ => {
                    break;
                }
            }
        }
        return n;
    }
}
static mut aNanInfName: [NanInfName; 5] = [
    NanInfName {
        c1: 'i' as i32 as ::core::ffi::c_char,
        c2: 'I' as i32 as ::core::ffi::c_char,
        n: 3 as ::core::ffi::c_char,
        eType: JSONB_FLOAT as ::core::ffi::c_char,
        nRepl: 7 as ::core::ffi::c_char,
        zMatch: b"inf\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        zRepl: b"9.0e999\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char

    },
    NanInfName {
        c1: 'i' as i32 as ::core::ffi::c_char,
        c2: 'I' as i32 as ::core::ffi::c_char,
        n: 8 as ::core::ffi::c_char,
        eType: JSONB_FLOAT as ::core::ffi::c_char,
        nRepl: 7 as ::core::ffi::c_char,
        zMatch: b"infinity\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        zRepl: b"9.0e999\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char

    },
    NanInfName {
        c1: 'n' as i32 as ::core::ffi::c_char,
        c2: 'N' as i32 as ::core::ffi::c_char,
        n: 3 as ::core::ffi::c_char,
        eType: JSONB_NULL as ::core::ffi::c_char,
        nRepl: 4 as ::core::ffi::c_char,
        zMatch: b"NaN\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        zRepl: b"null\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char

    },
    NanInfName {
        c1: 'q' as i32 as ::core::ffi::c_char,
        c2: 'Q' as i32 as ::core::ffi::c_char,
        n: 4 as ::core::ffi::c_char,
        eType: JSONB_NULL as ::core::ffi::c_char,
        nRepl: 4 as ::core::ffi::c_char,
        zMatch: b"QNaN\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        zRepl: b"null\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char

    },
    NanInfName {
        c1: 's' as i32 as ::core::ffi::c_char,
        c2: 'S' as i32 as ::core::ffi::c_char,
        n: 4 as ::core::ffi::c_char,
        eType: JSONB_NULL as ::core::ffi::c_char,
        nRepl: 4 as ::core::ffi::c_char,
        zMatch: b"SNaN\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        zRepl: b"null\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char

    },
];
unsafe extern "C" fn jsonWrongNumArgs(
    mut pCtx: *mut sqlite3_context,
    mut zFuncName: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut zMsg: *mut ::core::ffi::c_char = sqlite3_mprintf(
            b"json_%s() needs an odd number of arguments\0".as_ptr()
                as *const ::core::ffi::c_char,
            zFuncName,
        );
        sqlite3_result_error(pCtx, zMsg, -1 as ::core::ffi::c_int);
        sqlite3_free(zMsg as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn jsonBlobExpand(
    mut pParse: *mut JsonParse,
    mut N: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aNew: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut t: u64_0 = 0;
        if (*pParse).nBlobAlloc == 0 as u32_0 {
            t = 100 as u64_0;
        } else {
            t = (*pParse).nBlobAlloc.wrapping_mul(2 as u32_0) as u64_0;
        }
        if t < N as ::core::ffi::c_ulonglong {
            t = N.wrapping_add(100 as u32_0) as u64_0;
        }
        aNew = sqlite3DbRealloc(
            (*pParse).db,
            (*pParse).aBlob as *mut ::core::ffi::c_void,
            t,
        ) as *mut u8_0;
        if aNew.is_null() {
            (*pParse).oom = 1 as u8_0;
            return 1 as ::core::ffi::c_int;
        }
        (*pParse).aBlob = aNew;
        (*pParse).nBlobAlloc = t as u32_0;
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn jsonBlobMakeEditable(
    mut pParse: *mut JsonParse,
    mut nExtra: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aOld: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut nSize: u32_0 = 0;
        if (*pParse).oom != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*pParse).nBlobAlloc > 0 as u32_0 {
            return 1 as ::core::ffi::c_int;
        }
        aOld = (*pParse).aBlob;
        nSize = (*pParse).nBlob.wrapping_add(nExtra);
        (*pParse).aBlob = ::core::ptr::null_mut::<u8_0>();
        if jsonBlobExpand(pParse, nSize) != 0 {
            return 0 as ::core::ffi::c_int;
        }
        memcpy(
            (*pParse).aBlob as *mut ::core::ffi::c_void,
            aOld as *const ::core::ffi::c_void,
            (*pParse).nBlob as size_t,
        );
        return 1 as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn jsonBlobExpandAndAppendOneByte(
    mut pParse: *mut JsonParse,
    mut c: u8_0,
) {
    unsafe {
        jsonBlobExpand(pParse, (*pParse).nBlob.wrapping_add(1 as u32_0));
        if (*pParse).oom as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let c2rust_fresh1 = (*pParse).nBlob;
            (*pParse).nBlob = (*pParse).nBlob.wrapping_add(1);
            *(*pParse).aBlob.offset(c2rust_fresh1 as isize) = c;
        }
    }
}
unsafe extern "C" fn jsonBlobAppendOneByte(mut pParse: *mut JsonParse, mut c: u8_0) {
    unsafe {
        if (*pParse).nBlob >= (*pParse).nBlobAlloc {
            jsonBlobExpandAndAppendOneByte(pParse, c);
        } else {
            let c2rust_fresh0 = (*pParse).nBlob;
            (*pParse).nBlob = (*pParse).nBlob.wrapping_add(1);
            *(*pParse).aBlob.offset(c2rust_fresh0 as isize) = c;
        };
    }
}
#[inline(never)]
unsafe extern "C" fn jsonBlobExpandAndAppendNode(
    mut pParse: *mut JsonParse,
    mut eType: u8_0,
    mut szPayload: u32_0,
    mut aPayload: *const ::core::ffi::c_void,
) {
    unsafe {
        if jsonBlobExpand(
            pParse,
            (*pParse).nBlob.wrapping_add(szPayload).wrapping_add(9 as u32_0),
        ) != 0
        {
            return;
        }
        jsonBlobAppendNode(pParse, eType, szPayload, aPayload);
    }
}
unsafe extern "C" fn jsonBlobAppendNode(
    mut pParse: *mut JsonParse,
    mut eType: u8_0,
    mut szPayload: u32_0,
    mut aPayload: *const ::core::ffi::c_void,
) {
    unsafe {
        let mut a: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        if (*pParse).nBlob.wrapping_add(szPayload).wrapping_add(9 as u32_0)
            > (*pParse).nBlobAlloc
        {
            jsonBlobExpandAndAppendNode(pParse, eType, szPayload, aPayload);
            return;
        }
        a = (*pParse).aBlob.offset((*pParse).nBlob as isize) as *mut u8_0;
        if szPayload <= 11 as u32_0 {
            *a.offset(0 as ::core::ffi::c_int as isize) = (eType as u32_0
                | szPayload << 4 as ::core::ffi::c_int) as u8_0;
            (*pParse).nBlob = ((*pParse).nBlob as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as u32_0 as u32_0;
        } else if szPayload <= 0xff as u32_0 {
            *a.offset(0 as ::core::ffi::c_int as isize) = (eType as ::core::ffi::c_int
                | 0xc0 as ::core::ffi::c_int) as u8_0;
            *a.offset(1 as ::core::ffi::c_int as isize) = (szPayload & 0xff as u32_0)
                as u8_0;
            (*pParse).nBlob = ((*pParse).nBlob as ::core::ffi::c_uint)
                .wrapping_add(2 as ::core::ffi::c_uint) as u32_0 as u32_0;
        } else if szPayload <= 0xffff as u32_0 {
            *a.offset(0 as ::core::ffi::c_int as isize) = (eType as ::core::ffi::c_int
                | 0xd0 as ::core::ffi::c_int) as u8_0;
            *a.offset(1 as ::core::ffi::c_int as isize) = (szPayload
                >> 8 as ::core::ffi::c_int & 0xff as u32_0) as u8_0;
            *a.offset(2 as ::core::ffi::c_int as isize) = (szPayload & 0xff as u32_0)
                as u8_0;
            (*pParse).nBlob = ((*pParse).nBlob as ::core::ffi::c_uint)
                .wrapping_add(3 as ::core::ffi::c_uint) as u32_0 as u32_0;
        } else {
            *a.offset(0 as ::core::ffi::c_int as isize) = (eType as ::core::ffi::c_int
                | 0xe0 as ::core::ffi::c_int) as u8_0;
            *a.offset(1 as ::core::ffi::c_int as isize) = (szPayload
                >> 24 as ::core::ffi::c_int & 0xff as u32_0) as u8_0;
            *a.offset(2 as ::core::ffi::c_int as isize) = (szPayload
                >> 16 as ::core::ffi::c_int & 0xff as u32_0) as u8_0;
            *a.offset(3 as ::core::ffi::c_int as isize) = (szPayload
                >> 8 as ::core::ffi::c_int & 0xff as u32_0) as u8_0;
            *a.offset(4 as ::core::ffi::c_int as isize) = (szPayload & 0xff as u32_0)
                as u8_0;
            (*pParse).nBlob = ((*pParse).nBlob as ::core::ffi::c_uint)
                .wrapping_add(5 as ::core::ffi::c_uint) as u32_0 as u32_0;
        }
        if !aPayload.is_null() {
            (*pParse).nBlob = ((*pParse).nBlob as ::core::ffi::c_uint)
                .wrapping_add(szPayload as ::core::ffi::c_uint) as u32_0 as u32_0;
            memcpy(
                (*pParse).aBlob.offset((*pParse).nBlob.wrapping_sub(szPayload) as isize)
                    as *mut u8_0 as *mut ::core::ffi::c_void,
                aPayload,
                szPayload as size_t,
            );
        }
    }
}
unsafe extern "C" fn jsonBlobChangePayloadSize(
    mut pParse: *mut JsonParse,
    mut i: u32_0,
    mut szPayload: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut a: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut szType: u8_0 = 0;
        let mut nExtra: u8_0 = 0;
        let mut nNeeded: u8_0 = 0;
        let mut delta: ::core::ffi::c_int = 0;
        if (*pParse).oom != 0 {
            return 0 as ::core::ffi::c_int;
        }
        a = (*pParse).aBlob.offset(i as isize) as *mut u8_0;
        szType = (*a.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) as u8_0;
        if szType as ::core::ffi::c_int <= 11 as ::core::ffi::c_int {
            nExtra = 0 as u8_0;
        } else if szType as ::core::ffi::c_int == 12 as ::core::ffi::c_int {
            nExtra = 1 as u8_0;
        } else if szType as ::core::ffi::c_int == 13 as ::core::ffi::c_int {
            nExtra = 2 as u8_0;
        } else if szType as ::core::ffi::c_int == 14 as ::core::ffi::c_int {
            nExtra = 4 as u8_0;
        } else {
            nExtra = 8 as u8_0;
        }
        if szPayload <= 11 as u32_0 {
            nNeeded = 0 as u8_0;
        } else if szPayload <= 0xff as u32_0 {
            nNeeded = 1 as u8_0;
        } else if szPayload <= 0xffff as u32_0 {
            nNeeded = 2 as u8_0;
        } else {
            nNeeded = 4 as u8_0;
        }
        delta = nNeeded as ::core::ffi::c_int - nExtra as ::core::ffi::c_int;
        if delta != 0 {
            let mut newSize: u32_0 = (*pParse).nBlob.wrapping_add(delta as u32_0);
            if delta > 0 as ::core::ffi::c_int {
                if newSize > (*pParse).nBlobAlloc && jsonBlobExpand(pParse, newSize) != 0
                {
                    return 0 as ::core::ffi::c_int;
                }
                a = (*pParse).aBlob.offset(i as isize) as *mut u8_0;
                memmove(
                    a.offset((1 as ::core::ffi::c_int + delta) as isize) as *mut u8_0
                        as *mut ::core::ffi::c_void,
                    a.offset(1 as ::core::ffi::c_int as isize) as *mut u8_0
                        as *const ::core::ffi::c_void,
                    (*pParse).nBlob.wrapping_sub(i.wrapping_add(1 as u32_0)) as size_t,
                );
            } else {
                memmove(
                    a.offset(1 as ::core::ffi::c_int as isize) as *mut u8_0
                        as *mut ::core::ffi::c_void,
                    a.offset((1 as ::core::ffi::c_int - delta) as isize) as *mut u8_0
                        as *const ::core::ffi::c_void,
                    (*pParse)
                        .nBlob
                        .wrapping_sub(
                            i.wrapping_add(1 as u32_0).wrapping_sub(delta as u32_0),
                        ) as size_t,
                );
            }
            (*pParse).nBlob = newSize;
        }
        if nNeeded as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *a.offset(0 as ::core::ffi::c_int as isize) = ((*a
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as u32_0
                | szPayload << 4 as ::core::ffi::c_int) as u8_0;
        } else if nNeeded as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            *a.offset(0 as ::core::ffi::c_int as isize) = (*a
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int | 0xc0 as ::core::ffi::c_int) as u8_0;
            *a.offset(1 as ::core::ffi::c_int as isize) = (szPayload & 0xff as u32_0)
                as u8_0;
        } else if nNeeded as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            *a.offset(0 as ::core::ffi::c_int as isize) = (*a
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int | 0xd0 as ::core::ffi::c_int) as u8_0;
            *a.offset(1 as ::core::ffi::c_int as isize) = (szPayload
                >> 8 as ::core::ffi::c_int & 0xff as u32_0) as u8_0;
            *a.offset(2 as ::core::ffi::c_int as isize) = (szPayload & 0xff as u32_0)
                as u8_0;
        } else {
            *a.offset(0 as ::core::ffi::c_int as isize) = (*a
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int | 0xe0 as ::core::ffi::c_int) as u8_0;
            *a.offset(1 as ::core::ffi::c_int as isize) = (szPayload
                >> 24 as ::core::ffi::c_int & 0xff as u32_0) as u8_0;
            *a.offset(2 as ::core::ffi::c_int as isize) = (szPayload
                >> 16 as ::core::ffi::c_int & 0xff as u32_0) as u8_0;
            *a.offset(3 as ::core::ffi::c_int as isize) = (szPayload
                >> 8 as ::core::ffi::c_int & 0xff as u32_0) as u8_0;
            *a.offset(4 as ::core::ffi::c_int as isize) = (szPayload & 0xff as u32_0)
                as u8_0;
        }
        return delta;
    }
}
unsafe extern "C" fn jsonIs4HexB(
    mut z: *const ::core::ffi::c_char,
    mut pOp: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 'u' as i32
        {
            return 0 as ::core::ffi::c_int;
        }
        if jsonIs4Hex(
            z.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        *pOp = JSONB_TEXTJ;
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn jsonbValidityCheck(
    mut pParse: *const JsonParse,
    mut i: u32_0,
    mut iEnd: u32_0,
    mut iDepth: u32_0,
) -> u32_0 {
    unsafe {
        let mut n: u32_0 = 0;
        let mut sz: u32_0 = 0;
        let mut j: u32_0 = 0;
        let mut k: u32_0 = 0;
        let mut z: *const u8_0 = ::core::ptr::null::<u8_0>();
        let mut x: u8_0 = 0;
        if iDepth > JSON_MAX_DEPTH as u32_0 {
            return i.wrapping_add(1 as u32_0);
        }
        sz = 0 as u32_0;
        n = jsonbPayloadSize(pParse, i, &raw mut sz);
        if n == 0 as u32_0 {
            return i.wrapping_add(1 as u32_0);
        }
        if i.wrapping_add(n).wrapping_add(sz) != iEnd {
            return i.wrapping_add(1 as u32_0);
        }
        z = (*pParse).aBlob;
        x = (*z.offset(i as isize) as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
            as u8_0;
        match x as ::core::ffi::c_int {
            JSONB_NULL | JSONB_TRUE | JSONB_FALSE => {
                return if n.wrapping_add(sz) == 1 as u32_0 {
                    0 as u32_0
                } else {
                    i.wrapping_add(1 as u32_0)
                };
            }
            JSONB_INT => {
                if sz < 1 as u32_0 {
                    return i.wrapping_add(1 as u32_0);
                }
                j = i.wrapping_add(n);
                if *z.offset(j as isize) as ::core::ffi::c_int == '-' as i32 {
                    j = j.wrapping_add(1);
                    if sz < 2 as u32_0 {
                        return i.wrapping_add(1 as u32_0);
                    }
                }
                k = i.wrapping_add(n).wrapping_add(sz);
                while j < k {
                    if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(*z.offset(j as isize) as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                    {
                        j = j.wrapping_add(1);
                    } else {
                        return j.wrapping_add(1 as u32_0)
                    }
                }
                return 0 as u32_0;
            }
            JSONB_INT5 => {
                if sz < 3 as u32_0 {
                    return i.wrapping_add(1 as u32_0);
                }
                j = i.wrapping_add(n);
                if *z.offset(j as isize) as ::core::ffi::c_int == '-' as i32 {
                    if sz < 4 as u32_0 {
                        return i.wrapping_add(1 as u32_0);
                    }
                    j = j.wrapping_add(1);
                }
                if *z.offset(j as isize) as ::core::ffi::c_int != '0' as i32 {
                    return i.wrapping_add(1 as u32_0);
                }
                if *z.offset(j.wrapping_add(1 as u32_0) as isize) as ::core::ffi::c_int
                    != 'x' as i32
                    && *z.offset(j.wrapping_add(1 as u32_0) as isize)
                        as ::core::ffi::c_int != 'X' as i32
                {
                    return j.wrapping_add(2 as u32_0);
                }
                j = (j as ::core::ffi::c_uint).wrapping_add(2 as ::core::ffi::c_uint)
                    as u32_0 as u32_0;
                k = i.wrapping_add(n).wrapping_add(sz);
                while j < k {
                    if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(*z.offset(j as isize) as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                    {
                        j = j.wrapping_add(1);
                    } else {
                        return j.wrapping_add(1 as u32_0)
                    }
                }
                return 0 as u32_0;
            }
            JSONB_FLOAT | JSONB_FLOAT5 => {
                let mut seen: u8_0 = 0 as u8_0;
                if sz < 2 as u32_0 {
                    return i.wrapping_add(1 as u32_0);
                }
                j = i.wrapping_add(n);
                k = j.wrapping_add(sz);
                if *z.offset(j as isize) as ::core::ffi::c_int == '-' as i32 {
                    j = j.wrapping_add(1);
                    if sz < 3 as u32_0 {
                        return i.wrapping_add(1 as u32_0);
                    }
                }
                if *z.offset(j as isize) as ::core::ffi::c_int == '.' as i32 {
                    if x as ::core::ffi::c_int == JSONB_FLOAT {
                        return j.wrapping_add(1 as u32_0);
                    }
                    if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(
                            *z.offset(j.wrapping_add(1 as u32_0) as isize)
                                as ::core::ffi::c_uchar as isize,
                        ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
                    {
                        return j.wrapping_add(1 as u32_0);
                    }
                    j = (j as ::core::ffi::c_uint).wrapping_add(2 as ::core::ffi::c_uint)
                        as u32_0 as u32_0;
                    seen = 1 as u8_0;
                } else if *z.offset(j as isize) as ::core::ffi::c_int == '0' as i32
                    && x as ::core::ffi::c_int == JSONB_FLOAT
                {
                    if j.wrapping_add(3 as u32_0) > k {
                        return j.wrapping_add(1 as u32_0);
                    }
                    if *z.offset(j.wrapping_add(1 as u32_0) as isize)
                        as ::core::ffi::c_int != '.' as i32
                        && *z.offset(j.wrapping_add(1 as u32_0) as isize)
                            as ::core::ffi::c_int != 'e' as i32
                        && *z.offset(j.wrapping_add(1 as u32_0) as isize)
                            as ::core::ffi::c_int != 'E' as i32
                    {
                        return j.wrapping_add(1 as u32_0);
                    }
                    j = j.wrapping_add(1);
                }
                while j < k {
                    if !(*(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(*z.offset(j as isize) as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0)
                    {
                        if *z.offset(j as isize) as ::core::ffi::c_int == '.' as i32 {
                            if seen as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                return j.wrapping_add(1 as u32_0);
                            }
                            if x as ::core::ffi::c_int == JSONB_FLOAT
                                && (j == k.wrapping_sub(1 as u32_0)
                                    || *(&raw const sqlite3CtypeMap
                                        as *const ::core::ffi::c_uchar)
                                        .offset(
                                            *z.offset(j.wrapping_add(1 as u32_0) as isize)
                                                as ::core::ffi::c_uchar as isize,
                                        ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0)
                            {
                                return j.wrapping_add(1 as u32_0);
                            }
                            seen = 1 as u8_0;
                        } else if *z.offset(j as isize) as ::core::ffi::c_int
                            == 'e' as i32
                            || *z.offset(j as isize) as ::core::ffi::c_int == 'E' as i32
                        {
                            if seen as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                return j.wrapping_add(1 as u32_0);
                            }
                            if j == k.wrapping_sub(1 as u32_0) {
                                return j.wrapping_add(1 as u32_0);
                            }
                            if *z.offset(j.wrapping_add(1 as u32_0) as isize)
                                as ::core::ffi::c_int == '+' as i32
                                || *z.offset(j.wrapping_add(1 as u32_0) as isize)
                                    as ::core::ffi::c_int == '-' as i32
                            {
                                j = j.wrapping_add(1);
                                if j == k.wrapping_sub(1 as u32_0) {
                                    return j.wrapping_add(1 as u32_0);
                                }
                            }
                            seen = 2 as u8_0;
                        } else {
                            return j.wrapping_add(1 as u32_0)
                        }
                    }
                    j = j.wrapping_add(1);
                }
                if seen as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    return i.wrapping_add(1 as u32_0);
                }
                return 0 as u32_0;
            }
            JSONB_TEXT => {
                j = i.wrapping_add(n);
                k = j.wrapping_add(sz);
                while j < k {
                    if jsonIsOk[*z.offset(j as isize) as usize] == 0
                        && *z.offset(j as isize) as ::core::ffi::c_int != '\'' as i32
                    {
                        return j.wrapping_add(1 as u32_0);
                    }
                    j = j.wrapping_add(1);
                }
                return 0 as u32_0;
            }
            JSONB_TEXTJ | JSONB_TEXT5 => {
                j = i.wrapping_add(n);
                k = j.wrapping_add(sz);
                while j < k {
                    if jsonIsOk[*z.offset(j as isize) as usize] == 0
                        && *z.offset(j as isize) as ::core::ffi::c_int != '\'' as i32
                    {
                        if *z.offset(j as isize) as ::core::ffi::c_int == '"' as i32 {
                            if x as ::core::ffi::c_int == JSONB_TEXTJ {
                                return j.wrapping_add(1 as u32_0);
                            }
                        } else if *z.offset(j as isize) as ::core::ffi::c_int
                            <= 0x1f as ::core::ffi::c_int
                        {
                            if x as ::core::ffi::c_int == JSONB_TEXTJ {
                                return j.wrapping_add(1 as u32_0);
                            }
                        } else if *z.offset(j as isize) as ::core::ffi::c_int
                            != '\\' as i32 || j.wrapping_add(1 as u32_0) >= k
                        {
                            return j.wrapping_add(1 as u32_0)
                        } else if !strchr(
                                b"\"\\/bfnrt\0".as_ptr() as *const ::core::ffi::c_char,
                                *z.offset(j.wrapping_add(1 as u32_0) as isize)
                                    as ::core::ffi::c_int,
                            )
                            .is_null()
                        {
                            j = j.wrapping_add(1);
                        } else if *z.offset(j.wrapping_add(1 as u32_0) as isize)
                            as ::core::ffi::c_int == 'u' as i32
                        {
                            if j.wrapping_add(5 as u32_0) >= k {
                                return j.wrapping_add(1 as u32_0);
                            }
                            if jsonIs4Hex(
                                z.offset(j.wrapping_add(2 as u32_0) as isize) as *const u8_0
                                    as *const ::core::ffi::c_char,
                            ) == 0
                            {
                                return j.wrapping_add(1 as u32_0);
                            }
                            j = j.wrapping_add(1);
                        } else if x as ::core::ffi::c_int != JSONB_TEXT5 {
                            return j.wrapping_add(1 as u32_0)
                        } else {
                            let mut c: u32_0 = 0 as u32_0;
                            let mut szC: u32_0 = jsonUnescapeOneChar(
                                z.offset(j as isize) as *const u8_0
                                    as *const ::core::ffi::c_char,
                                k.wrapping_sub(j),
                                &raw mut c,
                            );
                            if c == JSON_INVALID_CHAR as u32_0 {
                                return j.wrapping_add(1 as u32_0);
                            }
                            j = (j as ::core::ffi::c_uint)
                                .wrapping_add(
                                    szC.wrapping_sub(1 as u32_0) as ::core::ffi::c_uint,
                                ) as u32_0 as u32_0;
                        }
                    }
                    j = j.wrapping_add(1);
                }
                return 0 as u32_0;
            }
            JSONB_TEXTRAW => return 0 as u32_0,
            JSONB_ARRAY => {
                let mut sub: u32_0 = 0;
                j = i.wrapping_add(n);
                k = j.wrapping_add(sz);
                while j < k {
                    sz = 0 as u32_0;
                    n = jsonbPayloadSize(pParse, j, &raw mut sz);
                    if n == 0 as u32_0 {
                        return j.wrapping_add(1 as u32_0);
                    }
                    if j.wrapping_add(n).wrapping_add(sz) > k {
                        return j.wrapping_add(1 as u32_0);
                    }
                    sub = jsonbValidityCheck(
                        pParse,
                        j,
                        j.wrapping_add(n).wrapping_add(sz),
                        iDepth.wrapping_add(1 as u32_0),
                    );
                    if sub != 0 {
                        return sub;
                    }
                    j = (j as ::core::ffi::c_uint)
                        .wrapping_add(n.wrapping_add(sz) as ::core::ffi::c_uint) as u32_0
                        as u32_0;
                }
                return 0 as u32_0;
            }
            JSONB_OBJECT => {
                let mut cnt: u32_0 = 0 as u32_0;
                let mut sub_0: u32_0 = 0;
                j = i.wrapping_add(n);
                k = j.wrapping_add(sz);
                while j < k {
                    sz = 0 as u32_0;
                    n = jsonbPayloadSize(pParse, j, &raw mut sz);
                    if n == 0 as u32_0 {
                        return j.wrapping_add(1 as u32_0);
                    }
                    if j.wrapping_add(n).wrapping_add(sz) > k {
                        return j.wrapping_add(1 as u32_0);
                    }
                    if cnt & 1 as u32_0 == 0 as u32_0 {
                        x = (*z.offset(j as isize) as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int) as u8_0;
                        if (x as ::core::ffi::c_int) < JSONB_TEXT
                            || x as ::core::ffi::c_int > JSONB_TEXTRAW
                        {
                            return j.wrapping_add(1 as u32_0);
                        }
                    }
                    sub_0 = jsonbValidityCheck(
                        pParse,
                        j,
                        j.wrapping_add(n).wrapping_add(sz),
                        iDepth.wrapping_add(1 as u32_0),
                    );
                    if sub_0 != 0 {
                        return sub_0;
                    }
                    cnt = cnt.wrapping_add(1);
                    j = (j as ::core::ffi::c_uint)
                        .wrapping_add(n.wrapping_add(sz) as ::core::ffi::c_uint) as u32_0
                        as u32_0;
                }
                if cnt & 1 as u32_0 != 0 as u32_0 {
                    return j.wrapping_add(1 as u32_0);
                }
                return 0 as u32_0;
            }
            _ => return i.wrapping_add(1 as u32_0)

        };
    }
}
unsafe extern "C" fn jsonTranslateTextToBlob(
    mut pParse: *mut JsonParse,
    mut i: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut opcode: u8_0 = 0;
        let mut cDelim: ::core::ffi::c_char = 0;
        let mut seenE: u8_0 = 0;
        let mut c2rust_current_block: u64;
        let mut c: ::core::ffi::c_char = 0;
        let mut j: u32_0 = 0;
        let mut iThis: u32_0 = 0;
        let mut iStart: u32_0 = 0;
        let mut x: ::core::ffi::c_int = 0;
        let mut t: u8_0 = 0;
        let mut z: *const ::core::ffi::c_char = (*pParse).zJson;
        loop {
            match *z.offset(i as isize) as u8_0 as ::core::ffi::c_int {
                123 => {
                    iThis = (*pParse).nBlob;
                    jsonBlobAppendNode(
                        pParse,
                        JSONB_OBJECT as u8_0,
                        ((*pParse).nJson as u32_0).wrapping_sub(i),
                        ::core::ptr::null::<::core::ffi::c_void>(),
                    );
                    (*pParse).iDepth = (*pParse).iDepth.wrapping_add(1);
                    if (*pParse).iDepth as ::core::ffi::c_int > JSON_MAX_DEPTH {
                        (*pParse).iErr = i;
                        return -1 as ::core::ffi::c_int;
                    }
                    iStart = (*pParse).nBlob;
                    let mut c2rust_current_block_60: u64;
                    j = i.wrapping_add(1 as u32_0);
                    loop {
                        let mut iBlob: u32_0 = (*pParse).nBlob;
                        x = jsonTranslateTextToBlob(pParse, j);
                        if x <= 0 as ::core::ffi::c_int {
                            let mut op: ::core::ffi::c_int = 0;
                            if x == -2 as ::core::ffi::c_int {
                                j = (*pParse).iErr;
                                if (*pParse).nBlob != iStart {
                                    (*pParse).hasNonstd = 1 as u8_0;
                                }
                                break;
                            } else {
                                j = (j as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        json5Whitespace(
                                            z.offset(j as isize) as *const ::core::ffi::c_char,
                                        ) as ::core::ffi::c_uint,
                                    ) as u32_0 as u32_0;
                                op = JSONB_TEXT;
                                if *(&raw const sqlite3CtypeMap
                                    as *const ::core::ffi::c_uchar)
                                    .offset(
                                        *z.offset(j as isize) as ::core::ffi::c_uchar as isize,
                                    ) as ::core::ffi::c_int & 0x42 as ::core::ffi::c_int != 0
                                    || *z.offset(j as isize) as ::core::ffi::c_int
                                        == '\\' as i32
                                        && jsonIs4HexB(
                                            z.offset(j.wrapping_add(1 as u32_0) as isize)
                                                as *const ::core::ffi::c_char,
                                            &raw mut op,
                                        ) != 0
                                {
                                    let mut k: ::core::ffi::c_int = j.wrapping_add(1 as u32_0)
                                        as ::core::ffi::c_int;
                                    while *(&raw const sqlite3CtypeMap
                                        as *const ::core::ffi::c_uchar)
                                        .offset(
                                            *z.offset(k as isize) as ::core::ffi::c_uchar as isize,
                                        ) as ::core::ffi::c_int & 0x46 as ::core::ffi::c_int != 0
                                        && json5Whitespace(
                                            z.offset(k as isize) as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        || *z.offset(k as isize) as ::core::ffi::c_int
                                            == '\\' as i32
                                            && jsonIs4HexB(
                                                z.offset((k + 1 as ::core::ffi::c_int) as isize)
                                                    as *const ::core::ffi::c_char,
                                                &raw mut op,
                                            ) != 0
                                    {
                                        k += 1;
                                    }
                                    jsonBlobAppendNode(
                                        pParse,
                                        op as u8_0,
                                        (k as u32_0).wrapping_sub(j),
                                        z.offset(j as isize) as *const ::core::ffi::c_char
                                            as *const ::core::ffi::c_void,
                                    );
                                    (*pParse).hasNonstd = 1 as u8_0;
                                    x = k;
                                } else {
                                    if x != -1 as ::core::ffi::c_int {
                                        (*pParse).iErr = j;
                                    }
                                    return -1 as ::core::ffi::c_int;
                                }
                            }
                        }
                        if (*pParse).oom != 0 {
                            return -1 as ::core::ffi::c_int;
                        }
                        t = (*(*pParse).aBlob.offset(iBlob as isize)
                            as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as u8_0;
                        if (t as ::core::ffi::c_int) < JSONB_TEXT
                            || t as ::core::ffi::c_int > JSONB_TEXTRAW
                        {
                            (*pParse).iErr = j;
                            return -1 as ::core::ffi::c_int;
                        }
                        j = x as u32_0;
                        if *z.offset(j as isize) as ::core::ffi::c_int == ':' as i32 {
                            j = j.wrapping_add(1);
                        } else {
                            if jsonIsSpace[*z.offset(j as isize) as ::core::ffi::c_uchar
                                as usize] != 0
                            {
                                loop {
                                    j = j.wrapping_add(1);
                                    if !(jsonIsSpace[*z.offset(j as isize)
                                        as ::core::ffi::c_uchar as usize] != 0)
                                    {
                                        break;
                                    }
                                }
                                if *z.offset(j as isize) as ::core::ffi::c_int == ':' as i32
                                {
                                    j = j.wrapping_add(1);
                                    c2rust_current_block_60 = 646160108097468510;
                                } else {
                                    c2rust_current_block_60 = 6450597802325118133;
                                }
                            } else {
                                c2rust_current_block_60 = 6450597802325118133;
                            }
                            match c2rust_current_block_60 {
                                646160108097468510 => {}
                                _ => {
                                    x = jsonTranslateTextToBlob(pParse, j);
                                    if x != -5 as ::core::ffi::c_int {
                                        if x != -1 as ::core::ffi::c_int {
                                            (*pParse).iErr = j;
                                        }
                                        return -1 as ::core::ffi::c_int;
                                    }
                                    j = (*pParse).iErr.wrapping_add(1 as u32_0);
                                }
                            }
                        }
                        x = jsonTranslateTextToBlob(pParse, j);
                        if x <= 0 as ::core::ffi::c_int {
                            if x != -1 as ::core::ffi::c_int {
                                (*pParse).iErr = j;
                            }
                            return -1 as ::core::ffi::c_int;
                        }
                        j = x as u32_0;
                        if !(*z.offset(j as isize) as ::core::ffi::c_int == ',' as i32) {
                            if *z.offset(j as isize) as ::core::ffi::c_int == '}' as i32
                            {
                                break;
                            }
                            if jsonIsSpace[*z.offset(j as isize) as ::core::ffi::c_uchar
                                as usize] != 0
                            {
                                j = (j as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        (1 as u32_0)
                                            .wrapping_add(
                                                strspn(
                                                    z.offset(j.wrapping_add(1 as u32_0) as isize)
                                                        as *const ::core::ffi::c_char,
                                                    &raw const jsonSpaces as *const ::core::ffi::c_char,
                                                ) as u32_0,
                                            ) as ::core::ffi::c_uint,
                                    ) as u32_0 as u32_0;
                                if *z.offset(j as isize) as ::core::ffi::c_int == ',' as i32
                                {
                                    c2rust_current_block_60 = 14523784380283086299;
                                } else {
                                    if *z.offset(j as isize) as ::core::ffi::c_int == '}' as i32
                                    {
                                        break;
                                    }
                                    c2rust_current_block_60 = 9859671972921157070;
                                }
                            } else {
                                c2rust_current_block_60 = 9859671972921157070;
                            }
                            match c2rust_current_block_60 {
                                14523784380283086299 => {}
                                _ => {
                                    x = jsonTranslateTextToBlob(pParse, j);
                                    if x == -4 as ::core::ffi::c_int {
                                        j = (*pParse).iErr;
                                    } else if x == -2 as ::core::ffi::c_int {
                                        j = (*pParse).iErr;
                                        break;
                                    } else {
                                        (*pParse).iErr = j;
                                        return -1 as ::core::ffi::c_int;
                                    }
                                }
                            }
                        }
                        j = j.wrapping_add(1);
                    }
                    jsonBlobChangePayloadSize(
                        pParse,
                        iThis,
                        (*pParse).nBlob.wrapping_sub(iStart),
                    );
                    (*pParse).iDepth = (*pParse).iDepth.wrapping_sub(1);
                    return j.wrapping_add(1 as u32_0) as ::core::ffi::c_int;
                }
                91 => {
                    iThis = (*pParse).nBlob;
                    jsonBlobAppendNode(
                        pParse,
                        JSONB_ARRAY as u8_0,
                        ((*pParse).nJson as u32_0).wrapping_sub(i),
                        ::core::ptr::null::<::core::ffi::c_void>(),
                    );
                    iStart = (*pParse).nBlob;
                    if (*pParse).oom != 0 {
                        return -1 as ::core::ffi::c_int;
                    }
                    (*pParse).iDepth = (*pParse).iDepth.wrapping_add(1);
                    if (*pParse).iDepth as ::core::ffi::c_int > JSON_MAX_DEPTH {
                        (*pParse).iErr = i;
                        return -1 as ::core::ffi::c_int;
                    }
                    let mut c2rust_current_block_90: u64;
                    j = i.wrapping_add(1 as u32_0);
                    loop {
                        x = jsonTranslateTextToBlob(pParse, j);
                        if x <= 0 as ::core::ffi::c_int {
                            if x == -3 as ::core::ffi::c_int {
                                j = (*pParse).iErr;
                                if (*pParse).nBlob != iStart {
                                    (*pParse).hasNonstd = 1 as u8_0;
                                }
                                break;
                            } else {
                                if x != -1 as ::core::ffi::c_int {
                                    (*pParse).iErr = j;
                                }
                                return -1 as ::core::ffi::c_int;
                            }
                        } else {
                            j = x as u32_0;
                            if !(*z.offset(j as isize) as ::core::ffi::c_int
                                == ',' as i32)
                            {
                                if *z.offset(j as isize) as ::core::ffi::c_int == ']' as i32
                                {
                                    break;
                                }
                                if jsonIsSpace[*z.offset(j as isize) as ::core::ffi::c_uchar
                                    as usize] != 0
                                {
                                    j = (j as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            (1 as u32_0)
                                                .wrapping_add(
                                                    strspn(
                                                        z.offset(j.wrapping_add(1 as u32_0) as isize)
                                                            as *const ::core::ffi::c_char,
                                                        &raw const jsonSpaces as *const ::core::ffi::c_char,
                                                    ) as u32_0,
                                                ) as ::core::ffi::c_uint,
                                        ) as u32_0 as u32_0;
                                    if *z.offset(j as isize) as ::core::ffi::c_int == ',' as i32
                                    {
                                        c2rust_current_block_90 = 11796148217846552555;
                                    } else {
                                        if *z.offset(j as isize) as ::core::ffi::c_int == ']' as i32
                                        {
                                            break;
                                        }
                                        c2rust_current_block_90 = 2616667235040759262;
                                    }
                                } else {
                                    c2rust_current_block_90 = 2616667235040759262;
                                }
                                match c2rust_current_block_90 {
                                    11796148217846552555 => {}
                                    _ => {
                                        x = jsonTranslateTextToBlob(pParse, j);
                                        if x == -4 as ::core::ffi::c_int {
                                            j = (*pParse).iErr;
                                        } else if x == -3 as ::core::ffi::c_int {
                                            j = (*pParse).iErr;
                                            break;
                                        } else {
                                            (*pParse).iErr = j;
                                            return -1 as ::core::ffi::c_int;
                                        }
                                    }
                                }
                            }
                            j = j.wrapping_add(1);
                        }
                    }
                    jsonBlobChangePayloadSize(
                        pParse,
                        iThis,
                        (*pParse).nBlob.wrapping_sub(iStart),
                    );
                    (*pParse).iDepth = (*pParse).iDepth.wrapping_sub(1);
                    return j.wrapping_add(1 as u32_0) as ::core::ffi::c_int;
                }
                39 => {
                    opcode = 0;
                    cDelim = 0;
                    (*pParse).hasNonstd = 1 as u8_0;
                    opcode = JSONB_TEXT as u8_0;
                    c2rust_current_block = 16508451873000240240;
                    break;
                }
                34 => {
                    opcode = JSONB_TEXT as u8_0;
                    c2rust_current_block = 16508451873000240240;
                    break;
                }
                116 => {
                    if strncmp(
                        z.offset(i as isize),
                        b"true\0".as_ptr() as *const ::core::ffi::c_char,
                        4 as size_t,
                    ) == 0 as ::core::ffi::c_int
                        && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *z.offset(i.wrapping_add(4 as u32_0) as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x6 as ::core::ffi::c_int == 0
                    {
                        jsonBlobAppendOneByte(pParse, JSONB_TRUE as u8_0);
                        return i.wrapping_add(4 as u32_0) as ::core::ffi::c_int;
                    }
                    (*pParse).iErr = i;
                    return -1 as ::core::ffi::c_int;
                }
                102 => {
                    if strncmp(
                        z.offset(i as isize),
                        b"false\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as size_t,
                    ) == 0 as ::core::ffi::c_int
                        && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *z.offset(i.wrapping_add(5 as u32_0) as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x6 as ::core::ffi::c_int == 0
                    {
                        jsonBlobAppendOneByte(pParse, JSONB_FALSE as u8_0);
                        return i.wrapping_add(5 as u32_0) as ::core::ffi::c_int;
                    }
                    (*pParse).iErr = i;
                    return -1 as ::core::ffi::c_int;
                }
                43 => {
                    seenE = 0;
                    (*pParse).hasNonstd = 1 as u8_0;
                    t = 0 as u8_0;
                    c2rust_current_block = 8487799160110596214;
                    break;
                }
                46 => {
                    if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(
                            *z.offset(i.wrapping_add(1 as u32_0) as isize)
                                as ::core::ffi::c_uchar as isize,
                        ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                    {
                        c2rust_current_block = 9270770154621591809;
                        break;
                    } else {
                        c2rust_current_block = 7848525887314104415;
                        break;
                    }
                }
                45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    t = 0 as u8_0;
                    c2rust_current_block = 8487799160110596214;
                    break;
                }
                125 => {
                    (*pParse).iErr = i;
                    return -2 as ::core::ffi::c_int;
                }
                93 => {
                    (*pParse).iErr = i;
                    return -3 as ::core::ffi::c_int;
                }
                44 => {
                    (*pParse).iErr = i;
                    return -4 as ::core::ffi::c_int;
                }
                58 => {
                    (*pParse).iErr = i;
                    return -5 as ::core::ffi::c_int;
                }
                0 => return 0 as ::core::ffi::c_int,
                9 | 10 | 13 | 32 => {
                    i = (i as ::core::ffi::c_uint)
                        .wrapping_add(
                            (1 as u32_0)
                                .wrapping_add(
                                    strspn(
                                        z.offset(i.wrapping_add(1 as u32_0) as isize)
                                            as *const ::core::ffi::c_char,
                                        &raw const jsonSpaces as *const ::core::ffi::c_char,
                                    ) as u32_0,
                                ) as ::core::ffi::c_uint,
                        ) as u32_0 as u32_0;
                }
                11 | 12 | 47 | 194 | 225 | 226 | 227 | 239 => {
                    j = json5Whitespace(
                        z.offset(i as isize) as *const ::core::ffi::c_char,
                    ) as u32_0;
                    if j > 0 as u32_0 {
                        i = (i as ::core::ffi::c_uint)
                            .wrapping_add(j as ::core::ffi::c_uint) as u32_0 as u32_0;
                        (*pParse).hasNonstd = 1 as u8_0;
                    } else {
                        (*pParse).iErr = i;
                        return -1 as ::core::ffi::c_int;
                    }
                }
                110 => {
                    if strncmp(
                        z.offset(i as isize),
                        b"null\0".as_ptr() as *const ::core::ffi::c_char,
                        4 as size_t,
                    ) == 0 as ::core::ffi::c_int
                        && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *z.offset(i.wrapping_add(4 as u32_0) as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x6 as ::core::ffi::c_int == 0
                    {
                        jsonBlobAppendOneByte(pParse, JSONB_NULL as u8_0);
                        return i.wrapping_add(4 as u32_0) as ::core::ffi::c_int;
                    }
                    c2rust_current_block = 13633301081409456424;
                    break;
                }
                _ => {
                    c2rust_current_block = 13633301081409456424;
                    break;
                }
            }
        }
        match c2rust_current_block {
            13633301081409456424 => {
                let mut k_0: u32_0 = 0;
                let mut nn: ::core::ffi::c_int = 0;
                c = *z.offset(i as isize);
                k_0 = 0 as u32_0;
                while (k_0 as usize)
                    < (::core::mem::size_of::<[NanInfName; 5]>() as usize)
                        .wrapping_div(::core::mem::size_of::<NanInfName>() as usize)
                {
                    if !(c as ::core::ffi::c_int
                        != aNanInfName[k_0 as usize].c1 as ::core::ffi::c_int
                        && c as ::core::ffi::c_int
                            != aNanInfName[k_0 as usize].c2 as ::core::ffi::c_int)
                    {
                        nn = aNanInfName[k_0 as usize].n as ::core::ffi::c_int;
                        if !(sqlite3_strnicmp(
                            z.offset(i as isize) as *const ::core::ffi::c_char,
                            aNanInfName[k_0 as usize].zMatch,
                            nn,
                        ) != 0 as ::core::ffi::c_int)
                        {
                            if !(*(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(
                                    *z.offset(i.wrapping_add(nn as u32_0) as isize)
                                        as ::core::ffi::c_uchar as isize,
                                ) as ::core::ffi::c_int & 0x6 as ::core::ffi::c_int != 0)
                            {
                                if aNanInfName[k_0 as usize].eType as ::core::ffi::c_int
                                    == JSONB_FLOAT
                                {
                                    jsonBlobAppendNode(
                                        pParse,
                                        JSONB_FLOAT as u8_0,
                                        5 as u32_0,
                                        b"9e999\0".as_ptr() as *const ::core::ffi::c_char
                                            as *const ::core::ffi::c_void,
                                    );
                                } else {
                                    jsonBlobAppendOneByte(pParse, JSONB_NULL as u8_0);
                                }
                                (*pParse).hasNonstd = 1 as u8_0;
                                return i.wrapping_add(nn as u32_0) as ::core::ffi::c_int;
                            }
                        }
                    }
                    k_0 = k_0.wrapping_add(1);
                }
                (*pParse).iErr = i;
                return -1 as ::core::ffi::c_int;
            }
            7848525887314104415 => {
                (*pParse).iErr = i;
                return -1 as ::core::ffi::c_int;
            }
            9270770154621591809 => {
                (*pParse).hasNonstd = 1 as u8_0;
                t = 0x3 as u8_0;
                seenE = 0 as u8_0;
                c2rust_current_block = 794937143156254362;
            }
            16508451873000240240 => {
                cDelim = *z.offset(i as isize);
                j = i.wrapping_add(1 as u32_0);
                loop {
                    if jsonIsOk[*z.offset(j as isize) as u8_0 as usize] != 0 {
                        if jsonIsOk[*z.offset(j.wrapping_add(1 as u32_0) as isize)
                            as u8_0 as usize] == 0
                        {
                            j = (j as ::core::ffi::c_uint)
                                .wrapping_add(1 as ::core::ffi::c_uint) as u32_0 as u32_0;
                        } else if jsonIsOk[*z.offset(j.wrapping_add(2 as u32_0) as isize)
                            as u8_0 as usize] == 0
                        {
                            j = (j as ::core::ffi::c_uint)
                                .wrapping_add(2 as ::core::ffi::c_uint) as u32_0 as u32_0;
                        } else {
                            j = (j as ::core::ffi::c_uint)
                                .wrapping_add(3 as ::core::ffi::c_uint) as u32_0 as u32_0;
                            continue;
                        }
                    }
                    c = *z.offset(j as isize);
                    if c as ::core::ffi::c_int == cDelim as ::core::ffi::c_int {
                        break;
                    }
                    if c as ::core::ffi::c_int == '\\' as i32 {
                        j = j.wrapping_add(1);
                        c = *z.offset(j as isize);
                        if c as ::core::ffi::c_int == '"' as i32
                            || c as ::core::ffi::c_int == '\\' as i32
                            || c as ::core::ffi::c_int == '/' as i32
                            || c as ::core::ffi::c_int == 'b' as i32
                            || c as ::core::ffi::c_int == 'f' as i32
                            || c as ::core::ffi::c_int == 'n' as i32
                            || c as ::core::ffi::c_int == 'r' as i32
                            || c as ::core::ffi::c_int == 't' as i32
                            || c as ::core::ffi::c_int == 'u' as i32
                                && jsonIs4Hex(
                                    z.offset(j.wrapping_add(1 as u32_0) as isize)
                                        as *const ::core::ffi::c_char,
                                ) != 0
                        {
                            if opcode as ::core::ffi::c_int == JSONB_TEXT {
                                opcode = JSONB_TEXTJ as u8_0;
                            }
                        } else if c as ::core::ffi::c_int == '\'' as i32
                            || c as ::core::ffi::c_int == 'v' as i32
                            || c as ::core::ffi::c_int == '\n' as i32
                            || c as ::core::ffi::c_int == '0' as i32
                                && *(&raw const sqlite3CtypeMap
                                    as *const ::core::ffi::c_uchar)
                                    .offset(
                                        *z.offset(j.wrapping_add(1 as u32_0) as isize)
                                            as ::core::ffi::c_uchar as isize,
                                    ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
                            || 0xe2 as ::core::ffi::c_int
                                == c as u8_0 as ::core::ffi::c_int
                                && 0x80 as ::core::ffi::c_int
                                    == *z.offset(j.wrapping_add(1 as u32_0) as isize) as u8_0
                                        as ::core::ffi::c_int
                                && (0xa8 as ::core::ffi::c_int
                                    == *z.offset(j.wrapping_add(2 as u32_0) as isize) as u8_0
                                        as ::core::ffi::c_int
                                    || 0xa9 as ::core::ffi::c_int
                                        == *z.offset(j.wrapping_add(2 as u32_0) as isize) as u8_0
                                            as ::core::ffi::c_int)
                            || c as ::core::ffi::c_int == 'x' as i32
                                && jsonIs2Hex(
                                    z.offset(j.wrapping_add(1 as u32_0) as isize)
                                        as *const ::core::ffi::c_char,
                                ) != 0
                        {
                            opcode = JSONB_TEXT5 as u8_0;
                            (*pParse).hasNonstd = 1 as u8_0;
                        } else if c as ::core::ffi::c_int == '\r' as i32 {
                            if *z.offset(j.wrapping_add(1 as u32_0) as isize)
                                as ::core::ffi::c_int == '\n' as i32
                            {
                                j = j.wrapping_add(1);
                            }
                            opcode = JSONB_TEXT5 as u8_0;
                            (*pParse).hasNonstd = 1 as u8_0;
                        } else {
                            (*pParse).iErr = j;
                            return -1 as ::core::ffi::c_int;
                        }
                    } else if c as ::core::ffi::c_int <= 0x1f as ::core::ffi::c_int {
                        if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            (*pParse).iErr = j;
                            return -1 as ::core::ffi::c_int;
                        }
                        opcode = JSONB_TEXT5 as u8_0;
                        (*pParse).hasNonstd = 1 as u8_0;
                    } else if c as ::core::ffi::c_int == '"' as i32 {
                        opcode = JSONB_TEXT5 as u8_0;
                    }
                    j = j.wrapping_add(1);
                }
                jsonBlobAppendNode(
                    pParse,
                    opcode,
                    j.wrapping_sub(1 as u32_0).wrapping_sub(i),
                    z.offset(i.wrapping_add(1 as u32_0) as isize)
                        as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                );
                return j.wrapping_add(1 as u32_0) as ::core::ffi::c_int;
            }
            _ => {
                seenE = 0 as u8_0;
                c = *z.offset(i as isize);
                if c as ::core::ffi::c_int <= '0' as i32 {
                    if c as ::core::ffi::c_int == '0' as i32 {
                        if (*z.offset(i.wrapping_add(1 as u32_0) as isize)
                            as ::core::ffi::c_int == 'x' as i32
                            || *z.offset(i.wrapping_add(1 as u32_0) as isize)
                                as ::core::ffi::c_int == 'X' as i32)
                            && *(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(
                                    *z.offset(i.wrapping_add(2 as u32_0) as isize)
                                        as ::core::ffi::c_uchar as isize,
                                ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                        {
                            (*pParse).hasNonstd = 1 as u8_0;
                            t = 0x1 as u8_0;
                            j = i.wrapping_add(3 as u32_0);
                            while *(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(
                                    *z.offset(j as isize) as ::core::ffi::c_uchar as isize,
                                ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                            {
                                j = j.wrapping_add(1);
                            }
                            c2rust_current_block = 5798072534372498777;
                        } else {
                            if *(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(
                                    *z.offset(i.wrapping_add(1 as u32_0) as isize)
                                        as ::core::ffi::c_uchar as isize,
                                ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                            {
                                (*pParse).iErr = i.wrapping_add(1 as u32_0);
                                return -1 as ::core::ffi::c_int;
                            }
                            c2rust_current_block = 794937143156254362;
                        }
                    } else if *(&raw const sqlite3CtypeMap
                        as *const ::core::ffi::c_uchar)
                        .offset(
                            *z.offset(i.wrapping_add(1 as u32_0) as isize)
                                as ::core::ffi::c_uchar as isize,
                        ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
                    {
                        if (*z.offset(i.wrapping_add(1 as u32_0) as isize)
                            as ::core::ffi::c_int == 'I' as i32
                            || *z.offset(i.wrapping_add(1 as u32_0) as isize)
                                as ::core::ffi::c_int == 'i' as i32)
                            && sqlite3_strnicmp(
                                z.offset(i.wrapping_add(1 as u32_0) as isize)
                                    as *const ::core::ffi::c_char,
                                b"inf\0".as_ptr() as *const ::core::ffi::c_char,
                                3 as ::core::ffi::c_int,
                            ) == 0 as ::core::ffi::c_int
                        {
                            (*pParse).hasNonstd = 1 as u8_0;
                            if *z.offset(i as isize) as ::core::ffi::c_int == '-' as i32
                            {
                                jsonBlobAppendNode(
                                    pParse,
                                    JSONB_FLOAT as u8_0,
                                    6 as u32_0,
                                    b"-9e999\0".as_ptr() as *const ::core::ffi::c_char
                                        as *const ::core::ffi::c_void,
                                );
                            } else {
                                jsonBlobAppendNode(
                                    pParse,
                                    JSONB_FLOAT as u8_0,
                                    5 as u32_0,
                                    b"9e999\0".as_ptr() as *const ::core::ffi::c_char
                                        as *const ::core::ffi::c_void,
                                );
                            }
                            return i
                                .wrapping_add(
                                    (if sqlite3_strnicmp(
                                        z.offset(i.wrapping_add(4 as u32_0) as isize)
                                            as *const ::core::ffi::c_char,
                                        b"inity\0".as_ptr() as *const ::core::ffi::c_char,
                                        5 as ::core::ffi::c_int,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        9 as ::core::ffi::c_int
                                    } else {
                                        4 as ::core::ffi::c_int
                                    }) as u32_0,
                                ) as ::core::ffi::c_int;
                        }
                        if *z.offset(i.wrapping_add(1 as u32_0) as isize)
                            as ::core::ffi::c_int == '.' as i32
                        {
                            (*pParse).hasNonstd = 1 as u8_0;
                            t = (t as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
                                as u8_0;
                        } else {
                            (*pParse).iErr = i;
                            return -1 as ::core::ffi::c_int;
                        }
                        c2rust_current_block = 794937143156254362;
                    } else if *z.offset(i.wrapping_add(1 as u32_0) as isize)
                        as ::core::ffi::c_int == '0' as i32
                    {
                        if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *z.offset(i.wrapping_add(2 as u32_0) as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                        {
                            (*pParse).iErr = i.wrapping_add(1 as u32_0);
                            return -1 as ::core::ffi::c_int;
                        } else if (*z.offset(i.wrapping_add(2 as u32_0) as isize)
                            as ::core::ffi::c_int == 'x' as i32
                            || *z.offset(i.wrapping_add(2 as u32_0) as isize)
                                as ::core::ffi::c_int == 'X' as i32)
                            && *(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(
                                    *z.offset(i.wrapping_add(3 as u32_0) as isize)
                                        as ::core::ffi::c_uchar as isize,
                                ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                        {
                            (*pParse).hasNonstd = 1 as u8_0;
                            t = (t as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
                                as u8_0;
                            j = i.wrapping_add(4 as u32_0);
                            while *(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(
                                    *z.offset(j as isize) as ::core::ffi::c_uchar as isize,
                                ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                            {
                                j = j.wrapping_add(1);
                            }
                            c2rust_current_block = 5798072534372498777;
                        } else {
                            c2rust_current_block = 794937143156254362;
                        }
                    } else {
                        c2rust_current_block = 794937143156254362;
                    }
                } else {
                    c2rust_current_block = 794937143156254362;
                }
            }
        }
        match c2rust_current_block {
            794937143156254362 => {
                j = i.wrapping_add(1 as u32_0);
                loop {
                    c = *z.offset(j as isize);
                    if !(*(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(c as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                        & 0x4 as ::core::ffi::c_int != 0)
                    {
                        if c as ::core::ffi::c_int == '.' as i32 {
                            if t as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int
                            {
                                (*pParse).iErr = j;
                                return -1 as ::core::ffi::c_int;
                            }
                            t = (t as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)
                                as u8_0;
                        } else {
                            if !(c as ::core::ffi::c_int == 'e' as i32
                                || c as ::core::ffi::c_int == 'E' as i32)
                            {
                                break;
                            }
                            if (*z.offset(j.wrapping_sub(1 as u32_0) as isize)
                                as ::core::ffi::c_int) < '0' as i32
                            {
                                if *z.offset(j.wrapping_sub(1 as u32_0) as isize)
                                    as ::core::ffi::c_int == '.' as i32
                                    && j.wrapping_sub(2 as u32_0) >= i
                                    && *(&raw const sqlite3CtypeMap
                                        as *const ::core::ffi::c_uchar)
                                        .offset(
                                            *z.offset(j.wrapping_sub(2 as u32_0) as isize)
                                                as ::core::ffi::c_uchar as isize,
                                        ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                                {
                                    (*pParse).hasNonstd = 1 as u8_0;
                                    t = (t as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
                                        as u8_0;
                                } else {
                                    (*pParse).iErr = j;
                                    return -1 as ::core::ffi::c_int;
                                }
                            }
                            if seenE != 0 {
                                (*pParse).iErr = j;
                                return -1 as ::core::ffi::c_int;
                            }
                            t = (t as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)
                                as u8_0;
                            seenE = 1 as u8_0;
                            c = *z.offset(j.wrapping_add(1 as u32_0) as isize);
                            if c as ::core::ffi::c_int == '+' as i32
                                || c as ::core::ffi::c_int == '-' as i32
                            {
                                j = j.wrapping_add(1);
                                c = *z.offset(j.wrapping_add(1 as u32_0) as isize);
                            }
                            if (c as ::core::ffi::c_int) < '0' as i32
                                || c as ::core::ffi::c_int > '9' as i32
                            {
                                (*pParse).iErr = j;
                                return -1 as ::core::ffi::c_int;
                            }
                        }
                    }
                    j = j.wrapping_add(1);
                }
                if (*z.offset(j.wrapping_sub(1 as u32_0) as isize) as ::core::ffi::c_int)
                    < '0' as i32
                {
                    if *z.offset(j.wrapping_sub(1 as u32_0) as isize)
                        as ::core::ffi::c_int == '.' as i32
                        && j.wrapping_sub(2 as u32_0) >= i
                        && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *z.offset(j.wrapping_sub(2 as u32_0) as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                    {
                        (*pParse).hasNonstd = 1 as u8_0;
                        t = (t as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
                            as u8_0;
                    } else {
                        (*pParse).iErr = j;
                        return -1 as ::core::ffi::c_int;
                    }
                }
            }
            _ => {}
        }
        if *z.offset(i as isize) as ::core::ffi::c_int == '+' as i32 {
            i = i.wrapping_add(1);
        }
        jsonBlobAppendNode(
            pParse,
            (JSONB_INT + t as ::core::ffi::c_int) as u8_0,
            j.wrapping_sub(i),
            z.offset(i as isize) as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
        );
        return j as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn jsonConvertTextToBlob(
    mut pParse: *mut JsonParse,
    mut pCtx: *mut sqlite3_context,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut zJson: *const ::core::ffi::c_char = (*pParse).zJson;
        i = jsonTranslateTextToBlob(pParse, 0 as u32_0);
        if (*pParse).oom != 0 {
            i = -1 as ::core::ffi::c_int;
        }
        if i > 0 as ::core::ffi::c_int {
            while jsonIsSpace[*zJson.offset(i as isize) as ::core::ffi::c_uchar as usize]
                != 0
            {
                i += 1;
            }
            if *zJson.offset(i as isize) != 0 {
                i
                    += json5Whitespace(
                        zJson.offset(i as isize) as *const ::core::ffi::c_char,
                    );
                if *zJson.offset(i as isize) != 0 {
                    if !pCtx.is_null() {
                        sqlite3_result_error(
                            pCtx,
                            b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                        );
                    }
                    jsonParseReset(pParse);
                    return 1 as ::core::ffi::c_int;
                }
                (*pParse).hasNonstd = 1 as u8_0;
            }
        }
        if i <= 0 as ::core::ffi::c_int {
            if !pCtx.is_null() {
                if (*pParse).oom != 0 {
                    sqlite3_result_error_nomem(pCtx);
                } else {
                    sqlite3_result_error(
                        pCtx,
                        b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                }
            }
            jsonParseReset(pParse);
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn jsonReturnStringAsBlob(mut pStr: *mut JsonString) {
    unsafe {
        let mut px: JsonParse = JsonParse {
            aBlob: ::core::ptr::null_mut::<u8_0>(),
            nBlob: 0,
            nBlobAlloc: 0,
            zJson: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            db: ::core::ptr::null_mut::<sqlite3>(),
            nJson: 0,
            nJPRef: 0,
            iErr: 0,
            iDepth: 0,
            nErr: 0,
            oom: 0,
            bJsonIsRCStr: 0,
            hasNonstd: 0,
            bReadOnly: 0,
            eEdit: 0,
            delta: 0,
            nIns: 0,
            iLabel: 0,
            aIns: ::core::ptr::null_mut::<u8_0>()

        };
        memset(
            &raw mut px as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<JsonParse>() as size_t,
        );
        jsonStringTerminate(pStr);
        if (*pStr).eErr != 0 {
            sqlite3_result_error_nomem((*pStr).pCtx);
            return;
        }
        px.zJson = (*pStr).zBuf;
        px.nJson = (*pStr).nUsed as ::core::ffi::c_int;
        px.db = sqlite3_context_db_handle((*pStr).pCtx);
        jsonTranslateTextToBlob(&raw mut px, 0 as u32_0);
        if px.oom != 0 {
            sqlite3DbFree(px.db, px.aBlob as *mut ::core::ffi::c_void);
            sqlite3_result_error_nomem((*pStr).pCtx);
        } else {
            sqlite3_result_blob(
                (*pStr).pCtx,
                px.aBlob as *const ::core::ffi::c_void,
                px.nBlob as ::core::ffi::c_int,
                Some(
                    sqlite3RowSetClear
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        };
    }
}
unsafe extern "C" fn jsonbPayloadSize(
    mut pParse: *const JsonParse,
    mut i: u32_0,
    mut pSz: *mut u32_0,
) -> u32_0 {
    unsafe {
        let mut x: u8_0 = 0;
        let mut sz: u32_0 = 0;
        let mut n: u32_0 = 0;
        x = (*(*pParse).aBlob.offset(i as isize) as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) as u8_0;
        if x as ::core::ffi::c_int <= 11 as ::core::ffi::c_int {
            sz = x as u32_0;
            n = 1 as u32_0;
        } else if x as ::core::ffi::c_int == 12 as ::core::ffi::c_int {
            if i.wrapping_add(1 as u32_0) >= (*pParse).nBlob {
                *pSz = 0 as u32_0;
                return 0 as u32_0;
            }
            sz = *(*pParse).aBlob.offset(i.wrapping_add(1 as u32_0) as isize) as u32_0;
            n = 2 as u32_0;
        } else if x as ::core::ffi::c_int == 13 as ::core::ffi::c_int {
            if i.wrapping_add(2 as u32_0) >= (*pParse).nBlob {
                *pSz = 0 as u32_0;
                return 0 as u32_0;
            }
            sz = (((*(*pParse).aBlob.offset(i.wrapping_add(1 as u32_0) as isize)
                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                + *(*pParse).aBlob.offset(i.wrapping_add(2 as u32_0) as isize)
                    as ::core::ffi::c_int) as u32_0;
            n = 3 as u32_0;
        } else if x as ::core::ffi::c_int == 14 as ::core::ffi::c_int {
            if i.wrapping_add(4 as u32_0) >= (*pParse).nBlob {
                *pSz = 0 as u32_0;
                return 0 as u32_0;
            }
            sz = ((*(*pParse).aBlob.offset(i.wrapping_add(1 as u32_0) as isize) as u32_0)
                << 24 as ::core::ffi::c_int)
                .wrapping_add(
                    ((*(*pParse).aBlob.offset(i.wrapping_add(2 as u32_0) as isize)
                        as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32_0,
                )
                .wrapping_add(
                    ((*(*pParse).aBlob.offset(i.wrapping_add(3 as u32_0) as isize)
                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32_0,
                )
                .wrapping_add(
                    *(*pParse).aBlob.offset(i.wrapping_add(4 as u32_0) as isize) as u32_0,
                );
            n = 5 as u32_0;
        } else {
            if i.wrapping_add(8 as u32_0) >= (*pParse).nBlob
                || *(*pParse).aBlob.offset(i.wrapping_add(1 as u32_0) as isize)
                    as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                || *(*pParse).aBlob.offset(i.wrapping_add(2 as u32_0) as isize)
                    as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                || *(*pParse).aBlob.offset(i.wrapping_add(3 as u32_0) as isize)
                    as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                || *(*pParse).aBlob.offset(i.wrapping_add(4 as u32_0) as isize)
                    as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                *pSz = 0 as u32_0;
                return 0 as u32_0;
            }
            sz = ((*(*pParse).aBlob.offset(i.wrapping_add(5 as u32_0) as isize) as u32_0)
                << 24 as ::core::ffi::c_int)
                .wrapping_add(
                    ((*(*pParse).aBlob.offset(i.wrapping_add(6 as u32_0) as isize)
                        as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32_0,
                )
                .wrapping_add(
                    ((*(*pParse).aBlob.offset(i.wrapping_add(7 as u32_0) as isize)
                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32_0,
                )
                .wrapping_add(
                    *(*pParse).aBlob.offset(i.wrapping_add(8 as u32_0) as isize) as u32_0,
                );
            n = 9 as u32_0;
        }
        if i as ::core::ffi::c_longlong + sz as ::core::ffi::c_longlong
            + n as ::core::ffi::c_longlong > (*pParse).nBlob as ::core::ffi::c_longlong
            && i as ::core::ffi::c_longlong + sz as ::core::ffi::c_longlong
                + n as ::core::ffi::c_longlong
                > (*pParse).nBlob.wrapping_sub((*pParse).delta as u32_0)
                    as ::core::ffi::c_longlong
        {
            *pSz = 0 as u32_0;
            return 0 as u32_0;
        }
        *pSz = sz;
        return n;
    }
}
unsafe extern "C" fn jsonTranslateBlobToText(
    mut pParse: *const JsonParse,
    mut i: u32_0,
    mut pOut: *mut JsonString,
) -> u32_0 {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut sz: u32_0 = 0;
        let mut n: u32_0 = 0;
        let mut j: u32_0 = 0;
        let mut iEnd: u32_0 = 0;
        n = jsonbPayloadSize(pParse, i, &raw mut sz);
        if n == 0 as u32_0 {
            (*pOut).eErr = ((*pOut).eErr as ::core::ffi::c_int | JSTRING_MALFORMED)
                as u8_0;
            return (*pParse).nBlob.wrapping_add(1 as u32_0);
        }
        match *(*pParse).aBlob.offset(i as isize) as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int
        {
            JSONB_NULL => {
                jsonAppendRawNZ(
                    pOut,
                    b"null\0".as_ptr() as *const ::core::ffi::c_char,
                    4 as u32_0,
                );
                return i.wrapping_add(1 as u32_0);
            }
            JSONB_TRUE => {
                jsonAppendRawNZ(
                    pOut,
                    b"true\0".as_ptr() as *const ::core::ffi::c_char,
                    4 as u32_0,
                );
                return i.wrapping_add(1 as u32_0);
            }
            JSONB_FALSE => {
                jsonAppendRawNZ(
                    pOut,
                    b"false\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as u32_0,
                );
                return i.wrapping_add(1 as u32_0);
            }
            JSONB_INT | JSONB_FLOAT => {
                if sz == 0 as u32_0 {
                    c2rust_current_block = 14943419880104871395;
                } else {
                    jsonAppendRaw(
                        pOut,
                        (*pParse).aBlob.offset(i.wrapping_add(n) as isize) as *mut u8_0
                            as *const ::core::ffi::c_char,
                        sz,
                    );
                    c2rust_current_block = 9180031981464905198;
                }
            }
            JSONB_INT5 => {
                let mut k: u32_0 = 2 as u32_0;
                let mut u: sqlite3_uint64 = 0 as sqlite3_uint64;
                let mut zIn: *const ::core::ffi::c_char = (*pParse)
                    .aBlob
                    .offset(i.wrapping_add(n) as isize) as *mut u8_0
                    as *const ::core::ffi::c_char;
                let mut bOverflow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if sz == 0 as u32_0 {
                    c2rust_current_block = 14943419880104871395;
                } else {
                    if *zIn.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '-' as i32
                    {
                        jsonAppendChar(pOut, '-' as i32 as ::core::ffi::c_char);
                        k = k.wrapping_add(1);
                    } else if *zIn.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '+' as i32
                    {
                        k = k.wrapping_add(1);
                    }
                    while k < sz {
                        if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *zIn.offset(k as isize) as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0
                        {
                            (*pOut).eErr = ((*pOut).eErr as ::core::ffi::c_int
                                | JSTRING_MALFORMED) as u8_0;
                            break;
                        } else {
                            if u >> 60 as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_ulonglong
                            {
                                bOverflow = 1 as ::core::ffi::c_int;
                            } else {
                                u = (u as ::core::ffi::c_ulonglong)
                                    .wrapping_mul(16 as ::core::ffi::c_ulonglong)
                                    .wrapping_add(
                                        sqlite3HexToInt(
                                            *zIn.offset(k as isize) as ::core::ffi::c_int,
                                        ) as ::core::ffi::c_ulonglong,
                                    ) as sqlite3_uint64;
                            }
                            k = k.wrapping_add(1);
                        }
                    }
                    jsonPrintf(
                        100 as ::core::ffi::c_int,
                        pOut,
                        if bOverflow != 0 {
                            b"9.0e999\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"%llu\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        u,
                    );
                    c2rust_current_block = 9180031981464905198;
                }
            }
            JSONB_FLOAT5 => {
                let mut k_0: u32_0 = 0 as u32_0;
                let mut zIn_0: *const ::core::ffi::c_char = (*pParse)
                    .aBlob
                    .offset(i.wrapping_add(n) as isize) as *mut u8_0
                    as *const ::core::ffi::c_char;
                if sz == 0 as u32_0 {
                    c2rust_current_block = 14943419880104871395;
                } else {
                    if *zIn_0.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '-' as i32
                    {
                        jsonAppendChar(pOut, '-' as i32 as ::core::ffi::c_char);
                        k_0 = k_0.wrapping_add(1);
                    }
                    if *zIn_0.offset(k_0 as isize) as ::core::ffi::c_int == '.' as i32 {
                        jsonAppendChar(pOut, '0' as i32 as ::core::ffi::c_char);
                    }
                    while k_0 < sz {
                        jsonAppendChar(pOut, *zIn_0.offset(k_0 as isize));
                        if *zIn_0.offset(k_0 as isize) as ::core::ffi::c_int
                            == '.' as i32
                            && (k_0.wrapping_add(1 as u32_0) == sz
                                || *(&raw const sqlite3CtypeMap
                                    as *const ::core::ffi::c_uchar)
                                    .offset(
                                        *zIn_0.offset(k_0.wrapping_add(1 as u32_0) as isize)
                                            as ::core::ffi::c_uchar as isize,
                                    ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0)
                        {
                            jsonAppendChar(pOut, '0' as i32 as ::core::ffi::c_char);
                        }
                        k_0 = k_0.wrapping_add(1);
                    }
                    c2rust_current_block = 9180031981464905198;
                }
            }
            JSONB_TEXT | JSONB_TEXTJ => {
                if ((*pOut).nUsed as ::core::ffi::c_ulonglong)
                    .wrapping_add(sz as ::core::ffi::c_ulonglong)
                    .wrapping_add(2 as ::core::ffi::c_ulonglong) <= (*pOut).nAlloc
                    || jsonStringGrow(pOut, sz.wrapping_add(2 as u32_0))
                        == 0 as ::core::ffi::c_int
                {
                    *(*pOut).zBuf.offset((*pOut).nUsed as isize) = '"' as i32
                        as ::core::ffi::c_char;
                    memcpy(
                        (*pOut)
                            .zBuf
                            .offset((*pOut).nUsed as isize)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_void,
                        (*pParse).aBlob.offset(i.wrapping_add(n) as isize) as *mut u8_0
                            as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        sz as size_t,
                    );
                    *(*pOut)
                        .zBuf
                        .offset(
                            ((*pOut).nUsed as ::core::ffi::c_ulonglong)
                                .wrapping_add(sz as ::core::ffi::c_ulonglong)
                                .wrapping_add(1 as ::core::ffi::c_ulonglong) as isize,
                        ) = '"' as i32 as ::core::ffi::c_char;
                    (*pOut).nUsed = ((*pOut).nUsed as ::core::ffi::c_ulonglong)
                        .wrapping_add(
                            sz.wrapping_add(2 as u32_0) as ::core::ffi::c_ulonglong,
                        ) as u64_0 as u64_0;
                }
                c2rust_current_block = 9180031981464905198;
            }
            JSONB_TEXT5 => {
                let mut zIn_1: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut k_1: u32_0 = 0;
                let mut sz2: u32_0 = sz;
                zIn_1 = (*pParse).aBlob.offset(i.wrapping_add(n) as isize) as *mut u8_0
                    as *const ::core::ffi::c_char;
                jsonAppendChar(pOut, '"' as i32 as ::core::ffi::c_char);
                while sz2 > 0 as u32_0 {
                    k_1 = 0 as u32_0;
                    while k_1 < sz2
                        && (jsonIsOk[*zIn_1.offset(k_1 as isize) as u8_0 as usize]
                            as ::core::ffi::c_int != 0
                            || *zIn_1.offset(k_1 as isize) as ::core::ffi::c_int
                                == '\'' as i32)
                    {
                        k_1 = k_1.wrapping_add(1);
                    }
                    if k_1 > 0 as u32_0 {
                        jsonAppendRawNZ(pOut, zIn_1, k_1);
                        if k_1 >= sz2 {
                            break;
                        }
                        zIn_1 = zIn_1.offset(k_1 as isize);
                        sz2 = (sz2 as ::core::ffi::c_uint)
                            .wrapping_sub(k_1 as ::core::ffi::c_uint) as u32_0 as u32_0;
                    }
                    if *zIn_1.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '"' as i32
                    {
                        jsonAppendRawNZ(
                            pOut,
                            b"\\\"\0".as_ptr() as *const ::core::ffi::c_char,
                            2 as u32_0,
                        );
                        zIn_1 = zIn_1.offset(1);
                        sz2 = sz2.wrapping_sub(1);
                    } else if *zIn_1.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int <= 0x1f as ::core::ffi::c_int
                    {
                        if ((*pOut).nUsed as ::core::ffi::c_ulonglong)
                            .wrapping_add(7 as ::core::ffi::c_ulonglong) > (*pOut).nAlloc
                            && jsonStringGrow(pOut, 7 as u32_0) != 0
                        {
                            break;
                        }
                        jsonAppendControlChar(
                            pOut,
                            *zIn_1.offset(0 as ::core::ffi::c_int as isize) as u8_0,
                        );
                        zIn_1 = zIn_1.offset(1);
                        sz2 = sz2.wrapping_sub(1);
                    } else if sz2 < 2 as u32_0 {
                        (*pOut).eErr = ((*pOut).eErr as ::core::ffi::c_int
                            | JSTRING_MALFORMED) as u8_0;
                        break;
                    } else {
                        match *zIn_1.offset(1 as ::core::ffi::c_int as isize) as u8_0
                            as ::core::ffi::c_int
                        {
                            39 => {
                                jsonAppendChar(pOut, '\'' as i32 as ::core::ffi::c_char);
                            }
                            118 => {
                                jsonAppendRawNZ(
                                    pOut,
                                    b"\\u000b\0".as_ptr() as *const ::core::ffi::c_char,
                                    6 as u32_0,
                                );
                            }
                            120 => {
                                if sz2 < 4 as u32_0 {
                                    (*pOut).eErr = ((*pOut).eErr as ::core::ffi::c_int
                                        | JSTRING_MALFORMED) as u8_0;
                                    sz2 = 2 as u32_0;
                                } else {
                                    jsonAppendRawNZ(
                                        pOut,
                                        b"\\u00\0".as_ptr() as *const ::core::ffi::c_char,
                                        4 as u32_0,
                                    );
                                    jsonAppendRawNZ(
                                        pOut,
                                        zIn_1.offset(2 as ::core::ffi::c_int as isize)
                                            as *const ::core::ffi::c_char,
                                        2 as u32_0,
                                    );
                                    zIn_1 = zIn_1.offset(2 as ::core::ffi::c_int as isize);
                                    sz2 = (sz2 as ::core::ffi::c_uint)
                                        .wrapping_sub(2 as ::core::ffi::c_uint) as u32_0 as u32_0;
                                }
                            }
                            48 => {
                                jsonAppendRawNZ(
                                    pOut,
                                    b"\\u0000\0".as_ptr() as *const ::core::ffi::c_char,
                                    6 as u32_0,
                                );
                            }
                            13 => {
                                if sz2 > 2 as u32_0
                                    && *zIn_1.offset(2 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int == '\n' as i32
                                {
                                    zIn_1 = zIn_1.offset(1);
                                    sz2 = sz2.wrapping_sub(1);
                                }
                            }
                            10 => {}
                            226 => {
                                if sz2 < 4 as u32_0
                                    || 0x80 as ::core::ffi::c_int
                                        != *zIn_1.offset(2 as ::core::ffi::c_int as isize) as u8_0
                                            as ::core::ffi::c_int
                                    || 0xa8 as ::core::ffi::c_int
                                        != *zIn_1.offset(3 as ::core::ffi::c_int as isize) as u8_0
                                            as ::core::ffi::c_int
                                        && 0xa9 as ::core::ffi::c_int
                                            != *zIn_1.offset(3 as ::core::ffi::c_int as isize) as u8_0
                                                as ::core::ffi::c_int
                                {
                                    (*pOut).eErr = ((*pOut).eErr as ::core::ffi::c_int
                                        | JSTRING_MALFORMED) as u8_0;
                                    sz2 = 2 as u32_0;
                                } else {
                                    zIn_1 = zIn_1.offset(2 as ::core::ffi::c_int as isize);
                                    sz2 = (sz2 as ::core::ffi::c_uint)
                                        .wrapping_sub(2 as ::core::ffi::c_uint) as u32_0 as u32_0;
                                }
                            }
                            _ => {
                                jsonAppendRawNZ(pOut, zIn_1, 2 as u32_0);
                            }
                        }
                        zIn_1 = zIn_1.offset(2 as ::core::ffi::c_int as isize);
                        sz2 = (sz2 as ::core::ffi::c_uint)
                            .wrapping_sub(2 as ::core::ffi::c_uint) as u32_0 as u32_0;
                    }
                }
                jsonAppendChar(pOut, '"' as i32 as ::core::ffi::c_char);
                c2rust_current_block = 9180031981464905198;
            }
            JSONB_TEXTRAW => {
                jsonAppendString(
                    pOut,
                    (*pParse).aBlob.offset(i.wrapping_add(n) as isize) as *mut u8_0
                        as *const ::core::ffi::c_char,
                    sz,
                );
                c2rust_current_block = 9180031981464905198;
            }
            JSONB_ARRAY => {
                jsonAppendChar(pOut, '[' as i32 as ::core::ffi::c_char);
                j = i.wrapping_add(n);
                iEnd = j.wrapping_add(sz);
                while j < iEnd
                    && (*pOut).eErr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    j = jsonTranslateBlobToText(pParse, j, pOut);
                    jsonAppendChar(pOut, ',' as i32 as ::core::ffi::c_char);
                }
                if j > iEnd {
                    (*pOut).eErr = ((*pOut).eErr as ::core::ffi::c_int
                        | JSTRING_MALFORMED) as u8_0;
                }
                if sz > 0 as u32_0 {
                    jsonStringTrimOneChar(pOut);
                }
                jsonAppendChar(pOut, ']' as i32 as ::core::ffi::c_char);
                c2rust_current_block = 9180031981464905198;
            }
            JSONB_OBJECT => {
                let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                jsonAppendChar(pOut, '{' as i32 as ::core::ffi::c_char);
                j = i.wrapping_add(n);
                iEnd = j.wrapping_add(sz);
                while j < iEnd
                    && (*pOut).eErr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    j = jsonTranslateBlobToText(pParse, j, pOut);
                    let c2rust_fresh4 = x;
                    x = x + 1;
                    jsonAppendChar(
                        pOut,
                        (if c2rust_fresh4 & 1 as ::core::ffi::c_int != 0 {
                            ',' as i32
                        } else {
                            ':' as i32
                        }) as ::core::ffi::c_char,
                    );
                }
                if x & 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int || j > iEnd {
                    (*pOut).eErr = ((*pOut).eErr as ::core::ffi::c_int
                        | JSTRING_MALFORMED) as u8_0;
                }
                if sz > 0 as u32_0 {
                    jsonStringTrimOneChar(pOut);
                }
                jsonAppendChar(pOut, '}' as i32 as ::core::ffi::c_char);
                c2rust_current_block = 9180031981464905198;
            }
            _ => {
                c2rust_current_block = 14943419880104871395;
            }
        }
        match c2rust_current_block {
            14943419880104871395 => {
                (*pOut).eErr = ((*pOut).eErr as ::core::ffi::c_int | JSTRING_MALFORMED)
                    as u8_0;
            }
            _ => {}
        }
        return i.wrapping_add(n).wrapping_add(sz);
    }
}
unsafe extern "C" fn jsonPrettyIndent(mut pPretty: *mut JsonPretty) {
    unsafe {
        let mut jj: u32_0 = 0;
        jj = 0 as u32_0;
        while jj < (*pPretty).nIndent {
            jsonAppendRaw((*pPretty).pOut, (*pPretty).zIndent, (*pPretty).szIndent);
            jj = jj.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn jsonTranslateBlobToPrettyText(
    mut pPretty: *mut JsonPretty,
    mut i: u32_0,
) -> u32_0 {
    unsafe {
        let mut sz: u32_0 = 0;
        let mut n: u32_0 = 0;
        let mut j: u32_0 = 0;
        let mut iEnd: u32_0 = 0;
        let mut pParse: *const JsonParse = (*pPretty).pParse;
        let mut pOut: *mut JsonString = (*pPretty).pOut;
        n = jsonbPayloadSize(pParse, i, &raw mut sz);
        if n == 0 as u32_0 {
            (*pOut).eErr = ((*pOut).eErr as ::core::ffi::c_int | JSTRING_MALFORMED)
                as u8_0;
            return (*pParse).nBlob.wrapping_add(1 as u32_0);
        }
        match *(*pParse).aBlob.offset(i as isize) as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int
        {
            JSONB_ARRAY => {
                j = i.wrapping_add(n);
                iEnd = j.wrapping_add(sz);
                jsonAppendChar(pOut, '[' as i32 as ::core::ffi::c_char);
                if j < iEnd {
                    jsonAppendChar(pOut, '\n' as i32 as ::core::ffi::c_char);
                    (*pPretty).nIndent = (*pPretty).nIndent.wrapping_add(1);
                    while (*pOut).eErr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        jsonPrettyIndent(pPretty);
                        j = jsonTranslateBlobToPrettyText(pPretty, j);
                        if j >= iEnd {
                            break;
                        }
                        jsonAppendRawNZ(
                            pOut,
                            b",\n\0".as_ptr() as *const ::core::ffi::c_char,
                            2 as u32_0,
                        );
                    }
                    jsonAppendChar(pOut, '\n' as i32 as ::core::ffi::c_char);
                    (*pPretty).nIndent = (*pPretty).nIndent.wrapping_sub(1);
                    jsonPrettyIndent(pPretty);
                }
                jsonAppendChar(pOut, ']' as i32 as ::core::ffi::c_char);
                i = iEnd;
            }
            JSONB_OBJECT => {
                j = i.wrapping_add(n);
                iEnd = j.wrapping_add(sz);
                jsonAppendChar(pOut, '{' as i32 as ::core::ffi::c_char);
                if j < iEnd {
                    jsonAppendChar(pOut, '\n' as i32 as ::core::ffi::c_char);
                    (*pPretty).nIndent = (*pPretty).nIndent.wrapping_add(1);
                    while (*pOut).eErr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        jsonPrettyIndent(pPretty);
                        j = jsonTranslateBlobToText(pParse, j, pOut);
                        if j > iEnd {
                            (*pOut).eErr = ((*pOut).eErr as ::core::ffi::c_int
                                | JSTRING_MALFORMED) as u8_0;
                            break;
                        } else {
                            jsonAppendRawNZ(
                                pOut,
                                b": \0".as_ptr() as *const ::core::ffi::c_char,
                                2 as u32_0,
                            );
                            j = jsonTranslateBlobToPrettyText(pPretty, j);
                            if j >= iEnd {
                                break;
                            }
                            jsonAppendRawNZ(
                                pOut,
                                b",\n\0".as_ptr() as *const ::core::ffi::c_char,
                                2 as u32_0,
                            );
                        }
                    }
                    jsonAppendChar(pOut, '\n' as i32 as ::core::ffi::c_char);
                    (*pPretty).nIndent = (*pPretty).nIndent.wrapping_sub(1);
                    jsonPrettyIndent(pPretty);
                }
                jsonAppendChar(pOut, '}' as i32 as ::core::ffi::c_char);
                i = iEnd;
            }
            _ => {
                i = jsonTranslateBlobToText(pParse, i, pOut);
            }
        }
        return i;
    }
}
unsafe extern "C" fn jsonbArrayCount(
    mut pParse: *mut JsonParse,
    mut iRoot: u32_0,
) -> u32_0 {
    unsafe {
        let mut n: u32_0 = 0;
        let mut sz: u32_0 = 0;
        let mut i: u32_0 = 0;
        let mut iEnd: u32_0 = 0;
        let mut k: u32_0 = 0 as u32_0;
        n = jsonbPayloadSize(pParse, iRoot, &raw mut sz);
        iEnd = iRoot.wrapping_add(n).wrapping_add(sz);
        i = iRoot.wrapping_add(n);
        while n > 0 as u32_0 && i < iEnd {
            n = jsonbPayloadSize(pParse, i, &raw mut sz);
            i = (i as ::core::ffi::c_uint)
                .wrapping_add(sz.wrapping_add(n) as ::core::ffi::c_uint) as u32_0
                as u32_0;
            k = k.wrapping_add(1);
        }
        return k;
    }
}
unsafe extern "C" fn jsonAfterEditSizeAdjust(
    mut pParse: *mut JsonParse,
    mut iRoot: u32_0,
) {
    unsafe {
        let mut sz: u32_0 = 0 as u32_0;
        let mut nBlob: u32_0 = 0;
        nBlob = (*pParse).nBlob;
        (*pParse).nBlob = (*pParse).nBlobAlloc;
        jsonbPayloadSize(pParse, iRoot, &raw mut sz);
        (*pParse).nBlob = nBlob;
        sz = (sz as ::core::ffi::c_uint)
            .wrapping_add((*pParse).delta as ::core::ffi::c_uint) as u32_0 as u32_0;
        (*pParse).delta += jsonBlobChangePayloadSize(pParse, iRoot, sz);
    }
}
unsafe extern "C" fn jsonBlobOverwrite(
    mut aOut: *mut u8_0,
    mut aIns: *const u8_0,
    mut nIns: u32_0,
    mut d: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut szPayload: u32_0 = 0;
        let mut i: u32_0 = 0;
        let mut szHdr: u8_0 = 0;
        static mut aType: [u8_0; 8] = [
            0xc0 as ::core::ffi::c_int as u8_0,
            0xd0 as ::core::ffi::c_int as u8_0,
            0 as ::core::ffi::c_int as u8_0,
            0xe0 as ::core::ffi::c_int as u8_0,
            0 as ::core::ffi::c_int as u8_0,
            0 as ::core::ffi::c_int as u8_0,
            0 as ::core::ffi::c_int as u8_0,
            0xf0 as ::core::ffi::c_int as u8_0,
        ];
        if *aIns.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int <= 2 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        match *aIns.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int
        {
            12 => {
                if (1 as ::core::ffi::c_int) << d & 0x8a as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
                i = d.wrapping_add(2 as u32_0);
                szHdr = 2 as u8_0;
            }
            13 => {
                if d != 2 as u32_0 && d != 6 as u32_0 {
                    return 0 as ::core::ffi::c_int;
                }
                i = d.wrapping_add(3 as u32_0);
                szHdr = 3 as u8_0;
            }
            14 => {
                if d != 4 as u32_0 {
                    return 0 as ::core::ffi::c_int;
                }
                i = 9 as u32_0;
                szHdr = 5 as u8_0;
            }
            15 => return 0 as ::core::ffi::c_int,
            _ => {
                if (1 as ::core::ffi::c_int) << d & 0x116 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
                i = d.wrapping_add(1 as u32_0);
                szHdr = 1 as u8_0;
            }
        }
        *aOut.offset(0 as ::core::ffi::c_int as isize) = (*aIns
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int
            | aType[i.wrapping_sub(2 as u32_0) as usize] as ::core::ffi::c_int) as u8_0;
        memcpy(
            aOut.offset(i as isize) as *mut u8_0 as *mut ::core::ffi::c_void,
            aIns.offset(szHdr as isize) as *const u8_0 as *const ::core::ffi::c_void,
            nIns.wrapping_sub(szHdr as u32_0) as size_t,
        );
        szPayload = nIns.wrapping_sub(szHdr as u32_0);
        loop {
            i = i.wrapping_sub(1);
            *aOut.offset(i as isize) = (szPayload & 0xff as u32_0) as u8_0;
            if i == 1 as u32_0 {
                break;
            }
            szPayload >>= 8 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn jsonBlobEdit(
    mut pParse: *mut JsonParse,
    mut iDel: u32_0,
    mut nDel: u32_0,
    mut aIns: *const u8_0,
    mut nIns: u32_0,
) {
    unsafe {
        let mut d: i64_0 = nIns as i64_0 - nDel as i64_0;
        if d < 0 as ::core::ffi::c_longlong
            && d >= -8 as ::core::ffi::c_int as ::core::ffi::c_longlong
            && !aIns.is_null()
            && jsonBlobOverwrite(
                (*pParse).aBlob.offset(iDel as isize) as *mut u8_0,
                aIns,
                nIns,
                -d as ::core::ffi::c_int as u32_0,
            ) != 0
        {
            return;
        }
        if d != 0 as ::core::ffi::c_longlong {
            if (*pParse).nBlob as i64_0 + d
                > (*pParse).nBlobAlloc as ::core::ffi::c_longlong
            {
                jsonBlobExpand(pParse, ((*pParse).nBlob as i64_0 + d) as u32_0);
                if (*pParse).oom != 0 {
                    return;
                }
            }
            memmove(
                (*pParse).aBlob.offset(iDel.wrapping_add(nIns) as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void,
                (*pParse).aBlob.offset(iDel.wrapping_add(nDel) as isize) as *mut u8_0
                    as *const ::core::ffi::c_void,
                (*pParse).nBlob.wrapping_sub(iDel.wrapping_add(nDel)) as size_t,
            );
            (*pParse).nBlob = ((*pParse).nBlob as ::core::ffi::c_longlong
                + d as ::core::ffi::c_longlong) as u32_0;
            (*pParse).delta = ((*pParse).delta as ::core::ffi::c_longlong
                + d as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        }
        if nIns != 0 && !aIns.is_null() {
            memcpy(
                (*pParse).aBlob.offset(iDel as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void,
                aIns as *const ::core::ffi::c_void,
                nIns as size_t,
            );
        }
    }
}
unsafe extern "C" fn jsonBytesToBypass(
    mut z: *const ::core::ffi::c_char,
    mut n: u32_0,
) -> u32_0 {
    unsafe {
        let mut i: u32_0 = 0 as u32_0;
        while i.wrapping_add(1 as u32_0) < n {
            if *z.offset(i as isize) as ::core::ffi::c_int != '\\' as i32 {
                return i;
            }
            if *z.offset(i.wrapping_add(1 as u32_0) as isize) as ::core::ffi::c_int
                == '\n' as i32
            {
                i = (i as ::core::ffi::c_uint).wrapping_add(2 as ::core::ffi::c_uint)
                    as u32_0 as u32_0;
            } else if *z.offset(i.wrapping_add(1 as u32_0) as isize)
                as ::core::ffi::c_int == '\r' as i32
            {
                if i.wrapping_add(2 as u32_0) < n
                    && *z.offset(i.wrapping_add(2 as u32_0) as isize)
                        as ::core::ffi::c_int == '\n' as i32
                {
                    i = (i as ::core::ffi::c_uint).wrapping_add(3 as ::core::ffi::c_uint)
                        as u32_0 as u32_0;
                } else {
                    i = (i as ::core::ffi::c_uint).wrapping_add(2 as ::core::ffi::c_uint)
                        as u32_0 as u32_0;
                }
            } else {
                if !(0xe2 as ::core::ffi::c_int
                    == *z.offset(i.wrapping_add(1 as u32_0) as isize) as u8_0
                        as ::core::ffi::c_int && i.wrapping_add(3 as u32_0) < n
                    && 0x80 as ::core::ffi::c_int
                        == *z.offset(i.wrapping_add(2 as u32_0) as isize) as u8_0
                            as ::core::ffi::c_int
                    && (0xa8 as ::core::ffi::c_int
                        == *z.offset(i.wrapping_add(3 as u32_0) as isize) as u8_0
                            as ::core::ffi::c_int
                        || 0xa9 as ::core::ffi::c_int
                            == *z.offset(i.wrapping_add(3 as u32_0) as isize) as u8_0
                                as ::core::ffi::c_int))
                {
                    break;
                }
                i = (i as ::core::ffi::c_uint).wrapping_add(4 as ::core::ffi::c_uint)
                    as u32_0 as u32_0;
            }
        }
        return i;
    }
}
unsafe extern "C" fn jsonUnescapeOneChar(
    mut z: *const ::core::ffi::c_char,
    mut n: u32_0,
    mut piOut: *mut u32_0,
) -> u32_0 {
    unsafe {
        if n < 2 as u32_0 {
            *piOut = JSON_INVALID_CHAR as u32_0;
            return n;
        }
        match *z.offset(1 as ::core::ffi::c_int as isize) as u8_0 as ::core::ffi::c_int {
            117 => {
                let mut v: u32_0 = 0;
                let mut vlo: u32_0 = 0;
                if n < 6 as u32_0 {
                    *piOut = JSON_INVALID_CHAR as u32_0;
                    return n;
                }
                v = jsonHexToInt4(
                    z.offset(2 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char,
                );
                if v & 0xfc00 as u32_0 == 0xd800 as u32_0 && n >= 12 as u32_0
                    && *z.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '\\' as i32
                    && *z.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'u' as i32
                    && {
                        vlo = jsonHexToInt4(
                            z.offset(8 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_char,
                        );
                        vlo & 0xfc00 as u32_0 == 0xdc00 as u32_0
                    }
                {
                    *piOut = ((v & 0x3ff as u32_0) << 10 as ::core::ffi::c_int)
                        .wrapping_add(vlo & 0x3ff as u32_0)
                        .wrapping_add(0x10000 as u32_0);
                    return 12 as u32_0;
                } else {
                    *piOut = v;
                    return 6 as u32_0;
                }
            }
            98 => {
                *piOut = '\u{8}' as i32 as u32_0;
                return 2 as u32_0;
            }
            102 => {
                *piOut = '\u{c}' as i32 as u32_0;
                return 2 as u32_0;
            }
            110 => {
                *piOut = '\n' as i32 as u32_0;
                return 2 as u32_0;
            }
            114 => {
                *piOut = '\r' as i32 as u32_0;
                return 2 as u32_0;
            }
            116 => {
                *piOut = '\t' as i32 as u32_0;
                return 2 as u32_0;
            }
            118 => {
                *piOut = '\u{b}' as i32 as u32_0;
                return 2 as u32_0;
            }
            48 => {
                *piOut = (if n > 2 as u32_0
                    && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(
                            *z.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar as isize,
                        ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                {
                    JSON_INVALID_CHAR
                } else {
                    0 as ::core::ffi::c_int
                }) as u32_0;
                return 2 as u32_0;
            }
            39 | 34 | 47 | 92 => {
                *piOut = *z.offset(1 as ::core::ffi::c_int as isize) as u32_0;
                return 2 as u32_0;
            }
            120 => {
                if n < 4 as u32_0 {
                    *piOut = JSON_INVALID_CHAR as u32_0;
                    return n;
                }
                *piOut = ((jsonHexToInt(
                    *z.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                ) as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | jsonHexToInt(
                        *z.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int) as u32_0;
                return 4 as u32_0;
            }
            226 | 13 | 10 => {
                let mut nSkip: u32_0 = jsonBytesToBypass(z, n);
                if nSkip == 0 as u32_0 {
                    *piOut = JSON_INVALID_CHAR as u32_0;
                    return n;
                } else if nSkip == n {
                    *piOut = 0 as u32_0;
                    return n;
                } else if *z.offset(nSkip as isize) as ::core::ffi::c_int == '\\' as i32
                {
                    return nSkip
                        .wrapping_add(
                            jsonUnescapeOneChar(
                                z.offset(nSkip as isize) as *const ::core::ffi::c_char,
                                n.wrapping_sub(nSkip),
                                piOut,
                            ),
                        )
                } else {
                    let mut sz: ::core::ffi::c_int = sqlite3Utf8ReadLimited(
                        z.offset(nSkip as isize) as *const ::core::ffi::c_char
                            as *mut u8_0,
                        n.wrapping_sub(nSkip) as ::core::ffi::c_int,
                        piOut,
                    );
                    return nSkip.wrapping_add(sz as u32_0);
                }
            }
            _ => {
                *piOut = JSON_INVALID_CHAR as u32_0;
                return 2 as u32_0;
            }
        };
    }
}
#[inline(never)]
unsafe extern "C" fn jsonLabelCompareEscaped(
    mut zLeft: *const ::core::ffi::c_char,
    mut nLeft: u32_0,
    mut rawLeft: ::core::ffi::c_int,
    mut zRight: *const ::core::ffi::c_char,
    mut nRight: u32_0,
    mut rawRight: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut cLeft: u32_0 = 0;
        let mut cRight: u32_0 = 0;
        loop {
            if nLeft == 0 as u32_0 {
                cLeft = 0 as u32_0;
            } else if rawLeft != 0
                || *zLeft.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '\\' as i32
            {
                cLeft = *(zLeft as *mut u8_0).offset(0 as ::core::ffi::c_int as isize)
                    as u32_0;
                if cLeft >= 0xc0 as u32_0 {
                    let mut sz: ::core::ffi::c_int = sqlite3Utf8ReadLimited(
                        zLeft as *mut u8_0,
                        nLeft as ::core::ffi::c_int,
                        &raw mut cLeft,
                    );
                    zLeft = zLeft.offset(sz as isize);
                    nLeft = (nLeft as ::core::ffi::c_uint)
                        .wrapping_sub(sz as ::core::ffi::c_uint) as u32_0 as u32_0;
                } else {
                    zLeft = zLeft.offset(1);
                    nLeft = nLeft.wrapping_sub(1);
                }
            } else {
                let mut n: u32_0 = jsonUnescapeOneChar(zLeft, nLeft, &raw mut cLeft);
                zLeft = zLeft.offset(n as isize);
                nLeft = (nLeft as ::core::ffi::c_uint)
                    .wrapping_sub(n as ::core::ffi::c_uint) as u32_0 as u32_0;
            }
            if nRight == 0 as u32_0 {
                cRight = 0 as u32_0;
            } else if rawRight != 0
                || *zRight.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '\\' as i32
            {
                cRight = *(zRight as *mut u8_0).offset(0 as ::core::ffi::c_int as isize)
                    as u32_0;
                if cRight >= 0xc0 as u32_0 {
                    let mut sz_0: ::core::ffi::c_int = sqlite3Utf8ReadLimited(
                        zRight as *mut u8_0,
                        nRight as ::core::ffi::c_int,
                        &raw mut cRight,
                    );
                    zRight = zRight.offset(sz_0 as isize);
                    nRight = (nRight as ::core::ffi::c_uint)
                        .wrapping_sub(sz_0 as ::core::ffi::c_uint) as u32_0 as u32_0;
                } else {
                    zRight = zRight.offset(1);
                    nRight = nRight.wrapping_sub(1);
                }
            } else {
                let mut n_0: u32_0 = jsonUnescapeOneChar(
                    zRight,
                    nRight,
                    &raw mut cRight,
                );
                zRight = zRight.offset(n_0 as isize);
                nRight = (nRight as ::core::ffi::c_uint)
                    .wrapping_sub(n_0 as ::core::ffi::c_uint) as u32_0 as u32_0;
            }
            if cLeft != cRight {
                return 0 as ::core::ffi::c_int;
            }
            if cLeft == 0 as u32_0 {
                return 1 as ::core::ffi::c_int;
            }
        };
    }
}
unsafe extern "C" fn jsonLabelCompare(
    mut zLeft: *const ::core::ffi::c_char,
    mut nLeft: u32_0,
    mut rawLeft: ::core::ffi::c_int,
    mut zRight: *const ::core::ffi::c_char,
    mut nRight: u32_0,
    mut rawRight: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if rawLeft != 0 && rawRight != 0 {
            if nLeft != nRight {
                return 0 as ::core::ffi::c_int;
            }
            return (memcmp(
                zLeft as *const ::core::ffi::c_void,
                zRight as *const ::core::ffi::c_void,
                nLeft as size_t,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        } else {
            return jsonLabelCompareEscaped(
                zLeft,
                nLeft,
                rawLeft,
                zRight,
                nRight,
                rawRight,
            )
        };
    }
}
pub const JSON_LOOKUP_ERROR: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const JSON_LOOKUP_NOTFOUND: ::core::ffi::c_uint = 0xfffffffe as ::core::ffi::c_uint;
pub const JSON_LOOKUP_PATHERROR: ::core::ffi::c_uint = 0xfffffffd as ::core::ffi::c_uint;
unsafe extern "C" fn jsonCreateEditSubstructure(
    mut pParse: *mut JsonParse,
    mut pIns: *mut JsonParse,
    mut zTail: *const ::core::ffi::c_char,
) -> u32_0 {
    unsafe {
        static mut emptyObject: [u8_0; 2] = [JSONB_ARRAY as u8_0, JSONB_OBJECT as u8_0];
        let mut rc: ::core::ffi::c_int = 0;
        memset(
            pIns as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<JsonParse>() as size_t,
        );
        (*pIns).db = (*pParse).db;
        if *zTail.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*pIns).aBlob = (*pParse).aIns;
            (*pIns).nBlob = (*pParse).nIns;
            rc = 0 as ::core::ffi::c_int;
        } else {
            (*pIns).nBlob = 1 as u32_0;
            (*pIns).aBlob = (&raw const emptyObject as *const u8_0)
                .offset(
                    (*zTail.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '.' as i32) as ::core::ffi::c_int
                        as isize,
                ) as *const u8_0 as *mut u8_0;
            (*pIns).eEdit = (*pParse).eEdit;
            (*pIns).nIns = (*pParse).nIns;
            (*pIns).aIns = (*pParse).aIns;
            rc = jsonLookupStep(pIns, 0 as u32_0, zTail, 0 as u32_0)
                as ::core::ffi::c_int;
            (*pParse).oom = ((*pParse).oom as ::core::ffi::c_int
                | (*pIns).oom as ::core::ffi::c_int) as u8_0;
        }
        return rc as u32_0;
    }
}
unsafe extern "C" fn jsonLookupStep(
    mut pParse: *mut JsonParse,
    mut iRoot: u32_0,
    mut zPath: *const ::core::ffi::c_char,
    mut iLabel: u32_0,
) -> u32_0 {
    unsafe {
        let mut i: u32_0 = 0;
        let mut j: u32_0 = 0;
        let mut k: u32_0 = 0;
        let mut nKey: u32_0 = 0;
        let mut sz: u32_0 = 0;
        let mut n: u32_0 = 0;
        let mut iEnd: u32_0 = 0;
        let mut rc: u32_0 = 0;
        let mut zKey: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut x: u8_0 = 0;
        if *zPath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            if (*pParse).eEdit as ::core::ffi::c_int != 0
                && jsonBlobMakeEditable(pParse, (*pParse).nIns) != 0
            {
                n = jsonbPayloadSize(pParse, iRoot, &raw mut sz);
                sz = (sz as ::core::ffi::c_uint).wrapping_add(n as ::core::ffi::c_uint)
                    as u32_0 as u32_0;
                if (*pParse).eEdit as ::core::ffi::c_int == JEDIT_DEL {
                    if iLabel > 0 as u32_0 {
                        sz = (sz as ::core::ffi::c_uint)
                            .wrapping_add(
                                iRoot.wrapping_sub(iLabel) as ::core::ffi::c_uint,
                            ) as u32_0 as u32_0;
                        iRoot = iLabel;
                    }
                    jsonBlobEdit(
                        pParse,
                        iRoot,
                        sz,
                        ::core::ptr::null::<u8_0>(),
                        0 as u32_0,
                    );
                } else if !((*pParse).eEdit as ::core::ffi::c_int == JEDIT_INS) {
                    jsonBlobEdit(pParse, iRoot, sz, (*pParse).aIns, (*pParse).nIns);
                }
            }
            (*pParse).iLabel = iLabel;
            return iRoot;
        }
        if *zPath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '.' as i32
        {
            let mut rawKey: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            x = *(*pParse).aBlob.offset(iRoot as isize);
            zPath = zPath.offset(1);
            if *zPath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '"' as i32
            {
                zKey = zPath.offset(1 as ::core::ffi::c_int as isize);
                i = 1 as u32_0;
                while *zPath.offset(i as isize) as ::core::ffi::c_int != 0
                    && *zPath.offset(i as isize) as ::core::ffi::c_int != '"' as i32
                {
                    if *zPath.offset(i as isize) as ::core::ffi::c_int == '\\' as i32
                        && *zPath.offset(i.wrapping_add(1 as u32_0) as isize)
                            as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    {
                        i = i.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
                nKey = i.wrapping_sub(1 as u32_0);
                if *zPath.offset(i as isize) != 0 {
                    i = i.wrapping_add(1);
                } else {
                    return JSON_LOOKUP_PATHERROR as u32_0
                }
                rawKey = memchr(
                        zKey as *const ::core::ffi::c_void,
                        '\\' as i32,
                        nKey as size_t,
                    )
                    .is_null() as ::core::ffi::c_int;
            } else {
                zKey = zPath;
                i = 0 as u32_0;
                while *zPath.offset(i as isize) as ::core::ffi::c_int != 0
                    && *zPath.offset(i as isize) as ::core::ffi::c_int != '.' as i32
                    && *zPath.offset(i as isize) as ::core::ffi::c_int != '[' as i32
                {
                    i = i.wrapping_add(1);
                }
                nKey = i;
                if nKey == 0 as u32_0 {
                    return JSON_LOOKUP_PATHERROR as u32_0;
                }
            }
            if x as ::core::ffi::c_int & 0xf as ::core::ffi::c_int != JSONB_OBJECT {
                return JSON_LOOKUP_NOTFOUND as u32_0;
            }
            n = jsonbPayloadSize(pParse, iRoot, &raw mut sz);
            j = iRoot.wrapping_add(n);
            iEnd = j.wrapping_add(sz);
            while j < iEnd {
                let mut rawLabel: ::core::ffi::c_int = 0;
                let mut zLabel: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                x = (*(*pParse).aBlob.offset(j as isize) as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int) as u8_0;
                if (x as ::core::ffi::c_int) < JSONB_TEXT
                    || x as ::core::ffi::c_int > JSONB_TEXTRAW
                {
                    return JSON_LOOKUP_ERROR as u32_0;
                }
                n = jsonbPayloadSize(pParse, j, &raw mut sz);
                if n == 0 as u32_0 {
                    return JSON_LOOKUP_ERROR as u32_0;
                }
                k = j.wrapping_add(n);
                if k.wrapping_add(sz) >= iEnd {
                    return JSON_LOOKUP_ERROR as u32_0;
                }
                zLabel = (*pParse).aBlob.offset(k as isize) as *mut u8_0
                    as *const ::core::ffi::c_char;
                rawLabel = (x as ::core::ffi::c_int == JSONB_TEXT
                    || x as ::core::ffi::c_int == JSONB_TEXTRAW) as ::core::ffi::c_int;
                if jsonLabelCompare(zKey, nKey, rawKey, zLabel, sz, rawLabel) != 0 {
                    let mut v: u32_0 = k.wrapping_add(sz);
                    if *(*pParse).aBlob.offset(v as isize) as ::core::ffi::c_int
                        & 0xf as ::core::ffi::c_int > JSONB_OBJECT
                    {
                        return JSON_LOOKUP_ERROR as u32_0;
                    }
                    n = jsonbPayloadSize(pParse, v, &raw mut sz);
                    if n == 0 as u32_0 || v.wrapping_add(n).wrapping_add(sz) > iEnd {
                        return JSON_LOOKUP_ERROR as u32_0;
                    }
                    rc = jsonLookupStep(
                        pParse,
                        v,
                        zPath.offset(i as isize) as *const ::core::ffi::c_char,
                        j,
                    );
                    if (*pParse).delta != 0 {
                        jsonAfterEditSizeAdjust(pParse, iRoot);
                    }
                    return rc;
                }
                j = k.wrapping_add(sz);
                if *(*pParse).aBlob.offset(j as isize) as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int > JSONB_OBJECT
                {
                    return JSON_LOOKUP_ERROR as u32_0;
                }
                n = jsonbPayloadSize(pParse, j, &raw mut sz);
                if n == 0 as u32_0 {
                    return JSON_LOOKUP_ERROR as u32_0;
                }
                j = (j as ::core::ffi::c_uint)
                    .wrapping_add(n.wrapping_add(sz) as ::core::ffi::c_uint) as u32_0
                    as u32_0;
            }
            if j > iEnd {
                return JSON_LOOKUP_ERROR as u32_0;
            }
            if (*pParse).eEdit as ::core::ffi::c_int >= JEDIT_INS {
                let mut nIns: u32_0 = 0;
                let mut v_0: JsonParse = JsonParse {
                    aBlob: ::core::ptr::null_mut::<u8_0>(),
                    nBlob: 0,
                    nBlobAlloc: 0,
                    zJson: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    db: ::core::ptr::null_mut::<sqlite3>(),
                    nJson: 0,
                    nJPRef: 0,
                    iErr: 0,
                    iDepth: 0,
                    nErr: 0,
                    oom: 0,
                    bJsonIsRCStr: 0,
                    hasNonstd: 0,
                    bReadOnly: 0,
                    eEdit: 0,
                    delta: 0,
                    nIns: 0,
                    iLabel: 0,
                    aIns: ::core::ptr::null_mut::<u8_0>()

                };
                let mut ix: JsonParse = JsonParse {
                    aBlob: ::core::ptr::null_mut::<u8_0>(),
                    nBlob: 0,
                    nBlobAlloc: 0,
                    zJson: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    db: ::core::ptr::null_mut::<sqlite3>(),
                    nJson: 0,
                    nJPRef: 0,
                    iErr: 0,
                    iDepth: 0,
                    nErr: 0,
                    oom: 0,
                    bJsonIsRCStr: 0,
                    hasNonstd: 0,
                    bReadOnly: 0,
                    eEdit: 0,
                    delta: 0,
                    nIns: 0,
                    iLabel: 0,
                    aIns: ::core::ptr::null_mut::<u8_0>()

                };
                memset(
                    &raw mut ix as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<JsonParse>() as size_t,
                );
                ix.db = (*pParse).db;
                jsonBlobAppendNode(
                    &raw mut ix,
                    (if rawKey != 0 { JSONB_TEXTRAW } else { JSONB_TEXT5 }) as u8_0,
                    nKey,
                    ::core::ptr::null::<::core::ffi::c_void>(),
                );
                (*pParse).oom = ((*pParse).oom as ::core::ffi::c_int
                    | ix.oom as ::core::ffi::c_int) as u8_0;
                rc = jsonCreateEditSubstructure(
                    pParse,
                    &raw mut v_0,
                    zPath.offset(i as isize) as *const ::core::ffi::c_char,
                );
                if !(rc >= JSON_LOOKUP_PATHERROR as u32_0)
                    && jsonBlobMakeEditable(
                        pParse,
                        ix.nBlob.wrapping_add(nKey).wrapping_add(v_0.nBlob),
                    ) != 0
                {
                    nIns = ix.nBlob.wrapping_add(nKey).wrapping_add(v_0.nBlob);
                    jsonBlobEdit(
                        pParse,
                        j,
                        0 as u32_0,
                        ::core::ptr::null::<u8_0>(),
                        nIns,
                    );
                    if (*pParse).oom == 0 {
                        memcpy(
                            (*pParse).aBlob.offset(j as isize) as *mut u8_0
                                as *mut ::core::ffi::c_void,
                            ix.aBlob as *const ::core::ffi::c_void,
                            ix.nBlob as size_t,
                        );
                        k = j.wrapping_add(ix.nBlob);
                        memcpy(
                            (*pParse).aBlob.offset(k as isize) as *mut u8_0
                                as *mut ::core::ffi::c_void,
                            zKey as *const ::core::ffi::c_void,
                            nKey as size_t,
                        );
                        k = (k as ::core::ffi::c_uint)
                            .wrapping_add(nKey as ::core::ffi::c_uint) as u32_0 as u32_0;
                        memcpy(
                            (*pParse).aBlob.offset(k as isize) as *mut u8_0
                                as *mut ::core::ffi::c_void,
                            v_0.aBlob as *const ::core::ffi::c_void,
                            v_0.nBlob as size_t,
                        );
                        if (*pParse).delta != 0 {
                            jsonAfterEditSizeAdjust(pParse, iRoot);
                        }
                    }
                }
                jsonParseReset(&raw mut v_0);
                jsonParseReset(&raw mut ix);
                return rc;
            }
        } else if *zPath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '[' as i32
        {
            x = (*(*pParse).aBlob.offset(iRoot as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as u8_0;
            if x as ::core::ffi::c_int != JSONB_ARRAY {
                return JSON_LOOKUP_NOTFOUND as u32_0;
            }
            n = jsonbPayloadSize(pParse, iRoot, &raw mut sz);
            k = 0 as u32_0;
            i = 1 as u32_0;
            while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(*zPath.offset(i as isize) as ::core::ffi::c_uchar as isize)
                as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
            {
                k = k
                    .wrapping_mul(10 as u32_0)
                    .wrapping_add(*zPath.offset(i as isize) as u32_0)
                    .wrapping_sub('0' as i32 as u32_0);
                i = i.wrapping_add(1);
            }
            if i < 2 as u32_0
                || *zPath.offset(i as isize) as ::core::ffi::c_int != ']' as i32
            {
                if *zPath.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '#' as i32
                {
                    k = jsonbArrayCount(pParse, iRoot);
                    i = 2 as u32_0;
                    if *zPath.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '-' as i32
                        && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *zPath.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                    {
                        let mut nn: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                        i = 3 as u32_0;
                        loop {
                            nn = nn
                                .wrapping_mul(10 as ::core::ffi::c_uint)
                                .wrapping_add(
                                    *zPath.offset(i as isize) as ::core::ffi::c_uint,
                                )
                                .wrapping_sub('0' as i32 as ::core::ffi::c_uint);
                            i = i.wrapping_add(1);
                            if !(*(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(
                                    *zPath.offset(i as isize) as ::core::ffi::c_uchar as isize,
                                ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0)
                            {
                                break;
                            }
                        }
                        if nn as u32_0 > k {
                            return JSON_LOOKUP_NOTFOUND as u32_0;
                        }
                        k = (k as ::core::ffi::c_uint).wrapping_sub(nn) as u32_0
                            as u32_0;
                    }
                    if *zPath.offset(i as isize) as ::core::ffi::c_int != ']' as i32 {
                        return JSON_LOOKUP_PATHERROR as u32_0;
                    }
                } else {
                    return JSON_LOOKUP_PATHERROR as u32_0
                }
            }
            j = iRoot.wrapping_add(n);
            iEnd = j.wrapping_add(sz);
            while j < iEnd {
                if k == 0 as u32_0 {
                    rc = jsonLookupStep(
                        pParse,
                        j,
                        zPath.offset(i.wrapping_add(1 as u32_0) as isize)
                            as *const ::core::ffi::c_char,
                        0 as u32_0,
                    );
                    if (*pParse).delta != 0 {
                        jsonAfterEditSizeAdjust(pParse, iRoot);
                    }
                    return rc;
                }
                k = k.wrapping_sub(1);
                n = jsonbPayloadSize(pParse, j, &raw mut sz);
                if n == 0 as u32_0 {
                    return JSON_LOOKUP_ERROR as u32_0;
                }
                j = (j as ::core::ffi::c_uint)
                    .wrapping_add(n.wrapping_add(sz) as ::core::ffi::c_uint) as u32_0
                    as u32_0;
            }
            if j > iEnd {
                return JSON_LOOKUP_ERROR as u32_0;
            }
            if k > 0 as u32_0 {
                return JSON_LOOKUP_NOTFOUND as u32_0;
            }
            if (*pParse).eEdit as ::core::ffi::c_int >= JEDIT_INS {
                let mut v_1: JsonParse = JsonParse {
                    aBlob: ::core::ptr::null_mut::<u8_0>(),
                    nBlob: 0,
                    nBlobAlloc: 0,
                    zJson: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    db: ::core::ptr::null_mut::<sqlite3>(),
                    nJson: 0,
                    nJPRef: 0,
                    iErr: 0,
                    iDepth: 0,
                    nErr: 0,
                    oom: 0,
                    bJsonIsRCStr: 0,
                    hasNonstd: 0,
                    bReadOnly: 0,
                    eEdit: 0,
                    delta: 0,
                    nIns: 0,
                    iLabel: 0,
                    aIns: ::core::ptr::null_mut::<u8_0>()

                };
                rc = jsonCreateEditSubstructure(
                    pParse,
                    &raw mut v_1,
                    zPath.offset(i.wrapping_add(1 as u32_0) as isize)
                        as *const ::core::ffi::c_char,
                );
                if !(rc >= JSON_LOOKUP_PATHERROR as u32_0)
                    && jsonBlobMakeEditable(pParse, v_1.nBlob) != 0
                {
                    jsonBlobEdit(pParse, j, 0 as u32_0, v_1.aBlob, v_1.nBlob);
                }
                jsonParseReset(&raw mut v_1);
                if (*pParse).delta != 0 {
                    jsonAfterEditSizeAdjust(pParse, iRoot);
                }
                return rc;
            }
        } else {
            return JSON_LOOKUP_PATHERROR as u32_0
        }
        return JSON_LOOKUP_NOTFOUND as u32_0;
    }
}
unsafe extern "C" fn jsonReturnTextJsonFromBlob(
    mut ctx: *mut sqlite3_context,
    mut aBlob: *const u8_0,
    mut nBlob: u32_0,
) {
    unsafe {
        let mut x: JsonParse = JsonParse {
            aBlob: ::core::ptr::null_mut::<u8_0>(),
            nBlob: 0,
            nBlobAlloc: 0,
            zJson: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            db: ::core::ptr::null_mut::<sqlite3>(),
            nJson: 0,
            nJPRef: 0,
            iErr: 0,
            iDepth: 0,
            nErr: 0,
            oom: 0,
            bJsonIsRCStr: 0,
            hasNonstd: 0,
            bReadOnly: 0,
            eEdit: 0,
            delta: 0,
            nIns: 0,
            iLabel: 0,
            aIns: ::core::ptr::null_mut::<u8_0>()

        };
        let mut s: JsonString = JsonString {
            pCtx: ::core::ptr::null_mut::<sqlite3_context>(),
            zBuf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            nUsed: 0,
            bStatic: 0,
            eErr: 0,
            zSpace: [0; 100]

        };
        if aBlob.is_null() {
            return;
        }
        memset(
            &raw mut x as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<JsonParse>() as size_t,
        );
        x.aBlob = aBlob as *mut u8_0;
        x.nBlob = nBlob;
        jsonStringInit(&raw mut s, ctx);
        jsonTranslateBlobToText(&raw mut x, 0 as u32_0, &raw mut s);
        jsonReturnString(
            &raw mut s,
            ::core::ptr::null_mut::<JsonParse>(),
            ::core::ptr::null_mut::<sqlite3_context>(),
        );
    }
}
unsafe extern "C" fn jsonReturnFromBlob(
    mut pParse: *mut JsonParse,
    mut i: u32_0,
    mut pCtx: *mut sqlite3_context,
    mut eMode: ::core::ffi::c_int,
) {
    unsafe {
        let mut r_0: ::core::ffi::c_double = 0.;
        let mut z_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut c2rust_current_block: u64;
        let mut n: u32_0 = 0;
        let mut sz: u32_0 = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(pCtx);
        n = jsonbPayloadSize(pParse, i, &raw mut sz);
        if n == 0 as u32_0 {
            sqlite3_result_error(
                pCtx,
                b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            );
            return;
        }
        match *(*pParse).aBlob.offset(i as isize) as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int
        {
            JSONB_NULL => {
                if sz != 0 {
                    c2rust_current_block = 4773211817026055473;
                } else {
                    sqlite3_result_null(pCtx);
                    c2rust_current_block = 2616667235040759262;
                }
            }
            JSONB_TRUE => {
                if sz != 0 {
                    c2rust_current_block = 4773211817026055473;
                } else {
                    sqlite3_result_int(pCtx, 1 as ::core::ffi::c_int);
                    c2rust_current_block = 2616667235040759262;
                }
            }
            JSONB_FALSE => {
                if sz != 0 {
                    c2rust_current_block = 4773211817026055473;
                } else {
                    sqlite3_result_int(pCtx, 0 as ::core::ffi::c_int);
                    c2rust_current_block = 2616667235040759262;
                }
            }
            JSONB_INT5 | JSONB_INT => {
                let mut iRes: sqlite3_int64 = 0 as sqlite3_int64;
                let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut bNeg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut x: ::core::ffi::c_char = 0;
                if sz == 0 as u32_0 {
                    c2rust_current_block = 4773211817026055473;
                } else {
                    x = *(*pParse).aBlob.offset(i.wrapping_add(n) as isize)
                        as ::core::ffi::c_char;
                    if x as ::core::ffi::c_int == '-' as i32 {
                        if sz < 2 as u32_0 {
                            c2rust_current_block = 4773211817026055473;
                        } else {
                            n = n.wrapping_add(1);
                            sz = sz.wrapping_sub(1);
                            bNeg = 1 as ::core::ffi::c_int;
                            c2rust_current_block = 12147880666119273379;
                        }
                    } else {
                        c2rust_current_block = 12147880666119273379;
                    }
                    match c2rust_current_block {
                        4773211817026055473 => {}
                        _ => {
                            z = sqlite3DbStrNDup(
                                db,
                                (*pParse).aBlob.offset(i.wrapping_add(n) as isize)
                                    as *mut u8_0 as *const ::core::ffi::c_char,
                                sz as ::core::ffi::c_int as u64_0,
                            );
                            if z.is_null() {
                                c2rust_current_block = 6991919280978899281;
                            } else {
                                rc = sqlite3DecOrHexToI64(z, &raw mut iRes);
                                sqlite3DbFree(db, z as *mut ::core::ffi::c_void);
                                if rc == 0 as ::core::ffi::c_int {
                                    if iRes < 0 as ::core::ffi::c_longlong {
                                        let mut r: ::core::ffi::c_double = 0.;
                                        r = *(&raw mut iRes as *mut sqlite3_uint64)
                                            as ::core::ffi::c_double;
                                        sqlite3_result_double(pCtx, if bNeg != 0 { -r } else { r });
                                    } else {
                                        sqlite3_result_int64(
                                            pCtx,
                                            if bNeg != 0 { -iRes } else { iRes },
                                        );
                                    }
                                    c2rust_current_block = 2616667235040759262;
                                } else if rc == 3 as ::core::ffi::c_int && bNeg != 0 {
                                    sqlite3_result_int64(pCtx, SMALLEST_INT64);
                                    c2rust_current_block = 2616667235040759262;
                                } else if rc == 1 as ::core::ffi::c_int {
                                    c2rust_current_block = 4773211817026055473;
                                } else {
                                    if bNeg != 0 {
                                        n = n.wrapping_sub(1);
                                        sz = sz.wrapping_add(1);
                                    }
                                    c2rust_current_block = 2398301974880704515;
                                }
                            }
                        }
                    }
                }
            }
            JSONB_FLOAT5 | JSONB_FLOAT => {
                r_0 = 0.;
                z_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if sz == 0 as u32_0 {
                    c2rust_current_block = 4773211817026055473;
                } else {
                    c2rust_current_block = 2398301974880704515;
                }
            }
            JSONB_TEXTRAW | JSONB_TEXT => {
                sqlite3_result_text(
                    pCtx,
                    (*pParse).aBlob.offset(i.wrapping_add(n) as isize) as *mut u8_0
                        as *mut ::core::ffi::c_char,
                    sz as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                c2rust_current_block = 2616667235040759262;
            }
            JSONB_TEXT5 | JSONB_TEXTJ => {
                let mut iIn: u32_0 = 0;
                let mut iOut: u32_0 = 0;
                let mut z_1: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut zOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut nOut: u32_0 = sz;
                z_1 = (*pParse).aBlob.offset(i.wrapping_add(n) as isize) as *mut u8_0
                    as *const ::core::ffi::c_char;
                zOut = sqlite3DbMallocRaw(db, (nOut as u64_0).wrapping_add(1 as u64_0))
                    as *mut ::core::ffi::c_char;
                if zOut.is_null() {
                    c2rust_current_block = 6991919280978899281;
                } else {
                    iOut = 0 as u32_0;
                    iIn = iOut;
                    while iIn < sz {
                        let mut c: ::core::ffi::c_char = *z_1.offset(iIn as isize);
                        if c as ::core::ffi::c_int == '\\' as i32 {
                            let mut v: u32_0 = 0;
                            let mut szEscape: u32_0 = jsonUnescapeOneChar(
                                z_1.offset(iIn as isize) as *const ::core::ffi::c_char,
                                sz.wrapping_sub(iIn),
                                &raw mut v,
                            );
                            if v <= 0x7f as u32_0 {
                                let c2rust_fresh10 = iOut;
                                iOut = iOut.wrapping_add(1);
                                *zOut.offset(c2rust_fresh10 as isize) = v
                                    as ::core::ffi::c_char;
                            } else if v <= 0x7ff as u32_0 {
                                let c2rust_fresh11 = iOut;
                                iOut = iOut.wrapping_add(1);
                                *zOut.offset(c2rust_fresh11 as isize) = (0xc0 as u32_0
                                    | v >> 6 as ::core::ffi::c_int) as ::core::ffi::c_char;
                                let c2rust_fresh12 = iOut;
                                iOut = iOut.wrapping_add(1);
                                *zOut.offset(c2rust_fresh12 as isize) = (0x80 as u32_0
                                    | v & 0x3f as u32_0) as ::core::ffi::c_char;
                            } else if v < 0x10000 as u32_0 {
                                let c2rust_fresh13 = iOut;
                                iOut = iOut.wrapping_add(1);
                                *zOut.offset(c2rust_fresh13 as isize) = (0xe0 as u32_0
                                    | v >> 12 as ::core::ffi::c_int) as ::core::ffi::c_char;
                                let c2rust_fresh14 = iOut;
                                iOut = iOut.wrapping_add(1);
                                *zOut.offset(c2rust_fresh14 as isize) = (0x80 as u32_0
                                    | v >> 6 as ::core::ffi::c_int & 0x3f as u32_0)
                                    as ::core::ffi::c_char;
                                let c2rust_fresh15 = iOut;
                                iOut = iOut.wrapping_add(1);
                                *zOut.offset(c2rust_fresh15 as isize) = (0x80 as u32_0
                                    | v & 0x3f as u32_0) as ::core::ffi::c_char;
                            } else if !(v == JSON_INVALID_CHAR as u32_0) {
                                let c2rust_fresh16 = iOut;
                                iOut = iOut.wrapping_add(1);
                                *zOut.offset(c2rust_fresh16 as isize) = (0xf0 as u32_0
                                    | v >> 18 as ::core::ffi::c_int) as ::core::ffi::c_char;
                                let c2rust_fresh17 = iOut;
                                iOut = iOut.wrapping_add(1);
                                *zOut.offset(c2rust_fresh17 as isize) = (0x80 as u32_0
                                    | v >> 12 as ::core::ffi::c_int & 0x3f as u32_0)
                                    as ::core::ffi::c_char;
                                let c2rust_fresh18 = iOut;
                                iOut = iOut.wrapping_add(1);
                                *zOut.offset(c2rust_fresh18 as isize) = (0x80 as u32_0
                                    | v >> 6 as ::core::ffi::c_int & 0x3f as u32_0)
                                    as ::core::ffi::c_char;
                                let c2rust_fresh19 = iOut;
                                iOut = iOut.wrapping_add(1);
                                *zOut.offset(c2rust_fresh19 as isize) = (0x80 as u32_0
                                    | v & 0x3f as u32_0) as ::core::ffi::c_char;
                            }
                            iIn = (iIn as ::core::ffi::c_uint)
                                .wrapping_add(
                                    szEscape.wrapping_sub(1 as u32_0) as ::core::ffi::c_uint,
                                ) as u32_0 as u32_0;
                        } else {
                            let c2rust_fresh20 = iOut;
                            iOut = iOut.wrapping_add(1);
                            *zOut.offset(c2rust_fresh20 as isize) = c;
                        }
                        iIn = iIn.wrapping_add(1);
                    }
                    *zOut.offset(iOut as isize) = 0 as ::core::ffi::c_char;
                    sqlite3_result_text(
                        pCtx,
                        zOut,
                        iOut as ::core::ffi::c_int,
                        Some(
                            sqlite3RowSetClear
                                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                        ),
                    );
                    c2rust_current_block = 2616667235040759262;
                }
            }
            JSONB_ARRAY | JSONB_OBJECT => {
                if eMode == 0 as ::core::ffi::c_int {
                    if sqlite3_user_data(pCtx) as intptr_t as ::core::ffi::c_int
                        & JSON_BLOB != 0 as ::core::ffi::c_int
                    {
                        eMode = 2 as ::core::ffi::c_int;
                    } else {
                        eMode = 1 as ::core::ffi::c_int;
                    }
                }
                if eMode == 2 as ::core::ffi::c_int {
                    sqlite3_result_blob(
                        pCtx,
                        (*pParse).aBlob.offset(i as isize) as *mut u8_0
                            as *const ::core::ffi::c_void,
                        sz.wrapping_add(n) as ::core::ffi::c_int,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                } else {
                    jsonReturnTextJsonFromBlob(
                        pCtx,
                        (*pParse).aBlob.offset(i as isize) as *mut u8_0,
                        sz.wrapping_add(n),
                    );
                }
                c2rust_current_block = 2616667235040759262;
            }
            _ => {
                c2rust_current_block = 4773211817026055473;
            }
        }
        match c2rust_current_block {
            2398301974880704515 => {
                z_0 = sqlite3DbStrNDup(
                    db,
                    (*pParse).aBlob.offset(i.wrapping_add(n) as isize) as *mut u8_0
                        as *const ::core::ffi::c_char,
                    sz as ::core::ffi::c_int as u64_0,
                );
                if z_0.is_null() {
                    c2rust_current_block = 6991919280978899281;
                } else {
                    rc = sqlite3AtoF(
                        z_0,
                        &raw mut r_0,
                        sqlite3Strlen30(z_0),
                        SQLITE_UTF8 as u8_0,
                    );
                    sqlite3DbFree(db, z_0 as *mut ::core::ffi::c_void);
                    if rc <= 0 as ::core::ffi::c_int {
                        c2rust_current_block = 4773211817026055473;
                    } else {
                        sqlite3_result_double(pCtx, r_0);
                        c2rust_current_block = 2616667235040759262;
                    }
                }
            }
            _ => {}
        }
        match c2rust_current_block {
            2616667235040759262 => return,
            4773211817026055473 => {
                sqlite3_result_error(
                    pCtx,
                    b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                return;
            }
            _ => {
                sqlite3_result_error_nomem(pCtx);
                return;
            }
        };
    }
}
unsafe extern "C" fn jsonFunctionArgToBlob(
    mut ctx: *mut sqlite3_context,
    mut pArg: *mut sqlite3_value,
    mut pParse: *mut JsonParse,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eType: ::core::ffi::c_int = sqlite3_value_type(pArg);
        static mut aNull: [u8_0; 1] = [0 as ::core::ffi::c_int as u8_0];
        memset(
            pParse as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<JsonParse>() as size_t,
        );
        (*pParse).db = sqlite3_context_db_handle(ctx);
        match eType {
            SQLITE_BLOB => {
                if jsonArgIsJsonb(pArg, pParse) == 0 {
                    sqlite3_result_error(
                        ctx,
                        b"JSON cannot hold BLOB values\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                    return 1 as ::core::ffi::c_int;
                }
            }
            SQLITE_TEXT => {
                let mut zJson: *const ::core::ffi::c_char = sqlite3_value_text(pArg)
                    as *const ::core::ffi::c_char;
                let mut nJson: ::core::ffi::c_int = sqlite3_value_bytes(pArg);
                if zJson.is_null() {
                    return 1 as ::core::ffi::c_int;
                }
                if sqlite3_value_subtype(pArg) == JSON_SUBTYPE as ::core::ffi::c_uint {
                    (*pParse).zJson = zJson as *mut ::core::ffi::c_char;
                    (*pParse).nJson = nJson;
                    if jsonConvertTextToBlob(pParse, ctx) != 0 {
                        sqlite3_result_error(
                            ctx,
                            b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                        );
                        sqlite3DbFree(
                            (*pParse).db,
                            (*pParse).aBlob as *mut ::core::ffi::c_void,
                        );
                        memset(
                            pParse as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<JsonParse>() as size_t,
                        );
                        return 1 as ::core::ffi::c_int;
                    }
                } else {
                    jsonBlobAppendNode(
                        pParse,
                        JSONB_TEXTRAW as u8_0,
                        nJson as u32_0,
                        zJson as *const ::core::ffi::c_void,
                    );
                }
            }
            SQLITE_FLOAT => {
                let mut r: ::core::ffi::c_double = sqlite3_value_double(pArg);
                if sqlite3IsNaN(r) != 0 {
                    jsonBlobAppendNode(
                        pParse,
                        JSONB_NULL as u8_0,
                        0 as u32_0,
                        ::core::ptr::null::<::core::ffi::c_void>(),
                    );
                } else {
                    let mut n: ::core::ffi::c_int = sqlite3_value_bytes(pArg);
                    let mut z: *const ::core::ffi::c_char = sqlite3_value_text(pArg)
                        as *const ::core::ffi::c_char;
                    if z.is_null() {
                        return 1 as ::core::ffi::c_int;
                    }
                    if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'I' as i32
                    {
                        jsonBlobAppendNode(
                            pParse,
                            JSONB_FLOAT as u8_0,
                            5 as u32_0,
                            b"9e999\0".as_ptr() as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                        );
                    } else if *z.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '-' as i32
                        && *z.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 'I' as i32
                    {
                        jsonBlobAppendNode(
                            pParse,
                            JSONB_FLOAT as u8_0,
                            6 as u32_0,
                            b"-9e999\0".as_ptr() as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                        );
                    } else {
                        jsonBlobAppendNode(
                            pParse,
                            JSONB_FLOAT as u8_0,
                            n as u32_0,
                            z as *const ::core::ffi::c_void,
                        );
                    }
                }
            }
            SQLITE_INTEGER => {
                let mut n_0: ::core::ffi::c_int = sqlite3_value_bytes(pArg);
                let mut z_0: *const ::core::ffi::c_char = sqlite3_value_text(pArg)
                    as *const ::core::ffi::c_char;
                if z_0.is_null() {
                    return 1 as ::core::ffi::c_int;
                }
                jsonBlobAppendNode(
                    pParse,
                    JSONB_INT as u8_0,
                    n_0 as u32_0,
                    z_0 as *const ::core::ffi::c_void,
                );
            }
            _ => {
                (*pParse).aBlob = &raw mut aNull as *mut u8_0;
                (*pParse).nBlob = 1 as u32_0;
                return 0 as ::core::ffi::c_int;
            }
        }
        if (*pParse).oom != 0 {
            sqlite3_result_error_nomem(ctx);
            return 1 as ::core::ffi::c_int;
        } else {
            return 0 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn jsonBadPathError(
    mut ctx: *mut sqlite3_context,
    mut zPath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zMsg: *mut ::core::ffi::c_char = sqlite3_mprintf(
            b"bad JSON path: %Q\0".as_ptr() as *const ::core::ffi::c_char,
            zPath,
        );
        if ctx.is_null() {
            return zMsg;
        }
        if !zMsg.is_null() {
            sqlite3_result_error(ctx, zMsg, -1 as ::core::ffi::c_int);
            sqlite3_free(zMsg as *mut ::core::ffi::c_void);
        } else {
            sqlite3_result_error_nomem(ctx);
        }
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn jsonInsertIntoBlob(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
    mut eEdit: ::core::ffi::c_int,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: u32_0 = 0 as u32_0;
        let mut zPath: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut flgs: ::core::ffi::c_int = 0;
        let mut p: *mut JsonParse = ::core::ptr::null_mut::<JsonParse>();
        let mut ax: JsonParse = JsonParse {
            aBlob: ::core::ptr::null_mut::<u8_0>(),
            nBlob: 0,
            nBlobAlloc: 0,
            zJson: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            db: ::core::ptr::null_mut::<sqlite3>(),
            nJson: 0,
            nJPRef: 0,
            iErr: 0,
            iDepth: 0,
            nErr: 0,
            oom: 0,
            bJsonIsRCStr: 0,
            hasNonstd: 0,
            bReadOnly: 0,
            eEdit: 0,
            delta: 0,
            nIns: 0,
            iLabel: 0,
            aIns: ::core::ptr::null_mut::<u8_0>()

        };
        flgs = if argc == 1 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            JSON_EDITABLE
        };
        p = jsonParseFuncArg(
            ctx,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            flgs as u32_0,
        );
        if p.is_null() {
            return;
        }
        i = 1 as ::core::ffi::c_int;
        loop {
            if !(i < argc - 1 as ::core::ffi::c_int) {
                c2rust_current_block = 3437258052017859086;
                break;
            }
            if !(sqlite3_value_type(*argv.offset(i as isize)) == SQLITE_NULL) {
                zPath = sqlite3_value_text(*argv.offset(i as isize))
                    as *const ::core::ffi::c_char;
                if zPath.is_null() {
                    sqlite3_result_error_nomem(ctx);
                    jsonParseFree(p);
                    return;
                }
                if *zPath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '$' as i32
                {
                    c2rust_current_block = 10468985709449339327;
                    break;
                }
                if jsonFunctionArgToBlob(
                    ctx,
                    *argv.offset((i + 1 as ::core::ffi::c_int) as isize),
                    &raw mut ax,
                ) != 0
                {
                    jsonParseReset(&raw mut ax);
                    jsonParseFree(p);
                    return;
                }
                if *zPath.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    if eEdit == JEDIT_REPL || eEdit == JEDIT_SET {
                        jsonBlobEdit(p, 0 as u32_0, (*p).nBlob, ax.aBlob, ax.nBlob);
                    }
                    rc = 0 as u32_0;
                } else {
                    (*p).eEdit = eEdit as u8_0;
                    (*p).nIns = ax.nBlob;
                    (*p).aIns = ax.aBlob;
                    (*p).delta = 0 as ::core::ffi::c_int;
                    rc = jsonLookupStep(
                        p,
                        0 as u32_0,
                        zPath.offset(1 as ::core::ffi::c_int as isize),
                        0 as u32_0,
                    );
                }
                jsonParseReset(&raw mut ax);
                if !(rc == JSON_LOOKUP_NOTFOUND as u32_0) {
                    if rc >= JSON_LOOKUP_PATHERROR as u32_0 {
                        c2rust_current_block = 10468985709449339327;
                        break;
                    }
                }
            }
            i += 2 as ::core::ffi::c_int;
        }
        match c2rust_current_block {
            10468985709449339327 => {
                jsonParseFree(p);
                if rc == JSON_LOOKUP_ERROR as u32_0 {
                    sqlite3_result_error(
                        ctx,
                        b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                } else {
                    jsonBadPathError(ctx, zPath);
                }
                return;
            }
            _ => {
                jsonReturnParse(ctx, p);
                jsonParseFree(p);
                return;
            }
        };
    }
}
unsafe extern "C" fn jsonArgIsJsonb(
    mut pArg: *mut sqlite3_value,
    mut p: *mut JsonParse,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: u32_0 = 0;
        let mut sz: u32_0 = 0 as u32_0;
        let mut c: u8_0 = 0;
        if sqlite3_value_type(pArg) != SQLITE_BLOB {
            return 0 as ::core::ffi::c_int;
        }
        (*p).aBlob = sqlite3_value_blob(pArg) as *mut u8_0;
        (*p).nBlob = sqlite3_value_bytes(pArg) as u32_0;
        if (*p).nBlob > 0 as u32_0 && !(*p).aBlob.is_null()
            && {
                c = *(*p).aBlob.offset(0 as ::core::ffi::c_int as isize);
                c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int <= JSONB_OBJECT
            }
            && {
                n = jsonbPayloadSize(p, 0 as u32_0, &raw mut sz);
                n > 0 as u32_0
            } && sz.wrapping_add(n) == (*p).nBlob
            && (c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int > JSONB_FALSE
                || sz == 0 as u32_0)
            && (sz > 7 as u32_0
                || c as ::core::ffi::c_int != 0x7b as ::core::ffi::c_int
                    && c as ::core::ffi::c_int != 0x5b as ::core::ffi::c_int
                    && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(c as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                        & 0x4 as ::core::ffi::c_int == 0
                || jsonbValidityCheck(p, 0 as u32_0, (*p).nBlob, 1 as u32_0)
                    == 0 as u32_0)
        {
            return 1 as ::core::ffi::c_int;
        }
        (*p).aBlob = ::core::ptr::null_mut::<u8_0>();
        (*p).nBlob = 0 as u32_0;
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn jsonParseFuncArg(
    mut ctx: *mut sqlite3_context,
    mut pArg: *mut sqlite3_value,
    mut flgs: u32_0,
) -> *mut JsonParse {
    unsafe {
        let mut nBlob: u32_0 = 0;
        let mut c2rust_current_block: u64;
        let mut eType: ::core::ffi::c_int = 0;
        let mut p: *mut JsonParse = ::core::ptr::null_mut::<JsonParse>();
        let mut pFromCache: *mut JsonParse = ::core::ptr::null_mut::<JsonParse>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        eType = sqlite3_value_type(pArg);
        if eType == SQLITE_NULL {
            return ::core::ptr::null_mut::<JsonParse>();
        }
        pFromCache = jsonCacheSearch(ctx, pArg);
        if !pFromCache.is_null() {
            (*pFromCache).nJPRef = (*pFromCache).nJPRef.wrapping_add(1);
            if flgs & JSON_EDITABLE as u32_0 == 0 as u32_0 {
                return pFromCache;
            }
        }
        db = sqlite3_context_db_handle(ctx);
        loop {
            p = sqlite3DbMallocZero(db, ::core::mem::size_of::<JsonParse>() as u64_0)
                as *mut JsonParse;
            if p.is_null() {
                c2rust_current_block = 3203591771111562301;
                break;
            }
            memset(
                p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<JsonParse>() as size_t,
            );
            (*p).db = db;
            (*p).nJPRef = 1 as u32_0;
            if !pFromCache.is_null() {
                nBlob = (*pFromCache).nBlob;
                (*p).aBlob = sqlite3DbMallocRaw(db, nBlob as u64_0) as *mut u8_0;
                if (*p).aBlob.is_null() {
                    c2rust_current_block = 3203591771111562301;
                    break;
                } else {
                    c2rust_current_block = 7149356873433890176;
                    break;
                }
            } else {
                if eType == SQLITE_BLOB {
                    if jsonArgIsJsonb(pArg, p) != 0 {
                        if flgs & JSON_EDITABLE as u32_0 != 0 as u32_0
                            && jsonBlobMakeEditable(p, 0 as u32_0)
                                == 0 as ::core::ffi::c_int
                        {
                            c2rust_current_block = 3203591771111562301;
                            break;
                        } else {
                            c2rust_current_block = 14576567515993809846;
                            break;
                        }
                    }
                }
                (*p).zJson = sqlite3_value_text(pArg) as *mut ::core::ffi::c_char;
                (*p).nJson = sqlite3_value_bytes(pArg);
                if (*db).mallocFailed != 0 {
                    c2rust_current_block = 3203591771111562301;
                    break;
                }
                if (*p).nJson == 0 as ::core::ffi::c_int {
                    if flgs & JSON_KEEPERROR as u32_0 != 0 {
                        (*p).nErr = 1 as u8_0;
                        return p;
                    } else {
                        jsonParseFree(p);
                        sqlite3_result_error(
                            ctx,
                            b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                        );
                        return ::core::ptr::null_mut::<JsonParse>();
                    }
                } else if jsonConvertTextToBlob(
                    p,
                    if flgs & JSON_KEEPERROR as u32_0 != 0 {
                        ::core::ptr::null_mut::<sqlite3_context>()
                    } else {
                        ctx
                    },
                ) != 0
                {
                    if flgs & JSON_KEEPERROR as u32_0 != 0 {
                        (*p).nErr = 1 as u8_0;
                        return p;
                    } else {
                        jsonParseFree(p);
                        return ::core::ptr::null_mut::<JsonParse>();
                    }
                } else {
                    let mut isRCStr: ::core::ffi::c_int = sqlite3ValueIsOfClass(
                        pArg,
                        Some(
                            sqlite3RCStrUnref
                                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                        ),
                    );
                    let mut rc: ::core::ffi::c_int = 0;
                    if isRCStr == 0 {
                        let mut zNew: *mut ::core::ffi::c_char = sqlite3RCStrNew(
                            (*p).nJson as u64_0,
                        );
                        if zNew.is_null() {
                            c2rust_current_block = 3203591771111562301;
                            break;
                        }
                        memcpy(
                            zNew as *mut ::core::ffi::c_void,
                            (*p).zJson as *const ::core::ffi::c_void,
                            (*p).nJson as size_t,
                        );
                        (*p).zJson = zNew;
                        *(*p).zJson.offset((*p).nJson as isize) = 0
                            as ::core::ffi::c_char;
                    } else {
                        sqlite3RCStrRef((*p).zJson);
                    }
                    (*p).bJsonIsRCStr = 1 as u8_0;
                    rc = jsonCacheInsert(ctx, p);
                    if rc == SQLITE_NOMEM {
                        c2rust_current_block = 3203591771111562301;
                        break;
                    }
                    if flgs & JSON_EDITABLE as u32_0 != 0 {
                        pFromCache = p;
                        p = ::core::ptr::null_mut::<JsonParse>();
                    } else {
                        return p
                    }
                }
            }
        }
        match c2rust_current_block {
            14576567515993809846 => return p,
            3203591771111562301 => {
                jsonParseFree(pFromCache);
                jsonParseFree(p);
                sqlite3_result_error_nomem(ctx);
                return ::core::ptr::null_mut::<JsonParse>();
            }
            _ => {
                memcpy(
                    (*p).aBlob as *mut ::core::ffi::c_void,
                    (*pFromCache).aBlob as *const ::core::ffi::c_void,
                    nBlob as size_t,
                );
                (*p).nBlob = nBlob;
                (*p).nBlobAlloc = (*p).nBlob;
                (*p).hasNonstd = (*pFromCache).hasNonstd;
                jsonParseFree(pFromCache);
                return p;
            }
        };
    }
}
unsafe extern "C" fn jsonReturnParse(
    mut ctx: *mut sqlite3_context,
    mut p: *mut JsonParse,
) {
    unsafe {
        let mut flgs: ::core::ffi::c_int = 0;
        if (*p).oom != 0 {
            sqlite3_result_error_nomem(ctx);
            return;
        }
        flgs = sqlite3_user_data(ctx) as intptr_t as ::core::ffi::c_int;
        if flgs & JSON_BLOB != 0 {
            if (*p).nBlobAlloc > 0 as u32_0 && (*p).bReadOnly == 0 {
                sqlite3_result_blob(
                    ctx,
                    (*p).aBlob as *const ::core::ffi::c_void,
                    (*p).nBlob as ::core::ffi::c_int,
                    Some(
                        sqlite3RowSetClear
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
                (*p).nBlobAlloc = 0 as u32_0;
            } else {
                sqlite3_result_blob(
                    ctx,
                    (*p).aBlob as *const ::core::ffi::c_void,
                    (*p).nBlob as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            }
        } else {
            let mut s: JsonString = JsonString {
                pCtx: ::core::ptr::null_mut::<sqlite3_context>(),
                zBuf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                nAlloc: 0,
                nUsed: 0,
                bStatic: 0,
                eErr: 0,
                zSpace: [0; 100]

            };
            jsonStringInit(&raw mut s, ctx);
            (*p).delta = 0 as ::core::ffi::c_int;
            jsonTranslateBlobToText(p, 0 as u32_0, &raw mut s);
            jsonReturnString(&raw mut s, p, ctx);
            sqlite3_result_subtype(ctx, JSON_SUBTYPE as ::core::ffi::c_uint);
        };
    }
}
unsafe extern "C" fn jsonQuoteFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut jx: JsonString = JsonString {
            pCtx: ::core::ptr::null_mut::<sqlite3_context>(),
            zBuf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            nUsed: 0,
            bStatic: 0,
            eErr: 0,
            zSpace: [0; 100]

        };
        jsonStringInit(&raw mut jx, ctx);
        jsonAppendSqlValue(&raw mut jx, *argv.offset(0 as ::core::ffi::c_int as isize));
        jsonReturnString(
            &raw mut jx,
            ::core::ptr::null_mut::<JsonParse>(),
            ::core::ptr::null_mut::<sqlite3_context>(),
        );
        sqlite3_result_subtype(ctx, JSON_SUBTYPE as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn jsonArrayFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut jx: JsonString = JsonString {
            pCtx: ::core::ptr::null_mut::<sqlite3_context>(),
            zBuf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            nUsed: 0,
            bStatic: 0,
            eErr: 0,
            zSpace: [0; 100]

        };
        jsonStringInit(&raw mut jx, ctx);
        jsonAppendChar(&raw mut jx, '[' as i32 as ::core::ffi::c_char);
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            jsonAppendSeparator(&raw mut jx);
            jsonAppendSqlValue(&raw mut jx, *argv.offset(i as isize));
            i += 1;
        }
        jsonAppendChar(&raw mut jx, ']' as i32 as ::core::ffi::c_char);
        jsonReturnString(
            &raw mut jx,
            ::core::ptr::null_mut::<JsonParse>(),
            ::core::ptr::null_mut::<sqlite3_context>(),
        );
        sqlite3_result_subtype(ctx, JSON_SUBTYPE as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn jsonArrayLengthFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut JsonParse = ::core::ptr::null_mut::<JsonParse>();
        let mut cnt: sqlite3_int64 = 0 as sqlite3_int64;
        let mut i: u32_0 = 0;
        let mut eErr: u8_0 = 0 as u8_0;
        p = jsonParseFuncArg(
            ctx,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            0 as u32_0,
        );
        if p.is_null() {
            return;
        }
        if argc == 2 as ::core::ffi::c_int {
            let mut zPath: *const ::core::ffi::c_char = sqlite3_value_text(
                *argv.offset(1 as ::core::ffi::c_int as isize),
            ) as *const ::core::ffi::c_char;
            if zPath.is_null() {
                jsonParseFree(p);
                return;
            }
            i = jsonLookupStep(
                p,
                0 as u32_0,
                if *zPath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '$' as i32
                {
                    zPath.offset(1 as ::core::ffi::c_int as isize)
                } else {
                    b"@\0".as_ptr() as *const ::core::ffi::c_char
                },
                0 as u32_0,
            );
            if i >= JSON_LOOKUP_PATHERROR as u32_0 {
                if !(i == JSON_LOOKUP_NOTFOUND as u32_0) {
                    if i == JSON_LOOKUP_PATHERROR as u32_0 {
                        jsonBadPathError(ctx, zPath);
                    } else {
                        sqlite3_result_error(
                            ctx,
                            b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                        );
                    }
                }
                eErr = 1 as u8_0;
                i = 0 as u32_0;
            }
        } else {
            i = 0 as u32_0;
        }
        if *(*p).aBlob.offset(i as isize) as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int == JSONB_ARRAY
        {
            cnt = jsonbArrayCount(p, i) as sqlite3_int64;
        }
        if eErr == 0 {
            sqlite3_result_int64(ctx, cnt);
        }
        jsonParseFree(p);
    }
}
unsafe extern "C" fn jsonAllAlphanum(
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < n
            && (*(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(*z.offset(i as isize) as ::core::ffi::c_uchar as isize)
                as ::core::ffi::c_int & 0x6 as ::core::ffi::c_int != 0
                || *z.offset(i as isize) as ::core::ffi::c_int == '_' as i32)
        {
            i += 1;
        }
        return (i == n) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn jsonExtractFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut p: *mut JsonParse = ::core::ptr::null_mut::<JsonParse>();
        let mut flags: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut jx: JsonString = JsonString {
            pCtx: ::core::ptr::null_mut::<sqlite3_context>(),
            zBuf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            nUsed: 0,
            bStatic: 0,
            eErr: 0,
            zSpace: [0; 100]

        };
        if argc < 2 as ::core::ffi::c_int {
            return;
        }
        p = jsonParseFuncArg(
            ctx,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            0 as u32_0,
        );
        if p.is_null() {
            return;
        }
        flags = sqlite3_user_data(ctx) as intptr_t as ::core::ffi::c_int;
        jsonStringInit(&raw mut jx, ctx);
        if argc > 2 as ::core::ffi::c_int {
            jsonAppendChar(&raw mut jx, '[' as i32 as ::core::ffi::c_char);
        }
        i = 1 as ::core::ffi::c_int;
        loop {
            if !(i < argc) {
                c2rust_current_block = 2116367355679836638;
                break;
            }
            let mut zPath: *const ::core::ffi::c_char = sqlite3_value_text(
                *argv.offset(i as isize),
            ) as *const ::core::ffi::c_char;
            let mut nPath: ::core::ffi::c_int = 0;
            let mut j: u32_0 = 0;
            if zPath.is_null() {
                c2rust_current_block = 2603583310971537513;
                break;
            }
            nPath = sqlite3Strlen30(zPath);
            if *zPath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '$' as i32
            {
                j = jsonLookupStep(
                    p,
                    0 as u32_0,
                    zPath.offset(1 as ::core::ffi::c_int as isize),
                    0 as u32_0,
                );
            } else if flags & JSON_ABPATH != 0 {
                jsonStringInit(&raw mut jx, ctx);
                if sqlite3_value_type(*argv.offset(i as isize)) == SQLITE_INTEGER {
                    jsonAppendRawNZ(
                        &raw mut jx,
                        b"[\0".as_ptr() as *const ::core::ffi::c_char,
                        1 as u32_0,
                    );
                    if *zPath.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '-' as i32
                    {
                        jsonAppendRawNZ(
                            &raw mut jx,
                            b"#\0".as_ptr() as *const ::core::ffi::c_char,
                            1 as u32_0,
                        );
                    }
                    jsonAppendRaw(&raw mut jx, zPath, nPath as u32_0);
                    jsonAppendRawNZ(
                        &raw mut jx,
                        b"]\0".as_ptr() as *const ::core::ffi::c_char,
                        2 as u32_0,
                    );
                } else if jsonAllAlphanum(zPath, nPath) != 0 {
                    jsonAppendRawNZ(
                        &raw mut jx,
                        b".\0".as_ptr() as *const ::core::ffi::c_char,
                        1 as u32_0,
                    );
                    jsonAppendRaw(&raw mut jx, zPath, nPath as u32_0);
                } else if *zPath.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == '[' as i32
                    && nPath >= 3 as ::core::ffi::c_int
                    && *zPath.offset((nPath - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == ']' as i32
                {
                    jsonAppendRaw(&raw mut jx, zPath, nPath as u32_0);
                } else {
                    jsonAppendRawNZ(
                        &raw mut jx,
                        b".\"\0".as_ptr() as *const ::core::ffi::c_char,
                        2 as u32_0,
                    );
                    jsonAppendRaw(&raw mut jx, zPath, nPath as u32_0);
                    jsonAppendRawNZ(
                        &raw mut jx,
                        b"\"\0".as_ptr() as *const ::core::ffi::c_char,
                        1 as u32_0,
                    );
                }
                jsonStringTerminate(&raw mut jx);
                j = jsonLookupStep(p, 0 as u32_0, jx.zBuf, 0 as u32_0);
                jsonStringReset(&raw mut jx);
            } else {
                jsonBadPathError(ctx, zPath);
                c2rust_current_block = 2603583310971537513;
                break;
            }
            if j < (*p).nBlob {
                if argc == 2 as ::core::ffi::c_int {
                    if flags & JSON_JSON != 0 {
                        jsonStringInit(&raw mut jx, ctx);
                        jsonTranslateBlobToText(p, j, &raw mut jx);
                        jsonReturnString(
                            &raw mut jx,
                            ::core::ptr::null_mut::<JsonParse>(),
                            ::core::ptr::null_mut::<sqlite3_context>(),
                        );
                        jsonStringReset(&raw mut jx);
                        sqlite3_result_subtype(ctx, JSON_SUBTYPE as ::core::ffi::c_uint);
                    } else {
                        jsonReturnFromBlob(p, j, ctx, 0 as ::core::ffi::c_int);
                        if flags & (JSON_SQL | JSON_BLOB) == 0 as ::core::ffi::c_int
                            && *(*p).aBlob.offset(j as isize) as ::core::ffi::c_int
                                & 0xf as ::core::ffi::c_int >= JSONB_ARRAY
                        {
                            sqlite3_result_subtype(
                                ctx,
                                JSON_SUBTYPE as ::core::ffi::c_uint,
                            );
                        }
                    }
                } else {
                    jsonAppendSeparator(&raw mut jx);
                    jsonTranslateBlobToText(p, j, &raw mut jx);
                }
            } else if j == JSON_LOOKUP_NOTFOUND as u32_0 {
                if argc == 2 as ::core::ffi::c_int {
                    c2rust_current_block = 2603583310971537513;
                    break;
                }
                jsonAppendSeparator(&raw mut jx);
                jsonAppendRawNZ(
                    &raw mut jx,
                    b"null\0".as_ptr() as *const ::core::ffi::c_char,
                    4 as u32_0,
                );
            } else if j == JSON_LOOKUP_ERROR as u32_0 {
                sqlite3_result_error(
                    ctx,
                    b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                c2rust_current_block = 2603583310971537513;
                break;
            } else {
                jsonBadPathError(ctx, zPath);
                c2rust_current_block = 2603583310971537513;
                break;
            }
            i += 1;
        }
        match c2rust_current_block {
            2116367355679836638 => {
                if argc > 2 as ::core::ffi::c_int {
                    jsonAppendChar(&raw mut jx, ']' as i32 as ::core::ffi::c_char);
                    jsonReturnString(
                        &raw mut jx,
                        ::core::ptr::null_mut::<JsonParse>(),
                        ::core::ptr::null_mut::<sqlite3_context>(),
                    );
                    if flags & JSON_BLOB == 0 as ::core::ffi::c_int {
                        sqlite3_result_subtype(ctx, JSON_SUBTYPE as ::core::ffi::c_uint);
                    }
                }
            }
            _ => {}
        }
        jsonStringReset(&raw mut jx);
        jsonParseFree(p);
    }
}
pub const JSON_MERGE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_MERGE_BADTARGET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JSON_MERGE_BADPATCH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const JSON_MERGE_OOM: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
unsafe extern "C" fn jsonMergePatch(
    mut pTarget: *mut JsonParse,
    mut iTarget: u32_0,
    mut pPatch: *const JsonParse,
    mut iPatch: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut x: u8_0 = 0;
        let mut n: u32_0 = 0;
        let mut sz: u32_0 = 0 as u32_0;
        let mut iTCursor: u32_0 = 0;
        let mut iTStart: u32_0 = 0;
        let mut iTEndBE: u32_0 = 0;
        let mut iTEnd: u32_0 = 0;
        let mut eTLabel: u8_0 = 0;
        let mut iTLabel: u32_0 = 0 as u32_0;
        let mut nTLabel: u32_0 = 0 as u32_0;
        let mut szTLabel: u32_0 = 0 as u32_0;
        let mut iTValue: u32_0 = 0 as u32_0;
        let mut nTValue: u32_0 = 0 as u32_0;
        let mut szTValue: u32_0 = 0 as u32_0;
        let mut iPCursor: u32_0 = 0;
        let mut iPEnd: u32_0 = 0;
        let mut ePLabel: u8_0 = 0;
        let mut iPLabel: u32_0 = 0;
        let mut nPLabel: u32_0 = 0;
        let mut szPLabel: u32_0 = 0;
        let mut iPValue: u32_0 = 0;
        let mut nPValue: u32_0 = 0;
        let mut szPValue: u32_0 = 0;
        x = (*(*pPatch).aBlob.offset(iPatch as isize) as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int) as u8_0;
        if x as ::core::ffi::c_int != JSONB_OBJECT {
            let mut szPatch: u32_0 = 0;
            let mut szTarget: u32_0 = 0;
            n = jsonbPayloadSize(pPatch, iPatch, &raw mut sz);
            szPatch = n.wrapping_add(sz);
            sz = 0 as u32_0;
            n = jsonbPayloadSize(pTarget, iTarget, &raw mut sz);
            szTarget = n.wrapping_add(sz);
            jsonBlobEdit(
                pTarget,
                iTarget,
                szTarget,
                (*pPatch).aBlob.offset(iPatch as isize),
                szPatch,
            );
            return if (*pTarget).oom as ::core::ffi::c_int != 0 {
                JSON_MERGE_OOM
            } else {
                JSON_MERGE_OK
            };
        }
        x = (*(*pTarget).aBlob.offset(iTarget as isize) as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int) as u8_0;
        if x as ::core::ffi::c_int != JSONB_OBJECT {
            n = jsonbPayloadSize(pTarget, iTarget, &raw mut sz);
            jsonBlobEdit(
                pTarget,
                iTarget.wrapping_add(n),
                sz,
                ::core::ptr::null::<u8_0>(),
                0 as u32_0,
            );
            x = *(*pTarget).aBlob.offset(iTarget as isize);
            *(*pTarget).aBlob.offset(iTarget as isize) = (x as ::core::ffi::c_int
                & 0xf0 as ::core::ffi::c_int | JSONB_OBJECT) as u8_0;
        }
        n = jsonbPayloadSize(pPatch, iPatch, &raw mut sz);
        if n == 0 as u32_0 {
            return JSON_MERGE_BADPATCH;
        }
        iPCursor = iPatch.wrapping_add(n);
        iPEnd = iPCursor.wrapping_add(sz);
        n = jsonbPayloadSize(pTarget, iTarget, &raw mut sz);
        if n == 0 as u32_0 {
            return JSON_MERGE_BADTARGET;
        }
        iTStart = iTarget.wrapping_add(n);
        iTEndBE = iTStart.wrapping_add(sz);
        while iPCursor < iPEnd {
            iPLabel = iPCursor;
            ePLabel = (*(*pPatch).aBlob.offset(iPCursor as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as u8_0;
            if (ePLabel as ::core::ffi::c_int) < JSONB_TEXT
                || ePLabel as ::core::ffi::c_int > JSONB_TEXTRAW
            {
                return JSON_MERGE_BADPATCH;
            }
            nPLabel = jsonbPayloadSize(pPatch, iPCursor, &raw mut szPLabel);
            if nPLabel == 0 as u32_0 {
                return JSON_MERGE_BADPATCH;
            }
            iPValue = iPCursor.wrapping_add(nPLabel).wrapping_add(szPLabel);
            if iPValue >= iPEnd {
                return JSON_MERGE_BADPATCH;
            }
            nPValue = jsonbPayloadSize(pPatch, iPValue, &raw mut szPValue);
            if nPValue == 0 as u32_0 {
                return JSON_MERGE_BADPATCH;
            }
            iPCursor = iPValue.wrapping_add(nPValue).wrapping_add(szPValue);
            if iPCursor > iPEnd {
                return JSON_MERGE_BADPATCH;
            }
            iTCursor = iTStart;
            iTEnd = iTEndBE.wrapping_add((*pTarget).delta as u32_0);
            while iTCursor < iTEnd {
                let mut isEqual: ::core::ffi::c_int = 0;
                iTLabel = iTCursor;
                eTLabel = (*(*pTarget).aBlob.offset(iTCursor as isize)
                    as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as u8_0;
                if (eTLabel as ::core::ffi::c_int) < JSONB_TEXT
                    || eTLabel as ::core::ffi::c_int > JSONB_TEXTRAW
                {
                    return JSON_MERGE_BADTARGET;
                }
                nTLabel = jsonbPayloadSize(pTarget, iTCursor, &raw mut szTLabel);
                if nTLabel == 0 as u32_0 {
                    return JSON_MERGE_BADTARGET;
                }
                iTValue = iTLabel.wrapping_add(nTLabel).wrapping_add(szTLabel);
                if iTValue >= iTEnd {
                    return JSON_MERGE_BADTARGET;
                }
                nTValue = jsonbPayloadSize(pTarget, iTValue, &raw mut szTValue);
                if nTValue == 0 as u32_0 {
                    return JSON_MERGE_BADTARGET;
                }
                if iTValue.wrapping_add(nTValue).wrapping_add(szTValue) > iTEnd {
                    return JSON_MERGE_BADTARGET;
                }
                isEqual = jsonLabelCompare(
                    (*pPatch).aBlob.offset(iPLabel.wrapping_add(nPLabel) as isize)
                        as *mut u8_0 as *const ::core::ffi::c_char,
                    szPLabel,
                    (ePLabel as ::core::ffi::c_int == JSONB_TEXT
                        || ePLabel as ::core::ffi::c_int == JSONB_TEXTRAW)
                        as ::core::ffi::c_int,
                    (*pTarget).aBlob.offset(iTLabel.wrapping_add(nTLabel) as isize)
                        as *mut u8_0 as *const ::core::ffi::c_char,
                    szTLabel,
                    (eTLabel as ::core::ffi::c_int == JSONB_TEXT
                        || eTLabel as ::core::ffi::c_int == JSONB_TEXTRAW)
                        as ::core::ffi::c_int,
                );
                if isEqual != 0 {
                    break;
                }
                iTCursor = iTValue.wrapping_add(nTValue).wrapping_add(szTValue);
            }
            x = (*(*pPatch).aBlob.offset(iPValue as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as u8_0;
            if iTCursor < iTEnd {
                if x as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    jsonBlobEdit(
                        pTarget,
                        iTLabel,
                        nTLabel
                            .wrapping_add(szTLabel)
                            .wrapping_add(nTValue)
                            .wrapping_add(szTValue),
                        ::core::ptr::null::<u8_0>(),
                        0 as u32_0,
                    );
                    if (*pTarget).oom != 0 {
                        return JSON_MERGE_OOM;
                    }
                } else {
                    let mut rc: ::core::ffi::c_int = 0;
                    let mut savedDelta: ::core::ffi::c_int = (*pTarget).delta;
                    (*pTarget).delta = 0 as ::core::ffi::c_int;
                    rc = jsonMergePatch(pTarget, iTValue, pPatch, iPValue);
                    if rc != 0 {
                        return rc;
                    }
                    (*pTarget).delta += savedDelta;
                }
            } else if x as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                let mut szNew: u32_0 = szPLabel.wrapping_add(nPLabel);
                if *(*pPatch).aBlob.offset(iPValue as isize) as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int != JSONB_OBJECT
                {
                    jsonBlobEdit(
                        pTarget,
                        iTEnd,
                        0 as u32_0,
                        ::core::ptr::null::<u8_0>(),
                        szPValue.wrapping_add(nPValue).wrapping_add(szNew),
                    );
                    if (*pTarget).oom != 0 {
                        return JSON_MERGE_OOM;
                    }
                    memcpy(
                        (*pTarget).aBlob.offset(iTEnd as isize) as *mut u8_0
                            as *mut ::core::ffi::c_void,
                        (*pPatch).aBlob.offset(iPLabel as isize) as *mut u8_0
                            as *const ::core::ffi::c_void,
                        szNew as size_t,
                    );
                    memcpy(
                        (*pTarget).aBlob.offset(iTEnd.wrapping_add(szNew) as isize)
                            as *mut u8_0 as *mut ::core::ffi::c_void,
                        (*pPatch).aBlob.offset(iPValue as isize) as *mut u8_0
                            as *const ::core::ffi::c_void,
                        szPValue.wrapping_add(nPValue) as size_t,
                    );
                } else {
                    let mut rc_0: ::core::ffi::c_int = 0;
                    let mut savedDelta_0: ::core::ffi::c_int = 0;
                    jsonBlobEdit(
                        pTarget,
                        iTEnd,
                        0 as u32_0,
                        ::core::ptr::null::<u8_0>(),
                        szNew.wrapping_add(1 as u32_0),
                    );
                    if (*pTarget).oom != 0 {
                        return JSON_MERGE_OOM;
                    }
                    memcpy(
                        (*pTarget).aBlob.offset(iTEnd as isize) as *mut u8_0
                            as *mut ::core::ffi::c_void,
                        (*pPatch).aBlob.offset(iPLabel as isize) as *mut u8_0
                            as *const ::core::ffi::c_void,
                        szNew as size_t,
                    );
                    *(*pTarget).aBlob.offset(iTEnd.wrapping_add(szNew) as isize) = 0
                        as u8_0;
                    savedDelta_0 = (*pTarget).delta;
                    (*pTarget).delta = 0 as ::core::ffi::c_int;
                    rc_0 = jsonMergePatch(
                        pTarget,
                        iTEnd.wrapping_add(szNew),
                        pPatch,
                        iPValue,
                    );
                    if rc_0 != 0 {
                        return rc_0;
                    }
                    (*pTarget).delta += savedDelta_0;
                }
            }
        }
        if (*pTarget).delta != 0 {
            jsonAfterEditSizeAdjust(pTarget, iTarget);
        }
        return if (*pTarget).oom as ::core::ffi::c_int != 0 {
            JSON_MERGE_OOM
        } else {
            JSON_MERGE_OK
        };
    }
}
unsafe extern "C" fn jsonPatchFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pTarget: *mut JsonParse = ::core::ptr::null_mut::<JsonParse>();
        let mut pPatch: *mut JsonParse = ::core::ptr::null_mut::<JsonParse>();
        let mut rc: ::core::ffi::c_int = 0;
        pTarget = jsonParseFuncArg(
            ctx,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            JSON_EDITABLE as u32_0,
        );
        if pTarget.is_null() {
            return;
        }
        pPatch = jsonParseFuncArg(
            ctx,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            0 as u32_0,
        );
        if !pPatch.is_null() {
            rc = jsonMergePatch(pTarget, 0 as u32_0, pPatch, 0 as u32_0);
            if rc == JSON_MERGE_OK {
                jsonReturnParse(ctx, pTarget);
            } else if rc == JSON_MERGE_OOM {
                sqlite3_result_error_nomem(ctx);
            } else {
                sqlite3_result_error(
                    ctx,
                    b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
            }
            jsonParseFree(pPatch);
        }
        jsonParseFree(pTarget);
    }
}
unsafe extern "C" fn jsonObjectFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut jx: JsonString = JsonString {
            pCtx: ::core::ptr::null_mut::<sqlite3_context>(),
            zBuf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            nUsed: 0,
            bStatic: 0,
            eErr: 0,
            zSpace: [0; 100]

        };
        let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut n: u32_0 = 0;
        if argc & 1 as ::core::ffi::c_int != 0 {
            sqlite3_result_error(
                ctx,
                b"json_object() requires an even number of arguments\0".as_ptr()
                    as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            );
            return;
        }
        jsonStringInit(&raw mut jx, ctx);
        jsonAppendChar(&raw mut jx, '{' as i32 as ::core::ffi::c_char);
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            if sqlite3_value_type(*argv.offset(i as isize)) != SQLITE_TEXT {
                sqlite3_result_error(
                    ctx,
                    b"json_object() labels must be TEXT\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                jsonStringReset(&raw mut jx);
                return;
            }
            jsonAppendSeparator(&raw mut jx);
            z = sqlite3_value_text(*argv.offset(i as isize))
                as *const ::core::ffi::c_char;
            n = sqlite3_value_bytes(*argv.offset(i as isize)) as u32_0;
            jsonAppendString(&raw mut jx, z, n);
            jsonAppendChar(&raw mut jx, ':' as i32 as ::core::ffi::c_char);
            jsonAppendSqlValue(
                &raw mut jx,
                *argv.offset((i + 1 as ::core::ffi::c_int) as isize),
            );
            i += 2 as ::core::ffi::c_int;
        }
        jsonAppendChar(&raw mut jx, '}' as i32 as ::core::ffi::c_char);
        jsonReturnString(
            &raw mut jx,
            ::core::ptr::null_mut::<JsonParse>(),
            ::core::ptr::null_mut::<sqlite3_context>(),
        );
        sqlite3_result_subtype(ctx, JSON_SUBTYPE as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn jsonRemoveFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut p: *mut JsonParse = ::core::ptr::null_mut::<JsonParse>();
        let mut zPath: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: u32_0 = 0;
        if argc < 1 as ::core::ffi::c_int {
            return;
        }
        p = jsonParseFuncArg(
            ctx,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            (if argc > 1 as ::core::ffi::c_int {
                JSON_EDITABLE
            } else {
                0 as ::core::ffi::c_int
            }) as u32_0,
        );
        if p.is_null() {
            return;
        }
        i = 1 as ::core::ffi::c_int;
        loop {
            if !(i < argc) {
                c2rust_current_block = 224731115979188411;
                break;
            }
            zPath = sqlite3_value_text(*argv.offset(i as isize))
                as *const ::core::ffi::c_char;
            if zPath.is_null() {
                c2rust_current_block = 15310964101578011829;
                break;
            }
            if *zPath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '$' as i32
            {
                c2rust_current_block = 14888089897950921805;
                break;
            }
            if *zPath.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                c2rust_current_block = 15310964101578011829;
                break;
            }
            (*p).eEdit = JEDIT_DEL as u8_0;
            (*p).delta = 0 as ::core::ffi::c_int;
            rc = jsonLookupStep(
                p,
                0 as u32_0,
                zPath.offset(1 as ::core::ffi::c_int as isize),
                0 as u32_0,
            );
            if rc >= JSON_LOOKUP_PATHERROR as u32_0 {
                if !(rc == JSON_LOOKUP_NOTFOUND as u32_0) {
                    if rc == JSON_LOOKUP_PATHERROR as u32_0 {
                        jsonBadPathError(ctx, zPath);
                    } else {
                        sqlite3_result_error(
                            ctx,
                            b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                        );
                    }
                    c2rust_current_block = 15310964101578011829;
                    break;
                }
            }
            i += 1;
        }
        match c2rust_current_block {
            224731115979188411 => {
                jsonReturnParse(ctx, p);
                jsonParseFree(p);
                return;
            }
            14888089897950921805 => {
                jsonBadPathError(ctx, zPath);
            }
            _ => {}
        }
        jsonParseFree(p);
    }
}
unsafe extern "C" fn jsonReplaceFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        if argc < 1 as ::core::ffi::c_int {
            return;
        }
        if argc & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            jsonWrongNumArgs(ctx, b"replace\0".as_ptr() as *const ::core::ffi::c_char);
            return;
        }
        jsonInsertIntoBlob(ctx, argc, argv, JEDIT_REPL);
    }
}
unsafe extern "C" fn jsonSetFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut flags: ::core::ffi::c_int = sqlite3_user_data(ctx) as intptr_t
            as ::core::ffi::c_int;
        let mut bIsSet: ::core::ffi::c_int = (flags & JSON_ISSET
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if argc < 1 as ::core::ffi::c_int {
            return;
        }
        if argc & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            jsonWrongNumArgs(
                ctx,
                if bIsSet != 0 {
                    b"set\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"insert\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            return;
        }
        jsonInsertIntoBlob(
            ctx,
            argc,
            argv,
            if bIsSet != 0 { JEDIT_SET } else { JEDIT_INS },
        );
    }
}
unsafe extern "C" fn jsonTypeFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut p: *mut JsonParse = ::core::ptr::null_mut::<JsonParse>();
        let mut zPath: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut i: u32_0 = 0;
        p = jsonParseFuncArg(
            ctx,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            0 as u32_0,
        );
        if p.is_null() {
            return;
        }
        if argc == 2 as ::core::ffi::c_int {
            zPath = sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
            if zPath.is_null() {
                c2rust_current_block = 15260353391160353906;
            } else if *zPath.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int != '$' as i32
            {
                jsonBadPathError(ctx, zPath);
                c2rust_current_block = 15260353391160353906;
            } else {
                i = jsonLookupStep(
                    p,
                    0 as u32_0,
                    zPath.offset(1 as ::core::ffi::c_int as isize),
                    0 as u32_0,
                );
                if i >= JSON_LOOKUP_PATHERROR as u32_0 {
                    if !(i == JSON_LOOKUP_NOTFOUND as u32_0) {
                        if i == JSON_LOOKUP_PATHERROR as u32_0 {
                            jsonBadPathError(ctx, zPath);
                        } else {
                            sqlite3_result_error(
                                ctx,
                                b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                                -1 as ::core::ffi::c_int,
                            );
                        }
                    }
                    c2rust_current_block = 15260353391160353906;
                } else {
                    c2rust_current_block = 17833034027772472439;
                }
            }
        } else {
            i = 0 as u32_0;
            c2rust_current_block = 17833034027772472439;
        }
        match c2rust_current_block {
            17833034027772472439 => {
                sqlite3_result_text(
                    ctx,
                    jsonbType[(*(*p).aBlob.offset(i as isize) as ::core::ffi::c_int
                        & 0xf as ::core::ffi::c_int) as usize],
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            _ => {}
        }
        jsonParseFree(p);
    }
}
unsafe extern "C" fn jsonPrettyFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut s: JsonString = JsonString {
            pCtx: ::core::ptr::null_mut::<sqlite3_context>(),
            zBuf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            nUsed: 0,
            bStatic: 0,
            eErr: 0,
            zSpace: [0; 100]

        };
        let mut x: JsonPretty = JsonPretty {
            pParse: ::core::ptr::null_mut::<JsonParse>(),
            pOut: ::core::ptr::null_mut::<JsonString>(),
            zIndent: ::core::ptr::null::<::core::ffi::c_char>(),
            szIndent: 0,
            nIndent: 0

        };
        memset(
            &raw mut x as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<JsonPretty>() as size_t,
        );
        x.pParse = jsonParseFuncArg(
            ctx,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            0 as u32_0,
        );
        if x.pParse.is_null() {
            return;
        }
        x.pOut = &raw mut s;
        jsonStringInit(&raw mut s, ctx);
        if argc == 1 as ::core::ffi::c_int
            || {
                x.zIndent = sqlite3_value_text(
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                ) as *const ::core::ffi::c_char;
                x.zIndent.is_null()
            }
        {
            x.zIndent = b"    \0".as_ptr() as *const ::core::ffi::c_char;
            x.szIndent = 4 as u32_0;
        } else {
            x.szIndent = strlen(x.zIndent) as u32_0;
        }
        jsonTranslateBlobToPrettyText(&raw mut x, 0 as u32_0);
        jsonReturnString(
            &raw mut s,
            ::core::ptr::null_mut::<JsonParse>(),
            ::core::ptr::null_mut::<sqlite3_context>(),
        );
        jsonParseFree(x.pParse);
    }
}
unsafe extern "C" fn jsonValidFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut JsonParse = ::core::ptr::null_mut::<JsonParse>();
        let mut flags: u8_0 = 1 as u8_0;
        let mut res: u8_0 = 0 as u8_0;
        if argc == 2 as ::core::ffi::c_int {
            let mut f: i64_0 = sqlite3_value_int64(
                *argv.offset(1 as ::core::ffi::c_int as isize),
            ) as i64_0;
            if f < 1 as ::core::ffi::c_longlong || f > 15 as ::core::ffi::c_longlong {
                sqlite3_result_error(
                    ctx,
                    b"FLAGS parameter to json_valid() must be between 1 and 15\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                return;
            }
            flags = (f as ::core::ffi::c_longlong & 0xf as ::core::ffi::c_longlong)
                as u8_0;
        }
        let mut c2rust_current_block_31: u64;
        match sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize)) {
            SQLITE_NULL => return,
            SQLITE_BLOB => {
                let mut py: JsonParse = JsonParse {
                    aBlob: ::core::ptr::null_mut::<u8_0>(),
                    nBlob: 0,
                    nBlobAlloc: 0,
                    zJson: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    db: ::core::ptr::null_mut::<sqlite3>(),
                    nJson: 0,
                    nJPRef: 0,
                    iErr: 0,
                    iDepth: 0,
                    nErr: 0,
                    oom: 0,
                    bJsonIsRCStr: 0,
                    hasNonstd: 0,
                    bReadOnly: 0,
                    eEdit: 0,
                    delta: 0,
                    nIns: 0,
                    iLabel: 0,
                    aIns: ::core::ptr::null_mut::<u8_0>()

                };
                memset(
                    &raw mut py as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<JsonParse>() as size_t,
                );
                if jsonArgIsJsonb(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                    &raw mut py,
                ) != 0
                {
                    if flags as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0 {
                        res = 1 as u8_0;
                    } else if flags as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int
                        != 0
                    {
                        res = (0 as u32_0
                            == jsonbValidityCheck(
                                &raw mut py,
                                0 as u32_0,
                                py.nBlob,
                                1 as u32_0,
                            )) as ::core::ffi::c_int as u8_0;
                    }
                    c2rust_current_block_31 = 652864300344834934;
                } else {
                    c2rust_current_block_31 = 17833034027772472439;
                }
            }
            _ => {
                c2rust_current_block_31 = 17833034027772472439;
            }
        }
        match c2rust_current_block_31 {
            17833034027772472439 => {
                let mut px: JsonParse = JsonParse {
                    aBlob: ::core::ptr::null_mut::<u8_0>(),
                    nBlob: 0,
                    nBlobAlloc: 0,
                    zJson: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    db: ::core::ptr::null_mut::<sqlite3>(),
                    nJson: 0,
                    nJPRef: 0,
                    iErr: 0,
                    iDepth: 0,
                    nErr: 0,
                    oom: 0,
                    bJsonIsRCStr: 0,
                    hasNonstd: 0,
                    bReadOnly: 0,
                    eEdit: 0,
                    delta: 0,
                    nIns: 0,
                    iLabel: 0,
                    aIns: ::core::ptr::null_mut::<u8_0>()

                };
                if !(flags as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int)
                {
                    memset(
                        &raw mut px as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<JsonParse>() as size_t,
                    );
                    p = jsonParseFuncArg(
                        ctx,
                        *argv.offset(0 as ::core::ffi::c_int as isize),
                        JSON_KEEPERROR as u32_0,
                    );
                    if !p.is_null() {
                        if (*p).oom != 0 {
                            sqlite3_result_error_nomem(ctx);
                        } else if !((*p).nErr != 0) {
                            if flags as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int
                                || (*p).hasNonstd as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                            {
                                res = 1 as u8_0;
                            }
                        }
                        jsonParseFree(p);
                    } else {
                        sqlite3_result_error_nomem(ctx);
                    }
                }
            }
            _ => {}
        }
        sqlite3_result_int(ctx, res as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn jsonErrorFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut iErrPos: i64_0 = 0 as i64_0;
        let mut s: JsonParse = JsonParse {
            aBlob: ::core::ptr::null_mut::<u8_0>(),
            nBlob: 0,
            nBlobAlloc: 0,
            zJson: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            db: ::core::ptr::null_mut::<sqlite3>(),
            nJson: 0,
            nJPRef: 0,
            iErr: 0,
            iDepth: 0,
            nErr: 0,
            oom: 0,
            bJsonIsRCStr: 0,
            hasNonstd: 0,
            bReadOnly: 0,
            eEdit: 0,
            delta: 0,
            nIns: 0,
            iLabel: 0,
            aIns: ::core::ptr::null_mut::<u8_0>()

        };
        memset(
            &raw mut s as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<JsonParse>() as size_t,
        );
        s.db = sqlite3_context_db_handle(ctx);
        if jsonArgIsJsonb(*argv.offset(0 as ::core::ffi::c_int as isize), &raw mut s)
            != 0
        {
            iErrPos = jsonbValidityCheck(&raw mut s, 0 as u32_0, s.nBlob, 1 as u32_0)
                as i64_0;
        } else {
            s.zJson = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *mut ::core::ffi::c_char;
            if s.zJson.is_null() {
                return;
            }
            s.nJson = sqlite3_value_bytes(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            if jsonConvertTextToBlob(
                &raw mut s,
                ::core::ptr::null_mut::<sqlite3_context>(),
            ) != 0
            {
                if s.oom != 0 {
                    iErrPos = -1 as ::core::ffi::c_int as i64_0;
                } else {
                    let mut k: u32_0 = 0;
                    k = 0 as u32_0;
                    while k < s.iErr
                        && *s.zJson.offset(k as isize) as ::core::ffi::c_int != 0
                    {
                        if *s.zJson.offset(k as isize) as ::core::ffi::c_int
                            & 0xc0 as ::core::ffi::c_int != 0x80 as ::core::ffi::c_int
                        {
                            iErrPos += 1;
                        }
                        k = k.wrapping_add(1);
                    }
                    iErrPos += 1;
                }
            }
        }
        jsonParseReset(&raw mut s);
        if iErrPos < 0 as ::core::ffi::c_longlong {
            sqlite3_result_error_nomem(ctx);
        } else {
            sqlite3_result_int64(ctx, iErrPos as sqlite3_int64);
        };
    }
}
unsafe extern "C" fn jsonArrayStep(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pStr: *mut JsonString = ::core::ptr::null_mut::<JsonString>();
        pStr = sqlite3_aggregate_context(
            ctx,
            ::core::mem::size_of::<JsonString>() as ::core::ffi::c_int,
        ) as *mut JsonString;
        if !pStr.is_null() {
            if (*pStr).zBuf.is_null() {
                jsonStringInit(pStr, ctx);
                jsonAppendChar(pStr, '[' as i32 as ::core::ffi::c_char);
            } else if (*pStr).nUsed > 1 as ::core::ffi::c_ulonglong {
                jsonAppendChar(pStr, ',' as i32 as ::core::ffi::c_char);
            }
            (*pStr).pCtx = ctx;
            jsonAppendSqlValue(pStr, *argv.offset(0 as ::core::ffi::c_int as isize));
        }
    }
}
unsafe extern "C" fn jsonArrayCompute(
    mut ctx: *mut sqlite3_context,
    mut isFinal: ::core::ffi::c_int,
) {
    unsafe {
        let mut pStr: *mut JsonString = ::core::ptr::null_mut::<JsonString>();
        pStr = sqlite3_aggregate_context(ctx, 0 as ::core::ffi::c_int)
            as *mut JsonString;
        if !pStr.is_null() {
            let mut flags: ::core::ffi::c_int = 0;
            (*pStr).pCtx = ctx;
            jsonAppendChar(pStr, ']' as i32 as ::core::ffi::c_char);
            flags = sqlite3_user_data(ctx) as intptr_t as ::core::ffi::c_int;
            if (*pStr).eErr != 0 {
                jsonReturnString(
                    pStr,
                    ::core::ptr::null_mut::<JsonParse>(),
                    ::core::ptr::null_mut::<sqlite3_context>(),
                );
                return;
            } else if flags & JSON_BLOB != 0 {
                jsonReturnStringAsBlob(pStr);
                if isFinal != 0 {
                    if (*pStr).bStatic == 0 {
                        sqlite3RCStrUnref((*pStr).zBuf as *mut ::core::ffi::c_void);
                    }
                } else {
                    jsonStringTrimOneChar(pStr);
                }
                return;
            } else if isFinal != 0 {
                sqlite3_result_text(
                    ctx,
                    (*pStr).zBuf,
                    (*pStr).nUsed as ::core::ffi::c_int,
                    if (*pStr).bStatic as ::core::ffi::c_int != 0 {
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t)
                    } else {
                        Some(
                            sqlite3RCStrUnref
                                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                        )
                    },
                );
                (*pStr).bStatic = 1 as u8_0;
            } else {
                sqlite3_result_text(
                    ctx,
                    (*pStr).zBuf,
                    (*pStr).nUsed as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                jsonStringTrimOneChar(pStr);
            }
        } else {
            sqlite3_result_text(
                ctx,
                b"[]\0".as_ptr() as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
        }
        sqlite3_result_subtype(ctx, JSON_SUBTYPE as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn jsonArrayValue(mut ctx: *mut sqlite3_context) {
    unsafe {
        jsonArrayCompute(ctx, 0 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn jsonArrayFinal(mut ctx: *mut sqlite3_context) {
    unsafe {
        jsonArrayCompute(ctx, 1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn jsonGroupInverse(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut inStr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nNest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut c: ::core::ffi::c_char = 0;
        let mut pStr: *mut JsonString = ::core::ptr::null_mut::<JsonString>();
        pStr = sqlite3_aggregate_context(ctx, 0 as ::core::ffi::c_int)
            as *mut JsonString;
        if pStr.is_null() {
            return;
        }
        z = (*pStr).zBuf;
        i = 1 as ::core::ffi::c_uint;
        while (i as ::core::ffi::c_ulonglong) < (*pStr).nUsed
            && {
                c = *z.offset(i as isize);
                c as ::core::ffi::c_int != ',' as i32 || inStr != 0 || nNest != 0
            }
        {
            if c as ::core::ffi::c_int == '"' as i32 {
                inStr = (inStr == 0) as ::core::ffi::c_int;
            } else if c as ::core::ffi::c_int == '\\' as i32 {
                i = i.wrapping_add(1);
            } else if inStr == 0 {
                if c as ::core::ffi::c_int == '{' as i32
                    || c as ::core::ffi::c_int == '[' as i32
                {
                    nNest += 1;
                }
                if c as ::core::ffi::c_int == '}' as i32
                    || c as ::core::ffi::c_int == ']' as i32
                {
                    nNest -= 1;
                }
            }
            i = i.wrapping_add(1);
        }
        if (i as ::core::ffi::c_ulonglong) < (*pStr).nUsed {
            (*pStr).nUsed = ((*pStr).nUsed as ::core::ffi::c_ulonglong)
                .wrapping_sub(i as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
            memmove(
                z.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                z.offset(i.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                    as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                ((*pStr).nUsed as size_t).wrapping_sub(1 as size_t),
            );
            *z.offset((*pStr).nUsed as isize) = 0 as ::core::ffi::c_char;
        } else {
            (*pStr).nUsed = 1 as u64_0;
        };
    }
}
unsafe extern "C" fn jsonObjectStep(
    mut ctx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pStr: *mut JsonString = ::core::ptr::null_mut::<JsonString>();
        let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut n: u32_0 = 0;
        pStr = sqlite3_aggregate_context(
            ctx,
            ::core::mem::size_of::<JsonString>() as ::core::ffi::c_int,
        ) as *mut JsonString;
        if !pStr.is_null() {
            z = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
            n = sqlite3Strlen30(z) as u32_0;
            if (*pStr).zBuf.is_null() {
                jsonStringInit(pStr, ctx);
                jsonAppendChar(pStr, '{' as i32 as ::core::ffi::c_char);
            } else if (*pStr).nUsed > 1 as ::core::ffi::c_ulonglong && !z.is_null() {
                jsonAppendChar(pStr, ',' as i32 as ::core::ffi::c_char);
            }
            (*pStr).pCtx = ctx;
            if !z.is_null() {
                jsonAppendString(pStr, z, n);
                jsonAppendChar(pStr, ':' as i32 as ::core::ffi::c_char);
                jsonAppendSqlValue(pStr, *argv.offset(1 as ::core::ffi::c_int as isize));
            }
        }
    }
}
unsafe extern "C" fn jsonObjectCompute(
    mut ctx: *mut sqlite3_context,
    mut isFinal: ::core::ffi::c_int,
) {
    unsafe {
        let mut pStr: *mut JsonString = ::core::ptr::null_mut::<JsonString>();
        pStr = sqlite3_aggregate_context(ctx, 0 as ::core::ffi::c_int)
            as *mut JsonString;
        if !pStr.is_null() {
            let mut flags: ::core::ffi::c_int = 0;
            jsonAppendChar(pStr, '}' as i32 as ::core::ffi::c_char);
            (*pStr).pCtx = ctx;
            flags = sqlite3_user_data(ctx) as intptr_t as ::core::ffi::c_int;
            if (*pStr).eErr != 0 {
                jsonReturnString(
                    pStr,
                    ::core::ptr::null_mut::<JsonParse>(),
                    ::core::ptr::null_mut::<sqlite3_context>(),
                );
                return;
            } else if flags & JSON_BLOB != 0 {
                jsonReturnStringAsBlob(pStr);
                if isFinal != 0 {
                    if (*pStr).bStatic == 0 {
                        sqlite3RCStrUnref((*pStr).zBuf as *mut ::core::ffi::c_void);
                    }
                } else {
                    jsonStringTrimOneChar(pStr);
                }
                return;
            } else if isFinal != 0 {
                sqlite3_result_text(
                    ctx,
                    (*pStr).zBuf,
                    (*pStr).nUsed as ::core::ffi::c_int,
                    if (*pStr).bStatic as ::core::ffi::c_int != 0 {
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t)
                    } else {
                        Some(
                            sqlite3RCStrUnref
                                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                        )
                    },
                );
                (*pStr).bStatic = 1 as u8_0;
            } else {
                sqlite3_result_text(
                    ctx,
                    (*pStr).zBuf,
                    (*pStr).nUsed as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                jsonStringTrimOneChar(pStr);
            }
        } else {
            sqlite3_result_text(
                ctx,
                b"{}\0".as_ptr() as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
        }
        sqlite3_result_subtype(ctx, JSON_SUBTYPE as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn jsonObjectValue(mut ctx: *mut sqlite3_context) {
    unsafe {
        jsonObjectCompute(ctx, 0 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn jsonObjectFinal(mut ctx: *mut sqlite3_context) {
    unsafe {
        jsonObjectCompute(ctx, 1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn jsonEachConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pNew: *mut JsonEachConnection = ::core::ptr::null_mut::<
            JsonEachConnection,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3_declare_vtab(
            db,
            b"CREATE TABLE x(key,value,type,atom,id,parent,fullkey,path,json HIDDEN,root HIDDEN)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        if rc == SQLITE_OK {
            pNew = sqlite3DbMallocZero(
                db,
                ::core::mem::size_of::<JsonEachConnection>() as u64_0,
            ) as *mut JsonEachConnection;
            *ppVtab = pNew as *mut sqlite3_vtab;
            if pNew.is_null() {
                return SQLITE_NOMEM;
            }
            sqlite3_vtab_config(db, SQLITE_VTAB_INNOCUOUS);
            (*pNew).db = db;
            (*pNew).eMode = (if *(*argv.offset(0 as ::core::ffi::c_int as isize))
                .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'b' as i32
            {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as u8_0;
            (*pNew).bRecursive = (*(*argv.offset(0 as ::core::ffi::c_int as isize))
                .offset(
                    (4 as ::core::ffi::c_int + (*pNew).eMode as ::core::ffi::c_int)
                        as isize,
                ) as ::core::ffi::c_int == 't' as i32) as ::core::ffi::c_int as u8_0;
        }
        return rc;
    }
}
pub const JEACH_KEY: ::core::ffi::c_int = 0;
pub const JEACH_VALUE: ::core::ffi::c_int = 1;
pub const JEACH_TYPE: ::core::ffi::c_int = 2;
pub const JEACH_ATOM: ::core::ffi::c_int = 3;
pub const JEACH_ID: ::core::ffi::c_int = 4;
pub const JEACH_PARENT: ::core::ffi::c_int = 5;
pub const JEACH_FULLKEY: ::core::ffi::c_int = 6;
pub const JEACH_PATH: ::core::ffi::c_int = 7;
pub const JEACH_JSON: ::core::ffi::c_int = 8;
unsafe extern "C" fn jsonEachDisconnect(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut JsonEachConnection = pVtab as *mut JsonEachConnection;
        sqlite3DbFree((*p).db, pVtab as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn jsonEachOpen(
    mut p: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVtab: *mut JsonEachConnection = p as *mut JsonEachConnection;
        let mut pCur: *mut JsonEachCursor = ::core::ptr::null_mut::<JsonEachCursor>();
        pCur = sqlite3DbMallocZero(
            (*pVtab).db,
            ::core::mem::size_of::<JsonEachCursor>() as u64_0,
        ) as *mut JsonEachCursor;
        if pCur.is_null() {
            return SQLITE_NOMEM;
        }
        (*pCur).db = (*pVtab).db;
        (*pCur).eMode = (*pVtab).eMode;
        (*pCur).bRecursive = (*pVtab).bRecursive;
        jsonStringZero(&raw mut (*pCur).path);
        *ppCursor = &raw mut (*pCur).base;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn jsonEachCursorReset(mut p: *mut JsonEachCursor) {
    unsafe {
        jsonParseReset(&raw mut (*p).sParse);
        jsonStringReset(&raw mut (*p).path);
        sqlite3DbFree((*p).db, (*p).aParent as *mut ::core::ffi::c_void);
        (*p).iRowid = 0 as u32_0;
        (*p).i = 0 as u32_0;
        (*p).aParent = ::core::ptr::null_mut::<JsonParent>();
        (*p).nParent = 0 as u32_0;
        (*p).nParentAlloc = 0 as u32_0;
        (*p).iEnd = 0 as u32_0;
        (*p).eType = 0 as u8_0;
    }
}
unsafe extern "C" fn jsonEachClose(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
        jsonEachCursorReset(p);
        sqlite3DbFree((*p).db, cur as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn jsonEachEof(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
        return ((*p).i >= (*p).iEnd) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn jsonSkipLabel(mut p: *mut JsonEachCursor) -> ::core::ffi::c_int {
    unsafe {
        if (*p).eType as ::core::ffi::c_int == JSONB_OBJECT {
            let mut sz: u32_0 = 0 as u32_0;
            let mut n: u32_0 = jsonbPayloadSize(
                &raw mut (*p).sParse,
                (*p).i,
                &raw mut sz,
            );
            return (*p).i.wrapping_add(n).wrapping_add(sz) as ::core::ffi::c_int;
        } else {
            return (*p).i as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn jsonAppendPathName(mut p: *mut JsonEachCursor) {
    unsafe {
        if (*p).eType as ::core::ffi::c_int == JSONB_ARRAY {
            jsonPrintf(
                30 as ::core::ffi::c_int,
                &raw mut (*p).path,
                b"[%lld]\0".as_ptr() as *const ::core::ffi::c_char,
                (*(*p).aParent.offset((*p).nParent.wrapping_sub(1 as u32_0) as isize))
                    .iKey,
            );
        } else {
            let mut n: u32_0 = 0;
            let mut sz: u32_0 = 0 as u32_0;
            let mut k: u32_0 = 0;
            let mut i: u32_0 = 0;
            let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut needQuote: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            n = jsonbPayloadSize(&raw mut (*p).sParse, (*p).i, &raw mut sz);
            k = (*p).i.wrapping_add(n);
            z = (*p).sParse.aBlob.offset(k as isize) as *mut u8_0
                as *const ::core::ffi::c_char;
            if sz == 0 as u32_0
                || *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(
                        *z.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar as isize,
                    ) as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int == 0
            {
                needQuote = 1 as ::core::ffi::c_int;
            } else {
                i = 0 as u32_0;
                while i < sz {
                    if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(*z.offset(i as isize) as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int & 0x6 as ::core::ffi::c_int == 0
                    {
                        needQuote = 1 as ::core::ffi::c_int;
                        break;
                    } else {
                        i = i.wrapping_add(1);
                    }
                }
            }
            if needQuote != 0 {
                jsonPrintf(
                    sz.wrapping_add(4 as u32_0) as ::core::ffi::c_int,
                    &raw mut (*p).path,
                    b".\"%.*s\"\0".as_ptr() as *const ::core::ffi::c_char,
                    sz,
                    z,
                );
            } else {
                jsonPrintf(
                    sz.wrapping_add(2 as u32_0) as ::core::ffi::c_int,
                    &raw mut (*p).path,
                    b".%.*s\0".as_ptr() as *const ::core::ffi::c_char,
                    sz,
                    z,
                );
            }
        };
    }
}
unsafe extern "C" fn jsonEachNext(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*p).bRecursive != 0 {
            let mut x: u8_0 = 0;
            let mut levelChange: u8_0 = 0 as u8_0;
            let mut n: u32_0 = 0;
            let mut sz: u32_0 = 0 as u32_0;
            let mut i: u32_0 = jsonSkipLabel(p) as u32_0;
            x = (*(*p).sParse.aBlob.offset(i as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as u8_0;
            n = jsonbPayloadSize(&raw mut (*p).sParse, i, &raw mut sz);
            if x as ::core::ffi::c_int == JSONB_OBJECT
                || x as ::core::ffi::c_int == JSONB_ARRAY
            {
                let mut pParent: *mut JsonParent = ::core::ptr::null_mut::<JsonParent>();
                if (*p).nParent >= (*p).nParentAlloc {
                    let mut pNew: *mut JsonParent = ::core::ptr::null_mut::<
                        JsonParent,
                    >();
                    let mut nNew: u64_0 = 0;
                    nNew = (*p)
                        .nParentAlloc
                        .wrapping_mul(2 as u32_0)
                        .wrapping_add(3 as u32_0) as u64_0;
                    pNew = sqlite3DbRealloc(
                        (*p).db,
                        (*p).aParent as *mut ::core::ffi::c_void,
                        (::core::mem::size_of::<JsonParent>() as u64_0)
                            .wrapping_mul(nNew),
                    ) as *mut JsonParent;
                    if pNew.is_null() {
                        return SQLITE_NOMEM;
                    }
                    (*p).nParentAlloc = nNew as u32_0;
                    (*p).aParent = pNew;
                }
                levelChange = 1 as u8_0;
                pParent = (*p).aParent.offset((*p).nParent as isize) as *mut JsonParent;
                (*pParent).iHead = (*p).i;
                (*pParent).iValue = i;
                (*pParent).iEnd = i.wrapping_add(n).wrapping_add(sz);
                (*pParent).iKey = -1 as ::core::ffi::c_int as i64_0;
                (*pParent).nPath = (*p).path.nUsed as u32_0;
                if (*p).eType as ::core::ffi::c_int != 0 && (*p).nParent != 0 {
                    jsonAppendPathName(p);
                    if (*p).path.eErr != 0 {
                        rc = SQLITE_NOMEM;
                    }
                }
                (*p).nParent = (*p).nParent.wrapping_add(1);
                (*p).i = i.wrapping_add(n);
            } else {
                (*p).i = i.wrapping_add(n).wrapping_add(sz);
            }
            while (*p).nParent > 0 as u32_0
                && (*p).i
                    >= (*(*p)
                        .aParent
                        .offset((*p).nParent.wrapping_sub(1 as u32_0) as isize))
                        .iEnd
            {
                (*p).nParent = (*p).nParent.wrapping_sub(1);
                (*p).path.nUsed = (*(*p).aParent.offset((*p).nParent as isize)).nPath
                    as u64_0;
                levelChange = 1 as u8_0;
            }
            if levelChange != 0 {
                if (*p).nParent > 0 as u32_0 {
                    let mut pParent_0: *mut JsonParent = (*p)
                        .aParent
                        .offset((*p).nParent.wrapping_sub(1 as u32_0) as isize)
                        as *mut JsonParent;
                    let mut iVal: u32_0 = (*pParent_0).iValue;
                    (*p).eType = (*(*p).sParse.aBlob.offset(iVal as isize)
                        as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as u8_0;
                } else {
                    (*p).eType = 0 as u8_0;
                }
            }
        } else {
            let mut n_0: u32_0 = 0;
            let mut sz_0: u32_0 = 0 as u32_0;
            let mut i_0: u32_0 = jsonSkipLabel(p) as u32_0;
            n_0 = jsonbPayloadSize(&raw mut (*p).sParse, i_0, &raw mut sz_0);
            (*p).i = i_0.wrapping_add(n_0).wrapping_add(sz_0);
        }
        if (*p).eType as ::core::ffi::c_int == JSONB_ARRAY && (*p).nParent != 0 {
            let ref mut c2rust_fresh21 = (*(*p)
                .aParent
                .offset((*p).nParent.wrapping_sub(1 as u32_0) as isize))
                .iKey;
            *c2rust_fresh21 += 1;
        }
        (*p).iRowid = (*p).iRowid.wrapping_add(1);
        return rc;
    }
}
unsafe extern "C" fn jsonEachPathLength(
    mut p: *mut JsonEachCursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: u32_0 = (*p).path.nUsed as u32_0;
        let mut z: *mut ::core::ffi::c_char = (*p).path.zBuf;
        if (*p).iRowid == 0 as u32_0 && (*p).bRecursive as ::core::ffi::c_int != 0
            && n >= 2 as u32_0
        {
            while n > 1 as u32_0 {
                n = n.wrapping_sub(1);
                if !(*z.offset(n as isize) as ::core::ffi::c_int == '[' as i32
                    || *z.offset(n as isize) as ::core::ffi::c_int == '.' as i32)
                {
                    continue;
                }
                let mut x: u32_0 = 0;
                let mut sz: u32_0 = 0 as u32_0;
                let mut cSaved: ::core::ffi::c_char = *z.offset(n as isize);
                *z.offset(n as isize) = 0 as ::core::ffi::c_char;
                x = jsonLookupStep(
                    &raw mut (*p).sParse,
                    0 as u32_0,
                    z.offset(1 as ::core::ffi::c_int as isize),
                    0 as u32_0,
                );
                *z.offset(n as isize) = cSaved;
                if x >= JSON_LOOKUP_PATHERROR as u32_0 {
                    continue;
                }
                if x.wrapping_add(jsonbPayloadSize(&raw mut (*p).sParse, x, &raw mut sz))
                    == (*p).i
                {
                    break;
                }
            }
        }
        return n as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn jsonEachColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut iColumn: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
        match iColumn {
            JEACH_KEY => {
                if (*p).nParent == 0 as u32_0 {
                    let mut n: u32_0 = 0;
                    let mut j: u32_0 = 0;
                    if !((*p).nRoot == 1 as u32_0) {
                        j = jsonEachPathLength(p) as u32_0;
                        n = (*p).nRoot.wrapping_sub(j);
                        if !(n == 0 as u32_0) {
                            if *(*p).path.zBuf.offset(j as isize) as ::core::ffi::c_int
                                == '[' as i32
                            {
                                let mut x: i64_0 = 0;
                                sqlite3Atoi64(
                                    (*p).path.zBuf.offset(j.wrapping_add(1 as u32_0) as isize)
                                        as *mut ::core::ffi::c_char,
                                    &raw mut x,
                                    n.wrapping_sub(1 as u32_0) as ::core::ffi::c_int,
                                    SQLITE_UTF8 as u8_0,
                                );
                                sqlite3_result_int64(ctx, x as sqlite3_int64);
                            } else if *(*p)
                                .path
                                .zBuf
                                .offset(j.wrapping_add(1 as u32_0) as isize)
                                as ::core::ffi::c_int == '"' as i32
                            {
                                sqlite3_result_text(
                                    ctx,
                                    (*p).path.zBuf.offset(j.wrapping_add(2 as u32_0) as isize)
                                        as *mut ::core::ffi::c_char,
                                    n.wrapping_sub(3 as u32_0) as ::core::ffi::c_int,
                                    ::core::mem::transmute::<
                                        ::libc::intptr_t,
                                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                                );
                            } else {
                                sqlite3_result_text(
                                    ctx,
                                    (*p).path.zBuf.offset(j.wrapping_add(1 as u32_0) as isize)
                                        as *mut ::core::ffi::c_char,
                                    n.wrapping_sub(1 as u32_0) as ::core::ffi::c_int,
                                    ::core::mem::transmute::<
                                        ::libc::intptr_t,
                                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                                );
                            }
                        }
                    }
                } else if (*p).eType as ::core::ffi::c_int == JSONB_OBJECT {
                    jsonReturnFromBlob(
                        &raw mut (*p).sParse,
                        (*p).i,
                        ctx,
                        1 as ::core::ffi::c_int,
                    );
                } else {
                    sqlite3_result_int64(
                        ctx,
                        (*(*p)
                            .aParent
                            .offset((*p).nParent.wrapping_sub(1 as u32_0) as isize))
                            .iKey as sqlite3_int64,
                    );
                }
            }
            JEACH_VALUE => {
                let mut i: u32_0 = jsonSkipLabel(p) as u32_0;
                jsonReturnFromBlob(
                    &raw mut (*p).sParse,
                    i,
                    ctx,
                    (*p).eMode as ::core::ffi::c_int,
                );
                if *(*p).sParse.aBlob.offset(i as isize) as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int >= JSONB_ARRAY
                {
                    sqlite3_result_subtype(ctx, JSON_SUBTYPE as ::core::ffi::c_uint);
                }
            }
            JEACH_TYPE => {
                let mut i_0: u32_0 = jsonSkipLabel(p) as u32_0;
                let mut eType: u8_0 = (*(*p).sParse.aBlob.offset(i_0 as isize)
                    as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as u8_0;
                sqlite3_result_text(
                    ctx,
                    jsonbType[eType as usize],
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            JEACH_ATOM => {
                let mut i_1: u32_0 = jsonSkipLabel(p) as u32_0;
                if (*(*p).sParse.aBlob.offset(i_1 as isize) as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int) < JSONB_ARRAY
                {
                    jsonReturnFromBlob(
                        &raw mut (*p).sParse,
                        i_1,
                        ctx,
                        1 as ::core::ffi::c_int,
                    );
                }
            }
            JEACH_ID => {
                sqlite3_result_int64(ctx, (*p).i as sqlite3_int64);
            }
            JEACH_PARENT => {
                if (*p).nParent > 0 as u32_0
                    && (*p).bRecursive as ::core::ffi::c_int != 0
                {
                    sqlite3_result_int64(
                        ctx,
                        (*(*p)
                            .aParent
                            .offset((*p).nParent.wrapping_sub(1 as u32_0) as isize))
                            .iHead as sqlite3_int64,
                    );
                }
            }
            JEACH_FULLKEY => {
                let mut nBase: u64_0 = (*p).path.nUsed;
                if (*p).nParent != 0 {
                    jsonAppendPathName(p);
                }
                sqlite3_result_text64(
                    ctx,
                    (*p).path.zBuf,
                    (*p).path.nUsed as sqlite3_uint64,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    SQLITE_UTF8 as ::core::ffi::c_uchar,
                );
                (*p).path.nUsed = nBase;
            }
            JEACH_PATH => {
                let mut n_0: u32_0 = jsonEachPathLength(p) as u32_0;
                sqlite3_result_text64(
                    ctx,
                    (*p).path.zBuf,
                    n_0 as sqlite3_uint64,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    SQLITE_UTF8 as ::core::ffi::c_uchar,
                );
            }
            JEACH_JSON => {
                if (*p).sParse.zJson.is_null() {
                    sqlite3_result_blob(
                        ctx,
                        (*p).sParse.aBlob as *const ::core::ffi::c_void,
                        (*p).sParse.nBlob as ::core::ffi::c_int,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                } else {
                    sqlite3_result_text(
                        ctx,
                        (*p).sParse.zJson,
                        -1 as ::core::ffi::c_int,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                }
            }
            _ => {
                sqlite3_result_text(
                    ctx,
                    (*p).path.zBuf,
                    (*p).nRoot as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn jsonEachRowid(
    mut cur: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
        *pRowid = (*p).iRowid as sqlite_int64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn jsonEachBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut aIdx: [::core::ffi::c_int; 2] = [0; 2];
        let mut unusableMask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut idxMask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pConstraint: *const sqlite3_index_constraint = ::core::ptr::null::<
            sqlite3_index_constraint,
        >();
        aIdx[1 as ::core::ffi::c_int as usize] = -1 as ::core::ffi::c_int;
        aIdx[0 as ::core::ffi::c_int as usize] = aIdx[1 as ::core::ffi::c_int as usize];
        pConstraint = (*pIdxInfo).aConstraint;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdxInfo).nConstraint {
            let mut iCol: ::core::ffi::c_int = 0;
            let mut iMask: ::core::ffi::c_int = 0;
            if !((*pConstraint).iColumn < JEACH_JSON) {
                iCol = (*pConstraint).iColumn - JEACH_JSON;
                iMask = (1 as ::core::ffi::c_int) << iCol;
                if (*pConstraint).usable as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    unusableMask |= iMask;
                } else if (*pConstraint).op as ::core::ffi::c_int
                    == SQLITE_INDEX_CONSTRAINT_EQ
                {
                    aIdx[iCol as usize] = i;
                    idxMask |= iMask;
                }
            }
            i += 1;
            pConstraint = pConstraint.offset(1);
        }
        if (*pIdxInfo).nOrderBy > 0 as ::core::ffi::c_int
            && (*(*pIdxInfo).aOrderBy.offset(0 as ::core::ffi::c_int as isize)).iColumn
                < 0 as ::core::ffi::c_int
            && (*(*pIdxInfo).aOrderBy.offset(0 as ::core::ffi::c_int as isize)).desc
                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*pIdxInfo).orderByConsumed = 1 as ::core::ffi::c_int;
        }
        if unusableMask & !idxMask != 0 as ::core::ffi::c_int {
            return SQLITE_CONSTRAINT;
        }
        if aIdx[0 as ::core::ffi::c_int as usize] < 0 as ::core::ffi::c_int {
            (*pIdxInfo).idxNum = 0 as ::core::ffi::c_int;
        } else {
            (*pIdxInfo).estimatedCost = 1.0f64;
            i = aIdx[0 as ::core::ffi::c_int as usize];
            (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).argvIndex = 1
                as ::core::ffi::c_int;
            (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).omit = 1
                as ::core::ffi::c_uchar;
            if aIdx[1 as ::core::ffi::c_int as usize] < 0 as ::core::ffi::c_int {
                (*pIdxInfo).idxNum = 1 as ::core::ffi::c_int;
            } else {
                i = aIdx[1 as ::core::ffi::c_int as usize];
                (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).argvIndex = 2
                    as ::core::ffi::c_int;
                (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).omit = 1
                    as ::core::ffi::c_uchar;
                (*pIdxInfo).idxNum = 3 as ::core::ffi::c_int;
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn jsonEachFilter(
    mut cur: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
        let mut zRoot: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut i: u32_0 = 0;
        let mut n: u32_0 = 0;
        let mut sz: u32_0 = 0;
        jsonEachCursorReset(p);
        if idxNum == 0 as ::core::ffi::c_int {
            return SQLITE_OK;
        }
        memset(
            &raw mut (*p).sParse as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<JsonParse>() as size_t,
        );
        (*p).sParse.nJPRef = 1 as u32_0;
        (*p).sParse.db = (*p).db;
        if !(jsonArgIsJsonb(
            *argv.offset(0 as ::core::ffi::c_int as isize),
            &raw mut (*p).sParse,
        ) != 0)
        {
            (*p).sParse.zJson = sqlite3_value_text(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            ) as *mut ::core::ffi::c_char;
            (*p).sParse.nJson = sqlite3_value_bytes(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            if (*p).sParse.zJson.is_null() {
                (*p).iEnd = 0 as u32_0;
                (*p).i = (*p).iEnd;
                return SQLITE_OK;
            }
            if jsonConvertTextToBlob(
                &raw mut (*p).sParse,
                ::core::ptr::null_mut::<sqlite3_context>(),
            ) != 0
            {
                if (*p).sParse.oom != 0 {
                    return SQLITE_NOMEM;
                }
                sqlite3_free((*(*cur).pVtab).zErrMsg as *mut ::core::ffi::c_void);
                (*(*cur).pVtab).zErrMsg = sqlite3_mprintf(
                    b"malformed JSON\0".as_ptr() as *const ::core::ffi::c_char,
                );
                jsonEachCursorReset(p);
                return if !(*(*cur).pVtab).zErrMsg.is_null() {
                    SQLITE_ERROR
                } else {
                    SQLITE_NOMEM
                };
            }
        }
        if idxNum == 3 as ::core::ffi::c_int {
            zRoot = sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
            if zRoot.is_null() {
                return SQLITE_OK;
            }
            if *zRoot.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '$' as i32
            {
                sqlite3_free((*(*cur).pVtab).zErrMsg as *mut ::core::ffi::c_void);
                (*(*cur).pVtab).zErrMsg = jsonBadPathError(
                    ::core::ptr::null_mut::<sqlite3_context>(),
                    zRoot,
                );
                jsonEachCursorReset(p);
                return if !(*(*cur).pVtab).zErrMsg.is_null() {
                    SQLITE_ERROR
                } else {
                    SQLITE_NOMEM
                };
            }
            (*p).nRoot = sqlite3Strlen30(zRoot) as u32_0;
            if *zRoot.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*p).i = 0 as u32_0;
                i = (*p).i;
                (*p).eType = 0 as u8_0;
            } else {
                i = jsonLookupStep(
                    &raw mut (*p).sParse,
                    0 as u32_0,
                    zRoot.offset(1 as ::core::ffi::c_int as isize),
                    0 as u32_0,
                );
                if i >= JSON_LOOKUP_PATHERROR as u32_0 {
                    if i == JSON_LOOKUP_NOTFOUND as u32_0 {
                        (*p).i = 0 as u32_0;
                        (*p).eType = 0 as u8_0;
                        (*p).iEnd = 0 as u32_0;
                        return SQLITE_OK;
                    }
                    sqlite3_free((*(*cur).pVtab).zErrMsg as *mut ::core::ffi::c_void);
                    (*(*cur).pVtab).zErrMsg = jsonBadPathError(
                        ::core::ptr::null_mut::<sqlite3_context>(),
                        zRoot,
                    );
                    jsonEachCursorReset(p);
                    return if !(*(*cur).pVtab).zErrMsg.is_null() {
                        SQLITE_ERROR
                    } else {
                        SQLITE_NOMEM
                    };
                }
                if (*p).sParse.iLabel != 0 {
                    (*p).i = (*p).sParse.iLabel;
                    (*p).eType = JSONB_OBJECT as u8_0;
                } else {
                    (*p).i = i;
                    (*p).eType = JSONB_ARRAY as u8_0;
                }
            }
            jsonAppendRaw(&raw mut (*p).path, zRoot, (*p).nRoot);
        } else {
            (*p).i = 0 as u32_0;
            i = (*p).i;
            (*p).eType = 0 as u8_0;
            (*p).nRoot = 1 as u32_0;
            jsonAppendRaw(
                &raw mut (*p).path,
                b"$\0".as_ptr() as *const ::core::ffi::c_char,
                1 as u32_0,
            );
        }
        (*p).nParent = 0 as u32_0;
        n = jsonbPayloadSize(&raw mut (*p).sParse, i, &raw mut sz);
        (*p).iEnd = i.wrapping_add(n).wrapping_add(sz);
        if *(*p).sParse.aBlob.offset(i as isize) as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int >= JSONB_ARRAY && (*p).bRecursive == 0
        {
            (*p).i = i.wrapping_add(n);
            (*p).eType = (*(*p).sParse.aBlob.offset(i as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as u8_0;
            (*p).aParent = sqlite3DbMallocZero(
                (*p).db,
                ::core::mem::size_of::<JsonParent>() as u64_0,
            ) as *mut JsonParent;
            if (*p).aParent.is_null() {
                return SQLITE_NOMEM;
            }
            (*p).nParent = 1 as u32_0;
            (*p).nParentAlloc = 1 as u32_0;
            (*(*p).aParent.offset(0 as ::core::ffi::c_int as isize)).iKey = 0 as i64_0;
            (*(*p).aParent.offset(0 as ::core::ffi::c_int as isize)).iEnd = (*p).iEnd;
            (*(*p).aParent.offset(0 as ::core::ffi::c_int as isize)).iHead = (*p).i;
            (*(*p).aParent.offset(0 as ::core::ffi::c_int as isize)).iValue = i;
        }
        return SQLITE_OK;
    }
}
static mut jsonEachModule: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: None,
        xConnect: Some(
            jsonEachConnect
                as unsafe extern "C" fn(
                    *mut sqlite3,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const *const ::core::ffi::c_char,
                    *mut *mut sqlite3_vtab,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xBestIndex: Some(
            jsonEachBestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            jsonEachDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: None,
        xOpen: Some(
            jsonEachOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            jsonEachClose
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            jsonEachFilter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            jsonEachNext
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            jsonEachEof
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            jsonEachColumn
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            jsonEachRowid
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xUpdate: None,
        xBegin: None,
        xSync: None,
        xCommit: None,
        xRollback: None,
        xFindFunction: None,
        xRename: None,
        xSavepoint: None,
        xRelease: None,
        xRollbackTo: None,
        xShadowName: None,
        xIntegrity: None

    }
};
#[no_mangle]
pub unsafe extern "C" fn sqlite3RegisterJsonFunctions() {
    unsafe {
        static mut aJsonFunc: [FuncDef; 34] = unsafe {
            [
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonRemoveFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: (0 as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int * 0x8 as ::core::ffi::c_int)
                        as intptr_t as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonRemoveFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"jsonb\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 1 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonArrayFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_array\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 1 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: (0 as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int * 0x8 as ::core::ffi::c_int)
                        as intptr_t as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonArrayFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"jsonb_array\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonArrayLengthFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_array_length\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonArrayLengthFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_array_length\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonErrorFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_error_position\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonExtractFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_extract\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: (0 as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int * 0x8 as ::core::ffi::c_int)
                        as intptr_t as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonExtractFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"jsonb_extract\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: (0x1 as ::core::ffi::c_int
                        | 0 as ::core::ffi::c_int * 0x8 as ::core::ffi::c_int)
                        as intptr_t as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonExtractFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"->\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: (0x2 as ::core::ffi::c_int
                        | 0 as ::core::ffi::c_int * 0x8 as ::core::ffi::c_int)
                        as intptr_t as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonExtractFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"->>\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 1 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonSetFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_insert\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 1 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: (0 as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int * 0x8 as ::core::ffi::c_int)
                        as intptr_t as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonSetFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"jsonb_insert\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 1 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonObjectFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_object\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 1 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: (0 as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int * 0x8 as ::core::ffi::c_int)
                        as intptr_t as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonObjectFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"jsonb_object\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonPatchFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_patch\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: (0 as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int * 0x8 as ::core::ffi::c_int)
                        as intptr_t as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonPatchFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"jsonb_patch\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonPrettyFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_pretty\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonPrettyFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_pretty\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 1 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonQuoteFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_quote\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonRemoveFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_remove\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: (0 as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int * 0x8 as ::core::ffi::c_int)
                        as intptr_t as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonRemoveFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"jsonb_remove\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 1 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonReplaceFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_replace\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 1 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: (0 as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int * 0x8 as ::core::ffi::c_int)
                        as intptr_t as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonReplaceFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"jsonb_replace\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 1 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 1 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: (0x4 as ::core::ffi::c_int
                        | 0 as ::core::ffi::c_int * 0x8 as ::core::ffi::c_int)
                        as intptr_t as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonSetFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_set\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 1 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: (0x4 as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int * 0x8 as ::core::ffi::c_int)
                        as intptr_t as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonSetFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"jsonb_set\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonTypeFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_type\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonTypeFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_type\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonValidFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_valid\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_DETERMINISTIC
                        | SQLITE_FUNC_CONSTANT | SQLITE_UTF8
                        | 1 as ::core::ffi::c_int * SQLITE_FUNC_RUNONLY
                        | 0 as ::core::ffi::c_int * SQLITE_SUBTYPE
                        | 0 as ::core::ffi::c_int * SQLITE_RESULT_SUBTYPE) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonValidFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"json_valid\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x100000 as ::core::ffi::c_int
                        | 0x1000000 as ::core::ffi::c_int | 1 as ::core::ffi::c_int
                        | 0x800 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonArrayStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        jsonArrayFinal
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        jsonArrayValue
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        jsonGroupInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"json_group_array\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x100000 as ::core::ffi::c_int
                        | 0x1000000 as ::core::ffi::c_int | 1 as ::core::ffi::c_int
                        | 0x800 as ::core::ffi::c_int) as u32_0,
                    pUserData: 0x8 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonArrayStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        jsonArrayFinal
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        jsonArrayValue
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        jsonGroupInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"jsonb_group_array\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x100000 as ::core::ffi::c_int
                        | 0x1000000 as ::core::ffi::c_int | 1 as ::core::ffi::c_int
                        | 0x800 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonObjectStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        jsonObjectFinal
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        jsonObjectValue
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        jsonGroupInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"json_group_object\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8
                        | 0 as ::core::ffi::c_int * SQLITE_FUNC_NEEDCOLL
                        | 0x100000 as ::core::ffi::c_int
                        | 0x1000000 as ::core::ffi::c_int | 1 as ::core::ffi::c_int
                        | 0x800 as ::core::ffi::c_int) as u32_0,
                    pUserData: 0x8 as ::core::ffi::c_int as intptr_t
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        jsonObjectStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        jsonObjectFinal
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        jsonObjectValue
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        jsonGroupInverse
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: b"jsonb_group_object\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
            ]
        };
        sqlite3InsertBuiltinFuncs(
            &raw mut aJsonFunc as *mut FuncDef,
            (::core::mem::size_of::<[FuncDef; 34]>() as usize)
                .wrapping_div(::core::mem::size_of::<FuncDef>() as usize)
                as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3JsonVtabRegister(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
) -> *mut Module {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        static mut azModule: [*const ::core::ffi::c_char; 4] = [
            b"json_each\0".as_ptr() as *const ::core::ffi::c_char,
            b"json_tree\0".as_ptr() as *const ::core::ffi::c_char,
            b"jsonb_each\0".as_ptr() as *const ::core::ffi::c_char,
            b"jsonb_tree\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        i = 0 as ::core::ffi::c_uint;
        while (i as usize)
            < (::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>() as usize)
                .wrapping_div(
                    ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize,
                )
        {
            if sqlite3StrICmp(azModule[i as usize], zName) == 0 as ::core::ffi::c_int {
                return sqlite3VtabCreateModule(
                    db,
                    azModule[i as usize],
                    &raw mut jsonEachModule,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    None,
                );
            }
            i = i.wrapping_add(1);
        }
        return ::core::ptr::null_mut::<Module>();
    }
}
pub const SQLITE_FUNC_NEEDCOLL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_FUNC_CONSTANT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_FUNC_RUNONLY: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_BUILTIN: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
