use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Bitvec;
    pub type sqlite3_mutex;
    pub type Pager;
    pub type PCache;
    pub type VtabCtx;
    pub type PreUpdate;
    pub type RenameToken;
    pub type Vdbe;
    pub type TableLock;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
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
    fn sqlite3OsWrite(
        _: *mut sqlite3_file,
        _: *const ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsTruncate(_: *mut sqlite3_file, size: i64_0) -> ::core::ffi::c_int;
    fn sqlite3OsFileSize(_: *mut sqlite3_file, pSize: *mut i64_0) -> ::core::ffi::c_int;
    fn sqlite3OsFileControl(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerGetJournalMode(_: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerBackupPtr(_: *mut Pager) -> *mut *mut sqlite3_backup;
    fn sqlite3PagerGet(
        pPager: *mut Pager,
        pgno: Pgno,
        ppPage: *mut *mut DbPage,
        clrFlag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerUnref(_: *mut DbPage);
    fn sqlite3PagerWrite(_: *mut DbPage) -> ::core::ffi::c_int;
    fn sqlite3PagerGetData(_: *mut DbPage) -> *mut ::core::ffi::c_void;
    fn sqlite3PagerGetExtra(_: *mut DbPage) -> *mut ::core::ffi::c_void;
    fn sqlite3PagerPagecount(_: *mut Pager, _: *mut ::core::ffi::c_int);
    fn sqlite3PagerCommitPhaseOne(
        _: *mut Pager,
        zSuper: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerSync(
        pPager: *mut Pager,
        zSuper: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerFile(_: *mut Pager) -> *mut sqlite3_file;
    fn sqlite3PagerIsMemdb(_: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerClearCache(_: *mut Pager);
    fn sqlite3PagerTruncateImage(_: *mut Pager, _: Pgno);
    fn sqlite3BtreeSetPageSize(
        p: *mut Btree,
        nPagesize: ::core::ffi::c_int,
        nReserve: ::core::ffi::c_int,
        eFix: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeGetPageSize(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeLastPage(_: *mut Btree) -> Pgno;
    fn sqlite3BtreeBeginTrans(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeCommitPhaseOne(
        _: *mut Btree,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeCommitPhaseTwo(
        _: *mut Btree,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeRollback(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeTxnState(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeUpdateMeta(
        _: *mut Btree,
        idx: ::core::ffi::c_int,
        value: u32_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeNewDb(p: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeSetVersion(
        pBt: *mut Btree,
        iVersion: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeEnter(_: *mut Btree);
    fn sqlite3BtreeLeave(_: *mut Btree);
    fn sqlite3MallocZero(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3ResetAllSchemasOfConnection(_: *mut sqlite3);
    fn sqlite3LeaveMutexAndCloseZombie(_: *mut sqlite3);
    fn sqlite3ErrorWithMsg(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3Error(_: *mut sqlite3, _: ::core::ffi::c_int);
    static mut sqlite3PendingByte: ::core::ffi::c_int;
    fn sqlite3FindDbName(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3OpenTempDatabase(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3ParseObjectInit(_: *mut Parse, _: *mut sqlite3);
    fn sqlite3ParseObjectReset(_: *mut Parse);
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
pub struct Btree {
    pub db: *mut sqlite3,
    pub pBt: *mut BtShared,
    pub inTrans: u8_0,
    pub sharable: u8_0,
    pub locked: u8_0,
    pub hasIncrblobCur: u8_0,
    pub wantToLock: ::core::ffi::c_int,
    pub nBackup: ::core::ffi::c_int,
    pub iBDataVersion: u32_0,
    pub pNext: *mut Btree,
    pub pPrev: *mut Btree,
    pub lock: BtLock

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BtLock {
    pub pBtree: *mut Btree,
    pub iTable: Pgno,
    pub eLock: u8_0,
    pub pNext: *mut BtLock

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BtShared {
    pub pPager: *mut Pager,
    pub db: *mut sqlite3,
    pub pCursor: *mut BtCursor,
    pub pPage1: *mut MemPage,
    pub openFlags: u8_0,
    pub autoVacuum: u8_0,
    pub incrVacuum: u8_0,
    pub bDoTruncate: u8_0,
    pub inTransaction: u8_0,
    pub max1bytePayload: u8_0,
    pub nReserveWanted: u8_0,
    pub btsFlags: u16_0,
    pub maxLocal: u16_0,
    pub minLocal: u16_0,
    pub maxLeaf: u16_0,
    pub minLeaf: u16_0,
    pub pageSize: u32_0,
    pub usableSize: u32_0,
    pub nTransaction: ::core::ffi::c_int,
    pub nPage: u32_0,
    pub pSchema: *mut ::core::ffi::c_void,
    pub xFreeSchema: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub mutex: *mut sqlite3_mutex,
    pub pHasContent: *mut Bitvec,
    pub nRef: ::core::ffi::c_int,
    pub pNext: *mut BtShared,
    pub pLock: *mut BtLock,
    pub pWriter: *mut Btree,
    pub pTmpSpace: *mut u8_0,
    pub nPreformatSize: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemPage {
    pub isInit: u8_0,
    pub intKey: u8_0,
    pub intKeyLeaf: u8_0,
    pub pgno: Pgno,
    pub leaf: u8_0,
    pub hdrOffset: u8_0,
    pub childPtrSize: u8_0,
    pub max1bytePayload: u8_0,
    pub nOverflow: u8_0,
    pub maxLocal: u16_0,
    pub minLocal: u16_0,
    pub cellOffset: u16_0,
    pub nFree: ::core::ffi::c_int,
    pub nCell: u16_0,
    pub maskPage: u16_0,
    pub aiOvfl: [u16_0; 4],
    pub apOvfl: [*mut u8_0; 4],
    pub pBt: *mut BtShared,
    pub aData: *mut u8_0,
    pub aDataEnd: *mut u8_0,
    pub aCellIdx: *mut u8_0,
    pub aDataOfst: *mut u8_0,
    pub pDbPage: *mut DbPage,
    pub xCellSize: Option<unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0>,
    pub xParseCell: Option<
        unsafe extern "C" fn(*mut MemPage, *mut u8_0, *mut CellInfo) -> ()
    >

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CellInfo {
    pub nKey: i64_0,
    pub pPayload: *mut u8_0,
    pub nPayload: u32_0,
    pub nLocal: u16_0,
    pub nSize: u16_0

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BtCursor {
    pub eState: u8_0,
    pub curFlags: u8_0,
    pub curPagerFlags: u8_0,
    pub hints: u8_0,
    pub skipNext: ::core::ffi::c_int,
    pub pBtree: *mut Btree,
    pub aOverflow: *mut Pgno,
    pub pKey: *mut ::core::ffi::c_void,
    pub pBt: *mut BtShared,
    pub pNext: *mut BtCursor,
    pub info: CellInfo,
    pub nKey: i64_0,
    pub pgnoRoot: Pgno,
    pub iPage: i8_0,
    pub curIntKey: u8_0,
    pub ix: u16_0,
    pub aiIdx: [u16_0; 19],
    pub pKeyInfo: *mut KeyInfo,
    pub pPage: *mut MemPage,
    pub apPage: [*mut MemPage; 19]

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
pub type i8_0 = int8_t;
pub type int8_t = __int8_t;
pub type __int8_t = i8;
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
pub struct sqlite3_backup {
    pub pDestDb: *mut sqlite3,
    pub pDest: *mut Btree,
    pub iDestSchema: u32_0,
    pub bDestLocked: ::core::ffi::c_int,
    pub iNext: Pgno,
    pub pSrcDb: *mut sqlite3,
    pub pSrc: *mut Btree,
    pub rc: ::core::ffi::c_int,
    pub nRemaining: Pgno,
    pub nPagecount: Pgno,
    pub isAttached: ::core::ffi::c_int,
    pub pNext: *mut sqlite3_backup

}
pub type size_t = usize;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_READONLY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_OVERWRITE: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_TXN_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_WAL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PAGER_GET_READONLY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const TRANS_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BTS_PAGESIZE_FIXED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
unsafe extern "C" fn findBtree(
    mut pErrorDb: *mut sqlite3,
    mut pDb: *mut sqlite3,
    mut zDb: *const ::core::ffi::c_char,
) -> *mut Btree {
    unsafe {
        let mut i: ::core::ffi::c_int = sqlite3FindDbName(pDb, zDb);
        if i == 1 as ::core::ffi::c_int {
            let mut sParse: Parse = Parse {
                db: ::core::ptr::null_mut::<sqlite3>(),
                zErrMsg: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                pVdbe: ::core::ptr::null_mut::<Vdbe>(),
                rc: 0,
                nQueryLoop: 0,
                nested: 0,
                nTempReg: 0,
                isMultiWrite: 0,
                mayAbort: 0,
                hasCompound: 0,
                disableLookaside: 0,
                prepFlags: 0,
                withinRJSubrtn: 0,
                bHasExists: 0,
                mSubrtnSig: 0,
                eTriggerOp: 0,
                bReturning: 0,
                eOrconf: 0,
                disableTriggers: 0,
                colNamesSet_bHasWith_okConstFactor_checkSchema: [0; 1],
                c2rust_padding: [0; 3],
                nRangeReg: 0,
                iRangeReg: 0,
                nErr: 0,
                nTab: 0,
                nMem: 0,
                szOpAlloc: 0,
                iSelfTab: 0,
                nLabel: 0,
                nLabelAlloc: 0,
                aLabel: ::core::ptr::null_mut::<::core::ffi::c_int>(),
                pConstExpr: ::core::ptr::null_mut::<ExprList>(),
                pIdxEpr: ::core::ptr::null_mut::<IndexedExpr>(),
                pIdxPartExpr: ::core::ptr::null_mut::<IndexedExpr>(),
                writeMask: 0,
                cookieMask: 0,
                nMaxArg: 0,
                nSelect: 0,
                nProgressSteps: 0,
                nTableLock: 0,
                aTableLock: ::core::ptr::null_mut::<TableLock>(),
                pAinc: ::core::ptr::null_mut::<AutoincInfo>(),
                pToplevel: ::core::ptr::null_mut::<Parse>(),
                pTriggerTab: ::core::ptr::null_mut::<Table>(),
                pTriggerPrg: ::core::ptr::null_mut::<TriggerPrg>(),
                pCleanup: ::core::ptr::null_mut::<ParseCleanup>(),
                aTempReg: [0; 8],
                pOuterParse: ::core::ptr::null_mut::<Parse>(),
                sNameToken: Token {
                    z: ::core::ptr::null::<::core::ffi::c_char>(),
                    n: 0

                },
                oldmask: 0,
                newmask: 0,
                u1: C2Rust_Unnamed_18 {
                    cr: C2Rust_Unnamed_20 {
                        addrCrTab: 0,
                        regRowid: 0,
                        regRoot: 0,
                        constraintName: Token {
                            z: ::core::ptr::null::<::core::ffi::c_char>(),
                            n: 0

                        }

                    }

                },
                sLastToken: Token {
                    z: ::core::ptr::null::<::core::ffi::c_char>(),
                    n: 0

                },
                nVar: 0,
                iPkSortOrder: 0,
                explain: 0,
                eParseMode: 0,
                nVtabLock: 0,
                nHeight: 0,
                addrExplain: 0,
                pVList: ::core::ptr::null_mut::<VList>(),
                pReprepare: ::core::ptr::null_mut::<Vdbe>(),
                zTail: ::core::ptr::null::<::core::ffi::c_char>(),
                pNewTable: ::core::ptr::null_mut::<Table>(),
                pNewIndex: ::core::ptr::null_mut::<Index>(),
                pNewTrigger: ::core::ptr::null_mut::<Trigger>(),
                zAuthContext: ::core::ptr::null::<::core::ffi::c_char>(),
                sArg: Token {
                    z: ::core::ptr::null::<::core::ffi::c_char>(),
                    n: 0

                },
                apVtabLock: ::core::ptr::null_mut::<*mut Table>(),
                pWith: ::core::ptr::null_mut::<With>(),
                pRename: ::core::ptr::null_mut::<RenameToken>()

            };
            let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            sqlite3ParseObjectInit(&raw mut sParse, pDb);
            if sqlite3OpenTempDatabase(&raw mut sParse) != 0 {
                sqlite3ErrorWithMsg(
                    pErrorDb,
                    sParse.rc,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    sParse.zErrMsg,
                );
                rc = SQLITE_ERROR;
            }
            sqlite3DbFree(pErrorDb, sParse.zErrMsg as *mut ::core::ffi::c_void);
            sqlite3ParseObjectReset(&raw mut sParse);
            if rc != 0 {
                return ::core::ptr::null_mut::<Btree>();
            }
        }
        if i < 0 as ::core::ffi::c_int {
            sqlite3ErrorWithMsg(
                pErrorDb,
                SQLITE_ERROR,
                b"unknown database %s\0".as_ptr() as *const ::core::ffi::c_char,
                zDb,
            );
            return ::core::ptr::null_mut::<Btree>();
        }
        return (*(*pDb).aDb.offset(i as isize)).pBt;
    }
}
unsafe extern "C" fn setDestPgsz(mut p: *mut sqlite3_backup) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3BtreeSetPageSize(
            (*p).pDest,
            sqlite3BtreeGetPageSize((*p).pSrc),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn checkReadTransaction(
    mut db: *mut sqlite3,
    mut p: *mut Btree,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3BtreeTxnState(p) != SQLITE_TXN_NONE {
            sqlite3ErrorWithMsg(
                db,
                SQLITE_ERROR,
                b"destination database is in use\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return SQLITE_ERROR;
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_backup_init(
    mut pDestDb: *mut sqlite3,
    mut zDestDb: *const ::core::ffi::c_char,
    mut pSrcDb: *mut sqlite3,
    mut zSrcDb: *const ::core::ffi::c_char,
) -> *mut sqlite3_backup {
    unsafe {
        let mut p: *mut sqlite3_backup = ::core::ptr::null_mut::<sqlite3_backup>();
        sqlite3_mutex_enter((*pSrcDb).mutex);
        sqlite3_mutex_enter((*pDestDb).mutex);
        if pSrcDb == pDestDb {
            sqlite3ErrorWithMsg(
                pDestDb,
                SQLITE_ERROR,
                b"source and destination must be distinct\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            p = ::core::ptr::null_mut::<sqlite3_backup>();
        } else {
            p = sqlite3MallocZero(::core::mem::size_of::<sqlite3_backup>() as u64_0)
                as *mut sqlite3_backup;
            if p.is_null() {
                sqlite3Error(pDestDb, SQLITE_NOMEM_BKPT);
            }
        }
        if !p.is_null() {
            (*p).pSrc = findBtree(pDestDb, pSrcDb, zSrcDb);
            (*p).pDest = findBtree(pDestDb, pDestDb, zDestDb);
            (*p).pDestDb = pDestDb;
            (*p).pSrcDb = pSrcDb;
            (*p).iNext = 1 as Pgno;
            (*p).isAttached = 0 as ::core::ffi::c_int;
            if (*p).pSrc.is_null() || (*p).pDest.is_null()
                || checkReadTransaction(pDestDb, (*p).pDest) != SQLITE_OK
            {
                sqlite3_free(p as *mut ::core::ffi::c_void);
                p = ::core::ptr::null_mut::<sqlite3_backup>();
            }
        }
        if !p.is_null() {
            (*(*p).pSrc).nBackup += 1;
        }
        sqlite3_mutex_leave((*pDestDb).mutex);
        sqlite3_mutex_leave((*pSrcDb).mutex);
        return p;
    }
}
unsafe extern "C" fn isFatalError(mut rc: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return (rc != SQLITE_OK && rc != SQLITE_BUSY && rc != 6 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn backupOnePage(
    mut p: *mut sqlite3_backup,
    mut iSrcPg: Pgno,
    mut zSrcData: *const u8_0,
    mut bUpdate: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let pDestPager: *mut Pager = sqlite3BtreePager((*p).pDest) as *mut Pager;
        let nSrcPgsz: ::core::ffi::c_int = sqlite3BtreeGetPageSize((*p).pSrc)
            as ::core::ffi::c_int;
        let mut nDestPgsz: ::core::ffi::c_int = sqlite3BtreeGetPageSize((*p).pDest);
        let nCopy: ::core::ffi::c_int = if nSrcPgsz < nDestPgsz {
            nSrcPgsz
        } else {
            nDestPgsz
        };
        let iEnd: i64_0 = iSrcPg as i64_0 * nSrcPgsz as i64_0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut iOff: i64_0 = 0;
        iOff = iEnd - nSrcPgsz as i64_0;
        while rc == SQLITE_OK && iOff < iEnd {
            let mut pDestPg: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
            let mut iDest: Pgno = ((iOff as ::core::ffi::c_longlong
                / nDestPgsz as ::core::ffi::c_longlong) as Pgno)
                .wrapping_add(1 as Pgno);
            if !(iDest
                == (sqlite3PendingByte as u32_0)
                    .wrapping_div((*(*(*p).pDest).pBt).pageSize)
                    .wrapping_add(1 as u32_0))
            {
                rc = sqlite3PagerGet(
                    pDestPager,
                    iDest,
                    &raw mut pDestPg,
                    0 as ::core::ffi::c_int,
                );
                if SQLITE_OK == rc
                    && {
                        rc = sqlite3PagerWrite(pDestPg);
                        SQLITE_OK == rc
                    }
                {
                    let mut zIn: *const u8_0 = zSrcData
                        .offset(
                            (iOff as ::core::ffi::c_longlong
                                % nSrcPgsz as ::core::ffi::c_longlong) as isize,
                        ) as *const u8_0;
                    let mut zDestData: *mut u8_0 = sqlite3PagerGetData(pDestPg)
                        as *mut u8_0;
                    let mut zOut: *mut u8_0 = zDestData
                        .offset(
                            (iOff as ::core::ffi::c_longlong
                                % nDestPgsz as ::core::ffi::c_longlong) as isize,
                        ) as *mut u8_0;
                    memcpy(
                        zOut as *mut ::core::ffi::c_void,
                        zIn as *const ::core::ffi::c_void,
                        nCopy as size_t,
                    );
                    *(sqlite3PagerGetExtra(pDestPg) as *mut u8_0)
                        .offset(0 as ::core::ffi::c_int as isize) = 0 as u8_0;
                    if iOff == 0 as ::core::ffi::c_longlong
                        && bUpdate == 0 as ::core::ffi::c_int
                    {
                        sqlite3Put4byte(
                            zOut.offset(28 as ::core::ffi::c_int as isize) as *mut u8_0,
                            sqlite3BtreeLastPage((*p).pSrc) as u32_0,
                        );
                    }
                }
                sqlite3PagerUnref(pDestPg);
            }
            iOff += nDestPgsz as ::core::ffi::c_longlong;
        }
        return rc;
    }
}
unsafe extern "C" fn backupTruncateFile(
    mut pFile: *mut sqlite3_file,
    mut iSize: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iCurrent: i64_0 = 0;
        let mut rc: ::core::ffi::c_int = sqlite3OsFileSize(pFile, &raw mut iCurrent);
        if rc == SQLITE_OK && iCurrent > iSize {
            rc = sqlite3OsTruncate(pFile, iSize);
        }
        return rc;
    }
}
unsafe extern "C" fn attachBackupObject(mut p: *mut sqlite3_backup) {
    unsafe {
        let mut pp: *mut *mut sqlite3_backup = ::core::ptr::null_mut::<
            *mut sqlite3_backup,
        >();
        pp = sqlite3PagerBackupPtr(sqlite3BtreePager((*p).pSrc) as *mut Pager);
        (*p).pNext = *pp;
        *pp = p;
        (*p).isAttached = 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_backup_step(
    mut p: *mut sqlite3_backup,
    mut nPage: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut destMode: ::core::ffi::c_int = 0;
        let mut pgszSrc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pgszDest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        sqlite3_mutex_enter((*(*p).pSrcDb).mutex);
        sqlite3BtreeEnter((*p).pSrc);
        if !(*p).pDestDb.is_null() {
            sqlite3_mutex_enter((*(*p).pDestDb).mutex);
        }
        rc = (*p).rc;
        if isFatalError(rc) == 0 {
            let pSrcPager: *mut Pager = sqlite3BtreePager((*p).pSrc) as *mut Pager;
            let pDestPager: *mut Pager = sqlite3BtreePager((*p).pDest) as *mut Pager;
            let mut ii: ::core::ffi::c_int = 0;
            let mut nSrcPage: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
            let mut bCloseTrans: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if !(*p).pDestDb.is_null()
                && (*(*(*p).pSrc).pBt).inTransaction as ::core::ffi::c_int == TRANS_WRITE
            {
                rc = SQLITE_BUSY;
            } else {
                rc = SQLITE_OK;
            }
            if rc == SQLITE_OK && SQLITE_TXN_NONE == sqlite3BtreeTxnState((*p).pSrc) {
                rc = sqlite3BtreeBeginTrans(
                    (*p).pSrc,
                    0 as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                bCloseTrans = 1 as ::core::ffi::c_int;
            }
            if (*p).bDestLocked == 0 as ::core::ffi::c_int && rc == SQLITE_OK
                && setDestPgsz(p) == SQLITE_NOMEM
            {
                rc = SQLITE_NOMEM;
            }
            if SQLITE_OK == rc && (*p).bDestLocked == 0 as ::core::ffi::c_int
                && {
                    rc = sqlite3BtreeBeginTrans(
                        (*p).pDest,
                        2 as ::core::ffi::c_int,
                        &raw mut (*p).iDestSchema as *mut ::core::ffi::c_int,
                    );
                    SQLITE_OK == rc
                }
            {
                (*p).bDestLocked = 1 as ::core::ffi::c_int;
            }
            pgszSrc = sqlite3BtreeGetPageSize((*p).pSrc);
            pgszDest = sqlite3BtreeGetPageSize((*p).pDest);
            destMode = sqlite3PagerGetJournalMode(
                sqlite3BtreePager((*p).pDest) as *mut Pager,
            );
            if SQLITE_OK == rc
                && (destMode == PAGER_JOURNALMODE_WAL
                    || sqlite3PagerIsMemdb(pDestPager) != 0) && pgszSrc != pgszDest
            {
                rc = SQLITE_READONLY;
            }
            nSrcPage = sqlite3BtreeLastPage((*p).pSrc) as ::core::ffi::c_int;
            ii = 0 as ::core::ffi::c_int;
            while (nPage < 0 as ::core::ffi::c_int || ii < nPage)
                && (*p).iNext <= nSrcPage as Pgno && rc == 0
            {
                let iSrcPg: Pgno = (*p).iNext;
                if iSrcPg
                    != (sqlite3PendingByte as u32_0)
                        .wrapping_div((*(*(*p).pSrc).pBt).pageSize)
                        .wrapping_add(1 as u32_0)
                {
                    let mut pSrcPg: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
                    rc = sqlite3PagerGet(
                        pSrcPager,
                        iSrcPg,
                        &raw mut pSrcPg,
                        PAGER_GET_READONLY,
                    );
                    if rc == SQLITE_OK {
                        rc = backupOnePage(
                            p,
                            iSrcPg,
                            sqlite3PagerGetData(pSrcPg) as *const u8_0,
                            0 as ::core::ffi::c_int,
                        );
                        sqlite3PagerUnref(pSrcPg);
                    }
                }
                (*p).iNext = (*p).iNext.wrapping_add(1);
                ii += 1;
            }
            if rc == SQLITE_OK {
                (*p).nPagecount = nSrcPage as Pgno;
                (*p).nRemaining = ((nSrcPage + 1 as ::core::ffi::c_int) as Pgno)
                    .wrapping_sub((*p).iNext);
                if (*p).iNext > nSrcPage as Pgno {
                    rc = SQLITE_DONE;
                } else if (*p).isAttached == 0 {
                    attachBackupObject(p);
                }
            }
            if rc == SQLITE_DONE {
                if nSrcPage == 0 as ::core::ffi::c_int {
                    rc = sqlite3BtreeNewDb((*p).pDest);
                    nSrcPage = 1 as ::core::ffi::c_int;
                }
                if rc == SQLITE_OK || rc == SQLITE_DONE {
                    rc = sqlite3BtreeUpdateMeta(
                        (*p).pDest,
                        1 as ::core::ffi::c_int,
                        (*p).iDestSchema.wrapping_add(1 as u32_0),
                    );
                }
                if rc == SQLITE_OK {
                    if !(*p).pDestDb.is_null() {
                        sqlite3ResetAllSchemasOfConnection((*p).pDestDb);
                    }
                    if destMode == PAGER_JOURNALMODE_WAL {
                        rc = sqlite3BtreeSetVersion((*p).pDest, 2 as ::core::ffi::c_int);
                    }
                }
                if rc == SQLITE_OK {
                    let mut nDestTruncate: ::core::ffi::c_int = 0;
                    if pgszSrc < pgszDest {
                        let mut ratio: ::core::ffi::c_int = pgszDest / pgszSrc;
                        nDestTruncate = (nSrcPage + ratio - 1 as ::core::ffi::c_int)
                            / ratio;
                        if nDestTruncate
                            == (sqlite3PendingByte as u32_0)
                                .wrapping_div((*(*(*p).pDest).pBt).pageSize)
                                .wrapping_add(1 as u32_0) as ::core::ffi::c_int
                        {
                            nDestTruncate -= 1;
                        }
                    } else {
                        nDestTruncate = nSrcPage * (pgszSrc / pgszDest);
                    }
                    if pgszSrc < pgszDest {
                        let iSize: i64_0 = pgszSrc as i64_0 * nSrcPage as i64_0;
                        let pFile: *mut sqlite3_file = sqlite3PagerFile(pDestPager)
                            as *mut sqlite3_file;
                        let mut iPg: Pgno = 0;
                        let mut nDstPage: ::core::ffi::c_int = 0;
                        let mut iOff: i64_0 = 0;
                        let mut iEnd: i64_0 = 0;
                        sqlite3PagerPagecount(pDestPager, &raw mut nDstPage);
                        iPg = nDestTruncate as Pgno;
                        while rc == SQLITE_OK && iPg <= nDstPage as Pgno {
                            if iPg
                                != (sqlite3PendingByte as u32_0)
                                    .wrapping_div((*(*(*p).pDest).pBt).pageSize)
                                    .wrapping_add(1 as u32_0)
                            {
                                let mut pPg: *mut DbPage = ::core::ptr::null_mut::<
                                    DbPage,
                                >();
                                rc = sqlite3PagerGet(
                                    pDestPager,
                                    iPg,
                                    &raw mut pPg,
                                    0 as ::core::ffi::c_int,
                                );
                                if rc == SQLITE_OK {
                                    rc = sqlite3PagerWrite(pPg);
                                    sqlite3PagerUnref(pPg);
                                }
                            }
                            iPg = iPg.wrapping_add(1);
                        }
                        if rc == SQLITE_OK {
                            rc = sqlite3PagerCommitPhaseOne(
                                pDestPager,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                1 as ::core::ffi::c_int,
                            );
                        }
                        iEnd = (if ((sqlite3PendingByte + pgszDest)
                            as ::core::ffi::c_longlong) < iSize
                        {
                            (sqlite3PendingByte + pgszDest) as ::core::ffi::c_longlong
                        } else {
                            iSize as ::core::ffi::c_longlong
                        }) as i64_0;
                        iOff = (sqlite3PendingByte + pgszSrc) as i64_0;
                        while rc == SQLITE_OK && iOff < iEnd {
                            let mut pSrcPg_0: *mut PgHdr = ::core::ptr::null_mut::<
                                PgHdr,
                            >();
                            let iSrcPg_0: Pgno = (iOff as ::core::ffi::c_longlong
                                / pgszSrc as ::core::ffi::c_longlong
                                + 1 as ::core::ffi::c_longlong) as Pgno;
                            rc = sqlite3PagerGet(
                                pSrcPager,
                                iSrcPg_0,
                                &raw mut pSrcPg_0,
                                0 as ::core::ffi::c_int,
                            );
                            if rc == SQLITE_OK {
                                let mut zData: *mut u8_0 = sqlite3PagerGetData(
                                    pSrcPg_0 as *mut DbPage,
                                ) as *mut u8_0;
                                rc = sqlite3OsWrite(
                                    pFile,
                                    zData as *const ::core::ffi::c_void,
                                    pgszSrc,
                                    iOff,
                                );
                            }
                            sqlite3PagerUnref(pSrcPg_0 as *mut DbPage);
                            iOff += pgszSrc as ::core::ffi::c_longlong;
                        }
                        if rc == SQLITE_OK {
                            rc = backupTruncateFile(pFile, iSize);
                        }
                        if rc == SQLITE_OK {
                            rc = sqlite3PagerSync(
                                pDestPager,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                            );
                        }
                    } else {
                        sqlite3PagerTruncateImage(pDestPager, nDestTruncate as Pgno);
                        rc = sqlite3PagerCommitPhaseOne(
                            pDestPager,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            0 as ::core::ffi::c_int,
                        );
                    }
                    if SQLITE_OK == rc
                        && {
                            rc = sqlite3BtreeCommitPhaseTwo(
                                (*p).pDest,
                                0 as ::core::ffi::c_int,
                            );
                            SQLITE_OK == rc
                        }
                    {
                        rc = SQLITE_DONE;
                    }
                }
            }
            if bCloseTrans != 0 {
                sqlite3BtreeCommitPhaseOne(
                    (*p).pSrc,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                sqlite3BtreeCommitPhaseTwo((*p).pSrc, 0 as ::core::ffi::c_int);
            }
            if rc == SQLITE_IOERR_NOMEM {
                rc = SQLITE_NOMEM_BKPT;
            }
            (*p).rc = rc;
        }
        if !(*p).pDestDb.is_null() {
            sqlite3_mutex_leave((*(*p).pDestDb).mutex);
        }
        sqlite3BtreeLeave((*p).pSrc);
        sqlite3_mutex_leave((*(*p).pSrcDb).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_backup_finish(
    mut p: *mut sqlite3_backup,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pp: *mut *mut sqlite3_backup = ::core::ptr::null_mut::<
            *mut sqlite3_backup,
        >();
        let mut pSrcDb: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if p.is_null() {
            return SQLITE_OK;
        }
        pSrcDb = (*p).pSrcDb;
        sqlite3_mutex_enter((*pSrcDb).mutex);
        sqlite3BtreeEnter((*p).pSrc);
        if !(*p).pDestDb.is_null() {
            sqlite3_mutex_enter((*(*p).pDestDb).mutex);
        }
        if !(*p).pDestDb.is_null() {
            (*(*p).pSrc).nBackup -= 1;
        }
        if (*p).isAttached != 0 {
            pp = sqlite3PagerBackupPtr(sqlite3BtreePager((*p).pSrc) as *mut Pager);
            while *pp != p {
                pp = &raw mut (**pp).pNext;
            }
            *pp = (*p).pNext;
        }
        sqlite3BtreeRollback((*p).pDest, SQLITE_OK, 0 as ::core::ffi::c_int);
        rc = if (*p).rc == SQLITE_DONE { SQLITE_OK } else { (*p).rc };
        if !(*p).pDestDb.is_null() {
            sqlite3Error((*p).pDestDb, rc);
            sqlite3LeaveMutexAndCloseZombie((*p).pDestDb);
        }
        sqlite3BtreeLeave((*p).pSrc);
        if !(*p).pDestDb.is_null() {
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
        sqlite3LeaveMutexAndCloseZombie(pSrcDb);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_backup_remaining(
    mut p: *mut sqlite3_backup,
) -> ::core::ffi::c_int {
    unsafe {
        return (*p).nRemaining as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_backup_pagecount(
    mut p: *mut sqlite3_backup,
) -> ::core::ffi::c_int {
    unsafe {
        return (*p).nPagecount as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn backupUpdate(
    mut p: *mut sqlite3_backup,
    mut iPage: Pgno,
    mut aData: *const u8_0,
) {
    unsafe {
        loop {
            if isFatalError((*p).rc) == 0 && iPage < (*p).iNext {
                let mut rc: ::core::ffi::c_int = 0;
                sqlite3_mutex_enter((*(*p).pDestDb).mutex);
                rc = backupOnePage(p, iPage, aData, 1 as ::core::ffi::c_int);
                sqlite3_mutex_leave((*(*p).pDestDb).mutex);
                if rc != SQLITE_OK {
                    (*p).rc = rc;
                }
            }
            p = (*p).pNext;
            if p.is_null() {
                break;
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BackupUpdate(
    mut pBackup: *mut sqlite3_backup,
    mut iPage: Pgno,
    mut aData: *const u8_0,
) {
    unsafe {
        if !pBackup.is_null() {
            backupUpdate(pBackup, iPage, aData);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BackupRestart(mut pBackup: *mut sqlite3_backup) {
    unsafe {
        let mut p: *mut sqlite3_backup = ::core::ptr::null_mut::<sqlite3_backup>();
        p = pBackup;
        while !p.is_null() {
            (*p).iNext = 1 as Pgno;
            p = (*p).pNext;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCopyFile(
    mut pTo: *mut Btree,
    mut pFrom: *mut Btree,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pFd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut b: sqlite3_backup = sqlite3_backup {
            pDestDb: ::core::ptr::null_mut::<sqlite3>(),
            pDest: ::core::ptr::null_mut::<Btree>(),
            iDestSchema: 0,
            bDestLocked: 0,
            iNext: 0,
            pSrcDb: ::core::ptr::null_mut::<sqlite3>(),
            pSrc: ::core::ptr::null_mut::<Btree>(),
            rc: 0,
            nRemaining: 0,
            nPagecount: 0,
            isAttached: 0,
            pNext: ::core::ptr::null_mut::<sqlite3_backup>()

        };
        sqlite3BtreeEnter(pTo);
        sqlite3BtreeEnter(pFrom);
        pFd = sqlite3PagerFile(sqlite3BtreePager(pTo) as *mut Pager);
        if !(*pFd).pMethods.is_null() {
            let mut nByte: i64_0 = sqlite3BtreeGetPageSize(pFrom) as i64_0
                * sqlite3BtreeLastPage(pFrom) as i64_0;
            rc = sqlite3OsFileControl(
                pFd,
                SQLITE_FCNTL_OVERWRITE,
                &raw mut nByte as *mut ::core::ffi::c_void,
            );
            if rc == SQLITE_NOTFOUND {
                rc = SQLITE_OK;
            }
            if rc != 0 {
                c2rust_current_block = 6906026730051105889;
            } else {
                c2rust_current_block = 17216689946888361452;
            }
        } else {
            c2rust_current_block = 17216689946888361452;
        }
        match c2rust_current_block {
            17216689946888361452 => {
                memset(
                    &raw mut b as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<sqlite3_backup>() as size_t,
                );
                b.pSrcDb = (*pFrom).db;
                b.pSrc = pFrom;
                b.pDest = pTo;
                b.iNext = 1 as Pgno;
                sqlite3_backup_step(&raw mut b, 0x7fffffff as ::core::ffi::c_int);
                rc = sqlite3_backup_finish(&raw mut b);
                if rc == SQLITE_OK {
                    (*(*pTo).pBt).btsFlags = ((*(*pTo).pBt).btsFlags
                        as ::core::ffi::c_int & !BTS_PAGESIZE_FIXED) as u16_0;
                } else {
                    sqlite3PagerClearCache(sqlite3BtreePager(b.pDest) as *mut Pager);
                }
            }
            _ => {}
        }
        sqlite3BtreeLeave(pFrom);
        sqlite3BtreeLeave(pTo);
        return rc;
    }
}
