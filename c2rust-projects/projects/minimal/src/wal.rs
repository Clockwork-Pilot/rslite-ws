use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type RenameToken;
    pub type Vdbe;
    pub type sqlite3_mutex;
    pub type Pager;
    pub type PCache;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
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
    fn sqlite3OsClose(_: *mut sqlite3_file);
    fn sqlite3OsRead(
        _: *mut sqlite3_file,
        _: *mut ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsWrite(
        _: *mut sqlite3_file,
        _: *const ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsTruncate(_: *mut sqlite3_file, size: i64_0) -> ::core::ffi::c_int;
    fn sqlite3OsSync(_: *mut sqlite3_file, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3OsFileSize(_: *mut sqlite3_file, pSize: *mut i64_0) -> ::core::ffi::c_int;
    fn sqlite3OsLock(_: *mut sqlite3_file, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3OsFileControl(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsFileControlHint(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    );
    fn sqlite3OsDeviceCharacteristics(id: *mut sqlite3_file) -> ::core::ffi::c_int;
    fn sqlite3OsShmMap(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsShmLock(
        id: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsShmBarrier(id: *mut sqlite3_file);
    fn sqlite3OsShmUnmap(
        id: *mut sqlite3_file,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsUnfetch(
        _: *mut sqlite3_file,
        _: i64_0,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsOpen(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsDelete(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsSleep(_: *mut sqlite3_vfs, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3SectorSize(_: *mut sqlite3_file) -> ::core::ffi::c_int;
    fn sqlite3CorruptError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3CantopenError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3MallocZero(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3Realloc(_: *mut ::core::ffi::c_void, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3FaultSim(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
    fn sqlite3Get4byte(_: *const u8_0) -> u32_0;
    fn sqlite3Put4byte(_: *mut u8_0, _: u32_0);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void

}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Wal {
    pub pVfs: *mut sqlite3_vfs,
    pub pDbFd: *mut sqlite3_file,
    pub pWalFd: *mut sqlite3_file,
    pub iCallback: u32_0,
    pub mxWalSize: i64_0,
    pub nWiData: ::core::ffi::c_int,
    pub szFirstBlock: ::core::ffi::c_int,
    pub apWiData: *mut *mut u32_0,
    pub szPage: u32_0,
    pub readLock: i16_0,
    pub syncFlags: u8_0,
    pub exclusiveMode: u8_0,
    pub writeLock: u8_0,
    pub ckptLock: u8_0,
    pub readOnly: u8_0,
    pub truncateOnCommit: u8_0,
    pub syncHeader: u8_0,
    pub padToSectorBoundary: u8_0,
    pub bShmUnreliable: u8_0,
    pub hdr: WalIndexHdr,
    pub minFrame: u32_0,
    pub iReCksum: u32_0,
    pub zWalName: *const ::core::ffi::c_char,
    pub nCkpt: u32_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WalIndexHdr {
    pub iVersion: u32_0,
    pub unused: u32_0,
    pub iChange: u32_0,
    pub isInit: u8_0,
    pub bigEndCksum: u8_0,
    pub szPage: u16_0,
    pub mxFrame: u32_0,
    pub nPage: u32_0,
    pub aFrameCksum: [u32_0; 2],
    pub aSalt: [u32_0; 2],
    pub aCksum: [u32_0; 2]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WalCkptInfo {
    pub nBackfill: u32_0,
    pub aReadMark: [u32_0; 5],
    pub aLock: [u8_0; 8],
    pub nBackfillAttempted: u32_0,
    pub notUsed0: u32_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WalIterator {
    pub iPrior: u32_0,
    pub nSegment: ::core::ffi::c_int,
    pub aSegment: [WalSegment; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WalSegment {
    pub iNext: ::core::ffi::c_int,
    pub aIndex: *mut ht_slot,
    pub aPgno: *mut u32_0,
    pub nEntry: ::core::ffi::c_int,
    pub iZero: ::core::ffi::c_int

}
pub type ht_slot = u16_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WalHashLoc {
    pub aHash: *mut ht_slot,
    pub aPgno: *mut u32_0,
    pub iZero: u32_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sublist {
    pub nList: ::core::ffi::c_int,
    pub aList: *mut ht_slot

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WalWriter {
    pub pWal: *mut Wal,
    pub pFd: *mut sqlite3_file,
    pub iSyncPoint: sqlite3_int64,
    pub syncFlags: ::core::ffi::c_int,
    pub szPage: ::core::ffi::c_int

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_READONLY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_INTERRUPT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_PROTOCOL: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SQLITE_NOTICE: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const SQLITE_IOERR_SHORT_READ: ::core::ffi::c_int = SQLITE_IOERR
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_BUSY_RECOVERY: ::core::ffi::c_int = SQLITE_BUSY
    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_BUSY_SNAPSHOT: ::core::ffi::c_int = SQLITE_BUSY
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_READONLY_RECOVERY: ::core::ffi::c_int = SQLITE_READONLY
    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_READONLY_CANTINIT: ::core::ffi::c_int = SQLITE_READONLY
    | (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_NOTICE_RECOVER_WAL: ::core::ffi::c_int = SQLITE_NOTICE
    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_WAL: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SEQUENTIAL: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 0x1000
    as ::core::ffi::c_int;
pub const SQLITE_LOCK_EXCLUSIVE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_SIZE_HINT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_PERSIST_WAL: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_CKPT_DONE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_CKPT_START: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const SQLITE_SHM_UNLOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_SHM_LOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_SHM_SHARED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_SHM_EXCLUSIVE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_SHM_NLOCK: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_CHECKPOINT_NOOP: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const SQLITE_CHECKPOINT_PASSIVE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CHECKPOINT_RESTART: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_CHECKPOINT_TRUNCATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PGHDR_WAL_APPEND: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_BIGENDIAN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WAL_MAX_VERSION: ::core::ffi::c_int = 3007000 as ::core::ffi::c_int;
pub const WALINDEX_MAX_VERSION: ::core::ffi::c_int = 3007000 as ::core::ffi::c_int;
pub const WAL_WRITE_LOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WAL_ALL_BUT_WRITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WAL_CKPT_LOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WAL_RECOVER_LOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WAL_NREADER: ::core::ffi::c_int = SQLITE_SHM_NLOCK - 3 as ::core::ffi::c_int;
pub const READMARK_NOT_USED: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const WALINDEX_HDR_SIZE: usize = (::core::mem::size_of::<WalIndexHdr>() as usize)
    .wrapping_mul(2 as usize)
    .wrapping_add(::core::mem::size_of::<WalCkptInfo>() as usize);
pub const WAL_FRAME_HDRSIZE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const WAL_HDRSIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const WAL_MAGIC: ::core::ffi::c_int = 0x377f0682 as ::core::ffi::c_int;
pub const WAL_NORMAL_MODE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WAL_EXCLUSIVE_MODE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WAL_HEAPMEMORY_MODE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WAL_RDONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WAL_SHM_RDONLY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const HASHTABLE_NPAGE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const HASHTABLE_HASH_1: ::core::ffi::c_int = 383 as ::core::ffi::c_int;
pub const HASHTABLE_NSLOT: ::core::ffi::c_int = HASHTABLE_NPAGE
    * 2 as ::core::ffi::c_int;
pub const HASHTABLE_NPAGE_ONE: usize = (HASHTABLE_NPAGE as usize)
    .wrapping_sub(
        WALINDEX_HDR_SIZE.wrapping_div(::core::mem::size_of::<u32_0>() as usize),
    );
pub const WALINDEX_PGSZ: usize = (::core::mem::size_of::<ht_slot>() as usize)
    .wrapping_mul(HASHTABLE_NSLOT as usize)
    .wrapping_add(
        (HASHTABLE_NPAGE as usize).wrapping_mul(::core::mem::size_of::<u32_0>() as usize),
    );
#[inline(never)]
unsafe extern "C" fn walIndexPageRealloc(
    mut pWal: *mut Wal,
    mut iPage: ::core::ffi::c_int,
    mut ppPage: *mut *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pWal).nWiData <= iPage {
            let mut nByte: sqlite3_int64 = (::core::mem::size_of::<*mut u32_0>()
                as ::core::ffi::c_ulonglong)
                .wrapping_mul((1 as i64_0 + iPage as i64_0) as ::core::ffi::c_ulonglong)
                as sqlite3_int64;
            let mut apNew: *mut *mut u32_0 = ::core::ptr::null_mut::<*mut u32_0>();
            apNew = sqlite3Realloc(
                (*pWal).apWiData as *mut ::core::ffi::c_void,
                nByte as u64_0,
            ) as *mut *mut u32_0;
            if apNew.is_null() {
                *ppPage = ::core::ptr::null_mut::<u32_0>();
                return SQLITE_NOMEM_BKPT;
            }
            memset(
                apNew.offset((*pWal).nWiData as isize) as *mut *mut u32_0
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<*mut u32_0>() as size_t)
                    .wrapping_mul(
                        (iPage + 1 as ::core::ffi::c_int - (*pWal).nWiData) as size_t,
                    ),
            );
            (*pWal).apWiData = apNew;
            (*pWal).nWiData = iPage + 1 as ::core::ffi::c_int;
        }
        if (*pWal).exclusiveMode as ::core::ffi::c_int == WAL_HEAPMEMORY_MODE {
            let ref mut c2rust_fresh24 = *(*pWal).apWiData.offset(iPage as isize);
            *c2rust_fresh24 = sqlite3MallocZero(WALINDEX_PGSZ as u64_0) as *mut u32_0;
            if (*(*pWal).apWiData.offset(iPage as isize)).is_null() {
                rc = SQLITE_NOMEM_BKPT;
            }
        } else {
            rc = sqlite3OsShmMap(
                (*pWal).pDbFd,
                iPage,
                WALINDEX_PGSZ as ::core::ffi::c_int,
                (*pWal).writeLock as ::core::ffi::c_int,
                (*pWal).apWiData.offset(iPage as isize) as *mut *mut u32_0
                    as *mut *mut ::core::ffi::c_void,
            );
            if rc == SQLITE_OK {
                if iPage > 0 as ::core::ffi::c_int
                    && sqlite3FaultSim(600 as ::core::ffi::c_int) != 0
                {
                    rc = SQLITE_NOMEM;
                }
            } else if rc & 0xff as ::core::ffi::c_int == SQLITE_READONLY {
                (*pWal).readOnly = ((*pWal).readOnly as ::core::ffi::c_int
                    | WAL_SHM_RDONLY) as u8_0;
                if rc == SQLITE_READONLY {
                    rc = SQLITE_OK;
                }
            }
        }
        *ppPage = *(*pWal).apWiData.offset(iPage as isize);
        return rc;
    }
}
unsafe extern "C" fn walIndexPage(
    mut pWal: *mut Wal,
    mut iPage: ::core::ffi::c_int,
    mut ppPage: *mut *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pWal).nWiData <= iPage
            || {
                *ppPage = *(*pWal).apWiData.offset(iPage as isize);
                (*ppPage).is_null()
            }
        {
            return walIndexPageRealloc(pWal, iPage, ppPage);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn walCkptInfo(mut pWal: *mut Wal) -> *mut WalCkptInfo {
    unsafe {
        return (*(*pWal).apWiData.offset(0 as ::core::ffi::c_int as isize))
            .offset(
                (::core::mem::size_of::<WalIndexHdr>() as usize).wrapping_div(2 as usize)
                    as isize,
            ) as *mut u32_0 as *mut WalCkptInfo;
    }
}
unsafe extern "C" fn walIndexHdr(mut pWal: *mut Wal) -> *mut WalIndexHdr {
    unsafe {
        return *(*pWal).apWiData.offset(0 as ::core::ffi::c_int as isize)
            as *mut WalIndexHdr;
    }
}
unsafe extern "C" fn walChecksumBytes(
    mut nativeCksum: ::core::ffi::c_int,
    mut a: *mut u8_0,
    mut nByte: ::core::ffi::c_int,
    mut aIn: *const u32_0,
    mut aOut: *mut u32_0,
) {
    unsafe {
        let mut s1: u32_0 = 0;
        let mut s2: u32_0 = 0;
        let mut aData: *mut u32_0 = a as *mut u32_0;
        let mut aEnd: *mut u32_0 = a.offset(nByte as isize) as *mut u8_0 as *mut u32_0;
        if !aIn.is_null() {
            s1 = *aIn.offset(0 as ::core::ffi::c_int as isize);
            s2 = *aIn.offset(1 as ::core::ffi::c_int as isize);
        } else {
            s2 = 0 as u32_0;
            s1 = s2;
        }
        if nativeCksum == 0 {
            loop {
                s1 = (s1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        ((*aData.offset(0 as ::core::ffi::c_int as isize)
                            & 0xff as u32_0) << 24 as ::core::ffi::c_int)
                            .wrapping_add(
                                (*aData.offset(0 as ::core::ffi::c_int as isize)
                                    & 0xff00 as u32_0) << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*aData.offset(0 as ::core::ffi::c_int as isize)
                                    & 0xff0000 as ::core::ffi::c_int as u32_0)
                                    >> 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*aData.offset(0 as ::core::ffi::c_int as isize)
                                    & 0xff000000 as u32_0) >> 24 as ::core::ffi::c_int,
                            )
                            .wrapping_add(s2) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                s2 = (s2 as ::core::ffi::c_uint)
                    .wrapping_add(
                        ((*aData.offset(1 as ::core::ffi::c_int as isize)
                            & 0xff as u32_0) << 24 as ::core::ffi::c_int)
                            .wrapping_add(
                                (*aData.offset(1 as ::core::ffi::c_int as isize)
                                    & 0xff00 as u32_0) << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*aData.offset(1 as ::core::ffi::c_int as isize)
                                    & 0xff0000 as ::core::ffi::c_int as u32_0)
                                    >> 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*aData.offset(1 as ::core::ffi::c_int as isize)
                                    & 0xff000000 as u32_0) >> 24 as ::core::ffi::c_int,
                            )
                            .wrapping_add(s1) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                aData = aData.offset(2 as ::core::ffi::c_int as isize);
                if !(aData < aEnd) {
                    break;
                }
            }
        } else if nByte % 64 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            loop {
                let c2rust_fresh1 = aData;
                aData = aData.offset(1);
                s1 = (s1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh1).wrapping_add(s2) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh2 = aData;
                aData = aData.offset(1);
                s2 = (s2 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh2).wrapping_add(s1) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh3 = aData;
                aData = aData.offset(1);
                s1 = (s1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh3).wrapping_add(s2) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh4 = aData;
                aData = aData.offset(1);
                s2 = (s2 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh4).wrapping_add(s1) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh5 = aData;
                aData = aData.offset(1);
                s1 = (s1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh5).wrapping_add(s2) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh6 = aData;
                aData = aData.offset(1);
                s2 = (s2 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh6).wrapping_add(s1) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh7 = aData;
                aData = aData.offset(1);
                s1 = (s1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh7).wrapping_add(s2) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh8 = aData;
                aData = aData.offset(1);
                s2 = (s2 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh8).wrapping_add(s1) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh9 = aData;
                aData = aData.offset(1);
                s1 = (s1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh9).wrapping_add(s2) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh10 = aData;
                aData = aData.offset(1);
                s2 = (s2 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh10).wrapping_add(s1) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh11 = aData;
                aData = aData.offset(1);
                s1 = (s1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh11).wrapping_add(s2) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh12 = aData;
                aData = aData.offset(1);
                s2 = (s2 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh12).wrapping_add(s1) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh13 = aData;
                aData = aData.offset(1);
                s1 = (s1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh13).wrapping_add(s2) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh14 = aData;
                aData = aData.offset(1);
                s2 = (s2 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh14).wrapping_add(s1) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh15 = aData;
                aData = aData.offset(1);
                s1 = (s1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh15).wrapping_add(s2) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh16 = aData;
                aData = aData.offset(1);
                s2 = (s2 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh16).wrapping_add(s1) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                if !(aData < aEnd) {
                    break;
                }
            }
        } else {
            loop {
                let c2rust_fresh17 = aData;
                aData = aData.offset(1);
                s1 = (s1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh17).wrapping_add(s2) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh18 = aData;
                aData = aData.offset(1);
                s2 = (s2 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh18).wrapping_add(s1) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                if !(aData < aEnd) {
                    break;
                }
            }
        }
        *aOut.offset(0 as ::core::ffi::c_int as isize) = s1;
        *aOut.offset(1 as ::core::ffi::c_int as isize) = s2;
    }
}
unsafe extern "C" fn walShmBarrier(mut pWal: *mut Wal) {
    unsafe {
        if (*pWal).exclusiveMode as ::core::ffi::c_int != WAL_HEAPMEMORY_MODE {
            sqlite3OsShmBarrier((*pWal).pDbFd);
        }
    }
}
unsafe extern "C" fn walIndexWriteHdr(mut pWal: *mut Wal) {
    unsafe {
        let mut aHdr: *mut WalIndexHdr = walIndexHdr(pWal);
        let nCksum: ::core::ffi::c_int = 40 as ::core::ffi::c_ulong
            as ::core::ffi::c_int;
        (*pWal).hdr.isInit = 1 as u8_0;
        (*pWal).hdr.iVersion = WALINDEX_MAX_VERSION as u32_0;
        walChecksumBytes(
            1 as ::core::ffi::c_int,
            &raw mut (*pWal).hdr as *mut u8_0,
            nCksum,
            ::core::ptr::null::<u32_0>(),
            &raw mut (*pWal).hdr.aCksum as *mut u32_0,
        );
        memcpy(
            aHdr.offset(1 as ::core::ffi::c_int as isize) as *mut WalIndexHdr
                as *mut ::core::ffi::c_void,
            &raw mut (*pWal).hdr as *const ::core::ffi::c_void,
            ::core::mem::size_of::<WalIndexHdr>() as size_t,
        );
        walShmBarrier(pWal);
        memcpy(
            aHdr.offset(0 as ::core::ffi::c_int as isize) as *mut WalIndexHdr
                as *mut ::core::ffi::c_void,
            &raw mut (*pWal).hdr as *const ::core::ffi::c_void,
            ::core::mem::size_of::<WalIndexHdr>() as size_t,
        );
    }
}
unsafe extern "C" fn walEncodeFrame(
    mut pWal: *mut Wal,
    mut iPage: u32_0,
    mut nTruncate: u32_0,
    mut aData: *mut u8_0,
    mut aFrame: *mut u8_0,
) {
    unsafe {
        let mut nativeCksum: ::core::ffi::c_int = 0;
        let mut aCksum: *mut u32_0 = &raw mut (*pWal).hdr.aFrameCksum as *mut u32_0;
        sqlite3Put4byte(
            aFrame.offset(0 as ::core::ffi::c_int as isize) as *mut u8_0,
            iPage,
        );
        sqlite3Put4byte(
            aFrame.offset(4 as ::core::ffi::c_int as isize) as *mut u8_0,
            nTruncate,
        );
        if (*pWal).iReCksum == 0 as ::core::ffi::c_uint {
            memcpy(
                aFrame.offset(8 as ::core::ffi::c_int as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void,
                &raw mut (*pWal).hdr.aSalt as *mut u32_0 as *const ::core::ffi::c_void,
                8 as size_t,
            );
            nativeCksum = ((*pWal).hdr.bigEndCksum as ::core::ffi::c_int
                == SQLITE_BIGENDIAN) as ::core::ffi::c_int;
            walChecksumBytes(
                nativeCksum,
                aFrame,
                8 as ::core::ffi::c_int,
                aCksum,
                aCksum,
            );
            walChecksumBytes(
                nativeCksum,
                aData,
                (*pWal).szPage as ::core::ffi::c_int,
                aCksum,
                aCksum,
            );
            sqlite3Put4byte(
                aFrame.offset(16 as ::core::ffi::c_int as isize) as *mut u8_0,
                *aCksum.offset(0 as ::core::ffi::c_int as isize),
            );
            sqlite3Put4byte(
                aFrame.offset(20 as ::core::ffi::c_int as isize) as *mut u8_0,
                *aCksum.offset(1 as ::core::ffi::c_int as isize),
            );
        } else {
            memset(
                aFrame.offset(8 as ::core::ffi::c_int as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                16 as size_t,
            );
        };
    }
}
unsafe extern "C" fn walDecodeFrame(
    mut pWal: *mut Wal,
    mut piPage: *mut u32_0,
    mut pnTruncate: *mut u32_0,
    mut aData: *mut u8_0,
    mut aFrame: *mut u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nativeCksum: ::core::ffi::c_int = 0;
        let mut aCksum: *mut u32_0 = &raw mut (*pWal).hdr.aFrameCksum as *mut u32_0;
        let mut pgno: u32_0 = 0;
        if memcmp(
            &raw mut (*pWal).hdr.aSalt as *const ::core::ffi::c_void,
            aFrame.offset(8 as ::core::ffi::c_int as isize) as *mut u8_0
                as *const ::core::ffi::c_void,
            8 as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        pgno = sqlite3Get4byte(
            aFrame.offset(0 as ::core::ffi::c_int as isize) as *mut u8_0,
        );
        if pgno == 0 as ::core::ffi::c_uint {
            return 0 as ::core::ffi::c_int;
        }
        nativeCksum = ((*pWal).hdr.bigEndCksum as ::core::ffi::c_int == SQLITE_BIGENDIAN)
            as ::core::ffi::c_int;
        walChecksumBytes(nativeCksum, aFrame, 8 as ::core::ffi::c_int, aCksum, aCksum);
        walChecksumBytes(
            nativeCksum,
            aData,
            (*pWal).szPage as ::core::ffi::c_int,
            aCksum,
            aCksum,
        );
        if *aCksum.offset(0 as ::core::ffi::c_int as isize)
            != sqlite3Get4byte(
                aFrame.offset(16 as ::core::ffi::c_int as isize) as *mut u8_0,
            )
            || *aCksum.offset(1 as ::core::ffi::c_int as isize)
                != sqlite3Get4byte(
                    aFrame.offset(20 as ::core::ffi::c_int as isize) as *mut u8_0,
                )
        {
            return 0 as ::core::ffi::c_int;
        }
        *piPage = pgno;
        *pnTruncate = sqlite3Get4byte(
            aFrame.offset(4 as ::core::ffi::c_int as isize) as *mut u8_0,
        );
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn walLockShared(
    mut pWal: *mut Wal,
    mut lockIdx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if (*pWal).exclusiveMode != 0 {
            return SQLITE_OK;
        }
        rc = sqlite3OsShmLock(
            (*pWal).pDbFd,
            lockIdx,
            1 as ::core::ffi::c_int,
            SQLITE_SHM_LOCK | SQLITE_SHM_SHARED,
        );
        return rc;
    }
}
unsafe extern "C" fn walUnlockShared(
    mut pWal: *mut Wal,
    mut lockIdx: ::core::ffi::c_int,
) {
    unsafe {
        if (*pWal).exclusiveMode != 0 {
            return;
        }
        sqlite3OsShmLock(
            (*pWal).pDbFd,
            lockIdx,
            1 as ::core::ffi::c_int,
            SQLITE_SHM_UNLOCK | SQLITE_SHM_SHARED,
        );
    }
}
unsafe extern "C" fn walLockExclusive(
    mut pWal: *mut Wal,
    mut lockIdx: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if (*pWal).exclusiveMode != 0 {
            return SQLITE_OK;
        }
        rc = sqlite3OsShmLock(
            (*pWal).pDbFd,
            lockIdx,
            n,
            SQLITE_SHM_LOCK | SQLITE_SHM_EXCLUSIVE,
        );
        return rc;
    }
}
unsafe extern "C" fn walUnlockExclusive(
    mut pWal: *mut Wal,
    mut lockIdx: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) {
    unsafe {
        if (*pWal).exclusiveMode != 0 {
            return;
        }
        sqlite3OsShmLock(
            (*pWal).pDbFd,
            lockIdx,
            n,
            SQLITE_SHM_UNLOCK | SQLITE_SHM_EXCLUSIVE,
        );
    }
}
unsafe extern "C" fn walHash(mut iPage: u32_0) -> ::core::ffi::c_int {
    unsafe {
        return ((iPage as ::core::ffi::c_uint)
            .wrapping_mul(HASHTABLE_HASH_1 as ::core::ffi::c_uint)
            & (HASHTABLE_NSLOT - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn walNextHash(
    mut iPriorHash: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return iPriorHash + 1 as ::core::ffi::c_int
            & HASHTABLE_NSLOT - 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn walHashGet(
    mut pWal: *mut Wal,
    mut iHash: ::core::ffi::c_int,
    mut pLoc: *mut WalHashLoc,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = walIndexPage(pWal, iHash, &raw mut (*pLoc).aPgno);
        if !(*pLoc).aPgno.is_null() {
            (*pLoc).aHash = (*pLoc).aPgno.offset(HASHTABLE_NPAGE as isize) as *mut u32_0
                as *mut ht_slot;
            if iHash == 0 as ::core::ffi::c_int {
                (*pLoc).aPgno = (*pLoc)
                    .aPgno
                    .offset(
                        WALINDEX_HDR_SIZE
                            .wrapping_div(::core::mem::size_of::<u32_0>() as usize)
                            as isize,
                    ) as *mut u32_0;
                (*pLoc).iZero = 0 as u32_0;
            } else {
                (*pLoc).iZero = HASHTABLE_NPAGE_ONE
                    .wrapping_add(
                        ((iHash - 1 as ::core::ffi::c_int) * HASHTABLE_NPAGE) as usize,
                    ) as u32_0;
            }
        } else if rc == 0 as ::core::ffi::c_int {
            rc = SQLITE_ERROR;
        }
        return rc;
    }
}
unsafe extern "C" fn walFramePage(mut iFrame: u32_0) -> ::core::ffi::c_int {
    unsafe {
        let mut iHash: ::core::ffi::c_int = ((iFrame as ::core::ffi::c_uint)
            .wrapping_add(HASHTABLE_NPAGE as ::core::ffi::c_uint) as usize)
            .wrapping_sub(HASHTABLE_NPAGE_ONE)
            .wrapping_sub(1 as usize)
            .wrapping_div(HASHTABLE_NPAGE as usize) as ::core::ffi::c_int;
        return iHash;
    }
}
unsafe extern "C" fn walFramePgno(mut pWal: *mut Wal, mut iFrame: u32_0) -> u32_0 {
    unsafe {
        let mut iHash: ::core::ffi::c_int = walFramePage(iFrame);
        if iHash == 0 as ::core::ffi::c_int {
            return *(*(*pWal).apWiData.offset(0 as ::core::ffi::c_int as isize))
                .offset(
                    WALINDEX_HDR_SIZE
                        .wrapping_div(::core::mem::size_of::<u32_0>() as usize)
                        .wrapping_add(iFrame as usize)
                        .wrapping_sub(1 as usize) as isize,
                );
        }
        return *(*(*pWal).apWiData.offset(iHash as isize))
            .offset(
                ((iFrame as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
                    as usize)
                    .wrapping_sub(HASHTABLE_NPAGE_ONE)
                    .wrapping_rem(HASHTABLE_NPAGE as usize) as isize,
            );
    }
}
unsafe extern "C" fn walCleanupHash(mut pWal: *mut Wal) {
    unsafe {
        let mut sLoc: WalHashLoc = WalHashLoc {
            aHash: ::core::ptr::null_mut::<ht_slot>(),
            aPgno: ::core::ptr::null_mut::<u32_0>(),
            iZero: 0

        };
        let mut iLimit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nByte: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        if (*pWal).hdr.mxFrame == 0 as ::core::ffi::c_uint {
            return;
        }
        i = walHashGet(pWal, walFramePage((*pWal).hdr.mxFrame), &raw mut sLoc);
        if i != 0 {
            return;
        }
        iLimit = (*pWal).hdr.mxFrame.wrapping_sub(sLoc.iZero) as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < HASHTABLE_NSLOT {
            if *sLoc.aHash.offset(i as isize) as ::core::ffi::c_int > iLimit {
                ::core::ptr::write_volatile(sLoc.aHash.offset(i as isize), 0 as ht_slot);
            }
            i += 1;
        }
        nByte = (sLoc.aHash as *mut ::core::ffi::c_char)
            .offset_from(
                sLoc.aPgno.offset(iLimit as isize) as *mut u32_0
                    as *mut ::core::ffi::c_char,
            ) as ::core::ffi::c_long as ::core::ffi::c_int;
        memset(
            sLoc.aPgno.offset(iLimit as isize) as *mut u32_0 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            nByte as size_t,
        );
    }
}
unsafe extern "C" fn walIndexAppend(
    mut pWal: *mut Wal,
    mut iFrame: u32_0,
    mut iPage: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut sLoc: WalHashLoc = WalHashLoc {
            aHash: ::core::ptr::null_mut::<ht_slot>(),
            aPgno: ::core::ptr::null_mut::<u32_0>(),
            iZero: 0

        };
        rc = walHashGet(pWal, walFramePage(iFrame), &raw mut sLoc);
        if rc == SQLITE_OK {
            let mut iKey: ::core::ffi::c_int = 0;
            let mut idx: ::core::ffi::c_int = 0;
            let mut nCollide: ::core::ffi::c_int = 0;
            idx = iFrame.wrapping_sub(sLoc.iZero) as ::core::ffi::c_int;
            if idx == 1 as ::core::ffi::c_int {
                let mut nByte: ::core::ffi::c_int = (sLoc
                    .aHash
                    .offset(HASHTABLE_NSLOT as isize) as *mut ht_slot as *mut u8_0)
                    .offset_from(sLoc.aPgno as *mut u8_0) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
                memset(
                    sLoc.aPgno as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    nByte as size_t,
                );
            }
            if *sLoc.aPgno.offset((idx - 1 as ::core::ffi::c_int) as isize) != 0 {
                walCleanupHash(pWal);
            }
            nCollide = idx;
            iKey = walHash(iPage);
            while *sLoc.aHash.offset(iKey as isize) != 0 {
                let c2rust_fresh27 = nCollide;
                nCollide = nCollide - 1;
                if c2rust_fresh27 == 0 as ::core::ffi::c_int {
                    return sqlite3CorruptError(1335 as ::core::ffi::c_int);
                }
                iKey = walNextHash(iKey);
            }
            ::core::ptr::write_volatile(
                sLoc.aPgno.offset((idx - 1 as ::core::ffi::c_int) as isize),
                iPage,
            );
            ;
        }
        return rc;
    }
}
unsafe extern "C" fn walIndexRecover(mut pWal: *mut Wal) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = 0;
        let mut nSize: i64_0 = 0;
        let mut aFrameCksum: [u32_0; 2] = [
            0 as ::core::ffi::c_int as u32_0,
            0 as ::core::ffi::c_int as u32_0,
        ];
        let mut iLock: ::core::ffi::c_int = 0;
        iLock = WAL_ALL_BUT_WRITE + (*pWal).ckptLock as ::core::ffi::c_int;
        rc = walLockExclusive(
            pWal,
            iLock,
            3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int - iLock,
        );
        if rc != 0 {
            return rc;
        }
        memset(
            &raw mut (*pWal).hdr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<WalIndexHdr>() as size_t,
        );
        rc = sqlite3OsFileSize((*pWal).pWalFd, &raw mut nSize);
        if !(rc != SQLITE_OK) {
            if nSize > WAL_HDRSIZE as ::core::ffi::c_longlong {
                let mut aBuf: [u8_0; 32] = [0; 32];
                let mut aPrivate: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
                let mut aFrame: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                let mut szFrame: ::core::ffi::c_int = 0;
                let mut aData: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                let mut szPage: ::core::ffi::c_int = 0;
                let mut magic: u32_0 = 0;
                let mut version: u32_0 = 0;
                let mut isValid: ::core::ffi::c_int = 0;
                let mut iPg: u32_0 = 0;
                let mut iLastFrame: u32_0 = 0;
                rc = sqlite3OsRead(
                    (*pWal).pWalFd,
                    &raw mut aBuf as *mut u8_0 as *mut ::core::ffi::c_void,
                    WAL_HDRSIZE,
                    0 as i64_0,
                );
                if rc != SQLITE_OK {
                    c2rust_current_block = 2310077433060450808;
                } else {
                    magic = sqlite3Get4byte(
                        (&raw mut aBuf as *mut u8_0)
                            .offset(0 as ::core::ffi::c_int as isize) as *mut u8_0,
                    );
                    szPage = sqlite3Get4byte(
                        (&raw mut aBuf as *mut u8_0)
                            .offset(8 as ::core::ffi::c_int as isize) as *mut u8_0,
                    ) as ::core::ffi::c_int;
                    if magic as ::core::ffi::c_uint & 0xfffffffe as ::core::ffi::c_uint
                        != WAL_MAGIC as ::core::ffi::c_uint
                        || szPage & szPage - 1 as ::core::ffi::c_int != 0
                        || szPage > SQLITE_MAX_PAGE_SIZE
                        || szPage < 512 as ::core::ffi::c_int
                    {
                        c2rust_current_block = 16616366918342538296;
                    } else {
                        (*pWal).hdr.bigEndCksum = (magic as ::core::ffi::c_uint
                            & 0x1 as ::core::ffi::c_uint) as u8_0;
                        (*pWal).szPage = szPage as u32_0;
                        (*pWal).nCkpt = sqlite3Get4byte(
                            (&raw mut aBuf as *mut u8_0)
                                .offset(12 as ::core::ffi::c_int as isize) as *mut u8_0,
                        );
                        memcpy(
                            &raw mut (*pWal).hdr.aSalt as *mut ::core::ffi::c_void,
                            (&raw mut aBuf as *mut u8_0)
                                .offset(16 as ::core::ffi::c_int as isize) as *mut u8_0
                                as *const ::core::ffi::c_void,
                            8 as size_t,
                        );
                        walChecksumBytes(
                            ((*pWal).hdr.bigEndCksum as ::core::ffi::c_int
                                == SQLITE_BIGENDIAN) as ::core::ffi::c_int,
                            &raw mut aBuf as *mut u8_0,
                            WAL_HDRSIZE
                                - 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
                            ::core::ptr::null::<u32_0>(),
                            &raw mut (*pWal).hdr.aFrameCksum as *mut u32_0,
                        );
                        if (*pWal).hdr.aFrameCksum[0 as ::core::ffi::c_int as usize]
                            != sqlite3Get4byte(
                                (&raw mut aBuf as *mut u8_0)
                                    .offset(24 as ::core::ffi::c_int as isize) as *mut u8_0,
                            )
                            || (*pWal).hdr.aFrameCksum[1 as ::core::ffi::c_int as usize]
                                != sqlite3Get4byte(
                                    (&raw mut aBuf as *mut u8_0)
                                        .offset(28 as ::core::ffi::c_int as isize) as *mut u8_0,
                                )
                        {
                            c2rust_current_block = 16616366918342538296;
                        } else {
                            version = sqlite3Get4byte(
                                (&raw mut aBuf as *mut u8_0)
                                    .offset(4 as ::core::ffi::c_int as isize) as *mut u8_0,
                            );
                            if version != WAL_MAX_VERSION as ::core::ffi::c_uint {
                                rc = sqlite3CantopenError(1467 as ::core::ffi::c_int);
                                c2rust_current_block = 16616366918342538296;
                            } else {
                                szFrame = szPage + WAL_FRAME_HDRSIZE;
                                aFrame = sqlite3_malloc64(
                                    (szFrame as usize).wrapping_add(WALINDEX_PGSZ)
                                        as sqlite3_uint64,
                                ) as *mut u8_0;
                                if aFrame.is_null() {
                                    rc = SQLITE_NOMEM_BKPT;
                                    c2rust_current_block = 2310077433060450808;
                                } else {
                                    aData = aFrame.offset(WAL_FRAME_HDRSIZE as isize)
                                        as *mut u8_0;
                                    aPrivate = aData.offset(szPage as isize) as *mut u8_0
                                        as *mut u32_0;
                                    iLastFrame = ((nSize as ::core::ffi::c_longlong
                                        - WAL_HDRSIZE as ::core::ffi::c_longlong)
                                        / szFrame as ::core::ffi::c_longlong) as u32_0;
                                    iPg = 0 as u32_0;
                                    while iPg <= walFramePage(iLastFrame) as u32_0 {
                                        let mut aShare: *mut u32_0 = ::core::ptr::null_mut::<
                                            u32_0,
                                        >();
                                        let mut iFrame: u32_0 = 0;
                                        let mut iLast: u32_0 = (if (iLastFrame as usize)
                                            < (4096 as usize)
                                                .wrapping_sub(
                                                    (::core::mem::size_of::<WalIndexHdr>() as usize)
                                                        .wrapping_mul(2 as usize)
                                                        .wrapping_add(
                                                            ::core::mem::size_of::<WalCkptInfo>() as usize,
                                                        )
                                                        .wrapping_div(::core::mem::size_of::<u32_0>() as usize),
                                                )
                                                .wrapping_add(
                                                    (iPg as ::core::ffi::c_uint)
                                                        .wrapping_mul(4096 as ::core::ffi::c_uint) as usize,
                                                )
                                        {
                                            iLastFrame as usize
                                        } else {
                                            (4096 as usize)
                                                .wrapping_sub(
                                                    (::core::mem::size_of::<WalIndexHdr>() as usize)
                                                        .wrapping_mul(2 as usize)
                                                        .wrapping_add(
                                                            ::core::mem::size_of::<WalCkptInfo>() as usize,
                                                        )
                                                        .wrapping_div(::core::mem::size_of::<u32_0>() as usize),
                                                )
                                                .wrapping_add(
                                                    (iPg as ::core::ffi::c_uint)
                                                        .wrapping_mul(4096 as ::core::ffi::c_uint) as usize,
                                                )
                                        }) as u32_0;
                                        let mut iFirst: u32_0 = (1 as usize)
                                            .wrapping_add(
                                                (if iPg == 0 as ::core::ffi::c_uint {
                                                    0 as usize
                                                } else {
                                                    HASHTABLE_NPAGE_ONE
                                                        .wrapping_add(
                                                            (iPg as ::core::ffi::c_uint)
                                                                .wrapping_sub(1 as ::core::ffi::c_uint)
                                                                .wrapping_mul(HASHTABLE_NPAGE as ::core::ffi::c_uint)
                                                                as usize,
                                                        )
                                                }),
                                            ) as u32_0;
                                        let mut nHdr: u32_0 = 0;
                                        let mut nHdr32: u32_0 = 0;
                                        rc = walIndexPage(
                                            pWal,
                                            iPg as ::core::ffi::c_int,
                                            &raw mut aShare as *mut *mut u32_0,
                                        );
                                        if aShare.is_null() {
                                            break;
                                        }
                                        let ref mut c2rust_fresh25 = *(*pWal)
                                            .apWiData
                                            .offset(iPg as isize);
                                        *c2rust_fresh25 = aPrivate as *mut u32_0;
                                        iFrame = iFirst;
                                        while iFrame <= iLast {
                                            let mut iOffset: i64_0 = WAL_HDRSIZE as i64_0
                                                + (iFrame as ::core::ffi::c_uint)
                                                    .wrapping_sub(1 as ::core::ffi::c_uint) as i64_0
                                                    * (szPage + WAL_FRAME_HDRSIZE) as i64_0;
                                            let mut pgno: u32_0 = 0;
                                            let mut nTruncate: u32_0 = 0;
                                            rc = sqlite3OsRead(
                                                (*pWal).pWalFd,
                                                aFrame as *mut ::core::ffi::c_void,
                                                szFrame,
                                                iOffset,
                                            );
                                            if rc != SQLITE_OK {
                                                break;
                                            }
                                            isValid = walDecodeFrame(
                                                pWal,
                                                &raw mut pgno,
                                                &raw mut nTruncate,
                                                aData,
                                                aFrame,
                                            );
                                            if isValid == 0 {
                                                break;
                                            }
                                            rc = walIndexAppend(pWal, iFrame, pgno);
                                            if rc != 0 as ::core::ffi::c_int {
                                                break;
                                            }
                                            if nTruncate != 0 {
                                                (*pWal).hdr.mxFrame = iFrame;
                                                (*pWal).hdr.nPage = nTruncate;
                                                (*pWal).hdr.szPage = (szPage & 0xff00 as ::core::ffi::c_int
                                                    | szPage >> 16 as ::core::ffi::c_int) as u16_0;
                                                aFrameCksum[0 as ::core::ffi::c_int as usize] = (*pWal)
                                                    .hdr
                                                    .aFrameCksum[0 as ::core::ffi::c_int as usize];
                                                aFrameCksum[1 as ::core::ffi::c_int as usize] = (*pWal)
                                                    .hdr
                                                    .aFrameCksum[1 as ::core::ffi::c_int as usize];
                                            }
                                            iFrame = iFrame.wrapping_add(1);
                                        }
                                        let ref mut c2rust_fresh26 = *(*pWal)
                                            .apWiData
                                            .offset(iPg as isize);
                                        *c2rust_fresh26 = aShare as *mut u32_0;
                                        nHdr = (if iPg == 0 as ::core::ffi::c_uint {
                                            WALINDEX_HDR_SIZE
                                        } else {
                                            0 as usize
                                        }) as u32_0;
                                        nHdr32 = (nHdr as usize)
                                            .wrapping_div(::core::mem::size_of::<u32_0>() as usize)
                                            as u32_0;
                                        memcpy(
                                            aShare.offset(nHdr32 as isize) as *mut u32_0
                                                as *mut ::core::ffi::c_void,
                                            aPrivate.offset(nHdr32 as isize) as *mut u32_0
                                                as *const ::core::ffi::c_void,
                                            WALINDEX_PGSZ.wrapping_sub(nHdr as size_t),
                                        );
                                        if iFrame <= iLast {
                                            break;
                                        }
                                        iPg = iPg.wrapping_add(1);
                                    }
                                    sqlite3_free(aFrame as *mut ::core::ffi::c_void);
                                    c2rust_current_block = 16616366918342538296;
                                }
                            }
                        }
                    }
                }
            } else {
                c2rust_current_block = 16616366918342538296;
            }
            match c2rust_current_block {
                2310077433060450808 => {}
                _ => {
                    if rc == SQLITE_OK {
                        let mut pInfo: *mut WalCkptInfo = ::core::ptr::null_mut::<
                            WalCkptInfo,
                        >();
                        let mut i: ::core::ffi::c_int = 0;
                        (*pWal).hdr.aFrameCksum[0 as ::core::ffi::c_int as usize] = aFrameCksum[0
                            as ::core::ffi::c_int as usize];
                        (*pWal).hdr.aFrameCksum[1 as ::core::ffi::c_int as usize] = aFrameCksum[1
                            as ::core::ffi::c_int as usize];
                        walIndexWriteHdr(pWal);
                        pInfo = walCkptInfo(pWal);
                        ::core::ptr::write_volatile(
                            &mut (*pInfo).nBackfill as *mut u32_0,
                            0 as u32_0,
                        );
                        ::core::ptr::write_volatile(
                            &mut (*pInfo).nBackfillAttempted as *mut u32_0,
                            (*pWal).hdr.mxFrame,
                        );
                        ::core::ptr::write_volatile(
                            &mut (*pInfo).aReadMark[0 as ::core::ffi::c_int as usize]
                                as *mut u32_0,
                            0 as u32_0,
                        );
                        i = 1 as ::core::ffi::c_int;
                        loop {
                            if !(i < WAL_NREADER) {
                                c2rust_current_block = 17787701279558130514;
                                break;
                            }
                            rc = walLockExclusive(
                                pWal,
                                3 as ::core::ffi::c_int + i,
                                1 as ::core::ffi::c_int,
                            );
                            if rc == SQLITE_OK {
                                if i == 1 as ::core::ffi::c_int && (*pWal).hdr.mxFrame != 0
                                {
                                    ::core::ptr::write_volatile(
                                        &mut (*pInfo).aReadMark[i as usize] as *mut u32_0,
                                        (*pWal).hdr.mxFrame,
                                    );
                                } else {
                                    ::core::ptr::write_volatile(
                                        &mut (*pInfo).aReadMark[i as usize] as *mut u32_0,
                                        READMARK_NOT_USED as u32_0,
                                    );
                                }
                                walUnlockExclusive(
                                    pWal,
                                    3 as ::core::ffi::c_int + i,
                                    1 as ::core::ffi::c_int,
                                );
                            } else if rc != SQLITE_BUSY {
                                c2rust_current_block = 2310077433060450808;
                                break;
                            }
                            i += 1;
                        }
                        match c2rust_current_block {
                            2310077433060450808 => {}
                            _ => {
                                if (*pWal).hdr.nPage != 0 {
                                    sqlite3_log(
                                        SQLITE_NOTICE_RECOVER_WAL,
                                        b"recovered %d frames from WAL file %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        (*pWal).hdr.mxFrame,
                                        (*pWal).zWalName,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        walUnlockExclusive(
            pWal,
            iLock,
            3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int - iLock,
        );
        return rc;
    }
}
unsafe extern "C" fn walIndexClose(
    mut pWal: *mut Wal,
    mut isDelete: ::core::ffi::c_int,
) {
    unsafe {
        if (*pWal).exclusiveMode as ::core::ffi::c_int == WAL_HEAPMEMORY_MODE
            || (*pWal).bShmUnreliable as ::core::ffi::c_int != 0
        {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pWal).nWiData {
                sqlite3_free(
                    *(*pWal).apWiData.offset(i as isize) as *mut ::core::ffi::c_void,
                );
                let ref mut c2rust_fresh0 = *(*pWal).apWiData.offset(i as isize);
                *c2rust_fresh0 = ::core::ptr::null_mut::<u32_0>();
                i += 1;
            }
        }
        if (*pWal).exclusiveMode as ::core::ffi::c_int != WAL_HEAPMEMORY_MODE {
            sqlite3OsShmUnmap((*pWal).pDbFd, isDelete);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut pDbFd: *mut sqlite3_file,
    mut zWalName: *const ::core::ffi::c_char,
    mut bNoShm: ::core::ffi::c_int,
    mut mxWalSize: i64_0,
    mut ppWal: *mut *mut Wal,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pRet: *mut Wal = ::core::ptr::null_mut::<Wal>();
        let mut flags: ::core::ffi::c_int = 0;
        *ppWal = ::core::ptr::null_mut::<Wal>();
        pRet = sqlite3MallocZero(
            (::core::mem::size_of::<Wal>() as usize)
                .wrapping_add((*pVfs).szOsFile as usize) as u64_0,
        ) as *mut Wal;
        if pRet.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        (*pRet).pVfs = pVfs;
        (*pRet).pWalFd = pRet.offset(1 as ::core::ffi::c_int as isize) as *mut Wal
            as *mut sqlite3_file;
        (*pRet).pDbFd = pDbFd;
        (*pRet).readLock = -1 as ::core::ffi::c_int as i16_0;
        (*pRet).mxWalSize = mxWalSize;
        (*pRet).zWalName = zWalName;
        (*pRet).syncHeader = 1 as u8_0;
        (*pRet).padToSectorBoundary = 1 as u8_0;
        (*pRet).exclusiveMode = (if bNoShm != 0 {
            WAL_HEAPMEMORY_MODE
        } else {
            WAL_NORMAL_MODE
        }) as u8_0;
        flags = SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_WAL;
        rc = sqlite3OsOpen(pVfs, zWalName, (*pRet).pWalFd, flags, &raw mut flags);
        if rc == SQLITE_OK && flags & SQLITE_OPEN_READONLY != 0 {
            (*pRet).readOnly = WAL_RDONLY as u8_0;
        }
        if rc != SQLITE_OK {
            walIndexClose(pRet, 0 as ::core::ffi::c_int);
            sqlite3OsClose((*pRet).pWalFd);
            sqlite3_free(pRet as *mut ::core::ffi::c_void);
        } else {
            let mut iDC: ::core::ffi::c_int = sqlite3OsDeviceCharacteristics(pDbFd);
            if iDC & SQLITE_IOCAP_SEQUENTIAL != 0 {
                (*pRet).syncHeader = 0 as u8_0;
            }
            if iDC & SQLITE_IOCAP_POWERSAFE_OVERWRITE != 0 {
                (*pRet).padToSectorBoundary = 0 as u8_0;
            }
            *ppWal = pRet;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalLimit(mut pWal: *mut Wal, mut iLimit: i64_0) {
    unsafe {
        if !pWal.is_null() {
            (*pWal).mxWalSize = iLimit;
        }
    }
}
unsafe extern "C" fn walIteratorNext(
    mut p: *mut WalIterator,
    mut piPage: *mut u32_0,
    mut piFrame: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iMin: u32_0 = 0;
        let mut iRet: u32_0 = 0xffffffff as u32_0;
        let mut i: ::core::ffi::c_int = 0;
        iMin = (*p).iPrior;
        i = (*p).nSegment - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            let mut pSegment: *mut WalSegment = (&raw mut (*p).aSegment
                as *mut WalSegment)
                .offset(i as isize) as *mut WalSegment;
            while (*pSegment).iNext < (*pSegment).nEntry {
                let mut iPg: u32_0 = *(*pSegment)
                    .aPgno
                    .offset(
                        *(*pSegment).aIndex.offset((*pSegment).iNext as isize) as isize,
                    );
                if iPg > iMin {
                    if iPg < iRet {
                        iRet = iPg;
                        *piFrame = ((*pSegment).iZero
                            + *(*pSegment).aIndex.offset((*pSegment).iNext as isize)
                                as ::core::ffi::c_int) as u32_0;
                    }
                    break;
                } else {
                    (*pSegment).iNext += 1;
                }
            }
            i -= 1;
        }
        (*p).iPrior = iRet;
        *piPage = (*p).iPrior;
        return (iRet == 0xffffffff as ::core::ffi::c_uint) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn walMerge(
    mut aContent: *const u32_0,
    mut aLeft: *mut ht_slot,
    mut nLeft: ::core::ffi::c_int,
    mut paRight: *mut *mut ht_slot,
    mut pnRight: *mut ::core::ffi::c_int,
    mut aTmp: *mut ht_slot,
) {
    unsafe {
        let mut iLeft: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iRight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nRight: ::core::ffi::c_int = *pnRight;
        let mut aRight: *mut ht_slot = *paRight;
        while iRight < nRight || iLeft < nLeft {
            let mut logpage: ht_slot = 0;
            let mut dbpage: Pgno = 0;
            if iLeft < nLeft
                && (iRight >= nRight
                    || *aContent.offset(*aLeft.offset(iLeft as isize) as isize)
                        < *aContent.offset(*aRight.offset(iRight as isize) as isize))
            {
                let c2rust_fresh21 = iLeft;
                iLeft = iLeft + 1;
                logpage = *aLeft.offset(c2rust_fresh21 as isize);
            } else {
                let c2rust_fresh22 = iRight;
                iRight = iRight + 1;
                logpage = *aRight.offset(c2rust_fresh22 as isize);
            }
            dbpage = *aContent.offset(logpage as isize) as Pgno;
            let c2rust_fresh23 = iOut;
            iOut = iOut + 1;
            *aTmp.offset(c2rust_fresh23 as isize) = logpage;
            if iLeft < nLeft
                && *aContent.offset(*aLeft.offset(iLeft as isize) as isize) == dbpage
            {
                iLeft += 1;
            }
        }
        *paRight = aLeft;
        *pnRight = iOut;
        memcpy(
            aLeft as *mut ::core::ffi::c_void,
            aTmp as *const ::core::ffi::c_void,
            (::core::mem::size_of::<ht_slot>() as size_t).wrapping_mul(iOut as size_t),
        );
    }
}
unsafe extern "C" fn walMergesort(
    mut aContent: *const u32_0,
    mut aBuffer: *mut ht_slot,
    mut aList: *mut ht_slot,
    mut pnList: *mut ::core::ffi::c_int,
) {
    unsafe {
        let nList: ::core::ffi::c_int = *pnList;
        let mut nMerge: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aMerge: *mut ht_slot = ::core::ptr::null_mut::<ht_slot>();
        let mut iList: ::core::ffi::c_int = 0;
        let mut iSub: u32_0 = 0 as u32_0;
        let mut aSub: [Sublist; 13] = [Sublist {
            nList: 0,
            aList: ::core::ptr::null_mut::<ht_slot>()

        }; 13];
        memset(
            &raw mut aSub as *mut Sublist as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[Sublist; 13]>() as size_t,
        );
        iList = 0 as ::core::ffi::c_int;
        while iList < nList {
            nMerge = 1 as ::core::ffi::c_int;
            aMerge = aList.offset(iList as isize) as *mut ht_slot;
            iSub = 0 as u32_0;
            while iList & (1 as ::core::ffi::c_int) << iSub != 0 {
                let mut p: *mut Sublist = ::core::ptr::null_mut::<Sublist>();
                p = (&raw mut aSub as *mut Sublist).offset(iSub as isize)
                    as *mut Sublist;
                walMerge(
                    aContent,
                    (*p).aList,
                    (*p).nList,
                    &raw mut aMerge,
                    &raw mut nMerge,
                    aBuffer,
                );
                iSub = iSub.wrapping_add(1);
            }
            aSub[iSub as usize].aList = aMerge;
            aSub[iSub as usize].nList = nMerge;
            iList += 1;
        }
        iSub = iSub.wrapping_add(1);
        while iSub
            < (::core::mem::size_of::<[Sublist; 13]>() as usize)
                .wrapping_div(::core::mem::size_of::<Sublist>() as usize)
                as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if nList & (1 as ::core::ffi::c_int) << iSub != 0 {
                let mut p_0: *mut Sublist = ::core::ptr::null_mut::<Sublist>();
                p_0 = (&raw mut aSub as *mut Sublist).offset(iSub as isize)
                    as *mut Sublist;
                walMerge(
                    aContent,
                    (*p_0).aList,
                    (*p_0).nList,
                    &raw mut aMerge,
                    &raw mut nMerge,
                    aBuffer,
                );
            }
            iSub = iSub.wrapping_add(1);
        }
        *pnList = nMerge;
    }
}
unsafe extern "C" fn walIteratorFree(mut p: *mut WalIterator) {
    unsafe {
        sqlite3_free(p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn walIteratorInit(
    mut pWal: *mut Wal,
    mut nBackfill: u32_0,
    mut pp: *mut *mut WalIterator,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut WalIterator = ::core::ptr::null_mut::<WalIterator>();
        let mut nSegment: ::core::ffi::c_int = 0;
        let mut iLast: u32_0 = 0;
        let mut nByte: sqlite3_int64 = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut aTmp: *mut ht_slot = ::core::ptr::null_mut::<ht_slot>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        iLast = (*pWal).hdr.mxFrame;
        nSegment = walFramePage(iLast) + 1 as ::core::ffi::c_int;
        nByte = (8 as usize)
            .wrapping_add(
                (nSegment as usize)
                    .wrapping_mul(::core::mem::size_of::<WalSegment>() as usize),
            )
            .wrapping_add(
                (iLast as usize).wrapping_mul(::core::mem::size_of::<ht_slot>() as usize),
            ) as sqlite3_int64;
        p = sqlite3_malloc64(
            (nByte as sqlite3_uint64)
                .wrapping_add(
                    (::core::mem::size_of::<ht_slot>() as usize)
                        .wrapping_mul(
                            (if iLast > HASHTABLE_NPAGE as ::core::ffi::c_uint {
                                HASHTABLE_NPAGE as ::core::ffi::c_uint
                            } else {
                                iLast as ::core::ffi::c_uint
                            }) as usize,
                        ) as sqlite3_uint64,
                ),
        ) as *mut WalIterator;
        if p.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        memset(p as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, nByte as size_t);
        (*p).nSegment = nSegment;
        aTmp = (p as *mut u8_0).offset(nByte as isize) as *mut u8_0 as *mut ht_slot;
        i = walFramePage(nBackfill.wrapping_add(1 as u32_0));
        while rc == SQLITE_OK && i < nSegment {
            let mut sLoc: WalHashLoc = WalHashLoc {
                aHash: ::core::ptr::null_mut::<ht_slot>(),
                aPgno: ::core::ptr::null_mut::<u32_0>(),
                iZero: 0

            };
            rc = walHashGet(pWal, i, &raw mut sLoc);
            if rc == SQLITE_OK {
                let mut j: ::core::ffi::c_int = 0;
                let mut nEntry: ::core::ffi::c_int = 0;
                let mut aIndex: *mut ht_slot = ::core::ptr::null_mut::<ht_slot>();
                if i + 1 as ::core::ffi::c_int == nSegment {
                    nEntry = iLast.wrapping_sub(sLoc.iZero) as ::core::ffi::c_int;
                } else {
                    nEntry = (sLoc.aHash as *mut u32_0)
                        .offset_from(sLoc.aPgno as *mut u32_0) as ::core::ffi::c_long
                        as ::core::ffi::c_int;
                }
                aIndex = ((&raw mut (*p).aSegment as *mut WalSegment)
                    .offset((*p).nSegment as isize) as *mut WalSegment as *mut ht_slot)
                    .offset(sLoc.iZero as isize) as *mut ht_slot;
                sLoc.iZero = sLoc.iZero.wrapping_add(1);
                j = 0 as ::core::ffi::c_int;
                while j < nEntry {
                    *aIndex.offset(j as isize) = j as ht_slot;
                    j += 1;
                }
                walMergesort(sLoc.aPgno as *mut u32_0, aTmp, aIndex, &raw mut nEntry);
                (*(&raw mut (*p).aSegment as *mut WalSegment).offset(i as isize))
                    .iZero = sLoc.iZero as ::core::ffi::c_int;
                (*(&raw mut (*p).aSegment as *mut WalSegment).offset(i as isize))
                    .nEntry = nEntry;
                let ref mut c2rust_fresh19 = (*(&raw mut (*p).aSegment
                    as *mut WalSegment)
                    .offset(i as isize))
                    .aIndex;
                *c2rust_fresh19 = aIndex;
                let ref mut c2rust_fresh20 = (*(&raw mut (*p).aSegment
                    as *mut WalSegment)
                    .offset(i as isize))
                    .aPgno;
                *c2rust_fresh20 = sLoc.aPgno as *mut u32_0;
            }
            i += 1;
        }
        if rc != SQLITE_OK {
            walIteratorFree(p);
            p = ::core::ptr::null_mut::<WalIterator>();
        }
        *pp = p;
        return rc;
    }
}
unsafe extern "C" fn walBusyLock(
    mut pWal: *mut Wal,
    mut xBusy: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    mut pBusyArg: *mut ::core::ffi::c_void,
    mut lockIdx: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        loop {
            rc = walLockExclusive(pWal, lockIdx, n);
            if !(xBusy.is_some() && rc == SQLITE_BUSY
                && xBusy.expect("non-null function pointer")(pBusyArg) != 0)
            {
                break;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn walPagesize(mut pWal: *mut Wal) -> ::core::ffi::c_int {
    unsafe {
        return ((*pWal).hdr.szPage as ::core::ffi::c_int & 0xfe00 as ::core::ffi::c_int)
            + (((*pWal).hdr.szPage as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn walRestartHdr(mut pWal: *mut Wal, mut salt1: u32_0) {
    unsafe {
        let mut pInfo: *mut WalCkptInfo = walCkptInfo(pWal);
        let mut i: ::core::ffi::c_int = 0;
        let mut aSalt: *mut u32_0 = &raw mut (*pWal).hdr.aSalt as *mut u32_0;
        (*pWal).nCkpt = (*pWal).nCkpt.wrapping_add(1);
        (*pWal).hdr.mxFrame = 0 as u32_0;
        sqlite3Put4byte(
            aSalt.offset(0 as ::core::ffi::c_int as isize) as *mut u32_0 as *mut u8_0,
            (1 as u32_0)
                .wrapping_add(
                    sqlite3Get4byte(
                        aSalt.offset(0 as ::core::ffi::c_int as isize) as *mut u32_0
                            as *mut u8_0,
                    ),
                ),
        );
        memcpy(
            (&raw mut (*pWal).hdr.aSalt as *mut u32_0)
                .offset(1 as ::core::ffi::c_int as isize) as *mut u32_0
                as *mut ::core::ffi::c_void,
            &raw mut salt1 as *const ::core::ffi::c_void,
            4 as size_t,
        );
        walIndexWriteHdr(pWal);
        ;
        ::core::ptr::write_volatile(
            &mut (*pInfo).nBackfillAttempted as *mut u32_0,
            0 as u32_0,
        );
        ::core::ptr::write_volatile(
            &mut (*pInfo).aReadMark[1 as ::core::ffi::c_int as usize] as *mut u32_0,
            0 as u32_0,
        );
        i = 2 as ::core::ffi::c_int;
        while i < WAL_NREADER {
            ::core::ptr::write_volatile(
                &mut (*pInfo).aReadMark[i as usize] as *mut u32_0,
                READMARK_NOT_USED as u32_0,
            );
            i += 1;
        }
    }
}
unsafe extern "C" fn walCheckpoint(
    mut pWal: *mut Wal,
    mut db: *mut sqlite3,
    mut eMode: ::core::ffi::c_int,
    mut xBusy: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    mut pBusyArg: *mut ::core::ffi::c_void,
    mut sync_flags: ::core::ffi::c_int,
    mut zBuf: *mut u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut szPage: ::core::ffi::c_int = 0;
        let mut pIter: *mut WalIterator = ::core::ptr::null_mut::<WalIterator>();
        let mut iDbpage: u32_0 = 0 as u32_0;
        let mut iFrame: u32_0 = 0 as u32_0;
        let mut mxSafeFrame: u32_0 = 0;
        let mut mxPage: u32_0 = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut pInfo: *mut WalCkptInfo = ::core::ptr::null_mut::<WalCkptInfo>();
        szPage = walPagesize(pWal);
        pInfo = walCkptInfo(pWal);
        if (*pInfo).nBackfill < (*pWal).hdr.mxFrame {
            mxSafeFrame = (*pWal).hdr.mxFrame;
            mxPage = (*pWal).hdr.nPage;
            i = 1 as ::core::ffi::c_int;
            loop {
                if !(i < WAL_NREADER) {
                    c2rust_current_block = 5634871135123216486;
                    break;
                }
                let mut y: u32_0 = (*pInfo).aReadMark[i as usize];
                if mxSafeFrame > y {
                    rc = walBusyLock(
                        pWal,
                        xBusy,
                        pBusyArg,
                        3 as ::core::ffi::c_int + i,
                        1 as ::core::ffi::c_int,
                    );
                    if rc == SQLITE_OK {
                        let mut iMark: u32_0 = if i == 1 as ::core::ffi::c_int {
                            mxSafeFrame
                        } else {
                            READMARK_NOT_USED
                        };
                        ;
                        walUnlockExclusive(
                            pWal,
                            3 as ::core::ffi::c_int + i,
                            1 as ::core::ffi::c_int,
                        );
                    } else {
                        if !(rc == SQLITE_BUSY) {
                            c2rust_current_block = 2310077433060450808;
                            break;
                        }
                        mxSafeFrame = y;
                        xBusy = None;
                    }
                }
                i += 1;
            }
            match c2rust_current_block {
                2310077433060450808 => {}
                _ => {
                    if (*pInfo).nBackfill < mxSafeFrame {
                        rc = walIteratorInit(pWal, (*pInfo).nBackfill, &raw mut pIter);
                    }
                    if !pIter.is_null()
                        && {
                            rc = walBusyLock(
                                pWal,
                                xBusy,
                                pBusyArg,
                                3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
                                1 as ::core::ffi::c_int,
                            );
                            rc == SQLITE_OK
                        }
                    {
                        let mut nBackfill: u32_0 = (*pInfo).nBackfill;
                        ::core::ptr::write_volatile(
                            &mut (*pInfo).nBackfillAttempted as *mut u32_0,
                            mxSafeFrame,
                        );
                        rc = sqlite3OsSync(
                            (*pWal).pWalFd,
                            sync_flags >> 2 as ::core::ffi::c_int
                                & 0x3 as ::core::ffi::c_int,
                        );
                        if rc == SQLITE_OK {
                            let mut nReq: i64_0 = mxPage as i64_0 * szPage as i64_0;
                            let mut nSize: i64_0 = 0;
                            sqlite3OsFileControl(
                                (*pWal).pDbFd,
                                SQLITE_FCNTL_CKPT_START,
                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            );
                            rc = sqlite3OsFileSize((*pWal).pDbFd, &raw mut nSize);
                            if rc == SQLITE_OK && nSize < nReq {
                                if (nSize as ::core::ffi::c_longlong
                                    + 65536 as ::core::ffi::c_longlong
                                    + (*pWal).hdr.mxFrame as ::core::ffi::c_longlong
                                        * szPage as ::core::ffi::c_longlong) < nReq
                                {
                                    rc = sqlite3CorruptError(2276 as ::core::ffi::c_int);
                                } else {
                                    sqlite3OsFileControlHint(
                                        (*pWal).pDbFd,
                                        SQLITE_FCNTL_SIZE_HINT,
                                        &raw mut nReq as *mut ::core::ffi::c_void,
                                    );
                                }
                            }
                        }
                        while rc == SQLITE_OK
                            && 0 as ::core::ffi::c_int
                                == walIteratorNext(pIter, &raw mut iDbpage, &raw mut iFrame)
                        {
                            let mut iOffset: i64_0 = 0;
                            if (*db).u1.isInterrupted != 0
                            {
                                rc = if (*db).mallocFailed as ::core::ffi::c_int != 0 {
                                    SQLITE_NOMEM_BKPT
                                } else {
                                    SQLITE_INTERRUPT
                                };
                                break;
                            } else {
                                if iFrame <= nBackfill || iFrame > mxSafeFrame
                                    || iDbpage > mxPage
                                {
                                    continue;
                                }
                                iOffset = (WAL_HDRSIZE as ::core::ffi::c_longlong
                                    + (iFrame as ::core::ffi::c_uint)
                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                        as ::core::ffi::c_longlong
                                        * (szPage + WAL_FRAME_HDRSIZE) as ::core::ffi::c_longlong
                                    + WAL_FRAME_HDRSIZE as ::core::ffi::c_longlong) as i64_0;
                                rc = sqlite3OsRead(
                                    (*pWal).pWalFd,
                                    zBuf as *mut ::core::ffi::c_void,
                                    szPage,
                                    iOffset,
                                );
                                if rc != SQLITE_OK {
                                    break;
                                }
                                iOffset = (iDbpage as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint) as i64_0
                                    * szPage as i64_0;
                                rc = sqlite3OsWrite(
                                    (*pWal).pDbFd,
                                    zBuf as *const ::core::ffi::c_void,
                                    szPage,
                                    iOffset,
                                );
                                if rc != SQLITE_OK {
                                    break;
                                }
                            }
                        }
                        sqlite3OsFileControl(
                            (*pWal).pDbFd,
                            SQLITE_FCNTL_CKPT_DONE,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        );
                        if rc == SQLITE_OK {
                            if mxSafeFrame == (*walIndexHdr(pWal)).mxFrame {
                                let mut szDb: i64_0 = (*pWal).hdr.nPage as i64_0
                                    * szPage as i64_0;
                                rc = sqlite3OsTruncate((*pWal).pDbFd, szDb);
                                if rc == SQLITE_OK {
                                    rc = sqlite3OsSync(
                                        (*pWal).pDbFd,
                                        sync_flags >> 2 as ::core::ffi::c_int
                                            & 0x3 as ::core::ffi::c_int,
                                    );
                                }
                            }
                            if rc == SQLITE_OK {
                                ;
                            }
                        }
                        walUnlockExclusive(
                            pWal,
                            3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                        );
                    }
                    if rc == SQLITE_BUSY {
                        rc = SQLITE_OK;
                    }
                    c2rust_current_block = 12264624100856317061;
                }
            }
        } else {
            c2rust_current_block = 12264624100856317061;
        }
        match c2rust_current_block {
            12264624100856317061 => {
                if rc == SQLITE_OK && eMode != SQLITE_CHECKPOINT_PASSIVE {
                    if (*pInfo).nBackfill < (*pWal).hdr.mxFrame {
                        rc = SQLITE_BUSY;
                    } else if eMode >= SQLITE_CHECKPOINT_RESTART {
                        let mut salt1: u32_0 = 0;
                        sqlite3_randomness(
                            4 as ::core::ffi::c_int,
                            &raw mut salt1 as *mut ::core::ffi::c_void,
                        );
                        rc = walBusyLock(
                            pWal,
                            xBusy,
                            pBusyArg,
                            3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                            WAL_NREADER - 1 as ::core::ffi::c_int,
                        );
                        if rc == SQLITE_OK {
                            if eMode == SQLITE_CHECKPOINT_TRUNCATE {
                                walRestartHdr(pWal, salt1);
                                rc = sqlite3OsTruncate((*pWal).pWalFd, 0 as i64_0);
                            }
                            walUnlockExclusive(
                                pWal,
                                3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                WAL_NREADER - 1 as ::core::ffi::c_int,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        walIteratorFree(pIter);
        return rc;
    }
}
unsafe extern "C" fn walLimitSize(mut pWal: *mut Wal, mut nMax: i64_0) {
    unsafe {
        let mut sz: i64_0 = 0;
        let mut rx: ::core::ffi::c_int = 0;
        sqlite3BeginBenignMalloc();
        rx = sqlite3OsFileSize((*pWal).pWalFd, &raw mut sz);
        if rx == SQLITE_OK && sz > nMax {
            rx = sqlite3OsTruncate((*pWal).pWalFd, nMax);
        }
        sqlite3EndBenignMalloc();
        if rx != 0 {
            sqlite3_log(
                rx,
                b"cannot limit WAL size: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*pWal).zWalName,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalClose(
    mut pWal: *mut Wal,
    mut db: *mut sqlite3,
    mut sync_flags: ::core::ffi::c_int,
    mut nBuf: ::core::ffi::c_int,
    mut zBuf: *mut u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !pWal.is_null() {
            let mut isDelete: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if !zBuf.is_null()
                && {
                    rc = sqlite3OsLock((*pWal).pDbFd, SQLITE_LOCK_EXCLUSIVE);
                    SQLITE_OK == rc
                }
            {
                if (*pWal).exclusiveMode as ::core::ffi::c_int == WAL_NORMAL_MODE {
                    (*pWal).exclusiveMode = WAL_EXCLUSIVE_MODE as u8_0;
                }
                rc = sqlite3WalCheckpoint(
                    pWal,
                    db,
                    SQLITE_CHECKPOINT_PASSIVE,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    sync_flags,
                    nBuf,
                    zBuf,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if rc == SQLITE_OK {
                    let mut bPersist: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                    sqlite3OsFileControlHint(
                        (*pWal).pDbFd,
                        SQLITE_FCNTL_PERSIST_WAL,
                        &raw mut bPersist as *mut ::core::ffi::c_void,
                    );
                    if bPersist != 1 as ::core::ffi::c_int {
                        isDelete = 1 as ::core::ffi::c_int;
                    } else if (*pWal).mxWalSize >= 0 as ::core::ffi::c_longlong {
                        walLimitSize(pWal, 0 as i64_0);
                    }
                }
            }
            walIndexClose(pWal, isDelete);
            sqlite3OsClose((*pWal).pWalFd);
            if isDelete != 0 {
                sqlite3BeginBenignMalloc();
                sqlite3OsDelete((*pWal).pVfs, (*pWal).zWalName, 0 as ::core::ffi::c_int);
                sqlite3EndBenignMalloc();
            }
            sqlite3_free((*pWal).apWiData as *mut ::core::ffi::c_void);
            sqlite3_free(pWal as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn walIndexTryHdr(
    mut pWal: *mut Wal,
    mut pChanged: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aCksum: [u32_0; 2] = [0; 2];
        let mut h1: WalIndexHdr = WalIndexHdr {
            iVersion: 0,
            unused: 0,
            iChange: 0,
            isInit: 0,
            bigEndCksum: 0,
            szPage: 0,
            mxFrame: 0,
            nPage: 0,
            aFrameCksum: [0; 2],
            aSalt: [0; 2],
            aCksum: [0; 2]

        };
        let mut h2: WalIndexHdr = WalIndexHdr {
            iVersion: 0,
            unused: 0,
            iChange: 0,
            isInit: 0,
            bigEndCksum: 0,
            szPage: 0,
            mxFrame: 0,
            nPage: 0,
            aFrameCksum: [0; 2],
            aSalt: [0; 2],
            aCksum: [0; 2]

        };
        let mut aHdr: *mut WalIndexHdr = ::core::ptr::null_mut::<WalIndexHdr>();
        aHdr = walIndexHdr(pWal);
        memcpy(
            &raw mut h1 as *mut ::core::ffi::c_void,
            aHdr.offset(0 as ::core::ffi::c_int as isize) as *mut WalIndexHdr
                as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<WalIndexHdr>() as size_t,
        );
        walShmBarrier(pWal);
        memcpy(
            &raw mut h2 as *mut ::core::ffi::c_void,
            aHdr.offset(1 as ::core::ffi::c_int as isize) as *mut WalIndexHdr
                as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<WalIndexHdr>() as size_t,
        );
        if memcmp(
            &raw mut h1 as *const ::core::ffi::c_void,
            &raw mut h2 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<WalIndexHdr>() as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        if h1.isInit as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        walChecksumBytes(
            1 as ::core::ffi::c_int,
            &raw mut h1 as *mut u8_0,
            (::core::mem::size_of::<WalIndexHdr>() as usize)
                .wrapping_sub(::core::mem::size_of::<[u32_0; 2]>() as usize)
                as ::core::ffi::c_int,
            ::core::ptr::null::<u32_0>(),
            &raw mut aCksum as *mut u32_0,
        );
        if aCksum[0 as ::core::ffi::c_int as usize]
            != h1.aCksum[0 as ::core::ffi::c_int as usize]
            || aCksum[1 as ::core::ffi::c_int as usize]
                != h1.aCksum[1 as ::core::ffi::c_int as usize]
        {
            return 1 as ::core::ffi::c_int;
        }
        if memcmp(
            &raw mut (*pWal).hdr as *const ::core::ffi::c_void,
            &raw mut h1 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<WalIndexHdr>() as size_t,
        ) != 0
        {
            *pChanged = 1 as ::core::ffi::c_int;
            memcpy(
                &raw mut (*pWal).hdr as *mut ::core::ffi::c_void,
                &raw mut h1 as *const ::core::ffi::c_void,
                ::core::mem::size_of::<WalIndexHdr>() as size_t,
            );
            (*pWal).szPage = (((*pWal).hdr.szPage as ::core::ffi::c_int
                & 0xfe00 as ::core::ffi::c_int)
                + (((*pWal).hdr.szPage as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int)
                    << 16 as ::core::ffi::c_int)) as u32_0;
        }
        return 0 as ::core::ffi::c_int;
    }
}
pub const WAL_RETRY: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
unsafe extern "C" fn walIndexReadHdr(
    mut pWal: *mut Wal,
    mut pChanged: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut badHdr: ::core::ffi::c_int = 0;
        let mut page0: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
        rc = walIndexPage(pWal, 0 as ::core::ffi::c_int, &raw mut page0);
        if rc != SQLITE_OK {
            if rc == SQLITE_READONLY_CANTINIT {
                (*pWal).bShmUnreliable = 1 as u8_0;
                (*pWal).exclusiveMode = WAL_HEAPMEMORY_MODE as u8_0;
                *pChanged = 1 as ::core::ffi::c_int;
            } else {
                return rc
            }
        }
        badHdr = if !page0.is_null() {
            walIndexTryHdr(pWal, pChanged)
        } else {
            1 as ::core::ffi::c_int
        };
        if badHdr != 0 {
            if (*pWal).bShmUnreliable as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (*pWal).readOnly as ::core::ffi::c_int & WAL_SHM_RDONLY != 0
            {
                rc = walLockShared(pWal, WAL_WRITE_LOCK);
                if SQLITE_OK == rc {
                    walUnlockShared(pWal, WAL_WRITE_LOCK);
                    rc = SQLITE_READONLY_RECOVERY;
                }
            } else {
                let mut bWriteLock: ::core::ffi::c_int = (*pWal).writeLock
                    as ::core::ffi::c_int;
                if bWriteLock != 0
                    || {
                        rc = walLockExclusive(
                            pWal,
                            WAL_WRITE_LOCK,
                            1 as ::core::ffi::c_int,
                        );
                        SQLITE_OK == rc
                    }
                {
                    if bWriteLock == 0 {
                        (*pWal).writeLock = 2 as u8_0;
                    }
                    rc = walIndexPage(pWal, 0 as ::core::ffi::c_int, &raw mut page0);
                    if SQLITE_OK == rc {
                        badHdr = walIndexTryHdr(pWal, pChanged);
                        if badHdr != 0 {
                            rc = walIndexRecover(pWal);
                            *pChanged = 1 as ::core::ffi::c_int;
                        }
                    }
                    if bWriteLock == 0 as ::core::ffi::c_int {
                        (*pWal).writeLock = 0 as u8_0;
                        walUnlockExclusive(
                            pWal,
                            WAL_WRITE_LOCK,
                            1 as ::core::ffi::c_int,
                        );
                    }
                }
            }
        }
        if badHdr == 0 as ::core::ffi::c_int
            && (*pWal).hdr.iVersion != WALINDEX_MAX_VERSION as ::core::ffi::c_uint
        {
            rc = sqlite3CantopenError(2727 as ::core::ffi::c_int);
        }
        if (*pWal).bShmUnreliable != 0 {
            if rc != SQLITE_OK {
                walIndexClose(pWal, 0 as ::core::ffi::c_int);
                (*pWal).bShmUnreliable = 0 as u8_0;
                if rc == SQLITE_IOERR_SHORT_READ {
                    rc = WAL_RETRY;
                }
            }
            (*pWal).exclusiveMode = WAL_NORMAL_MODE as u8_0;
        }
        return rc;
    }
}
unsafe extern "C" fn walBeginShmUnreliable(
    mut pWal: *mut Wal,
    mut pChanged: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut szWal: i64_0 = 0;
        let mut iOffset: i64_0 = 0;
        let mut aBuf: [u8_0; 32] = [0; 32];
        let mut aFrame: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut szFrame: ::core::ffi::c_int = 0;
        let mut aData: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pDummy: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut aSaveCksum: [u32_0; 2] = [0; 2];
        rc = walLockShared(pWal, 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int);
        if rc != SQLITE_OK {
            if rc == SQLITE_BUSY {
                rc = WAL_RETRY;
            }
        } else {
            (*pWal).readLock = 0 as i16_0;
            rc = sqlite3OsShmMap(
                (*pWal).pDbFd,
                0 as ::core::ffi::c_int,
                WALINDEX_PGSZ as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                &raw mut pDummy,
            );
            if rc != SQLITE_READONLY_CANTINIT {
                rc = if rc == SQLITE_READONLY { WAL_RETRY } else { rc };
            } else {
                memcpy(
                    &raw mut (*pWal).hdr as *mut ::core::ffi::c_void,
                    walIndexHdr(pWal) as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<WalIndexHdr>() as size_t,
                );
                rc = sqlite3OsFileSize((*pWal).pWalFd, &raw mut szWal);
                if !(rc != SQLITE_OK) {
                    if szWal < WAL_HDRSIZE as ::core::ffi::c_longlong {
                        *pChanged = 1 as ::core::ffi::c_int;
                        rc = if (*pWal).hdr.mxFrame == 0 as ::core::ffi::c_uint {
                            SQLITE_OK
                        } else {
                            WAL_RETRY
                        };
                    } else {
                        rc = sqlite3OsRead(
                            (*pWal).pWalFd,
                            &raw mut aBuf as *mut u8_0 as *mut ::core::ffi::c_void,
                            WAL_HDRSIZE,
                            0 as i64_0,
                        );
                        if !(rc != SQLITE_OK) {
                            if memcmp(
                                &raw mut (*pWal).hdr.aSalt as *const ::core::ffi::c_void,
                                (&raw mut aBuf as *mut u8_0)
                                    .offset(16 as ::core::ffi::c_int as isize) as *mut u8_0
                                    as *const ::core::ffi::c_void,
                                8 as size_t,
                            ) != 0
                            {
                                rc = WAL_RETRY;
                            } else {
                                szFrame = ((*pWal).szPage as ::core::ffi::c_uint)
                                    .wrapping_add(WAL_FRAME_HDRSIZE as ::core::ffi::c_uint)
                                    as ::core::ffi::c_int;
                                aFrame = sqlite3_malloc64(szFrame as sqlite3_uint64)
                                    as *mut u8_0;
                                if aFrame.is_null() {
                                    rc = SQLITE_NOMEM_BKPT;
                                } else {
                                    aData = aFrame.offset(WAL_FRAME_HDRSIZE as isize)
                                        as *mut u8_0;
                                    aSaveCksum[0 as ::core::ffi::c_int as usize] = (*pWal)
                                        .hdr
                                        .aFrameCksum[0 as ::core::ffi::c_int as usize];
                                    aSaveCksum[1 as ::core::ffi::c_int as usize] = (*pWal)
                                        .hdr
                                        .aFrameCksum[1 as ::core::ffi::c_int as usize];
                                    iOffset = WAL_HDRSIZE as i64_0
                                        + ((*pWal).hdr.mxFrame as ::core::ffi::c_uint)
                                            .wrapping_add(1 as ::core::ffi::c_uint)
                                            .wrapping_sub(1 as ::core::ffi::c_uint) as i64_0
                                            * ((*pWal).szPage as ::core::ffi::c_uint)
                                                .wrapping_add(WAL_FRAME_HDRSIZE as ::core::ffi::c_uint)
                                                as i64_0;
                                    while iOffset as ::core::ffi::c_longlong
                                        + szFrame as ::core::ffi::c_longlong <= szWal
                                    {
                                        let mut pgno: u32_0 = 0;
                                        let mut nTruncate: u32_0 = 0;
                                        rc = sqlite3OsRead(
                                            (*pWal).pWalFd,
                                            aFrame as *mut ::core::ffi::c_void,
                                            szFrame,
                                            iOffset,
                                        );
                                        if rc != SQLITE_OK {
                                            break;
                                        }
                                        if walDecodeFrame(
                                            pWal,
                                            &raw mut pgno,
                                            &raw mut nTruncate,
                                            aData,
                                            aFrame,
                                        ) == 0
                                        {
                                            break;
                                        }
                                        if nTruncate != 0 {
                                            rc = WAL_RETRY;
                                            break;
                                        } else {
                                            iOffset += szFrame as ::core::ffi::c_longlong;
                                        }
                                    }
                                    (*pWal).hdr.aFrameCksum[0 as ::core::ffi::c_int as usize] = aSaveCksum[0
                                        as ::core::ffi::c_int as usize];
                                    (*pWal).hdr.aFrameCksum[1 as ::core::ffi::c_int as usize] = aSaveCksum[1
                                        as ::core::ffi::c_int as usize];
                                }
                            }
                        }
                    }
                }
            }
        }
        sqlite3_free(aFrame as *mut ::core::ffi::c_void);
        if rc != SQLITE_OK {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pWal).nWiData {
                sqlite3_free(
                    *(*pWal).apWiData.offset(i as isize) as *mut ::core::ffi::c_void,
                );
                let ref mut c2rust_fresh28 = *(*pWal).apWiData.offset(i as isize);
                *c2rust_fresh28 = ::core::ptr::null_mut::<u32_0>();
                i += 1;
            }
            (*pWal).bShmUnreliable = 0 as u8_0;
            sqlite3WalEndReadTransaction(pWal);
            *pChanged = 1 as ::core::ffi::c_int;
        }
        return rc;
    }
}
pub const WAL_RETRY_PROTOCOL_LIMIT: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const WAL_RETRY_BLOCKED_MASK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn walTryBeginRead(
    mut pWal: *mut Wal,
    mut pChanged: *mut ::core::ffi::c_int,
    mut useWal: ::core::ffi::c_int,
    mut pCnt: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pInfo: *mut WalCkptInfo = ::core::ptr::null_mut::<WalCkptInfo>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        *pCnt += 1;
        if *pCnt > 5 as ::core::ffi::c_int {
            let mut nDelay: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut cnt: ::core::ffi::c_int = *pCnt & !WAL_RETRY_BLOCKED_MASK;
            if cnt > WAL_RETRY_PROTOCOL_LIMIT {
                return SQLITE_PROTOCOL;
            }
            if *pCnt >= 10 as ::core::ffi::c_int {
                nDelay = (cnt - 9 as ::core::ffi::c_int)
                    * (cnt - 9 as ::core::ffi::c_int) * 39 as ::core::ffi::c_int;
            }
            sqlite3OsSleep((*pWal).pVfs, nDelay);
            *pCnt &= !WAL_RETRY_BLOCKED_MASK;
        }
        if useWal == 0 {
            if (*pWal).bShmUnreliable as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                rc = walIndexReadHdr(pWal, pChanged);
            }
            if rc == SQLITE_BUSY {
                if (*(*pWal).apWiData.offset(0 as ::core::ffi::c_int as isize)).is_null()
                {
                    rc = WAL_RETRY;
                } else {
                    rc = walLockShared(pWal, WAL_RECOVER_LOCK);
                    if SQLITE_OK == rc {
                        walUnlockShared(pWal, WAL_RECOVER_LOCK);
                        rc = WAL_RETRY;
                    } else if rc == SQLITE_BUSY {
                        rc = SQLITE_BUSY_RECOVERY;
                    }
                }
            }
            if rc != SQLITE_OK {
                return rc
            } else if (*pWal).bShmUnreliable != 0 {
                return walBeginShmUnreliable(pWal, pChanged)
            }
        }
        pInfo = walCkptInfo(pWal);
        let mut mxReadMark: u32_0 = 0;
        let mut mxI: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut mxFrame: u32_0 = 0;
        if useWal == 0
            && (*pInfo).nBackfill
                == (*pWal).hdr.mxFrame
        {
            rc = walLockShared(pWal, 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int);
            walShmBarrier(pWal);
            if rc == SQLITE_OK {
                if memcmp(
                    walIndexHdr(pWal) as *mut ::core::ffi::c_void,
                    &raw mut (*pWal).hdr as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<WalIndexHdr>() as size_t,
                ) != 0
                {
                    walUnlockShared(
                        pWal,
                        3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
                    );
                    return WAL_RETRY;
                }
                (*pWal).readLock = 0 as i16_0;
                return SQLITE_OK;
            } else if rc != SQLITE_BUSY {
                return rc
            }
        }
        mxReadMark = 0 as u32_0;
        mxI = 0 as ::core::ffi::c_int;
        mxFrame = (*pWal).hdr.mxFrame;
        i = 1 as ::core::ffi::c_int;
        while i < WAL_NREADER {
            let mut thisMark: u32_0 = (*pInfo).aReadMark[i as usize];
            if mxReadMark <= thisMark && thisMark <= mxFrame {
                mxReadMark = thisMark;
                mxI = i;
            }
            i += 1;
        }
        if (*pWal).readOnly as ::core::ffi::c_int & WAL_SHM_RDONLY
            == 0 as ::core::ffi::c_int
            && (mxReadMark < mxFrame || mxI == 0 as ::core::ffi::c_int)
        {
            i = 1 as ::core::ffi::c_int;
            while i < WAL_NREADER {
                rc = walLockExclusive(
                    pWal,
                    3 as ::core::ffi::c_int + i,
                    1 as ::core::ffi::c_int,
                );
                if rc == SQLITE_OK {
                    ;
                    mxReadMark = mxFrame;
                    mxI = i;
                    walUnlockExclusive(
                        pWal,
                        3 as ::core::ffi::c_int + i,
                        1 as ::core::ffi::c_int,
                    );
                    break;
                } else {
                    if rc != SQLITE_BUSY {
                        return rc;
                    }
                    i += 1;
                }
            }
        }
        if mxI == 0 as ::core::ffi::c_int {
            return if rc == SQLITE_BUSY { WAL_RETRY } else { SQLITE_READONLY_CANTINIT };
        }
        rc = walLockShared(pWal, 3 as ::core::ffi::c_int + mxI);
        if rc != 0 {
            return if rc & 0xff as ::core::ffi::c_int == SQLITE_BUSY {
                WAL_RETRY
            } else {
                rc
            };
        }
        (*pWal).minFrame = (*pInfo).nBackfill
            .wrapping_add(1 as ::core::ffi::c_uint) as u32_0;
        walShmBarrier(pWal);
        if (*pInfo).aReadMark[mxI as usize] != mxReadMark
            || memcmp(
                walIndexHdr(pWal) as *mut ::core::ffi::c_void,
                &raw mut (*pWal).hdr as *const ::core::ffi::c_void,
                ::core::mem::size_of::<WalIndexHdr>() as size_t,
            ) != 0
        {
            walUnlockShared(pWal, 3 as ::core::ffi::c_int + mxI);
            return WAL_RETRY;
        } else {
            (*pWal).readLock = mxI as i16_0;
        }
        return rc;
    }
}
unsafe extern "C" fn walBeginReadTransaction(
    mut pWal: *mut Wal,
    mut pChanged: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            rc = walTryBeginRead(pWal, pChanged, 0 as ::core::ffi::c_int, &raw mut cnt);
            if !(rc == WAL_RETRY) {
                break;
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalBeginReadTransaction(
    mut pWal: *mut Wal,
    mut pChanged: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = walBeginReadTransaction(pWal, pChanged);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalEndReadTransaction(mut pWal: *mut Wal) {
    unsafe {
        if (*pWal).readLock as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            sqlite3WalEndWriteTransaction(pWal);
            walUnlockShared(
                pWal,
                3 as ::core::ffi::c_int + (*pWal).readLock as ::core::ffi::c_int,
            );
            (*pWal).readLock = -1 as ::core::ffi::c_int as i16_0;
        }
    }
}
unsafe extern "C" fn walFindFrame(
    mut pWal: *mut Wal,
    mut pgno: Pgno,
    mut piRead: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iRead: u32_0 = 0 as u32_0;
        let mut iLast: u32_0 = (*pWal).hdr.mxFrame;
        let mut iHash: ::core::ffi::c_int = 0;
        let mut iMinHash: ::core::ffi::c_int = 0;
        if iLast == 0 as ::core::ffi::c_uint
            || (*pWal).readLock as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (*pWal).bShmUnreliable as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
        {
            *piRead = 0 as u32_0;
            return SQLITE_OK;
        }
        iMinHash = walFramePage((*pWal).minFrame);
        iHash = walFramePage(iLast);
        while iHash >= iMinHash {
            let mut sLoc: WalHashLoc = WalHashLoc {
                aHash: ::core::ptr::null_mut::<ht_slot>(),
                aPgno: ::core::ptr::null_mut::<u32_0>(),
                iZero: 0

            };
            let mut iKey: ::core::ffi::c_int = 0;
            let mut nCollide: ::core::ffi::c_int = 0;
            let mut rc: ::core::ffi::c_int = 0;
            let mut iH: u32_0 = 0;
            rc = walHashGet(pWal, iHash, &raw mut sLoc);
            if rc != SQLITE_OK {
                return rc;
            }
            nCollide = HASHTABLE_NSLOT;
            iKey = walHash(pgno as u32_0);
            loop {
                iH = sLoc.aHash.offset(iKey as isize) as *mut ht_slot as u32_0;
                if !(iH != 0 as ::core::ffi::c_uint) {
                    break;
                }
                let mut iFrame: u32_0 = iH.wrapping_add(sLoc.iZero);
                if iFrame <= iLast && iFrame >= (*pWal).minFrame
                    && *sLoc
                        .aPgno
                        .offset(
                            (iH as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint) as isize,
                        ) == pgno
                {
                    iRead = iFrame;
                }
                let c2rust_fresh29 = nCollide;
                nCollide = nCollide - 1;
                if c2rust_fresh29 == 0 as ::core::ffi::c_int {
                    *piRead = 0 as u32_0;
                    return sqlite3CorruptError(3577 as ::core::ffi::c_int);
                }
                iKey = walNextHash(iKey);
            }
            if iRead != 0 {
                break;
            }
            iHash -= 1;
        }
        *piRead = iRead;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalFindFrame(
    mut pWal: *mut Wal,
    mut pgno: Pgno,
    mut piRead: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = walFindFrame(pWal, pgno, piRead);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalReadFrame(
    mut pWal: *mut Wal,
    mut iRead: u32_0,
    mut nOut: ::core::ffi::c_int,
    mut pOut: *mut u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sz: ::core::ffi::c_int = 0;
        let mut iOffset: i64_0 = 0;
        sz = (*pWal).hdr.szPage as ::core::ffi::c_int;
        sz = (sz & 0xfe00 as ::core::ffi::c_int)
            + ((sz & 0x1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int);
        iOffset = (WAL_HDRSIZE as ::core::ffi::c_longlong
            + (iRead as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
                as ::core::ffi::c_longlong
                * (sz + WAL_FRAME_HDRSIZE) as ::core::ffi::c_longlong
            + WAL_FRAME_HDRSIZE as ::core::ffi::c_longlong) as i64_0;
        return sqlite3OsRead(
            (*pWal).pWalFd,
            pOut as *mut ::core::ffi::c_void,
            if nOut > sz { sz } else { nOut },
            iOffset,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalDbsize(mut pWal: *mut Wal) -> Pgno {
    unsafe {
        if !pWal.is_null()
            && (*pWal).readLock as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        {
            return (*pWal).hdr.nPage as Pgno;
        }
        return 0 as Pgno;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalBeginWriteTransaction(
    mut pWal: *mut Wal,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if (*pWal).readOnly != 0 {
            return SQLITE_READONLY;
        }
        rc = walLockExclusive(pWal, WAL_WRITE_LOCK, 1 as ::core::ffi::c_int);
        if rc != 0 {
            return rc;
        }
        (*pWal).writeLock = 1 as u8_0;
        if memcmp(
            &raw mut (*pWal).hdr as *const ::core::ffi::c_void,
            walIndexHdr(pWal) as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<WalIndexHdr>() as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            rc = SQLITE_BUSY_SNAPSHOT;
        }
        if rc != SQLITE_OK {
            walUnlockExclusive(pWal, WAL_WRITE_LOCK, 1 as ::core::ffi::c_int);
            (*pWal).writeLock = 0 as u8_0;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalEndWriteTransaction(
    mut pWal: *mut Wal,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pWal).writeLock != 0 {
            walUnlockExclusive(pWal, WAL_WRITE_LOCK, 1 as ::core::ffi::c_int);
            (*pWal).writeLock = 0 as u8_0;
            (*pWal).iReCksum = 0 as u32_0;
            (*pWal).truncateOnCommit = 0 as u8_0;
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalUndo(
    mut pWal: *mut Wal,
    mut xUndo: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, Pgno) -> ::core::ffi::c_int,
    >,
    mut pUndoCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pWal).writeLock != 0 {
            let mut iMax: Pgno = (*pWal).hdr.mxFrame as Pgno;
            let mut iFrame: Pgno = 0;
            memcpy(
                &raw mut (*pWal).hdr as *mut ::core::ffi::c_void,
                walIndexHdr(pWal) as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<WalIndexHdr>() as size_t,
            );
            iFrame = ((*pWal).hdr.mxFrame as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as Pgno;
            while rc == 0 as ::core::ffi::c_int && iFrame <= iMax {
                rc = xUndo
                    .expect(
                        "non-null function pointer",
                    )(pUndoCtx, walFramePgno(pWal, iFrame as u32_0) as Pgno);
                iFrame = iFrame.wrapping_add(1);
            }
            if iMax != (*pWal).hdr.mxFrame {
                walCleanupHash(pWal);
            }
            (*pWal).iReCksum = 0 as u32_0;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalSavepoint(
    mut pWal: *mut Wal,
    mut aWalData: *mut u32_0,
) {
    unsafe {
        *aWalData.offset(0 as ::core::ffi::c_int as isize) = (*pWal).hdr.mxFrame;
        *aWalData.offset(1 as ::core::ffi::c_int as isize) = (*pWal)
            .hdr
            .aFrameCksum[0 as ::core::ffi::c_int as usize];
        *aWalData.offset(2 as ::core::ffi::c_int as isize) = (*pWal)
            .hdr
            .aFrameCksum[1 as ::core::ffi::c_int as usize];
        *aWalData.offset(3 as ::core::ffi::c_int as isize) = (*pWal).nCkpt;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalSavepointUndo(
    mut pWal: *mut Wal,
    mut aWalData: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if *aWalData.offset(3 as ::core::ffi::c_int as isize) != (*pWal).nCkpt {
            *aWalData.offset(0 as ::core::ffi::c_int as isize) = 0 as u32_0;
            *aWalData.offset(3 as ::core::ffi::c_int as isize) = (*pWal).nCkpt;
        }
        if *aWalData.offset(0 as ::core::ffi::c_int as isize) < (*pWal).hdr.mxFrame {
            (*pWal).hdr.mxFrame = *aWalData.offset(0 as ::core::ffi::c_int as isize);
            (*pWal).hdr.aFrameCksum[0 as ::core::ffi::c_int as usize] = *aWalData
                .offset(1 as ::core::ffi::c_int as isize);
            (*pWal).hdr.aFrameCksum[1 as ::core::ffi::c_int as usize] = *aWalData
                .offset(2 as ::core::ffi::c_int as isize);
            walCleanupHash(pWal);
            if (*pWal).iReCksum > (*pWal).hdr.mxFrame {
                (*pWal).iReCksum = 0 as u32_0;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn walRestartLog(mut pWal: *mut Wal) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut cnt: ::core::ffi::c_int = 0;
        if (*pWal).readLock as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut pInfo: *mut WalCkptInfo = walCkptInfo(pWal);
            if (*pInfo).nBackfill > 0 as ::core::ffi::c_uint {
                let mut salt1: u32_0 = 0;
                sqlite3_randomness(
                    4 as ::core::ffi::c_int,
                    &raw mut salt1 as *mut ::core::ffi::c_void,
                );
                rc = walLockExclusive(
                    pWal,
                    3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                    WAL_NREADER - 1 as ::core::ffi::c_int,
                );
                if rc == SQLITE_OK {
                    walRestartHdr(pWal, salt1);
                    walUnlockExclusive(
                        pWal,
                        3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                        WAL_NREADER - 1 as ::core::ffi::c_int,
                    );
                } else if rc != SQLITE_BUSY {
                    return rc
                }
            }
            walUnlockShared(pWal, 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int);
            (*pWal).readLock = -1 as ::core::ffi::c_int as i16_0;
            cnt = 0 as ::core::ffi::c_int;
            loop {
                let mut notUsed: ::core::ffi::c_int = 0;
                rc = walTryBeginRead(
                    pWal,
                    &raw mut notUsed,
                    1 as ::core::ffi::c_int,
                    &raw mut cnt,
                );
                if !(rc == WAL_RETRY) {
                    break;
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn walWriteToLog(
    mut p: *mut WalWriter,
    mut pContent: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOffset: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if iOffset < (*p).iSyncPoint
            && iOffset as ::core::ffi::c_longlong + iAmt as ::core::ffi::c_longlong
                >= (*p).iSyncPoint
        {
            let mut iFirstAmt: ::core::ffi::c_int = ((*p).iSyncPoint - iOffset)
                as ::core::ffi::c_int;
            rc = sqlite3OsWrite((*p).pFd, pContent, iFirstAmt, iOffset as i64_0);
            if rc != 0 {
                return rc;
            }
            iOffset += iFirstAmt as ::core::ffi::c_longlong;
            iAmt -= iFirstAmt;
            pContent = (pContent as *mut ::core::ffi::c_char).offset(iFirstAmt as isize)
                as *mut ::core::ffi::c_void;
            rc = sqlite3OsSync((*p).pFd, (*p).syncFlags & 0x3 as ::core::ffi::c_int);
            if iAmt == 0 as ::core::ffi::c_int || rc != 0 {
                return rc;
            }
        }
        rc = sqlite3OsWrite((*p).pFd, pContent, iAmt, iOffset as i64_0);
        return rc;
    }
}
unsafe extern "C" fn walWriteOneFrame(
    mut p: *mut WalWriter,
    mut pPage: *mut PgHdr,
    mut nTruncate: ::core::ffi::c_int,
    mut iOffset: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut aFrame: [u8_0; 24] = [0; 24];
        pData = (*pPage).pData;
        walEncodeFrame(
            (*p).pWal,
            (*pPage).pgno as u32_0,
            nTruncate as u32_0,
            pData as *mut u8_0,
            &raw mut aFrame as *mut u8_0,
        );
        rc = walWriteToLog(
            p,
            &raw mut aFrame as *mut u8_0 as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[u8_0; 24]>() as ::core::ffi::c_int,
            iOffset,
        );
        if rc != 0 {
            return rc;
        }
        rc = walWriteToLog(
            p,
            pData,
            (*p).szPage,
            (iOffset as ::core::ffi::c_ulonglong)
                .wrapping_add(
                    ::core::mem::size_of::<[u8_0; 24]>() as ::core::ffi::c_ulonglong,
                ) as sqlite3_int64,
        );
        return rc;
    }
}
unsafe extern "C" fn walRewriteChecksums(
    mut pWal: *mut Wal,
    mut iLast: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let szPage: ::core::ffi::c_int = (*pWal).szPage as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut aBuf: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut aFrame: [u8_0; 24] = [0; 24];
        let mut iRead: u32_0 = 0;
        let mut iCksumOff: i64_0 = 0;
        aBuf = sqlite3_malloc(szPage + WAL_FRAME_HDRSIZE) as *mut u8_0;
        if aBuf.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        if (*pWal).iReCksum == 1 as ::core::ffi::c_uint {
            iCksumOff = 24 as i64_0;
        } else {
            iCksumOff = (WAL_HDRSIZE as ::core::ffi::c_longlong
                + ((*pWal).iReCksum as ::core::ffi::c_uint)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_longlong
                    * (szPage + WAL_FRAME_HDRSIZE) as ::core::ffi::c_longlong
                + 16 as ::core::ffi::c_longlong) as i64_0;
        }
        rc = sqlite3OsRead(
            (*pWal).pWalFd,
            aBuf as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<u32_0>() as usize).wrapping_mul(2 as usize)
                as ::core::ffi::c_int,
            iCksumOff,
        );
        (*pWal).hdr.aFrameCksum[0 as ::core::ffi::c_int as usize] = sqlite3Get4byte(
            aBuf,
        );
        (*pWal).hdr.aFrameCksum[1 as ::core::ffi::c_int as usize] = sqlite3Get4byte(
            aBuf.offset(::core::mem::size_of::<u32_0>() as isize) as *mut u8_0,
        );
        iRead = (*pWal).iReCksum;
        (*pWal).iReCksum = 0 as u32_0;
        while rc == SQLITE_OK && iRead <= iLast {
            let mut iOff: i64_0 = WAL_HDRSIZE as i64_0
                + (iRead as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
                    as i64_0 * (szPage + WAL_FRAME_HDRSIZE) as i64_0;
            rc = sqlite3OsRead(
                (*pWal).pWalFd,
                aBuf as *mut ::core::ffi::c_void,
                szPage + WAL_FRAME_HDRSIZE,
                iOff,
            );
            if rc == SQLITE_OK {
                let mut iPgno: u32_0 = 0;
                let mut nDbSize: u32_0 = 0;
                iPgno = sqlite3Get4byte(aBuf);
                nDbSize = sqlite3Get4byte(
                    aBuf.offset(4 as ::core::ffi::c_int as isize) as *mut u8_0,
                );
                walEncodeFrame(
                    pWal,
                    iPgno,
                    nDbSize,
                    aBuf.offset(WAL_FRAME_HDRSIZE as isize) as *mut u8_0,
                    &raw mut aFrame as *mut u8_0,
                );
                rc = sqlite3OsWrite(
                    (*pWal).pWalFd,
                    &raw mut aFrame as *mut u8_0 as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[u8_0; 24]>() as ::core::ffi::c_int,
                    iOff,
                );
            }
            iRead = iRead.wrapping_add(1);
        }
        sqlite3_free(aBuf as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn walFrames(
    mut pWal: *mut Wal,
    mut szPage: ::core::ffi::c_int,
    mut pList: *mut PgHdr,
    mut nTruncate: Pgno,
    mut isCommit: ::core::ffi::c_int,
    mut sync_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut iFrame: u32_0 = 0;
        let mut p: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        let mut pLast: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        let mut nExtra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut szFrame: ::core::ffi::c_int = 0;
        let mut iOffset: i64_0 = 0;
        let mut w: WalWriter = WalWriter {
            pWal: ::core::ptr::null_mut::<Wal>(),
            pFd: ::core::ptr::null_mut::<sqlite3_file>(),
            iSyncPoint: 0,
            syncFlags: 0,
            szPage: 0

        };
        let mut iFirst: u32_0 = 0 as u32_0;
        let mut pLive: *mut WalIndexHdr = ::core::ptr::null_mut::<WalIndexHdr>();
        pLive = walIndexHdr(pWal) as *mut WalIndexHdr;
        if memcmp(
            &raw mut (*pWal).hdr as *const ::core::ffi::c_void,
            pLive as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<WalIndexHdr>() as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            iFirst = ((*pLive).mxFrame as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as u32_0;
        }
        rc = walRestartLog(pWal);
        if SQLITE_OK != rc {
            return rc;
        }
        iFrame = (*pWal).hdr.mxFrame;
        if iFrame == 0 as ::core::ffi::c_uint {
            let mut aWalHdr: [u8_0; 32] = [0; 32];
            let mut aCksum: [u32_0; 2] = [0; 2];
            sqlite3Put4byte(
                (&raw mut aWalHdr as *mut u8_0).offset(0 as ::core::ffi::c_int as isize)
                    as *mut u8_0,
                (WAL_MAGIC | SQLITE_BIGENDIAN) as u32_0,
            );
            sqlite3Put4byte(
                (&raw mut aWalHdr as *mut u8_0).offset(4 as ::core::ffi::c_int as isize)
                    as *mut u8_0,
                WAL_MAX_VERSION as u32_0,
            );
            sqlite3Put4byte(
                (&raw mut aWalHdr as *mut u8_0).offset(8 as ::core::ffi::c_int as isize)
                    as *mut u8_0,
                szPage as u32_0,
            );
            sqlite3Put4byte(
                (&raw mut aWalHdr as *mut u8_0).offset(12 as ::core::ffi::c_int as isize)
                    as *mut u8_0,
                (*pWal).nCkpt,
            );
            if (*pWal).nCkpt == 0 as ::core::ffi::c_uint {
                sqlite3_randomness(
                    8 as ::core::ffi::c_int,
                    &raw mut (*pWal).hdr.aSalt as *mut u32_0 as *mut ::core::ffi::c_void,
                );
            }
            memcpy(
                (&raw mut aWalHdr as *mut u8_0).offset(16 as ::core::ffi::c_int as isize)
                    as *mut u8_0 as *mut ::core::ffi::c_void,
                &raw mut (*pWal).hdr.aSalt as *mut u32_0 as *const ::core::ffi::c_void,
                8 as size_t,
            );
            walChecksumBytes(
                1 as ::core::ffi::c_int,
                &raw mut aWalHdr as *mut u8_0,
                WAL_HDRSIZE - 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
                ::core::ptr::null::<u32_0>(),
                &raw mut aCksum as *mut u32_0,
            );
            sqlite3Put4byte(
                (&raw mut aWalHdr as *mut u8_0).offset(24 as ::core::ffi::c_int as isize)
                    as *mut u8_0,
                aCksum[0 as ::core::ffi::c_int as usize],
            );
            sqlite3Put4byte(
                (&raw mut aWalHdr as *mut u8_0).offset(28 as ::core::ffi::c_int as isize)
                    as *mut u8_0,
                aCksum[1 as ::core::ffi::c_int as usize],
            );
            (*pWal).szPage = szPage as u32_0;
            (*pWal).hdr.bigEndCksum = SQLITE_BIGENDIAN as u8_0;
            (*pWal).hdr.aFrameCksum[0 as ::core::ffi::c_int as usize] = aCksum[0
                as ::core::ffi::c_int as usize];
            (*pWal).hdr.aFrameCksum[1 as ::core::ffi::c_int as usize] = aCksum[1
                as ::core::ffi::c_int as usize];
            (*pWal).truncateOnCommit = 1 as u8_0;
            rc = sqlite3OsWrite(
                (*pWal).pWalFd,
                &raw mut aWalHdr as *mut u8_0 as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[u8_0; 32]>() as ::core::ffi::c_int,
                0 as i64_0,
            );
            if rc != SQLITE_OK {
                return rc;
            }
            if (*pWal).syncHeader != 0 {
                rc = sqlite3OsSync(
                    (*pWal).pWalFd,
                    sync_flags >> 2 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int,
                );
                if rc != 0 {
                    return rc;
                }
            }
        }
        if (*pWal).szPage as ::core::ffi::c_int != szPage {
            return sqlite3CorruptError(4104 as ::core::ffi::c_int);
        }
        w.pWal = pWal;
        w.pFd = (*pWal).pWalFd;
        w.iSyncPoint = 0 as sqlite3_int64;
        w.syncFlags = sync_flags;
        w.szPage = szPage;
        iOffset = WAL_HDRSIZE as i64_0
            + (iFrame as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_sub(1 as ::core::ffi::c_uint) as i64_0
                * (szPage + WAL_FRAME_HDRSIZE) as i64_0;
        szFrame = szPage + WAL_FRAME_HDRSIZE;
        let mut c2rust_current_block_67: u64;
        p = pList;
        while !p.is_null() {
            let mut nDbSize: ::core::ffi::c_int = 0;
            if iFirst != 0
                && (!(*p).pDirty.is_null() || isCommit == 0 as ::core::ffi::c_int)
            {
                let mut iWrite: u32_0 = 0 as u32_0;
                walFindFrame(pWal, (*p).pgno, &raw mut iWrite);
                if iWrite >= iFirst {
                    let mut iOff: i64_0 = WAL_HDRSIZE as i64_0
                        + (iWrite as ::core::ffi::c_uint)
                            .wrapping_sub(1 as ::core::ffi::c_uint) as i64_0
                            * (szPage + WAL_FRAME_HDRSIZE) as i64_0
                        + WAL_FRAME_HDRSIZE as i64_0;
                    let mut pData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                        ::core::ffi::c_void,
                    >();
                    if (*pWal).iReCksum == 0 as ::core::ffi::c_uint
                        || iWrite < (*pWal).iReCksum
                    {
                        (*pWal).iReCksum = iWrite;
                    }
                    pData = (*p).pData;
                    rc = sqlite3OsWrite((*pWal).pWalFd, pData, szPage, iOff);
                    if rc != 0 {
                        return rc;
                    }
                    (*p).flags = ((*p).flags as ::core::ffi::c_int & !PGHDR_WAL_APPEND)
                        as u16_0;
                    c2rust_current_block_67 = 6417057564578538666;
                } else {
                    c2rust_current_block_67 = 14220266465818359136;
                }
            } else {
                c2rust_current_block_67 = 14220266465818359136;
            }
            match c2rust_current_block_67 {
                14220266465818359136 => {
                    iFrame = iFrame.wrapping_add(1);
                    nDbSize = (if isCommit != 0 && (*p).pDirty.is_null() {
                        nTruncate as ::core::ffi::c_uint
                    } else {
                        0 as ::core::ffi::c_uint
                    }) as ::core::ffi::c_int;
                    rc = walWriteOneFrame(
                        &raw mut w,
                        p,
                        nDbSize,
                        iOffset as sqlite3_int64,
                    );
                    if rc != 0 {
                        return rc;
                    }
                    pLast = p;
                    iOffset += szFrame as ::core::ffi::c_longlong;
                    (*p).flags = ((*p).flags as ::core::ffi::c_int | PGHDR_WAL_APPEND)
                        as u16_0;
                }
                _ => {}
            }
            p = (*p).pDirty;
        }
        if isCommit != 0 && (*pWal).iReCksum != 0 {
            rc = walRewriteChecksums(pWal, iFrame);
            if rc != 0 {
                return rc;
            }
        }
        if isCommit != 0
            && sync_flags & 0x3 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            let mut bSync: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            if (*pWal).padToSectorBoundary != 0 {
                let mut sectorSize: ::core::ffi::c_int = sqlite3SectorSize(
                    (*pWal).pWalFd,
                );
                w.iSyncPoint = ((iOffset as ::core::ffi::c_longlong
                    + sectorSize as ::core::ffi::c_longlong
                    - 1 as ::core::ffi::c_longlong)
                    / sectorSize as ::core::ffi::c_longlong
                    * sectorSize as ::core::ffi::c_longlong) as sqlite3_int64;
                bSync = (w.iSyncPoint == iOffset) as ::core::ffi::c_int;
                while iOffset < w.iSyncPoint {
                    rc = walWriteOneFrame(
                        &raw mut w,
                        pLast,
                        nTruncate as ::core::ffi::c_int,
                        iOffset as sqlite3_int64,
                    );
                    if rc != 0 {
                        return rc;
                    }
                    iOffset += szFrame as ::core::ffi::c_longlong;
                    nExtra += 1;
                }
            }
            if bSync != 0 {
                rc = sqlite3OsSync(w.pFd, sync_flags & 0x3 as ::core::ffi::c_int);
            }
        }
        if isCommit != 0 && (*pWal).truncateOnCommit as ::core::ffi::c_int != 0
            && (*pWal).mxWalSize >= 0 as ::core::ffi::c_longlong
        {
            let mut sz: i64_0 = (*pWal).mxWalSize;
            if WAL_HDRSIZE as i64_0
                + (iFrame as ::core::ffi::c_uint)
                    .wrapping_add(nExtra as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_sub(1 as ::core::ffi::c_uint) as i64_0
                    * (szPage + WAL_FRAME_HDRSIZE) as i64_0 > (*pWal).mxWalSize
            {
                sz = WAL_HDRSIZE as i64_0
                    + (iFrame as ::core::ffi::c_uint)
                        .wrapping_add(nExtra as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_sub(1 as ::core::ffi::c_uint) as i64_0
                        * (szPage + WAL_FRAME_HDRSIZE) as i64_0;
            }
            walLimitSize(pWal, sz);
            (*pWal).truncateOnCommit = 0 as u8_0;
        }
        iFrame = (*pWal).hdr.mxFrame;
        p = pList;
        while !p.is_null() && rc == SQLITE_OK {
            if !((*p).flags as ::core::ffi::c_int & PGHDR_WAL_APPEND
                == 0 as ::core::ffi::c_int)
            {
                iFrame = iFrame.wrapping_add(1);
                rc = walIndexAppend(pWal, iFrame, (*p).pgno as u32_0);
            }
            p = (*p).pDirty;
        }
        while rc == SQLITE_OK && nExtra > 0 as ::core::ffi::c_int {
            iFrame = iFrame.wrapping_add(1);
            nExtra -= 1;
            rc = walIndexAppend(pWal, iFrame, (*pLast).pgno as u32_0);
        }
        if rc == SQLITE_OK {
            (*pWal).hdr.szPage = (szPage & 0xff00 as ::core::ffi::c_int
                | szPage >> 16 as ::core::ffi::c_int) as u16_0;
            (*pWal).hdr.mxFrame = iFrame;
            if isCommit != 0 {
                (*pWal).hdr.iChange = (*pWal).hdr.iChange.wrapping_add(1);
                (*pWal).hdr.nPage = nTruncate as u32_0;
            }
            if isCommit != 0 {
                walIndexWriteHdr(pWal);
                (*pWal).iCallback = iFrame;
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalFrames(
    mut pWal: *mut Wal,
    mut szPage: ::core::ffi::c_int,
    mut pList: *mut PgHdr,
    mut nTruncate: Pgno,
    mut isCommit: ::core::ffi::c_int,
    mut sync_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = walFrames(pWal, szPage, pList, nTruncate, isCommit, sync_flags);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalCheckpoint(
    mut pWal: *mut Wal,
    mut db: *mut sqlite3,
    mut eMode: ::core::ffi::c_int,
    mut xBusy: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    mut pBusyArg: *mut ::core::ffi::c_void,
    mut sync_flags: ::core::ffi::c_int,
    mut nBuf: ::core::ffi::c_int,
    mut zBuf: *mut u8_0,
    mut pnLog: *mut ::core::ffi::c_int,
    mut pnCkpt: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut isChanged: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut eMode2: ::core::ffi::c_int = eMode;
        let mut xBusy2: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int
        > = xBusy;
        if (*pWal).readOnly != 0 {
            return SQLITE_READONLY;
        }
        xBusy2.is_some();
        if eMode != SQLITE_CHECKPOINT_NOOP {
            rc = walLockExclusive(pWal, WAL_CKPT_LOCK, 1 as ::core::ffi::c_int);
            if rc == SQLITE_OK {
                (*pWal).ckptLock = 1 as u8_0;
                if eMode != SQLITE_CHECKPOINT_PASSIVE {
                    rc = walBusyLock(
                        pWal,
                        xBusy2,
                        pBusyArg,
                        WAL_WRITE_LOCK,
                        1 as ::core::ffi::c_int,
                    );
                    if rc == SQLITE_OK {
                        (*pWal).writeLock = 1 as u8_0;
                    } else if rc == SQLITE_BUSY {
                        eMode2 = SQLITE_CHECKPOINT_PASSIVE;
                        xBusy2 = None;
                        rc = SQLITE_OK;
                    }
                }
            }
        } else {
            rc = SQLITE_OK;
        }
        if rc == SQLITE_OK {
            rc = walIndexReadHdr(pWal, &raw mut isChanged);
            eMode2 > SQLITE_CHECKPOINT_PASSIVE;
            if isChanged != 0
                && (*(*(*pWal).pDbFd).pMethods).iVersion >= 3 as ::core::ffi::c_int
            {
                sqlite3OsUnfetch(
                    (*pWal).pDbFd,
                    0 as i64_0,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
            }
        }
        if rc == SQLITE_OK {
            if (*pWal).hdr.mxFrame != 0 && walPagesize(pWal) != nBuf {
                rc = sqlite3CorruptError(4369 as ::core::ffi::c_int);
            } else if eMode2 != SQLITE_CHECKPOINT_NOOP {
                rc = walCheckpoint(pWal, db, eMode2, xBusy2, pBusyArg, sync_flags, zBuf);
            }
            if rc == SQLITE_OK || rc == SQLITE_BUSY {
                if !pnLog.is_null() {
                    *pnLog = (*pWal).hdr.mxFrame as ::core::ffi::c_int;
                }
                if !pnCkpt.is_null() {
                    *pnCkpt = (*walCkptInfo(pWal)).nBackfill as ::core::ffi::c_int;
                }
            }
        }
        if isChanged != 0 {
            memset(
                &raw mut (*pWal).hdr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<WalIndexHdr>() as size_t,
            );
        }
        sqlite3WalEndWriteTransaction(pWal);
        if (*pWal).ckptLock != 0 {
            walUnlockExclusive(pWal, WAL_CKPT_LOCK, 1 as ::core::ffi::c_int);
            (*pWal).ckptLock = 0 as u8_0;
        }
        return if rc == SQLITE_OK && eMode != eMode2 { SQLITE_BUSY } else { rc };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalCallback(mut pWal: *mut Wal) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: u32_0 = 0 as u32_0;
        if !pWal.is_null() {
            ret = (*pWal).iCallback;
            (*pWal).iCallback = 0 as u32_0;
        }
        return ret as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalExclusiveMode(
    mut pWal: *mut Wal,
    mut op: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if op == 0 as ::core::ffi::c_int {
            if (*pWal).exclusiveMode as ::core::ffi::c_int != WAL_NORMAL_MODE {
                (*pWal).exclusiveMode = WAL_NORMAL_MODE as u8_0;
                if walLockShared(
                    pWal,
                    3 as ::core::ffi::c_int + (*pWal).readLock as ::core::ffi::c_int,
                ) != SQLITE_OK
                {
                    (*pWal).exclusiveMode = WAL_EXCLUSIVE_MODE as u8_0;
                }
                rc = ((*pWal).exclusiveMode as ::core::ffi::c_int == WAL_NORMAL_MODE)
                    as ::core::ffi::c_int;
            } else {
                rc = 0 as ::core::ffi::c_int;
            }
        } else if op > 0 as ::core::ffi::c_int {
            walUnlockShared(
                pWal,
                3 as ::core::ffi::c_int + (*pWal).readLock as ::core::ffi::c_int,
            );
            (*pWal).exclusiveMode = WAL_EXCLUSIVE_MODE as u8_0;
            rc = 1 as ::core::ffi::c_int;
        } else {
            rc = ((*pWal).exclusiveMode as ::core::ffi::c_int == WAL_NORMAL_MODE)
                as ::core::ffi::c_int;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalHeapMemory(mut pWal: *mut Wal) -> ::core::ffi::c_int {
    unsafe {
        return (!pWal.is_null()
            && (*pWal).exclusiveMode as ::core::ffi::c_int == WAL_HEAPMEMORY_MODE)
            as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalFile(mut pWal: *mut Wal) -> *mut sqlite3_file {
    unsafe {
        return (*pWal).pWalFd;
    }
}
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_MAX_PAGE_SIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
