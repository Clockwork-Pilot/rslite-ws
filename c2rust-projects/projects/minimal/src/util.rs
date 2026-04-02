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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strspn(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3OsGetLastError(_: *mut sqlite3_vfs) -> ::core::ffi::c_int;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbRealloc(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: u64_0,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3VMPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3RowSetClear(_: *mut ::core::ffi::c_void);
    fn sqlite3ValueSetStr(
        _: *mut sqlite3_value,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: u8_0,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3ValueSetNull(_: *mut sqlite3_value);
    fn sqlite3ValueNew(_: *mut sqlite3) -> *mut sqlite3_value;
    static mut sqlite3StdType: [*const ::core::ffi::c_char; 0];
    static sqlite3UpperToLower: [::core::ffi::c_uchar; 0];
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    static mut sqlite3Config: Sqlite3Config;
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
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_INTERRUPT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_CANTOPEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DIGIT_SEPARATOR: ::core::ffi::c_int = '_' as i32;
pub const SQLITE_MAX_U32: ::core::ffi::c_ulonglong = ((1 as ::core::ffi::c_int
    as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_ulonglong);
pub const LARGEST_INT64: i64_0 = 0xffffffff as i64_0
    | (0x7fffffff as ::core::ffi::c_int as i64_0) << 32 as ::core::ffi::c_int;
pub const LARGEST_UINT64: u64_0 = 0xffffffff as u64_0
    | (0xffffffff as ::core::ffi::c_uint as u64_0) << 32 as ::core::ffi::c_int;
pub const SMALLEST_INT64: i64_0 = -1 as ::core::ffi::c_int as i64_0 - LARGEST_INT64;
pub const SQLITE_STATE_OPEN: ::core::ffi::c_int = 0x76 as ::core::ffi::c_int;
pub const SQLITE_STATE_SICK: ::core::ffi::c_int = 0xba as ::core::ffi::c_int;
pub const SQLITE_STATE_BUSY: ::core::ffi::c_int = 0x6d as ::core::ffi::c_int;
pub const COLFLAG_HASTYPE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const EP_DblQuoted: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const EP_IntValue: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const EP_Quoted: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const EXP754: u64_0 = (0x7ff as ::core::ffi::c_int as u64_0)
    << 52 as ::core::ffi::c_int;
pub const MAN754: ::core::ffi::c_ulonglong = ((1 as ::core::ffi::c_int
    as ::core::ffi::c_ulonglong) << 52 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_ulonglong);
pub const TK_FLOAT: ::core::ffi::c_int = 154 as ::core::ffi::c_int;
pub const TK_INTEGER: ::core::ffi::c_int = 156 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3FaultSim(
    mut iTest: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut xCallback: Option<
            unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int
        > = sqlite3Config.xTestCallback;
        return if xCallback.is_some() {
            xCallback.expect("non-null function pointer")(iTest)
        } else {
            SQLITE_OK
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IsNaN(
    mut x: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut y: u64_0 = 0;
        memcpy(
            &raw mut y as *mut ::core::ffi::c_void,
            &raw mut x as *const ::core::ffi::c_void,
            ::core::mem::size_of::<u64_0>() as size_t,
        );
        rc = (y & EXP754 == EXP754
            && y as ::core::ffi::c_ulonglong & MAN754 != 0 as ::core::ffi::c_ulonglong)
            as ::core::ffi::c_int;
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IsOverflow(
    mut x: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut y: u64_0 = 0;
        memcpy(
            &raw mut y as *mut ::core::ffi::c_void,
            &raw mut x as *const ::core::ffi::c_void,
            ::core::mem::size_of::<u64_0>() as size_t,
        );
        rc = (y & EXP754 == EXP754) as ::core::ffi::c_int;
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Strlen30(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if z.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        return 0x3fffffff as ::core::ffi::c_int & strlen(z) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ColumnType(
    mut pCol: *mut Column,
    mut zDflt: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_HASTYPE != 0 {
            return (*pCol)
                .zCnName
                .offset(strlen((*pCol).zCnName) as isize)
                .offset(1 as ::core::ffi::c_int as isize)
        } else if (*pCol).eCType() != 0 {
            return *(&raw mut sqlite3StdType as *mut *const ::core::ffi::c_char)
                .offset(
                    ((*pCol).eCType() as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as isize,
                ) as *mut ::core::ffi::c_char
        } else {
            return zDflt
        };
    }
}
#[inline(never)]
unsafe extern "C" fn sqlite3ErrorFinish(
    mut db: *mut sqlite3,
    mut err_code: ::core::ffi::c_int,
) {
    unsafe {
        if !(*db).pErr.is_null() {
            sqlite3ValueSetNull((*db).pErr);
        }
        sqlite3SystemError(db, err_code);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Error(
    mut db: *mut sqlite3,
    mut err_code: ::core::ffi::c_int,
) {
    unsafe {
        (*db).errCode = err_code;
        if err_code != 0 || !(*db).pErr.is_null() {
            sqlite3ErrorFinish(db, err_code);
        } else {
            (*db).errByteOffset = -1 as ::core::ffi::c_int;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ErrorClear(mut db: *mut sqlite3) {
    unsafe {
        (*db).errCode = SQLITE_OK;
        (*db).errByteOffset = -1 as ::core::ffi::c_int;
        if !(*db).pErr.is_null() {
            sqlite3ValueSetNull((*db).pErr);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SystemError(
    mut db: *mut sqlite3,
    mut rc: ::core::ffi::c_int,
) {
    unsafe {
        if rc == SQLITE_IOERR_NOMEM {
            return;
        }
        rc &= 0xff as ::core::ffi::c_int;
        if rc == SQLITE_CANTOPEN || rc == SQLITE_IOERR {
            (*db).iSysErrno = sqlite3OsGetLastError((*db).pVfs);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ErrorWithMsg(
    mut db: *mut sqlite3,
    mut err_code: ::core::ffi::c_int,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        (*db).errCode = err_code;
        sqlite3SystemError(db, err_code);
        if zFormat.is_null() {
            sqlite3Error(db, err_code);
        } else if !(*db).pErr.is_null()
            || {
                (*db).pErr = sqlite3ValueNew(db);
                !(*db).pErr.is_null()
            }
        {
            let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            
            let mut ap = c2rust_args;
            z = sqlite3VMPrintf(db, zFormat, ap);
            sqlite3ValueSetStr(
                (*db).pErr,
                -1 as ::core::ffi::c_int,
                z as *const ::core::ffi::c_void,
                SQLITE_UTF8 as u8_0,
                Some(
                    sqlite3RowSetClear
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ProgressCheck(mut p: *mut Parse) {
    unsafe {
        let mut db: *mut sqlite3 = (*p).db;
        if (*db).u1.isInterrupted != 0
        {
            (*p).nErr += 1;
            (*p).rc = SQLITE_INTERRUPT;
        }
        if (*db).xProgress.is_some() {
            if (*p).rc == SQLITE_INTERRUPT {
                (*p).nProgressSteps = 0 as u32_0;
            } else {
                (*p).nProgressSteps = (*p).nProgressSteps.wrapping_add(1);
                if (*p).nProgressSteps >= (*db).nProgressOps {
                    if (*db)
                        .xProgress
                        .expect("non-null function pointer")((*db).pProgressArg) != 0
                    {
                        (*p).nErr += 1;
                        (*p).rc = SQLITE_INTERRUPT;
                    }
                    (*p).nProgressSteps = 0 as u32_0;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ErrorMsg(
    mut pParse: *mut Parse,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut zMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        
        let mut db: *mut sqlite3 = (*pParse).db;
        (*db).errByteOffset = -2 as ::core::ffi::c_int;
        let mut ap = c2rust_args;
        zMsg = sqlite3VMPrintf(db, zFormat, ap);
        if (*db).errByteOffset < -1 as ::core::ffi::c_int {
            (*db).errByteOffset = -1 as ::core::ffi::c_int;
        }
        if (*db).suppressErr != 0 {
            sqlite3DbFree(db, zMsg as *mut ::core::ffi::c_void);
            if (*db).mallocFailed != 0 {
                (*pParse).nErr += 1;
                (*pParse).rc = SQLITE_NOMEM;
            }
        } else {
            (*pParse).nErr += 1;
            sqlite3DbFree(db, (*pParse).zErrMsg as *mut ::core::ffi::c_void);
            (*pParse).zErrMsg = zMsg;
            (*pParse).rc = SQLITE_ERROR;
            (*pParse).pWith = ::core::ptr::null_mut::<With>();
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ErrorToParser(
    mut db: *mut sqlite3,
    mut errCode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pParse: *mut Parse = ::core::ptr::null_mut::<Parse>();
        if db.is_null()
            || {
                pParse = (*db).pParse;
                pParse.is_null()
            }
        {
            return errCode;
        }
        (*pParse).rc = errCode;
        (*pParse).nErr += 1;
        return errCode;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Dequote(mut z: *mut ::core::ffi::c_char) {
    unsafe {
        let mut quote: ::core::ffi::c_char = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        if z.is_null() {
            return;
        }
        quote = *z.offset(0 as ::core::ffi::c_int as isize);
        if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(quote as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int == 0
        {
            return;
        }
        if quote as ::core::ffi::c_int == '[' as i32 {
            quote = ']' as i32 as ::core::ffi::c_char;
        }
        i = 1 as ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_int;
        loop {
            if *z.offset(i as isize) as ::core::ffi::c_int == quote as ::core::ffi::c_int
            {
                if !(*z.offset((i + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int == quote as ::core::ffi::c_int)
                {
                    break;
                }
                let c2rust_fresh6 = j;
                j = j + 1;
                *z.offset(c2rust_fresh6 as isize) = quote;
                i += 1;
            } else {
                let c2rust_fresh7 = j;
                j = j + 1;
                *z.offset(c2rust_fresh7 as isize) = *z.offset(i as isize);
            }
            i += 1;
        }
        *z.offset(j as isize) = 0 as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DequoteExpr(mut p: *mut Expr) {
    unsafe {
        (*p).flags
            |= (if *(*p).u.zToken.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int == '"' as i32
            {
                EP_Quoted | EP_DblQuoted
            } else {
                EP_Quoted
            }) as ::core::ffi::c_uint;
        sqlite3Dequote((*p).u.zToken);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DequoteNumber(mut pParse: *mut Parse, mut p: *mut Expr) {
    unsafe {
        if !p.is_null() {
            let mut pIn: *const ::core::ffi::c_char = (*p).u.zToken;
            let mut pOut: *mut ::core::ffi::c_char = (*p).u.zToken;
            let mut bHex: ::core::ffi::c_int = (*pIn
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '0' as i32
                && (*pIn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'x' as i32
                    || *pIn.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'X' as i32)) as ::core::ffi::c_int;
            let mut iValue: ::core::ffi::c_int = 0;
            (*p).op = TK_INTEGER as u8_0;
            loop {
                if *pIn as ::core::ffi::c_int != SQLITE_DIGIT_SEPARATOR {
                    let c2rust_fresh8 = pOut;
                    pOut = pOut.offset(1);
                    *c2rust_fresh8 = *pIn;
                    if *pIn as ::core::ffi::c_int == 'e' as i32
                        || *pIn as ::core::ffi::c_int == 'E' as i32
                        || *pIn as ::core::ffi::c_int == '.' as i32
                    {
                        (*p).op = TK_FLOAT as u8_0;
                    }
                } else if bHex == 0 as ::core::ffi::c_int
                    && (*(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(
                            *pIn.offset(-1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uchar as isize,
                        ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
                        || *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *pIn.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0)
                    || bHex == 1 as ::core::ffi::c_int
                        && (*(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *pIn.offset(-1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0
                            || *(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(
                                    *pIn.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uchar as isize,
                                ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int == 0)
                {
                    sqlite3ErrorMsg(
                        pParse,
                        b"unrecognized token: \"%s\"\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*p).u.zToken,
                    );
                }
                let c2rust_fresh9 = pIn;
                pIn = pIn.offset(1);
                if !(*c2rust_fresh9 != 0) {
                    break;
                }
            }
            if bHex != 0 {
                (*p).op = TK_INTEGER as u8_0;
            }
            if (*p).op as ::core::ffi::c_int == TK_INTEGER
                && sqlite3GetInt32((*p).u.zToken, &raw mut iValue) != 0
            {
                (*p).u.iValue = iValue;
                (*p).flags |= EP_IntValue as ::core::ffi::c_uint;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DequoteToken(mut p: *mut Token) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        if (*p).n < 2 as ::core::ffi::c_uint {
            return;
        }
        if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(
                *(*p).z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                    as isize,
            ) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0
        {
            return;
        }
        i = 1 as ::core::ffi::c_uint;
        while i < (*p).n.wrapping_sub(1 as ::core::ffi::c_uint) {
            if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(*(*p).z.offset(i as isize) as ::core::ffi::c_uchar as isize)
                as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
            {
                return;
            }
            i = i.wrapping_add(1);
        }
        (*p).n = (*p).n.wrapping_sub(2 as ::core::ffi::c_uint);
        (*p).z = (*p).z.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TokenInit(
    mut p: *mut Token,
    mut z: *mut ::core::ffi::c_char,
) {
    unsafe {
        (*p).z = z;
        (*p).n = sqlite3Strlen30(z) as ::core::ffi::c_uint;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_stricmp(
    mut zLeft: *const ::core::ffi::c_char,
    mut zRight: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if zLeft.is_null() {
            return if !zRight.is_null() {
                -1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        } else if zRight.is_null() {
            return 1 as ::core::ffi::c_int
        }
        return sqlite3StrICmp(zLeft, zRight);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3StrICmp(
    mut zLeft: *const ::core::ffi::c_char,
    mut zRight: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut a: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut b: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut c: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_int = 0;
        a = zLeft as *mut ::core::ffi::c_uchar;
        b = zRight as *mut ::core::ffi::c_uchar;
        loop {
            c = *a as ::core::ffi::c_int;
            x = *b as ::core::ffi::c_int;
            if c == x {
                if c == 0 as ::core::ffi::c_int {
                    break;
                }
            } else {
                c = *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                    .offset(c as isize) as ::core::ffi::c_int
                    - *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                        .offset(x as isize) as ::core::ffi::c_int;
                if c != 0 {
                    break;
                }
            }
            a = a.offset(1);
            b = b.offset(1);
        }
        return c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_strnicmp(
    mut zLeft: *const ::core::ffi::c_char,
    mut zRight: *const ::core::ffi::c_char,
    mut N: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut a: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut b: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        if zLeft.is_null() {
            return if !zRight.is_null() {
                -1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        } else if zRight.is_null() {
            return 1 as ::core::ffi::c_int
        }
        a = zLeft as *mut ::core::ffi::c_uchar;
        b = zRight as *mut ::core::ffi::c_uchar;
        loop {
            let c2rust_fresh0 = N;
            N = N - 1;
            if !(c2rust_fresh0 > 0 as ::core::ffi::c_int
                && *a as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                    .offset(*a as isize) as ::core::ffi::c_int
                    == *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                        .offset(*b as isize) as ::core::ffi::c_int)
            {
                break;
            }
            a = a.offset(1);
            b = b.offset(1);
        }
        return if N < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                .offset(*a as isize) as ::core::ffi::c_int
                - *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                    .offset(*b as isize) as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3StrIHash(mut z: *const ::core::ffi::c_char) -> u8_0 {
    unsafe {
        let mut h: u8_0 = 0 as u8_0;
        if z.is_null() {
            return 0 as u8_0;
        }
        while *z.offset(0 as ::core::ffi::c_int as isize) != 0 {
            h = (h as ::core::ffi::c_int
                + *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                    .offset(
                        *z.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar as isize,
                    ) as ::core::ffi::c_int) as u8_0;
            z = z.offset(1);
        }
        return h;
    }
}
unsafe extern "C" fn dekkerMul2(
    mut x: *mut ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut yy: ::core::ffi::c_double,
) {
    unsafe {
        let mut tx: ::core::ffi::c_double = 0.;
        let mut ty: ::core::ffi::c_double = 0.;
        let mut p: ::core::ffi::c_double = 0.;
        let mut q: ::core::ffi::c_double = 0.;
        let mut c: ::core::ffi::c_double = 0.;
        let mut cc: ::core::ffi::c_double = 0.;
        let mut hx: ::core::ffi::c_double = 0.;
        let mut hy: ::core::ffi::c_double = 0.;
        let mut m: u64_0 = 0;
        memcpy(
            &raw mut m as *mut ::core::ffi::c_void,
            x.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double
                as *mut ::core::ffi::c_void,
            8 as size_t,
        );
        m &= 0xfffffffffc000000 as ::core::ffi::c_ulonglong;
        memcpy(
            &raw mut hx as *mut ::core::ffi::c_void,
            &raw mut m as *const ::core::ffi::c_void,
            8 as size_t,
        );
        ::core::ptr::write_volatile(
            &mut tx as *mut ::core::ffi::c_double,
            *x.offset(0 as ::core::ffi::c_int as isize) - hx,
        );
        memcpy(
            &raw mut m as *mut ::core::ffi::c_void,
            &raw mut y as *const ::core::ffi::c_void,
            8 as size_t,
        );
        m &= 0xfffffffffc000000 as ::core::ffi::c_ulonglong;
        memcpy(
            &raw mut hy as *mut ::core::ffi::c_void,
            &raw mut m as *const ::core::ffi::c_void,
            8 as size_t,
        );
        ::core::ptr::write_volatile(&mut ty as *mut ::core::ffi::c_double, y - hy);
        ::core::ptr::write_volatile(&mut p as *mut ::core::ffi::c_double, hx * hy);
        ::core::ptr::write_volatile(
            &mut q as *mut ::core::ffi::c_double,
            hx * ty + tx * hy,
        );
        ::core::ptr::write_volatile(&mut c as *mut ::core::ffi::c_double, p + q);
        ::core::ptr::write_volatile(
            &mut cc as *mut ::core::ffi::c_double,
            p - c + q + tx * ty,
        );
        ::core::ptr::write_volatile(
            &mut cc as *mut ::core::ffi::c_double,
            *x.offset(0 as ::core::ffi::c_int as isize) * yy
                + *x.offset(1 as ::core::ffi::c_int as isize) * y + cc,
        );
        ::core::ptr::write_volatile(x.offset(0 as ::core::ffi::c_int as isize), c + cc);
        ::core::ptr::write_volatile(
            x.offset(1 as ::core::ffi::c_int as isize),
            c - *x.offset(0 as ::core::ffi::c_int as isize),
        );
        let ref mut c2rust_fresh5 = *x.offset(1 as ::core::ffi::c_int as isize);
        ::core::ptr::write_volatile(
            c2rust_fresh5,
            ::core::ptr::read_volatile::<
                ::core::ffi::c_double,
            >(c2rust_fresh5 as *const ::core::ffi::c_double) + cc,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AtoF(
    mut z: *const ::core::ffi::c_char,
    mut pResult: *mut ::core::ffi::c_double,
    mut length: ::core::ffi::c_int,
    mut enc: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut incr: ::core::ffi::c_int = 0;
        let mut zEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut sign: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut s: u64_0 = 0 as u64_0;
        let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut esign: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut e: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut eValid: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut nDigit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut eType: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut s2: u64_0 = 0;
        let mut rr: [::core::ffi::c_double; 2] = [0.; 2];
        *pResult = 0.0f64;
        if length == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if enc as ::core::ffi::c_int == SQLITE_UTF8 {
            incr = 1 as ::core::ffi::c_int;
            zEnd = z.offset(length as isize);
        } else {
            let mut i: ::core::ffi::c_int = 0;
            incr = 2 as ::core::ffi::c_int;
            length &= !(1 as ::core::ffi::c_int);
            i = 3 as ::core::ffi::c_int - enc as ::core::ffi::c_int;
            while i < length
                && *z.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                i += 2 as ::core::ffi::c_int;
            }
            if i < length {
                eType = -100 as ::core::ffi::c_int;
            }
            zEnd = z.offset((i ^ 1 as ::core::ffi::c_int) as isize)
                as *const ::core::ffi::c_char;
            z = z.offset((enc as ::core::ffi::c_int & 1 as ::core::ffi::c_int) as isize);
        }
        while z < zEnd
            && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(*z as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                & 0x1 as ::core::ffi::c_int != 0
        {
            z = z.offset(incr as isize);
        }
        if z >= zEnd {
            return 0 as ::core::ffi::c_int;
        }
        if *z as ::core::ffi::c_int == '-' as i32 {
            sign = -1 as ::core::ffi::c_int;
            z = z.offset(incr as isize);
        } else if *z as ::core::ffi::c_int == '+' as i32 {
            z = z.offset(incr as isize);
        }
        while z < zEnd
            && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(*z as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                & 0x4 as ::core::ffi::c_int != 0
        {
            s = (s as ::core::ffi::c_ulonglong)
                .wrapping_mul(10 as ::core::ffi::c_ulonglong)
                .wrapping_add(
                    (*z as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_ulonglong,
                ) as u64_0;
            z = z.offset(incr as isize);
            nDigit += 1;
            if s
                >= LARGEST_UINT64
                    .wrapping_sub(9 as ::core::ffi::c_ulonglong)
                    .wrapping_div(10 as ::core::ffi::c_ulonglong)
            {
                while z < zEnd
                    && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(*z as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                {
                    z = z.offset(incr as isize);
                    d += 1;
                }
            }
        }
        if !(z >= zEnd) {
            if *z as ::core::ffi::c_int == '.' as i32 {
                z = z.offset(incr as isize);
                eType += 1;
                while z < zEnd
                    && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(*z as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                {
                    if s
                        < LARGEST_UINT64
                            .wrapping_sub(9 as ::core::ffi::c_ulonglong)
                            .wrapping_div(10 as ::core::ffi::c_ulonglong)
                    {
                        s = (s as ::core::ffi::c_ulonglong)
                            .wrapping_mul(10 as ::core::ffi::c_ulonglong)
                            .wrapping_add(
                                (*z as ::core::ffi::c_int - '0' as i32)
                                    as ::core::ffi::c_ulonglong,
                            ) as u64_0;
                        d -= 1;
                        nDigit += 1;
                    }
                    z = z.offset(incr as isize);
                }
            }
            if !(z >= zEnd) {
                if *z as ::core::ffi::c_int == 'e' as i32
                    || *z as ::core::ffi::c_int == 'E' as i32
                {
                    z = z.offset(incr as isize);
                    eValid = 0 as ::core::ffi::c_int;
                    eType += 1;
                    if z >= zEnd {
                        c2rust_current_block = 11184888009852336325;
                    } else {
                        if *z as ::core::ffi::c_int == '-' as i32 {
                            esign = -1 as ::core::ffi::c_int;
                            z = z.offset(incr as isize);
                        } else if *z as ::core::ffi::c_int == '+' as i32 {
                            z = z.offset(incr as isize);
                        }
                        while z < zEnd
                            && *(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(*z as ::core::ffi::c_uchar as isize)
                                as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                        {
                            e = if e < 10000 as ::core::ffi::c_int {
                                e * 10 as ::core::ffi::c_int
                                    + (*z as ::core::ffi::c_int - '0' as i32)
                            } else {
                                10000 as ::core::ffi::c_int
                            };
                            z = z.offset(incr as isize);
                            eValid = 1 as ::core::ffi::c_int;
                        }
                        c2rust_current_block = 12930649117290160518;
                    }
                } else {
                    c2rust_current_block = 12930649117290160518;
                }
                match c2rust_current_block {
                    11184888009852336325 => {}
                    _ => {
                        while z < zEnd
                            && *(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(*z as ::core::ffi::c_uchar as isize)
                                as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
                        {
                            z = z.offset(incr as isize);
                        }
                    }
                }
            }
        }
        if s == 0 as ::core::ffi::c_ulonglong {
            *pResult = if sign < 0 as ::core::ffi::c_int { -0.0f64 } else { 0.0f64 };
        } else {
            e = e * esign + d;
            while e > 0 as ::core::ffi::c_int
                && s
                    < LARGEST_UINT64
                        .wrapping_sub(0x7ff as ::core::ffi::c_ulonglong)
                        .wrapping_div(10 as ::core::ffi::c_ulonglong)
            {
                s = (s as ::core::ffi::c_ulonglong)
                    .wrapping_mul(10 as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
                e -= 1;
            }
            while e < 0 as ::core::ffi::c_int
                && (s as ::core::ffi::c_ulonglong)
                    .wrapping_rem(10 as ::core::ffi::c_ulonglong)
                    == 0 as ::core::ffi::c_ulonglong
            {
                s = (s as ::core::ffi::c_ulonglong)
                    .wrapping_div(10 as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
                e += 1;
            }
            rr[0 as ::core::ffi::c_int as usize] = s as ::core::ffi::c_double;
            if rr[0 as ::core::ffi::c_int as usize] <= 18446744073709549568.0f64 {
                s2 = rr[0 as ::core::ffi::c_int as usize] as u64_0;
                rr[1 as ::core::ffi::c_int as usize] = if s >= s2 {
                    s.wrapping_sub(s2) as ::core::ffi::c_double
                } else {
                    -(s2.wrapping_sub(s) as ::core::ffi::c_double)
                };
            } else {
                rr[1 as ::core::ffi::c_int as usize] = 0.0f64;
            }
            if e > 0 as ::core::ffi::c_int {
                while e >= 100 as ::core::ffi::c_int {
                    e -= 100 as ::core::ffi::c_int;
                    dekkerMul2(
                        &raw mut rr as *mut ::core::ffi::c_double
                            as *mut ::core::ffi::c_double,
                        1.0e+100f64,
                        -1.5902891109759918046e+83f64,
                    );
                }
                while e >= 10 as ::core::ffi::c_int {
                    e -= 10 as ::core::ffi::c_int;
                    dekkerMul2(
                        &raw mut rr as *mut ::core::ffi::c_double
                            as *mut ::core::ffi::c_double,
                        1.0e+10f64,
                        0.0f64,
                    );
                }
                while e >= 1 as ::core::ffi::c_int {
                    e -= 1 as ::core::ffi::c_int;
                    dekkerMul2(
                        &raw mut rr as *mut ::core::ffi::c_double
                            as *mut ::core::ffi::c_double,
                        1.0e+01f64,
                        0.0f64,
                    );
                }
            } else {
                while e <= -100 as ::core::ffi::c_int {
                    e += 100 as ::core::ffi::c_int;
                    dekkerMul2(
                        &raw mut rr as *mut ::core::ffi::c_double
                            as *mut ::core::ffi::c_double,
                        1.0e-100f64,
                        -1.99918998026028836196e-117f64,
                    );
                }
                while e <= -10 as ::core::ffi::c_int {
                    e += 10 as ::core::ffi::c_int;
                    dekkerMul2(
                        &raw mut rr as *mut ::core::ffi::c_double
                            as *mut ::core::ffi::c_double,
                        1.0e-10f64,
                        -3.6432197315497741579e-27f64,
                    );
                }
                while e <= -1 as ::core::ffi::c_int {
                    e += 1 as ::core::ffi::c_int;
                    dekkerMul2(
                        &raw mut rr as *mut ::core::ffi::c_double
                            as *mut ::core::ffi::c_double,
                        1.0e-01f64,
                        -5.5511151231257827021e-18f64,
                    );
                }
            }
            *pResult = rr[0 as ::core::ffi::c_int as usize]
                + rr[1 as ::core::ffi::c_int as usize];
            if sqlite3IsNaN(*pResult) != 0 {
                *pResult = 1e300f64 * 1e300f64;
            }
            if sign < 0 as ::core::ffi::c_int {
                *pResult = -*pResult;
            }
        }
        if z == zEnd && nDigit > 0 as ::core::ffi::c_int && eValid != 0
            && eType > 0 as ::core::ffi::c_int
        {
            return eType
        } else if eType >= 2 as ::core::ffi::c_int
            && (eType == 3 as ::core::ffi::c_int || eValid != 0)
            && nDigit > 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int
        } else {
            return 0 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Int64ToText(
    mut v: i64_0,
    mut zOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut x: u64_0 = 0;
        let mut zTemp: [::core::ffi::c_char; 22] = [0; 22];
        if v < 0 as ::core::ffi::c_longlong {
            x = (if v == SMALLEST_INT64 {
                (1 as ::core::ffi::c_int as ::core::ffi::c_ulonglong)
                    << 63 as ::core::ffi::c_int
            } else {
                -v as ::core::ffi::c_ulonglong
            }) as u64_0;
        } else {
            x = v as u64_0;
        }
        i = (::core::mem::size_of::<[::core::ffi::c_char; 22]>() as usize)
            .wrapping_sub(2 as usize) as ::core::ffi::c_int;
        zTemp[(::core::mem::size_of::<[::core::ffi::c_char; 22]>() as usize)
            .wrapping_sub(1 as usize) as usize] = 0 as ::core::ffi::c_char;
        loop {
            zTemp[i as usize] = (x as ::core::ffi::c_ulonglong)
                .wrapping_rem(10 as ::core::ffi::c_ulonglong)
                .wrapping_add('0' as i32 as ::core::ffi::c_ulonglong)
                as ::core::ffi::c_char;
            x = (x as ::core::ffi::c_ulonglong)
                .wrapping_div(10 as ::core::ffi::c_ulonglong) as u64_0;
            if x == 0 as ::core::ffi::c_ulonglong {
                break;
            }
            i -= 1;
        }
        if v < 0 as ::core::ffi::c_longlong {
            i -= 1;
            zTemp[i as usize] = '-' as i32 as ::core::ffi::c_char;
        }
        memcpy(
            zOut as *mut ::core::ffi::c_void,
            (&raw mut zTemp as *mut ::core::ffi::c_char).offset(i as isize)
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            (::core::mem::size_of::<[::core::ffi::c_char; 22]>() as size_t)
                .wrapping_sub(i as size_t),
        );
        return (::core::mem::size_of::<[::core::ffi::c_char; 22]>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_sub(i as usize) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn compare2pow63(
    mut zNum: *const ::core::ffi::c_char,
    mut incr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut pow63: *const ::core::ffi::c_char = b"922337203685477580\0".as_ptr()
            as *const ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while c == 0 as ::core::ffi::c_int && i < 18 as ::core::ffi::c_int {
            c = (*zNum.offset((i * incr) as isize) as ::core::ffi::c_int
                - *pow63.offset(i as isize) as ::core::ffi::c_int)
                * 10 as ::core::ffi::c_int;
            i += 1;
        }
        if c == 0 as ::core::ffi::c_int {
            c = *zNum.offset((18 as ::core::ffi::c_int * incr) as isize)
                as ::core::ffi::c_int - '8' as i32;
        }
        return c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Atoi64(
    mut zNum: *const ::core::ffi::c_char,
    mut pNum: *mut i64_0,
    mut length: ::core::ffi::c_int,
    mut enc: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut incr: ::core::ffi::c_int = 0;
        let mut u: u64_0 = 0 as u64_0;
        let mut neg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nonNum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = 0;
        let mut zStart: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zEnd: *const ::core::ffi::c_char = zNum.offset(length as isize);
        if enc as ::core::ffi::c_int == SQLITE_UTF8 {
            incr = 1 as ::core::ffi::c_int;
        } else {
            incr = 2 as ::core::ffi::c_int;
            length &= !(1 as ::core::ffi::c_int);
            i = 3 as ::core::ffi::c_int - enc as ::core::ffi::c_int;
            while i < length
                && *zNum.offset(i as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                i += 2 as ::core::ffi::c_int;
            }
            nonNum = (i < length) as ::core::ffi::c_int;
            zEnd = zNum.offset((i ^ 1 as ::core::ffi::c_int) as isize)
                as *const ::core::ffi::c_char;
            zNum = zNum
                .offset((enc as ::core::ffi::c_int & 1 as ::core::ffi::c_int) as isize);
        }
        while zNum < zEnd
            && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(*zNum as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                & 0x1 as ::core::ffi::c_int != 0
        {
            zNum = zNum.offset(incr as isize);
        }
        if zNum < zEnd {
            if *zNum as ::core::ffi::c_int == '-' as i32 {
                neg = 1 as ::core::ffi::c_int;
                zNum = zNum.offset(incr as isize);
            } else if *zNum as ::core::ffi::c_int == '+' as i32 {
                zNum = zNum.offset(incr as isize);
            }
        }
        zStart = zNum;
        while zNum < zEnd
            && *zNum.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '0' as i32
        {
            zNum = zNum.offset(incr as isize);
        }
        i = 0 as ::core::ffi::c_int;
        while (zNum.offset(i as isize) as *const ::core::ffi::c_char) < zEnd
            && {
                c = *zNum.offset(i as isize) as ::core::ffi::c_int;
                c >= '0' as i32
            } && c <= '9' as i32
        {
            u = (u as ::core::ffi::c_ulonglong)
                .wrapping_mul(10 as ::core::ffi::c_ulonglong)
                .wrapping_add(c as ::core::ffi::c_ulonglong)
                .wrapping_sub('0' as i32 as ::core::ffi::c_ulonglong) as u64_0;
            i += incr;
        }
        if u > LARGEST_INT64 as ::core::ffi::c_ulonglong {
            *pNum = (if neg != 0 { SMALLEST_INT64 } else { LARGEST_INT64 }) as i64_0;
        } else if neg != 0 {
            *pNum = -(u as i64_0);
        } else {
            *pNum = u as i64_0;
        }
        rc = 0 as ::core::ffi::c_int;
        if i == 0 as ::core::ffi::c_int && zStart == zNum {
            rc = -1 as ::core::ffi::c_int;
        } else if nonNum != 0 {
            rc = 1 as ::core::ffi::c_int;
        } else if (zNum.offset(i as isize) as *const ::core::ffi::c_char) < zEnd {
            let mut jj: ::core::ffi::c_int = i;
            loop {
                if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(*zNum.offset(jj as isize) as ::core::ffi::c_uchar as isize)
                    as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0
                {
                    rc = 1 as ::core::ffi::c_int;
                    break;
                } else {
                    jj += incr;
                    if !((zNum.offset(jj as isize) as *const ::core::ffi::c_char) < zEnd)
                    {
                        break;
                    }
                }
            }
        }
        if i < 19 as ::core::ffi::c_int * incr {
            return rc
        } else {
            c = if i > 19 as ::core::ffi::c_int * incr {
                1 as ::core::ffi::c_int
            } else {
                compare2pow63(zNum, incr)
            };
            if c < 0 as ::core::ffi::c_int {
                return rc
            } else {
                *pNum = (if neg != 0 { SMALLEST_INT64 } else { LARGEST_INT64 }) as i64_0;
                if c > 0 as ::core::ffi::c_int {
                    return 2 as ::core::ffi::c_int
                } else {
                    return if neg != 0 { rc } else { 3 as ::core::ffi::c_int }
                }
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DecOrHexToI64(
    mut z: *const ::core::ffi::c_char,
    mut pOut: *mut i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '0' as i32
            && (*z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'x' as i32
                || *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'X' as i32)
        {
            let mut u: u64_0 = 0 as u64_0;
            let mut i: ::core::ffi::c_int = 0;
            let mut k: ::core::ffi::c_int = 0;
            i = 2 as ::core::ffi::c_int;
            while *z.offset(i as isize) as ::core::ffi::c_int == '0' as i32 {
                i += 1;
            }
            k = i;
            while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(*z.offset(k as isize) as ::core::ffi::c_uchar as isize)
                as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
            {
                u = (u as ::core::ffi::c_ulonglong)
                    .wrapping_mul(16 as ::core::ffi::c_ulonglong)
                    .wrapping_add(
                        sqlite3HexToInt(*z.offset(k as isize) as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    ) as u64_0;
                k += 1;
            }
            memcpy(
                pOut as *mut ::core::ffi::c_void,
                &raw mut u as *const ::core::ffi::c_void,
                8 as size_t,
            );
            if k - i > 16 as ::core::ffi::c_int {
                return 2 as ::core::ffi::c_int;
            }
            if *z.offset(k as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                return 1 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        } else {
            let mut n: ::core::ffi::c_int = (0x3fffffff as ::core::ffi::c_ulong
                & strspn(
                    z,
                    b"+- \n\t0123456789\0".as_ptr() as *const ::core::ffi::c_char,
                )) as ::core::ffi::c_int;
            if *z.offset(n as isize) != 0 {
                n += 1;
            }
            return sqlite3Atoi64(z, pOut, n, SQLITE_UTF8 as u8_0);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GetInt32(
    mut zNum: *const ::core::ffi::c_char,
    mut pValue: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: sqlite_int64 = 0 as sqlite_int64;
        let mut i: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        let mut neg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if *zNum.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32
        {
            neg = 1 as ::core::ffi::c_int;
            zNum = zNum.offset(1);
        } else if *zNum.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '+' as i32
        {
            zNum = zNum.offset(1);
        } else if *zNum.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '0' as i32
            && (*zNum.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'x' as i32
                || *zNum.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'X' as i32)
            && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(
                    *zNum.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar as isize,
                ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
        {
            let mut u: u32_0 = 0 as u32_0;
            zNum = zNum.offset(2 as ::core::ffi::c_int as isize);
            while *zNum.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '0' as i32
            {
                zNum = zNum.offset(1);
            }
            i = 0 as ::core::ffi::c_int;
            while i < 8 as ::core::ffi::c_int
                && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(*zNum.offset(i as isize) as ::core::ffi::c_uchar as isize)
                    as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
            {
                u = (u as ::core::ffi::c_uint)
                    .wrapping_mul(16 as ::core::ffi::c_uint)
                    .wrapping_add(
                        sqlite3HexToInt(*zNum.offset(i as isize) as ::core::ffi::c_int)
                            as ::core::ffi::c_uint,
                    ) as u32_0;
                i += 1;
            }
            if u as ::core::ffi::c_uint & 0x80000000 as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
                && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(*zNum.offset(i as isize) as ::core::ffi::c_uchar as isize)
                    as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                memcpy(
                    pValue as *mut ::core::ffi::c_void,
                    &raw mut u as *const ::core::ffi::c_void,
                    4 as size_t,
                );
                return 1 as ::core::ffi::c_int;
            } else {
                return 0 as ::core::ffi::c_int
            }
        }
        if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(
                *zNum.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                    as isize,
            ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        while *zNum.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '0' as i32
        {
            zNum = zNum.offset(1);
        }
        i = 0 as ::core::ffi::c_int;
        while i < 11 as ::core::ffi::c_int
            && {
                c = *zNum.offset(i as isize) as ::core::ffi::c_int - '0' as i32;
                c >= 0 as ::core::ffi::c_int
            } && c <= 9 as ::core::ffi::c_int
        {
            v = (v as ::core::ffi::c_longlong * 10 as ::core::ffi::c_longlong
                + c as ::core::ffi::c_longlong) as sqlite_int64;
            i += 1;
        }
        if i > 10 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if v as ::core::ffi::c_longlong - neg as ::core::ffi::c_longlong
            > 2147483647 as ::core::ffi::c_longlong
        {
            return 0 as ::core::ffi::c_int;
        }
        if neg != 0 {
            v = -v;
        }
        *pValue = v as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Atoi(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        sqlite3GetInt32(z, &raw mut x);
        return x;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FpDecode(
    mut p: *mut FpDecode,
    mut r: ::core::ffi::c_double,
    mut iRound: ::core::ffi::c_int,
    mut mxRound: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut v: u64_0 = 0;
        let mut e: ::core::ffi::c_int = 0;
        let mut exp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rr: [::core::ffi::c_double; 2] = [0.; 2];
        (*p).isSpecial = 0 as ::core::ffi::c_char;
        (*p).z = &raw mut (*p).zBuf as *mut ::core::ffi::c_char;
        if r < 0.0f64 {
            (*p).sign = '-' as i32 as ::core::ffi::c_char;
            r = -r;
        } else if r == 0.0f64 {
            (*p).sign = '+' as i32 as ::core::ffi::c_char;
            (*p).n = 1 as ::core::ffi::c_int;
            (*p).iDP = 1 as ::core::ffi::c_int;
            (*p).z = b"0\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            return;
        } else {
            (*p).sign = '+' as i32 as ::core::ffi::c_char;
        }
        memcpy(
            &raw mut v as *mut ::core::ffi::c_void,
            &raw mut r as *const ::core::ffi::c_void,
            8 as size_t,
        );
        e = (v >> 52 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if e & 0x7ff as ::core::ffi::c_int == 0x7ff as ::core::ffi::c_int {
            (*p).isSpecial = (1 as ::core::ffi::c_int
                + (v != 0x7ff0000000000000 as ::core::ffi::c_ulonglong)
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            (*p).n = 0 as ::core::ffi::c_int;
            (*p).iDP = 0 as ::core::ffi::c_int;
            return;
        }
        rr[0 as ::core::ffi::c_int as usize] = r;
        rr[1 as ::core::ffi::c_int as usize] = 0.0f64;
        if rr[0 as ::core::ffi::c_int as usize] > 9.223372036854774784e+18f64 {
            while rr[0 as ::core::ffi::c_int as usize] > 9.223372036854774784e+118f64 {
                exp += 100 as ::core::ffi::c_int;
                dekkerMul2(
                    &raw mut rr as *mut ::core::ffi::c_double
                        as *mut ::core::ffi::c_double,
                    1.0e-100f64,
                    -1.99918998026028836196e-117f64,
                );
            }
            while rr[0 as ::core::ffi::c_int as usize] > 9.223372036854774784e+28f64 {
                exp += 10 as ::core::ffi::c_int;
                dekkerMul2(
                    &raw mut rr as *mut ::core::ffi::c_double
                        as *mut ::core::ffi::c_double,
                    1.0e-10f64,
                    -3.6432197315497741579e-27f64,
                );
            }
            while rr[0 as ::core::ffi::c_int as usize] > 9.223372036854774784e+18f64 {
                exp += 1 as ::core::ffi::c_int;
                dekkerMul2(
                    &raw mut rr as *mut ::core::ffi::c_double
                        as *mut ::core::ffi::c_double,
                    1.0e-01f64,
                    -5.5511151231257827021e-18f64,
                );
            }
        } else {
            while rr[0 as ::core::ffi::c_int as usize] < 9.223372036854774784e-83f64 {
                exp -= 100 as ::core::ffi::c_int;
                dekkerMul2(
                    &raw mut rr as *mut ::core::ffi::c_double
                        as *mut ::core::ffi::c_double,
                    1.0e+100f64,
                    -1.5902891109759918046e+83f64,
                );
            }
            while rr[0 as ::core::ffi::c_int as usize] < 9.223372036854774784e+07f64 {
                exp -= 10 as ::core::ffi::c_int;
                dekkerMul2(
                    &raw mut rr as *mut ::core::ffi::c_double
                        as *mut ::core::ffi::c_double,
                    1.0e+10f64,
                    0.0f64,
                );
            }
            while rr[0 as ::core::ffi::c_int as usize] < 9.22337203685477478e+17f64 {
                exp -= 1 as ::core::ffi::c_int;
                dekkerMul2(
                    &raw mut rr as *mut ::core::ffi::c_double
                        as *mut ::core::ffi::c_double,
                    1.0e+01f64,
                    0.0f64,
                );
            }
        }
        v = (if rr[1 as ::core::ffi::c_int as usize] < 0.0f64 {
            (rr[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulonglong)
                .wrapping_sub(
                    -rr[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulonglong,
                )
        } else {
            (rr[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulonglong)
                .wrapping_add(
                    rr[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulonglong,
                )
        }) as u64_0;
        i = (::core::mem::size_of::<[::core::ffi::c_char; 24]>() as usize)
            .wrapping_sub(1 as usize) as ::core::ffi::c_int;
        while v != 0 {
            let c2rust_fresh1 = i;
            i = i - 1;
            (*p).zBuf[c2rust_fresh1 as usize] = (v as ::core::ffi::c_ulonglong)
                .wrapping_rem(10 as ::core::ffi::c_ulonglong)
                .wrapping_add('0' as i32 as ::core::ffi::c_ulonglong)
                as ::core::ffi::c_char;
            v = (v as ::core::ffi::c_ulonglong)
                .wrapping_div(10 as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
        }
        (*p).n = (::core::mem::size_of::<[::core::ffi::c_char; 24]>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_sub(i as usize) as ::core::ffi::c_int;
        (*p).iDP = (*p).n + exp;
        if iRound <= 0 as ::core::ffi::c_int {
            iRound = (*p).iDP - iRound;
            if iRound == 0 as ::core::ffi::c_int
                && (*p).zBuf[(i + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int >= '5' as i32
            {
                iRound = 1 as ::core::ffi::c_int;
                let c2rust_fresh2 = i;
                i = i - 1;
                (*p).zBuf[c2rust_fresh2 as usize] = '0' as i32 as ::core::ffi::c_char;
                (*p).n += 1;
                (*p).iDP += 1;
            }
        }
        if iRound > 0 as ::core::ffi::c_int && (iRound < (*p).n || (*p).n > mxRound) {
            let mut z: *mut ::core::ffi::c_char = (&raw mut (*p).zBuf
                as *mut ::core::ffi::c_char)
                .offset((i + 1 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_char;
            if iRound > mxRound {
                iRound = mxRound;
            }
            (*p).n = iRound;
            if *z.offset(iRound as isize) as ::core::ffi::c_int >= '5' as i32 {
                let mut j: ::core::ffi::c_int = iRound - 1 as ::core::ffi::c_int;
                loop {
                    let ref mut c2rust_fresh3 = *z.offset(j as isize);
                    *c2rust_fresh3 += 1;
                    if *z.offset(j as isize) as ::core::ffi::c_int <= '9' as i32 {
                        break;
                    }
                    *z.offset(j as isize) = '0' as i32 as ::core::ffi::c_char;
                    if j == 0 as ::core::ffi::c_int {
                        let c2rust_fresh4 = i;
                        i = i - 1;
                        *(*p).z.offset(c2rust_fresh4 as isize) = '1' as i32
                            as ::core::ffi::c_char;
                        (*p).n += 1;
                        (*p).iDP += 1;
                        break;
                    } else {
                        j -= 1;
                    }
                }
            }
        }
        (*p).z = (&raw mut (*p).zBuf as *mut ::core::ffi::c_char)
            .offset((i + 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_char;
        while *(*p).z.offset(((*p).n - 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int == '0' as i32
        {
            (*p).n -= 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GetUInt32(
    mut z: *const ::core::ffi::c_char,
    mut pI: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: u64_0 = 0 as u64_0;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(*z.offset(i as isize) as ::core::ffi::c_uchar as isize)
            as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
        {
            v = (v as ::core::ffi::c_ulonglong)
                .wrapping_mul(10 as ::core::ffi::c_ulonglong)
                .wrapping_add(*z.offset(i as isize) as ::core::ffi::c_ulonglong)
                .wrapping_sub('0' as i32 as ::core::ffi::c_ulonglong) as u64_0;
            if v > 4294967296 as ::core::ffi::c_ulonglong {
                *pI = 0 as u32_0;
                return 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int
            || *z.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            *pI = 0 as u32_0;
            return 0 as ::core::ffi::c_int;
        }
        *pI = v as u32_0;
        return 1 as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn putVarint64(
    mut p: *mut ::core::ffi::c_uchar,
    mut v: u64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut buf: [u8_0; 10] = [0; 10];
        if v & (0xff000000 as ::core::ffi::c_uint as u64_0) << 32 as ::core::ffi::c_int
            != 0
        {
            *p.offset(8 as ::core::ffi::c_int as isize) = v as u8_0
                as ::core::ffi::c_uchar;
            v >>= 8 as ::core::ffi::c_int;
            i = 7 as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                *p.offset(i as isize) = (v as ::core::ffi::c_ulonglong
                    & 0x7f as ::core::ffi::c_ulonglong
                    | 0x80 as ::core::ffi::c_ulonglong) as u8_0 as ::core::ffi::c_uchar;
                v >>= 7 as ::core::ffi::c_int;
                i -= 1;
            }
            return 9 as ::core::ffi::c_int;
        }
        n = 0 as ::core::ffi::c_int;
        loop {
            let c2rust_fresh10 = n;
            n = n + 1;
            buf[c2rust_fresh10 as usize] = (v as ::core::ffi::c_ulonglong
                & 0x7f as ::core::ffi::c_ulonglong | 0x80 as ::core::ffi::c_ulonglong)
                as u8_0;
            v >>= 7 as ::core::ffi::c_int;
            if !(v != 0 as ::core::ffi::c_ulonglong) {
                break;
            }
        }
        buf[0 as ::core::ffi::c_int as usize] = (buf[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as u8_0;
        i = 0 as ::core::ffi::c_int;
        j = n - 1 as ::core::ffi::c_int;
        while j >= 0 as ::core::ffi::c_int {
            *p.offset(i as isize) = buf[j as usize] as ::core::ffi::c_uchar;
            j -= 1;
            i += 1;
        }
        return n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PutVarint(
    mut p: *mut ::core::ffi::c_uchar,
    mut v: u64_0,
) -> ::core::ffi::c_int {
    unsafe {
        if v <= 0x7f as ::core::ffi::c_ulonglong {
            *p.offset(0 as ::core::ffi::c_int as isize) = (v as ::core::ffi::c_ulonglong
                & 0x7f as ::core::ffi::c_ulonglong) as ::core::ffi::c_uchar;
            return 1 as ::core::ffi::c_int;
        }
        if v <= 0x3fff as ::core::ffi::c_ulonglong {
            *p.offset(0 as ::core::ffi::c_int as isize) = (v as ::core::ffi::c_ulonglong
                >> 7 as ::core::ffi::c_int & 0x7f as ::core::ffi::c_ulonglong
                | 0x80 as ::core::ffi::c_ulonglong) as ::core::ffi::c_uchar;
            *p.offset(1 as ::core::ffi::c_int as isize) = (v as ::core::ffi::c_ulonglong
                & 0x7f as ::core::ffi::c_ulonglong) as ::core::ffi::c_uchar;
            return 2 as ::core::ffi::c_int;
        }
        return putVarint64(p, v);
    }
}
pub const SLOT_2_0: ::core::ffi::c_int = 0x1fc07f as ::core::ffi::c_int;
pub const SLOT_4_2_0: ::core::ffi::c_uint = 0xf01fc07f as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn sqlite3GetVarint(
    mut p: *const ::core::ffi::c_uchar,
    mut v: *mut u64_0,
) -> u8_0 {
    unsafe {
        let mut a: u32_0 = 0;
        let mut b: u32_0 = 0;
        let mut s: u32_0 = 0;
        if *(p as *mut ::core::ffi::c_schar).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        {
            *v = *p as u64_0;
            return 1 as u8_0;
        }
        if *(p as *mut ::core::ffi::c_schar).offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        {
            *v = (((*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x7f as ::core::ffi::c_int) as ::core::ffi::c_uint)
                << 7 as ::core::ffi::c_int
                | *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                as u64_0;
            return 2 as u8_0;
        }
        a = (*p.offset(0 as ::core::ffi::c_int as isize) as u32_0)
            << 14 as ::core::ffi::c_int;
        b = *p.offset(1 as ::core::ffi::c_int as isize) as u32_0;
        p = p.offset(2 as ::core::ffi::c_int as isize);
        a |= *p as ::core::ffi::c_uint;
        if a as ::core::ffi::c_uint & 0x80 as ::core::ffi::c_uint == 0 {
            a &= SLOT_2_0 as ::core::ffi::c_uint;
            b &= 0x7f as ::core::ffi::c_uint;
            b = b << 7 as ::core::ffi::c_int;
            a |= b as ::core::ffi::c_uint;
            *v = a as u64_0;
            return 3 as u8_0;
        }
        a &= SLOT_2_0 as ::core::ffi::c_uint;
        p = p.offset(1);
        b = b << 14 as ::core::ffi::c_int;
        b |= *p as ::core::ffi::c_uint;
        if b as ::core::ffi::c_uint & 0x80 as ::core::ffi::c_uint == 0 {
            b &= SLOT_2_0 as ::core::ffi::c_uint;
            a = a << 7 as ::core::ffi::c_int;
            a |= b as ::core::ffi::c_uint;
            *v = a as u64_0;
            return 4 as u8_0;
        }
        b &= SLOT_2_0 as ::core::ffi::c_uint;
        s = a;
        p = p.offset(1);
        a = a << 14 as ::core::ffi::c_int;
        a |= *p as ::core::ffi::c_uint;
        if a as ::core::ffi::c_uint & 0x80 as ::core::ffi::c_uint == 0 {
            b = b << 7 as ::core::ffi::c_int;
            a |= b as ::core::ffi::c_uint;
            s = s >> 18 as ::core::ffi::c_int;
            *v = ((s as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int
                | a as ::core::ffi::c_ulonglong) as u64_0;
            return 5 as u8_0;
        }
        s = s << 7 as ::core::ffi::c_int;
        s |= b as ::core::ffi::c_uint;
        p = p.offset(1);
        b = b << 14 as ::core::ffi::c_int;
        b |= *p as ::core::ffi::c_uint;
        if b as ::core::ffi::c_uint & 0x80 as ::core::ffi::c_uint == 0 {
            a &= SLOT_2_0 as ::core::ffi::c_uint;
            a = a << 7 as ::core::ffi::c_int;
            a |= b as ::core::ffi::c_uint;
            s = s >> 18 as ::core::ffi::c_int;
            *v = ((s as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int
                | a as ::core::ffi::c_ulonglong) as u64_0;
            return 6 as u8_0;
        }
        p = p.offset(1);
        a = a << 14 as ::core::ffi::c_int;
        a |= *p as ::core::ffi::c_uint;
        if a as ::core::ffi::c_uint & 0x80 as ::core::ffi::c_uint == 0 {
            a &= SLOT_4_2_0;
            b &= SLOT_2_0 as ::core::ffi::c_uint;
            b = b << 7 as ::core::ffi::c_int;
            a |= b as ::core::ffi::c_uint;
            s = s >> 11 as ::core::ffi::c_int;
            *v = ((s as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int
                | a as ::core::ffi::c_ulonglong) as u64_0;
            return 7 as u8_0;
        }
        a &= SLOT_2_0 as ::core::ffi::c_uint;
        p = p.offset(1);
        b = b << 14 as ::core::ffi::c_int;
        b |= *p as ::core::ffi::c_uint;
        if b as ::core::ffi::c_uint & 0x80 as ::core::ffi::c_uint == 0 {
            b &= SLOT_4_2_0;
            a = a << 7 as ::core::ffi::c_int;
            a |= b as ::core::ffi::c_uint;
            s = s >> 4 as ::core::ffi::c_int;
            *v = ((s as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int
                | a as ::core::ffi::c_ulonglong) as u64_0;
            return 8 as u8_0;
        }
        p = p.offset(1);
        a = a << 15 as ::core::ffi::c_int;
        a |= *p as ::core::ffi::c_uint;
        b &= SLOT_2_0 as ::core::ffi::c_uint;
        b = b << 8 as ::core::ffi::c_int;
        a |= b as ::core::ffi::c_uint;
        s = s << 4 as ::core::ffi::c_int;
        b = *p.offset(-4 as ::core::ffi::c_int as isize) as u32_0;
        b &= 0x7f as ::core::ffi::c_uint;
        b = b >> 3 as ::core::ffi::c_int;
        s |= b as ::core::ffi::c_uint;
        *v = ((s as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int
            | a as ::core::ffi::c_ulonglong) as u64_0;
        return 9 as u8_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GetVarint32(
    mut p: *const ::core::ffi::c_uchar,
    mut v: *mut u32_0,
) -> u8_0 {
    unsafe {
        let mut v64: u64_0 = 0;
        let mut n: u8_0 = 0;
        if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            *v = ((*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x7f as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as u32_0;
            return 2 as u8_0;
        }
        if *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            *v = ((*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x7f as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
                | (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x7f as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as u32_0;
            return 3 as u8_0;
        }
        n = sqlite3GetVarint(p, &raw mut v64);
        if v64 as ::core::ffi::c_ulonglong & SQLITE_MAX_U32 != v64 {
            *v = 0xffffffff as ::core::ffi::c_uint as u32_0;
        } else {
            *v = v64 as u32_0;
        }
        return n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VarintLen(mut v: u64_0) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 1 as ::core::ffi::c_int;
        loop {
            v >>= 7 as ::core::ffi::c_int;
            if !(v != 0 as ::core::ffi::c_ulonglong) {
                break;
            }
            i += 1;
        }
        return i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Get4byte(mut p: *const u8_0) -> u32_0 {
    unsafe {
        return (*p.offset(0 as ::core::ffi::c_int as isize) as u32_0)
            << 24 as ::core::ffi::c_int
            | ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as u32_0
            | ((*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as u32_0
            | *p.offset(3 as ::core::ffi::c_int as isize) as u32_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Put4byte(
    mut p: *mut ::core::ffi::c_uchar,
    mut v: u32_0,
) {
    unsafe {
        *p.offset(0 as ::core::ffi::c_int as isize) = (v >> 24 as ::core::ffi::c_int)
            as u8_0 as ::core::ffi::c_uchar;
        *p.offset(1 as ::core::ffi::c_int as isize) = (v >> 16 as ::core::ffi::c_int)
            as u8_0 as ::core::ffi::c_uchar;
        *p.offset(2 as ::core::ffi::c_int as isize) = (v >> 8 as ::core::ffi::c_int)
            as u8_0 as ::core::ffi::c_uchar;
        *p.offset(3 as ::core::ffi::c_int as isize) = v as u8_0 as ::core::ffi::c_uchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3HexToInt(mut h: ::core::ffi::c_int) -> u8_0 {
    unsafe {
        h
            += 9 as ::core::ffi::c_int
                * (1 as ::core::ffi::c_int & h >> 6 as ::core::ffi::c_int);
        return (h & 0xf as ::core::ffi::c_int) as u8_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3HexToBlob(
    mut db: *mut sqlite3,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut zBlob: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        zBlob = sqlite3DbMallocRawNN(
            db,
            (n / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u64_0,
        ) as *mut ::core::ffi::c_char;
        n -= 1;
        if !zBlob.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < n {
                *zBlob.offset((i / 2 as ::core::ffi::c_int) as isize) = ((sqlite3HexToInt(
                    *z.offset(i as isize) as ::core::ffi::c_int,
                ) as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | sqlite3HexToInt(
                        *z.offset((i + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int) as ::core::ffi::c_char;
                i += 2 as ::core::ffi::c_int;
            }
            *zBlob.offset((i / 2 as ::core::ffi::c_int) as isize) = 0
                as ::core::ffi::c_char;
        }
        return zBlob as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn logBadConnection(mut zType: *const ::core::ffi::c_char) {
    unsafe {
        sqlite3_log(
            SQLITE_MISUSE,
            b"API call with %s database connection pointer\0".as_ptr()
                as *const ::core::ffi::c_char,
            zType,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SafetyCheckOk(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eOpenState: u8_0 = 0;
        if db.is_null() {
            logBadConnection(b"NULL\0".as_ptr() as *const ::core::ffi::c_char);
            return 0 as ::core::ffi::c_int;
        }
        eOpenState = (*db).eOpenState;
        if eOpenState as ::core::ffi::c_int != SQLITE_STATE_OPEN {
            if sqlite3SafetyCheckSickOrOk(db) != 0 {
                logBadConnection(b"unopened\0".as_ptr() as *const ::core::ffi::c_char);
            }
            return 0 as ::core::ffi::c_int;
        } else {
            return 1 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SafetyCheckSickOrOk(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eOpenState: u8_0 = 0;
        eOpenState = (*db).eOpenState;
        if eOpenState as ::core::ffi::c_int != SQLITE_STATE_SICK
            && eOpenState as ::core::ffi::c_int != SQLITE_STATE_OPEN
            && eOpenState as ::core::ffi::c_int != SQLITE_STATE_BUSY
        {
            logBadConnection(b"invalid\0".as_ptr() as *const ::core::ffi::c_char);
            return 0 as ::core::ffi::c_int;
        } else {
            return 1 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AddInt64(
    mut pA: *mut i64_0,
    mut iB: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iA: i64_0 = *pA;
        if iB >= 0 as ::core::ffi::c_longlong {
            if iA > 0 as ::core::ffi::c_longlong && LARGEST_INT64 - iA < iB {
                return 1 as ::core::ffi::c_int;
            }
        } else if iA < 0 as ::core::ffi::c_longlong
            && -(iA + LARGEST_INT64)
                > iB as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong
        {
            return 1 as ::core::ffi::c_int
        }
        *pA += iB as ::core::ffi::c_longlong;
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SubInt64(
    mut pA: *mut i64_0,
    mut iB: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        if iB == SMALLEST_INT64 {
            if *pA >= 0 as ::core::ffi::c_longlong {
                return 1 as ::core::ffi::c_int;
            }
            *pA -= iB as ::core::ffi::c_longlong;
            return 0 as ::core::ffi::c_int;
        } else {
            return sqlite3AddInt64(pA, -iB)
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MulInt64(
    mut pA: *mut i64_0,
    mut iB: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iA: i64_0 = *pA;
        if iB > 0 as ::core::ffi::c_longlong {
            if iA > LARGEST_INT64 / iB {
                return 1 as ::core::ffi::c_int;
            }
            if iA < SMALLEST_INT64 / iB {
                return 1 as ::core::ffi::c_int;
            }
        } else if iB < 0 as ::core::ffi::c_longlong {
            if iA > 0 as ::core::ffi::c_longlong {
                if iB < SMALLEST_INT64 / iA {
                    return 1 as ::core::ffi::c_int;
                }
            } else if iA < 0 as ::core::ffi::c_longlong {
                if iB == SMALLEST_INT64 {
                    return 1 as ::core::ffi::c_int;
                }
                if iA == SMALLEST_INT64 {
                    return 1 as ::core::ffi::c_int;
                }
                if -iA > LARGEST_INT64 / -iB {
                    return 1 as ::core::ffi::c_int;
                }
            }
        }
        *pA = iA * iB;
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AbsInt32(
    mut x: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if x >= 0 as ::core::ffi::c_int {
            return x;
        }
        if x == 0x80000000 as ::core::ffi::c_uint as ::core::ffi::c_int {
            return 0x7fffffff as ::core::ffi::c_int;
        }
        return -x;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3LogEstAdd(mut a: LogEst, mut b: LogEst) -> LogEst {
    unsafe {
        static mut x: [::core::ffi::c_uchar; 32] = [
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ];
        if a as ::core::ffi::c_int >= b as ::core::ffi::c_int {
            if a as ::core::ffi::c_int
                > b as ::core::ffi::c_int + 49 as ::core::ffi::c_int
            {
                return a;
            }
            if a as ::core::ffi::c_int
                > b as ::core::ffi::c_int + 31 as ::core::ffi::c_int
            {
                return (a as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as LogEst;
            }
            return (a as ::core::ffi::c_int
                + x[(a as ::core::ffi::c_int - b as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int) as LogEst;
        } else {
            if b as ::core::ffi::c_int
                > a as ::core::ffi::c_int + 49 as ::core::ffi::c_int
            {
                return b;
            }
            if b as ::core::ffi::c_int
                > a as ::core::ffi::c_int + 31 as ::core::ffi::c_int
            {
                return (b as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as LogEst;
            }
            return (b as ::core::ffi::c_int
                + x[(b as ::core::ffi::c_int - a as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int) as LogEst;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3LogEst(mut x: u64_0) -> LogEst {
    unsafe {
        static mut a: [LogEst; 8] = [
            0 as ::core::ffi::c_int as LogEst,
            2 as ::core::ffi::c_int as LogEst,
            3 as ::core::ffi::c_int as LogEst,
            5 as ::core::ffi::c_int as LogEst,
            6 as ::core::ffi::c_int as LogEst,
            7 as ::core::ffi::c_int as LogEst,
            8 as ::core::ffi::c_int as LogEst,
            9 as ::core::ffi::c_int as LogEst,
        ];
        let mut y: LogEst = 40 as LogEst;
        if x < 8 as ::core::ffi::c_ulonglong {
            if x < 2 as ::core::ffi::c_ulonglong {
                return 0 as LogEst;
            }
            while x < 8 as ::core::ffi::c_ulonglong {
                y = (y as ::core::ffi::c_int - 10 as ::core::ffi::c_int) as LogEst;
                x <<= 1 as ::core::ffi::c_int;
            }
        } else {
            while x > 255 as ::core::ffi::c_ulonglong {
                y = (y as ::core::ffi::c_int + 40 as ::core::ffi::c_int) as LogEst;
                x >>= 4 as ::core::ffi::c_int;
            }
            while x > 15 as ::core::ffi::c_ulonglong {
                y = (y as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as LogEst;
                x >>= 1 as ::core::ffi::c_int;
            }
        }
        return (a[(x as ::core::ffi::c_ulonglong & 7 as ::core::ffi::c_ulonglong)
            as usize] as ::core::ffi::c_int + y as ::core::ffi::c_int
            - 10 as ::core::ffi::c_int) as LogEst;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3LogEstFromDouble(
    mut x: ::core::ffi::c_double,
) -> LogEst {
    unsafe {
        let mut a: u64_0 = 0;
        let mut e: LogEst = 0;
        if x <= 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            return 0 as LogEst;
        }
        if x <= 2000000000 as ::core::ffi::c_int as ::core::ffi::c_double {
            return sqlite3LogEst(x as u64_0);
        }
        memcpy(
            &raw mut a as *mut ::core::ffi::c_void,
            &raw mut x as *const ::core::ffi::c_void,
            8 as size_t,
        );
        e = (a as ::core::ffi::c_ulonglong >> 52 as ::core::ffi::c_int)
            .wrapping_sub(1022 as ::core::ffi::c_ulonglong) as LogEst;
        return (e as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as LogEst;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3LogEstToInt(mut x: LogEst) -> u64_0 {
    unsafe {
        let mut n: u64_0 = 0;
        n = (x as ::core::ffi::c_int % 10 as ::core::ffi::c_int) as u64_0;
        x = (x as ::core::ffi::c_int / 10 as ::core::ffi::c_int) as LogEst;
        if n >= 5 as ::core::ffi::c_ulonglong {
            n = (n as ::core::ffi::c_ulonglong)
                .wrapping_sub(2 as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
        } else if n >= 1 as ::core::ffi::c_ulonglong {
            n = (n as ::core::ffi::c_ulonglong)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
        }
        if x as ::core::ffi::c_int > 60 as ::core::ffi::c_int {
            return LARGEST_INT64 as u64_0;
        }
        return if x as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
            n.wrapping_add(8 as u64_0)
                << x as ::core::ffi::c_int - 3 as ::core::ffi::c_int
        } else {
            n.wrapping_add(8 as u64_0)
                >> 3 as ::core::ffi::c_int - x as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VListAdd(
    mut db: *mut sqlite3,
    mut pIn: *mut VList,
    mut zName: *const ::core::ffi::c_char,
    mut nName: ::core::ffi::c_int,
    mut iVal: ::core::ffi::c_int,
) -> *mut VList {
    unsafe {
        let mut nInt: ::core::ffi::c_int = 0;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        nInt = nName / 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int;
        if pIn.is_null()
            || *pIn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int + nInt
                > *pIn.offset(0 as ::core::ffi::c_int as isize)
        {
            let mut nAlloc: sqlite3_int64 = (if !pIn.is_null() {
                2 as sqlite3_int64
                    * *pIn.offset(0 as ::core::ffi::c_int as isize) as sqlite3_int64
            } else {
                10 as sqlite3_int64
            }) + nInt as sqlite3_int64;
            let mut pOut: *mut VList = sqlite3DbRealloc(
                db,
                pIn as *mut ::core::ffi::c_void,
                (nAlloc as u64_0)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as u64_0),
            ) as *mut VList;
            if pOut.is_null() {
                return pIn;
            }
            if pIn.is_null() {
                *pOut.offset(1 as ::core::ffi::c_int as isize) = 2 as ::core::ffi::c_int
                    as VList;
            }
            pIn = pOut;
            *pIn.offset(0 as ::core::ffi::c_int as isize) = nAlloc as VList;
        }
        i = *pIn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *pIn.offset(i as isize) = iVal as VList;
        *pIn.offset((i + 1 as ::core::ffi::c_int) as isize) = nInt as VList;
        z = pIn.offset((i + 2 as ::core::ffi::c_int) as isize) as *mut VList
            as *mut ::core::ffi::c_char;
        *pIn.offset(1 as ::core::ffi::c_int as isize) = (i + nInt) as VList;
        memcpy(
            z as *mut ::core::ffi::c_void,
            zName as *const ::core::ffi::c_void,
            nName as size_t,
        );
        *z.offset(nName as isize) = 0 as ::core::ffi::c_char;
        return pIn;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VListNumToName(
    mut pIn: *mut VList,
    mut iVal: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut mx: ::core::ffi::c_int = 0;
        if pIn.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        mx = *pIn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        i = 2 as ::core::ffi::c_int;
        loop {
            if *pIn.offset(i as isize) == iVal {
                return pIn.offset((i + 2 as ::core::ffi::c_int) as isize) as *mut VList
                    as *mut ::core::ffi::c_char;
            }
            i
                += *pIn.offset((i + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            if !(i < mx) {
                break;
            }
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VListNameToNum(
    mut pIn: *mut VList,
    mut zName: *const ::core::ffi::c_char,
    mut nName: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut mx: ::core::ffi::c_int = 0;
        if pIn.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        mx = *pIn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        i = 2 as ::core::ffi::c_int;
        loop {
            let mut z: *const ::core::ffi::c_char = pIn
                .offset((i + 2 as ::core::ffi::c_int) as isize) as *mut VList
                as *const ::core::ffi::c_char;
            if strncmp(z, zName, nName as size_t) == 0 as ::core::ffi::c_int
                && *z.offset(nName as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                return *pIn.offset(i as isize) as ::core::ffi::c_int;
            }
            i
                += *pIn.offset((i + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            if !(i < mx) {
                break;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
