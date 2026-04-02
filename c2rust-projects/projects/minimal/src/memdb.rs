use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type RenameToken;
    pub type Vdbe;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type sqlite3_pcache;
    pub type Pager;
    pub type PCache;
    fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
        errmsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_int64(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> sqlite3_int64;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(
        _: *mut sqlite3_vfs,
        makeDflt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_mutex_alloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    fn sqlite3_mutex_free(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn sqlite3_file_control(
        _: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
        op: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerGet(
        pPager: *mut Pager,
        pgno: Pgno,
        ppPage: *mut *mut DbPage,
        clrFlag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerUnref(_: *mut DbPage);
    fn sqlite3PagerGetData(_: *mut DbPage) -> *mut ::core::ffi::c_void;
    fn sqlite3BtreeGetPageSize(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3Malloc(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3Realloc(_: *mut ::core::ffi::c_void, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3MutexAlloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3FindDbName(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
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
pub type DbPage = PgHdr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PgHdr {
    pub pPage: *mut sqlite3_pcache_page,
    pub pData: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void,
    pub pCache: *mut PCache,
    pub pDirty: *mut PgHdr,
    pub pPager: *mut Pager,
    pub pgno: Pgno,
    pub flags: u16_0,
    pub nRef: i64_0,
    pub pDirtyNext: *mut PgHdr,
    pub pDirtyPrev: *mut PgHdr

}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemStore {
    pub sz: sqlite3_int64,
    pub szAlloc: sqlite3_int64,
    pub szMax: sqlite3_int64,
    pub aData: *mut ::core::ffi::c_uchar,
    pub pMutex: *mut sqlite3_mutex,
    pub nMmap: ::core::ffi::c_int,
    pub mFlags: ::core::ffi::c_uint,
    pub nRdLock: ::core::ffi::c_int,
    pub nWrLock: ::core::ffi::c_int,
    pub nRef: ::core::ffi::c_int,
    pub zFName: *mut ::core::ffi::c_char

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemFile {
    pub base: sqlite3_file,
    pub pStore: *mut MemStore,
    pub eLock: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemFS {
    pub nMemStore: ::core::ffi::c_int,
    pub apMemStore: *mut *mut MemStore

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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_READONLY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_FULL: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_IOERR_SHORT_READ: ::core::ffi::c_int = SQLITE_IOERR
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_WRITE: ::core::ffi::c_int = SQLITE_IOERR
    | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MEMORY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SAFE_APPEND: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SEQUENTIAL: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 0x1000
    as ::core::ffi::c_int;
pub const SQLITE_LOCK_SHARED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_LOCK_RESERVED: ::core::ffi::c_int = 2;
pub const SQLITE_LOCK_PENDING: ::core::ffi::c_int = 3;
pub const SQLITE_FCNTL_FILE_POINTER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_VFSNAME: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_SIZE_LIMIT: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_FAST: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_STATIC_VFS1: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_SERIALIZE_NOCOPY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_DESERIALIZE_FREEONCLOSE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DESERIALIZE_RESIZEABLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_DESERIALIZE_READONLY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut memdb_g: MemFS = MemFS {
    nMemStore: 0,
    apMemStore: ::core::ptr::null::<*mut MemStore>() as *mut *mut MemStore

};
static mut memdb_vfs: sqlite3_vfs = unsafe {
    sqlite3_vfs {
        iVersion: 2 as ::core::ffi::c_int,
        szOsFile: 0 as ::core::ffi::c_int,
        mxPathname: 1024 as ::core::ffi::c_int,
        pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
        zName: b"memdb\0".as_ptr() as *const ::core::ffi::c_char,
        pAppData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        xOpen: Some(
            memdbOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xDelete: None,
        xAccess: Some(
            memdbAccess
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFullPathname: Some(
            memdbFullPathname
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xDlOpen: Some(
            memdbDlOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_void,
        ),
        xDlError: Some(
            memdbDlError
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> (),
        ),
        xDlSym: Some(
            memdbDlSym
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> Option<unsafe extern "C" fn() -> ()>,
        ),
        xDlClose: Some(
            memdbDlClose
                as unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
        ),
        xRandomness: Some(
            memdbRandomness
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xSleep: Some(
            memdbSleep
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCurrentTime: None,
        xGetLastError: Some(
            memdbGetLastError
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xCurrentTimeInt64: Some(
            memdbCurrentTimeInt64
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSetSystemCall: None,
        xGetSystemCall: None,
        xNextSystemCall: None

    }
};
static mut memdb_io_methods: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 3 as ::core::ffi::c_int,
        xClose: Some(
            memdbClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            memdbRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            memdbWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            memdbTruncate
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            memdbSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            memdbFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xLock: Some(
            memdbLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xUnlock: Some(
            memdbUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCheckReservedLock: None,
        xFileControl: Some(
            memdbFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xSectorSize: None,
        xDeviceCharacteristics: Some(
            memdbDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xShmMap: None,
        xShmLock: None,
        xShmBarrier: None,
        xShmUnmap: None,
        xFetch: Some(
            memdbFetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xUnfetch: Some(
            memdbUnfetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )

    }
};
unsafe extern "C" fn memdbEnter(mut p: *mut MemStore) {
    unsafe {
        sqlite3_mutex_enter((*p).pMutex);
    }
}
unsafe extern "C" fn memdbLeave(mut p: *mut MemStore) {
    unsafe {
        sqlite3_mutex_leave((*p).pMutex);
    }
}
unsafe extern "C" fn memdbClose(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemStore = (*(pFile as *mut MemFile)).pStore;
        if !(*p).zFName.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            let mut pVfsMutex: *mut sqlite3_mutex = sqlite3MutexAlloc(
                SQLITE_MUTEX_STATIC_VFS1,
            );
            sqlite3_mutex_enter(pVfsMutex);
            i = 0 as ::core::ffi::c_int;
            while i < memdb_g.nMemStore {
                if *memdb_g.apMemStore.offset(i as isize) == p {
                    memdbEnter(p);
                    if (*p).nRef == 1 as ::core::ffi::c_int {
                        memdb_g.nMemStore -= 1;
                        let ref mut c2rust_fresh1 = *memdb_g
                            .apMemStore
                            .offset(i as isize);
                        *c2rust_fresh1 = *memdb_g
                            .apMemStore
                            .offset(memdb_g.nMemStore as isize);
                        if memdb_g.nMemStore == 0 as ::core::ffi::c_int {
                            sqlite3_free(memdb_g.apMemStore as *mut ::core::ffi::c_void);
                            memdb_g.apMemStore = ::core::ptr::null_mut::<
                                *mut MemStore,
                            >();
                        }
                    }
                    break;
                } else {
                    i += 1;
                }
            }
            sqlite3_mutex_leave(pVfsMutex);
        } else {
            memdbEnter(p);
        }
        (*p).nRef -= 1;
        if (*p).nRef <= 0 as ::core::ffi::c_int {
            if (*p).mFlags & SQLITE_DESERIALIZE_FREEONCLOSE as ::core::ffi::c_uint != 0 {
                sqlite3_free((*p).aData as *mut ::core::ffi::c_void);
            }
            memdbLeave(p);
            sqlite3_mutex_free((*p).pMutex);
            sqlite3_free(p as *mut ::core::ffi::c_void);
        } else {
            memdbLeave(p);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memdbRead(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemStore = (*(pFile as *mut MemFile)).pStore;
        memdbEnter(p);
        if iOfst as ::core::ffi::c_longlong + iAmt as ::core::ffi::c_longlong > (*p).sz {
            memset(zBuf, 0 as ::core::ffi::c_int, iAmt as size_t);
            if iOfst < (*p).sz {
                memcpy(
                    zBuf,
                    (*p).aData.offset(iOfst as isize) as *const ::core::ffi::c_void,
                    ((*p).sz as sqlite_int64 - iOfst) as size_t,
                );
            }
            memdbLeave(p);
            return SQLITE_IOERR_SHORT_READ;
        }
        memcpy(
            zBuf,
            (*p).aData.offset(iOfst as isize) as *const ::core::ffi::c_void,
            iAmt as size_t,
        );
        memdbLeave(p);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memdbEnlarge(
    mut p: *mut MemStore,
    mut newSz: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pNew: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        if (*p).mFlags & SQLITE_DESERIALIZE_RESIZEABLE as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint || (*p).nMmap > 0 as ::core::ffi::c_int
        {
            return SQLITE_FULL;
        }
        if newSz > (*p).szMax {
            return SQLITE_FULL;
        }
        newSz *= 2 as ::core::ffi::c_longlong;
        if newSz > (*p).szMax {
            newSz = (*p).szMax;
        }
        pNew = sqlite3Realloc((*p).aData as *mut ::core::ffi::c_void, newSz as u64_0)
            as *mut ::core::ffi::c_uchar;
        if pNew.is_null() {
            return SQLITE_IOERR_NOMEM;
        }
        (*p).aData = pNew;
        (*p).szAlloc = newSz;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memdbWrite(
    mut pFile: *mut sqlite3_file,
    mut z: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemStore = (*(pFile as *mut MemFile)).pStore;
        memdbEnter(p);
        if (*p).mFlags & 4 as ::core::ffi::c_uint != 0 {
            memdbLeave(p);
            return SQLITE_IOERR_WRITE;
        }
        if iOfst as ::core::ffi::c_longlong + iAmt as ::core::ffi::c_longlong > (*p).sz {
            let mut rc: ::core::ffi::c_int = 0;
            if iOfst as ::core::ffi::c_longlong + iAmt as ::core::ffi::c_longlong
                > (*p).szAlloc
                && {
                    rc = memdbEnlarge(p, iOfst as sqlite3_int64 + iAmt as sqlite3_int64);
                    rc != SQLITE_OK
                }
            {
                memdbLeave(p);
                return rc;
            }
            if iOfst > (*p).sz {
                memset(
                    (*p).aData.offset((*p).sz as isize) as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (iOfst as sqlite3_int64 - (*p).sz) as size_t,
                );
            }
            (*p).sz = (iOfst as ::core::ffi::c_longlong
                + iAmt as ::core::ffi::c_longlong) as sqlite3_int64;
        }
        memcpy(
            (*p).aData.offset(iOfst as isize) as *mut ::core::ffi::c_void,
            z,
            iAmt as size_t,
        );
        memdbLeave(p);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memdbTruncate(
    mut pFile: *mut sqlite3_file,
    mut size: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemStore = (*(pFile as *mut MemFile)).pStore;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        memdbEnter(p);
        if size > (*p).sz {
            rc = SQLITE_CORRUPT;
        } else {
            (*p).sz = size as sqlite3_int64;
        }
        memdbLeave(p);
        return rc;
    }
}
unsafe extern "C" fn memdbSync(
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memdbFileSize(
    mut pFile: *mut sqlite3_file,
    mut pSize: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemStore = (*(pFile as *mut MemFile)).pStore;
        memdbEnter(p);
        *pSize = (*p).sz as sqlite_int64;
        memdbLeave(p);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memdbLock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pThis: *mut MemFile = pFile as *mut MemFile;
        let mut p: *mut MemStore = (*pThis).pStore;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if eLock <= (*pThis).eLock {
            return SQLITE_OK;
        }
        memdbEnter(p);
        if eLock > SQLITE_LOCK_SHARED
            && (*p).mFlags & SQLITE_DESERIALIZE_READONLY as ::core::ffi::c_uint != 0
        {
            rc = SQLITE_READONLY;
        } else {
            match eLock {
                SQLITE_LOCK_SHARED => {
                    if (*p).nWrLock > 0 as ::core::ffi::c_int {
                        rc = SQLITE_BUSY;
                    } else {
                        (*p).nRdLock += 1;
                    }
                }
                SQLITE_LOCK_RESERVED | SQLITE_LOCK_PENDING => {
                    if (*pThis).eLock == 1 as ::core::ffi::c_int {
                        if (*p).nWrLock > 0 as ::core::ffi::c_int {
                            rc = SQLITE_BUSY;
                        } else {
                            (*p).nWrLock = 1 as ::core::ffi::c_int;
                        }
                    }
                }
                _ => {
                    if (*p).nRdLock > 1 as ::core::ffi::c_int {
                        rc = SQLITE_BUSY;
                    } else if (*pThis).eLock == SQLITE_LOCK_SHARED {
                        (*p).nWrLock = 1 as ::core::ffi::c_int;
                    }
                }
            }
        }
        if rc == SQLITE_OK {
            (*pThis).eLock = eLock;
        }
        memdbLeave(p);
        return rc;
    }
}
unsafe extern "C" fn memdbUnlock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pThis: *mut MemFile = pFile as *mut MemFile;
        let mut p: *mut MemStore = (*pThis).pStore;
        if eLock >= (*pThis).eLock {
            return SQLITE_OK;
        }
        memdbEnter(p);
        if eLock == SQLITE_LOCK_SHARED {
            if (*pThis).eLock > 1 as ::core::ffi::c_int {
                (*p).nWrLock -= 1;
            }
        } else {
            if (*pThis).eLock > SQLITE_LOCK_SHARED {
                (*p).nWrLock -= 1;
            }
            (*p).nRdLock -= 1;
        }
        (*pThis).eLock = eLock;
        memdbLeave(p);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memdbFileControl(
    mut pFile: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemStore = (*(pFile as *mut MemFile)).pStore;
        let mut rc: ::core::ffi::c_int = SQLITE_NOTFOUND;
        memdbEnter(p);
        if op == SQLITE_FCNTL_VFSNAME {
            let ref mut c2rust_fresh0 = *(pArg as *mut *mut ::core::ffi::c_char);
            *c2rust_fresh0 = sqlite3_mprintf(
                b"memdb(%p,%lld)\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).aData,
                (*p).sz,
            );
            rc = SQLITE_OK;
        }
        if op == SQLITE_FCNTL_SIZE_LIMIT {
            let mut iLimit: sqlite3_int64 = *(pArg as *mut sqlite3_int64);
            if iLimit < (*p).sz {
                if iLimit < 0 as ::core::ffi::c_longlong {
                    iLimit = (*p).szMax;
                } else {
                    iLimit = (*p).sz;
                }
            }
            (*p).szMax = iLimit;
            *(pArg as *mut sqlite3_int64) = iLimit;
            rc = SQLITE_OK;
        }
        memdbLeave(p);
        return rc;
    }
}
unsafe extern "C" fn memdbDeviceCharacteristics(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_IOCAP_ATOMIC | SQLITE_IOCAP_POWERSAFE_OVERWRITE
            | SQLITE_IOCAP_SAFE_APPEND | SQLITE_IOCAP_SEQUENTIAL;
    }
}
unsafe extern "C" fn memdbFetch(
    mut pFile: *mut sqlite3_file,
    mut iOfst: sqlite3_int64,
    mut iAmt: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemStore = (*(pFile as *mut MemFile)).pStore;
        memdbEnter(p);
        if iOfst as ::core::ffi::c_longlong + iAmt as ::core::ffi::c_longlong > (*p).sz
            || (*p).mFlags & SQLITE_DESERIALIZE_RESIZEABLE as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
        {
            *pp = ::core::ptr::null_mut::<::core::ffi::c_void>();
        } else {
            (*p).nMmap += 1;
            *pp = (*p).aData.offset(iOfst as isize) as *mut ::core::ffi::c_void;
        }
        memdbLeave(p);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memdbUnfetch(
    mut pFile: *mut sqlite3_file,
    mut iOfst: sqlite3_int64,
    mut pPage: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemStore = (*(pFile as *mut MemFile)).pStore;
        memdbEnter(p);
        (*p).nMmap -= 1;
        memdbLeave(p);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memdbOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pFd: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFile: *mut MemFile = pFd as *mut MemFile;
        let mut p: *mut MemStore = ::core::ptr::null_mut::<MemStore>();
        let mut szName: ::core::ffi::c_int = 0;
        memset(
            pFile as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<MemFile>() as size_t,
        );
        szName = sqlite3Strlen30(zName);
        if szName > 1 as ::core::ffi::c_int
            && (*zName.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '/' as i32
                || *zName.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\\' as i32)
        {
            let mut i: ::core::ffi::c_int = 0;
            let mut pVfsMutex: *mut sqlite3_mutex = sqlite3MutexAlloc(
                SQLITE_MUTEX_STATIC_VFS1,
            );
            sqlite3_mutex_enter(pVfsMutex);
            i = 0 as ::core::ffi::c_int;
            while i < memdb_g.nMemStore {
                if strcmp((**memdb_g.apMemStore.offset(i as isize)).zFName, zName)
                    == 0 as ::core::ffi::c_int
                {
                    p = *memdb_g.apMemStore.offset(i as isize);
                    break;
                } else {
                    i += 1;
                }
            }
            if p.is_null() {
                let mut apNew: *mut *mut MemStore = ::core::ptr::null_mut::<
                    *mut MemStore,
                >();
                p = sqlite3Malloc(
                    (::core::mem::size_of::<MemStore>() as u64_0)
                        .wrapping_add(szName as i64_0 as u64_0)
                        .wrapping_add(3 as u64_0),
                ) as *mut MemStore;
                if p.is_null() {
                    sqlite3_mutex_leave(pVfsMutex);
                    return SQLITE_NOMEM;
                }
                apNew = sqlite3Realloc(
                    memdb_g.apMemStore as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<*mut MemStore>() as u64_0)
                        .wrapping_mul((1 as i64_0 + memdb_g.nMemStore as i64_0) as u64_0),
                ) as *mut *mut MemStore;
                if apNew.is_null() {
                    sqlite3_free(p as *mut ::core::ffi::c_void);
                    sqlite3_mutex_leave(pVfsMutex);
                    return SQLITE_NOMEM;
                }
                let c2rust_fresh2 = memdb_g.nMemStore;
                memdb_g.nMemStore = memdb_g.nMemStore + 1;
                let ref mut c2rust_fresh3 = *apNew.offset(c2rust_fresh2 as isize);
                *c2rust_fresh3 = p;
                memdb_g.apMemStore = apNew;
                memset(
                    p as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<MemStore>() as size_t,
                );
                (*p).mFlags = (SQLITE_DESERIALIZE_RESIZEABLE
                    | SQLITE_DESERIALIZE_FREEONCLOSE) as ::core::ffi::c_uint;
                (*p).szMax = sqlite3Config.mxMemdbSize;
                (*p).zFName = p.offset(1 as ::core::ffi::c_int as isize) as *mut MemStore
                    as *mut ::core::ffi::c_char;
                memcpy(
                    (*p).zFName as *mut ::core::ffi::c_void,
                    zName as *const ::core::ffi::c_void,
                    (szName + 1 as ::core::ffi::c_int) as size_t,
                );
                (*p).pMutex = sqlite3_mutex_alloc(SQLITE_MUTEX_FAST);
                if (*p).pMutex.is_null() {
                    memdb_g.nMemStore -= 1;
                    sqlite3_free(p as *mut ::core::ffi::c_void);
                    sqlite3_mutex_leave(pVfsMutex);
                    return SQLITE_NOMEM;
                }
                (*p).nRef = 1 as ::core::ffi::c_int;
                memdbEnter(p);
            } else {
                memdbEnter(p);
                (*p).nRef += 1;
            }
            sqlite3_mutex_leave(pVfsMutex);
        } else {
            p = sqlite3Malloc(::core::mem::size_of::<MemStore>() as u64_0)
                as *mut MemStore;
            if p.is_null() {
                return SQLITE_NOMEM;
            }
            memset(
                p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<MemStore>() as size_t,
            );
            (*p).mFlags = (SQLITE_DESERIALIZE_RESIZEABLE
                | SQLITE_DESERIALIZE_FREEONCLOSE) as ::core::ffi::c_uint;
            (*p).szMax = sqlite3Config.mxMemdbSize;
        }
        (*pFile).pStore = p;
        if !pOutFlags.is_null() {
            *pOutFlags = flags | SQLITE_OPEN_MEMORY;
        }
        (*pFd).pMethods = &raw const memdb_io_methods as *const sqlite3_io_methods;
        memdbLeave(p);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memdbAccess(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        *pResOut = 0 as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memdbFullPathname(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut nOut: ::core::ffi::c_int,
    mut zOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_snprintf(
            nOut,
            zOut,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            zPath,
        );
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memdbDlOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xDlOpen
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, zPath);
    }
}
unsafe extern "C" fn memdbDlError(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zErrMsg: *mut ::core::ffi::c_char,
) {
    unsafe {
        (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xDlError
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, nByte, zErrMsg);
    }
}
unsafe extern "C" fn memdbDlSym(
    mut pVfs: *mut sqlite3_vfs,
    mut p: *mut ::core::ffi::c_void,
    mut zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xDlSym
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, p, zSym);
    }
}
unsafe extern "C" fn memdbDlClose(
    mut pVfs: *mut sqlite3_vfs,
    mut pHandle: *mut ::core::ffi::c_void,
) {
    unsafe {
        (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xDlClose
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, pHandle);
    }
}
unsafe extern "C" fn memdbRandomness(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xRandomness
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, nByte, zBufOut);
    }
}
unsafe extern "C" fn memdbSleep(
    mut pVfs: *mut sqlite3_vfs,
    mut nMicro: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xSleep
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, nMicro);
    }
}
unsafe extern "C" fn memdbGetLastError(
    mut pVfs: *mut sqlite3_vfs,
    mut a: ::core::ffi::c_int,
    mut b: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xGetLastError
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, a, b);
    }
}
unsafe extern "C" fn memdbCurrentTimeInt64(
    mut pVfs: *mut sqlite3_vfs,
    mut p: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xCurrentTimeInt64
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, p);
    }
}
unsafe extern "C" fn memdbFromDbSchema(
    mut db: *mut sqlite3,
    mut zSchema: *const ::core::ffi::c_char,
) -> *mut MemFile {
    unsafe {
        let mut p: *mut MemFile = ::core::ptr::null_mut::<MemFile>();
        let mut pStore: *mut MemStore = ::core::ptr::null_mut::<MemStore>();
        let mut rc: ::core::ffi::c_int = sqlite3_file_control(
            db,
            zSchema,
            SQLITE_FCNTL_FILE_POINTER,
            &raw mut p as *mut ::core::ffi::c_void,
        );
        if rc != 0 {
            return ::core::ptr::null_mut::<MemFile>();
        }
        if (*p).base.pMethods != &raw const memdb_io_methods {
            return ::core::ptr::null_mut::<MemFile>();
        }
        pStore = (*p).pStore;
        memdbEnter(pStore);
        if !(*pStore).zFName.is_null() {
            p = ::core::ptr::null_mut::<MemFile>();
        }
        memdbLeave(pStore);
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_serialize(
    mut db: *mut sqlite3,
    mut zSchema: *const ::core::ffi::c_char,
    mut piSize: *mut sqlite3_int64,
    mut mFlags: ::core::ffi::c_uint,
) -> *mut ::core::ffi::c_uchar {
    unsafe {
        let mut p: *mut MemFile = ::core::ptr::null_mut::<MemFile>();
        let mut iDb: ::core::ffi::c_int = 0;
        let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
        let mut sz: sqlite3_int64 = 0;
        let mut szPage: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut pOut: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        if zSchema.is_null() {
            zSchema = (*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).zDbSName;
        }
        p = memdbFromDbSchema(db, zSchema);
        iDb = sqlite3FindDbName(db, zSchema);
        if !piSize.is_null() {
            *piSize = -1 as ::core::ffi::c_int as sqlite3_int64;
        }
        if iDb < 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        if !p.is_null() {
            let mut pStore: *mut MemStore = (*p).pStore;
            if !piSize.is_null() {
                *piSize = (*pStore).sz;
            }
            if mFlags & SQLITE_SERIALIZE_NOCOPY as ::core::ffi::c_uint != 0 {
                pOut = (*pStore).aData;
            } else {
                pOut = sqlite3_malloc64((*pStore).sz as sqlite3_uint64)
                    as *mut ::core::ffi::c_uchar;
                if !pOut.is_null() {
                    memcpy(
                        pOut as *mut ::core::ffi::c_void,
                        (*pStore).aData as *const ::core::ffi::c_void,
                        (*pStore).sz as size_t,
                    );
                }
            }
            return pOut;
        }
        pBt = (*(*db).aDb.offset(iDb as isize)).pBt;
        if pBt.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        szPage = sqlite3BtreeGetPageSize(pBt);
        zSql = sqlite3_mprintf(
            b"PRAGMA \"%w\".page_count\0".as_ptr() as *const ::core::ffi::c_char,
            zSchema,
        );
        rc = if !zSql.is_null() {
            sqlite3_prepare_v2(
                db,
                zSql,
                -1 as ::core::ffi::c_int,
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            )
        } else {
            SQLITE_NOMEM
        };
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        if rc != 0 {
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        rc = sqlite3_step(pStmt);
        if rc != SQLITE_ROW {
            pOut = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        } else {
            sz = (sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int)
                as ::core::ffi::c_longlong * szPage as ::core::ffi::c_longlong)
                as sqlite3_int64;
            if sz == 0 as ::core::ffi::c_longlong {
                sqlite3_reset(pStmt);
                sqlite3_exec(
                    db,
                    b"BEGIN IMMEDIATE; COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                rc = sqlite3_step(pStmt);
                if rc == SQLITE_ROW {
                    sz = (sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_longlong * szPage as ::core::ffi::c_longlong)
                        as sqlite3_int64;
                }
            }
            if !piSize.is_null() {
                *piSize = sz;
            }
            if mFlags & SQLITE_SERIALIZE_NOCOPY as ::core::ffi::c_uint != 0 {
                pOut = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            } else {
                pOut = sqlite3_malloc64(sz as sqlite3_uint64)
                    as *mut ::core::ffi::c_uchar;
                if !pOut.is_null() {
                    let mut nPage: ::core::ffi::c_int = sqlite3_column_int(
                        pStmt,
                        0 as ::core::ffi::c_int,
                    );
                    let mut pPager: *mut Pager = sqlite3BtreePager(pBt) as *mut Pager;
                    let mut pgno: ::core::ffi::c_int = 0;
                    pgno = 1 as ::core::ffi::c_int;
                    while pgno <= nPage {
                        let mut pPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
                        let mut pTo: *mut ::core::ffi::c_uchar = pOut
                            .offset(
                                (szPage as sqlite3_int64
                                    * (pgno - 1 as ::core::ffi::c_int) as sqlite3_int64)
                                    as isize,
                            );
                        rc = sqlite3PagerGet(
                            pPager,
                            pgno as Pgno,
                            &raw mut pPage,
                            0 as ::core::ffi::c_int,
                        );
                        if rc == SQLITE_OK {
                            memcpy(
                                pTo as *mut ::core::ffi::c_void,
                                sqlite3PagerGetData(pPage),
                                szPage as size_t,
                            );
                        } else {
                            memset(
                                pTo as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                szPage as size_t,
                            );
                        }
                        sqlite3PagerUnref(pPage);
                        pgno += 1;
                    }
                }
            }
        }
        sqlite3_finalize(pStmt);
        return pOut;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_deserialize(
    mut db: *mut sqlite3,
    mut zSchema: *const ::core::ffi::c_char,
    mut pData: *mut ::core::ffi::c_uchar,
    mut szDb: sqlite3_int64,
    mut szBuf: sqlite3_int64,
    mut mFlags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemFile = ::core::ptr::null_mut::<MemFile>();
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut iDb: ::core::ffi::c_int = 0;
        sqlite3_mutex_enter((*db).mutex);
        if zSchema.is_null() {
            zSchema = (*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).zDbSName;
        }
        iDb = sqlite3FindDbName(db, zSchema);
        if iDb < 2 as ::core::ffi::c_int && iDb != 0 as ::core::ffi::c_int {
            rc = SQLITE_ERROR;
        } else {
            zSql = sqlite3_mprintf(
                b"ATTACH x AS %Q\0".as_ptr() as *const ::core::ffi::c_char,
                zSchema,
            );
            if zSql.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                rc = sqlite3_prepare_v2(
                    db,
                    zSql,
                    -1 as ::core::ffi::c_int,
                    &raw mut pStmt,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                sqlite3_free(zSql as *mut ::core::ffi::c_void);
            }
            if !(rc != 0) {
                (*db).init.iDb = iDb as u8_0;
                (*db)
                    .init
                    .set_reopenMemdb(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                rc = sqlite3_step(pStmt);
                (*db)
                    .init
                    .set_reopenMemdb(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                if rc != SQLITE_DONE {
                    rc = SQLITE_ERROR;
                } else {
                    p = memdbFromDbSchema(db, zSchema);
                    if p.is_null() {
                        rc = SQLITE_ERROR;
                    } else {
                        let mut pStore: *mut MemStore = (*p).pStore;
                        (*pStore).aData = pData;
                        pData = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        (*pStore).sz = szDb;
                        (*pStore).szAlloc = szBuf;
                        (*pStore).szMax = szBuf;
                        if (*pStore).szMax < sqlite3Config.mxMemdbSize {
                            (*pStore).szMax = sqlite3Config.mxMemdbSize;
                        }
                        (*pStore).mFlags = mFlags;
                        rc = SQLITE_OK;
                    }
                }
            }
        }
        sqlite3_finalize(pStmt);
        if !pData.is_null()
            && mFlags & SQLITE_DESERIALIZE_FREEONCLOSE as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
        {
            sqlite3_free(pData as *mut ::core::ffi::c_void);
        }
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IsMemdb(
    mut pVfs: *const sqlite3_vfs,
) -> ::core::ffi::c_int {
    unsafe {
        return (pVfs == &raw mut memdb_vfs as *const sqlite3_vfs) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MemdbInit() -> ::core::ffi::c_int {
    unsafe {
        let mut pLower: *mut sqlite3_vfs = sqlite3_vfs_find(
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        let mut sz: ::core::ffi::c_uint = 0;
        if pLower.is_null() {
            return SQLITE_ERROR;
        }
        sz = (*pLower).szOsFile as ::core::ffi::c_uint;
        memdb_vfs.pAppData = pLower as *mut ::core::ffi::c_void;
        if (sz as usize) < ::core::mem::size_of::<MemFile>() as usize {
            sz = ::core::mem::size_of::<MemFile>() as ::core::ffi::c_uint;
        }
        memdb_vfs.szOsFile = sz as ::core::ffi::c_int;
        return sqlite3_vfs_register(&raw mut memdb_vfs, 0 as ::core::ffi::c_int);
    }
}
