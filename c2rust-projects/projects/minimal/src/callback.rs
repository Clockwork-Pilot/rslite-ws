use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type RenameToken;
    pub type Vdbe;
    pub type sqlite3_mutex;
    fn sqlite3HashInit(_: *mut Hash);
    fn sqlite3HashInsert(
        _: *mut Hash,
        pKey: *const ::core::ffi::c_char,
        pData: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3HashFind(
        _: *const Hash,
        pKey: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3HashClear(_: *mut Hash);
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
    fn sqlite3BtreeSchema(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3DeleteTable(_: *mut sqlite3, _: *mut Table);
    fn sqlite3DeleteTrigger(_: *mut sqlite3, _: *mut Trigger);
    fn sqlite3ValueText(_: *mut sqlite3_value, _: u8_0) -> *const ::core::ffi::c_void;
    fn sqlite3ValueSetStr(
        _: *mut sqlite3_value,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: u8_0,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3ValueFree(_: *mut sqlite3_value);
    fn sqlite3ValueNew(_: *mut sqlite3) -> *mut sqlite3_value;
    static sqlite3StrBINARY: [::core::ffi::c_char; 0];
    static sqlite3UpperToLower: [::core::ffi::c_uchar; 0];
    static mut sqlite3BuiltinFunctions: FuncDefHash;
    fn sqlite3ExpirePreparedStatements(_: *mut sqlite3, _: ::core::ffi::c_int);
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
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncDefHash {
    pub a: [*mut FuncDef; 23]

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_ERROR_MISSING_COLLSEQ: ::core::ffi::c_int = SQLITE_ERROR
    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_UTF16LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_UTF16BE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_UTF16NATIVE: ::core::ffi::c_int = SQLITE_UTF16LE;
unsafe extern "C" fn callCollNeeded(
    mut db: *mut sqlite3,
    mut enc: ::core::ffi::c_int,
    mut zName: *const ::core::ffi::c_char,
) {
    unsafe {
        if (*db).xCollNeeded.is_some() {
            let mut zExternal: *mut ::core::ffi::c_char = sqlite3DbStrDup(db, zName);
            if zExternal.is_null() {
                return;
            }
            (*db)
                .xCollNeeded
                .expect(
                    "non-null function pointer",
                )((*db).pCollNeededArg, db, enc, zExternal);
            sqlite3DbFree(db, zExternal as *mut ::core::ffi::c_void);
        }
        if (*db).xCollNeeded16.is_some() {
            let mut zExternal_0: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut pTmp: *mut sqlite3_value = sqlite3ValueNew(db);
            sqlite3ValueSetStr(
                pTmp,
                -1 as ::core::ffi::c_int,
                zName as *const ::core::ffi::c_void,
                SQLITE_UTF8 as u8_0,
                SQLITE_STATIC,
            );
            zExternal_0 = sqlite3ValueText(pTmp, SQLITE_UTF16NATIVE as u8_0)
                as *const ::core::ffi::c_char;
            if !zExternal_0.is_null() {
                (*db)
                    .xCollNeeded16
                    .expect(
                        "non-null function pointer",
                    )(
                    (*db).pCollNeededArg,
                    db,
                    (*db).enc as ::core::ffi::c_int,
                    zExternal_0 as *const ::core::ffi::c_void,
                );
            }
            sqlite3ValueFree(pTmp);
        }
    }
}
unsafe extern "C" fn synthCollSeq(
    mut db: *mut sqlite3,
    mut pColl: *mut CollSeq,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pColl2: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        let mut z: *mut ::core::ffi::c_char = (*pColl).zName;
        let mut i: ::core::ffi::c_int = 0;
        static mut aEnc: [u8_0; 3] = [
            SQLITE_UTF16BE as u8_0,
            SQLITE_UTF16LE as u8_0,
            SQLITE_UTF8 as u8_0,
        ];
        i = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int {
            pColl2 = sqlite3FindCollSeq(
                db,
                aEnc[i as usize],
                z,
                0 as ::core::ffi::c_int,
            );
            if (*pColl2).xCmp.is_some() {
                memcpy(
                    pColl as *mut ::core::ffi::c_void,
                    pColl2 as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<CollSeq>() as size_t,
                );
                (*pColl).xDel = None;
                return SQLITE_OK;
            }
            i += 1;
        }
        return SQLITE_ERROR;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CheckCollSeq(
    mut pParse: *mut Parse,
    mut pColl: *mut CollSeq,
) -> ::core::ffi::c_int {
    unsafe {
        if !pColl.is_null() && (*pColl).xCmp.is_none() {
            let mut zName: *const ::core::ffi::c_char = (*pColl).zName;
            let mut db: *mut sqlite3 = (*pParse).db;
            let mut p: *mut CollSeq = sqlite3GetCollSeq(pParse, (*db).enc, pColl, zName);
            if p.is_null() {
                return SQLITE_ERROR;
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn findCollSeqEntry(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut create: ::core::ffi::c_int,
) -> *mut CollSeq {
    unsafe {
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        pColl = sqlite3HashFind(&raw mut (*db).aCollSeq, zName) as *mut CollSeq;
        if pColl.is_null() && create != 0 {
            let mut nName: ::core::ffi::c_int = sqlite3Strlen30(zName)
                + 1 as ::core::ffi::c_int;
            pColl = sqlite3DbMallocZero(
                db,
                (3 as usize)
                    .wrapping_mul(::core::mem::size_of::<CollSeq>() as usize)
                    .wrapping_add(nName as usize) as u64_0,
            ) as *mut CollSeq;
            if !pColl.is_null() {
                let mut pDel: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
                let ref mut c2rust_fresh3 = (*pColl
                    .offset(0 as ::core::ffi::c_int as isize))
                    .zName;
                *c2rust_fresh3 = pColl.offset(3 as ::core::ffi::c_int as isize)
                    as *mut CollSeq as *mut ::core::ffi::c_char;
                (*pColl.offset(0 as ::core::ffi::c_int as isize)).enc = SQLITE_UTF8
                    as u8_0;
                let ref mut c2rust_fresh4 = (*pColl
                    .offset(1 as ::core::ffi::c_int as isize))
                    .zName;
                *c2rust_fresh4 = pColl.offset(3 as ::core::ffi::c_int as isize)
                    as *mut CollSeq as *mut ::core::ffi::c_char;
                (*pColl.offset(1 as ::core::ffi::c_int as isize)).enc = SQLITE_UTF16LE
                    as u8_0;
                let ref mut c2rust_fresh5 = (*pColl
                    .offset(2 as ::core::ffi::c_int as isize))
                    .zName;
                *c2rust_fresh5 = pColl.offset(3 as ::core::ffi::c_int as isize)
                    as *mut CollSeq as *mut ::core::ffi::c_char;
                (*pColl.offset(2 as ::core::ffi::c_int as isize)).enc = SQLITE_UTF16BE
                    as u8_0;
                memcpy(
                    (*pColl.offset(0 as ::core::ffi::c_int as isize)).zName
                        as *mut ::core::ffi::c_void,
                    zName as *const ::core::ffi::c_void,
                    nName as size_t,
                );
                pDel = sqlite3HashInsert(
                    &raw mut (*db).aCollSeq,
                    (*pColl.offset(0 as ::core::ffi::c_int as isize)).zName,
                    pColl as *mut ::core::ffi::c_void,
                ) as *mut CollSeq;
                if !pDel.is_null() {
                    sqlite3OomFault(db);
                    sqlite3DbFree(db, pDel as *mut ::core::ffi::c_void);
                    pColl = ::core::ptr::null_mut::<CollSeq>();
                }
            }
        }
        return pColl;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FindCollSeq(
    mut db: *mut sqlite3,
    mut enc: u8_0,
    mut zName: *const ::core::ffi::c_char,
    mut create: ::core::ffi::c_int,
) -> *mut CollSeq {
    unsafe {
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        if !zName.is_null() {
            pColl = findCollSeqEntry(db, zName, create);
            if !pColl.is_null() {
                pColl = pColl
                    .offset(
                        (enc as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                    );
            }
        } else {
            pColl = (*db).pDfltColl;
        }
        return pColl;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SetTextEncoding(mut db: *mut sqlite3, mut enc: u8_0) {
    unsafe {
        (*db).enc = enc;
        (*db).pDfltColl = sqlite3FindCollSeq(
            db,
            enc,
            &raw const sqlite3StrBINARY as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        sqlite3ExpirePreparedStatements(db, 1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GetCollSeq(
    mut pParse: *mut Parse,
    mut enc: u8_0,
    mut pColl: *mut CollSeq,
    mut zName: *const ::core::ffi::c_char,
) -> *mut CollSeq {
    unsafe {
        let mut p: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        let mut db: *mut sqlite3 = (*pParse).db;
        p = pColl;
        if p.is_null() {
            p = sqlite3FindCollSeq(db, enc, zName, 0 as ::core::ffi::c_int);
        }
        if p.is_null() || (*p).xCmp.is_none() {
            callCollNeeded(db, enc as ::core::ffi::c_int, zName);
            p = sqlite3FindCollSeq(db, enc, zName, 0 as ::core::ffi::c_int);
        }
        if !p.is_null() && (*p).xCmp.is_none() && synthCollSeq(db, p) != 0 {
            p = ::core::ptr::null_mut::<CollSeq>();
        }
        if p.is_null() {
            sqlite3ErrorMsg(
                pParse,
                b"no such collation sequence: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                zName,
            );
            (*pParse).rc = SQLITE_ERROR_MISSING_COLLSEQ;
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3LocateCollSeq(
    mut pParse: *mut Parse,
    mut zName: *const ::core::ffi::c_char,
) -> *mut CollSeq {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut enc: u8_0 = (*db).enc;
        let mut initbusy: u8_0 = (*db).init.busy;
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        pColl = sqlite3FindCollSeq(db, enc, zName, initbusy as ::core::ffi::c_int);
        if initbusy == 0 && (pColl.is_null() || (*pColl).xCmp.is_none()) {
            pColl = sqlite3GetCollSeq(pParse, enc, pColl, zName);
        }
        return pColl;
    }
}
pub const FUNC_PERFECT_MATCH: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
unsafe extern "C" fn matchQuality(
    mut p: *mut FuncDef,
    mut nArg: ::core::ffi::c_int,
    mut enc: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut match_0: ::core::ffi::c_int = 0;
        if (*p).nArg as ::core::ffi::c_int != nArg {
            if nArg == -2 as ::core::ffi::c_int {
                return if (*p).xSFunc.is_none() {
                    0 as ::core::ffi::c_int
                } else {
                    FUNC_PERFECT_MATCH
                };
            }
            if (*p).nArg as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            if ((*p).nArg as ::core::ffi::c_int) < -2 as ::core::ffi::c_int
                && nArg < -2 as ::core::ffi::c_int - (*p).nArg as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        if (*p).nArg as ::core::ffi::c_int == nArg {
            match_0 = 4 as ::core::ffi::c_int;
        } else {
            match_0 = 1 as ::core::ffi::c_int;
        }
        if enc as ::core::ffi::c_uint
            == (*p).funcFlags as ::core::ffi::c_uint
                & SQLITE_FUNC_ENCMASK as ::core::ffi::c_uint
        {
            match_0 += 2 as ::core::ffi::c_int;
        } else if enc as ::core::ffi::c_uint & (*p).funcFlags as ::core::ffi::c_uint
            & 2 as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
        {
            match_0 += 1 as ::core::ffi::c_int;
        }
        return match_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FunctionSearch(
    mut h: ::core::ffi::c_int,
    mut zFunc: *const ::core::ffi::c_char,
) -> *mut FuncDef {
    unsafe {
        let mut p: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
        p = sqlite3BuiltinFunctions.a[h as usize];
        while !p.is_null() {
            if sqlite3StrICmp((*p).zName, zFunc) == 0 as ::core::ffi::c_int {
                return p;
            }
            p = (*p).u.pHash;
        }
        return ::core::ptr::null_mut::<FuncDef>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3InsertBuiltinFuncs(
    mut aDef: *mut FuncDef,
    mut nDef: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < nDef {
            let mut pOther: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
            let mut zName: *const ::core::ffi::c_char = (*aDef.offset(i as isize)).zName;
            let mut nName: ::core::ffi::c_int = sqlite3Strlen30(zName);
            let mut h: ::core::ffi::c_int = (*zName
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int + nName)
                % SQLITE_FUNC_HASH_SZ;
            pOther = sqlite3FunctionSearch(h, zName);
            if !pOther.is_null() {
                let ref mut c2rust_fresh0 = (*aDef.offset(i as isize)).pNext;
                *c2rust_fresh0 = (*pOther).pNext;
                (*pOther).pNext = aDef.offset(i as isize) as *mut FuncDef;
            } else {
                let ref mut c2rust_fresh1 = (*aDef.offset(i as isize)).pNext;
                *c2rust_fresh1 = ::core::ptr::null_mut::<FuncDef>();
                let ref mut c2rust_fresh2 = (*aDef.offset(i as isize)).u.pHash;
                *c2rust_fresh2 = sqlite3BuiltinFunctions.a[h as usize];
                sqlite3BuiltinFunctions.a[h as usize] = aDef.offset(i as isize)
                    as *mut FuncDef;
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FindFunction(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut nArg: ::core::ffi::c_int,
    mut enc: u8_0,
    mut createFlag: u8_0,
) -> *mut FuncDef {
    unsafe {
        let mut p: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
        let mut pBest: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
        let mut bestScore: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut h: ::core::ffi::c_int = 0;
        let mut nName: ::core::ffi::c_int = 0;
        nName = sqlite3Strlen30(zName);
        p = sqlite3HashFind(&raw mut (*db).aFunc, zName) as *mut FuncDef;
        while !p.is_null() {
            let mut score: ::core::ffi::c_int = matchQuality(p, nArg, enc);
            if score > bestScore {
                pBest = p;
                bestScore = score;
            }
            p = (*p).pNext;
        }
        if createFlag == 0
            && (pBest.is_null()
                || (*db).mDbFlags as ::core::ffi::c_uint
                    & DBFLAG_PreferBuiltin as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint)
        {
            bestScore = 0 as ::core::ffi::c_int;
            h = (*(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                .offset(*zName.offset(0 as ::core::ffi::c_int as isize) as u8_0 as isize)
                as ::core::ffi::c_int + nName) % SQLITE_FUNC_HASH_SZ;
            p = sqlite3FunctionSearch(h, zName);
            while !p.is_null() {
                let mut score_0: ::core::ffi::c_int = matchQuality(p, nArg, enc);
                if score_0 > bestScore {
                    pBest = p;
                    bestScore = score_0;
                }
                p = (*p).pNext;
            }
        }
        if createFlag as ::core::ffi::c_int != 0 && bestScore < FUNC_PERFECT_MATCH
            && {
                pBest = sqlite3DbMallocZero(
                    db,
                    (::core::mem::size_of::<FuncDef>() as usize)
                        .wrapping_add(nName as usize)
                        .wrapping_add(1 as usize) as u64_0,
                ) as *mut FuncDef;
                !pBest.is_null()
            }
        {
            let mut pOther: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
            let mut z: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
            (*pBest).zName = pBest.offset(1 as ::core::ffi::c_int as isize)
                as *mut FuncDef as *const ::core::ffi::c_char;
            (*pBest).nArg = nArg as u16_0 as i16_0;
            (*pBest).funcFlags = enc as u32_0;
            memcpy(
                pBest.offset(1 as ::core::ffi::c_int as isize) as *mut FuncDef
                    as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                zName as *const ::core::ffi::c_void,
                (nName + 1 as ::core::ffi::c_int) as size_t,
            );
            z = (*pBest).zName as *mut u8_0;
            while *z != 0 {
                *z = *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                    .offset(*z as isize) as u8_0;
                z = z.offset(1);
            }
            pOther = sqlite3HashInsert(
                &raw mut (*db).aFunc,
                (*pBest).zName,
                pBest as *mut ::core::ffi::c_void,
            ) as *mut FuncDef;
            if pOther == pBest {
                sqlite3DbFree(db, pBest as *mut ::core::ffi::c_void);
                sqlite3OomFault(db);
                return ::core::ptr::null_mut::<FuncDef>();
            } else {
                (*pBest).pNext = pOther;
            }
        }
        if !pBest.is_null()
            && ((*pBest).xSFunc.is_some() || createFlag as ::core::ffi::c_int != 0)
        {
            return pBest;
        }
        return ::core::ptr::null_mut::<FuncDef>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SchemaClear(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        let mut temp1: Hash = Hash {
            htsize: 0,
            count: 0,
            first: ::core::ptr::null_mut::<HashElem>(),
            ht: ::core::ptr::null_mut::<_ht>()

        };
        let mut temp2: Hash = Hash {
            htsize: 0,
            count: 0,
            first: ::core::ptr::null_mut::<HashElem>(),
            ht: ::core::ptr::null_mut::<_ht>()

        };
        let mut pElem: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut pSchema: *mut Schema = p as *mut Schema;
        let mut xdb: sqlite3 = sqlite3 {
            pVfs: ::core::ptr::null_mut::<sqlite3_vfs>(),
            pVdbe: ::core::ptr::null_mut::<Vdbe>(),
            pDfltColl: ::core::ptr::null_mut::<CollSeq>(),
            mutex: ::core::ptr::null_mut::<sqlite3_mutex>(),
            aDb: ::core::ptr::null_mut::<Db>(),
            nDb: 0,
            mDbFlags: 0,
            flags: 0,
            lastRowid: 0,
            szMmap: 0,
            nSchemaLock: 0,
            openFlags: 0,
            errCode: 0,
            errByteOffset: 0,
            errMask: 0,
            iSysErrno: 0,
            dbOptFlags: 0,
            enc: 0,
            autoCommit: 0,
            temp_store: 0,
            mallocFailed: 0,
            bBenignMalloc: 0,
            dfltLockMode: 0,
            nextAutovac: 0,
            suppressErr: 0,
            vtabOnConflict: 0,
            isTransactionSavepoint: 0,
            mTrace: 0,
            noSharedCache: 0,
            nSqlExec: 0,
            eOpenState: 0,
            nextPagesize: 0,
            nChange: 0,
            nTotalChange: 0,
            aLimit: [0; 12],
            nMaxSorterMmap: 0,
            init: sqlite3InitInfo {
                newTnum: 0,
                iDb: 0,
                busy: 0,
                orphanTrigger_imposterTable_reopenMemdb: [0; 1],
                c2rust_padding: [0; 1],
                azInit: ::core::ptr::null_mut::<*const ::core::ffi::c_char>()

            },
            nVdbeActive: 0,
            nVdbeRead: 0,
            nVdbeWrite: 0,
            nVdbeExec: 0,
            nVDestroy: 0,
            nExtension: 0,
            aExtension: ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
            trace: C2Rust_Unnamed_21 { xLegacy: None },
            pTraceArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pCommitArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            xCommitCallback: None,
            pRollbackArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            xRollbackCallback: None,
            pUpdateArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            xUpdateCallback: None,
            pAutovacPagesArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            xAutovacDestr: None,
            xAutovacPages: None,
            pParse: ::core::ptr::null_mut::<Parse>(),
            xWalCallback: None,
            pWalArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            xCollNeeded: None,
            xCollNeeded16: None,
            pCollNeededArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pErr: ::core::ptr::null_mut::<sqlite3_value>(),
            u1: C2Rust_Unnamed_17 {
                isInterrupted: 0

            },
            lookaside: Lookaside {
                bDisable: 0,
                sz: 0,
                szTrue: 0,
                bMalloced: 0,
                nSlot: 0,
                anStat: [0; 3],
                pInit: ::core::ptr::null_mut::<LookasideSlot>(),
                pFree: ::core::ptr::null_mut::<LookasideSlot>(),
                pSmallInit: ::core::ptr::null_mut::<LookasideSlot>(),
                pSmallFree: ::core::ptr::null_mut::<LookasideSlot>(),
                pMiddle: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pStart: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pEnd: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pTrueEnd: ::core::ptr::null_mut::<::core::ffi::c_void>()

            },
            xAuth: None,
            pAuthArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            xProgress: None,
            pProgressArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            nProgressOps: 0,
            nVTrans: 0,
            aModule: Hash {
                htsize: 0,
                count: 0,
                first: ::core::ptr::null_mut::<HashElem>(),
                ht: ::core::ptr::null_mut::<_ht>()

            },
            pVtabCtx: ::core::ptr::null_mut::<VtabCtx>(),
            aVTrans: ::core::ptr::null_mut::<*mut VTable>(),
            pDisconnect: ::core::ptr::null_mut::<VTable>(),
            aFunc: Hash {
                htsize: 0,
                count: 0,
                first: ::core::ptr::null_mut::<HashElem>(),
                ht: ::core::ptr::null_mut::<_ht>()

            },
            aCollSeq: Hash {
                htsize: 0,
                count: 0,
                first: ::core::ptr::null_mut::<HashElem>(),
                ht: ::core::ptr::null_mut::<_ht>()

            },
            busyHandler: BusyHandler {
                xBusyHandler: None,
                pBusyArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                nBusy: 0

            },
            aDbStatic: [Db {
                zDbSName: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                pBt: ::core::ptr::null_mut::<Btree>(),
                safety_level: 0,
                bSyncSet: 0,
                pSchema: ::core::ptr::null_mut::<Schema>()

            }; 2],
            pSavepoint: ::core::ptr::null_mut::<Savepoint>(),
            nAnalysisLimit: 0,
            busyTimeout: 0,
            nSavepoint: 0,
            nStatement: 0,
            nDeferredCons: 0,
            nDeferredImmCons: 0,
            pnBytesFreed: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            pDbData: ::core::ptr::null_mut::<DbClientData>(),
            nSpill: 0

        };
        memset(
            &raw mut xdb as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sqlite3>() as size_t,
        );
        temp1 = (*pSchema).tblHash;
        temp2 = (*pSchema).trigHash;
        sqlite3HashInit(&raw mut (*pSchema).trigHash);
        sqlite3HashClear(&raw mut (*pSchema).idxHash);
        pElem = temp2.first;
        while !pElem.is_null() {
            sqlite3DeleteTrigger(&raw mut xdb, (*pElem).data as *mut Trigger);
            pElem = (*pElem).next;
        }
        sqlite3HashClear(&raw mut temp2);
        sqlite3HashInit(&raw mut (*pSchema).tblHash);
        pElem = temp1.first;
        while !pElem.is_null() {
            let mut pTab: *mut Table = (*pElem).data as *mut Table;
            sqlite3DeleteTable(&raw mut xdb, pTab);
            pElem = (*pElem).next;
        }
        sqlite3HashClear(&raw mut temp1);
        sqlite3HashClear(&raw mut (*pSchema).fkeyHash);
        (*pSchema).pSeqTab = ::core::ptr::null_mut::<Table>();
        if (*pSchema).schemaFlags as ::core::ffi::c_int & DB_SchemaLoaded != 0 {
            (*pSchema).iGeneration += 1;
        }
        (*pSchema).schemaFlags = ((*pSchema).schemaFlags as ::core::ffi::c_int
            & !(DB_SchemaLoaded | DB_ResetWanted)) as u16_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SchemaGet(
    mut db: *mut sqlite3,
    mut pBt: *mut Btree,
) -> *mut Schema {
    unsafe {
        let mut p: *mut Schema = ::core::ptr::null_mut::<Schema>();
        if !pBt.is_null() {
            p = sqlite3BtreeSchema(
                pBt,
                ::core::mem::size_of::<Schema>() as ::core::ffi::c_int,
                Some(
                    sqlite3SchemaClear
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            ) as *mut Schema;
        } else {
            p = sqlite3DbMallocZero(
                ::core::ptr::null_mut::<sqlite3>(),
                ::core::mem::size_of::<Schema>() as u64_0,
            ) as *mut Schema;
        }
        if p.is_null() {
            sqlite3OomFault(db);
        } else if 0 as ::core::ffi::c_int == (*p).file_format as ::core::ffi::c_int {
            sqlite3HashInit(&raw mut (*p).tblHash);
            sqlite3HashInit(&raw mut (*p).idxHash);
            sqlite3HashInit(&raw mut (*p).trigHash);
            sqlite3HashInit(&raw mut (*p).fkeyHash);
            (*p).enc = SQLITE_UTF8 as u8_0;
        }
        return p;
    }
}
pub const DB_SchemaLoaded: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const DB_ResetWanted: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_FUNC_HASH_SZ: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const DBFLAG_PreferBuiltin: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_FUNC_ENCMASK: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
