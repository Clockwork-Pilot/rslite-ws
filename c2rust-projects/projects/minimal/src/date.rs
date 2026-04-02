use c2rust_bitfields::BitfieldStruct;
use ::libc;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type RenameToken;
    pub type Vdbe;
    pub type sqlite3_mutex;
    pub type sqlite3_pcache;
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_double(_: *mut sqlite3_context, _: ::core::ffi::c_double);
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
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
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3NotPureFunc(_: *mut sqlite3_context) -> ::core::ffi::c_int;
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3MutexAlloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    fn sqlite3InsertBuiltinFuncs(_: *mut FuncDef, _: ::core::ffi::c_int);
    fn sqlite3AtoF(
        z: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_double,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    static sqlite3UpperToLower: [::core::ffi::c_uchar; 0];
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3StrAccumInit(
        _: *mut StrAccum,
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ResultStrAccum(_: *mut sqlite3_context, _: *mut StrAccum);
    fn sqlite3StmtCurrentTime(_: *mut sqlite3_context) -> sqlite3_int64;
    fn localtime(__timer: *const time_t) -> *mut tm;
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
pub type u16_0 = ::core::ffi::c_ushort;
pub type u8_0 = ::core::ffi::c_uchar;
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
pub type u32_0 = ::core::ffi::c_uint;
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
pub type i16_0 = ::core::ffi::c_short;
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
pub type LogEst = ::core::ffi::c_short;
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
        ) -> ::core::ffi::c_int
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
        unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> ()
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
        ) -> ::core::ffi::c_int
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
        ) -> *const ::core::ffi::c_char
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
        ) -> ::core::ffi::c_int
    >

}
pub type sqlite3_filename = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_mem_methods {
    pub xMalloc: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *mut ::core::ffi::c_void,
    >,
    pub xFree: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub xRealloc: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub xSize: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub xRoundup: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xInit: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub xShutdown: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pAppData: *mut ::core::ffi::c_void

}
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_mutex_methods {
    pub xMutexInit: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub xMutexEnd: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub xMutexAlloc: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *mut sqlite3_mutex,
    >,
    pub xMutexFree: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    pub xMutexEnter: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    pub xMutexTry: Option<
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
    >,
    pub xMutexLeave: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    pub xMutexHeld: Option<
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
    >,
    pub xMutexNotheld: Option<
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int
    >

}
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
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_pcache_methods2 {
    pub iVersion: ::core::ffi::c_int,
    pub pArg: *mut ::core::ffi::c_void,
    pub xInit: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub xShutdown: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub xCreate: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> *mut sqlite3_pcache,
    >,
    pub xCachesize: Option<
        unsafe extern "C" fn(*mut sqlite3_pcache, ::core::ffi::c_int) -> (),
    >,
    pub xPagecount: Option<
        unsafe extern "C" fn(*mut sqlite3_pcache) -> ::core::ffi::c_int,
    >,
    pub xFetch: Option<
        unsafe extern "C" fn(
            *mut sqlite3_pcache,
            ::core::ffi::c_uint,
            ::core::ffi::c_int,
        ) -> *mut sqlite3_pcache_page,
    >,
    pub xUnpin: Option<
        unsafe extern "C" fn(
            *mut sqlite3_pcache,
            *mut sqlite3_pcache_page,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub xRekey: Option<
        unsafe extern "C" fn(
            *mut sqlite3_pcache,
            *mut sqlite3_pcache_page,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
        ) -> (),
    >,
    pub xTruncate: Option<
        unsafe extern "C" fn(*mut sqlite3_pcache, ::core::ffi::c_uint) -> (),
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> ()>,
    pub xShrink: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> ()>

}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type StrAccum = sqlite3_str;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sqlite3Config {
    pub bMemstat: ::core::ffi::c_int,
    pub bCoreMutex: u8_0,
    pub bFullMutex: u8_0,
    pub bOpenUri: u8_0,
    pub bUseCis: u8_0,
    pub bSmallMalloc: u8_0,
    pub bExtraSchemaChecks: u8_0,
    pub mxStrlen: ::core::ffi::c_int,
    pub neverCorrupt: ::core::ffi::c_int,
    pub szLookaside: ::core::ffi::c_int,
    pub nLookaside: ::core::ffi::c_int,
    pub nStmtSpill: ::core::ffi::c_int,
    pub m: sqlite3_mem_methods,
    pub mutex: sqlite3_mutex_methods,
    pub pcache2: sqlite3_pcache_methods2,
    pub pHeap: *mut ::core::ffi::c_void,
    pub nHeap: ::core::ffi::c_int,
    pub mnReq: ::core::ffi::c_int,
    pub mxReq: ::core::ffi::c_int,
    pub szMmap: sqlite3_int64,
    pub mxMmap: sqlite3_int64,
    pub pPage: *mut ::core::ffi::c_void,
    pub szPage: ::core::ffi::c_int,
    pub nPage: ::core::ffi::c_int,
    pub mxParserStack: ::core::ffi::c_int,
    pub sharedCacheEnabled: ::core::ffi::c_int,
    pub szPma: u32_0,
    pub isInit: ::core::ffi::c_int,
    pub inProgress: ::core::ffi::c_int,
    pub isMutexInit: ::core::ffi::c_int,
    pub isMallocInit: ::core::ffi::c_int,
    pub isPCacheInit: ::core::ffi::c_int,
    pub nRefInitMutex: ::core::ffi::c_int,
    pub pInitMutex: *mut sqlite3_mutex,
    pub xLog: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub pLogArg: *mut ::core::ffi::c_void,
    pub mxMemdbSize: sqlite3_int64,
    pub xTestCallback: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub bLocaltimeFault: ::core::ffi::c_int,
    pub xAltLocaltime: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub iOnceResetThreshold: ::core::ffi::c_int,
    pub szSorterRef: u32_0,
    pub iPrngSeed: ::core::ffi::c_uint

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct DateTime {
    pub iJD: sqlite3_int64,
    pub Y: ::core::ffi::c_int,
    pub M: ::core::ffi::c_int,
    pub D: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub m: ::core::ffi::c_int,
    pub tz: ::core::ffi::c_int,
    pub s: ::core::ffi::c_double,
    pub validJD: ::core::ffi::c_char,
    pub validYMD: ::core::ffi::c_char,
    pub validHMS: ::core::ffi::c_char,
    pub nFloor: ::core::ffi::c_char,
    #[bitfield(name = "rawS", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "isError", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "useSubsec", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "isUtc", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "isLocal", ty = "::core::ffi::c_uint", bits = "4..=4")]
    pub rawS_isError_useSubsec_isUtc_isLocal: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub nName: u8_0,
    pub zName: [::core::ffi::c_char; 7],
    pub rLimit: ::core::ffi::c_float,
    pub rXform: ::core::ffi::c_float

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LENGTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_STATIC_MAIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn getDigits(
    mut zDate: *const ::core::ffi::c_char,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    unsafe {
        static mut aMx: [u16_0; 6] = [
            12 as ::core::ffi::c_int as u16_0,
            14 as ::core::ffi::c_int as u16_0,
            24 as ::core::ffi::c_int as u16_0,
            31 as ::core::ffi::c_int as u16_0,
            59 as ::core::ffi::c_int as u16_0,
            14712 as ::core::ffi::c_int as u16_0,
        ];
        
        let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nextC: ::core::ffi::c_char = 0;
        let mut ap = c2rust_args;
        's_15: loop {
            let mut N: ::core::ffi::c_char = (*zFormat
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - '0' as i32) as ::core::ffi::c_char;
            let mut min: ::core::ffi::c_char = (*zFormat
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - '0' as i32) as ::core::ffi::c_char;
            let mut val: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut max: u16_0 = 0;
            max = aMx[(*zFormat.offset(2 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int - 'a' as i32) as usize];
            nextC = *zFormat.offset(3 as ::core::ffi::c_int as isize);
            val = 0 as ::core::ffi::c_int;
            loop {
                let c2rust_fresh1 = N;
                N = N - 1;
                if !(c2rust_fresh1 != 0) {
                    break;
                }
                if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(*zDate as ::core::ffi::c_uchar as isize)
                    as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
                {
                    break 's_15;
                }
                val = val * 10 as ::core::ffi::c_int + *zDate as ::core::ffi::c_int
                    - '0' as i32;
                zDate = zDate.offset(1);
            }
            if val < min as ::core::ffi::c_int || val > max as ::core::ffi::c_int
                || nextC as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    && nextC as ::core::ffi::c_int != *zDate as ::core::ffi::c_int
            {
                break;
            }
            *ap.arg::<*mut ::core::ffi::c_int>() = val;
            zDate = zDate.offset(1);
            cnt += 1;
            zFormat = zFormat.offset(4 as ::core::ffi::c_int as isize);
            if !(nextC != 0) {
                break;
            }
        }
        return cnt;
    }
}
unsafe extern "C" fn parseTimezone(
    mut zDate: *const ::core::ffi::c_char,
    mut p: *mut DateTime,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut sgn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nHr: ::core::ffi::c_int = 0;
        let mut nMn: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(*zDate as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & 0x1 as ::core::ffi::c_int != 0
        {
            zDate = zDate.offset(1);
        }
        (*p).tz = 0 as ::core::ffi::c_int;
        c = *zDate as ::core::ffi::c_int;
        if c == '-' as i32 {
            sgn = -1 as ::core::ffi::c_int;
            c2rust_current_block = 3512920355445576850;
        } else if c == '+' as i32 {
            sgn = 1 as ::core::ffi::c_int;
            c2rust_current_block = 3512920355445576850;
        } else {
            if c == 'Z' as i32 || c == 'z' as i32 {
                zDate = zDate.offset(1);
                (*p).set_isLocal(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*p).set_isUtc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else {
                return (c != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            }
            c2rust_current_block = 3772056902217916273;
        }
        match c2rust_current_block {
            3512920355445576850 => {
                zDate = zDate.offset(1);
                if getDigits(
                    zDate,
                    b"20b:20e\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut nHr,
                    &raw mut nMn,
                ) != 2 as ::core::ffi::c_int
                {
                    return 1 as ::core::ffi::c_int;
                }
                zDate = zDate.offset(5 as ::core::ffi::c_int as isize);
                (*p).tz = sgn * (nMn + nHr * 60 as ::core::ffi::c_int);
                if (*p).tz == 0 as ::core::ffi::c_int {
                    (*p).set_isLocal(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*p).set_isUtc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
            _ => {}
        }
        while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(*zDate as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & 0x1 as ::core::ffi::c_int != 0
        {
            zDate = zDate.offset(1);
        }
        return (*zDate as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn parseHhMmSs(
    mut zDate: *const ::core::ffi::c_char,
    mut p: *mut DateTime,
) -> ::core::ffi::c_int {
    unsafe {
        let mut h: ::core::ffi::c_int = 0;
        let mut m: ::core::ffi::c_int = 0;
        let mut s: ::core::ffi::c_int = 0;
        let mut ms: ::core::ffi::c_double = 0.0f64;
        if getDigits(
            zDate,
            b"20c:20e\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut h,
            &raw mut m,
        ) != 2 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        zDate = zDate.offset(5 as ::core::ffi::c_int as isize);
        if *zDate as ::core::ffi::c_int == ':' as i32 {
            zDate = zDate.offset(1);
            if getDigits(
                zDate,
                b"20e\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut s,
            ) != 1 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            zDate = zDate.offset(2 as ::core::ffi::c_int as isize);
            if *zDate as ::core::ffi::c_int == '.' as i32
                && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(
                        *zDate.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar as isize,
                    ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
            {
                let mut rScale: ::core::ffi::c_double = 1.0f64;
                zDate = zDate.offset(1);
                while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(*zDate as ::core::ffi::c_uchar as isize)
                    as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                {
                    ms = ms * 10.0f64
                        + *zDate as ::core::ffi::c_int as ::core::ffi::c_double
                        - '0' as i32 as ::core::ffi::c_double;
                    rScale *= 10.0f64;
                    zDate = zDate.offset(1);
                }
                ms /= rScale;
                if ms > 0.999f64 {
                    ms = 0.999f64;
                }
            }
        } else {
            s = 0 as ::core::ffi::c_int;
        }
        (*p).validJD = 0 as ::core::ffi::c_char;
        (*p).set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).validHMS = 1 as ::core::ffi::c_char;
        (*p).h = h;
        (*p).m = m;
        (*p).s = s as ::core::ffi::c_double + ms;
        if parseTimezone(zDate, p) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn datetimeError(mut p: *mut DateTime) {
    unsafe {
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<DateTime>() as size_t,
        );
        (*p).set_isError(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn computeJD(mut p: *mut DateTime) {
    unsafe {
        let mut Y: ::core::ffi::c_int = 0;
        let mut M: ::core::ffi::c_int = 0;
        let mut D: ::core::ffi::c_int = 0;
        let mut A: ::core::ffi::c_int = 0;
        let mut B: ::core::ffi::c_int = 0;
        let mut X1: ::core::ffi::c_int = 0;
        let mut X2: ::core::ffi::c_int = 0;
        if (*p).validJD != 0 {
            return;
        }
        if (*p).validYMD != 0 {
            Y = (*p).Y;
            M = (*p).M;
            D = (*p).D;
        } else {
            Y = 2000 as ::core::ffi::c_int;
            M = 1 as ::core::ffi::c_int;
            D = 1 as ::core::ffi::c_int;
        }
        if Y < -4713 as ::core::ffi::c_int || Y > 9999 as ::core::ffi::c_int
            || (*p).rawS() as ::core::ffi::c_int != 0
        {
            datetimeError(p);
            return;
        }
        if M <= 2 as ::core::ffi::c_int {
            Y -= 1;
            M += 12 as ::core::ffi::c_int;
        }
        A = (Y + 4800 as ::core::ffi::c_int) / 100 as ::core::ffi::c_int;
        B = 38 as ::core::ffi::c_int - A + A / 4 as ::core::ffi::c_int;
        X1 = 36525 as ::core::ffi::c_int * (Y + 4716 as ::core::ffi::c_int)
            / 100 as ::core::ffi::c_int;
        X2 = 306001 as ::core::ffi::c_int * (M + 1 as ::core::ffi::c_int)
            / 10000 as ::core::ffi::c_int;
        (*p).iJD = (((X1 + X2 + D + B) as ::core::ffi::c_double - 1524.5f64)
            * 86400000 as ::core::ffi::c_int as ::core::ffi::c_double) as sqlite3_int64;
        (*p).validJD = 1 as ::core::ffi::c_char;
        if (*p).validHMS != 0 {
            (*p).iJD
                += (((*p).h * 3600000 as ::core::ffi::c_int
                    + (*p).m * 60000 as ::core::ffi::c_int) as sqlite3_int64
                    + ((*p).s * 1000 as ::core::ffi::c_int as ::core::ffi::c_double
                        + 0.5f64) as sqlite3_int64) as ::core::ffi::c_longlong;
            if (*p).tz != 0 {
                (*p).iJD
                    -= ((*p).tz * 60000 as ::core::ffi::c_int)
                        as ::core::ffi::c_longlong;
                (*p).validYMD = 0 as ::core::ffi::c_char;
                (*p).validHMS = 0 as ::core::ffi::c_char;
                (*p).tz = 0 as ::core::ffi::c_int;
                (*p).set_isUtc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*p).set_isLocal(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
    }
}
unsafe extern "C" fn computeFloor(mut p: *mut DateTime) {
    unsafe {
        if (*p).D <= 28 as ::core::ffi::c_int {
            (*p).nFloor = 0 as ::core::ffi::c_char;
        } else if (1 as ::core::ffi::c_int) << (*p).M & 0x15aa as ::core::ffi::c_int != 0
        {
            (*p).nFloor = 0 as ::core::ffi::c_char;
        } else if (*p).M != 2 as ::core::ffi::c_int {
            (*p).nFloor = ((*p).D == 31 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_char;
        } else if (*p).Y % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || (*p).Y % 100 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (*p).Y % 400 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            (*p).nFloor = ((*p).D - 28 as ::core::ffi::c_int) as ::core::ffi::c_char;
        } else {
            (*p).nFloor = ((*p).D - 29 as ::core::ffi::c_int) as ::core::ffi::c_char;
        };
    }
}
unsafe extern "C" fn parseYyyyMmDd(
    mut zDate: *const ::core::ffi::c_char,
    mut p: *mut DateTime,
) -> ::core::ffi::c_int {
    unsafe {
        let mut Y: ::core::ffi::c_int = 0;
        let mut M: ::core::ffi::c_int = 0;
        let mut D: ::core::ffi::c_int = 0;
        let mut neg: ::core::ffi::c_int = 0;
        if *zDate.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32
        {
            zDate = zDate.offset(1);
            neg = 1 as ::core::ffi::c_int;
        } else {
            neg = 0 as ::core::ffi::c_int;
        }
        if getDigits(
            zDate,
            b"40f-21a-21d\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut Y,
            &raw mut M,
            &raw mut D,
        ) != 3 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        zDate = zDate.offset(10 as ::core::ffi::c_int as isize);
        while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(*zDate as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & 0x1 as ::core::ffi::c_int != 0
            || 'T' as i32 == *(zDate as *mut u8_0) as ::core::ffi::c_int
        {
            zDate = zDate.offset(1);
        }
        if !(parseHhMmSs(zDate, p) == 0 as ::core::ffi::c_int) {
            if *zDate as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*p).validHMS = 0 as ::core::ffi::c_char;
            } else {
                return 1 as ::core::ffi::c_int
            }
        }
        (*p).validJD = 0 as ::core::ffi::c_char;
        (*p).validYMD = 1 as ::core::ffi::c_char;
        (*p).Y = if neg != 0 { -Y } else { Y };
        (*p).M = M;
        (*p).D = D;
        computeFloor(p);
        if (*p).tz != 0 {
            computeJD(p);
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn setDateTimeToCurrent(
    mut context: *mut sqlite3_context,
    mut p: *mut DateTime,
) -> ::core::ffi::c_int {
    unsafe {
        (*p).iJD = sqlite3StmtCurrentTime(context);
        if (*p).iJD > 0 as ::core::ffi::c_longlong {
            (*p).validJD = 1 as ::core::ffi::c_char;
            (*p).set_isUtc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*p).set_isLocal(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            clearYMD_HMS_TZ(p);
            return 0 as ::core::ffi::c_int;
        } else {
            return 1 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn setRawDateNumber(
    mut p: *mut DateTime,
    mut r: ::core::ffi::c_double,
) {
    unsafe {
        (*p).s = r;
        (*p).set_rawS(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if r >= 0.0f64 && r < 5373484.5f64 {
            (*p).iJD = (r * 86400000.0f64 + 0.5f64) as sqlite3_int64;
            (*p).validJD = 1 as ::core::ffi::c_char;
        }
    }
}
unsafe extern "C" fn parseDateOrTime(
    mut context: *mut sqlite3_context,
    mut zDate: *const ::core::ffi::c_char,
    mut p: *mut DateTime,
) -> ::core::ffi::c_int {
    unsafe {
        let mut r: ::core::ffi::c_double = 0.;
        if parseYyyyMmDd(zDate, p) == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int
        } else if parseHhMmSs(zDate, p) == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int
        } else if sqlite3StrICmp(zDate, b"now\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int && sqlite3NotPureFunc(context) != 0
        {
            return setDateTimeToCurrent(context, p)
        } else if sqlite3AtoF(
            zDate,
            &raw mut r,
            sqlite3Strlen30(zDate),
            SQLITE_UTF8 as u8_0,
        ) > 0 as ::core::ffi::c_int
        {
            setRawDateNumber(p, r);
            return 0 as ::core::ffi::c_int;
        } else if (sqlite3StrICmp(
            zDate,
            b"subsec\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || sqlite3StrICmp(
                zDate,
                b"subsecond\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int) && sqlite3NotPureFunc(context) != 0
        {
            (*p).set_useSubsec(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            return setDateTimeToCurrent(context, p);
        }
        return 1 as ::core::ffi::c_int;
    }
}
pub const INT_464269060799999: ::core::ffi::c_longlong = (0x1a640 as ::core::ffi::c_int
    as ::core::ffi::c_longlong) << 32 as ::core::ffi::c_int
    | 0x1072fdff as ::core::ffi::c_longlong;
unsafe extern "C" fn validJulianDay(mut iJD: sqlite3_int64) -> ::core::ffi::c_int {
    unsafe {
        return (iJD >= 0 as ::core::ffi::c_longlong && iJD <= INT_464269060799999)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn computeYMD(mut p: *mut DateTime) {
    unsafe {
        let mut Z: ::core::ffi::c_int = 0;
        let mut alpha: ::core::ffi::c_int = 0;
        let mut A: ::core::ffi::c_int = 0;
        let mut B: ::core::ffi::c_int = 0;
        let mut C: ::core::ffi::c_int = 0;
        let mut D: ::core::ffi::c_int = 0;
        let mut E: ::core::ffi::c_int = 0;
        let mut X1: ::core::ffi::c_int = 0;
        if (*p).validYMD != 0 {
            return;
        }
        if (*p).validJD == 0 {
            (*p).Y = 2000 as ::core::ffi::c_int;
            (*p).M = 1 as ::core::ffi::c_int;
            (*p).D = 1 as ::core::ffi::c_int;
        } else if validJulianDay((*p).iJD) == 0 {
            datetimeError(p);
            return;
        } else {
            Z = (((*p).iJD as ::core::ffi::c_longlong
                + 43200000 as ::core::ffi::c_longlong)
                / 86400000 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            alpha = ((Z as ::core::ffi::c_double + 32044.75f64) / 36524.25f64)
                as ::core::ffi::c_int - 52 as ::core::ffi::c_int;
            A = Z + 1 as ::core::ffi::c_int + alpha
                - (alpha + 100 as ::core::ffi::c_int) / 4 as ::core::ffi::c_int
                + 25 as ::core::ffi::c_int;
            B = A + 1524 as ::core::ffi::c_int;
            C = ((B as ::core::ffi::c_double - 122.1f64) / 365.25f64)
                as ::core::ffi::c_int;
            D = 36525 as ::core::ffi::c_int * (C & 32767 as ::core::ffi::c_int)
                / 100 as ::core::ffi::c_int;
            E = ((B - D) as ::core::ffi::c_double / 30.6001f64) as ::core::ffi::c_int;
            X1 = (30.6001f64 * E as ::core::ffi::c_double) as ::core::ffi::c_int;
            (*p).D = B - D - X1;
            (*p).M = if E < 14 as ::core::ffi::c_int {
                E - 1 as ::core::ffi::c_int
            } else {
                E - 13 as ::core::ffi::c_int
            };
            (*p).Y = if (*p).M > 2 as ::core::ffi::c_int {
                C - 4716 as ::core::ffi::c_int
            } else {
                C - 4715 as ::core::ffi::c_int
            };
        }
        (*p).validYMD = 1 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn computeHMS(mut p: *mut DateTime) {
    unsafe {
        let mut day_ms: ::core::ffi::c_int = 0;
        let mut day_min: ::core::ffi::c_int = 0;
        if (*p).validHMS != 0 {
            return;
        }
        computeJD(p);
        day_ms = (((*p).iJD as ::core::ffi::c_longlong
            + 43200000 as ::core::ffi::c_longlong) % 86400000 as ::core::ffi::c_longlong)
            as ::core::ffi::c_int;
        (*p).s = (day_ms % 60000 as ::core::ffi::c_int) as ::core::ffi::c_double
            / 1000.0f64;
        day_min = day_ms / 60000 as ::core::ffi::c_int;
        (*p).m = day_min % 60 as ::core::ffi::c_int;
        (*p).h = day_min / 60 as ::core::ffi::c_int;
        (*p).set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).validHMS = 1 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn computeYMD_HMS(mut p: *mut DateTime) {
    unsafe {
        computeYMD(p);
        computeHMS(p);
    }
}
unsafe extern "C" fn clearYMD_HMS_TZ(mut p: *mut DateTime) {
    unsafe {
        (*p).validYMD = 0 as ::core::ffi::c_char;
        (*p).validHMS = 0 as ::core::ffi::c_char;
        (*p).tz = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn osLocaltime(
    mut t: *mut time_t,
    mut pTm: *mut tm,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pX: *mut tm = ::core::ptr::null_mut::<tm>();
        let mut mutex: *mut sqlite3_mutex = sqlite3MutexAlloc(SQLITE_MUTEX_STATIC_MAIN);
        sqlite3_mutex_enter(mutex);
        pX = localtime(t);
        if sqlite3Config.bLocaltimeFault != 0 {
            if sqlite3Config.xAltLocaltime.is_some()
                && 0 as ::core::ffi::c_int
                    == sqlite3Config
                        .xAltLocaltime
                        .expect(
                            "non-null function pointer",
                        )(
                        t as *const ::core::ffi::c_void,
                        pTm as *mut ::core::ffi::c_void,
                    )
            {
                pX = pTm;
            } else {
                pX = ::core::ptr::null_mut::<tm>();
            }
        }
        if !pX.is_null() {
            *pTm = *pX;
        }
        sqlite3_mutex_leave(mutex);
        rc = pX.is_null() as ::core::ffi::c_int;
        return rc;
    }
}
unsafe extern "C" fn toLocaltime(
    mut p: *mut DateTime,
    mut pCtx: *mut sqlite3_context,
) -> ::core::ffi::c_int {
    unsafe {
        let mut t: time_t = 0;
        let mut sLocal: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: ::core::ptr::null::<::core::ffi::c_char>()

        };
        let mut iYearDiff: ::core::ffi::c_int = 0;
        memset(
            &raw mut sLocal as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<tm>() as size_t,
        );
        computeJD(p);
        if (*p).iJD < 2108667600 as i64_0 * 100000 as ::core::ffi::c_int as i64_0
            || (*p).iJD > 2130141456 as i64_0 * 100000 as ::core::ffi::c_int as i64_0
        {
            let mut x: DateTime = *p;
            computeYMD_HMS(&raw mut x);
            iYearDiff = 2000 as ::core::ffi::c_int + x.Y % 4 as ::core::ffi::c_int - x.Y;
            x.Y += iYearDiff;
            x.validJD = 0 as ::core::ffi::c_char;
            computeJD(&raw mut x);
            t = (x.iJD as i64_0 / 1000 as i64_0
                - 21086676 as i64_0 * 10000 as ::core::ffi::c_int as i64_0) as time_t;
        } else {
            iYearDiff = 0 as ::core::ffi::c_int;
            t = ((*p).iJD as i64_0 / 1000 as i64_0
                - 21086676 as i64_0 * 10000 as ::core::ffi::c_int as i64_0) as time_t;
        }
        if osLocaltime(&raw mut t, &raw mut sLocal) != 0 {
            sqlite3_result_error(
                pCtx,
                b"local time unavailable\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            );
            return SQLITE_ERROR;
        }
        (*p).Y = sLocal.tm_year + 1900 as ::core::ffi::c_int - iYearDiff;
        (*p).M = sLocal.tm_mon + 1 as ::core::ffi::c_int;
        (*p).D = sLocal.tm_mday;
        (*p).h = sLocal.tm_hour;
        (*p).m = sLocal.tm_min;
        (*p).s = sLocal.tm_sec as ::core::ffi::c_double
            + ((*p).iJD as ::core::ffi::c_longlong % 1000 as ::core::ffi::c_longlong)
                as ::core::ffi::c_double * 0.001f64;
        (*p).validYMD = 1 as ::core::ffi::c_char;
        (*p).validHMS = 1 as ::core::ffi::c_char;
        (*p).validJD = 0 as ::core::ffi::c_char;
        (*p).set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).tz = 0 as ::core::ffi::c_int;
        (*p).set_isError(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        return SQLITE_OK;
    }
}
static mut aXformType: [C2Rust_Unnamed_22; 6] = unsafe {
    [
        C2Rust_Unnamed_22 {
            nName: 6 as u8_0,
            zName: ::core::mem::transmute::<
                [u8; 7],
                [::core::ffi::c_char; 7],
            >(*b"second\0"),
            rLimit: 4.6427e+14f32,
            rXform: 1.0f32

        },
        C2Rust_Unnamed_22 {
            nName: 6 as u8_0,
            zName: ::core::mem::transmute::<
                [u8; 7],
                [::core::ffi::c_char; 7],
            >(*b"minute\0"),
            rLimit: 7.7379e+12f32,
            rXform: 60.0f32

        },
        C2Rust_Unnamed_22 {
            nName: 4 as u8_0,
            zName: ::core::mem::transmute::<
                [u8; 7],
                [::core::ffi::c_char; 7],
            >(*b"hour\0\0\0"),
            rLimit: 1.2897e+11f32,
            rXform: 3600.0f32

        },
        C2Rust_Unnamed_22 {
            nName: 3 as u8_0,
            zName: ::core::mem::transmute::<
                [u8; 7],
                [::core::ffi::c_char; 7],
            >(*b"day\0\0\0\0"),
            rLimit: 5373485.0f32,
            rXform: 86400.0f32

        },
        C2Rust_Unnamed_22 {
            nName: 5 as u8_0,
            zName: ::core::mem::transmute::<
                [u8; 7],
                [::core::ffi::c_char; 7],
            >(*b"month\0\0"),
            rLimit: 176546.0f32,
            rXform: 2592000.0f32

        },
        C2Rust_Unnamed_22 {
            nName: 4 as u8_0,
            zName: ::core::mem::transmute::<
                [u8; 7],
                [::core::ffi::c_char; 7],
            >(*b"year\0\0\0"),
            rLimit: 14713.0f32,
            rXform: 31536000.0f32

        },
    ]
};
unsafe extern "C" fn autoAdjustDate(mut p: *mut DateTime) {
    unsafe {
        if (*p).rawS() == 0 || (*p).validJD as ::core::ffi::c_int != 0 {
            (*p).set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else if (*p).s
            >= (-21086676 as ::core::ffi::c_int as i64_0
                * 10000 as ::core::ffi::c_int as i64_0) as ::core::ffi::c_double
            && (*p).s
                <= (25340230 as ::core::ffi::c_longlong
                    * 10000 as ::core::ffi::c_int as ::core::ffi::c_longlong
                    + 799 as ::core::ffi::c_longlong) as ::core::ffi::c_double
        {
            let mut r: ::core::ffi::c_double = (*p).s * 1000.0f64 + 210866760000000.0f64;
            clearYMD_HMS_TZ(p);
            (*p).iJD = (r + 0.5f64) as sqlite3_int64;
            (*p).validJD = 1 as ::core::ffi::c_char;
            (*p).set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
}
unsafe extern "C" fn parseModifier(
    mut pCtx: *mut sqlite3_context,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut p: *mut DateTime,
    mut idx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut r: ::core::ffi::c_double = 0.;
        let mut c2rust_current_block_175: u64;
        match *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
            .offset(*z.offset(0 as ::core::ffi::c_int as isize) as u8_0 as isize)
            as ::core::ffi::c_int
        {
            97 => {
                if sqlite3_stricmp(z, b"auto\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if idx > 1 as ::core::ffi::c_int {
                        return 1 as ::core::ffi::c_int;
                    }
                    autoAdjustDate(p);
                    rc = 0 as ::core::ffi::c_int;
                }
            }
            99 => {
                if sqlite3_stricmp(
                    z,
                    b"ceiling\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    computeJD(p);
                    clearYMD_HMS_TZ(p);
                    rc = 0 as ::core::ffi::c_int;
                    (*p).nFloor = 0 as ::core::ffi::c_char;
                }
            }
            102 => {
                if sqlite3_stricmp(z, b"floor\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    computeJD(p);
                    (*p).iJD
                        -= ((*p).nFloor as ::core::ffi::c_int
                            * 86400000 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
                    clearYMD_HMS_TZ(p);
                    rc = 0 as ::core::ffi::c_int;
                }
            }
            106 => {
                if sqlite3_stricmp(
                    z,
                    b"julianday\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if idx > 1 as ::core::ffi::c_int {
                        return 1 as ::core::ffi::c_int;
                    }
                    if (*p).validJD as ::core::ffi::c_int != 0
                        && (*p).rawS() as ::core::ffi::c_int != 0
                    {
                        rc = 0 as ::core::ffi::c_int;
                        (*p).set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                }
            }
            108 => {
                if sqlite3_stricmp(
                    z,
                    b"localtime\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int && sqlite3NotPureFunc(pCtx) != 0
                {
                    rc = if (*p).isLocal() as ::core::ffi::c_int != 0 {
                        SQLITE_OK
                    } else {
                        toLocaltime(p, pCtx)
                    };
                    (*p).set_isUtc(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*p).set_isLocal(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
            117 => {
                if sqlite3_stricmp(
                    z,
                    b"unixepoch\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int && (*p).rawS() as ::core::ffi::c_int != 0
                {
                    if idx > 1 as ::core::ffi::c_int {
                        return 1 as ::core::ffi::c_int;
                    }
                    r = (*p).s * 1000.0f64 + 210866760000000.0f64;
                    if r >= 0.0f64 && r < 464269060800000.0f64 {
                        clearYMD_HMS_TZ(p);
                        (*p).iJD = (r + 0.5f64) as sqlite3_int64;
                        (*p).validJD = 1 as ::core::ffi::c_char;
                        (*p).set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        rc = 0 as ::core::ffi::c_int;
                    }
                } else if sqlite3_stricmp(
                    z,
                    b"utc\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int && sqlite3NotPureFunc(pCtx) != 0
                {
                    if (*p).isUtc() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        let mut iOrigJD: i64_0 = 0;
                        let mut iGuess: i64_0 = 0;
                        let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut iErr: i64_0 = 0;
                        computeJD(p);
                        iOrigJD = (*p).iJD as i64_0;
                        iGuess = iOrigJD;
                        iErr = 0 as i64_0;
                        loop {
                            let mut new: DateTime = DateTime {
                                iJD: 0,
                                Y: 0,
                                M: 0,
                                D: 0,
                                h: 0,
                                m: 0,
                                tz: 0,
                                s: 0.,
                                validJD: 0,
                                validYMD: 0,
                                validHMS: 0,
                                nFloor: 0,
                                rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
                                c2rust_padding: [0; 3]

                            };
                            memset(
                                &raw mut new as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<DateTime>() as size_t,
                            );
                            iGuess -= iErr as ::core::ffi::c_longlong;
                            new.iJD = iGuess as sqlite3_int64;
                            new.validJD = 1 as ::core::ffi::c_char;
                            rc = toLocaltime(&raw mut new, pCtx);
                            if rc != 0 {
                                return rc;
                            }
                            computeJD(&raw mut new);
                            iErr = new.iJD as i64_0 - iOrigJD;
                            if !(iErr != 0
                                && {
                                    let c2rust_fresh0 = cnt;
                                    cnt = cnt + 1;
                                    c2rust_fresh0 < 3 as ::core::ffi::c_int
                                })
                            {
                                break;
                            }
                        }
                        memset(
                            p as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<DateTime>() as size_t,
                        );
                        (*p).iJD = iGuess as sqlite3_int64;
                        (*p).validJD = 1 as ::core::ffi::c_char;
                        (*p).set_isUtc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*p)
                            .set_isLocal(
                                0 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                    }
                    rc = SQLITE_OK;
                }
            }
            119 => {
                if sqlite3_strnicmp(
                    z,
                    b"weekday \0".as_ptr() as *const ::core::ffi::c_char,
                    8 as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
                    && sqlite3AtoF(
                        z.offset(8 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_char,
                        &raw mut r,
                        sqlite3Strlen30(
                            z.offset(8 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_char,
                        ),
                        SQLITE_UTF8 as u8_0,
                    ) > 0 as ::core::ffi::c_int && r >= 0.0f64 && r < 7.0f64
                    && {
                        n = r as ::core::ffi::c_int;
                        n as ::core::ffi::c_double == r
                    }
                {
                    let mut Z: sqlite3_int64 = 0;
                    computeYMD_HMS(p);
                    (*p).tz = 0 as ::core::ffi::c_int;
                    (*p).validJD = 0 as ::core::ffi::c_char;
                    computeJD(p);
                    Z = (((*p).iJD as ::core::ffi::c_longlong
                        + 129600000 as ::core::ffi::c_longlong)
                        / 86400000 as ::core::ffi::c_longlong
                        % 7 as ::core::ffi::c_longlong) as sqlite3_int64;
                    if Z > n as ::core::ffi::c_longlong {
                        Z -= 7 as ::core::ffi::c_longlong;
                    }
                    (*p).iJD
                        += (n as ::core::ffi::c_longlong - Z as ::core::ffi::c_longlong)
                            * 86400000 as ::core::ffi::c_longlong;
                    clearYMD_HMS_TZ(p);
                    rc = 0 as ::core::ffi::c_int;
                }
            }
            115 => {
                if sqlite3_strnicmp(
                    z,
                    b"start of \0".as_ptr() as *const ::core::ffi::c_char,
                    9 as ::core::ffi::c_int,
                ) != 0 as ::core::ffi::c_int
                {
                    if sqlite3_stricmp(
                        z,
                        b"subsec\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                        || sqlite3_stricmp(
                            z,
                            b"subsecond\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        (*p)
                            .set_useSubsec(
                                1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                        rc = 0 as ::core::ffi::c_int;
                    }
                } else if !((*p).validJD == 0 && (*p).validYMD == 0
                    && (*p).validHMS == 0)
                {
                    z = z.offset(9 as ::core::ffi::c_int as isize);
                    computeYMD(p);
                    (*p).validHMS = 1 as ::core::ffi::c_char;
                    (*p).m = 0 as ::core::ffi::c_int;
                    (*p).h = (*p).m;
                    (*p).s = 0.0f64;
                    (*p).set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*p).tz = 0 as ::core::ffi::c_int;
                    (*p).validJD = 0 as ::core::ffi::c_char;
                    if sqlite3_stricmp(
                        z,
                        b"month\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*p).D = 1 as ::core::ffi::c_int;
                        rc = 0 as ::core::ffi::c_int;
                    } else if sqlite3_stricmp(
                        z,
                        b"year\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*p).M = 1 as ::core::ffi::c_int;
                        (*p).D = 1 as ::core::ffi::c_int;
                        rc = 0 as ::core::ffi::c_int;
                    } else if sqlite3_stricmp(
                        z,
                        b"day\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        rc = 0 as ::core::ffi::c_int;
                    }
                }
            }
            43 | 45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                let mut rRounder: ::core::ffi::c_double = 0.;
                let mut i: ::core::ffi::c_int = 0;
                let mut Y: ::core::ffi::c_int = 0;
                let mut M: ::core::ffi::c_int = 0;
                let mut D: ::core::ffi::c_int = 0;
                let mut h: ::core::ffi::c_int = 0;
                let mut m: ::core::ffi::c_int = 0;
                let mut x: ::core::ffi::c_int = 0;
                let mut z2: *const ::core::ffi::c_char = z;
                let mut z0: ::core::ffi::c_char = *z
                    .offset(0 as ::core::ffi::c_int as isize);
                n = 1 as ::core::ffi::c_int;
                while *z.offset(n as isize) != 0 {
                    if *z.offset(n as isize) as ::core::ffi::c_int == ':' as i32 {
                        break;
                    }
                    if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(*z.offset(n as isize) as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
                    {
                        break;
                    }
                    if *z.offset(n as isize) as ::core::ffi::c_int == '-' as i32 {
                        if n == 5 as ::core::ffi::c_int
                            && getDigits(
                                z.offset(1 as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_char,
                                b"40f\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut Y,
                            ) == 1 as ::core::ffi::c_int
                        {
                            break;
                        }
                        if n == 6 as ::core::ffi::c_int
                            && getDigits(
                                z.offset(1 as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_char,
                                b"50f\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut Y,
                            ) == 1 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    n += 1;
                }
                if !(sqlite3AtoF(z, &raw mut r, n, SQLITE_UTF8 as u8_0)
                    <= 0 as ::core::ffi::c_int)
                {
                    if *z.offset(n as isize) as ::core::ffi::c_int == '-' as i32 {
                        if z0 as ::core::ffi::c_int != '+' as i32
                            && z0 as ::core::ffi::c_int != '-' as i32
                        {
                            c2rust_current_block_175 = 2413388577390654262;
                        } else {
                            if n == 5 as ::core::ffi::c_int {
                                if getDigits(
                                    z.offset(1 as ::core::ffi::c_int as isize)
                                        as *const ::core::ffi::c_char,
                                    b"40f-20a-20d\0".as_ptr() as *const ::core::ffi::c_char,
                                    &raw mut Y,
                                    &raw mut M,
                                    &raw mut D,
                                ) != 3 as ::core::ffi::c_int
                                {
                                    c2rust_current_block_175 = 2413388577390654262;
                                } else {
                                    c2rust_current_block_175 = 3024367268842933116;
                                }
                            } else if getDigits(
                                z.offset(1 as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_char,
                                b"50f-20a-20d\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut Y,
                                &raw mut M,
                                &raw mut D,
                            ) != 3 as ::core::ffi::c_int
                            {
                                c2rust_current_block_175 = 2413388577390654262;
                            } else {
                                z = z.offset(1);
                                c2rust_current_block_175 = 3024367268842933116;
                            }
                            match c2rust_current_block_175 {
                                2413388577390654262 => {}
                                _ => {
                                    if M >= 12 as ::core::ffi::c_int {
                                        c2rust_current_block_175 = 2413388577390654262;
                                    } else if D >= 31 as ::core::ffi::c_int {
                                        c2rust_current_block_175 = 2413388577390654262;
                                    } else {
                                        computeYMD_HMS(p);
                                        (*p).validJD = 0 as ::core::ffi::c_char;
                                        if z0 as ::core::ffi::c_int == '-' as i32 {
                                            (*p).Y -= Y;
                                            (*p).M -= M;
                                            D = -D;
                                        } else {
                                            (*p).Y += Y;
                                            (*p).M += M;
                                        }
                                        x = if (*p).M > 0 as ::core::ffi::c_int {
                                            ((*p).M - 1 as ::core::ffi::c_int)
                                                / 12 as ::core::ffi::c_int
                                        } else {
                                            ((*p).M - 12 as ::core::ffi::c_int)
                                                / 12 as ::core::ffi::c_int
                                        };
                                        (*p).Y += x;
                                        (*p).M -= x * 12 as ::core::ffi::c_int;
                                        computeFloor(p);
                                        computeJD(p);
                                        (*p).validHMS = 0 as ::core::ffi::c_char;
                                        (*p).validYMD = 0 as ::core::ffi::c_char;
                                        (*p).iJD
                                            += D as ::core::ffi::c_longlong
                                                * 86400000 as ::core::ffi::c_longlong;
                                        if *z.offset(11 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                        {
                                            rc = 0 as ::core::ffi::c_int;
                                            c2rust_current_block_175 = 2413388577390654262;
                                        } else if *(&raw const sqlite3CtypeMap
                                            as *const ::core::ffi::c_uchar)
                                            .offset(
                                                *z.offset(11 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_uchar as isize,
                                            ) as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
                                            && getDigits(
                                                z.offset(12 as ::core::ffi::c_int as isize)
                                                    as *const ::core::ffi::c_char,
                                                b"20c:20e\0".as_ptr() as *const ::core::ffi::c_char,
                                                &raw mut h,
                                                &raw mut m,
                                            ) == 2 as ::core::ffi::c_int
                                        {
                                            z2 = z.offset(12 as ::core::ffi::c_int as isize)
                                                as *const ::core::ffi::c_char;
                                            n = 2 as ::core::ffi::c_int;
                                            c2rust_current_block_175 = 6897179874198677617;
                                        } else {
                                            c2rust_current_block_175 = 2413388577390654262;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        c2rust_current_block_175 = 6897179874198677617;
                    }
                    match c2rust_current_block_175 {
                        2413388577390654262 => {}
                        _ => {
                            if *z2.offset(n as isize) as ::core::ffi::c_int == ':' as i32
                            {
                                let mut tx: DateTime = DateTime {
                                    iJD: 0,
                                    Y: 0,
                                    M: 0,
                                    D: 0,
                                    h: 0,
                                    m: 0,
                                    tz: 0,
                                    s: 0.,
                                    validJD: 0,
                                    validYMD: 0,
                                    validHMS: 0,
                                    nFloor: 0,
                                    rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
                                    c2rust_padding: [0; 3]

                                };
                                let mut day: sqlite3_int64 = 0;
                                if *(&raw const sqlite3CtypeMap
                                    as *const ::core::ffi::c_uchar)
                                    .offset(*z2 as ::core::ffi::c_uchar as isize)
                                    as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
                                {
                                    z2 = z2.offset(1);
                                }
                                memset(
                                    &raw mut tx as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    ::core::mem::size_of::<DateTime>() as size_t,
                                );
                                if !(parseHhMmSs(z2, &raw mut tx) != 0) {
                                    computeJD(&raw mut tx);
                                    tx.iJD -= 43200000 as ::core::ffi::c_longlong;
                                    day = (tx.iJD as ::core::ffi::c_longlong
                                        / 86400000 as ::core::ffi::c_longlong) as sqlite3_int64;
                                    tx.iJD
                                        -= day as ::core::ffi::c_longlong
                                            * 86400000 as ::core::ffi::c_longlong;
                                    if z0 as ::core::ffi::c_int == '-' as i32 {
                                        tx.iJD = -tx.iJD;
                                    }
                                    computeJD(p);
                                    clearYMD_HMS_TZ(p);
                                    (*p).iJD += tx.iJD as ::core::ffi::c_longlong;
                                    rc = 0 as ::core::ffi::c_int;
                                }
                            } else {
                                z = z.offset(n as isize);
                                while *(&raw const sqlite3CtypeMap
                                    as *const ::core::ffi::c_uchar)
                                    .offset(*z as ::core::ffi::c_uchar as isize)
                                    as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
                                {
                                    z = z.offset(1);
                                }
                                n = sqlite3Strlen30(z);
                                if !(n < 3 as ::core::ffi::c_int
                                    || n > 10 as ::core::ffi::c_int)
                                {
                                    if *(&raw const sqlite3UpperToLower
                                        as *const ::core::ffi::c_uchar)
                                        .offset(
                                            *z.offset((n - 1 as ::core::ffi::c_int) as isize) as u8_0
                                                as isize,
                                        ) as ::core::ffi::c_int == 's' as i32
                                    {
                                        n -= 1;
                                    }
                                    computeJD(p);
                                    rRounder = if r
                                        < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        -0.5f64
                                    } else {
                                        0.5f64
                                    };
                                    (*p).nFloor = 0 as ::core::ffi::c_char;
                                    i = 0 as ::core::ffi::c_int;
                                    while i
                                        < (::core::mem::size_of::<[C2Rust_Unnamed_22; 6]>()
                                            as usize)
                                            .wrapping_div(
                                                ::core::mem::size_of::<C2Rust_Unnamed_22>() as usize,
                                            ) as ::core::ffi::c_int
                                    {
                                        if aXformType[i as usize].nName as ::core::ffi::c_int == n
                                            && sqlite3_strnicmp(
                                                &raw const (*(&raw const aXformType
                                                    as *const C2Rust_Unnamed_22)
                                                    .offset(i as isize))
                                                    .zName as *const ::core::ffi::c_char,
                                                z,
                                                n,
                                            ) == 0 as ::core::ffi::c_int
                                            && r
                                                > -aXformType[i as usize].rLimit as ::core::ffi::c_double
                                            && r
                                                < aXformType[i as usize].rLimit as ::core::ffi::c_double
                                        {
                                            match i {
                                                4 => {
                                                    computeYMD_HMS(p);
                                                    (*p).M += r as ::core::ffi::c_int;
                                                    x = if (*p).M > 0 as ::core::ffi::c_int {
                                                        ((*p).M - 1 as ::core::ffi::c_int)
                                                            / 12 as ::core::ffi::c_int
                                                    } else {
                                                        ((*p).M - 12 as ::core::ffi::c_int)
                                                            / 12 as ::core::ffi::c_int
                                                    };
                                                    (*p).Y += x;
                                                    (*p).M -= x * 12 as ::core::ffi::c_int;
                                                    computeFloor(p);
                                                    (*p).validJD = 0 as ::core::ffi::c_char;
                                                    r -= r as ::core::ffi::c_int as ::core::ffi::c_double;
                                                }
                                                5 => {
                                                    let mut y: ::core::ffi::c_int = r as ::core::ffi::c_int;
                                                    computeYMD_HMS(p);
                                                    (*p).Y += y;
                                                    computeFloor(p);
                                                    (*p).validJD = 0 as ::core::ffi::c_char;
                                                    r -= r as ::core::ffi::c_int as ::core::ffi::c_double;
                                                }
                                                _ => {}
                                            }
                                            computeJD(p);
                                            (*p).iJD
                                                += (r * 1000.0f64
                                                    * aXformType[i as usize].rXform as ::core::ffi::c_double
                                                    + rRounder) as sqlite3_int64 as ::core::ffi::c_longlong;
                                            rc = 0 as ::core::ffi::c_int;
                                            break;
                                        } else {
                                            i += 1;
                                        }
                                    }
                                    clearYMD_HMS_TZ(p);
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        return rc;
    }
}
unsafe extern "C" fn isDate(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
    mut p: *mut DateTime,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut z: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut eType: ::core::ffi::c_int = 0;
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<DateTime>() as size_t,
        );
        if argc == 0 as ::core::ffi::c_int {
            if sqlite3NotPureFunc(context) == 0 {
                return 1 as ::core::ffi::c_int;
            }
            return setDateTimeToCurrent(context, p);
        }
        eType = sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize));
        if eType == SQLITE_FLOAT || eType == SQLITE_INTEGER {
            setRawDateNumber(
                p,
                sqlite3_value_double(*argv.offset(0 as ::core::ffi::c_int as isize)),
            );
        } else {
            z = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
            if z.is_null()
                || parseDateOrTime(context, z as *mut ::core::ffi::c_char, p) != 0
            {
                return 1 as ::core::ffi::c_int;
            }
        }
        i = 1 as ::core::ffi::c_int;
        while i < argc {
            z = sqlite3_value_text(*argv.offset(i as isize));
            n = sqlite3_value_bytes(*argv.offset(i as isize));
            if z.is_null()
                || parseModifier(context, z as *mut ::core::ffi::c_char, n, p, i) != 0
            {
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        computeJD(p);
        if (*p).isError() as ::core::ffi::c_int != 0 || validJulianDay((*p).iJD) == 0 {
            return 1 as ::core::ffi::c_int;
        }
        if argc == 1 as ::core::ffi::c_int && (*p).validYMD as ::core::ffi::c_int != 0
            && (*p).D > 28 as ::core::ffi::c_int
        {
            (*p).validYMD = 0 as ::core::ffi::c_char;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn juliandayFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut x: DateTime = DateTime {
            iJD: 0,
            Y: 0,
            M: 0,
            D: 0,
            h: 0,
            m: 0,
            tz: 0,
            s: 0.,
            validJD: 0,
            validYMD: 0,
            validHMS: 0,
            nFloor: 0,
            rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
            c2rust_padding: [0; 3]

        };
        if isDate(context, argc, argv, &raw mut x) == 0 as ::core::ffi::c_int {
            computeJD(&raw mut x);
            sqlite3_result_double(
                context,
                x.iJD as ::core::ffi::c_double / 86400000.0f64,
            );
        }
    }
}
unsafe extern "C" fn unixepochFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut x: DateTime = DateTime {
            iJD: 0,
            Y: 0,
            M: 0,
            D: 0,
            h: 0,
            m: 0,
            tz: 0,
            s: 0.,
            validJD: 0,
            validYMD: 0,
            validHMS: 0,
            nFloor: 0,
            rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
            c2rust_padding: [0; 3]

        };
        if isDate(context, argc, argv, &raw mut x) == 0 as ::core::ffi::c_int {
            computeJD(&raw mut x);
            if x.useSubsec() != 0 {
                sqlite3_result_double(
                    context,
                    (x.iJD as i64_0
                        - 21086676 as i64_0 * 10000000 as ::core::ffi::c_int as i64_0)
                        as ::core::ffi::c_double / 1000.0f64,
                );
            } else {
                sqlite3_result_int64(
                    context,
                    x.iJD / 1000 as sqlite3_int64
                        - 21086676 as sqlite3_int64
                            * 10000 as ::core::ffi::c_int as sqlite3_int64,
                );
            }
        }
    }
}
unsafe extern "C" fn datetimeFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut x: DateTime = DateTime {
            iJD: 0,
            Y: 0,
            M: 0,
            D: 0,
            h: 0,
            m: 0,
            tz: 0,
            s: 0.,
            validJD: 0,
            validYMD: 0,
            validHMS: 0,
            nFloor: 0,
            rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
            c2rust_padding: [0; 3]

        };
        if isDate(context, argc, argv, &raw mut x) == 0 as ::core::ffi::c_int {
            let mut Y: ::core::ffi::c_int = 0;
            let mut s: ::core::ffi::c_int = 0;
            let mut n: ::core::ffi::c_int = 0;
            let mut zBuf: [::core::ffi::c_char; 32] = [0; 32];
            computeYMD_HMS(&raw mut x);
            Y = x.Y;
            if Y < 0 as ::core::ffi::c_int {
                Y = -Y;
            }
            zBuf[1 as ::core::ffi::c_int as usize] = ('0' as i32
                + Y / 1000 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[2 as ::core::ffi::c_int as usize] = ('0' as i32
                + Y / 100 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[3 as ::core::ffi::c_int as usize] = ('0' as i32
                + Y / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[4 as ::core::ffi::c_int as usize] = ('0' as i32
                + Y % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[5 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
            zBuf[6 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.M / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[7 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.M % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[8 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
            zBuf[9 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.D / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[10 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.D % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[11 as ::core::ffi::c_int as usize] = ' ' as i32 as ::core::ffi::c_char;
            zBuf[12 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.h / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[13 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.h % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[14 as ::core::ffi::c_int as usize] = ':' as i32 as ::core::ffi::c_char;
            zBuf[15 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.m / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[16 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.m % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[17 as ::core::ffi::c_int as usize] = ':' as i32 as ::core::ffi::c_char;
            if x.useSubsec() != 0 {
                s = (1000.0f64 * x.s + 0.5f64) as ::core::ffi::c_int;
                zBuf[18 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s / 10000 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                zBuf[19 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s / 1000 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                zBuf[20 as ::core::ffi::c_int as usize] = '.' as i32
                    as ::core::ffi::c_char;
                zBuf[21 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s / 100 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                zBuf[22 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                zBuf[23 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
                zBuf[24 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
                n = 24 as ::core::ffi::c_int;
            } else {
                s = x.s as ::core::ffi::c_int;
                zBuf[18 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                zBuf[19 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
                zBuf[20 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
                n = 20 as ::core::ffi::c_int;
            }
            if x.Y < 0 as ::core::ffi::c_int {
                zBuf[0 as ::core::ffi::c_int as usize] = '-' as i32
                    as ::core::ffi::c_char;
                sqlite3_result_text(
                    context,
                    &raw mut zBuf as *mut ::core::ffi::c_char,
                    n,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            } else {
                sqlite3_result_text(
                    context,
                    (&raw mut zBuf as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    n - 1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            }
        }
    }
}
unsafe extern "C" fn timeFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut x: DateTime = DateTime {
            iJD: 0,
            Y: 0,
            M: 0,
            D: 0,
            h: 0,
            m: 0,
            tz: 0,
            s: 0.,
            validJD: 0,
            validYMD: 0,
            validHMS: 0,
            nFloor: 0,
            rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
            c2rust_padding: [0; 3]

        };
        if isDate(context, argc, argv, &raw mut x) == 0 as ::core::ffi::c_int {
            let mut s: ::core::ffi::c_int = 0;
            let mut n: ::core::ffi::c_int = 0;
            let mut zBuf: [::core::ffi::c_char; 16] = [0; 16];
            computeHMS(&raw mut x);
            zBuf[0 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.h / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[1 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.h % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[2 as ::core::ffi::c_int as usize] = ':' as i32 as ::core::ffi::c_char;
            zBuf[3 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.m / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[4 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.m % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[5 as ::core::ffi::c_int as usize] = ':' as i32 as ::core::ffi::c_char;
            if x.useSubsec() != 0 {
                s = (1000.0f64 * x.s + 0.5f64) as ::core::ffi::c_int;
                zBuf[6 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s / 10000 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                zBuf[7 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s / 1000 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                zBuf[8 as ::core::ffi::c_int as usize] = '.' as i32
                    as ::core::ffi::c_char;
                zBuf[9 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s / 100 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                zBuf[10 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                zBuf[11 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
                zBuf[12 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
                n = 12 as ::core::ffi::c_int;
            } else {
                s = x.s as ::core::ffi::c_int;
                zBuf[6 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                zBuf[7 as ::core::ffi::c_int as usize] = ('0' as i32
                    + s % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
                zBuf[8 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
                n = 8 as ::core::ffi::c_int;
            }
            sqlite3_result_text(
                context,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                n,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
    }
}
unsafe extern "C" fn dateFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut x: DateTime = DateTime {
            iJD: 0,
            Y: 0,
            M: 0,
            D: 0,
            h: 0,
            m: 0,
            tz: 0,
            s: 0.,
            validJD: 0,
            validYMD: 0,
            validHMS: 0,
            nFloor: 0,
            rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
            c2rust_padding: [0; 3]

        };
        if isDate(context, argc, argv, &raw mut x) == 0 as ::core::ffi::c_int {
            let mut Y: ::core::ffi::c_int = 0;
            let mut zBuf: [::core::ffi::c_char; 16] = [0; 16];
            computeYMD(&raw mut x);
            Y = x.Y;
            if Y < 0 as ::core::ffi::c_int {
                Y = -Y;
            }
            zBuf[1 as ::core::ffi::c_int as usize] = ('0' as i32
                + Y / 1000 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[2 as ::core::ffi::c_int as usize] = ('0' as i32
                + Y / 100 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[3 as ::core::ffi::c_int as usize] = ('0' as i32
                + Y / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[4 as ::core::ffi::c_int as usize] = ('0' as i32
                + Y % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[5 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
            zBuf[6 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.M / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[7 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.M % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[8 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
            zBuf[9 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.D / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[10 as ::core::ffi::c_int as usize] = ('0' as i32
                + x.D % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[11 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            if x.Y < 0 as ::core::ffi::c_int {
                zBuf[0 as ::core::ffi::c_int as usize] = '-' as i32
                    as ::core::ffi::c_char;
                sqlite3_result_text(
                    context,
                    &raw mut zBuf as *mut ::core::ffi::c_char,
                    11 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            } else {
                sqlite3_result_text(
                    context,
                    (&raw mut zBuf as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    10 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            }
        }
    }
}
unsafe extern "C" fn daysAfterJan01(mut pDate: *mut DateTime) -> ::core::ffi::c_int {
    unsafe {
        let mut jan01: DateTime = *pDate;
        jan01.validJD = 0 as ::core::ffi::c_char;
        jan01.M = 1 as ::core::ffi::c_int;
        jan01.D = 1 as ::core::ffi::c_int;
        computeJD(&raw mut jan01);
        return (((*pDate).iJD as ::core::ffi::c_longlong
            - jan01.iJD as ::core::ffi::c_longlong + 43200000 as ::core::ffi::c_longlong)
            / 86400000 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn daysAfterMonday(mut pDate: *mut DateTime) -> ::core::ffi::c_int {
    unsafe {
        return (((*pDate).iJD as ::core::ffi::c_longlong
            + 43200000 as ::core::ffi::c_longlong) / 86400000 as ::core::ffi::c_longlong)
            as ::core::ffi::c_int % 7 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn daysAfterSunday(mut pDate: *mut DateTime) -> ::core::ffi::c_int {
    unsafe {
        return (((*pDate).iJD as ::core::ffi::c_longlong
            + 129600000 as ::core::ffi::c_longlong)
            / 86400000 as ::core::ffi::c_longlong) as ::core::ffi::c_int
            % 7 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn strftimeFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut x: DateTime = DateTime {
            iJD: 0,
            Y: 0,
            M: 0,
            D: 0,
            h: 0,
            m: 0,
            tz: 0,
            s: 0.,
            validJD: 0,
            validYMD: 0,
            validHMS: 0,
            nFloor: 0,
            rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
            c2rust_padding: [0; 3]

        };
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zFmt: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut sRes: sqlite3_str = sqlite3_str {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        if argc == 0 as ::core::ffi::c_int {
            return;
        }
        zFmt = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        if zFmt.is_null()
            || isDate(
                context,
                argc - 1 as ::core::ffi::c_int,
                argv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut x,
            ) != 0
        {
            return;
        }
        db = sqlite3_context_db_handle(context);
        sqlite3StrAccumInit(
            &raw mut sRes,
            ::core::ptr::null_mut::<sqlite3>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            (*db).aLimit[SQLITE_LIMIT_LENGTH as usize],
        );
        computeJD(&raw mut x);
        computeYMD_HMS(&raw mut x);
        j = 0 as size_t;
        i = j;
        while *zFmt.offset(i as isize) != 0 {
            let mut cf: ::core::ffi::c_char = 0;
            if !(*zFmt.offset(i as isize) as ::core::ffi::c_int != '%' as i32) {
                if j < i {
                    sqlite3_str_append(
                        &raw mut sRes,
                        zFmt.offset(j as isize),
                        i.wrapping_sub(j) as ::core::ffi::c_int,
                    );
                }
                i = i.wrapping_add(1);
                j = i.wrapping_add(1 as size_t);
                cf = *zFmt.offset(i as isize);
                match cf as ::core::ffi::c_int {
                    100 | 101 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            if cf as ::core::ffi::c_int == 'd' as i32 {
                                b"%02d\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"%2d\0".as_ptr() as *const ::core::ffi::c_char
                            },
                            x.D,
                        );
                    }
                    102 => {
                        let mut s: ::core::ffi::c_double = x.s;
                        if s > 59.999f64 {
                            s = 59.999f64;
                        }
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%06.3f\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                        );
                    }
                    70 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%04d-%02d-%02d\0".as_ptr() as *const ::core::ffi::c_char,
                            x.Y,
                            x.M,
                            x.D,
                        );
                    }
                    71 | 103 => {
                        let mut y: DateTime = x;
                        y.iJD
                            += ((3 as ::core::ffi::c_int - daysAfterMonday(&raw mut x))
                                * 86400000 as ::core::ffi::c_int)
                                as ::core::ffi::c_longlong;
                        y.validYMD = 0 as ::core::ffi::c_char;
                        computeYMD(&raw mut y);
                        if cf as ::core::ffi::c_int == 'g' as i32 {
                            sqlite3_str_appendf(
                                &raw mut sRes,
                                b"%02d\0".as_ptr() as *const ::core::ffi::c_char,
                                y.Y % 100 as ::core::ffi::c_int,
                            );
                        } else {
                            sqlite3_str_appendf(
                                &raw mut sRes,
                                b"%04d\0".as_ptr() as *const ::core::ffi::c_char,
                                y.Y,
                            );
                        }
                    }
                    72 | 107 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            if cf as ::core::ffi::c_int == 'H' as i32 {
                                b"%02d\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"%2d\0".as_ptr() as *const ::core::ffi::c_char
                            },
                            x.h,
                        );
                    }
                    73 | 108 => {
                        let mut h: ::core::ffi::c_int = x.h;
                        if h > 12 as ::core::ffi::c_int {
                            h -= 12 as ::core::ffi::c_int;
                        }
                        if h == 0 as ::core::ffi::c_int {
                            h = 12 as ::core::ffi::c_int;
                        }
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            if cf as ::core::ffi::c_int == 'I' as i32 {
                                b"%02d\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"%2d\0".as_ptr() as *const ::core::ffi::c_char
                            },
                            h,
                        );
                    }
                    106 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%03d\0".as_ptr() as *const ::core::ffi::c_char,
                            daysAfterJan01(&raw mut x) + 1 as ::core::ffi::c_int,
                        );
                    }
                    74 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%.16g\0".as_ptr() as *const ::core::ffi::c_char,
                            x.iJD as ::core::ffi::c_double / 86400000.0f64,
                        );
                    }
                    109 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%02d\0".as_ptr() as *const ::core::ffi::c_char,
                            x.M,
                        );
                    }
                    77 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%02d\0".as_ptr() as *const ::core::ffi::c_char,
                            x.m,
                        );
                    }
                    112 | 80 => {
                        if x.h >= 12 as ::core::ffi::c_int {
                            sqlite3_str_append(
                                &raw mut sRes,
                                if cf as ::core::ffi::c_int == 'p' as i32 {
                                    b"PM\0".as_ptr() as *const ::core::ffi::c_char
                                } else {
                                    b"pm\0".as_ptr() as *const ::core::ffi::c_char
                                },
                                2 as ::core::ffi::c_int,
                            );
                        } else {
                            sqlite3_str_append(
                                &raw mut sRes,
                                if cf as ::core::ffi::c_int == 'p' as i32 {
                                    b"AM\0".as_ptr() as *const ::core::ffi::c_char
                                } else {
                                    b"am\0".as_ptr() as *const ::core::ffi::c_char
                                },
                                2 as ::core::ffi::c_int,
                            );
                        }
                    }
                    82 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%02d:%02d\0".as_ptr() as *const ::core::ffi::c_char,
                            x.h,
                            x.m,
                        );
                    }
                    115 => {
                        if x.useSubsec() != 0 {
                            sqlite3_str_appendf(
                                &raw mut sRes,
                                b"%.3f\0".as_ptr() as *const ::core::ffi::c_char,
                                (x.iJD as i64_0
                                    - 21086676 as i64_0
                                        * 10000000 as ::core::ffi::c_int as i64_0)
                                    as ::core::ffi::c_double / 1000.0f64,
                            );
                        } else {
                            let mut iS: i64_0 = x.iJD as i64_0 / 1000 as i64_0
                                - 21086676 as i64_0 * 10000 as ::core::ffi::c_int as i64_0;
                            sqlite3_str_appendf(
                                &raw mut sRes,
                                b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
                                iS,
                            );
                        }
                    }
                    83 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%02d\0".as_ptr() as *const ::core::ffi::c_char,
                            x.s as ::core::ffi::c_int,
                        );
                    }
                    84 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%02d:%02d:%02d\0".as_ptr() as *const ::core::ffi::c_char,
                            x.h,
                            x.m,
                            x.s as ::core::ffi::c_int,
                        );
                    }
                    117 | 119 => {
                        let mut c: ::core::ffi::c_char = (daysAfterSunday(&raw mut x)
                            as ::core::ffi::c_char as ::core::ffi::c_int + '0' as i32)
                            as ::core::ffi::c_char;
                        if c as ::core::ffi::c_int == '0' as i32
                            && cf as ::core::ffi::c_int == 'u' as i32
                        {
                            c = '7' as i32 as ::core::ffi::c_char;
                        }
                        sqlite3_str_appendchar(
                            &raw mut sRes,
                            1 as ::core::ffi::c_int,
                            c,
                        );
                    }
                    85 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%02d\0".as_ptr() as *const ::core::ffi::c_char,
                            (daysAfterJan01(&raw mut x) - daysAfterSunday(&raw mut x)
                                + 7 as ::core::ffi::c_int) / 7 as ::core::ffi::c_int,
                        );
                    }
                    86 => {
                        let mut y_0: DateTime = x;
                        y_0.iJD
                            += ((3 as ::core::ffi::c_int - daysAfterMonday(&raw mut x))
                                * 86400000 as ::core::ffi::c_int)
                                as ::core::ffi::c_longlong;
                        y_0.validYMD = 0 as ::core::ffi::c_char;
                        computeYMD(&raw mut y_0);
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%02d\0".as_ptr() as *const ::core::ffi::c_char,
                            daysAfterJan01(&raw mut y_0) / 7 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int,
                        );
                    }
                    87 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%02d\0".as_ptr() as *const ::core::ffi::c_char,
                            (daysAfterJan01(&raw mut x) - daysAfterMonday(&raw mut x)
                                + 7 as ::core::ffi::c_int) / 7 as ::core::ffi::c_int,
                        );
                    }
                    89 => {
                        sqlite3_str_appendf(
                            &raw mut sRes,
                            b"%04d\0".as_ptr() as *const ::core::ffi::c_char,
                            x.Y,
                        );
                    }
                    37 => {
                        sqlite3_str_appendchar(
                            &raw mut sRes,
                            1 as ::core::ffi::c_int,
                            '%' as i32 as ::core::ffi::c_char,
                        );
                    }
                    _ => {
                        sqlite3_str_reset(&raw mut sRes);
                        return;
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        if j < i {
            sqlite3_str_append(
                &raw mut sRes,
                zFmt.offset(j as isize),
                i.wrapping_sub(j) as ::core::ffi::c_int,
            );
        }
        sqlite3ResultStrAccum(context, &raw mut sRes);
    }
}
unsafe extern "C" fn ctimeFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    unsafe {
        timeFunc(
            context,
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
}
unsafe extern "C" fn cdateFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    unsafe {
        dateFunc(
            context,
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
}
unsafe extern "C" fn timediffFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed1: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut sign: ::core::ffi::c_char = 0;
        let mut Y: ::core::ffi::c_int = 0;
        let mut M: ::core::ffi::c_int = 0;
        let mut d1: DateTime = DateTime {
            iJD: 0,
            Y: 0,
            M: 0,
            D: 0,
            h: 0,
            m: 0,
            tz: 0,
            s: 0.,
            validJD: 0,
            validYMD: 0,
            validHMS: 0,
            nFloor: 0,
            rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
            c2rust_padding: [0; 3]

        };
        let mut d2: DateTime = DateTime {
            iJD: 0,
            Y: 0,
            M: 0,
            D: 0,
            h: 0,
            m: 0,
            tz: 0,
            s: 0.,
            validJD: 0,
            validYMD: 0,
            validHMS: 0,
            nFloor: 0,
            rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
            c2rust_padding: [0; 3]

        };
        let mut sRes: sqlite3_str = sqlite3_str {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        if isDate(
            context,
            1 as ::core::ffi::c_int,
            argv.offset(0 as ::core::ffi::c_int as isize) as *mut *mut sqlite3_value,
            &raw mut d1,
        ) != 0
        {
            return;
        }
        if isDate(
            context,
            1 as ::core::ffi::c_int,
            argv.offset(1 as ::core::ffi::c_int as isize) as *mut *mut sqlite3_value,
            &raw mut d2,
        ) != 0
        {
            return;
        }
        computeYMD_HMS(&raw mut d1);
        computeYMD_HMS(&raw mut d2);
        if d1.iJD >= d2.iJD {
            sign = '+' as i32 as ::core::ffi::c_char;
            Y = d1.Y - d2.Y;
            if Y != 0 {
                d2.Y = d1.Y;
                d2.validJD = 0 as ::core::ffi::c_char;
                computeJD(&raw mut d2);
            }
            M = d1.M - d2.M;
            if M < 0 as ::core::ffi::c_int {
                Y -= 1;
                M += 12 as ::core::ffi::c_int;
            }
            if M != 0 as ::core::ffi::c_int {
                d2.M = d1.M;
                d2.validJD = 0 as ::core::ffi::c_char;
                computeJD(&raw mut d2);
            }
            while d1.iJD < d2.iJD {
                M -= 1;
                if M < 0 as ::core::ffi::c_int {
                    M = 11 as ::core::ffi::c_int;
                    Y -= 1;
                }
                d2.M -= 1;
                if d2.M < 1 as ::core::ffi::c_int {
                    d2.M = 12 as ::core::ffi::c_int;
                    d2.Y -= 1;
                }
                d2.validJD = 0 as ::core::ffi::c_char;
                computeJD(&raw mut d2);
            }
            d1.iJD -= d2.iJD as ::core::ffi::c_longlong;
            d1.iJD = (d1.iJD as ::core::ffi::c_ulonglong)
                .wrapping_add(
                    (1486995408 as ::core::ffi::c_int as u64_0)
                        .wrapping_mul(100000 as ::core::ffi::c_int as u64_0)
                        as ::core::ffi::c_ulonglong,
                ) as sqlite3_int64 as sqlite3_int64;
        } else {
            sign = '-' as i32 as ::core::ffi::c_char;
            Y = d2.Y - d1.Y;
            if Y != 0 {
                d2.Y = d1.Y;
                d2.validJD = 0 as ::core::ffi::c_char;
                computeJD(&raw mut d2);
            }
            M = d2.M - d1.M;
            if M < 0 as ::core::ffi::c_int {
                Y -= 1;
                M += 12 as ::core::ffi::c_int;
            }
            if M != 0 as ::core::ffi::c_int {
                d2.M = d1.M;
                d2.validJD = 0 as ::core::ffi::c_char;
                computeJD(&raw mut d2);
            }
            while d1.iJD > d2.iJD {
                M -= 1;
                if M < 0 as ::core::ffi::c_int {
                    M = 11 as ::core::ffi::c_int;
                    Y -= 1;
                }
                d2.M += 1;
                if d2.M > 12 as ::core::ffi::c_int {
                    d2.M = 1 as ::core::ffi::c_int;
                    d2.Y += 1;
                }
                d2.validJD = 0 as ::core::ffi::c_char;
                computeJD(&raw mut d2);
            }
            d1.iJD = d2.iJD - d1.iJD;
            d1.iJD = (d1.iJD as ::core::ffi::c_ulonglong)
                .wrapping_add(
                    (1486995408 as ::core::ffi::c_int as u64_0)
                        .wrapping_mul(100000 as ::core::ffi::c_int as u64_0)
                        as ::core::ffi::c_ulonglong,
                ) as sqlite3_int64 as sqlite3_int64;
        }
        clearYMD_HMS_TZ(&raw mut d1);
        computeYMD_HMS(&raw mut d1);
        sqlite3StrAccumInit(
            &raw mut sRes,
            ::core::ptr::null_mut::<sqlite3>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
        );
        sqlite3_str_appendf(
            &raw mut sRes,
            b"%c%04d-%02d-%02d %02d:%02d:%06.3f\0".as_ptr()
                as *const ::core::ffi::c_char,
            sign as ::core::ffi::c_int,
            Y,
            M,
            d1.D - 1 as ::core::ffi::c_int,
            d1.h,
            d1.m,
            d1.s,
        );
        sqlite3ResultStrAccum(context, &raw mut sRes);
    }
}
unsafe extern "C" fn ctimestampFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    unsafe {
        datetimeFunc(
            context,
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RegisterDateTimeFunctions() {
    unsafe {
        static mut aDateTimeFuncs: [FuncDef; 10] = unsafe {
            [
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_SLOCHNG | SQLITE_UTF8
                        | SQLITE_FUNC_CONSTANT) as u32_0,
                    pUserData: &raw const sqlite3Config as *mut Sqlite3Config
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        juliandayFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"julianday\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_SLOCHNG | SQLITE_UTF8
                        | SQLITE_FUNC_CONSTANT) as u32_0,
                    pUserData: &raw const sqlite3Config as *mut Sqlite3Config
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        unixepochFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"unixepoch\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_SLOCHNG | SQLITE_UTF8
                        | SQLITE_FUNC_CONSTANT) as u32_0,
                    pUserData: &raw const sqlite3Config as *mut Sqlite3Config
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        dateFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"date\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_SLOCHNG | SQLITE_UTF8
                        | SQLITE_FUNC_CONSTANT) as u32_0,
                    pUserData: &raw const sqlite3Config as *mut Sqlite3Config
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        timeFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"time\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_SLOCHNG | SQLITE_UTF8
                        | SQLITE_FUNC_CONSTANT) as u32_0,
                    pUserData: &raw const sqlite3Config as *mut Sqlite3Config
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        datetimeFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"datetime\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: -1 as ::core::ffi::c_int as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_SLOCHNG | SQLITE_UTF8
                        | SQLITE_FUNC_CONSTANT) as u32_0,
                    pUserData: &raw const sqlite3Config as *mut Sqlite3Config
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        strftimeFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"strftime\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_SLOCHNG | SQLITE_UTF8
                        | SQLITE_FUNC_CONSTANT) as u32_0,
                    pUserData: &raw const sqlite3Config as *mut Sqlite3Config
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        timediffFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"timediff\0".as_ptr() as *const ::core::ffi::c_char,
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
                        ctimeFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"current_time\0".as_ptr() as *const ::core::ffi::c_char,
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
                        ctimestampFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"current_timestamp\0".as_ptr() as *const ::core::ffi::c_char,
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
                        cdateFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"current_date\0".as_ptr() as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
            ]
        };
        sqlite3InsertBuiltinFuncs(
            &raw mut aDateTimeFuncs as *mut FuncDef,
            (::core::mem::size_of::<[FuncDef; 10]>() as usize)
                .wrapping_div(::core::mem::size_of::<FuncDef>() as usize)
                as ::core::ffi::c_int,
        );
    }
}
pub const SQLITE_FUNC_CONSTANT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_FUNC_SLOCHNG: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_BUILTIN: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
