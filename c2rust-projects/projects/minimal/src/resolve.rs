use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type RenameToken;
    pub type Vdbe;
    pub type sqlite3_mutex;
    pub type CheckOnCtx;
    pub type CoveringIndexCheck;
    pub type RenameCtx;
    pub type WhereConst;
    pub type WindowRewrite;
    pub type IdxCover;
    pub type RefSrcList;
    pub type CCurHint;
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3WalkExpr(_: *mut Walker, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3WalkExprNN(_: *mut Walker, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3WalkExprList(_: *mut Walker, _: *mut ExprList) -> ::core::ffi::c_int;
    fn sqlite3WalkSelect(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3WindowUnlinkFromSelect(_: *mut Window);
    fn sqlite3WindowLink(pSel: *mut Select, pWin: *mut Window);
    fn sqlite3WindowUpdate(
        _: *mut Parse,
        _: *mut Window,
        _: *mut Window,
        _: *mut FuncDef,
    );
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
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
    fn sqlite3ExprOrderByAggregateError(_: *mut Parse, _: *mut Expr);
    fn sqlite3ExprFunctionUsable(_: *mut Parse, _: *const Expr, _: *const FuncDef);
    fn sqlite3ExprDelete(_: *mut sqlite3, _: *mut Expr);
    fn sqlite3ExprDeferredDelete(_: *mut Parse, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3ExprListAppend(
        _: *mut Parse,
        _: *mut ExprList,
        _: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListDelete(_: *mut sqlite3, _: *mut ExprList);
    fn sqlite3TableColumnToStorage(_: *mut Table, _: i16_0) -> i16_0;
    fn sqlite3IdListIndex(
        _: *mut IdList,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCompare(
        _: *const Parse,
        _: *const Expr,
        _: *const Expr,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ReferencesSrcList(
        _: *mut Parse,
        _: *mut Expr,
        _: *mut SrcList,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprIdToTrueFalse(_: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3ExprIsInteger(
        _: *const Expr,
        _: *mut ::core::ffi::c_int,
        _: *mut Parse,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCanBeNull(_: *const Expr) -> ::core::ffi::c_int;
    fn sqlite3IsRowid(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3ExprDup(
        _: *mut sqlite3,
        _: *const Expr,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3FindFunction(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: u8_0,
        _: u8_0,
    ) -> *mut FuncDef;
    fn sqlite3ColumnIndex(
        pTab: *mut Table,
        zCol: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3SrcItemColumnUsed(_: *mut SrcItem, _: ::core::ffi::c_int);
    fn sqlite3AuthRead(_: *mut Parse, _: *mut Expr, _: *mut Schema, _: *mut SrcList);
    fn sqlite3AuthCheck(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3AtoF(
        z: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_double,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprAddCollateString(
        _: *const Parse,
        _: *mut Expr,
        _: *const ::core::ffi::c_char,
    ) -> *mut Expr;
    fn sqlite3ExprSkipCollateAndLikely(_: *mut Expr) -> *mut Expr;
    fn sqlite3WritableSchema(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3SelectPrep(_: *mut Parse, _: *mut Select, _: *mut NameContext);
    fn sqlite3SelectWrongNumTermsError(pParse: *mut Parse, p: *mut Select);
    fn sqlite3RenameTokenRemap(
        _: *mut Parse,
        pTo: *const ::core::ffi::c_void,
        pFrom: *const ::core::ffi::c_void,
    );
    fn sqlite3RecordErrorOffsetOfExpr(_: *mut sqlite3, _: *const Expr);
    fn sqlite3ExprCheckHeight(
        _: *mut Parse,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprVectorSize(pExpr: *const Expr) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbFixer {
    pub pParse: *mut Parse,
    pub w: Walker,
    pub pSchema: *mut Schema,
    pub bTemp: u8_0,
    pub zDb: *const ::core::ffi::c_char,
    pub zType: *const ::core::ffi::c_char,
    pub pName: *const Token

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Walker {
    pub pParse: *mut Parse,
    pub xExprCallback: Option<
        unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
    >,
    pub xSelectCallback: Option<
        unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
    >,
    pub xSelectCallback2: Option<unsafe extern "C" fn(*mut Walker, *mut Select) -> ()>,
    pub walkerDepth: ::core::ffi::c_int,
    pub eCode: u16_0,
    pub mWFlags: u16_0,
    pub u: C2Rust_Unnamed_22

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_22 {
    pub pNC: *mut NameContext,
    pub n: ::core::ffi::c_int,
    pub iCur: ::core::ffi::c_int,
    pub pSrcList: *mut SrcList,
    pub pCCurHint: *mut CCurHint,
    pub pRefSrcList: *mut RefSrcList,
    pub aiCol: *mut ::core::ffi::c_int,
    pub pIdxCover: *mut IdxCover,
    pub pGroupBy: *mut ExprList,
    pub pSelect: *mut Select,
    pub pRewrite: *mut WindowRewrite,
    pub pConst: *mut WhereConst,
    pub pRename: *mut RenameCtx,
    pub pTab: *mut Table,
    pub pCovIdxCk: *mut CoveringIndexCheck,
    pub pSrcItem: *mut SrcItem,
    pub pFix: *mut DbFixer,
    pub aMem: *mut Mem,
    pub pCheckOnCtx: *mut CheckOnCtx

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameContext {
    pub pParse: *mut Parse,
    pub pSrcList: *mut SrcList,
    pub uNC: C2Rust_Unnamed_23,
    pub pNext: *mut NameContext,
    pub nRef: ::core::ffi::c_int,
    pub nNcErr: ::core::ffi::c_int,
    pub ncFlags: ::core::ffi::c_int,
    pub nNestedSelect: u32_0,
    pub pWinSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_23 {
    pub pEList: *mut ExprList,
    pub pAggInfo: *mut AggInfo,
    pub pUpsert: *mut Upsert,
    pub iBaseReg: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_24 {
    pub sSrc: SrcList,
    pub srcSpace: [u8_0; 80]

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_WARNING: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const SQLITE_DENY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_FUNCTION: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_COLUMN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_SUBTYPE: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const LEGACY_SCHEMA_TABLE: [::core::ffi::c_char; 14] = unsafe {
    ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"sqlite_master\0")
};
pub const LEGACY_TEMP_SCHEMA_TABLE: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<
        [u8; 19],
        [::core::ffi::c_char; 19],
    >(*b"sqlite_temp_master\0")
};
pub const PREFERRED_SCHEMA_TABLE: [::core::ffi::c_char; 14] = unsafe {
    ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"sqlite_schema\0")
};
pub const PREFERRED_TEMP_SCHEMA_TABLE: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<
        [u8; 19],
        [::core::ffi::c_char; 19],
    >(*b"sqlite_temp_schema\0")
};
pub const BMS: ::core::ffi::c_int = (::core::mem::size_of::<Bitmask>() as usize)
    .wrapping_mul(8 as usize) as ::core::ffi::c_int;
pub const ALLBITS: Bitmask = -1 as ::core::ffi::c_int as Bitmask;
pub const EXCLUDED_TABLE_NUMBER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn incrAggDepth(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int == TK_AGG_FUNCTION {
            (*pExpr).op2 = ((*pExpr).op2 as ::core::ffi::c_int + (*pWalker).u.n) as u8_0;
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn incrAggFunctionDepth(
    mut pExpr: *mut Expr,
    mut N: ::core::ffi::c_int,
) {
    unsafe {
        if N > 0 as ::core::ffi::c_int {
            let mut w: Walker = Walker {
                pParse: ::core::ptr::null_mut::<Parse>(),
                xExprCallback: None,
                xSelectCallback: None,
                xSelectCallback2: None,
                walkerDepth: 0,
                eCode: 0,
                mWFlags: 0,
                u: C2Rust_Unnamed_22 {
                    pNC: ::core::ptr::null_mut::<NameContext>()

                }

            };
            memset(
                &raw mut w as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Walker>() as size_t,
            );
            w.xExprCallback = Some(
                incrAggDepth
                    as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
                >;
            w.u.n = N;
            sqlite3WalkExpr(&raw mut w, pExpr);
        }
    }
}
unsafe extern "C" fn resolveAlias(
    mut pParse: *mut Parse,
    mut pEList: *mut ExprList,
    mut iCol: ::core::ffi::c_int,
    mut pExpr: *mut Expr,
    mut nSubquery: ::core::ffi::c_int,
) {
    unsafe {
        let mut pOrig: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pDup: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        pOrig = (*(&raw mut (*pEList).a as *mut ExprList_item).offset(iCol as isize))
            .pExpr;
        if !(*pExpr).pAggInfo.is_null() {
            return;
        }
        db = (*pParse).db;
        pDup = sqlite3ExprDup(db, pOrig, 0 as ::core::ffi::c_int);
        if (*db).mallocFailed != 0 {
            sqlite3ExprDelete(db, pDup);
            pDup = ::core::ptr::null_mut::<Expr>();
        } else {
            let mut temp: Expr = Expr {
                op: 0,
                affExpr: 0,
                op2: 0,
                flags: 0,
                u: C2Rust_Unnamed_8 {
                    zToken: ::core::ptr::null_mut::<::core::ffi::c_char>()

                },
                pLeft: ::core::ptr::null_mut::<Expr>(),
                pRight: ::core::ptr::null_mut::<Expr>(),
                x: C2Rust_Unnamed_7 {
                    pList: ::core::ptr::null_mut::<ExprList>()

                },
                nHeight: 0,
                iTable: 0,
                iColumn: 0,
                iAgg: 0,
                w: C2Rust_Unnamed_6 { iJoin: 0 },
                pAggInfo: ::core::ptr::null_mut::<AggInfo>(),
                y: C2Rust_Unnamed_0 {
                    pTab: ::core::ptr::null_mut::<Table>()

                }

            };
            incrAggFunctionDepth(pDup, nSubquery);
            if (*pExpr).op as ::core::ffi::c_int == TK_COLLATE {
                pDup = sqlite3ExprAddCollateString(pParse, pDup, (*pExpr).u.zToken);
            }
            memcpy(
                &raw mut temp as *mut ::core::ffi::c_void,
                pDup as *const ::core::ffi::c_void,
                ::core::mem::size_of::<Expr>() as size_t,
            );
            memcpy(
                pDup as *mut ::core::ffi::c_void,
                pExpr as *const ::core::ffi::c_void,
                ::core::mem::size_of::<Expr>() as size_t,
            );
            memcpy(
                pExpr as *mut ::core::ffi::c_void,
                &raw mut temp as *const ::core::ffi::c_void,
                ::core::mem::size_of::<Expr>() as size_t,
            );
            if (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
            {
                if !(*pExpr).y.pWin.is_null() {
                    (*(*pExpr).y.pWin).pOwner = pExpr;
                }
            }
            sqlite3ExprDeferredDelete(pParse, pDup);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MatchEName(
    mut pItem: *const ExprList_item,
    mut zCol: *const ::core::ffi::c_char,
    mut zTab: *const ::core::ffi::c_char,
    mut zDb: *const ::core::ffi::c_char,
    mut pbRowid: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut zSpan: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut eEName: ::core::ffi::c_int = (*pItem).fg.eEName() as ::core::ffi::c_int;
        if eEName != ENAME_TAB && (eEName != ENAME_ROWID || pbRowid.is_null()) {
            return 0 as ::core::ffi::c_int;
        }
        zSpan = (*pItem).zEName;
        n = 0 as ::core::ffi::c_int;
        while *zSpan.offset(n as isize) as ::core::ffi::c_int != 0
            && *zSpan.offset(n as isize) as ::core::ffi::c_int != '.' as i32
        {
            n += 1;
        }
        if !zDb.is_null()
            && (sqlite3_strnicmp(zSpan, zDb, n) != 0 as ::core::ffi::c_int
                || *zDb.offset(n as isize) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
        {
            return 0 as ::core::ffi::c_int;
        }
        zSpan = zSpan.offset((n + 1 as ::core::ffi::c_int) as isize);
        n = 0 as ::core::ffi::c_int;
        while *zSpan.offset(n as isize) as ::core::ffi::c_int != 0
            && *zSpan.offset(n as isize) as ::core::ffi::c_int != '.' as i32
        {
            n += 1;
        }
        if !zTab.is_null()
            && (sqlite3_strnicmp(zSpan, zTab, n) != 0 as ::core::ffi::c_int
                || *zTab.offset(n as isize) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
        {
            return 0 as ::core::ffi::c_int;
        }
        zSpan = zSpan.offset((n + 1 as ::core::ffi::c_int) as isize);
        if !zCol.is_null() {
            if eEName == ENAME_TAB
                && sqlite3StrICmp(zSpan, zCol) != 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            if eEName == ENAME_ROWID && sqlite3IsRowid(zCol) == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
        }
        if eEName == ENAME_ROWID {
            *pbRowid = 1 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn areDoubleQuotedStringsEnabled(
    mut db: *mut sqlite3,
    mut pTopNC: *mut NameContext,
) -> ::core::ffi::c_int {
    unsafe {
        if (*db).init.busy != 0 {
            return 1 as ::core::ffi::c_int;
        }
        if (*pTopNC).ncFlags & NC_IsDDL != 0 {
            if sqlite3WritableSchema(db) != 0
                && (*db).flags as ::core::ffi::c_ulonglong
                    & SQLITE_DqsDML as ::core::ffi::c_ulonglong
                    != 0 as ::core::ffi::c_ulonglong
            {
                return 1 as ::core::ffi::c_int;
            }
            return ((*db).flags as ::core::ffi::c_ulonglong
                & SQLITE_DqsDDL as ::core::ffi::c_ulonglong
                != 0 as ::core::ffi::c_ulonglong) as ::core::ffi::c_int;
        } else {
            return ((*db).flags as ::core::ffi::c_ulonglong
                & SQLITE_DqsDML as ::core::ffi::c_ulonglong
                != 0 as ::core::ffi::c_ulonglong) as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprColUsed(mut pExpr: *mut Expr) -> Bitmask {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut pExTab: *mut Table = ::core::ptr::null_mut::<Table>();
        n = (*pExpr).iColumn as ::core::ffi::c_int;
        pExTab = (*pExpr).y.pTab;
        if (*pExTab).tabFlags as ::core::ffi::c_uint
            & TF_HasGenerated as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            && (*(*pExTab).aCol.offset(n as isize)).colFlags as ::core::ffi::c_int
                & COLFLAG_GENERATED != 0 as ::core::ffi::c_int
        {
            return if (*pExTab).nCol as ::core::ffi::c_int >= BMS {
                ALLBITS
            } else {
                ((1 as ::core::ffi::c_int as Bitmask)
                    << (*pExTab).nCol as ::core::ffi::c_int)
                    .wrapping_sub(1 as Bitmask)
            }
        } else {
            if n >= BMS {
                n = BMS - 1 as ::core::ffi::c_int;
            }
            return (1 as ::core::ffi::c_int as Bitmask) << n;
        };
    }
}
unsafe extern "C" fn extendFJMatch(
    mut pParse: *mut Parse,
    mut ppList: *mut *mut ExprList,
    mut pMatch: *mut SrcItem,
    mut iColumn: i16_0,
) {
    unsafe {
        let mut pNew: *mut Expr = sqlite3ExprAlloc(
            (*pParse).db,
            TK_COLUMN,
            ::core::ptr::null::<Token>(),
            0 as ::core::ffi::c_int,
        );
        if !pNew.is_null() {
            (*pNew).iTable = (*pMatch).iCursor;
            (*pNew).iColumn = iColumn as ynVar;
            (*pNew).y.pTab = (*pMatch).pSTab;
            (*pNew).flags
                |= 0x200000 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint;
            *ppList = sqlite3ExprListAppend(pParse, *ppList, pNew);
        }
    }
}
#[inline(never)]
unsafe extern "C" fn isValidSchemaTableName(
    mut zTab: *const ::core::ffi::c_char,
    mut pTab: *mut Table,
    mut zDb: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zLegacy: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if sqlite3_strnicmp(
            zTab,
            b"sqlite_\0".as_ptr() as *const ::core::ffi::c_char,
            7 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        zLegacy = (*pTab).zName;
        if strcmp(
            zLegacy.offset(7 as ::core::ffi::c_int as isize),
            LEGACY_TEMP_SCHEMA_TABLE.as_ptr().offset(7 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if sqlite3StrICmp(
                zTab.offset(7 as ::core::ffi::c_int as isize),
                PREFERRED_TEMP_SCHEMA_TABLE
                    .as_ptr()
                    .offset(7 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            if zDb.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if sqlite3StrICmp(
                zTab.offset(7 as ::core::ffi::c_int as isize),
                LEGACY_SCHEMA_TABLE.as_ptr().offset(7 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            if sqlite3StrICmp(
                zTab.offset(7 as ::core::ffi::c_int as isize),
                PREFERRED_SCHEMA_TABLE.as_ptr().offset(7 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
        } else if sqlite3StrICmp(
            zTab.offset(7 as ::core::ffi::c_int as isize),
            PREFERRED_SCHEMA_TABLE.as_ptr().offset(7 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn lookupName(
    mut pParse: *mut Parse,
    mut zDb: *const ::core::ffi::c_char,
    mut zTab: *const ::core::ffi::c_char,
    mut pRight: *const Expr,
    mut pNC: *mut NameContext,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut cntTab: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nSubquery: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut pMatch: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut pTopNC: *mut NameContext = pNC;
        let mut pSchema: *mut Schema = ::core::ptr::null_mut::<Schema>();
        let mut eNewExprOp: ::core::ffi::c_int = TK_COLUMN;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pFJMatch: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut zCol: *const ::core::ffi::c_char = (*pRight).u.zToken;
        (*pExpr).iTable = -1 as ::core::ffi::c_int;
        if !zDb.is_null() {
            if (*pNC).ncFlags & (NC_PartIdx | NC_IsCheck) != 0 as ::core::ffi::c_int {
                zDb = ::core::ptr::null::<::core::ffi::c_char>();
            } else {
                i = 0 as ::core::ffi::c_int;
                while i < (*db).nDb {
                    if sqlite3StrICmp((*(*db).aDb.offset(i as isize)).zDbSName, zDb)
                        == 0 as ::core::ffi::c_int
                    {
                        pSchema = (*(*db).aDb.offset(i as isize)).pSchema;
                        break;
                    } else {
                        i += 1;
                    }
                }
                if i == (*db).nDb
                    && sqlite3StrICmp(
                        b"main\0".as_ptr() as *const ::core::ffi::c_char,
                        zDb,
                    ) == 0 as ::core::ffi::c_int
                {
                    pSchema = (*(*db).aDb.offset(0 as ::core::ffi::c_int as isize))
                        .pSchema;
                    zDb = (*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).zDbSName;
                }
            }
        }
        's_125: loop {
            let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            let mut pSrcList: *mut SrcList = (*pNC).pSrcList;
            if !pSrcList.is_null() {
                let mut c2rust_current_block_79: u64;
                i = 0 as ::core::ffi::c_int;
                pItem = &raw mut (*pSrcList).a as *mut SrcItem;
                while i < (*pSrcList).nSrc {
                    pTab = (*pItem).pSTab;
                    if (*pItem).fg.isNestedFrom() != 0 {
                        let mut hit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut pSel: *mut Select = ::core::ptr::null_mut::<Select>();
                        pSel = (*(*pItem).u4.pSubq).pSelect;
                        pEList = (*pSel).pEList;
                        let mut c2rust_current_block_52: u64;
                        j = 0 as ::core::ffi::c_int;
                        while j < (*pEList).nExpr {
                            let mut bRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            if !(sqlite3MatchEName(
                                (&raw mut (*pEList).a as *mut ExprList_item)
                                    .offset(j as isize) as *mut ExprList_item,
                                zCol,
                                zTab,
                                zDb,
                                &raw mut bRowid,
                            ) == 0)
                            {
                                if bRowid == 0 as ::core::ffi::c_int {
                                    if cnt > 0 as ::core::ffi::c_int {
                                        if (*pItem).fg.isUsing() as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                            || sqlite3IdListIndex((*pItem).u3.pUsing, zCol)
                                                < 0 as ::core::ffi::c_int || pMatch == pItem
                                        {
                                            sqlite3ExprListDelete(db, pFJMatch);
                                            pFJMatch = ::core::ptr::null_mut::<ExprList>();
                                            c2rust_current_block_52 = 10095721787123848864;
                                        } else if (*pItem).fg.jointype as ::core::ffi::c_int
                                            & JT_RIGHT == 0 as ::core::ffi::c_int
                                        {
                                            c2rust_current_block_52 = 11048769245176032998;
                                        } else {
                                            if (*pItem).fg.jointype as ::core::ffi::c_int & JT_LEFT
                                                == 0 as ::core::ffi::c_int
                                            {
                                                cnt = 0 as ::core::ffi::c_int;
                                                sqlite3ExprListDelete(db, pFJMatch);
                                                pFJMatch = ::core::ptr::null_mut::<ExprList>();
                                            } else {
                                                extendFJMatch(
                                                    pParse,
                                                    &raw mut pFJMatch,
                                                    pMatch,
                                                    (*pExpr).iColumn as i16_0,
                                                );
                                            }
                                            c2rust_current_block_52 = 10095721787123848864;
                                        }
                                    } else {
                                        c2rust_current_block_52 = 10095721787123848864;
                                    }
                                    match c2rust_current_block_52 {
                                        11048769245176032998 => {}
                                        _ => {
                                            cnt += 1;
                                            hit = 1 as ::core::ffi::c_int;
                                            c2rust_current_block_52 = 9353995356876505083;
                                        }
                                    }
                                } else if cnt > 0 as ::core::ffi::c_int {
                                    c2rust_current_block_52 = 11048769245176032998;
                                } else {
                                    c2rust_current_block_52 = 9353995356876505083;
                                }
                                match c2rust_current_block_52 {
                                    11048769245176032998 => {}
                                    _ => {
                                        cntTab += 1;
                                        pMatch = pItem;
                                        (*pExpr).iColumn = j as ynVar;
                                        let ref mut c2rust_fresh2 = (*(&raw mut (*pEList).a
                                            as *mut ExprList_item)
                                            .offset(j as isize))
                                            .fg;
                                        (*c2rust_fresh2)
                                            .set_bUsed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                                        if (*(&raw mut (*pEList).a as *mut ExprList_item)
                                            .offset(j as isize))
                                            .fg
                                            .bUsingTerm() != 0
                                        {
                                            break;
                                        }
                                    }
                                }
                            }
                            j += 1;
                        }
                        if hit != 0 || zTab.is_null() {
                            c2rust_current_block_79 = 15125582407903384992;
                        } else {
                            c2rust_current_block_79 = 17075014677070940716;
                        }
                    } else {
                        c2rust_current_block_79 = 17075014677070940716;
                    }
                    match c2rust_current_block_79 {
                        17075014677070940716 => {
                            if !zTab.is_null() {
                                if !zDb.is_null() {
                                    if (*pTab).pSchema != pSchema {
                                        c2rust_current_block_79 = 15125582407903384992;
                                    } else if pSchema.is_null()
                                        && strcmp(
                                            zDb,
                                            b"*\0".as_ptr() as *const ::core::ffi::c_char,
                                        ) != 0 as ::core::ffi::c_int
                                    {
                                        c2rust_current_block_79 = 15125582407903384992;
                                    } else {
                                        c2rust_current_block_79 = 8869332144787829186;
                                    }
                                } else {
                                    c2rust_current_block_79 = 8869332144787829186;
                                }
                                match c2rust_current_block_79 {
                                    15125582407903384992 => {}
                                    _ => {
                                        if !(*pItem).zAlias.is_null() {
                                            if sqlite3StrICmp(zTab, (*pItem).zAlias)
                                                != 0 as ::core::ffi::c_int
                                            {
                                                c2rust_current_block_79 = 15125582407903384992;
                                            } else {
                                                c2rust_current_block_79 = 2723324002591448311;
                                            }
                                        } else if sqlite3StrICmp(zTab, (*pTab).zName)
                                            != 0 as ::core::ffi::c_int
                                        {
                                            if (*pTab).tnum != 1 as ::core::ffi::c_uint {
                                                c2rust_current_block_79 = 15125582407903384992;
                                            } else if isValidSchemaTableName(zTab, pTab, zDb) == 0 {
                                                c2rust_current_block_79 = 15125582407903384992;
                                            } else {
                                                c2rust_current_block_79 = 2723324002591448311;
                                            }
                                        } else {
                                            c2rust_current_block_79 = 2723324002591448311;
                                        }
                                        match c2rust_current_block_79 {
                                            15125582407903384992 => {}
                                            _ => {
                                                if (*pParse).eParseMode as ::core::ffi::c_int
                                                    >= PARSE_MODE_RENAME && !(*pItem).zAlias.is_null()
                                                {
                                                    sqlite3RenameTokenRemap(
                                                        pParse,
                                                        ::core::ptr::null::<::core::ffi::c_void>(),
                                                        &raw mut (*pExpr).y.pTab as *mut ::core::ffi::c_void,
                                                    );
                                                }
                                                c2rust_current_block_79 = 10435735846551762309;
                                            }
                                        }
                                    }
                                }
                            } else {
                                c2rust_current_block_79 = 10435735846551762309;
                            }
                            match c2rust_current_block_79 {
                                15125582407903384992 => {}
                                _ => {
                                    j = sqlite3ColumnIndex(pTab, zCol);
                                    if j >= 0 as ::core::ffi::c_int {
                                        if cnt > 0 as ::core::ffi::c_int {
                                            if (*pItem).fg.isUsing() as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                                || sqlite3IdListIndex((*pItem).u3.pUsing, zCol)
                                                    < 0 as ::core::ffi::c_int
                                            {
                                                sqlite3ExprListDelete(db, pFJMatch);
                                                pFJMatch = ::core::ptr::null_mut::<ExprList>();
                                                c2rust_current_block_79 = 16791665189521845338;
                                            } else if (*pItem).fg.jointype as ::core::ffi::c_int
                                                & JT_RIGHT == 0 as ::core::ffi::c_int
                                            {
                                                c2rust_current_block_79 = 15125582407903384992;
                                            } else {
                                                if (*pItem).fg.jointype as ::core::ffi::c_int & JT_LEFT
                                                    == 0 as ::core::ffi::c_int
                                                {
                                                    cnt = 0 as ::core::ffi::c_int;
                                                    sqlite3ExprListDelete(db, pFJMatch);
                                                    pFJMatch = ::core::ptr::null_mut::<ExprList>();
                                                } else {
                                                    extendFJMatch(
                                                        pParse,
                                                        &raw mut pFJMatch,
                                                        pMatch,
                                                        (*pExpr).iColumn as i16_0,
                                                    );
                                                }
                                                c2rust_current_block_79 = 16791665189521845338;
                                            }
                                        } else {
                                            c2rust_current_block_79 = 16791665189521845338;
                                        }
                                        match c2rust_current_block_79 {
                                            15125582407903384992 => {}
                                            _ => {
                                                cnt += 1;
                                                pMatch = pItem;
                                                (*pExpr).iColumn = (if j
                                                    == (*pTab).iPKey as ::core::ffi::c_int
                                                {
                                                    -1 as ::core::ffi::c_int
                                                } else {
                                                    j as i16_0 as ::core::ffi::c_int
                                                }) as ynVar;
                                                if (*pItem).fg.isNestedFrom() != 0 {
                                                    sqlite3SrcItemColumnUsed(pItem, j);
                                                }
                                                c2rust_current_block_79 = 16313536926714486912;
                                            }
                                        }
                                    } else {
                                        c2rust_current_block_79 = 16313536926714486912;
                                    }
                                    match c2rust_current_block_79 {
                                        15125582407903384992 => {}
                                        _ => {
                                            if 0 as ::core::ffi::c_int == cnt
                                                && (*pTab).tabFlags as ::core::ffi::c_uint
                                                    & TF_NoVisibleRowid as ::core::ffi::c_uint
                                                    == 0 as ::core::ffi::c_uint
                                            {
                                                cntTab += 1;
                                                pMatch = pItem;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                    pItem = pItem.offset(1);
                }
                if !pMatch.is_null() {
                    (*pExpr).iTable = (*pMatch).iCursor;
                    (*pExpr).y.pTab = (*pMatch).pSTab;
                    if (*pMatch).fg.jointype as ::core::ffi::c_int & (JT_LEFT | JT_LTORJ)
                        != 0 as ::core::ffi::c_int
                    {
                        (*pExpr).flags
                            |= 0x200000 as ::core::ffi::c_int as u32_0
                                as ::core::ffi::c_uint;
                    }
                    pSchema = (*(*pExpr).y.pTab).pSchema;
                }
            }
            if cnt == 0 as ::core::ffi::c_int && zDb.is_null() {
                pTab = ::core::ptr::null_mut::<Table>();
                if !(*pParse).pTriggerTab.is_null() {
                    let mut op: ::core::ffi::c_int = (*pParse).eTriggerOp
                        as ::core::ffi::c_int;
                    if (*pParse).bReturning != 0 {
                        if (*pNC).ncFlags & NC_UBaseReg != 0 as ::core::ffi::c_int
                            && (zTab.is_null()
                                || sqlite3StrICmp(zTab, (*(*pParse).pTriggerTab).zName)
                                    == 0 as ::core::ffi::c_int
                                || isValidSchemaTableName(
                                    zTab,
                                    (*pParse).pTriggerTab,
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                ) != 0)
                        {
                            (*pExpr).iTable = (op != TK_DELETE) as ::core::ffi::c_int;
                            pTab = (*pParse).pTriggerTab;
                        }
                    } else if op != TK_DELETE && !zTab.is_null()
                        && sqlite3StrICmp(
                            b"new\0".as_ptr() as *const ::core::ffi::c_char,
                            zTab,
                        ) == 0 as ::core::ffi::c_int
                    {
                        (*pExpr).iTable = 1 as ::core::ffi::c_int;
                        pTab = (*pParse).pTriggerTab;
                    } else if op != TK_INSERT && !zTab.is_null()
                        && sqlite3StrICmp(
                            b"old\0".as_ptr() as *const ::core::ffi::c_char,
                            zTab,
                        ) == 0 as ::core::ffi::c_int
                    {
                        (*pExpr).iTable = 0 as ::core::ffi::c_int;
                        pTab = (*pParse).pTriggerTab;
                    }
                }
                if (*pNC).ncFlags & NC_UUpsert != 0 as ::core::ffi::c_int
                    && !zTab.is_null()
                {
                    let mut pUpsert: *mut Upsert = (*pNC).uNC.pUpsert;
                    if !pUpsert.is_null()
                        && sqlite3StrICmp(
                            b"excluded\0".as_ptr() as *const ::core::ffi::c_char,
                            zTab,
                        ) == 0 as ::core::ffi::c_int
                    {
                        pTab = (*(&raw mut (*(*pUpsert).pUpsertSrc).a as *mut SrcItem)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .pSTab;
                        (*pExpr).iTable = EXCLUDED_TABLE_NUMBER;
                    }
                }
                if !pTab.is_null() {
                    let mut iCol: ::core::ffi::c_int = 0;
                    pSchema = (*pTab).pSchema;
                    cntTab += 1;
                    iCol = sqlite3ColumnIndex(pTab, zCol);
                    if iCol >= 0 as ::core::ffi::c_int {
                        if (*pTab).iPKey as ::core::ffi::c_int == iCol {
                            iCol = -1 as ::core::ffi::c_int;
                        }
                    } else if sqlite3IsRowid(zCol) != 0
                        && (*pTab).tabFlags as ::core::ffi::c_uint
                            & TF_NoVisibleRowid as ::core::ffi::c_uint
                            == 0 as ::core::ffi::c_uint
                    {
                        iCol = -1 as ::core::ffi::c_int;
                    } else {
                        iCol = (*pTab).nCol as ::core::ffi::c_int;
                    }
                    if iCol < (*pTab).nCol as ::core::ffi::c_int {
                        cnt += 1;
                        pMatch = ::core::ptr::null_mut::<SrcItem>();
                        if (*pExpr).iTable == EXCLUDED_TABLE_NUMBER {
                            if (*pParse).eParseMode as ::core::ffi::c_int
                                >= PARSE_MODE_RENAME
                            {
                                (*pExpr).iColumn = iCol as ynVar;
                                (*pExpr).y.pTab = pTab;
                                eNewExprOp = TK_COLUMN;
                            } else {
                                (*pExpr).iTable = (*(*pNC).uNC.pUpsert).regData
                                    + sqlite3TableColumnToStorage(pTab, iCol as i16_0)
                                        as ::core::ffi::c_int;
                                eNewExprOp = TK_REGISTER;
                            }
                        } else {
                            (*pExpr).y.pTab = pTab;
                            if (*pParse).bReturning != 0 {
                                eNewExprOp = TK_REGISTER;
                                (*pExpr).op2 = TK_COLUMN as u8_0;
                                (*pExpr).iColumn = iCol as ynVar;
                                (*pExpr).iTable = (*pNC).uNC.iBaseReg
                                    + ((*pTab).nCol as ::core::ffi::c_int
                                        + 1 as ::core::ffi::c_int) * (*pExpr).iTable
                                    + sqlite3TableColumnToStorage(pTab, iCol as i16_0)
                                        as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                            } else {
                                (*pExpr).iColumn = iCol as i16_0 as ynVar;
                                eNewExprOp = TK_TRIGGER;
                                if iCol < 0 as ::core::ffi::c_int {
                                    (*pExpr).affExpr = SQLITE_AFF_INTEGER
                                        as ::core::ffi::c_char;
                                } else if (*pExpr).iTable == 0 as ::core::ffi::c_int {
                                    (*pParse).oldmask
                                        |= if iCol >= 32 as ::core::ffi::c_int {
                                            0xffffffff as ::core::ffi::c_uint
                                        } else {
                                            (1 as ::core::ffi::c_int as ::core::ffi::c_uint) << iCol
                                        };
                                } else {
                                    (*pParse).newmask
                                        |= if iCol >= 32 as ::core::ffi::c_int {
                                            0xffffffff as ::core::ffi::c_uint
                                        } else {
                                            (1 as ::core::ffi::c_int as ::core::ffi::c_uint) << iCol
                                        };
                                }
                            }
                        }
                    }
                }
            }
            if cnt == 0 as ::core::ffi::c_int && cntTab >= 1 as ::core::ffi::c_int
                && !pMatch.is_null()
                && (*pNC).ncFlags & (NC_IdxExpr | NC_GenCol) == 0 as ::core::ffi::c_int
                && sqlite3IsRowid(zCol) != 0
                && ((*(*pMatch).pSTab).tabFlags as ::core::ffi::c_uint
                    & 0x200 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                    || (*pMatch).fg.isNestedFrom() as ::core::ffi::c_int != 0)
            {
                cnt = cntTab;
                if (*pMatch).fg.isNestedFrom() as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*pExpr).iColumn = -1 as ::core::ffi::c_int as ynVar;
                }
                (*pExpr).affExpr = SQLITE_AFF_INTEGER as ::core::ffi::c_char;
            }
            if cnt == 0 as ::core::ffi::c_int
                && (*pNC).ncFlags & NC_UEList != 0 as ::core::ffi::c_int
                && zTab.is_null()
            {
                pEList = (*pNC).uNC.pEList;
                j = 0 as ::core::ffi::c_int;
                while j < (*pEList).nExpr {
                    let mut zAs: *mut ::core::ffi::c_char = (*(&raw mut (*pEList).a
                        as *mut ExprList_item)
                        .offset(j as isize))
                        .zEName;
                    if (*(&raw mut (*pEList).a as *mut ExprList_item).offset(j as isize))
                        .fg
                        .eEName() as ::core::ffi::c_int == ENAME_NAME
                        && sqlite3_stricmp(zAs, zCol) == 0 as ::core::ffi::c_int
                    {
                        let mut pOrig: *mut Expr = ::core::ptr::null_mut::<Expr>();
                        pOrig = (*(&raw mut (*pEList).a as *mut ExprList_item)
                            .offset(j as isize))
                            .pExpr;
                        if (*pNC).ncFlags & NC_AllowAgg == 0 as ::core::ffi::c_int
                            && (*pOrig).flags & 0x10 as ::core::ffi::c_int as u32_0
                                != 0 as ::core::ffi::c_uint
                        {
                            sqlite3ErrorMsg(
                                pParse,
                                b"misuse of aliased aggregate %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                zAs,
                            );
                            return WRC_Abort;
                        }
                        if (*pOrig).flags & 0x8000 as ::core::ffi::c_int as u32_0
                            != 0 as ::core::ffi::c_uint
                            && ((*pNC).ncFlags & NC_AllowWin == 0 as ::core::ffi::c_int
                                || pNC != pTopNC)
                        {
                            sqlite3ErrorMsg(
                                pParse,
                                b"misuse of aliased window function %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                zAs,
                            );
                            return WRC_Abort;
                        }
                        if sqlite3ExprVectorSize(pOrig) != 1 as ::core::ffi::c_int {
                            sqlite3ErrorMsg(
                                pParse,
                                b"row value misused\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            return WRC_Abort;
                        }
                        resolveAlias(pParse, pEList, j, pExpr, nSubquery);
                        cnt = 1 as ::core::ffi::c_int;
                        pMatch = ::core::ptr::null_mut::<SrcItem>();
                        if (*pParse).eParseMode as ::core::ffi::c_int
                            >= PARSE_MODE_RENAME
                        {
                            sqlite3RenameTokenRemap(
                                pParse,
                                ::core::ptr::null::<::core::ffi::c_void>(),
                                pExpr as *mut ::core::ffi::c_void,
                            );
                        }
                        c2rust_current_block = 11699136672944622576;
                        break 's_125;
                    } else {
                        j += 1;
                    }
                }
            }
            if cnt != 0 {
                c2rust_current_block = 2661260690706066674;
                break;
            }
            pNC = (*pNC).pNext;
            nSubquery += 1;
            if pNC.is_null() {
                c2rust_current_block = 2661260690706066674;
                break;
            }
        }
        match c2rust_current_block {
            2661260690706066674 => {
                if cnt == 0 as ::core::ffi::c_int && zTab.is_null() {
                    if (*pExpr).flags & 0x80 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint
                        && areDoubleQuotedStringsEnabled(db, pTopNC) != 0
                    {
                        sqlite3_log(
                            SQLITE_WARNING,
                            b"double-quoted string literal: \"%w\"\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            zCol,
                        );
                        (*pExpr).op = TK_STRING as u8_0;
                        memset(
                            &raw mut (*pExpr).y as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<C2Rust_Unnamed_0>() as size_t,
                        );
                        return WRC_Prune;
                    }
                    if sqlite3ExprIdToTrueFalse(pExpr) != 0 {
                        return WRC_Prune;
                    }
                }
                if cnt != 1 as ::core::ffi::c_int {
                    let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
                        ::core::ffi::c_char,
                    >();
                    if !pFJMatch.is_null() {
                        if (*pFJMatch).nExpr == cnt - 1 as ::core::ffi::c_int {
                            if (*pExpr).flags & 0x800000 as ::core::ffi::c_int as u32_0
                                != 0 as ::core::ffi::c_uint
                            {
                                (*pExpr).flags
                                    &= !(0x800000 as ::core::ffi::c_int as u32_0)
                                        as ::core::ffi::c_uint;
                            } else {
                                sqlite3ExprDelete(db, (*pExpr).pLeft);
                                (*pExpr).pLeft = ::core::ptr::null_mut::<Expr>();
                                sqlite3ExprDelete(db, (*pExpr).pRight);
                                (*pExpr).pRight = ::core::ptr::null_mut::<Expr>();
                            }
                            extendFJMatch(
                                pParse,
                                &raw mut pFJMatch,
                                pMatch,
                                (*pExpr).iColumn as i16_0,
                            );
                            (*pExpr).op = TK_FUNCTION as u8_0;
                            (*pExpr).u.zToken = b"coalesce\0".as_ptr()
                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                            (*pExpr).x.pList = pFJMatch;
                            cnt = 1 as ::core::ffi::c_int;
                            c2rust_current_block = 11699136672944622576;
                        } else {
                            sqlite3ExprListDelete(db, pFJMatch);
                            pFJMatch = ::core::ptr::null_mut::<ExprList>();
                            c2rust_current_block = 17664728594743454682;
                        }
                    } else {
                        c2rust_current_block = 17664728594743454682;
                    }
                    match c2rust_current_block {
                        11699136672944622576 => {}
                        _ => {
                            zErr = if cnt == 0 as ::core::ffi::c_int {
                                b"no such column\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"ambiguous column name\0".as_ptr()
                                    as *const ::core::ffi::c_char
                            };
                            if !zDb.is_null() {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"%s: %s.%s.%s\0".as_ptr() as *const ::core::ffi::c_char,
                                    zErr,
                                    zDb,
                                    zTab,
                                    zCol,
                                );
                            } else if !zTab.is_null() {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"%s: %s.%s\0".as_ptr() as *const ::core::ffi::c_char,
                                    zErr,
                                    zTab,
                                    zCol,
                                );
                            } else if cnt == 0 as ::core::ffi::c_int
                                && (*pRight).flags & 0x80 as ::core::ffi::c_int as u32_0
                                    != 0 as ::core::ffi::c_uint
                            {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"%s: \"%s\" - should this be a string literal in single-quotes?\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    zErr,
                                    zCol,
                                );
                            } else {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                    zErr,
                                    zCol,
                                );
                            }
                            sqlite3RecordErrorOffsetOfExpr((*pParse).db, pExpr);
                            (*pParse).set_checkSchema(1 as bft as bft);
                            (*pTopNC).nNcErr += 1;
                            eNewExprOp = TK_NULL;
                            c2rust_current_block = 3244376785761602731;
                        }
                    }
                } else {
                    c2rust_current_block = 3244376785761602731;
                }
                match c2rust_current_block {
                    11699136672944622576 => {}
                    _ => {
                        if !((*pExpr).flags
                            & (0x10000 as ::core::ffi::c_int
                                | 0x800000 as ::core::ffi::c_int) as u32_0
                            != 0 as ::core::ffi::c_uint)
                        {
                            sqlite3ExprDelete(db, (*pExpr).pLeft);
                            (*pExpr).pLeft = ::core::ptr::null_mut::<Expr>();
                            sqlite3ExprDelete(db, (*pExpr).pRight);
                            (*pExpr).pRight = ::core::ptr::null_mut::<Expr>();
                            (*pExpr).flags
                                |= 0x800000 as ::core::ffi::c_int as u32_0
                                    as ::core::ffi::c_uint;
                        }
                        if !pMatch.is_null() {
                            if (*pExpr).iColumn as ::core::ffi::c_int
                                >= 0 as ::core::ffi::c_int
                            {
                                (*pMatch).colUsed
                                    |= sqlite3ExprColUsed(pExpr) as ::core::ffi::c_ulonglong;
                            } else {
                                (*pMatch)
                                    .fg
                                    .set_rowidUsed(
                                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                    );
                            }
                        }
                        (*pExpr).op = eNewExprOp as u8_0;
                    }
                }
            }
            _ => {}
        }
        if cnt == 1 as ::core::ffi::c_int {
            if (*(*pParse).db).xAuth.is_some()
                && ((*pExpr).op as ::core::ffi::c_int == TK_COLUMN
                    || (*pExpr).op as ::core::ffi::c_int == TK_TRIGGER)
            {
                sqlite3AuthRead(pParse, pExpr, pSchema, (*pNC).pSrcList);
            }
            loop {
                (*pTopNC).nRef += 1;
                if pTopNC == pNC {
                    break;
                }
                pTopNC = (*pTopNC).pNext;
            }
            return WRC_Prune;
        } else {
            return WRC_Abort
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CreateColumnExpr(
    mut db: *mut sqlite3,
    mut pSrc: *mut SrcList,
    mut iSrc: ::core::ffi::c_int,
    mut iCol: ::core::ffi::c_int,
) -> *mut Expr {
    unsafe {
        let mut p: *mut Expr = sqlite3ExprAlloc(
            db,
            TK_COLUMN,
            ::core::ptr::null::<Token>(),
            0 as ::core::ffi::c_int,
        );
        if !p.is_null() {
            let mut pItem: *mut SrcItem = (&raw mut (*pSrc).a as *mut SrcItem)
                .offset(iSrc as isize) as *mut SrcItem;
            let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
            (*p).y.pTab = (*pItem).pSTab;
            pTab = (*p).y.pTab;
            (*p).iTable = (*pItem).iCursor;
            if (*(*p).y.pTab).iPKey as ::core::ffi::c_int == iCol {
                (*p).iColumn = -1 as ::core::ffi::c_int as ynVar;
            } else {
                (*p).iColumn = iCol as ynVar;
                if (*pTab).tabFlags as ::core::ffi::c_uint
                    & TF_HasGenerated as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                    && (*(*pTab).aCol.offset(iCol as isize)).colFlags
                        as ::core::ffi::c_int & COLFLAG_GENERATED
                        != 0 as ::core::ffi::c_int
                {
                    (*pItem).colUsed = (if (*pTab).nCol as ::core::ffi::c_int
                        >= 64 as ::core::ffi::c_int
                    {
                        ALLBITS
                    } else {
                        ((1 as ::core::ffi::c_int as ::core::ffi::c_ulonglong)
                            << (*pTab).nCol as ::core::ffi::c_int)
                            .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                    }) as Bitmask;
                } else {
                    (*pItem).colUsed
                        |= ((1 as ::core::ffi::c_int as Bitmask)
                            << (if iCol >= BMS {
                                BMS - 1 as ::core::ffi::c_int
                            } else {
                                iCol
                            })) as ::core::ffi::c_ulonglong;
                }
            }
        }
        return p;
    }
}
unsafe extern "C" fn notValidImpl(
    mut pParse: *mut Parse,
    mut pNC: *mut NameContext,
    mut zMsg: *const ::core::ffi::c_char,
    mut pExpr: *mut Expr,
    mut pError: *mut Expr,
) {
    unsafe {
        let mut zIn: *const ::core::ffi::c_char = b"partial index WHERE clauses\0"
            .as_ptr() as *const ::core::ffi::c_char;
        if (*pNC).ncFlags & NC_IdxExpr != 0 {
            zIn = b"index expressions\0".as_ptr() as *const ::core::ffi::c_char;
        } else if (*pNC).ncFlags & NC_IsCheck != 0 {
            zIn = b"CHECK constraints\0".as_ptr() as *const ::core::ffi::c_char;
        } else if (*pNC).ncFlags & NC_GenCol != 0 {
            zIn = b"generated columns\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sqlite3ErrorMsg(
            pParse,
            b"%s prohibited in %s\0".as_ptr() as *const ::core::ffi::c_char,
            zMsg,
            zIn,
        );
        if !pExpr.is_null() {
            (*pExpr).op = TK_NULL as u8_0;
        }
        sqlite3RecordErrorOffsetOfExpr((*pParse).db, pError);
    }
}
unsafe extern "C" fn exprProbability(mut p: *mut Expr) -> ::core::ffi::c_int {
    unsafe {
        let mut r: ::core::ffi::c_double = -1.0f64;
        if (*p).op as ::core::ffi::c_int != TK_FLOAT {
            return -1 as ::core::ffi::c_int;
        }
        sqlite3AtoF(
            (*p).u.zToken,
            &raw mut r,
            sqlite3Strlen30((*p).u.zToken),
            SQLITE_UTF8 as u8_0,
        );
        if r > 1.0f64 {
            return -1 as ::core::ffi::c_int;
        }
        return (r * 134217728.0f64) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn resolveExprStep(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pNC: *mut NameContext = ::core::ptr::null_mut::<NameContext>();
        let mut pParse: *mut Parse = ::core::ptr::null_mut::<Parse>();
        pNC = (*pWalker).u.pNC;
        pParse = (*pNC).pParse;
        let mut c2rust_current_block_278: u64;
        match (*pExpr).op as ::core::ffi::c_int {
            TK_ROW => {
                let mut pSrcList: *mut SrcList = (*pNC).pSrcList;
                let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
                pItem = &raw mut (*pSrcList).a as *mut SrcItem;
                (*pExpr).op = TK_COLUMN as u8_0;
                (*pExpr).y.pTab = (*pItem).pSTab;
                (*pExpr).iTable = (*pItem).iCursor;
                (*pExpr).iColumn -= 1;
                (*pExpr).affExpr = SQLITE_AFF_INTEGER as ::core::ffi::c_char;
                c2rust_current_block_278 = 14666402909811248417;
            }
            TK_NOTNULL | TK_ISNULL => {
                let mut anRef: [::core::ffi::c_int; 8] = [0; 8];
                let mut p: *mut NameContext = ::core::ptr::null_mut::<NameContext>();
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                p = pNC;
                while !p.is_null()
                    && i
                        < (::core::mem::size_of::<[::core::ffi::c_int; 8]>() as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) as ::core::ffi::c_int
                {
                    anRef[i as usize] = (*p).nRef;
                    p = (*p).pNext;
                    i += 1;
                }
                sqlite3WalkExpr(pWalker, (*pExpr).pLeft);
                if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                    return WRC_Prune;
                }
                if sqlite3ExprCanBeNull((*pExpr).pLeft) != 0 {
                    return WRC_Prune;
                }
                i = 0 as ::core::ffi::c_int;
                p = pNC;
                while !p.is_null() {
                    if (*p).ncFlags & NC_Where == 0 as ::core::ffi::c_int {
                        return WRC_Prune;
                    }
                    p = (*p).pNext;
                    i += 1;
                }
                (*pExpr).u.iValue = ((*pExpr).op as ::core::ffi::c_int == TK_NOTNULL)
                    as ::core::ffi::c_int;
                (*pExpr).flags |= EP_IntValue as ::core::ffi::c_uint;
                (*pExpr).op = TK_INTEGER as u8_0;
                i = 0 as ::core::ffi::c_int;
                p = pNC;
                while !p.is_null()
                    && i
                        < (::core::mem::size_of::<[::core::ffi::c_int; 8]>() as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) as ::core::ffi::c_int
                {
                    (*p).nRef = anRef[i as usize];
                    p = (*p).pNext;
                    i += 1;
                }
                sqlite3ExprDelete((*pParse).db, (*pExpr).pLeft);
                (*pExpr).pLeft = ::core::ptr::null_mut::<Expr>();
                return WRC_Prune;
            }
            TK_ID | TK_DOT => {
                let mut zTable: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut pRight: *mut Expr = ::core::ptr::null_mut::<Expr>();
                if (*pExpr).op as ::core::ffi::c_int == TK_ID {
                    zDb = ::core::ptr::null::<::core::ffi::c_char>();
                    zTable = ::core::ptr::null::<::core::ffi::c_char>();
                    pRight = pExpr;
                } else {
                    let mut pLeft: *mut Expr = (*pExpr).pLeft;
                    if (*pNC).ncFlags
                        & (0x20 as ::core::ffi::c_int | 0x8 as ::core::ffi::c_int)
                        != 0 as ::core::ffi::c_int
                    {
                        notValidImpl(
                            pParse,
                            pNC,
                            b"the \".\" operator\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            ::core::ptr::null_mut::<Expr>(),
                            pExpr,
                        );
                    }
                    pRight = (*pExpr).pRight;
                    if (*pRight).op as ::core::ffi::c_int == TK_ID {
                        zDb = ::core::ptr::null::<::core::ffi::c_char>();
                    } else {
                        zDb = (*pLeft).u.zToken;
                        pLeft = (*pRight).pLeft;
                        pRight = (*pRight).pRight;
                    }
                    zTable = (*pLeft).u.zToken;
                    if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                        sqlite3RenameTokenRemap(
                            pParse,
                            pExpr as *mut ::core::ffi::c_void,
                            pRight as *mut ::core::ffi::c_void,
                        );
                        sqlite3RenameTokenRemap(
                            pParse,
                            &raw mut (*pExpr).y.pTab as *mut ::core::ffi::c_void,
                            pLeft as *mut ::core::ffi::c_void,
                        );
                    }
                }
                return lookupName(pParse, zDb, zTable, pRight, pNC, pExpr);
            }
            TK_FUNCTION => {
                let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                let mut n: ::core::ffi::c_int = 0;
                let mut no_such_func: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut wrong_num_args: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut is_agg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut zId: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut pDef: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
                let mut enc: u8_0 = (*(*pParse).db).enc;
                let mut savedAllowFlags: ::core::ffi::c_int = (*pNC).ncFlags
                    & (NC_AllowAgg | NC_AllowWin);
                let mut pWin: *mut Window = if (*pExpr).flags
                    & 0x1000000 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint
                    && (*(*pExpr).y.pWin).eFrmType as ::core::ffi::c_int != TK_FILTER
                {
                    (*pExpr).y.pWin
                } else {
                    ::core::ptr::null_mut::<Window>()
                };
                pList = (*pExpr).x.pList;
                n = if !pList.is_null() {
                    (*pList).nExpr
                } else {
                    0 as ::core::ffi::c_int
                };
                zId = (*pExpr).u.zToken;
                pDef = sqlite3FindFunction((*pParse).db, zId, n, enc, 0 as u8_0);
                if pDef.is_null() {
                    pDef = sqlite3FindFunction(
                        (*pParse).db,
                        zId,
                        -2 as ::core::ffi::c_int,
                        enc,
                        0 as u8_0,
                    );
                    if pDef.is_null() {
                        no_such_func = 1 as ::core::ffi::c_int;
                    } else {
                        wrong_num_args = 1 as ::core::ffi::c_int;
                    }
                } else {
                    is_agg = (*pDef).xFinalize.is_some() as ::core::ffi::c_int;
                    if (*pDef).funcFlags as ::core::ffi::c_uint
                        & SQLITE_FUNC_UNLIKELY as ::core::ffi::c_uint != 0
                    {
                        (*pExpr).flags
                            |= 0x80000 as ::core::ffi::c_int as u32_0
                                as ::core::ffi::c_uint;
                        if n == 2 as ::core::ffi::c_int {
                            (*pExpr).iTable = exprProbability(
                                (*(&raw mut (*pList).a as *mut ExprList_item)
                                    .offset(1 as ::core::ffi::c_int as isize))
                                    .pExpr,
                            );
                            if (*pExpr).iTable < 0 as ::core::ffi::c_int {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"second argument to %#T() must be a constant between 0.0 and 1.0\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    pExpr,
                                );
                                (*pNC).nNcErr += 1;
                            }
                        } else {
                            (*pExpr).iTable = if *(*pDef)
                                .zName
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int == 'u' as i32
                            {
                                8388608 as ::core::ffi::c_int
                            } else {
                                125829120 as ::core::ffi::c_int
                            };
                        }
                    }
                    let mut auth: ::core::ffi::c_int = sqlite3AuthCheck(
                        pParse,
                        SQLITE_FUNCTION,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        (*pDef).zName,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                    if auth != SQLITE_OK {
                        if auth == SQLITE_DENY {
                            sqlite3ErrorMsg(
                                pParse,
                                b"not authorized to use function: %#T\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                pExpr,
                            );
                            (*pNC).nNcErr += 1;
                        }
                        (*pExpr).op = TK_NULL as u8_0;
                        return WRC_Prune;
                    }
                    if (*pDef).funcFlags as ::core::ffi::c_uint
                        & SQLITE_SUBTYPE as ::core::ffi::c_uint != 0
                        || (*pExpr).flags & 0x80000000 as ::core::ffi::c_uint
                            != 0 as ::core::ffi::c_uint
                    {
                        let mut ii: ::core::ffi::c_int = 0;
                        ii = 0 as ::core::ffi::c_int;
                        while ii < n {
                            let ref mut c2rust_fresh1 = (*(*(&raw mut (*pList).a
                                as *mut ExprList_item)
                                .offset(ii as isize))
                                .pExpr)
                                .flags;
                            *c2rust_fresh1
                                |= 0x80000000 as ::core::ffi::c_uint as ::core::ffi::c_uint;
                            ii += 1;
                        }
                    }
                    if (*pDef).funcFlags as ::core::ffi::c_uint
                        & (SQLITE_FUNC_CONSTANT | SQLITE_FUNC_SLOCHNG)
                            as ::core::ffi::c_uint != 0
                    {
                        (*pExpr).flags
                            |= 0x100000 as ::core::ffi::c_int as u32_0
                                as ::core::ffi::c_uint;
                    }
                    if (*pDef).funcFlags as ::core::ffi::c_uint
                        & SQLITE_FUNC_CONSTANT as ::core::ffi::c_uint
                        == 0 as ::core::ffi::c_uint
                    {
                        if (*pNC).ncFlags
                            & (0x20 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int
                                | 0x8 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int
                        {
                            notValidImpl(
                                pParse,
                                pNC,
                                b"non-deterministic functions\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                ::core::ptr::null_mut::<Expr>(),
                                pExpr,
                            );
                        }
                    } else {
                        (*pExpr).op2 = ((*pNC).ncFlags & NC_SelfRef) as u8_0;
                    }
                    if (*pDef).funcFlags as ::core::ffi::c_uint
                        & SQLITE_FUNC_INTERNAL as ::core::ffi::c_uint
                        != 0 as ::core::ffi::c_uint
                        && (*pParse).nested as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        && (*(*pParse).db).mDbFlags as ::core::ffi::c_uint
                            & DBFLAG_InternalFunc as ::core::ffi::c_uint
                            == 0 as ::core::ffi::c_uint
                    {
                        no_such_func = 1 as ::core::ffi::c_int;
                        pDef = ::core::ptr::null_mut::<FuncDef>();
                    } else if (*pDef).funcFlags as ::core::ffi::c_uint
                        & (SQLITE_FUNC_DIRECT | SQLITE_FUNC_UNSAFE)
                            as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                        && !((*pParse).eParseMode as ::core::ffi::c_int
                            >= PARSE_MODE_RENAME)
                    {
                        if (*pNC).ncFlags & NC_FromDDL != 0 {
                            (*pExpr).flags
                                |= 0x40000000 as ::core::ffi::c_int as u32_0
                                    as ::core::ffi::c_uint;
                        }
                        sqlite3ExprFunctionUsable(pParse, pExpr, pDef);
                    }
                }
                if 0 as ::core::ffi::c_int
                    == ((*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME)
                        as ::core::ffi::c_int
                {
                    if !pDef.is_null() && (*pDef).xValue.is_none() && !pWin.is_null() {
                        sqlite3ErrorMsg(
                            pParse,
                            b"%#T() may not be used as a window function\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            pExpr,
                        );
                        (*pNC).nNcErr += 1;
                    } else if is_agg != 0
                        && (*pNC).ncFlags & NC_AllowAgg == 0 as ::core::ffi::c_int
                        || is_agg != 0
                            && (*pDef).funcFlags as ::core::ffi::c_uint
                                & SQLITE_FUNC_WINDOW as ::core::ffi::c_uint != 0
                            && pWin.is_null()
                        || is_agg != 0 && !pWin.is_null()
                            && (*pNC).ncFlags & NC_AllowWin == 0 as ::core::ffi::c_int
                    {
                        let mut zType: *const ::core::ffi::c_char = ::core::ptr::null::<
                            ::core::ffi::c_char,
                        >();
                        if (*pDef).funcFlags as ::core::ffi::c_uint
                            & SQLITE_FUNC_WINDOW as ::core::ffi::c_uint != 0
                            || !pWin.is_null()
                        {
                            zType = b"window\0".as_ptr() as *const ::core::ffi::c_char;
                        } else {
                            zType = b"aggregate\0".as_ptr()
                                as *const ::core::ffi::c_char;
                        }
                        sqlite3ErrorMsg(
                            pParse,
                            b"misuse of %s function %#T()\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            zType,
                            pExpr,
                        );
                        (*pNC).nNcErr += 1;
                        is_agg = 0 as ::core::ffi::c_int;
                    } else if no_such_func != 0
                        && (*(*pParse).db).init.busy as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        sqlite3ErrorMsg(
                            pParse,
                            b"no such function: %#T\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            pExpr,
                        );
                        (*pNC).nNcErr += 1;
                    } else if wrong_num_args != 0 {
                        sqlite3ErrorMsg(
                            pParse,
                            b"wrong number of arguments to function %#T()\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            pExpr,
                        );
                        (*pNC).nNcErr += 1;
                    } else if is_agg == 0 as ::core::ffi::c_int
                        && (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                            != 0 as ::core::ffi::c_uint
                    {
                        sqlite3ErrorMsg(
                            pParse,
                            b"FILTER may not be used with non-aggregate %#T()\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            pExpr,
                        );
                        (*pNC).nNcErr += 1;
                    } else if is_agg == 0 as ::core::ffi::c_int
                        && !(*pExpr).pLeft.is_null()
                    {
                        sqlite3ExprOrderByAggregateError(pParse, pExpr);
                        (*pNC).nNcErr += 1;
                    }
                    if is_agg != 0 {
                        (*pNC).ncFlags
                            &= !(NC_AllowWin
                                | (if pWin.is_null() {
                                    NC_AllowAgg
                                } else {
                                    0 as ::core::ffi::c_int
                                }));
                    }
                } else if (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint || !(*pExpr).pLeft.is_null()
                {
                    is_agg = 1 as ::core::ffi::c_int;
                }
                sqlite3WalkExprList(pWalker, pList);
                if is_agg != 0 {
                    if !(*pExpr).pLeft.is_null() {
                        sqlite3WalkExprList(pWalker, (*(*pExpr).pLeft).x.pList);
                    }
                    if !pWin.is_null() && (*pParse).nErr == 0 as ::core::ffi::c_int {
                        let mut pSel: *mut Select = (*pNC).pWinSelect;
                        if ((*pParse).eParseMode as ::core::ffi::c_int
                            >= PARSE_MODE_RENAME) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            sqlite3WindowUpdate(
                                pParse,
                                if !pSel.is_null() {
                                    (*pSel).pWinDefn
                                } else {
                                    ::core::ptr::null_mut::<Window>()
                                },
                                pWin,
                                pDef,
                            );
                            if (*(*pParse).db).mallocFailed != 0 {
                                c2rust_current_block_278 = 14666402909811248417;
                            } else {
                                c2rust_current_block_278 = 11735322225073324345;
                            }
                        } else {
                            c2rust_current_block_278 = 11735322225073324345;
                        }
                        match c2rust_current_block_278 {
                            14666402909811248417 => {}
                            _ => {
                                sqlite3WalkExprList(pWalker, (*pWin).pPartition);
                                sqlite3WalkExprList(pWalker, (*pWin).pOrderBy);
                                sqlite3WalkExpr(pWalker, (*pWin).pFilter);
                                sqlite3WindowLink(pSel, pWin);
                                (*pNC).ncFlags |= NC_HasWin;
                                c2rust_current_block_278 = 145651165234646754;
                            }
                        }
                    } else {
                        let mut pNC2: *mut NameContext = ::core::ptr::null_mut::<
                            NameContext,
                        >();
                        (*pExpr).op = TK_AGG_FUNCTION as u8_0;
                        (*pExpr).op2 = 0 as u8_0;
                        if (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                            != 0 as ::core::ffi::c_uint
                        {
                            sqlite3WalkExpr(pWalker, (*(*pExpr).y.pWin).pFilter);
                        }
                        pNC2 = pNC;
                        while !pNC2.is_null()
                            && sqlite3ReferencesSrcList(pParse, pExpr, (*pNC2).pSrcList)
                                == 0 as ::core::ffi::c_int
                        {
                            (*pExpr).op2 = ((*pExpr).op2 as ::core::ffi::c_uint)
                                .wrapping_add(
                                    (1 as u32_0).wrapping_add((*pNC2).nNestedSelect)
                                        as ::core::ffi::c_uint,
                                ) as u8_0 as u8_0;
                            pNC2 = (*pNC2).pNext;
                        }
                        if !pNC2.is_null() && !pDef.is_null() {
                            (*pExpr).op2 = ((*pExpr).op2 as ::core::ffi::c_uint)
                                .wrapping_add((*pNC2).nNestedSelect as ::core::ffi::c_uint)
                                as u8_0 as u8_0;
                            (*pNC2).ncFlags = ((*pNC2).ncFlags as ::core::ffi::c_uint
                                | (NC_HasAgg as ::core::ffi::c_uint
                                    | ((*pDef).funcFlags as ::core::ffi::c_uint
                                        ^ SQLITE_FUNC_ANYORDER as ::core::ffi::c_uint)
                                        & (SQLITE_FUNC_MINMAX | SQLITE_FUNC_ANYORDER)
                                            as ::core::ffi::c_uint)) as ::core::ffi::c_int;
                        }
                        c2rust_current_block_278 = 145651165234646754;
                    }
                    match c2rust_current_block_278 {
                        14666402909811248417 => {}
                        _ => {
                            (*pNC).ncFlags |= savedAllowFlags;
                            c2rust_current_block_278 = 8466485602140941539;
                        }
                    }
                } else {
                    c2rust_current_block_278 = 8466485602140941539;
                }
                match c2rust_current_block_278 {
                    14666402909811248417 => {}
                    _ => return WRC_Prune

                }
            }
            TK_EXISTS | TK_SELECT | TK_IN => {
                if (*pExpr).flags as ::core::ffi::c_uint
                    & EP_xIsSelect as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                {
                    let mut nRef: ::core::ffi::c_int = (*pNC).nRef;
                    if (*pExpr).op as ::core::ffi::c_int == TK_EXISTS {
                        (*pParse).bHasExists = 1 as u8_0;
                    }
                    if (*pNC).ncFlags & NC_SelfRef != 0 {
                        notValidImpl(
                            pParse,
                            pNC,
                            b"subqueries\0".as_ptr() as *const ::core::ffi::c_char,
                            pExpr,
                            pExpr,
                        );
                    } else {
                        sqlite3WalkSelect(pWalker, (*pExpr).x.pSelect);
                    }
                    if nRef != (*pNC).nRef {
                        (*pExpr).flags
                            |= 0x40 as ::core::ffi::c_int as u32_0
                                as ::core::ffi::c_uint;
                        (*(*pExpr).x.pSelect).selFlags
                            |= SF_Correlated as ::core::ffi::c_uint;
                    }
                    (*pNC).ncFlags |= NC_Subquery;
                }
                c2rust_current_block_278 = 14666402909811248417;
            }
            TK_VARIABLE => {
                if (*pNC).ncFlags
                    & (0x4 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int
                        | 0x20 as ::core::ffi::c_int | 0x8 as ::core::ffi::c_int)
                    != 0 as ::core::ffi::c_int
                {
                    notValidImpl(
                        pParse,
                        pNC,
                        b"parameters\0".as_ptr() as *const ::core::ffi::c_char,
                        pExpr,
                        pExpr,
                    );
                }
                c2rust_current_block_278 = 14666402909811248417;
            }
            TK_IS | TK_ISNOT => {
                let mut pRight_0: *mut Expr = sqlite3ExprSkipCollateAndLikely(
                    (*pExpr).pRight,
                );
                if !pRight_0.is_null()
                    && ((*pRight_0).op as ::core::ffi::c_int == TK_ID
                        || (*pRight_0).op as ::core::ffi::c_int == TK_TRUEFALSE)
                {
                    let mut rc: ::core::ffi::c_int = resolveExprStep(pWalker, pRight_0);
                    if rc == WRC_Abort {
                        return WRC_Abort;
                    }
                    if (*pRight_0).op as ::core::ffi::c_int == TK_TRUEFALSE {
                        (*pExpr).op2 = (*pExpr).op;
                        (*pExpr).op = TK_TRUTH as u8_0;
                        return WRC_Continue;
                    }
                }
                c2rust_current_block_278 = 12972356734185922794;
            }
            TK_BETWEEN | TK_EQ | TK_NE | TK_LT | TK_LE | TK_GT | TK_GE => {
                c2rust_current_block_278 = 12972356734185922794;
            }
            _ => {
                c2rust_current_block_278 = 14666402909811248417;
            }
        }
        match c2rust_current_block_278 {
            12972356734185922794 => {
                let mut nLeft: ::core::ffi::c_int = 0;
                let mut nRight: ::core::ffi::c_int = 0;
                if !((*(*pParse).db).mallocFailed != 0) {
                    nLeft = sqlite3ExprVectorSize((*pExpr).pLeft);
                    if (*pExpr).op as ::core::ffi::c_int == TK_BETWEEN {
                        nRight = sqlite3ExprVectorSize(
                            (*(&raw mut (*(*pExpr).x.pList).a as *mut ExprList_item)
                                .offset(0 as ::core::ffi::c_int as isize))
                                .pExpr,
                        );
                        if nRight == nLeft {
                            nRight = sqlite3ExprVectorSize(
                                (*(&raw mut (*(*pExpr).x.pList).a as *mut ExprList_item)
                                    .offset(1 as ::core::ffi::c_int as isize))
                                    .pExpr,
                            );
                        }
                    } else {
                        nRight = sqlite3ExprVectorSize((*pExpr).pRight);
                    }
                    if nLeft != nRight {
                        sqlite3ErrorMsg(
                            pParse,
                            b"row value misused\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        sqlite3RecordErrorOffsetOfExpr((*pParse).db, pExpr);
                    }
                }
            }
            _ => {}
        }
        return if (*pParse).nErr != 0 { WRC_Abort } else { WRC_Continue };
    }
}
unsafe extern "C" fn resolveAsName(
    mut pParse: *mut Parse,
    mut pEList: *mut ExprList,
    mut pE: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if (*pE).op as ::core::ffi::c_int == TK_ID {
            let mut zCol: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            zCol = (*pE).u.zToken;
            i = 0 as ::core::ffi::c_int;
            while i < (*pEList).nExpr {
                if (*(&raw mut (*pEList).a as *mut ExprList_item).offset(i as isize))
                    .fg
                    .eEName() as ::core::ffi::c_int == ENAME_NAME
                    && sqlite3_stricmp(
                        (*(&raw mut (*pEList).a as *mut ExprList_item)
                            .offset(i as isize))
                            .zEName,
                        zCol,
                    ) == 0 as ::core::ffi::c_int
                {
                    return i + 1 as ::core::ffi::c_int;
                }
                i += 1;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn resolveOrderByTermToExprList(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
    mut pE: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut nc: NameContext = NameContext {
            pParse: ::core::ptr::null_mut::<Parse>(),
            pSrcList: ::core::ptr::null_mut::<SrcList>(),
            uNC: C2Rust_Unnamed_23 {
                pEList: ::core::ptr::null_mut::<ExprList>()

            },
            pNext: ::core::ptr::null_mut::<NameContext>(),
            nRef: 0,
            nNcErr: 0,
            ncFlags: 0,
            nNestedSelect: 0,
            pWinSelect: ::core::ptr::null_mut::<Select>()

        };
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut savedSuppErr: u8_0 = 0;
        pEList = (*pSelect).pEList;
        memset(
            &raw mut nc as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<NameContext>() as size_t,
        );
        nc.pParse = pParse;
        nc.pSrcList = (*pSelect).pSrc;
        nc.uNC.pEList = pEList;
        nc.ncFlags = NC_AllowAgg | NC_UEList | NC_NoSelect;
        nc.nNcErr = 0 as ::core::ffi::c_int;
        db = (*pParse).db;
        savedSuppErr = (*db).suppressErr;
        (*db).suppressErr = 1 as u8_0;
        rc = sqlite3ResolveExprNames(&raw mut nc, pE);
        (*db).suppressErr = savedSuppErr;
        if rc != 0 {
            return 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pEList).nExpr {
            if sqlite3ExprCompare(
                ::core::ptr::null::<Parse>(),
                (*(&raw mut (*pEList).a as *mut ExprList_item).offset(i as isize)).pExpr,
                pE,
                -1 as ::core::ffi::c_int,
            ) < 2 as ::core::ffi::c_int
            {
                return i + 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn resolveOutOfRangeError(
    mut pParse: *mut Parse,
    mut zType: *const ::core::ffi::c_char,
    mut i: ::core::ffi::c_int,
    mut mx: ::core::ffi::c_int,
    mut pError: *mut Expr,
) {
    unsafe {
        sqlite3ErrorMsg(
            pParse,
            b"%r %s BY term out of range - should be between 1 and %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            i,
            zType,
            mx,
        );
        sqlite3RecordErrorOffsetOfExpr((*pParse).db, pError);
    }
}
unsafe extern "C" fn resolveCompoundOrderBy(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pOrderBy: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut moreToDo: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        pOrderBy = (*pSelect).pOrderBy;
        if pOrderBy.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        db = (*pParse).db;
        if (*pOrderBy).nExpr > (*db).aLimit[SQLITE_LIMIT_COLUMN as usize] {
            sqlite3ErrorMsg(
                pParse,
                b"too many terms in ORDER BY clause\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return 1 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pOrderBy).nExpr {
            let ref mut c2rust_fresh0 = (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                .offset(i as isize))
                .fg;
            (*c2rust_fresh0).set_done(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            i += 1;
        }
        (*pSelect).pNext = ::core::ptr::null_mut::<Select>();
        while !(*pSelect).pPrior.is_null() {
            (*(*pSelect).pPrior).pNext = pSelect;
            pSelect = (*pSelect).pPrior;
        }
        while !pSelect.is_null() && moreToDo != 0 {
            let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
            moreToDo = 0 as ::core::ffi::c_int;
            pEList = (*pSelect).pEList;
            i = 0 as ::core::ffi::c_int;
            pItem = &raw mut (*pOrderBy).a as *mut ExprList_item as *mut ExprList_item;
            while i < (*pOrderBy).nExpr {
                let mut iCol: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                let mut pE: *mut Expr = ::core::ptr::null_mut::<Expr>();
                let mut pDup: *mut Expr = ::core::ptr::null_mut::<Expr>();
                if !((*pItem).fg.done() != 0) {
                    pE = sqlite3ExprSkipCollateAndLikely((*pItem).pExpr);
                    if !pE.is_null() {
                        if sqlite3ExprIsInteger(
                            pE,
                            &raw mut iCol,
                            ::core::ptr::null_mut::<Parse>(),
                        ) != 0
                        {
                            if iCol <= 0 as ::core::ffi::c_int || iCol > (*pEList).nExpr
                            {
                                resolveOutOfRangeError(
                                    pParse,
                                    b"ORDER\0".as_ptr() as *const ::core::ffi::c_char,
                                    i + 1 as ::core::ffi::c_int,
                                    (*pEList).nExpr,
                                    pE,
                                );
                                return 1 as ::core::ffi::c_int;
                            }
                        } else {
                            iCol = resolveAsName(pParse, pEList, pE);
                            if iCol == 0 as ::core::ffi::c_int {
                                pDup = sqlite3ExprDup(db, pE, 0 as ::core::ffi::c_int);
                                if (*db).mallocFailed == 0 {
                                    iCol = resolveOrderByTermToExprList(pParse, pSelect, pDup);
                                    if (*pParse).eParseMode as ::core::ffi::c_int
                                        >= PARSE_MODE_RENAME && iCol > 0 as ::core::ffi::c_int
                                    {
                                        resolveOrderByTermToExprList(pParse, pSelect, pE);
                                    }
                                }
                                sqlite3ExprDelete(db, pDup);
                            }
                        }
                        if iCol > 0 as ::core::ffi::c_int {
                            if !((*pParse).eParseMode as ::core::ffi::c_int
                                >= PARSE_MODE_RENAME)
                            {
                                let mut pNew: *mut Expr = sqlite3Expr(
                                    db,
                                    TK_INTEGER,
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                );
                                if pNew.is_null() {
                                    return 1 as ::core::ffi::c_int;
                                }
                                (*pNew).flags |= EP_IntValue as ::core::ffi::c_uint;
                                (*pNew).u.iValue = iCol;
                                if (*pItem).pExpr == pE {
                                    (*pItem).pExpr = pNew;
                                } else {
                                    let mut pParent: *mut Expr = (*pItem).pExpr;
                                    while (*(*pParent).pLeft).op as ::core::ffi::c_int
                                        == TK_COLLATE
                                    {
                                        pParent = (*pParent).pLeft;
                                    }
                                    (*pParent).pLeft = pNew;
                                }
                                sqlite3ExprDelete(db, pE);
                                (*pItem).u.x.iOrderByCol = iCol as u16_0;
                            }
                            (*pItem)
                                .fg
                                .set_done(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        } else {
                            moreToDo = 1 as ::core::ffi::c_int;
                        }
                    }
                }
                i += 1;
                pItem = pItem.offset(1);
            }
            pSelect = (*pSelect).pNext;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pOrderBy).nExpr {
            if (*(&raw mut (*pOrderBy).a as *mut ExprList_item).offset(i as isize))
                .fg
                .done() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                sqlite3ErrorMsg(
                    pParse,
                    b"%r ORDER BY term does not match any column in the result set\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    i + 1 as ::core::ffi::c_int,
                );
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ResolveOrderGroupBy(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
    mut pOrderBy: *mut ExprList,
    mut zType: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        if pOrderBy.is_null() || (*(*pParse).db).mallocFailed as ::core::ffi::c_int != 0
            || (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pOrderBy).nExpr > (*db).aLimit[SQLITE_LIMIT_COLUMN as usize] {
            sqlite3ErrorMsg(
                pParse,
                b"too many terms in %s BY clause\0".as_ptr()
                    as *const ::core::ffi::c_char,
                zType,
            );
            return 1 as ::core::ffi::c_int;
        }
        pEList = (*pSelect).pEList;
        i = 0 as ::core::ffi::c_int;
        pItem = &raw mut (*pOrderBy).a as *mut ExprList_item as *mut ExprList_item;
        while i < (*pOrderBy).nExpr {
            if (*pItem).u.x.iOrderByCol != 0 {
                if (*pItem).u.x.iOrderByCol as ::core::ffi::c_int > (*pEList).nExpr {
                    resolveOutOfRangeError(
                        pParse,
                        zType,
                        i + 1 as ::core::ffi::c_int,
                        (*pEList).nExpr,
                        ::core::ptr::null_mut::<Expr>(),
                    );
                    return 1 as ::core::ffi::c_int;
                }
                resolveAlias(
                    pParse,
                    pEList,
                    (*pItem).u.x.iOrderByCol as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int,
                    (*pItem).pExpr,
                    0 as ::core::ffi::c_int,
                );
            }
            i += 1;
            pItem = pItem.offset(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn resolveRemoveWindowsCb(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint
        {
            let mut pWin: *mut Window = (*pExpr).y.pWin;
            sqlite3WindowUnlinkFromSelect(pWin);
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn windowRemoveExprFromSelect(
    mut pSelect: *mut Select,
    mut pExpr: *mut Expr,
) {
    unsafe {
        if !(*pSelect).pWin.is_null() {
            let mut sWalker: Walker = Walker {
                pParse: ::core::ptr::null_mut::<Parse>(),
                xExprCallback: None,
                xSelectCallback: None,
                xSelectCallback2: None,
                walkerDepth: 0,
                eCode: 0,
                mWFlags: 0,
                u: C2Rust_Unnamed_22 {
                    pNC: ::core::ptr::null_mut::<NameContext>()

                }

            };
            memset(
                &raw mut sWalker as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Walker>() as size_t,
            );
            sWalker.xExprCallback = Some(
                resolveRemoveWindowsCb
                    as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
                >;
            sWalker.u.pSelect = pSelect;
            sqlite3WalkExpr(&raw mut sWalker, pExpr);
        }
    }
}
unsafe extern "C" fn resolveOrderGroupBy(
    mut pNC: *mut NameContext,
    mut pSelect: *mut Select,
    mut pOrderBy: *mut ExprList,
    mut zType: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut iCol: ::core::ffi::c_int = 0;
        let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        let mut pParse: *mut Parse = ::core::ptr::null_mut::<Parse>();
        let mut nResult: ::core::ffi::c_int = 0;
        nResult = (*(*pSelect).pEList).nExpr;
        pParse = (*pNC).pParse;
        let mut c2rust_current_block_21: u64;
        i = 0 as ::core::ffi::c_int;
        pItem = &raw mut (*pOrderBy).a as *mut ExprList_item as *mut ExprList_item;
        while i < (*pOrderBy).nExpr {
            let mut pE: *mut Expr = (*pItem).pExpr;
            let mut pE2: *mut Expr = sqlite3ExprSkipCollateAndLikely(pE);
            if !pE2.is_null() {
                if *zType.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 'G' as i32
                {
                    iCol = resolveAsName(pParse, (*pSelect).pEList, pE2);
                    if iCol > 0 as ::core::ffi::c_int {
                        (*pItem).u.x.iOrderByCol = iCol as u16_0;
                        c2rust_current_block_21 = 715039052867723359;
                    } else {
                        c2rust_current_block_21 = 17965632435239708295;
                    }
                } else {
                    c2rust_current_block_21 = 17965632435239708295;
                }
                match c2rust_current_block_21 {
                    715039052867723359 => {}
                    _ => {
                        if sqlite3ExprIsInteger(
                            pE2,
                            &raw mut iCol,
                            ::core::ptr::null_mut::<Parse>(),
                        ) != 0
                        {
                            if iCol < 1 as ::core::ffi::c_int
                                || iCol > 0xffff as ::core::ffi::c_int
                            {
                                resolveOutOfRangeError(
                                    pParse,
                                    zType,
                                    i + 1 as ::core::ffi::c_int,
                                    nResult,
                                    pE2,
                                );
                                return 1 as ::core::ffi::c_int;
                            }
                            (*pItem).u.x.iOrderByCol = iCol as u16_0;
                        } else {
                            (*pItem).u.x.iOrderByCol = 0 as u16_0;
                            if sqlite3ResolveExprNames(pNC, pE) != 0 {
                                return 1 as ::core::ffi::c_int;
                            }
                            j = 0 as ::core::ffi::c_int;
                            while j < (*(*pSelect).pEList).nExpr {
                                if sqlite3ExprCompare(
                                    ::core::ptr::null::<Parse>(),
                                    pE,
                                    (*(&raw mut (*(*pSelect).pEList).a as *mut ExprList_item)
                                        .offset(j as isize))
                                        .pExpr,
                                    -1 as ::core::ffi::c_int,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    windowRemoveExprFromSelect(pSelect, pE);
                                    (*pItem).u.x.iOrderByCol = (j + 1 as ::core::ffi::c_int)
                                        as u16_0;
                                }
                                j += 1;
                            }
                        }
                    }
                }
            }
            i += 1;
            pItem = pItem.offset(1);
        }
        return sqlite3ResolveOrderGroupBy(pParse, pSelect, pOrderBy, zType);
    }
}
unsafe extern "C" fn resolveSelectStep(
    mut pWalker: *mut Walker,
    mut p: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pOuterNC: *mut NameContext = ::core::ptr::null_mut::<NameContext>();
        let mut sNC: NameContext = NameContext {
            pParse: ::core::ptr::null_mut::<Parse>(),
            pSrcList: ::core::ptr::null_mut::<SrcList>(),
            uNC: C2Rust_Unnamed_23 {
                pEList: ::core::ptr::null_mut::<ExprList>()

            },
            pNext: ::core::ptr::null_mut::<NameContext>(),
            nRef: 0,
            nNcErr: 0,
            ncFlags: 0,
            nNestedSelect: 0,
            pWinSelect: ::core::ptr::null_mut::<Select>()

        };
        let mut isCompound: ::core::ffi::c_int = 0;
        let mut nCompound: ::core::ffi::c_int = 0;
        let mut pParse: *mut Parse = ::core::ptr::null_mut::<Parse>();
        let mut i: ::core::ffi::c_int = 0;
        let mut pGroupBy: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pLeftmost: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if (*p).selFlags as ::core::ffi::c_uint & SF_Resolved as ::core::ffi::c_uint != 0
        {
            return WRC_Prune;
        }
        pOuterNC = (*pWalker).u.pNC;
        pParse = (*pWalker).pParse;
        db = (*pParse).db;
        if (*p).selFlags as ::core::ffi::c_uint & SF_Expanded as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            sqlite3SelectPrep(pParse, p, pOuterNC);
            return if (*pParse).nErr != 0 { WRC_Abort } else { WRC_Prune };
        }
        isCompound = !(*p).pPrior.is_null() as ::core::ffi::c_int;
        nCompound = 0 as ::core::ffi::c_int;
        pLeftmost = p;
        while !p.is_null() {
            (*p).selFlags |= SF_Resolved as ::core::ffi::c_uint;
            memset(
                &raw mut sNC as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<NameContext>() as size_t,
            );
            sNC.pParse = pParse;
            sNC.pWinSelect = p;
            if sqlite3ResolveExprNames(&raw mut sNC, (*p).pLimit) != 0 {
                return WRC_Abort;
            }
            if (*p).selFlags as ::core::ffi::c_uint & SF_Converted as ::core::ffi::c_uint
                != 0
            {
                let mut pSub: *mut Select = ::core::ptr::null_mut::<Select>();
                pSub = (*(*(&raw mut (*(*p).pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .u4
                    .pSubq)
                    .pSelect;
                (*pSub).pOrderBy = (*p).pOrderBy;
                (*p).pOrderBy = ::core::ptr::null_mut::<ExprList>();
            }
            if !pOuterNC.is_null() {
                (*pOuterNC).nNestedSelect = (*pOuterNC).nNestedSelect.wrapping_add(1);
            }
            i = 0 as ::core::ffi::c_int;
            while i < (*(*p).pSrc).nSrc {
                let mut pItem: *mut SrcItem = (&raw mut (*(*p).pSrc).a as *mut SrcItem)
                    .offset(i as isize) as *mut SrcItem;
                if (*pItem).fg.isSubquery() as ::core::ffi::c_int != 0
                    && (*(*(*pItem).u4.pSubq).pSelect).selFlags as ::core::ffi::c_uint
                        & SF_Resolved as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                {
                    let mut nRef: ::core::ffi::c_int = if !pOuterNC.is_null() {
                        (*pOuterNC).nRef
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    let mut zSavedContext: *const ::core::ffi::c_char = (*pParse)
                        .zAuthContext;
                    if !(*pItem).zName.is_null() {
                        (*pParse).zAuthContext = (*pItem).zName;
                    }
                    sqlite3ResolveSelectNames(
                        pParse,
                        (*(*pItem).u4.pSubq).pSelect,
                        pOuterNC,
                    );
                    (*pParse).zAuthContext = zSavedContext;
                    if (*pParse).nErr != 0 {
                        return WRC_Abort;
                    }
                    if !pOuterNC.is_null() {
                        (*pItem)
                            .fg
                            .set_isCorrelated(
                                ((*pOuterNC).nRef > nRef) as ::core::ffi::c_int
                                    as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                    }
                }
                i += 1;
            }
            if !pOuterNC.is_null()
                && (*pOuterNC).nNestedSelect > 0 as ::core::ffi::c_uint
            {
                (*pOuterNC).nNestedSelect = (*pOuterNC).nNestedSelect.wrapping_sub(1);
            }
            sNC.ncFlags = NC_AllowAgg | NC_AllowWin;
            sNC.pSrcList = (*p).pSrc;
            sNC.pNext = pOuterNC;
            if sqlite3ResolveExprListNames(&raw mut sNC, (*p).pEList) != 0 {
                return WRC_Abort;
            }
            sNC.ncFlags &= !NC_AllowWin;
            pGroupBy = (*p).pGroupBy;
            if !pGroupBy.is_null() || sNC.ncFlags & NC_HasAgg != 0 as ::core::ffi::c_int
            {
                (*p).selFlags
                    |= (SF_Aggregate | sNC.ncFlags & (NC_MinMaxAgg | NC_OrderAgg))
                        as ::core::ffi::c_uint;
            } else {
                sNC.ncFlags &= !NC_AllowAgg;
            }
            sNC.uNC.pEList = (*p).pEList;
            sNC.ncFlags |= NC_UEList;
            if !(*p).pHaving.is_null() {
                if (*p).selFlags as ::core::ffi::c_uint
                    & SF_Aggregate as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                {
                    sqlite3ErrorMsg(
                        pParse,
                        b"HAVING clause on a non-aggregate query\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return WRC_Abort;
                }
                if sqlite3ResolveExprNames(&raw mut sNC, (*p).pHaving) != 0 {
                    return WRC_Abort;
                }
            }
            sNC.ncFlags |= NC_Where;
            if sqlite3ResolveExprNames(&raw mut sNC, (*p).pWhere) != 0 {
                return WRC_Abort;
            }
            sNC.ncFlags &= !NC_Where;
            i = 0 as ::core::ffi::c_int;
            while i < (*(*p).pSrc).nSrc {
                let mut pItem_0: *mut SrcItem = (&raw mut (*(*p).pSrc).a as *mut SrcItem)
                    .offset(i as isize) as *mut SrcItem;
                if (*pItem_0).fg.isTabFunc() as ::core::ffi::c_int != 0
                    && sqlite3ResolveExprListNames(&raw mut sNC, (*pItem_0).u1.pFuncArg)
                        != 0
                {
                    return WRC_Abort;
                }
                i += 1;
            }
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                let mut pWin: *mut Window = ::core::ptr::null_mut::<Window>();
                pWin = (*p).pWinDefn;
                while !pWin.is_null() {
                    if sqlite3ResolveExprListNames(&raw mut sNC, (*pWin).pOrderBy) != 0
                        || sqlite3ResolveExprListNames(&raw mut sNC, (*pWin).pPartition)
                            != 0
                    {
                        return WRC_Abort;
                    }
                    pWin = (*pWin).pNextWin;
                }
            }
            sNC.pNext = ::core::ptr::null_mut::<NameContext>();
            sNC.ncFlags |= NC_AllowAgg | NC_AllowWin;
            if (*p).selFlags as ::core::ffi::c_uint & SF_Converted as ::core::ffi::c_uint
                != 0
            {
                let mut pSub_0: *mut Select = ::core::ptr::null_mut::<Select>();
                pSub_0 = (*(*(&raw mut (*(*p).pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .u4
                    .pSubq)
                    .pSelect;
                (*p).pOrderBy = (*pSub_0).pOrderBy;
                (*pSub_0).pOrderBy = ::core::ptr::null_mut::<ExprList>();
            }
            if !(*p).pOrderBy.is_null() && isCompound <= nCompound
                && resolveOrderGroupBy(
                    &raw mut sNC,
                    p,
                    (*p).pOrderBy,
                    b"ORDER\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0
            {
                return WRC_Abort;
            }
            if (*db).mallocFailed != 0 {
                return WRC_Abort;
            }
            sNC.ncFlags &= !NC_AllowWin;
            if !pGroupBy.is_null() {
                let mut pItem_1: *mut ExprList_item = ::core::ptr::null_mut::<
                    ExprList_item,
                >();
                if resolveOrderGroupBy(
                    &raw mut sNC,
                    p,
                    pGroupBy,
                    b"GROUP\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 || (*db).mallocFailed as ::core::ffi::c_int != 0
                {
                    return WRC_Abort;
                }
                i = 0 as ::core::ffi::c_int;
                pItem_1 = &raw mut (*pGroupBy).a as *mut ExprList_item
                    as *mut ExprList_item;
                while i < (*pGroupBy).nExpr {
                    if (*(*pItem_1).pExpr).flags & 0x10 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint
                    {
                        sqlite3ErrorMsg(
                            pParse,
                            b"aggregate functions are not allowed in the GROUP BY clause\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        return WRC_Abort;
                    }
                    i += 1;
                    pItem_1 = pItem_1.offset(1);
                }
            }
            if !(*p).pNext.is_null()
                && (*(*p).pEList).nExpr != (*(*(*p).pNext).pEList).nExpr
            {
                sqlite3SelectWrongNumTermsError(pParse, (*p).pNext);
                return WRC_Abort;
            }
            p = (*p).pPrior;
            nCompound += 1;
        }
        if isCompound != 0 && resolveCompoundOrderBy(pParse, pLeftmost) != 0 {
            return WRC_Abort;
        }
        return WRC_Prune;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ResolveExprNames(
    mut pNC: *mut NameContext,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut savedHasAgg: ::core::ffi::c_int = 0;
        let mut w: Walker = Walker {
            pParse: ::core::ptr::null_mut::<Parse>(),
            xExprCallback: None,
            xSelectCallback: None,
            xSelectCallback2: None,
            walkerDepth: 0,
            eCode: 0,
            mWFlags: 0,
            u: C2Rust_Unnamed_22 {
                pNC: ::core::ptr::null_mut::<NameContext>()

            }

        };
        if pExpr.is_null() {
            return SQLITE_OK;
        }
        savedHasAgg = (*pNC).ncFlags
            & (NC_HasAgg | NC_MinMaxAgg | NC_HasWin | NC_OrderAgg);
        (*pNC).ncFlags &= !(NC_HasAgg | NC_MinMaxAgg | NC_HasWin | NC_OrderAgg);
        w.pParse = (*pNC).pParse;
        w.xExprCallback = Some(
            resolveExprStep
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = (if (*pNC).ncFlags & NC_NoSelect != 0 {
            None
        } else {
            Some(
                resolveSelectStep
                    as unsafe extern "C" fn(
                        *mut Walker,
                        *mut Select,
                    ) -> ::core::ffi::c_int,
            )
        })
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        w.xSelectCallback2 = None;
        w.u.pNC = pNC;
        (*w.pParse).nHeight += (*pExpr).nHeight;
        if sqlite3ExprCheckHeight(w.pParse, (*w.pParse).nHeight) != 0 {
            return SQLITE_ERROR;
        }
        sqlite3WalkExprNN(&raw mut w, pExpr);
        (*w.pParse).nHeight -= (*pExpr).nHeight;
        (*pExpr).flags
            |= ((*pNC).ncFlags
                & (0x10 as ::core::ffi::c_int | 0x8000 as ::core::ffi::c_int)) as u32_0
                as ::core::ffi::c_uint;
        (*pNC).ncFlags |= savedHasAgg;
        return ((*pNC).nNcErr > 0 as ::core::ffi::c_int
            || (*w.pParse).nErr > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ResolveExprListNames(
    mut pNC: *mut NameContext,
    mut pList: *mut ExprList,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut savedHasAgg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut w: Walker = Walker {
            pParse: ::core::ptr::null_mut::<Parse>(),
            xExprCallback: None,
            xSelectCallback: None,
            xSelectCallback2: None,
            walkerDepth: 0,
            eCode: 0,
            mWFlags: 0,
            u: C2Rust_Unnamed_22 {
                pNC: ::core::ptr::null_mut::<NameContext>()

            }

        };
        if pList.is_null() {
            return SQLITE_OK;
        }
        w.pParse = (*pNC).pParse;
        w.xExprCallback = Some(
            resolveExprStep
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = Some(
            resolveSelectStep
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        w.xSelectCallback2 = None;
        w.u.pNC = pNC;
        savedHasAgg = (*pNC).ncFlags
            & (NC_HasAgg | NC_MinMaxAgg | NC_HasWin | NC_OrderAgg);
        (*pNC).ncFlags &= !(NC_HasAgg | NC_MinMaxAgg | NC_HasWin | NC_OrderAgg);
        i = 0 as ::core::ffi::c_int;
        while i < (*pList).nExpr {
            let mut pExpr: *mut Expr = (*(&raw mut (*pList).a as *mut ExprList_item)
                .offset(i as isize))
                .pExpr;
            if !pExpr.is_null() {
                (*w.pParse).nHeight += (*pExpr).nHeight;
                if sqlite3ExprCheckHeight(w.pParse, (*w.pParse).nHeight) != 0 {
                    return SQLITE_ERROR;
                }
                sqlite3WalkExprNN(&raw mut w, pExpr);
                (*w.pParse).nHeight -= (*pExpr).nHeight;
                if (*pNC).ncFlags & (NC_HasAgg | NC_MinMaxAgg | NC_HasWin | NC_OrderAgg)
                    != 0
                {
                    (*pExpr).flags
                        |= ((*pNC).ncFlags
                            & (0x10 as ::core::ffi::c_int
                                | 0x8000 as ::core::ffi::c_int)) as u32_0
                            as ::core::ffi::c_uint;
                    savedHasAgg
                        |= (*pNC).ncFlags
                            & (NC_HasAgg | NC_MinMaxAgg | NC_HasWin | NC_OrderAgg);
                    (*pNC).ncFlags
                        &= !(NC_HasAgg | NC_MinMaxAgg | NC_HasWin | NC_OrderAgg);
                }
                if (*w.pParse).nErr > 0 as ::core::ffi::c_int {
                    return SQLITE_ERROR;
                }
            }
            i += 1;
        }
        (*pNC).ncFlags |= savedHasAgg;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ResolveSelectNames(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pOuterNC: *mut NameContext,
) {
    unsafe {
        let mut w: Walker = Walker {
            pParse: ::core::ptr::null_mut::<Parse>(),
            xExprCallback: None,
            xSelectCallback: None,
            xSelectCallback2: None,
            walkerDepth: 0,
            eCode: 0,
            mWFlags: 0,
            u: C2Rust_Unnamed_22 {
                pNC: ::core::ptr::null_mut::<NameContext>()

            }

        };
        w.xExprCallback = Some(
            resolveExprStep
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = Some(
            resolveSelectStep
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        w.xSelectCallback2 = None;
        w.pParse = pParse;
        w.u.pNC = pOuterNC;
        sqlite3WalkSelect(&raw mut w, p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ResolveSelfReference(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut type_0: ::core::ffi::c_int,
    mut pExpr: *mut Expr,
    mut pList: *mut ExprList,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSrc: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut sNC: NameContext = NameContext {
            pParse: ::core::ptr::null_mut::<Parse>(),
            pSrcList: ::core::ptr::null_mut::<SrcList>(),
            uNC: C2Rust_Unnamed_23 {
                pEList: ::core::ptr::null_mut::<ExprList>()

            },
            pNext: ::core::ptr::null_mut::<NameContext>(),
            nRef: 0,
            nNcErr: 0,
            ncFlags: 0,
            nNestedSelect: 0,
            pWinSelect: ::core::ptr::null_mut::<Select>()

        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut uSrc: C2Rust_Unnamed_24 = C2Rust_Unnamed_24 {
            sSrc: SrcList {
                nSrc: 0,
                nAlloc: 0,
                a: []

            }

        };
        memset(
            &raw mut sNC as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<NameContext>() as size_t,
        );
        memset(
            &raw mut uSrc as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<C2Rust_Unnamed_24>() as size_t,
        );
        pSrc = &raw mut uSrc.sSrc;
        if !pTab.is_null() {
            (*pSrc).nSrc = 1 as ::core::ffi::c_int;
            let ref mut c2rust_fresh3 = (*(&raw mut (*pSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .zName;
            *c2rust_fresh3 = (*pTab).zName;
            let ref mut c2rust_fresh4 = (*(&raw mut (*pSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .pSTab;
            *c2rust_fresh4 = pTab;
            (*(&raw mut (*pSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .iCursor = -1 as ::core::ffi::c_int;
            if (*pTab).pSchema
                != (*(*(*pParse).db).aDb.offset(1 as ::core::ffi::c_int as isize))
                    .pSchema
            {
                type_0 |= NC_FromDDL;
            }
        }
        sNC.pParse = pParse;
        sNC.pSrcList = pSrc;
        sNC.ncFlags = type_0 | NC_IsDDL;
        rc = sqlite3ResolveExprNames(&raw mut sNC, pExpr);
        if rc != SQLITE_OK {
            return rc;
        }
        if !pList.is_null() {
            rc = sqlite3ResolveExprListNames(&raw mut sNC, pList);
        }
        return rc;
    }
}
pub const TK_EXISTS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const TK_IS: ::core::ffi::c_int = 45;
pub const TK_ISNOT: ::core::ffi::c_int = 46;
pub const TK_BETWEEN: ::core::ffi::c_int = 49 as ::core::ffi::c_int;
pub const TK_IN: ::core::ffi::c_int = 50;
pub const TK_ISNULL: ::core::ffi::c_int = 51;
pub const TK_NOTNULL: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const TK_NE: ::core::ffi::c_int = 53;
pub const TK_EQ: ::core::ffi::c_int = 54;
pub const TK_GT: ::core::ffi::c_int = 55;
pub const TK_LE: ::core::ffi::c_int = 56;
pub const TK_LT: ::core::ffi::c_int = 57;
pub const TK_GE: ::core::ffi::c_int = 58;
pub const TK_ID: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const TK_ROW: ::core::ffi::c_int = 76;
pub const TK_TRIGGER: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
pub const TK_COLLATE: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const TK_STRING: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_INSERT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const TK_DELETE: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const TK_SELECT: ::core::ffi::c_int = 139;
pub const TK_DOT: ::core::ffi::c_int = 142;
pub const TK_FLOAT: ::core::ffi::c_int = 154 as ::core::ffi::c_int;
pub const TK_INTEGER: ::core::ffi::c_int = 156 as ::core::ffi::c_int;
pub const TK_VARIABLE: ::core::ffi::c_int = 157;
pub const TK_FILTER: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const TK_COLUMN: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
pub const TK_AGG_FUNCTION: ::core::ffi::c_int = 169 as ::core::ffi::c_int;
pub const TK_TRUEFALSE: ::core::ffi::c_int = 171 as ::core::ffi::c_int;
pub const TK_FUNCTION: ::core::ffi::c_int = 172;
pub const TK_TRUTH: ::core::ffi::c_int = 175 as ::core::ffi::c_int;
pub const TK_REGISTER: ::core::ffi::c_int = 176 as ::core::ffi::c_int;
pub const SQLITE_DqsDDL: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const SQLITE_DqsDML: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
pub const DBFLAG_InternalFunc: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_FUNC_UNLIKELY: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SQLITE_FUNC_CONSTANT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_FUNC_MINMAX: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_SLOCHNG: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_WINDOW: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_INTERNAL: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_DIRECT: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_UNSAFE: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_ANYORDER: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const COLFLAG_GENERATED: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const SQLITE_AFF_INTEGER: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;
pub const TF_HasGenerated: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const TF_NoVisibleRowid: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const EP_IntValue: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const EP_xIsSelect: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const ENAME_NAME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENAME_TAB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ENAME_ROWID: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const JT_LEFT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const JT_RIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const JT_LTORJ: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const NC_AllowAgg: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const NC_PartIdx: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const NC_IsCheck: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const NC_GenCol: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const NC_HasAgg: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const NC_IdxExpr: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const NC_SelfRef: ::core::ffi::c_int = 0x2e as ::core::ffi::c_int;
pub const NC_Subquery: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const NC_UEList: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const NC_UUpsert: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const NC_UBaseReg: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const NC_MinMaxAgg: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const NC_AllowWin: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const NC_HasWin: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const NC_IsDDL: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const NC_FromDDL: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const NC_NoSelect: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const NC_Where: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const NC_OrderAgg: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const SF_Resolved: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SF_Aggregate: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SF_Expanded: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SF_Converted: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const SF_Correlated: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const PARSE_MODE_RENAME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WRC_Continue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WRC_Prune: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WRC_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
