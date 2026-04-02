use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type RenameToken;
    pub type Vdbe;
    pub type sqlite3_mutex;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_log(
        iErrCode: ::core::ffi::c_int,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3Malloc(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbNNFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3DeleteTable(_: *mut sqlite3, _: *mut Table);
    fn sqlite3DeleteTrigger(_: *mut sqlite3, _: *mut Trigger);
    fn sqlite3ErrStr(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    static sqlite3UpperToLower: [::core::ffi::c_uchar; 0];
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3ParserAlloc(
        _: Option<unsafe extern "C" fn(u64_0) -> *mut ::core::ffi::c_void>,
        _: *mut Parse,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3ParserFree(
        _: *mut ::core::ffi::c_void,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3Parser(_: *mut ::core::ffi::c_void, _: ::core::ffi::c_int, _: Token);
    fn sqlite3ParserFallback(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_INTERRUPT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_SQL_LENGTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_PREPARE_DONT_LOG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_DIGIT_SEPARATOR: ::core::ffi::c_int = '_' as i32;
pub const CC_X: ::core::ffi::c_int = 0;
pub const CC_KYWD0: ::core::ffi::c_int = 1;
pub const CC_KYWD: ::core::ffi::c_int = 2;
pub const CC_DIGIT: ::core::ffi::c_int = 3;
pub const CC_DOLLAR: ::core::ffi::c_int = 4;
pub const CC_VARALPHA: ::core::ffi::c_int = 5;
pub const CC_VARNUM: ::core::ffi::c_int = 6;
pub const CC_SPACE: ::core::ffi::c_int = 7;
pub const CC_QUOTE: ::core::ffi::c_int = 8;
pub const CC_QUOTE2: ::core::ffi::c_int = 9;
pub const CC_PIPE: ::core::ffi::c_int = 10;
pub const CC_MINUS: ::core::ffi::c_int = 11;
pub const CC_LT: ::core::ffi::c_int = 12;
pub const CC_GT: ::core::ffi::c_int = 13;
pub const CC_EQ: ::core::ffi::c_int = 14;
pub const CC_BANG: ::core::ffi::c_int = 15;
pub const CC_SLASH: ::core::ffi::c_int = 16;
pub const CC_LP: ::core::ffi::c_int = 17;
pub const CC_RP: ::core::ffi::c_int = 18;
pub const CC_SEMI: ::core::ffi::c_int = 19;
pub const CC_PLUS: ::core::ffi::c_int = 20;
pub const CC_STAR: ::core::ffi::c_int = 21;
pub const CC_PERCENT: ::core::ffi::c_int = 22;
pub const CC_COMMA: ::core::ffi::c_int = 23;
pub const CC_AND: ::core::ffi::c_int = 24;
pub const CC_TILDA: ::core::ffi::c_int = 25;
pub const CC_DOT: ::core::ffi::c_int = 26;
pub const CC_ID: ::core::ffi::c_int = 27;
pub const CC_NUL: ::core::ffi::c_int = 29;
pub const CC_BOM: ::core::ffi::c_int = 30;
static mut aiClass: [::core::ffi::c_uchar; 256] = [
    29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn sqlite3IsIdChar(mut c: u8_0) -> ::core::ffi::c_int {
    unsafe {
        return (*(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(c as isize) as ::core::ffi::c_int & 0x46 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn getToken(
    mut pz: *mut *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    unsafe {
        let mut z: *const ::core::ffi::c_uchar = *pz;
        let mut t: ::core::ffi::c_int = 0;
        loop {
            z = z.offset(sqlite3GetToken(z, &raw mut t) as isize);
            if !(t == TK_SPACE || t == TK_COMMENT) {
                break;
            }
        }
        if t == TK_ID || t == TK_STRING || t == TK_JOIN_KW || t == TK_WINDOW
            || t == TK_OVER || sqlite3ParserFallback(t) == TK_ID
        {
            t = TK_ID;
        }
        *pz = z;
        return t;
    }
}
unsafe extern "C" fn analyzeWindowKeyword(
    mut z: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    unsafe {
        let mut t: ::core::ffi::c_int = 0;
        t = getToken(&raw mut z);
        if t != TK_ID {
            return TK_ID;
        }
        t = getToken(&raw mut z);
        if t != TK_AS {
            return TK_ID;
        }
        return TK_WINDOW;
    }
}
unsafe extern "C" fn analyzeOverKeyword(
    mut z: *const ::core::ffi::c_uchar,
    mut lastToken: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if lastToken == TK_RP {
            let mut t: ::core::ffi::c_int = getToken(&raw mut z);
            if t == TK_LP || t == TK_ID {
                return TK_OVER;
            }
        }
        return TK_ID;
    }
}
unsafe extern "C" fn analyzeFilterKeyword(
    mut z: *const ::core::ffi::c_uchar,
    mut lastToken: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if lastToken == TK_RP && getToken(&raw mut z) == TK_LP {
            return TK_FILTER;
        }
        return TK_ID;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GetToken(
    mut z: *const ::core::ffi::c_uchar,
    mut tokenType: *mut ::core::ffi::c_int,
) -> i64_0 {
    unsafe {
        let mut i: i64_0 = 0;
        let mut c: ::core::ffi::c_int = 0;
        let mut c2rust_current_block_253: u64;
        match aiClass[*z as usize] as ::core::ffi::c_int {
            CC_SPACE => {
                i = 1 as i64_0;
                while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(*z.offset(i as isize) as isize) as ::core::ffi::c_int
                    & 0x1 as ::core::ffi::c_int != 0
                {
                    i += 1;
                }
                *tokenType = TK_SPACE;
                return i;
            }
            CC_MINUS => {
                if *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '-' as i32
                {
                    i = 2 as i64_0;
                    loop {
                        c = *z.offset(i as isize) as ::core::ffi::c_int;
                        if !(c != 0 as ::core::ffi::c_int && c != '\n' as i32) {
                            break;
                        }
                        i += 1;
                    }
                    *tokenType = TK_COMMENT;
                    return i;
                } else if *z.offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == '>' as i32
                {
                    *tokenType = TK_PTR;
                    return (2 as ::core::ffi::c_int
                        + (*z.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == '>' as i32) as ::core::ffi::c_int)
                        as i64_0;
                }
                *tokenType = TK_MINUS;
                return 1 as i64_0;
            }
            CC_LP => {
                *tokenType = TK_LP;
                return 1 as i64_0;
            }
            CC_RP => {
                *tokenType = TK_RP;
                return 1 as i64_0;
            }
            CC_SEMI => {
                *tokenType = TK_SEMI;
                return 1 as i64_0;
            }
            CC_PLUS => {
                *tokenType = TK_PLUS;
                return 1 as i64_0;
            }
            CC_STAR => {
                *tokenType = TK_STAR;
                return 1 as i64_0;
            }
            CC_SLASH => {
                if *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '*' as i32
                    || *z.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    *tokenType = TK_SLASH;
                    return 1 as i64_0;
                }
                i = 3 as i64_0;
                c = *z.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                while (c != '*' as i32
                    || *z.offset(i as isize) as ::core::ffi::c_int != '/' as i32)
                    && {
                        c = *z.offset(i as isize) as ::core::ffi::c_int;
                        c != 0 as ::core::ffi::c_int
                    }
                {
                    i += 1;
                }
                if c != 0 {
                    i += 1;
                }
                *tokenType = TK_COMMENT;
                return i;
            }
            CC_PERCENT => {
                *tokenType = TK_REM;
                return 1 as i64_0;
            }
            CC_EQ => {
                *tokenType = TK_EQ;
                return (1 as ::core::ffi::c_int
                    + (*z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '=' as i32) as ::core::ffi::c_int) as i64_0;
            }
            CC_LT => {
                c = *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                if c == '=' as i32 {
                    *tokenType = TK_LE;
                    return 2 as i64_0;
                } else if c == '>' as i32 {
                    *tokenType = TK_NE;
                    return 2 as i64_0;
                } else if c == '<' as i32 {
                    *tokenType = TK_LSHIFT;
                    return 2 as i64_0;
                } else {
                    *tokenType = TK_LT;
                    return 1 as i64_0;
                }
            }
            CC_GT => {
                c = *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                if c == '=' as i32 {
                    *tokenType = TK_GE;
                    return 2 as i64_0;
                } else if c == '>' as i32 {
                    *tokenType = TK_RSHIFT;
                    return 2 as i64_0;
                } else {
                    *tokenType = TK_GT;
                    return 1 as i64_0;
                }
            }
            CC_BANG => {
                if *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '=' as i32
                {
                    *tokenType = TK_ILLEGAL;
                    return 1 as i64_0;
                } else {
                    *tokenType = TK_NE;
                    return 2 as i64_0;
                }
            }
            CC_PIPE => {
                if *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '|' as i32
                {
                    *tokenType = TK_BITOR;
                    return 1 as i64_0;
                } else {
                    *tokenType = TK_CONCAT;
                    return 2 as i64_0;
                }
            }
            CC_COMMA => {
                *tokenType = TK_COMMA;
                return 1 as i64_0;
            }
            CC_AND => {
                *tokenType = TK_BITAND;
                return 1 as i64_0;
            }
            CC_TILDA => {
                *tokenType = TK_BITNOT;
                return 1 as i64_0;
            }
            CC_QUOTE => {
                let mut delim: ::core::ffi::c_int = *z
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                i = 1 as i64_0;
                loop {
                    c = *z.offset(i as isize) as ::core::ffi::c_int;
                    if !(c != 0 as ::core::ffi::c_int) {
                        break;
                    }
                    if c == delim {
                        if !(*z
                            .offset(
                                (i as ::core::ffi::c_longlong
                                    + 1 as ::core::ffi::c_longlong) as isize,
                            ) as ::core::ffi::c_int == delim)
                        {
                            break;
                        }
                        i += 1;
                    }
                    i += 1;
                }
                if c == '\'' as i32 {
                    *tokenType = TK_STRING;
                    return i + 1 as i64_0;
                } else if c != 0 as ::core::ffi::c_int {
                    *tokenType = TK_ID;
                    return i + 1 as i64_0;
                } else {
                    *tokenType = TK_ILLEGAL;
                    return i;
                }
            }
            CC_DOT => {
                if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(*z.offset(1 as ::core::ffi::c_int as isize) as isize)
                    as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
                {
                    *tokenType = TK_DOT;
                    return 1 as i64_0;
                }
                c2rust_current_block_253 = 10209126403243722321;
            }
            CC_DIGIT => {
                c2rust_current_block_253 = 10209126403243722321;
            }
            CC_QUOTE2 => {
                i = 1 as i64_0;
                c = *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                while c != ']' as i32
                    && {
                        c = *z.offset(i as isize) as ::core::ffi::c_int;
                        c != 0 as ::core::ffi::c_int
                    }
                {
                    i += 1;
                }
                *tokenType = if c == ']' as i32 { TK_ID } else { TK_ILLEGAL };
                return i;
            }
            CC_VARNUM => {
                *tokenType = TK_VARIABLE;
                i = 1 as i64_0;
                while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(*z.offset(i as isize) as isize) as ::core::ffi::c_int
                    & 0x4 as ::core::ffi::c_int != 0
                {
                    i += 1;
                }
                return i;
            }
            CC_DOLLAR | CC_VARALPHA => {
                let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                *tokenType = TK_VARIABLE;
                i = 1 as i64_0;
                loop {
                    c = *z.offset(i as isize) as ::core::ffi::c_int;
                    if !(c != 0 as ::core::ffi::c_int) {
                        break;
                    }
                    if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(c as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                        & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    {
                        n += 1;
                    } else if c == '(' as i32 && n > 0 as ::core::ffi::c_int {
                        loop {
                            i += 1;
                            c = *z.offset(i as isize) as ::core::ffi::c_int;
                            if !(c != 0 as ::core::ffi::c_int
                                && *(&raw const sqlite3CtypeMap
                                    as *const ::core::ffi::c_uchar)
                                    .offset(c as ::core::ffi::c_uchar as isize)
                                    as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0
                                && c != ')' as i32)
                            {
                                break;
                            }
                        }
                        if c == ')' as i32 {
                            i += 1;
                        } else {
                            *tokenType = TK_ILLEGAL;
                        }
                        break;
                    } else {
                        if !(c == ':' as i32
                            && *z
                                .offset(
                                    (i as ::core::ffi::c_longlong
                                        + 1 as ::core::ffi::c_longlong) as isize,
                                ) as ::core::ffi::c_int == ':' as i32)
                        {
                            break;
                        }
                        i += 1;
                    }
                    i += 1;
                }
                if n == 0 as ::core::ffi::c_int {
                    *tokenType = TK_ILLEGAL;
                }
                return i;
            }
            CC_KYWD0 => {
                if aiClass[*z.offset(1 as ::core::ffi::c_int as isize) as usize]
                    as ::core::ffi::c_int > CC_KYWD
                {
                    i = 1 as i64_0;
                } else {
                    i = 2 as i64_0;
                    while aiClass[*z.offset(i as isize) as usize] as ::core::ffi::c_int
                        <= CC_KYWD
                    {
                        i += 1;
                    }
                    if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(*z.offset(i as isize) as isize) as ::core::ffi::c_int
                        & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    {
                        i += 1;
                    } else {
                        *tokenType = TK_ID;
                        return keywordCode(
                            z as *mut ::core::ffi::c_char,
                            i as ::core::ffi::c_int,
                            tokenType,
                        ) as i64_0;
                    }
                }
                c2rust_current_block_253 = 9239588423676249671;
            }
            CC_X => {
                if *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\'' as i32
                {
                    *tokenType = TK_BLOB;
                    i = 2 as i64_0;
                    while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(*z.offset(i as isize) as isize) as ::core::ffi::c_int
                        & 0x8 as ::core::ffi::c_int != 0
                    {
                        i += 1;
                    }
                    if *z.offset(i as isize) as ::core::ffi::c_int != '\'' as i32
                        || i as ::core::ffi::c_longlong % 2 as ::core::ffi::c_longlong
                            != 0
                    {
                        *tokenType = TK_ILLEGAL;
                        while *z.offset(i as isize) as ::core::ffi::c_int != 0
                            && *z.offset(i as isize) as ::core::ffi::c_int != '\'' as i32
                        {
                            i += 1;
                        }
                    }
                    if *z.offset(i as isize) != 0 {
                        i += 1;
                    }
                    return i;
                }
                c2rust_current_block_253 = 13797367574128857302;
            }
            CC_KYWD | CC_ID => {
                c2rust_current_block_253 = 13797367574128857302;
            }
            CC_BOM => {
                if *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0xbb as ::core::ffi::c_int
                    && *z.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0xbf as ::core::ffi::c_int
                {
                    *tokenType = TK_SPACE;
                    return 3 as i64_0;
                }
                i = 1 as i64_0;
                c2rust_current_block_253 = 9239588423676249671;
            }
            CC_NUL => {
                *tokenType = TK_ILLEGAL;
                return 0 as i64_0;
            }
            _ => {
                *tokenType = TK_ILLEGAL;
                return 1 as i64_0;
            }
        }
        match c2rust_current_block_253 {
            10209126403243722321 => {
                *tokenType = TK_INTEGER;
                if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '0' as i32
                    && (*z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'x' as i32
                        || *z.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 'X' as i32)
                    && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(*z.offset(2 as ::core::ffi::c_int as isize) as isize)
                        as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                {
                    i = 3 as i64_0;
                    loop {
                        if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(*z.offset(i as isize) as isize) as ::core::ffi::c_int
                            & 0x8 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            if !(*z.offset(i as isize) as ::core::ffi::c_int
                                == SQLITE_DIGIT_SEPARATOR)
                            {
                                break;
                            }
                            *tokenType = TK_QNUMBER;
                        }
                        i += 1;
                    }
                } else {
                    i = 0 as i64_0;
                    loop {
                        if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(*z.offset(i as isize) as isize) as ::core::ffi::c_int
                            & 0x4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            if !(*z.offset(i as isize) as ::core::ffi::c_int
                                == SQLITE_DIGIT_SEPARATOR)
                            {
                                break;
                            }
                            *tokenType = TK_QNUMBER;
                        }
                        i += 1;
                    }
                    if *z.offset(i as isize) as ::core::ffi::c_int == '.' as i32 {
                        if *tokenType == TK_INTEGER {
                            *tokenType = TK_FLOAT;
                        }
                        i += 1;
                        loop {
                            if *(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(*z.offset(i as isize) as isize)
                                as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                if !(*z.offset(i as isize) as ::core::ffi::c_int
                                    == SQLITE_DIGIT_SEPARATOR)
                                {
                                    break;
                                }
                                *tokenType = TK_QNUMBER;
                            }
                            i += 1;
                        }
                    }
                    if (*z.offset(i as isize) as ::core::ffi::c_int == 'e' as i32
                        || *z.offset(i as isize) as ::core::ffi::c_int == 'E' as i32)
                        && (*(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *z
                                    .offset(
                                        (i as ::core::ffi::c_longlong
                                            + 1 as ::core::ffi::c_longlong) as isize,
                                    ) as isize,
                            ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                            || (*z
                                .offset(
                                    (i as ::core::ffi::c_longlong
                                        + 1 as ::core::ffi::c_longlong) as isize,
                                ) as ::core::ffi::c_int == '+' as i32
                                || *z
                                    .offset(
                                        (i as ::core::ffi::c_longlong
                                            + 1 as ::core::ffi::c_longlong) as isize,
                                    ) as ::core::ffi::c_int == '-' as i32)
                                && *(&raw const sqlite3CtypeMap
                                    as *const ::core::ffi::c_uchar)
                                    .offset(
                                        *z
                                            .offset(
                                                (i as ::core::ffi::c_longlong
                                                    + 2 as ::core::ffi::c_longlong) as isize,
                                            ) as isize,
                                    ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0)
                    {
                        if *tokenType == TK_INTEGER {
                            *tokenType = TK_FLOAT;
                        }
                        i += 2 as ::core::ffi::c_longlong;
                        loop {
                            if *(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(*z.offset(i as isize) as isize)
                                as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                if !(*z.offset(i as isize) as ::core::ffi::c_int
                                    == SQLITE_DIGIT_SEPARATOR)
                                {
                                    break;
                                }
                                *tokenType = TK_QNUMBER;
                            }
                            i += 1;
                        }
                    }
                }
                while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(*z.offset(i as isize) as isize) as ::core::ffi::c_int
                    & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    *tokenType = TK_ILLEGAL;
                    i += 1;
                }
                return i;
            }
            13797367574128857302 => {
                i = 1 as i64_0;
            }
            _ => {}
        }
        while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(*z.offset(i as isize) as isize) as ::core::ffi::c_int
            & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            i += 1;
        }
        *tokenType = TK_ID;
        return i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RunParser(
    mut pParse: *mut Parse,
    mut zSql: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nErr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pEngine: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut n: i64_0 = 0 as i64_0;
        let mut tokenType: ::core::ffi::c_int = 0;
        let mut lastTokenParsed: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut mxSqlLen: ::core::ffi::c_int = 0;
        let mut pParentParse: *mut Parse = ::core::ptr::null_mut::<Parse>();
        mxSqlLen = (*db).aLimit[SQLITE_LIMIT_SQL_LENGTH as usize];
        if (*db).nVdbeActive == 0 as ::core::ffi::c_int {
            ;
        }
        (*pParse).rc = SQLITE_OK;
        (*pParse).zTail = zSql;
        pEngine = sqlite3ParserAlloc(
            Some(
                sqlite3Malloc as unsafe extern "C" fn(u64_0) -> *mut ::core::ffi::c_void,
            ),
            pParse,
        );
        if pEngine.is_null() {
            sqlite3OomFault(db);
            return SQLITE_NOMEM_BKPT;
        }
        pParentParse = (*db).pParse;
        (*db).pParse = pParse;
        loop {
            n = sqlite3GetToken(zSql as *mut u8_0, &raw mut tokenType);
            mxSqlLen = (mxSqlLen as ::core::ffi::c_longlong
                - n as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            if mxSqlLen < 0 as ::core::ffi::c_int {
                (*pParse).rc = SQLITE_TOOBIG;
                (*pParse).nErr += 1;
                break;
            } else {
                if tokenType >= TK_WINDOW {
                    if (*db).u1.isInterrupted != 0
                    {
                        (*pParse).rc = SQLITE_INTERRUPT;
                        (*pParse).nErr += 1;
                        break;
                    } else if tokenType == TK_SPACE {
                        zSql = zSql.offset(n as isize);
                        continue;
                    } else if *zSql.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        if lastTokenParsed == TK_SEMI {
                            tokenType = 0 as ::core::ffi::c_int;
                        } else {
                            if lastTokenParsed == 0 as ::core::ffi::c_int {
                                break;
                            }
                            tokenType = TK_SEMI;
                        }
                        n = 0 as i64_0;
                    } else if tokenType == TK_WINDOW {
                        tokenType = analyzeWindowKeyword(
                            zSql.offset(6 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_char as *const ::core::ffi::c_uchar,
                        );
                    } else if tokenType == TK_OVER {
                        tokenType = analyzeOverKeyword(
                            zSql.offset(4 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_uchar,
                            lastTokenParsed,
                        );
                    } else if tokenType == TK_FILTER {
                        tokenType = analyzeFilterKeyword(
                            zSql.offset(6 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_uchar,
                            lastTokenParsed,
                        );
                    } else if tokenType == TK_COMMENT
                        && ((*db).init.busy as ::core::ffi::c_int != 0
                            || (*db).flags & SQLITE_Comments
                                != 0 as ::core::ffi::c_ulonglong)
                    {
                        zSql = zSql.offset(n as isize);
                        continue;
                    } else if tokenType != TK_QNUMBER {
                        let mut x: Token = Token {
                            z: ::core::ptr::null::<::core::ffi::c_char>(),
                            n: 0

                        };
                        x.z = zSql;
                        x.n = n as u32_0 as ::core::ffi::c_uint;
                        sqlite3ErrorMsg(
                            pParse,
                            b"unrecognized token: \"%T\"\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut x,
                        );
                        break;
                    }
                }
                (*pParse).sLastToken.z = zSql;
                (*pParse).sLastToken.n = n as u32_0 as ::core::ffi::c_uint;
                sqlite3Parser(pEngine, tokenType, (*pParse).sLastToken);
                lastTokenParsed = tokenType;
                zSql = zSql.offset(n as isize);
                if (*pParse).rc != SQLITE_OK {
                    break;
                }
            }
        }
        sqlite3ParserFree(
            pEngine,
            Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        if (*db).mallocFailed != 0 {
            (*pParse).rc = SQLITE_NOMEM_BKPT;
        }
        if !(*pParse).zErrMsg.is_null()
            || (*pParse).rc != SQLITE_OK && (*pParse).rc != SQLITE_DONE
        {
            if (*pParse).zErrMsg.is_null() {
                (*pParse).zErrMsg = sqlite3MPrintf(
                    db,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3ErrStr((*pParse).rc),
                );
            }
            if (*pParse).prepFlags as ::core::ffi::c_int & SQLITE_PREPARE_DONT_LOG
                == 0 as ::core::ffi::c_int
            {
                sqlite3_log(
                    (*pParse).rc,
                    b"%s in \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pParse).zErrMsg,
                    (*pParse).zTail,
                );
            }
            nErr += 1;
        }
        (*pParse).zTail = zSql;
        sqlite3_free((*pParse).apVtabLock as *mut ::core::ffi::c_void);
        if !(*pParse).pNewTable.is_null()
            && !((*pParse).eParseMode as ::core::ffi::c_int != PARSE_MODE_NORMAL)
        {
            sqlite3DeleteTable(db, (*pParse).pNewTable);
        }
        if !(*pParse).pNewTrigger.is_null()
            && !((*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME)
        {
            sqlite3DeleteTrigger(db, (*pParse).pNewTrigger);
        }
        if !(*pParse).pVList.is_null() {
            sqlite3DbNNFreeNN(db, (*pParse).pVList as *mut ::core::ffi::c_void);
        }
        (*db).pParse = pParentParse;
        return nErr;
    }
}
pub const TK_SEMI: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TK_EXPLAIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TK_QUERY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const TK_PLAN: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TK_BEGIN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const TK_TRANSACTION: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const TK_DEFERRED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const TK_IMMEDIATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const TK_EXCLUSIVE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const TK_COMMIT: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const TK_END: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const TK_ROLLBACK: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const TK_SAVEPOINT: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const TK_RELEASE: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const TK_TO: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const TK_TABLE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const TK_CREATE: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const TK_IF: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const TK_NOT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const TK_EXISTS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const TK_TEMP: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const TK_LP: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const TK_RP: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const TK_AS: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const TK_COMMA: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const TK_WITHOUT: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const TK_ABORT: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const TK_ACTION: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const TK_AFTER: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const TK_ANALYZE: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const TK_ASC: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const TK_ATTACH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const TK_BEFORE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const TK_BY: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const TK_CASCADE: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const TK_CAST: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const TK_CONFLICT: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const TK_DATABASE: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const TK_DESC: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const TK_DETACH: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const TK_EACH: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
pub const TK_FAIL: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const TK_OR: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const TK_AND: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const TK_IS: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const TK_MATCH: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
pub const TK_LIKE_KW: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const TK_BETWEEN: ::core::ffi::c_int = 49 as ::core::ffi::c_int;
pub const TK_IN: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const TK_ISNULL: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const TK_NOTNULL: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const TK_NE: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const TK_EQ: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const TK_GT: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
pub const TK_LE: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const TK_LT: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
pub const TK_GE: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
pub const TK_ESCAPE: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const TK_ID: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const TK_COLUMNKW: ::core::ffi::c_int = 61 as ::core::ffi::c_int;
pub const TK_DO: ::core::ffi::c_int = 62 as ::core::ffi::c_int;
pub const TK_FOR: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
pub const TK_IGNORE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const TK_INITIALLY: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const TK_INSTEAD: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const TK_NO: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
pub const TK_KEY: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const TK_OF: ::core::ffi::c_int = 69 as ::core::ffi::c_int;
pub const TK_OFFSET: ::core::ffi::c_int = 70 as ::core::ffi::c_int;
pub const TK_PRAGMA: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
pub const TK_RAISE: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const TK_RECURSIVE: ::core::ffi::c_int = 73 as ::core::ffi::c_int;
pub const TK_REPLACE: ::core::ffi::c_int = 74 as ::core::ffi::c_int;
pub const TK_RESTRICT: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const TK_ROW: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const TK_ROWS: ::core::ffi::c_int = 77 as ::core::ffi::c_int;
pub const TK_TRIGGER: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
pub const TK_VACUUM: ::core::ffi::c_int = 79 as ::core::ffi::c_int;
pub const TK_VIEW: ::core::ffi::c_int = 80 as ::core::ffi::c_int;
pub const TK_VIRTUAL: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
pub const TK_WITH: ::core::ffi::c_int = 82 as ::core::ffi::c_int;
pub const TK_NULLS: ::core::ffi::c_int = 83 as ::core::ffi::c_int;
pub const TK_FIRST: ::core::ffi::c_int = 84 as ::core::ffi::c_int;
pub const TK_LAST: ::core::ffi::c_int = 85 as ::core::ffi::c_int;
pub const TK_CURRENT: ::core::ffi::c_int = 86 as ::core::ffi::c_int;
pub const TK_FOLLOWING: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
pub const TK_PARTITION: ::core::ffi::c_int = 88 as ::core::ffi::c_int;
pub const TK_PRECEDING: ::core::ffi::c_int = 89 as ::core::ffi::c_int;
pub const TK_RANGE: ::core::ffi::c_int = 90 as ::core::ffi::c_int;
pub const TK_UNBOUNDED: ::core::ffi::c_int = 91 as ::core::ffi::c_int;
pub const TK_EXCLUDE: ::core::ffi::c_int = 92 as ::core::ffi::c_int;
pub const TK_GROUPS: ::core::ffi::c_int = 93 as ::core::ffi::c_int;
pub const TK_OTHERS: ::core::ffi::c_int = 94 as ::core::ffi::c_int;
pub const TK_TIES: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const TK_GENERATED: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const TK_ALWAYS: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
pub const TK_MATERIALIZED: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const TK_REINDEX: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
pub const TK_RENAME: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const TK_CTIME_KW: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const TK_BITAND: ::core::ffi::c_int = 103 as ::core::ffi::c_int;
pub const TK_BITOR: ::core::ffi::c_int = 104 as ::core::ffi::c_int;
pub const TK_LSHIFT: ::core::ffi::c_int = 105 as ::core::ffi::c_int;
pub const TK_RSHIFT: ::core::ffi::c_int = 106 as ::core::ffi::c_int;
pub const TK_PLUS: ::core::ffi::c_int = 107 as ::core::ffi::c_int;
pub const TK_MINUS: ::core::ffi::c_int = 108 as ::core::ffi::c_int;
pub const TK_STAR: ::core::ffi::c_int = 109 as ::core::ffi::c_int;
pub const TK_SLASH: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const TK_REM: ::core::ffi::c_int = 111 as ::core::ffi::c_int;
pub const TK_CONCAT: ::core::ffi::c_int = 112 as ::core::ffi::c_int;
pub const TK_PTR: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const TK_COLLATE: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const TK_BITNOT: ::core::ffi::c_int = 115 as ::core::ffi::c_int;
pub const TK_ON: ::core::ffi::c_int = 116 as ::core::ffi::c_int;
pub const TK_INDEXED: ::core::ffi::c_int = 117 as ::core::ffi::c_int;
pub const TK_STRING: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const TK_JOIN_KW: ::core::ffi::c_int = 119 as ::core::ffi::c_int;
pub const TK_CONSTRAINT: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
pub const TK_DEFAULT: ::core::ffi::c_int = 121 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_PRIMARY: ::core::ffi::c_int = 123 as ::core::ffi::c_int;
pub const TK_UNIQUE: ::core::ffi::c_int = 124 as ::core::ffi::c_int;
pub const TK_CHECK: ::core::ffi::c_int = 125 as ::core::ffi::c_int;
pub const TK_REFERENCES: ::core::ffi::c_int = 126 as ::core::ffi::c_int;
pub const TK_AUTOINCR: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const TK_INSERT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const TK_DELETE: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const TK_UPDATE: ::core::ffi::c_int = 130 as ::core::ffi::c_int;
pub const TK_SET: ::core::ffi::c_int = 131 as ::core::ffi::c_int;
pub const TK_DEFERRABLE: ::core::ffi::c_int = 132 as ::core::ffi::c_int;
pub const TK_FOREIGN: ::core::ffi::c_int = 133 as ::core::ffi::c_int;
pub const TK_DROP: ::core::ffi::c_int = 134 as ::core::ffi::c_int;
pub const TK_UNION: ::core::ffi::c_int = 135 as ::core::ffi::c_int;
pub const TK_ALL: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const TK_EXCEPT: ::core::ffi::c_int = 137 as ::core::ffi::c_int;
pub const TK_INTERSECT: ::core::ffi::c_int = 138 as ::core::ffi::c_int;
pub const TK_SELECT: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const TK_VALUES: ::core::ffi::c_int = 140 as ::core::ffi::c_int;
pub const TK_DISTINCT: ::core::ffi::c_int = 141 as ::core::ffi::c_int;
pub const TK_DOT: ::core::ffi::c_int = 142 as ::core::ffi::c_int;
pub const TK_FROM: ::core::ffi::c_int = 143 as ::core::ffi::c_int;
pub const TK_JOIN: ::core::ffi::c_int = 144 as ::core::ffi::c_int;
pub const TK_USING: ::core::ffi::c_int = 145 as ::core::ffi::c_int;
pub const TK_ORDER: ::core::ffi::c_int = 146 as ::core::ffi::c_int;
pub const TK_GROUP: ::core::ffi::c_int = 147 as ::core::ffi::c_int;
pub const TK_HAVING: ::core::ffi::c_int = 148 as ::core::ffi::c_int;
pub const TK_LIMIT: ::core::ffi::c_int = 149 as ::core::ffi::c_int;
pub const TK_WHERE: ::core::ffi::c_int = 150 as ::core::ffi::c_int;
pub const TK_RETURNING: ::core::ffi::c_int = 151 as ::core::ffi::c_int;
pub const TK_INTO: ::core::ffi::c_int = 152 as ::core::ffi::c_int;
pub const TK_NOTHING: ::core::ffi::c_int = 153 as ::core::ffi::c_int;
pub const TK_FLOAT: ::core::ffi::c_int = 154 as ::core::ffi::c_int;
pub const TK_BLOB: ::core::ffi::c_int = 155 as ::core::ffi::c_int;
pub const TK_INTEGER: ::core::ffi::c_int = 156 as ::core::ffi::c_int;
pub const TK_VARIABLE: ::core::ffi::c_int = 157 as ::core::ffi::c_int;
pub const TK_CASE: ::core::ffi::c_int = 158 as ::core::ffi::c_int;
pub const TK_WHEN: ::core::ffi::c_int = 159 as ::core::ffi::c_int;
pub const TK_THEN: ::core::ffi::c_int = 160 as ::core::ffi::c_int;
pub const TK_ELSE: ::core::ffi::c_int = 161 as ::core::ffi::c_int;
pub const TK_INDEX: ::core::ffi::c_int = 162 as ::core::ffi::c_int;
pub const TK_ALTER: ::core::ffi::c_int = 163 as ::core::ffi::c_int;
pub const TK_ADD: ::core::ffi::c_int = 164 as ::core::ffi::c_int;
pub const TK_WINDOW: ::core::ffi::c_int = 165 as ::core::ffi::c_int;
pub const TK_OVER: ::core::ffi::c_int = 166 as ::core::ffi::c_int;
pub const TK_FILTER: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const TK_QNUMBER: ::core::ffi::c_int = 183 as ::core::ffi::c_int;
pub const TK_SPACE: ::core::ffi::c_int = 184 as ::core::ffi::c_int;
pub const TK_COMMENT: ::core::ffi::c_int = 185 as ::core::ffi::c_int;
pub const TK_ILLEGAL: ::core::ffi::c_int = 186 as ::core::ffi::c_int;
pub const SQLITE_Comments: u64_0 = (0x40 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const PARSE_MODE_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PARSE_MODE_RENAME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
static mut zKWText: [::core::ffi::c_char; 666] = [
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'X' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'K' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'Y' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'X' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'X' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'Y' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'K' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'X' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'X' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'X' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'Q' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'Y' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    '_' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'J' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'Y' as i32 as ::core::ffi::c_char,
    'Z' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'Z' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'Y' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    '_' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'K' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'Y' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'Y' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'A' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'Y' as i32 as ::core::ffi::c_char,
];
static mut aKWHash: [::core::ffi::c_uchar; 127] = [
    84 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    134 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    82 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    105 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    85 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    86 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    54 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    89 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    135 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    140 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    129 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    107 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    123 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    78 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    65 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    103 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    147 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    136 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    115 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    48 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    142 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    110 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    122 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    91 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    145 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    120 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    74 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    41 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    113 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    109 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    111 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    116 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    125 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    124 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    100 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    121 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    144 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    56 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    130 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    139 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    88 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    83 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    37 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    126 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    108 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    131 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    128 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    34 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    132 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    117 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut aKWNext: [::core::ffi::c_uchar; 148] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    43 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    106 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    114 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    143 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    141 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    119 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    52 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    137 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    138 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    133 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    36 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    77 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    69 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    146 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    75 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    127 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    104 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    81 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    112 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    118 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    68 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    44 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    102 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut aKWLen: [::core::ffi::c_uchar; 148] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut aKWOffset: [::core::ffi::c_ushort; 148] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    8 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    9 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    14 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    29 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    33 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    41 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    46 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    59 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    62 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    65 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    67 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    69 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    78 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    81 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    90 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    90 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    94 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    99 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    126 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    147 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    160 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    168 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    174 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    181 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    184 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    184 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    226 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    284 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    299 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    324 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    333 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    335 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    341 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    346 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    348 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    355 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    359 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    370 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    378 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    385 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    391 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    397 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    402 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    408 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    412 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    415 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    424 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    429 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    433 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    439 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    441 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    444 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    453 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    457 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    466 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    470 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    476 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    482 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    490 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    511 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    520 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    523 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    527 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    532 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    539 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    544 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    553 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    557 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    560 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    565 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    579 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    585 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    588 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    597 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    602 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    614 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    623 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    628 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    633 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    639 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    642 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    645 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    648 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    650 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    655 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    659 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];
static mut aKWCode: [::core::ffi::c_uchar; 148] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    TK_REINDEX as ::core::ffi::c_uchar,
    TK_INDEXED as ::core::ffi::c_uchar,
    TK_INDEX as ::core::ffi::c_uchar,
    TK_DESC as ::core::ffi::c_uchar,
    TK_ESCAPE as ::core::ffi::c_uchar,
    TK_EACH as ::core::ffi::c_uchar,
    TK_CHECK as ::core::ffi::c_uchar,
    TK_KEY as ::core::ffi::c_uchar,
    TK_BEFORE as ::core::ffi::c_uchar,
    TK_FOREIGN as ::core::ffi::c_uchar,
    TK_FOR as ::core::ffi::c_uchar,
    TK_IGNORE as ::core::ffi::c_uchar,
    TK_LIKE_KW as ::core::ffi::c_uchar,
    TK_EXPLAIN as ::core::ffi::c_uchar,
    TK_INSTEAD as ::core::ffi::c_uchar,
    TK_ADD as ::core::ffi::c_uchar,
    TK_DATABASE as ::core::ffi::c_uchar,
    TK_AS as ::core::ffi::c_uchar,
    TK_SELECT as ::core::ffi::c_uchar,
    TK_TABLE as ::core::ffi::c_uchar,
    TK_JOIN_KW as ::core::ffi::c_uchar,
    TK_THEN as ::core::ffi::c_uchar,
    TK_END as ::core::ffi::c_uchar,
    TK_DEFERRABLE as ::core::ffi::c_uchar,
    TK_ELSE as ::core::ffi::c_uchar,
    TK_EXCLUDE as ::core::ffi::c_uchar,
    TK_DELETE as ::core::ffi::c_uchar,
    TK_TEMP as ::core::ffi::c_uchar,
    TK_TEMP as ::core::ffi::c_uchar,
    TK_OR as ::core::ffi::c_uchar,
    TK_ISNULL as ::core::ffi::c_uchar,
    TK_NULLS as ::core::ffi::c_uchar,
    TK_SAVEPOINT as ::core::ffi::c_uchar,
    TK_INTERSECT as ::core::ffi::c_uchar,
    TK_TIES as ::core::ffi::c_uchar,
    TK_NOTNULL as ::core::ffi::c_uchar,
    TK_NOT as ::core::ffi::c_uchar,
    TK_NO as ::core::ffi::c_uchar,
    TK_NULL as ::core::ffi::c_uchar,
    TK_LIKE_KW as ::core::ffi::c_uchar,
    TK_EXCEPT as ::core::ffi::c_uchar,
    TK_TRANSACTION as ::core::ffi::c_uchar,
    TK_ACTION as ::core::ffi::c_uchar,
    TK_ON as ::core::ffi::c_uchar,
    TK_JOIN_KW as ::core::ffi::c_uchar,
    TK_ALTER as ::core::ffi::c_uchar,
    TK_RAISE as ::core::ffi::c_uchar,
    TK_EXCLUSIVE as ::core::ffi::c_uchar,
    TK_EXISTS as ::core::ffi::c_uchar,
    TK_CONSTRAINT as ::core::ffi::c_uchar,
    TK_INTO as ::core::ffi::c_uchar,
    TK_OFFSET as ::core::ffi::c_uchar,
    TK_OF as ::core::ffi::c_uchar,
    TK_SET as ::core::ffi::c_uchar,
    TK_TRIGGER as ::core::ffi::c_uchar,
    TK_RANGE as ::core::ffi::c_uchar,
    TK_GENERATED as ::core::ffi::c_uchar,
    TK_DETACH as ::core::ffi::c_uchar,
    TK_HAVING as ::core::ffi::c_uchar,
    TK_LIKE_KW as ::core::ffi::c_uchar,
    TK_BEGIN as ::core::ffi::c_uchar,
    TK_JOIN_KW as ::core::ffi::c_uchar,
    TK_REFERENCES as ::core::ffi::c_uchar,
    TK_UNIQUE as ::core::ffi::c_uchar,
    TK_QUERY as ::core::ffi::c_uchar,
    TK_WITHOUT as ::core::ffi::c_uchar,
    TK_WITH as ::core::ffi::c_uchar,
    TK_JOIN_KW as ::core::ffi::c_uchar,
    TK_RELEASE as ::core::ffi::c_uchar,
    TK_ATTACH as ::core::ffi::c_uchar,
    TK_BETWEEN as ::core::ffi::c_uchar,
    TK_NOTHING as ::core::ffi::c_uchar,
    TK_GROUPS as ::core::ffi::c_uchar,
    TK_GROUP as ::core::ffi::c_uchar,
    TK_CASCADE as ::core::ffi::c_uchar,
    TK_ASC as ::core::ffi::c_uchar,
    TK_DEFAULT as ::core::ffi::c_uchar,
    TK_CASE as ::core::ffi::c_uchar,
    TK_COLLATE as ::core::ffi::c_uchar,
    TK_CREATE as ::core::ffi::c_uchar,
    TK_CTIME_KW as ::core::ffi::c_uchar,
    TK_IMMEDIATE as ::core::ffi::c_uchar,
    TK_JOIN as ::core::ffi::c_uchar,
    TK_INSERT as ::core::ffi::c_uchar,
    TK_MATCH as ::core::ffi::c_uchar,
    TK_PLAN as ::core::ffi::c_uchar,
    TK_ANALYZE as ::core::ffi::c_uchar,
    TK_PRAGMA as ::core::ffi::c_uchar,
    TK_MATERIALIZED as ::core::ffi::c_uchar,
    TK_DEFERRED as ::core::ffi::c_uchar,
    TK_DISTINCT as ::core::ffi::c_uchar,
    TK_IS as ::core::ffi::c_uchar,
    TK_UPDATE as ::core::ffi::c_uchar,
    TK_VALUES as ::core::ffi::c_uchar,
    TK_VIRTUAL as ::core::ffi::c_uchar,
    TK_ALWAYS as ::core::ffi::c_uchar,
    TK_WHEN as ::core::ffi::c_uchar,
    TK_WHERE as ::core::ffi::c_uchar,
    TK_RECURSIVE as ::core::ffi::c_uchar,
    TK_ABORT as ::core::ffi::c_uchar,
    TK_AFTER as ::core::ffi::c_uchar,
    TK_RENAME as ::core::ffi::c_uchar,
    TK_AND as ::core::ffi::c_uchar,
    TK_DROP as ::core::ffi::c_uchar,
    TK_PARTITION as ::core::ffi::c_uchar,
    TK_AUTOINCR as ::core::ffi::c_uchar,
    TK_TO as ::core::ffi::c_uchar,
    TK_IN as ::core::ffi::c_uchar,
    TK_CAST as ::core::ffi::c_uchar,
    TK_COLUMNKW as ::core::ffi::c_uchar,
    TK_COMMIT as ::core::ffi::c_uchar,
    TK_CONFLICT as ::core::ffi::c_uchar,
    TK_JOIN_KW as ::core::ffi::c_uchar,
    TK_CTIME_KW as ::core::ffi::c_uchar,
    TK_CTIME_KW as ::core::ffi::c_uchar,
    TK_CURRENT as ::core::ffi::c_uchar,
    TK_PRECEDING as ::core::ffi::c_uchar,
    TK_FAIL as ::core::ffi::c_uchar,
    TK_LAST as ::core::ffi::c_uchar,
    TK_FILTER as ::core::ffi::c_uchar,
    TK_REPLACE as ::core::ffi::c_uchar,
    TK_FIRST as ::core::ffi::c_uchar,
    TK_FOLLOWING as ::core::ffi::c_uchar,
    TK_FROM as ::core::ffi::c_uchar,
    TK_JOIN_KW as ::core::ffi::c_uchar,
    TK_LIMIT as ::core::ffi::c_uchar,
    TK_IF as ::core::ffi::c_uchar,
    TK_ORDER as ::core::ffi::c_uchar,
    TK_RESTRICT as ::core::ffi::c_uchar,
    TK_OTHERS as ::core::ffi::c_uchar,
    TK_OVER as ::core::ffi::c_uchar,
    TK_RETURNING as ::core::ffi::c_uchar,
    TK_JOIN_KW as ::core::ffi::c_uchar,
    TK_ROLLBACK as ::core::ffi::c_uchar,
    TK_ROWS as ::core::ffi::c_uchar,
    TK_ROW as ::core::ffi::c_uchar,
    TK_UNBOUNDED as ::core::ffi::c_uchar,
    TK_UNION as ::core::ffi::c_uchar,
    TK_USING as ::core::ffi::c_uchar,
    TK_VACUUM as ::core::ffi::c_uchar,
    TK_VIEW as ::core::ffi::c_uchar,
    TK_WINDOW as ::core::ffi::c_uchar,
    TK_DO as ::core::ffi::c_uchar,
    TK_BY as ::core::ffi::c_uchar,
    TK_INITIALLY as ::core::ffi::c_uchar,
    TK_ALL as ::core::ffi::c_uchar,
    TK_PRIMARY as ::core::ffi::c_uchar,
];
unsafe extern "C" fn keywordCode(
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut pType: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut zKW: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        i = (*(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
            .offset(
                *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                    as isize,
            ) as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            ^ *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                .offset(
                    *z.offset((n - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_uchar as isize,
                ) as ::core::ffi::c_int * 3 as ::core::ffi::c_int
            ^ n * 1 as ::core::ffi::c_int) % 127 as ::core::ffi::c_int;
        i = aKWHash[i as usize] as ::core::ffi::c_int;
        while i > 0 as ::core::ffi::c_int {
            if !(aKWLen[i as usize] as ::core::ffi::c_int != n) {
                zKW = (&raw const zKWText as *const ::core::ffi::c_char)
                    .offset(
                        *(&raw const aKWOffset as *const ::core::ffi::c_ushort)
                            .offset(i as isize) as isize,
                    ) as *const ::core::ffi::c_char;
                if !(*z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & !(0x20 as ::core::ffi::c_int)
                    != *zKW.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int)
                {
                    if !(*z.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int & !(0x20 as ::core::ffi::c_int)
                        != *zKW.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int)
                    {
                        j = 2 as ::core::ffi::c_int;
                        while j < n
                            && *z.offset(j as isize) as ::core::ffi::c_int
                                & !(0x20 as ::core::ffi::c_int)
                                == *zKW.offset(j as isize) as ::core::ffi::c_int
                        {
                            j += 1;
                        }
                        if !(j < n) {
                            *pType = aKWCode[i as usize] as ::core::ffi::c_int;
                            break;
                        }
                    }
                }
            }
            i = aKWNext[i as usize] as ::core::ffi::c_int;
        }
        return n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3KeywordCode(
    mut z: *const ::core::ffi::c_uchar,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut id: ::core::ffi::c_int = TK_ID;
        if n >= 2 as ::core::ffi::c_int {
            keywordCode(z as *mut ::core::ffi::c_char, n, &raw mut id);
        }
        return id;
    }
}
pub const SQLITE_N_KEYWORD: ::core::ffi::c_int = 147 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3_keyword_name(
    mut i: ::core::ffi::c_int,
    mut pzName: *mut *const ::core::ffi::c_char,
    mut pnName: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if i < 0 as ::core::ffi::c_int || i >= SQLITE_N_KEYWORD {
            return SQLITE_ERROR;
        }
        i += 1;
        *pzName = (&raw const zKWText as *const ::core::ffi::c_char)
            .offset(aKWOffset[i as usize] as ::core::ffi::c_int as isize);
        *pnName = aKWLen[i as usize] as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_keyword_count() -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_N_KEYWORD;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_keyword_check(
    mut zName: *const ::core::ffi::c_char,
    mut nName: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (TK_ID != sqlite3KeywordCode(zName as *const ::core::ffi::c_uchar, nName))
            as ::core::ffi::c_int;
    }
}
pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
