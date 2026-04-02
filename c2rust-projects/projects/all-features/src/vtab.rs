use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type PreUpdate;
    pub type RenameToken;
    pub type Vdbe;
    pub type TableLock;
    pub type sqlite3_mutex;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3HashInsert(
        _: *mut Hash,
        pKey: *const ::core::ffi::c_char,
        pData: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3HashFind(
        _: *const Hash,
        pKey: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
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
    fn sqlite3VdbeAddOp0(_: *mut Vdbe, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddOp2(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeLoadString(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddParseSchemaOp(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: u16_0,
    );
    fn sqlite3VdbeFinalize(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3MisuseError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3ColumnType(
        _: *mut Column,
        _: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3Malloc(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3MallocZero(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbStrNDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: u64_0,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3Realloc(_: *mut ::core::ffi::c_void, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbRealloc(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: u64_0,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3RunParser(
        _: *mut Parse,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprListDelete(_: *mut sqlite3, _: *mut ExprList);
    fn sqlite3PrimaryKeyIndex(_: *mut Table) -> *mut Index;
    fn sqlite3StartTable(
        _: *mut Parse,
        _: *mut Token,
        _: *mut Token,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3DeleteTable(_: *mut sqlite3, _: *mut Table);
    fn sqlite3FindTable(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut Table;
    fn sqlite3NameFromToken(
        _: *mut sqlite3,
        _: *const Token,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3GetVdbe(_: *mut Parse) -> *mut Vdbe;
    fn sqlite3MayAbort(_: *mut Parse);
    fn sqlite3ChangeCookie(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3AuthCheck(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ErrorWithMsg(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3Error(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3GetToken(
        _: *const ::core::ffi::c_uchar,
        _: *mut ::core::ffi::c_int,
    ) -> i64_0;
    fn sqlite3NestedParse(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3ApiExit(db: *mut sqlite3, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3VtabImportErrmsg(_: *mut Vdbe, _: *mut sqlite3_vtab);
    fn sqlite3MarkAllShadowTablesOf(_: *mut sqlite3, _: *mut Table);
    fn sqlite3ParseObjectInit(_: *mut Parse, _: *mut sqlite3);
    fn sqlite3ParseObjectReset(_: *mut Parse);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VtabCtx {
    pub pVTable: *mut VTable,
    pub pTab: *mut Table,
    pub pPrior: *mut VtabCtx,
    pub bDeclared: ::core::ffi::c_int

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
pub type size_t = usize;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_ABORT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_LOCKED: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_IGNORE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_CREATE_VTABLE: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_COLUMN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_VTAB_CONSTRAINT_SUPPORT: ::core::ffi::c_int = 1;
pub const SQLITE_VTAB_INNOCUOUS: ::core::ffi::c_int = 2;
pub const SQLITE_VTAB_DIRECTONLY: ::core::ffi::c_int = 3;
pub const SQLITE_VTAB_USES_ALL_SCHEMAS: ::core::ffi::c_int = 4;
pub const SQLITE_ROLLBACK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_FAIL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_REPLACE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const OP_Expire: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const OP_VCreate: ::core::ffi::c_int = 172 as ::core::ffi::c_int;
pub const TK_TABLE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const TK_CREATE: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const TK_COLUMN: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
pub const TK_SPACE: ::core::ffi::c_int = 184 as ::core::ffi::c_int;
pub const TK_COMMENT: ::core::ffi::c_int = 185 as ::core::ffi::c_int;
pub const SQLITE_Defensive: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_EPHEM: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SAVEPOINT_BEGIN: ::core::ffi::c_int = 0;
pub const SAVEPOINT_ROLLBACK: ::core::ffi::c_int = 2;
pub const COLFLAG_HIDDEN: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_VTABRISK_Low: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_VTABRISK_Normal: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_VTABRISK_High: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TF_HasHidden: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_NoVisibleRowid: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const TF_OOOHidden: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const TF_Ephemeral: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const TF_Eponymous: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PARSE_MODE_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PARSE_MODE_DECLARE_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabCreateModule(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut pModule: *const sqlite3_module,
    mut pAux: *mut ::core::ffi::c_void,
    mut xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> *mut Module {
    unsafe {
        let mut pMod: *mut Module = ::core::ptr::null_mut::<Module>();
        let mut pDel: *mut Module = ::core::ptr::null_mut::<Module>();
        let mut zCopy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if pModule.is_null() {
            zCopy = zName as *mut ::core::ffi::c_char;
            pMod = ::core::ptr::null_mut::<Module>();
        } else {
            let mut nName: ::core::ffi::c_int = sqlite3Strlen30(zName);
            pMod = sqlite3Malloc(
                (::core::mem::size_of::<Module>() as usize)
                    .wrapping_add(nName as usize)
                    .wrapping_add(1 as usize) as u64_0,
            ) as *mut Module;
            if pMod.is_null() {
                sqlite3OomFault(db);
                return ::core::ptr::null_mut::<Module>();
            }
            zCopy = pMod.offset(1 as ::core::ffi::c_int as isize) as *mut Module
                as *mut ::core::ffi::c_char;
            memcpy(
                zCopy as *mut ::core::ffi::c_void,
                zName as *const ::core::ffi::c_void,
                (nName + 1 as ::core::ffi::c_int) as size_t,
            );
            (*pMod).zName = zCopy;
            (*pMod).pModule = pModule;
            (*pMod).pAux = pAux;
            (*pMod).xDestroy = xDestroy;
            (*pMod).pEpoTab = ::core::ptr::null_mut::<Table>();
            (*pMod).nRefModule = 1 as ::core::ffi::c_int;
        }
        pDel = sqlite3HashInsert(
            &raw mut (*db).aModule,
            zCopy,
            pMod as *mut ::core::ffi::c_void,
        ) as *mut Module;
        if !pDel.is_null() {
            if pDel == pMod {
                sqlite3OomFault(db);
                sqlite3DbFree(db, pDel as *mut ::core::ffi::c_void);
                pMod = ::core::ptr::null_mut::<Module>();
            } else {
                sqlite3VtabEponymousTableClear(db, pDel);
                sqlite3VtabModuleUnref(db, pDel);
            }
        }
        return pMod;
    }
}
unsafe extern "C" fn createModule(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut pModule: *const sqlite3_module,
    mut pAux: *mut ::core::ffi::c_void,
    mut xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        sqlite3_mutex_enter((*db).mutex);
        sqlite3VtabCreateModule(db, zName, pModule, pAux, xDestroy);
        rc = sqlite3ApiExit(db, rc);
        if rc != SQLITE_OK && xDestroy.is_some() {
            xDestroy.expect("non-null function pointer")(pAux);
        }
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_module(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut pModule: *const sqlite3_module,
    mut pAux: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        return createModule(db, zName, pModule, pAux, None);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_module_v2(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut pModule: *const sqlite3_module,
    mut pAux: *mut ::core::ffi::c_void,
    mut xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        return createModule(db, zName, pModule, pAux, xDestroy);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_drop_modules(
    mut db: *mut sqlite3,
    mut azNames: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pThis: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut pNext: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut c2rust_current_block_5: u64;
        pThis = (*db).aModule.first;
        while !pThis.is_null() {
            let mut pMod: *mut Module = (*pThis).data as *mut Module;
            pNext = (*pThis).next;
            if !azNames.is_null() {
                let mut ii: ::core::ffi::c_int = 0;
                ii = 0 as ::core::ffi::c_int;
                while !(*azNames.offset(ii as isize)).is_null()
                    && strcmp(*azNames.offset(ii as isize), (*pMod).zName)
                        != 0 as ::core::ffi::c_int
                {
                    ii += 1;
                }
                if !(*azNames.offset(ii as isize)).is_null() {
                    c2rust_current_block_5 = 11174649648027449784;
                } else {
                    c2rust_current_block_5 = 13513818773234778473;
                }
            } else {
                c2rust_current_block_5 = 13513818773234778473;
            }
            match c2rust_current_block_5 {
                13513818773234778473 => {
                    createModule(
                        db,
                        (*pMod).zName,
                        ::core::ptr::null::<sqlite3_module>(),
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        None,
                    );
                }
                _ => {}
            }
            pThis = pNext;
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabModuleUnref(
    mut db: *mut sqlite3,
    mut pMod: *mut Module,
) {
    unsafe {
        (*pMod).nRefModule -= 1;
        if (*pMod).nRefModule == 0 as ::core::ffi::c_int {
            if (*pMod).xDestroy.is_some() {
                (*pMod).xDestroy.expect("non-null function pointer")((*pMod).pAux);
            }
            sqlite3DbFree(db, pMod as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabLock(mut pVTab: *mut VTable) {
    unsafe {
        (*pVTab).nRef += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GetVTable(
    mut db: *mut sqlite3,
    mut pTab: *mut Table,
) -> *mut VTable {
    unsafe {
        let mut pVtab: *mut VTable = ::core::ptr::null_mut::<VTable>();
        pVtab = (*pTab).u.vtab.p;
        while !pVtab.is_null() && (*pVtab).db != db {
            pVtab = (*pVtab).pNext;
        }
        return pVtab;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabUnlock(mut pVTab: *mut VTable) {
    unsafe {
        let mut db: *mut sqlite3 = (*pVTab).db;
        (*pVTab).nRef -= 1;
        if (*pVTab).nRef == 0 as ::core::ffi::c_int {
            let mut p: *mut sqlite3_vtab = (*pVTab).pVtab;
            if !p.is_null() {
                (*(*p).pModule).xDisconnect.expect("non-null function pointer")(p);
            }
            sqlite3VtabModuleUnref((*pVTab).db, (*pVTab).pMod);
            sqlite3DbFree(db, pVTab as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn vtabDisconnectAll(
    mut db: *mut sqlite3,
    mut p: *mut Table,
) -> *mut VTable {
    unsafe {
        let mut pRet: *mut VTable = ::core::ptr::null_mut::<VTable>();
        let mut pVTable: *mut VTable = ::core::ptr::null_mut::<VTable>();
        pVTable = (*p).u.vtab.p;
        (*p).u.vtab.p = ::core::ptr::null_mut::<VTable>();
        while !pVTable.is_null() {
            let mut db2: *mut sqlite3 = (*pVTable).db;
            let mut pNext: *mut VTable = (*pVTable).pNext;
            if db2 == db {
                pRet = pVTable;
                (*p).u.vtab.p = pRet;
                (*pRet).pNext = ::core::ptr::null_mut::<VTable>();
            } else {
                (*pVTable).pNext = (*db2).pDisconnect;
                (*db2).pDisconnect = pVTable;
            }
            pVTable = pNext;
        }
        return pRet;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabDisconnect(mut db: *mut sqlite3, mut p: *mut Table) {
    unsafe {
        let mut ppVTab: *mut *mut VTable = ::core::ptr::null_mut::<*mut VTable>();
        ppVTab = &raw mut (*p).u.vtab.p;
        while !(*ppVTab).is_null() {
            if (**ppVTab).db == db {
                let mut pVTab: *mut VTable = *ppVTab;
                *ppVTab = (*pVTab).pNext;
                sqlite3VtabUnlock(pVTab);
                break;
            } else {
                ppVTab = &raw mut (**ppVTab).pNext;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabUnlockList(mut db: *mut sqlite3) {
    unsafe {
        let mut p: *mut VTable = (*db).pDisconnect;
        if !p.is_null() {
            (*db).pDisconnect = ::core::ptr::null_mut::<VTable>();
            loop {
                let mut pNext: *mut VTable = (*p).pNext;
                sqlite3VtabUnlock(p);
                p = pNext;
                if p.is_null() {
                    break;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabClear(mut db: *mut sqlite3, mut p: *mut Table) {
    unsafe {
        if (*db).pnBytesFreed.is_null() {
            vtabDisconnectAll(::core::ptr::null_mut::<sqlite3>(), p);
        }
        if !(*p).u.vtab.azArg.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*p).u.vtab.nArg {
                if i != 1 as ::core::ffi::c_int {
                    sqlite3DbFree(
                        db,
                        *(*p).u.vtab.azArg.offset(i as isize) as *mut ::core::ffi::c_void,
                    );
                }
                i += 1;
            }
            sqlite3DbFree(db, (*p).u.vtab.azArg as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn addModuleArgument(
    mut pParse: *mut Parse,
    mut pTable: *mut Table,
    mut zArg: *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut nBytes: sqlite3_int64 = 0;
        let mut azModuleArg: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            *mut ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = (*pParse).db;
        nBytes = (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            .wrapping_mul((2 as ::core::ffi::c_int + (*pTable).u.vtab.nArg) as usize)
            as sqlite3_int64;
        if (*pTable).u.vtab.nArg + 3 as ::core::ffi::c_int
            >= (*db).aLimit[SQLITE_LIMIT_COLUMN as usize]
        {
            sqlite3ErrorMsg(
                pParse,
                b"too many columns on %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*pTable).zName,
            );
        }
        azModuleArg = sqlite3DbRealloc(
            db,
            (*pTable).u.vtab.azArg as *mut ::core::ffi::c_void,
            nBytes as u64_0,
        ) as *mut *mut ::core::ffi::c_char;
        if azModuleArg.is_null() {
            sqlite3DbFree(db, zArg as *mut ::core::ffi::c_void);
        } else {
            let c2rust_fresh2 = (*pTable).u.vtab.nArg;
            (*pTable).u.vtab.nArg = (*pTable).u.vtab.nArg + 1;
            let mut i: ::core::ffi::c_int = c2rust_fresh2;
            let ref mut c2rust_fresh3 = *azModuleArg.offset(i as isize);
            *c2rust_fresh3 = zArg;
            let ref mut c2rust_fresh4 = *azModuleArg
                .offset((i + 1 as ::core::ffi::c_int) as isize);
            *c2rust_fresh4 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*pTable).u.vtab.azArg = azModuleArg;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabBeginParse(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
    mut pModuleName: *mut Token,
    mut ifNotExists: ::core::ffi::c_int,
) {
    unsafe {
        let mut pTable: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        sqlite3StartTable(
            pParse,
            pName1,
            pName2,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            ifNotExists,
        );
        pTable = (*pParse).pNewTable;
        if pTable.is_null() {
            return;
        }
        (*pTable).eTabType = TABTYP_VTAB as u8_0;
        db = (*pParse).db;
        addModuleArgument(pParse, pTable, sqlite3NameFromToken(db, pModuleName));
        addModuleArgument(
            pParse,
            pTable,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        addModuleArgument(pParse, pTable, sqlite3DbStrDup(db, (*pTable).zName));
        (*pParse).sNameToken.n = ((*pModuleName).z.offset((*pModuleName).n as isize)
            as *const ::core::ffi::c_char)
            .offset_from((*pParse).sNameToken.z) as ::core::ffi::c_long
            as ::core::ffi::c_int as ::core::ffi::c_uint;
        if !(*pTable).u.vtab.azArg.is_null() {
            let mut iDb: ::core::ffi::c_int = sqlite3SchemaToIndex(
                db,
                (*pTable).pSchema,
            );
            sqlite3AuthCheck(
                pParse,
                SQLITE_CREATE_VTABLE,
                (*pTable).zName,
                *(*pTable).u.vtab.azArg.offset(0 as ::core::ffi::c_int as isize),
                (*(*(*pParse).db).aDb.offset(iDb as isize)).zDbSName,
            );
        }
    }
}
unsafe extern "C" fn addArgumentToVtab(mut pParse: *mut Parse) {
    unsafe {
        if !(*pParse).sArg.z.is_null() && !(*pParse).pNewTable.is_null() {
            let mut z: *const ::core::ffi::c_char = (*pParse).sArg.z;
            let mut n: ::core::ffi::c_int = (*pParse).sArg.n as ::core::ffi::c_int;
            let mut db: *mut sqlite3 = (*pParse).db;
            addModuleArgument(
                pParse,
                (*pParse).pNewTable,
                sqlite3DbStrNDup(db, z, n as u64_0),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabFinishParse(
    mut pParse: *mut Parse,
    mut pEnd: *mut Token,
) {
    unsafe {
        let mut pTab: *mut Table = (*pParse).pNewTable;
        let mut db: *mut sqlite3 = (*pParse).db;
        if pTab.is_null() {
            return;
        }
        addArgumentToVtab(pParse);
        (*pParse).sArg.z = ::core::ptr::null::<::core::ffi::c_char>();
        if (*pTab).u.vtab.nArg < 1 as ::core::ffi::c_int {
            return;
        }
        if (*db).init.busy == 0 {
            let mut zStmt: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut zWhere: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut iDb: ::core::ffi::c_int = 0;
            let mut iReg: ::core::ffi::c_int = 0;
            let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
            sqlite3MayAbort(pParse);
            if !pEnd.is_null() {
                (*pParse).sNameToken.n = ((*pEnd).z.offset_from((*pParse).sNameToken.z)
                    as ::core::ffi::c_long as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_add((*pEnd).n);
            }
            zStmt = sqlite3MPrintf(
                db,
                b"CREATE VIRTUAL TABLE %T\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*pParse).sNameToken,
            );
            iDb = sqlite3SchemaToIndex(db, (*pTab).pSchema);
            sqlite3NestedParse(
                pParse,
                b"UPDATE %Q.sqlite_master SET type='table', name=%Q, tbl_name=%Q, rootpage=0, sql=%Q WHERE rowid=#%d\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*(*db).aDb.offset(iDb as isize)).zDbSName,
                (*pTab).zName,
                (*pTab).zName,
                zStmt,
                (*pParse).u1.cr.regRowid,
            );
            v = sqlite3GetVdbe(pParse);
            sqlite3ChangeCookie(pParse, iDb);
            sqlite3VdbeAddOp0(v, OP_Expire);
            zWhere = sqlite3MPrintf(
                db,
                b"name=%Q AND sql=%Q\0".as_ptr() as *const ::core::ffi::c_char,
                (*pTab).zName,
                zStmt,
            );
            sqlite3VdbeAddParseSchemaOp(v, iDb, zWhere, 0 as u16_0);
            sqlite3DbFree(db, zStmt as *mut ::core::ffi::c_void);
            (*pParse).nMem += 1;
            iReg = (*pParse).nMem;
            sqlite3VdbeLoadString(v, iReg, (*pTab).zName);
            sqlite3VdbeAddOp2(v, OP_VCreate, iDb, iReg);
        } else {
            let mut pOld: *mut Table = ::core::ptr::null_mut::<Table>();
            let mut pSchema: *mut Schema = (*pTab).pSchema;
            let mut zName: *const ::core::ffi::c_char = (*pTab).zName;
            sqlite3MarkAllShadowTablesOf(db, pTab);
            pOld = sqlite3HashInsert(
                &raw mut (*pSchema).tblHash,
                zName,
                pTab as *mut ::core::ffi::c_void,
            ) as *mut Table;
            if !pOld.is_null() {
                sqlite3OomFault(db);
                return;
            }
            (*pParse).pNewTable = ::core::ptr::null_mut::<Table>();
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabArgInit(mut pParse: *mut Parse) {
    unsafe {
        addArgumentToVtab(pParse);
        (*pParse).sArg.z = ::core::ptr::null::<::core::ffi::c_char>();
        (*pParse).sArg.n = 0 as ::core::ffi::c_uint;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabArgExtend(
    mut pParse: *mut Parse,
    mut p: *mut Token,
) {
    unsafe {
        let mut pArg: *mut Token = &raw mut (*pParse).sArg;
        if (*pArg).z.is_null() {
            (*pArg).z = (*p).z;
            (*pArg).n = (*p).n;
        } else {
            (*pArg).n = ((*p).z.offset((*p).n as isize) as *const ::core::ffi::c_char)
                .offset_from((*pArg).z) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint;
        };
    }
}
unsafe extern "C" fn vtabCallConstructor(
    mut db: *mut sqlite3,
    mut pTab: *mut Table,
    mut pMod: *mut Module,
    mut xConstruct: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sCtx: VtabCtx = VtabCtx {
            pVTable: ::core::ptr::null_mut::<VTable>(),
            pTab: ::core::ptr::null_mut::<Table>(),
            pPrior: ::core::ptr::null_mut::<VtabCtx>(),
            bDeclared: 0

        };
        let mut pVTable: *mut VTable = ::core::ptr::null_mut::<VTable>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut azArg: *const *const ::core::ffi::c_char = ::core::ptr::null::<
            *const ::core::ffi::c_char,
        >();
        let mut nArg: ::core::ffi::c_int = (*pTab).u.vtab.nArg;
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zModuleName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut iDb: ::core::ffi::c_int = 0;
        let mut pCtx: *mut VtabCtx = ::core::ptr::null_mut::<VtabCtx>();
        azArg = (*pTab).u.vtab.azArg as *const *const ::core::ffi::c_char;
        pCtx = (*db).pVtabCtx;
        while !pCtx.is_null() {
            if (*pCtx).pTab == pTab {
                *pzErr = sqlite3MPrintf(
                    db,
                    b"vtable constructor called recursively: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*pTab).zName,
                );
                return SQLITE_LOCKED;
            }
            pCtx = (*pCtx).pPrior;
        }
        zModuleName = sqlite3DbStrDup(db, (*pTab).zName);
        if zModuleName.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        pVTable = sqlite3MallocZero(::core::mem::size_of::<VTable>() as u64_0)
            as *mut VTable;
        if pVTable.is_null() {
            sqlite3OomFault(db);
            sqlite3DbFree(db, zModuleName as *mut ::core::ffi::c_void);
            return SQLITE_NOMEM_BKPT;
        }
        (*pVTable).db = db;
        (*pVTable).pMod = pMod;
        (*pVTable).eVtabRisk = SQLITE_VTABRISK_Normal as u8_0;
        iDb = sqlite3SchemaToIndex(db, (*pTab).pSchema);
        let ref mut c2rust_fresh0 = *(*pTab)
            .u
            .vtab
            .azArg
            .offset(1 as ::core::ffi::c_int as isize);
        *c2rust_fresh0 = (*(*db).aDb.offset(iDb as isize)).zDbSName;
        sCtx.pTab = pTab;
        sCtx.pVTable = pVTable;
        sCtx.pPrior = (*db).pVtabCtx;
        sCtx.bDeclared = 0 as ::core::ffi::c_int;
        (*db).pVtabCtx = &raw mut sCtx;
        (*pTab).nTabRef = (*pTab).nTabRef.wrapping_add(1);
        rc = xConstruct
            .expect(
                "non-null function pointer",
            )(db, (*pMod).pAux, nArg, azArg, &raw mut (*pVTable).pVtab, &raw mut zErr);
        sqlite3DeleteTable(db, pTab);
        (*db).pVtabCtx = sCtx.pPrior;
        if rc == SQLITE_NOMEM {
            sqlite3OomFault(db);
        }
        if SQLITE_OK != rc {
            if zErr.is_null() {
                *pzErr = sqlite3MPrintf(
                    db,
                    b"vtable constructor failed: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    zModuleName,
                );
            } else {
                *pzErr = sqlite3MPrintf(
                    db,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    zErr,
                );
                sqlite3_free(zErr as *mut ::core::ffi::c_void);
            }
            sqlite3DbFree(db, pVTable as *mut ::core::ffi::c_void);
        } else if !(*pVTable).pVtab.is_null() {
            memset(
                (*pVTable).pVtab as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<sqlite3_vtab>() as size_t,
            );
            (*(*pVTable).pVtab).pModule = (*pMod).pModule;
            (*pMod).nRefModule += 1;
            (*pVTable).nRef = 1 as ::core::ffi::c_int;
            if sCtx.bDeclared == 0 as ::core::ffi::c_int {
                let mut zFormat: *const ::core::ffi::c_char = b"vtable constructor did not declare schema: %s\0"
                    .as_ptr() as *const ::core::ffi::c_char;
                *pzErr = sqlite3MPrintf(db, zFormat, zModuleName);
                sqlite3VtabUnlock(pVTable);
                rc = SQLITE_ERROR;
            } else {
                let mut iCol: ::core::ffi::c_int = 0;
                let mut oooHidden: u16_0 = 0 as u16_0;
                (*pVTable).pNext = (*pTab).u.vtab.p;
                (*pTab).u.vtab.p = pVTable;
                iCol = 0 as ::core::ffi::c_int;
                while iCol < (*pTab).nCol as ::core::ffi::c_int {
                    let mut zType: *mut ::core::ffi::c_char = sqlite3ColumnType(
                        (*pTab).aCol.offset(iCol as isize) as *mut Column,
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    let mut nType: ::core::ffi::c_int = 0;
                    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    nType = sqlite3Strlen30(zType);
                    i = 0 as ::core::ffi::c_int;
                    while i < nType {
                        if 0 as ::core::ffi::c_int
                            == sqlite3_strnicmp(
                                b"hidden\0".as_ptr() as *const ::core::ffi::c_char,
                                zType.offset(i as isize) as *mut ::core::ffi::c_char,
                                6 as ::core::ffi::c_int,
                            )
                            && (i == 0 as ::core::ffi::c_int
                                || *zType.offset((i - 1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int == ' ' as i32)
                            && (*zType.offset((i + 6 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int == '\0' as i32
                                || *zType.offset((i + 6 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int == ' ' as i32)
                        {
                            break;
                        }
                        i += 1;
                    }
                    if i < nType {
                        let mut j: ::core::ffi::c_int = 0;
                        let mut nDel: ::core::ffi::c_int = 6 as ::core::ffi::c_int
                            + (if *zType.offset((i + 6 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            });
                        j = i;
                        while j + nDel <= nType {
                            *zType.offset(j as isize) = *zType
                                .offset((j + nDel) as isize);
                            j += 1;
                        }
                        if *zType.offset(i as isize) as ::core::ffi::c_int == '\0' as i32
                            && i > 0 as ::core::ffi::c_int
                        {
                            *zType.offset((i - 1 as ::core::ffi::c_int) as isize) = '\0'
                                as i32 as ::core::ffi::c_char;
                        }
                        let ref mut c2rust_fresh1 = (*(*pTab).aCol.offset(iCol as isize))
                            .colFlags;
                        *c2rust_fresh1 = (*c2rust_fresh1 as ::core::ffi::c_int
                            | COLFLAG_HIDDEN) as u16_0;
                        (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                            | TF_HasHidden as ::core::ffi::c_uint) as u32_0;
                        oooHidden = TF_OOOHidden as u16_0;
                    } else {
                        (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                            | oooHidden as ::core::ffi::c_uint) as u32_0;
                    }
                    iCol += 1;
                }
            }
        }
        sqlite3DbFree(db, zModuleName as *mut ::core::ffi::c_void);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabCallConnect(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut zMod: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pMod: *mut Module = ::core::ptr::null_mut::<Module>();
        let mut rc: ::core::ffi::c_int = 0;
        if !sqlite3GetVTable(db, pTab).is_null() {
            return SQLITE_OK;
        }
        zMod = *(*pTab).u.vtab.azArg.offset(0 as ::core::ffi::c_int as isize);
        pMod = sqlite3HashFind(&raw mut (*db).aModule, zMod) as *mut Module;
        if pMod.is_null() {
            let mut zModule: *const ::core::ffi::c_char = *(*pTab)
                .u
                .vtab
                .azArg
                .offset(0 as ::core::ffi::c_int as isize);
            sqlite3ErrorMsg(
                pParse,
                b"no such module: %s\0".as_ptr() as *const ::core::ffi::c_char,
                zModule,
            );
            rc = SQLITE_ERROR;
        } else {
            let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            rc = vtabCallConstructor(
                db,
                pTab,
                pMod,
                (*(*pMod).pModule).xConnect,
                &raw mut zErr,
            );
            if rc != SQLITE_OK {
                sqlite3ErrorMsg(
                    pParse,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    zErr,
                );
                (*pParse).rc = rc;
            }
            sqlite3DbFree(db, zErr as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn growVTrans(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        let ARRAY_INCR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
        if (*db).nVTrans % ARRAY_INCR == 0 as ::core::ffi::c_int {
            let mut aVTrans: *mut *mut VTable = ::core::ptr::null_mut::<*mut VTable>();
            let mut nBytes: sqlite3_int64 = (::core::mem::size_of::<*mut sqlite3_vtab>()
                as ::core::ffi::c_ulonglong)
                .wrapping_mul(
                    ((*db).nVTrans as ::core::ffi::c_longlong
                        + ARRAY_INCR as ::core::ffi::c_longlong)
                        as ::core::ffi::c_ulonglong,
                ) as sqlite3_int64;
            aVTrans = sqlite3DbRealloc(
                db,
                (*db).aVTrans as *mut ::core::ffi::c_void,
                nBytes as u64_0,
            ) as *mut *mut VTable;
            if aVTrans.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            memset(
                aVTrans.offset((*db).nVTrans as isize) as *mut *mut VTable
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<*mut sqlite3_vtab>() as size_t)
                    .wrapping_mul(ARRAY_INCR as size_t),
            );
            (*db).aVTrans = aVTrans;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn addToVTrans(mut db: *mut sqlite3, mut pVTab: *mut VTable) {
    unsafe {
        let c2rust_fresh7 = (*db).nVTrans;
        (*db).nVTrans = (*db).nVTrans + 1;
        let ref mut c2rust_fresh8 = *(*db).aVTrans.offset(c2rust_fresh7 as isize);
        *c2rust_fresh8 = pVTab;
        sqlite3VtabLock(pVTab);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabCallCreate(
    mut db: *mut sqlite3,
    mut iDb: ::core::ffi::c_int,
    mut zTab: *const ::core::ffi::c_char,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pMod: *mut Module = ::core::ptr::null_mut::<Module>();
        let mut zMod: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        pTab = sqlite3FindTable(db, zTab, (*(*db).aDb.offset(iDb as isize)).zDbSName);
        zMod = *(*pTab).u.vtab.azArg.offset(0 as ::core::ffi::c_int as isize);
        pMod = sqlite3HashFind(&raw mut (*db).aModule, zMod) as *mut Module;
        if pMod.is_null() || (*(*pMod).pModule).xCreate.is_none()
            || (*(*pMod).pModule).xDestroy.is_none()
        {
            *pzErr = sqlite3MPrintf(
                db,
                b"no such module: %s\0".as_ptr() as *const ::core::ffi::c_char,
                zMod,
            );
            rc = SQLITE_ERROR;
        } else {
            rc = vtabCallConstructor(db, pTab, pMod, (*(*pMod).pModule).xCreate, pzErr);
        }
        if rc == SQLITE_OK && !sqlite3GetVTable(db, pTab).is_null() {
            rc = growVTrans(db);
            if rc == SQLITE_OK {
                addToVTrans(db, sqlite3GetVTable(db, pTab));
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_declare_vtab(
    mut db: *mut sqlite3,
    mut zCreateTable: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCtx: *mut VtabCtx = ::core::ptr::null_mut::<VtabCtx>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
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
        let mut initBusy: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut z: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        static mut aKeyword: [u8_0; 3] = [
            TK_CREATE as u8_0,
            TK_TABLE as u8_0,
            0 as ::core::ffi::c_int as u8_0,
        ];
        z = zCreateTable as *const ::core::ffi::c_uchar;
        i = 0 as ::core::ffi::c_int;
        while aKeyword[i as usize] != 0 {
            let mut tokenType: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            loop {
                z = z.offset(sqlite3GetToken(z, &raw mut tokenType) as isize);
                if !(tokenType == TK_SPACE || tokenType == TK_COMMENT) {
                    break;
                }
            }
            if tokenType != aKeyword[i as usize] as ::core::ffi::c_int {
                sqlite3ErrorWithMsg(
                    db,
                    SQLITE_ERROR,
                    b"syntax error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return SQLITE_ERROR;
            }
            i += 1;
        }
        sqlite3_mutex_enter((*db).mutex);
        pCtx = (*db).pVtabCtx;
        if pCtx.is_null() || (*pCtx).bDeclared != 0 {
            sqlite3Error(db, sqlite3MisuseError(846 as ::core::ffi::c_int));
            sqlite3_mutex_leave((*db).mutex);
            return sqlite3MisuseError(848 as ::core::ffi::c_int);
        }
        pTab = (*pCtx).pTab;
        sqlite3ParseObjectInit(&raw mut sParse, db);
        sParse.eParseMode = PARSE_MODE_DECLARE_VTAB as u8_0;
        sParse.disableTriggers = 1 as u8_0;
        initBusy = (*db).init.busy as ::core::ffi::c_int;
        (*db).init.busy = 0 as u8_0;
        sParse.nQueryLoop = 1 as LogEst;
        if SQLITE_OK == sqlite3RunParser(&raw mut sParse, zCreateTable) {
            if (*pTab).aCol.is_null() {
                let mut pNew: *mut Table = sParse.pNewTable;
                let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
                (*pTab).aCol = (*pNew).aCol;
                sqlite3ExprListDelete(db, (*pNew).u.tab.pDfltList);
                (*pTab).nCol = (*pNew).nCol;
                (*pTab).nNVCol = (*pTab).nCol;
                (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                    | ((*pNew).tabFlags & (TF_WithoutRowid | TF_NoVisibleRowid) as u32_0)
                        as ::core::ffi::c_uint) as u32_0;
                (*pNew).nCol = 0 as i16_0;
                (*pNew).aCol = ::core::ptr::null_mut::<Column>();
                if !((*pNew).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0)
                    && (*(*(*(*pCtx).pVTable).pMod).pModule).xUpdate.is_some()
                    && (*sqlite3PrimaryKeyIndex(pNew)).nKeyCol as ::core::ffi::c_int
                        != 1 as ::core::ffi::c_int
                {
                    rc = SQLITE_ERROR;
                }
                pIdx = (*pNew).pIndex;
                if !pIdx.is_null() {
                    (*pTab).pIndex = pIdx;
                    (*pNew).pIndex = ::core::ptr::null_mut::<Index>();
                    (*pIdx).pTable = pTab;
                }
            }
            (*pCtx).bDeclared = 1 as ::core::ffi::c_int;
        } else {
            sqlite3ErrorWithMsg(
                db,
                SQLITE_ERROR,
                if !sParse.zErrMsg.is_null() {
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                sParse.zErrMsg,
            );
            sqlite3DbFree(db, sParse.zErrMsg as *mut ::core::ffi::c_void);
            rc = SQLITE_ERROR;
        }
        sParse.eParseMode = PARSE_MODE_NORMAL as u8_0;
        if !sParse.pVdbe.is_null() {
            sqlite3VdbeFinalize(sParse.pVdbe);
        }
        sqlite3DeleteTable(db, sParse.pNewTable);
        sqlite3ParseObjectReset(&raw mut sParse);
        (*db).init.busy = initBusy as u8_0;
        rc = sqlite3ApiExit(db, rc);
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabCallDestroy(
    mut db: *mut sqlite3,
    mut iDb: ::core::ffi::c_int,
    mut zTab: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        pTab = sqlite3FindTable(db, zTab, (*(*db).aDb.offset(iDb as isize)).zDbSName);
        if !pTab.is_null()
            && (*pTab).eTabType as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && !(*pTab).u.vtab.p.is_null()
        {
            let mut p: *mut VTable = ::core::ptr::null_mut::<VTable>();
            let mut xDestroy: Option<
                unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int
            > = None;
            p = (*pTab).u.vtab.p;
            while !p.is_null() {
                if (*(*p).pVtab).nRef > 0 as ::core::ffi::c_int {
                    return SQLITE_LOCKED;
                }
                p = (*p).pNext;
            }
            p = vtabDisconnectAll(db, pTab);
            xDestroy = (*(*(*p).pMod).pModule).xDestroy;
            if xDestroy.is_none() {
                xDestroy = (*(*(*p).pMod).pModule).xDisconnect;
            }
            (*pTab).nTabRef = (*pTab).nTabRef.wrapping_add(1);
            rc = xDestroy.expect("non-null function pointer")((*p).pVtab);
            if rc == SQLITE_OK {
                (*p).pVtab = ::core::ptr::null_mut::<sqlite3_vtab>();
                (*pTab).u.vtab.p = ::core::ptr::null_mut::<VTable>();
                sqlite3VtabUnlock(p);
            }
            sqlite3DeleteTable(db, pTab);
        }
        return rc;
    }
}
unsafe extern "C" fn callFinaliser(
    mut db: *mut sqlite3,
    mut offset: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if !(*db).aVTrans.is_null() {
            let mut aVTrans: *mut *mut VTable = (*db).aVTrans;
            (*db).aVTrans = ::core::ptr::null_mut::<*mut VTable>();
            i = 0 as ::core::ffi::c_int;
            while i < (*db).nVTrans {
                let mut pVTab: *mut VTable = *aVTrans.offset(i as isize);
                let mut p: *mut sqlite3_vtab = (*pVTab).pVtab;
                if !p.is_null() {
                    let mut x: Option<
                        unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int
                    > = None;
                    x = *(((*p).pModule as *mut ::core::ffi::c_char)
                        .offset(offset as isize)
                        as *mut Option<
                            unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                        >);
                    if x.is_some() {
                        x.expect("non-null function pointer")(p);
                    }
                }
                (*pVTab).iSavepoint = 0 as ::core::ffi::c_int;
                sqlite3VtabUnlock(pVTab);
                i += 1;
            }
            sqlite3DbFree(db, aVTrans as *mut ::core::ffi::c_void);
            (*db).nVTrans = 0 as ::core::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabSync(
    mut db: *mut sqlite3,
    mut p: *mut Vdbe,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut aVTrans: *mut *mut VTable = (*db).aVTrans;
        (*db).aVTrans = ::core::ptr::null_mut::<*mut VTable>();
        i = 0 as ::core::ffi::c_int;
        while rc == SQLITE_OK && i < (*db).nVTrans {
            let mut x: Option<
                unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int
            > = None;
            let mut pVtab: *mut sqlite3_vtab = (**aVTrans.offset(i as isize)).pVtab;
            if !pVtab.is_null()
                && {
                    x = (*(*pVtab).pModule).xSync;
                    x.is_some()
                }
            {
                rc = x.expect("non-null function pointer")(pVtab);
                sqlite3VtabImportErrmsg(p, pVtab);
            }
            i += 1;
        }
        (*db).aVTrans = aVTrans;
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabRollback(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        callFinaliser(db, 136 as ::core::ffi::c_ulong as ::core::ffi::c_int);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabCommit(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        callFinaliser(db, 128 as ::core::ffi::c_ulong as ::core::ffi::c_int);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabBegin(
    mut db: *mut sqlite3,
    mut pVTab: *mut VTable,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pModule: *const sqlite3_module = ::core::ptr::null::<sqlite3_module>();
        if (*db).nVTrans > 0 as ::core::ffi::c_int && (*db).aVTrans.is_null() {
            return SQLITE_LOCKED;
        }
        if pVTab.is_null() {
            return SQLITE_OK;
        }
        pModule = (*(*pVTab).pVtab).pModule;
        if (*pModule).xBegin.is_some() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*db).nVTrans {
                if *(*db).aVTrans.offset(i as isize) == pVTab {
                    return SQLITE_OK;
                }
                i += 1;
            }
            rc = growVTrans(db);
            if rc == SQLITE_OK {
                rc = (*pModule)
                    .xBegin
                    .expect("non-null function pointer")((*pVTab).pVtab);
                if rc == SQLITE_OK {
                    let mut iSvpt: ::core::ffi::c_int = (*db).nStatement
                        + (*db).nSavepoint;
                    addToVTrans(db, pVTab);
                    if iSvpt != 0 && (*pModule).xSavepoint.is_some() {
                        (*pVTab).iSavepoint = iSvpt;
                        rc = (*pModule)
                            .xSavepoint
                            .expect(
                                "non-null function pointer",
                            )((*pVTab).pVtab, iSvpt - 1 as ::core::ffi::c_int);
                    }
                }
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabSavepoint(
    mut db: *mut sqlite3,
    mut op: ::core::ffi::c_int,
    mut iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !(*db).aVTrans.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while rc == SQLITE_OK && i < (*db).nVTrans {
                let mut pVTab: *mut VTable = *(*db).aVTrans.offset(i as isize);
                let mut pMod: *const sqlite3_module = (*(*pVTab).pMod).pModule;
                if !(*pVTab).pVtab.is_null()
                    && (*pMod).iVersion >= 2 as ::core::ffi::c_int
                {
                    let mut xMethod: Option<
                        unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int
                    > = None;
                    sqlite3VtabLock(pVTab);
                    match op {
                        SAVEPOINT_BEGIN => {
                            xMethod = (*pMod).xSavepoint;
                            (*pVTab).iSavepoint = iSavepoint + 1 as ::core::ffi::c_int;
                        }
                        SAVEPOINT_ROLLBACK => {
                            xMethod = (*pMod).xRollbackTo;
                        }
                        _ => {
                            xMethod = (*pMod).xRelease;
                        }
                    }
                    if xMethod.is_some() && (*pVTab).iSavepoint > iSavepoint {
                        let mut savedFlags: u64_0 = (*db).flags
                            & SQLITE_Defensive as u64_0;
                        (*db).flags
                            &= !(SQLITE_Defensive as u64_0) as ::core::ffi::c_ulonglong;
                        rc = xMethod
                            .expect(
                                "non-null function pointer",
                            )((*pVTab).pVtab, iSavepoint);
                        (*db).flags |= savedFlags as ::core::ffi::c_ulonglong;
                    }
                    sqlite3VtabUnlock(pVTab);
                }
                i += 1;
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabOverloadFunction(
    mut db: *mut sqlite3,
    mut pDef: *mut FuncDef,
    mut nArg: ::core::ffi::c_int,
    mut pExpr: *mut Expr,
) -> *mut FuncDef {
    unsafe {
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pVtab: *mut sqlite3_vtab = ::core::ptr::null_mut::<sqlite3_vtab>();
        let mut pMod: *mut sqlite3_module = ::core::ptr::null_mut::<sqlite3_module>();
        let mut xSFunc: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> ()
        > = None;
        let mut pArg: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut pNew: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
        let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if pExpr.is_null() {
            return pDef;
        }
        if (*pExpr).op as ::core::ffi::c_int != TK_COLUMN {
            return pDef;
        }
        pTab = (*pExpr).y.pTab;
        if pTab.is_null() {
            return pDef;
        }
        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB) {
            return pDef;
        }
        pVtab = (*sqlite3GetVTable(db, pTab)).pVtab;
        pMod = (*pVtab).pModule as *mut sqlite3_module;
        if (*pMod).xFindFunction.is_none() {
            return pDef;
        }
        rc = (*pMod)
            .xFindFunction
            .expect(
                "non-null function pointer",
            )(pVtab, nArg, (*pDef).zName, &raw mut xSFunc, &raw mut pArg);
        if rc == 0 as ::core::ffi::c_int {
            return pDef;
        }
        pNew = sqlite3DbMallocZero(
            db,
            (::core::mem::size_of::<FuncDef>() as usize)
                .wrapping_add(sqlite3Strlen30((*pDef).zName) as usize)
                .wrapping_add(1 as usize) as u64_0,
        ) as *mut FuncDef;
        if pNew.is_null() {
            return pDef;
        }
        *pNew = *pDef;
        (*pNew).zName = pNew.offset(1 as ::core::ffi::c_int as isize) as *mut FuncDef
            as *const ::core::ffi::c_char;
        memcpy(
            pNew.offset(1 as ::core::ffi::c_int as isize) as *mut FuncDef
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            (*pDef).zName as *const ::core::ffi::c_void,
            (sqlite3Strlen30((*pDef).zName) + 1 as ::core::ffi::c_int) as size_t,
        );
        (*pNew).xSFunc = xSFunc;
        (*pNew).pUserData = pArg;
        (*pNew).funcFlags = ((*pNew).funcFlags as ::core::ffi::c_uint
            | SQLITE_FUNC_EPHEM as ::core::ffi::c_uint) as u32_0;
        return pNew;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabMakeWritable(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
) {
    unsafe {
        let mut pToplevel: *mut Parse = if !(*pParse).pToplevel.is_null() {
            (*pParse).pToplevel
        } else {
            pParse
        };
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut apVtabLock: *mut *mut Table = ::core::ptr::null_mut::<*mut Table>();
        i = 0 as ::core::ffi::c_int;
        while i < (*pToplevel).nVtabLock {
            if pTab == *(*pToplevel).apVtabLock.offset(i as isize) {
                return;
            }
            i += 1;
        }
        n = (((*pToplevel).nVtabLock + 1 as ::core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<*mut Table>() as usize)
            as ::core::ffi::c_int;
        apVtabLock = sqlite3Realloc(
            (*pToplevel).apVtabLock as *mut ::core::ffi::c_void,
            n as u64_0,
        ) as *mut *mut Table;
        if !apVtabLock.is_null() {
            (*pToplevel).apVtabLock = apVtabLock;
            let c2rust_fresh5 = (*pToplevel).nVtabLock;
            (*pToplevel).nVtabLock = (*pToplevel).nVtabLock + 1;
            let ref mut c2rust_fresh6 = *(*pToplevel)
                .apVtabLock
                .offset(c2rust_fresh5 as isize);
            *c2rust_fresh6 = pTab;
        } else {
            sqlite3OomFault((*pToplevel).db);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabEponymousTableInit(
    mut pParse: *mut Parse,
    mut pMod: *mut Module,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pModule: *const sqlite3_module = (*pMod).pModule;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        if !(*pMod).pEpoTab.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        if (*pModule).xCreate.is_some() && (*pModule).xCreate != (*pModule).xConnect {
            return 0 as ::core::ffi::c_int;
        }
        pTab = sqlite3DbMallocZero(db, ::core::mem::size_of::<Table>() as u64_0)
            as *mut Table;
        if pTab.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*pTab).zName = sqlite3DbStrDup(db, (*pMod).zName);
        if (*pTab).zName.is_null() {
            sqlite3DbFree(db, pTab as *mut ::core::ffi::c_void);
            return 0 as ::core::ffi::c_int;
        }
        (*pMod).pEpoTab = pTab;
        (*pTab).nTabRef = 1 as u32_0;
        (*pTab).eTabType = TABTYP_VTAB as u8_0;
        (*pTab).pSchema = (*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).pSchema;
        (*pTab).iPKey = -1 as ::core::ffi::c_int as i16_0;
        (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
            | TF_Eponymous as ::core::ffi::c_uint) as u32_0;
        addModuleArgument(pParse, pTab, sqlite3DbStrDup(db, (*pTab).zName));
        addModuleArgument(pParse, pTab, ::core::ptr::null_mut::<::core::ffi::c_char>());
        addModuleArgument(pParse, pTab, sqlite3DbStrDup(db, (*pTab).zName));
        (*db).nSchemaLock = (*db).nSchemaLock.wrapping_add(1);
        rc = vtabCallConstructor(db, pTab, pMod, (*pModule).xConnect, &raw mut zErr);
        (*db).nSchemaLock = (*db).nSchemaLock.wrapping_sub(1);
        if rc != 0 {
            sqlite3ErrorMsg(
                pParse,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                zErr,
            );
            (*pParse).rc = rc;
            sqlite3DbFree(db, zErr as *mut ::core::ffi::c_void);
            sqlite3VtabEponymousTableClear(db, pMod);
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabEponymousTableClear(
    mut db: *mut sqlite3,
    mut pMod: *mut Module,
) {
    unsafe {
        let mut pTab: *mut Table = (*pMod).pEpoTab;
        if !pTab.is_null() {
            (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                | TF_Ephemeral as ::core::ffi::c_uint) as u32_0;
            sqlite3DeleteTable(db, pTab);
            (*pMod).pEpoTab = ::core::ptr::null_mut::<Table>();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vtab_on_conflict(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aMap: [::core::ffi::c_uchar; 5] = [
            SQLITE_ROLLBACK as ::core::ffi::c_uchar,
            SQLITE_ABORT as ::core::ffi::c_uchar,
            SQLITE_FAIL as ::core::ffi::c_uchar,
            SQLITE_IGNORE as ::core::ffi::c_uchar,
            SQLITE_REPLACE as ::core::ffi::c_uchar,
        ];
        return aMap[((*db).vtabOnConflict as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vtab_config(
    mut db: *mut sqlite3,
    mut op: ::core::ffi::c_int,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    unsafe {
        
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut p: *mut VtabCtx = ::core::ptr::null_mut::<VtabCtx>();
        sqlite3_mutex_enter((*db).mutex);
        p = (*db).pVtabCtx;
        if p.is_null() {
            rc = sqlite3MisuseError(1346 as ::core::ffi::c_int);
        } else {
            let mut ap = c2rust_args;
            match op {
                SQLITE_VTAB_CONSTRAINT_SUPPORT => {
                    (*(*p).pVTable).bConstraint = ap.arg::<::core::ffi::c_int>() as u8_0;
                }
                SQLITE_VTAB_INNOCUOUS => {
                    (*(*p).pVTable).eVtabRisk = SQLITE_VTABRISK_Low as u8_0;
                }
                SQLITE_VTAB_DIRECTONLY => {
                    (*(*p).pVTable).eVtabRisk = SQLITE_VTABRISK_High as u8_0;
                }
                SQLITE_VTAB_USES_ALL_SCHEMAS => {
                    (*(*p).pVTable).bAllSchemas = 1 as u8_0;
                }
                _ => {
                    rc = sqlite3MisuseError(1368 as ::core::ffi::c_int);
                }
            }
        }
        if rc != SQLITE_OK {
            sqlite3Error(db, rc);
        }
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
