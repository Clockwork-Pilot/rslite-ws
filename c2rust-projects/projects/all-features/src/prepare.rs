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
    pub type sqlite3_stmt;
    pub type sqlite3_pcache;
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
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const ::core::ffi::c_char;
    fn sqlite3_stmt_isexplain(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3BtreeSetCacheSize(
        _: *mut Btree,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeLastPage(_: *mut Btree) -> Pgno;
    fn sqlite3BtreeBeginTrans(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeCommit(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeTxnState(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeSchemaLocked(pBtree: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeGetMeta(
        pBtree: *mut Btree,
        idx: ::core::ffi::c_int,
        pValue: *mut u32_0,
    );
    fn sqlite3BtreeEnter(_: *mut Btree);
    fn sqlite3BtreeEnterAll(_: *mut sqlite3);
    fn sqlite3BtreeLeave(_: *mut Btree);
    fn sqlite3BtreeLeaveAll(_: *mut sqlite3);
    fn sqlite3VdbeFinalize(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeResetStepResult(_: *mut Vdbe);
    fn sqlite3VdbeDb(_: *mut Vdbe) -> *mut sqlite3;
    fn sqlite3VdbePrepareFlags(_: *mut Vdbe) -> u8_0;
    fn sqlite3VdbeSetSql(
        _: *mut Vdbe,
        z: *const ::core::ffi::c_char,
        n: ::core::ffi::c_int,
        _: u8_0,
    );
    fn sqlite3VdbeSwap(_: *mut Vdbe, _: *mut Vdbe);
    fn sqlite3CorruptError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3MisuseError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3DbMallocRaw(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrNDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: u64_0,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbNNFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3SetString(
        _: *mut *mut ::core::ffi::c_char,
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3RunParser(
        _: *mut Parse,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprListDelete(_: *mut sqlite3, _: *mut ExprList);
    fn sqlite3ResetAllSchemasOfConnection(_: *mut sqlite3);
    fn sqlite3ResetOneSchema(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3CommitInternalChanges(_: *mut sqlite3);
    fn sqlite3FaultSim(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3FindIndex(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut Index;
    fn sqlite3SafetyCheckOk(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3GetUInt32(
        _: *const ::core::ffi::c_char,
        _: *mut u32_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3Utf16ByteLen(
        pData: *const ::core::ffi::c_void,
        nByte: ::core::ffi::c_int,
        nChar: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Utf8CharLen(
        pData: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ErrorWithMsg(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3Error(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3ErrorClear(_: *mut sqlite3);
    fn sqlite3ErrStr(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3SetTextEncoding(db: *mut sqlite3, _: u8_0);
    fn sqlite3AbsInt32(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3Utf16to8(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> *mut ::core::ffi::c_char;
    static mut sqlite3StdType: [*const ::core::ffi::c_char; 0];
    static sqlite3UpperToLower: [::core::ffi::c_uchar; 0];
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3AnalysisLoad(
        _: *mut sqlite3,
        iDB: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3ApiExit(db: *mut sqlite3, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3VtabUnlockList(_: *mut sqlite3);
    fn sqlite3TransferBindings(
        _: *mut sqlite3_stmt,
        _: *mut sqlite3_stmt,
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
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InitData {
    pub db: *mut sqlite3,
    pub pzErrMsg: *mut *mut ::core::ffi::c_char,
    pub iDb: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub mInitFlags: u32_0,
    pub nInitRow: u32_0,
    pub mxPage: Pgno

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
pub const SQLITE_LOCKED: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_INTERRUPT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_SCHEMA: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_ERROR_RETRY: ::core::ffi::c_int = SQLITE_ERROR
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_SQL_LENGTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_PREPARE_PERSISTENT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_UTF16LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_TXN_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OMIT_TEMPDB: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_MAX_FILE_FORMAT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_UTF16NATIVE: ::core::ffi::c_int = SQLITE_UTF16LE;
pub const LEGACY_SCHEMA_TABLE: [::core::ffi::c_char; 14] = unsafe {
    ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"sqlite_master\0")
};
pub const LEGACY_TEMP_SCHEMA_TABLE: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<
        [u8; 19],
        [::core::ffi::c_char; 19],
    >(*b"sqlite_temp_master\0")
};
pub const BTREE_SCHEMA_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTREE_FILE_FORMAT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BTREE_DEFAULT_CACHE_SIZE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const BTREE_TEXT_ENCODING: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_PREPARE_SAVESQL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_PREPARE_MASK: ::core::ffi::c_int = 0x1f as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_CACHE_SIZE: ::core::ffi::c_int = -2000 as ::core::ffi::c_int;
pub const SQLITE_WriteSchema: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_LegacyFileFmt: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_ResetDatabase: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const SQLITE_NoSchemaError: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const DBFLAG_SchemaChange: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const DBFLAG_SchemaKnownOk: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const DBFLAG_EncodingFixed: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const PARSE_HDR_SZ: ::core::ffi::c_ulong = (192 as ::core::ffi::c_ulong)
    .wrapping_sub(8 as ::core::ffi::c_ulong);
pub const PARSE_RECURSE_SZ: ::core::ffi::c_ulong = 288 as ::core::ffi::c_ulong;
pub const PARSE_TAIL_SZ: usize = (::core::mem::size_of::<Parse>() as usize)
    .wrapping_sub(PARSE_RECURSE_SZ as usize);
pub const INITFLAG_AlterMask: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
unsafe extern "C" fn corruptSchema(
    mut pData: *mut InitData,
    mut azObj: *mut *mut ::core::ffi::c_char,
    mut zExtra: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pData).db;
        if (*db).mallocFailed != 0 {
            (*pData).rc = SQLITE_NOMEM_BKPT;
        } else if (*(*pData).pzErrMsg.offset(0 as ::core::ffi::c_int as isize)).is_null()
        {
            if (*pData).mInitFlags & 0x3 as u32_0 != 0 {
                static mut azAlterType: [*const ::core::ffi::c_char; 3] = [
                    b"rename\0".as_ptr() as *const ::core::ffi::c_char,
                    b"drop column\0".as_ptr() as *const ::core::ffi::c_char,
                    b"add column\0".as_ptr() as *const ::core::ffi::c_char,
                ];
                *(*pData).pzErrMsg = sqlite3MPrintf(
                    db,
                    b"error in %s %s after %s: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *azObj.offset(0 as ::core::ffi::c_int as isize),
                    *azObj.offset(1 as ::core::ffi::c_int as isize),
                    azAlterType[((*pData).mInitFlags & INITFLAG_AlterMask as u32_0)
                        .wrapping_sub(1 as u32_0) as usize],
                    zExtra,
                );
                (*pData).rc = SQLITE_ERROR;
            } else if (*db).flags as ::core::ffi::c_ulonglong
                & SQLITE_WriteSchema as ::core::ffi::c_ulonglong != 0
            {
                (*pData).rc = sqlite3CorruptError(45 as ::core::ffi::c_int);
            } else {
                let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut zObj: *const ::core::ffi::c_char = if !(*azObj
                    .offset(1 as ::core::ffi::c_int as isize))
                    .is_null()
                {
                    *azObj.offset(1 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char
                } else {
                    b"?\0".as_ptr() as *const ::core::ffi::c_char
                };
                z = sqlite3MPrintf(
                    db,
                    b"malformed database schema (%s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    zObj,
                );
                if !zExtra.is_null()
                    && *zExtra.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int != 0
                {
                    z = sqlite3MPrintf(
                        db,
                        b"%z - %s\0".as_ptr() as *const ::core::ffi::c_char,
                        z,
                        zExtra,
                    );
                }
                *(*pData).pzErrMsg = z;
                (*pData).rc = sqlite3CorruptError(52 as ::core::ffi::c_int);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IndexHasDuplicateRootPage(
    mut pIndex: *mut Index,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Index = ::core::ptr::null_mut::<Index>();
        p = (*(*pIndex).pTable).pIndex;
        while !p.is_null() {
            if (*p).tnum == (*pIndex).tnum && p != pIndex {
                return 1 as ::core::ffi::c_int;
            }
            p = (*p).pNext;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3InitCallback(
    mut pInit: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut NotUsed: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pData: *mut InitData = pInit as *mut InitData;
        let mut db: *mut sqlite3 = (*pData).db;
        let mut iDb: ::core::ffi::c_int = (*pData).iDb;
        (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
            | DBFLAG_EncodingFixed as ::core::ffi::c_uint) as u32_0;
        if argv.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*pData).nInitRow = (*pData).nInitRow.wrapping_add(1);
        if (*db).mallocFailed != 0 {
            corruptSchema(pData, argv, ::core::ptr::null::<::core::ffi::c_char>());
            return 1 as ::core::ffi::c_int;
        }
        if (*argv.offset(3 as ::core::ffi::c_int as isize)).is_null() {
            corruptSchema(pData, argv, ::core::ptr::null::<::core::ffi::c_char>());
        } else if !(*argv.offset(4 as ::core::ffi::c_int as isize)).is_null()
            && 'c' as i32
                == *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                    .offset(
                        *(*argv.offset(4 as ::core::ffi::c_int as isize))
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar as isize,
                    ) as ::core::ffi::c_int
            && 'r' as i32
                == *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                    .offset(
                        *(*argv.offset(4 as ::core::ffi::c_int as isize))
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar as isize,
                    ) as ::core::ffi::c_int
        {
            let mut rc: ::core::ffi::c_int = 0;
            let mut saved_iDb: u8_0 = (*db).init.iDb;
            let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            (*db).init.iDb = iDb as u8_0;
            if sqlite3GetUInt32(
                *argv.offset(3 as ::core::ffi::c_int as isize),
                &raw mut (*db).init.newTnum,
            ) == 0 as ::core::ffi::c_int
                || (*db).init.newTnum > (*pData).mxPage && (*pData).mxPage > 0 as Pgno
            {
                if sqlite3Config.bExtraSchemaChecks != 0 {
                    corruptSchema(
                        pData,
                        argv,
                        b"invalid rootpage\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
            (*db)
                .init
                .set_orphanTrigger(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*db).init.azInit = argv as *mut *const ::core::ffi::c_char;
            pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            sqlite3Prepare(
                db,
                *argv.offset(4 as ::core::ffi::c_int as isize),
                -1 as ::core::ffi::c_int,
                0 as u32_0,
                ::core::ptr::null_mut::<Vdbe>(),
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            rc = (*db).errCode;
            (*db).init.iDb = saved_iDb;
            if SQLITE_OK != rc {
                if !((*db).init.orphanTrigger() != 0) {
                    if rc > (*pData).rc {
                        (*pData).rc = rc;
                    }
                    if rc == SQLITE_NOMEM {
                        sqlite3OomFault(db);
                    } else if rc != SQLITE_INTERRUPT
                        && rc & 0xff as ::core::ffi::c_int != SQLITE_LOCKED
                    {
                        corruptSchema(pData, argv, sqlite3_errmsg(db));
                    }
                }
            }
            (*db).init.azInit = &raw mut sqlite3StdType
                as *mut *const ::core::ffi::c_char;
            sqlite3_finalize(pStmt);
        } else if (*argv.offset(1 as ::core::ffi::c_int as isize)).is_null()
            || !(*argv.offset(4 as ::core::ffi::c_int as isize)).is_null()
                && *(*argv.offset(4 as ::core::ffi::c_int as isize))
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
        {
            corruptSchema(pData, argv, ::core::ptr::null::<::core::ffi::c_char>());
        } else {
            let mut pIndex: *mut Index = ::core::ptr::null_mut::<Index>();
            pIndex = sqlite3FindIndex(
                db,
                *argv.offset(1 as ::core::ffi::c_int as isize),
                (*(*db).aDb.offset(iDb as isize)).zDbSName,
            );
            if pIndex.is_null() {
                corruptSchema(
                    pData,
                    argv,
                    b"orphan index\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else if sqlite3GetUInt32(
                *argv.offset(3 as ::core::ffi::c_int as isize),
                &raw mut (*pIndex).tnum,
            ) == 0 as ::core::ffi::c_int || (*pIndex).tnum < 2 as Pgno
                || (*pIndex).tnum > (*pData).mxPage
                || sqlite3IndexHasDuplicateRootPage(pIndex) != 0
            {
                if sqlite3Config.bExtraSchemaChecks != 0 {
                    corruptSchema(
                        pData,
                        argv,
                        b"invalid rootpage\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3InitOne(
    mut db: *mut sqlite3,
    mut iDb: ::core::ffi::c_int,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut mFlags: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut size: ::core::ffi::c_int = 0;
        let mut pDb: *mut Db = ::core::ptr::null_mut::<Db>();
        let mut azArg: [*const ::core::ffi::c_char; 6] = [::core::ptr::null::<
            ::core::ffi::c_char,
        >(); 6];
        let mut meta: [::core::ffi::c_int; 5] = [0; 5];
        let mut initData: InitData = InitData {
            db: ::core::ptr::null_mut::<sqlite3>(),
            pzErrMsg: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            iDb: 0,
            rc: 0,
            mInitFlags: 0,
            nInitRow: 0,
            mxPage: 0

        };
        let mut zSchemaTabName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut openedTransaction: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mask: ::core::ffi::c_int = ((*db).mDbFlags
            & DBFLAG_EncodingFixed as u32_0 | !DBFLAG_EncodingFixed as u32_0)
            as ::core::ffi::c_int;
        (*db).init.busy = 1 as u8_0;
        azArg[0 as ::core::ffi::c_int as usize] = b"table\0".as_ptr()
            as *const ::core::ffi::c_char;
        zSchemaTabName = if OMIT_TEMPDB == 0 && iDb == 1 as ::core::ffi::c_int {
            LEGACY_TEMP_SCHEMA_TABLE.as_ptr()
        } else {
            LEGACY_SCHEMA_TABLE.as_ptr()
        };
        azArg[1 as ::core::ffi::c_int as usize] = zSchemaTabName;
        azArg[2 as ::core::ffi::c_int as usize] = azArg[1 as ::core::ffi::c_int
            as usize];
        azArg[3 as ::core::ffi::c_int as usize] = b"1\0".as_ptr()
            as *const ::core::ffi::c_char;
        azArg[4 as ::core::ffi::c_int as usize] = b"CREATE TABLE x(type text,name text,tbl_name text,rootpage int,sql text)\0"
            .as_ptr() as *const ::core::ffi::c_char;
        azArg[5 as ::core::ffi::c_int as usize] = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        initData.db = db;
        initData.iDb = iDb;
        initData.rc = SQLITE_OK;
        initData.pzErrMsg = pzErrMsg;
        initData.mInitFlags = mFlags;
        initData.nInitRow = 0 as u32_0;
        initData.mxPage = 0 as Pgno;
        sqlite3InitCallback(
            &raw mut initData as *mut ::core::ffi::c_void,
            5 as ::core::ffi::c_int,
            &raw mut azArg as *mut *const ::core::ffi::c_char
                as *mut *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
            & mask as ::core::ffi::c_uint) as u32_0;
        if initData.rc != 0 {
            rc = initData.rc;
        } else {
            pDb = (*db).aDb.offset(iDb as isize) as *mut Db;
            if (*pDb).pBt.is_null() {
                let ref mut c2rust_fresh0 = (*(*(*db)
                    .aDb
                    .offset(1 as ::core::ffi::c_int as isize))
                    .pSchema)
                    .schemaFlags;
                *c2rust_fresh0 = (*c2rust_fresh0 as ::core::ffi::c_int
                    | 0x1 as ::core::ffi::c_int) as u16_0;
                rc = SQLITE_OK;
            } else {
                sqlite3BtreeEnter((*pDb).pBt);
                if sqlite3BtreeTxnState((*pDb).pBt) == SQLITE_TXN_NONE {
                    rc = sqlite3BtreeBeginTrans(
                        (*pDb).pBt,
                        0 as ::core::ffi::c_int,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    );
                    if rc != SQLITE_OK {
                        sqlite3SetString(pzErrMsg, db, sqlite3ErrStr(rc));
                        c2rust_current_block = 3818621012670888794;
                    } else {
                        openedTransaction = 1 as ::core::ffi::c_int;
                        c2rust_current_block = 14763689060501151050;
                    }
                } else {
                    c2rust_current_block = 14763689060501151050;
                }
                match c2rust_current_block {
                    14763689060501151050 => {
                        i = 0 as ::core::ffi::c_int;
                        while i
                            < (::core::mem::size_of::<[::core::ffi::c_int; 5]>()
                                as usize)
                                .wrapping_div(
                                    ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                ) as ::core::ffi::c_int
                        {
                            sqlite3BtreeGetMeta(
                                (*pDb).pBt,
                                i + 1 as ::core::ffi::c_int,
                                (&raw mut meta as *mut ::core::ffi::c_int)
                                    .offset(i as isize) as *mut ::core::ffi::c_int as *mut u32_0,
                            );
                            i += 1;
                        }
                        if (*db).flags as ::core::ffi::c_ulonglong
                            & SQLITE_ResetDatabase as ::core::ffi::c_ulonglong
                            != 0 as ::core::ffi::c_ulonglong
                        {
                            memset(
                                &raw mut meta as *mut ::core::ffi::c_int
                                    as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<[::core::ffi::c_int; 5]>() as size_t,
                            );
                        }
                        (*(*pDb).pSchema).schema_cookie = meta[(BTREE_SCHEMA_VERSION
                            - 1 as ::core::ffi::c_int) as usize];
                        if meta[(BTREE_TEXT_ENCODING - 1 as ::core::ffi::c_int) as usize]
                            != 0
                        {
                            if iDb == 0 as ::core::ffi::c_int
                                && (*db).mDbFlags & DBFLAG_EncodingFixed as u32_0
                                    == 0 as u32_0
                            {
                                let mut encoding: u8_0 = 0;
                                encoding = (meta[(BTREE_TEXT_ENCODING
                                    - 1 as ::core::ffi::c_int) as usize] as u8_0
                                    as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as u8_0;
                                if encoding as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                {
                                    encoding = SQLITE_UTF8 as u8_0;
                                }
                                sqlite3SetTextEncoding(db, encoding);
                                c2rust_current_block = 790185930182612747;
                            } else if meta[(BTREE_TEXT_ENCODING
                                - 1 as ::core::ffi::c_int) as usize]
                                & 3 as ::core::ffi::c_int != (*db).enc as ::core::ffi::c_int
                            {
                                sqlite3SetString(
                                    pzErrMsg,
                                    db,
                                    b"attached databases must use the same text encoding as main database\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                );
                                rc = SQLITE_ERROR;
                                c2rust_current_block = 3818621012670888794;
                            } else {
                                c2rust_current_block = 790185930182612747;
                            }
                        } else {
                            c2rust_current_block = 790185930182612747;
                        }
                        match c2rust_current_block {
                            3818621012670888794 => {}
                            _ => {
                                (*(*pDb).pSchema).enc = (*db).enc;
                                if (*(*pDb).pSchema).cache_size == 0 as ::core::ffi::c_int {
                                    size = sqlite3AbsInt32(
                                        meta[(BTREE_DEFAULT_CACHE_SIZE - 1 as ::core::ffi::c_int)
                                            as usize],
                                    );
                                    if size == 0 as ::core::ffi::c_int {
                                        size = SQLITE_DEFAULT_CACHE_SIZE;
                                    }
                                    (*(*pDb).pSchema).cache_size = size;
                                    sqlite3BtreeSetCacheSize(
                                        (*pDb).pBt,
                                        (*(*pDb).pSchema).cache_size,
                                    );
                                }
                                (*(*pDb).pSchema).file_format = meta[(BTREE_FILE_FORMAT
                                    - 1 as ::core::ffi::c_int) as usize] as u8_0;
                                if (*(*pDb).pSchema).file_format as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    (*(*pDb).pSchema).file_format = 1 as u8_0;
                                }
                                if (*(*pDb).pSchema).file_format as ::core::ffi::c_int
                                    > SQLITE_MAX_FILE_FORMAT
                                {
                                    sqlite3SetString(
                                        pzErrMsg,
                                        db,
                                        b"unsupported file format\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                    rc = SQLITE_ERROR;
                                } else {
                                    if iDb == 0 as ::core::ffi::c_int
                                        && meta[(BTREE_FILE_FORMAT - 1 as ::core::ffi::c_int)
                                            as usize] >= 4 as ::core::ffi::c_int
                                    {
                                        (*db).flags
                                            &= !(SQLITE_LegacyFileFmt as u64_0)
                                                as ::core::ffi::c_ulonglong;
                                    }
                                    initData.mxPage = sqlite3BtreeLastPage((*pDb).pBt);
                                    let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                        ::core::ffi::c_char,
                                    >();
                                    zSql = sqlite3MPrintf(
                                        db,
                                        b"SELECT*FROM\"%w\".%s ORDER BY rowid\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        (*(*db).aDb.offset(iDb as isize)).zDbSName,
                                        zSchemaTabName,
                                    );
                                    let mut xAuth: sqlite3_xauth = None;
                                    xAuth = (*db).xAuth;
                                    (*db).xAuth = None;
                                    rc = sqlite3_exec(
                                        db,
                                        zSql,
                                        Some(
                                            sqlite3InitCallback
                                                as unsafe extern "C" fn(
                                                    *mut ::core::ffi::c_void,
                                                    ::core::ffi::c_int,
                                                    *mut *mut ::core::ffi::c_char,
                                                    *mut *mut ::core::ffi::c_char,
                                                ) -> ::core::ffi::c_int,
                                        ),
                                        &raw mut initData as *mut ::core::ffi::c_void,
                                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                    );
                                    (*db).xAuth = xAuth;
                                    if rc == SQLITE_OK {
                                        rc = initData.rc;
                                    }
                                    sqlite3DbFree(db, zSql as *mut ::core::ffi::c_void);
                                    if rc == SQLITE_OK {
                                        sqlite3AnalysisLoad(db, iDb);
                                    }
                                    if (*db).mallocFailed != 0 {
                                        rc = SQLITE_NOMEM_BKPT;
                                        sqlite3ResetAllSchemasOfConnection(db);
                                        pDb = (*db).aDb.offset(iDb as isize) as *mut Db;
                                    } else if rc == SQLITE_OK
                                        || (*db).flags as ::core::ffi::c_ulonglong
                                            & SQLITE_NoSchemaError as ::core::ffi::c_ulonglong != 0
                                            && rc != SQLITE_NOMEM
                                    {
                                        let ref mut c2rust_fresh1 = (*(*(*db)
                                            .aDb
                                            .offset(iDb as isize))
                                            .pSchema)
                                            .schemaFlags;
                                        *c2rust_fresh1 = (*c2rust_fresh1 as ::core::ffi::c_int
                                            | 0x1 as ::core::ffi::c_int) as u16_0;
                                        rc = SQLITE_OK;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                if openedTransaction != 0 {
                    sqlite3BtreeCommit((*pDb).pBt);
                }
                sqlite3BtreeLeave((*pDb).pBt);
            }
        }
        if rc != 0 {
            if rc == SQLITE_NOMEM || rc == SQLITE_IOERR_NOMEM {
                sqlite3OomFault(db);
            }
            sqlite3ResetOneSchema(db, iDb);
        }
        (*db).init.busy = 0 as u8_0;
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut commit_internal: ::core::ffi::c_int = ((*db).mDbFlags
            & DBFLAG_SchemaChange as u32_0 == 0) as ::core::ffi::c_int;
        (*db).enc = (*(*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).pSchema).enc;
        if !((*(*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).pSchema).schemaFlags
            as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
            == 0x1 as ::core::ffi::c_int)
        {
            rc = sqlite3InitOne(db, 0 as ::core::ffi::c_int, pzErrMsg, 0 as u32_0);
            if rc != 0 {
                return rc;
            }
        }
        i = (*db).nDb - 1 as ::core::ffi::c_int;
        while i > 0 as ::core::ffi::c_int {
            if !((*(*(*db).aDb.offset(i as isize)).pSchema).schemaFlags
                as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
                == 0x1 as ::core::ffi::c_int)
            {
                rc = sqlite3InitOne(db, i, pzErrMsg, 0 as u32_0);
                if rc != 0 {
                    return rc;
                }
            }
            i -= 1;
        }
        if commit_internal != 0 {
            sqlite3CommitInternalChanges(db);
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ReadSchema(
    mut pParse: *mut Parse,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut db: *mut sqlite3 = (*pParse).db;
        if (*db).init.busy == 0 {
            rc = sqlite3Init(db, &raw mut (*pParse).zErrMsg);
            if rc != SQLITE_OK {
                (*pParse).rc = rc;
                (*pParse).nErr += 1;
            } else if (*db).noSharedCache != 0 {
                (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
                    | DBFLAG_SchemaKnownOk as ::core::ffi::c_uint) as u32_0;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn schemaIsValid(mut pParse: *mut Parse) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut iDb: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut cookie: ::core::ffi::c_int = 0;
        iDb = 0 as ::core::ffi::c_int;
        while iDb < (*db).nDb {
            let mut openedTransaction: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pBt: *mut Btree = (*(*db).aDb.offset(iDb as isize)).pBt;
            if !pBt.is_null() {
                if sqlite3BtreeTxnState(pBt) == SQLITE_TXN_NONE {
                    rc = sqlite3BtreeBeginTrans(
                        pBt,
                        0 as ::core::ffi::c_int,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    );
                    if rc == SQLITE_NOMEM || rc == SQLITE_IOERR_NOMEM {
                        sqlite3OomFault(db);
                        (*pParse).rc = SQLITE_NOMEM;
                    }
                    if rc != SQLITE_OK {
                        return;
                    }
                    openedTransaction = 1 as ::core::ffi::c_int;
                }
                sqlite3BtreeGetMeta(
                    pBt,
                    BTREE_SCHEMA_VERSION,
                    &raw mut cookie as *mut u32_0,
                );
                if cookie != (*(*(*db).aDb.offset(iDb as isize)).pSchema).schema_cookie {
                    if (*(*(*db).aDb.offset(iDb as isize)).pSchema).schemaFlags
                        as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
                        == 0x1 as ::core::ffi::c_int
                    {
                        (*pParse).rc = SQLITE_SCHEMA;
                    }
                    sqlite3ResetOneSchema(db, iDb);
                }
                if openedTransaction != 0 {
                    sqlite3BtreeCommit(pBt);
                }
            }
            iDb += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SchemaToIndex(
    mut db: *mut sqlite3,
    mut pSchema: *mut Schema,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = -32768 as ::core::ffi::c_int;
        if !pSchema.is_null() {
            i = 0 as ::core::ffi::c_int;
            while !((*(*db).aDb.offset(i as isize)).pSchema == pSchema) {
                i += 1;
            }
        }
        return i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ParseObjectReset(mut pParse: *mut Parse) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        if !(*pParse).aTableLock.is_null() {
            sqlite3DbNNFreeNN(db, (*pParse).aTableLock as *mut ::core::ffi::c_void);
        }
        while !(*pParse).pCleanup.is_null() {
            let mut pCleanup: *mut ParseCleanup = (*pParse).pCleanup;
            (*pParse).pCleanup = (*pCleanup).pNext;
            (*pCleanup)
                .xCleanup
                .expect("non-null function pointer")(db, (*pCleanup).pPtr);
            sqlite3DbNNFreeNN(db, pCleanup as *mut ::core::ffi::c_void);
        }
        if !(*pParse).aLabel.is_null() {
            sqlite3DbNNFreeNN(db, (*pParse).aLabel as *mut ::core::ffi::c_void);
        }
        if !(*pParse).pConstExpr.is_null() {
            sqlite3ExprListDelete(db, (*pParse).pConstExpr);
        }
        (*db).lookaside.bDisable = ((*db).lookaside.bDisable as ::core::ffi::c_uint)
            .wrapping_sub((*pParse).disableLookaside as ::core::ffi::c_uint) as u32_0
            as u32_0;
        (*db).lookaside.sz = (if (*db).lookaside.bDisable != 0 {
            0 as ::core::ffi::c_int
        } else {
            (*db).lookaside.szTrue as ::core::ffi::c_int
        }) as u16_0;
        (*db).pParse = (*pParse).pOuterParse;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ParserAddCleanup(
    mut pParse: *mut Parse,
    mut xCleanup: Option<
        unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> (),
    >,
    mut pPtr: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pCleanup: *mut ParseCleanup = ::core::ptr::null_mut::<ParseCleanup>();
        if sqlite3FaultSim(300 as ::core::ffi::c_int) != 0 {
            pCleanup = ::core::ptr::null_mut::<ParseCleanup>();
            sqlite3OomFault((*pParse).db);
        } else {
            pCleanup = sqlite3DbMallocRaw(
                (*pParse).db,
                ::core::mem::size_of::<ParseCleanup>() as u64_0,
            ) as *mut ParseCleanup;
        }
        if !pCleanup.is_null() {
            (*pCleanup).pNext = (*pParse).pCleanup;
            (*pParse).pCleanup = pCleanup;
            (*pCleanup).pPtr = pPtr;
            (*pCleanup).xCleanup = xCleanup;
        } else {
            xCleanup.expect("non-null function pointer")((*pParse).db, pPtr);
            pPtr = ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        return pPtr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ParseObjectInit(
    mut pParse: *mut Parse,
    mut db: *mut sqlite3,
) {
    unsafe {
        memset(
            (pParse as *mut ::core::ffi::c_char)
                .offset(8 as ::core::ffi::c_ulong as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            PARSE_HDR_SZ as size_t,
        );
        memset(
            (pParse as *mut ::core::ffi::c_char).offset(PARSE_RECURSE_SZ as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            PARSE_TAIL_SZ,
        );
        (*pParse).pOuterParse = (*db).pParse;
        (*db).pParse = pParse;
        (*pParse).db = db;
        if (*db).mallocFailed != 0 {
            sqlite3ErrorMsg(
                pParse,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn sqlite3Prepare(
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_char,
    mut nBytes: ::core::ffi::c_int,
    mut prepFlags: u32_0,
    mut pReprepare: *mut Vdbe,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut i: ::core::ffi::c_int = 0;
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
        memset(
            (&raw mut sParse as *mut ::core::ffi::c_char)
                .offset(8 as ::core::ffi::c_ulong as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            PARSE_HDR_SZ as size_t,
        );
        memset(
            (&raw mut sParse as *mut ::core::ffi::c_char)
                .offset(PARSE_RECURSE_SZ as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            PARSE_TAIL_SZ,
        );
        sParse.pOuterParse = (*db).pParse;
        (*db).pParse = &raw mut sParse;
        sParse.db = db;
        if !pReprepare.is_null() {
            sParse.pReprepare = pReprepare;
            sParse.explain = sqlite3_stmt_isexplain(pReprepare as *mut sqlite3_stmt)
                as u8_0;
        }
        if (*db).mallocFailed != 0 {
            sqlite3ErrorMsg(
                &raw mut sParse,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            rc = SQLITE_NOMEM;
            (*db).errCode = rc;
        } else {
            if prepFlags & SQLITE_PREPARE_PERSISTENT as u32_0 != 0 {
                sParse.disableLookaside = sParse.disableLookaside.wrapping_add(1);
                (*db).lookaside.bDisable = (*db).lookaside.bDisable.wrapping_add(1);
                (*db).lookaside.sz = 0 as u16_0;
            }
            sParse.prepFlags = (prepFlags & 0xff as u32_0) as u8_0;
            if (*db).noSharedCache == 0 {
                i = 0 as ::core::ffi::c_int;
                loop {
                    if !(i < (*db).nDb) {
                        c2rust_current_block = 7056779235015430508;
                        break;
                    }
                    let mut pBt: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
                    if !pBt.is_null() {
                        rc = sqlite3BtreeSchemaLocked(pBt);
                        if rc != 0 {
                            let mut zDb: *const ::core::ffi::c_char = (*(*db)
                                .aDb
                                .offset(i as isize))
                                .zDbSName;
                            sqlite3ErrorWithMsg(
                                db,
                                rc,
                                b"database schema is locked: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                zDb,
                            );
                            c2rust_current_block = 371018172581504561;
                            break;
                        }
                    }
                    i += 1;
                }
            } else {
                c2rust_current_block = 7056779235015430508;
            }
            match c2rust_current_block {
                371018172581504561 => {}
                _ => {
                    if !(*db).pDisconnect.is_null() {
                        sqlite3VtabUnlockList(db);
                    }
                    if nBytes >= 0 as ::core::ffi::c_int
                        && (nBytes == 0 as ::core::ffi::c_int
                            || *zSql.offset((nBytes - 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                    {
                        let mut zSqlCopy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        let mut mxLen: ::core::ffi::c_int = (*db)
                            .aLimit[SQLITE_LIMIT_SQL_LENGTH as usize];
                        if nBytes > mxLen {
                            sqlite3ErrorWithMsg(
                                db,
                                SQLITE_TOOBIG,
                                b"statement too long\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            rc = sqlite3ApiExit(db, SQLITE_TOOBIG);
                            c2rust_current_block = 371018172581504561;
                        } else {
                            zSqlCopy = sqlite3DbStrNDup(db, zSql, nBytes as u64_0);
                            if !zSqlCopy.is_null() {
                                sqlite3RunParser(&raw mut sParse, zSqlCopy);
                                sParse.zTail = zSql
                                    .offset(
                                        sParse.zTail.offset_from(zSqlCopy) as ::core::ffi::c_long
                                            as isize,
                                    ) as *const ::core::ffi::c_char;
                                sqlite3DbFree(db, zSqlCopy as *mut ::core::ffi::c_void);
                            } else {
                                sParse.zTail = zSql.offset(nBytes as isize)
                                    as *const ::core::ffi::c_char;
                            }
                            c2rust_current_block = 5330834795799507926;
                        }
                    } else {
                        sqlite3RunParser(&raw mut sParse, zSql);
                        c2rust_current_block = 5330834795799507926;
                    }
                    match c2rust_current_block {
                        371018172581504561 => {}
                        _ => {
                            if !pzTail.is_null() {
                                *pzTail = sParse.zTail;
                            }
                            if (*db).init.busy as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                sqlite3VdbeSetSql(
                                    sParse.pVdbe,
                                    zSql,
                                    sParse.zTail.offset_from(zSql) as ::core::ffi::c_long
                                        as ::core::ffi::c_int,
                                    prepFlags as u8_0,
                                );
                            }
                            if (*db).mallocFailed != 0 {
                                sParse.rc = SQLITE_NOMEM_BKPT;
                                sParse.set_checkSchema(0 as bft as bft);
                            }
                            if sParse.rc != SQLITE_OK && sParse.rc != SQLITE_DONE {
                                if sParse.checkSchema() as ::core::ffi::c_int != 0
                                    && (*db).init.busy as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int
                                {
                                    schemaIsValid(&raw mut sParse);
                                }
                                if !sParse.pVdbe.is_null() {
                                    sqlite3VdbeFinalize(sParse.pVdbe);
                                }
                                rc = sParse.rc;
                                if !sParse.zErrMsg.is_null() {
                                    sqlite3ErrorWithMsg(
                                        db,
                                        rc,
                                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                        sParse.zErrMsg,
                                    );
                                    sqlite3DbFree(
                                        db,
                                        sParse.zErrMsg as *mut ::core::ffi::c_void,
                                    );
                                } else {
                                    sqlite3Error(db, rc);
                                }
                            } else {
                                *ppStmt = sParse.pVdbe as *mut sqlite3_stmt;
                                rc = SQLITE_OK;
                                sqlite3ErrorClear(db);
                            }
                            while !sParse.pTriggerPrg.is_null() {
                                let mut pT: *mut TriggerPrg = sParse.pTriggerPrg;
                                sParse.pTriggerPrg = (*pT).pNext;
                                sqlite3DbFree(db, pT as *mut ::core::ffi::c_void);
                            }
                        }
                    }
                }
            }
        }
        sqlite3ParseObjectReset(&raw mut sParse);
        return rc;
    }
}
unsafe extern "C" fn sqlite3LockAndPrepare(
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_char,
    mut nBytes: ::core::ffi::c_int,
    mut prepFlags: u32_0,
    mut pOld: *mut Vdbe,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        *ppStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if sqlite3SafetyCheckOk(db) == 0 || zSql.is_null() {
            return sqlite3MisuseError(852 as ::core::ffi::c_int);
        }
        sqlite3_mutex_enter((*db).mutex);
        sqlite3BtreeEnterAll(db);
        loop {
            rc = sqlite3Prepare(db, zSql, nBytes, prepFlags, pOld, ppStmt, pzTail);
            if rc == SQLITE_OK || (*db).mallocFailed as ::core::ffi::c_int != 0 {
                break;
            }
            cnt += 1;
            if !(rc == SQLITE_ERROR_RETRY && cnt <= 25 as ::core::ffi::c_int
                || rc == SQLITE_SCHEMA
                    && {
                        sqlite3ResetOneSchema(db, -1 as ::core::ffi::c_int);
                        cnt == 1 as ::core::ffi::c_int
                    })
            {
                break;
            }
        }
        sqlite3BtreeLeaveAll(db);
        rc = sqlite3ApiExit(db, rc);
        (*db).busyHandler.nBusy = 0 as ::core::ffi::c_int;
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Reprepare(mut p: *mut Vdbe) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pNew: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zSql: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut prepFlags: u8_0 = 0;
        zSql = sqlite3_sql(p as *mut sqlite3_stmt);
        db = sqlite3VdbeDb(p);
        prepFlags = sqlite3VdbePrepareFlags(p);
        rc = sqlite3LockAndPrepare(
            db,
            zSql,
            -1 as ::core::ffi::c_int,
            prepFlags as u32_0,
            p,
            &raw mut pNew,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != 0 {
            if rc == SQLITE_NOMEM {
                sqlite3OomFault(db);
            }
            return rc;
        }
        sqlite3VdbeSwap(pNew as *mut Vdbe, p);
        sqlite3TransferBindings(pNew, p as *mut sqlite3_stmt);
        sqlite3VdbeResetStepResult(pNew as *mut Vdbe);
        sqlite3VdbeFinalize(pNew as *mut Vdbe);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_prepare(
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_char,
    mut nBytes: ::core::ffi::c_int,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3LockAndPrepare(
            db,
            zSql,
            nBytes,
            0 as u32_0,
            ::core::ptr::null_mut::<Vdbe>(),
            ppStmt,
            pzTail,
        );
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_prepare_v2(
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_char,
    mut nBytes: ::core::ffi::c_int,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3LockAndPrepare(
            db,
            zSql,
            nBytes,
            SQLITE_PREPARE_SAVESQL as u32_0,
            ::core::ptr::null_mut::<Vdbe>(),
            ppStmt,
            pzTail,
        );
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_prepare_v3(
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_char,
    mut nBytes: ::core::ffi::c_int,
    mut prepFlags: ::core::ffi::c_uint,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3LockAndPrepare(
            db,
            zSql,
            nBytes,
            SQLITE_PREPARE_SAVESQL as u32_0
                | prepFlags as u32_0 & SQLITE_PREPARE_MASK as u32_0,
            ::core::ptr::null_mut::<Vdbe>(),
            ppStmt,
            pzTail,
        );
        return rc;
    }
}
unsafe extern "C" fn sqlite3Prepare16(
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_void,
    mut nBytes: ::core::ffi::c_int,
    mut prepFlags: u32_0,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zSql8: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zTail8: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        *ppStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if sqlite3SafetyCheckOk(db) == 0 || zSql.is_null() {
            return sqlite3MisuseError(1003 as ::core::ffi::c_int);
        }
        if nBytes >= 0 as ::core::ffi::c_int {
            let mut sz: ::core::ffi::c_int = 0;
            let mut z: *const ::core::ffi::c_char = zSql as *const ::core::ffi::c_char;
            sz = 0 as ::core::ffi::c_int;
            while sz < nBytes
                && (*z.offset(sz as isize) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                    || *z.offset((sz + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
            {
                sz += 2 as ::core::ffi::c_int;
            }
            nBytes = sz;
        } else {
            let mut sz_0: ::core::ffi::c_int = 0;
            let mut z_0: *const ::core::ffi::c_char = zSql as *const ::core::ffi::c_char;
            sz_0 = 0 as ::core::ffi::c_int;
            while *z_0.offset(sz_0 as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
                || *z_0.offset((sz_0 + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                sz_0 += 2 as ::core::ffi::c_int;
            }
            nBytes = sz_0;
        }
        sqlite3_mutex_enter((*db).mutex);
        zSql8 = sqlite3Utf16to8(db, zSql, nBytes, SQLITE_UTF16NATIVE as u8_0);
        if !zSql8.is_null() {
            rc = sqlite3LockAndPrepare(
                db,
                zSql8,
                -1 as ::core::ffi::c_int,
                prepFlags,
                ::core::ptr::null_mut::<Vdbe>(),
                ppStmt,
                &raw mut zTail8,
            );
        }
        if !zTail8.is_null() && !pzTail.is_null() {
            let mut chars_parsed: ::core::ffi::c_int = sqlite3Utf8CharLen(
                zSql8,
                zTail8.offset_from(zSql8) as ::core::ffi::c_long as ::core::ffi::c_int,
            );
            *pzTail = (zSql as *mut u8_0)
                .offset(sqlite3Utf16ByteLen(zSql, nBytes, chars_parsed) as isize)
                as *const ::core::ffi::c_void;
        }
        sqlite3DbFree(db, zSql8 as *mut ::core::ffi::c_void);
        rc = sqlite3ApiExit(db, rc);
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_prepare16(
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_void,
    mut nBytes: ::core::ffi::c_int,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3Prepare16(db, zSql, nBytes, 0 as u32_0, ppStmt, pzTail);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_prepare16_v2(
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_void,
    mut nBytes: ::core::ffi::c_int,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3Prepare16(
            db,
            zSql,
            nBytes,
            SQLITE_PREPARE_SAVESQL as u32_0,
            ppStmt,
            pzTail,
        );
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_prepare16_v3(
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_void,
    mut nBytes: ::core::ffi::c_int,
    mut prepFlags: ::core::ffi::c_uint,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3Prepare16(
            db,
            zSql,
            nBytes,
            SQLITE_PREPARE_SAVESQL as u32_0
                | prepFlags as u32_0 & SQLITE_PREPARE_MASK as u32_0,
            ppStmt,
            pzTail,
        );
        return rc;
    }
}
