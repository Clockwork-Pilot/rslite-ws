use c2rust_bitfields::BitfieldStruct;
use ::libc;
extern "C" {
    pub type VdbeSorter;
    pub type BtCursor;
    pub type Btree;
    pub type RenameToken;
    pub type VtabCtx;
    pub type sqlite3_mutex;
    pub type RowSet;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_msize(_: *mut ::core::ffi::c_void) -> sqlite3_uint64;
    fn sqlite3_str_appendf(
        _: *mut sqlite3_str,
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3BtreePayload(
        _: *mut BtCursor,
        offset: u32_0,
        amt: u32_0,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreePayloadFetch(
        _: *mut BtCursor,
        pAmt: *mut u32_0,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3BtreeMaxRecordSize(_: *mut BtCursor) -> sqlite3_int64;
    fn sqlite3CorruptError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRaw(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3Realloc(_: *mut ::core::ffi::c_void, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbReallocOrFree(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: u64_0,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbMallocSize(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3IsNaN(_: ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3ErrorToParser(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3RowSetInit(_: *mut sqlite3) -> *mut RowSet;
    fn sqlite3RowSetDelete(_: *mut ::core::ffi::c_void);
    fn sqlite3RowSetClear(_: *mut ::core::ffi::c_void);
    fn sqlite3Int64ToText(_: i64_0, _: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3AtoF(
        z: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_double,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3Atoi64(
        _: *const ::core::ffi::c_char,
        _: *mut i64_0,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3DecOrHexToI64(
        _: *const ::core::ffi::c_char,
        _: *mut i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3HexToBlob(
        _: *mut sqlite3,
        z: *const ::core::ffi::c_char,
        n: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3ValueApplyAffinity(_: *mut sqlite3_value, _: u8_0, _: u8_0);
    fn sqlite3AffinityType(
        _: *const ::core::ffi::c_char,
        _: *mut Column,
    ) -> ::core::ffi::c_char;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3RCStrUnref(_: *mut ::core::ffi::c_void);
    fn sqlite3StrAccumInit(
        _: *mut StrAccum,
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3VdbeMemTranslate(_: *mut Mem, _: u8_0) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemHandleBom(pMem: *mut Mem) -> ::core::ffi::c_int;
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
    pub trace: C2Rust_Unnamed_23,
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
pub type i16_0 = ::core::ffi::c_short;
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
pub type i8_0 = ::core::ffi::c_schar;
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
pub type LogEst = ::core::ffi::c_short;
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
pub union C2Rust_Unnamed_23 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_str {
    pub db: *mut sqlite3,
    pub zText: *mut ::core::ffi::c_char,
    pub nAlloc: u32_0,
    pub mxAlloc: u32_0,
    pub nChar: u32_0,
    pub accError: u8_0,
    pub printfFlags: u8_0

}
pub type size_t = usize;
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
pub type StrAccum = sqlite3_str;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueNewStat4Ctx {
    pub pParse: *mut Parse,
    pub pIdx: *mut Index,
    pub ppRec: *mut *mut UnpackedRecord,
    pub iVal: ::core::ffi::c_int

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LENGTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_UTF16_ALIGNED: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const LARGEST_INT64: i64_0 = 0xffffffff as i64_0
    | (0x7fffffff as ::core::ffi::c_int as i64_0) << 32 as ::core::ffi::c_int;
pub const SMALLEST_INT64: i64_0 = -1 as ::core::ffi::c_int as i64_0 - LARGEST_INT64;
unsafe extern "C" fn vdbeMemRenderNum(
    mut sz: ::core::ffi::c_int,
    mut zBuf: *mut ::core::ffi::c_char,
    mut p: *mut Mem,
) {
    unsafe {
        let mut acc: StrAccum = StrAccum {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        if (*p).flags as ::core::ffi::c_int & MEM_Int != 0 {
            (*p).n = sqlite3Int64ToText((*p).u.i, zBuf);
        } else {
            sqlite3StrAccumInit(
                &raw mut acc,
                ::core::ptr::null_mut::<sqlite3>(),
                zBuf,
                sz,
                0 as ::core::ffi::c_int,
            );
            sqlite3_str_appendf(
                &raw mut acc,
                b"%!.15g\0".as_ptr() as *const ::core::ffi::c_char,
                if (*p).flags as ::core::ffi::c_int & MEM_IntReal
                    != 0 as ::core::ffi::c_int
                {
                    (*p).u.i as ::core::ffi::c_double
                } else {
                    (*p).u.r
                },
            );
            *zBuf.offset(acc.nChar as isize) = 0 as ::core::ffi::c_char;
            (*p).n = acc.nChar as ::core::ffi::c_int;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeChangeEncoding(
    mut pMem: *mut Mem,
    mut desiredEnc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if (*pMem).flags as ::core::ffi::c_int & MEM_Str == 0 {
            (*pMem).enc = desiredEnc as u8_0;
            return SQLITE_OK;
        }
        if (*pMem).enc as ::core::ffi::c_int == desiredEnc {
            return SQLITE_OK;
        }
        rc = sqlite3VdbeMemTranslate(pMem, desiredEnc as u8_0);
        return rc;
    }
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sqlite3VdbeMemGrow(
    mut pMem: *mut Mem,
    mut n: ::core::ffi::c_int,
    mut bPreserve: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pMem).szMalloc > 0 as ::core::ffi::c_int && bPreserve != 0
            && (*pMem).z == (*pMem).zMalloc
        {
            if !(*pMem).db.is_null() {
                (*pMem).zMalloc = sqlite3DbReallocOrFree(
                    (*pMem).db,
                    (*pMem).z as *mut ::core::ffi::c_void,
                    n as u64_0,
                ) as *mut ::core::ffi::c_char;
                (*pMem).z = (*pMem).zMalloc;
            } else {
                (*pMem).zMalloc = sqlite3Realloc(
                    (*pMem).z as *mut ::core::ffi::c_void,
                    n as u64_0,
                ) as *mut ::core::ffi::c_char;
                if (*pMem).zMalloc.is_null() {
                    sqlite3_free((*pMem).z as *mut ::core::ffi::c_void);
                }
                (*pMem).z = (*pMem).zMalloc;
            }
            bPreserve = 0 as ::core::ffi::c_int;
        } else {
            if (*pMem).szMalloc > 0 as ::core::ffi::c_int {
                sqlite3DbFreeNN((*pMem).db, (*pMem).zMalloc as *mut ::core::ffi::c_void);
            }
            (*pMem).zMalloc = sqlite3DbMallocRaw((*pMem).db, n as u64_0)
                as *mut ::core::ffi::c_char;
        }
        if (*pMem).zMalloc.is_null() {
            sqlite3VdbeMemSetNull(pMem);
            (*pMem).z = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*pMem).szMalloc = 0 as ::core::ffi::c_int;
            return SQLITE_NOMEM_BKPT;
        } else {
            (*pMem).szMalloc = sqlite3DbMallocSize(
                (*pMem).db,
                (*pMem).zMalloc as *const ::core::ffi::c_void,
            );
        }
        if bPreserve != 0 && !(*pMem).z.is_null() {
            memcpy(
                (*pMem).zMalloc as *mut ::core::ffi::c_void,
                (*pMem).z as *const ::core::ffi::c_void,
                (*pMem).n as size_t,
            );
        }
        if (*pMem).flags as ::core::ffi::c_int & MEM_Dyn != 0 as ::core::ffi::c_int {
            (*pMem)
                .xDel
                .expect(
                    "non-null function pointer",
                )((*pMem).z as *mut ::core::ffi::c_void);
        }
        (*pMem).z = (*pMem).zMalloc;
        (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int
            & !(MEM_Dyn | MEM_Ephem | MEM_Static)) as u16_0;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemClearAndResize(
    mut pMem: *mut Mem,
    mut szNew: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pMem).szMalloc < szNew {
            return sqlite3VdbeMemGrow(pMem, szNew, 0 as ::core::ffi::c_int);
        }
        (*pMem).z = (*pMem).zMalloc;
        (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int
            & (MEM_Null | MEM_Int | MEM_Real | MEM_IntReal)) as u16_0;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemZeroTerminateIfAble(mut pMem: *mut Mem) {
    unsafe {
        if (*pMem).flags as ::core::ffi::c_int
            & (MEM_Str | MEM_Term | MEM_Ephem | MEM_Static) != MEM_Str
        {
            return;
        }
        if (*pMem).enc as ::core::ffi::c_int != SQLITE_UTF8 {
            return;
        }
        if (*pMem).flags as ::core::ffi::c_int & MEM_Dyn != 0 {
            if (*pMem).xDel
                == Some(
                    sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                )
                && sqlite3_msize((*pMem).z as *mut ::core::ffi::c_void)
                    >= ((*pMem).n + 1 as ::core::ffi::c_int) as u64_0
            {
                *(*pMem).z.offset((*pMem).n as isize) = 0 as ::core::ffi::c_char;
                (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int | MEM_Term)
                    as u16_0;
                return;
            }
            if (*pMem).xDel
                == Some(
                    sqlite3RCStrUnref
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                )
            {
                (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int | MEM_Term)
                    as u16_0;
                return;
            }
        } else if (*pMem).szMalloc >= (*pMem).n + 1 as ::core::ffi::c_int {
            *(*pMem).z.offset((*pMem).n as isize) = 0 as ::core::ffi::c_char;
            (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int | MEM_Term) as u16_0;
            return;
        }
    }
}
#[inline(never)]
unsafe extern "C" fn vdbeMemAddTerminator(mut pMem: *mut Mem) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3VdbeMemGrow(
            pMem,
            (*pMem).n + 3 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        ) != 0
        {
            return SQLITE_NOMEM_BKPT;
        }
        *(*pMem).z.offset((*pMem).n as isize) = 0 as ::core::ffi::c_char;
        *(*pMem).z.offset(((*pMem).n + 1 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_char;
        *(*pMem).z.offset(((*pMem).n + 2 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_char;
        (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int | MEM_Term) as u16_0;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemMakeWriteable(
    mut pMem: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pMem).flags as ::core::ffi::c_int & (MEM_Str | MEM_Blob)
            != 0 as ::core::ffi::c_int
        {
            if if (*pMem).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                sqlite3VdbeMemExpandBlob(pMem)
            } else {
                0 as ::core::ffi::c_int
            } != 0
            {
                return SQLITE_NOMEM;
            }
            if (*pMem).szMalloc == 0 as ::core::ffi::c_int
                || (*pMem).z != (*pMem).zMalloc
            {
                let mut rc: ::core::ffi::c_int = vdbeMemAddTerminator(pMem);
                if rc != 0 {
                    return rc;
                }
            }
        }
        (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int & !MEM_Ephem) as u16_0;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemExpandBlob(
    mut pMem: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nByte: ::core::ffi::c_int = 0;
        nByte = (*pMem).n + (*pMem).u.nZero;
        if nByte <= 0 as ::core::ffi::c_int {
            if (*pMem).flags as ::core::ffi::c_int & MEM_Blob == 0 as ::core::ffi::c_int
            {
                return SQLITE_OK;
            }
            nByte = 1 as ::core::ffi::c_int;
        }
        if sqlite3VdbeMemGrow(pMem, nByte, 1 as ::core::ffi::c_int) != 0 {
            return SQLITE_NOMEM_BKPT;
        }
        memset(
            (*pMem).z.offset((*pMem).n as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (*pMem).u.nZero as size_t,
        );
        (*pMem).n += (*pMem).u.nZero;
        (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int & !(MEM_Zero | MEM_Term))
            as u16_0;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemNulTerminate(
    mut pMem: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pMem).flags as ::core::ffi::c_int & (MEM_Term | MEM_Str) != MEM_Str {
            return SQLITE_OK
        } else {
            return vdbeMemAddTerminator(pMem)
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemStringify(
    mut pMem: *mut Mem,
    mut enc: u8_0,
    mut bForce: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let nByte: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
        if sqlite3VdbeMemClearAndResize(pMem, nByte) != 0 {
            (*pMem).enc = 0 as u8_0;
            return SQLITE_NOMEM_BKPT;
        }
        vdbeMemRenderNum(nByte, (*pMem).z, pMem);
        (*pMem).enc = SQLITE_UTF8 as u8_0;
        (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int | (MEM_Str | MEM_Term))
            as u16_0;
        if bForce != 0 {
            (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int
                & !(MEM_Int | MEM_Real | MEM_IntReal)) as u16_0;
        }
        sqlite3VdbeChangeEncoding(pMem, enc as ::core::ffi::c_int);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemFinalize(
    mut pMem: *mut Mem,
    mut pFunc: *mut FuncDef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ctx: sqlite3_context = sqlite3_context {
            pOut: ::core::ptr::null_mut::<Mem>(),
            pFunc: ::core::ptr::null_mut::<FuncDef>(),
            pMem: ::core::ptr::null_mut::<Mem>(),
            pVdbe: ::core::ptr::null_mut::<Vdbe>(),
            iOp: 0,
            isError: 0,
            enc: 0,
            skipFlag: 0,
            argc: 0,
            argv: []

        };
        let mut t: Mem = Mem {
            u: MemValue { r: 0. },
            z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            n: 0,
            flags: 0,
            enc: 0,
            eSubtype: 0,
            db: ::core::ptr::null_mut::<sqlite3>(),
            szMalloc: 0,
            uTemp: 0,
            zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            xDel: None

        };
        memset(
            &raw mut ctx as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sqlite3_context>() as size_t,
        );
        memset(
            &raw mut t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Mem>() as size_t,
        );
        t.flags = MEM_Null as u16_0;
        t.db = (*pMem).db;
        ctx.pOut = &raw mut t;
        ctx.pMem = pMem;
        ctx.pFunc = pFunc;
        ctx.enc = (*t.db).enc;
        (*pFunc).xFinalize.expect("non-null function pointer")(&raw mut ctx);
        if (*pMem).szMalloc > 0 as ::core::ffi::c_int {
            sqlite3DbFreeNN((*pMem).db, (*pMem).zMalloc as *mut ::core::ffi::c_void);
        }
        memcpy(
            pMem as *mut ::core::ffi::c_void,
            &raw mut t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<Mem>() as size_t,
        );
        return ctx.isError;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemAggValue(
    mut pAccum: *mut Mem,
    mut pOut: *mut Mem,
    mut pFunc: *mut FuncDef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ctx: sqlite3_context = sqlite3_context {
            pOut: ::core::ptr::null_mut::<Mem>(),
            pFunc: ::core::ptr::null_mut::<FuncDef>(),
            pMem: ::core::ptr::null_mut::<Mem>(),
            pVdbe: ::core::ptr::null_mut::<Vdbe>(),
            iOp: 0,
            isError: 0,
            enc: 0,
            skipFlag: 0,
            argc: 0,
            argv: []

        };
        memset(
            &raw mut ctx as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sqlite3_context>() as size_t,
        );
        sqlite3VdbeMemSetNull(pOut);
        ctx.pOut = pOut;
        ctx.pMem = pAccum;
        ctx.pFunc = pFunc;
        ctx.enc = (*(*pAccum).db).enc;
        (*pFunc).xValue.expect("non-null function pointer")(&raw mut ctx);
        return ctx.isError;
    }
}
#[inline(never)]
unsafe extern "C" fn vdbeMemClearExternAndSetNull(mut p: *mut Mem) {
    unsafe {
        if (*p).flags as ::core::ffi::c_int & MEM_Agg != 0 {
            sqlite3VdbeMemFinalize(p, (*p).u.pDef);
        }
        if (*p).flags as ::core::ffi::c_int & MEM_Dyn != 0 {
            (*p)
                .xDel
                .expect("non-null function pointer")((*p).z as *mut ::core::ffi::c_void);
        }
        (*p).flags = MEM_Null as u16_0;
    }
}
#[inline(never)]
unsafe extern "C" fn vdbeMemClear(mut p: *mut Mem) {
    unsafe {
        if (*p).flags as ::core::ffi::c_int & (MEM_Agg | MEM_Dyn)
            != 0 as ::core::ffi::c_int
        {
            vdbeMemClearExternAndSetNull(p);
        }
        if (*p).szMalloc != 0 {
            sqlite3DbFreeNN((*p).db, (*p).zMalloc as *mut ::core::ffi::c_void);
            (*p).szMalloc = 0 as ::core::ffi::c_int;
        }
        (*p).z = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemRelease(mut p: *mut Mem) {
    unsafe {
        if (*p).flags as ::core::ffi::c_int & (MEM_Agg | MEM_Dyn)
            != 0 as ::core::ffi::c_int || (*p).szMalloc != 0
        {
            vdbeMemClear(p);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemReleaseMalloc(mut p: *mut Mem) {
    unsafe {
        if (*p).szMalloc != 0 {
            vdbeMemClear(p);
        }
    }
}
#[inline(never)]
unsafe extern "C" fn memIntValue(mut pMem: *const Mem) -> i64_0 {
    unsafe {
        let mut value: i64_0 = 0 as i64_0;
        sqlite3Atoi64((*pMem).z, &raw mut value, (*pMem).n, (*pMem).enc);
        return value;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeIntValue(mut pMem: *const Mem) -> i64_0 {
    unsafe {
        let mut flags: ::core::ffi::c_int = 0;
        flags = (*pMem).flags as ::core::ffi::c_int;
        if flags & (MEM_Int | MEM_IntReal) != 0 {
            return (*pMem).u.i
        } else if flags & MEM_Real != 0 {
            return sqlite3RealToI64((*pMem).u.r)
        } else if flags & (MEM_Str | MEM_Blob) != 0 as ::core::ffi::c_int
            && !(*pMem).z.is_null()
        {
            return memIntValue(pMem)
        } else {
            return 0 as i64_0
        };
    }
}
#[inline(never)]
unsafe extern "C" fn memRealValue(mut pMem: *mut Mem) -> ::core::ffi::c_double {
    unsafe {
        let mut val: ::core::ffi::c_double = 0 as ::core::ffi::c_int
            as ::core::ffi::c_double;
        sqlite3AtoF((*pMem).z, &raw mut val, (*pMem).n, (*pMem).enc);
        return val;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeRealValue(
    mut pMem: *mut Mem,
) -> ::core::ffi::c_double {
    unsafe {
        if (*pMem).flags as ::core::ffi::c_int & MEM_Real != 0 {
            return (*pMem).u.r
        } else if (*pMem).flags as ::core::ffi::c_int & (MEM_Int | MEM_IntReal) != 0 {
            return (*pMem).u.i as ::core::ffi::c_double
        } else if (*pMem).flags as ::core::ffi::c_int & (MEM_Str | MEM_Blob) != 0 {
            return memRealValue(pMem)
        } else {
            return 0 as ::core::ffi::c_int as ::core::ffi::c_double
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeBooleanValue(
    mut pMem: *mut Mem,
    mut ifNull: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pMem).flags as ::core::ffi::c_int & (MEM_Int | MEM_IntReal) != 0 {
            return ((*pMem).u.i != 0 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        }
        if (*pMem).flags as ::core::ffi::c_int & MEM_Null != 0 {
            return ifNull;
        }
        return (sqlite3VdbeRealValue(pMem) != 0.0f64) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeIntegerAffinity(mut pMem: *mut Mem) {
    unsafe {
        if (*pMem).flags as ::core::ffi::c_int & MEM_IntReal != 0 {
            (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int
                & !(MEM_TypeMask | MEM_Zero) | 0x4 as ::core::ffi::c_int) as u16_0;
        } else {
            let mut ix: i64_0 = sqlite3RealToI64((*pMem).u.r);
            if (*pMem).u.r == ix as ::core::ffi::c_double && ix > SMALLEST_INT64
                && ix < LARGEST_INT64
            {
                (*pMem).u.i = ix;
                (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int
                    & !(MEM_TypeMask | MEM_Zero) | 0x4 as ::core::ffi::c_int) as u16_0;
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemIntegerify(
    mut pMem: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        (*pMem).u.i = sqlite3VdbeIntValue(pMem);
        (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int & !(MEM_TypeMask | MEM_Zero)
            | 0x4 as ::core::ffi::c_int) as u16_0;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemRealify(
    mut pMem: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        (*pMem).u.r = sqlite3VdbeRealValue(pMem);
        (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int & !(MEM_TypeMask | MEM_Zero)
            | 0x8 as ::core::ffi::c_int) as u16_0;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RealSameAsInt(
    mut r1: ::core::ffi::c_double,
    mut i: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut r2: ::core::ffi::c_double = i as ::core::ffi::c_double;
        return (r1 == 0.0f64
            || memcmp(
                &raw mut r1 as *const ::core::ffi::c_void,
                &raw mut r2 as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
            ) == 0 as ::core::ffi::c_int
                && i >= -2251799813685248 as ::core::ffi::c_longlong
                && i < 2251799813685248 as ::core::ffi::c_longlong)
            as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RealToI64(mut r: ::core::ffi::c_double) -> i64_0 {
    unsafe {
        if r < -9223372036854774784.0f64 {
            return SMALLEST_INT64;
        }
        if r > 9223372036854774784.0f64 {
            return LARGEST_INT64;
        }
        return r as i64_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemNumerify(
    mut pMem: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pMem).flags as ::core::ffi::c_int
            & (MEM_Int | MEM_Real | MEM_IntReal | MEM_Null) == 0 as ::core::ffi::c_int
        {
            let mut rc: ::core::ffi::c_int = 0;
            let mut ix: sqlite3_int64 = 0;
            rc = sqlite3AtoF((*pMem).z, &raw mut (*pMem).u.r, (*pMem).n, (*pMem).enc);
            if (rc == 0 as ::core::ffi::c_int || rc == 1 as ::core::ffi::c_int)
                && sqlite3Atoi64((*pMem).z, &raw mut ix, (*pMem).n, (*pMem).enc)
                    <= 1 as ::core::ffi::c_int
                || {
                    ix = sqlite3RealToI64((*pMem).u.r) as sqlite3_int64;
                    sqlite3RealSameAsInt((*pMem).u.r, ix) != 0
                }
            {
                (*pMem).u.i = ix as i64_0;
                (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int
                    & !(MEM_TypeMask | MEM_Zero) | 0x4 as ::core::ffi::c_int) as u16_0;
            } else {
                (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int
                    & !(MEM_TypeMask | MEM_Zero) | 0x8 as ::core::ffi::c_int) as u16_0;
            }
        }
        (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int
            & !(MEM_Str | MEM_Blob | MEM_Zero)) as u16_0;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemCast(
    mut pMem: *mut Mem,
    mut aff: u8_0,
    mut encoding: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pMem).flags as ::core::ffi::c_int & MEM_Null != 0 {
            return SQLITE_OK;
        }
        match aff as ::core::ffi::c_int {
            SQLITE_AFF_BLOB => {
                if (*pMem).flags as ::core::ffi::c_int & MEM_Blob
                    == 0 as ::core::ffi::c_int
                {
                    sqlite3ValueApplyAffinity(
                        pMem as *mut sqlite3_value,
                        SQLITE_AFF_TEXT as u8_0,
                        encoding,
                    );
                    if (*pMem).flags as ::core::ffi::c_int & MEM_Str != 0 {
                        (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int
                            & !(MEM_TypeMask | MEM_Zero) | 0x10 as ::core::ffi::c_int)
                            as u16_0;
                    }
                } else {
                    (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int
                        & !(MEM_TypeMask & !MEM_Blob)) as u16_0;
                }
            }
            SQLITE_AFF_NUMERIC => {
                sqlite3VdbeMemNumerify(pMem);
            }
            SQLITE_AFF_INTEGER => {
                sqlite3VdbeMemIntegerify(pMem);
            }
            SQLITE_AFF_REAL => {
                sqlite3VdbeMemRealify(pMem);
            }
            _ => {
                let mut rc: ::core::ffi::c_int = 0;
                (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int
                    | ((*pMem).flags as ::core::ffi::c_int & MEM_Blob)
                        >> 3 as ::core::ffi::c_int) as u16_0;
                sqlite3ValueApplyAffinity(
                    pMem as *mut sqlite3_value,
                    SQLITE_AFF_TEXT as u8_0,
                    encoding,
                );
                (*pMem).flags = ((*pMem).flags as ::core::ffi::c_int
                    & !(MEM_Int | MEM_Real | MEM_IntReal | MEM_Blob | MEM_Zero))
                    as u16_0;
                if encoding as ::core::ffi::c_int != SQLITE_UTF8 {
                    (*pMem).n &= !(1 as ::core::ffi::c_int);
                }
                rc = sqlite3VdbeChangeEncoding(pMem, encoding as ::core::ffi::c_int);
                if rc != 0 {
                    return rc;
                }
                sqlite3VdbeMemZeroTerminateIfAble(pMem);
            }
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemInit(
    mut pMem: *mut Mem,
    mut db: *mut sqlite3,
    mut flags: u16_0,
) {
    unsafe {
        (*pMem).flags = flags;
        (*pMem).db = db;
        (*pMem).szMalloc = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemSetNull(mut pMem: *mut Mem) {
    unsafe {
        if (*pMem).flags as ::core::ffi::c_int & (MEM_Agg | MEM_Dyn)
            != 0 as ::core::ffi::c_int
        {
            vdbeMemClearExternAndSetNull(pMem);
        } else {
            (*pMem).flags = MEM_Null as u16_0;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ValueSetNull(mut p: *mut sqlite3_value) {
    unsafe {
        sqlite3VdbeMemSetNull(p as *mut Mem);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemSetZeroBlob(
    mut pMem: *mut Mem,
    mut n: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3VdbeMemRelease(pMem);
        (*pMem).flags = (MEM_Blob | MEM_Zero) as u16_0;
        (*pMem).n = 0 as ::core::ffi::c_int;
        if n < 0 as ::core::ffi::c_int {
            n = 0 as ::core::ffi::c_int;
        }
        (*pMem).u.nZero = n;
        (*pMem).enc = SQLITE_UTF8 as u8_0;
        (*pMem).z = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
#[inline(never)]
unsafe extern "C" fn vdbeReleaseAndSetInt64(mut pMem: *mut Mem, mut val: i64_0) {
    unsafe {
        sqlite3VdbeMemSetNull(pMem);
        (*pMem).u.i = val;
        (*pMem).flags = MEM_Int as u16_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemSetInt64(mut pMem: *mut Mem, mut val: i64_0) {
    unsafe {
        if (*pMem).flags as ::core::ffi::c_int & (MEM_Agg | MEM_Dyn)
            != 0 as ::core::ffi::c_int
        {
            vdbeReleaseAndSetInt64(pMem, val);
        } else {
            (*pMem).u.i = val;
            (*pMem).flags = MEM_Int as u16_0;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MemSetArrayInt64(
    mut aMem: *mut sqlite3_value,
    mut iIdx: ::core::ffi::c_int,
    mut val: i64_0,
) {
    unsafe {
        sqlite3VdbeMemSetInt64(aMem.offset(iIdx as isize) as *mut Mem, val);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3NoopDestructor(mut p: *mut ::core::ffi::c_void) {}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemSetPointer(
    mut pMem: *mut Mem,
    mut pPtr: *mut ::core::ffi::c_void,
    mut zPType: *const ::core::ffi::c_char,
    mut xDestructor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    unsafe {
        vdbeMemClear(pMem);
        (*pMem).u.zPType = if !zPType.is_null() {
            zPType
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        };
        (*pMem).z = pPtr as *mut ::core::ffi::c_char;
        (*pMem).flags = (MEM_Null | MEM_Dyn | MEM_Subtype | MEM_Term) as u16_0;
        (*pMem).eSubtype = 'p' as i32 as u8_0;
        (*pMem).xDel = if xDestructor.is_some() {
            xDestructor
        } else {
            Some(
                sqlite3NoopDestructor
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            )
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemSetDouble(
    mut pMem: *mut Mem,
    mut val: ::core::ffi::c_double,
) {
    unsafe {
        sqlite3VdbeMemSetNull(pMem);
        if sqlite3IsNaN(val) == 0 {
            (*pMem).u.r = val;
            (*pMem).flags = MEM_Real as u16_0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemSetRowSet(
    mut pMem: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = (*pMem).db;
        let mut p: *mut RowSet = ::core::ptr::null_mut::<RowSet>();
        sqlite3VdbeMemRelease(pMem);
        p = sqlite3RowSetInit(db);
        if p.is_null() {
            return SQLITE_NOMEM;
        }
        (*pMem).z = p as *mut ::core::ffi::c_char;
        (*pMem).flags = (MEM_Blob | MEM_Dyn) as u16_0;
        (*pMem).xDel = Some(
            sqlite3RowSetDelete as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemTooBig(mut p: *mut Mem) -> ::core::ffi::c_int {
    unsafe {
        if (*p).flags as ::core::ffi::c_int & (MEM_Str | MEM_Blob) != 0 {
            let mut n: ::core::ffi::c_int = (*p).n;
            if (*p).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                n += (*p).u.nZero;
            }
            return (n > (*(*p).db).aLimit[SQLITE_LIMIT_LENGTH as usize])
                as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn vdbeClrCopy(
    mut pTo: *mut Mem,
    mut pFrom: *const Mem,
    mut eType: ::core::ffi::c_int,
) {
    unsafe {
        vdbeMemClearExternAndSetNull(pTo);
        sqlite3VdbeMemShallowCopy(pTo, pFrom, eType);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemShallowCopy(
    mut pTo: *mut Mem,
    mut pFrom: *const Mem,
    mut srcType: ::core::ffi::c_int,
) {
    unsafe {
        if (*pTo).flags as ::core::ffi::c_int & (MEM_Agg | MEM_Dyn)
            != 0 as ::core::ffi::c_int
        {
            vdbeClrCopy(pTo, pFrom, srcType);
            return;
        }
        memcpy(
            pTo as *mut ::core::ffi::c_void,
            pFrom as *const ::core::ffi::c_void,
            MEMCELLSIZE as size_t,
        );
        if (*pFrom).flags as ::core::ffi::c_int & MEM_Static == 0 as ::core::ffi::c_int {
            (*pTo).flags = ((*pTo).flags as ::core::ffi::c_int
                & !(MEM_Dyn | MEM_Static | MEM_Ephem)) as u16_0;
            (*pTo).flags = ((*pTo).flags as ::core::ffi::c_int | srcType) as u16_0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemCopy(
    mut pTo: *mut Mem,
    mut pFrom: *const Mem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pTo).flags as ::core::ffi::c_int & (MEM_Agg | MEM_Dyn)
            != 0 as ::core::ffi::c_int
        {
            vdbeMemClearExternAndSetNull(pTo);
        }
        memcpy(
            pTo as *mut ::core::ffi::c_void,
            pFrom as *const ::core::ffi::c_void,
            MEMCELLSIZE as size_t,
        );
        (*pTo).flags = ((*pTo).flags as ::core::ffi::c_int & !MEM_Dyn) as u16_0;
        if (*pTo).flags as ::core::ffi::c_int & (MEM_Str | MEM_Blob) != 0 {
            if 0 as ::core::ffi::c_int
                == (*pFrom).flags as ::core::ffi::c_int & MEM_Static
            {
                (*pTo).flags = ((*pTo).flags as ::core::ffi::c_int | MEM_Ephem) as u16_0;
                rc = sqlite3VdbeMemMakeWriteable(pTo);
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemMove(mut pTo: *mut Mem, mut pFrom: *mut Mem) {
    unsafe {
        sqlite3VdbeMemRelease(pTo);
        memcpy(
            pTo as *mut ::core::ffi::c_void,
            pFrom as *const ::core::ffi::c_void,
            ::core::mem::size_of::<Mem>() as size_t,
        );
        (*pFrom).flags = MEM_Null as u16_0;
        (*pFrom).szMalloc = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemSetStr(
    mut pMem: *mut Mem,
    mut z: *const ::core::ffi::c_char,
    mut n: i64_0,
    mut enc: u8_0,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nByte: i64_0 = n;
        let mut iLimit: ::core::ffi::c_int = 0;
        let mut flags: u16_0 = 0;
        if z.is_null() {
            sqlite3VdbeMemSetNull(pMem);
            return SQLITE_OK;
        }
        if !(*pMem).db.is_null() {
            iLimit = (*(*pMem).db).aLimit[SQLITE_LIMIT_LENGTH as usize];
        } else {
            iLimit = SQLITE_MAX_LENGTH;
        }
        if nByte < 0 as ::core::ffi::c_longlong {
            if enc as ::core::ffi::c_int == SQLITE_UTF8 {
                nByte = strlen(z) as i64_0;
            } else {
                nByte = 0 as i64_0;
                while nByte <= iLimit as ::core::ffi::c_longlong
                    && *z.offset(nByte as isize) as ::core::ffi::c_int
                        | *z
                            .offset(
                                (nByte as ::core::ffi::c_longlong
                                    + 1 as ::core::ffi::c_longlong) as isize,
                            ) as ::core::ffi::c_int != 0
                {
                    nByte += 2 as ::core::ffi::c_longlong;
                }
            }
            flags = (MEM_Str | MEM_Term) as u16_0;
        } else if enc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            flags = MEM_Blob as u16_0;
            enc = SQLITE_UTF8 as u8_0;
        } else {
            flags = MEM_Str as u16_0;
        }
        if nByte > iLimit as ::core::ffi::c_longlong {
            if xDel.is_some()
                && xDel
                    != ::core::mem::transmute::<
                        ::libc::intptr_t,
                        sqlite3_destructor_type,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t)
            {
                if xDel
                    == ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        sqlite3_destructor_type,
                    >(
                        Some(
                            sqlite3RowSetClear
                                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                        ),
                    )
                {
                    sqlite3DbFree((*pMem).db, z as *mut ::core::ffi::c_void);
                } else {
                    xDel
                        .expect(
                            "non-null function pointer",
                        )(z as *mut ::core::ffi::c_void);
                }
            }
            sqlite3VdbeMemSetNull(pMem);
            return sqlite3ErrorToParser((*pMem).db, SQLITE_TOOBIG);
        }
        if xDel
            == ::core::mem::transmute::<
                ::libc::intptr_t,
                sqlite3_destructor_type,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t)
        {
            let mut nAlloc: i64_0 = nByte;
            if flags as ::core::ffi::c_int & MEM_Term != 0 {
                nAlloc
                    += (if enc as ::core::ffi::c_int == SQLITE_UTF8 {
                        1 as ::core::ffi::c_int
                    } else {
                        2 as ::core::ffi::c_int
                    }) as ::core::ffi::c_longlong;
            }
            if sqlite3VdbeMemClearAndResize(
                pMem,
                (if nAlloc > 32 as ::core::ffi::c_longlong {
                    nAlloc as ::core::ffi::c_longlong
                } else {
                    32 as ::core::ffi::c_longlong
                }) as ::core::ffi::c_int,
            ) != 0
            {
                return SQLITE_NOMEM_BKPT;
            }
            memcpy(
                (*pMem).z as *mut ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                nAlloc as size_t,
            );
        } else {
            sqlite3VdbeMemRelease(pMem);
            (*pMem).z = z as *mut ::core::ffi::c_char;
            if xDel
                == ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    sqlite3_destructor_type,
                >(
                    Some(
                        sqlite3RowSetClear
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                )
            {
                (*pMem).zMalloc = (*pMem).z;
                (*pMem).szMalloc = sqlite3DbMallocSize(
                    (*pMem).db,
                    (*pMem).zMalloc as *const ::core::ffi::c_void,
                );
            } else {
                (*pMem).xDel = xDel;
                flags = (flags as ::core::ffi::c_int
                    | if xDel.is_none() { MEM_Static } else { MEM_Dyn }) as u16_0;
            }
        }
        (*pMem).n = (nByte as ::core::ffi::c_longlong
            & 0x7fffffff as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        (*pMem).flags = flags;
        (*pMem).enc = enc;
        if enc as ::core::ffi::c_int > SQLITE_UTF8 && sqlite3VdbeMemHandleBom(pMem) != 0
        {
            return SQLITE_NOMEM_BKPT;
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemFromBtree(
    mut pCur: *mut BtCursor,
    mut offset: u32_0,
    mut amt: u32_0,
    mut pMem: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        (*pMem).flags = MEM_Null as u16_0;
        if sqlite3BtreeMaxRecordSize(pCur)
            < offset.wrapping_add(amt) as ::core::ffi::c_longlong
        {
            return sqlite3CorruptError(1281 as ::core::ffi::c_int);
        }
        rc = sqlite3VdbeMemClearAndResize(
            pMem,
            (amt as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint)
                as ::core::ffi::c_int,
        );
        if SQLITE_OK == rc {
            rc = sqlite3BtreePayload(
                pCur,
                offset,
                amt,
                (*pMem).z as *mut ::core::ffi::c_void,
            );
            if rc == SQLITE_OK {
                *(*pMem).z.offset(amt as isize) = 0 as ::core::ffi::c_char;
                (*pMem).flags = MEM_Blob as u16_0;
                (*pMem).n = amt as ::core::ffi::c_int;
            } else {
                sqlite3VdbeMemRelease(pMem);
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMemFromBtreeZeroOffset(
    mut pCur: *mut BtCursor,
    mut amt: u32_0,
    mut pMem: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut available: u32_0 = 0 as u32_0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        (*pMem).z = sqlite3BtreePayloadFetch(pCur, &raw mut available)
            as *mut ::core::ffi::c_char;
        if amt <= available {
            (*pMem).flags = (MEM_Blob | MEM_Ephem) as u16_0;
            (*pMem).n = amt as ::core::ffi::c_int;
        } else {
            rc = sqlite3VdbeMemFromBtree(pCur, 0 as u32_0, amt, pMem);
        }
        return rc;
    }
}
#[inline(never)]
unsafe extern "C" fn valueToText(
    mut pVal: *mut sqlite3_value,
    mut enc: u8_0,
) -> *const ::core::ffi::c_void {
    unsafe {
        if (*pVal).flags as ::core::ffi::c_int & (MEM_Blob | MEM_Str) != 0 {
            if if (*pVal).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                sqlite3VdbeMemExpandBlob(pVal as *mut Mem)
            } else {
                0 as ::core::ffi::c_int
            } != 0
            {
                return ::core::ptr::null::<::core::ffi::c_void>();
            }
            (*pVal).flags = ((*pVal).flags as ::core::ffi::c_int | MEM_Str) as u16_0;
            if (*pVal).enc as ::core::ffi::c_int
                != enc as ::core::ffi::c_int & !SQLITE_UTF16_ALIGNED
            {
                sqlite3VdbeChangeEncoding(
                    pVal as *mut Mem,
                    enc as ::core::ffi::c_int & !SQLITE_UTF16_ALIGNED,
                );
            }
            if enc as ::core::ffi::c_int & SQLITE_UTF16_ALIGNED
                != 0 as ::core::ffi::c_int
                && 1 as ::core::ffi::c_int
                    == 1 as ::core::ffi::c_int
                        & (*pVal).z as ::core::ffi::c_long as ::core::ffi::c_int
            {
                if sqlite3VdbeMemMakeWriteable(pVal as *mut Mem) != SQLITE_OK {
                    return ::core::ptr::null::<::core::ffi::c_void>();
                }
            }
            sqlite3VdbeMemNulTerminate(pVal as *mut Mem);
        } else {
            sqlite3VdbeMemStringify(pVal as *mut Mem, enc, 0 as u8_0);
        }
        if (*pVal).enc as ::core::ffi::c_int
            == enc as ::core::ffi::c_int & !SQLITE_UTF16_ALIGNED
        {
            return (*pVal).z as *const ::core::ffi::c_void
        } else {
            return ::core::ptr::null::<::core::ffi::c_void>()
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ValueText(
    mut pVal: *mut sqlite3_value,
    mut enc: u8_0,
) -> *const ::core::ffi::c_void {
    unsafe {
        if pVal.is_null() {
            return ::core::ptr::null::<::core::ffi::c_void>();
        }
        if (*pVal).flags as ::core::ffi::c_int & (MEM_Str | MEM_Term)
            == MEM_Str | MEM_Term
            && (*pVal).enc as ::core::ffi::c_int == enc as ::core::ffi::c_int
        {
            return (*pVal).z as *const ::core::ffi::c_void;
        }
        if (*pVal).flags as ::core::ffi::c_int & MEM_Null != 0 {
            return ::core::ptr::null::<::core::ffi::c_void>();
        }
        return valueToText(pVal, enc);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ValueIsOfClass(
    mut pVal: *const sqlite3_value,
    mut xFree: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        if !pVal.is_null()
            && (*pVal).flags as ::core::ffi::c_int
                & (0x2 as ::core::ffi::c_int | 0x10 as ::core::ffi::c_int)
                != 0 as ::core::ffi::c_int
            && (*pVal).flags as ::core::ffi::c_int & MEM_Dyn != 0 as ::core::ffi::c_int
            && (*pVal).xDel == xFree
        {
            return 1 as ::core::ffi::c_int
        } else {
            return 0 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ValueNew(mut db: *mut sqlite3) -> *mut sqlite3_value {
    unsafe {
        let mut p: *mut Mem = sqlite3DbMallocZero(
            db,
            ::core::mem::size_of::<Mem>() as u64_0,
        ) as *mut Mem;
        if !p.is_null() {
            (*p).flags = MEM_Null as u16_0;
            (*p).db = db;
        }
        return p as *mut sqlite3_value;
    }
}
unsafe extern "C" fn valueNew(
    mut db: *mut sqlite3,
    mut p: *mut ValueNewStat4Ctx,
) -> *mut sqlite3_value {
    unsafe {
        return sqlite3ValueNew(db);
    }
}
unsafe extern "C" fn valueFromExpr(
    mut db: *mut sqlite3,
    mut pExpr: *const Expr,
    mut enc: u8_0,
    mut affinity: u8_0,
    mut ppVal: *mut *mut sqlite3_value,
    mut pCtx: *mut ValueNewStat4Ctx,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut op: ::core::ffi::c_int = 0;
        let mut zVal: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut negInt: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut zNeg: *const ::core::ffi::c_char = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        loop {
            op = (*pExpr).op as ::core::ffi::c_int;
            if !(op == TK_UPLUS || op == TK_SPAN) {
                break;
            }
            pExpr = (*pExpr).pLeft;
        }
        if op == TK_REGISTER {
            op = (*pExpr).op2 as ::core::ffi::c_int;
        }
        if op == TK_CAST {
            let mut aff: u8_0 = 0;
            aff = sqlite3AffinityType(
                (*pExpr).u.zToken,
                ::core::ptr::null_mut::<Column>(),
            ) as u8_0;
            rc = valueFromExpr(db, (*pExpr).pLeft, enc, aff, ppVal, pCtx);
            if !(*ppVal).is_null() {
                sqlite3VdbeMemCast(*ppVal, aff, enc);
                sqlite3ValueApplyAffinity(*ppVal, affinity, enc);
            }
            return rc;
        }
        if op == TK_UMINUS {
            let mut pLeft: *mut Expr = (*pExpr).pLeft;
            if (*pLeft).op as ::core::ffi::c_int == TK_INTEGER
                || (*pLeft).op as ::core::ffi::c_int == TK_FLOAT
            {
                if (*pLeft).flags & 0x800 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint
                    || *(*pLeft).u.zToken.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int != '0' as i32
                    || *(*pLeft).u.zToken.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int & !(0x20 as ::core::ffi::c_int)
                        != 'X' as i32
                {
                    pExpr = pLeft;
                    op = (*pExpr).op as ::core::ffi::c_int;
                    negInt = -1 as ::core::ffi::c_int;
                    zNeg = b"-\0".as_ptr() as *const ::core::ffi::c_char;
                }
            }
        }
        if op == TK_STRING || op == TK_FLOAT || op == TK_INTEGER {
            pVal = valueNew(db, pCtx);
            if pVal.is_null() {
                c2rust_current_block = 2278638688076196614;
            } else {
                if (*pExpr).flags & 0x800 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint
                {
                    sqlite3VdbeMemSetInt64(
                        pVal as *mut Mem,
                        (*pExpr).u.iValue as i64_0 * negInt as i64_0,
                    );
                    c2rust_current_block = 9853141518545631134;
                } else {
                    let mut iVal: i64_0 = 0;
                    if op == TK_INTEGER
                        && 0 as ::core::ffi::c_int
                            == sqlite3DecOrHexToI64((*pExpr).u.zToken, &raw mut iVal)
                    {
                        sqlite3VdbeMemSetInt64(pVal as *mut Mem, iVal * negInt as i64_0);
                        c2rust_current_block = 9853141518545631134;
                    } else {
                        zVal = sqlite3MPrintf(
                            db,
                            b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                            zNeg,
                            (*pExpr).u.zToken,
                        );
                        if zVal.is_null() {
                            c2rust_current_block = 2278638688076196614;
                        } else {
                            sqlite3ValueSetStr(
                                pVal,
                                -1 as ::core::ffi::c_int,
                                zVal as *const ::core::ffi::c_void,
                                SQLITE_UTF8 as u8_0,
                                Some(
                                    sqlite3RowSetClear
                                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                                ),
                            );
                            c2rust_current_block = 9853141518545631134;
                        }
                    }
                }
                match c2rust_current_block {
                    2278638688076196614 => {}
                    _ => {
                        if affinity as ::core::ffi::c_int == SQLITE_AFF_BLOB {
                            if op == TK_FLOAT {
                                sqlite3AtoF(
                                    (*pVal).z,
                                    &raw mut (*pVal).u.r,
                                    (*pVal).n,
                                    SQLITE_UTF8 as u8_0,
                                );
                                (*pVal).flags = MEM_Real as u16_0;
                            } else if op == TK_INTEGER {
                                sqlite3ValueApplyAffinity(
                                    pVal,
                                    SQLITE_AFF_NUMERIC as u8_0,
                                    SQLITE_UTF8 as u8_0,
                                );
                            }
                        } else {
                            sqlite3ValueApplyAffinity(
                                pVal,
                                affinity,
                                SQLITE_UTF8 as u8_0,
                            );
                        }
                        if (*pVal).flags as ::core::ffi::c_int
                            & (MEM_Int | MEM_IntReal | MEM_Real) != 0
                        {
                            (*pVal).flags = ((*pVal).flags as ::core::ffi::c_int
                                & !MEM_Str) as u16_0;
                        }
                        if enc as ::core::ffi::c_int != SQLITE_UTF8 {
                            rc = sqlite3VdbeChangeEncoding(
                                pVal as *mut Mem,
                                enc as ::core::ffi::c_int,
                            );
                        }
                        c2rust_current_block = 576355610076403033;
                    }
                }
            }
        } else if op == TK_UMINUS {
            if SQLITE_OK
                == valueFromExpr(db, (*pExpr).pLeft, enc, affinity, &raw mut pVal, pCtx)
                && !pVal.is_null()
            {
                sqlite3VdbeMemNumerify(pVal as *mut Mem);
                if (*pVal).flags as ::core::ffi::c_int & MEM_Real != 0 {
                    (*pVal).u.r = -(*pVal).u.r;
                } else if (*pVal).u.i == SMALLEST_INT64 {
                    (*pVal).u.r = -(SMALLEST_INT64 as ::core::ffi::c_double);
                    (*pVal).flags = ((*pVal).flags as ::core::ffi::c_int
                        & !(MEM_TypeMask | MEM_Zero) | 0x8 as ::core::ffi::c_int)
                        as u16_0;
                } else {
                    (*pVal).u.i = -(*pVal).u.i;
                }
                sqlite3ValueApplyAffinity(pVal, affinity, enc);
            }
            c2rust_current_block = 576355610076403033;
        } else if op == TK_NULL {
            pVal = valueNew(db, pCtx);
            if pVal.is_null() {
                c2rust_current_block = 2278638688076196614;
            } else {
                sqlite3VdbeMemSetNull(pVal as *mut Mem);
                c2rust_current_block = 576355610076403033;
            }
        } else if op == TK_BLOB {
            let mut nVal: ::core::ffi::c_int = 0;
            pVal = valueNew(db, pCtx);
            if pVal.is_null() {
                c2rust_current_block = 2278638688076196614;
            } else {
                zVal = (*pExpr).u.zToken.offset(2 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char;
                nVal = sqlite3Strlen30(zVal) - 1 as ::core::ffi::c_int;
                sqlite3VdbeMemSetStr(
                    pVal as *mut Mem,
                    sqlite3HexToBlob(db, zVal, nVal) as *const ::core::ffi::c_char,
                    (nVal / 2 as ::core::ffi::c_int) as i64_0,
                    0 as u8_0,
                    Some(
                        sqlite3RowSetClear
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
                c2rust_current_block = 576355610076403033;
            }
        } else {
            if op == TK_TRUEFALSE {
                pVal = valueNew(db, pCtx);
                if !pVal.is_null() {
                    (*pVal).flags = MEM_Int as u16_0;
                    (*pVal).u.i = (*(*pExpr)
                        .u
                        .zToken
                        .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as i64_0;
                    sqlite3ValueApplyAffinity(pVal, affinity, enc);
                }
            }
            c2rust_current_block = 576355610076403033;
        }
        match c2rust_current_block {
            2278638688076196614 => {
                sqlite3OomFault(db);
                sqlite3DbFree(db, zVal as *mut ::core::ffi::c_void);
                sqlite3ValueFree(pVal);
                return SQLITE_NOMEM_BKPT;
            }
            _ => {
                *ppVal = pVal;
                return rc;
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ValueFromExpr(
    mut db: *mut sqlite3,
    mut pExpr: *const Expr,
    mut enc: u8_0,
    mut affinity: u8_0,
    mut ppVal: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        return if !pExpr.is_null() {
            valueFromExpr(
                db,
                pExpr,
                enc,
                affinity,
                ppVal,
                ::core::ptr::null_mut::<ValueNewStat4Ctx>(),
            )
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ValueSetStr(
    mut v: *mut sqlite3_value,
    mut n: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_void,
    mut enc: u8_0,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    unsafe {
        if !v.is_null() {
            sqlite3VdbeMemSetStr(
                v as *mut Mem,
                z as *const ::core::ffi::c_char,
                n as i64_0,
                enc,
                xDel,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ValueFree(mut v: *mut sqlite3_value) {
    unsafe {
        if v.is_null() {
            return;
        }
        sqlite3VdbeMemRelease(v as *mut Mem);
        sqlite3DbFreeNN((*(v as *mut Mem)).db, v as *mut ::core::ffi::c_void);
    }
}
#[inline(never)]
unsafe extern "C" fn valueBytes(
    mut pVal: *mut sqlite3_value,
    mut enc: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        return if !valueToText(pVal, enc).is_null() {
            (*pVal).n
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ValueBytes(
    mut pVal: *mut sqlite3_value,
    mut enc: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Mem = pVal as *mut Mem;
        if (*p).flags as ::core::ffi::c_int & MEM_Str != 0 as ::core::ffi::c_int
            && (*pVal).enc as ::core::ffi::c_int == enc as ::core::ffi::c_int
        {
            return (*p).n;
        }
        if (*p).flags as ::core::ffi::c_int & MEM_Str != 0 as ::core::ffi::c_int
            && enc as ::core::ffi::c_int != SQLITE_UTF8
            && (*pVal).enc as ::core::ffi::c_int != SQLITE_UTF8
        {
            return (*p).n;
        }
        if (*p).flags as ::core::ffi::c_int & MEM_Blob != 0 as ::core::ffi::c_int {
            if (*p).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                return (*p).n + (*p).u.nZero
            } else {
                return (*p).n
            }
        }
        if (*p).flags as ::core::ffi::c_int & MEM_Null != 0 {
            return 0 as ::core::ffi::c_int;
        }
        return valueBytes(pVal, enc);
    }
}
pub const TK_CAST: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const TK_STRING: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_FLOAT: ::core::ffi::c_int = 154 as ::core::ffi::c_int;
pub const TK_BLOB: ::core::ffi::c_int = 155 as ::core::ffi::c_int;
pub const TK_INTEGER: ::core::ffi::c_int = 156 as ::core::ffi::c_int;
pub const TK_TRUEFALSE: ::core::ffi::c_int = 171 as ::core::ffi::c_int;
pub const TK_UPLUS: ::core::ffi::c_int = 173 as ::core::ffi::c_int;
pub const TK_UMINUS: ::core::ffi::c_int = 174 as ::core::ffi::c_int;
pub const TK_REGISTER: ::core::ffi::c_int = 176 as ::core::ffi::c_int;
pub const TK_SPAN: ::core::ffi::c_int = 181 as ::core::ffi::c_int;
pub const SQLITE_AFF_BLOB: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const SQLITE_AFF_TEXT: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const SQLITE_AFF_INTEGER: ::core::ffi::c_int = 68;
pub const SQLITE_AFF_REAL: ::core::ffi::c_int = 69;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const MEMCELLSIZE: ::core::ffi::c_ulong = 24 as ::core::ffi::c_ulong;
pub const MEM_Null: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MEM_Str: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MEM_Int: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MEM_Real: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MEM_Blob: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MEM_IntReal: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MEM_Term: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const MEM_Zero: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const MEM_Subtype: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const MEM_TypeMask: ::core::ffi::c_int = 0xdbf as ::core::ffi::c_int;
pub const MEM_Dyn: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const MEM_Static: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const MEM_Ephem: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const MEM_Agg: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_MAX_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
