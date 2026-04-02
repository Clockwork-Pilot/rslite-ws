use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type VdbeSorter;
    pub type BtCursor;
    pub type Btree;
    pub type RenameToken;
    pub type TableLock;
    pub type VtabCtx;
    pub type sqlite3_mutex;
    pub type Pager;
    pub type sqlite3_pcache;
    fn sqlite3_msize(_: *mut ::core::ffi::c_void) -> sqlite3_uint64;
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn sqlite3PagerMemUsed(_: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerCacheStat(
        _: *mut Pager,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut u64_0,
    );
    fn sqlite3BtreeGetPageSize(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeEnterAll(_: *mut sqlite3);
    fn sqlite3BtreeConnectionCount(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeLeaveAll(_: *mut sqlite3);
    fn sqlite3VdbeDelete(_: *mut Vdbe);
    fn sqlite3MisuseError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3Pcache1Mutex() -> *mut sqlite3_mutex;
    fn sqlite3MallocMutex() -> *mut sqlite3_mutex;
    fn sqlite3DeleteTable(_: *mut sqlite3, _: *mut Table);
    fn sqlite3DeleteTrigger(_: *mut sqlite3, _: *mut Trigger);
    static mut sqlite3Config: Sqlite3Config;
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
    pub trace: C2Rust_Unnamed_25,
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
    pub u1: C2Rust_Unnamed_22,
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
    pub u: C2Rust_Unnamed_18,
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
    pub fg: C2Rust_Unnamed_17,
    pub iCursor: ::core::ffi::c_int,
    pub colUsed: Bitmask,
    pub u1: C2Rust_Unnamed_16,
    pub u2: C2Rust_Unnamed_15,
    pub u3: C2Rust_Unnamed_14,
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
    pub u: C2Rust_Unnamed_13,
    pub pLeft: *mut Expr,
    pub pRight: *mut Expr,
    pub x: C2Rust_Unnamed_12,
    pub nHeight: ::core::ffi::c_int,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ynVar,
    pub iAgg: i16_0,
    pub w: C2Rust_Unnamed_11,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_value {
    pub u: MemValue,
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub flags: u16_0,
    pub enc: u8_0,
    pub eSubtype: u8_0,
    pub db: *mut sqlite3,
    pub szMalloc: ::core::ffi::c_int,
    pub uTemp: u32_0,
    pub zMalloc: *mut ::core::ffi::c_char,
    pub xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union MemValue {
    pub r: ::core::ffi::c_double,
    pub i: i64_0,
    pub nZero: ::core::ffi::c_int,
    pub zPType: *const ::core::ffi::c_char,
    pub pDef: *mut FuncDef

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_context {
    pub pOut: *mut Mem,
    pub pFunc: *mut FuncDef,
    pub pMem: *mut Mem,
    pub pVdbe: *mut Vdbe,
    pub iOp: ::core::ffi::c_int,
    pub isError: ::core::ffi::c_int,
    pub enc: u8_0,
    pub skipFlag: u8_0,
    pub argc: u16_0,
    pub argv: [*mut sqlite3_value; 0]

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Vdbe {
    pub db: *mut sqlite3,
    pub ppVPrev: *mut *mut Vdbe,
    pub pVNext: *mut Vdbe,
    pub pParse: *mut Parse,
    pub nVar: ynVar,
    pub nMem: ::core::ffi::c_int,
    pub nCursor: ::core::ffi::c_int,
    pub cacheCtr: u32_0,
    pub pc: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub iStatement: ::core::ffi::c_int,
    pub iCurrentTime: i64_0,
    pub nFkConstraint: i64_0,
    pub nStmtDefCons: i64_0,
    pub nStmtDefImmCons: i64_0,
    pub aMem: *mut Mem,
    pub apArg: *mut *mut Mem,
    pub apCsr: *mut *mut VdbeCursor,
    pub aVar: *mut Mem,
    pub aOp: *mut Op,
    pub nOp: ::core::ffi::c_int,
    pub nOpAlloc: ::core::ffi::c_int,
    pub aColName: *mut Mem,
    pub pResultRow: *mut Mem,
    pub zErrMsg: *mut ::core::ffi::c_char,
    pub pVList: *mut VList,
    pub startTime: i64_0,
    pub nResColumn: u16_0,
    pub nResAlloc: u16_0,
    pub errorAction: u8_0,
    pub minWriteFileFormat: u8_0,
    pub prepFlags: u8_0,
    pub eVdbeState: u8_0,
    #[bitfield(name = "expired", ty = "bft", bits = "0..=1")]
    #[bitfield(name = "explain", ty = "bft", bits = "2..=3")]
    #[bitfield(name = "changeCntOn", ty = "bft", bits = "4..=4")]
    #[bitfield(name = "usesStmtJournal", ty = "bft", bits = "5..=5")]
    #[bitfield(name = "readOnly", ty = "bft", bits = "6..=6")]
    #[bitfield(name = "bIsReader", ty = "bft", bits = "7..=7")]
    #[bitfield(name = "haveEqpOps", ty = "bft", bits = "8..=8")]
    pub expired_explain_changeCntOn_usesStmtJournal_readOnly_bIsReader_haveEqpOps: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub btreeMask: yDbMask,
    pub lockMask: yDbMask,
    pub aCounter: [u32_0; 9],
    pub zSql: *mut ::core::ffi::c_char,
    pub pFree: *mut ::core::ffi::c_void,
    pub pFrame: *mut VdbeFrame,
    pub pDelFrame: *mut VdbeFrame,
    pub nFrame: ::core::ffi::c_int,
    pub expmask: u32_0,
    pub pProgram: *mut SubProgram,
    pub pAuxData: *mut AuxData

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AuxData {
    pub iAuxOp: ::core::ffi::c_int,
    pub iAuxArg: ::core::ffi::c_int,
    pub pAux: *mut ::core::ffi::c_void,
    pub xDeleteAux: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pNextAux: *mut AuxData

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
pub type Mem = sqlite3_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeFrame {
    pub v: *mut Vdbe,
    pub pParent: *mut VdbeFrame,
    pub aOp: *mut Op,
    pub aMem: *mut Mem,
    pub apCsr: *mut *mut VdbeCursor,
    pub aOnce: *mut u8_0,
    pub token: *mut ::core::ffi::c_void,
    pub lastRowid: i64_0,
    pub pAuxData: *mut AuxData,
    pub nCursor: ::core::ffi::c_int,
    pub pc: ::core::ffi::c_int,
    pub nOp: ::core::ffi::c_int,
    pub nMem: ::core::ffi::c_int,
    pub nChildMem: ::core::ffi::c_int,
    pub nChildCsr: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub nDbChange: i64_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VdbeCursor {
    pub eCurType: u8_0,
    pub iDb: i8_0,
    pub nullRow: u8_0,
    pub deferredMoveto: u8_0,
    pub isTable: u8_0,
    #[bitfield(name = "isEphemeral", ty = "Bool", bits = "0..=0")]
    #[bitfield(name = "useRandomRowid", ty = "Bool", bits = "1..=1")]
    #[bitfield(name = "isOrdered", ty = "Bool", bits = "2..=2")]
    #[bitfield(name = "noReuse", ty = "Bool", bits = "3..=3")]
    #[bitfield(name = "colCache", ty = "Bool", bits = "4..=4")]
    pub isEphemeral_useRandomRowid_isOrdered_noReuse_colCache: [u8; 1],
    pub seekHit: u16_0,
    pub ub: C2Rust_Unnamed_4,
    pub seqCount: i64_0,
    pub cacheStatus: u32_0,
    pub seekResult: ::core::ffi::c_int,
    pub pAltCursor: *mut VdbeCursor,
    pub uc: C2Rust_Unnamed_3,
    pub pKeyInfo: *mut KeyInfo,
    pub iHdrOffset: u32_0,
    pub pgnoRoot: Pgno,
    pub nField: i16_0,
    pub nHdrParsed: u16_0,
    pub movetoTarget: i64_0,
    pub aOffset: *mut u32_0,
    pub aRow: *const u8_0,
    pub payloadSize: u32_0,
    pub szRow: u32_0,
    pub pCache: *mut VdbeTxtBlbCache,
    pub aType: [u32_0; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeTxtBlbCache {
    pub pCValue: *mut ::core::ffi::c_char,
    pub iOffset: i64_0,
    pub iCol: ::core::ffi::c_int,
    pub cacheStatus: u32_0,
    pub colCacheCtr: u32_0

}
pub type i16_0 = int16_t;
pub type int16_t = __int16_t;
pub type __int16_t = i16;
pub type Pgno = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub pCursor: *mut BtCursor,
    pub pVCur: *mut sqlite3_vtab_cursor,
    pub pSorter: *mut VdbeSorter

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub pBtx: *mut Btree,
    pub aAltMap: *mut u32_0

}
pub type Bool = ::core::ffi::c_uint;
pub type i8_0 = int8_t;
pub type int8_t = __int8_t;
pub type __int8_t = i8;
pub type Op = VdbeOp;
pub type yDbMask = ::core::ffi::c_uint;
pub type bft = ::core::ffi::c_uint;
pub type VList = ::core::ffi::c_int;
pub type ynVar = i16_0;
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
    pub u1: C2Rust_Unnamed_8,
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
    pub fg: C2Rust_Unnamed_7,
    pub u: C2Rust_Unnamed_5

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub x: C2Rust_Unnamed_6,
    pub iConstExprReg: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub iOrderByCol: u16_0,
    pub iAlias: u16_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
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
pub struct Token {
    pub z: *const ::core::ffi::c_char,
    pub n: ::core::ffi::c_uint

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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub cr: C2Rust_Unnamed_10,
    pub d: C2Rust_Unnamed_9

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
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
pub struct C2Rust_Unnamed_10 {
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
pub struct AutoincInfo {
    pub pNext: *mut AutoincInfo,
    pub pTab: *mut Table,
    pub iDb: ::core::ffi::c_int,
    pub regCtr: ::core::ffi::c_int

}
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
pub union C2Rust_Unnamed_11 {
    pub iJoin: ::core::ffi::c_int,
    pub iOfst: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub pList: *mut ExprList,
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_13 {
    pub zToken: *mut ::core::ffi::c_char,
    pub iValue: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_14 {
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
pub union C2Rust_Unnamed_15 {
    pub pIBIndex: *mut Index,
    pub pCteUse: *mut CteUse

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_16 {
    pub zIndexedBy: *mut ::core::ffi::c_char,
    pub pFuncArg: *mut ExprList,
    pub nRow: u32_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
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
pub union C2Rust_Unnamed_18 {
    pub tab: C2Rust_Unnamed_21,
    pub view: C2Rust_Unnamed_20,
    pub vtab: C2Rust_Unnamed_19

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub nArg: ::core::ffi::c_int,
    pub azArg: *mut *mut ::core::ffi::c_char,
    pub p: *mut VTable

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
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
pub union C2Rust_Unnamed_22 {
    pub isInterrupted: ::core::ffi::c_int,
    pub notUsed1: ::core::ffi::c_double

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreUpdate {
    pub v: *mut Vdbe,
    pub pCsr: *mut VdbeCursor,
    pub op: ::core::ffi::c_int,
    pub aRecord: *mut u8_0,
    pub pKeyinfo: *mut KeyInfo,
    pub pUnpacked: *mut UnpackedRecord,
    pub pNewUnpacked: *mut UnpackedRecord,
    pub iNewReg: ::core::ffi::c_int,
    pub iBlobWrite: ::core::ffi::c_int,
    pub iKey1: i64_0,
    pub iKey2: i64_0,
    pub oldipk: Mem,
    pub aNew: *mut Mem,
    pub pTab: *mut Table,
    pub pPk: *mut Index,
    pub apDflt: *mut *mut sqlite3_value,
    pub uKey: C2Rust_Unnamed_23

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub keyinfoSpace: [u8_0; 32]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnpackedRecord {
    pub pKeyInfo: *mut KeyInfo,
    pub aMem: *mut Mem,
    pub u: C2Rust_Unnamed_24,
    pub n: ::core::ffi::c_int,
    pub nField: u16_0,
    pub default_rc: i8_0,
    pub errCode: u8_0,
    pub r1: i8_0,
    pub r2: i8_0,
    pub eqSeen: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_24 {
    pub z: *mut ::core::ffi::c_char,
    pub i: i64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_25 {
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
pub type sqlite3StatValueType = sqlite3_int64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3StatType {
    pub nowValue: [sqlite3StatValueType; 10],
    pub mxValue: [sqlite3StatValueType; 10]

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
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_LOOKASIDE_USED: ::core::ffi::c_int = 0;
pub const SQLITE_DBSTATUS_CACHE_USED: ::core::ffi::c_int = 1;
pub const SQLITE_DBSTATUS_SCHEMA_USED: ::core::ffi::c_int = 2;
pub const SQLITE_DBSTATUS_STMT_USED: ::core::ffi::c_int = 3;
pub const SQLITE_DBSTATUS_LOOKASIDE_HIT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_SIZE: ::core::ffi::c_int = 5;
pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_FULL: ::core::ffi::c_int = 6;
pub const SQLITE_DBSTATUS_CACHE_HIT: ::core::ffi::c_int = 7;
pub const SQLITE_DBSTATUS_CACHE_MISS: ::core::ffi::c_int = 8;
pub const SQLITE_DBSTATUS_CACHE_WRITE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_DEFERRED_FKS: ::core::ffi::c_int = 10;
pub const SQLITE_DBSTATUS_CACHE_USED_SHARED: ::core::ffi::c_int = 11
    as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_CACHE_SPILL: ::core::ffi::c_int = 12;
pub const SQLITE_DBSTATUS_TEMPBUF_SPILL: ::core::ffi::c_int = 13;
static mut sqlite3Stat: sqlite3StatType = sqlite3StatType {
    nowValue: [
        0 as ::core::ffi::c_int as sqlite3StatValueType,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    mxValue: [0 as ::core::ffi::c_int as sqlite3StatValueType, 0, 0, 0, 0, 0, 0, 0, 0, 0]

};
static mut statMutex: [::core::ffi::c_char; 10] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn sqlite3StatusValue(
    mut op: ::core::ffi::c_int,
) -> sqlite3_int64 {
    unsafe {
        return sqlite3Stat.nowValue[op as usize];
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3StatusUp(
    mut op: ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3Stat.nowValue[op as usize] += N as ::core::ffi::c_longlong;
        if sqlite3Stat.nowValue[op as usize] > sqlite3Stat.mxValue[op as usize] {
            sqlite3Stat.mxValue[op as usize] = sqlite3Stat.nowValue[op as usize];
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3StatusDown(
    mut op: ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3Stat.nowValue[op as usize] -= N as ::core::ffi::c_longlong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3StatusHighwater(
    mut op: ::core::ffi::c_int,
    mut X: ::core::ffi::c_int,
) {
    unsafe {
        let mut newValue: sqlite3StatValueType = 0;
        newValue = X as sqlite3StatValueType;
        if newValue > sqlite3Stat.mxValue[op as usize] {
            sqlite3Stat.mxValue[op as usize] = newValue;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_status64(
    mut op: ::core::ffi::c_int,
    mut pCurrent: *mut sqlite3_int64,
    mut pHighwater: *mut sqlite3_int64,
    mut resetFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pMutex: *mut sqlite3_mutex = ::core::ptr::null_mut::<sqlite3_mutex>();
        if op < 0 as ::core::ffi::c_int
            || op
                >= (::core::mem::size_of::<[sqlite3StatValueType; 10]>() as usize)
                    .wrapping_div(
                        ::core::mem::size_of::<sqlite3StatValueType>() as usize,
                    ) as ::core::ffi::c_int
        {
            return sqlite3MisuseError(143 as ::core::ffi::c_int);
        }
        pMutex = if statMutex[op as usize] as ::core::ffi::c_int != 0 {
            sqlite3Pcache1Mutex()
        } else {
            sqlite3MallocMutex()
        };
        sqlite3_mutex_enter(pMutex);
        *pCurrent = sqlite3Stat.nowValue[op as usize] as sqlite3_int64;
        *pHighwater = sqlite3Stat.mxValue[op as usize] as sqlite3_int64;
        if resetFlag != 0 {
            sqlite3Stat.mxValue[op as usize] = sqlite3Stat.nowValue[op as usize];
        }
        sqlite3_mutex_leave(pMutex);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_status(
    mut op: ::core::ffi::c_int,
    mut pCurrent: *mut ::core::ffi::c_int,
    mut pHighwater: *mut ::core::ffi::c_int,
    mut resetFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iCur: sqlite3_int64 = 0 as sqlite3_int64;
        let mut iHwtr: sqlite3_int64 = 0 as sqlite3_int64;
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3_status64(op, &raw mut iCur, &raw mut iHwtr, resetFlag);
        if rc == 0 as ::core::ffi::c_int {
            *pCurrent = iCur as ::core::ffi::c_int;
            *pHighwater = iHwtr as ::core::ffi::c_int;
        }
        return rc;
    }
}
unsafe extern "C" fn countLookasideSlots(mut p: *mut LookasideSlot) -> u32_0 {
    unsafe {
        let mut cnt: u32_0 = 0 as u32_0;
        while !p.is_null() {
            p = (*p).pNext;
            cnt = cnt.wrapping_add(1);
        }
        return cnt;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3LookasideUsed(
    mut db: *mut sqlite3,
    mut pHighwater: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nInit: u32_0 = countLookasideSlots((*db).lookaside.pInit);
        let mut nFree: u32_0 = countLookasideSlots((*db).lookaside.pFree);
        nInit = (nInit as ::core::ffi::c_uint)
            .wrapping_add(
                countLookasideSlots((*db).lookaside.pSmallInit) as ::core::ffi::c_uint,
            ) as u32_0 as u32_0;
        nFree = (nFree as ::core::ffi::c_uint)
            .wrapping_add(
                countLookasideSlots((*db).lookaside.pSmallFree) as ::core::ffi::c_uint,
            ) as u32_0 as u32_0;
        if !pHighwater.is_null() {
            *pHighwater = (*db).lookaside.nSlot.wrapping_sub(nInit)
                as ::core::ffi::c_int;
        }
        return (*db).lookaside.nSlot.wrapping_sub(nInit.wrapping_add(nFree))
            as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_db_status64(
    mut db: *mut sqlite3,
    mut op: ::core::ffi::c_int,
    mut pCurrent: *mut sqlite3_int64,
    mut pHighwtr: *mut sqlite3_int64,
    mut resetFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        sqlite3_mutex_enter((*db).mutex);
        let mut c2rust_current_block_105: u64;
        match op {
            SQLITE_DBSTATUS_LOOKASIDE_USED => {
                let mut H: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                *pCurrent = sqlite3LookasideUsed(db, &raw mut H) as sqlite3_int64;
                *pHighwtr = H as sqlite3_int64;
                if resetFlag != 0 {
                    let mut p: *mut LookasideSlot = (*db).lookaside.pFree;
                    if !p.is_null() {
                        while !(*p).pNext.is_null() {
                            p = (*p).pNext;
                        }
                        (*p).pNext = (*db).lookaside.pInit;
                        (*db).lookaside.pInit = (*db).lookaside.pFree;
                        (*db).lookaside.pFree = ::core::ptr::null_mut::<LookasideSlot>();
                    }
                    p = (*db).lookaside.pSmallFree;
                    if !p.is_null() {
                        while !(*p).pNext.is_null() {
                            p = (*p).pNext;
                        }
                        (*p).pNext = (*db).lookaside.pSmallInit;
                        (*db).lookaside.pSmallInit = (*db).lookaside.pSmallFree;
                        (*db).lookaside.pSmallFree = ::core::ptr::null_mut::<
                            LookasideSlot,
                        >();
                    }
                }
                c2rust_current_block_105 = 6406431739208918833;
            }
            SQLITE_DBSTATUS_LOOKASIDE_HIT
            | SQLITE_DBSTATUS_LOOKASIDE_MISS_SIZE
            | SQLITE_DBSTATUS_LOOKASIDE_MISS_FULL => {
                *pCurrent = 0 as sqlite3_int64;
                *pHighwtr = (*db)
                    .lookaside
                    .anStat[(op - SQLITE_DBSTATUS_LOOKASIDE_HIT) as usize]
                    as sqlite3_int64;
                if resetFlag != 0 {
                    (*db)
                        .lookaside
                        .anStat[(op - SQLITE_DBSTATUS_LOOKASIDE_HIT) as usize] = 0
                        as u32_0;
                }
                c2rust_current_block_105 = 6406431739208918833;
            }
            SQLITE_DBSTATUS_CACHE_USED_SHARED | SQLITE_DBSTATUS_CACHE_USED => {
                let mut totalUsed: sqlite3_int64 = 0 as sqlite3_int64;
                let mut i: ::core::ffi::c_int = 0;
                sqlite3BtreeEnterAll(db);
                i = 0 as ::core::ffi::c_int;
                while i < (*db).nDb {
                    let mut pBt: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
                    if !pBt.is_null() {
                        let mut pPager: *mut Pager = sqlite3BtreePager(pBt)
                            as *mut Pager;
                        let mut nByte: ::core::ffi::c_int = sqlite3PagerMemUsed(pPager);
                        if op == SQLITE_DBSTATUS_CACHE_USED_SHARED {
                            nByte = nByte / sqlite3BtreeConnectionCount(pBt);
                        }
                        totalUsed += nByte as ::core::ffi::c_longlong;
                    }
                    i += 1;
                }
                sqlite3BtreeLeaveAll(db);
                *pCurrent = totalUsed;
                *pHighwtr = 0 as sqlite3_int64;
                c2rust_current_block_105 = 6406431739208918833;
            }
            SQLITE_DBSTATUS_SCHEMA_USED => {
                let mut i_0: ::core::ffi::c_int = 0;
                let mut nByte_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                sqlite3BtreeEnterAll(db);
                (*db).pnBytesFreed = &raw mut nByte_0;
                (*db).lookaside.pEnd = (*db).lookaside.pStart;
                i_0 = 0 as ::core::ffi::c_int;
                while i_0 < (*db).nDb {
                    let mut pSchema: *mut Schema = (*(*db).aDb.offset(i_0 as isize))
                        .pSchema;
                    if !pSchema.is_null() {
                        let mut p_0: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
                        nByte_0 = (nByte_0 as ::core::ffi::c_uint)
                            .wrapping_add(
                                (sqlite3Config
                                    .m
                                    .xRoundup
                                    .expect(
                                        "non-null function pointer",
                                    )(::core::mem::size_of::<HashElem>() as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint)
                                    .wrapping_mul(
                                        (*pSchema)
                                            .tblHash
                                            .count
                                            .wrapping_add((*pSchema).trigHash.count)
                                            .wrapping_add((*pSchema).idxHash.count)
                                            .wrapping_add((*pSchema).fkeyHash.count),
                                    ),
                            ) as ::core::ffi::c_int as ::core::ffi::c_int;
                        nByte_0 = (nByte_0 as ::core::ffi::c_ulonglong)
                            .wrapping_add(
                                sqlite3_msize(
                                    (*pSchema).tblHash.ht as *mut ::core::ffi::c_void,
                                ) as ::core::ffi::c_ulonglong,
                            ) as ::core::ffi::c_int as ::core::ffi::c_int;
                        nByte_0 = (nByte_0 as ::core::ffi::c_ulonglong)
                            .wrapping_add(
                                sqlite3_msize(
                                    (*pSchema).trigHash.ht as *mut ::core::ffi::c_void,
                                ) as ::core::ffi::c_ulonglong,
                            ) as ::core::ffi::c_int as ::core::ffi::c_int;
                        nByte_0 = (nByte_0 as ::core::ffi::c_ulonglong)
                            .wrapping_add(
                                sqlite3_msize(
                                    (*pSchema).idxHash.ht as *mut ::core::ffi::c_void,
                                ) as ::core::ffi::c_ulonglong,
                            ) as ::core::ffi::c_int as ::core::ffi::c_int;
                        nByte_0 = (nByte_0 as ::core::ffi::c_ulonglong)
                            .wrapping_add(
                                sqlite3_msize(
                                    (*pSchema).fkeyHash.ht as *mut ::core::ffi::c_void,
                                ) as ::core::ffi::c_ulonglong,
                            ) as ::core::ffi::c_int as ::core::ffi::c_int;
                        p_0 = (*pSchema).trigHash.first;
                        while !p_0.is_null() {
                            sqlite3DeleteTrigger(db, (*p_0).data as *mut Trigger);
                            p_0 = (*p_0).next;
                        }
                        p_0 = (*pSchema).tblHash.first;
                        while !p_0.is_null() {
                            sqlite3DeleteTable(db, (*p_0).data as *mut Table);
                            p_0 = (*p_0).next;
                        }
                    }
                    i_0 += 1;
                }
                (*db).pnBytesFreed = ::core::ptr::null_mut::<::core::ffi::c_int>();
                (*db).lookaside.pEnd = (*db).lookaside.pTrueEnd;
                sqlite3BtreeLeaveAll(db);
                *pHighwtr = 0 as sqlite3_int64;
                *pCurrent = nByte_0 as sqlite3_int64;
                c2rust_current_block_105 = 6406431739208918833;
            }
            SQLITE_DBSTATUS_STMT_USED => {
                let mut pVdbe: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
                let mut nByte_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                (*db).pnBytesFreed = &raw mut nByte_1;
                (*db).lookaside.pEnd = (*db).lookaside.pStart;
                pVdbe = (*db).pVdbe;
                while !pVdbe.is_null() {
                    sqlite3VdbeDelete(pVdbe as *mut Vdbe);
                    pVdbe = (*pVdbe).pVNext as *mut Vdbe;
                }
                (*db).lookaside.pEnd = (*db).lookaside.pTrueEnd;
                (*db).pnBytesFreed = ::core::ptr::null_mut::<::core::ffi::c_int>();
                *pHighwtr = 0 as sqlite3_int64;
                *pCurrent = nByte_1 as sqlite3_int64;
                c2rust_current_block_105 = 6406431739208918833;
            }
            SQLITE_DBSTATUS_CACHE_SPILL => {
                op = SQLITE_DBSTATUS_CACHE_WRITE + 1 as ::core::ffi::c_int;
                c2rust_current_block_105 = 2723324002591448311;
            }
            SQLITE_DBSTATUS_CACHE_HIT
            | SQLITE_DBSTATUS_CACHE_MISS
            | SQLITE_DBSTATUS_CACHE_WRITE => {
                c2rust_current_block_105 = 2723324002591448311;
            }
            SQLITE_DBSTATUS_TEMPBUF_SPILL => {
                let mut nRet_0: u64_0 = 0 as u64_0;
                if !(*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pBt.is_null() {
                    let mut pPager_1: *mut Pager = sqlite3BtreePager(
                        (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pBt,
                    ) as *mut Pager;
                    sqlite3PagerCacheStat(
                        pPager_1,
                        SQLITE_DBSTATUS_CACHE_WRITE,
                        resetFlag,
                        &raw mut nRet_0,
                    );
                    nRet_0 = (nRet_0 as ::core::ffi::c_ulonglong)
                        .wrapping_mul(
                            sqlite3BtreeGetPageSize(
                                (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pBt,
                            ) as ::core::ffi::c_ulonglong,
                        ) as u64_0 as u64_0;
                }
                nRet_0 = (nRet_0 as ::core::ffi::c_ulonglong)
                    .wrapping_add((*db).nSpill as ::core::ffi::c_ulonglong) as u64_0
                    as u64_0;
                if resetFlag != 0 {
                    (*db).nSpill = 0 as u64_0;
                }
                *pHighwtr = 0 as sqlite3_int64;
                *pCurrent = nRet_0 as sqlite3_int64;
                c2rust_current_block_105 = 6406431739208918833;
            }
            SQLITE_DBSTATUS_DEFERRED_FKS => {
                *pHighwtr = 0 as sqlite3_int64;
                *pCurrent = ((*db).nDeferredImmCons > 0 as ::core::ffi::c_longlong
                    || (*db).nDeferredCons > 0 as ::core::ffi::c_longlong)
                    as ::core::ffi::c_int as sqlite3_int64;
                c2rust_current_block_105 = 6406431739208918833;
            }
            _ => {
                rc = SQLITE_ERROR;
                c2rust_current_block_105 = 6406431739208918833;
            }
        }
        match c2rust_current_block_105 {
            2723324002591448311 => {
                let mut i_1: ::core::ffi::c_int = 0;
                let mut nRet: u64_0 = 0 as u64_0;
                i_1 = 0 as ::core::ffi::c_int;
                while i_1 < (*db).nDb {
                    if !(*(*db).aDb.offset(i_1 as isize)).pBt.is_null() {
                        let mut pPager_0: *mut Pager = sqlite3BtreePager(
                            (*(*db).aDb.offset(i_1 as isize)).pBt,
                        ) as *mut Pager;
                        sqlite3PagerCacheStat(pPager_0, op, resetFlag, &raw mut nRet);
                    }
                    i_1 += 1;
                }
                *pHighwtr = 0 as sqlite3_int64;
                *pCurrent = nRet as sqlite3_int64;
            }
            _ => {}
        }
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_db_status(
    mut db: *mut sqlite3,
    mut op: ::core::ffi::c_int,
    mut pCurrent: *mut ::core::ffi::c_int,
    mut pHighwtr: *mut ::core::ffi::c_int,
    mut resetFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut C: sqlite3_int64 = 0 as sqlite3_int64;
        let mut H: sqlite3_int64 = 0 as sqlite3_int64;
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3_db_status64(db, op, &raw mut C, &raw mut H, resetFlag);
        if rc == 0 as ::core::ffi::c_int {
            *pCurrent = (C as ::core::ffi::c_longlong
                & 0x7fffffff as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            *pHighwtr = (H as ::core::ffi::c_longlong
                & 0x7fffffff as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        }
        return rc;
    }
}
