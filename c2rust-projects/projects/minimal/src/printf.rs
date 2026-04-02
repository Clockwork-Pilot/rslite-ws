use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type RenameToken;
    pub type Vdbe;
    pub type sqlite3_mutex;
    pub type sqlite3_pcache;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_result_error_code(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3DbMallocRaw(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3Realloc(_: *mut ::core::ffi::c_void, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbRealloc(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: u64_0,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbMallocSize(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3FpDecode(
        _: *mut FpDecode,
        _: ::core::ffi::c_double,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ErrorToParser(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3RowSetClear(_: *mut ::core::ffi::c_void);
    fn sqlite3AppendOneUtf8Character(
        _: *mut ::core::ffi::c_char,
        _: u32_0,
    ) -> ::core::ffi::c_int;
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
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
pub type StrAccum = sqlite3_str;
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
pub type size_t = usize;
pub type etByte = ::core::ffi::c_uchar;
pub type uptr = u64_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PrintfArguments {
    pub nArg: ::core::ffi::c_int,
    pub nUsed: ::core::ffi::c_int,
    pub apArg: *mut *mut sqlite3_value

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct et_info {
    pub fmttype: ::core::ffi::c_char,
    pub base: etByte,
    pub flags: etByte,
    pub type_0: etByte,
    pub charset: etByte,
    pub prefix: etByte,
    pub iNxt: ::core::ffi::c_char

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpDecode {
    pub sign: ::core::ffi::c_char,
    pub isSpecial: ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub iDP: ::core::ffi::c_int,
    pub z: *mut ::core::ffi::c_char,
    pub zBuf: [::core::ffi::c_char; 24]

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RCStr {
    pub nRCRef: u64_0

}
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LENGTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_MAX_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
pub const SF_MultiValue: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SF_NestedFrom: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_PRINTF_INTERNAL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_PRINTF_SQLFUNC: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_PRINTF_MALLOCED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const etRADIX: ::core::ffi::c_int = 0;
pub const etFLOAT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const etEXP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const etGENERIC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const etSIZE: ::core::ffi::c_int = 4;
pub const etSTRING: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const etDYNSTRING: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const etPERCENT: ::core::ffi::c_int = 7;
pub const etCHARX: ::core::ffi::c_int = 8;
pub const etESCAPE_q: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const etESCAPE_Q: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const etTOKEN: ::core::ffi::c_int = 11;
pub const etSRCITEM: ::core::ffi::c_int = 12;
pub const etPOINTER: ::core::ffi::c_int = 13;
pub const etESCAPE_w: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const etORDINAL: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const etDECIMAL: ::core::ffi::c_int = 16;
pub const etINVALID: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const FLAG_SIGNED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut aDigits: [::core::ffi::c_char; 33] = unsafe {
    ::core::mem::transmute::<
        [u8; 33],
        [::core::ffi::c_char; 33],
    >(*b"0123456789ABCDEF0123456789abcdef\0")
};
static mut aPrefix: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"-x0\0X0\0")
};
static mut fmtinfo: [et_info; 23] = [
    et_info {
        fmttype: 's' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 4 as etByte,
        type_0: etSTRING as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 1 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'E' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 1 as etByte,
        type_0: etEXP as etByte,
        charset: 14 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'u' as i32 as ::core::ffi::c_char,
        base: 10 as etByte,
        flags: 0 as etByte,
        type_0: etDECIMAL as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 3 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'G' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 1 as etByte,
        type_0: etGENERIC as etByte,
        charset: 14 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'w' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 4 as etByte,
        type_0: etESCAPE_w as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'x' as i32 as ::core::ffi::c_char,
        base: 16 as etByte,
        flags: 0 as etByte,
        type_0: etRADIX as etByte,
        charset: 16 as etByte,
        prefix: 1 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'c' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 0 as etByte,
        type_0: etCHARX as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'z' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 4 as etByte,
        type_0: etDYNSTRING as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 6 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'd' as i32 as ::core::ffi::c_char,
        base: 10 as etByte,
        flags: 1 as etByte,
        type_0: etDECIMAL as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'e' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 1 as etByte,
        type_0: etEXP as etByte,
        charset: 30 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'f' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 1 as etByte,
        type_0: etFLOAT as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'g' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 1 as etByte,
        type_0: etGENERIC as etByte,
        charset: 30 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'Q' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 4 as etByte,
        type_0: etESCAPE_Q as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'i' as i32 as ::core::ffi::c_char,
        base: 10 as etByte,
        flags: 1 as etByte,
        type_0: etDECIMAL as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: '%' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 0 as etByte,
        type_0: etPERCENT as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 16 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'T' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 0 as etByte,
        type_0: etTOKEN as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'S' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 0 as etByte,
        type_0: etSRCITEM as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'X' as i32 as ::core::ffi::c_char,
        base: 16 as etByte,
        flags: 0 as etByte,
        type_0: etRADIX as etByte,
        charset: 0 as etByte,
        prefix: 4 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'n' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 0 as etByte,
        type_0: etSIZE as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'o' as i32 as ::core::ffi::c_char,
        base: 8 as etByte,
        flags: 0 as etByte,
        type_0: etRADIX as etByte,
        charset: 0 as etByte,
        prefix: 2 as etByte,
        iNxt: 17 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'p' as i32 as ::core::ffi::c_char,
        base: 16 as etByte,
        flags: 0 as etByte,
        type_0: etPOINTER as etByte,
        charset: 0 as etByte,
        prefix: 1 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'q' as i32 as ::core::ffi::c_char,
        base: 0 as etByte,
        flags: 4 as etByte,
        type_0: etESCAPE_q as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
    et_info {
        fmttype: 'r' as i32 as ::core::ffi::c_char,
        base: 10 as etByte,
        flags: 1 as etByte,
        type_0: etORDINAL as etByte,
        charset: 0 as etByte,
        prefix: 0 as etByte,
        iNxt: 0 as ::core::ffi::c_char

    },
];
#[no_mangle]
pub unsafe extern "C" fn sqlite3StrAccumSetError(
    mut p: *mut StrAccum,
    mut eError: u8_0,
) {
    unsafe {
        (*p).accError = eError;
        if (*p).mxAlloc != 0 {
            sqlite3_str_reset(p);
        }
        if eError as ::core::ffi::c_int == SQLITE_TOOBIG {
            sqlite3ErrorToParser((*p).db, eError as ::core::ffi::c_int);
        }
    }
}
unsafe extern "C" fn getIntArg(mut p: *mut PrintfArguments) -> sqlite3_int64 {
    unsafe {
        if (*p).nArg <= (*p).nUsed {
            return 0 as sqlite3_int64;
        }
        let c2rust_fresh42 = (*p).nUsed;
        (*p).nUsed = (*p).nUsed + 1;
        return sqlite3_value_int64(*(*p).apArg.offset(c2rust_fresh42 as isize));
    }
}
unsafe extern "C" fn getDoubleArg(mut p: *mut PrintfArguments) -> ::core::ffi::c_double {
    unsafe {
        if (*p).nArg <= (*p).nUsed {
            return 0.0f64;
        }
        let c2rust_fresh41 = (*p).nUsed;
        (*p).nUsed = (*p).nUsed + 1;
        return sqlite3_value_double(*(*p).apArg.offset(c2rust_fresh41 as isize));
    }
}
unsafe extern "C" fn getTextArg(
    mut p: *mut PrintfArguments,
) -> *mut ::core::ffi::c_char {
    unsafe {
        if (*p).nArg <= (*p).nUsed {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        let c2rust_fresh40 = (*p).nUsed;
        (*p).nUsed = (*p).nUsed + 1;
        return sqlite3_value_text(*(*p).apArg.offset(c2rust_fresh40 as isize))
            as *mut ::core::ffi::c_char;
    }
}
unsafe extern "C" fn printfTempBuf(
    mut pAccum: *mut sqlite3_str,
    mut n: sqlite3_int64,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if (*pAccum).accError != 0 {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if n > (*pAccum).nAlloc as ::core::ffi::c_longlong
            && n > (*pAccum).mxAlloc as ::core::ffi::c_longlong
        {
            sqlite3StrAccumSetError(pAccum as *mut StrAccum, SQLITE_TOOBIG as u8_0);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        z = sqlite3DbMallocRaw((*pAccum).db, n as u64_0) as *mut ::core::ffi::c_char;
        if z.is_null() {
            sqlite3StrAccumSetError(pAccum as *mut StrAccum, SQLITE_NOMEM as u8_0);
        }
        return z;
    }
}
pub const SQLITE_PRINT_BUF_SIZE: ::core::ffi::c_int = 70 as ::core::ffi::c_int;
pub const etBUFSIZE: ::core::ffi::c_int = SQLITE_PRINT_BUF_SIZE;
pub const SQLITE_FP_PRECISION_LIMIT: ::core::ffi::c_int = 100000000
    as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_vappendf(
    mut pAccum: *mut sqlite3_str,
    mut fmt: *const ::core::ffi::c_char,
    mut ap: ::core::ffi::VaList,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut c: ::core::ffi::c_int = 0;
        let mut bufpt: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut precision: ::core::ffi::c_int = 0;
        let mut length: ::core::ffi::c_int = 0;
        let mut idx: ::core::ffi::c_int = 0;
        let mut width: ::core::ffi::c_int = 0;
        let mut flag_leftjustify: etByte = 0;
        let mut flag_prefix: etByte = 0;
        let mut flag_alternateform: etByte = 0;
        let mut flag_altform2: etByte = 0;
        let mut flag_zeropad: etByte = 0;
        let mut flag_long: etByte = 0;
        let mut done: etByte = 0;
        let mut cThousand: etByte = 0;
        let mut xtype: etByte = etINVALID as etByte;
        let mut bArgList: u8_0 = 0;
        let mut prefix: ::core::ffi::c_char = 0;
        let mut longvalue: sqlite_uint64 = 0;
        let mut realvalue: ::core::ffi::c_double = 0.;
        let mut infop: *const et_info = ::core::ptr::null::<et_info>();
        let mut zOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nOut: ::core::ffi::c_int = 0;
        let mut zExtra: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut exp: ::core::ffi::c_int = 0;
        let mut e2: ::core::ffi::c_int = 0;
        let mut flag_dp: etByte = 0;
        let mut flag_rtz: etByte = 0;
        let mut pArgList: *mut PrintfArguments = ::core::ptr::null_mut::<
            PrintfArguments,
        >();
        let mut buf: [::core::ffi::c_char; 70] = [0; 70];
        bufpt = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if (*pAccum).printfFlags as ::core::ffi::c_int & SQLITE_PRINTF_SQLFUNC
            != 0 as ::core::ffi::c_int
        {
            pArgList = ap.arg::<*mut PrintfArguments>();
            bArgList = 1 as u8_0;
        } else {
            bArgList = 0 as u8_0;
        }
        loop {
            c = *fmt as ::core::ffi::c_int;
            if !(c != 0 as ::core::ffi::c_int) {
                break;
            }
            if c != '%' as i32 {
                bufpt = fmt as *mut ::core::ffi::c_char;
                fmt = strchr(fmt, '%' as i32);
                if fmt.is_null() {
                    fmt = bufpt.offset(strlen(bufpt) as isize);
                }
                sqlite3_str_append(
                    pAccum,
                    bufpt,
                    fmt.offset_from(bufpt) as ::core::ffi::c_long as ::core::ffi::c_int,
                );
                if *fmt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
            }
            fmt = fmt.offset(1);
            c = *fmt as ::core::ffi::c_int;
            if c == 0 as ::core::ffi::c_int {
                sqlite3_str_append(
                    pAccum,
                    b"%\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                );
                break;
            } else {
                flag_zeropad = 0 as etByte;
                flag_altform2 = flag_zeropad;
                flag_alternateform = flag_altform2;
                cThousand = flag_alternateform;
                flag_prefix = cThousand;
                flag_leftjustify = flag_prefix;
                done = 0 as etByte;
                width = 0 as ::core::ffi::c_int;
                flag_long = 0 as etByte;
                precision = -1 as ::core::ffi::c_int;
                loop {
                    match c {
                        45 => {
                            flag_leftjustify = 1 as etByte;
                        }
                        43 => {
                            flag_prefix = '+' as i32 as etByte;
                        }
                        32 => {
                            flag_prefix = ' ' as i32 as etByte;
                        }
                        35 => {
                            flag_alternateform = 1 as etByte;
                        }
                        33 => {
                            flag_altform2 = 1 as etByte;
                        }
                        48 => {
                            flag_zeropad = 1 as etByte;
                        }
                        44 => {
                            cThousand = ',' as i32 as etByte;
                        }
                        108 => {
                            flag_long = 1 as etByte;
                            fmt = fmt.offset(1);
                            c = *fmt as ::core::ffi::c_int;
                            if c == 'l' as i32 {
                                fmt = fmt.offset(1);
                                c = *fmt as ::core::ffi::c_int;
                                flag_long = 2 as etByte;
                            }
                            done = 1 as etByte;
                        }
                        49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                            let mut wx: ::core::ffi::c_uint = (c - '0' as i32)
                                as ::core::ffi::c_uint;
                            loop {
                                fmt = fmt.offset(1);
                                c = *fmt as ::core::ffi::c_int;
                                if !(c >= '0' as i32 && c <= '9' as i32) {
                                    break;
                                }
                                wx = wx
                                    .wrapping_mul(10 as ::core::ffi::c_uint)
                                    .wrapping_add(c as ::core::ffi::c_uint)
                                    .wrapping_sub('0' as i32 as ::core::ffi::c_uint);
                            }
                            width = (wx
                                & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint)
                                as ::core::ffi::c_int;
                            if c != '.' as i32 && c != 'l' as i32 {
                                done = 1 as etByte;
                            } else {
                                fmt = fmt.offset(-1);
                            }
                        }
                        42 => {
                            if bArgList != 0 {
                                width = getIntArg(pArgList) as ::core::ffi::c_int;
                            } else {
                                width = ap.arg::<::core::ffi::c_int>();
                            }
                            if width < 0 as ::core::ffi::c_int {
                                flag_leftjustify = 1 as etByte;
                                width = if width >= -2147483647 as ::core::ffi::c_int {
                                    -width
                                } else {
                                    0 as ::core::ffi::c_int
                                };
                            }
                            c = *fmt.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int;
                            if c != '.' as i32 && c != 'l' as i32 {
                                fmt = fmt.offset(1);
                                c = *fmt as ::core::ffi::c_int;
                                done = 1 as etByte;
                            }
                        }
                        46 => {
                            fmt = fmt.offset(1);
                            c = *fmt as ::core::ffi::c_int;
                            if c == '*' as i32 {
                                if bArgList != 0 {
                                    precision = getIntArg(pArgList) as ::core::ffi::c_int;
                                } else {
                                    precision = ap.arg::<::core::ffi::c_int>();
                                }
                                if precision < 0 as ::core::ffi::c_int {
                                    precision = if precision
                                        >= -2147483647 as ::core::ffi::c_int
                                    {
                                        -precision
                                    } else {
                                        -1 as ::core::ffi::c_int
                                    };
                                }
                                fmt = fmt.offset(1);
                                c = *fmt as ::core::ffi::c_int;
                            } else {
                                let mut px: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                                while c >= '0' as i32 && c <= '9' as i32 {
                                    px = px
                                        .wrapping_mul(10 as ::core::ffi::c_uint)
                                        .wrapping_add(c as ::core::ffi::c_uint)
                                        .wrapping_sub('0' as i32 as ::core::ffi::c_uint);
                                    fmt = fmt.offset(1);
                                    c = *fmt as ::core::ffi::c_int;
                                }
                                precision = (px
                                    & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint)
                                    as ::core::ffi::c_int;
                            }
                            if c == 'l' as i32 {
                                fmt = fmt.offset(-1);
                            } else {
                                done = 1 as etByte;
                            }
                        }
                        _ => {
                            done = 1 as etByte;
                        }
                    }
                    if !(done == 0
                        && {
                            fmt = fmt.offset(1);
                            c = *fmt as ::core::ffi::c_int;
                            c != 0 as ::core::ffi::c_int
                        })
                    {
                        break;
                    }
                }
                idx = (c as ::core::ffi::c_uint).wrapping_rem(23 as ::core::ffi::c_uint)
                    as ::core::ffi::c_int;
                if fmtinfo[idx as usize].fmttype as ::core::ffi::c_int == c
                    || {
                        idx = fmtinfo[idx as usize].iNxt as ::core::ffi::c_int;
                        fmtinfo[idx as usize].fmttype as ::core::ffi::c_int == c
                    }
                {
                    infop = (&raw const fmtinfo as *const et_info).offset(idx as isize)
                        as *const et_info;
                    xtype = (*infop).type_0;
                } else {
                    infop = (&raw const fmtinfo as *const et_info)
                        .offset(0 as ::core::ffi::c_int as isize) as *const et_info;
                    xtype = etINVALID as etByte;
                }
                match xtype as ::core::ffi::c_int {
                    etPOINTER => {
                        flag_long = (if ::core::mem::size_of::<
                            *mut ::core::ffi::c_char,
                        >() as usize == ::core::mem::size_of::<i64_0>() as usize
                        {
                            2 as ::core::ffi::c_int
                        } else if ::core::mem::size_of::<*mut ::core::ffi::c_char>()
                            as usize
                            == ::core::mem::size_of::<::core::ffi::c_long>() as usize
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as etByte;
                        c2rust_current_block = 2578242482123444017;
                    }
                    etORDINAL | etRADIX => {
                        c2rust_current_block = 2578242482123444017;
                    }
                    etDECIMAL => {
                        c2rust_current_block = 17822572103894488873;
                    }
                    etFLOAT | etEXP | etGENERIC => {
                        let mut s: FpDecode = FpDecode {
                            sign: 0,
                            isSpecial: 0,
                            n: 0,
                            iDP: 0,
                            z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            zBuf: [0; 24]

                        };
                        let mut iRound: ::core::ffi::c_int = 0;
                        let mut j: ::core::ffi::c_int = 0;
                        if bArgList != 0 {
                            realvalue = getDoubleArg(pArgList);
                        } else {
                            realvalue = ap.arg::<::core::ffi::c_double>();
                        }
                        if precision < 0 as ::core::ffi::c_int {
                            precision = 6 as ::core::ffi::c_int;
                        }
                        if precision > SQLITE_FP_PRECISION_LIMIT {
                            precision = SQLITE_FP_PRECISION_LIMIT;
                        }
                        if xtype as ::core::ffi::c_int == etFLOAT {
                            iRound = -precision;
                        } else if xtype as ::core::ffi::c_int == etGENERIC {
                            if precision == 0 as ::core::ffi::c_int {
                                precision = 1 as ::core::ffi::c_int;
                            }
                            iRound = precision;
                        } else {
                            iRound = precision + 1 as ::core::ffi::c_int;
                        }
                        sqlite3FpDecode(
                            &raw mut s,
                            realvalue,
                            iRound,
                            if flag_altform2 as ::core::ffi::c_int != 0 {
                                26 as ::core::ffi::c_int
                            } else {
                                16 as ::core::ffi::c_int
                            },
                        );
                        if s.isSpecial != 0 {
                            if s.isSpecial as ::core::ffi::c_int
                                == 2 as ::core::ffi::c_int
                            {
                                bufpt = (if flag_zeropad as ::core::ffi::c_int != 0 {
                                    b"null\0".as_ptr() as *const ::core::ffi::c_char
                                } else {
                                    b"NaN\0".as_ptr() as *const ::core::ffi::c_char
                                }) as *mut ::core::ffi::c_char;
                                length = sqlite3Strlen30(bufpt);
                                c2rust_current_block = 11068715555910905014;
                            } else if flag_zeropad != 0 {
                                *s.z.offset(0 as ::core::ffi::c_int as isize) = '9' as i32
                                    as ::core::ffi::c_char;
                                s.iDP = 1000 as ::core::ffi::c_int;
                                s.n = 1 as ::core::ffi::c_int;
                                c2rust_current_block = 12252098823794565961;
                            } else {
                                memcpy(
                                    &raw mut buf as *mut ::core::ffi::c_char
                                        as *mut ::core::ffi::c_void,
                                    b"-Inf\0".as_ptr() as *const ::core::ffi::c_char
                                        as *const ::core::ffi::c_void,
                                    5 as size_t,
                                );
                                bufpt = &raw mut buf as *mut ::core::ffi::c_char;
                                if !(s.sign as ::core::ffi::c_int == '-' as i32) {
                                    if flag_prefix != 0 {
                                        buf[0 as ::core::ffi::c_int as usize] = flag_prefix
                                            as ::core::ffi::c_char;
                                    } else {
                                        bufpt = bufpt.offset(1);
                                    }
                                }
                                length = sqlite3Strlen30(bufpt);
                                c2rust_current_block = 11068715555910905014;
                            }
                        } else {
                            c2rust_current_block = 12252098823794565961;
                        }
                        match c2rust_current_block {
                            11068715555910905014 => {}
                            _ => {
                                if s.sign as ::core::ffi::c_int == '-' as i32 {
                                    if flag_alternateform as ::core::ffi::c_int != 0
                                        && flag_prefix == 0
                                        && xtype as ::core::ffi::c_int == etFLOAT && s.iDP <= iRound
                                    {
                                        prefix = 0 as ::core::ffi::c_char;
                                    } else {
                                        prefix = '-' as i32 as ::core::ffi::c_char;
                                    }
                                } else {
                                    prefix = flag_prefix as ::core::ffi::c_char;
                                }
                                exp = s.iDP - 1 as ::core::ffi::c_int;
                                if xtype as ::core::ffi::c_int == etGENERIC {
                                    precision -= 1;
                                    flag_rtz = (flag_alternateform == 0) as ::core::ffi::c_int
                                        as etByte;
                                    if exp < -4 as ::core::ffi::c_int || exp > precision {
                                        xtype = etEXP as etByte;
                                    } else {
                                        precision = precision - exp;
                                        xtype = etFLOAT as etByte;
                                    }
                                } else {
                                    flag_rtz = flag_altform2;
                                }
                                if xtype as ::core::ffi::c_int == etEXP {
                                    e2 = 0 as ::core::ffi::c_int;
                                } else {
                                    e2 = s.iDP - 1 as ::core::ffi::c_int;
                                }
                                bufpt = &raw mut buf as *mut ::core::ffi::c_char;
                                let mut szBufNeeded: i64_0 = 0;
                                szBufNeeded = ((if e2 > 0 as ::core::ffi::c_int {
                                    e2
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as ::core::ffi::c_longlong
                                    + precision as ::core::ffi::c_longlong
                                    + width as ::core::ffi::c_longlong
                                    + 15 as ::core::ffi::c_longlong) as i64_0;
                                if cThousand as ::core::ffi::c_int != 0
                                    && e2 > 0 as ::core::ffi::c_int
                                {
                                    szBufNeeded
                                        += ((e2 + 2 as ::core::ffi::c_int)
                                            / 3 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
                                }
                                if szBufNeeded > etBUFSIZE as ::core::ffi::c_longlong {
                                    zExtra = printfTempBuf(
                                        pAccum,
                                        szBufNeeded as sqlite3_int64,
                                    );
                                    bufpt = zExtra;
                                    if bufpt.is_null() {
                                        return;
                                    }
                                }
                                zOut = bufpt;
                                flag_dp = ((if precision > 0 as ::core::ffi::c_int {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) | flag_alternateform as ::core::ffi::c_int
                                    | flag_altform2 as ::core::ffi::c_int) as etByte;
                                if prefix != 0 {
                                    let c2rust_fresh0 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    *c2rust_fresh0 = prefix;
                                }
                                j = 0 as ::core::ffi::c_int;
                                if e2 < 0 as ::core::ffi::c_int {
                                    let c2rust_fresh1 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    *c2rust_fresh1 = '0' as i32 as ::core::ffi::c_char;
                                } else {
                                    while e2 >= 0 as ::core::ffi::c_int {
                                        let c2rust_fresh3 = bufpt;
                                        bufpt = bufpt.offset(1);
                                        *c2rust_fresh3 = (if j < s.n {
                                            let c2rust_fresh2 = j;
                                            j = j + 1;
                                            *s.z.offset(c2rust_fresh2 as isize) as ::core::ffi::c_int
                                        } else {
                                            '0' as i32
                                        }) as ::core::ffi::c_char;
                                        if cThousand as ::core::ffi::c_int != 0
                                            && e2 % 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                            && e2 > 1 as ::core::ffi::c_int
                                        {
                                            let c2rust_fresh4 = bufpt;
                                            bufpt = bufpt.offset(1);
                                            *c2rust_fresh4 = ',' as i32 as ::core::ffi::c_char;
                                        }
                                        e2 -= 1;
                                    }
                                }
                                if flag_dp != 0 {
                                    let c2rust_fresh5 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    *c2rust_fresh5 = '.' as i32 as ::core::ffi::c_char;
                                }
                                e2 += 1;
                                while e2 < 0 as ::core::ffi::c_int
                                    && precision > 0 as ::core::ffi::c_int
                                {
                                    let c2rust_fresh6 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    *c2rust_fresh6 = '0' as i32 as ::core::ffi::c_char;
                                    precision -= 1;
                                    e2 += 1;
                                }
                                loop {
                                    let c2rust_fresh7 = precision;
                                    precision = precision - 1;
                                    if !(c2rust_fresh7 > 0 as ::core::ffi::c_int) {
                                        break;
                                    }
                                    let c2rust_fresh9 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    *c2rust_fresh9 = (if j < s.n {
                                        let c2rust_fresh8 = j;
                                        j = j + 1;
                                        *s.z.offset(c2rust_fresh8 as isize) as ::core::ffi::c_int
                                    } else {
                                        '0' as i32
                                    }) as ::core::ffi::c_char;
                                }
                                if flag_rtz as ::core::ffi::c_int != 0
                                    && flag_dp as ::core::ffi::c_int != 0
                                {
                                    while *bufpt.offset(-1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int == '0' as i32
                                    {
                                        bufpt = bufpt.offset(-1);
                                        *bufpt = 0 as ::core::ffi::c_char;
                                    }
                                    if *bufpt.offset(-1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int == '.' as i32
                                    {
                                        if flag_altform2 != 0 {
                                            let c2rust_fresh10 = bufpt;
                                            bufpt = bufpt.offset(1);
                                            *c2rust_fresh10 = '0' as i32 as ::core::ffi::c_char;
                                        } else {
                                            bufpt = bufpt.offset(-1);
                                            *bufpt = 0 as ::core::ffi::c_char;
                                        }
                                    }
                                }
                                if xtype as ::core::ffi::c_int == etEXP {
                                    exp = s.iDP - 1 as ::core::ffi::c_int;
                                    let c2rust_fresh11 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    *c2rust_fresh11 = aDigits[(*infop).charset as usize];
                                    if exp < 0 as ::core::ffi::c_int {
                                        let c2rust_fresh12 = bufpt;
                                        bufpt = bufpt.offset(1);
                                        *c2rust_fresh12 = '-' as i32 as ::core::ffi::c_char;
                                        exp = -exp;
                                    } else {
                                        let c2rust_fresh13 = bufpt;
                                        bufpt = bufpt.offset(1);
                                        *c2rust_fresh13 = '+' as i32 as ::core::ffi::c_char;
                                    }
                                    if exp >= 100 as ::core::ffi::c_int {
                                        let c2rust_fresh14 = bufpt;
                                        bufpt = bufpt.offset(1);
                                        *c2rust_fresh14 = (exp / 100 as ::core::ffi::c_int
                                            + '0' as i32) as ::core::ffi::c_char;
                                        exp %= 100 as ::core::ffi::c_int;
                                    }
                                    let c2rust_fresh15 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    *c2rust_fresh15 = (exp / 10 as ::core::ffi::c_int
                                        + '0' as i32) as ::core::ffi::c_char;
                                    let c2rust_fresh16 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    *c2rust_fresh16 = (exp % 10 as ::core::ffi::c_int
                                        + '0' as i32) as ::core::ffi::c_char;
                                }
                                *bufpt = 0 as ::core::ffi::c_char;
                                length = bufpt.offset_from(zOut) as ::core::ffi::c_long
                                    as ::core::ffi::c_int;
                                bufpt = zOut;
                                if flag_zeropad as ::core::ffi::c_int != 0
                                    && flag_leftjustify == 0 && length < width
                                {
                                    let mut i: ::core::ffi::c_int = 0;
                                    let mut nPad: ::core::ffi::c_int = width - length;
                                    i = width;
                                    while i >= nPad {
                                        *bufpt.offset(i as isize) = *bufpt
                                            .offset((i - nPad) as isize);
                                        i -= 1;
                                    }
                                    i = (prefix as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                                    loop {
                                        let c2rust_fresh17 = nPad;
                                        nPad = nPad - 1;
                                        if !(c2rust_fresh17 != 0) {
                                            break;
                                        }
                                        let c2rust_fresh18 = i;
                                        i = i + 1;
                                        *bufpt.offset(c2rust_fresh18 as isize) = '0' as i32
                                            as ::core::ffi::c_char;
                                    }
                                    length = width;
                                }
                                c2rust_current_block = 11068715555910905014;
                            }
                        }
                    }
                    etSIZE => {
                        if bArgList == 0 {
                            *ap.arg::<*mut ::core::ffi::c_int>() = (*pAccum).nChar
                                as ::core::ffi::c_int;
                        }
                        width = 0 as ::core::ffi::c_int;
                        length = width;
                        c2rust_current_block = 11068715555910905014;
                    }
                    etPERCENT => {
                        buf[0 as ::core::ffi::c_int as usize] = '%' as i32
                            as ::core::ffi::c_char;
                        bufpt = &raw mut buf as *mut ::core::ffi::c_char;
                        length = 1 as ::core::ffi::c_int;
                        c2rust_current_block = 11068715555910905014;
                    }
                    etCHARX => {
                        if bArgList != 0 {
                            bufpt = getTextArg(pArgList);
                            length = 1 as ::core::ffi::c_int;
                            if !bufpt.is_null() {
                                let c2rust_fresh19 = bufpt;
                                bufpt = bufpt.offset(1);
                                c = *c2rust_fresh19 as ::core::ffi::c_int;
                                buf[0 as ::core::ffi::c_int as usize] = c
                                    as ::core::ffi::c_char;
                                if c & 0xc0 as ::core::ffi::c_int
                                    == 0xc0 as ::core::ffi::c_int
                                {
                                    while length < 4 as ::core::ffi::c_int
                                        && *bufpt.offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                                            == 0x80 as ::core::ffi::c_int
                                    {
                                        let c2rust_fresh20 = bufpt;
                                        bufpt = bufpt.offset(1);
                                        let c2rust_fresh21 = length;
                                        length = length + 1;
                                        buf[c2rust_fresh21 as usize] = *c2rust_fresh20;
                                    }
                                }
                            } else {
                                buf[0 as ::core::ffi::c_int as usize] = 0
                                    as ::core::ffi::c_char;
                            }
                        } else {
                            let mut ch: ::core::ffi::c_uint = ap
                                .arg::<::core::ffi::c_uint>();
                            length = sqlite3AppendOneUtf8Character(
                                &raw mut buf as *mut ::core::ffi::c_char,
                                ch as u32_0,
                            );
                        }
                        if precision > 1 as ::core::ffi::c_int {
                            let mut nPrior: i64_0 = 1 as i64_0;
                            width -= precision - 1 as ::core::ffi::c_int;
                            if width > 1 as ::core::ffi::c_int && flag_leftjustify == 0 {
                                sqlite3_str_appendchar(
                                    pAccum,
                                    width - 1 as ::core::ffi::c_int,
                                    ' ' as i32 as ::core::ffi::c_char,
                                );
                                width = 0 as ::core::ffi::c_int;
                            }
                            sqlite3_str_append(
                                pAccum,
                                &raw mut buf as *mut ::core::ffi::c_char,
                                length,
                            );
                            precision -= 1;
                            while precision > 1 as ::core::ffi::c_int {
                                let mut nCopyBytes: i64_0 = 0;
                                if nPrior
                                    > (precision - 1 as ::core::ffi::c_int)
                                        as ::core::ffi::c_longlong
                                {
                                    nPrior = (precision - 1 as ::core::ffi::c_int) as i64_0;
                                }
                                nCopyBytes = length as i64_0 * nPrior;
                                if nCopyBytes as ::core::ffi::c_longlong
                                    + (*pAccum).nChar as ::core::ffi::c_longlong
                                    >= (*pAccum).nAlloc as ::core::ffi::c_longlong
                                {
                                    sqlite3StrAccumEnlarge(pAccum as *mut StrAccum, nCopyBytes);
                                }
                                if (*pAccum).accError != 0 {
                                    break;
                                }
                                sqlite3_str_append(
                                    pAccum,
                                    (*pAccum)
                                        .zText
                                        .offset(((*pAccum).nChar as i64_0 - nCopyBytes) as isize)
                                        as *mut ::core::ffi::c_char,
                                    nCopyBytes as ::core::ffi::c_int,
                                );
                                precision = (precision as ::core::ffi::c_longlong
                                    - nPrior as ::core::ffi::c_longlong) as ::core::ffi::c_int;
                                nPrior *= 2 as ::core::ffi::c_longlong;
                            }
                        }
                        bufpt = &raw mut buf as *mut ::core::ffi::c_char;
                        flag_altform2 = 1 as etByte;
                        c2rust_current_block = 1710040039456088087;
                    }
                    etSTRING | etDYNSTRING => {
                        if bArgList != 0 {
                            bufpt = getTextArg(pArgList);
                            xtype = etSTRING as etByte;
                        } else {
                            bufpt = ap.arg::<*mut ::core::ffi::c_char>();
                        }
                        if bufpt.is_null() {
                            bufpt = b"\0".as_ptr() as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            c2rust_current_block = 7175397428936041062;
                        } else if xtype as ::core::ffi::c_int == etDYNSTRING {
                            if (*pAccum).nChar == 0 as ::core::ffi::c_uint
                                && (*pAccum).mxAlloc != 0
                                && width == 0 as ::core::ffi::c_int
                                && precision < 0 as ::core::ffi::c_int
                                && (*pAccum).accError as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                            {
                                (*pAccum).zText = bufpt;
                                (*pAccum).nAlloc = sqlite3DbMallocSize(
                                    (*pAccum).db,
                                    bufpt as *const ::core::ffi::c_void,
                                ) as u32_0;
                                (*pAccum).nChar = (0x7fffffff as ::core::ffi::c_int
                                    & strlen(bufpt) as ::core::ffi::c_int) as u32_0;
                                (*pAccum).printfFlags = ((*pAccum).printfFlags
                                    as ::core::ffi::c_int | SQLITE_PRINTF_MALLOCED) as u8_0;
                                length = 0 as ::core::ffi::c_int;
                                c2rust_current_block = 11068715555910905014;
                            } else {
                                zExtra = bufpt;
                                c2rust_current_block = 7175397428936041062;
                            }
                        } else {
                            c2rust_current_block = 7175397428936041062;
                        }
                        match c2rust_current_block {
                            11068715555910905014 => {}
                            _ => {
                                if precision >= 0 as ::core::ffi::c_int {
                                    if flag_altform2 != 0 {
                                        let mut z: *mut ::core::ffi::c_uchar = bufpt
                                            as *mut ::core::ffi::c_uchar;
                                        loop {
                                            let c2rust_fresh22 = precision;
                                            precision = precision - 1;
                                            if !(c2rust_fresh22 > 0 as ::core::ffi::c_int
                                                && *z.offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int != 0)
                                            {
                                                break;
                                            }
                                            let c2rust_fresh23 = z;
                                            z = z.offset(1);
                                            if *c2rust_fresh23 as ::core::ffi::c_int
                                                >= 0xc0 as ::core::ffi::c_int
                                            {
                                                while *z as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                                                    == 0x80 as ::core::ffi::c_int
                                                {
                                                    z = z.offset(1);
                                                }
                                            }
                                        }
                                        length = z.offset_from(bufpt as *mut ::core::ffi::c_uchar)
                                            as ::core::ffi::c_long as ::core::ffi::c_int;
                                    } else {
                                        length = 0 as ::core::ffi::c_int;
                                        while length < precision
                                            && *bufpt.offset(length as isize) as ::core::ffi::c_int != 0
                                        {
                                            length += 1;
                                        }
                                    }
                                } else {
                                    length = 0x7fffffff as ::core::ffi::c_int
                                        & strlen(bufpt) as ::core::ffi::c_int;
                                }
                                c2rust_current_block = 1710040039456088087;
                            }
                        }
                    }
                    etESCAPE_q | etESCAPE_Q | etESCAPE_w => {
                        let mut i_0: i64_0 = 0;
                        let mut j_0: i64_0 = 0;
                        let mut k: i64_0 = 0;
                        let mut n_0: i64_0 = 0;
                        let mut needQuote: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut ch_0: ::core::ffi::c_char = 0;
                        let mut escarg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        let mut q: ::core::ffi::c_char = 0;
                        if bArgList != 0 {
                            escarg = getTextArg(pArgList);
                        } else {
                            escarg = ap.arg::<*mut ::core::ffi::c_char>();
                        }
                        if escarg.is_null() {
                            escarg = (if xtype as ::core::ffi::c_int == etESCAPE_Q {
                                b"NULL\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"(NULL)\0".as_ptr() as *const ::core::ffi::c_char
                            }) as *mut ::core::ffi::c_char;
                        } else if xtype as ::core::ffi::c_int == etESCAPE_Q {
                            needQuote = 1 as ::core::ffi::c_int;
                        }
                        if xtype as ::core::ffi::c_int == etESCAPE_w {
                            q = '"' as i32 as ::core::ffi::c_char;
                            flag_alternateform = 0 as etByte;
                        } else {
                            q = '\'' as i32 as ::core::ffi::c_char;
                        }
                        k = precision as i64_0;
                        n_0 = 0 as i64_0;
                        i_0 = n_0;
                        while k != 0 as ::core::ffi::c_longlong
                            && {
                                ch_0 = *escarg.offset(i_0 as isize);
                                ch_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                            }
                        {
                            if ch_0 as ::core::ffi::c_int == q as ::core::ffi::c_int {
                                n_0 += 1;
                            }
                            if flag_altform2 as ::core::ffi::c_int != 0
                                && ch_0 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                                    == 0xc0 as ::core::ffi::c_int
                            {
                                while *escarg
                                    .offset(
                                        (i_0 as ::core::ffi::c_longlong
                                            + 1 as ::core::ffi::c_longlong) as isize,
                                    ) as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                                    == 0x80 as ::core::ffi::c_int
                                {
                                    i_0 += 1;
                                }
                            }
                            i_0 += 1;
                            k -= 1;
                        }
                        if flag_alternateform != 0 {
                            let mut nBack: u32_0 = 0 as u32_0;
                            let mut nCtrl: u32_0 = 0 as u32_0;
                            k = 0 as i64_0;
                            while k < i_0 {
                                if *escarg.offset(k as isize) as ::core::ffi::c_int
                                    == '\\' as i32
                                {
                                    nBack = nBack.wrapping_add(1);
                                } else if *(escarg as *mut u8_0).offset(k as isize)
                                    as ::core::ffi::c_int <= 0x1f as ::core::ffi::c_int
                                {
                                    nCtrl = nCtrl.wrapping_add(1);
                                }
                                k += 1;
                            }
                            if nCtrl != 0 || xtype as ::core::ffi::c_int == etESCAPE_q {
                                n_0
                                    += nBack.wrapping_add((5 as u32_0).wrapping_mul(nCtrl))
                                        as ::core::ffi::c_longlong;
                                if xtype as ::core::ffi::c_int == etESCAPE_Q {
                                    n_0 += 10 as ::core::ffi::c_longlong;
                                    needQuote = 2 as ::core::ffi::c_int;
                                }
                            } else {
                                flag_alternateform = 0 as etByte;
                            }
                        }
                        n_0
                            += i_0 as ::core::ffi::c_longlong
                                + 3 as ::core::ffi::c_longlong;
                        if n_0 > etBUFSIZE as ::core::ffi::c_longlong {
                            zExtra = printfTempBuf(pAccum, n_0 as sqlite3_int64);
                            bufpt = zExtra;
                            if bufpt.is_null() {
                                return;
                            }
                        } else {
                            bufpt = &raw mut buf as *mut ::core::ffi::c_char;
                        }
                        j_0 = 0 as i64_0;
                        if needQuote != 0 {
                            if needQuote == 2 as ::core::ffi::c_int {
                                memcpy(
                                    bufpt.offset(j_0 as isize) as *mut ::core::ffi::c_char
                                        as *mut ::core::ffi::c_void,
                                    b"unistr('\0".as_ptr() as *const ::core::ffi::c_char
                                        as *const ::core::ffi::c_void,
                                    8 as size_t,
                                );
                                j_0 += 8 as ::core::ffi::c_longlong;
                            } else {
                                let c2rust_fresh25 = j_0;
                                j_0 = j_0 + 1;
                                *bufpt.offset(c2rust_fresh25 as isize) = '\'' as i32
                                    as ::core::ffi::c_char;
                            }
                        }
                        k = i_0;
                        if flag_alternateform != 0 {
                            i_0 = 0 as i64_0;
                            while i_0 < k {
                                ch_0 = *escarg.offset(i_0 as isize);
                                let c2rust_fresh26 = j_0;
                                j_0 = j_0 + 1;
                                *bufpt.offset(c2rust_fresh26 as isize) = ch_0;
                                if ch_0 as ::core::ffi::c_int == q as ::core::ffi::c_int {
                                    let c2rust_fresh27 = j_0;
                                    j_0 = j_0 + 1;
                                    *bufpt.offset(c2rust_fresh27 as isize) = ch_0;
                                } else if ch_0 as ::core::ffi::c_int == '\\' as i32 {
                                    let c2rust_fresh28 = j_0;
                                    j_0 = j_0 + 1;
                                    *bufpt.offset(c2rust_fresh28 as isize) = '\\' as i32
                                        as ::core::ffi::c_char;
                                } else if ch_0 as ::core::ffi::c_uchar as ::core::ffi::c_int
                                    <= 0x1f as ::core::ffi::c_int
                                {
                                    *bufpt
                                        .offset(
                                            (j_0 as ::core::ffi::c_longlong
                                                - 1 as ::core::ffi::c_longlong) as isize,
                                        ) = '\\' as i32 as ::core::ffi::c_char;
                                    let c2rust_fresh29 = j_0;
                                    j_0 = j_0 + 1;
                                    *bufpt.offset(c2rust_fresh29 as isize) = 'u' as i32
                                        as ::core::ffi::c_char;
                                    let c2rust_fresh30 = j_0;
                                    j_0 = j_0 + 1;
                                    *bufpt.offset(c2rust_fresh30 as isize) = '0' as i32
                                        as ::core::ffi::c_char;
                                    let c2rust_fresh31 = j_0;
                                    j_0 = j_0 + 1;
                                    *bufpt.offset(c2rust_fresh31 as isize) = '0' as i32
                                        as ::core::ffi::c_char;
                                    let c2rust_fresh32 = j_0;
                                    j_0 = j_0 + 1;
                                    *bufpt.offset(c2rust_fresh32 as isize) = (if ch_0
                                        as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                                    {
                                        '1' as i32
                                    } else {
                                        '0' as i32
                                    }) as ::core::ffi::c_char;
                                    let c2rust_fresh33 = j_0;
                                    j_0 = j_0 + 1;
                                    *bufpt.offset(c2rust_fresh33 as isize) = ::core::mem::transmute::<
                                        [u8; 17],
                                        [::core::ffi::c_char; 17],
                                    >(
                                        *b"0123456789abcdef\0",
                                    )[(ch_0 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                                        as usize];
                                }
                                i_0 += 1;
                            }
                        } else {
                            i_0 = 0 as i64_0;
                            while i_0 < k {
                                ch_0 = *escarg.offset(i_0 as isize);
                                let c2rust_fresh34 = j_0;
                                j_0 = j_0 + 1;
                                *bufpt.offset(c2rust_fresh34 as isize) = ch_0;
                                if ch_0 as ::core::ffi::c_int == q as ::core::ffi::c_int {
                                    let c2rust_fresh35 = j_0;
                                    j_0 = j_0 + 1;
                                    *bufpt.offset(c2rust_fresh35 as isize) = ch_0;
                                }
                                i_0 += 1;
                            }
                        }
                        if needQuote != 0 {
                            let c2rust_fresh36 = j_0;
                            j_0 = j_0 + 1;
                            *bufpt.offset(c2rust_fresh36 as isize) = '\'' as i32
                                as ::core::ffi::c_char;
                            if needQuote == 2 as ::core::ffi::c_int {
                                let c2rust_fresh37 = j_0;
                                j_0 = j_0 + 1;
                                *bufpt.offset(c2rust_fresh37 as isize) = ')' as i32
                                    as ::core::ffi::c_char;
                            }
                        }
                        *bufpt.offset(j_0 as isize) = 0 as ::core::ffi::c_char;
                        length = j_0 as ::core::ffi::c_int;
                        c2rust_current_block = 1710040039456088087;
                    }
                    etTOKEN => {
                        if (*pAccum).printfFlags as ::core::ffi::c_int
                            & SQLITE_PRINTF_INTERNAL == 0 as ::core::ffi::c_int
                        {
                            return;
                        }
                        if flag_alternateform != 0 {
                            let mut pExpr: *mut Expr = ap.arg::<*mut Expr>();
                            if !pExpr.is_null()
                                && !((*pExpr).flags & 0x800 as ::core::ffi::c_int as u32_0
                                    != 0 as ::core::ffi::c_uint)
                            {
                                sqlite3_str_appendall(
                                    pAccum,
                                    (*pExpr).u.zToken as *const ::core::ffi::c_char,
                                );
                                sqlite3RecordErrorOffsetOfExpr((*pAccum).db, pExpr);
                            }
                        } else {
                            let mut pToken: *mut Token = ap.arg::<*mut Token>();
                            if !pToken.is_null() && (*pToken).n != 0 {
                                sqlite3_str_append(
                                    pAccum,
                                    (*pToken).z,
                                    (*pToken).n as ::core::ffi::c_int,
                                );
                                sqlite3RecordErrorByteOffset((*pAccum).db, (*pToken).z);
                            }
                        }
                        width = 0 as ::core::ffi::c_int;
                        length = width;
                        c2rust_current_block = 11068715555910905014;
                    }
                    etSRCITEM => {
                        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
                        if (*pAccum).printfFlags as ::core::ffi::c_int
                            & SQLITE_PRINTF_INTERNAL == 0 as ::core::ffi::c_int
                        {
                            return;
                        }
                        pItem = ap.arg::<*mut SrcItem>();
                        if !(*pItem).zAlias.is_null() && flag_altform2 == 0 {
                            sqlite3_str_appendall(pAccum, (*pItem).zAlias);
                        } else if !(*pItem).zName.is_null() {
                            if (*pItem).fg.fixedSchema() as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                                && (*pItem).fg.isSubquery() as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                && !(*pItem).u4.zDatabase.is_null()
                            {
                                sqlite3_str_appendall(pAccum, (*pItem).u4.zDatabase);
                                sqlite3_str_append(
                                    pAccum,
                                    b".\0".as_ptr() as *const ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int,
                                );
                            }
                            sqlite3_str_appendall(pAccum, (*pItem).zName);
                        } else if !(*pItem).zAlias.is_null() {
                            sqlite3_str_appendall(pAccum, (*pItem).zAlias);
                        } else if (*pItem).fg.isSubquery() != 0 {
                            let mut pSel: *mut Select = (*(*pItem).u4.pSubq).pSelect;
                            if (*pSel).selFlags as ::core::ffi::c_uint
                                & SF_NestedFrom as ::core::ffi::c_uint != 0
                            {
                                sqlite3_str_appendf(
                                    pAccum as *mut StrAccum,
                                    b"(join-%u)\0".as_ptr() as *const ::core::ffi::c_char,
                                    (*pSel).selId,
                                );
                            } else if (*pSel).selFlags as ::core::ffi::c_uint
                                & SF_MultiValue as ::core::ffi::c_uint != 0
                            {
                                sqlite3_str_appendf(
                                    pAccum as *mut StrAccum,
                                    b"%u-ROW VALUES CLAUSE\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*pItem).u1.nRow,
                                );
                            } else {
                                sqlite3_str_appendf(
                                    pAccum as *mut StrAccum,
                                    b"(subquery-%u)\0".as_ptr() as *const ::core::ffi::c_char,
                                    (*pSel).selId,
                                );
                            }
                        }
                        width = 0 as ::core::ffi::c_int;
                        length = width;
                        c2rust_current_block = 11068715555910905014;
                    }
                    _ => return

                }
                match c2rust_current_block {
                    2578242482123444017 => {
                        cThousand = 0 as etByte;
                        c2rust_current_block = 17822572103894488873;
                    }
                    1710040039456088087 => {
                        if flag_altform2 as ::core::ffi::c_int != 0
                            && width > 0 as ::core::ffi::c_int
                        {
                            let mut ii: ::core::ffi::c_int = length
                                - 1 as ::core::ffi::c_int;
                            while ii >= 0 as ::core::ffi::c_int {
                                let c2rust_fresh24 = ii;
                                ii = ii - 1;
                                if *bufpt.offset(c2rust_fresh24 as isize)
                                    as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                                    == 0x80 as ::core::ffi::c_int
                                {
                                    width += 1;
                                }
                            }
                        }
                        c2rust_current_block = 11068715555910905014;
                    }
                    _ => {}
                }
                match c2rust_current_block {
                    17822572103894488873 => {
                        if (*infop).flags as ::core::ffi::c_int & FLAG_SIGNED != 0 {
                            let mut v: i64_0 = 0;
                            if bArgList != 0 {
                                v = getIntArg(pArgList) as i64_0;
                            } else if flag_long != 0 {
                                if flag_long as ::core::ffi::c_int
                                    == 2 as ::core::ffi::c_int
                                {
                                    v = ap.arg::<i64_0>();
                                } else {
                                    v = ap.arg::<::core::ffi::c_long>() as i64_0;
                                }
                            } else {
                                v = ap.arg::<::core::ffi::c_int>() as i64_0;
                            }
                            if v < 0 as ::core::ffi::c_longlong {
                                longvalue = !v as sqlite_uint64;
                                longvalue = longvalue.wrapping_add(1);
                                prefix = '-' as i32 as ::core::ffi::c_char;
                            } else {
                                longvalue = v as sqlite_uint64;
                                prefix = flag_prefix as ::core::ffi::c_char;
                            }
                        } else {
                            if bArgList != 0 {
                                longvalue = getIntArg(pArgList) as u64_0 as sqlite_uint64;
                            } else if flag_long != 0 {
                                if flag_long as ::core::ffi::c_int
                                    == 2 as ::core::ffi::c_int
                                {
                                    longvalue = ap.arg::<u64_0>() as sqlite_uint64;
                                } else {
                                    longvalue = ap.arg::<::core::ffi::c_ulong>()
                                        as sqlite_uint64;
                                }
                            } else {
                                longvalue = ap.arg::<::core::ffi::c_uint>()
                                    as sqlite_uint64;
                            }
                            prefix = 0 as ::core::ffi::c_char;
                        }
                        if longvalue == 0 as ::core::ffi::c_ulonglong {
                            flag_alternateform = 0 as etByte;
                        }
                        if flag_zeropad as ::core::ffi::c_int != 0
                            && precision
                                < width
                                    - (prefix as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                        {
                            precision = width
                                - (prefix as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int;
                        }
                        if precision
                            < etBUFSIZE - 10 as ::core::ffi::c_int
                                - etBUFSIZE / 3 as ::core::ffi::c_int
                        {
                            nOut = etBUFSIZE;
                            zOut = &raw mut buf as *mut ::core::ffi::c_char;
                        } else {
                            let mut n: u64_0 = 0;
                            n = (precision as ::core::ffi::c_ulonglong)
                                .wrapping_add(10 as ::core::ffi::c_ulonglong) as u64_0;
                            if cThousand != 0 {
                                n = (n as ::core::ffi::c_ulonglong)
                                    .wrapping_add(
                                        (precision / 3 as ::core::ffi::c_int)
                                            as ::core::ffi::c_ulonglong,
                                    ) as u64_0 as u64_0;
                            }
                            zExtra = printfTempBuf(pAccum, n as sqlite3_int64);
                            zOut = zExtra;
                            if zOut.is_null() {
                                return;
                            }
                            nOut = n as ::core::ffi::c_int;
                        }
                        bufpt = zOut.offset((nOut - 1 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_char;
                        if xtype as ::core::ffi::c_int == etORDINAL {
                            static mut zOrd: [::core::ffi::c_char; 9] = unsafe {
                                ::core::mem::transmute::<
                                    [u8; 9],
                                    [::core::ffi::c_char; 9],
                                >(*b"thstndrd\0")
                            };
                            let mut x: ::core::ffi::c_int = (longvalue
                                as ::core::ffi::c_ulonglong)
                                .wrapping_rem(10 as ::core::ffi::c_ulonglong)
                                as ::core::ffi::c_int;
                            if x >= 4 as ::core::ffi::c_int
                                || (longvalue as ::core::ffi::c_ulonglong)
                                    .wrapping_div(10 as ::core::ffi::c_ulonglong)
                                    .wrapping_rem(10 as ::core::ffi::c_ulonglong)
                                    == 1 as ::core::ffi::c_ulonglong
                            {
                                x = 0 as ::core::ffi::c_int;
                            }
                            bufpt = bufpt.offset(-1);
                            *bufpt = zOrd[(x * 2 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as usize];
                            bufpt = bufpt.offset(-1);
                            *bufpt = zOrd[(x * 2 as ::core::ffi::c_int) as usize];
                        }
                        let mut cset: *const ::core::ffi::c_char = (&raw const aDigits
                            as *const ::core::ffi::c_char)
                            .offset((*infop).charset as isize)
                            as *const ::core::ffi::c_char;
                        let mut base: u8_0 = (*infop).base as u8_0;
                        loop {
                            bufpt = bufpt.offset(-1);
                            *bufpt = *cset
                                .offset(
                                    (longvalue as ::core::ffi::c_ulonglong)
                                        .wrapping_rem(base as ::core::ffi::c_ulonglong) as isize,
                                );
                            longvalue = (longvalue as ::core::ffi::c_ulonglong)
                                .wrapping_div(base as ::core::ffi::c_ulonglong)
                                as sqlite_uint64;
                            if !(longvalue > 0 as ::core::ffi::c_ulonglong) {
                                break;
                            }
                        }
                        length = (zOut.offset((nOut - 1 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_char)
                            .offset_from(bufpt) as ::core::ffi::c_long
                            as ::core::ffi::c_int;
                        while precision > length {
                            bufpt = bufpt.offset(-1);
                            *bufpt = '0' as i32 as ::core::ffi::c_char;
                            length += 1;
                        }
                        if cThousand != 0 {
                            let mut nn: ::core::ffi::c_int = (length
                                - 1 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                            let mut ix: ::core::ffi::c_int = (length
                                - 1 as ::core::ffi::c_int) % 3 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int;
                            bufpt = bufpt.offset(-(nn as isize));
                            idx = 0 as ::core::ffi::c_int;
                            while nn > 0 as ::core::ffi::c_int {
                                *bufpt.offset(idx as isize) = *bufpt
                                    .offset((idx + nn) as isize);
                                ix -= 1;
                                if ix == 0 as ::core::ffi::c_int {
                                    idx += 1;
                                    *bufpt.offset(idx as isize) = cThousand
                                        as ::core::ffi::c_char;
                                    nn -= 1;
                                    ix = 3 as ::core::ffi::c_int;
                                }
                                idx += 1;
                            }
                        }
                        if prefix != 0 {
                            bufpt = bufpt.offset(-1);
                            *bufpt = prefix;
                        }
                        if flag_alternateform as ::core::ffi::c_int != 0
                            && (*infop).prefix as ::core::ffi::c_int != 0
                        {
                            let mut pre: *const ::core::ffi::c_char = ::core::ptr::null::<
                                ::core::ffi::c_char,
                            >();
                            let mut x_0: ::core::ffi::c_char = 0;
                            pre = (&raw const aPrefix as *const ::core::ffi::c_char)
                                .offset((*infop).prefix as isize)
                                as *const ::core::ffi::c_char;
                            loop {
                                x_0 = *pre;
                                if !(x_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                                    break;
                                }
                                bufpt = bufpt.offset(-1);
                                *bufpt = x_0;
                                pre = pre.offset(1);
                            }
                        }
                        length = (zOut.offset((nOut - 1 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_char)
                            .offset_from(bufpt) as ::core::ffi::c_long
                            as ::core::ffi::c_int;
                    }
                    _ => {}
                }
                width -= length;
                if width > 0 as ::core::ffi::c_int {
                    if flag_leftjustify == 0 {
                        sqlite3_str_appendchar(
                            pAccum,
                            width,
                            ' ' as i32 as ::core::ffi::c_char,
                        );
                    }
                    sqlite3_str_append(pAccum, bufpt, length);
                    if flag_leftjustify != 0 {
                        sqlite3_str_appendchar(
                            pAccum,
                            width,
                            ' ' as i32 as ::core::ffi::c_char,
                        );
                    }
                } else {
                    sqlite3_str_append(pAccum, bufpt, length);
                }
                if !zExtra.is_null() {
                    sqlite3DbFree((*pAccum).db, zExtra as *mut ::core::ffi::c_void);
                    zExtra = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                fmt = fmt.offset(1);
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RecordErrorByteOffset(
    mut db: *mut sqlite3,
    mut z: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut pParse: *const Parse = ::core::ptr::null::<Parse>();
        let mut zText: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if db.is_null() {
            return;
        }
        if (*db).errByteOffset != -2 as ::core::ffi::c_int {
            return;
        }
        pParse = (*db).pParse;
        if pParse.is_null() {
            return;
        }
        zText = (*pParse).zTail;
        if zText.is_null() {
            return;
        }
        zEnd = zText
            .offset(
                (strlen
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> size_t)(zText)
                    as isize,
            ) as *const ::core::ffi::c_char;
        if z as uptr >= zText as uptr && (z as uptr) < zEnd as uptr {
            (*db).errByteOffset = z.offset_from(zText) as ::core::ffi::c_long
                as ::core::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RecordErrorOffsetOfExpr(
    mut db: *mut sqlite3,
    mut pExpr: *const Expr,
) {
    unsafe {
        while !pExpr.is_null()
            && ((*pExpr).flags
                & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int) as u32_0
                != 0 as ::core::ffi::c_uint
                || (*pExpr).w.iOfst <= 0 as ::core::ffi::c_int)
        {
            pExpr = (*pExpr).pLeft;
        }
        if pExpr.is_null() {
            return;
        }
        if (*pExpr).flags & 0x40000000 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint
        {
            return;
        }
        (*db).errByteOffset = (*pExpr).w.iOfst;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3StrAccumEnlarge(
    mut p: *mut StrAccum,
    mut N: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if (*p).accError != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*p).mxAlloc == 0 as ::core::ffi::c_uint {
            sqlite3StrAccumSetError(p, SQLITE_TOOBIG as u8_0);
            return ((*p).nAlloc as ::core::ffi::c_uint)
                .wrapping_sub((*p).nChar as ::core::ffi::c_uint)
                .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        } else {
            let mut zOld: *mut ::core::ffi::c_char = if (*p).printfFlags
                as ::core::ffi::c_int & SQLITE_PRINTF_MALLOCED != 0 as ::core::ffi::c_int
            {
                (*p).zText
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
            let mut szNew: i64_0 = (*p).nChar as i64_0 + N + 1 as i64_0;
            if szNew as ::core::ffi::c_longlong + (*p).nChar as ::core::ffi::c_longlong
                <= (*p).mxAlloc as ::core::ffi::c_longlong
            {
                szNew += (*p).nChar as ::core::ffi::c_longlong;
            }
            if szNew > (*p).mxAlloc as ::core::ffi::c_longlong {
                sqlite3_str_reset(p);
                sqlite3StrAccumSetError(p, SQLITE_TOOBIG as u8_0);
                return 0 as ::core::ffi::c_int;
            } else {
                (*p).nAlloc = szNew as ::core::ffi::c_int as u32_0;
            }
            if !(*p).db.is_null() {
                zNew = sqlite3DbRealloc(
                    (*p).db,
                    zOld as *mut ::core::ffi::c_void,
                    (*p).nAlloc as u64_0,
                ) as *mut ::core::ffi::c_char;
            } else {
                zNew = sqlite3Realloc(
                    zOld as *mut ::core::ffi::c_void,
                    (*p).nAlloc as u64_0,
                ) as *mut ::core::ffi::c_char;
            }
            if !zNew.is_null() {
                if !((*p).printfFlags as ::core::ffi::c_int & SQLITE_PRINTF_MALLOCED
                    != 0 as ::core::ffi::c_int) && (*p).nChar > 0 as ::core::ffi::c_uint
                {
                    memcpy(
                        zNew as *mut ::core::ffi::c_void,
                        (*p).zText as *const ::core::ffi::c_void,
                        (*p).nChar as size_t,
                    );
                }
                (*p).zText = zNew;
                (*p).nAlloc = sqlite3DbMallocSize(
                    (*p).db,
                    zNew as *const ::core::ffi::c_void,
                ) as u32_0;
                (*p).printfFlags = ((*p).printfFlags as ::core::ffi::c_int
                    | SQLITE_PRINTF_MALLOCED) as u8_0;
            } else {
                sqlite3_str_reset(p);
                sqlite3StrAccumSetError(p, SQLITE_NOMEM as u8_0);
                return 0 as ::core::ffi::c_int;
            }
        }
        return N as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_appendchar(
    mut p: *mut sqlite3_str,
    mut N: ::core::ffi::c_int,
    mut c: ::core::ffi::c_char,
) {
    unsafe {
        if (*p).nChar as i64_0 + N as i64_0 >= (*p).nAlloc as ::core::ffi::c_longlong
            && {
                N = sqlite3StrAccumEnlarge(p as *mut StrAccum, N as i64_0);
                N <= 0 as ::core::ffi::c_int
            }
        {
            return;
        }
        loop {
            let c2rust_fresh38 = N;
            N = N - 1;
            if !(c2rust_fresh38 > 0 as ::core::ffi::c_int) {
                break;
            }
            let c2rust_fresh39 = (*p).nChar;
            (*p).nChar = (*p).nChar.wrapping_add(1);
            *(*p).zText.offset(c2rust_fresh39 as isize) = c;
        };
    }
}
#[inline(never)]
unsafe extern "C" fn enlargeAndAppend(
    mut p: *mut StrAccum,
    mut z: *const ::core::ffi::c_char,
    mut N: ::core::ffi::c_int,
) {
    unsafe {
        N = sqlite3StrAccumEnlarge(p, N as i64_0);
        if N > 0 as ::core::ffi::c_int {
            memcpy(
                (*p).zText.offset((*p).nChar as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                N as size_t,
            );
            (*p).nChar = ((*p).nChar as ::core::ffi::c_uint)
                .wrapping_add(N as ::core::ffi::c_uint) as u32_0 as u32_0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_append(
    mut p: *mut sqlite3_str,
    mut z: *const ::core::ffi::c_char,
    mut N: ::core::ffi::c_int,
) {
    unsafe {
        if ((*p).nChar as ::core::ffi::c_uint).wrapping_add(N as ::core::ffi::c_uint)
            >= (*p).nAlloc
        {
            enlargeAndAppend(p as *mut StrAccum, z, N);
        } else if N != 0 {
            (*p).nChar = ((*p).nChar as ::core::ffi::c_uint)
                .wrapping_add(N as ::core::ffi::c_uint) as u32_0 as u32_0;
            memcpy(
                (*p)
                    .zText
                    .offset(
                        ((*p).nChar as ::core::ffi::c_uint)
                            .wrapping_sub(N as ::core::ffi::c_uint) as isize,
                    ) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                N as size_t,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_appendall(
    mut p: *mut sqlite3_str,
    mut z: *const ::core::ffi::c_char,
) {
    unsafe {
        sqlite3_str_append(p, z, sqlite3Strlen30(z));
    }
}
#[inline(never)]
unsafe extern "C" fn strAccumFinishRealloc(
    mut p: *mut StrAccum,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zText: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        zText = sqlite3DbMallocRaw(
            (*p).db,
            (1 as u64_0).wrapping_add((*p).nChar as u64_0),
        ) as *mut ::core::ffi::c_char;
        if !zText.is_null() {
            memcpy(
                zText as *mut ::core::ffi::c_void,
                (*p).zText as *const ::core::ffi::c_void,
                ((*p).nChar as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint) as size_t,
            );
            (*p).printfFlags = ((*p).printfFlags as ::core::ffi::c_int
                | SQLITE_PRINTF_MALLOCED) as u8_0;
        } else {
            sqlite3StrAccumSetError(p, SQLITE_NOMEM as u8_0);
        }
        (*p).zText = zText;
        return zText;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3StrAccumFinish(
    mut p: *mut StrAccum,
) -> *mut ::core::ffi::c_char {
    unsafe {
        if !(*p).zText.is_null() {
            *(*p).zText.offset((*p).nChar as isize) = 0 as ::core::ffi::c_char;
            if (*p).mxAlloc > 0 as ::core::ffi::c_uint
                && !((*p).printfFlags as ::core::ffi::c_int & SQLITE_PRINTF_MALLOCED
                    != 0 as ::core::ffi::c_int)
            {
                return strAccumFinishRealloc(p);
            }
        }
        return (*p).zText;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ResultStrAccum(
    mut pCtx: *mut sqlite3_context,
    mut p: *mut StrAccum,
) {
    unsafe {
        if (*p).accError != 0 {
            sqlite3_result_error_code(pCtx, (*p).accError as ::core::ffi::c_int);
            sqlite3_str_reset(p);
        } else if (*p).printfFlags as ::core::ffi::c_int & SQLITE_PRINTF_MALLOCED
            != 0 as ::core::ffi::c_int
        {
            sqlite3_result_text(
                pCtx,
                (*p).zText,
                (*p).nChar as ::core::ffi::c_int,
                Some(
                    sqlite3RowSetClear
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        } else {
            sqlite3_result_text(
                pCtx,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            sqlite3_str_reset(p);
        };
    }
}
static mut sqlite3OomStr: sqlite3_str = sqlite3_str {
    db: ::core::ptr::null::<sqlite3>() as *mut sqlite3,
    zText: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    nAlloc: 0 as u32_0,
    mxAlloc: 0 as u32_0,
    nChar: 0 as u32_0,
    accError: SQLITE_NOMEM as u8_0,
    printfFlags: 0 as u8_0

};
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_finish(
    mut p: *mut sqlite3_str,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if !p.is_null() && p != &raw mut sqlite3OomStr {
            z = sqlite3StrAccumFinish(p as *mut StrAccum);
            sqlite3_free(p as *mut ::core::ffi::c_void);
        } else {
            z = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return z;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_errcode(
    mut p: *mut sqlite3_str,
) -> ::core::ffi::c_int {
    unsafe {
        return if !p.is_null() {
            (*p).accError as ::core::ffi::c_int
        } else {
            SQLITE_NOMEM
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_length(
    mut p: *mut sqlite3_str,
) -> ::core::ffi::c_int {
    unsafe {
        return (if !p.is_null() {
            (*p).nChar as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        }) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_value(
    mut p: *mut sqlite3_str,
) -> *mut ::core::ffi::c_char {
    unsafe {
        if p.is_null() || (*p).nChar == 0 as ::core::ffi::c_uint {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        *(*p).zText.offset((*p).nChar as isize) = 0 as ::core::ffi::c_char;
        return (*p).zText;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_reset(mut p: *mut StrAccum) {
    unsafe {
        if (*p).printfFlags as ::core::ffi::c_int & SQLITE_PRINTF_MALLOCED
            != 0 as ::core::ffi::c_int
        {
            sqlite3DbFree((*p).db, (*p).zText as *mut ::core::ffi::c_void);
            (*p).printfFlags = ((*p).printfFlags as ::core::ffi::c_int
                & !SQLITE_PRINTF_MALLOCED) as u8_0;
        }
        (*p).nAlloc = 0 as u32_0;
        (*p).nChar = 0 as u32_0;
        (*p).zText = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3StrAccumInit(
    mut p: *mut StrAccum,
    mut db: *mut sqlite3,
    mut zBase: *mut ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut mx: ::core::ffi::c_int,
) {
    unsafe {
        (*p).zText = zBase;
        (*p).db = db;
        (*p).nAlloc = n as u32_0;
        (*p).mxAlloc = mx as u32_0;
        (*p).nChar = 0 as u32_0;
        (*p).accError = 0 as u8_0;
        (*p).printfFlags = 0 as u8_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_new(mut db: *mut sqlite3) -> *mut sqlite3_str {
    unsafe {
        let mut p: *mut sqlite3_str = sqlite3_malloc64(
            ::core::mem::size_of::<sqlite3_str>() as sqlite3_uint64,
        ) as *mut sqlite3_str;
        if !p.is_null() {
            sqlite3StrAccumInit(
                p as *mut StrAccum,
                ::core::ptr::null_mut::<sqlite3>(),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
                if !db.is_null() {
                    (*db).aLimit[SQLITE_LIMIT_LENGTH as usize]
                } else {
                    SQLITE_MAX_LENGTH
                },
            );
        } else {
            p = &raw mut sqlite3OomStr;
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VMPrintf(
    mut db: *mut sqlite3,
    mut zFormat: *const ::core::ffi::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zBase: [::core::ffi::c_char; 70] = [0; 70];
        let mut acc: StrAccum = StrAccum {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        sqlite3StrAccumInit(
            &raw mut acc,
            db,
            &raw mut zBase as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 70]>() as ::core::ffi::c_int,
            (*db).aLimit[SQLITE_LIMIT_LENGTH as usize],
        );
        acc.printfFlags = SQLITE_PRINTF_INTERNAL as u8_0;
        sqlite3_str_vappendf(&raw mut acc, zFormat, ap);
        z = sqlite3StrAccumFinish(&raw mut acc);
        if acc.accError as ::core::ffi::c_int == SQLITE_NOMEM {
            sqlite3OomFault(db);
        }
        return z;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MPrintf(
    mut db: *mut sqlite3,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut ::core::ffi::c_char {
    unsafe {
        
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut ap = c2rust_args;
        z = sqlite3VMPrintf(db, zFormat, ap);
        return z;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vmprintf(
    mut zFormat: *const ::core::ffi::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zBase: [::core::ffi::c_char; 70] = [0; 70];
        let mut acc: StrAccum = StrAccum {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        sqlite3StrAccumInit(
            &raw mut acc,
            ::core::ptr::null_mut::<sqlite3>(),
            &raw mut zBase as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 70]>() as ::core::ffi::c_int,
            SQLITE_MAX_LENGTH,
        );
        sqlite3_str_vappendf(&raw mut acc, zFormat, ap);
        z = sqlite3StrAccumFinish(&raw mut acc);
        return z;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_mprintf(
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut ::core::ffi::c_char {
    unsafe {
        
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut ap = c2rust_args;
        z = sqlite3_vmprintf(zFormat, ap);
        return z;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vsnprintf(
    mut n: ::core::ffi::c_int,
    mut zBuf: *mut ::core::ffi::c_char,
    mut zFormat: *const ::core::ffi::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut acc: StrAccum = StrAccum {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        if n <= 0 as ::core::ffi::c_int {
            return zBuf;
        }
        sqlite3StrAccumInit(
            &raw mut acc,
            ::core::ptr::null_mut::<sqlite3>(),
            zBuf,
            n,
            0 as ::core::ffi::c_int,
        );
        sqlite3_str_vappendf(&raw mut acc, zFormat, ap);
        *zBuf.offset(acc.nChar as isize) = 0 as ::core::ffi::c_char;
        return zBuf;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_snprintf(
    mut n: ::core::ffi::c_int,
    mut zBuf: *mut ::core::ffi::c_char,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut acc: StrAccum = StrAccum {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        
        if n <= 0 as ::core::ffi::c_int {
            return zBuf;
        }
        sqlite3StrAccumInit(
            &raw mut acc,
            ::core::ptr::null_mut::<sqlite3>(),
            zBuf,
            n,
            0 as ::core::ffi::c_int,
        );
        let mut ap = c2rust_args;
        sqlite3_str_vappendf(&raw mut acc, zFormat, ap);
        *zBuf.offset(acc.nChar as isize) = 0 as ::core::ffi::c_char;
        return zBuf;
    }
}
unsafe extern "C" fn renderLogMsg(
    mut iErrCode: ::core::ffi::c_int,
    mut zFormat: *const ::core::ffi::c_char,
    mut ap: ::core::ffi::VaList,
) {
    unsafe {
        let mut acc: StrAccum = StrAccum {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        let mut zMsg: [::core::ffi::c_char; 700] = [0; 700];
        sqlite3StrAccumInit(
            &raw mut acc,
            ::core::ptr::null_mut::<sqlite3>(),
            &raw mut zMsg as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 700]>() as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        sqlite3_str_vappendf(&raw mut acc, zFormat, ap);
        sqlite3Config
            .xLog
            .expect(
                "non-null function pointer",
            )(sqlite3Config.pLogArg, iErrCode, sqlite3StrAccumFinish(&raw mut acc));
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_log(
    mut iErrCode: ::core::ffi::c_int,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        
        if sqlite3Config.xLog.is_some() {
            let mut ap = c2rust_args;
            renderLogMsg(iErrCode, zFormat, ap);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_str_appendf(
    mut p: *mut StrAccum,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        
        let mut ap = c2rust_args;
        sqlite3_str_vappendf(p as *mut sqlite3_str, zFormat, ap);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RCStrRef(
    mut z: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut p: *mut RCStr = z as *mut RCStr;
        p = p.offset(-1);
        (*p).nRCRef = (*p).nRCRef.wrapping_add(1);
        return z;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RCStrUnref(mut z: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut RCStr = z as *mut RCStr;
        p = p.offset(-1);
        if (*p).nRCRef >= 2 as ::core::ffi::c_ulonglong {
            (*p).nRCRef = (*p).nRCRef.wrapping_sub(1);
        } else {
            sqlite3_free(p as *mut ::core::ffi::c_void);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RCStrNew(mut N: u64_0) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut p: *mut RCStr = sqlite3_malloc64(
            (N as sqlite3_uint64)
                .wrapping_add(::core::mem::size_of::<RCStr>() as sqlite3_uint64)
                .wrapping_add(1 as sqlite3_uint64),
        ) as *mut RCStr;
        if p.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        (*p).nRCRef = 1 as u64_0;
        return p.offset(1 as ::core::ffi::c_int as isize) as *mut RCStr
            as *mut ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RCStrResize(
    mut z: *mut ::core::ffi::c_char,
    mut N: u64_0,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut p: *mut RCStr = z as *mut RCStr;
        let mut pNew: *mut RCStr = ::core::ptr::null_mut::<RCStr>();
        p = p.offset(-1);
        pNew = sqlite3_realloc64(
            p as *mut ::core::ffi::c_void,
            (N as sqlite3_uint64)
                .wrapping_add(::core::mem::size_of::<RCStr>() as sqlite3_uint64)
                .wrapping_add(1 as sqlite3_uint64),
        ) as *mut RCStr;
        if pNew.is_null() {
            sqlite3_free(p as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            return pNew.offset(1 as ::core::ffi::c_int as isize) as *mut RCStr
                as *mut ::core::ffi::c_char
        };
    }
}
