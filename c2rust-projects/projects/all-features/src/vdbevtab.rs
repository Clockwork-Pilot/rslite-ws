use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type VdbeSorter;
    pub type BtCursor;
    pub type Btree;
    pub type RenameToken;
    pub type TableLock;
    pub type VtabCtx;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_value_pointer(
        _: *mut sqlite3_value,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_create_module(
        db: *mut sqlite3,
        zName: *const ::core::ffi::c_char,
        p: *const sqlite3_module,
        pClientData: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_declare_vtab(
        _: *mut sqlite3,
        zSQL: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3VdbeNextOpcode(
        _: *mut Vdbe,
        _: *mut Mem,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut *mut Op,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeDisplayP4(_: *mut sqlite3, _: *mut Op) -> *mut ::core::ffi::c_char;
    fn sqlite3VdbeMemInit(_: *mut Mem, _: *mut sqlite3, _: u16_0);
    fn sqlite3VdbeMemSetNull(_: *mut Mem);
    fn sqlite3VdbeMemRelease(p: *mut Mem);
    fn sqlite3OpcodeName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
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
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bytecodevtab_cursor {
    pub base: sqlite3_vtab_cursor,
    pub pStmt: *mut sqlite3_stmt,
    pub iRowid: ::core::ffi::c_int,
    pub iAddr: ::core::ffi::c_int,
    pub needFinalize: ::core::ffi::c_int,
    pub showSubprograms: ::core::ffi::c_int,
    pub aOp: *mut Op,
    pub zP4: *mut ::core::ffi::c_char,
    pub zType: *const ::core::ffi::c_char,
    pub zSchema: *const ::core::ffi::c_char,
    pub zName: *const ::core::ffi::c_char,
    pub sub: Mem

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bytecodevtab {
    pub base: sqlite3_vtab,
    pub db: *mut sqlite3,
    pub bTablesUsed: ::core::ffi::c_int

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_ISNULL: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
pub const OP_OpenWrite: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
unsafe extern "C" fn bytecodevtabConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pNew: *mut bytecodevtab = ::core::ptr::null_mut::<bytecodevtab>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut isTabUsed: ::core::ffi::c_int = !pAux.is_null() as ::core::ffi::c_int;
        let mut azSchema: [*const ::core::ffi::c_char; 2] = [
            b"CREATE TABLE x(addr INT,opcode TEXT,p1 INT,p2 INT,p3 INT,p4 TEXT,p5 INT,comment TEXT,subprog TEXT,nexec INT,ncycle INT,stmt HIDDEN);\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"CREATE TABLE x(type TEXT,schema TEXT,name TEXT,wr INT,subprog TEXT,stmt HIDDEN);\0"
                .as_ptr() as *const ::core::ffi::c_char,
        ];
        rc = sqlite3_declare_vtab(db, azSchema[isTabUsed as usize]);
        if rc == SQLITE_OK {
            pNew = sqlite3_malloc(
                ::core::mem::size_of::<bytecodevtab>() as ::core::ffi::c_int,
            ) as *mut bytecodevtab;
            *ppVtab = pNew as *mut sqlite3_vtab;
            if pNew.is_null() {
                return SQLITE_NOMEM;
            }
            memset(
                pNew as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<bytecodevtab>() as size_t,
            );
            (*pNew).db = db;
            (*pNew).bTablesUsed = isTabUsed * 2 as ::core::ffi::c_int;
        }
        return rc;
    }
}
unsafe extern "C" fn bytecodevtabDisconnect(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut bytecodevtab = pVtab as *mut bytecodevtab;
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn bytecodevtabOpen(
    mut p: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVTab: *mut bytecodevtab = p as *mut bytecodevtab;
        let mut pCur: *mut bytecodevtab_cursor = ::core::ptr::null_mut::<
            bytecodevtab_cursor,
        >();
        pCur = sqlite3_malloc(
            ::core::mem::size_of::<bytecodevtab_cursor>() as ::core::ffi::c_int,
        ) as *mut bytecodevtab_cursor;
        if pCur.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pCur as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<bytecodevtab_cursor>() as size_t,
        );
        sqlite3VdbeMemInit(&raw mut (*pCur).sub, (*pVTab).db, 1 as u16_0);
        *ppCursor = &raw mut (*pCur).base;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn bytecodevtabCursorClear(mut pCur: *mut bytecodevtab_cursor) {
    unsafe {
        sqlite3_free((*pCur).zP4 as *mut ::core::ffi::c_void);
        (*pCur).zP4 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sqlite3VdbeMemRelease(&raw mut (*pCur).sub);
        sqlite3VdbeMemSetNull(&raw mut (*pCur).sub);
        if (*pCur).needFinalize != 0 {
            sqlite3_finalize((*pCur).pStmt);
        }
        (*pCur).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        (*pCur).needFinalize = 0 as ::core::ffi::c_int;
        (*pCur).zType = ::core::ptr::null::<::core::ffi::c_char>();
        (*pCur).zSchema = ::core::ptr::null::<::core::ffi::c_char>();
        (*pCur).zName = ::core::ptr::null::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn bytecodevtabClose(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut bytecodevtab_cursor = cur as *mut bytecodevtab_cursor;
        bytecodevtabCursorClear(pCur);
        sqlite3_free(pCur as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn bytecodevtabNext(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut bytecodevtab_cursor = cur as *mut bytecodevtab_cursor;
        let mut pTab: *mut bytecodevtab = (*cur).pVtab as *mut bytecodevtab;
        let mut rc: ::core::ffi::c_int = 0;
        if !(*pCur).zP4.is_null() {
            sqlite3_free((*pCur).zP4 as *mut ::core::ffi::c_void);
            (*pCur).zP4 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !(*pCur).zName.is_null() {
            (*pCur).zName = ::core::ptr::null::<::core::ffi::c_char>();
            (*pCur).zType = ::core::ptr::null::<::core::ffi::c_char>();
            (*pCur).zSchema = ::core::ptr::null::<::core::ffi::c_char>();
        }
        rc = sqlite3VdbeNextOpcode(
            (*pCur).pStmt as *mut Vdbe,
            if (*pCur).showSubprograms != 0 {
                &raw mut (*pCur).sub
            } else {
                ::core::ptr::null_mut::<Mem>()
            },
            (*pTab).bTablesUsed,
            &raw mut (*pCur).iRowid,
            &raw mut (*pCur).iAddr,
            &raw mut (*pCur).aOp,
        );
        if rc != SQLITE_OK {
            sqlite3VdbeMemSetNull(&raw mut (*pCur).sub);
            (*pCur).aOp = ::core::ptr::null_mut::<Op>();
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn bytecodevtabEof(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut bytecodevtab_cursor = cur as *mut bytecodevtab_cursor;
        return (*pCur).aOp.is_null() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn bytecodevtabColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut bytecodevtab_cursor = cur as *mut bytecodevtab_cursor;
        let mut pVTab: *mut bytecodevtab = (*cur).pVtab as *mut bytecodevtab;
        let mut pOp: *mut Op = (*pCur).aOp.offset((*pCur).iAddr as isize);
        if (*pVTab).bTablesUsed != 0 {
            if i == 4 as ::core::ffi::c_int {
                i = 8 as ::core::ffi::c_int;
            } else {
                if i <= 2 as ::core::ffi::c_int && (*pCur).zType.is_null() {
                    let mut pSchema: *mut Schema = ::core::ptr::null_mut::<Schema>();
                    let mut k: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
                    let mut iDb: ::core::ffi::c_int = (*pOp).p3;
                    let mut iRoot: Pgno = (*pOp).p2 as Pgno;
                    let mut db: *mut sqlite3 = (*pVTab).db;
                    pSchema = (*(*db).aDb.offset(iDb as isize)).pSchema;
                    (*pCur).zSchema = (*(*db).aDb.offset(iDb as isize)).zDbSName;
                    k = (*pSchema).tblHash.first;
                    while !k.is_null() {
                        let mut pTab: *mut Table = (*k).data as *mut Table;
                        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB)
                            && (*pTab).tnum == iRoot
                        {
                            (*pCur).zName = (*pTab).zName;
                            (*pCur).zType = b"table\0".as_ptr()
                                as *const ::core::ffi::c_char;
                            break;
                        } else {
                            k = (*k).next;
                        }
                    }
                    if (*pCur).zName.is_null() {
                        k = (*pSchema).idxHash.first;
                        while !k.is_null() {
                            let mut pIdx: *mut Index = (*k).data as *mut Index;
                            if (*pIdx).tnum == iRoot {
                                (*pCur).zName = (*pIdx).zName;
                                (*pCur).zType = b"index\0".as_ptr()
                                    as *const ::core::ffi::c_char;
                            }
                            k = (*k).next;
                        }
                    }
                }
                i += 20 as ::core::ffi::c_int;
            }
        }
        match i {
            0 => {
                sqlite3_result_int(ctx, (*pCur).iAddr);
            }
            1 => {
                sqlite3_result_text(
                    ctx,
                    sqlite3OpcodeName((*pOp).opcode as ::core::ffi::c_int)
                        as *mut ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            2 => {
                sqlite3_result_int(ctx, (*pOp).p1);
            }
            3 => {
                sqlite3_result_int(ctx, (*pOp).p2);
            }
            4 => {
                sqlite3_result_int(ctx, (*pOp).p3);
            }
            5 | 7 => {
                if (*pCur).zP4.is_null() {
                    (*pCur).zP4 = sqlite3VdbeDisplayP4((*pVTab).db, pOp);
                }
                if i == 5 as ::core::ffi::c_int {
                    sqlite3_result_text(
                        ctx,
                        (*pCur).zP4,
                        -1 as ::core::ffi::c_int,
                        SQLITE_STATIC,
                    );
                }
            }
            6 => {
                sqlite3_result_int(ctx, (*pOp).p5 as ::core::ffi::c_int);
            }
            8 => {
                let mut aOp: *mut Op = (*pCur).aOp;
                if !((*pCur).iRowid == (*pCur).iAddr + 1 as ::core::ffi::c_int) {
                    if !(*aOp.offset(0 as ::core::ffi::c_int as isize)).p4.z.is_null() {
                        sqlite3_result_text(
                            ctx,
                            (*aOp.offset(0 as ::core::ffi::c_int as isize))
                                .p4
                                .z
                                .offset(3 as ::core::ffi::c_int as isize),
                            -1 as ::core::ffi::c_int,
                            SQLITE_STATIC,
                        );
                    } else {
                        sqlite3_result_text(
                            ctx,
                            b"(FK)\0".as_ptr() as *const ::core::ffi::c_char,
                            4 as ::core::ffi::c_int,
                            SQLITE_STATIC,
                        );
                    }
                }
            }
            9 | 10 => {
                sqlite3_result_int(ctx, 0 as ::core::ffi::c_int);
            }
            20 => {
                sqlite3_result_text(
                    ctx,
                    (*pCur).zType,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            21 => {
                sqlite3_result_text(
                    ctx,
                    (*pCur).zSchema,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            22 => {
                sqlite3_result_text(
                    ctx,
                    (*pCur).zName,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            23 => {
                sqlite3_result_int(
                    ctx,
                    ((*pOp).opcode as ::core::ffi::c_int == OP_OpenWrite)
                        as ::core::ffi::c_int,
                );
            }
            _ => {}
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn bytecodevtabRowid(
    mut cur: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut bytecodevtab_cursor = cur as *mut bytecodevtab_cursor;
        *pRowid = (*pCur).iRowid as sqlite_int64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn bytecodevtabFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut bytecodevtab_cursor = pVtabCursor as *mut bytecodevtab_cursor;
        let mut pVTab: *mut bytecodevtab = (*pVtabCursor).pVtab as *mut bytecodevtab;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        bytecodevtabCursorClear(pCur);
        (*pCur).iRowid = 0 as ::core::ffi::c_int;
        (*pCur).iAddr = 0 as ::core::ffi::c_int;
        (*pCur).showSubprograms = (idxNum == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            == SQLITE_TEXT
        {
            let mut zSql: *const ::core::ffi::c_char = sqlite3_value_text(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            ) as *const ::core::ffi::c_char;
            if zSql.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                rc = sqlite3_prepare_v2(
                    (*pVTab).db,
                    zSql,
                    -1 as ::core::ffi::c_int,
                    &raw mut (*pCur).pStmt,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                (*pCur).needFinalize = 1 as ::core::ffi::c_int;
            }
        } else {
            (*pCur).pStmt = sqlite3_value_pointer(
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b"stmt-pointer\0".as_ptr() as *const ::core::ffi::c_char,
            ) as *mut sqlite3_stmt;
        }
        if (*pCur).pStmt.is_null() {
            (*pVTab).base.zErrMsg = sqlite3_mprintf(
                b"argument to %s() is not a valid SQL statement\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if (*pVTab).bTablesUsed != 0 {
                    b"tables_used\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"bytecode\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            rc = SQLITE_ERROR;
        } else {
            bytecodevtabNext(pVtabCursor);
        }
        return rc;
    }
}
unsafe extern "C" fn bytecodevtabBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_CONSTRAINT;
        let mut p: *mut sqlite3_index_constraint = ::core::ptr::null_mut::<
            sqlite3_index_constraint,
        >();
        let mut pVTab: *mut bytecodevtab = tab as *mut bytecodevtab;
        let mut iBaseCol: ::core::ffi::c_int = if (*pVTab).bTablesUsed != 0 {
            4 as ::core::ffi::c_int
        } else {
            10 as ::core::ffi::c_int
        };
        (*pIdxInfo).estimatedCost = 100 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*pIdxInfo).estimatedRows = 100 as sqlite3_int64;
        (*pIdxInfo).idxNum = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        p = (*pIdxInfo).aConstraint as *mut sqlite3_index_constraint;
        while i < (*pIdxInfo).nConstraint {
            if !((*p).usable as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
                if (*p).op as ::core::ffi::c_int == SQLITE_INDEX_CONSTRAINT_EQ
                    && (*p).iColumn == iBaseCol + 1 as ::core::ffi::c_int
                {
                    rc = SQLITE_OK;
                    (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).omit = 1
                        as ::core::ffi::c_uchar;
                    (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).argvIndex = 1
                        as ::core::ffi::c_int;
                }
                if (*p).op as ::core::ffi::c_int == SQLITE_INDEX_CONSTRAINT_ISNULL
                    && (*p).iColumn == iBaseCol
                {
                    (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).omit = 1
                        as ::core::ffi::c_uchar;
                    (*pIdxInfo).idxNum = 1 as ::core::ffi::c_int;
                }
            }
            i += 1;
            p = p.offset(1);
        }
        return rc;
    }
}
static mut bytecodevtabModule: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: None,
        xConnect: Some(
            bytecodevtabConnect
                as unsafe extern "C" fn(
                    *mut sqlite3,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const *const ::core::ffi::c_char,
                    *mut *mut sqlite3_vtab,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xBestIndex: Some(
            bytecodevtabBestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            bytecodevtabDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: None,
        xOpen: Some(
            bytecodevtabOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            bytecodevtabClose
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            bytecodevtabFilter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            bytecodevtabNext
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            bytecodevtabEof
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            bytecodevtabColumn
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            bytecodevtabRowid
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xUpdate: None,
        xBegin: None,
        xSync: None,
        xCommit: None,
        xRollback: None,
        xFindFunction: None,
        xRename: None,
        xSavepoint: None,
        xRelease: None,
        xRollbackTo: None,
        xShadowName: None,
        xIntegrity: None

    }
};
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeBytecodeVtabInit(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3_create_module(
            db,
            b"bytecode\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut bytecodevtabModule,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        if rc == SQLITE_OK {
            rc = sqlite3_create_module(
                db,
                b"tables_used\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut bytecodevtabModule,
                &raw mut db as *mut ::core::ffi::c_void,
            );
        }
        return rc;
    }
}
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
