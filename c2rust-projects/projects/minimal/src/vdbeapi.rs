use c2rust_bitfields::BitfieldStruct;
use ::libc;
extern "C" {
    pub type VdbeSorter;
    pub type BtCursor;
    pub type Btree;
    pub type RenameToken;
    pub type VtabCtx;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type Pager;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsCurrentTimeInt64(
        _: *mut sqlite3_vfs,
        _: *mut sqlite3_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerWalCallback(pPager: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3BtreeFirst(
        _: *mut BtCursor,
        pRes: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeNext(
        _: *mut BtCursor,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeEof(_: *mut BtCursor) -> ::core::ffi::c_int;
    fn sqlite3BtreePayloadSize(_: *mut BtCursor) -> u32_0;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3VdbeDelete(_: *mut Vdbe);
    fn sqlite3VdbeRewind(_: *mut Vdbe);
    fn sqlite3VdbeReset(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeExpandSql(
        _: *mut Vdbe,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3MisuseError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3LeaveMutexAndCloseZombie(_: *mut sqlite3);
    fn sqlite3VListNumToName(
        _: *mut VList,
        _: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3VListNameToNum(
        _: *mut VList,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3GetVarint32(_: *const ::core::ffi::c_uchar, _: *mut u32_0) -> u8_0;
    fn sqlite3Error(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3ErrStr(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3ValueText(_: *mut sqlite3_value, _: u8_0) -> *const ::core::ffi::c_void;
    fn sqlite3ValueBytes(_: *mut sqlite3_value, _: u8_0) -> ::core::ffi::c_int;
    fn sqlite3ValueFree(_: *mut sqlite3_value);
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3OomClear(_: *mut sqlite3);
    fn sqlite3ApiExit(db: *mut sqlite3, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3Reprepare(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeSerialGet(_: *const ::core::ffi::c_uchar, _: u32_0, _: *mut Mem);
    fn sqlite3VdbeExec(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeList(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeChangeEncoding(
        _: *mut Mem,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemTooBig(_: *mut Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemCopy(_: *mut Mem, _: *const Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemMove(_: *mut Mem, _: *mut Mem);
    fn sqlite3VdbeMemSetStr(
        _: *mut Mem,
        _: *const ::core::ffi::c_char,
        _: i64_0,
        _: u8_0,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemSetInt64(_: *mut Mem, _: i64_0);
    fn sqlite3VdbeMemSetDouble(_: *mut Mem, _: ::core::ffi::c_double);
    fn sqlite3VdbeMemSetPointer(
        _: *mut Mem,
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_char,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3VdbeMemSetNull(_: *mut Mem);
    fn sqlite3VdbeMemSetZeroBlob(_: *mut Mem, _: ::core::ffi::c_int);
    fn sqlite3VdbeMemZeroTerminateIfAble(_: *mut Mem);
    fn sqlite3VdbeMemMakeWriteable(_: *mut Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeIntValue(_: *const Mem) -> i64_0;
    fn sqlite3VdbeRealValue(_: *mut Mem) -> ::core::ffi::c_double;
    fn sqlite3VdbeMemFromBtreeZeroOffset(
        _: *mut BtCursor,
        _: u32_0,
        _: *mut Mem,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemRelease(p: *mut Mem);
    fn sqlite3VdbeMemClearAndResize(
        pMem: *mut Mem,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeTransferError(p: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemExpandBlob(_: *mut Mem) -> ::core::ffi::c_int;
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueList {
    pub pCsr: *mut BtCursor,
    pub pOut: *mut sqlite3_value

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_SCHEMA: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_RANGE: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_TRACE_PROFILE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LENGTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_UTF16LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_UTF16BE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_UTF16: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_STMTSTATUS_MEMUSED: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
pub const SQLITE_UTF16NATIVE: ::core::ffi::c_int = SQLITE_UTF16LE;
pub const COLNAME_NAME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_PREPARE_SAVESQL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_TRACE_XPROFILE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const SQLITE_MAX_SCHEMA_RETRY: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const MEMCELLSIZE: ::core::ffi::c_ulong = 24 as ::core::ffi::c_ulong;
pub const MEM_Null: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MEM_Str: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MEM_Int: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MEM_Real: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MEM_Blob: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MEM_IntReal: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MEM_AffMask: ::core::ffi::c_int = 0x3f as ::core::ffi::c_int;
pub const MEM_FromBind: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const MEM_Term: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const MEM_Zero: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const MEM_Subtype: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const MEM_TypeMask: ::core::ffi::c_int = 0xdbf as ::core::ffi::c_int;
pub const MEM_Dyn: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const MEM_Static: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const MEM_Ephem: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const MEM_Agg: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const VDBE_READY_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const VDBE_RUN_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn vdbeSafety(mut p: *mut Vdbe) -> ::core::ffi::c_int {
    unsafe {
        if (*p).db.is_null() {
            sqlite3_log(
                SQLITE_MISUSE,
                b"API called with finalized prepared statement\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return 1 as ::core::ffi::c_int;
        } else {
            return 0 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn vdbeSafetyNotNull(mut p: *mut Vdbe) -> ::core::ffi::c_int {
    unsafe {
        if p.is_null() {
            sqlite3_log(
                SQLITE_MISUSE,
                b"API called with NULL prepared statement\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return 1 as ::core::ffi::c_int;
        } else {
            return vdbeSafety(p)
        };
    }
}
#[inline(never)]
unsafe extern "C" fn invokeProfileCallback(mut db: *mut sqlite3, mut p: *mut Vdbe) {
    unsafe {
        let mut iNow: sqlite3_int64 = 0;
        let mut iElapse: sqlite3_int64 = 0;
        sqlite3OsCurrentTimeInt64((*db).pVfs, &raw mut iNow);
        iElapse = ((iNow as ::core::ffi::c_longlong
            - (*p).startTime as ::core::ffi::c_longlong)
            * 1000000 as ::core::ffi::c_longlong) as sqlite3_int64;
        if (*db).mTrace as ::core::ffi::c_int & SQLITE_TRACE_PROFILE != 0 {
            (*db)
                .trace
                .xV2
                .expect(
                    "non-null function pointer",
                )(
                SQLITE_TRACE_PROFILE as u32_0,
                (*db).pTraceArg,
                p as *mut ::core::ffi::c_void,
                &raw mut iElapse as *mut ::core::ffi::c_void,
            );
        }
        (*p).startTime = 0 as i64_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_finalize(
    mut pStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if pStmt.is_null() {
            rc = SQLITE_OK;
        } else {
            let mut v: *mut Vdbe = pStmt as *mut Vdbe;
            let mut db: *mut sqlite3 = (*v).db;
            if vdbeSafety(v) != 0 {
                return sqlite3MisuseError(108 as ::core::ffi::c_int);
            }
            sqlite3_mutex_enter((*db).mutex);
            if (*v).startTime > 0 as ::core::ffi::c_longlong {
                invokeProfileCallback(db, v);
            }
            rc = sqlite3VdbeReset(v);
            sqlite3VdbeDelete(v);
            rc = sqlite3ApiExit(db, rc);
            sqlite3LeaveMutexAndCloseZombie(db);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_reset(
    mut pStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if pStmt.is_null() {
            rc = SQLITE_OK;
        } else {
            let mut v: *mut Vdbe = pStmt as *mut Vdbe;
            let mut db: *mut sqlite3 = (*v).db;
            sqlite3_mutex_enter((*db).mutex);
            if (*v).startTime > 0 as ::core::ffi::c_longlong {
                invokeProfileCallback(db, v);
            }
            rc = sqlite3VdbeReset(v);
            sqlite3VdbeRewind(v);
            rc = sqlite3ApiExit(db, rc);
            sqlite3_mutex_leave((*db).mutex);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_clear_bindings(
    mut pStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        let mut mutex: *mut sqlite3_mutex = ::core::ptr::null_mut::<sqlite3_mutex>();
        mutex = (*(*p).db).mutex;
        sqlite3_mutex_enter(mutex);
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nVar as ::core::ffi::c_int {
            sqlite3VdbeMemRelease((*p).aVar.offset(i as isize) as *mut Mem);
            (*(*p).aVar.offset(i as isize)).flags = MEM_Null as u16_0;
            i += 1;
        }
        if (*p).expmask != 0 {
            (*p).set_expired(1 as bft as bft);
        }
        sqlite3_mutex_leave(mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_blob(
    mut pVal: *mut sqlite3_value,
) -> *const ::core::ffi::c_void {
    unsafe {
        let mut p: *mut Mem = pVal as *mut Mem;
        if (*p).flags as ::core::ffi::c_int & (MEM_Blob | MEM_Str) != 0 {
            if (if (*p).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                sqlite3VdbeMemExpandBlob(p)
            } else {
                0 as ::core::ffi::c_int
            }) != SQLITE_OK
            {
                return ::core::ptr::null::<::core::ffi::c_void>();
            }
            (*p).flags = ((*p).flags as ::core::ffi::c_int | MEM_Blob) as u16_0;
            return (if (*p).n != 0 {
                (*p).z
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            }) as *const ::core::ffi::c_void;
        } else {
            return sqlite3_value_text(pVal) as *const ::core::ffi::c_void
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_bytes(
    mut pVal: *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3ValueBytes(pVal, SQLITE_UTF8 as u8_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_bytes16(
    mut pVal: *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3ValueBytes(pVal, SQLITE_UTF16NATIVE as u8_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_double(
    mut pVal: *mut sqlite3_value,
) -> ::core::ffi::c_double {
    unsafe {
        return sqlite3VdbeRealValue(pVal as *mut Mem);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_int(
    mut pVal: *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3VdbeIntValue(pVal as *mut Mem) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_int64(
    mut pVal: *mut sqlite3_value,
) -> sqlite3_int64 {
    unsafe {
        return sqlite3VdbeIntValue(pVal as *mut Mem) as sqlite3_int64;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_subtype(
    mut pVal: *mut sqlite3_value,
) -> ::core::ffi::c_uint {
    unsafe {
        let mut pMem: *mut Mem = pVal as *mut Mem;
        return (if (*pMem).flags as ::core::ffi::c_int & MEM_Subtype != 0 {
            (*pMem).eSubtype as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_pointer(
    mut pVal: *mut sqlite3_value,
    mut zPType: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut p: *mut Mem = pVal as *mut Mem;
        if (*p).flags as ::core::ffi::c_int & (MEM_TypeMask | MEM_Term | MEM_Subtype)
            == MEM_Null | MEM_Term | MEM_Subtype && !zPType.is_null()
            && (*p).eSubtype as ::core::ffi::c_int == 'p' as i32
            && strcmp((*p).u.zPType, zPType) == 0 as ::core::ffi::c_int
        {
            return (*p).z as *mut ::core::ffi::c_void
        } else {
            return ::core::ptr::null_mut::<::core::ffi::c_void>()
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_text(
    mut pVal: *mut sqlite3_value,
) -> *const ::core::ffi::c_uchar {
    unsafe {
        return sqlite3ValueText(pVal, SQLITE_UTF8 as u8_0)
            as *const ::core::ffi::c_uchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_text16(
    mut pVal: *mut sqlite3_value,
) -> *const ::core::ffi::c_void {
    unsafe {
        return sqlite3ValueText(pVal, SQLITE_UTF16NATIVE as u8_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_text16be(
    mut pVal: *mut sqlite3_value,
) -> *const ::core::ffi::c_void {
    unsafe {
        return sqlite3ValueText(pVal, SQLITE_UTF16BE as u8_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_text16le(
    mut pVal: *mut sqlite3_value,
) -> *const ::core::ffi::c_void {
    unsafe {
        return sqlite3ValueText(pVal, SQLITE_UTF16LE as u8_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_type(
    mut pVal: *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aType: [u8_0; 64] = [
            SQLITE_BLOB as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_TEXT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_INTEGER as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_INTEGER as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_INTEGER as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_INTEGER as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_BLOB as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_TEXT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_INTEGER as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_INTEGER as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_INTEGER as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_INTEGER as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_BLOB as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_TEXT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
            SQLITE_FLOAT as u8_0,
            SQLITE_NULL as u8_0,
        ];
        return aType[((*pVal).flags as ::core::ffi::c_int & MEM_AffMask) as usize]
            as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_encoding(
    mut pVal: *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        return (*pVal).enc as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_nochange(
    mut pVal: *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        return ((*pVal).flags as ::core::ffi::c_int & (MEM_Null | MEM_Zero)
            == MEM_Null | MEM_Zero) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_frombind(
    mut pVal: *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        return ((*pVal).flags as ::core::ffi::c_int & MEM_FromBind
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_dup(
    mut pOrig: *const sqlite3_value,
) -> *mut sqlite3_value {
    unsafe {
        let mut pNew: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        if pOrig.is_null() {
            return ::core::ptr::null_mut::<sqlite3_value>();
        }
        pNew = sqlite3_malloc(
            ::core::mem::size_of::<sqlite3_value>() as ::core::ffi::c_int,
        ) as *mut sqlite3_value;
        if pNew.is_null() {
            return ::core::ptr::null_mut::<sqlite3_value>();
        }
        memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sqlite3_value>() as size_t,
        );
        memcpy(
            pNew as *mut ::core::ffi::c_void,
            pOrig as *const ::core::ffi::c_void,
            MEMCELLSIZE as size_t,
        );
        (*pNew).flags = ((*pNew).flags as ::core::ffi::c_int & !MEM_Dyn) as u16_0;
        (*pNew).db = ::core::ptr::null_mut::<sqlite3>();
        if (*pNew).flags as ::core::ffi::c_int & (MEM_Str | MEM_Blob) != 0 {
            (*pNew).flags = ((*pNew).flags as ::core::ffi::c_int
                & !(MEM_Static | MEM_Dyn)) as u16_0;
            (*pNew).flags = ((*pNew).flags as ::core::ffi::c_int | MEM_Ephem) as u16_0;
            if sqlite3VdbeMemMakeWriteable(pNew as *mut Mem) != SQLITE_OK {
                sqlite3ValueFree(pNew);
                pNew = ::core::ptr::null_mut::<sqlite3_value>();
            }
        } else if (*pNew).flags as ::core::ffi::c_int & MEM_Null != 0 {
            (*pNew).flags = ((*pNew).flags as ::core::ffi::c_int
                & !(MEM_Term | MEM_Subtype)) as u16_0;
        }
        return pNew;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_free(mut pOld: *mut sqlite3_value) {
    unsafe {
        sqlite3ValueFree(pOld);
    }
}
unsafe extern "C" fn setResultStrOrError(
    mut pCtx: *mut sqlite3_context,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut enc: u8_0,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    unsafe {
        let mut pOut: *mut Mem = (*pCtx).pOut;
        let mut rc: ::core::ffi::c_int = sqlite3VdbeMemSetStr(
            pOut,
            z,
            n as i64_0,
            enc,
            xDel,
        );
        if rc != 0 {
            if rc == SQLITE_TOOBIG {
                sqlite3_result_error_toobig(pCtx);
            } else {
                sqlite3_result_error_nomem(pCtx);
            }
            return;
        }
        sqlite3VdbeChangeEncoding(pOut, (*pCtx).enc as ::core::ffi::c_int);
        if sqlite3VdbeMemTooBig(pOut) != 0 {
            sqlite3_result_error_toobig(pCtx);
        }
    }
}
unsafe extern "C" fn invokeValueDestructor(
    mut p: *const ::core::ffi::c_void,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    mut pCtx: *mut sqlite3_context,
) -> ::core::ffi::c_int {
    unsafe {
        if !xDel.is_none() {
            if !(xDel
                == ::core::mem::transmute::<
                    ::libc::intptr_t,
                    sqlite3_destructor_type,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t))
            {
                xDel.expect("non-null function pointer")(p as *mut ::core::ffi::c_void);
            }
        }
        sqlite3_result_error_toobig(pCtx);
        return SQLITE_TOOBIG;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_blob(
    mut pCtx: *mut sqlite3_context,
    mut z: *const ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    unsafe {
        setResultStrOrError(pCtx, z as *const ::core::ffi::c_char, n, 0 as u8_0, xDel);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_blob64(
    mut pCtx: *mut sqlite3_context,
    mut z: *const ::core::ffi::c_void,
    mut n: sqlite3_uint64,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    unsafe {
        if n > 0x7fffffff as ::core::ffi::c_ulonglong {
            invokeValueDestructor(z, xDel, pCtx);
        } else {
            setResultStrOrError(
                pCtx,
                z as *const ::core::ffi::c_char,
                n as ::core::ffi::c_int,
                0 as u8_0,
                xDel,
            );
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_double(
    mut pCtx: *mut sqlite3_context,
    mut rVal: ::core::ffi::c_double,
) {
    unsafe {
        sqlite3VdbeMemSetDouble((*pCtx).pOut, rVal);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_error(
    mut pCtx: *mut sqlite3_context,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) {
    unsafe {
        (*pCtx).isError = SQLITE_ERROR;
        sqlite3VdbeMemSetStr(
            (*pCtx).pOut,
            z,
            n as i64_0,
            SQLITE_UTF8 as u8_0,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_error16(
    mut pCtx: *mut sqlite3_context,
    mut z: *const ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
) {
    unsafe {
        (*pCtx).isError = SQLITE_ERROR;
        sqlite3VdbeMemSetStr(
            (*pCtx).pOut,
            z as *const ::core::ffi::c_char,
            n as i64_0,
            SQLITE_UTF16NATIVE as u8_0,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_int(
    mut pCtx: *mut sqlite3_context,
    mut iVal: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3VdbeMemSetInt64((*pCtx).pOut, iVal as i64_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_int64(
    mut pCtx: *mut sqlite3_context,
    mut iVal: i64_0,
) {
    unsafe {
        sqlite3VdbeMemSetInt64((*pCtx).pOut, iVal);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_null(mut pCtx: *mut sqlite3_context) {
    unsafe {
        sqlite3VdbeMemSetNull((*pCtx).pOut);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_pointer(
    mut pCtx: *mut sqlite3_context,
    mut pPtr: *mut ::core::ffi::c_void,
    mut zPType: *const ::core::ffi::c_char,
    mut xDestructor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    unsafe {
        let mut pOut: *mut Mem = ::core::ptr::null_mut::<Mem>();
        pOut = (*pCtx).pOut;
        sqlite3VdbeMemRelease(pOut);
        (*pOut).flags = MEM_Null as u16_0;
        sqlite3VdbeMemSetPointer(pOut, pPtr, zPType, xDestructor);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_subtype(
    mut pCtx: *mut sqlite3_context,
    mut eSubtype: ::core::ffi::c_uint,
) {
    unsafe {
        let mut pOut: *mut Mem = ::core::ptr::null_mut::<Mem>();
        pOut = (*pCtx).pOut;
        (*pOut).eSubtype = (eSubtype & 0xff as ::core::ffi::c_uint) as u8_0;
        (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int | MEM_Subtype) as u16_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_text(
    mut pCtx: *mut sqlite3_context,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    unsafe {
        setResultStrOrError(pCtx, z, n, SQLITE_UTF8 as u8_0, xDel);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_text64(
    mut pCtx: *mut sqlite3_context,
    mut z: *const ::core::ffi::c_char,
    mut n: sqlite3_uint64,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    mut enc: ::core::ffi::c_uchar,
) {
    unsafe {
        if enc as ::core::ffi::c_int != SQLITE_UTF8 {
            if enc as ::core::ffi::c_int == SQLITE_UTF16 {
                enc = SQLITE_UTF16NATIVE as ::core::ffi::c_uchar;
            }
            n &= !(1 as ::core::ffi::c_int as u64_0) as ::core::ffi::c_ulonglong;
        }
        if n > 0x7fffffff as ::core::ffi::c_ulonglong {
            invokeValueDestructor(z as *const ::core::ffi::c_void, xDel, pCtx);
        } else {
            setResultStrOrError(pCtx, z, n as ::core::ffi::c_int, enc as u8_0, xDel);
            sqlite3VdbeMemZeroTerminateIfAble((*pCtx).pOut);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_text16(
    mut pCtx: *mut sqlite3_context,
    mut z: *const ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    unsafe {
        setResultStrOrError(
            pCtx,
            z as *const ::core::ffi::c_char,
            (n as u64_0 & !(1 as ::core::ffi::c_int as u64_0)) as ::core::ffi::c_int,
            SQLITE_UTF16NATIVE as u8_0,
            xDel,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_text16be(
    mut pCtx: *mut sqlite3_context,
    mut z: *const ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    unsafe {
        setResultStrOrError(
            pCtx,
            z as *const ::core::ffi::c_char,
            (n as u64_0 & !(1 as ::core::ffi::c_int as u64_0)) as ::core::ffi::c_int,
            SQLITE_UTF16BE as u8_0,
            xDel,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_text16le(
    mut pCtx: *mut sqlite3_context,
    mut z: *const ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    unsafe {
        setResultStrOrError(
            pCtx,
            z as *const ::core::ffi::c_char,
            (n as u64_0 & !(1 as ::core::ffi::c_int as u64_0)) as ::core::ffi::c_int,
            SQLITE_UTF16LE as u8_0,
            xDel,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_value(
    mut pCtx: *mut sqlite3_context,
    mut pValue: *mut sqlite3_value,
) {
    unsafe {
        let mut pOut: *mut Mem = ::core::ptr::null_mut::<Mem>();
        pOut = (*pCtx).pOut;
        sqlite3VdbeMemCopy(pOut, pValue);
        sqlite3VdbeChangeEncoding(pOut, (*pCtx).enc as ::core::ffi::c_int);
        if sqlite3VdbeMemTooBig(pOut) != 0 {
            sqlite3_result_error_toobig(pCtx);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_zeroblob(
    mut pCtx: *mut sqlite3_context,
    mut n: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3_result_zeroblob64(
            pCtx,
            (if n > 0 as ::core::ffi::c_int { n } else { 0 as ::core::ffi::c_int })
                as u64_0,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_zeroblob64(
    mut pCtx: *mut sqlite3_context,
    mut n: u64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pOut: *mut Mem = ::core::ptr::null_mut::<Mem>();
        pOut = (*pCtx).pOut;
        if n > (*(*pOut).db).aLimit[SQLITE_LIMIT_LENGTH as usize] as u64_0 {
            sqlite3_result_error_toobig(pCtx);
            return SQLITE_TOOBIG;
        }
        sqlite3VdbeMemSetZeroBlob((*pCtx).pOut, n as ::core::ffi::c_int);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_error_code(
    mut pCtx: *mut sqlite3_context,
    mut errCode: ::core::ffi::c_int,
) {
    unsafe {
        (*pCtx).isError = if errCode != 0 { errCode } else { -1 as ::core::ffi::c_int };
        if (*(*pCtx).pOut).flags as ::core::ffi::c_int & MEM_Null != 0 {
            setResultStrOrError(
                pCtx,
                sqlite3ErrStr(errCode),
                -1 as ::core::ffi::c_int,
                SQLITE_UTF8 as u8_0,
                SQLITE_STATIC,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_error_toobig(mut pCtx: *mut sqlite3_context) {
    unsafe {
        (*pCtx).isError = SQLITE_TOOBIG;
        sqlite3VdbeMemSetStr(
            (*pCtx).pOut,
            b"string or blob too big\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int as i64_0,
            SQLITE_UTF8 as u8_0,
            SQLITE_STATIC,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_result_error_nomem(mut pCtx: *mut sqlite3_context) {
    unsafe {
        sqlite3VdbeMemSetNull((*pCtx).pOut);
        (*pCtx).isError = SQLITE_NOMEM_BKPT;
        sqlite3OomFault((*(*pCtx).pOut).db);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ResultIntReal(mut pCtx: *mut sqlite3_context) {
    unsafe {
        if (*(*pCtx).pOut).flags as ::core::ffi::c_int & MEM_Int != 0 {
            (*(*pCtx).pOut).flags = ((*(*pCtx).pOut).flags as ::core::ffi::c_int
                & !MEM_Int) as u16_0;
            (*(*pCtx).pOut).flags = ((*(*pCtx).pOut).flags as ::core::ffi::c_int
                | MEM_IntReal) as u16_0;
        }
    }
}
unsafe extern "C" fn doWalCallbacks(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*db).nDb {
            let mut pBt: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
            if !pBt.is_null() {
                let mut nEntry: ::core::ffi::c_int = 0;
                nEntry = sqlite3PagerWalCallback(sqlite3BtreePager(pBt) as *mut Pager);
                if nEntry > 0 as ::core::ffi::c_int && (*db).xWalCallback.is_some()
                    && rc == SQLITE_OK
                {
                    rc = (*db)
                        .xWalCallback
                        .expect(
                            "non-null function pointer",
                        )(
                        (*db).pWalArg,
                        db,
                        (*(*db).aDb.offset(i as isize)).zDbSName,
                        nEntry,
                    );
                }
            }
            i += 1;
        }
        return rc;
    }
}
unsafe extern "C" fn sqlite3Step(mut p: *mut Vdbe) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        db = (*p).db;
        if (*p).eVdbeState as ::core::ffi::c_int != VDBE_RUN_STATE {
            loop {
                if (*p).eVdbeState as ::core::ffi::c_int == VDBE_READY_STATE {
                    if (*p).expired() != 0 {
                        c2rust_current_block = 4644295000439058019;
                        break;
                    } else {
                        c2rust_current_block = 14523784380283086299;
                        break;
                    }
                } else {
                    if !((*p).eVdbeState as ::core::ffi::c_int
                        == 3 as ::core::ffi::c_int)
                    {
                        c2rust_current_block = 2719512138335094285;
                        break;
                    }
                    sqlite3_reset(p as *mut sqlite3_stmt);
                }
            }
            match c2rust_current_block {
                2719512138335094285 => {}
                _ => {
                    match c2rust_current_block {
                        4644295000439058019 => {
                            (*p).rc = SQLITE_SCHEMA;
                            rc = SQLITE_ERROR;
                            if (*p).prepFlags as ::core::ffi::c_int
                                & SQLITE_PREPARE_SAVESQL != 0 as ::core::ffi::c_int
                            {
                                rc = sqlite3VdbeTransferError(p);
                            }
                            c2rust_current_block = 12556861819962772176;
                        }
                        _ => {
                            if (*db).nVdbeActive == 0 as ::core::ffi::c_int {
                                ;
                            }
                            if (*db).mTrace as ::core::ffi::c_int
                                & (SQLITE_TRACE_PROFILE | SQLITE_TRACE_XPROFILE)
                                != 0 as ::core::ffi::c_int && (*db).init.busy == 0
                                && !(*p).zSql.is_null()
                            {
                                sqlite3OsCurrentTimeInt64(
                                    (*db).pVfs,
                                    &raw mut (*p).startTime,
                                );
                            }
                            (*db).nVdbeActive += 1;
                            if (*p).readOnly() as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                (*db).nVdbeWrite += 1;
                            }
                            if (*p).bIsReader() != 0 {
                                (*db).nVdbeRead += 1;
                            }
                            (*p).pc = 0 as ::core::ffi::c_int;
                            (*p).eVdbeState = VDBE_RUN_STATE as u8_0;
                            c2rust_current_block = 2719512138335094285;
                        }
                    }
                }
            }
        } else {
            c2rust_current_block = 2719512138335094285;
        }
        match c2rust_current_block {
            2719512138335094285 => {
                if (*p).explain() != 0 {
                    rc = sqlite3VdbeList(p);
                } else {
                    (*db).nVdbeExec += 1;
                    rc = sqlite3VdbeExec(p);
                    (*db).nVdbeExec -= 1;
                }
                if rc == SQLITE_ROW {
                    (*db).errCode = SQLITE_ROW;
                    return SQLITE_ROW;
                } else {
                    if (*p).startTime > 0 as ::core::ffi::c_longlong {
                        invokeProfileCallback(db, p);
                    }
                    (*p).pResultRow = ::core::ptr::null_mut::<Mem>();
                    if rc == SQLITE_DONE && (*db).autoCommit as ::core::ffi::c_int != 0 {
                        (*p).rc = doWalCallbacks(db);
                        if (*p).rc != SQLITE_OK {
                            rc = SQLITE_ERROR;
                        }
                    } else if rc != SQLITE_DONE
                        && (*p).prepFlags as ::core::ffi::c_int & SQLITE_PREPARE_SAVESQL
                            != 0 as ::core::ffi::c_int
                    {
                        rc = sqlite3VdbeTransferError(p);
                    }
                }
                (*db).errCode = rc;
                if SQLITE_NOMEM == sqlite3ApiExit((*p).db, (*p).rc) {
                    (*p).rc = SQLITE_NOMEM_BKPT;
                    if (*p).prepFlags as ::core::ffi::c_int & SQLITE_PREPARE_SAVESQL
                        != 0 as ::core::ffi::c_int
                    {
                        rc = (*p).rc;
                    }
                }
            }
            _ => {}
        }
        return rc & (*db).errMask;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_step(
    mut pStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut v: *mut Vdbe = pStmt as *mut Vdbe;
        let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if vdbeSafetyNotNull(v) != 0 {
            return sqlite3MisuseError(902 as ::core::ffi::c_int);
        }
        db = (*v).db;
        sqlite3_mutex_enter((*db).mutex);
        loop {
            rc = sqlite3Step(v);
            if !(rc == SQLITE_SCHEMA
                && {
                    let c2rust_fresh0 = cnt;
                    cnt = cnt + 1;
                    c2rust_fresh0 < SQLITE_MAX_SCHEMA_RETRY
                })
            {
                break;
            }
            let mut savedPc: ::core::ffi::c_int = (*v).pc;
            rc = sqlite3Reprepare(v);
            if rc != SQLITE_OK {
                let mut zErr: *const ::core::ffi::c_char = sqlite3_value_text((*db).pErr)
                    as *const ::core::ffi::c_char;
                sqlite3DbFree(db, (*v).zErrMsg as *mut ::core::ffi::c_void);
                if (*db).mallocFailed == 0 {
                    (*v).zErrMsg = sqlite3DbStrDup(db, zErr);
                    rc = sqlite3ApiExit(db, rc);
                    (*v).rc = rc;
                } else {
                    (*v).zErrMsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    rc = SQLITE_NOMEM_BKPT;
                    (*v).rc = rc;
                }
                break;
            } else {
                sqlite3_reset(pStmt);
                if savedPc >= 0 as ::core::ffi::c_int {
                    (*v).minWriteFileFormat = 254 as u8_0;
                }
            }
        }
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_user_data(
    mut p: *mut sqlite3_context,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*(*p).pFunc).pUserData;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_context_db_handle(
    mut p: *mut sqlite3_context,
) -> *mut sqlite3 {
    unsafe {
        return (*(*p).pOut).db;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vtab_nochange(
    mut p: *mut sqlite3_context,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3_value_nochange((*p).pOut as *mut sqlite3_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeValueListFree(
    mut pToDelete: *mut ::core::ffi::c_void,
) {
    unsafe {
        sqlite3_free(pToDelete);
    }
}
unsafe extern "C" fn valueFromValueList(
    mut pVal: *mut sqlite3_value,
    mut ppOut: *mut *mut sqlite3_value,
    mut bNext: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pRhs: *mut ValueList = ::core::ptr::null_mut::<ValueList>();
        *ppOut = ::core::ptr::null_mut::<sqlite3_value>();
        if pVal.is_null() {
            return sqlite3MisuseError(1023 as ::core::ffi::c_int);
        }
        if (*pVal).flags as ::core::ffi::c_int & MEM_Dyn == 0 as ::core::ffi::c_int
            || (*pVal).xDel
                != Some(
                    sqlite3VdbeValueListFree
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                )
        {
            return SQLITE_ERROR
        } else {
            pRhs = (*pVal).z as *mut ValueList;
        }
        if bNext != 0 {
            rc = sqlite3BtreeNext((*pRhs).pCsr, 0 as ::core::ffi::c_int);
        } else {
            let mut dummy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            rc = sqlite3BtreeFirst((*pRhs).pCsr, &raw mut dummy);
            if sqlite3BtreeEof((*pRhs).pCsr) != 0 {
                rc = SQLITE_DONE;
            }
        }
        if rc == SQLITE_OK {
            let mut sz: u32_0 = 0;
            let mut sMem: Mem = Mem {
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
                &raw mut sMem as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Mem>() as size_t,
            );
            sz = sqlite3BtreePayloadSize((*pRhs).pCsr);
            rc = sqlite3VdbeMemFromBtreeZeroOffset(
                (*pRhs).pCsr,
                sz as ::core::ffi::c_int as u32_0,
                &raw mut sMem,
            );
            if rc == SQLITE_OK {
                let mut zBuf: *mut u8_0 = sMem.z as *mut u8_0;
                let mut iSerial: u32_0 = 0;
                let mut pOut: *mut sqlite3_value = (*pRhs).pOut;
                let mut iOff: ::core::ffi::c_int = 1 as ::core::ffi::c_int
                    + (if (*zBuf.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int)
                        < 0x80 as ::core::ffi::c_int as u8_0 as ::core::ffi::c_int
                    {
                        iSerial = *zBuf.offset(1 as ::core::ffi::c_int as isize)
                            as u32_0;
                        1 as ::core::ffi::c_int
                    } else {
                        sqlite3GetVarint32(
                            zBuf.offset(1 as ::core::ffi::c_int as isize) as *mut u8_0,
                            &raw mut iSerial,
                        ) as ::core::ffi::c_int
                    }) as u8_0 as ::core::ffi::c_int;
                sqlite3VdbeSerialGet(
                    zBuf.offset(iOff as isize) as *mut u8_0,
                    iSerial,
                    pOut as *mut Mem,
                );
                (*pOut).enc = (*(*pOut).db).enc;
                if (*pOut).flags as ::core::ffi::c_int & MEM_Ephem
                    != 0 as ::core::ffi::c_int
                    && sqlite3VdbeMemMakeWriteable(pOut as *mut Mem) != 0
                {
                    rc = SQLITE_NOMEM;
                } else {
                    *ppOut = pOut;
                }
            }
            sqlite3VdbeMemRelease(&raw mut sMem);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vtab_in_first(
    mut pVal: *mut sqlite3_value,
    mut ppOut: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        return valueFromValueList(pVal, ppOut, 0 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vtab_in_next(
    mut pVal: *mut sqlite3_value,
    mut ppOut: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        return valueFromValueList(pVal, ppOut, 1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3StmtCurrentTime(
    mut p: *mut sqlite3_context,
) -> sqlite3_int64 {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut piTime: *mut sqlite3_int64 = &raw mut (*(*p).pVdbe).iCurrentTime;
        if *piTime == 0 as ::core::ffi::c_longlong {
            rc = sqlite3OsCurrentTimeInt64((*(*(*p).pOut).db).pVfs, piTime);
            if rc != 0 {
                *piTime = 0 as sqlite3_int64;
            }
        }
        return *piTime;
    }
}
#[inline(never)]
unsafe extern "C" fn createAggContext(
    mut p: *mut sqlite3_context,
    mut nByte: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pMem: *mut Mem = (*p).pMem;
        if nByte <= 0 as ::core::ffi::c_int {
            sqlite3VdbeMemSetNull(pMem);
            (*pMem).z = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            sqlite3VdbeMemClearAndResize(pMem, nByte);
            (*pMem).flags = MEM_Agg as u16_0;
            (*pMem).u.pDef = (*p).pFunc;
            if !(*pMem).z.is_null() {
                memset(
                    (*pMem).z as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    nByte as size_t,
                );
            }
        }
        return (*pMem).z as *mut ::core::ffi::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_aggregate_context(
    mut p: *mut sqlite3_context,
    mut nByte: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        if (*(*p).pMem).flags as ::core::ffi::c_int & MEM_Agg == 0 as ::core::ffi::c_int
        {
            return createAggContext(p, nByte)
        } else {
            return (*(*p).pMem).z as *mut ::core::ffi::c_void
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_get_auxdata(
    mut pCtx: *mut sqlite3_context,
    mut iArg: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pAuxData: *mut AuxData = ::core::ptr::null_mut::<AuxData>();
        pAuxData = (*(*pCtx).pVdbe).pAuxData;
        while !pAuxData.is_null() {
            if (*pAuxData).iAuxArg == iArg
                && ((*pAuxData).iAuxOp == (*pCtx).iOp || iArg < 0 as ::core::ffi::c_int)
            {
                return (*pAuxData).pAux;
            }
            pAuxData = (*pAuxData).pNextAux;
        }
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_set_auxdata(
    mut pCtx: *mut sqlite3_context,
    mut iArg: ::core::ffi::c_int,
    mut pAux: *mut ::core::ffi::c_void,
    mut xDelete: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    unsafe {
        let mut pAuxData: *mut AuxData = ::core::ptr::null_mut::<AuxData>();
        let mut pVdbe: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        pVdbe = (*pCtx).pVdbe;
        pAuxData = (*pVdbe).pAuxData;
        while !pAuxData.is_null() {
            if (*pAuxData).iAuxArg == iArg
                && ((*pAuxData).iAuxOp == (*pCtx).iOp || iArg < 0 as ::core::ffi::c_int)
            {
                break;
            }
            pAuxData = (*pAuxData).pNextAux;
        }
        if pAuxData.is_null() {
            pAuxData = sqlite3DbMallocZero(
                (*pVdbe).db,
                ::core::mem::size_of::<AuxData>() as u64_0,
            ) as *mut AuxData;
            if pAuxData.is_null() {
                if xDelete.is_some() {
                    xDelete.expect("non-null function pointer")(pAux);
                }
                return;
            } else {
                (*pAuxData).iAuxOp = (*pCtx).iOp;
                (*pAuxData).iAuxArg = iArg;
                (*pAuxData).pNextAux = (*pVdbe).pAuxData;
                (*pVdbe).pAuxData = pAuxData;
                if (*pCtx).isError == 0 as ::core::ffi::c_int {
                    (*pCtx).isError = -1 as ::core::ffi::c_int;
                }
            }
        } else if (*pAuxData).xDeleteAux.is_some() {
            (*pAuxData).xDeleteAux.expect("non-null function pointer")((*pAuxData).pAux);
        }
        (*pAuxData).pAux = pAux;
        (*pAuxData).xDeleteAux = xDelete;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_count(
    mut pStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVm: *mut Vdbe = pStmt as *mut Vdbe;
        if pVm.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        return (*pVm).nResColumn as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_data_count(
    mut pStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVm: *mut Vdbe = pStmt as *mut Vdbe;
        if pVm.is_null() || (*pVm).pResultRow.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        return (*pVm).nResColumn as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn columnNullValue() -> *const Mem {
    unsafe {
        static mut nullMem: Mem = sqlite3_value {
            u: MemValue {
                r: 0 as ::core::ffi::c_int as ::core::ffi::c_double

            },
            z: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            n: 0 as ::core::ffi::c_int,
            flags: MEM_Null as u16_0,
            enc: 0 as ::core::ffi::c_int as u8_0,
            eSubtype: 0 as ::core::ffi::c_int as u8_0,
            db: ::core::ptr::null::<sqlite3>() as *mut sqlite3,
            szMalloc: 0 as ::core::ffi::c_int,
            uTemp: 0 as ::core::ffi::c_int as u32_0,
            zMalloc: ::core::ptr::null::<::core::ffi::c_char>()
                as *mut ::core::ffi::c_char,
            xDel: None

        };
        return &raw const nullMem;
    }
}
unsafe extern "C" fn columnMem(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> *mut Mem {
    unsafe {
        let mut pVm: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut pOut: *mut Mem = ::core::ptr::null_mut::<Mem>();
        pVm = pStmt as *mut Vdbe;
        if pVm.is_null() {
            return columnNullValue() as *mut Mem;
        }
        sqlite3_mutex_enter((*(*pVm).db).mutex);
        if !(*pVm).pResultRow.is_null() && i < (*pVm).nResColumn as ::core::ffi::c_int
            && i >= 0 as ::core::ffi::c_int
        {
            pOut = (*pVm).pResultRow.offset(i as isize) as *mut Mem;
        } else {
            sqlite3Error((*pVm).db, SQLITE_RANGE);
            pOut = columnNullValue() as *mut Mem;
        }
        return pOut;
    }
}
unsafe extern "C" fn columnMallocFailure(mut pStmt: *mut sqlite3_stmt) {
    unsafe {
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        if !p.is_null() {
            (*p).rc = sqlite3ApiExit((*p).db, (*p).rc);
            sqlite3_mutex_leave((*(*p).db).mutex);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_blob(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> *const ::core::ffi::c_void {
    unsafe {
        let mut val: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        val = sqlite3_value_blob(columnMem(pStmt, i) as *mut sqlite3_value);
        columnMallocFailure(pStmt);
        return val;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_bytes(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut val: ::core::ffi::c_int = sqlite3_value_bytes(
            columnMem(pStmt, i) as *mut sqlite3_value,
        );
        columnMallocFailure(pStmt);
        return val;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_bytes16(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut val: ::core::ffi::c_int = sqlite3_value_bytes16(
            columnMem(pStmt, i) as *mut sqlite3_value,
        );
        columnMallocFailure(pStmt);
        return val;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_double(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    unsafe {
        let mut val: ::core::ffi::c_double = sqlite3_value_double(
            columnMem(pStmt, i) as *mut sqlite3_value,
        );
        columnMallocFailure(pStmt);
        return val;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_int(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut val: ::core::ffi::c_int = sqlite3_value_int(
            columnMem(pStmt, i) as *mut sqlite3_value,
        );
        columnMallocFailure(pStmt);
        return val;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_int64(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> sqlite3_int64 {
    unsafe {
        let mut val: sqlite_int64 = sqlite3_value_int64(
            columnMem(pStmt, i) as *mut sqlite3_value,
        ) as sqlite_int64;
        columnMallocFailure(pStmt);
        return val as sqlite3_int64;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_text(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> *const ::core::ffi::c_uchar {
    unsafe {
        let mut val: *const ::core::ffi::c_uchar = sqlite3_value_text(
            columnMem(pStmt, i) as *mut sqlite3_value,
        );
        columnMallocFailure(pStmt);
        return val;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_value(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> *mut sqlite3_value {
    unsafe {
        let mut pOut: *mut Mem = columnMem(pStmt, i);
        if (*pOut).flags as ::core::ffi::c_int & MEM_Static != 0 {
            (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int & !MEM_Static) as u16_0;
            (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int | MEM_Ephem) as u16_0;
        }
        columnMallocFailure(pStmt);
        return pOut as *mut sqlite3_value;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_text16(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> *const ::core::ffi::c_void {
    unsafe {
        let mut val: *const ::core::ffi::c_void = sqlite3_value_text16(
            columnMem(pStmt, i) as *mut sqlite3_value,
        );
        columnMallocFailure(pStmt);
        return val;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_type(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iType: ::core::ffi::c_int = sqlite3_value_type(
            columnMem(pStmt, i) as *mut sqlite3_value,
        );
        columnMallocFailure(pStmt);
        return iType;
    }
}
static mut azExplainColNames8: [*const ::core::ffi::c_char; 12] = [
    b"addr\0".as_ptr() as *const ::core::ffi::c_char,
    b"opcode\0".as_ptr() as *const ::core::ffi::c_char,
    b"p1\0".as_ptr() as *const ::core::ffi::c_char,
    b"p2\0".as_ptr() as *const ::core::ffi::c_char,
    b"p3\0".as_ptr() as *const ::core::ffi::c_char,
    b"p4\0".as_ptr() as *const ::core::ffi::c_char,
    b"p5\0".as_ptr() as *const ::core::ffi::c_char,
    b"comment\0".as_ptr() as *const ::core::ffi::c_char,
    b"id\0".as_ptr() as *const ::core::ffi::c_char,
    b"parent\0".as_ptr() as *const ::core::ffi::c_char,
    b"notused\0".as_ptr() as *const ::core::ffi::c_char,
    b"detail\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut azExplainColNames16data: [u16_0; 60] = [
    'a' as i32 as u16_0,
    'd' as i32 as u16_0,
    'd' as i32 as u16_0,
    'r' as i32 as u16_0,
    0 as ::core::ffi::c_int as u16_0,
    'o' as i32 as u16_0,
    'p' as i32 as u16_0,
    'c' as i32 as u16_0,
    'o' as i32 as u16_0,
    'd' as i32 as u16_0,
    'e' as i32 as u16_0,
    0 as ::core::ffi::c_int as u16_0,
    'p' as i32 as u16_0,
    '1' as i32 as u16_0,
    0 as ::core::ffi::c_int as u16_0,
    'p' as i32 as u16_0,
    '2' as i32 as u16_0,
    0 as ::core::ffi::c_int as u16_0,
    'p' as i32 as u16_0,
    '3' as i32 as u16_0,
    0 as ::core::ffi::c_int as u16_0,
    'p' as i32 as u16_0,
    '4' as i32 as u16_0,
    0 as ::core::ffi::c_int as u16_0,
    'p' as i32 as u16_0,
    '5' as i32 as u16_0,
    0 as ::core::ffi::c_int as u16_0,
    'c' as i32 as u16_0,
    'o' as i32 as u16_0,
    'm' as i32 as u16_0,
    'm' as i32 as u16_0,
    'e' as i32 as u16_0,
    'n' as i32 as u16_0,
    't' as i32 as u16_0,
    0 as ::core::ffi::c_int as u16_0,
    'i' as i32 as u16_0,
    'd' as i32 as u16_0,
    0 as ::core::ffi::c_int as u16_0,
    'p' as i32 as u16_0,
    'a' as i32 as u16_0,
    'r' as i32 as u16_0,
    'e' as i32 as u16_0,
    'n' as i32 as u16_0,
    't' as i32 as u16_0,
    0 as ::core::ffi::c_int as u16_0,
    'n' as i32 as u16_0,
    'o' as i32 as u16_0,
    't' as i32 as u16_0,
    'u' as i32 as u16_0,
    's' as i32 as u16_0,
    'e' as i32 as u16_0,
    'd' as i32 as u16_0,
    0 as ::core::ffi::c_int as u16_0,
    'd' as i32 as u16_0,
    'e' as i32 as u16_0,
    't' as i32 as u16_0,
    'a' as i32 as u16_0,
    'i' as i32 as u16_0,
    'l' as i32 as u16_0,
    0 as ::core::ffi::c_int as u16_0,
];
static mut iExplainColNames16: [u8_0; 12] = [
    0 as ::core::ffi::c_int as u8_0,
    5 as ::core::ffi::c_int as u8_0,
    12 as ::core::ffi::c_int as u8_0,
    15 as ::core::ffi::c_int as u8_0,
    18 as ::core::ffi::c_int as u8_0,
    21 as ::core::ffi::c_int as u8_0,
    24 as ::core::ffi::c_int as u8_0,
    27 as ::core::ffi::c_int as u8_0,
    35 as ::core::ffi::c_int as u8_0,
    38 as ::core::ffi::c_int as u8_0,
    45 as ::core::ffi::c_int as u8_0,
    53 as ::core::ffi::c_int as u8_0,
];
unsafe extern "C" fn columnName(
    mut pStmt: *mut sqlite3_stmt,
    mut N: ::core::ffi::c_int,
    mut useUtf16: ::core::ffi::c_int,
    mut useType: ::core::ffi::c_int,
) -> *const ::core::ffi::c_void {
    unsafe {
        let mut ret: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut p: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut n: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if N < 0 as ::core::ffi::c_int {
            return ::core::ptr::null::<::core::ffi::c_void>();
        }
        ret = ::core::ptr::null::<::core::ffi::c_void>();
        p = pStmt as *mut Vdbe;
        db = (*p).db;
        sqlite3_mutex_enter((*db).mutex);
        if (*p).explain() != 0 {
            if !(useType > 0 as ::core::ffi::c_int) {
                n = if (*p).explain() as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    8 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                };
                if !(N >= n) {
                    if useUtf16 != 0 {
                        let mut i: ::core::ffi::c_int = iExplainColNames16[(N
                            + 8 as ::core::ffi::c_int
                                * (*p).explain() as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
                        ret = (&raw const azExplainColNames16data as *const u16_0)
                            .offset(i as isize) as *const u16_0
                            as *mut ::core::ffi::c_void;
                    } else {
                        ret = azExplainColNames8[(N
                            + 8 as ::core::ffi::c_int
                                * (*p).explain() as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as usize]
                            as *mut ::core::ffi::c_void;
                    }
                }
            }
        } else {
            n = (*p).nResColumn as ::core::ffi::c_int;
            if N < n {
                let mut prior_mallocFailed: u8_0 = (*db).mallocFailed;
                N += useType * n;
                if useUtf16 != 0 {
                    ret = sqlite3_value_text16(
                        (*p).aColName.offset(N as isize) as *mut Mem
                            as *mut sqlite3_value,
                    );
                } else {
                    ret = sqlite3_value_text(
                        (*p).aColName.offset(N as isize) as *mut Mem
                            as *mut sqlite3_value,
                    ) as *const ::core::ffi::c_void;
                }
                if (*db).mallocFailed as ::core::ffi::c_int
                    > prior_mallocFailed as ::core::ffi::c_int
                {
                    sqlite3OomClear(db);
                    ret = ::core::ptr::null::<::core::ffi::c_void>();
                }
            }
        }
        sqlite3_mutex_leave((*db).mutex);
        return ret;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_name(
    mut pStmt: *mut sqlite3_stmt,
    mut N: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        return columnName(pStmt, N, 0 as ::core::ffi::c_int, COLNAME_NAME)
            as *const ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_column_name16(
    mut pStmt: *mut sqlite3_stmt,
    mut N: ::core::ffi::c_int,
) -> *const ::core::ffi::c_void {
    unsafe {
        return columnName(pStmt, N, 1 as ::core::ffi::c_int, COLNAME_NAME);
    }
}
unsafe extern "C" fn vdbeUnbind(
    mut p: *mut Vdbe,
    mut i: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVar: *mut Mem = ::core::ptr::null_mut::<Mem>();
        if vdbeSafetyNotNull(p) != 0 {
            return sqlite3MisuseError(1639 as ::core::ffi::c_int);
        }
        sqlite3_mutex_enter((*(*p).db).mutex);
        if (*p).eVdbeState as ::core::ffi::c_int != VDBE_READY_STATE {
            sqlite3Error((*p).db, sqlite3MisuseError(1643 as ::core::ffi::c_int));
            sqlite3_mutex_leave((*(*p).db).mutex);
            sqlite3_log(
                SQLITE_MISUSE,
                b"bind on a busy prepared statement: [%s]\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zSql,
            );
            return sqlite3MisuseError(1647 as ::core::ffi::c_int);
        }
        if i >= (*p).nVar as ::core::ffi::c_uint {
            sqlite3Error((*p).db, SQLITE_RANGE);
            sqlite3_mutex_leave((*(*p).db).mutex);
            return SQLITE_RANGE;
        }
        pVar = (*p).aVar.offset(i as isize) as *mut Mem;
        sqlite3VdbeMemRelease(pVar);
        (*pVar).flags = MEM_Null as u16_0;
        (*(*p).db).errCode = SQLITE_OK;
        if (*p).expmask != 0 as ::core::ffi::c_uint
            && (*p).expmask as ::core::ffi::c_uint
                & (if i >= 31 as ::core::ffi::c_uint {
                    0x80000000 as ::core::ffi::c_uint
                } else {
                    (1 as ::core::ffi::c_int as ::core::ffi::c_uint) << i
                }) != 0 as ::core::ffi::c_uint
        {
            (*p).set_expired(1 as bft as bft);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn bindText(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut zData: *const ::core::ffi::c_void,
    mut nData: i64_0,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    mut encoding: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        let mut pVar: *mut Mem = ::core::ptr::null_mut::<Mem>();
        let mut rc: ::core::ffi::c_int = 0;
        rc = vdbeUnbind(p, (i - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        if rc == SQLITE_OK {
            if !zData.is_null() {
                pVar = (*p).aVar.offset((i - 1 as ::core::ffi::c_int) as isize)
                    as *mut Mem;
                rc = sqlite3VdbeMemSetStr(
                    pVar,
                    zData as *const ::core::ffi::c_char,
                    nData,
                    encoding,
                    xDel,
                );
                if rc == SQLITE_OK {
                    if encoding as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        (*pVar).enc = (*(*p).db).enc;
                    } else {
                        rc = sqlite3VdbeChangeEncoding(
                            pVar,
                            (*(*p).db).enc as ::core::ffi::c_int,
                        );
                    }
                }
                if rc != 0 {
                    sqlite3Error((*p).db, rc);
                    rc = sqlite3ApiExit((*p).db, rc);
                }
            }
            sqlite3_mutex_leave((*(*p).db).mutex);
        } else if xDel.is_some()
            && xDel
                != ::core::mem::transmute::<
                    ::libc::intptr_t,
                    sqlite3_destructor_type,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t)
        {
            xDel.expect("non-null function pointer")(zData as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_blob(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut zData: *const ::core::ffi::c_void,
    mut nData: ::core::ffi::c_int,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        return bindText(pStmt, i, zData, nData as i64_0, xDel, 0 as u8_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_blob64(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut zData: *const ::core::ffi::c_void,
    mut nData: sqlite3_uint64,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        return bindText(pStmt, i, zData, nData as i64_0, xDel, 0 as u8_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_double(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut rValue: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        rc = vdbeUnbind(p, (i - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        if rc == SQLITE_OK {
            sqlite3VdbeMemSetDouble(
                (*p).aVar.offset((i - 1 as ::core::ffi::c_int) as isize) as *mut Mem,
                rValue,
            );
            sqlite3_mutex_leave((*(*p).db).mutex);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_int(
    mut p: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut iValue: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3_bind_int64(p, i, iValue as sqlite_int64);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_int64(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut iValue: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        rc = vdbeUnbind(p, (i - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        if rc == SQLITE_OK {
            sqlite3VdbeMemSetInt64(
                (*p).aVar.offset((i - 1 as ::core::ffi::c_int) as isize) as *mut Mem,
                iValue as i64_0,
            );
            sqlite3_mutex_leave((*(*p).db).mutex);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_null(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        rc = vdbeUnbind(p, (i - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        if rc == SQLITE_OK {
            sqlite3_mutex_leave((*(*p).db).mutex);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_pointer(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut pPtr: *mut ::core::ffi::c_void,
    mut zPTtype: *const ::core::ffi::c_char,
    mut xDestructor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        rc = vdbeUnbind(p, (i - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        if rc == SQLITE_OK {
            sqlite3VdbeMemSetPointer(
                (*p).aVar.offset((i - 1 as ::core::ffi::c_int) as isize) as *mut Mem,
                pPtr,
                zPTtype,
                xDestructor,
            );
            sqlite3_mutex_leave((*(*p).db).mutex);
        } else if xDestructor.is_some() {
            xDestructor.expect("non-null function pointer")(pPtr);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_text(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut zData: *const ::core::ffi::c_char,
    mut nData: ::core::ffi::c_int,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        return bindText(
            pStmt,
            i,
            zData as *const ::core::ffi::c_void,
            nData as i64_0,
            xDel,
            SQLITE_UTF8 as u8_0,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_text64(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut zData: *const ::core::ffi::c_char,
    mut nData: sqlite3_uint64,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    mut enc: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    unsafe {
        if enc as ::core::ffi::c_int != SQLITE_UTF8 {
            if enc as ::core::ffi::c_int == SQLITE_UTF16 {
                enc = SQLITE_UTF16NATIVE as ::core::ffi::c_uchar;
            }
            nData &= !(1 as ::core::ffi::c_int as u64_0) as ::core::ffi::c_ulonglong;
        }
        return bindText(
            pStmt,
            i,
            zData as *const ::core::ffi::c_void,
            nData as i64_0,
            xDel,
            enc as u8_0,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_text16(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut zData: *const ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        return bindText(
            pStmt,
            i,
            zData,
            (n as u64_0 & !(1 as ::core::ffi::c_int as u64_0)) as i64_0,
            xDel,
            SQLITE_UTF16NATIVE as u8_0,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_value(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut pValue: *const sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        match sqlite3_value_type(pValue as *mut sqlite3_value) {
            SQLITE_INTEGER => {
                rc = sqlite3_bind_int64(pStmt, i, (*pValue).u.i as sqlite_int64);
            }
            SQLITE_FLOAT => {
                rc = sqlite3_bind_double(
                    pStmt,
                    i,
                    if (*pValue).flags as ::core::ffi::c_int & MEM_Real != 0 {
                        (*pValue).u.r
                    } else {
                        (*pValue).u.i as ::core::ffi::c_double
                    },
                );
            }
            SQLITE_BLOB => {
                if (*pValue).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                    rc = sqlite3_bind_zeroblob(pStmt, i, (*pValue).u.nZero);
                } else {
                    rc = sqlite3_bind_blob(
                        pStmt,
                        i,
                        (*pValue).z as *const ::core::ffi::c_void,
                        (*pValue).n,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                }
            }
            SQLITE_TEXT => {
                rc = bindText(
                    pStmt,
                    i,
                    (*pValue).z as *const ::core::ffi::c_void,
                    (*pValue).n as i64_0,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    (*pValue).enc,
                );
            }
            _ => {
                rc = sqlite3_bind_null(pStmt, i);
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_zeroblob(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        rc = vdbeUnbind(p, (i - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        if rc == SQLITE_OK {
            sqlite3VdbeMemSetZeroBlob(
                (*p).aVar.offset((i - 1 as ::core::ffi::c_int) as isize) as *mut Mem,
                n,
            );
            sqlite3_mutex_leave((*(*p).db).mutex);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_zeroblob64(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
    mut n: sqlite3_uint64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        sqlite3_mutex_enter((*(*p).db).mutex);
        if n > (*(*p).db).aLimit[SQLITE_LIMIT_LENGTH as usize] as u64_0 {
            rc = SQLITE_TOOBIG;
        } else {
            rc = sqlite3_bind_zeroblob(pStmt, i, n as ::core::ffi::c_int);
        }
        rc = sqlite3ApiExit((*p).db, rc);
        sqlite3_mutex_leave((*(*p).db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_parameter_count(
    mut pStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        return if !p.is_null() {
            (*p).nVar as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_parameter_name(
    mut pStmt: *mut sqlite3_stmt,
    mut i: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        if p.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        return sqlite3VListNumToName((*p).pVList, i);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeParameterIndex(
    mut p: *mut Vdbe,
    mut zName: *const ::core::ffi::c_char,
    mut nName: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if p.is_null() || zName.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        return sqlite3VListNameToNum((*p).pVList, zName, nName);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_bind_parameter_index(
    mut pStmt: *mut sqlite3_stmt,
    mut zName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3VdbeParameterIndex(
            pStmt as *mut Vdbe,
            zName,
            sqlite3Strlen30(zName),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TransferBindings(
    mut pFromStmt: *mut sqlite3_stmt,
    mut pToStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFrom: *mut Vdbe = pFromStmt as *mut Vdbe;
        let mut pTo: *mut Vdbe = pToStmt as *mut Vdbe;
        let mut i: ::core::ffi::c_int = 0;
        sqlite3_mutex_enter((*(*pTo).db).mutex);
        i = 0 as ::core::ffi::c_int;
        while i < (*pFrom).nVar as ::core::ffi::c_int {
            sqlite3VdbeMemMove(
                (*pTo).aVar.offset(i as isize) as *mut Mem,
                (*pFrom).aVar.offset(i as isize) as *mut Mem,
            );
            i += 1;
        }
        sqlite3_mutex_leave((*(*pTo).db).mutex);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_db_handle(
    mut pStmt: *mut sqlite3_stmt,
) -> *mut sqlite3 {
    unsafe {
        return if !pStmt.is_null() {
            (*(pStmt as *mut Vdbe)).db
        } else {
            ::core::ptr::null_mut::<sqlite3>()
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_stmt_readonly(
    mut pStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        return if !pStmt.is_null() {
            (*(pStmt as *mut Vdbe)).readOnly() as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_stmt_isexplain(
    mut pStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        return if !pStmt.is_null() {
            (*(pStmt as *mut Vdbe)).explain() as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_stmt_explain(
    mut pStmt: *mut sqlite3_stmt,
    mut eMode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: *mut Vdbe = pStmt as *mut Vdbe;
        let mut rc: ::core::ffi::c_int = 0;
        sqlite3_mutex_enter((*(*v).db).mutex);
        if (*v).explain() as ::core::ffi::c_int == eMode {
            rc = SQLITE_OK;
        } else if eMode < 0 as ::core::ffi::c_int || eMode > 2 as ::core::ffi::c_int {
            rc = SQLITE_ERROR;
        } else if (*v).prepFlags as ::core::ffi::c_int & SQLITE_PREPARE_SAVESQL
            == 0 as ::core::ffi::c_int
        {
            rc = SQLITE_ERROR;
        } else if (*v).eVdbeState as ::core::ffi::c_int != VDBE_READY_STATE {
            rc = SQLITE_BUSY;
        } else if (*v).nMem >= 10 as ::core::ffi::c_int
            && (eMode != 2 as ::core::ffi::c_int
                || (*v).haveEqpOps() as ::core::ffi::c_int != 0)
        {
            (*v).set_explain(eMode as bft as bft);
            rc = SQLITE_OK;
        } else {
            (*v).set_explain(eMode as bft as bft);
            rc = sqlite3Reprepare(v);
            (*v)
                .set_haveEqpOps(
                    (eMode == 2 as ::core::ffi::c_int) as ::core::ffi::c_int as bft
                        as bft,
                );
        }
        if (*v).explain() != 0 {
            (*v).nResColumn = (12 as ::core::ffi::c_int
                - 4 as ::core::ffi::c_int * (*v).explain() as ::core::ffi::c_int)
                as u16_0;
        } else {
            (*v).nResColumn = (*v).nResAlloc;
        }
        sqlite3_mutex_leave((*(*v).db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_stmt_busy(
    mut pStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: *mut Vdbe = pStmt as *mut Vdbe;
        return (!v.is_null() && (*v).eVdbeState as ::core::ffi::c_int == VDBE_RUN_STATE)
            as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_next_stmt(
    mut pDb: *mut sqlite3,
    mut pStmt: *mut sqlite3_stmt,
) -> *mut sqlite3_stmt {
    unsafe {
        let mut pNext: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        sqlite3_mutex_enter((*pDb).mutex);
        if pStmt.is_null() {
            pNext = (*pDb).pVdbe as *mut sqlite3_stmt;
        } else {
            pNext = (*(pStmt as *mut Vdbe)).pVNext as *mut sqlite3_stmt;
        }
        sqlite3_mutex_leave((*pDb).mutex);
        return pNext;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_stmt_status(
    mut pStmt: *mut sqlite3_stmt,
    mut op: ::core::ffi::c_int,
    mut resetFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVdbe: *mut Vdbe = pStmt as *mut Vdbe;
        let mut v: u32_0 = 0;
        if op == SQLITE_STMTSTATUS_MEMUSED {
            let mut db: *mut sqlite3 = (*pVdbe).db;
            sqlite3_mutex_enter((*db).mutex);
            v = 0 as u32_0;
            (*db).pnBytesFreed = &raw mut v as *mut ::core::ffi::c_int;
            (*db).lookaside.pEnd = (*db).lookaside.pStart;
            sqlite3VdbeDelete(pVdbe);
            (*db).pnBytesFreed = ::core::ptr::null_mut::<::core::ffi::c_int>();
            (*db).lookaside.pEnd = (*db).lookaside.pTrueEnd;
            sqlite3_mutex_leave((*db).mutex);
        } else {
            v = (*pVdbe).aCounter[op as usize];
            if resetFlag != 0 {
                (*pVdbe).aCounter[op as usize] = 0 as u32_0;
            }
        }
        return v as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_sql(
    mut pStmt: *mut sqlite3_stmt,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        return if !p.is_null() {
            (*p).zSql
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_expanded_sql(
    mut pStmt: *mut sqlite3_stmt,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zSql: *const ::core::ffi::c_char = sqlite3_sql(pStmt);
        if !zSql.is_null() {
            let mut p: *mut Vdbe = pStmt as *mut Vdbe;
            sqlite3_mutex_enter((*(*p).db).mutex);
            z = sqlite3VdbeExpandSql(p, zSql);
            sqlite3_mutex_leave((*(*p).db).mutex);
        }
        return z;
    }
}
pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
