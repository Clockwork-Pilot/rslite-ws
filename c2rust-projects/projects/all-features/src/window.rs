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
    pub type CheckOnCtx;
    pub type CoveringIndexCheck;
    pub type RenameCtx;
    pub type WhereConst;
    pub type IdxCover;
    pub type RefSrcList;
    pub type CCurHint;
    pub type WhereInfo;
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_numeric_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_dup(_: *const sqlite3_value) -> *mut sqlite3_value;
    fn sqlite3_value_free(_: *mut sqlite3_value);
    fn sqlite3_aggregate_context(
        _: *mut sqlite3_context,
        nBytes: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_result_double(_: *mut sqlite3_context, _: ::core::ffi::c_double);
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value);
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
    fn sqlite3VdbeAddOp0(_: *mut Vdbe, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddOp1(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddOp2(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddOp3(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddOp4(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        zP4: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddOp4Int(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeChangeP1(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
        P1: ::core::ffi::c_int,
    );
    fn sqlite3VdbeChangeP5(_: *mut Vdbe, P5: u16_0);
    fn sqlite3VdbeJumpHere(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeAppendP4(
        _: *mut Vdbe,
        pP4: *mut ::core::ffi::c_void,
        p4type: ::core::ffi::c_int,
    );
    fn sqlite3VdbeGetOp(_: *mut Vdbe, _: ::core::ffi::c_int) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3VdbeResolveLabel(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeCurrentAddr(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3WalkExprList(_: *mut Walker, _: *mut ExprList) -> ::core::ffi::c_int;
    fn sqlite3WalkSelect(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3WalkerDepthIncrease(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3WalkerDepthDecrease(_: *mut Walker, _: *mut Select);
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3ErrorToParser(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3GetTempReg(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempReg(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3GetTempRange(_: *mut Parse, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempRange(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
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
    fn sqlite3ExprDelete(_: *mut sqlite3, _: *mut Expr);
    fn sqlite3ExprListAppend(
        _: *mut Parse,
        _: *mut ExprList,
        _: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListDelete(_: *mut sqlite3, _: *mut ExprList);
    fn sqlite3ResultSetOfSelect(
        _: *mut Parse,
        _: *mut Select,
        _: ::core::ffi::c_char,
    ) -> *mut Table;
    fn sqlite3SrcListAppend(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Token,
        _: *mut Token,
    ) -> *mut SrcList;
    fn sqlite3SrcItemAttachSubquery(
        _: *mut Parse,
        _: *mut SrcItem,
        _: *mut Select,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3SrcListAssignCursors(_: *mut Parse, _: *mut SrcList);
    fn sqlite3SelectNew(
        _: *mut Parse,
        _: *mut ExprList,
        _: *mut SrcList,
        _: *mut Expr,
        _: *mut ExprList,
        _: *mut Expr,
        _: *mut ExprList,
        _: u32_0,
        _: *mut Expr,
    ) -> *mut Select;
    fn sqlite3SelectDelete(_: *mut sqlite3, _: *mut Select);
    fn sqlite3WhereEnd(_: *mut WhereInfo);
    fn sqlite3ExprCode(_: *mut Parse, _: *mut Expr, _: ::core::ffi::c_int);
    fn sqlite3ExprCodeExprList(
        _: *mut Parse,
        _: *mut ExprList,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCompare(
        _: *const Parse,
        _: *const Expr,
        _: *const Expr,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprListCompare(
        _: *const ExprList,
        _: *const ExprList,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3AggInfoPersistWalkerInit(_: *mut Walker, _: *mut Parse);
    fn sqlite3GetVdbe(_: *mut Parse) -> *mut Vdbe;
    fn sqlite3ExprIsConstant(_: *mut Parse, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3ExprIsInteger(
        _: *const Expr,
        _: *mut ::core::ffi::c_int,
        _: *mut Parse,
    ) -> ::core::ffi::c_int;
    fn sqlite3MayAbort(_: *mut Parse);
    fn sqlite3ExprDup(
        _: *mut sqlite3,
        _: *const Expr,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3ExprListDup(
        _: *mut sqlite3,
        _: *const ExprList,
        _: ::core::ffi::c_int,
    ) -> *mut ExprList;
    fn sqlite3InsertBuiltinFuncs(_: *mut FuncDef, _: ::core::ffi::c_int);
    fn sqlite3ExprNNCollSeq(pParse: *mut Parse, pExpr: *const Expr) -> *mut CollSeq;
    fn sqlite3ExprSkipCollateAndLikely(_: *mut Expr) -> *mut Expr;
    fn sqlite3ValueFree(_: *mut sqlite3_value);
    fn sqlite3ValueFromExpr(
        _: *mut sqlite3,
        _: *const Expr,
        _: u8_0,
        _: u8_0,
        _: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3RenameExprUnmap(_: *mut Parse, _: *mut Expr);
    fn sqlite3KeyInfoFromExprList(
        _: *mut Parse,
        _: *mut ExprList,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut KeyInfo;
    fn sqlite3ParserAddCleanup(
        _: *mut Parse,
        _: Option<unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> ()>,
        _: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
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
pub struct WindowRewrite {
    pub pWin: *mut Window,
    pub pSrc: *mut SrcList,
    pub pSub: *mut ExprList,
    pub pTab: *mut Table,
    pub pSubSelect: *mut Select

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
pub struct WindowCsrAndReg {
    pub csr: ::core::ffi::c_int,
    pub reg: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WindowCodeArg {
    pub pParse: *mut Parse,
    pub pMWin: *mut Window,
    pub pVdbe: *mut Vdbe,
    pub addrGosub: ::core::ffi::c_int,
    pub regGosub: ::core::ffi::c_int,
    pub regArg: ::core::ffi::c_int,
    pub eDelete: ::core::ffi::c_int,
    pub regRowid: ::core::ffi::c_int,
    pub start: WindowCsrAndReg,
    pub current: WindowCsrAndReg,
    pub end: WindowCsrAndReg

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WindowUpdate {
    pub zFunc: *const ::core::ffi::c_char,
    pub eFrmType: ::core::ffi::c_int,
    pub eStart: ::core::ffi::c_int,
    pub eEnd: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NthValueCtx {
    pub nStep: i64_0,
    pub pValue: *mut sqlite3_value

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LastValueCtx {
    pub pVal: *mut sqlite3_value,
    pub nVal: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NtileCtx {
    pub nTotal: i64_0,
    pub nParam: i64_0,
    pub iRow: i64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallCount {
    pub nValue: i64_0,
    pub nStep: i64_0,
    pub nTotal: i64_0

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_SUBTYPE: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const OP_Goto: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OP_Gosub: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const OP_MustBeInt: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const OP_Jump: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const OP_IfNot: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const OP_SeekGE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const OP_SeekRowid: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const OP_Last: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const OP_IsNull: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const OP_NotNull: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const OP_Ne: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const OP_Eq: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const OP_Gt: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
pub const OP_Le: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const OP_Lt: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
pub const OP_Ge: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
pub const OP_IfPos: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const OP_Return: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const OP_Halt: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const OP_Copy: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
pub const OP_SCopy: ::core::ffi::c_int = 82 as ::core::ffi::c_int;
pub const OP_CollSeq: ::core::ffi::c_int = 86 as ::core::ffi::c_int;
pub const OP_AddImm: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
pub const OP_Compare: ::core::ffi::c_int = 91 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_MakeRecord: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const OP_Add: ::core::ffi::c_int = 107 as ::core::ffi::c_int;
pub const OP_Subtract: ::core::ffi::c_int = 108 as ::core::ffi::c_int;
pub const OP_OpenDup: ::core::ffi::c_int = 116 as ::core::ffi::c_int;
pub const OP_String8: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const OP_OpenEphemeral: ::core::ffi::c_int = 119 as ::core::ffi::c_int;
pub const OP_NewRowid: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const OP_Insert: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const OP_Delete: ::core::ffi::c_int = 131 as ::core::ffi::c_int;
pub const OP_Rowid: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const OP_IdxInsert: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const OP_ResetSorter: ::core::ffi::c_int = 147 as ::core::ffi::c_int;
pub const OP_AggInverse: ::core::ffi::c_int = 162 as ::core::ffi::c_int;
pub const OP_AggStep: ::core::ffi::c_int = 163 as ::core::ffi::c_int;
pub const OP_AggValue: ::core::ffi::c_int = 165 as ::core::ffi::c_int;
pub const OP_AggFinal: ::core::ffi::c_int = 166 as ::core::ffi::c_int;
unsafe extern "C" fn row_numberStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut i64_0 = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<i64_0>() as ::core::ffi::c_int,
        ) as *mut i64_0;
        if !p.is_null() {
            *p += 1;
        }
    }
}
unsafe extern "C" fn row_numberValueFunc(mut pCtx: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut i64_0 = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<i64_0>() as ::core::ffi::c_int,
        ) as *mut i64_0;
        sqlite3_result_int64(pCtx, if !p.is_null() { *p } else { 0 as sqlite3_int64 });
    }
}
unsafe extern "C" fn dense_rankStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut CallCount = ::core::ptr::null_mut::<CallCount>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<CallCount>() as ::core::ffi::c_int,
        ) as *mut CallCount;
        if !p.is_null() {
            (*p).nStep = 1 as i64_0;
        }
    }
}
unsafe extern "C" fn dense_rankValueFunc(mut pCtx: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut CallCount = ::core::ptr::null_mut::<CallCount>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<CallCount>() as ::core::ffi::c_int,
        ) as *mut CallCount;
        if !p.is_null() {
            if (*p).nStep != 0 {
                (*p).nValue += 1;
                (*p).nStep = 0 as i64_0;
            }
            sqlite3_result_int64(pCtx, (*p).nValue as sqlite3_int64);
        }
    }
}
unsafe extern "C" fn nth_valueStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut p: *mut NthValueCtx = ::core::ptr::null_mut::<NthValueCtx>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<NthValueCtx>() as ::core::ffi::c_int,
        ) as *mut NthValueCtx;
        if !p.is_null() {
            let mut iVal: i64_0 = 0;
            match sqlite3_value_numeric_type(
                *apArg.offset(1 as ::core::ffi::c_int as isize),
            ) {
                SQLITE_INTEGER => {
                    iVal = sqlite3_value_int64(
                        *apArg.offset(1 as ::core::ffi::c_int as isize),
                    ) as i64_0;
                    c2rust_current_block = 6937071982253665452;
                }
                SQLITE_FLOAT => {
                    let mut fVal: ::core::ffi::c_double = sqlite3_value_double(
                        *apArg.offset(1 as ::core::ffi::c_int as isize),
                    );
                    if fVal as i64_0 as ::core::ffi::c_double != fVal {
                        c2rust_current_block = 9754664151470290522;
                    } else {
                        iVal = fVal as i64_0;
                        c2rust_current_block = 6937071982253665452;
                    }
                }
                _ => {
                    c2rust_current_block = 9754664151470290522;
                }
            }
            match c2rust_current_block {
                6937071982253665452 => {
                    if iVal <= 0 as ::core::ffi::c_longlong {
                        c2rust_current_block = 9754664151470290522;
                    } else {
                        (*p).nStep += 1;
                        if iVal == (*p).nStep {
                            (*p).pValue = sqlite3_value_dup(
                                *apArg.offset(0 as ::core::ffi::c_int as isize),
                            );
                            if (*p).pValue.is_null() {
                                sqlite3_result_error_nomem(pCtx);
                            }
                        }
                        c2rust_current_block = 8831408221741692167;
                    }
                }
                _ => {}
            }
            match c2rust_current_block {
                8831408221741692167 => {}
                _ => {
                    sqlite3_result_error(
                        pCtx,
                        b"second argument to nth_value must be a positive integer\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                    return;
                }
            }
        }
    }
}
unsafe extern "C" fn nth_valueFinalizeFunc(mut pCtx: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut NthValueCtx = ::core::ptr::null_mut::<NthValueCtx>();
        p = sqlite3_aggregate_context(pCtx, 0 as ::core::ffi::c_int) as *mut NthValueCtx;
        if !p.is_null() && !(*p).pValue.is_null() {
            sqlite3_result_value(pCtx, (*p).pValue);
            sqlite3_value_free((*p).pValue);
            (*p).pValue = ::core::ptr::null_mut::<sqlite3_value>();
        }
    }
}
unsafe extern "C" fn first_valueStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut NthValueCtx = ::core::ptr::null_mut::<NthValueCtx>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<NthValueCtx>() as ::core::ffi::c_int,
        ) as *mut NthValueCtx;
        if !p.is_null() && (*p).pValue.is_null() {
            (*p).pValue = sqlite3_value_dup(
                *apArg.offset(0 as ::core::ffi::c_int as isize),
            );
            if (*p).pValue.is_null() {
                sqlite3_result_error_nomem(pCtx);
            }
        }
    }
}
unsafe extern "C" fn first_valueFinalizeFunc(mut pCtx: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut NthValueCtx = ::core::ptr::null_mut::<NthValueCtx>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<NthValueCtx>() as ::core::ffi::c_int,
        ) as *mut NthValueCtx;
        if !p.is_null() && !(*p).pValue.is_null() {
            sqlite3_result_value(pCtx, (*p).pValue);
            sqlite3_value_free((*p).pValue);
            (*p).pValue = ::core::ptr::null_mut::<sqlite3_value>();
        }
    }
}
unsafe extern "C" fn rankStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut CallCount = ::core::ptr::null_mut::<CallCount>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<CallCount>() as ::core::ffi::c_int,
        ) as *mut CallCount;
        if !p.is_null() {
            (*p).nStep += 1;
            if (*p).nValue == 0 as ::core::ffi::c_longlong {
                (*p).nValue = (*p).nStep;
            }
        }
    }
}
unsafe extern "C" fn rankValueFunc(mut pCtx: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut CallCount = ::core::ptr::null_mut::<CallCount>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<CallCount>() as ::core::ffi::c_int,
        ) as *mut CallCount;
        if !p.is_null() {
            sqlite3_result_int64(pCtx, (*p).nValue as sqlite3_int64);
            (*p).nValue = 0 as i64_0;
        }
    }
}
unsafe extern "C" fn percent_rankStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut CallCount = ::core::ptr::null_mut::<CallCount>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<CallCount>() as ::core::ffi::c_int,
        ) as *mut CallCount;
        if !p.is_null() {
            (*p).nTotal += 1;
        }
    }
}
unsafe extern "C" fn percent_rankInvFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut CallCount = ::core::ptr::null_mut::<CallCount>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<CallCount>() as ::core::ffi::c_int,
        ) as *mut CallCount;
        (*p).nStep += 1;
    }
}
unsafe extern "C" fn percent_rankValueFunc(mut pCtx: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut CallCount = ::core::ptr::null_mut::<CallCount>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<CallCount>() as ::core::ffi::c_int,
        ) as *mut CallCount;
        if !p.is_null() {
            (*p).nValue = (*p).nStep;
            if (*p).nTotal > 1 as ::core::ffi::c_longlong {
                let mut r: ::core::ffi::c_double = (*p).nValue as ::core::ffi::c_double
                    / ((*p).nTotal as ::core::ffi::c_longlong
                        - 1 as ::core::ffi::c_longlong) as ::core::ffi::c_double;
                sqlite3_result_double(pCtx, r);
            } else {
                sqlite3_result_double(pCtx, 0.0f64);
            }
        }
    }
}
unsafe extern "C" fn cume_distStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut CallCount = ::core::ptr::null_mut::<CallCount>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<CallCount>() as ::core::ffi::c_int,
        ) as *mut CallCount;
        if !p.is_null() {
            (*p).nTotal += 1;
        }
    }
}
unsafe extern "C" fn cume_distInvFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut CallCount = ::core::ptr::null_mut::<CallCount>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<CallCount>() as ::core::ffi::c_int,
        ) as *mut CallCount;
        (*p).nStep += 1;
    }
}
unsafe extern "C" fn cume_distValueFunc(mut pCtx: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut CallCount = ::core::ptr::null_mut::<CallCount>();
        p = sqlite3_aggregate_context(pCtx, 0 as ::core::ffi::c_int) as *mut CallCount;
        if !p.is_null() {
            let mut r: ::core::ffi::c_double = (*p).nStep as ::core::ffi::c_double
                / (*p).nTotal as ::core::ffi::c_double;
            sqlite3_result_double(pCtx, r);
        }
    }
}
unsafe extern "C" fn ntileStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut NtileCtx = ::core::ptr::null_mut::<NtileCtx>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<NtileCtx>() as ::core::ffi::c_int,
        ) as *mut NtileCtx;
        if !p.is_null() {
            if (*p).nTotal == 0 as ::core::ffi::c_longlong {
                (*p).nParam = sqlite3_value_int64(
                    *apArg.offset(0 as ::core::ffi::c_int as isize),
                ) as i64_0;
                if (*p).nParam <= 0 as ::core::ffi::c_longlong {
                    sqlite3_result_error(
                        pCtx,
                        b"argument of ntile must be a positive integer\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                }
            }
            (*p).nTotal += 1;
        }
    }
}
unsafe extern "C" fn ntileInvFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut NtileCtx = ::core::ptr::null_mut::<NtileCtx>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<NtileCtx>() as ::core::ffi::c_int,
        ) as *mut NtileCtx;
        (*p).iRow += 1;
    }
}
unsafe extern "C" fn ntileValueFunc(mut pCtx: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut NtileCtx = ::core::ptr::null_mut::<NtileCtx>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<NtileCtx>() as ::core::ffi::c_int,
        ) as *mut NtileCtx;
        if !p.is_null() && (*p).nParam > 0 as ::core::ffi::c_longlong {
            let mut nSize: ::core::ffi::c_int = ((*p).nTotal / (*p).nParam)
                as ::core::ffi::c_int;
            if nSize == 0 as ::core::ffi::c_int {
                sqlite3_result_int64(
                    pCtx,
                    (*p).iRow as sqlite3_int64 + 1 as sqlite3_int64,
                );
            } else {
                let mut nLarge: i64_0 = (*p).nTotal - (*p).nParam * nSize as i64_0;
                let mut iSmall: i64_0 = nLarge
                    * (nSize + 1 as ::core::ffi::c_int) as i64_0;
                let mut iRow: i64_0 = (*p).iRow;
                if iRow < iSmall {
                    sqlite3_result_int64(
                        pCtx,
                        1 as sqlite3_int64
                            + iRow as sqlite3_int64
                                / (nSize + 1 as ::core::ffi::c_int) as sqlite3_int64,
                    );
                } else {
                    sqlite3_result_int64(
                        pCtx,
                        1 as sqlite3_int64 + nLarge as sqlite3_int64
                            + (iRow as sqlite3_int64 - iSmall as sqlite3_int64)
                                / nSize as sqlite3_int64,
                    );
                }
            }
        }
    }
}
unsafe extern "C" fn last_valueStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut LastValueCtx = ::core::ptr::null_mut::<LastValueCtx>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<LastValueCtx>() as ::core::ffi::c_int,
        ) as *mut LastValueCtx;
        if !p.is_null() {
            sqlite3_value_free((*p).pVal);
            (*p).pVal = sqlite3_value_dup(
                *apArg.offset(0 as ::core::ffi::c_int as isize),
            );
            if (*p).pVal.is_null() {
                sqlite3_result_error_nomem(pCtx);
            } else {
                (*p).nVal += 1;
            }
        }
    }
}
unsafe extern "C" fn last_valueInvFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut LastValueCtx = ::core::ptr::null_mut::<LastValueCtx>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<LastValueCtx>() as ::core::ffi::c_int,
        ) as *mut LastValueCtx;
        if !p.is_null() {
            (*p).nVal -= 1;
            if (*p).nVal == 0 as ::core::ffi::c_int {
                sqlite3_value_free((*p).pVal);
                (*p).pVal = ::core::ptr::null_mut::<sqlite3_value>();
            }
        }
    }
}
unsafe extern "C" fn last_valueValueFunc(mut pCtx: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut LastValueCtx = ::core::ptr::null_mut::<LastValueCtx>();
        p = sqlite3_aggregate_context(pCtx, 0 as ::core::ffi::c_int)
            as *mut LastValueCtx;
        if !p.is_null() && !(*p).pVal.is_null() {
            sqlite3_result_value(pCtx, (*p).pVal);
        }
    }
}
unsafe extern "C" fn last_valueFinalizeFunc(mut pCtx: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut LastValueCtx = ::core::ptr::null_mut::<LastValueCtx>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<LastValueCtx>() as ::core::ffi::c_int,
        ) as *mut LastValueCtx;
        if !p.is_null() && !(*p).pVal.is_null() {
            sqlite3_result_value(pCtx, (*p).pVal);
            sqlite3_value_free((*p).pVal);
            (*p).pVal = ::core::ptr::null_mut::<sqlite3_value>();
        }
    }
}
static mut row_numberName: [::core::ffi::c_char; 11] = unsafe {
    ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"row_number\0")
};
static mut dense_rankName: [::core::ffi::c_char; 11] = unsafe {
    ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"dense_rank\0")
};
static mut rankName: [::core::ffi::c_char; 5] = unsafe {
    ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"rank\0")
};
static mut percent_rankName: [::core::ffi::c_char; 13] = unsafe {
    ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"percent_rank\0")
};
static mut cume_distName: [::core::ffi::c_char; 10] = unsafe {
    ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"cume_dist\0")
};
static mut ntileName: [::core::ffi::c_char; 6] = unsafe {
    ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"ntile\0")
};
static mut last_valueName: [::core::ffi::c_char; 11] = unsafe {
    ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"last_value\0")
};
static mut nth_valueName: [::core::ffi::c_char; 10] = unsafe {
    ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"nth_value\0")
};
static mut first_valueName: [::core::ffi::c_char; 12] = unsafe {
    ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"first_value\0")
};
static mut leadName: [::core::ffi::c_char; 5] = unsafe {
    ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"lead\0")
};
static mut lagName: [::core::ffi::c_char; 4] = unsafe {
    ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"lag\0")
};
unsafe extern "C" fn noopStepFunc(
    mut p: *mut sqlite3_context,
    mut n: ::core::ffi::c_int,
    mut a: *mut *mut sqlite3_value,
) {}
unsafe extern "C" fn noopValueFunc(mut p: *mut sqlite3_context) {}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowFunctions() {
    unsafe {
        static mut aWindowFuncs: [FuncDef; 15] = unsafe {
            [
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        row_numberStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        row_numberValueFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        row_numberValueFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const row_numberName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        dense_rankStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        dense_rankValueFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        dense_rankValueFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const dense_rankName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        rankStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        rankValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        rankValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const rankName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        percent_rankStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        percent_rankValueFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        percent_rankValueFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        percent_rankInvFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const percent_rankName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 0 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        cume_distStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        cume_distValueFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        cume_distValueFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        cume_distInvFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const cume_distName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        ntileStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        ntileValueFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        ntileValueFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        ntileInvFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const ntileName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        last_valueStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        last_valueFinalizeFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        last_valueValueFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        last_valueInvFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const last_valueName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        nth_valueStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        nth_valueFinalizeFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const nth_valueName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        first_valueStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        first_valueFinalizeFunc
                            as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const first_valueName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const leadName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const leadName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 3 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const leadName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 1 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const lagName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const lagName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 3 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_UTF8 | SQLITE_FUNC_WINDOW
                        | 0 as ::core::ffi::c_int) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xValue: Some(
                        noopValueFunc as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                    ),
                    xInverse: Some(
                        noopStepFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    zName: &raw const lagName as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
            ]
        };
        sqlite3InsertBuiltinFuncs(
            &raw mut aWindowFuncs as *mut FuncDef,
            (::core::mem::size_of::<[FuncDef; 15]>() as usize)
                .wrapping_div(::core::mem::size_of::<FuncDef>() as usize)
                as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn windowFind(
    mut pParse: *mut Parse,
    mut pList: *mut Window,
    mut zName: *const ::core::ffi::c_char,
) -> *mut Window {
    unsafe {
        let mut p: *mut Window = ::core::ptr::null_mut::<Window>();
        p = pList;
        while !p.is_null() {
            if sqlite3StrICmp((*p).zName, zName) == 0 as ::core::ffi::c_int {
                break;
            }
            p = (*p).pNextWin;
        }
        if p.is_null() {
            sqlite3ErrorMsg(
                pParse,
                b"no such window: %s\0".as_ptr() as *const ::core::ffi::c_char,
                zName,
            );
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowUpdate(
    mut pParse: *mut Parse,
    mut pList: *mut Window,
    mut pWin: *mut Window,
    mut pFunc: *mut FuncDef,
) {
    unsafe {
        if !(*pWin).zName.is_null()
            && (*pWin).eFrmType as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            let mut p: *mut Window = windowFind(pParse, pList, (*pWin).zName);
            if p.is_null() {
                return;
            }
            (*pWin).pPartition = sqlite3ExprListDup(
                (*pParse).db,
                (*p).pPartition,
                0 as ::core::ffi::c_int,
            );
            (*pWin).pOrderBy = sqlite3ExprListDup(
                (*pParse).db,
                (*p).pOrderBy,
                0 as ::core::ffi::c_int,
            );
            (*pWin).pStart = sqlite3ExprDup(
                (*pParse).db,
                (*p).pStart,
                0 as ::core::ffi::c_int,
            );
            (*pWin).pEnd = sqlite3ExprDup(
                (*pParse).db,
                (*p).pEnd,
                0 as ::core::ffi::c_int,
            );
            (*pWin).eStart = (*p).eStart;
            (*pWin).eEnd = (*p).eEnd;
            (*pWin).eFrmType = (*p).eFrmType;
            (*pWin).eExclude = (*p).eExclude;
        } else {
            sqlite3WindowChain(pParse, pWin, pList);
        }
        if (*pWin).eFrmType as ::core::ffi::c_int == TK_RANGE
            && (!(*pWin).pStart.is_null() || !(*pWin).pEnd.is_null())
            && ((*pWin).pOrderBy.is_null()
                || (*(*pWin).pOrderBy).nExpr != 1 as ::core::ffi::c_int)
        {
            sqlite3ErrorMsg(
                pParse,
                b"RANGE with offset PRECEDING/FOLLOWING requires one ORDER BY expression\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        } else if (*pFunc).funcFlags & SQLITE_FUNC_WINDOW as u32_0 != 0 {
            let mut db: *mut sqlite3 = (*pParse).db;
            if !(*pWin).pFilter.is_null() {
                sqlite3ErrorMsg(
                    pParse,
                    b"FILTER clause may only be used with aggregate window functions\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                let mut aUp: [WindowUpdate; 8] = [
                    WindowUpdate {
                        zFunc: &raw const row_numberName as *const ::core::ffi::c_char,
                        eFrmType: TK_ROWS,
                        eStart: TK_UNBOUNDED,
                        eEnd: TK_CURRENT

                    },
                    WindowUpdate {
                        zFunc: &raw const dense_rankName as *const ::core::ffi::c_char,
                        eFrmType: TK_RANGE,
                        eStart: TK_UNBOUNDED,
                        eEnd: TK_CURRENT

                    },
                    WindowUpdate {
                        zFunc: &raw const rankName as *const ::core::ffi::c_char,
                        eFrmType: TK_RANGE,
                        eStart: TK_UNBOUNDED,
                        eEnd: TK_CURRENT

                    },
                    WindowUpdate {
                        zFunc: &raw const percent_rankName as *const ::core::ffi::c_char,
                        eFrmType: TK_GROUPS,
                        eStart: TK_CURRENT,
                        eEnd: TK_UNBOUNDED

                    },
                    WindowUpdate {
                        zFunc: &raw const cume_distName as *const ::core::ffi::c_char,
                        eFrmType: TK_GROUPS,
                        eStart: TK_FOLLOWING,
                        eEnd: TK_UNBOUNDED

                    },
                    WindowUpdate {
                        zFunc: &raw const ntileName as *const ::core::ffi::c_char,
                        eFrmType: TK_ROWS,
                        eStart: TK_CURRENT,
                        eEnd: TK_UNBOUNDED

                    },
                    WindowUpdate {
                        zFunc: &raw const leadName as *const ::core::ffi::c_char,
                        eFrmType: TK_ROWS,
                        eStart: TK_UNBOUNDED,
                        eEnd: TK_UNBOUNDED

                    },
                    WindowUpdate {
                        zFunc: &raw const lagName as *const ::core::ffi::c_char,
                        eFrmType: TK_ROWS,
                        eStart: TK_UNBOUNDED,
                        eEnd: TK_CURRENT

                    },
                ];
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while i
                    < (::core::mem::size_of::<[WindowUpdate; 8]>() as usize)
                        .wrapping_div(::core::mem::size_of::<WindowUpdate>() as usize)
                        as ::core::ffi::c_int
                {
                    if (*pFunc).zName == aUp[i as usize].zFunc {
                        sqlite3ExprDelete(db, (*pWin).pStart);
                        sqlite3ExprDelete(db, (*pWin).pEnd);
                        (*pWin).pStart = ::core::ptr::null_mut::<Expr>();
                        (*pWin).pEnd = (*pWin).pStart;
                        (*pWin).eFrmType = aUp[i as usize].eFrmType as u8_0;
                        (*pWin).eStart = aUp[i as usize].eStart as u8_0;
                        (*pWin).eEnd = aUp[i as usize].eEnd as u8_0;
                        (*pWin).eExclude = 0 as u8_0;
                        if (*pWin).eStart as ::core::ffi::c_int == TK_FOLLOWING {
                            (*pWin).pStart = sqlite3Expr(
                                db,
                                TK_INTEGER,
                                b"1\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        (*pWin).pWFunc = pFunc;
    }
}
unsafe extern "C" fn selectWindowRewriteExprCb(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut WindowRewrite = (*pWalker).u.pRewrite as *mut WindowRewrite;
        let mut pParse: *mut Parse = (*pWalker).pParse;
        if !(*p).pSubSelect.is_null() {
            if (*pExpr).op as ::core::ffi::c_int != TK_COLUMN {
                return WRC_Continue
            } else {
                let mut nSrc: ::core::ffi::c_int = (*(*p).pSrc).nSrc;
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while i < nSrc {
                    if (*pExpr).iTable
                        == (*(&raw mut (*(*p).pSrc).a as *mut SrcItem)
                            .offset(i as isize))
                            .iCursor
                    {
                        break;
                    }
                    i += 1;
                }
                if i == nSrc {
                    return WRC_Continue;
                }
            }
        }
        let mut c2rust_current_block_46: u64;
        match (*pExpr).op as ::core::ffi::c_int {
            TK_FUNCTION => {
                if !((*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                    != 0 as u32_0)
                {
                    c2rust_current_block_46 = 10150597327160359210;
                } else {
                    let mut pWin: *mut Window = ::core::ptr::null_mut::<Window>();
                    pWin = (*p).pWin;
                    while !pWin.is_null() {
                        if (*pExpr).y.pWin == pWin {
                            return WRC_Prune;
                        }
                        pWin = (*pWin).pNextWin;
                    }
                    c2rust_current_block_46 = 1109700713171191020;
                }
            }
            TK_IF_NULL_ROW | TK_AGG_FUNCTION | TK_COLUMN => {
                c2rust_current_block_46 = 1109700713171191020;
            }
            _ => {
                c2rust_current_block_46 = 10150597327160359210;
            }
        }
        match c2rust_current_block_46 {
            1109700713171191020 => {
                let mut iCol: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                if (*(*pParse).db).mallocFailed != 0 {
                    return WRC_Abort;
                }
                if !(*p).pSub.is_null() {
                    let mut i_0: ::core::ffi::c_int = 0;
                    i_0 = 0 as ::core::ffi::c_int;
                    while i_0 < (*(*p).pSub).nExpr {
                        if 0 as ::core::ffi::c_int
                            == sqlite3ExprCompare(
                                ::core::ptr::null::<Parse>(),
                                (*(&raw mut (*(*p).pSub).a as *mut ExprList_item)
                                    .offset(i_0 as isize))
                                    .pExpr,
                                pExpr,
                                -1 as ::core::ffi::c_int,
                            )
                        {
                            iCol = i_0;
                            break;
                        } else {
                            i_0 += 1;
                        }
                    }
                }
                if iCol < 0 as ::core::ffi::c_int {
                    let mut pDup: *mut Expr = sqlite3ExprDup(
                        (*pParse).db,
                        pExpr,
                        0 as ::core::ffi::c_int,
                    );
                    if !pDup.is_null()
                        && (*pDup).op as ::core::ffi::c_int == TK_AGG_FUNCTION
                    {
                        (*pDup).op = TK_FUNCTION as u8_0;
                    }
                    (*p).pSub = sqlite3ExprListAppend(pParse, (*p).pSub, pDup);
                }
                if !(*p).pSub.is_null() {
                    let mut f: ::core::ffi::c_int = ((*pExpr).flags
                        & EP_Collate as u32_0) as ::core::ffi::c_int;
                    (*pExpr).flags = ((*pExpr).flags as ::core::ffi::c_uint
                        | 0x8000000 as ::core::ffi::c_int as u32_0
                            as ::core::ffi::c_uint) as u32_0;
                    sqlite3ExprDelete((*pParse).db, pExpr);
                    (*pExpr).flags = ((*pExpr).flags as ::core::ffi::c_uint
                        & !(0x8000000 as ::core::ffi::c_int as u32_0)
                            as ::core::ffi::c_uint) as u32_0;
                    memset(
                        pExpr as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<Expr>() as size_t,
                    );
                    (*pExpr).op = TK_COLUMN as u8_0;
                    (*pExpr).iColumn = (if iCol < 0 as ::core::ffi::c_int {
                        (*(*p).pSub).nExpr - 1 as ::core::ffi::c_int
                    } else {
                        iCol
                    }) as ynVar;
                    (*pExpr).iTable = (*(*p).pWin).iEphCsr;
                    (*pExpr).y.pTab = (*p).pTab;
                    (*pExpr).flags = f as u32_0;
                }
                if (*(*pParse).db).mallocFailed != 0 {
                    return WRC_Abort;
                }
            }
            _ => {}
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn selectWindowRewriteSelectCb(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut WindowRewrite = (*pWalker).u.pRewrite as *mut WindowRewrite;
        let mut pSave: *mut Select = (*p).pSubSelect;
        if pSave == pSelect {
            return WRC_Continue
        } else {
            (*p).pSubSelect = pSelect;
            sqlite3WalkSelect(pWalker, pSelect);
            (*p).pSubSelect = pSave;
        }
        return WRC_Prune;
    }
}
unsafe extern "C" fn selectWindowRewriteEList(
    mut pParse: *mut Parse,
    mut pWin: *mut Window,
    mut pSrc: *mut SrcList,
    mut pEList: *mut ExprList,
    mut pTab: *mut Table,
    mut ppSub: *mut *mut ExprList,
) {
    unsafe {
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
        let mut sRewrite: WindowRewrite = WindowRewrite {
            pWin: ::core::ptr::null_mut::<Window>(),
            pSrc: ::core::ptr::null_mut::<SrcList>(),
            pSub: ::core::ptr::null_mut::<ExprList>(),
            pTab: ::core::ptr::null_mut::<Table>(),
            pSubSelect: ::core::ptr::null_mut::<Select>()

        };
        memset(
            &raw mut sWalker as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        memset(
            &raw mut sRewrite as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<WindowRewrite>() as size_t,
        );
        sRewrite.pSub = *ppSub;
        sRewrite.pWin = pWin;
        sRewrite.pSrc = pSrc;
        sRewrite.pTab = pTab;
        sWalker.pParse = pParse;
        sWalker.xExprCallback = Some(
            selectWindowRewriteExprCb
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        sWalker.xSelectCallback = Some(
            selectWindowRewriteSelectCb
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        sWalker.u.pRewrite = &raw mut sRewrite as *mut WindowRewrite;
        sqlite3WalkExprList(&raw mut sWalker, pEList);
        *ppSub = sRewrite.pSub;
    }
}
unsafe extern "C" fn exprListAppendList(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut pAppend: *mut ExprList,
    mut bIntToNull: ::core::ffi::c_int,
) -> *mut ExprList {
    unsafe {
        if !pAppend.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            let mut nInit: ::core::ffi::c_int = if !pList.is_null() {
                (*pList).nExpr
            } else {
                0 as ::core::ffi::c_int
            };
            i = 0 as ::core::ffi::c_int;
            while i < (*pAppend).nExpr {
                let mut db: *mut sqlite3 = (*pParse).db;
                let mut pDup: *mut Expr = sqlite3ExprDup(
                    db,
                    (*(&raw mut (*pAppend).a as *mut ExprList_item).offset(i as isize))
                        .pExpr,
                    0 as ::core::ffi::c_int,
                );
                if (*db).mallocFailed != 0 {
                    sqlite3ExprDelete(db, pDup);
                    break;
                } else {
                    if bIntToNull != 0 {
                        let mut iDummy: ::core::ffi::c_int = 0;
                        let mut pSub: *mut Expr = ::core::ptr::null_mut::<Expr>();
                        pSub = sqlite3ExprSkipCollateAndLikely(pDup);
                        if sqlite3ExprIsInteger(
                            pSub,
                            &raw mut iDummy,
                            ::core::ptr::null_mut::<Parse>(),
                        ) != 0
                        {
                            (*pSub).op = TK_NULL as u8_0;
                            (*pSub).flags = ((*pSub).flags as ::core::ffi::c_uint
                                & !(EP_IntValue | EP_IsTrue | EP_IsFalse)
                                    as ::core::ffi::c_uint) as u32_0;
                            (*pSub).u.zToken = ::core::ptr::null_mut::<
                                ::core::ffi::c_char,
                            >();
                        }
                    }
                    pList = sqlite3ExprListAppend(pParse, pList, pDup);
                    if !pList.is_null() {
                        (*(&raw mut (*pList).a as *mut ExprList_item)
                            .offset((nInit + i) as isize))
                            .fg
                            .sortFlags = (*(&raw mut (*pAppend).a as *mut ExprList_item)
                            .offset(i as isize))
                            .fg
                            .sortFlags;
                    }
                    i += 1;
                }
            }
        }
        return pList;
    }
}
unsafe extern "C" fn sqlite3WindowExtraAggFuncDepth(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int == TK_AGG_FUNCTION
            && (*pExpr).op2 as ::core::ffi::c_int >= (*pWalker).walkerDepth
        {
            (*pExpr).op2 = (*pExpr).op2.wrapping_add(1);
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn disallowAggregatesInOrderByCb(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int == TK_AGG_FUNCTION
            && (*pExpr).pAggInfo.is_null()
        {
            sqlite3ErrorMsg(
                (*pWalker).pParse,
                b"misuse of aggregate: %s()\0".as_ptr() as *const ::core::ffi::c_char,
                (*pExpr).u.zToken,
            );
        }
        return WRC_Continue;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowRewrite(
    mut pParse: *mut Parse,
    mut p: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !(*p).pWin.is_null() && (*p).pPrior.is_null()
            && (*p).selFlags & 0x100000 as u32_0 == 0 as u32_0
            && !((*pParse).eParseMode as ::core::ffi::c_int >= 2 as ::core::ffi::c_int)
        {
            let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
            let mut db: *mut sqlite3 = (*pParse).db;
            let mut pSub: *mut Select = ::core::ptr::null_mut::<Select>();
            let mut pSrc: *mut SrcList = (*p).pSrc;
            let mut pWhere: *mut Expr = (*p).pWhere;
            let mut pGroupBy: *mut ExprList = (*p).pGroupBy;
            let mut pHaving: *mut Expr = (*p).pHaving;
            let mut pSort: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            let mut pSublist: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            let mut pMWin: *mut Window = (*p).pWin;
            let mut pWin: *mut Window = ::core::ptr::null_mut::<Window>();
            let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
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
            let mut selFlags: u32_0 = (*p).selFlags;
            pTab = sqlite3DbMallocZero(db, ::core::mem::size_of::<Table>() as u64_0)
                as *mut Table;
            if pTab.is_null() {
                return sqlite3ErrorToParser(db, SQLITE_NOMEM);
            }
            sqlite3AggInfoPersistWalkerInit(&raw mut w, pParse);
            sqlite3WalkSelect(&raw mut w, p);
            if (*p).selFlags & SF_Aggregate as u32_0 == 0 as u32_0 {
                w.xExprCallback = Some(
                    disallowAggregatesInOrderByCb
                        as unsafe extern "C" fn(
                            *mut Walker,
                            *mut Expr,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut Walker,
                            *mut Expr,
                        ) -> ::core::ffi::c_int,
                    >;
                w.xSelectCallback = None;
                sqlite3WalkExprList(&raw mut w, (*p).pOrderBy);
            }
            (*p).pSrc = ::core::ptr::null_mut::<SrcList>();
            (*p).pWhere = ::core::ptr::null_mut::<Expr>();
            (*p).pGroupBy = ::core::ptr::null_mut::<ExprList>();
            (*p).pHaving = ::core::ptr::null_mut::<Expr>();
            (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
                & !(SF_Aggregate as u32_0) as ::core::ffi::c_uint) as u32_0;
            (*p).selFlags = ((*p).selFlags as ::core::ffi::c_uint
                | SF_WinRewrite as ::core::ffi::c_uint) as u32_0;
            pSort = exprListAppendList(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                (*pMWin).pPartition,
                1 as ::core::ffi::c_int,
            );
            pSort = exprListAppendList(
                pParse,
                pSort,
                (*pMWin).pOrderBy,
                1 as ::core::ffi::c_int,
            );
            if !pSort.is_null() && !(*p).pOrderBy.is_null()
                && (*(*p).pOrderBy).nExpr <= (*pSort).nExpr
            {
                let mut nSave: ::core::ffi::c_int = (*pSort).nExpr;
                (*pSort).nExpr = (*(*p).pOrderBy).nExpr;
                if sqlite3ExprListCompare(pSort, (*p).pOrderBy, -1 as ::core::ffi::c_int)
                    == 0 as ::core::ffi::c_int
                {
                    sqlite3ExprListDelete(db, (*p).pOrderBy);
                    (*p).pOrderBy = ::core::ptr::null_mut::<ExprList>();
                }
                (*pSort).nExpr = nSave;
            }
            let c2rust_fresh4 = (*pParse).nTab;
            (*pParse).nTab = (*pParse).nTab + 1;
            (*pMWin).iEphCsr = c2rust_fresh4;
            (*pParse).nTab += 3 as ::core::ffi::c_int;
            selectWindowRewriteEList(
                pParse,
                pMWin,
                pSrc,
                (*p).pEList,
                pTab,
                &raw mut pSublist,
            );
            selectWindowRewriteEList(
                pParse,
                pMWin,
                pSrc,
                (*p).pOrderBy,
                pTab,
                &raw mut pSublist,
            );
            (*pMWin).nBufferCol = if !pSublist.is_null() {
                (*pSublist).nExpr
            } else {
                0 as ::core::ffi::c_int
            };
            pSublist = exprListAppendList(
                pParse,
                pSublist,
                (*pMWin).pPartition,
                0 as ::core::ffi::c_int,
            );
            pSublist = exprListAppendList(
                pParse,
                pSublist,
                (*pMWin).pOrderBy,
                0 as ::core::ffi::c_int,
            );
            pWin = pMWin;
            while !pWin.is_null() {
                let mut pArgs: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                pArgs = (*(*pWin).pOwner).x.pList;
                if (*(*pWin).pWFunc).funcFlags & SQLITE_SUBTYPE as u32_0 != 0 {
                    selectWindowRewriteEList(
                        pParse,
                        pMWin,
                        pSrc,
                        pArgs,
                        pTab,
                        &raw mut pSublist,
                    );
                    (*pWin).iArgCol = if !pSublist.is_null() {
                        (*pSublist).nExpr
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    (*pWin).bExprArgs = 1 as u8_0;
                } else {
                    (*pWin).iArgCol = if !pSublist.is_null() {
                        (*pSublist).nExpr
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    pSublist = exprListAppendList(
                        pParse,
                        pSublist,
                        pArgs,
                        0 as ::core::ffi::c_int,
                    );
                }
                if !(*pWin).pFilter.is_null() {
                    let mut pFilter: *mut Expr = sqlite3ExprDup(
                        db,
                        (*pWin).pFilter,
                        0 as ::core::ffi::c_int,
                    );
                    pSublist = sqlite3ExprListAppend(pParse, pSublist, pFilter);
                }
                (*pParse).nMem += 1;
                (*pWin).regAccum = (*pParse).nMem;
                (*pParse).nMem += 1;
                (*pWin).regResult = (*pParse).nMem;
                sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, (*pWin).regAccum);
                pWin = (*pWin).pNextWin;
            }
            if pSublist.is_null() {
                pSublist = sqlite3ExprListAppend(
                    pParse,
                    ::core::ptr::null_mut::<ExprList>(),
                    sqlite3Expr(
                        db,
                        TK_INTEGER,
                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                    ),
                );
            }
            pSub = sqlite3SelectNew(
                pParse,
                pSublist,
                pSrc,
                pWhere,
                pGroupBy,
                pHaving,
                pSort,
                0 as u32_0,
                ::core::ptr::null_mut::<Expr>(),
            );
            (*p).pSrc = sqlite3SrcListAppend(
                pParse,
                ::core::ptr::null_mut::<SrcList>(),
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<Token>(),
            );
            if (*p).pSrc.is_null() {
                sqlite3SelectDelete(db, pSub);
            } else if sqlite3SrcItemAttachSubquery(
                pParse,
                (&raw mut (*(*p).pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem,
                pSub,
                0 as ::core::ffi::c_int,
            ) != 0
            {
                let mut pTab2: *mut Table = ::core::ptr::null_mut::<Table>();
                let ref mut c2rust_fresh5 = (*(&raw mut (*(*p).pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .fg;
                (*c2rust_fresh5)
                    .set_isCorrelated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                sqlite3SrcListAssignCursors(pParse, (*p).pSrc);
                (*pSub).selFlags = ((*pSub).selFlags as ::core::ffi::c_uint
                    | (SF_Expanded | SF_OrderByReqd) as ::core::ffi::c_uint) as u32_0;
                pTab2 = sqlite3ResultSetOfSelect(
                    pParse,
                    pSub,
                    SQLITE_AFF_NONE as ::core::ffi::c_char,
                );
                (*pSub).selFlags = ((*pSub).selFlags as ::core::ffi::c_uint
                    | (selFlags & SF_Aggregate as u32_0) as ::core::ffi::c_uint)
                    as u32_0;
                if pTab2.is_null() {
                    rc = SQLITE_NOMEM;
                } else {
                    memcpy(
                        pTab as *mut ::core::ffi::c_void,
                        pTab2 as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<Table>() as size_t,
                    );
                    (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                        | TF_Ephemeral as ::core::ffi::c_uint) as u32_0;
                    let ref mut c2rust_fresh6 = (*(&raw mut (*(*p).pSrc).a
                        as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .pSTab;
                    *c2rust_fresh6 = pTab;
                    pTab = pTab2;
                    memset(
                        &raw mut w as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<Walker>() as size_t,
                    );
                    w.xExprCallback = Some(
                        sqlite3WindowExtraAggFuncDepth
                            as unsafe extern "C" fn(
                                *mut Walker,
                                *mut Expr,
                            ) -> ::core::ffi::c_int,
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut Walker,
                                *mut Expr,
                            ) -> ::core::ffi::c_int,
                        >;
                    w.xSelectCallback = Some(
                        sqlite3WalkerDepthIncrease
                            as unsafe extern "C" fn(
                                *mut Walker,
                                *mut Select,
                            ) -> ::core::ffi::c_int,
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut Walker,
                                *mut Select,
                            ) -> ::core::ffi::c_int,
                        >;
                    w.xSelectCallback2 = Some(
                        sqlite3WalkerDepthDecrease
                            as unsafe extern "C" fn(*mut Walker, *mut Select) -> (),
                    ) as Option<unsafe extern "C" fn(*mut Walker, *mut Select) -> ()>;
                    sqlite3WalkSelect(&raw mut w, pSub);
                }
            }
            if (*db).mallocFailed != 0 {
                rc = SQLITE_NOMEM;
            }
            sqlite3ParserAddCleanup(
                pParse,
                Some(
                    sqlite3DbFree
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                pTab as *mut ::core::ffi::c_void,
            );
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowUnlinkFromSelect(mut p: *mut Window) {
    unsafe {
        if !(*p).ppThis.is_null() {
            *(*p).ppThis = (*p).pNextWin;
            if !(*p).pNextWin.is_null() {
                (*(*p).pNextWin).ppThis = (*p).ppThis;
            }
            (*p).ppThis = ::core::ptr::null_mut::<*mut Window>();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowDelete(mut db: *mut sqlite3, mut p: *mut Window) {
    unsafe {
        if !p.is_null() {
            sqlite3WindowUnlinkFromSelect(p);
            sqlite3ExprDelete(db, (*p).pFilter);
            sqlite3ExprListDelete(db, (*p).pPartition);
            sqlite3ExprListDelete(db, (*p).pOrderBy);
            sqlite3ExprDelete(db, (*p).pEnd);
            sqlite3ExprDelete(db, (*p).pStart);
            sqlite3DbFree(db, (*p).zName as *mut ::core::ffi::c_void);
            sqlite3DbFree(db, (*p).zBase as *mut ::core::ffi::c_void);
            sqlite3DbFree(db, p as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowListDelete(
    mut db: *mut sqlite3,
    mut p: *mut Window,
) {
    unsafe {
        while !p.is_null() {
            let mut pNext: *mut Window = (*p).pNextWin;
            sqlite3WindowDelete(db, p);
            p = pNext;
        }
    }
}
unsafe extern "C" fn sqlite3WindowOffsetExpr(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
) -> *mut Expr {
    unsafe {
        if 0 as ::core::ffi::c_int
            == sqlite3ExprIsConstant(::core::ptr::null_mut::<Parse>(), pExpr)
        {
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                sqlite3RenameExprUnmap(pParse, pExpr);
            }
            sqlite3ExprDelete((*pParse).db, pExpr);
            pExpr = sqlite3ExprAlloc(
                (*pParse).db,
                TK_NULL,
                ::core::ptr::null::<Token>(),
                0 as ::core::ffi::c_int,
            );
        }
        return pExpr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowAlloc(
    mut pParse: *mut Parse,
    mut eType: ::core::ffi::c_int,
    mut eStart: ::core::ffi::c_int,
    mut pStart: *mut Expr,
    mut eEnd: ::core::ffi::c_int,
    mut pEnd: *mut Expr,
    mut eExclude: u8_0,
) -> *mut Window {
    unsafe {
        let mut pWin: *mut Window = ::core::ptr::null_mut::<Window>();
        let mut bImplicitFrame: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if eType == 0 as ::core::ffi::c_int {
            bImplicitFrame = 1 as ::core::ffi::c_int;
            eType = TK_RANGE;
        }
        if eStart == TK_CURRENT && eEnd == TK_PRECEDING
            || eStart == TK_FOLLOWING && (eEnd == TK_PRECEDING || eEnd == TK_CURRENT)
        {
            sqlite3ErrorMsg(
                pParse,
                b"unsupported frame specification\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            pWin = sqlite3DbMallocZero(
                (*pParse).db,
                ::core::mem::size_of::<Window>() as u64_0,
            ) as *mut Window;
            if !pWin.is_null() {
                (*pWin).eFrmType = eType as u8_0;
                (*pWin).eStart = eStart as u8_0;
                (*pWin).eEnd = eEnd as u8_0;
                if eExclude as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*(*pParse).db).dbOptFlags & 0x2 as u32_0 != 0 as u32_0
                {
                    eExclude = TK_NO as u8_0;
                }
                (*pWin).eExclude = eExclude;
                (*pWin).bImplicitFrame = bImplicitFrame as u8_0;
                (*pWin).pEnd = sqlite3WindowOffsetExpr(pParse, pEnd);
                (*pWin).pStart = sqlite3WindowOffsetExpr(pParse, pStart);
                return pWin;
            }
        }
        sqlite3ExprDelete((*pParse).db, pEnd);
        sqlite3ExprDelete((*pParse).db, pStart);
        return ::core::ptr::null_mut::<Window>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowAssemble(
    mut pParse: *mut Parse,
    mut pWin: *mut Window,
    mut pPartition: *mut ExprList,
    mut pOrderBy: *mut ExprList,
    mut pBase: *mut Token,
) -> *mut Window {
    unsafe {
        if !pWin.is_null() {
            (*pWin).pPartition = pPartition;
            (*pWin).pOrderBy = pOrderBy;
            if !pBase.is_null() {
                (*pWin).zBase = sqlite3DbStrNDup(
                    (*pParse).db,
                    (*pBase).z,
                    (*pBase).n as u64_0,
                );
            }
        } else {
            sqlite3ExprListDelete((*pParse).db, pPartition);
            sqlite3ExprListDelete((*pParse).db, pOrderBy);
        }
        return pWin;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowChain(
    mut pParse: *mut Parse,
    mut pWin: *mut Window,
    mut pList: *mut Window,
) {
    unsafe {
        if !(*pWin).zBase.is_null() {
            let mut db: *mut sqlite3 = (*pParse).db;
            let mut pExist: *mut Window = windowFind(pParse, pList, (*pWin).zBase);
            if !pExist.is_null() {
                let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                if !(*pWin).pPartition.is_null() {
                    zErr = b"PARTITION clause\0".as_ptr() as *const ::core::ffi::c_char;
                } else if !(*pExist).pOrderBy.is_null() && !(*pWin).pOrderBy.is_null() {
                    zErr = b"ORDER BY clause\0".as_ptr() as *const ::core::ffi::c_char;
                } else if (*pExist).bImplicitFrame as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    zErr = b"frame specification\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                if !zErr.is_null() {
                    sqlite3ErrorMsg(
                        pParse,
                        b"cannot override %s of window: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        zErr,
                        (*pWin).zBase,
                    );
                } else {
                    (*pWin).pPartition = sqlite3ExprListDup(
                        db,
                        (*pExist).pPartition,
                        0 as ::core::ffi::c_int,
                    );
                    if !(*pExist).pOrderBy.is_null() {
                        (*pWin).pOrderBy = sqlite3ExprListDup(
                            db,
                            (*pExist).pOrderBy,
                            0 as ::core::ffi::c_int,
                        );
                    }
                    sqlite3DbFree(db, (*pWin).zBase as *mut ::core::ffi::c_void);
                    (*pWin).zBase = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowAttach(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
    mut pWin: *mut Window,
) {
    unsafe {
        if !p.is_null() {
            (*p).y.pWin = pWin;
            (*p).flags = ((*p).flags as ::core::ffi::c_uint
                | (0x1000000 as ::core::ffi::c_int | 0x20000 as ::core::ffi::c_int)
                    as u32_0 as ::core::ffi::c_uint) as u32_0;
            (*pWin).pOwner = p;
            if (*p).flags & EP_Distinct as u32_0 != 0
                && (*pWin).eFrmType as ::core::ffi::c_int != TK_FILTER
            {
                sqlite3ErrorMsg(
                    pParse,
                    b"DISTINCT is not supported for window functions\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        } else {
            sqlite3WindowDelete((*pParse).db, pWin);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowLink(
    mut pSel: *mut Select,
    mut pWin: *mut Window,
) {
    unsafe {
        if !pSel.is_null() {
            if (*pSel).pWin.is_null()
                || 0 as ::core::ffi::c_int
                    == sqlite3WindowCompare(
                        ::core::ptr::null::<Parse>(),
                        (*pSel).pWin,
                        pWin,
                        0 as ::core::ffi::c_int,
                    )
            {
                (*pWin).pNextWin = (*pSel).pWin;
                if !(*pSel).pWin.is_null() {
                    (*(*pSel).pWin).ppThis = &raw mut (*pWin).pNextWin;
                }
                (*pSel).pWin = pWin;
                (*pWin).ppThis = &raw mut (*pSel).pWin;
            } else if sqlite3ExprListCompare(
                (*pWin).pPartition,
                (*(*pSel).pWin).pPartition,
                -1 as ::core::ffi::c_int,
            ) != 0
            {
                (*pSel).selFlags = ((*pSel).selFlags as ::core::ffi::c_uint
                    | SF_MultiPart as ::core::ffi::c_uint) as u32_0;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowCompare(
    mut pParse: *const Parse,
    mut p1: *const Window,
    mut p2: *const Window,
    mut bFilter: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut res: ::core::ffi::c_int = 0;
        if p1.is_null() || p2.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        if (*p1).eFrmType as ::core::ffi::c_int != (*p2).eFrmType as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if (*p1).eStart as ::core::ffi::c_int != (*p2).eStart as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if (*p1).eEnd as ::core::ffi::c_int != (*p2).eEnd as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if (*p1).eExclude as ::core::ffi::c_int != (*p2).eExclude as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if sqlite3ExprCompare(
            pParse,
            (*p1).pStart,
            (*p2).pStart,
            -1 as ::core::ffi::c_int,
        ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        if sqlite3ExprCompare(pParse, (*p1).pEnd, (*p2).pEnd, -1 as ::core::ffi::c_int)
            != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        res = sqlite3ExprListCompare(
            (*p1).pPartition,
            (*p2).pPartition,
            -1 as ::core::ffi::c_int,
        );
        if res != 0 {
            return res;
        }
        res = sqlite3ExprListCompare(
            (*p1).pOrderBy,
            (*p2).pOrderBy,
            -1 as ::core::ffi::c_int,
        );
        if res != 0 {
            return res;
        }
        if bFilter != 0 {
            res = sqlite3ExprCompare(
                pParse,
                (*p1).pFilter,
                (*p2).pFilter,
                -1 as ::core::ffi::c_int,
            );
            if res != 0 {
                return res;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowCodeInit(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
) {
    unsafe {
        let mut pWin: *mut Window = ::core::ptr::null_mut::<Window>();
        let mut nEphExpr: ::core::ffi::c_int = 0;
        let mut pMWin: *mut Window = ::core::ptr::null_mut::<Window>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        nEphExpr = (*(*(*(*(&raw mut (*(*pSelect).pSrc).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .u4
            .pSubq)
            .pSelect)
            .pEList)
            .nExpr;
        pMWin = (*pSelect).pWin;
        v = sqlite3GetVdbe(pParse);
        sqlite3VdbeAddOp2(v, OP_OpenEphemeral, (*pMWin).iEphCsr, nEphExpr);
        sqlite3VdbeAddOp2(
            v,
            OP_OpenDup,
            (*pMWin).iEphCsr + 1 as ::core::ffi::c_int,
            (*pMWin).iEphCsr,
        );
        sqlite3VdbeAddOp2(
            v,
            OP_OpenDup,
            (*pMWin).iEphCsr + 2 as ::core::ffi::c_int,
            (*pMWin).iEphCsr,
        );
        sqlite3VdbeAddOp2(
            v,
            OP_OpenDup,
            (*pMWin).iEphCsr + 3 as ::core::ffi::c_int,
            (*pMWin).iEphCsr,
        );
        if !(*pMWin).pPartition.is_null() {
            let mut nExpr: ::core::ffi::c_int = (*(*pMWin).pPartition).nExpr;
            (*pMWin).regPart = (*pParse).nMem + 1 as ::core::ffi::c_int;
            (*pParse).nMem += nExpr;
            sqlite3VdbeAddOp3(
                v,
                OP_Null,
                0 as ::core::ffi::c_int,
                (*pMWin).regPart,
                (*pMWin).regPart + nExpr - 1 as ::core::ffi::c_int,
            );
        }
        (*pParse).nMem += 1;
        (*pMWin).regOne = (*pParse).nMem;
        sqlite3VdbeAddOp2(v, OP_Integer, 1 as ::core::ffi::c_int, (*pMWin).regOne);
        if (*pMWin).eExclude != 0 {
            (*pParse).nMem += 1;
            (*pMWin).regStartRowid = (*pParse).nMem;
            (*pParse).nMem += 1;
            (*pMWin).regEndRowid = (*pParse).nMem;
            let c2rust_fresh0 = (*pParse).nTab;
            (*pParse).nTab = (*pParse).nTab + 1;
            (*pMWin).csrApp = c2rust_fresh0;
            sqlite3VdbeAddOp2(
                v,
                OP_Integer,
                1 as ::core::ffi::c_int,
                (*pMWin).regStartRowid,
            );
            sqlite3VdbeAddOp2(
                v,
                OP_Integer,
                0 as ::core::ffi::c_int,
                (*pMWin).regEndRowid,
            );
            sqlite3VdbeAddOp2(v, OP_OpenDup, (*pMWin).csrApp, (*pMWin).iEphCsr);
            return;
        }
        pWin = pMWin;
        while !pWin.is_null() {
            let mut p: *mut FuncDef = (*pWin).pWFunc;
            if (*p).funcFlags & SQLITE_FUNC_MINMAX as u32_0 != 0
                && (*pWin).eStart as ::core::ffi::c_int != TK_UNBOUNDED
            {
                let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                let mut pKeyInfo: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
                pList = (*(*pWin).pOwner).x.pList;
                pKeyInfo = sqlite3KeyInfoFromExprList(
                    pParse,
                    pList,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                let c2rust_fresh1 = (*pParse).nTab;
                (*pParse).nTab = (*pParse).nTab + 1;
                (*pWin).csrApp = c2rust_fresh1;
                (*pWin).regApp = (*pParse).nMem + 1 as ::core::ffi::c_int;
                (*pParse).nMem += 3 as ::core::ffi::c_int;
                if !pKeyInfo.is_null()
                    && *(*(*pWin).pWFunc).zName.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'i' as i32
                {
                    *(*pKeyInfo).aSortFlags.offset(0 as ::core::ffi::c_int as isize) = KEYINFO_ORDER_DESC
                        as u8_0;
                }
                sqlite3VdbeAddOp2(
                    v,
                    OP_OpenEphemeral,
                    (*pWin).csrApp,
                    2 as ::core::ffi::c_int,
                );
                sqlite3VdbeAppendP4(v, pKeyInfo as *mut ::core::ffi::c_void, P4_KEYINFO);
                sqlite3VdbeAddOp2(
                    v,
                    OP_Integer,
                    0 as ::core::ffi::c_int,
                    (*pWin).regApp + 1 as ::core::ffi::c_int,
                );
            } else if (*p).zName
                == &raw const nth_valueName as *const ::core::ffi::c_char
                || (*p).zName == &raw const first_valueName as *const ::core::ffi::c_char
            {
                (*pWin).regApp = (*pParse).nMem + 1 as ::core::ffi::c_int;
                let c2rust_fresh2 = (*pParse).nTab;
                (*pParse).nTab = (*pParse).nTab + 1;
                (*pWin).csrApp = c2rust_fresh2;
                (*pParse).nMem += 2 as ::core::ffi::c_int;
                sqlite3VdbeAddOp2(v, OP_OpenDup, (*pWin).csrApp, (*pMWin).iEphCsr);
            } else if (*p).zName == &raw const leadName as *const ::core::ffi::c_char
                || (*p).zName == &raw const lagName as *const ::core::ffi::c_char
            {
                let c2rust_fresh3 = (*pParse).nTab;
                (*pParse).nTab = (*pParse).nTab + 1;
                (*pWin).csrApp = c2rust_fresh3;
                sqlite3VdbeAddOp2(v, OP_OpenDup, (*pWin).csrApp, (*pMWin).iEphCsr);
            }
            pWin = (*pWin).pNextWin;
        }
    }
}
pub const WINDOW_STARTING_NUM: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
unsafe extern "C" fn windowCheckValue(
    mut pParse: *mut Parse,
    mut reg: ::core::ffi::c_int,
    mut eCond: ::core::ffi::c_int,
) {
    unsafe {
        static mut azErr: [*const ::core::ffi::c_char; 5] = [
            b"frame starting offset must be a non-negative integer\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"frame ending offset must be a non-negative integer\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"second argument to nth_value must be a positive integer\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"frame starting offset must be a non-negative number\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"frame ending offset must be a non-negative number\0".as_ptr()
                as *const ::core::ffi::c_char,
        ];
        static mut aOp: [::core::ffi::c_int; 5] = [OP_Ge, OP_Ge, OP_Gt, OP_Ge, OP_Ge];
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        let mut regZero: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
        sqlite3VdbeAddOp2(v, OP_Integer, 0 as ::core::ffi::c_int, regZero);
        if eCond >= WINDOW_STARTING_NUM {
            let mut regString: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
            sqlite3VdbeAddOp4(
                v,
                OP_String8,
                0 as ::core::ffi::c_int,
                regString,
                0 as ::core::ffi::c_int,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
                P4_STATIC,
            );
            sqlite3VdbeAddOp3(
                v,
                OP_Ge,
                regString,
                sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
                reg,
            );
            sqlite3VdbeChangeP5(v, (SQLITE_AFF_NUMERIC | SQLITE_JUMPIFNULL) as u16_0);
        } else {
            sqlite3VdbeAddOp2(
                v,
                OP_MustBeInt,
                reg,
                sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
            );
        }
        sqlite3VdbeAddOp3(
            v,
            aOp[eCond as usize],
            regZero,
            sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
            reg,
        );
        sqlite3VdbeChangeP5(v, SQLITE_AFF_NUMERIC as u16_0);
        sqlite3MayAbort(pParse);
        sqlite3VdbeAddOp2(v, OP_Halt, SQLITE_ERROR, OE_Abort);
        sqlite3VdbeAppendP4(
            v,
            azErr[eCond as usize] as *mut ::core::ffi::c_void,
            P4_STATIC,
        );
        sqlite3ReleaseTempReg(pParse, regZero);
    }
}
unsafe extern "C" fn windowArgCount(mut pWin: *mut Window) -> ::core::ffi::c_int {
    unsafe {
        let mut pList: *const ExprList = ::core::ptr::null::<ExprList>();
        pList = (*(*pWin).pOwner).x.pList;
        return if !pList.is_null() { (*pList).nExpr } else { 0 as ::core::ffi::c_int };
    }
}
unsafe extern "C" fn windowReadPeerValues(
    mut p: *mut WindowCodeArg,
    mut csr: ::core::ffi::c_int,
    mut reg: ::core::ffi::c_int,
) {
    unsafe {
        let mut pMWin: *mut Window = (*p).pMWin;
        let mut pOrderBy: *mut ExprList = (*pMWin).pOrderBy;
        if !pOrderBy.is_null() {
            let mut v: *mut Vdbe = sqlite3GetVdbe((*p).pParse);
            let mut pPart: *mut ExprList = (*pMWin).pPartition;
            let mut iColOff: ::core::ffi::c_int = (*pMWin).nBufferCol
                + (if !pPart.is_null() {
                    (*pPart).nExpr
                } else {
                    0 as ::core::ffi::c_int
                });
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pOrderBy).nExpr {
                sqlite3VdbeAddOp3(v, OP_Column, csr, iColOff + i, reg + i);
                i += 1;
            }
        }
    }
}
unsafe extern "C" fn windowAggStep(
    mut p: *mut WindowCodeArg,
    mut pMWin: *mut Window,
    mut csr: ::core::ffi::c_int,
    mut bInverse: ::core::ffi::c_int,
    mut reg: ::core::ffi::c_int,
) {
    unsafe {
        let mut pParse: *mut Parse = (*p).pParse;
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        let mut pWin: *mut Window = ::core::ptr::null_mut::<Window>();
        pWin = pMWin;
        while !pWin.is_null() {
            let mut pFunc: *mut FuncDef = (*pWin).pWFunc;
            let mut regArg: ::core::ffi::c_int = 0;
            let mut nArg: ::core::ffi::c_int = if (*pWin).bExprArgs as ::core::ffi::c_int
                != 0
            {
                0 as ::core::ffi::c_int
            } else {
                windowArgCount(pWin)
            };
            let mut i: ::core::ffi::c_int = 0;
            let mut addrIf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < nArg {
                if i != 1 as ::core::ffi::c_int
                    || (*pFunc).zName
                        != &raw const nth_valueName as *const ::core::ffi::c_char
                {
                    sqlite3VdbeAddOp3(v, OP_Column, csr, (*pWin).iArgCol + i, reg + i);
                } else {
                    sqlite3VdbeAddOp3(
                        v,
                        OP_Column,
                        (*pMWin).iEphCsr,
                        (*pWin).iArgCol + i,
                        reg + i,
                    );
                }
                i += 1;
            }
            regArg = reg;
            if !(*pWin).pFilter.is_null() {
                let mut regTmp: ::core::ffi::c_int = 0;
                regTmp = sqlite3GetTempReg(pParse);
                sqlite3VdbeAddOp3(v, OP_Column, csr, (*pWin).iArgCol + nArg, regTmp);
                addrIf = sqlite3VdbeAddOp3(
                    v,
                    OP_IfNot,
                    regTmp,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                sqlite3ReleaseTempReg(pParse, regTmp);
            }
            if (*pMWin).regStartRowid == 0 as ::core::ffi::c_int
                && (*pFunc).funcFlags & SQLITE_FUNC_MINMAX as u32_0 != 0
                && (*pWin).eStart as ::core::ffi::c_int != TK_UNBOUNDED
            {
                let mut addrIsNull: ::core::ffi::c_int = sqlite3VdbeAddOp1(
                    v,
                    OP_IsNull,
                    regArg,
                );
                if bInverse == 0 as ::core::ffi::c_int {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_AddImm,
                        (*pWin).regApp + 1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    sqlite3VdbeAddOp2(v, OP_SCopy, regArg, (*pWin).regApp);
                    sqlite3VdbeAddOp3(
                        v,
                        OP_MakeRecord,
                        (*pWin).regApp,
                        2 as ::core::ffi::c_int,
                        (*pWin).regApp + 2 as ::core::ffi::c_int,
                    );
                    sqlite3VdbeAddOp2(
                        v,
                        OP_IdxInsert,
                        (*pWin).csrApp,
                        (*pWin).regApp + 2 as ::core::ffi::c_int,
                    );
                } else {
                    sqlite3VdbeAddOp4Int(
                        v,
                        OP_SeekGE,
                        (*pWin).csrApp,
                        0 as ::core::ffi::c_int,
                        regArg,
                        1 as ::core::ffi::c_int,
                    );
                    sqlite3VdbeAddOp1(v, OP_Delete, (*pWin).csrApp);
                    sqlite3VdbeJumpHere(
                        v,
                        sqlite3VdbeCurrentAddr(v) - 2 as ::core::ffi::c_int,
                    );
                }
                sqlite3VdbeJumpHere(v, addrIsNull);
            } else if (*pWin).regApp != 0 {
                sqlite3VdbeAddOp2(
                    v,
                    OP_AddImm,
                    (*pWin).regApp + 1 as ::core::ffi::c_int - bInverse,
                    1 as ::core::ffi::c_int,
                );
            } else if (*pFunc).xSFunc
                != Some(
                    noopStepFunc
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                )
            {
                if (*pWin).bExprArgs != 0 {
                    let mut iOp: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(v);
                    let mut iEnd: ::core::ffi::c_int = 0;
                    nArg = (*(*(*pWin).pOwner).x.pList).nExpr;
                    regArg = sqlite3GetTempRange(pParse, nArg);
                    sqlite3ExprCodeExprList(
                        pParse,
                        (*(*pWin).pOwner).x.pList,
                        regArg,
                        0 as ::core::ffi::c_int,
                        0 as u8_0,
                    );
                    iEnd = sqlite3VdbeCurrentAddr(v);
                    while iOp < iEnd {
                        let mut pOp: *mut VdbeOp = sqlite3VdbeGetOp(v, iOp);
                        if (*pOp).opcode as ::core::ffi::c_int == OP_Column
                            && (*pOp).p1 == (*pMWin).iEphCsr
                        {
                            (*pOp).p1 = csr;
                        }
                        iOp += 1;
                    }
                }
                if (*pFunc).funcFlags & SQLITE_FUNC_NEEDCOLL as u32_0 != 0 {
                    let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
                    pColl = sqlite3ExprNNCollSeq(
                        pParse,
                        (*(&raw mut (*(*(*pWin).pOwner).x.pList).a as *mut ExprList_item)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .pExpr,
                    );
                    sqlite3VdbeAddOp4(
                        v,
                        OP_CollSeq,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        pColl as *const ::core::ffi::c_char,
                        P4_COLLSEQ,
                    );
                }
                sqlite3VdbeAddOp3(
                    v,
                    if bInverse != 0 { OP_AggInverse } else { OP_AggStep },
                    bInverse,
                    regArg,
                    (*pWin).regAccum,
                );
                sqlite3VdbeAppendP4(v, pFunc as *mut ::core::ffi::c_void, P4_FUNCDEF);
                sqlite3VdbeChangeP5(v, nArg as u16_0);
                if (*pWin).bExprArgs != 0 {
                    sqlite3ReleaseTempRange(pParse, regArg, nArg);
                }
            }
            if addrIf != 0 {
                sqlite3VdbeJumpHere(v, addrIf);
            }
            pWin = (*pWin).pNextWin;
        }
    }
}
pub const WINDOW_RETURN_ROW: ::core::ffi::c_int = 1;
pub const WINDOW_AGGINVERSE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WINDOW_AGGSTEP: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
unsafe extern "C" fn windowAggFinal(
    mut p: *mut WindowCodeArg,
    mut bFin: ::core::ffi::c_int,
) {
    unsafe {
        let mut pParse: *mut Parse = (*p).pParse;
        let mut pMWin: *mut Window = (*p).pMWin;
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        let mut pWin: *mut Window = ::core::ptr::null_mut::<Window>();
        pWin = pMWin;
        while !pWin.is_null() {
            if (*pMWin).regStartRowid == 0 as ::core::ffi::c_int
                && (*(*pWin).pWFunc).funcFlags & SQLITE_FUNC_MINMAX as u32_0 != 0
                && (*pWin).eStart as ::core::ffi::c_int != TK_UNBOUNDED
            {
                sqlite3VdbeAddOp2(
                    v,
                    OP_Null,
                    0 as ::core::ffi::c_int,
                    (*pWin).regResult,
                );
                sqlite3VdbeAddOp1(v, OP_Last, (*pWin).csrApp);
                sqlite3VdbeAddOp3(
                    v,
                    OP_Column,
                    (*pWin).csrApp,
                    0 as ::core::ffi::c_int,
                    (*pWin).regResult,
                );
                sqlite3VdbeJumpHere(
                    v,
                    sqlite3VdbeCurrentAddr(v) - 2 as ::core::ffi::c_int,
                );
            } else if !((*pWin).regApp != 0) {
                let mut nArg: ::core::ffi::c_int = windowArgCount(pWin);
                if bFin != 0 {
                    sqlite3VdbeAddOp2(v, OP_AggFinal, (*pWin).regAccum, nArg);
                    sqlite3VdbeAppendP4(
                        v,
                        (*pWin).pWFunc as *mut ::core::ffi::c_void,
                        P4_FUNCDEF,
                    );
                    sqlite3VdbeAddOp2(v, OP_Copy, (*pWin).regAccum, (*pWin).regResult);
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Null,
                        0 as ::core::ffi::c_int,
                        (*pWin).regAccum,
                    );
                } else {
                    sqlite3VdbeAddOp3(
                        v,
                        OP_AggValue,
                        (*pWin).regAccum,
                        nArg,
                        (*pWin).regResult,
                    );
                    sqlite3VdbeAppendP4(
                        v,
                        (*pWin).pWFunc as *mut ::core::ffi::c_void,
                        P4_FUNCDEF,
                    );
                }
            }
            pWin = (*pWin).pNextWin;
        }
    }
}
unsafe extern "C" fn windowFullScan(mut p: *mut WindowCodeArg) {
    unsafe {
        let mut pWin: *mut Window = ::core::ptr::null_mut::<Window>();
        let mut pParse: *mut Parse = (*p).pParse;
        let mut pMWin: *mut Window = (*p).pMWin;
        let mut v: *mut Vdbe = (*p).pVdbe;
        let mut regCRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regCPeer: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regPeer: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nPeer: ::core::ffi::c_int = 0;
        let mut lblNext: ::core::ffi::c_int = 0;
        let mut lblBrk: ::core::ffi::c_int = 0;
        let mut addrNext: ::core::ffi::c_int = 0;
        let mut csr: ::core::ffi::c_int = 0;
        csr = (*pMWin).csrApp;
        nPeer = if !(*pMWin).pOrderBy.is_null() {
            (*(*pMWin).pOrderBy).nExpr
        } else {
            0 as ::core::ffi::c_int
        };
        lblNext = sqlite3VdbeMakeLabel(pParse);
        lblBrk = sqlite3VdbeMakeLabel(pParse);
        regCRowid = sqlite3GetTempReg(pParse);
        regRowid = sqlite3GetTempReg(pParse);
        if nPeer != 0 {
            regCPeer = sqlite3GetTempRange(pParse, nPeer);
            regPeer = sqlite3GetTempRange(pParse, nPeer);
        }
        sqlite3VdbeAddOp2(v, OP_Rowid, (*pMWin).iEphCsr, regCRowid);
        windowReadPeerValues(p, (*pMWin).iEphCsr, regCPeer);
        pWin = pMWin;
        while !pWin.is_null() {
            sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, (*pWin).regAccum);
            pWin = (*pWin).pNextWin;
        }
        sqlite3VdbeAddOp3(v, OP_SeekGE, csr, lblBrk, (*pMWin).regStartRowid);
        addrNext = sqlite3VdbeCurrentAddr(v);
        sqlite3VdbeAddOp2(v, OP_Rowid, csr, regRowid);
        sqlite3VdbeAddOp3(v, OP_Gt, (*pMWin).regEndRowid, lblBrk, regRowid);
        if (*pMWin).eExclude as ::core::ffi::c_int == TK_CURRENT {
            sqlite3VdbeAddOp3(v, OP_Eq, regCRowid, lblNext, regRowid);
        } else if (*pMWin).eExclude as ::core::ffi::c_int != TK_NO {
            let mut addr: ::core::ffi::c_int = 0;
            let mut addrEq: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pKeyInfo: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
            if !(*pMWin).pOrderBy.is_null() {
                pKeyInfo = sqlite3KeyInfoFromExprList(
                    pParse,
                    (*pMWin).pOrderBy,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
            }
            if (*pMWin).eExclude as ::core::ffi::c_int == TK_TIES {
                addrEq = sqlite3VdbeAddOp3(
                    v,
                    OP_Eq,
                    regCRowid,
                    0 as ::core::ffi::c_int,
                    regRowid,
                );
            }
            if !pKeyInfo.is_null() {
                windowReadPeerValues(p, csr, regPeer);
                sqlite3VdbeAddOp3(v, OP_Compare, regPeer, regCPeer, nPeer);
                sqlite3VdbeAppendP4(v, pKeyInfo as *mut ::core::ffi::c_void, P4_KEYINFO);
                addr = sqlite3VdbeCurrentAddr(v) + 1 as ::core::ffi::c_int;
                sqlite3VdbeAddOp3(v, OP_Jump, addr, lblNext, addr);
            } else {
                sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, lblNext);
            }
            if addrEq != 0 {
                sqlite3VdbeJumpHere(v, addrEq);
            }
        }
        windowAggStep(p, pMWin, csr, 0 as ::core::ffi::c_int, (*p).regArg);
        sqlite3VdbeResolveLabel(v, lblNext);
        sqlite3VdbeAddOp2(v, OP_Next, csr, addrNext);
        sqlite3VdbeJumpHere(v, addrNext - 1 as ::core::ffi::c_int);
        sqlite3VdbeJumpHere(v, addrNext + 1 as ::core::ffi::c_int);
        sqlite3ReleaseTempReg(pParse, regRowid);
        sqlite3ReleaseTempReg(pParse, regCRowid);
        if nPeer != 0 {
            sqlite3ReleaseTempRange(pParse, regPeer, nPeer);
            sqlite3ReleaseTempRange(pParse, regCPeer, nPeer);
        }
        windowAggFinal(p, 1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn windowReturnOneRow(mut p: *mut WindowCodeArg) {
    unsafe {
        let mut pMWin: *mut Window = (*p).pMWin;
        let mut v: *mut Vdbe = (*p).pVdbe;
        if (*pMWin).regStartRowid != 0 {
            windowFullScan(p);
        } else {
            let mut pParse: *mut Parse = (*p).pParse;
            let mut pWin: *mut Window = ::core::ptr::null_mut::<Window>();
            pWin = pMWin;
            while !pWin.is_null() {
                let mut pFunc: *mut FuncDef = (*pWin).pWFunc;
                if (*pFunc).zName
                    == &raw const nth_valueName as *const ::core::ffi::c_char
                    || (*pFunc).zName
                        == &raw const first_valueName as *const ::core::ffi::c_char
                {
                    let mut csr: ::core::ffi::c_int = (*pWin).csrApp;
                    let mut lbl: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
                    let mut tmpReg: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Null,
                        0 as ::core::ffi::c_int,
                        (*pWin).regResult,
                    );
                    if (*pFunc).zName
                        == &raw const nth_valueName as *const ::core::ffi::c_char
                    {
                        sqlite3VdbeAddOp3(
                            v,
                            OP_Column,
                            (*pMWin).iEphCsr,
                            (*pWin).iArgCol + 1 as ::core::ffi::c_int,
                            tmpReg,
                        );
                        windowCheckValue(pParse, tmpReg, 2 as ::core::ffi::c_int);
                    } else {
                        sqlite3VdbeAddOp2(
                            v,
                            OP_Integer,
                            1 as ::core::ffi::c_int,
                            tmpReg,
                        );
                    }
                    sqlite3VdbeAddOp3(v, OP_Add, tmpReg, (*pWin).regApp, tmpReg);
                    sqlite3VdbeAddOp3(
                        v,
                        OP_Gt,
                        (*pWin).regApp + 1 as ::core::ffi::c_int,
                        lbl,
                        tmpReg,
                    );
                    sqlite3VdbeAddOp3(
                        v,
                        OP_SeekRowid,
                        csr,
                        0 as ::core::ffi::c_int,
                        tmpReg,
                    );
                    sqlite3VdbeAddOp3(
                        v,
                        OP_Column,
                        csr,
                        (*pWin).iArgCol,
                        (*pWin).regResult,
                    );
                    sqlite3VdbeResolveLabel(v, lbl);
                    sqlite3ReleaseTempReg(pParse, tmpReg);
                } else if (*pFunc).zName
                    == &raw const leadName as *const ::core::ffi::c_char
                    || (*pFunc).zName == &raw const lagName as *const ::core::ffi::c_char
                {
                    let mut nArg: ::core::ffi::c_int = (*(*(*pWin).pOwner).x.pList)
                        .nExpr;
                    let mut csr_0: ::core::ffi::c_int = (*pWin).csrApp;
                    let mut lbl_0: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
                    let mut tmpReg_0: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                    let mut iEph: ::core::ffi::c_int = (*pMWin).iEphCsr;
                    if nArg < 3 as ::core::ffi::c_int {
                        sqlite3VdbeAddOp2(
                            v,
                            OP_Null,
                            0 as ::core::ffi::c_int,
                            (*pWin).regResult,
                        );
                    } else {
                        sqlite3VdbeAddOp3(
                            v,
                            OP_Column,
                            iEph,
                            (*pWin).iArgCol + 2 as ::core::ffi::c_int,
                            (*pWin).regResult,
                        );
                    }
                    sqlite3VdbeAddOp2(v, OP_Rowid, iEph, tmpReg_0);
                    if nArg < 2 as ::core::ffi::c_int {
                        let mut val: ::core::ffi::c_int = if (*pFunc).zName
                            == &raw const leadName as *const ::core::ffi::c_char
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        };
                        sqlite3VdbeAddOp2(v, OP_AddImm, tmpReg_0, val);
                    } else {
                        let mut op: ::core::ffi::c_int = if (*pFunc).zName
                            == &raw const leadName as *const ::core::ffi::c_char
                        {
                            OP_Add
                        } else {
                            OP_Subtract
                        };
                        let mut tmpReg2: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                        sqlite3VdbeAddOp3(
                            v,
                            OP_Column,
                            iEph,
                            (*pWin).iArgCol + 1 as ::core::ffi::c_int,
                            tmpReg2,
                        );
                        sqlite3VdbeAddOp3(v, op, tmpReg2, tmpReg_0, tmpReg_0);
                        sqlite3ReleaseTempReg(pParse, tmpReg2);
                    }
                    sqlite3VdbeAddOp3(v, OP_SeekRowid, csr_0, lbl_0, tmpReg_0);
                    sqlite3VdbeAddOp3(
                        v,
                        OP_Column,
                        csr_0,
                        (*pWin).iArgCol,
                        (*pWin).regResult,
                    );
                    sqlite3VdbeResolveLabel(v, lbl_0);
                    sqlite3ReleaseTempReg(pParse, tmpReg_0);
                }
                pWin = (*pWin).pNextWin;
            }
        }
        sqlite3VdbeAddOp2(v, OP_Gosub, (*p).regGosub, (*p).addrGosub);
    }
}
unsafe extern "C" fn windowInitAccum(
    mut pParse: *mut Parse,
    mut pMWin: *mut Window,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        let mut regArg: ::core::ffi::c_int = 0;
        let mut nArg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pWin: *mut Window = ::core::ptr::null_mut::<Window>();
        pWin = pMWin;
        while !pWin.is_null() {
            let mut pFunc: *mut FuncDef = (*pWin).pWFunc;
            sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, (*pWin).regAccum);
            nArg = if nArg > windowArgCount(pWin) { nArg } else { windowArgCount(pWin) };
            if (*pMWin).regStartRowid == 0 as ::core::ffi::c_int {
                if (*pFunc).zName
                    == &raw const nth_valueName as *const ::core::ffi::c_char
                    || (*pFunc).zName
                        == &raw const first_valueName as *const ::core::ffi::c_char
                {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Integer,
                        0 as ::core::ffi::c_int,
                        (*pWin).regApp,
                    );
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Integer,
                        0 as ::core::ffi::c_int,
                        (*pWin).regApp + 1 as ::core::ffi::c_int,
                    );
                }
                if (*pFunc).funcFlags & SQLITE_FUNC_MINMAX as u32_0 != 0
                    && (*pWin).csrApp != 0
                {
                    sqlite3VdbeAddOp1(v, OP_ResetSorter, (*pWin).csrApp);
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Integer,
                        0 as ::core::ffi::c_int,
                        (*pWin).regApp + 1 as ::core::ffi::c_int,
                    );
                }
            }
            pWin = (*pWin).pNextWin;
        }
        regArg = (*pParse).nMem + 1 as ::core::ffi::c_int;
        (*pParse).nMem += nArg;
        return regArg;
    }
}
unsafe extern "C" fn windowCacheFrame(mut pMWin: *mut Window) -> ::core::ffi::c_int {
    unsafe {
        let mut pWin: *mut Window = ::core::ptr::null_mut::<Window>();
        if (*pMWin).regStartRowid != 0 {
            return 1 as ::core::ffi::c_int;
        }
        pWin = pMWin;
        while !pWin.is_null() {
            let mut pFunc: *mut FuncDef = (*pWin).pWFunc;
            if (*pFunc).zName == &raw const nth_valueName as *const ::core::ffi::c_char
                || (*pFunc).zName
                    == &raw const first_valueName as *const ::core::ffi::c_char
                || (*pFunc).zName == &raw const leadName as *const ::core::ffi::c_char
                || (*pFunc).zName == &raw const lagName as *const ::core::ffi::c_char
            {
                return 1 as ::core::ffi::c_int;
            }
            pWin = (*pWin).pNextWin;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn windowIfNewPeer(
    mut pParse: *mut Parse,
    mut pOrderBy: *mut ExprList,
    mut regNew: ::core::ffi::c_int,
    mut regOld: ::core::ffi::c_int,
    mut addr: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        if !pOrderBy.is_null() {
            let mut nVal: ::core::ffi::c_int = (*pOrderBy).nExpr;
            let mut pKeyInfo: *mut KeyInfo = sqlite3KeyInfoFromExprList(
                pParse,
                pOrderBy,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp3(v, OP_Compare, regOld, regNew, nVal);
            sqlite3VdbeAppendP4(v, pKeyInfo as *mut ::core::ffi::c_void, P4_KEYINFO);
            sqlite3VdbeAddOp3(
                v,
                OP_Jump,
                sqlite3VdbeCurrentAddr(v) + 1 as ::core::ffi::c_int,
                addr,
                sqlite3VdbeCurrentAddr(v) + 1 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp3(
                v,
                OP_Copy,
                regNew,
                regOld,
                nVal - 1 as ::core::ffi::c_int,
            );
        } else {
            sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, addr);
        };
    }
}
unsafe extern "C" fn windowCodeRangeTest(
    mut p: *mut WindowCodeArg,
    mut op: ::core::ffi::c_int,
    mut csr1: ::core::ffi::c_int,
    mut regVal: ::core::ffi::c_int,
    mut csr2: ::core::ffi::c_int,
    mut lbl: ::core::ffi::c_int,
) {
    unsafe {
        let mut pParse: *mut Parse = (*p).pParse;
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        let mut pOrderBy: *mut ExprList = (*(*p).pMWin).pOrderBy;
        let mut reg1: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
        let mut reg2: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
        (*pParse).nMem += 1;
        let mut regString: ::core::ffi::c_int = (*pParse).nMem;
        let mut arith: ::core::ffi::c_int = OP_Add;
        let mut addrGe: ::core::ffi::c_int = 0;
        let mut addrDone: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        windowReadPeerValues(p, csr1, reg1);
        windowReadPeerValues(p, csr2, reg2);
        if (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
            .offset(0 as ::core::ffi::c_int as isize))
            .fg
            .sortFlags as ::core::ffi::c_int & KEYINFO_ORDER_DESC != 0
        {
            match op {
                OP_Ge => {
                    op = OP_Le;
                }
                OP_Gt => {
                    op = OP_Lt;
                }
                _ => {
                    op = OP_Ge;
                }
            }
            arith = OP_Subtract;
        }
        if (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
            .offset(0 as ::core::ffi::c_int as isize))
            .fg
            .sortFlags as ::core::ffi::c_int & KEYINFO_ORDER_BIGNULL != 0
        {
            let mut addr: ::core::ffi::c_int = sqlite3VdbeAddOp1(v, OP_NotNull, reg1);
            match op {
                OP_Ge => {
                    sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, lbl);
                }
                OP_Gt => {
                    sqlite3VdbeAddOp2(v, OP_NotNull, reg2, lbl);
                }
                OP_Le => {
                    sqlite3VdbeAddOp2(v, OP_IsNull, reg2, lbl);
                }
                _ => {}
            }
            sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, addrDone);
            sqlite3VdbeJumpHere(v, addr);
            sqlite3VdbeAddOp2(
                v,
                OP_IsNull,
                reg2,
                if op == OP_Gt || op == OP_Ge { addrDone } else { lbl },
            );
        }
        sqlite3VdbeAddOp4(
            v,
            OP_String8,
            0 as ::core::ffi::c_int,
            regString,
            0 as ::core::ffi::c_int,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            P4_STATIC,
        );
        addrGe = sqlite3VdbeAddOp3(v, OP_Ge, regString, 0 as ::core::ffi::c_int, reg1);
        if op == OP_Ge && arith == OP_Add || op == OP_Le && arith == OP_Subtract {
            sqlite3VdbeAddOp3(v, op, reg2, lbl, reg1);
        }
        sqlite3VdbeAddOp3(v, arith, regVal, reg1, reg1);
        sqlite3VdbeJumpHere(v, addrGe);
        sqlite3VdbeAddOp3(v, op, reg2, lbl, reg1);
        pColl = sqlite3ExprNNCollSeq(
            pParse,
            (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                .offset(0 as ::core::ffi::c_int as isize))
                .pExpr,
        );
        sqlite3VdbeAppendP4(v, pColl as *mut ::core::ffi::c_void, P4_COLLSEQ);
        sqlite3VdbeChangeP5(v, SQLITE_NULLEQ as u16_0);
        sqlite3VdbeResolveLabel(v, addrDone);
        sqlite3ReleaseTempReg(pParse, reg1);
        sqlite3ReleaseTempReg(pParse, reg2);
    }
}
unsafe extern "C" fn windowCodeOp(
    mut p: *mut WindowCodeArg,
    mut op: ::core::ffi::c_int,
    mut regCountdown: ::core::ffi::c_int,
    mut jumpOnEof: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut csr: ::core::ffi::c_int = 0;
        let mut reg: ::core::ffi::c_int = 0;
        let mut pParse: *mut Parse = (*p).pParse;
        let mut pMWin: *mut Window = (*p).pMWin;
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut v: *mut Vdbe = (*p).pVdbe;
        let mut addrContinue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bPeer: ::core::ffi::c_int = ((*pMWin).eFrmType as ::core::ffi::c_int
            != TK_ROWS) as ::core::ffi::c_int;
        let mut lblDone: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
        let mut addrNextRange: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if op == WINDOW_AGGINVERSE
            && (*pMWin).eStart as ::core::ffi::c_int == TK_UNBOUNDED
        {
            return 0 as ::core::ffi::c_int;
        }
        if regCountdown > 0 as ::core::ffi::c_int {
            if (*pMWin).eFrmType as ::core::ffi::c_int == TK_RANGE {
                addrNextRange = sqlite3VdbeCurrentAddr(v);
                if op == WINDOW_AGGINVERSE {
                    if (*pMWin).eStart as ::core::ffi::c_int == TK_FOLLOWING {
                        windowCodeRangeTest(
                            p,
                            OP_Le,
                            (*p).current.csr,
                            regCountdown,
                            (*p).start.csr,
                            lblDone,
                        );
                    } else {
                        windowCodeRangeTest(
                            p,
                            OP_Ge,
                            (*p).start.csr,
                            regCountdown,
                            (*p).current.csr,
                            lblDone,
                        );
                    }
                } else {
                    windowCodeRangeTest(
                        p,
                        OP_Gt,
                        (*p).end.csr,
                        regCountdown,
                        (*p).current.csr,
                        lblDone,
                    );
                }
            } else {
                sqlite3VdbeAddOp3(
                    v,
                    OP_IfPos,
                    regCountdown,
                    lblDone,
                    1 as ::core::ffi::c_int,
                );
            }
        }
        if op == WINDOW_RETURN_ROW && (*pMWin).regStartRowid == 0 as ::core::ffi::c_int {
            windowAggFinal(p, 0 as ::core::ffi::c_int);
        }
        addrContinue = sqlite3VdbeCurrentAddr(v);
        if (*pMWin).eStart as ::core::ffi::c_int == (*pMWin).eEnd as ::core::ffi::c_int
            && regCountdown != 0 && (*pMWin).eFrmType as ::core::ffi::c_int == TK_RANGE
        {
            let mut regRowid1: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
            let mut regRowid2: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
            if op == WINDOW_AGGINVERSE {
                sqlite3VdbeAddOp2(v, OP_Rowid, (*p).start.csr, regRowid1);
                sqlite3VdbeAddOp2(v, OP_Rowid, (*p).end.csr, regRowid2);
                sqlite3VdbeAddOp3(v, OP_Ge, regRowid2, lblDone, regRowid1);
            } else if (*p).regRowid != 0 {
                sqlite3VdbeAddOp2(v, OP_Rowid, (*p).end.csr, regRowid1);
                sqlite3VdbeAddOp3(v, OP_Ge, (*p).regRowid, lblDone, regRowid1);
            }
            sqlite3ReleaseTempReg(pParse, regRowid1);
            sqlite3ReleaseTempReg(pParse, regRowid2);
        }
        match op {
            WINDOW_RETURN_ROW => {
                csr = (*p).current.csr;
                reg = (*p).current.reg;
                windowReturnOneRow(p);
            }
            WINDOW_AGGINVERSE => {
                csr = (*p).start.csr;
                reg = (*p).start.reg;
                if (*pMWin).regStartRowid != 0 {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_AddImm,
                        (*pMWin).regStartRowid,
                        1 as ::core::ffi::c_int,
                    );
                } else {
                    windowAggStep(p, pMWin, csr, 1 as ::core::ffi::c_int, (*p).regArg);
                }
            }
            _ => {
                csr = (*p).end.csr;
                reg = (*p).end.reg;
                if (*pMWin).regStartRowid != 0 {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_AddImm,
                        (*pMWin).regEndRowid,
                        1 as ::core::ffi::c_int,
                    );
                } else {
                    windowAggStep(p, pMWin, csr, 0 as ::core::ffi::c_int, (*p).regArg);
                }
            }
        }
        if op == (*p).eDelete {
            sqlite3VdbeAddOp1(v, OP_Delete, csr);
            sqlite3VdbeChangeP5(v, OPFLAG_SAVEPOSITION as u16_0);
        }
        if jumpOnEof != 0 {
            sqlite3VdbeAddOp2(
                v,
                OP_Next,
                csr,
                sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
            );
            ret = sqlite3VdbeAddOp0(v, OP_Goto);
        } else {
            sqlite3VdbeAddOp2(
                v,
                OP_Next,
                csr,
                sqlite3VdbeCurrentAddr(v) + 1 as ::core::ffi::c_int + bPeer,
            );
            if bPeer != 0 {
                sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, lblDone);
            }
        }
        if bPeer != 0 {
            let mut nReg: ::core::ffi::c_int = if !(*pMWin).pOrderBy.is_null() {
                (*(*pMWin).pOrderBy).nExpr
            } else {
                0 as ::core::ffi::c_int
            };
            let mut regTmp: ::core::ffi::c_int = if nReg != 0 {
                sqlite3GetTempRange(pParse, nReg)
            } else {
                0 as ::core::ffi::c_int
            };
            windowReadPeerValues(p, csr, regTmp);
            windowIfNewPeer(pParse, (*pMWin).pOrderBy, regTmp, reg, addrContinue);
            sqlite3ReleaseTempRange(pParse, regTmp, nReg);
        }
        if addrNextRange != 0 {
            sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, addrNextRange);
        }
        sqlite3VdbeResolveLabel(v, lblDone);
        return ret;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowDup(
    mut db: *mut sqlite3,
    mut pOwner: *mut Expr,
    mut p: *mut Window,
) -> *mut Window {
    unsafe {
        let mut pNew: *mut Window = ::core::ptr::null_mut::<Window>();
        if !p.is_null() {
            pNew = sqlite3DbMallocZero(db, ::core::mem::size_of::<Window>() as u64_0)
                as *mut Window;
            if !pNew.is_null() {
                (*pNew).zName = sqlite3DbStrDup(db, (*p).zName);
                (*pNew).zBase = sqlite3DbStrDup(db, (*p).zBase);
                (*pNew).pFilter = sqlite3ExprDup(
                    db,
                    (*p).pFilter,
                    0 as ::core::ffi::c_int,
                );
                (*pNew).pWFunc = (*p).pWFunc;
                (*pNew).pPartition = sqlite3ExprListDup(
                    db,
                    (*p).pPartition,
                    0 as ::core::ffi::c_int,
                );
                (*pNew).pOrderBy = sqlite3ExprListDup(
                    db,
                    (*p).pOrderBy,
                    0 as ::core::ffi::c_int,
                );
                (*pNew).eFrmType = (*p).eFrmType;
                (*pNew).eEnd = (*p).eEnd;
                (*pNew).eStart = (*p).eStart;
                (*pNew).eExclude = (*p).eExclude;
                (*pNew).regResult = (*p).regResult;
                (*pNew).regAccum = (*p).regAccum;
                (*pNew).iArgCol = (*p).iArgCol;
                (*pNew).iEphCsr = (*p).iEphCsr;
                (*pNew).bExprArgs = (*p).bExprArgs;
                (*pNew).pStart = sqlite3ExprDup(
                    db,
                    (*p).pStart,
                    0 as ::core::ffi::c_int,
                );
                (*pNew).pEnd = sqlite3ExprDup(db, (*p).pEnd, 0 as ::core::ffi::c_int);
                (*pNew).pOwner = pOwner;
                (*pNew).bImplicitFrame = (*p).bImplicitFrame;
            }
        }
        return pNew;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowListDup(
    mut db: *mut sqlite3,
    mut p: *mut Window,
) -> *mut Window {
    unsafe {
        let mut pWin: *mut Window = ::core::ptr::null_mut::<Window>();
        let mut pRet: *mut Window = ::core::ptr::null_mut::<Window>();
        let mut pp: *mut *mut Window = &raw mut pRet;
        pWin = p;
        while !pWin.is_null() {
            *pp = sqlite3WindowDup(db, ::core::ptr::null_mut::<Expr>(), pWin);
            if (*pp).is_null() {
                break;
            }
            pp = &raw mut (**pp).pNextWin;
            pWin = (*pWin).pNextWin;
        }
        return pRet;
    }
}
unsafe extern "C" fn windowExprGtZero(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        sqlite3ValueFromExpr(
            db,
            pExpr,
            (*db).enc,
            SQLITE_AFF_NUMERIC as u8_0,
            &raw mut pVal,
        );
        if !pVal.is_null() && sqlite3_value_int(pVal) > 0 as ::core::ffi::c_int {
            ret = 1 as ::core::ffi::c_int;
        }
        sqlite3ValueFree(pVal);
        return ret;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WindowCodeStep(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pWInfo: *mut WhereInfo,
    mut regGosub: ::core::ffi::c_int,
    mut addrGosub: ::core::ffi::c_int,
) {
    unsafe {
        let mut pMWin: *mut Window = (*p).pWin;
        let mut pOrderBy: *mut ExprList = (*pMWin).pOrderBy;
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        let mut csrWrite: ::core::ffi::c_int = 0;
        let mut csrInput: ::core::ffi::c_int = (*(&raw mut (*(*p).pSrc).a
            as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .iCursor;
        let mut nInput: ::core::ffi::c_int = (*(*(&raw mut (*(*p).pSrc).a
            as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .pSTab)
            .nCol as ::core::ffi::c_int;
        let mut iInput: ::core::ffi::c_int = 0;
        let mut addrNe: ::core::ffi::c_int = 0;
        let mut addrGosubFlush: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addrInteger: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addrEmpty: ::core::ffi::c_int = 0;
        let mut regNew: ::core::ffi::c_int = 0;
        let mut regRecord: ::core::ffi::c_int = 0;
        let mut regNewPeer: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regPeer: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regFlushPart: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut s: WindowCodeArg = WindowCodeArg {
            pParse: ::core::ptr::null_mut::<Parse>(),
            pMWin: ::core::ptr::null_mut::<Window>(),
            pVdbe: ::core::ptr::null_mut::<Vdbe>(),
            addrGosub: 0,
            regGosub: 0,
            regArg: 0,
            eDelete: 0,
            regRowid: 0,
            start: WindowCsrAndReg { csr: 0, reg: 0 },
            current: WindowCsrAndReg { csr: 0, reg: 0 },
            end: WindowCsrAndReg { csr: 0, reg: 0 }

        };
        let mut lblWhereEnd: ::core::ffi::c_int = 0;
        let mut regStart: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        lblWhereEnd = sqlite3VdbeMakeLabel(pParse);
        memset(
            &raw mut s as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<WindowCodeArg>() as size_t,
        );
        s.pParse = pParse;
        s.pMWin = pMWin;
        s.pVdbe = v;
        s.regGosub = regGosub;
        s.addrGosub = addrGosub;
        s.current.csr = (*pMWin).iEphCsr;
        csrWrite = s.current.csr + 1 as ::core::ffi::c_int;
        s.start.csr = s.current.csr + 2 as ::core::ffi::c_int;
        s.end.csr = s.current.csr + 3 as ::core::ffi::c_int;
        match (*pMWin).eStart as ::core::ffi::c_int {
            TK_FOLLOWING => {
                if (*pMWin).eFrmType as ::core::ffi::c_int != TK_RANGE
                    && windowExprGtZero(pParse, (*pMWin).pStart) != 0
                {
                    s.eDelete = WINDOW_RETURN_ROW;
                }
            }
            TK_UNBOUNDED => {
                if windowCacheFrame(pMWin) == 0 as ::core::ffi::c_int {
                    if (*pMWin).eEnd as ::core::ffi::c_int == TK_PRECEDING {
                        if (*pMWin).eFrmType as ::core::ffi::c_int != TK_RANGE
                            && windowExprGtZero(pParse, (*pMWin).pEnd) != 0
                        {
                            s.eDelete = WINDOW_AGGSTEP;
                        }
                    } else {
                        s.eDelete = WINDOW_RETURN_ROW;
                    }
                }
            }
            _ => {
                s.eDelete = WINDOW_AGGINVERSE;
            }
        }
        regNew = (*pParse).nMem + 1 as ::core::ffi::c_int;
        (*pParse).nMem += nInput;
        (*pParse).nMem += 1;
        regRecord = (*pParse).nMem;
        (*pParse).nMem += 1;
        s.regRowid = (*pParse).nMem;
        if (*pMWin).eStart as ::core::ffi::c_int == TK_PRECEDING
            || (*pMWin).eStart as ::core::ffi::c_int == TK_FOLLOWING
        {
            (*pParse).nMem += 1;
            regStart = (*pParse).nMem;
        }
        if (*pMWin).eEnd as ::core::ffi::c_int == TK_PRECEDING
            || (*pMWin).eEnd as ::core::ffi::c_int == TK_FOLLOWING
        {
            (*pParse).nMem += 1;
            regEnd = (*pParse).nMem;
        }
        if (*pMWin).eFrmType as ::core::ffi::c_int != TK_ROWS {
            let mut nPeer: ::core::ffi::c_int = if !pOrderBy.is_null() {
                (*pOrderBy).nExpr
            } else {
                0 as ::core::ffi::c_int
            };
            regNewPeer = regNew + (*pMWin).nBufferCol;
            if !(*pMWin).pPartition.is_null() {
                regNewPeer += (*(*pMWin).pPartition).nExpr;
            }
            regPeer = (*pParse).nMem + 1 as ::core::ffi::c_int;
            (*pParse).nMem += nPeer;
            s.start.reg = (*pParse).nMem + 1 as ::core::ffi::c_int;
            (*pParse).nMem += nPeer;
            s.current.reg = (*pParse).nMem + 1 as ::core::ffi::c_int;
            (*pParse).nMem += nPeer;
            s.end.reg = (*pParse).nMem + 1 as ::core::ffi::c_int;
            (*pParse).nMem += nPeer;
        }
        iInput = 0 as ::core::ffi::c_int;
        while iInput < nInput {
            sqlite3VdbeAddOp3(v, OP_Column, csrInput, iInput, regNew + iInput);
            iInput += 1;
        }
        sqlite3VdbeAddOp3(v, OP_MakeRecord, regNew, nInput, regRecord);
        if !(*pMWin).pPartition.is_null() {
            let mut addr: ::core::ffi::c_int = 0;
            let mut pPart: *mut ExprList = (*pMWin).pPartition;
            let mut nPart: ::core::ffi::c_int = (*pPart).nExpr;
            let mut regNewPart: ::core::ffi::c_int = regNew + (*pMWin).nBufferCol;
            let mut pKeyInfo: *mut KeyInfo = sqlite3KeyInfoFromExprList(
                pParse,
                pPart,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            (*pParse).nMem += 1;
            regFlushPart = (*pParse).nMem;
            addr = sqlite3VdbeAddOp3(v, OP_Compare, regNewPart, (*pMWin).regPart, nPart);
            sqlite3VdbeAppendP4(v, pKeyInfo as *mut ::core::ffi::c_void, P4_KEYINFO);
            sqlite3VdbeAddOp3(
                v,
                OP_Jump,
                addr + 2 as ::core::ffi::c_int,
                addr + 4 as ::core::ffi::c_int,
                addr + 2 as ::core::ffi::c_int,
            );
            addrGosubFlush = sqlite3VdbeAddOp1(v, OP_Gosub, regFlushPart);
            sqlite3VdbeAddOp3(
                v,
                OP_Copy,
                regNewPart,
                (*pMWin).regPart,
                nPart - 1 as ::core::ffi::c_int,
            );
        }
        sqlite3VdbeAddOp2(v, OP_NewRowid, csrWrite, s.regRowid);
        sqlite3VdbeAddOp3(v, OP_Insert, csrWrite, regRecord, s.regRowid);
        addrNe = sqlite3VdbeAddOp3(
            v,
            OP_Ne,
            (*pMWin).regOne,
            0 as ::core::ffi::c_int,
            s.regRowid,
        );
        s.regArg = windowInitAccum(pParse, pMWin);
        if regStart != 0 {
            sqlite3ExprCode(pParse, (*pMWin).pStart, regStart);
            windowCheckValue(
                pParse,
                regStart,
                0 as ::core::ffi::c_int
                    + (if (*pMWin).eFrmType as ::core::ffi::c_int == TK_RANGE {
                        3 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }),
            );
        }
        if regEnd != 0 {
            sqlite3ExprCode(pParse, (*pMWin).pEnd, regEnd);
            windowCheckValue(
                pParse,
                regEnd,
                1 as ::core::ffi::c_int
                    + (if (*pMWin).eFrmType as ::core::ffi::c_int == TK_RANGE {
                        3 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }),
            );
        }
        if (*pMWin).eFrmType as ::core::ffi::c_int != TK_RANGE
            && (*pMWin).eStart as ::core::ffi::c_int
                == (*pMWin).eEnd as ::core::ffi::c_int && regStart != 0
        {
            let mut op: ::core::ffi::c_int = if (*pMWin).eStart as ::core::ffi::c_int
                == TK_FOLLOWING
            {
                OP_Ge
            } else {
                OP_Le
            };
            let mut addrGe: ::core::ffi::c_int = sqlite3VdbeAddOp3(
                v,
                op,
                regStart,
                0 as ::core::ffi::c_int,
                regEnd,
            );
            windowAggFinal(&raw mut s, 0 as ::core::ffi::c_int);
            sqlite3VdbeAddOp1(v, OP_Rewind, s.current.csr);
            windowReturnOneRow(&raw mut s);
            sqlite3VdbeAddOp1(v, OP_ResetSorter, s.current.csr);
            sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, lblWhereEnd);
            sqlite3VdbeJumpHere(v, addrGe);
        }
        if (*pMWin).eStart as ::core::ffi::c_int == TK_FOLLOWING
            && (*pMWin).eFrmType as ::core::ffi::c_int != TK_RANGE && regEnd != 0
        {
            sqlite3VdbeAddOp3(v, OP_Subtract, regStart, regEnd, regStart);
        }
        if (*pMWin).eStart as ::core::ffi::c_int != TK_UNBOUNDED {
            sqlite3VdbeAddOp1(v, OP_Rewind, s.start.csr);
        }
        sqlite3VdbeAddOp1(v, OP_Rewind, s.current.csr);
        sqlite3VdbeAddOp1(v, OP_Rewind, s.end.csr);
        if regPeer != 0 && !pOrderBy.is_null() {
            sqlite3VdbeAddOp3(
                v,
                OP_Copy,
                regNewPeer,
                regPeer,
                (*pOrderBy).nExpr - 1 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp3(
                v,
                OP_Copy,
                regPeer,
                s.start.reg,
                (*pOrderBy).nExpr - 1 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp3(
                v,
                OP_Copy,
                regPeer,
                s.current.reg,
                (*pOrderBy).nExpr - 1 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp3(
                v,
                OP_Copy,
                regPeer,
                s.end.reg,
                (*pOrderBy).nExpr - 1 as ::core::ffi::c_int,
            );
        }
        sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, lblWhereEnd);
        sqlite3VdbeJumpHere(v, addrNe);
        if regPeer != 0 {
            windowIfNewPeer(pParse, pOrderBy, regNewPeer, regPeer, lblWhereEnd);
        }
        if (*pMWin).eStart as ::core::ffi::c_int == TK_FOLLOWING {
            windowCodeOp(
                &raw mut s,
                WINDOW_AGGSTEP,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if (*pMWin).eEnd as ::core::ffi::c_int != TK_UNBOUNDED {
                if (*pMWin).eFrmType as ::core::ffi::c_int == TK_RANGE {
                    let mut lbl: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
                    let mut addrNext: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(v);
                    windowCodeRangeTest(
                        &raw mut s,
                        OP_Ge,
                        s.current.csr,
                        regEnd,
                        s.end.csr,
                        lbl,
                    );
                    windowCodeOp(
                        &raw mut s,
                        WINDOW_AGGINVERSE,
                        regStart,
                        0 as ::core::ffi::c_int,
                    );
                    windowCodeOp(
                        &raw mut s,
                        WINDOW_RETURN_ROW,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                    sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, addrNext);
                    sqlite3VdbeResolveLabel(v, lbl);
                } else {
                    windowCodeOp(
                        &raw mut s,
                        WINDOW_RETURN_ROW,
                        regEnd,
                        0 as ::core::ffi::c_int,
                    );
                    windowCodeOp(
                        &raw mut s,
                        WINDOW_AGGINVERSE,
                        regStart,
                        0 as ::core::ffi::c_int,
                    );
                }
            }
        } else if (*pMWin).eEnd as ::core::ffi::c_int == TK_PRECEDING {
            let mut bRPS: ::core::ffi::c_int = ((*pMWin).eStart as ::core::ffi::c_int
                == TK_PRECEDING && (*pMWin).eFrmType as ::core::ffi::c_int == TK_RANGE)
                as ::core::ffi::c_int;
            windowCodeOp(&raw mut s, WINDOW_AGGSTEP, regEnd, 0 as ::core::ffi::c_int);
            if bRPS != 0 {
                windowCodeOp(
                    &raw mut s,
                    WINDOW_AGGINVERSE,
                    regStart,
                    0 as ::core::ffi::c_int,
                );
            }
            windowCodeOp(
                &raw mut s,
                WINDOW_RETURN_ROW,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if bRPS == 0 {
                windowCodeOp(
                    &raw mut s,
                    WINDOW_AGGINVERSE,
                    regStart,
                    0 as ::core::ffi::c_int,
                );
            }
        } else {
            let mut addr_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            windowCodeOp(
                &raw mut s,
                WINDOW_AGGSTEP,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if (*pMWin).eEnd as ::core::ffi::c_int != TK_UNBOUNDED {
                if (*pMWin).eFrmType as ::core::ffi::c_int == TK_RANGE {
                    let mut lbl_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    addr_0 = sqlite3VdbeCurrentAddr(v);
                    if regEnd != 0 {
                        lbl_0 = sqlite3VdbeMakeLabel(pParse);
                        windowCodeRangeTest(
                            &raw mut s,
                            OP_Ge,
                            s.current.csr,
                            regEnd,
                            s.end.csr,
                            lbl_0,
                        );
                    }
                    windowCodeOp(
                        &raw mut s,
                        WINDOW_RETURN_ROW,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                    windowCodeOp(
                        &raw mut s,
                        WINDOW_AGGINVERSE,
                        regStart,
                        0 as ::core::ffi::c_int,
                    );
                    if regEnd != 0 {
                        sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, addr_0);
                        sqlite3VdbeResolveLabel(v, lbl_0);
                    }
                } else {
                    if regEnd != 0 {
                        addr_0 = sqlite3VdbeAddOp3(
                            v,
                            OP_IfPos,
                            regEnd,
                            0 as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                        );
                    }
                    windowCodeOp(
                        &raw mut s,
                        WINDOW_RETURN_ROW,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                    windowCodeOp(
                        &raw mut s,
                        WINDOW_AGGINVERSE,
                        regStart,
                        0 as ::core::ffi::c_int,
                    );
                    if regEnd != 0 {
                        sqlite3VdbeJumpHere(v, addr_0);
                    }
                }
            }
        }
        sqlite3VdbeResolveLabel(v, lblWhereEnd);
        sqlite3WhereEnd(pWInfo);
        if !(*pMWin).pPartition.is_null() {
            addrInteger = sqlite3VdbeAddOp2(
                v,
                OP_Integer,
                0 as ::core::ffi::c_int,
                regFlushPart,
            );
            sqlite3VdbeJumpHere(v, addrGosubFlush);
        }
        s.regRowid = 0 as ::core::ffi::c_int;
        addrEmpty = sqlite3VdbeAddOp1(v, OP_Rewind, csrWrite);
        if (*pMWin).eEnd as ::core::ffi::c_int == TK_PRECEDING {
            let mut bRPS_0: ::core::ffi::c_int = ((*pMWin).eStart as ::core::ffi::c_int
                == TK_PRECEDING && (*pMWin).eFrmType as ::core::ffi::c_int == TK_RANGE)
                as ::core::ffi::c_int;
            windowCodeOp(&raw mut s, WINDOW_AGGSTEP, regEnd, 0 as ::core::ffi::c_int);
            if bRPS_0 != 0 {
                windowCodeOp(
                    &raw mut s,
                    WINDOW_AGGINVERSE,
                    regStart,
                    0 as ::core::ffi::c_int,
                );
            }
            windowCodeOp(
                &raw mut s,
                WINDOW_RETURN_ROW,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        } else if (*pMWin).eStart as ::core::ffi::c_int == TK_FOLLOWING {
            let mut addrStart: ::core::ffi::c_int = 0;
            let mut addrBreak1: ::core::ffi::c_int = 0;
            let mut addrBreak2: ::core::ffi::c_int = 0;
            let mut addrBreak3: ::core::ffi::c_int = 0;
            windowCodeOp(
                &raw mut s,
                WINDOW_AGGSTEP,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if (*pMWin).eFrmType as ::core::ffi::c_int == TK_RANGE {
                addrStart = sqlite3VdbeCurrentAddr(v);
                addrBreak2 = windowCodeOp(
                    &raw mut s,
                    WINDOW_AGGINVERSE,
                    regStart,
                    1 as ::core::ffi::c_int,
                );
                addrBreak1 = windowCodeOp(
                    &raw mut s,
                    WINDOW_RETURN_ROW,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            } else if (*pMWin).eEnd as ::core::ffi::c_int == TK_UNBOUNDED {
                addrStart = sqlite3VdbeCurrentAddr(v);
                addrBreak1 = windowCodeOp(
                    &raw mut s,
                    WINDOW_RETURN_ROW,
                    regStart,
                    1 as ::core::ffi::c_int,
                );
                addrBreak2 = windowCodeOp(
                    &raw mut s,
                    WINDOW_AGGINVERSE,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            } else {
                sqlite3VdbeAddOp3(v, OP_Subtract, regStart, regEnd, regEnd);
                sqlite3VdbeAddOp2(v, OP_Integer, 0 as ::core::ffi::c_int, regStart);
                addrStart = sqlite3VdbeCurrentAddr(v);
                addrBreak1 = windowCodeOp(
                    &raw mut s,
                    WINDOW_RETURN_ROW,
                    regEnd,
                    1 as ::core::ffi::c_int,
                );
                addrBreak2 = windowCodeOp(
                    &raw mut s,
                    WINDOW_AGGINVERSE,
                    regStart,
                    1 as ::core::ffi::c_int,
                );
            }
            sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, addrStart);
            sqlite3VdbeJumpHere(v, addrBreak2);
            addrStart = sqlite3VdbeCurrentAddr(v);
            addrBreak3 = windowCodeOp(
                &raw mut s,
                WINDOW_RETURN_ROW,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, addrStart);
            sqlite3VdbeJumpHere(v, addrBreak1);
            sqlite3VdbeJumpHere(v, addrBreak3);
        } else {
            let mut addrBreak: ::core::ffi::c_int = 0;
            let mut addrStart_0: ::core::ffi::c_int = 0;
            windowCodeOp(
                &raw mut s,
                WINDOW_AGGSTEP,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            addrStart_0 = sqlite3VdbeCurrentAddr(v);
            addrBreak = windowCodeOp(
                &raw mut s,
                WINDOW_RETURN_ROW,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            windowCodeOp(
                &raw mut s,
                WINDOW_AGGINVERSE,
                regStart,
                0 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, addrStart_0);
            sqlite3VdbeJumpHere(v, addrBreak);
        }
        sqlite3VdbeJumpHere(v, addrEmpty);
        sqlite3VdbeAddOp1(v, OP_ResetSorter, s.current.csr);
        if !(*pMWin).pPartition.is_null() {
            if (*pMWin).regStartRowid != 0 {
                sqlite3VdbeAddOp2(
                    v,
                    OP_Integer,
                    1 as ::core::ffi::c_int,
                    (*pMWin).regStartRowid,
                );
                sqlite3VdbeAddOp2(
                    v,
                    OP_Integer,
                    0 as ::core::ffi::c_int,
                    (*pMWin).regEndRowid,
                );
            }
            sqlite3VdbeChangeP1(v, addrInteger, sqlite3VdbeCurrentAddr(v));
            sqlite3VdbeAddOp1(v, OP_Return, regFlushPart);
        }
    }
}
pub const P4_STATIC: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const P4_COLLSEQ: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const P4_FUNCDEF: ::core::ffi::c_int = -7 as ::core::ffi::c_int;
pub const P4_KEYINFO: ::core::ffi::c_int = -8 as ::core::ffi::c_int;
pub const TK_NO: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
pub const TK_ROWS: ::core::ffi::c_int = 77 as ::core::ffi::c_int;
pub const TK_CURRENT: ::core::ffi::c_int = 86 as ::core::ffi::c_int;
pub const TK_FOLLOWING: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
pub const TK_PRECEDING: ::core::ffi::c_int = 89 as ::core::ffi::c_int;
pub const TK_RANGE: ::core::ffi::c_int = 90 as ::core::ffi::c_int;
pub const TK_UNBOUNDED: ::core::ffi::c_int = 91 as ::core::ffi::c_int;
pub const TK_GROUPS: ::core::ffi::c_int = 93 as ::core::ffi::c_int;
pub const TK_TIES: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_INTEGER: ::core::ffi::c_int = 156 as ::core::ffi::c_int;
pub const TK_FILTER: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const TK_COLUMN: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
pub const TK_AGG_FUNCTION: ::core::ffi::c_int = 169 as ::core::ffi::c_int;
pub const TK_FUNCTION: ::core::ffi::c_int = 172 as ::core::ffi::c_int;
pub const TK_IF_NULL_ROW: ::core::ffi::c_int = 179;
pub const SQLITE_FUNC_NEEDCOLL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_FUNC_MINMAX: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_WINDOW: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_BUILTIN: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
pub const SQLITE_AFF_NONE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_NULLEQ: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_Ephemeral: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_DESC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_BIGNULL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const EP_Distinct: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const EP_Collate: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const EP_IntValue: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const EP_IsTrue: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const EP_IsFalse: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const SF_Aggregate: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SF_Expanded: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SF_WinRewrite: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const SF_MultiPart: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const SF_OrderByReqd: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const PARSE_MODE_RENAME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OPFLAG_SAVEPOSITION: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WRC_Continue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WRC_Prune: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WRC_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
