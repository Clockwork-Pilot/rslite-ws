use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type SQLiteThread;
    pub type BtCursor;
    pub type Btree;
    pub type RenameToken;
    pub type TableLock;
    pub type VtabCtx;
    pub type sqlite3_mutex;
    pub type sqlite3_pcache;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
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
    fn sqlite3OsFileControlHint(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    );
    fn sqlite3OsFetch(
        id: *mut sqlite3_file,
        _: i64_0,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsUnfetch(
        _: *mut sqlite3_file,
        _: i64_0,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsOpenMalloc(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: *mut *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsCloseFree(_: *mut sqlite3_file);
    fn sqlite3BtreeGetPageSize(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeEnter(_: *mut Btree);
    fn sqlite3BtreeLeave(_: *mut Btree);
    fn sqlite3VdbeRecordUnpack(
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: *mut UnpackedRecord,
    );
    fn sqlite3VdbeRecordCompare(
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: *mut UnpackedRecord,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeRecordCompareWithSkip(
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: *mut UnpackedRecord,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAllocUnpackedRecord(_: *mut KeyInfo) -> *mut UnpackedRecord;
    fn sqlite3Malloc(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3MallocZero(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3Realloc(_: *mut ::core::ffi::c_void, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MallocSize(_: *const ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn sqlite3HeapNearlyFull() -> ::core::ffi::c_int;
    fn sqlite3FaultSim(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3PutVarint(_: *mut ::core::ffi::c_uchar, _: u64_0) -> ::core::ffi::c_int;
    fn sqlite3GetVarint(_: *const ::core::ffi::c_uchar, _: *mut u64_0) -> u8_0;
    fn sqlite3GetVarint32(_: *const ::core::ffi::c_uchar, _: *mut u32_0) -> u8_0;
    fn sqlite3VarintLen(v: u64_0) -> ::core::ffi::c_int;
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3TempInMemory(_: *const sqlite3) -> ::core::ffi::c_int;
    fn sqlite3ThreadCreate(
        _: *mut *mut SQLiteThread,
        _: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3ThreadJoin(
        _: *mut SQLiteThread,
        _: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemClearAndResize(
        pMem: *mut Mem,
        n: ::core::ffi::c_int,
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
    pub trace: C2Rust_Unnamed_26,
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
    pub u1: C2Rust_Unnamed_24,
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
    pub u: C2Rust_Unnamed_20,
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
    pub fg: C2Rust_Unnamed_19,
    pub iCursor: ::core::ffi::c_int,
    pub colUsed: Bitmask,
    pub u1: C2Rust_Unnamed_18,
    pub u2: C2Rust_Unnamed_17,
    pub u3: C2Rust_Unnamed_16,
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
    pub u: C2Rust_Unnamed_15,
    pub pLeft: *mut Expr,
    pub pRight: *mut Expr,
    pub x: C2Rust_Unnamed_14,
    pub nHeight: ::core::ffi::c_int,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ynVar,
    pub iAgg: i16_0,
    pub w: C2Rust_Unnamed_13,
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
    pub ub: C2Rust_Unnamed_6,
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
pub struct VdbeSorter {
    pub mnPmaSize: ::core::ffi::c_int,
    pub mxPmaSize: ::core::ffi::c_int,
    pub mxKeysize: ::core::ffi::c_int,
    pub pgsz: ::core::ffi::c_int,
    pub pReader: *mut PmaReader,
    pub pMerger: *mut MergeEngine,
    pub db: *mut sqlite3,
    pub pKeyInfo: *mut KeyInfo,
    pub pUnpacked: *mut UnpackedRecord,
    pub list: SorterList,
    pub iMemory: ::core::ffi::c_int,
    pub nMemory: ::core::ffi::c_int,
    pub bUsePMA: u8_0,
    pub bUseThreads: u8_0,
    pub iPrev: u8_0,
    pub nTask: u8_0,
    pub typeMask: u8_0,
    pub aTask: [SortSubtask; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SortSubtask {
    pub pThread: *mut SQLiteThread,
    pub bDone: ::core::ffi::c_int,
    pub nPMA: ::core::ffi::c_int,
    pub pSorter: *mut VdbeSorter,
    pub pUnpacked: *mut UnpackedRecord,
    pub list: SorterList,
    pub xCompare: SorterCompare,
    pub file: SorterFile,
    pub file2: SorterFile,
    pub nSpill: u64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SorterFile {
    pub pFd: *mut sqlite3_file,
    pub iEof: i64_0

}
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
pub type SorterCompare = Option<
    unsafe extern "C" fn(
        *mut SortSubtask,
        *mut ::core::ffi::c_int,
        *const ::core::ffi::c_void,
        ::core::ffi::c_int,
        *const ::core::ffi::c_void,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SorterList {
    pub pList: *mut SorterRecord,
    pub aMemory: *mut u8_0,
    pub szPMA: i64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SorterRecord {
    pub nVal: ::core::ffi::c_int,
    pub u: C2Rust_Unnamed_4

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub pNext: *mut SorterRecord,
    pub iNext: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnpackedRecord {
    pub pKeyInfo: *mut KeyInfo,
    pub aMem: *mut Mem,
    pub u: C2Rust_Unnamed_5,
    pub n: ::core::ffi::c_int,
    pub nField: u16_0,
    pub default_rc: i8_0,
    pub errCode: u8_0,
    pub r1: i8_0,
    pub r2: i8_0,
    pub eqSeen: u8_0

}
pub type i8_0 = int8_t;
pub type int8_t = __int8_t;
pub type __int8_t = i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub z: *mut ::core::ffi::c_char,
    pub i: i64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MergeEngine {
    pub nTree: ::core::ffi::c_int,
    pub pTask: *mut SortSubtask,
    pub aTree: *mut ::core::ffi::c_int,
    pub aReadr: *mut PmaReader

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PmaReader {
    pub iReadOff: i64_0,
    pub iEof: i64_0,
    pub nAlloc: ::core::ffi::c_int,
    pub nKey: ::core::ffi::c_int,
    pub pFd: *mut sqlite3_file,
    pub aAlloc: *mut u8_0,
    pub aKey: *mut u8_0,
    pub aBuffer: *mut u8_0,
    pub nBuffer: ::core::ffi::c_int,
    pub aMap: *mut u8_0,
    pub pIncr: *mut IncrMerger

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IncrMerger {
    pub pTask: *mut SortSubtask,
    pub pMerger: *mut MergeEngine,
    pub iStartOff: i64_0,
    pub mxSz: ::core::ffi::c_int,
    pub bEof: ::core::ffi::c_int,
    pub bUseThread: ::core::ffi::c_int,
    pub aFile: [SorterFile; 2]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub pBtx: *mut Btree,
    pub aAltMap: *mut u32_0

}
pub type Bool = ::core::ffi::c_uint;
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
    pub u1: C2Rust_Unnamed_10,
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
    pub fg: C2Rust_Unnamed_9,
    pub u: C2Rust_Unnamed_7

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub x: C2Rust_Unnamed_8,
    pub iConstExprReg: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub iOrderByCol: u16_0,
    pub iAlias: u16_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
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
pub union C2Rust_Unnamed_10 {
    pub cr: C2Rust_Unnamed_12,
    pub d: C2Rust_Unnamed_11

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
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
pub struct C2Rust_Unnamed_12 {
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
pub union C2Rust_Unnamed_13 {
    pub iJoin: ::core::ffi::c_int,
    pub iOfst: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_14 {
    pub pList: *mut ExprList,
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_15 {
    pub zToken: *mut ::core::ffi::c_char,
    pub iValue: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_16 {
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
pub union C2Rust_Unnamed_17 {
    pub pIBIndex: *mut Index,
    pub pCteUse: *mut CteUse

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_18 {
    pub zIndexedBy: *mut ::core::ffi::c_char,
    pub pFuncArg: *mut ExprList,
    pub nRow: u32_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
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
pub union C2Rust_Unnamed_20 {
    pub tab: C2Rust_Unnamed_23,
    pub view: C2Rust_Unnamed_22,
    pub vtab: C2Rust_Unnamed_21

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub nArg: ::core::ffi::c_int,
    pub azArg: *mut *mut ::core::ffi::c_char,
    pub p: *mut VTable

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
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
pub union C2Rust_Unnamed_24 {
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
    pub uKey: C2Rust_Unnamed_25

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub keyinfoSpace: [u8_0; 32]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_26 {
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
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
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
pub type intptr_t = isize;
pub type size_t = usize;
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
pub struct PmaWriter {
    pub eFWErr: ::core::ffi::c_int,
    pub aBuffer: *mut u8_0,
    pub nBuffer: ::core::ffi::c_int,
    pub iBufStart: ::core::ffi::c_int,
    pub iBufEnd: ::core::ffi::c_int,
    pub iWriteOff: i64_0,
    pub pFd: *mut sqlite3_file,
    pub nPmaSpill: u64_0

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_IOERR_READ: ::core::ffi::c_int = SQLITE_IOERR
    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_ACCESS: ::core::ffi::c_int = SQLITE_IOERR
    | (13 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_DELETEONCLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_EXCLUSIVE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_OPEN_TEMP_JOURNAL: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_SIZE_HINT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_CHUNK_SIZE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_MMAP_SIZE: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_WORKER_THREADS: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_MAX_WORKER_THREADS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_MAX_MMAP_SIZE: ::core::ffi::c_int = 0x7fff0000 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_BIGNULL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const MEM_Null: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MEM_Zero: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const MEM_TypeMask: ::core::ffi::c_int = 0xdbf as ::core::ffi::c_int;
pub const SORTER_TYPE_INTEGER: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SORTER_TYPE_TEXT: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SORTER_MAX_MERGE_COUNT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
unsafe extern "C" fn vdbePmaReaderClear(mut pReadr: *mut PmaReader) {
    unsafe {
        sqlite3_free((*pReadr).aAlloc as *mut ::core::ffi::c_void);
        sqlite3_free((*pReadr).aBuffer as *mut ::core::ffi::c_void);
        if !(*pReadr).aMap.is_null() {
            sqlite3OsUnfetch(
                (*pReadr).pFd,
                0 as i64_0,
                (*pReadr).aMap as *mut ::core::ffi::c_void,
            );
        }
        vdbeIncrFree((*pReadr).pIncr);
        memset(
            pReadr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<PmaReader>() as size_t,
        );
    }
}
unsafe extern "C" fn vdbePmaReadBlob(
    mut p: *mut PmaReader,
    mut nByte: ::core::ffi::c_int,
    mut ppOut: *mut *mut u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iBuf: ::core::ffi::c_int = 0;
        let mut nAvail: ::core::ffi::c_int = 0;
        if !(*p).aMap.is_null() {
            *ppOut = (*p).aMap.offset((*p).iReadOff as isize) as *mut u8_0;
            (*p).iReadOff += nByte as ::core::ffi::c_longlong;
            return SQLITE_OK;
        }
        iBuf = ((*p).iReadOff as ::core::ffi::c_longlong
            % (*p).nBuffer as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        if iBuf == 0 as ::core::ffi::c_int {
            let mut nRead: ::core::ffi::c_int = 0;
            let mut rc: ::core::ffi::c_int = 0;
            if (*p).iEof - (*p).iReadOff > (*p).nBuffer as i64_0 {
                nRead = (*p).nBuffer;
            } else {
                nRead = ((*p).iEof - (*p).iReadOff) as ::core::ffi::c_int;
            }
            rc = sqlite3OsRead(
                (*p).pFd,
                (*p).aBuffer as *mut ::core::ffi::c_void,
                nRead,
                (*p).iReadOff,
            );
            if rc != SQLITE_OK {
                return rc;
            }
        }
        nAvail = (*p).nBuffer - iBuf;
        if nByte <= nAvail {
            *ppOut = (*p).aBuffer.offset(iBuf as isize) as *mut u8_0;
            (*p).iReadOff += nByte as ::core::ffi::c_longlong;
        } else {
            let mut nRem: ::core::ffi::c_int = 0;
            if (*p).nAlloc < nByte {
                let mut aNew: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                let mut nNew: sqlite3_int64 = if 128 as ::core::ffi::c_longlong
                    > 2 as sqlite3_int64 * (*p).nAlloc as sqlite3_int64
                {
                    128 as sqlite3_int64
                } else {
                    2 as sqlite3_int64 * (*p).nAlloc as sqlite3_int64
                };
                while nByte as ::core::ffi::c_longlong > nNew {
                    nNew = (nNew as ::core::ffi::c_longlong
                        * 2 as ::core::ffi::c_longlong) as sqlite3_int64;
                }
                aNew = sqlite3Realloc(
                    (*p).aAlloc as *mut ::core::ffi::c_void,
                    nNew as u64_0,
                ) as *mut u8_0;
                if aNew.is_null() {
                    return SQLITE_NOMEM_BKPT;
                }
                (*p).nAlloc = nNew as ::core::ffi::c_int;
                (*p).aAlloc = aNew;
            }
            memcpy(
                (*p).aAlloc as *mut ::core::ffi::c_void,
                (*p).aBuffer.offset(iBuf as isize) as *mut u8_0
                    as *const ::core::ffi::c_void,
                nAvail as size_t,
            );
            (*p).iReadOff += nAvail as ::core::ffi::c_longlong;
            nRem = nByte - nAvail;
            while nRem > 0 as ::core::ffi::c_int {
                let mut rc_0: ::core::ffi::c_int = 0;
                let mut nCopy: ::core::ffi::c_int = 0;
                let mut aNext: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                nCopy = nRem;
                if nRem > (*p).nBuffer {
                    nCopy = (*p).nBuffer;
                }
                rc_0 = vdbePmaReadBlob(p, nCopy, &raw mut aNext);
                if rc_0 != SQLITE_OK {
                    return rc_0;
                }
                memcpy(
                    (*p).aAlloc.offset((nByte - nRem) as isize) as *mut u8_0
                        as *mut ::core::ffi::c_void,
                    aNext as *const ::core::ffi::c_void,
                    nCopy as size_t,
                );
                nRem -= nCopy;
            }
            *ppOut = (*p).aAlloc;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn vdbePmaReadVarint(
    mut p: *mut PmaReader,
    mut pnOut: *mut u64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iBuf: ::core::ffi::c_int = 0;
        if !(*p).aMap.is_null() {
            (*p).iReadOff
                += sqlite3GetVarint(
                    (*p).aMap.offset((*p).iReadOff as isize) as *mut u8_0,
                    pnOut,
                ) as ::core::ffi::c_longlong;
        } else {
            iBuf = ((*p).iReadOff as ::core::ffi::c_longlong
                % (*p).nBuffer as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            if iBuf != 0 && (*p).nBuffer - iBuf >= 9 as ::core::ffi::c_int {
                (*p).iReadOff
                    += sqlite3GetVarint(
                        (*p).aBuffer.offset(iBuf as isize) as *mut u8_0,
                        pnOut,
                    ) as ::core::ffi::c_longlong;
            } else {
                let mut aVarint: [u8_0; 16] = [0; 16];
                let mut a: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut rc: ::core::ffi::c_int = 0;
                loop {
                    rc = vdbePmaReadBlob(p, 1 as ::core::ffi::c_int, &raw mut a);
                    if rc != 0 {
                        return rc;
                    }
                    let c2rust_fresh0 = i;
                    i = i + 1;
                    aVarint[(c2rust_fresh0 & 0xf as ::core::ffi::c_int) as usize] = *a
                        .offset(0 as ::core::ffi::c_int as isize);
                    if !(*a.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int)
                    {
                        break;
                    }
                }
                sqlite3GetVarint(&raw mut aVarint as *mut u8_0, pnOut);
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn vdbeSorterMapFile(
    mut pTask: *mut SortSubtask,
    mut pFile: *mut SorterFile,
    mut pp: *mut *mut u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFile).iEof <= (*(*(*pTask).pSorter).db).nMaxSorterMmap as i64_0 {
            let mut pFd: *mut sqlite3_file = (*pFile).pFd;
            if (*(*pFd).pMethods).iVersion >= 3 as ::core::ffi::c_int {
                rc = sqlite3OsFetch(
                    pFd,
                    0 as i64_0,
                    (*pFile).iEof as ::core::ffi::c_int,
                    pp as *mut *mut ::core::ffi::c_void,
                );
            }
        }
        return rc;
    }
}
unsafe extern "C" fn vdbePmaReaderSeek(
    mut pTask: *mut SortSubtask,
    mut pReadr: *mut PmaReader,
    mut pFile: *mut SorterFile,
    mut iOff: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if sqlite3FaultSim(201 as ::core::ffi::c_int) != 0 {
            return SQLITE_IOERR_READ;
        }
        if !(*pReadr).aMap.is_null() {
            sqlite3OsUnfetch(
                (*pReadr).pFd,
                0 as i64_0,
                (*pReadr).aMap as *mut ::core::ffi::c_void,
            );
            (*pReadr).aMap = ::core::ptr::null_mut::<u8_0>();
        }
        (*pReadr).iReadOff = iOff;
        (*pReadr).iEof = (*pFile).iEof;
        (*pReadr).pFd = (*pFile).pFd;
        rc = vdbeSorterMapFile(pTask, pFile, &raw mut (*pReadr).aMap);
        if rc == SQLITE_OK && (*pReadr).aMap.is_null() {
            let mut pgsz: ::core::ffi::c_int = (*(*pTask).pSorter).pgsz;
            let mut iBuf: ::core::ffi::c_int = ((*pReadr).iReadOff
                as ::core::ffi::c_longlong % pgsz as ::core::ffi::c_longlong)
                as ::core::ffi::c_int;
            if (*pReadr).aBuffer.is_null() {
                (*pReadr).aBuffer = sqlite3Malloc(pgsz as u64_0) as *mut u8_0;
                if (*pReadr).aBuffer.is_null() {
                    rc = SQLITE_NOMEM_BKPT;
                }
                (*pReadr).nBuffer = pgsz;
            }
            if rc == SQLITE_OK && iBuf != 0 {
                let mut nRead: ::core::ffi::c_int = pgsz - iBuf;
                if (*pReadr).iReadOff as ::core::ffi::c_longlong
                    + nRead as ::core::ffi::c_longlong > (*pReadr).iEof
                {
                    nRead = ((*pReadr).iEof - (*pReadr).iReadOff) as ::core::ffi::c_int;
                }
                rc = sqlite3OsRead(
                    (*pReadr).pFd,
                    (*pReadr).aBuffer.offset(iBuf as isize) as *mut u8_0
                        as *mut ::core::ffi::c_void,
                    nRead,
                    (*pReadr).iReadOff,
                );
            }
        }
        return rc;
    }
}
unsafe extern "C" fn vdbePmaReaderNext(
    mut pReadr: *mut PmaReader,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nRec: u64_0 = 0 as u64_0;
        if (*pReadr).iReadOff >= (*pReadr).iEof {
            let mut pIncr: *mut IncrMerger = (*pReadr).pIncr;
            let mut bEof: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            if !pIncr.is_null() {
                rc = vdbeIncrSwap(pIncr);
                if rc == SQLITE_OK && (*pIncr).bEof == 0 as ::core::ffi::c_int {
                    rc = vdbePmaReaderSeek(
                        (*pIncr).pTask,
                        pReadr,
                        (&raw mut (*pIncr).aFile as *mut SorterFile)
                            .offset(0 as ::core::ffi::c_int as isize) as *mut SorterFile,
                        (*pIncr).iStartOff,
                    );
                    bEof = 0 as ::core::ffi::c_int;
                }
            }
            if bEof != 0 {
                vdbePmaReaderClear(pReadr);
                return rc;
            }
        }
        if rc == SQLITE_OK {
            rc = vdbePmaReadVarint(pReadr, &raw mut nRec);
        }
        if rc == SQLITE_OK {
            (*pReadr).nKey = nRec as ::core::ffi::c_int;
            rc = vdbePmaReadBlob(
                pReadr,
                nRec as ::core::ffi::c_int,
                &raw mut (*pReadr).aKey,
            );
        }
        return rc;
    }
}
unsafe extern "C" fn vdbePmaReaderInit(
    mut pTask: *mut SortSubtask,
    mut pFile: *mut SorterFile,
    mut iStart: i64_0,
    mut pReadr: *mut PmaReader,
    mut pnByte: *mut i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = vdbePmaReaderSeek(pTask, pReadr, pFile, iStart);
        if rc == SQLITE_OK {
            let mut nByte: u64_0 = 0 as u64_0;
            rc = vdbePmaReadVarint(pReadr, &raw mut nByte);
            (*pReadr).iEof = ((*pReadr).iReadOff as u64_0).wrapping_add(nByte) as i64_0;
            *pnByte = (*pnByte as ::core::ffi::c_ulonglong)
                .wrapping_add(nByte as ::core::ffi::c_ulonglong) as i64_0 as i64_0;
        }
        if rc == SQLITE_OK {
            rc = vdbePmaReaderNext(pReadr);
        }
        return rc;
    }
}
unsafe extern "C" fn vdbeSorterCompareTail(
    mut pTask: *mut SortSubtask,
    mut pbKey2Cached: *mut ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut nKey1: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
    mut nKey2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut r2: *mut UnpackedRecord = (*pTask).pUnpacked;
        if *pbKey2Cached == 0 as ::core::ffi::c_int {
            sqlite3VdbeRecordUnpack(nKey2, pKey2, r2);
            *pbKey2Cached = 1 as ::core::ffi::c_int;
        }
        return sqlite3VdbeRecordCompareWithSkip(
            nKey1,
            pKey1,
            r2,
            1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn vdbeSorterCompare(
    mut pTask: *mut SortSubtask,
    mut pbKey2Cached: *mut ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut nKey1: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
    mut nKey2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut r2: *mut UnpackedRecord = (*pTask).pUnpacked;
        if *pbKey2Cached == 0 {
            sqlite3VdbeRecordUnpack(nKey2, pKey2, r2);
            *pbKey2Cached = 1 as ::core::ffi::c_int;
        }
        return sqlite3VdbeRecordCompare(nKey1, pKey1, r2);
    }
}
unsafe extern "C" fn vdbeSorterCompareText(
    mut pTask: *mut SortSubtask,
    mut pbKey2Cached: *mut ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut nKey1: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
    mut nKey2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let p1: *const u8_0 = pKey1 as *const u8_0;
        let p2: *const u8_0 = pKey2 as *const u8_0;
        let v1: *const u8_0 = p1
            .offset(*p1.offset(0 as ::core::ffi::c_int as isize) as isize)
            as *const u8_0;
        let v2: *const u8_0 = p2
            .offset(*p2.offset(0 as ::core::ffi::c_int as isize) as isize)
            as *const u8_0;
        let mut n1: ::core::ffi::c_int = 0;
        let mut n2: ::core::ffi::c_int = 0;
        let mut res: ::core::ffi::c_int = 0;
        n1 = *p1.offset(1 as ::core::ffi::c_int as isize) as u32_0 as ::core::ffi::c_int;
        if n1 >= 0x80 as ::core::ffi::c_int {
            sqlite3GetVarint32(
                p1.offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_uchar,
                &raw mut n1 as *mut u32_0,
            );
        }
        n2 = *p2.offset(1 as ::core::ffi::c_int as isize) as u32_0 as ::core::ffi::c_int;
        if n2 >= 0x80 as ::core::ffi::c_int {
            sqlite3GetVarint32(
                p2.offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_uchar,
                &raw mut n2 as *mut u32_0,
            );
        }
        res = memcmp(
            v1 as *const ::core::ffi::c_void,
            v2 as *const ::core::ffi::c_void,
            (((if n1 < n2 { n1 } else { n2 }) - 13 as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int) as size_t,
        );
        if res == 0 as ::core::ffi::c_int {
            res = n1 - n2;
        }
        if res == 0 as ::core::ffi::c_int {
            if (*(*(*pTask).pSorter).pKeyInfo).nKeyField as ::core::ffi::c_int
                > 1 as ::core::ffi::c_int
            {
                res = vdbeSorterCompareTail(
                    pTask,
                    pbKey2Cached,
                    pKey1,
                    nKey1,
                    pKey2,
                    nKey2,
                );
            }
        } else if *(*(*(*pTask).pSorter).pKeyInfo)
            .aSortFlags
            .offset(0 as ::core::ffi::c_int as isize) != 0
        {
            res = res * -1 as ::core::ffi::c_int;
        }
        return res;
    }
}
unsafe extern "C" fn vdbeSorterCompareInt(
    mut pTask: *mut SortSubtask,
    mut pbKey2Cached: *mut ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut nKey1: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
    mut nKey2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let p1: *const u8_0 = pKey1 as *const u8_0;
        let p2: *const u8_0 = pKey2 as *const u8_0;
        let s1: ::core::ffi::c_int = *p1.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int;
        let s2: ::core::ffi::c_int = *p2.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int;
        let v1: *const u8_0 = p1
            .offset(*p1.offset(0 as ::core::ffi::c_int as isize) as isize)
            as *const u8_0;
        let v2: *const u8_0 = p2
            .offset(*p2.offset(0 as ::core::ffi::c_int as isize) as isize)
            as *const u8_0;
        let mut res: ::core::ffi::c_int = 0;
        if s1 == s2 {
            static mut aLen: [u8_0; 10] = [
                0 as ::core::ffi::c_int as u8_0,
                1 as ::core::ffi::c_int as u8_0,
                2 as ::core::ffi::c_int as u8_0,
                3 as ::core::ffi::c_int as u8_0,
                4 as ::core::ffi::c_int as u8_0,
                6 as ::core::ffi::c_int as u8_0,
                8 as ::core::ffi::c_int as u8_0,
                0 as ::core::ffi::c_int as u8_0,
                0 as ::core::ffi::c_int as u8_0,
                0 as ::core::ffi::c_int as u8_0,
            ];
            let n: u8_0 = aLen[s1 as usize];
            let mut i: ::core::ffi::c_int = 0;
            res = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < n as ::core::ffi::c_int {
                res = *v1.offset(i as isize) as ::core::ffi::c_int
                    - *v2.offset(i as isize) as ::core::ffi::c_int;
                if res != 0 as ::core::ffi::c_int {
                    if (*v1.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        ^ *v2.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) & 0x80 as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        res = if *v1.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
                        {
                            -1 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        };
                    }
                    break;
                } else {
                    i += 1;
                }
            }
        } else if s1 > 7 as ::core::ffi::c_int && s2 > 7 as ::core::ffi::c_int {
            res = s1 - s2;
        } else {
            if s2 > 7 as ::core::ffi::c_int {
                res = 1 as ::core::ffi::c_int;
            } else if s1 > 7 as ::core::ffi::c_int {
                res = -1 as ::core::ffi::c_int;
            } else {
                res = s1 - s2;
            }
            if res > 0 as ::core::ffi::c_int {
                if *v1 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                    res = -1 as ::core::ffi::c_int;
                }
            } else if *v2 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                res = 1 as ::core::ffi::c_int;
            }
        }
        if res == 0 as ::core::ffi::c_int {
            if (*(*(*pTask).pSorter).pKeyInfo).nKeyField as ::core::ffi::c_int
                > 1 as ::core::ffi::c_int
            {
                res = vdbeSorterCompareTail(
                    pTask,
                    pbKey2Cached,
                    pKey1,
                    nKey1,
                    pKey2,
                    nKey2,
                );
            }
        } else if *(*(*(*pTask).pSorter).pKeyInfo)
            .aSortFlags
            .offset(0 as ::core::ffi::c_int as isize) != 0
        {
            res = res * -1 as ::core::ffi::c_int;
        }
        return res;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSorterInit(
    mut db: *mut sqlite3,
    mut nField: ::core::ffi::c_int,
    mut pCsr: *mut VdbeCursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pgsz: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut pSorter: *mut VdbeSorter = ::core::ptr::null_mut::<VdbeSorter>();
        let mut pKeyInfo: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        let mut szKeyInfo: ::core::ffi::c_int = 0;
        let mut sz: i64_0 = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nWorker: ::core::ffi::c_int = 0;
        if sqlite3TempInMemory(db) != 0
            || sqlite3Config.bCoreMutex as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            nWorker = 0 as ::core::ffi::c_int;
        } else {
            nWorker = (*db).aLimit[SQLITE_LIMIT_WORKER_THREADS as usize];
        }
        szKeyInfo = (32 as usize)
            .wrapping_add(
                ((*(*pCsr).pKeyInfo).nAllField as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut CollSeq>() as usize),
            ) as ::core::ffi::c_int;
        sz = (96 as usize)
            .wrapping_add(
                ((nWorker + 1 as ::core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<SortSubtask>() as usize),
            ) as i64_0;
        pSorter = sqlite3DbMallocZero(
            db,
            (sz as ::core::ffi::c_longlong + szKeyInfo as ::core::ffi::c_longlong)
                as u64_0,
        ) as *mut VdbeSorter;
        (*pCsr).uc.pSorter = pSorter;
        if pSorter.is_null() {
            rc = SQLITE_NOMEM_BKPT;
        } else {
            let mut pBt: *mut Btree = (*(*db)
                .aDb
                .offset(0 as ::core::ffi::c_int as isize))
                .pBt;
            pKeyInfo = (pSorter as *mut u8_0).offset(sz as isize) as *mut KeyInfo;
            (*pSorter).pKeyInfo = pKeyInfo;
            memcpy(
                pKeyInfo as *mut ::core::ffi::c_void,
                (*pCsr).pKeyInfo as *const ::core::ffi::c_void,
                szKeyInfo as size_t,
            );
            (*pKeyInfo).db = ::core::ptr::null_mut::<sqlite3>();
            if nField != 0 && nWorker == 0 as ::core::ffi::c_int {
                (*pKeyInfo).nKeyField = nField as u16_0;
            }
            sqlite3BtreeEnter(pBt);
            pgsz = sqlite3BtreeGetPageSize(pBt);
            (*pSorter).pgsz = pgsz;
            sqlite3BtreeLeave(pBt);
            (*pSorter).nTask = (nWorker + 1 as ::core::ffi::c_int) as u8_0;
            (*pSorter).iPrev = (nWorker - 1 as ::core::ffi::c_int) as u8_0;
            (*pSorter).bUseThreads = ((*pSorter).nTask as ::core::ffi::c_int
                > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as u8_0;
            (*pSorter).db = db;
            i = 0 as ::core::ffi::c_int;
            while i < (*pSorter).nTask as ::core::ffi::c_int {
                let mut pTask: *mut SortSubtask = (&raw mut (*pSorter).aTask
                    as *mut SortSubtask)
                    .offset(i as isize) as *mut SortSubtask;
                (*pTask).pSorter = pSorter;
                i += 1;
            }
            if sqlite3TempInMemory(db) == 0 {
                let mut mxCache: i64_0 = 0;
                let mut szPma: u32_0 = sqlite3Config.szPma;
                (*pSorter).mnPmaSize = szPma.wrapping_mul(pgsz as u32_0)
                    as ::core::ffi::c_int;
                mxCache = (*(*(*db).aDb.offset(0 as ::core::ffi::c_int as isize))
                    .pSchema)
                    .cache_size as i64_0;
                if mxCache < 0 as ::core::ffi::c_longlong {
                    mxCache = (mxCache as ::core::ffi::c_longlong
                        * -1024 as ::core::ffi::c_int as ::core::ffi::c_longlong)
                        as i64_0;
                } else {
                    mxCache = (mxCache as ::core::ffi::c_longlong
                        * pgsz as ::core::ffi::c_longlong) as i64_0;
                }
                mxCache = (if mxCache
                    < ((1 as ::core::ffi::c_int) << 29 as ::core::ffi::c_int)
                        as ::core::ffi::c_longlong
                {
                    mxCache as ::core::ffi::c_longlong
                } else {
                    ((1 as ::core::ffi::c_int) << 29 as ::core::ffi::c_int)
                        as ::core::ffi::c_longlong
                }) as i64_0;
                (*pSorter).mxPmaSize = if (*pSorter).mnPmaSize
                    > mxCache as ::core::ffi::c_int
                {
                    (*pSorter).mnPmaSize
                } else {
                    mxCache as ::core::ffi::c_int
                };
                if sqlite3Config.bSmallMalloc as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*pSorter).nMemory = pgsz;
                    (*pSorter).list.aMemory = sqlite3Malloc(pgsz as u64_0) as *mut u8_0;
                    if (*pSorter).list.aMemory.is_null() {
                        rc = SQLITE_NOMEM_BKPT;
                    }
                }
            }
            if ((*pKeyInfo).nAllField as ::core::ffi::c_int) < 13 as ::core::ffi::c_int
                && ((*(&raw mut (*pKeyInfo).aColl as *mut *mut CollSeq)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .is_null()
                    || *(&raw mut (*pKeyInfo).aColl as *mut *mut CollSeq)
                        .offset(0 as ::core::ffi::c_int as isize) == (*db).pDfltColl)
                && *(*pKeyInfo).aSortFlags.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int & KEYINFO_ORDER_BIGNULL
                    == 0 as ::core::ffi::c_int
            {
                (*pSorter).typeMask = (SORTER_TYPE_INTEGER | SORTER_TYPE_TEXT) as u8_0;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn vdbeSorterRecordFree(
    mut db: *mut sqlite3,
    mut pRecord: *mut SorterRecord,
) {
    unsafe {
        let mut p: *mut SorterRecord = ::core::ptr::null_mut::<SorterRecord>();
        let mut pNext: *mut SorterRecord = ::core::ptr::null_mut::<SorterRecord>();
        p = pRecord;
        while !p.is_null() {
            pNext = (*p).u.pNext;
            sqlite3DbFree(db, p as *mut ::core::ffi::c_void);
            p = pNext;
        }
    }
}
unsafe extern "C" fn vdbeSortSubtaskCleanup(
    mut db: *mut sqlite3,
    mut pTask: *mut SortSubtask,
) {
    unsafe {
        sqlite3DbFree(db, (*pTask).pUnpacked as *mut ::core::ffi::c_void);
        if !(*pTask).list.aMemory.is_null() {
            sqlite3_free((*pTask).list.aMemory as *mut ::core::ffi::c_void);
        } else {
            vdbeSorterRecordFree(
                ::core::ptr::null_mut::<sqlite3>(),
                (*pTask).list.pList,
            );
        }
        if !(*pTask).file.pFd.is_null() {
            sqlite3OsCloseFree((*pTask).file.pFd);
        }
        if !(*pTask).file2.pFd.is_null() {
            sqlite3OsCloseFree((*pTask).file2.pFd);
        }
        memset(
            pTask as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<SortSubtask>() as size_t,
        );
    }
}
unsafe extern "C" fn vdbeSorterJoinThread(
    mut pTask: *mut SortSubtask,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !(*pTask).pThread.is_null() {
            let mut pRet: *mut ::core::ffi::c_void = 1 as ::core::ffi::c_int as intptr_t
                as *mut ::core::ffi::c_void;
            sqlite3ThreadJoin((*pTask).pThread, &raw mut pRet);
            rc = pRet as intptr_t as ::core::ffi::c_int;
            (*pTask).bDone = 0 as ::core::ffi::c_int;
            (*pTask).pThread = ::core::ptr::null_mut::<SQLiteThread>();
        }
        return rc;
    }
}
unsafe extern "C" fn vdbeSorterCreateThread(
    mut pTask: *mut SortSubtask,
    mut xTask: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    >,
    mut pIn: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3ThreadCreate(&raw mut (*pTask).pThread, xTask, pIn);
    }
}
unsafe extern "C" fn vdbeSorterJoinAll(
    mut pSorter: *mut VdbeSorter,
    mut rcin: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = rcin;
        let mut i: ::core::ffi::c_int = 0;
        i = (*pSorter).nTask as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            let mut pTask: *mut SortSubtask = (&raw mut (*pSorter).aTask
                as *mut SortSubtask)
                .offset(i as isize) as *mut SortSubtask;
            let mut rc2: ::core::ffi::c_int = vdbeSorterJoinThread(pTask);
            if rc == SQLITE_OK {
                rc = rc2;
            }
            i -= 1;
        }
        return rc;
    }
}
unsafe extern "C" fn vdbeMergeEngineNew(
    mut nReader: ::core::ffi::c_int,
) -> *mut MergeEngine {
    unsafe {
        let mut N: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
        let mut nByte: i64_0 = 0;
        let mut pNew: *mut MergeEngine = ::core::ptr::null_mut::<MergeEngine>();
        while N < nReader {
            N += N;
        }
        nByte = (::core::mem::size_of::<MergeEngine>() as usize)
            .wrapping_add(
                (N as usize)
                    .wrapping_mul(
                        (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                            .wrapping_add(::core::mem::size_of::<PmaReader>() as usize),
                    ),
            ) as i64_0;
        pNew = if sqlite3FaultSim(100 as ::core::ffi::c_int) != 0 {
            ::core::ptr::null_mut::<MergeEngine>()
        } else {
            sqlite3MallocZero(nByte as u64_0) as *mut MergeEngine
        };
        if !pNew.is_null() {
            (*pNew).nTree = N;
            (*pNew).pTask = ::core::ptr::null_mut::<SortSubtask>();
            (*pNew).aReadr = pNew.offset(1 as ::core::ffi::c_int as isize)
                as *mut MergeEngine as *mut PmaReader;
            (*pNew).aTree = (*pNew).aReadr.offset(N as isize) as *mut PmaReader
                as *mut ::core::ffi::c_int;
        }
        return pNew;
    }
}
unsafe extern "C" fn vdbeMergeEngineFree(mut pMerger: *mut MergeEngine) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if !pMerger.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*pMerger).nTree {
                vdbePmaReaderClear(
                    (*pMerger).aReadr.offset(i as isize) as *mut PmaReader,
                );
                i += 1;
            }
        }
        sqlite3_free(pMerger as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn vdbeIncrFree(mut pIncr: *mut IncrMerger) {
    unsafe {
        if !pIncr.is_null() {
            if (*pIncr).bUseThread != 0 {
                vdbeSorterJoinThread((*pIncr).pTask);
                if !(*pIncr).aFile[0 as ::core::ffi::c_int as usize].pFd.is_null() {
                    sqlite3OsCloseFree(
                        (*pIncr).aFile[0 as ::core::ffi::c_int as usize].pFd,
                    );
                }
                if !(*pIncr).aFile[1 as ::core::ffi::c_int as usize].pFd.is_null() {
                    sqlite3OsCloseFree(
                        (*pIncr).aFile[1 as ::core::ffi::c_int as usize].pFd,
                    );
                }
            }
            vdbeMergeEngineFree((*pIncr).pMerger);
            sqlite3_free(pIncr as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSorterReset(
    mut db: *mut sqlite3,
    mut pSorter: *mut VdbeSorter,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        vdbeSorterJoinAll(pSorter, SQLITE_OK);
        if !(*pSorter).pReader.is_null() {
            vdbePmaReaderClear((*pSorter).pReader);
            sqlite3DbFree(db, (*pSorter).pReader as *mut ::core::ffi::c_void);
            (*pSorter).pReader = ::core::ptr::null_mut::<PmaReader>();
        }
        vdbeMergeEngineFree((*pSorter).pMerger);
        (*pSorter).pMerger = ::core::ptr::null_mut::<MergeEngine>();
        i = 0 as ::core::ffi::c_int;
        while i < (*pSorter).nTask as ::core::ffi::c_int {
            let mut pTask: *mut SortSubtask = (&raw mut (*pSorter).aTask
                as *mut SortSubtask)
                .offset(i as isize) as *mut SortSubtask;
            vdbeSortSubtaskCleanup(db, pTask);
            (*pTask).pSorter = pSorter;
            i += 1;
        }
        if (*pSorter).list.aMemory.is_null() {
            vdbeSorterRecordFree(
                ::core::ptr::null_mut::<sqlite3>(),
                (*pSorter).list.pList,
            );
        }
        (*pSorter).list.pList = ::core::ptr::null_mut::<SorterRecord>();
        (*pSorter).list.szPMA = 0 as i64_0;
        (*pSorter).bUsePMA = 0 as u8_0;
        (*pSorter).iMemory = 0 as ::core::ffi::c_int;
        (*pSorter).mxKeysize = 0 as ::core::ffi::c_int;
        sqlite3DbFree(db, (*pSorter).pUnpacked as *mut ::core::ffi::c_void);
        (*pSorter).pUnpacked = ::core::ptr::null_mut::<UnpackedRecord>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSorterClose(
    mut db: *mut sqlite3,
    mut pCsr: *mut VdbeCursor,
) {
    unsafe {
        let mut pSorter: *mut VdbeSorter = ::core::ptr::null_mut::<VdbeSorter>();
        pSorter = (*pCsr).uc.pSorter;
        if !pSorter.is_null() {
            let mut ii: ::core::ffi::c_int = 0;
            ii = 0 as ::core::ffi::c_int;
            while ii < (*pSorter).nTask as ::core::ffi::c_int {
                (*db).nSpill = ((*db).nSpill as ::core::ffi::c_ulonglong)
                    .wrapping_add(
                        (*(&raw mut (*pSorter).aTask as *mut SortSubtask)
                            .offset(ii as isize))
                            .nSpill as ::core::ffi::c_ulonglong,
                    ) as u64_0 as u64_0;
                ii += 1;
            }
            sqlite3VdbeSorterReset(db, pSorter);
            sqlite3_free((*pSorter).list.aMemory as *mut ::core::ffi::c_void);
            sqlite3DbFree(db, pSorter as *mut ::core::ffi::c_void);
            (*pCsr).uc.pSorter = ::core::ptr::null_mut::<VdbeSorter>();
        }
    }
}
unsafe extern "C" fn vdbeSorterExtendFile(
    mut db: *mut sqlite3,
    mut pFd: *mut sqlite3_file,
    mut nByte: i64_0,
) {
    unsafe {
        if nByte <= (*db).nMaxSorterMmap as i64_0
            && (*(*pFd).pMethods).iVersion >= 3 as ::core::ffi::c_int
        {
            let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            let mut chunksize: ::core::ffi::c_int = 4 as ::core::ffi::c_int
                * 1024 as ::core::ffi::c_int;
            sqlite3OsFileControlHint(
                pFd,
                SQLITE_FCNTL_CHUNK_SIZE,
                &raw mut chunksize as *mut ::core::ffi::c_void,
            );
            sqlite3OsFileControlHint(
                pFd,
                SQLITE_FCNTL_SIZE_HINT,
                &raw mut nByte as *mut ::core::ffi::c_void,
            );
            sqlite3OsFetch(pFd, 0 as i64_0, nByte as ::core::ffi::c_int, &raw mut p);
            if !p.is_null() {
                sqlite3OsUnfetch(pFd, 0 as i64_0, p);
            }
        }
    }
}
unsafe extern "C" fn vdbeSorterOpenTempFile(
    mut db: *mut sqlite3,
    mut nExtend: i64_0,
    mut ppFd: *mut *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if sqlite3FaultSim(202 as ::core::ffi::c_int) != 0 {
            return SQLITE_IOERR_ACCESS;
        }
        rc = sqlite3OsOpenMalloc(
            (*db).pVfs,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ppFd,
            SQLITE_OPEN_TEMP_JOURNAL | SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE
                | SQLITE_OPEN_EXCLUSIVE | SQLITE_OPEN_DELETEONCLOSE,
            &raw mut rc,
        );
        if rc == SQLITE_OK {
            let mut max: i64_0 = SQLITE_MAX_MMAP_SIZE as i64_0;
            sqlite3OsFileControlHint(
                *ppFd,
                SQLITE_FCNTL_MMAP_SIZE,
                &raw mut max as *mut ::core::ffi::c_void,
            );
            if nExtend > 0 as ::core::ffi::c_longlong {
                vdbeSorterExtendFile(db, *ppFd, nExtend);
            }
        }
        return rc;
    }
}
unsafe extern "C" fn vdbeSortAllocUnpacked(
    mut pTask: *mut SortSubtask,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pTask).pUnpacked.is_null() {
            (*pTask).pUnpacked = sqlite3VdbeAllocUnpackedRecord(
                (*(*pTask).pSorter).pKeyInfo,
            );
            if (*pTask).pUnpacked.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            (*(*pTask).pUnpacked).nField = (*(*(*pTask).pSorter).pKeyInfo).nKeyField;
            (*(*pTask).pUnpacked).errCode = 0 as u8_0;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn vdbeSorterMerge(
    mut pTask: *mut SortSubtask,
    mut p1: *mut SorterRecord,
    mut p2: *mut SorterRecord,
) -> *mut SorterRecord {
    unsafe {
        let mut pFinal: *mut SorterRecord = ::core::ptr::null_mut::<SorterRecord>();
        let mut pp: *mut *mut SorterRecord = &raw mut pFinal;
        let mut bCached: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            let mut res: ::core::ffi::c_int = 0;
            res = (*pTask)
                .xCompare
                .expect(
                    "non-null function pointer",
                )(
                pTask,
                &raw mut bCached,
                p1.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                (*p1).nVal,
                p2.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                (*p2).nVal,
            );
            if res <= 0 as ::core::ffi::c_int {
                *pp = p1;
                pp = &raw mut (*p1).u.pNext;
                p1 = (*p1).u.pNext;
                if !p1.is_null() {
                    continue;
                }
                *pp = p2;
                break;
            } else {
                *pp = p2;
                pp = &raw mut (*p2).u.pNext;
                p2 = (*p2).u.pNext;
                bCached = 0 as ::core::ffi::c_int;
                if !p2.is_null() {
                    continue;
                }
                *pp = p1;
                break;
            }
        }
        return pFinal;
    }
}
unsafe extern "C" fn vdbeSorterGetCompare(mut p: *mut VdbeSorter) -> SorterCompare {
    unsafe {
        if (*p).typeMask as ::core::ffi::c_int == SORTER_TYPE_INTEGER {
            return Some(
                vdbeSorterCompareInt
                    as unsafe extern "C" fn(
                        *mut SortSubtask,
                        *mut ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        } else if (*p).typeMask as ::core::ffi::c_int == SORTER_TYPE_TEXT {
            return Some(
                vdbeSorterCompareText
                    as unsafe extern "C" fn(
                        *mut SortSubtask,
                        *mut ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        }
        return Some(
            vdbeSorterCompare
                as unsafe extern "C" fn(
                    *mut SortSubtask,
                    *mut ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn vdbeSorterSort(
    mut pTask: *mut SortSubtask,
    mut pList: *mut SorterList,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut p: *mut SorterRecord = ::core::ptr::null_mut::<SorterRecord>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut aSlot: [*mut SorterRecord; 64] = [::core::ptr::null_mut::<
            SorterRecord,
        >(); 64];
        rc = vdbeSortAllocUnpacked(pTask);
        if rc != SQLITE_OK {
            return rc;
        }
        p = (*pList).pList;
        (*pTask).xCompare = vdbeSorterGetCompare((*pTask).pSorter);
        memset(
            &raw mut aSlot as *mut *mut SorterRecord as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[*mut SorterRecord; 64]>() as size_t,
        );
        while !p.is_null() {
            let mut pNext: *mut SorterRecord = ::core::ptr::null_mut::<SorterRecord>();
            if !(*pList).aMemory.is_null() {
                if p as *mut u8_0 == (*pList).aMemory {
                    pNext = ::core::ptr::null_mut::<SorterRecord>();
                } else {
                    pNext = (*pList).aMemory.offset((*p).u.iNext as isize) as *mut u8_0
                        as *mut SorterRecord;
                }
            } else {
                pNext = (*p).u.pNext;
            }
            (*p).u.pNext = ::core::ptr::null_mut::<SorterRecord>();
            i = 0 as ::core::ffi::c_int;
            while !aSlot[i as usize].is_null() {
                p = vdbeSorterMerge(pTask, p, aSlot[i as usize]);
                aSlot[i as usize] = ::core::ptr::null_mut::<SorterRecord>();
                i += 1;
            }
            aSlot[i as usize] = p;
            p = pNext;
        }
        p = ::core::ptr::null_mut::<SorterRecord>();
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[*mut SorterRecord; 64]>() as usize)
                .wrapping_div(::core::mem::size_of::<*mut SorterRecord>() as usize)
                as ::core::ffi::c_int
        {
            if !aSlot[i as usize].is_null() {
                p = if !p.is_null() {
                    vdbeSorterMerge(pTask, p, aSlot[i as usize])
                } else {
                    aSlot[i as usize]
                };
            }
            i += 1;
        }
        (*pList).pList = p;
        return (*(*pTask).pUnpacked).errCode as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn vdbePmaWriterInit(
    mut pFd: *mut sqlite3_file,
    mut p: *mut PmaWriter,
    mut nBuf: ::core::ffi::c_int,
    mut iStart: i64_0,
) {
    unsafe {
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<PmaWriter>() as size_t,
        );
        (*p).aBuffer = sqlite3Malloc(nBuf as u64_0) as *mut u8_0;
        if (*p).aBuffer.is_null() {
            (*p).eFWErr = SQLITE_NOMEM_BKPT;
        } else {
            (*p).iBufStart = (iStart as ::core::ffi::c_longlong
                % nBuf as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            (*p).iBufEnd = (*p).iBufStart;
            (*p).iWriteOff = (iStart as ::core::ffi::c_longlong
                - (*p).iBufStart as ::core::ffi::c_longlong) as i64_0;
            (*p).nBuffer = nBuf;
            (*p).pFd = pFd;
        };
    }
}
unsafe extern "C" fn vdbePmaWriteBlob(
    mut p: *mut PmaWriter,
    mut pData: *mut u8_0,
    mut nData: ::core::ffi::c_int,
) {
    unsafe {
        let mut nRem: ::core::ffi::c_int = nData;
        while nRem > 0 as ::core::ffi::c_int && (*p).eFWErr == 0 as ::core::ffi::c_int {
            let mut nCopy: ::core::ffi::c_int = nRem;
            if nCopy > (*p).nBuffer - (*p).iBufEnd {
                nCopy = (*p).nBuffer - (*p).iBufEnd;
            }
            memcpy(
                (*p).aBuffer.offset((*p).iBufEnd as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void,
                pData.offset((nData - nRem) as isize) as *mut u8_0
                    as *const ::core::ffi::c_void,
                nCopy as size_t,
            );
            (*p).iBufEnd += nCopy;
            if (*p).iBufEnd == (*p).nBuffer {
                (*p).eFWErr = sqlite3OsWrite(
                    (*p).pFd,
                    (*p).aBuffer.offset((*p).iBufStart as isize) as *mut u8_0
                        as *const ::core::ffi::c_void,
                    (*p).iBufEnd - (*p).iBufStart,
                    (*p).iWriteOff + (*p).iBufStart as i64_0,
                );
                (*p).nPmaSpill = ((*p).nPmaSpill as ::core::ffi::c_ulonglong)
                    .wrapping_add(
                        ((*p).iBufEnd - (*p).iBufStart) as ::core::ffi::c_ulonglong,
                    ) as u64_0 as u64_0;
                (*p).iBufEnd = 0 as ::core::ffi::c_int;
                (*p).iBufStart = (*p).iBufEnd;
                (*p).iWriteOff += (*p).nBuffer as ::core::ffi::c_longlong;
            }
            nRem -= nCopy;
        }
    }
}
unsafe extern "C" fn vdbePmaWriterFinish(
    mut p: *mut PmaWriter,
    mut piEof: *mut i64_0,
    mut pnSpill: *mut u64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if (*p).eFWErr == 0 as ::core::ffi::c_int && !(*p).aBuffer.is_null()
            && (*p).iBufEnd > (*p).iBufStart
        {
            (*p).eFWErr = sqlite3OsWrite(
                (*p).pFd,
                (*p).aBuffer.offset((*p).iBufStart as isize) as *mut u8_0
                    as *const ::core::ffi::c_void,
                (*p).iBufEnd - (*p).iBufStart,
                (*p).iWriteOff + (*p).iBufStart as i64_0,
            );
            (*p).nPmaSpill = ((*p).nPmaSpill as ::core::ffi::c_ulonglong)
                .wrapping_add(
                    ((*p).iBufEnd - (*p).iBufStart) as ::core::ffi::c_ulonglong,
                ) as u64_0 as u64_0;
        }
        *piEof = ((*p).iWriteOff as ::core::ffi::c_longlong
            + (*p).iBufEnd as ::core::ffi::c_longlong) as i64_0;
        *pnSpill = (*pnSpill as ::core::ffi::c_ulonglong)
            .wrapping_add((*p).nPmaSpill as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
        sqlite3_free((*p).aBuffer as *mut ::core::ffi::c_void);
        rc = (*p).eFWErr;
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<PmaWriter>() as size_t,
        );
        return rc;
    }
}
unsafe extern "C" fn vdbePmaWriteVarint(mut p: *mut PmaWriter, mut iVal: u64_0) {
    unsafe {
        let mut nByte: ::core::ffi::c_int = 0;
        let mut aByte: [u8_0; 10] = [0; 10];
        nByte = sqlite3PutVarint(&raw mut aByte as *mut ::core::ffi::c_uchar, iVal);
        vdbePmaWriteBlob(p, &raw mut aByte as *mut u8_0, nByte);
    }
}
unsafe extern "C" fn vdbeSorterListToPMA(
    mut pTask: *mut SortSubtask,
    mut pList: *mut SorterList,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = (*(*pTask).pSorter).db;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut writer: PmaWriter = PmaWriter {
            eFWErr: 0,
            aBuffer: ::core::ptr::null_mut::<u8_0>(),
            nBuffer: 0,
            iBufStart: 0,
            iBufEnd: 0,
            iWriteOff: 0,
            pFd: ::core::ptr::null_mut::<sqlite3_file>(),
            nPmaSpill: 0

        };
        memset(
            &raw mut writer as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<PmaWriter>() as size_t,
        );
        if (*pTask).file.pFd.is_null() {
            rc = vdbeSorterOpenTempFile(db, 0 as i64_0, &raw mut (*pTask).file.pFd);
        }
        if rc == SQLITE_OK {
            vdbeSorterExtendFile(
                db,
                (*pTask).file.pFd,
                (*pTask).file.iEof + (*pList).szPMA + 9 as i64_0,
            );
        }
        if rc == SQLITE_OK {
            rc = vdbeSorterSort(pTask, pList);
        }
        if rc == SQLITE_OK {
            let mut p: *mut SorterRecord = ::core::ptr::null_mut::<SorterRecord>();
            let mut pNext: *mut SorterRecord = ::core::ptr::null_mut::<SorterRecord>();
            vdbePmaWriterInit(
                (*pTask).file.pFd,
                &raw mut writer,
                (*(*pTask).pSorter).pgsz,
                (*pTask).file.iEof,
            );
            (*pTask).nPMA += 1;
            vdbePmaWriteVarint(&raw mut writer, (*pList).szPMA as u64_0);
            p = (*pList).pList;
            while !p.is_null() {
                pNext = (*p).u.pNext;
                vdbePmaWriteVarint(&raw mut writer, (*p).nVal as u64_0);
                vdbePmaWriteBlob(
                    &raw mut writer,
                    p.offset(1 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void as *mut u8_0,
                    (*p).nVal,
                );
                if (*pList).aMemory.is_null() {
                    sqlite3_free(p as *mut ::core::ffi::c_void);
                }
                p = pNext;
            }
            (*pList).pList = p;
            rc = vdbePmaWriterFinish(
                &raw mut writer,
                &raw mut (*pTask).file.iEof,
                &raw mut (*pTask).nSpill,
            );
        }
        return rc;
    }
}
unsafe extern "C" fn vdbeMergeEngineStep(
    mut pMerger: *mut MergeEngine,
    mut pbEof: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut iPrev: ::core::ffi::c_int = *(*pMerger)
            .aTree
            .offset(1 as ::core::ffi::c_int as isize);
        let mut pTask: *mut SortSubtask = (*pMerger).pTask;
        rc = vdbePmaReaderNext(
            (*pMerger).aReadr.offset(iPrev as isize) as *mut PmaReader,
        );
        if rc == SQLITE_OK {
            let mut i: ::core::ffi::c_int = 0;
            let mut pReadr1: *mut PmaReader = ::core::ptr::null_mut::<PmaReader>();
            let mut pReadr2: *mut PmaReader = ::core::ptr::null_mut::<PmaReader>();
            let mut bCached: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            pReadr1 = (*pMerger)
                .aReadr
                .offset((iPrev & 0xfffe as ::core::ffi::c_int) as isize)
                as *mut PmaReader;
            pReadr2 = (*pMerger)
                .aReadr
                .offset((iPrev | 0x1 as ::core::ffi::c_int) as isize) as *mut PmaReader;
            i = ((*pMerger).nTree + iPrev) / 2 as ::core::ffi::c_int;
            while i > 0 as ::core::ffi::c_int {
                let mut iRes: ::core::ffi::c_int = 0;
                if (*pReadr1).pFd.is_null() {
                    iRes = 1 as ::core::ffi::c_int;
                } else if (*pReadr2).pFd.is_null() {
                    iRes = -1 as ::core::ffi::c_int;
                } else {
                    iRes = (*pTask)
                        .xCompare
                        .expect(
                            "non-null function pointer",
                        )(
                        pTask,
                        &raw mut bCached,
                        (*pReadr1).aKey as *const ::core::ffi::c_void,
                        (*pReadr1).nKey,
                        (*pReadr2).aKey as *const ::core::ffi::c_void,
                        (*pReadr2).nKey,
                    );
                }
                if iRes < 0 as ::core::ffi::c_int
                    || iRes == 0 as ::core::ffi::c_int && pReadr1 < pReadr2
                {
                    *(*pMerger).aTree.offset(i as isize) = pReadr1
                        .offset_from((*pMerger).aReadr) as ::core::ffi::c_long
                        as ::core::ffi::c_int;
                    pReadr2 = (*pMerger)
                        .aReadr
                        .offset(
                            *(*pMerger)
                                .aTree
                                .offset((i ^ 0x1 as ::core::ffi::c_int) as isize) as isize,
                        ) as *mut PmaReader;
                    bCached = 0 as ::core::ffi::c_int;
                } else {
                    if !(*pReadr1).pFd.is_null() {
                        bCached = 0 as ::core::ffi::c_int;
                    }
                    *(*pMerger).aTree.offset(i as isize) = pReadr2
                        .offset_from((*pMerger).aReadr) as ::core::ffi::c_long
                        as ::core::ffi::c_int;
                    pReadr1 = (*pMerger)
                        .aReadr
                        .offset(
                            *(*pMerger)
                                .aTree
                                .offset((i ^ 0x1 as ::core::ffi::c_int) as isize) as isize,
                        ) as *mut PmaReader;
                }
                i = i / 2 as ::core::ffi::c_int;
            }
            *pbEof = (*(*pMerger)
                .aReadr
                .offset(
                    *(*pMerger).aTree.offset(1 as ::core::ffi::c_int as isize) as isize,
                ))
                .pFd
                .is_null() as ::core::ffi::c_int;
        }
        return if rc == SQLITE_OK {
            (*(*pTask).pUnpacked).errCode as ::core::ffi::c_int
        } else {
            rc
        };
    }
}
unsafe extern "C" fn vdbeSorterFlushThread(
    mut pCtx: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pTask: *mut SortSubtask = pCtx as *mut SortSubtask;
        let mut rc: ::core::ffi::c_int = 0;
        rc = vdbeSorterListToPMA(pTask, &raw mut (*pTask).list);
        (*pTask).bDone = 1 as ::core::ffi::c_int;
        return rc as intptr_t as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn vdbeSorterFlushPMA(
    mut pSorter: *mut VdbeSorter,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut i: ::core::ffi::c_int = 0;
        let mut pTask: *mut SortSubtask = ::core::ptr::null_mut::<SortSubtask>();
        let mut nWorker: ::core::ffi::c_int = (*pSorter).nTask as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
        (*pSorter).bUsePMA = 1 as u8_0;
        i = 0 as ::core::ffi::c_int;
        while i < nWorker {
            let mut iTest: ::core::ffi::c_int = ((*pSorter).iPrev as ::core::ffi::c_int
                + i + 1 as ::core::ffi::c_int) % nWorker;
            pTask = (&raw mut (*pSorter).aTask as *mut SortSubtask)
                .offset(iTest as isize) as *mut SortSubtask;
            if (*pTask).bDone != 0 {
                rc = vdbeSorterJoinThread(pTask);
            }
            if rc != SQLITE_OK || (*pTask).pThread.is_null() {
                break;
            }
            i += 1;
        }
        if rc == SQLITE_OK {
            if i == nWorker {
                rc = vdbeSorterListToPMA(
                    (&raw mut (*pSorter).aTask as *mut SortSubtask)
                        .offset(nWorker as isize) as *mut SortSubtask,
                    &raw mut (*pSorter).list,
                );
            } else {
                let mut aMem: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                let mut pCtx: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                aMem = (*pTask).list.aMemory;
                pCtx = pTask as *mut ::core::ffi::c_void;
                (*pSorter).iPrev = pTask
                    .offset_from(&raw mut (*pSorter).aTask as *mut SortSubtask)
                    as ::core::ffi::c_long as u8_0;
                (*pTask).list = (*pSorter).list;
                (*pSorter).list.pList = ::core::ptr::null_mut::<SorterRecord>();
                (*pSorter).list.szPMA = 0 as i64_0;
                if !aMem.is_null() {
                    (*pSorter).list.aMemory = aMem;
                    (*pSorter).nMemory = sqlite3MallocSize(
                        aMem as *const ::core::ffi::c_void,
                    );
                } else if !(*pSorter).list.aMemory.is_null() {
                    (*pSorter).list.aMemory = sqlite3Malloc((*pSorter).nMemory as u64_0)
                        as *mut u8_0;
                    if (*pSorter).list.aMemory.is_null() {
                        return SQLITE_NOMEM_BKPT;
                    }
                }
                rc = vdbeSorterCreateThread(
                    pTask,
                    Some(
                        vdbeSorterFlushThread
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                            ) -> *mut ::core::ffi::c_void,
                    ),
                    pCtx,
                );
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSorterWrite(
    mut pCsr: *const VdbeCursor,
    mut pVal: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSorter: *mut VdbeSorter = ::core::ptr::null_mut::<VdbeSorter>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pNew: *mut SorterRecord = ::core::ptr::null_mut::<SorterRecord>();
        let mut bFlush: ::core::ffi::c_int = 0;
        let mut nReq: i64_0 = 0;
        let mut nPMA: i64_0 = 0;
        let mut t: ::core::ffi::c_int = 0;
        pSorter = (*pCsr).uc.pSorter;
        t = *((*pVal).z.offset(1 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as *const u8_0) as u32_0 as ::core::ffi::c_int;
        if t >= 0x80 as ::core::ffi::c_int {
            sqlite3GetVarint32(
                (*pVal).z.offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char as *const ::core::ffi::c_uchar,
                &raw mut t as *mut u32_0,
            );
        }
        if t > 0 as ::core::ffi::c_int && t < 10 as ::core::ffi::c_int
            && t != 7 as ::core::ffi::c_int
        {
            (*pSorter).typeMask = ((*pSorter).typeMask as ::core::ffi::c_int
                & SORTER_TYPE_INTEGER) as u8_0;
        } else if t > 10 as ::core::ffi::c_int && t & 0x1 as ::core::ffi::c_int != 0 {
            (*pSorter).typeMask = ((*pSorter).typeMask as ::core::ffi::c_int
                & SORTER_TYPE_TEXT) as u8_0;
        } else {
            (*pSorter).typeMask = 0 as u8_0;
        }
        nReq = ((*pVal).n as usize)
            .wrapping_add(::core::mem::size_of::<SorterRecord>() as usize) as i64_0;
        nPMA = ((*pVal).n + sqlite3VarintLen((*pVal).n as u64_0)) as i64_0;
        if (*pSorter).mxPmaSize != 0 {
            if !(*pSorter).list.aMemory.is_null() {
                bFlush = ((*pSorter).iMemory != 0
                    && (*pSorter).iMemory as i64_0 + nReq
                        > (*pSorter).mxPmaSize as ::core::ffi::c_longlong)
                    as ::core::ffi::c_int;
            } else {
                bFlush = ((*pSorter).list.szPMA
                    > (*pSorter).mxPmaSize as ::core::ffi::c_longlong
                    || (*pSorter).list.szPMA
                        > (*pSorter).mnPmaSize as ::core::ffi::c_longlong
                        && sqlite3HeapNearlyFull() != 0) as ::core::ffi::c_int;
            }
            if bFlush != 0 {
                rc = vdbeSorterFlushPMA(pSorter);
                (*pSorter).list.szPMA = 0 as i64_0;
                (*pSorter).iMemory = 0 as ::core::ffi::c_int;
            }
        }
        (*pSorter).list.szPMA += nPMA as ::core::ffi::c_longlong;
        if nPMA > (*pSorter).mxKeysize as ::core::ffi::c_longlong {
            (*pSorter).mxKeysize = nPMA as ::core::ffi::c_int;
        }
        if !(*pSorter).list.aMemory.is_null() {
            let mut nMin: ::core::ffi::c_int = ((*pSorter).iMemory as i64_0 + nReq)
                as ::core::ffi::c_int;
            if nMin > (*pSorter).nMemory {
                let mut aNew: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                let mut nNew: sqlite3_int64 = 2 as sqlite3_int64
                    * (*pSorter).nMemory as sqlite3_int64;
                let mut iListOff: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                if !(*pSorter).list.pList.is_null() {
                    iListOff = ((*pSorter).list.pList as *mut u8_0)
                        .offset_from((*pSorter).list.aMemory) as ::core::ffi::c_long
                        as ::core::ffi::c_int;
                }
                while nNew < nMin as ::core::ffi::c_longlong {
                    nNew = (nNew as ::core::ffi::c_longlong
                        * 2 as ::core::ffi::c_longlong) as sqlite3_int64;
                }
                if nNew > (*pSorter).mxPmaSize as ::core::ffi::c_longlong {
                    nNew = (*pSorter).mxPmaSize as sqlite3_int64;
                }
                if nNew < nMin as ::core::ffi::c_longlong {
                    nNew = nMin as sqlite3_int64;
                }
                aNew = sqlite3Realloc(
                    (*pSorter).list.aMemory as *mut ::core::ffi::c_void,
                    nNew as u64_0,
                ) as *mut u8_0;
                if aNew.is_null() {
                    return SQLITE_NOMEM_BKPT;
                }
                if iListOff >= 0 as ::core::ffi::c_int {
                    (*pSorter).list.pList = aNew.offset(iListOff as isize) as *mut u8_0
                        as *mut SorterRecord;
                }
                (*pSorter).list.aMemory = aNew;
                (*pSorter).nMemory = nNew as ::core::ffi::c_int;
            }
            pNew = (*pSorter).list.aMemory.offset((*pSorter).iMemory as isize)
                as *mut u8_0 as *mut SorterRecord;
            (*pSorter).iMemory = ((*pSorter).iMemory as ::core::ffi::c_longlong
                + (nReq as ::core::ffi::c_longlong + 7 as ::core::ffi::c_longlong
                    & !(7 as ::core::ffi::c_int) as ::core::ffi::c_longlong))
                as ::core::ffi::c_int;
            if !(*pSorter).list.pList.is_null() {
                (*pNew).u.iNext = ((*pSorter).list.pList as *mut u8_0)
                    .offset_from((*pSorter).list.aMemory) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
            }
        } else {
            pNew = sqlite3Malloc(nReq as u64_0) as *mut SorterRecord;
            if pNew.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            (*pNew).u.pNext = (*pSorter).list.pList;
        }
        memcpy(
            pNew.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (*pVal).z as *const ::core::ffi::c_void,
            (*pVal).n as size_t,
        );
        (*pNew).nVal = (*pVal).n;
        (*pSorter).list.pList = pNew;
        return rc;
    }
}
unsafe extern "C" fn vdbeIncrPopulate(mut pIncr: *mut IncrMerger) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut rc2: ::core::ffi::c_int = 0;
        let mut iStart: i64_0 = (*pIncr).iStartOff;
        let mut pOut: *mut SorterFile = (&raw mut (*pIncr).aFile as *mut SorterFile)
            .offset(1 as ::core::ffi::c_int as isize) as *mut SorterFile;
        let mut pTask: *mut SortSubtask = (*pIncr).pTask;
        let mut pMerger: *mut MergeEngine = (*pIncr).pMerger;
        let mut writer: PmaWriter = PmaWriter {
            eFWErr: 0,
            aBuffer: ::core::ptr::null_mut::<u8_0>(),
            nBuffer: 0,
            iBufStart: 0,
            iBufEnd: 0,
            iWriteOff: 0,
            pFd: ::core::ptr::null_mut::<sqlite3_file>(),
            nPmaSpill: 0

        };
        vdbePmaWriterInit(
            (*pOut).pFd,
            &raw mut writer,
            (*(*pTask).pSorter).pgsz,
            iStart,
        );
        while rc == SQLITE_OK {
            let mut dummy: ::core::ffi::c_int = 0;
            let mut pReader: *mut PmaReader = (*pMerger)
                .aReadr
                .offset(
                    *(*pMerger).aTree.offset(1 as ::core::ffi::c_int as isize) as isize,
                ) as *mut PmaReader;
            let mut nKey: ::core::ffi::c_int = (*pReader).nKey;
            let mut iEof: i64_0 = writer.iWriteOff + writer.iBufEnd as i64_0;
            if (*pReader).pFd.is_null() {
                break;
            }
            if iEof as ::core::ffi::c_longlong + nKey as ::core::ffi::c_longlong
                + sqlite3VarintLen(nKey as u64_0) as ::core::ffi::c_longlong
                > iStart as ::core::ffi::c_longlong
                    + (*pIncr).mxSz as ::core::ffi::c_longlong
            {
                break;
            }
            vdbePmaWriteVarint(&raw mut writer, nKey as u64_0);
            vdbePmaWriteBlob(&raw mut writer, (*pReader).aKey, nKey);
            rc = vdbeMergeEngineStep((*pIncr).pMerger, &raw mut dummy);
        }
        rc2 = vdbePmaWriterFinish(
            &raw mut writer,
            &raw mut (*pOut).iEof,
            &raw mut (*pTask).nSpill,
        );
        if rc == SQLITE_OK {
            rc = rc2;
        }
        return rc;
    }
}
unsafe extern "C" fn vdbeIncrPopulateThread(
    mut pCtx: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pIncr: *mut IncrMerger = pCtx as *mut IncrMerger;
        let mut pRet: *mut ::core::ffi::c_void = vdbeIncrPopulate(pIncr) as intptr_t
            as *mut ::core::ffi::c_void;
        (*(*pIncr).pTask).bDone = 1 as ::core::ffi::c_int;
        return pRet;
    }
}
unsafe extern "C" fn vdbeIncrBgPopulate(
    mut pIncr: *mut IncrMerger,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut ::core::ffi::c_void = pIncr as *mut ::core::ffi::c_void;
        return vdbeSorterCreateThread(
            (*pIncr).pTask,
            Some(
                vdbeIncrPopulateThread
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                    ) -> *mut ::core::ffi::c_void,
            ),
            p,
        );
    }
}
unsafe extern "C" fn vdbeIncrSwap(mut pIncr: *mut IncrMerger) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pIncr).bUseThread != 0 {
            rc = vdbeSorterJoinThread((*pIncr).pTask);
            if rc == SQLITE_OK {
                let mut f0: SorterFile = (*pIncr)
                    .aFile[0 as ::core::ffi::c_int as usize];
                (*pIncr).aFile[0 as ::core::ffi::c_int as usize] = (*pIncr)
                    .aFile[1 as ::core::ffi::c_int as usize];
                (*pIncr).aFile[1 as ::core::ffi::c_int as usize] = f0;
            }
            if rc == SQLITE_OK {
                if (*pIncr).aFile[0 as ::core::ffi::c_int as usize].iEof
                    == (*pIncr).iStartOff
                {
                    (*pIncr).bEof = 1 as ::core::ffi::c_int;
                } else {
                    rc = vdbeIncrBgPopulate(pIncr);
                }
            }
        } else {
            rc = vdbeIncrPopulate(pIncr);
            (*pIncr).aFile[0 as ::core::ffi::c_int as usize] = (*pIncr)
                .aFile[1 as ::core::ffi::c_int as usize];
            if (*pIncr).aFile[0 as ::core::ffi::c_int as usize].iEof
                == (*pIncr).iStartOff
            {
                (*pIncr).bEof = 1 as ::core::ffi::c_int;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn vdbeIncrMergerNew(
    mut pTask: *mut SortSubtask,
    mut pMerger: *mut MergeEngine,
    mut ppOut: *mut *mut IncrMerger,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        *ppOut = (if sqlite3FaultSim(100 as ::core::ffi::c_int) != 0 {
            ::core::ptr::null_mut::<::core::ffi::c_void>()
        } else {
            sqlite3MallocZero(::core::mem::size_of::<IncrMerger>() as u64_0)
        }) as *mut IncrMerger;
        let mut pIncr: *mut IncrMerger = *ppOut;
        if !pIncr.is_null() {
            (*pIncr).pMerger = pMerger;
            (*pIncr).pTask = pTask;
            (*pIncr).mxSz = if (*(*pTask).pSorter).mxKeysize + 9 as ::core::ffi::c_int
                > (*(*pTask).pSorter).mxPmaSize / 2 as ::core::ffi::c_int
            {
                (*(*pTask).pSorter).mxKeysize + 9 as ::core::ffi::c_int
            } else {
                (*(*pTask).pSorter).mxPmaSize / 2 as ::core::ffi::c_int
            };
            (*pTask).file2.iEof += (*pIncr).mxSz as ::core::ffi::c_longlong;
        } else {
            vdbeMergeEngineFree(pMerger);
            rc = SQLITE_NOMEM_BKPT;
        }
        return rc;
    }
}
unsafe extern "C" fn vdbeIncrMergerSetThreads(mut pIncr: *mut IncrMerger) {
    unsafe {
        (*pIncr).bUseThread = 1 as ::core::ffi::c_int;
        (*(*pIncr).pTask).file2.iEof -= (*pIncr).mxSz as ::core::ffi::c_longlong;
    }
}
unsafe extern "C" fn vdbeMergeEngineCompare(
    mut pMerger: *mut MergeEngine,
    mut iOut: ::core::ffi::c_int,
) {
    unsafe {
        let mut i1: ::core::ffi::c_int = 0;
        let mut i2: ::core::ffi::c_int = 0;
        let mut iRes: ::core::ffi::c_int = 0;
        let mut p1: *mut PmaReader = ::core::ptr::null_mut::<PmaReader>();
        let mut p2: *mut PmaReader = ::core::ptr::null_mut::<PmaReader>();
        if iOut >= (*pMerger).nTree / 2 as ::core::ffi::c_int {
            i1 = (iOut - (*pMerger).nTree / 2 as ::core::ffi::c_int)
                * 2 as ::core::ffi::c_int;
            i2 = i1 + 1 as ::core::ffi::c_int;
        } else {
            i1 = *(*pMerger).aTree.offset((iOut * 2 as ::core::ffi::c_int) as isize);
            i2 = *(*pMerger)
                .aTree
                .offset(
                    (iOut * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                );
        }
        p1 = (*pMerger).aReadr.offset(i1 as isize) as *mut PmaReader;
        p2 = (*pMerger).aReadr.offset(i2 as isize) as *mut PmaReader;
        if (*p1).pFd.is_null() {
            iRes = i2;
        } else if (*p2).pFd.is_null() {
            iRes = i1;
        } else {
            let mut pTask: *mut SortSubtask = (*pMerger).pTask;
            let mut bCached: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut res: ::core::ffi::c_int = 0;
            res = (*pTask)
                .xCompare
                .expect(
                    "non-null function pointer",
                )(
                pTask,
                &raw mut bCached,
                (*p1).aKey as *const ::core::ffi::c_void,
                (*p1).nKey,
                (*p2).aKey as *const ::core::ffi::c_void,
                (*p2).nKey,
            );
            if res <= 0 as ::core::ffi::c_int {
                iRes = i1;
            } else {
                iRes = i2;
            }
        }
        *(*pMerger).aTree.offset(iOut as isize) = iRes;
    }
}
pub const INCRINIT_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INCRINIT_TASK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INCRINIT_ROOT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn vdbeMergeEngineInit(
    mut pTask: *mut SortSubtask,
    mut pMerger: *mut MergeEngine,
    mut eMode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut i: ::core::ffi::c_int = 0;
        let mut nTree: ::core::ffi::c_int = 0;
        (*pMerger).pTask = pTask;
        nTree = (*pMerger).nTree;
        i = 0 as ::core::ffi::c_int;
        while i < nTree {
            if SQLITE_MAX_WORKER_THREADS > 0 as ::core::ffi::c_int
                && eMode == INCRINIT_ROOT
            {
                rc = vdbePmaReaderNext(
                    (*pMerger)
                        .aReadr
                        .offset((nTree - i - 1 as ::core::ffi::c_int) as isize)
                        as *mut PmaReader,
                );
            } else {
                rc = vdbePmaReaderIncrInit(
                    (*pMerger).aReadr.offset(i as isize) as *mut PmaReader,
                    INCRINIT_NORMAL,
                );
            }
            if rc != SQLITE_OK {
                return rc;
            }
            i += 1;
        }
        i = (*pMerger).nTree - 1 as ::core::ffi::c_int;
        while i > 0 as ::core::ffi::c_int {
            vdbeMergeEngineCompare(pMerger, i);
            i -= 1;
        }
        return (*(*pTask).pUnpacked).errCode as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn vdbePmaReaderIncrMergeInit(
    mut pReadr: *mut PmaReader,
    mut eMode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pIncr: *mut IncrMerger = (*pReadr).pIncr;
        let mut pTask: *mut SortSubtask = (*pIncr).pTask;
        let mut db: *mut sqlite3 = (*(*pTask).pSorter).db;
        rc = vdbeMergeEngineInit(pTask, (*pIncr).pMerger, eMode);
        if rc == SQLITE_OK {
            let mut mxSz: ::core::ffi::c_int = (*pIncr).mxSz;
            if (*pIncr).bUseThread != 0 {
                rc = vdbeSorterOpenTempFile(
                    db,
                    mxSz as i64_0,
                    &raw mut (*(&raw mut (*pIncr).aFile as *mut SorterFile)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .pFd,
                );
                if rc == SQLITE_OK {
                    rc = vdbeSorterOpenTempFile(
                        db,
                        mxSz as i64_0,
                        &raw mut (*(&raw mut (*pIncr).aFile as *mut SorterFile)
                            .offset(1 as ::core::ffi::c_int as isize))
                            .pFd,
                    );
                }
            } else {
                if (*pTask).file2.pFd.is_null() {
                    rc = vdbeSorterOpenTempFile(
                        db,
                        (*pTask).file2.iEof,
                        &raw mut (*pTask).file2.pFd,
                    );
                    (*pTask).file2.iEof = 0 as i64_0;
                }
                if rc == SQLITE_OK {
                    (*pIncr).aFile[1 as ::core::ffi::c_int as usize].pFd = (*pTask)
                        .file2
                        .pFd;
                    (*pIncr).iStartOff = (*pTask).file2.iEof;
                    (*pTask).file2.iEof += mxSz as ::core::ffi::c_longlong;
                }
            }
        }
        if rc == SQLITE_OK && (*pIncr).bUseThread != 0 {
            rc = vdbeIncrPopulate(pIncr);
        }
        if rc == SQLITE_OK
            && (SQLITE_MAX_WORKER_THREADS == 0 as ::core::ffi::c_int
                || eMode != INCRINIT_TASK)
        {
            rc = vdbePmaReaderNext(pReadr);
        }
        return rc;
    }
}
unsafe extern "C" fn vdbePmaReaderBgIncrInit(
    mut pCtx: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pReader: *mut PmaReader = pCtx as *mut PmaReader;
        let mut pRet: *mut ::core::ffi::c_void = vdbePmaReaderIncrMergeInit(
            pReader,
            1 as ::core::ffi::c_int,
        ) as intptr_t as *mut ::core::ffi::c_void;
        (*(*(*pReader).pIncr).pTask).bDone = 1 as ::core::ffi::c_int;
        return pRet;
    }
}
unsafe extern "C" fn vdbePmaReaderIncrInit(
    mut pReadr: *mut PmaReader,
    mut eMode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pIncr: *mut IncrMerger = (*pReadr).pIncr;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !pIncr.is_null() {
            if (*pIncr).bUseThread != 0 {
                let mut pCtx: *mut ::core::ffi::c_void = pReadr
                    as *mut ::core::ffi::c_void;
                rc = vdbeSorterCreateThread(
                    (*pIncr).pTask,
                    Some(
                        vdbePmaReaderBgIncrInit
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                            ) -> *mut ::core::ffi::c_void,
                    ),
                    pCtx,
                );
            } else {
                rc = vdbePmaReaderIncrMergeInit(pReadr, eMode);
            }
        }
        return rc;
    }
}
unsafe extern "C" fn vdbeMergeEngineLevel0(
    mut pTask: *mut SortSubtask,
    mut nPMA: ::core::ffi::c_int,
    mut piOffset: *mut i64_0,
    mut ppOut: *mut *mut MergeEngine,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pNew: *mut MergeEngine = ::core::ptr::null_mut::<MergeEngine>();
        let mut iOff: i64_0 = *piOffset;
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        pNew = vdbeMergeEngineNew(nPMA);
        *ppOut = pNew;
        if pNew.is_null() {
            rc = SQLITE_NOMEM_BKPT;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nPMA && rc == SQLITE_OK {
            let mut nDummy: i64_0 = 0 as i64_0;
            let mut pReadr: *mut PmaReader = (*pNew).aReadr.offset(i as isize)
                as *mut PmaReader;
            rc = vdbePmaReaderInit(
                pTask,
                &raw mut (*pTask).file,
                iOff,
                pReadr,
                &raw mut nDummy,
            );
            iOff = (*pReadr).iEof;
            i += 1;
        }
        if rc != SQLITE_OK {
            vdbeMergeEngineFree(pNew);
            *ppOut = ::core::ptr::null_mut::<MergeEngine>();
        }
        *piOffset = iOff;
        return rc;
    }
}
unsafe extern "C" fn vdbeSorterTreeDepth(
    mut nPMA: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nDepth: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nDiv: i64_0 = SORTER_MAX_MERGE_COUNT as i64_0;
        while nDiv < nPMA as i64_0 {
            nDiv = (nDiv as ::core::ffi::c_longlong
                * SORTER_MAX_MERGE_COUNT as ::core::ffi::c_longlong) as i64_0;
            nDepth += 1;
        }
        return nDepth;
    }
}
unsafe extern "C" fn vdbeSorterAddToTree(
    mut pTask: *mut SortSubtask,
    mut nDepth: ::core::ffi::c_int,
    mut iSeq: ::core::ffi::c_int,
    mut pRoot: *mut MergeEngine,
    mut pLeaf: *mut MergeEngine,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nDiv: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut p: *mut MergeEngine = pRoot;
        let mut pIncr: *mut IncrMerger = ::core::ptr::null_mut::<IncrMerger>();
        rc = vdbeIncrMergerNew(pTask, pLeaf, &raw mut pIncr);
        i = 1 as ::core::ffi::c_int;
        while i < nDepth {
            nDiv = nDiv * SORTER_MAX_MERGE_COUNT;
            i += 1;
        }
        i = 1 as ::core::ffi::c_int;
        while i < nDepth && rc == SQLITE_OK {
            let mut iIter: ::core::ffi::c_int = iSeq / nDiv % SORTER_MAX_MERGE_COUNT;
            let mut pReadr: *mut PmaReader = (*p).aReadr.offset(iIter as isize)
                as *mut PmaReader;
            if (*pReadr).pIncr.is_null() {
                let mut pNew: *mut MergeEngine = vdbeMergeEngineNew(
                    SORTER_MAX_MERGE_COUNT,
                );
                if pNew.is_null() {
                    rc = SQLITE_NOMEM_BKPT;
                } else {
                    rc = vdbeIncrMergerNew(pTask, pNew, &raw mut (*pReadr).pIncr);
                }
            }
            if rc == SQLITE_OK {
                p = (*(*pReadr).pIncr).pMerger;
                nDiv = nDiv / SORTER_MAX_MERGE_COUNT;
            }
            i += 1;
        }
        if rc == SQLITE_OK {
            let ref mut c2rust_fresh3 = (*(*p)
                .aReadr
                .offset((iSeq % SORTER_MAX_MERGE_COUNT) as isize))
                .pIncr;
            *c2rust_fresh3 = pIncr;
        } else {
            vdbeIncrFree(pIncr);
        }
        return rc;
    }
}
unsafe extern "C" fn vdbeSorterMergeTreeBuild(
    mut pSorter: *mut VdbeSorter,
    mut ppOut: *mut *mut MergeEngine,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pMain: *mut MergeEngine = ::core::ptr::null_mut::<MergeEngine>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut iTask: ::core::ffi::c_int = 0;
        if (*pSorter).nTask as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            pMain = vdbeMergeEngineNew((*pSorter).nTask as ::core::ffi::c_int);
            if pMain.is_null() {
                rc = SQLITE_NOMEM_BKPT;
            }
        }
        iTask = 0 as ::core::ffi::c_int;
        while rc == SQLITE_OK && iTask < (*pSorter).nTask as ::core::ffi::c_int {
            let mut pTask: *mut SortSubtask = (&raw mut (*pSorter).aTask
                as *mut SortSubtask)
                .offset(iTask as isize) as *mut SortSubtask;
            if SQLITE_MAX_WORKER_THREADS == 0 as ::core::ffi::c_int || (*pTask).nPMA != 0
            {
                let mut pRoot: *mut MergeEngine = ::core::ptr::null_mut::<MergeEngine>();
                let mut nDepth: ::core::ffi::c_int = vdbeSorterTreeDepth((*pTask).nPMA);
                let mut iReadOff: i64_0 = 0 as i64_0;
                if (*pTask).nPMA <= SORTER_MAX_MERGE_COUNT {
                    rc = vdbeMergeEngineLevel0(
                        pTask,
                        (*pTask).nPMA,
                        &raw mut iReadOff,
                        &raw mut pRoot,
                    );
                } else {
                    let mut i: ::core::ffi::c_int = 0;
                    let mut iSeq: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    pRoot = vdbeMergeEngineNew(SORTER_MAX_MERGE_COUNT);
                    if pRoot.is_null() {
                        rc = SQLITE_NOMEM_BKPT;
                    }
                    i = 0 as ::core::ffi::c_int;
                    while i < (*pTask).nPMA && rc == SQLITE_OK {
                        let mut pMerger: *mut MergeEngine = ::core::ptr::null_mut::<
                            MergeEngine,
                        >();
                        let mut nReader: ::core::ffi::c_int = 0;
                        nReader = if (*pTask).nPMA - i < 16 as ::core::ffi::c_int {
                            (*pTask).nPMA - i
                        } else {
                            16 as ::core::ffi::c_int
                        };
                        rc = vdbeMergeEngineLevel0(
                            pTask,
                            nReader,
                            &raw mut iReadOff,
                            &raw mut pMerger,
                        );
                        if rc == SQLITE_OK {
                            let c2rust_fresh2 = iSeq;
                            iSeq = iSeq + 1;
                            rc = vdbeSorterAddToTree(
                                pTask,
                                nDepth,
                                c2rust_fresh2,
                                pRoot,
                                pMerger,
                            );
                        }
                        i += SORTER_MAX_MERGE_COUNT;
                    }
                }
                if rc == SQLITE_OK {
                    if !pMain.is_null() {
                        rc = vdbeIncrMergerNew(
                            pTask,
                            pRoot,
                            &raw mut (*(*pMain).aReadr.offset(iTask as isize)).pIncr,
                        );
                    } else {
                        pMain = pRoot;
                    }
                } else {
                    vdbeMergeEngineFree(pRoot);
                }
            }
            iTask += 1;
        }
        if rc != SQLITE_OK {
            vdbeMergeEngineFree(pMain);
            pMain = ::core::ptr::null_mut::<MergeEngine>();
        }
        *ppOut = pMain;
        return rc;
    }
}
unsafe extern "C" fn vdbeSorterSetupMerge(
    mut pSorter: *mut VdbeSorter,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pTask0: *mut SortSubtask = (&raw mut (*pSorter).aTask
            as *mut SortSubtask)
            .offset(0 as ::core::ffi::c_int as isize) as *mut SortSubtask;
        let mut pMain: *mut MergeEngine = ::core::ptr::null_mut::<MergeEngine>();
        let mut db: *mut sqlite3 = (*(*pTask0).pSorter).db;
        let mut i: ::core::ffi::c_int = 0;
        let mut xCompare: SorterCompare = vdbeSorterGetCompare(pSorter);
        i = 0 as ::core::ffi::c_int;
        while i < (*pSorter).nTask as ::core::ffi::c_int {
            let ref mut c2rust_fresh1 = (*(&raw mut (*pSorter).aTask as *mut SortSubtask)
                .offset(i as isize))
                .xCompare;
            *c2rust_fresh1 = xCompare;
            i += 1;
        }
        rc = vdbeSorterMergeTreeBuild(pSorter, &raw mut pMain);
        if rc == SQLITE_OK {
            if (*pSorter).bUseThreads != 0 {
                let mut iTask: ::core::ffi::c_int = 0;
                let mut pReadr: *mut PmaReader = ::core::ptr::null_mut::<PmaReader>();
                let mut pLast: *mut SortSubtask = (&raw mut (*pSorter).aTask
                    as *mut SortSubtask)
                    .offset(
                        ((*pSorter).nTask as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) as isize,
                    ) as *mut SortSubtask;
                rc = vdbeSortAllocUnpacked(pLast);
                if rc == SQLITE_OK {
                    pReadr = sqlite3DbMallocZero(
                        db,
                        ::core::mem::size_of::<PmaReader>() as u64_0,
                    ) as *mut PmaReader;
                    (*pSorter).pReader = pReadr;
                    if pReadr.is_null() {
                        rc = SQLITE_NOMEM_BKPT;
                    }
                }
                if rc == SQLITE_OK {
                    rc = vdbeIncrMergerNew(pLast, pMain, &raw mut (*pReadr).pIncr);
                    if rc == SQLITE_OK {
                        vdbeIncrMergerSetThreads((*pReadr).pIncr);
                        iTask = 0 as ::core::ffi::c_int;
                        while iTask
                            < (*pSorter).nTask as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                        {
                            let mut pIncr: *mut IncrMerger = ::core::ptr::null_mut::<
                                IncrMerger,
                            >();
                            pIncr = (*(*pMain).aReadr.offset(iTask as isize)).pIncr;
                            if !pIncr.is_null() {
                                vdbeIncrMergerSetThreads(pIncr);
                            }
                            iTask += 1;
                        }
                        iTask = 0 as ::core::ffi::c_int;
                        while rc == SQLITE_OK
                            && iTask < (*pSorter).nTask as ::core::ffi::c_int
                        {
                            let mut p: *mut PmaReader = (*pMain)
                                .aReadr
                                .offset(iTask as isize) as *mut PmaReader;
                            rc = vdbePmaReaderIncrInit(p, INCRINIT_TASK);
                            iTask += 1;
                        }
                    }
                    pMain = ::core::ptr::null_mut::<MergeEngine>();
                }
                if rc == SQLITE_OK {
                    rc = vdbePmaReaderIncrMergeInit(pReadr, INCRINIT_ROOT);
                }
            } else {
                rc = vdbeMergeEngineInit(pTask0, pMain, INCRINIT_NORMAL);
                (*pSorter).pMerger = pMain;
                pMain = ::core::ptr::null_mut::<MergeEngine>();
            }
        }
        if rc != SQLITE_OK {
            vdbeMergeEngineFree(pMain);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSorterRewind(
    mut pCsr: *const VdbeCursor,
    mut pbEof: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSorter: *mut VdbeSorter = ::core::ptr::null_mut::<VdbeSorter>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        pSorter = (*pCsr).uc.pSorter;
        if (*pSorter).bUsePMA as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if !(*pSorter).list.pList.is_null() {
                *pbEof = 0 as ::core::ffi::c_int;
                rc = vdbeSorterSort(
                    (&raw mut (*pSorter).aTask as *mut SortSubtask)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut SortSubtask,
                    &raw mut (*pSorter).list,
                );
            } else {
                *pbEof = 1 as ::core::ffi::c_int;
            }
            return rc;
        }
        rc = vdbeSorterFlushPMA(pSorter);
        rc = vdbeSorterJoinAll(pSorter, rc);
        if rc == SQLITE_OK {
            rc = vdbeSorterSetupMerge(pSorter);
            *pbEof = 0 as ::core::ffi::c_int;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSorterNext(
    mut db: *mut sqlite3,
    mut pCsr: *const VdbeCursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSorter: *mut VdbeSorter = ::core::ptr::null_mut::<VdbeSorter>();
        let mut rc: ::core::ffi::c_int = 0;
        pSorter = (*pCsr).uc.pSorter;
        if (*pSorter).bUsePMA != 0 {
            if (*pSorter).bUseThreads != 0 {
                rc = vdbePmaReaderNext((*pSorter).pReader);
                if rc == SQLITE_OK && (*(*pSorter).pReader).pFd.is_null() {
                    rc = SQLITE_DONE;
                }
            } else {
                let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                rc = vdbeMergeEngineStep((*pSorter).pMerger, &raw mut res);
                if rc == SQLITE_OK && res != 0 {
                    rc = SQLITE_DONE;
                }
            }
        } else {
            let mut pFree: *mut SorterRecord = (*pSorter).list.pList;
            (*pSorter).list.pList = (*pFree).u.pNext;
            (*pFree).u.pNext = ::core::ptr::null_mut::<SorterRecord>();
            if (*pSorter).list.aMemory.is_null() {
                vdbeSorterRecordFree(db, pFree);
            }
            rc = if !(*pSorter).list.pList.is_null() { SQLITE_OK } else { SQLITE_DONE };
        }
        return rc;
    }
}
unsafe extern "C" fn vdbeSorterRowkey(
    mut pSorter: *const VdbeSorter,
    mut pnKey: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pKey: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        if (*pSorter).bUsePMA != 0 {
            let mut pReader: *mut PmaReader = ::core::ptr::null_mut::<PmaReader>();
            if (*pSorter).bUseThreads != 0 {
                pReader = (*pSorter).pReader;
            } else {
                pReader = (*(*pSorter).pMerger)
                    .aReadr
                    .offset(
                        *(*(*pSorter).pMerger)
                            .aTree
                            .offset(1 as ::core::ffi::c_int as isize) as isize,
                    ) as *mut PmaReader;
            }
            *pnKey = (*pReader).nKey;
            pKey = (*pReader).aKey as *mut ::core::ffi::c_void;
        } else {
            *pnKey = (*(*pSorter).list.pList).nVal;
            pKey = (*pSorter).list.pList.offset(1 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void;
        }
        return pKey;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSorterRowkey(
    mut pCsr: *const VdbeCursor,
    mut pOut: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSorter: *mut VdbeSorter = ::core::ptr::null_mut::<VdbeSorter>();
        let mut pKey: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut nKey: ::core::ffi::c_int = 0;
        pSorter = (*pCsr).uc.pSorter;
        pKey = vdbeSorterRowkey(pSorter, &raw mut nKey);
        if sqlite3VdbeMemClearAndResize(pOut, nKey) != 0 {
            return SQLITE_NOMEM_BKPT;
        }
        (*pOut).n = nKey;
        (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int & !(MEM_TypeMask | MEM_Zero)
            | 0x10 as ::core::ffi::c_int) as u16_0;
        memcpy((*pOut).z as *mut ::core::ffi::c_void, pKey, nKey as size_t);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSorterCompare(
    mut pCsr: *const VdbeCursor,
    mut pVal: *mut Mem,
    mut nKeyCol: ::core::ffi::c_int,
    mut pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSorter: *mut VdbeSorter = ::core::ptr::null_mut::<VdbeSorter>();
        let mut r2: *mut UnpackedRecord = ::core::ptr::null_mut::<UnpackedRecord>();
        let mut pKeyInfo: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        let mut i: ::core::ffi::c_int = 0;
        let mut pKey: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut nKey: ::core::ffi::c_int = 0;
        pSorter = (*pCsr).uc.pSorter;
        r2 = (*pSorter).pUnpacked;
        pKeyInfo = (*pCsr).pKeyInfo;
        if r2.is_null() {
            (*pSorter).pUnpacked = sqlite3VdbeAllocUnpackedRecord(pKeyInfo);
            r2 = (*pSorter).pUnpacked;
            if r2.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            (*r2).nField = nKeyCol as u16_0;
        }
        pKey = vdbeSorterRowkey(pSorter, &raw mut nKey);
        sqlite3VdbeRecordUnpack(nKey, pKey, r2);
        i = 0 as ::core::ffi::c_int;
        while i < nKeyCol {
            if (*(*r2).aMem.offset(i as isize)).flags as ::core::ffi::c_int & MEM_Null
                != 0
            {
                *pRes = -1 as ::core::ffi::c_int;
                return SQLITE_OK;
            }
            i += 1;
        }
        *pRes = sqlite3VdbeRecordCompare(
            (*pVal).n,
            (*pVal).z as *const ::core::ffi::c_void,
            r2,
        );
        return SQLITE_OK;
    }
}
