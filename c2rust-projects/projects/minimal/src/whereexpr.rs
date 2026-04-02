use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type RenameToken;
    pub type Vdbe;
    pub type sqlite3_mutex;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
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
    fn sqlite3WhereGetMask(_: *mut WhereMaskSet, _: ::core::ffi::c_int) -> Bitmask;
    fn sqlite3WhereMalloc(
        pWInfo: *mut WhereInfo,
        nByte: u64_0,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3VdbeChangeP3(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
        P3: ::core::ffi::c_int,
    );
    fn sqlite3VdbeCurrentAddr(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeGetBoundValue(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> *mut sqlite3_value;
    fn sqlite3VdbeSetVarmask(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3GetTempReg(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempReg(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3ExprAlloc(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const Token,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3Expr(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> *mut Expr;
    fn sqlite3PExpr(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *mut Expr,
        _: *mut Expr,
    ) -> *mut Expr;
    fn sqlite3ExprDelete(_: *mut sqlite3, _: *mut Expr);
    fn sqlite3ExprListAppend(
        _: *mut Parse,
        _: *mut ExprList,
        _: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListDelete(_: *mut sqlite3, _: *mut ExprList);
    fn sqlite3ExprCodeTarget(
        _: *mut Parse,
        _: *mut Expr,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCompare(
        _: *const Parse,
        _: *const Expr,
        _: *const Expr,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCompareSkip(
        _: *mut Expr,
        _: *mut Expr,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprIsConstant(_: *mut Parse, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3ExprIsInteger(
        _: *const Expr,
        _: *mut ::core::ffi::c_int,
        _: *mut Parse,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCanBeNull(_: *const Expr) -> ::core::ffi::c_int;
    fn sqlite3ExprDup(
        _: *mut sqlite3,
        _: *const Expr,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3SetJoinExpr(_: *mut Expr, _: ::core::ffi::c_int, _: u32_0);
    fn sqlite3AtoF(
        z: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_double,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3Utf8Read(_: *mut *const u8_0) -> u32_0;
    fn sqlite3LogEst(_: u64_0) -> LogEst;
    fn sqlite3ExprAffinity(pExpr: *const Expr) -> ::core::ffi::c_char;
    fn sqlite3IsBinary(_: *const CollSeq) -> ::core::ffi::c_int;
    fn sqlite3ExprCollSeqMatch(
        _: *mut Parse,
        _: *const Expr,
        _: *const Expr,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprAddCollateString(
        _: *const Parse,
        _: *mut Expr,
        _: *const ::core::ffi::c_char,
    ) -> *mut Expr;
    fn sqlite3ExprSkipCollate(_: *mut Expr) -> *mut Expr;
    fn sqlite3ExprSkipCollateAndLikely(_: *mut Expr) -> *mut Expr;
    fn sqlite3ValueFree(_: *mut sqlite3_value);
    static sqlite3StrBINARY: [::core::ffi::c_char; 0];
    static sqlite3UpperToLower: [::core::ffi::c_uchar; 0];
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    fn sqlite3ExprColUsed(_: *mut Expr) -> Bitmask;
    fn sqlite3IsLikeFunction(
        _: *mut sqlite3,
        _: *mut Expr,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCheckIN(_: *mut Parse, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3GetVTable(_: *mut sqlite3, _: *mut Table) -> *mut VTable;
    fn sqlite3ExprCompareCollSeq(_: *mut Parse, _: *const Expr) -> *mut CollSeq;
    fn sqlite3BinaryCompareCollSeq(
        _: *mut Parse,
        _: *const Expr,
        _: *const Expr,
    ) -> *mut CollSeq;
    fn sqlite3ExprVectorSize(pExpr: *const Expr) -> ::core::ffi::c_int;
    fn sqlite3ExprForVectorField(
        _: *mut Parse,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
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
pub type size_t = usize;
pub type i8_0 = ::core::ffi::c_schar;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct WhereInfo {
    pub pParse: *mut Parse,
    pub pTabList: *mut SrcList,
    pub pOrderBy: *mut ExprList,
    pub pResultSet: *mut ExprList,
    pub pSelect: *mut Select,
    pub aiCurOnePass: [::core::ffi::c_int; 2],
    pub iContinue: ::core::ffi::c_int,
    pub iBreak: ::core::ffi::c_int,
    pub savedNQueryLoop: ::core::ffi::c_int,
    pub wctrlFlags: u16_0,
    pub iLimit: LogEst,
    pub nLevel: u8_0,
    pub nOBSat: i8_0,
    pub eOnePass: u8_0,
    pub eDistinct: u8_0,
    #[bitfield(name = "bDeferredSeek", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "untestedTerms", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "bOrderedInnerLoop", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "sorted", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "bStarDone", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "bStarUsed", ty = "::core::ffi::c_uint", bits = "5..=5")]
    pub bDeferredSeek_untestedTerms_bOrderedInnerLoop_sorted_bStarDone_bStarUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub nRowOut: LogEst,
    pub iTop: ::core::ffi::c_int,
    pub iEndWhere: ::core::ffi::c_int,
    pub pLoops: *mut WhereLoop,
    pub pMemToFree: *mut WhereMemBlock,
    pub revMask: Bitmask,
    pub sWC: WhereClause,
    pub sMaskSet: WhereMaskSet,
    pub a: [WhereLevel; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereLevel {
    pub iLeftJoin: ::core::ffi::c_int,
    pub iTabCur: ::core::ffi::c_int,
    pub iIdxCur: ::core::ffi::c_int,
    pub addrBrk: ::core::ffi::c_int,
    pub addrHalt: ::core::ffi::c_int,
    pub addrNxt: ::core::ffi::c_int,
    pub addrSkip: ::core::ffi::c_int,
    pub addrCont: ::core::ffi::c_int,
    pub addrFirst: ::core::ffi::c_int,
    pub addrBody: ::core::ffi::c_int,
    pub regBignull: ::core::ffi::c_int,
    pub addrBignull: ::core::ffi::c_int,
    pub iLikeRepCntr: u32_0,
    pub addrLikeRep: ::core::ffi::c_int,
    pub regFilter: ::core::ffi::c_int,
    pub pRJ: *mut WhereRightJoin,
    pub iFrom: u8_0,
    pub op: u8_0,
    pub p3: u8_0,
    pub p5: u8_0,
    pub p1: ::core::ffi::c_int,
    pub p2: ::core::ffi::c_int,
    pub u: C2Rust_Unnamed_27,
    pub pWLoop: *mut WhereLoop,
    pub notReady: Bitmask

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereLoop {
    pub prereq: Bitmask,
    pub maskSelf: Bitmask,
    pub iTab: u8_0,
    pub iSortIdx: u8_0,
    pub rSetup: LogEst,
    pub rRun: LogEst,
    pub nOut: LogEst,
    pub u: C2Rust_Unnamed_24,
    pub wsFlags: u32_0,
    pub nLTerm: u16_0,
    pub nSkip: u16_0,
    pub nLSlot: u16_0,
    pub aLTerm: *mut *mut WhereTerm,
    pub pNextLoop: *mut WhereLoop,
    pub aLTermSpace: [*mut WhereTerm; 3]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereTerm {
    pub pExpr: *mut Expr,
    pub pWC: *mut WhereClause,
    pub truthProb: LogEst,
    pub wtFlags: u16_0,
    pub eOperator: u16_0,
    pub nChild: u8_0,
    pub eMatchOp: u8_0,
    pub iParent: ::core::ffi::c_int,
    pub leftCursor: ::core::ffi::c_int,
    pub u: C2Rust_Unnamed_22,
    pub prereqRight: Bitmask,
    pub prereqAll: Bitmask

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_22 {
    pub x: C2Rust_Unnamed_23,
    pub pOrInfo: *mut WhereOrInfo,
    pub pAndInfo: *mut WhereAndInfo

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereAndInfo {
    pub wc: WhereClause

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereClause {
    pub pWInfo: *mut WhereInfo,
    pub pOuter: *mut WhereClause,
    pub op: u8_0,
    pub hasOr: u8_0,
    pub nTerm: ::core::ffi::c_int,
    pub nSlot: ::core::ffi::c_int,
    pub nBase: ::core::ffi::c_int,
    pub a: *mut WhereTerm,
    pub aStatic: [WhereTerm; 8]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereOrInfo {
    pub wc: WhereClause,
    pub indexable: Bitmask

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub leftColumn: ::core::ffi::c_int,
    pub iField: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_24 {
    pub btree: C2Rust_Unnamed_26,
    pub vtab: C2Rust_Unnamed_25

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub idxNum: ::core::ffi::c_int,
    #[bitfield(name = "needFree", ty = "u32_0", bits = "0..=0")]
    #[bitfield(name = "bOmitOffset", ty = "u32_0", bits = "1..=1")]
    #[bitfield(name = "bIdxNumHex", ty = "u32_0", bits = "2..=2")]
    pub needFree_bOmitOffset_bIdxNumHex: [u8; 1],
    pub isOrdered: i8_0,
    pub omitMask: u16_0,
    pub idxStr: *mut ::core::ffi::c_char,
    pub mHandleIn: u32_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub nEq: u16_0,
    pub nBtm: u16_0,
    pub nTop: u16_0,
    pub nDistinctCol: u16_0,
    pub pIndex: *mut Index,
    pub pOrderBy: *mut ExprList

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_27 {
    pub in_0: C2Rust_Unnamed_28,
    pub pCoveringIdx: *mut Index

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub nIn: ::core::ffi::c_int,
    pub aInLoop: *mut InLoop

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InLoop {
    pub iCur: ::core::ffi::c_int,
    pub addrInTop: ::core::ffi::c_int,
    pub iBase: ::core::ffi::c_int,
    pub nPrefix: ::core::ffi::c_int,
    pub eEndLoopOp: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereRightJoin {
    pub iMatch: ::core::ffi::c_int,
    pub regBloom: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int,
    pub addrSubrtn: ::core::ffi::c_int,
    pub endSubrtn: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereMaskSet {
    pub bVarSelect: ::core::ffi::c_int,
    pub n: ::core::ffi::c_int,
    pub ix: [::core::ffi::c_int; 64]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereMemBlock {
    pub pNext: *mut WhereMemBlock,
    pub sz: u64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Op2 {
    pub zOp: *const ::core::ffi::c_char,
    pub eOp2: ::core::ffi::c_uchar

}
pub const SQLITE_TEXT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_UTF16LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_MATCH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LIKE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_GLOB: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_REGEXP: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_NE: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_ISNOT: ::core::ffi::c_int = 69 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_ISNOTNULL: ::core::ffi::c_int = 70
    as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LIMIT: ::core::ffi::c_int = 73 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_OFFSET: ::core::ffi::c_int = 74 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_FUNCTION: ::core::ffi::c_int = 150
    as ::core::ffi::c_int;
pub const TERM_DYNAMIC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const TERM_VIRTUAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const TERM_CODED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const TERM_COPIED: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const TERM_ORINFO: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const TERM_ANDINFO: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const TERM_OK: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const TERM_VNULL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TERM_LIKEOPT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const TERM_LIKE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const TERM_IS: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const TERM_VARSELECT: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const TERM_SLICE: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const WO_IN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const WO_EQ: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WO_LT: ::core::ffi::c_int = WO_EQ << TK_LT - TK_EQ;
pub const WO_LE: ::core::ffi::c_int = WO_EQ << TK_LE - TK_EQ;
pub const WO_GT: ::core::ffi::c_int = WO_EQ << TK_GT - TK_EQ;
pub const WO_GE: ::core::ffi::c_int = WO_EQ << TK_GE - TK_EQ;
pub const WO_AUX: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const WO_IS: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const WO_ISNULL: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const WO_OR: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const WO_AND: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const WO_EQUIV: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const WO_ROWVAL: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const WO_ALL: ::core::ffi::c_int = 0x3fff as ::core::ffi::c_int;
pub const WO_SINGLE: ::core::ffi::c_int = 0x1ff as ::core::ffi::c_int;
unsafe extern "C" fn whereOrInfoDelete(mut db: *mut sqlite3, mut p: *mut WhereOrInfo) {
    unsafe {
        sqlite3WhereClauseClear(&raw mut (*p).wc);
        sqlite3DbFree(db, p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn whereAndInfoDelete(mut db: *mut sqlite3, mut p: *mut WhereAndInfo) {
    unsafe {
        sqlite3WhereClauseClear(&raw mut (*p).wc);
        sqlite3DbFree(db, p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn whereClauseInsert(
    mut pWC: *mut WhereClause,
    mut p: *mut Expr,
    mut wtFlags: u16_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut idx: ::core::ffi::c_int = 0;
        if (*pWC).nTerm >= (*pWC).nSlot {
            let mut pOld: *mut WhereTerm = (*pWC).a;
            let mut db: *mut sqlite3 = (*(*(*pWC).pWInfo).pParse).db;
            (*pWC).a = sqlite3WhereMalloc(
                (*pWC).pWInfo,
                (::core::mem::size_of::<WhereTerm>() as usize)
                    .wrapping_mul((*pWC).nSlot as usize)
                    .wrapping_mul(2 as usize) as u64_0,
            ) as *mut WhereTerm;
            if (*pWC).a.is_null() {
                if wtFlags as ::core::ffi::c_int & TERM_DYNAMIC != 0 {
                    sqlite3ExprDelete(db, p);
                }
                (*pWC).a = pOld;
                return 0 as ::core::ffi::c_int;
            }
            memcpy(
                (*pWC).a as *mut ::core::ffi::c_void,
                pOld as *const ::core::ffi::c_void,
                (::core::mem::size_of::<WhereTerm>() as size_t)
                    .wrapping_mul((*pWC).nTerm as size_t),
            );
            (*pWC).nSlot = (*pWC).nSlot * 2 as ::core::ffi::c_int;
        }
        let c2rust_fresh0 = (*pWC).nTerm;
        (*pWC).nTerm = (*pWC).nTerm + 1;
        idx = c2rust_fresh0;
        pTerm = (*pWC).a.offset(idx as isize) as *mut WhereTerm;
        if wtFlags as ::core::ffi::c_int & TERM_VIRTUAL == 0 as ::core::ffi::c_int {
            (*pWC).nBase = (*pWC).nTerm;
        }
        if !p.is_null()
            && (*p).flags & 0x80000 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
        {
            (*pTerm).truthProb = (sqlite3LogEst((*p).iTable as u64_0)
                as ::core::ffi::c_int - 270 as ::core::ffi::c_int) as LogEst;
        } else {
            (*pTerm).truthProb = 1 as LogEst;
        }
        (*pTerm).pExpr = sqlite3ExprSkipCollateAndLikely(p);
        (*pTerm).wtFlags = wtFlags;
        (*pTerm).pWC = pWC;
        (*pTerm).iParent = -1 as ::core::ffi::c_int;
        memset(
            &raw mut (*pTerm).eOperator as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<WhereTerm>() as size_t).wrapping_sub(20 as size_t),
        );
        return idx;
    }
}
unsafe extern "C" fn allowedOp(mut op: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        if op > TK_GE {
            return 0 as ::core::ffi::c_int;
        }
        if op >= TK_EQ {
            return 1 as ::core::ffi::c_int;
        }
        return (op == TK_IN || op == TK_ISNULL || op == TK_IS) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn exprCommute(mut pParse: *mut Parse, mut pExpr: *mut Expr) -> u16_0 {
    unsafe {
        if (*(*pExpr).pLeft).op as ::core::ffi::c_int == TK_VECTOR
            || (*(*pExpr).pRight).op as ::core::ffi::c_int == TK_VECTOR
            || sqlite3BinaryCompareCollSeq(pParse, (*pExpr).pLeft, (*pExpr).pRight)
                != sqlite3BinaryCompareCollSeq(pParse, (*pExpr).pRight, (*pExpr).pLeft)
        {
            (*pExpr).flags ^= EP_Commuted as ::core::ffi::c_uint;
        }
        let mut t: *mut Expr = (*pExpr).pRight;
        (*pExpr).pRight = (*pExpr).pLeft;
        (*pExpr).pLeft = t;
        if (*pExpr).op as ::core::ffi::c_int >= TK_GT {
            (*pExpr).op = (((*pExpr).op as ::core::ffi::c_int - TK_GT
                ^ 2 as ::core::ffi::c_int) + TK_GT) as u8_0;
        }
        return 0 as u16_0;
    }
}
unsafe extern "C" fn operatorMask(mut op: ::core::ffi::c_int) -> u16_0 {
    unsafe {
        let mut c: u16_0 = 0;
        if op >= TK_EQ {
            c = (WO_EQ << op - TK_EQ) as u16_0;
        } else if op == TK_IN {
            c = WO_IN as u16_0;
        } else if op == TK_ISNULL {
            c = WO_ISNULL as u16_0;
        } else {
            c = WO_IS as u16_0;
        }
        return c;
    }
}
unsafe extern "C" fn isLikeOrGlob(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut ppPrefix: *mut *mut Expr,
    mut pisComplete: *mut ::core::ffi::c_int,
    mut pnoCase: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut z: *const u8_0 = ::core::ptr::null::<u8_0>();
        let mut pRight: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pLeft: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut c: u8_0 = 0;
        let mut cnt: ::core::ffi::c_int = 0;
        let mut wc: [u8_0; 4] = [0; 4];
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut op: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if sqlite3IsLikeFunction(
            db,
            pExpr,
            pnoCase,
            &raw mut wc as *mut u8_0 as *mut ::core::ffi::c_char,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        pList = (*pExpr).x.pList;
        pLeft = (*(&raw mut (*pList).a as *mut ExprList_item)
            .offset(1 as ::core::ffi::c_int as isize))
            .pExpr;
        pRight = sqlite3ExprSkipCollate(
            (*(&raw mut (*pList).a as *mut ExprList_item)
                .offset(0 as ::core::ffi::c_int as isize))
                .pExpr,
        );
        op = (*pRight).op as ::core::ffi::c_int;
        if op == TK_VARIABLE
            && (*db).flags as ::core::ffi::c_ulonglong
                & SQLITE_EnableQPSG as ::core::ffi::c_ulonglong
                == 0 as ::core::ffi::c_ulonglong
        {
            let mut pReprepare: *mut Vdbe = (*pParse).pReprepare;
            let mut iCol: ::core::ffi::c_int = (*pRight).iColumn as ::core::ffi::c_int;
            pVal = sqlite3VdbeGetBoundValue(pReprepare, iCol, SQLITE_AFF_BLOB as u8_0);
            if !pVal.is_null() && sqlite3_value_type(pVal) == SQLITE_TEXT {
                z = sqlite3_value_text(pVal) as *const u8_0;
            }
            sqlite3VdbeSetVarmask((*pParse).pVdbe, iCol);
        } else if op == TK_STRING {
            z = (*pRight).u.zToken as *mut u8_0;
        }
        if !z.is_null() {
            cnt = 0 as ::core::ffi::c_int;
            loop {
                c = *z.offset(cnt as isize);
                if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    && c as ::core::ffi::c_int
                        != wc[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    && c as ::core::ffi::c_int
                        != wc[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    && c as ::core::ffi::c_int
                        != wc[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                {
                    break;
                }
                cnt += 1;
                if c as ::core::ffi::c_int
                    == wc[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    && *z.offset(cnt as isize) as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                    && (*z.offset(cnt as isize) as ::core::ffi::c_int)
                        < 0x80 as ::core::ffi::c_int
                {
                    cnt += 1;
                } else {
                    if !(c as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int) {
                        continue;
                    }
                    let mut z2: *const u8_0 = z
                        .offset(cnt as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize));
                    if c as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
                        || sqlite3Utf8Read(&raw mut z2) == 0xfffd as ::core::ffi::c_uint
                        || (*db).enc as ::core::ffi::c_int == SQLITE_UTF16LE
                    {
                        cnt -= 1;
                        break;
                    } else {
                        cnt = z2.offset_from(z) as ::core::ffi::c_long
                            as ::core::ffi::c_int;
                    }
                }
            }
            if (cnt > 1 as ::core::ffi::c_int
                || cnt > 0 as ::core::ffi::c_int
                    && *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != wc[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                && 255 as ::core::ffi::c_int
                    != *z.offset((cnt - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
            {
                let mut pPrefix: *mut Expr = ::core::ptr::null_mut::<Expr>();
                *pisComplete = (c as ::core::ffi::c_int
                    == wc[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    && *z.offset((cnt + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*db).enc as ::core::ffi::c_int != SQLITE_UTF16LE)
                    as ::core::ffi::c_int;
                pPrefix = sqlite3Expr(db, TK_STRING, z as *mut ::core::ffi::c_char);
                if !pPrefix.is_null() {
                    let mut iFrom: ::core::ffi::c_int = 0;
                    let mut iTo: ::core::ffi::c_int = 0;
                    let mut zNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    zNew = (*pPrefix).u.zToken;
                    *zNew.offset(cnt as isize) = 0 as ::core::ffi::c_char;
                    iTo = 0 as ::core::ffi::c_int;
                    iFrom = iTo;
                    while iFrom < cnt {
                        if *zNew.offset(iFrom as isize) as ::core::ffi::c_int
                            == wc[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        {
                            iFrom += 1;
                        }
                        let c2rust_fresh3 = iTo;
                        iTo = iTo + 1;
                        *zNew.offset(c2rust_fresh3 as isize) = *zNew
                            .offset(iFrom as isize);
                        iFrom += 1;
                    }
                    *zNew.offset(iTo as isize) = 0 as ::core::ffi::c_char;
                    if (*pLeft).op as ::core::ffi::c_int != TK_COLUMN
                        || sqlite3ExprAffinity(pLeft) as ::core::ffi::c_int
                            != SQLITE_AFF_TEXT
                        || (*pLeft).flags as ::core::ffi::c_uint
                            & (0x1000000 as ::core::ffi::c_int
                                | 0x2000000 as ::core::ffi::c_int) as ::core::ffi::c_uint
                            == 0 as ::core::ffi::c_uint && !(*pLeft).y.pTab.is_null()
                            && (*(*pLeft).y.pTab).eTabType as ::core::ffi::c_int
                                == TABTYP_VTAB
                    {
                        let mut isNum: ::core::ffi::c_int = 0;
                        let mut rDummy: ::core::ffi::c_double = 0.;
                        isNum = sqlite3AtoF(
                            zNew,
                            &raw mut rDummy,
                            iTo,
                            SQLITE_UTF8 as u8_0,
                        );
                        if isNum <= 0 as ::core::ffi::c_int {
                            if iTo == 1 as ::core::ffi::c_int
                                && *zNew.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int == '-' as i32
                            {
                                isNum = 1 as ::core::ffi::c_int;
                            } else {
                                let ref mut c2rust_fresh4 = *zNew
                                    .offset((iTo - 1 as ::core::ffi::c_int) as isize);
                                *c2rust_fresh4 += 1;
                                isNum = sqlite3AtoF(
                                    zNew,
                                    &raw mut rDummy,
                                    iTo,
                                    SQLITE_UTF8 as u8_0,
                                );
                                let ref mut c2rust_fresh5 = *zNew
                                    .offset((iTo - 1 as ::core::ffi::c_int) as isize);
                                *c2rust_fresh5 -= 1;
                            }
                        }
                        if isNum > 0 as ::core::ffi::c_int {
                            sqlite3ExprDelete(db, pPrefix);
                            sqlite3ValueFree(pVal);
                            return 0 as ::core::ffi::c_int;
                        }
                    }
                }
                *ppPrefix = pPrefix;
                if op == TK_VARIABLE {
                    let mut v: *mut Vdbe = (*pParse).pVdbe;
                    sqlite3VdbeSetVarmask(v, (*pRight).iColumn as ::core::ffi::c_int);
                    if *pisComplete != 0
                        && *(*pRight).u.zToken.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int != 0
                    {
                        let mut r1: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                        sqlite3ExprCodeTarget(pParse, pRight, r1);
                        sqlite3VdbeChangeP3(
                            v,
                            sqlite3VdbeCurrentAddr(v) - 1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        );
                        sqlite3ReleaseTempReg(pParse, r1);
                    }
                }
            } else {
                z = ::core::ptr::null::<u8_0>();
            }
        }
        rc = !z.is_null() as ::core::ffi::c_int;
        sqlite3ValueFree(pVal);
        return rc;
    }
}
unsafe extern "C" fn isAuxiliaryVtabOperator(
    mut db: *mut sqlite3,
    mut pExpr: *mut Expr,
    mut peOp2: *mut ::core::ffi::c_uchar,
    mut ppLeft: *mut *mut Expr,
    mut ppRight: *mut *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int == TK_FUNCTION {
            static mut aOp: [Op2; 4] = [
                Op2 {
                    zOp: b"match\0".as_ptr() as *const ::core::ffi::c_char,
                    eOp2: SQLITE_INDEX_CONSTRAINT_MATCH as ::core::ffi::c_uchar

                },
                Op2 {
                    zOp: b"glob\0".as_ptr() as *const ::core::ffi::c_char,
                    eOp2: SQLITE_INDEX_CONSTRAINT_GLOB as ::core::ffi::c_uchar

                },
                Op2 {
                    zOp: b"like\0".as_ptr() as *const ::core::ffi::c_char,
                    eOp2: SQLITE_INDEX_CONSTRAINT_LIKE as ::core::ffi::c_uchar

                },
                Op2 {
                    zOp: b"regexp\0".as_ptr() as *const ::core::ffi::c_char,
                    eOp2: SQLITE_INDEX_CONSTRAINT_REGEXP as ::core::ffi::c_uchar

                },
            ];
            let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            let mut pCol: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut i: ::core::ffi::c_int = 0;
            pList = (*pExpr).x.pList;
            if pList.is_null() || (*pList).nExpr != 2 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            pCol = (*(&raw mut (*pList).a as *mut ExprList_item)
                .offset(1 as ::core::ffi::c_int as isize))
                .pExpr;
            if (*pCol).op as ::core::ffi::c_int == TK_COLUMN
                && (*(*pCol).y.pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB
            {
                i = 0 as ::core::ffi::c_int;
                while i
                    < (::core::mem::size_of::<[Op2; 4]>() as usize)
                        .wrapping_div(::core::mem::size_of::<Op2>() as usize)
                        as ::core::ffi::c_int
                {
                    if sqlite3StrICmp((*pExpr).u.zToken, aOp[i as usize].zOp)
                        == 0 as ::core::ffi::c_int
                    {
                        *peOp2 = aOp[i as usize].eOp2;
                        *ppRight = (*(&raw mut (*pList).a as *mut ExprList_item)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .pExpr;
                        *ppLeft = pCol;
                        return 1 as ::core::ffi::c_int;
                    }
                    i += 1;
                }
            }
            pCol = (*(&raw mut (*pList).a as *mut ExprList_item)
                .offset(0 as ::core::ffi::c_int as isize))
                .pExpr;
            if (*pCol).op as ::core::ffi::c_int == TK_COLUMN
                && (*(*pCol).y.pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB
            {
                let mut pVtab: *mut sqlite3_vtab = ::core::ptr::null_mut::<
                    sqlite3_vtab,
                >();
                let mut pMod: *mut sqlite3_module = ::core::ptr::null_mut::<
                    sqlite3_module,
                >();
                let mut xNotUsed: Option<
                    unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> ()
                > = None;
                let mut pNotUsed: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                pVtab = (*sqlite3GetVTable(db, (*pCol).y.pTab)).pVtab;
                pMod = (*pVtab).pModule as *mut sqlite3_module;
                if (*pMod).xFindFunction.is_some() {
                    i = (*pMod)
                        .xFindFunction
                        .expect(
                            "non-null function pointer",
                        )(
                        pVtab,
                        2 as ::core::ffi::c_int,
                        (*pExpr).u.zToken,
                        &raw mut xNotUsed,
                        &raw mut pNotUsed,
                    );
                    if i >= SQLITE_INDEX_CONSTRAINT_FUNCTION {
                        *peOp2 = i as ::core::ffi::c_uchar;
                        *ppRight = (*(&raw mut (*pList).a as *mut ExprList_item)
                            .offset(1 as ::core::ffi::c_int as isize))
                            .pExpr;
                        *ppLeft = pCol;
                        return 1 as ::core::ffi::c_int;
                    }
                }
            }
        } else if (*pExpr).op as ::core::ffi::c_int >= TK_EQ {
            return 0 as ::core::ffi::c_int
        } else if (*pExpr).op as ::core::ffi::c_int == TK_NE
            || (*pExpr).op as ::core::ffi::c_int == TK_ISNOT
            || (*pExpr).op as ::core::ffi::c_int == TK_NOTNULL
        {
            let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pLeft: *mut Expr = (*pExpr).pLeft;
            let mut pRight: *mut Expr = (*pExpr).pRight;
            if (*pLeft).op as ::core::ffi::c_int == TK_COLUMN
                && (*(*pLeft).y.pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB
            {
                res += 1;
            }
            if !pRight.is_null()
                && ((*pRight).op as ::core::ffi::c_int == TK_COLUMN
                    && (*(*pRight).y.pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB)
            {
                res += 1;
                let mut t: *mut Expr = pLeft;
                pLeft = pRight;
                pRight = t;
            }
            *ppLeft = pLeft;
            *ppRight = pRight;
            if (*pExpr).op as ::core::ffi::c_int == TK_NE {
                *peOp2 = SQLITE_INDEX_CONSTRAINT_NE as ::core::ffi::c_uchar;
            }
            if (*pExpr).op as ::core::ffi::c_int == TK_ISNOT {
                *peOp2 = SQLITE_INDEX_CONSTRAINT_ISNOT as ::core::ffi::c_uchar;
            }
            if (*pExpr).op as ::core::ffi::c_int == TK_NOTNULL {
                *peOp2 = SQLITE_INDEX_CONSTRAINT_ISNOTNULL as ::core::ffi::c_uchar;
            }
            return res;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn transferJoinMarkings(
    mut pDerived: *mut Expr,
    mut pBase: *mut Expr,
) {
    unsafe {
        if !pDerived.is_null()
            && (*pBase).flags
                & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int) as u32_0
                != 0 as ::core::ffi::c_uint
        {
            (*pDerived).flags
                |= (*pBase).flags as ::core::ffi::c_uint
                    & (EP_OuterON | EP_InnerON) as ::core::ffi::c_uint;
            (*pDerived).w.iJoin = (*pBase).w.iJoin;
        }
    }
}
unsafe extern "C" fn markTermAsChild(
    mut pWC: *mut WhereClause,
    mut iChild: ::core::ffi::c_int,
    mut iParent: ::core::ffi::c_int,
) {
    unsafe {
        (*(*pWC).a.offset(iChild as isize)).iParent = iParent;
        (*(*pWC).a.offset(iChild as isize)).truthProb = (*(*pWC)
            .a
            .offset(iParent as isize))
            .truthProb;
        let ref mut c2rust_fresh2 = (*(*pWC).a.offset(iParent as isize)).nChild;
        *c2rust_fresh2 = (*c2rust_fresh2).wrapping_add(1);
    }
}
unsafe extern "C" fn whereNthSubterm(
    mut pTerm: *mut WhereTerm,
    mut N: ::core::ffi::c_int,
) -> *mut WhereTerm {
    unsafe {
        if (*pTerm).eOperator as ::core::ffi::c_int != WO_AND {
            return if N == 0 as ::core::ffi::c_int {
                pTerm
            } else {
                ::core::ptr::null_mut::<WhereTerm>()
            };
        }
        if N < (*(*pTerm).u.pAndInfo).wc.nTerm {
            return (*(*pTerm).u.pAndInfo).wc.a.offset(N as isize) as *mut WhereTerm;
        }
        return ::core::ptr::null_mut::<WhereTerm>();
    }
}
unsafe extern "C" fn whereCombineDisjuncts(
    mut pSrc: *mut SrcList,
    mut pWC: *mut WhereClause,
    mut pOne: *mut WhereTerm,
    mut pTwo: *mut WhereTerm,
) {
    unsafe {
        let mut eOp: u16_0 = ((*pOne).eOperator as ::core::ffi::c_int
            | (*pTwo).eOperator as ::core::ffi::c_int) as u16_0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pNew: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut op: ::core::ffi::c_int = 0;
        let mut idxNew: ::core::ffi::c_int = 0;
        if ((*pOne).wtFlags as ::core::ffi::c_int
            | (*pTwo).wtFlags as ::core::ffi::c_int) & TERM_VNULL != 0
        {
            return;
        }
        if (*pOne).eOperator as ::core::ffi::c_int
            & (WO_EQ | WO_LT | WO_LE | WO_GT | WO_GE) == 0 as ::core::ffi::c_int
        {
            return;
        }
        if (*pTwo).eOperator as ::core::ffi::c_int
            & (WO_EQ | WO_LT | WO_LE | WO_GT | WO_GE) == 0 as ::core::ffi::c_int
        {
            return;
        }
        if eOp as ::core::ffi::c_int & (WO_EQ | WO_LT | WO_LE)
            != eOp as ::core::ffi::c_int
            && eOp as ::core::ffi::c_int & (WO_EQ | WO_GT | WO_GE)
                != eOp as ::core::ffi::c_int
        {
            return;
        }
        if sqlite3ExprCompare(
            ::core::ptr::null::<Parse>(),
            (*(*pOne).pExpr).pLeft,
            (*(*pTwo).pExpr).pLeft,
            -1 as ::core::ffi::c_int,
        ) != 0
        {
            return;
        }
        if sqlite3ExprCompare(
            ::core::ptr::null::<Parse>(),
            (*(*pOne).pExpr).pRight,
            (*(*pTwo).pExpr).pRight,
            -1 as ::core::ffi::c_int,
        ) != 0
        {
            return;
        }
        if eOp as ::core::ffi::c_int
            & eOp as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            if eOp as ::core::ffi::c_int & (WO_LT | WO_LE) != 0 {
                eOp = WO_LE as u16_0;
            } else {
                eOp = WO_GE as u16_0;
            }
        }
        db = (*(*(*pWC).pWInfo).pParse).db;
        pNew = sqlite3ExprDup(db, (*pOne).pExpr, 0 as ::core::ffi::c_int);
        if pNew.is_null() {
            return;
        }
        op = TK_EQ;
        while eOp as ::core::ffi::c_int != WO_EQ << op - TK_EQ {
            op += 1;
        }
        (*pNew).op = op as u8_0;
        idxNew = whereClauseInsert(pWC, pNew, (TERM_VIRTUAL | TERM_DYNAMIC) as u16_0);
        exprAnalyze(pSrc, pWC, idxNew);
    }
}
unsafe extern "C" fn exprAnalyzeOrTerm(
    mut pSrc: *mut SrcList,
    mut pWC: *mut WhereClause,
    mut idxTerm: ::core::ffi::c_int,
) {
    unsafe {
        let mut pWInfo: *mut WhereInfo = (*pWC).pWInfo;
        let mut pParse: *mut Parse = (*pWInfo).pParse;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pTerm: *mut WhereTerm = (*pWC).a.offset(idxTerm as isize)
            as *mut WhereTerm;
        let mut pExpr: *mut Expr = (*pTerm).pExpr;
        let mut i: ::core::ffi::c_int = 0;
        let mut pOrWc: *mut WhereClause = ::core::ptr::null_mut::<WhereClause>();
        let mut pOrTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut pOrInfo: *mut WhereOrInfo = ::core::ptr::null_mut::<WhereOrInfo>();
        let mut chngToIN: Bitmask = 0;
        let mut indexable: Bitmask = 0;
        pOrInfo = sqlite3DbMallocZero(db, ::core::mem::size_of::<WhereOrInfo>() as u64_0)
            as *mut WhereOrInfo;
        (*pTerm).u.pOrInfo = pOrInfo;
        if pOrInfo.is_null() {
            return;
        }
        (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int | TERM_ORINFO)
            as u16_0;
        pOrWc = &raw mut (*pOrInfo).wc;
        memset(
            &raw mut (*pOrWc).aStatic as *mut WhereTerm as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[WhereTerm; 8]>() as size_t,
        );
        sqlite3WhereClauseInit(pOrWc, pWInfo);
        sqlite3WhereSplit(pOrWc, pExpr, TK_OR as u8_0);
        sqlite3WhereExprAnalyze(pSrc, pOrWc);
        if (*db).mallocFailed != 0 {
            return;
        }
        indexable = !(0 as ::core::ffi::c_int as Bitmask);
        chngToIN = !(0 as ::core::ffi::c_int as Bitmask);
        i = (*pOrWc).nTerm - 1 as ::core::ffi::c_int;
        pOrTerm = (*pOrWc).a;
        while i >= 0 as ::core::ffi::c_int && indexable != 0 {
            if (*pOrTerm).eOperator as ::core::ffi::c_int & WO_SINGLE
                == 0 as ::core::ffi::c_int
            {
                let mut pAndInfo: *mut WhereAndInfo = ::core::ptr::null_mut::<
                    WhereAndInfo,
                >();
                chngToIN = 0 as Bitmask;
                pAndInfo = sqlite3DbMallocRawNN(
                    db,
                    ::core::mem::size_of::<WhereAndInfo>() as u64_0,
                ) as *mut WhereAndInfo;
                if !pAndInfo.is_null() {
                    let mut pAndWC: *mut WhereClause = ::core::ptr::null_mut::<
                        WhereClause,
                    >();
                    let mut pAndTerm: *mut WhereTerm = ::core::ptr::null_mut::<
                        WhereTerm,
                    >();
                    let mut j: ::core::ffi::c_int = 0;
                    let mut b: Bitmask = 0 as Bitmask;
                    (*pOrTerm).u.pAndInfo = pAndInfo;
                    (*pOrTerm).wtFlags = ((*pOrTerm).wtFlags as ::core::ffi::c_int
                        | TERM_ANDINFO) as u16_0;
                    (*pOrTerm).eOperator = WO_AND as u16_0;
                    (*pOrTerm).leftCursor = -1 as ::core::ffi::c_int;
                    pAndWC = &raw mut (*pAndInfo).wc;
                    memset(
                        &raw mut (*pAndWC).aStatic as *mut WhereTerm
                            as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<[WhereTerm; 8]>() as size_t,
                    );
                    sqlite3WhereClauseInit(pAndWC, (*pWC).pWInfo);
                    sqlite3WhereSplit(pAndWC, (*pOrTerm).pExpr, TK_AND as u8_0);
                    sqlite3WhereExprAnalyze(pSrc, pAndWC);
                    (*pAndWC).pOuter = pWC;
                    if (*db).mallocFailed == 0 {
                        j = 0 as ::core::ffi::c_int;
                        pAndTerm = (*pAndWC).a;
                        while j < (*pAndWC).nTerm {
                            if allowedOp((*(*pAndTerm).pExpr).op as ::core::ffi::c_int)
                                != 0
                                || (*pAndTerm).eOperator as ::core::ffi::c_int == WO_AUX
                            {
                                b
                                    |= sqlite3WhereGetMask(
                                        &raw mut (*pWInfo).sMaskSet,
                                        (*pAndTerm).leftCursor,
                                    ) as ::core::ffi::c_ulonglong;
                            }
                            j += 1;
                            pAndTerm = pAndTerm.offset(1);
                        }
                    }
                    indexable &= b as ::core::ffi::c_ulonglong;
                }
            } else if !((*pOrTerm).wtFlags as ::core::ffi::c_int & TERM_COPIED != 0) {
                let mut b_0: Bitmask = 0;
                b_0 = sqlite3WhereGetMask(
                    &raw mut (*pWInfo).sMaskSet,
                    (*pOrTerm).leftCursor,
                );
                if (*pOrTerm).wtFlags as ::core::ffi::c_int & TERM_VIRTUAL != 0 {
                    let mut pOther: *mut WhereTerm = (*pOrWc)
                        .a
                        .offset((*pOrTerm).iParent as isize) as *mut WhereTerm;
                    b_0
                        |= sqlite3WhereGetMask(
                            &raw mut (*pWInfo).sMaskSet,
                            (*pOther).leftCursor,
                        ) as ::core::ffi::c_ulonglong;
                }
                indexable &= b_0 as ::core::ffi::c_ulonglong;
                if (*pOrTerm).eOperator as ::core::ffi::c_int & WO_EQ
                    == 0 as ::core::ffi::c_int
                {
                    chngToIN = 0 as Bitmask;
                } else {
                    chngToIN &= b_0 as ::core::ffi::c_ulonglong;
                }
            }
            i -= 1;
            pOrTerm = pOrTerm.offset(1);
        }
        (*pOrInfo).indexable = indexable;
        (*pTerm).eOperator = WO_OR as u16_0;
        (*pTerm).leftCursor = -1 as ::core::ffi::c_int;
        if indexable != 0 {
            (*pWC).hasOr = 1 as u8_0;
        }
        if indexable != 0 && (*pOrWc).nTerm == 2 as ::core::ffi::c_int {
            let mut iOne: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pOne: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
            loop {
                let c2rust_fresh6 = iOne;
                iOne = iOne + 1;
                pOne = whereNthSubterm(
                    (*pOrWc).a.offset(0 as ::core::ffi::c_int as isize)
                        as *mut WhereTerm,
                    c2rust_fresh6,
                );
                if pOne.is_null() {
                    break;
                }
                let mut iTwo: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut pTwo: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
                loop {
                    let c2rust_fresh7 = iTwo;
                    iTwo = iTwo + 1;
                    pTwo = whereNthSubterm(
                        (*pOrWc).a.offset(1 as ::core::ffi::c_int as isize)
                            as *mut WhereTerm,
                        c2rust_fresh7,
                    );
                    if pTwo.is_null() {
                        break;
                    }
                    whereCombineDisjuncts(pSrc, pWC, pOne, pTwo);
                }
            }
        }
        if chngToIN != 0 {
            let mut okToChngToIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iColumn: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
            let mut iCursor: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
            let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            j_0 = 0 as ::core::ffi::c_int;
            while j_0 < 2 as ::core::ffi::c_int && okToChngToIN == 0 {
                let mut pLeft: *mut Expr = ::core::ptr::null_mut::<Expr>();
                pOrTerm = (*pOrWc).a;
                i = (*pOrWc).nTerm - 1 as ::core::ffi::c_int;
                while i >= 0 as ::core::ffi::c_int {
                    (*pOrTerm).wtFlags = ((*pOrTerm).wtFlags as ::core::ffi::c_int
                        & !TERM_OK) as u16_0;
                    if !((*pOrTerm).leftCursor == iCursor) {
                        if !(chngToIN
                            & sqlite3WhereGetMask(
                                &raw mut (*pWInfo).sMaskSet,
                                (*pOrTerm).leftCursor,
                            ) == 0 as ::core::ffi::c_ulonglong)
                        {
                            iColumn = (*pOrTerm).u.x.leftColumn;
                            iCursor = (*pOrTerm).leftCursor;
                            pLeft = (*(*pOrTerm).pExpr).pLeft;
                            break;
                        }
                    }
                    i -= 1;
                    pOrTerm = pOrTerm.offset(1);
                }
                if i < 0 as ::core::ffi::c_int {
                    break;
                }
                okToChngToIN = 1 as ::core::ffi::c_int;
                while i >= 0 as ::core::ffi::c_int && okToChngToIN != 0 {
                    if (*pOrTerm).leftCursor != iCursor {
                        (*pOrTerm).wtFlags = ((*pOrTerm).wtFlags as ::core::ffi::c_int
                            & !TERM_OK) as u16_0;
                    } else if (*pOrTerm).u.x.leftColumn != iColumn
                        || iColumn == XN_EXPR
                            && sqlite3ExprCompare(
                                pParse,
                                (*(*pOrTerm).pExpr).pLeft,
                                pLeft,
                                -1 as ::core::ffi::c_int,
                            ) != 0
                    {
                        okToChngToIN = 0 as ::core::ffi::c_int;
                    } else {
                        let mut affLeft: ::core::ffi::c_int = 0;
                        let mut affRight: ::core::ffi::c_int = 0;
                        affRight = sqlite3ExprAffinity((*(*pOrTerm).pExpr).pRight)
                            as ::core::ffi::c_int;
                        affLeft = sqlite3ExprAffinity((*(*pOrTerm).pExpr).pLeft)
                            as ::core::ffi::c_int;
                        if affRight != 0 as ::core::ffi::c_int && affRight != affLeft {
                            okToChngToIN = 0 as ::core::ffi::c_int;
                        } else {
                            (*pOrTerm).wtFlags = ((*pOrTerm).wtFlags
                                as ::core::ffi::c_int | TERM_OK) as u16_0;
                        }
                    }
                    i -= 1;
                    pOrTerm = pOrTerm.offset(1);
                }
                j_0 += 1;
            }
            if okToChngToIN != 0 {
                let mut pDup: *mut Expr = ::core::ptr::null_mut::<Expr>();
                let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                let mut pLeft_0: *mut Expr = ::core::ptr::null_mut::<Expr>();
                let mut pNew: *mut Expr = ::core::ptr::null_mut::<Expr>();
                i = (*pOrWc).nTerm - 1 as ::core::ffi::c_int;
                pOrTerm = (*pOrWc).a;
                while i >= 0 as ::core::ffi::c_int {
                    if !((*pOrTerm).wtFlags as ::core::ffi::c_int & TERM_OK
                        == 0 as ::core::ffi::c_int)
                    {
                        pDup = sqlite3ExprDup(
                            db,
                            (*(*pOrTerm).pExpr).pRight,
                            0 as ::core::ffi::c_int,
                        );
                        pList = sqlite3ExprListAppend((*pWInfo).pParse, pList, pDup);
                        pLeft_0 = (*(*pOrTerm).pExpr).pLeft;
                    }
                    i -= 1;
                    pOrTerm = pOrTerm.offset(1);
                }
                pDup = sqlite3ExprDup(db, pLeft_0, 0 as ::core::ffi::c_int);
                pNew = sqlite3PExpr(
                    pParse,
                    TK_IN,
                    pDup,
                    ::core::ptr::null_mut::<Expr>(),
                );
                if !pNew.is_null() {
                    let mut idxNew: ::core::ffi::c_int = 0;
                    transferJoinMarkings(pNew, pExpr);
                    (*pNew).x.pList = pList;
                    idxNew = whereClauseInsert(
                        pWC,
                        pNew,
                        (TERM_VIRTUAL | TERM_DYNAMIC) as u16_0,
                    );
                    exprAnalyze(pSrc, pWC, idxNew);
                    markTermAsChild(pWC, idxNew, idxTerm);
                } else {
                    sqlite3ExprListDelete(db, pList);
                }
            }
        }
    }
}
unsafe extern "C" fn termIsEquivalence(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pSrc: *mut SrcList,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aff1: ::core::ffi::c_char = 0;
        let mut aff2: ::core::ffi::c_char = 0;
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        if !((*(*pParse).db).dbOptFlags as ::core::ffi::c_uint
            & 0x80 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint)
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pExpr).op as ::core::ffi::c_int != TK_EQ
            && (*pExpr).op as ::core::ffi::c_int != TK_IS
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pExpr).op as ::core::ffi::c_int == TK_IS
            && (*pSrc).nSrc >= 2 as ::core::ffi::c_int
            && (*(&raw mut (*pSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .fg
                .jointype as ::core::ffi::c_int & JT_LTORJ != 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        aff1 = sqlite3ExprAffinity((*pExpr).pLeft);
        aff2 = sqlite3ExprAffinity((*pExpr).pRight);
        if aff1 as ::core::ffi::c_int != aff2 as ::core::ffi::c_int
            && (!(aff1 as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC)
                || !(aff2 as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC))
        {
            return 0 as ::core::ffi::c_int;
        }
        pColl = sqlite3ExprCompareCollSeq(pParse, pExpr);
        if sqlite3IsBinary(pColl) == 0
            && sqlite3ExprCollSeqMatch(pParse, (*pExpr).pLeft, (*pExpr).pRight) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn exprSelectUsage(
    mut pMaskSet: *mut WhereMaskSet,
    mut pS: *mut Select,
) -> Bitmask {
    unsafe {
        let mut mask: Bitmask = 0 as Bitmask;
        while !pS.is_null() {
            let mut pSrc: *mut SrcList = (*pS).pSrc;
            mask
                |= sqlite3WhereExprListUsage(pMaskSet, (*pS).pEList)
                    as ::core::ffi::c_ulonglong;
            mask
                |= sqlite3WhereExprListUsage(pMaskSet, (*pS).pGroupBy)
                    as ::core::ffi::c_ulonglong;
            mask
                |= sqlite3WhereExprListUsage(pMaskSet, (*pS).pOrderBy)
                    as ::core::ffi::c_ulonglong;
            mask
                |= sqlite3WhereExprUsage(pMaskSet, (*pS).pWhere)
                    as ::core::ffi::c_ulonglong;
            mask
                |= sqlite3WhereExprUsage(pMaskSet, (*pS).pHaving)
                    as ::core::ffi::c_ulonglong;
            if !pSrc.is_null() {
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while i < (*pSrc).nSrc {
                    if (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize))
                        .fg
                        .isSubquery() != 0
                    {
                        mask
                            |= exprSelectUsage(
                                pMaskSet,
                                (*(*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize))
                                    .u4
                                    .pSubq)
                                    .pSelect,
                            ) as ::core::ffi::c_ulonglong;
                    }
                    if (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize))
                        .fg
                        .isUsing() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        mask
                            |= sqlite3WhereExprUsage(
                                pMaskSet,
                                (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize))
                                    .u3
                                    .pOn,
                            ) as ::core::ffi::c_ulonglong;
                    }
                    if (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize))
                        .fg
                        .isTabFunc() != 0
                    {
                        mask
                            |= sqlite3WhereExprListUsage(
                                pMaskSet,
                                (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize))
                                    .u1
                                    .pFuncArg,
                            ) as ::core::ffi::c_ulonglong;
                    }
                    i += 1;
                }
            }
            pS = (*pS).pPrior;
        }
        return mask;
    }
}
#[inline(never)]
unsafe extern "C" fn exprMightBeIndexed2(
    mut pFrom: *mut SrcList,
    mut aiCurCol: *mut ::core::ffi::c_int,
    mut pExpr: *mut Expr,
    mut j: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut i: ::core::ffi::c_int = 0;
        let mut iCur: ::core::ffi::c_int = 0;
        loop {
            iCur = (*(&raw mut (*pFrom).a as *mut SrcItem).offset(j as isize)).iCursor;
            pIdx = (*(*(&raw mut (*pFrom).a as *mut SrcItem).offset(j as isize)).pSTab)
                .pIndex;
            while !pIdx.is_null() {
                if !(*pIdx).aColExpr.is_null() {
                    i = 0 as ::core::ffi::c_int;
                    while i < (*pIdx).nKeyCol as ::core::ffi::c_int {
                        if !(*(*pIdx).aiColumn.offset(i as isize) as ::core::ffi::c_int
                            != XN_EXPR)
                        {
                            if sqlite3ExprCompareSkip(
                                pExpr,
                                (*(&raw mut (*(*pIdx).aColExpr).a as *mut ExprList_item)
                                    .offset(i as isize))
                                    .pExpr,
                                iCur,
                            ) == 0 as ::core::ffi::c_int
                                && sqlite3ExprIsConstant(
                                    ::core::ptr::null_mut::<Parse>(),
                                    (*(&raw mut (*(*pIdx).aColExpr).a as *mut ExprList_item)
                                        .offset(i as isize))
                                        .pExpr,
                                ) == 0
                            {
                                *aiCurCol.offset(0 as ::core::ffi::c_int as isize) = iCur;
                                *aiCurCol.offset(1 as ::core::ffi::c_int as isize) = XN_EXPR;
                                return 1 as ::core::ffi::c_int;
                            }
                        }
                        i += 1;
                    }
                }
                pIdx = (*pIdx).pNext;
            }
            j += 1;
            if !(j < (*pFrom).nSrc) {
                break;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn exprMightBeIndexed(
    mut pFrom: *mut SrcList,
    mut aiCurCol: *mut ::core::ffi::c_int,
    mut pExpr: *mut Expr,
    mut op: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if (*pExpr).op as ::core::ffi::c_int == TK_VECTOR
            && (op >= TK_GT && op <= 58 as ::core::ffi::c_int)
        {
            pExpr = (*(&raw mut (*(*pExpr).x.pList).a as *mut ExprList_item)
                .offset(0 as ::core::ffi::c_int as isize))
                .pExpr;
        }
        if (*pExpr).op as ::core::ffi::c_int == TK_COLUMN {
            *aiCurCol.offset(0 as ::core::ffi::c_int as isize) = (*pExpr).iTable;
            *aiCurCol.offset(1 as ::core::ffi::c_int as isize) = (*pExpr).iColumn
                as ::core::ffi::c_int;
            return 1 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pFrom).nSrc {
            let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
            pIdx = (*(*(&raw mut (*pFrom).a as *mut SrcItem).offset(i as isize)).pSTab)
                .pIndex;
            while !pIdx.is_null() {
                if !(*pIdx).aColExpr.is_null() {
                    return exprMightBeIndexed2(pFrom, aiCurCol, pExpr, i);
                }
                pIdx = (*pIdx).pNext;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn exprAnalyze(
    mut pSrc: *mut SrcList,
    mut pWC: *mut WhereClause,
    mut idxTerm: ::core::ffi::c_int,
) {
    unsafe {
        let mut pWInfo: *mut WhereInfo = (*pWC).pWInfo;
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut pMaskSet: *mut WhereMaskSet = ::core::ptr::null_mut::<WhereMaskSet>();
        let mut pExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut prereqLeft: Bitmask = 0;
        let mut prereqAll: Bitmask = 0;
        let mut extraRight: Bitmask = 0 as Bitmask;
        let mut pStr1: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut isComplete: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut noCase: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut op: ::core::ffi::c_int = 0;
        let mut pParse: *mut Parse = (*pWInfo).pParse;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut eOp2: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
        let mut nLeft: ::core::ffi::c_int = 0;
        if (*db).mallocFailed != 0 {
            return;
        }
        pTerm = (*pWC).a.offset(idxTerm as isize) as *mut WhereTerm;
        pMaskSet = &raw mut (*pWInfo).sMaskSet;
        pExpr = (*pTerm).pExpr;
        (*pMaskSet).bVarSelect = 0 as ::core::ffi::c_int;
        prereqLeft = sqlite3WhereExprUsage(pMaskSet, (*pExpr).pLeft);
        op = (*pExpr).op as ::core::ffi::c_int;
        if op == TK_IN {
            if sqlite3ExprCheckIN(pParse, pExpr) != 0 {
                return;
            }
            if (*pExpr).flags as ::core::ffi::c_uint
                & EP_xIsSelect as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            {
                (*pTerm).prereqRight = exprSelectUsage(pMaskSet, (*pExpr).x.pSelect);
            } else {
                (*pTerm).prereqRight = sqlite3WhereExprListUsage(
                    pMaskSet,
                    (*pExpr).x.pList,
                );
            }
            prereqAll = prereqLeft | (*pTerm).prereqRight;
        } else {
            (*pTerm).prereqRight = sqlite3WhereExprUsage(pMaskSet, (*pExpr).pRight);
            if (*pExpr).pLeft.is_null()
                || (*pExpr).flags
                    & (0x1000 as ::core::ffi::c_int | 0x40000 as ::core::ffi::c_int)
                        as u32_0 != 0 as ::core::ffi::c_uint
                || !(*pExpr).x.pList.is_null()
            {
                prereqAll = sqlite3WhereExprUsageNN(pMaskSet, pExpr);
            } else {
                prereqAll = prereqLeft | (*pTerm).prereqRight;
            }
        }
        if (*pMaskSet).bVarSelect != 0 {
            (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int | TERM_VARSELECT)
                as u16_0;
        }
        if (*pExpr).flags
            & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int) as u32_0
            != 0 as ::core::ffi::c_uint
        {
            let mut x: Bitmask = sqlite3WhereGetMask(pMaskSet, (*pExpr).w.iJoin);
            if (*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
            {
                prereqAll |= x as ::core::ffi::c_ulonglong;
                extraRight = (x as ::core::ffi::c_ulonglong)
                    .wrapping_sub(1 as ::core::ffi::c_ulonglong) as Bitmask;
            } else if prereqAll >> 1 as ::core::ffi::c_int >= x {
                (*pExpr).flags
                    &= !(0x2 as ::core::ffi::c_int as u32_0) as ::core::ffi::c_uint;
            }
        }
        (*pTerm).prereqAll = prereqAll;
        (*pTerm).leftCursor = -1 as ::core::ffi::c_int;
        (*pTerm).iParent = -1 as ::core::ffi::c_int;
        (*pTerm).eOperator = 0 as u16_0;
        if allowedOp(op) != 0 {
            let mut aiCurCol: [::core::ffi::c_int; 2] = [0; 2];
            let mut pLeft: *mut Expr = sqlite3ExprSkipCollate((*pExpr).pLeft);
            let mut pRight: *mut Expr = sqlite3ExprSkipCollate((*pExpr).pRight);
            let mut opMask: u16_0 = (if (*pTerm).prereqRight & prereqLeft
                == 0 as ::core::ffi::c_ulonglong
            {
                WO_ALL
            } else {
                WO_EQUIV
            }) as u16_0;
            if (*pTerm).u.x.iField > 0 as ::core::ffi::c_int {
                pLeft = (*(&raw mut (*(*pLeft).x.pList).a as *mut ExprList_item)
                    .offset(((*pTerm).u.x.iField - 1 as ::core::ffi::c_int) as isize))
                    .pExpr;
            }
            if exprMightBeIndexed(
                pSrc,
                &raw mut aiCurCol as *mut ::core::ffi::c_int,
                pLeft,
                op,
            ) != 0
            {
                (*pTerm).leftCursor = aiCurCol[0 as ::core::ffi::c_int as usize];
                (*pTerm).u.x.leftColumn = aiCurCol[1 as ::core::ffi::c_int as usize];
                (*pTerm).eOperator = (operatorMask(op) as ::core::ffi::c_int
                    & opMask as ::core::ffi::c_int) as u16_0;
            }
            if op == TK_IS {
                (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int | TERM_IS)
                    as u16_0;
            }
            if !pRight.is_null()
                && exprMightBeIndexed(
                    pSrc,
                    &raw mut aiCurCol as *mut ::core::ffi::c_int,
                    pRight,
                    op,
                ) != 0
                && !((*pRight).flags & 0x20 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint)
            {
                let mut pNew: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
                let mut pDup: *mut Expr = ::core::ptr::null_mut::<Expr>();
                let mut eExtraOp: u16_0 = 0 as u16_0;
                if (*pTerm).leftCursor >= 0 as ::core::ffi::c_int {
                    let mut idxNew: ::core::ffi::c_int = 0;
                    pDup = sqlite3ExprDup(db, pExpr, 0 as ::core::ffi::c_int);
                    if (*db).mallocFailed != 0 {
                        sqlite3ExprDelete(db, pDup);
                        return;
                    }
                    idxNew = whereClauseInsert(
                        pWC,
                        pDup,
                        (TERM_VIRTUAL | TERM_DYNAMIC) as u16_0,
                    );
                    if idxNew == 0 as ::core::ffi::c_int {
                        return;
                    }
                    pNew = (*pWC).a.offset(idxNew as isize) as *mut WhereTerm;
                    markTermAsChild(pWC, idxNew, idxTerm);
                    if op == TK_IS {
                        (*pNew).wtFlags = ((*pNew).wtFlags as ::core::ffi::c_int
                            | TERM_IS) as u16_0;
                    }
                    pTerm = (*pWC).a.offset(idxTerm as isize) as *mut WhereTerm;
                    (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int
                        | TERM_COPIED) as u16_0;
                    if termIsEquivalence(pParse, pDup, (*pWInfo).pTabList) != 0 {
                        (*pTerm).eOperator = ((*pTerm).eOperator as ::core::ffi::c_int
                            | WO_EQUIV) as u16_0;
                        eExtraOp = WO_EQUIV as u16_0;
                    }
                } else {
                    pDup = pExpr;
                    pNew = pTerm;
                }
                (*pNew).wtFlags = ((*pNew).wtFlags as ::core::ffi::c_int
                    | exprCommute(pParse, pDup) as ::core::ffi::c_int) as u16_0;
                (*pNew).leftCursor = aiCurCol[0 as ::core::ffi::c_int as usize];
                (*pNew).u.x.leftColumn = aiCurCol[1 as ::core::ffi::c_int as usize];
                (*pNew).prereqRight = prereqLeft | extraRight;
                (*pNew).prereqAll = prereqAll;
                (*pNew).eOperator = (operatorMask((*pDup).op as ::core::ffi::c_int)
                    as ::core::ffi::c_int + eExtraOp as ::core::ffi::c_int
                    & opMask as ::core::ffi::c_int) as u16_0;
            } else if op == TK_ISNULL
                && !((*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint)
                && 0 as ::core::ffi::c_int == sqlite3ExprCanBeNull(pLeft)
            {
                (*pExpr).op = TK_TRUEFALSE as u8_0;
                (*pExpr).u.zToken = b"false\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                (*pExpr).flags
                    |= 0x20000000 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint;
                (*pTerm).prereqAll = 0 as Bitmask;
                (*pTerm).eOperator = 0 as u16_0;
            }
        } else if (*pExpr).op as ::core::ffi::c_int == TK_BETWEEN
            && (*pWC).op as ::core::ffi::c_int == TK_AND
        {
            let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            let mut i: ::core::ffi::c_int = 0;
            static mut ops: [u8_0; 2] = [TK_GE as u8_0, TK_LE as u8_0];
            pList = (*pExpr).x.pList;
            i = 0 as ::core::ffi::c_int;
            while i < 2 as ::core::ffi::c_int {
                let mut pNewExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
                let mut idxNew_0: ::core::ffi::c_int = 0;
                pNewExpr = sqlite3PExpr(
                    pParse,
                    ops[i as usize] as ::core::ffi::c_int,
                    sqlite3ExprDup(db, (*pExpr).pLeft, 0 as ::core::ffi::c_int),
                    sqlite3ExprDup(
                        db,
                        (*(&raw mut (*pList).a as *mut ExprList_item).offset(i as isize))
                            .pExpr,
                        0 as ::core::ffi::c_int,
                    ),
                );
                transferJoinMarkings(pNewExpr, pExpr);
                idxNew_0 = whereClauseInsert(
                    pWC,
                    pNewExpr,
                    (TERM_VIRTUAL | TERM_DYNAMIC) as u16_0,
                );
                exprAnalyze(pSrc, pWC, idxNew_0);
                pTerm = (*pWC).a.offset(idxTerm as isize) as *mut WhereTerm;
                markTermAsChild(pWC, idxNew_0, idxTerm);
                i += 1;
            }
        } else if (*pExpr).op as ::core::ffi::c_int == TK_OR {
            exprAnalyzeOrTerm(pSrc, pWC, idxTerm);
            pTerm = (*pWC).a.offset(idxTerm as isize) as *mut WhereTerm;
        } else if (*pExpr).op as ::core::ffi::c_int == TK_NOTNULL {
            if (*(*pExpr).pLeft).op as ::core::ffi::c_int == TK_COLUMN
                && (*(*pExpr).pLeft).iColumn as ::core::ffi::c_int
                    >= 0 as ::core::ffi::c_int
                && !((*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint)
            {
                let mut pNewExpr_0: *mut Expr = ::core::ptr::null_mut::<Expr>();
                let mut pLeft_0: *mut Expr = (*pExpr).pLeft;
                let mut idxNew_1: ::core::ffi::c_int = 0;
                let mut pNewTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
                pNewExpr_0 = sqlite3PExpr(
                    pParse,
                    TK_GT,
                    sqlite3ExprDup(db, pLeft_0, 0 as ::core::ffi::c_int),
                    sqlite3ExprAlloc(
                        db,
                        TK_NULL,
                        ::core::ptr::null::<Token>(),
                        0 as ::core::ffi::c_int,
                    ),
                );
                idxNew_1 = whereClauseInsert(
                    pWC,
                    pNewExpr_0,
                    (TERM_VIRTUAL | TERM_DYNAMIC | TERM_VNULL) as u16_0,
                );
                if idxNew_1 != 0 {
                    pNewTerm = (*pWC).a.offset(idxNew_1 as isize) as *mut WhereTerm;
                    (*pNewTerm).prereqRight = 0 as Bitmask;
                    (*pNewTerm).leftCursor = (*pLeft_0).iTable;
                    (*pNewTerm).u.x.leftColumn = (*pLeft_0).iColumn
                        as ::core::ffi::c_int;
                    (*pNewTerm).eOperator = WO_GT as u16_0;
                    markTermAsChild(pWC, idxNew_1, idxTerm);
                    pTerm = (*pWC).a.offset(idxTerm as isize) as *mut WhereTerm;
                    (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int
                        | TERM_COPIED) as u16_0;
                    (*pNewTerm).prereqAll = (*pTerm).prereqAll;
                }
            }
        } else if (*pExpr).op as ::core::ffi::c_int == TK_FUNCTION
            && (*pWC).op as ::core::ffi::c_int == TK_AND
            && isLikeOrGlob(
                pParse,
                pExpr,
                &raw mut pStr1,
                &raw mut isComplete,
                &raw mut noCase,
            ) != 0
        {
            let mut pLeft_1: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pStr2: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pNewExpr1: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pNewExpr2: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut idxNew1: ::core::ffi::c_int = 0;
            let mut idxNew2: ::core::ffi::c_int = 0;
            let mut zCollSeqName: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let wtFlags: u16_0 = (TERM_LIKEOPT | TERM_VIRTUAL | TERM_DYNAMIC) as u16_0;
            pLeft_1 = (*(&raw mut (*(*pExpr).x.pList).a as *mut ExprList_item)
                .offset(1 as ::core::ffi::c_int as isize))
                .pExpr;
            pStr2 = sqlite3ExprDup(db, pStr1, 0 as ::core::ffi::c_int);
            if noCase != 0 && (*(*pParse).db).mallocFailed == 0 {
                let mut i_0: ::core::ffi::c_int = 0;
                let mut c: ::core::ffi::c_char = 0;
                (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int | TERM_LIKE)
                    as u16_0;
                i_0 = 0 as ::core::ffi::c_int;
                loop {
                    c = *(*pStr1).u.zToken.offset(i_0 as isize);
                    if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                        break;
                    }
                    *(*pStr1).u.zToken.offset(i_0 as isize) = (c as ::core::ffi::c_int
                        & !(*(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(c as ::core::ffi::c_uchar as isize)
                            as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                    *(*pStr2).u.zToken.offset(i_0 as isize) = *(&raw const sqlite3UpperToLower
                        as *const ::core::ffi::c_uchar)
                        .offset(c as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_char;
                    i_0 += 1;
                }
            }
            if (*db).mallocFailed == 0 {
                let mut pC: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                pC = (*pStr2)
                    .u
                    .zToken
                    .offset(
                        ((sqlite3Strlen30
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int)((*pStr2).u.zToken)
                            - 1 as ::core::ffi::c_int) as isize,
                    ) as *mut ::core::ffi::c_char as *mut u8_0;
                if noCase != 0 {
                    if *pC as ::core::ffi::c_int == 'A' as i32 - 1 as ::core::ffi::c_int
                    {
                        isComplete = 0 as ::core::ffi::c_int;
                    }
                    *pC = *(&raw const sqlite3UpperToLower
                        as *const ::core::ffi::c_uchar)
                        .offset(*pC as isize) as u8_0;
                }
                while *pC as ::core::ffi::c_int == 0xbf as ::core::ffi::c_int
                    && pC > (*pStr2).u.zToken as *mut u8_0
                {
                    *pC = 0x80 as u8_0;
                    pC = pC.offset(-1);
                }
                *pC = (*pC).wrapping_add(1);
            }
            zCollSeqName = if noCase != 0 {
                b"NOCASE\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                &raw const sqlite3StrBINARY as *const ::core::ffi::c_char
            };
            pNewExpr1 = sqlite3ExprDup(db, pLeft_1, 0 as ::core::ffi::c_int);
            pNewExpr1 = sqlite3PExpr(
                pParse,
                TK_GE,
                sqlite3ExprAddCollateString(pParse, pNewExpr1, zCollSeqName),
                pStr1,
            );
            transferJoinMarkings(pNewExpr1, pExpr);
            idxNew1 = whereClauseInsert(pWC, pNewExpr1, wtFlags);
            pNewExpr2 = sqlite3ExprDup(db, pLeft_1, 0 as ::core::ffi::c_int);
            pNewExpr2 = sqlite3PExpr(
                pParse,
                TK_LT,
                sqlite3ExprAddCollateString(pParse, pNewExpr2, zCollSeqName),
                pStr2,
            );
            transferJoinMarkings(pNewExpr2, pExpr);
            idxNew2 = whereClauseInsert(pWC, pNewExpr2, wtFlags);
            exprAnalyze(pSrc, pWC, idxNew1);
            exprAnalyze(pSrc, pWC, idxNew2);
            pTerm = (*pWC).a.offset(idxTerm as isize) as *mut WhereTerm;
            if isComplete != 0 {
                markTermAsChild(pWC, idxNew1, idxTerm);
                markTermAsChild(pWC, idxNew2, idxTerm);
            }
        }
        if ((*pExpr).op as ::core::ffi::c_int == TK_EQ
            || (*pExpr).op as ::core::ffi::c_int == TK_IS)
            && {
                nLeft = sqlite3ExprVectorSize((*pExpr).pLeft);
                nLeft > 1 as ::core::ffi::c_int
            } && sqlite3ExprVectorSize((*pExpr).pRight) == nLeft
            && ((*(*pExpr).pLeft).flags as ::core::ffi::c_uint
                & EP_xIsSelect as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                || (*(*pExpr).pRight).flags as ::core::ffi::c_uint
                    & EP_xIsSelect as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint)
            && (*pWC).op as ::core::ffi::c_int == TK_AND
        {
            let mut i_1: ::core::ffi::c_int = 0;
            i_1 = 0 as ::core::ffi::c_int;
            while i_1 < nLeft {
                let mut idxNew_2: ::core::ffi::c_int = 0;
                let mut pNew_0: *mut Expr = ::core::ptr::null_mut::<Expr>();
                let mut pLeft_2: *mut Expr = sqlite3ExprForVectorField(
                    pParse,
                    (*pExpr).pLeft,
                    i_1,
                    nLeft,
                );
                let mut pRight_0: *mut Expr = sqlite3ExprForVectorField(
                    pParse,
                    (*pExpr).pRight,
                    i_1,
                    nLeft,
                );
                pNew_0 = sqlite3PExpr(
                    pParse,
                    (*pExpr).op as ::core::ffi::c_int,
                    pLeft_2,
                    pRight_0,
                );
                transferJoinMarkings(pNew_0, pExpr);
                idxNew_2 = whereClauseInsert(
                    pWC,
                    pNew_0,
                    (TERM_DYNAMIC | TERM_SLICE) as u16_0,
                );
                exprAnalyze(pSrc, pWC, idxNew_2);
                i_1 += 1;
            }
            pTerm = (*pWC).a.offset(idxTerm as isize) as *mut WhereTerm;
            (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int
                | (TERM_CODED | TERM_VIRTUAL)) as u16_0;
            (*pTerm).eOperator = WO_ROWVAL as u16_0;
        } else if (*pExpr).op as ::core::ffi::c_int == TK_IN
            && (*pTerm).u.x.iField == 0 as ::core::ffi::c_int
            && (*(*pExpr).pLeft).op as ::core::ffi::c_int == TK_VECTOR
            && (*pExpr).flags as ::core::ffi::c_uint & 0x1000 as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            && ((*(*pExpr).x.pSelect).pPrior.is_null()
                || (*(*pExpr).x.pSelect).selFlags as ::core::ffi::c_uint
                    & SF_Values as ::core::ffi::c_uint != 0)
            && (*(*pExpr).x.pSelect).pWin.is_null()
            && (*pWC).op as ::core::ffi::c_int == TK_AND
        {
            let mut i_2: ::core::ffi::c_int = 0;
            i_2 = 0 as ::core::ffi::c_int;
            while i_2 < sqlite3ExprVectorSize((*pExpr).pLeft) {
                let mut idxNew_3: ::core::ffi::c_int = 0;
                idxNew_3 = whereClauseInsert(
                    pWC,
                    pExpr,
                    (TERM_VIRTUAL | TERM_SLICE) as u16_0,
                );
                (*(*pWC).a.offset(idxNew_3 as isize)).u.x.iField = i_2
                    + 1 as ::core::ffi::c_int;
                exprAnalyze(pSrc, pWC, idxNew_3);
                markTermAsChild(pWC, idxNew_3, idxTerm);
                i_2 += 1;
            }
        } else if (*pWC).op as ::core::ffi::c_int == TK_AND {
            let mut pRight_1: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pLeft_3: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut res: ::core::ffi::c_int = isAuxiliaryVtabOperator(
                db,
                pExpr,
                &raw mut eOp2,
                &raw mut pLeft_3,
                &raw mut pRight_1,
            );
            loop {
                let c2rust_fresh1 = res;
                res = res - 1;
                if !(c2rust_fresh1 > 0 as ::core::ffi::c_int) {
                    break;
                }
                let mut idxNew_4: ::core::ffi::c_int = 0;
                let mut pNewTerm_0: *mut WhereTerm = ::core::ptr::null_mut::<
                    WhereTerm,
                >();
                let mut prereqColumn: Bitmask = 0;
                let mut prereqExpr: Bitmask = 0;
                prereqExpr = sqlite3WhereExprUsage(pMaskSet, pRight_1);
                prereqColumn = sqlite3WhereExprUsage(pMaskSet, pLeft_3);
                if prereqExpr & prereqColumn == 0 as ::core::ffi::c_ulonglong {
                    let mut pNewExpr_1: *mut Expr = ::core::ptr::null_mut::<Expr>();
                    pNewExpr_1 = sqlite3PExpr(
                        pParse,
                        TK_MATCH,
                        ::core::ptr::null_mut::<Expr>(),
                        sqlite3ExprDup(db, pRight_1, 0 as ::core::ffi::c_int),
                    );
                    if (*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint && !pNewExpr_1.is_null()
                    {
                        (*pNewExpr_1).flags
                            |= 0x1 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint;
                        (*pNewExpr_1).w.iJoin = (*pExpr).w.iJoin;
                    }
                    idxNew_4 = whereClauseInsert(
                        pWC,
                        pNewExpr_1,
                        (TERM_VIRTUAL | TERM_DYNAMIC) as u16_0,
                    );
                    pNewTerm_0 = (*pWC).a.offset(idxNew_4 as isize) as *mut WhereTerm;
                    (*pNewTerm_0).prereqRight = prereqExpr | extraRight;
                    (*pNewTerm_0).leftCursor = (*pLeft_3).iTable;
                    (*pNewTerm_0).u.x.leftColumn = (*pLeft_3).iColumn
                        as ::core::ffi::c_int;
                    (*pNewTerm_0).eOperator = WO_AUX as u16_0;
                    (*pNewTerm_0).eMatchOp = eOp2 as u8_0;
                    markTermAsChild(pWC, idxNew_4, idxTerm);
                    pTerm = (*pWC).a.offset(idxTerm as isize) as *mut WhereTerm;
                    (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int
                        | TERM_COPIED) as u16_0;
                    (*pNewTerm_0).prereqAll = (*pTerm).prereqAll;
                }
                let mut t: *mut Expr = pLeft_3;
                pLeft_3 = pRight_1;
                pRight_1 = t;
            }
        }
        pTerm = (*pWC).a.offset(idxTerm as isize) as *mut WhereTerm;
        (*pTerm).prereqRight |= extraRight as ::core::ffi::c_ulonglong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereSplit(
    mut pWC: *mut WhereClause,
    mut pExpr: *mut Expr,
    mut op: u8_0,
) {
    unsafe {
        let mut pE2: *mut Expr = sqlite3ExprSkipCollateAndLikely(pExpr);
        (*pWC).op = op;
        if pE2.is_null() {
            return;
        }
        if (*pE2).op as ::core::ffi::c_int != op as ::core::ffi::c_int {
            whereClauseInsert(pWC, pExpr, 0 as u16_0);
        } else {
            sqlite3WhereSplit(pWC, (*pE2).pLeft, op);
            sqlite3WhereSplit(pWC, (*pE2).pRight, op);
        };
    }
}
unsafe extern "C" fn whereAddLimitExpr(
    mut pWC: *mut WhereClause,
    mut iReg: ::core::ffi::c_int,
    mut pExpr: *mut Expr,
    mut iCsr: ::core::ffi::c_int,
    mut eMatchOp: ::core::ffi::c_int,
) {
    unsafe {
        let mut pParse: *mut Parse = (*(*pWC).pWInfo).pParse;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pNew: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut iVal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if sqlite3ExprIsInteger(pExpr, &raw mut iVal, pParse) != 0
            && iVal >= 0 as ::core::ffi::c_int
        {
            let mut pVal: *mut Expr = sqlite3Expr(
                db,
                TK_INTEGER,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            if pVal.is_null() {
                return;
            }
            (*pVal).flags |= 0x800 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint;
            (*pVal).u.iValue = iVal;
            pNew = sqlite3PExpr(pParse, TK_MATCH, ::core::ptr::null_mut::<Expr>(), pVal);
        } else {
            let mut pVal_0: *mut Expr = sqlite3Expr(
                db,
                TK_REGISTER,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            if pVal_0.is_null() {
                return;
            }
            (*pVal_0).iTable = iReg;
            pNew = sqlite3PExpr(
                pParse,
                TK_MATCH,
                ::core::ptr::null_mut::<Expr>(),
                pVal_0,
            );
        }
        if !pNew.is_null() {
            let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
            let mut idx: ::core::ffi::c_int = 0;
            idx = whereClauseInsert(pWC, pNew, (TERM_DYNAMIC | TERM_VIRTUAL) as u16_0);
            pTerm = (*pWC).a.offset(idx as isize) as *mut WhereTerm;
            (*pTerm).leftCursor = iCsr;
            (*pTerm).eOperator = WO_AUX as u16_0;
            (*pTerm).eMatchOp = eMatchOp as u8_0;
        }
    }
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sqlite3WhereAddLimit(
    mut pWC: *mut WhereClause,
    mut p: *mut Select,
) {
    unsafe {
        if (*p).pGroupBy.is_null()
            && (*p).selFlags as ::core::ffi::c_uint
                & (SF_Distinct | SF_Aggregate) as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
            && ((*(*p).pSrc).nSrc == 1 as ::core::ffi::c_int
                && (*(*(&raw mut (*(*p).pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .pSTab)
                    .eTabType as ::core::ffi::c_int == TABTYP_VTAB)
        {
            let mut pOrderBy: *mut ExprList = (*p).pOrderBy;
            let mut iCsr: ::core::ffi::c_int = (*(&raw mut (*(*p).pSrc).a
                as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .iCursor;
            let mut ii: ::core::ffi::c_int = 0;
            let mut c2rust_current_block_4: u64;
            ii = 0 as ::core::ffi::c_int;
            while ii < (*pWC).nTerm {
                if !((*(*pWC).a.offset(ii as isize)).wtFlags as ::core::ffi::c_int
                    & TERM_CODED != 0)
                {
                    if !((*(*pWC).a.offset(ii as isize)).nChild != 0) {
                        if !((*(*pWC).a.offset(ii as isize)).leftCursor == iCsr
                            && (*(*pWC).a.offset(ii as isize)).prereqRight
                                == 0 as ::core::ffi::c_ulonglong)
                        {
                            if (*(*pWC).a.offset(ii as isize)).iParent
                                >= 0 as ::core::ffi::c_int
                            {
                                let mut pParent: *mut WhereTerm = (*pWC)
                                    .a
                                    .offset((*(*pWC).a.offset(ii as isize)).iParent as isize)
                                    as *mut WhereTerm;
                                if (*pParent).leftCursor == iCsr
                                    && (*pParent).prereqRight == 0 as ::core::ffi::c_ulonglong
                                    && (*pParent).nChild as ::core::ffi::c_int
                                        == 1 as ::core::ffi::c_int
                                {
                                    c2rust_current_block_4 = 12517898123489920830;
                                } else {
                                    c2rust_current_block_4 = 5399440093318478209;
                                }
                            } else {
                                c2rust_current_block_4 = 5399440093318478209;
                            }
                            match c2rust_current_block_4 {
                                12517898123489920830 => {}
                                _ => return

                            }
                        }
                    }
                }
                ii += 1;
            }
            if !pOrderBy.is_null() {
                ii = 0 as ::core::ffi::c_int;
                while ii < (*pOrderBy).nExpr {
                    let mut pExpr: *mut Expr = (*(&raw mut (*pOrderBy).a
                        as *mut ExprList_item)
                        .offset(ii as isize))
                        .pExpr;
                    if (*pExpr).op as ::core::ffi::c_int != TK_COLUMN {
                        return;
                    }
                    if (*pExpr).iTable != iCsr {
                        return;
                    }
                    if (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                        .offset(ii as isize))
                        .fg
                        .sortFlags as ::core::ffi::c_int & KEYINFO_ORDER_BIGNULL != 0
                    {
                        return;
                    }
                    ii += 1;
                }
            }
            if (*p).iOffset != 0 as ::core::ffi::c_int
                && (*p).selFlags as ::core::ffi::c_uint
                    & SF_Compound as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            {
                whereAddLimitExpr(
                    pWC,
                    (*p).iOffset,
                    (*(*p).pLimit).pRight,
                    iCsr,
                    SQLITE_INDEX_CONSTRAINT_OFFSET,
                );
            }
            if (*p).iOffset == 0 as ::core::ffi::c_int
                || (*p).selFlags as ::core::ffi::c_uint
                    & SF_Compound as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            {
                whereAddLimitExpr(
                    pWC,
                    (*p).iLimit,
                    (*(*p).pLimit).pLeft,
                    iCsr,
                    SQLITE_INDEX_CONSTRAINT_LIMIT,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereClauseInit(
    mut pWC: *mut WhereClause,
    mut pWInfo: *mut WhereInfo,
) {
    unsafe {
        (*pWC).pWInfo = pWInfo;
        (*pWC).hasOr = 0 as u8_0;
        (*pWC).pOuter = ::core::ptr::null_mut::<WhereClause>();
        (*pWC).nTerm = 0 as ::core::ffi::c_int;
        (*pWC).nBase = 0 as ::core::ffi::c_int;
        (*pWC).nSlot = (::core::mem::size_of::<[WhereTerm; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<WhereTerm>() as usize)
            as ::core::ffi::c_int;
        (*pWC).a = &raw mut (*pWC).aStatic as *mut WhereTerm;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereClauseClear(mut pWC: *mut WhereClause) {
    unsafe {
        let mut db: *mut sqlite3 = (*(*(*pWC).pWInfo).pParse).db;
        if (*pWC).nTerm > 0 as ::core::ffi::c_int {
            let mut a: *mut WhereTerm = (*pWC).a;
            let mut aLast: *mut WhereTerm = (*pWC)
                .a
                .offset(((*pWC).nTerm - 1 as ::core::ffi::c_int) as isize)
                as *mut WhereTerm;
            loop {
                if (*a).wtFlags as ::core::ffi::c_int & TERM_DYNAMIC != 0 {
                    sqlite3ExprDelete(db, (*a).pExpr);
                }
                if (*a).wtFlags as ::core::ffi::c_int & (TERM_ORINFO | TERM_ANDINFO) != 0
                {
                    if (*a).wtFlags as ::core::ffi::c_int & TERM_ORINFO != 0 {
                        whereOrInfoDelete(db, (*a).u.pOrInfo);
                    } else {
                        whereAndInfoDelete(db, (*a).u.pAndInfo);
                    }
                }
                if a == aLast {
                    break;
                }
                a = a.offset(1);
            }
        }
    }
}
#[inline(never)]
unsafe extern "C" fn sqlite3WhereExprUsageFull(
    mut pMaskSet: *mut WhereMaskSet,
    mut p: *mut Expr,
) -> Bitmask {
    unsafe {
        let mut mask: Bitmask = 0;
        mask = (if (*p).op as ::core::ffi::c_int == TK_IF_NULL_ROW {
            sqlite3WhereGetMask(pMaskSet, (*p).iTable) as ::core::ffi::c_ulonglong
        } else {
            0 as ::core::ffi::c_ulonglong
        }) as Bitmask;
        if !(*p).pLeft.is_null() {
            mask
                |= sqlite3WhereExprUsageNN(pMaskSet, (*p).pLeft)
                    as ::core::ffi::c_ulonglong;
        }
        if !(*p).pRight.is_null() {
            mask
                |= sqlite3WhereExprUsageNN(pMaskSet, (*p).pRight)
                    as ::core::ffi::c_ulonglong;
        } else if (*p).flags as ::core::ffi::c_uint & EP_xIsSelect as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            if (*p).flags & 0x40 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
            {
                (*pMaskSet).bVarSelect = 1 as ::core::ffi::c_int;
            }
            mask
                |= exprSelectUsage(pMaskSet, (*p).x.pSelect) as ::core::ffi::c_ulonglong;
        } else if !(*p).x.pList.is_null() {
            mask
                |= sqlite3WhereExprListUsage(pMaskSet, (*p).x.pList)
                    as ::core::ffi::c_ulonglong;
        }
        if ((*p).op as ::core::ffi::c_int == TK_FUNCTION
            || (*p).op as ::core::ffi::c_int == TK_AGG_FUNCTION)
            && (*p).flags as ::core::ffi::c_uint & EP_WinFunc as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
        {
            mask
                |= sqlite3WhereExprListUsage(pMaskSet, (*(*p).y.pWin).pPartition)
                    as ::core::ffi::c_ulonglong;
            mask
                |= sqlite3WhereExprListUsage(pMaskSet, (*(*p).y.pWin).pOrderBy)
                    as ::core::ffi::c_ulonglong;
            mask
                |= sqlite3WhereExprUsage(pMaskSet, (*(*p).y.pWin).pFilter)
                    as ::core::ffi::c_ulonglong;
        }
        return mask;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereExprUsageNN(
    mut pMaskSet: *mut WhereMaskSet,
    mut p: *mut Expr,
) -> Bitmask {
    unsafe {
        if (*p).op as ::core::ffi::c_int == TK_COLUMN
            && !((*p).flags & 0x20 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint)
        {
            return sqlite3WhereGetMask(pMaskSet, (*p).iTable)
        } else if (*p).flags
            & (0x10000 as ::core::ffi::c_int | 0x800000 as ::core::ffi::c_int) as u32_0
            != 0 as ::core::ffi::c_uint
        {
            return 0 as Bitmask
        }
        return sqlite3WhereExprUsageFull(pMaskSet, p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereExprUsage(
    mut pMaskSet: *mut WhereMaskSet,
    mut p: *mut Expr,
) -> Bitmask {
    unsafe {
        return if !p.is_null() {
            sqlite3WhereExprUsageNN(pMaskSet, p)
        } else {
            0 as Bitmask
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereExprListUsage(
    mut pMaskSet: *mut WhereMaskSet,
    mut pList: *mut ExprList,
) -> Bitmask {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut mask: Bitmask = 0 as Bitmask;
        if !pList.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*pList).nExpr {
                mask
                    |= sqlite3WhereExprUsage(
                        pMaskSet,
                        (*(&raw mut (*pList).a as *mut ExprList_item).offset(i as isize))
                            .pExpr,
                    ) as ::core::ffi::c_ulonglong;
                i += 1;
            }
        }
        return mask;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereExprAnalyze(
    mut pTabList: *mut SrcList,
    mut pWC: *mut WhereClause,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = (*pWC).nTerm - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            exprAnalyze(pTabList, pWC, i);
            i -= 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereTabFuncArgs(
    mut pParse: *mut Parse,
    mut pItem: *mut SrcItem,
    mut pWC: *mut WhereClause,
) {
    unsafe {
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut j: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut pArgs: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pColRef: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pTerm: *mut Expr = ::core::ptr::null_mut::<Expr>();
        if (*pItem).fg.isTabFunc() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return;
        }
        pTab = (*pItem).pSTab;
        pArgs = (*pItem).u1.pFuncArg;
        if pArgs.is_null() {
            return;
        }
        k = 0 as ::core::ffi::c_int;
        j = k;
        while j < (*pArgs).nExpr {
            let mut pRhs: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut joinType: u32_0 = 0;
            while k < (*pTab).nCol as ::core::ffi::c_int
                && (*(*pTab).aCol.offset(k as isize)).colFlags as ::core::ffi::c_int
                    & COLFLAG_HIDDEN == 0 as ::core::ffi::c_int
            {
                k += 1;
            }
            if k >= (*pTab).nCol as ::core::ffi::c_int {
                sqlite3ErrorMsg(
                    pParse,
                    b"too many arguments on %s() - max %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*pTab).zName,
                    j,
                );
                return;
            }
            pColRef = sqlite3ExprAlloc(
                (*pParse).db,
                TK_COLUMN,
                ::core::ptr::null::<Token>(),
                0 as ::core::ffi::c_int,
            );
            if pColRef.is_null() {
                return;
            }
            (*pColRef).iTable = (*pItem).iCursor;
            let c2rust_fresh8 = k;
            k = k + 1;
            (*pColRef).iColumn = c2rust_fresh8 as ynVar;
            (*pColRef).y.pTab = pTab;
            (*pItem).colUsed |= sqlite3ExprColUsed(pColRef) as ::core::ffi::c_ulonglong;
            pRhs = sqlite3PExpr(
                pParse,
                TK_UPLUS,
                sqlite3ExprDup(
                    (*pParse).db,
                    (*(&raw mut (*pArgs).a as *mut ExprList_item).offset(j as isize))
                        .pExpr,
                    0 as ::core::ffi::c_int,
                ),
                ::core::ptr::null_mut::<Expr>(),
            );
            pTerm = sqlite3PExpr(pParse, TK_EQ, pColRef, pRhs);
            if (*pItem).fg.jointype as ::core::ffi::c_int & (JT_LEFT | JT_RIGHT) != 0 {
                joinType = EP_OuterON as u32_0;
            } else {
                joinType = EP_InnerON as u32_0;
            }
            sqlite3SetJoinExpr(pTerm, (*pItem).iCursor, joinType);
            whereClauseInsert(pWC, pTerm, TERM_DYNAMIC as u16_0);
            j += 1;
        }
    }
}
pub const TK_OR: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const TK_AND: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const TK_IS: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const TK_ISNOT: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const TK_MATCH: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
pub const TK_BETWEEN: ::core::ffi::c_int = 49 as ::core::ffi::c_int;
pub const TK_IN: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const TK_ISNULL: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const TK_NOTNULL: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const TK_NE: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const TK_EQ: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const TK_GT: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
pub const TK_LE: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const TK_LT: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
pub const TK_GE: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
pub const TK_STRING: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_INTEGER: ::core::ffi::c_int = 156 as ::core::ffi::c_int;
pub const TK_VARIABLE: ::core::ffi::c_int = 157 as ::core::ffi::c_int;
pub const TK_COLUMN: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
pub const TK_AGG_FUNCTION: ::core::ffi::c_int = 169 as ::core::ffi::c_int;
pub const TK_TRUEFALSE: ::core::ffi::c_int = 171 as ::core::ffi::c_int;
pub const TK_FUNCTION: ::core::ffi::c_int = 172 as ::core::ffi::c_int;
pub const TK_UPLUS: ::core::ffi::c_int = 173 as ::core::ffi::c_int;
pub const TK_REGISTER: ::core::ffi::c_int = 176 as ::core::ffi::c_int;
pub const TK_VECTOR: ::core::ffi::c_int = 177 as ::core::ffi::c_int;
pub const TK_IF_NULL_ROW: ::core::ffi::c_int = 179 as ::core::ffi::c_int;
pub const SQLITE_EnableQPSG: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
pub const COLFLAG_HIDDEN: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_AFF_BLOB: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const SQLITE_AFF_TEXT: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_BIGNULL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const XN_EXPR: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const EP_OuterON: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const EP_InnerON: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const EP_Commuted: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const EP_xIsSelect: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const EP_WinFunc: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const JT_LEFT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const JT_RIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const JT_LTORJ: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SF_Distinct: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SF_Aggregate: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SF_Compound: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SF_Values: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
