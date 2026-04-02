use c2rust_bitfields::BitfieldStruct;
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
    pub type CheckOnCtx;
    pub type CoveringIndexCheck;
    pub type RenameCtx;
    pub type WhereConst;
    pub type WindowRewrite;
    pub type IdxCover;
    pub type RefSrcList;
    pub type CCurHint;
    pub type Pager;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_free_filename(_: sqlite3_filename);
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_error_code(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
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
    fn sqlite3PagerLockingMode(
        _: *mut Pager,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeOpen(
        pVfs: *mut sqlite3_vfs,
        zFilename: *const ::core::ffi::c_char,
        db: *mut sqlite3,
        ppBtree: *mut *mut Btree,
        flags: ::core::ffi::c_int,
        vfsFlags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeClose(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeSetPagerFlags(
        _: *mut Btree,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeSecureDelete(
        _: *mut Btree,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeTxnState(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeIsInBackup(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeEnter(_: *mut Btree);
    fn sqlite3BtreeEnterAll(_: *mut sqlite3);
    fn sqlite3BtreeLeave(_: *mut Btree);
    fn sqlite3BtreeLeaveAll(_: *mut sqlite3);
    fn sqlite3VdbeAddOp1(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddFunctionCall(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const FuncDef,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3WalkExpr(_: *mut Walker, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3WalkExprList(_: *mut Walker, _: *mut ExprList) -> ::core::ffi::c_int;
    fn sqlite3WalkSelect(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3WalkWinDefnDummyCallback(_: *mut Walker, _: *mut Select);
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbRealloc(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: u64_0,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3GetTempRange(_: *mut Parse, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3ExprDelete(_: *mut sqlite3, _: *mut Expr);
    fn sqlite3Init(
        _: *mut sqlite3,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ResetAllSchemasOfConnection(_: *mut sqlite3);
    fn sqlite3CollapseDatabaseArray(_: *mut sqlite3);
    fn sqlite3ParseUri(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_uint,
        _: *mut *mut sqlite3_vfs,
        _: *mut *mut ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCode(_: *mut Parse, _: *mut Expr, _: ::core::ffi::c_int);
    fn sqlite3GetVdbe(_: *mut Parse) -> *mut Vdbe;
    fn sqlite3AuthCheck(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ReadSchema(pParse: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3ResolveExprNames(_: *mut NameContext, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3FindDbName(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3SchemaGet(_: *mut sqlite3, _: *mut Btree) -> *mut Schema;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbFixer {
    pub pParse: *mut Parse,
    pub w: Walker,
    pub pSchema: *mut Schema,
    pub bTemp: u8_0,
    pub zDb: *const ::core::ffi::c_char,
    pub zType: *const ::core::ffi::c_char,
    pub pName: *const Token

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Walker {
    pub pParse: *mut Parse,
    pub xExprCallback: Option<
        unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
    >,
    pub xSelectCallback: Option<
        unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
    >,
    pub xSelectCallback2: Option<unsafe extern "C" fn(*mut Walker, *mut Select) -> ()>,
    pub walkerDepth: ::core::ffi::c_int,
    pub eCode: u16_0,
    pub mWFlags: u16_0,
    pub u: C2Rust_Unnamed_22

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_22 {
    pub pNC: *mut NameContext,
    pub n: ::core::ffi::c_int,
    pub iCur: ::core::ffi::c_int,
    pub pSrcList: *mut SrcList,
    pub pCCurHint: *mut CCurHint,
    pub pRefSrcList: *mut RefSrcList,
    pub aiCol: *mut ::core::ffi::c_int,
    pub pIdxCover: *mut IdxCover,
    pub pGroupBy: *mut ExprList,
    pub pSelect: *mut Select,
    pub pRewrite: *mut WindowRewrite,
    pub pConst: *mut WhereConst,
    pub pRename: *mut RenameCtx,
    pub pTab: *mut Table,
    pub pCovIdxCk: *mut CoveringIndexCheck,
    pub pSrcItem: *mut SrcItem,
    pub pFix: *mut DbFixer,
    pub aMem: *mut Mem,
    pub pCheckOnCtx: *mut CheckOnCtx

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameContext {
    pub pParse: *mut Parse,
    pub pSrcList: *mut SrcList,
    pub uNC: C2Rust_Unnamed_23,
    pub pNext: *mut NameContext,
    pub nRef: ::core::ffi::c_int,
    pub nNcErr: ::core::ffi::c_int,
    pub ncFlags: ::core::ffi::c_int,
    pub nNestedSelect: u32_0,
    pub pWinSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_23 {
    pub pEList: *mut ExprList,
    pub pAggInfo: *mut AggInfo,
    pub pUpsert: *mut Upsert,
    pub iBaseReg: ::core::ffi::c_int

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_ATTACH: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const SQLITE_DETACH: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_ATTACHED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_TXN_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OP_Expire: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const PAGER_SYNCHRONOUS_FULL: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
pub const PAGER_FLAGS_MASK: ::core::ffi::c_int = 0x38 as ::core::ffi::c_int;
unsafe extern "C" fn resolveAttachExpr(
    mut pName: *mut NameContext,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !pExpr.is_null() {
            if (*pExpr).op as ::core::ffi::c_int != TK_ID {
                rc = sqlite3ResolveExprNames(pName, pExpr);
            } else {
                (*pExpr).op = TK_STRING as u8_0;
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DbIsNamed(
    mut db: *mut sqlite3,
    mut iDb: ::core::ffi::c_int,
    mut zName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (sqlite3StrICmp((*(*db).aDb.offset(iDb as isize)).zDbSName, zName)
            == 0 as ::core::ffi::c_int
            || iDb == 0 as ::core::ffi::c_int
                && sqlite3StrICmp(
                    b"main\0".as_ptr() as *const ::core::ffi::c_char,
                    zName,
                ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn attachFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut zName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zFile: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut flags: ::core::ffi::c_uint = 0;
        let mut aNew: *mut Db = ::core::ptr::null_mut::<Db>();
        let mut pNew: *mut Db = ::core::ptr::null_mut::<Db>();
        let mut zErrDyn: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        zFile = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        zName = sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        if zFile.is_null() {
            zFile = b"\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if zName.is_null() {
            zName = b"\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if (*db).init.reopenMemdb() != 0 {
            let mut pNewBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
            pVfs = sqlite3_vfs_find(b"memdb\0".as_ptr() as *const ::core::ffi::c_char);
            if pVfs.is_null() {
                return;
            }
            rc = sqlite3BtreeOpen(
                pVfs,
                b"x\0\0".as_ptr() as *const ::core::ffi::c_char,
                db,
                &raw mut pNewBt,
                0 as ::core::ffi::c_int,
                SQLITE_OPEN_MAIN_DB,
            );
            if rc == SQLITE_OK {
                let mut pNewSchema: *mut Schema = sqlite3SchemaGet(db, pNewBt);
                if !pNewSchema.is_null() {
                    pNew = (*db).aDb.offset((*db).init.iDb as isize) as *mut Db;
                    if !(*pNew).pBt.is_null() {
                        sqlite3BtreeClose((*pNew).pBt);
                    }
                    (*pNew).pBt = pNewBt;
                    (*pNew).pSchema = pNewSchema;
                } else {
                    sqlite3BtreeClose(pNewBt);
                    rc = SQLITE_NOMEM;
                }
            }
            if rc != 0 {
                c2rust_current_block = 5897689976439292641;
            } else {
                c2rust_current_block = 10380409671385728102;
            }
        } else if (*db).nDb
            >= (*db).aLimit[SQLITE_LIMIT_ATTACHED as usize] + 2 as ::core::ffi::c_int
        {
            zErrDyn = sqlite3MPrintf(
                db,
                b"too many attached databases - max %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*db).aLimit[SQLITE_LIMIT_ATTACHED as usize],
            );
            c2rust_current_block = 5897689976439292641;
        } else {
            i = 0 as ::core::ffi::c_int;
            loop {
                if !(i < (*db).nDb) {
                    c2rust_current_block = 5494826135382683477;
                    break;
                }
                if sqlite3DbIsNamed(db, i, zName) != 0 {
                    zErrDyn = sqlite3MPrintf(
                        db,
                        b"database %s is already in use\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        zName,
                    );
                    c2rust_current_block = 5897689976439292641;
                    break;
                } else {
                    i += 1;
                }
            }
            match c2rust_current_block {
                5897689976439292641 => {}
                _ => {
                    if (*db).aDb == &raw mut (*db).aDbStatic as *mut Db {
                        aNew = sqlite3DbMallocRawNN(
                            db,
                            (::core::mem::size_of::<Db>() as usize)
                                .wrapping_mul(3 as usize) as u64_0,
                        ) as *mut Db;
                        if aNew.is_null() {
                            return;
                        }
                        memcpy(
                            aNew as *mut ::core::ffi::c_void,
                            (*db).aDb as *const ::core::ffi::c_void,
                            (::core::mem::size_of::<Db>() as size_t)
                                .wrapping_mul(2 as size_t),
                        );
                    } else {
                        aNew = sqlite3DbRealloc(
                            db,
                            (*db).aDb as *mut ::core::ffi::c_void,
                            (::core::mem::size_of::<Db>() as u64_0)
                                .wrapping_mul((1 as i64_0 + (*db).nDb as i64_0) as u64_0),
                        ) as *mut Db;
                        if aNew.is_null() {
                            return;
                        }
                    }
                    (*db).aDb = aNew;
                    pNew = (*db).aDb.offset((*db).nDb as isize) as *mut Db;
                    memset(
                        pNew as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<Db>() as size_t,
                    );
                    flags = (*db).openFlags;
                    rc = sqlite3ParseUri(
                        (*(*db).pVfs).zName,
                        zFile,
                        &raw mut flags,
                        &raw mut pVfs,
                        &raw mut zPath,
                        &raw mut zErr,
                    );
                    if rc != SQLITE_OK {
                        if rc == SQLITE_NOMEM {
                            sqlite3OomFault(db);
                        }
                        sqlite3_result_error(context, zErr, -1 as ::core::ffi::c_int);
                        sqlite3_free(zErr as *mut ::core::ffi::c_void);
                        return;
                    }
                    if (*db).flags & SQLITE_AttachWrite == 0 as ::core::ffi::c_ulonglong
                    {
                        flags
                            &= !(SQLITE_OPEN_CREATE | SQLITE_OPEN_READWRITE)
                                as ::core::ffi::c_uint;
                        flags |= SQLITE_OPEN_READONLY as ::core::ffi::c_uint;
                    } else if (*db).flags & SQLITE_AttachCreate
                        == 0 as ::core::ffi::c_ulonglong
                    {
                        flags &= !SQLITE_OPEN_CREATE as ::core::ffi::c_uint;
                    }
                    flags |= SQLITE_OPEN_MAIN_DB as ::core::ffi::c_uint;
                    rc = sqlite3BtreeOpen(
                        pVfs,
                        zPath,
                        db,
                        &raw mut (*pNew).pBt,
                        0 as ::core::ffi::c_int,
                        flags as ::core::ffi::c_int,
                    );
                    (*db).nDb += 1;
                    (*pNew).zDbSName = sqlite3DbStrDup(db, zName);
                    c2rust_current_block = 10380409671385728102;
                }
            }
        }
        match c2rust_current_block {
            10380409671385728102 => {
                (*db).noSharedCache = 0 as u8_0;
                if rc == SQLITE_CONSTRAINT {
                    rc = SQLITE_ERROR;
                    zErrDyn = sqlite3MPrintf(
                        db,
                        b"database is already attached\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else if rc == SQLITE_OK {
                    let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
                    (*pNew).pSchema = sqlite3SchemaGet(db, (*pNew).pBt);
                    if (*pNew).pSchema.is_null() {
                        rc = SQLITE_NOMEM_BKPT;
                    } else if (*(*pNew).pSchema).file_format as ::core::ffi::c_int != 0
                        && (*(*pNew).pSchema).enc as ::core::ffi::c_int
                            != (*db).enc as ::core::ffi::c_int
                    {
                        zErrDyn = sqlite3MPrintf(
                            db,
                            b"attached databases must use the same text encoding as main database\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        rc = SQLITE_ERROR;
                    }
                    sqlite3BtreeEnter((*pNew).pBt);
                    pPager = sqlite3BtreePager((*pNew).pBt) as *mut Pager;
                    sqlite3PagerLockingMode(
                        pPager,
                        (*db).dfltLockMode as ::core::ffi::c_int,
                    );
                    sqlite3BtreeSecureDelete(
                        (*pNew).pBt,
                        sqlite3BtreeSecureDelete(
                            (*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).pBt,
                            -1 as ::core::ffi::c_int,
                        ),
                    );
                    sqlite3BtreeSetPagerFlags(
                        (*pNew).pBt,
                        (PAGER_SYNCHRONOUS_FULL as ::core::ffi::c_ulonglong
                            | (*db).flags as ::core::ffi::c_ulonglong
                                & PAGER_FLAGS_MASK as ::core::ffi::c_ulonglong)
                            as ::core::ffi::c_uint,
                    );
                    sqlite3BtreeLeave((*pNew).pBt);
                }
                (*pNew).safety_level = (SQLITE_DEFAULT_SYNCHRONOUS
                    + 1 as ::core::ffi::c_int) as u8_0;
                if rc == SQLITE_OK && (*pNew).zDbSName.is_null() {
                    rc = SQLITE_NOMEM_BKPT;
                }
                sqlite3_free_filename(zPath as sqlite3_filename);
                if rc == SQLITE_OK {
                    sqlite3BtreeEnterAll(db);
                    (*db).init.iDb = 0 as u8_0;
                    (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
                        & !(0x10 as ::core::ffi::c_int) as ::core::ffi::c_uint) as u32_0;
                    if (*db).init.reopenMemdb() == 0 {
                        rc = sqlite3Init(db, &raw mut zErrDyn);
                    }
                    sqlite3BtreeLeaveAll(db);
                }
                if rc != 0 {
                    if (*db).init.reopenMemdb() == 0 {
                        let mut iDb: ::core::ffi::c_int = (*db).nDb
                            - 1 as ::core::ffi::c_int;
                        if !(*(*db).aDb.offset(iDb as isize)).pBt.is_null() {
                            sqlite3BtreeClose((*(*db).aDb.offset(iDb as isize)).pBt);
                            let ref mut c2rust_fresh0 = (*(*db).aDb.offset(iDb as isize))
                                .pBt;
                            *c2rust_fresh0 = ::core::ptr::null_mut::<Btree>();
                            let ref mut c2rust_fresh1 = (*(*db).aDb.offset(iDb as isize))
                                .pSchema;
                            *c2rust_fresh1 = ::core::ptr::null_mut::<Schema>();
                        }
                        sqlite3ResetAllSchemasOfConnection(db);
                        (*db).nDb = iDb;
                        if rc == SQLITE_NOMEM || rc == SQLITE_IOERR_NOMEM {
                            sqlite3OomFault(db);
                            sqlite3DbFree(db, zErrDyn as *mut ::core::ffi::c_void);
                            zErrDyn = sqlite3MPrintf(
                                db,
                                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        } else if zErrDyn.is_null() {
                            zErrDyn = sqlite3MPrintf(
                                db,
                                b"unable to open database: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                zFile,
                            );
                        }
                    }
                } else {
                    return
                }
            }
            _ => {}
        }
        if !zErrDyn.is_null() {
            sqlite3_result_error(context, zErrDyn, -1 as ::core::ffi::c_int);
            sqlite3DbFree(db, zErrDyn as *mut ::core::ffi::c_void);
        }
        if rc != 0 {
            sqlite3_result_error_code(context, rc);
        }
    }
}
unsafe extern "C" fn detachFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zName: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut i: ::core::ffi::c_int = 0;
        let mut pDb: *mut Db = ::core::ptr::null_mut::<Db>();
        let mut pEntry: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut zErr: [::core::ffi::c_char; 128] = [0; 128];
        if zName.is_null() {
            zName = b"\0".as_ptr() as *const ::core::ffi::c_char;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*db).nDb {
            pDb = (*db).aDb.offset(i as isize) as *mut Db;
            if !(*pDb).pBt.is_null() {
                if sqlite3DbIsNamed(db, i, zName) != 0 {
                    break;
                }
            }
            i += 1;
        }
        if i >= (*db).nDb {
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                    as ::core::ffi::c_int,
                &raw mut zErr as *mut ::core::ffi::c_char,
                b"no such database: %s\0".as_ptr() as *const ::core::ffi::c_char,
                zName,
            );
        } else if i < 2 as ::core::ffi::c_int {
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                    as ::core::ffi::c_int,
                &raw mut zErr as *mut ::core::ffi::c_char,
                b"cannot detach database %s\0".as_ptr() as *const ::core::ffi::c_char,
                zName,
            );
        } else if sqlite3BtreeTxnState((*pDb).pBt) != SQLITE_TXN_NONE
            || sqlite3BtreeIsInBackup((*pDb).pBt) != 0
        {
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                    as ::core::ffi::c_int,
                &raw mut zErr as *mut ::core::ffi::c_char,
                b"database %s is locked\0".as_ptr() as *const ::core::ffi::c_char,
                zName,
            );
        } else {
            pEntry = (*(*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pSchema)
                .trigHash
                .first;
            while !pEntry.is_null() {
                let mut pTrig: *mut Trigger = (*pEntry).data as *mut Trigger;
                if (*pTrig).pTabSchema == (*pDb).pSchema {
                    (*pTrig).pTabSchema = (*pTrig).pSchema;
                }
                pEntry = (*pEntry).next;
            }
            sqlite3BtreeClose((*pDb).pBt);
            (*pDb).pBt = ::core::ptr::null_mut::<Btree>();
            (*pDb).pSchema = ::core::ptr::null_mut::<Schema>();
            sqlite3CollapseDatabaseArray(db);
            return;
        }
        sqlite3_result_error(
            context,
            &raw mut zErr as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn codeAttach(
    mut pParse: *mut Parse,
    mut type_0: ::core::ffi::c_int,
    mut pFunc: *const FuncDef,
    mut pAuthArg: *mut Expr,
    mut pFilename: *mut Expr,
    mut pDbname: *mut Expr,
    mut pKey: *mut Expr,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = 0;
        let mut sName: NameContext = NameContext {
            pParse: ::core::ptr::null_mut::<Parse>(),
            pSrcList: ::core::ptr::null_mut::<SrcList>(),
            uNC: C2Rust_Unnamed_23 {
                pEList: ::core::ptr::null_mut::<ExprList>()

            },
            pNext: ::core::ptr::null_mut::<NameContext>(),
            nRef: 0,
            nNcErr: 0,
            ncFlags: 0,
            nNestedSelect: 0,
            pWinSelect: ::core::ptr::null_mut::<Select>()

        };
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut regArgs: ::core::ffi::c_int = 0;
        if !(SQLITE_OK != sqlite3ReadSchema(pParse)) {
            if !((*pParse).nErr != 0) {
                memset(
                    &raw mut sName as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<NameContext>() as size_t,
                );
                sName.pParse = pParse;
                if !(SQLITE_OK != resolveAttachExpr(&raw mut sName, pFilename)
                    || SQLITE_OK != resolveAttachExpr(&raw mut sName, pDbname)
                    || SQLITE_OK != resolveAttachExpr(&raw mut sName, pKey))
                {
                    if !pAuthArg.is_null() {
                        let mut zAuthArg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        if (*pAuthArg).op as ::core::ffi::c_int == TK_STRING {
                            zAuthArg = (*pAuthArg).u.zToken;
                        } else {
                            zAuthArg = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        rc = sqlite3AuthCheck(
                            pParse,
                            type_0,
                            zAuthArg,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                        if rc != SQLITE_OK {
                            c2rust_current_block = 18295644479810350070;
                        } else {
                            c2rust_current_block = 8236137900636309791;
                        }
                    } else {
                        c2rust_current_block = 8236137900636309791;
                    }
                    match c2rust_current_block {
                        18295644479810350070 => {}
                        _ => {
                            v = sqlite3GetVdbe(pParse);
                            regArgs = sqlite3GetTempRange(
                                pParse,
                                4 as ::core::ffi::c_int,
                            );
                            sqlite3ExprCode(pParse, pFilename, regArgs);
                            sqlite3ExprCode(
                                pParse,
                                pDbname,
                                regArgs + 1 as ::core::ffi::c_int,
                            );
                            sqlite3ExprCode(
                                pParse,
                                pKey,
                                regArgs + 2 as ::core::ffi::c_int,
                            );
                            if !v.is_null() {
                                sqlite3VdbeAddFunctionCall(
                                    pParse,
                                    0 as ::core::ffi::c_int,
                                    regArgs + 3 as ::core::ffi::c_int
                                        - (*pFunc).nArg as ::core::ffi::c_int,
                                    regArgs + 3 as ::core::ffi::c_int,
                                    (*pFunc).nArg as ::core::ffi::c_int,
                                    pFunc,
                                    0 as ::core::ffi::c_int,
                                );
                                sqlite3VdbeAddOp1(
                                    v,
                                    OP_Expire,
                                    (type_0 == SQLITE_ATTACH) as ::core::ffi::c_int,
                                );
                            }
                        }
                    }
                }
            }
        }
        sqlite3ExprDelete(db, pFilename);
        sqlite3ExprDelete(db, pDbname);
        sqlite3ExprDelete(db, pKey);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Detach(mut pParse: *mut Parse, mut pDbname: *mut Expr) {
    unsafe {
        static mut detach_func: FuncDef = unsafe {
            FuncDef {
                nArg: 1 as i16_0,
                funcFlags: SQLITE_UTF8 as u32_0,
                pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                    as *mut ::core::ffi::c_void,
                pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                xSFunc: Some(
                    detachFunc
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                xFinalize: None,
                xValue: None,
                xInverse: None,
                zName: b"sqlite_detach\0".as_ptr() as *const ::core::ffi::c_char,
                u: C2Rust_Unnamed_2 {
                    pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                }

            }
        };
        codeAttach(
            pParse,
            SQLITE_DETACH,
            &raw const detach_func,
            pDbname,
            ::core::ptr::null_mut::<Expr>(),
            ::core::ptr::null_mut::<Expr>(),
            pDbname,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Attach(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
    mut pDbname: *mut Expr,
    mut pKey: *mut Expr,
) {
    unsafe {
        static mut attach_func: FuncDef = unsafe {
            FuncDef {
                nArg: 3 as i16_0,
                funcFlags: SQLITE_UTF8 as u32_0,
                pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                    as *mut ::core::ffi::c_void,
                pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                xSFunc: Some(
                    attachFunc
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                xFinalize: None,
                xValue: None,
                xInverse: None,
                zName: b"sqlite_attach\0".as_ptr() as *const ::core::ffi::c_char,
                u: C2Rust_Unnamed_2 {
                    pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                }

            }
        };
        codeAttach(pParse, SQLITE_ATTACH, &raw const attach_func, p, p, pDbname, pKey);
    }
}
unsafe extern "C" fn fixExprCb(
    mut p: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFix: *mut DbFixer = (*p).u.pFix;
        if (*pFix).bTemp == 0 {
            (*pExpr).flags = ((*pExpr).flags as ::core::ffi::c_uint
                | 0x40000000 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint)
                as u32_0;
        }
        if (*pExpr).op as ::core::ffi::c_int == TK_VARIABLE {
            if (*(*(*pFix).pParse).db).init.busy != 0 {
                (*pExpr).op = TK_NULL as u8_0;
            } else {
                sqlite3ErrorMsg(
                    (*pFix).pParse,
                    b"%s cannot use variables\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pFix).zType,
                );
                return WRC_Abort;
            }
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn fixSelectCb(
    mut p: *mut Walker,
    mut pSelect: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFix: *mut DbFixer = (*p).u.pFix;
        let mut i: ::core::ffi::c_int = 0;
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut db: *mut sqlite3 = (*(*pFix).pParse).db;
        let mut iDb: ::core::ffi::c_int = sqlite3FindDbName(db, (*pFix).zDb);
        let mut pList: *mut SrcList = (*pSelect).pSrc;
        if pList.is_null() {
            return WRC_Continue;
        }
        i = 0 as ::core::ffi::c_int;
        pItem = &raw mut (*pList).a as *mut SrcItem;
        while i < (*pList).nSrc {
            if (*pFix).bTemp as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (*pItem).fg.isSubquery() as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                if (*pItem).fg.fixedSchema() as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int && !(*pItem).u4.zDatabase.is_null()
                {
                    if iDb != sqlite3FindDbName(db, (*pItem).u4.zDatabase) {
                        sqlite3ErrorMsg(
                            (*pFix).pParse,
                            b"%s %T cannot reference objects in database %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*pFix).zType,
                            (*pFix).pName,
                            (*pItem).u4.zDatabase,
                        );
                        return WRC_Abort;
                    }
                    sqlite3DbFree(db, (*pItem).u4.zDatabase as *mut ::core::ffi::c_void);
                    (*pItem)
                        .fg
                        .set_notCte(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*pItem)
                        .fg
                        .set_hadSchema(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                (*pItem).u4.pSchema = (*pFix).pSchema;
                (*pItem).fg.set_fromDDL(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*pItem)
                    .fg
                    .set_fixedSchema(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            if (*(&raw mut (*pList).a as *mut SrcItem).offset(i as isize)).fg.isUsing()
                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && sqlite3WalkExpr(
                    &raw mut (*pFix).w,
                    (*(&raw mut (*pList).a as *mut SrcItem).offset(i as isize)).u3.pOn,
                ) != 0
            {
                return WRC_Abort;
            }
            i += 1;
            pItem = pItem.offset(1);
        }
        if !(*pSelect).pWith.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*(*pSelect).pWith).nCte {
                if sqlite3WalkSelect(
                    p,
                    (*(&raw mut (*(*pSelect).pWith).a as *mut Cte).offset(i as isize))
                        .pSelect,
                ) != 0
                {
                    return WRC_Abort;
                }
                i += 1;
            }
        }
        return WRC_Continue;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FixInit(
    mut pFix: *mut DbFixer,
    mut pParse: *mut Parse,
    mut iDb: ::core::ffi::c_int,
    mut zType: *const ::core::ffi::c_char,
    mut pName: *const Token,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        (*pFix).pParse = pParse;
        (*pFix).zDb = (*(*db).aDb.offset(iDb as isize)).zDbSName;
        (*pFix).pSchema = (*(*db).aDb.offset(iDb as isize)).pSchema;
        (*pFix).zType = zType;
        (*pFix).pName = pName;
        (*pFix).bTemp = (iDb == 1 as ::core::ffi::c_int) as ::core::ffi::c_int as u8_0;
        (*pFix).w.pParse = pParse;
        (*pFix).w.xExprCallback = Some(
            fixExprCb
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        (*pFix).w.xSelectCallback = Some(
            fixSelectCb
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        (*pFix).w.xSelectCallback2 = Some(
            sqlite3WalkWinDefnDummyCallback
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> (),
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Select) -> ()>;
        (*pFix).w.walkerDepth = 0 as ::core::ffi::c_int;
        (*pFix).w.eCode = 0 as u16_0;
        (*pFix).w.u.pFix = pFix;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FixSrcList(
    mut pFix: *mut DbFixer,
    mut pList: *mut SrcList,
) -> ::core::ffi::c_int {
    unsafe {
        let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !pList.is_null() {
            let mut s: Select = Select {
                op: 0,
                nSelectRow: 0,
                selFlags: 0,
                iLimit: 0,
                iOffset: 0,
                selId: 0,
                addrOpenEphm: [0; 2],
                pEList: ::core::ptr::null_mut::<ExprList>(),
                pSrc: ::core::ptr::null_mut::<SrcList>(),
                pWhere: ::core::ptr::null_mut::<Expr>(),
                pGroupBy: ::core::ptr::null_mut::<ExprList>(),
                pHaving: ::core::ptr::null_mut::<Expr>(),
                pOrderBy: ::core::ptr::null_mut::<ExprList>(),
                pPrior: ::core::ptr::null_mut::<Select>(),
                pNext: ::core::ptr::null_mut::<Select>(),
                pLimit: ::core::ptr::null_mut::<Expr>(),
                pWith: ::core::ptr::null_mut::<With>(),
                pWin: ::core::ptr::null_mut::<Window>(),
                pWinDefn: ::core::ptr::null_mut::<Window>()

            };
            memset(
                &raw mut s as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Select>() as size_t,
            );
            s.pSrc = pList;
            res = sqlite3WalkSelect(&raw mut (*pFix).w, &raw mut s);
        }
        return res;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FixSelect(
    mut pFix: *mut DbFixer,
    mut pSelect: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3WalkSelect(&raw mut (*pFix).w, pSelect);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FixExpr(
    mut pFix: *mut DbFixer,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3WalkExpr(&raw mut (*pFix).w, pExpr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FixTriggerStep(
    mut pFix: *mut DbFixer,
    mut pStep: *mut TriggerStep,
) -> ::core::ffi::c_int {
    unsafe {
        while !pStep.is_null() {
            if sqlite3WalkSelect(&raw mut (*pFix).w, (*pStep).pSelect) != 0
                || sqlite3WalkExpr(&raw mut (*pFix).w, (*pStep).pWhere) != 0
                || sqlite3WalkExprList(&raw mut (*pFix).w, (*pStep).pExprList) != 0
                || sqlite3FixSrcList(pFix, (*pStep).pFrom) != 0
            {
                return 1 as ::core::ffi::c_int;
            }
            let mut pUp: *mut Upsert = ::core::ptr::null_mut::<Upsert>();
            pUp = (*pStep).pUpsert;
            while !pUp.is_null() {
                if sqlite3WalkExprList(&raw mut (*pFix).w, (*pUp).pUpsertTarget) != 0
                    || sqlite3WalkExpr(&raw mut (*pFix).w, (*pUp).pUpsertTargetWhere)
                        != 0
                    || sqlite3WalkExprList(&raw mut (*pFix).w, (*pUp).pUpsertSet) != 0
                    || sqlite3WalkExpr(&raw mut (*pFix).w, (*pUp).pUpsertWhere) != 0
                {
                    return 1 as ::core::ffi::c_int;
                }
                pUp = (*pUp).pNextUpsert;
            }
            pStep = (*pStep).pNext;
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub const SQLITE_DEFAULT_SYNCHRONOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_AttachCreate: u64_0 = (0x10 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const SQLITE_AttachWrite: u64_0 = (0x20 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const DBFLAG_SchemaKnownOk: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const WRC_Continue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WRC_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const TK_ID: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const TK_STRING: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_VARIABLE: ::core::ffi::c_int = 157 as ::core::ffi::c_int;
