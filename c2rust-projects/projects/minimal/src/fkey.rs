use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type RenameToken;
    pub type Vdbe;
    pub type sqlite3_mutex;
    pub type WhereInfo;
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
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
    fn sqlite3VdbeGoto(_: *mut Vdbe, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    fn sqlite3VdbeChangeP5(_: *mut Vdbe, P5: u16_0);
    fn sqlite3VdbeJumpHere(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeJumpHereOrPopInst(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeSetP4KeyInfo(_: *mut Parse, _: *mut Index);
    fn sqlite3VdbeMakeLabel(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3VdbeResolveLabel(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeCurrentAddr(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3TokenInit(_: *mut Token, _: *mut ::core::ffi::c_char);
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
    fn sqlite3PExpr(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *mut Expr,
        _: *mut Expr,
    ) -> *mut Expr;
    fn sqlite3ExprAnd(_: *mut Parse, _: *mut Expr, _: *mut Expr) -> *mut Expr;
    fn sqlite3ExprDelete(_: *mut sqlite3, _: *mut Expr);
    fn sqlite3ExprListAppend(
        _: *mut Parse,
        _: *mut ExprList,
        _: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListSetName(
        _: *mut Parse,
        _: *mut ExprList,
        _: *const Token,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprListDelete(_: *mut sqlite3, _: *mut ExprList);
    fn sqlite3ColumnExpr(_: *mut Table, _: *mut Column) -> *mut Expr;
    fn sqlite3ColumnColl(_: *mut Column) -> *const ::core::ffi::c_char;
    fn sqlite3TableColumnToStorage(_: *mut Table, _: i16_0) -> i16_0;
    fn sqlite3SrcListAppend(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Token,
        _: *mut Token,
    ) -> *mut SrcList;
    fn sqlite3SrcListDelete(_: *mut sqlite3, _: *mut SrcList);
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
    fn sqlite3OpenTable(
        _: *mut Parse,
        iCur: ::core::ffi::c_int,
        iDb: ::core::ffi::c_int,
        _: *mut Table,
        _: ::core::ffi::c_int,
    );
    fn sqlite3DeleteFrom(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Expr,
        _: *mut ExprList,
        _: *mut Expr,
    );
    fn sqlite3WhereBegin(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Expr,
        _: *mut ExprList,
        _: *mut ExprList,
        _: *mut Select,
        _: u16_0,
        _: ::core::ffi::c_int,
    ) -> *mut WhereInfo;
    fn sqlite3WhereEnd(_: *mut WhereInfo);
    fn sqlite3FindTable(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut Table;
    fn sqlite3LocateTable(
        _: *mut Parse,
        flags: u32_0,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut Table;
    fn sqlite3GetVdbe(_: *mut Parse) -> *mut Vdbe;
    fn sqlite3MayAbort(_: *mut Parse);
    fn sqlite3HaltConstraint(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: i8_0,
        _: u8_0,
    );
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
    fn sqlite3SrcListDup(
        _: *mut sqlite3,
        _: *const SrcList,
        _: ::core::ffi::c_int,
    ) -> *mut SrcList;
    fn sqlite3SelectDup(
        _: *mut sqlite3,
        _: *const Select,
        _: ::core::ffi::c_int,
    ) -> *mut Select;
    fn sqlite3CodeRowTriggerDirect(
        _: *mut Parse,
        _: *mut Trigger,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3AuthReadCol(
        _: *mut Parse,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3IndexAffinityStr(
        _: *mut sqlite3,
        _: *mut Index,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3ExprAddCollateString(
        _: *const Parse,
        _: *mut Expr,
        _: *const ::core::ffi::c_char,
    ) -> *mut Expr;
    static sqlite3StrBINARY: [::core::ffi::c_char; 0];
    fn sqlite3ResolveExprNames(_: *mut NameContext, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
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
pub type i8_0 = ::core::ffi::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameContext {
    pub pParse: *mut Parse,
    pub pSrcList: *mut SrcList,
    pub uNC: C2Rust_Unnamed_22,
    pub pNext: *mut NameContext,
    pub nRef: ::core::ffi::c_int,
    pub nNcErr: ::core::ffi::c_int,
    pub ncFlags: ::core::ffi::c_int,
    pub nNestedSelect: u32_0,
    pub pWinSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_22 {
    pub pEList: *mut ExprList,
    pub pAggInfo: *mut AggInfo,
    pub pUpsert: *mut Upsert,
    pub iBaseReg: ::core::ffi::c_int

}
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_FOREIGNKEY: ::core::ffi::c_int = SQLITE_CONSTRAINT
    | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IGNORE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OP_MustBeInt: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const OP_Found: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const OP_NotExists: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const OP_FkIfZero: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const OP_IsNull: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const OP_Ne: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const OP_Eq: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const OP_Copy: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
pub const OP_SCopy: ::core::ffi::c_int = 82 as ::core::ffi::c_int;
pub const OP_Affinity: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
pub const OP_OpenRead: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const OP_Close: ::core::ffi::c_int = 123 as ::core::ffi::c_int;
pub const OP_FkCounter: ::core::ffi::c_int = 159 as ::core::ffi::c_int;
pub const P4_STATIC: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const P5_ConstraintFK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3FkLocateIndex(
    mut pParse: *mut Parse,
    mut pParent: *mut Table,
    mut pFKey: *mut FKey,
    mut ppIdx: *mut *mut Index,
    mut paiCol: *mut *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut aiCol: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        let mut nCol: ::core::ffi::c_int = (*pFKey).nCol;
        let mut zKey: *mut ::core::ffi::c_char = (*(&raw mut (*pFKey).aCol
            as *mut sColMap)
            .offset(0 as ::core::ffi::c_int as isize))
            .zCol;
        if nCol == 1 as ::core::ffi::c_int {
            if (*pParent).iPKey as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                if zKey.is_null() {
                    return 0 as ::core::ffi::c_int;
                }
                if sqlite3StrICmp(
                    (*(*pParent).aCol.offset((*pParent).iPKey as isize)).zCnName,
                    zKey,
                ) == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
        } else if !paiCol.is_null() {
            aiCol = sqlite3DbMallocRawNN(
                (*pParse).db,
                (nCol as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                    as u64_0,
            ) as *mut ::core::ffi::c_int;
            if aiCol.is_null() {
                return 1 as ::core::ffi::c_int;
            }
            *paiCol = aiCol;
        }
        pIdx = (*pParent).pIndex;
        while !pIdx.is_null() {
            if (*pIdx).nKeyCol as ::core::ffi::c_int == nCol
                && (*pIdx).onError as ::core::ffi::c_int != OE_None
                && (*pIdx).pPartIdxWhere.is_null()
            {
                if zKey.is_null() {
                    if (*pIdx).idxType() as ::core::ffi::c_int
                        == SQLITE_IDXTYPE_PRIMARYKEY
                    {
                        if !aiCol.is_null() {
                            let mut i: ::core::ffi::c_int = 0;
                            i = 0 as ::core::ffi::c_int;
                            while i < nCol {
                                *aiCol.offset(i as isize) = (*(&raw mut (*pFKey).aCol
                                    as *mut sColMap)
                                    .offset(i as isize))
                                    .iFrom;
                                i += 1;
                            }
                        }
                        break;
                    }
                } else {
                    let mut i_0: ::core::ffi::c_int = 0;
                    let mut j: ::core::ffi::c_int = 0;
                    i_0 = 0 as ::core::ffi::c_int;
                    while i_0 < nCol {
                        let mut iCol: i16_0 = *(*pIdx).aiColumn.offset(i_0 as isize);
                        let mut zDfltColl: *const ::core::ffi::c_char = ::core::ptr::null::<
                            ::core::ffi::c_char,
                        >();
                        let mut zIdxCol: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        if (iCol as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                            break;
                        }
                        zDfltColl = sqlite3ColumnColl(
                            (*pParent).aCol.offset(iCol as isize) as *mut Column,
                        );
                        if zDfltColl.is_null() {
                            zDfltColl = &raw const sqlite3StrBINARY
                                as *const ::core::ffi::c_char;
                        }
                        if sqlite3StrICmp(
                            *(*pIdx).azColl.offset(i_0 as isize),
                            zDfltColl,
                        ) != 0
                        {
                            break;
                        }
                        zIdxCol = (*(*pParent).aCol.offset(iCol as isize)).zCnName;
                        j = 0 as ::core::ffi::c_int;
                        while j < nCol {
                            if sqlite3StrICmp(
                                (*(&raw mut (*pFKey).aCol as *mut sColMap)
                                    .offset(j as isize))
                                    .zCol,
                                zIdxCol,
                            ) == 0 as ::core::ffi::c_int
                            {
                                if !aiCol.is_null() {
                                    *aiCol.offset(i_0 as isize) = (*(&raw mut (*pFKey).aCol
                                        as *mut sColMap)
                                        .offset(j as isize))
                                        .iFrom;
                                }
                                break;
                            } else {
                                j += 1;
                            }
                        }
                        if j == nCol {
                            break;
                        }
                        i_0 += 1;
                    }
                    if i_0 == nCol {
                        break;
                    }
                }
            }
            pIdx = (*pIdx).pNext;
        }
        if pIdx.is_null() {
            if (*pParse).disableTriggers == 0 {
                sqlite3ErrorMsg(
                    pParse,
                    b"foreign key mismatch - \"%w\" referencing \"%w\"\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(*pFKey).pFrom).zName,
                    (*pFKey).zTo,
                );
            }
            sqlite3DbFree((*pParse).db, aiCol as *mut ::core::ffi::c_void);
            return 1 as ::core::ffi::c_int;
        }
        *ppIdx = pIdx;
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fkLookupParent(
    mut pParse: *mut Parse,
    mut iDb: ::core::ffi::c_int,
    mut pTab: *mut Table,
    mut pIdx: *mut Index,
    mut pFKey: *mut FKey,
    mut aiCol: *mut ::core::ffi::c_int,
    mut regData: ::core::ffi::c_int,
    mut nIncr: ::core::ffi::c_int,
    mut isIgnore: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        let mut iCur: ::core::ffi::c_int = (*pParse).nTab - 1 as ::core::ffi::c_int;
        let mut iOk: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
        if nIncr < 0 as ::core::ffi::c_int {
            sqlite3VdbeAddOp2(
                v,
                OP_FkIfZero,
                (*pFKey).isDeferred as ::core::ffi::c_int,
                iOk,
            );
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pFKey).nCol {
            let mut iReg: ::core::ffi::c_int = sqlite3TableColumnToStorage(
                (*pFKey).pFrom,
                *aiCol.offset(i as isize) as i16_0,
            ) as ::core::ffi::c_int + regData + 1 as ::core::ffi::c_int;
            sqlite3VdbeAddOp2(v, OP_IsNull, iReg, iOk);
            i += 1;
        }
        if isIgnore == 0 as ::core::ffi::c_int {
            if pIdx.is_null() {
                let mut iMustBeInt: ::core::ffi::c_int = 0;
                let mut regTemp: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                sqlite3VdbeAddOp2(
                    v,
                    OP_SCopy,
                    sqlite3TableColumnToStorage(
                        (*pFKey).pFrom,
                        *aiCol.offset(0 as ::core::ffi::c_int as isize) as i16_0,
                    ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int + regData,
                    regTemp,
                );
                iMustBeInt = sqlite3VdbeAddOp2(
                    v,
                    OP_MustBeInt,
                    regTemp,
                    0 as ::core::ffi::c_int,
                );
                if pTab == (*pFKey).pFrom && nIncr == 1 as ::core::ffi::c_int {
                    sqlite3VdbeAddOp3(v, OP_Eq, regData, iOk, regTemp);
                    sqlite3VdbeChangeP5(v, SQLITE_NOTNULL as u16_0);
                }
                sqlite3OpenTable(pParse, iCur, iDb, pTab, OP_OpenRead);
                sqlite3VdbeAddOp3(
                    v,
                    OP_NotExists,
                    iCur,
                    0 as ::core::ffi::c_int,
                    regTemp,
                );
                sqlite3VdbeGoto(v, iOk);
                sqlite3VdbeJumpHere(
                    v,
                    sqlite3VdbeCurrentAddr(v) - 2 as ::core::ffi::c_int,
                );
                sqlite3VdbeJumpHere(v, iMustBeInt);
                sqlite3ReleaseTempReg(pParse, regTemp);
            } else {
                let mut nCol: ::core::ffi::c_int = (*pFKey).nCol;
                let mut regTemp_0: ::core::ffi::c_int = sqlite3GetTempRange(
                    pParse,
                    nCol,
                );
                sqlite3VdbeAddOp3(
                    v,
                    OP_OpenRead,
                    iCur,
                    (*pIdx).tnum as ::core::ffi::c_int,
                    iDb,
                );
                sqlite3VdbeSetP4KeyInfo(pParse, pIdx);
                i = 0 as ::core::ffi::c_int;
                while i < nCol {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Copy,
                        sqlite3TableColumnToStorage(
                            (*pFKey).pFrom,
                            *aiCol.offset(i as isize) as i16_0,
                        ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int + regData,
                        regTemp_0 + i,
                    );
                    i += 1;
                }
                if pTab == (*pFKey).pFrom && nIncr == 1 as ::core::ffi::c_int {
                    let mut iJump: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(v) + nCol
                        + 1 as ::core::ffi::c_int;
                    i = 0 as ::core::ffi::c_int;
                    while i < nCol {
                        let mut iChild: ::core::ffi::c_int = sqlite3TableColumnToStorage(
                            (*pFKey).pFrom,
                            *aiCol.offset(i as isize) as i16_0,
                        ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int + regData;
                        let mut iParent: ::core::ffi::c_int = 1 as ::core::ffi::c_int
                            + regData;
                        iParent
                            += sqlite3TableColumnToStorage(
                                (*pIdx).pTable,
                                *(*pIdx).aiColumn.offset(i as isize),
                            ) as ::core::ffi::c_int;
                        if *(*pIdx).aiColumn.offset(i as isize) as ::core::ffi::c_int
                            == (*pTab).iPKey as ::core::ffi::c_int
                        {
                            iParent = regData;
                        }
                        sqlite3VdbeAddOp3(v, OP_Ne, iChild, iJump, iParent);
                        sqlite3VdbeChangeP5(v, SQLITE_JUMPIFNULL as u16_0);
                        i += 1;
                    }
                    sqlite3VdbeGoto(v, iOk);
                }
                sqlite3VdbeAddOp4(
                    v,
                    OP_Affinity,
                    regTemp_0,
                    nCol,
                    0 as ::core::ffi::c_int,
                    sqlite3IndexAffinityStr((*pParse).db, pIdx),
                    nCol,
                );
                sqlite3VdbeAddOp4Int(v, OP_Found, iCur, iOk, regTemp_0, nCol);
                sqlite3ReleaseTempRange(pParse, regTemp_0, nCol);
            }
        }
        if (*pFKey).isDeferred == 0
            && (*(*pParse).db).flags as ::core::ffi::c_ulonglong
                & SQLITE_DeferFKs as ::core::ffi::c_ulonglong == 0
            && (*pParse).pToplevel.is_null() && (*pParse).isMultiWrite == 0
        {
            sqlite3HaltConstraint(
                pParse,
                SQLITE_CONSTRAINT_FOREIGNKEY,
                OE_Abort,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                P4_STATIC as i8_0,
                P5_ConstraintFK as u8_0,
            );
        } else {
            if nIncr > 0 as ::core::ffi::c_int
                && (*pFKey).isDeferred as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                sqlite3MayAbort(pParse);
            }
            sqlite3VdbeAddOp2(
                v,
                OP_FkCounter,
                (*pFKey).isDeferred as ::core::ffi::c_int,
                nIncr,
            );
        }
        sqlite3VdbeResolveLabel(v, iOk);
        sqlite3VdbeAddOp1(v, OP_Close, iCur);
    }
}
unsafe extern "C" fn exprTableRegister(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut regBase: ::core::ffi::c_int,
    mut iCol: i16_0,
) -> *mut Expr {
    unsafe {
        let mut pExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        let mut zColl: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = (*pParse).db;
        pExpr = sqlite3Expr(db, TK_REGISTER, ::core::ptr::null::<::core::ffi::c_char>());
        if !pExpr.is_null() {
            if iCol as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                && iCol as ::core::ffi::c_int != (*pTab).iPKey as ::core::ffi::c_int
            {
                pCol = (*pTab).aCol.offset(iCol as isize) as *mut Column;
                (*pExpr).iTable = regBase
                    + sqlite3TableColumnToStorage(pTab, iCol) as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int;
                (*pExpr).affExpr = (*pCol).affinity;
                zColl = sqlite3ColumnColl(pCol);
                if zColl.is_null() {
                    zColl = (*(*db).pDfltColl).zName;
                }
                pExpr = sqlite3ExprAddCollateString(pParse, pExpr, zColl);
            } else {
                (*pExpr).iTable = regBase;
                (*pExpr).affExpr = SQLITE_AFF_INTEGER as ::core::ffi::c_char;
            }
        }
        return pExpr;
    }
}
unsafe extern "C" fn exprTableColumn(
    mut db: *mut sqlite3,
    mut pTab: *mut Table,
    mut iCursor: ::core::ffi::c_int,
    mut iCol: i16_0,
) -> *mut Expr {
    unsafe {
        let mut pExpr: *mut Expr = sqlite3Expr(
            db,
            TK_COLUMN,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if !pExpr.is_null() {
            (*pExpr).y.pTab = pTab;
            (*pExpr).iTable = iCursor;
            (*pExpr).iColumn = iCol as ynVar;
        }
        return pExpr;
    }
}
unsafe extern "C" fn fkScanChildren(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pTab: *mut Table,
    mut pIdx: *mut Index,
    mut pFKey: *mut FKey,
    mut aiCol: *mut ::core::ffi::c_int,
    mut regData: ::core::ffi::c_int,
    mut nIncr: ::core::ffi::c_int,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut i: ::core::ffi::c_int = 0;
        let mut pWhere: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut sNameContext: NameContext = NameContext {
            pParse: ::core::ptr::null_mut::<Parse>(),
            pSrcList: ::core::ptr::null_mut::<SrcList>(),
            uNC: C2Rust_Unnamed_22 {
                pEList: ::core::ptr::null_mut::<ExprList>()

            },
            pNext: ::core::ptr::null_mut::<NameContext>(),
            nRef: 0,
            nNcErr: 0,
            ncFlags: 0,
            nNestedSelect: 0,
            pWinSelect: ::core::ptr::null_mut::<Select>()

        };
        let mut pWInfo: *mut WhereInfo = ::core::ptr::null_mut::<WhereInfo>();
        let mut iFkIfZero: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        if nIncr < 0 as ::core::ffi::c_int {
            iFkIfZero = sqlite3VdbeAddOp2(
                v,
                OP_FkIfZero,
                (*pFKey).isDeferred as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pFKey).nCol {
            let mut pLeft: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pRight: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pEq: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut iCol: i16_0 = 0;
            let mut zCol: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            iCol = (if !pIdx.is_null() {
                *(*pIdx).aiColumn.offset(i as isize) as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            }) as i16_0;
            pLeft = exprTableRegister(pParse, pTab, regData, iCol);
            iCol = (if !aiCol.is_null() {
                *aiCol.offset(i as isize)
            } else {
                (*(&raw mut (*pFKey).aCol as *mut sColMap)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .iFrom
            }) as i16_0;
            zCol = (*(*(*pFKey).pFrom).aCol.offset(iCol as isize)).zCnName;
            pRight = sqlite3Expr(db, TK_ID, zCol);
            pEq = sqlite3PExpr(pParse, TK_EQ, pLeft, pRight);
            pWhere = sqlite3ExprAnd(pParse, pWhere, pEq);
            i += 1;
        }
        if pTab == (*pFKey).pFrom && nIncr > 0 as ::core::ffi::c_int {
            let mut pNe: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pLeft_0: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pRight_0: *mut Expr = ::core::ptr::null_mut::<Expr>();
            if (*pTab).tabFlags as ::core::ffi::c_uint
                & TF_WithoutRowid as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            {
                pLeft_0 = exprTableRegister(
                    pParse,
                    pTab,
                    regData,
                    -1 as ::core::ffi::c_int as i16_0,
                );
                pRight_0 = exprTableColumn(
                    db,
                    pTab,
                    (*(&raw mut (*pSrc).a as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .iCursor,
                    -1 as ::core::ffi::c_int as i16_0,
                );
                pNe = sqlite3PExpr(pParse, TK_NE, pLeft_0, pRight_0);
            } else {
                let mut pEq_0: *mut Expr = ::core::ptr::null_mut::<Expr>();
                let mut pAll: *mut Expr = ::core::ptr::null_mut::<Expr>();
                i = 0 as ::core::ffi::c_int;
                while i < (*pIdx).nKeyCol as ::core::ffi::c_int {
                    let mut iCol_0: i16_0 = *(*pIdx).aiColumn.offset(i as isize);
                    pLeft_0 = exprTableRegister(pParse, pTab, regData, iCol_0);
                    pRight_0 = sqlite3Expr(
                        db,
                        TK_ID,
                        (*(*pTab).aCol.offset(iCol_0 as isize)).zCnName,
                    );
                    pEq_0 = sqlite3PExpr(pParse, TK_IS, pLeft_0, pRight_0);
                    pAll = sqlite3ExprAnd(pParse, pAll, pEq_0);
                    i += 1;
                }
                pNe = sqlite3PExpr(
                    pParse,
                    TK_NOT,
                    pAll,
                    ::core::ptr::null_mut::<Expr>(),
                );
            }
            pWhere = sqlite3ExprAnd(pParse, pWhere, pNe);
        }
        memset(
            &raw mut sNameContext as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<NameContext>() as size_t,
        );
        sNameContext.pSrcList = pSrc;
        sNameContext.pParse = pParse;
        sqlite3ResolveExprNames(&raw mut sNameContext, pWhere);
        if (*pParse).nErr == 0 as ::core::ffi::c_int {
            pWInfo = sqlite3WhereBegin(
                pParse,
                pSrc,
                pWhere,
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Select>(),
                0 as u16_0,
                0 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp2(
                v,
                OP_FkCounter,
                (*pFKey).isDeferred as ::core::ffi::c_int,
                nIncr,
            );
            if !pWInfo.is_null() {
                sqlite3WhereEnd(pWInfo);
            }
        }
        sqlite3ExprDelete(db, pWhere);
        if iFkIfZero != 0 {
            sqlite3VdbeJumpHereOrPopInst(v, iFkIfZero);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FkReferences(mut pTab: *mut Table) -> *mut FKey {
    unsafe {
        return sqlite3HashFind(&raw mut (*(*pTab).pSchema).fkeyHash, (*pTab).zName)
            as *mut FKey;
    }
}
unsafe extern "C" fn fkTriggerDelete(mut dbMem: *mut sqlite3, mut p: *mut Trigger) {
    unsafe {
        if !p.is_null() {
            let mut pStep: *mut TriggerStep = (*p).step_list;
            sqlite3ExprDelete(dbMem, (*pStep).pWhere);
            sqlite3ExprListDelete(dbMem, (*pStep).pExprList);
            sqlite3SelectDelete(dbMem, (*pStep).pSelect);
            sqlite3ExprDelete(dbMem, (*p).pWhen);
            sqlite3DbFree(dbMem, p as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FkClearTriggerCache(
    mut db: *mut sqlite3,
    mut iDb: ::core::ffi::c_int,
) {
    unsafe {
        let mut k: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut pHash: *mut Hash = &raw mut (*(*(*db).aDb.offset(iDb as isize)).pSchema)
            .tblHash;
        k = (*pHash).first;
        while !k.is_null() {
            let mut pTab: *mut Table = (*k).data as *mut Table;
            let mut pFKey: *mut FKey = ::core::ptr::null_mut::<FKey>();
            if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_NORM {
                pFKey = (*pTab).u.tab.pFKey;
                while !pFKey.is_null() {
                    fkTriggerDelete(
                        db,
                        (*pFKey).apTrigger[0 as ::core::ffi::c_int as usize],
                    );
                    (*pFKey).apTrigger[0 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<
                        Trigger,
                    >();
                    fkTriggerDelete(
                        db,
                        (*pFKey).apTrigger[1 as ::core::ffi::c_int as usize],
                    );
                    (*pFKey).apTrigger[1 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<
                        Trigger,
                    >();
                    pFKey = (*pFKey).pNextFrom;
                }
            }
            k = (*k).next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FkDropTable(
    mut pParse: *mut Parse,
    mut pName: *mut SrcList,
    mut pTab: *mut Table,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        if (*db).flags as ::core::ffi::c_ulonglong
            & SQLITE_ForeignKeys as ::core::ffi::c_ulonglong != 0
            && (*pTab).eTabType as ::core::ffi::c_int == TABTYP_NORM
        {
            let mut iSkip: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
            if sqlite3FkReferences(pTab).is_null() {
                let mut p: *mut FKey = ::core::ptr::null_mut::<FKey>();
                p = (*pTab).u.tab.pFKey;
                while !p.is_null() {
                    if (*p).isDeferred as ::core::ffi::c_int != 0
                        || (*db).flags as ::core::ffi::c_ulonglong
                            & SQLITE_DeferFKs as ::core::ffi::c_ulonglong != 0
                    {
                        break;
                    }
                    p = (*p).pNextFrom;
                }
                if p.is_null() {
                    return;
                }
                iSkip = sqlite3VdbeMakeLabel(pParse);
                sqlite3VdbeAddOp2(v, OP_FkIfZero, 1 as ::core::ffi::c_int, iSkip);
            }
            (*pParse).disableTriggers = 1 as u8_0;
            sqlite3DeleteFrom(
                pParse,
                sqlite3SrcListDup(db, pName, 0 as ::core::ffi::c_int),
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Expr>(),
            );
            (*pParse).disableTriggers = 0 as u8_0;
            if (*db).flags as ::core::ffi::c_ulonglong
                & SQLITE_DeferFKs as ::core::ffi::c_ulonglong
                == 0 as ::core::ffi::c_ulonglong
            {
                sqlite3VdbeAddOp2(
                    v,
                    OP_FkIfZero,
                    0 as ::core::ffi::c_int,
                    sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
                );
                sqlite3HaltConstraint(
                    pParse,
                    SQLITE_CONSTRAINT_FOREIGNKEY,
                    OE_Abort,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    P4_STATIC as i8_0,
                    P5_ConstraintFK as u8_0,
                );
            }
            if iSkip != 0 {
                sqlite3VdbeResolveLabel(v, iSkip);
            }
        }
    }
}
unsafe extern "C" fn fkChildIsModified(
    mut pTab: *mut Table,
    mut p: *mut FKey,
    mut aChange: *mut ::core::ffi::c_int,
    mut bChngRowid: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nCol {
            let mut iChildKey: ::core::ffi::c_int = (*(&raw mut (*p).aCol
                as *mut sColMap)
                .offset(i as isize))
                .iFrom;
            if *aChange.offset(iChildKey as isize) >= 0 as ::core::ffi::c_int {
                return 1 as ::core::ffi::c_int;
            }
            if iChildKey == (*pTab).iPKey as ::core::ffi::c_int && bChngRowid != 0 {
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fkParentIsModified(
    mut pTab: *mut Table,
    mut p: *mut FKey,
    mut aChange: *mut ::core::ffi::c_int,
    mut bChngRowid: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nCol {
            let mut zKey: *mut ::core::ffi::c_char = (*(&raw mut (*p).aCol
                as *mut sColMap)
                .offset(i as isize))
                .zCol;
            let mut iKey: ::core::ffi::c_int = 0;
            iKey = 0 as ::core::ffi::c_int;
            while iKey < (*pTab).nCol as ::core::ffi::c_int {
                if *aChange.offset(iKey as isize) >= 0 as ::core::ffi::c_int
                    || iKey == (*pTab).iPKey as ::core::ffi::c_int && bChngRowid != 0
                {
                    let mut pCol: *mut Column = (*pTab).aCol.offset(iKey as isize)
                        as *mut Column;
                    if !zKey.is_null() {
                        if 0 as ::core::ffi::c_int
                            == sqlite3StrICmp((*pCol).zCnName, zKey)
                        {
                            return 1 as ::core::ffi::c_int;
                        }
                    } else if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_PRIMKEY
                        != 0
                    {
                        return 1 as ::core::ffi::c_int
                    }
                }
                iKey += 1;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn isSetNullAction(
    mut pParse: *mut Parse,
    mut pFKey: *mut FKey,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTop: *mut Parse = if !(*pParse).pToplevel.is_null() {
            (*pParse).pToplevel
        } else {
            pParse
        };
        if !(*pTop).pTriggerPrg.is_null() {
            let mut p: *mut Trigger = (*(*pTop).pTriggerPrg).pTrigger;
            if p == (*pFKey).apTrigger[0 as ::core::ffi::c_int as usize]
                && (*pFKey).aAction[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == OE_SetNull
                || p == (*pFKey).apTrigger[1 as ::core::ffi::c_int as usize]
                    && (*pFKey).aAction[1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int == OE_SetNull
            {
                return 1 as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FkCheck(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut regOld: ::core::ffi::c_int,
    mut regNew: ::core::ffi::c_int,
    mut aChange: *mut ::core::ffi::c_int,
    mut bChngRowid: ::core::ffi::c_int,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pFKey: *mut FKey = ::core::ptr::null_mut::<FKey>();
        let mut iDb: ::core::ffi::c_int = 0;
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut isIgnoreErrors: ::core::ffi::c_int = (*pParse).disableTriggers
            as ::core::ffi::c_int;
        if (*db).flags as ::core::ffi::c_ulonglong
            & SQLITE_ForeignKeys as ::core::ffi::c_ulonglong
            == 0 as ::core::ffi::c_ulonglong
        {
            return;
        }
        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_NORM) {
            return;
        }
        iDb = sqlite3SchemaToIndex(db, (*pTab).pSchema);
        zDb = (*(*db).aDb.offset(iDb as isize)).zDbSName;
        pFKey = (*pTab).u.tab.pFKey;
        while !pFKey.is_null() {
            let mut pTo: *mut Table = ::core::ptr::null_mut::<Table>();
            let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
            let mut aiFree: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
                ::core::ffi::c_int,
            >();
            let mut aiCol: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
                ::core::ffi::c_int,
            >();
            let mut iCol: ::core::ffi::c_int = 0;
            let mut i: ::core::ffi::c_int = 0;
            let mut bIgnore: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if !(!aChange.is_null()
                && sqlite3_stricmp((*pTab).zName, (*pFKey).zTo)
                    != 0 as ::core::ffi::c_int
                && fkChildIsModified(pTab, pFKey, aChange, bChngRowid)
                    == 0 as ::core::ffi::c_int)
            {
                if (*pParse).disableTriggers != 0 {
                    pTo = sqlite3FindTable(db, (*pFKey).zTo, zDb);
                } else {
                    pTo = sqlite3LocateTable(pParse, 0 as u32_0, (*pFKey).zTo, zDb);
                }
                if pTo.is_null()
                    || sqlite3FkLocateIndex(
                        pParse,
                        pTo,
                        pFKey,
                        &raw mut pIdx,
                        &raw mut aiFree,
                    ) != 0
                {
                    if isIgnoreErrors == 0
                        || (*db).mallocFailed as ::core::ffi::c_int != 0
                    {
                        return;
                    }
                    if pTo.is_null() {
                        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
                        let mut iJump: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(v)
                            + (*pFKey).nCol + 1 as ::core::ffi::c_int;
                        i = 0 as ::core::ffi::c_int;
                        while i < (*pFKey).nCol {
                            let mut iFromCol: ::core::ffi::c_int = 0;
                            let mut iReg: ::core::ffi::c_int = 0;
                            iFromCol = (*(&raw mut (*pFKey).aCol as *mut sColMap)
                                .offset(i as isize))
                                .iFrom;
                            iReg = sqlite3TableColumnToStorage(
                                (*pFKey).pFrom,
                                iFromCol as i16_0,
                            ) as ::core::ffi::c_int + regOld + 1 as ::core::ffi::c_int;
                            sqlite3VdbeAddOp2(v, OP_IsNull, iReg, iJump);
                            i += 1;
                        }
                        sqlite3VdbeAddOp2(
                            v,
                            OP_FkCounter,
                            (*pFKey).isDeferred as ::core::ffi::c_int,
                            -1 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    if !aiFree.is_null() {
                        aiCol = aiFree;
                    } else {
                        iCol = (*(&raw mut (*pFKey).aCol as *mut sColMap)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .iFrom;
                        aiCol = &raw mut iCol;
                    }
                    i = 0 as ::core::ffi::c_int;
                    while i < (*pFKey).nCol {
                        if *aiCol.offset(i as isize)
                            == (*pTab).iPKey as ::core::ffi::c_int
                        {
                            *aiCol.offset(i as isize) = -1 as ::core::ffi::c_int;
                        }
                        if (*db).xAuth.is_some() {
                            let mut rcauth: ::core::ffi::c_int = 0;
                            let mut zCol: *mut ::core::ffi::c_char = (*(*pTo)
                                .aCol
                                .offset(
                                    (if !pIdx.is_null() {
                                        *(*pIdx).aiColumn.offset(i as isize) as ::core::ffi::c_int
                                    } else {
                                        (*pTo).iPKey as ::core::ffi::c_int
                                    }) as isize,
                                ))
                                .zCnName;
                            rcauth = sqlite3AuthReadCol(pParse, (*pTo).zName, zCol, iDb);
                            bIgnore = (rcauth == SQLITE_IGNORE) as ::core::ffi::c_int;
                        }
                        i += 1;
                    }
                    (*pParse).nTab += 1;
                    if regOld != 0 as ::core::ffi::c_int {
                        fkLookupParent(
                            pParse,
                            iDb,
                            pTo,
                            pIdx,
                            pFKey,
                            aiCol,
                            regOld,
                            -1 as ::core::ffi::c_int,
                            bIgnore,
                        );
                    }
                    if regNew != 0 as ::core::ffi::c_int
                        && isSetNullAction(pParse, pFKey) == 0
                    {
                        fkLookupParent(
                            pParse,
                            iDb,
                            pTo,
                            pIdx,
                            pFKey,
                            aiCol,
                            regNew,
                            1 as ::core::ffi::c_int,
                            bIgnore,
                        );
                    }
                    sqlite3DbFree(db, aiFree as *mut ::core::ffi::c_void);
                }
            }
            pFKey = (*pFKey).pNextFrom;
        }
        pFKey = sqlite3FkReferences(pTab);
        while !pFKey.is_null() {
            let mut pIdx_0: *mut Index = ::core::ptr::null_mut::<Index>();
            let mut pSrc: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
            let mut aiCol_0: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
                ::core::ffi::c_int,
            >();
            if !(!aChange.is_null()
                && fkParentIsModified(pTab, pFKey, aChange, bChngRowid)
                    == 0 as ::core::ffi::c_int)
            {
                if !((*pFKey).isDeferred == 0
                    && (*db).flags as ::core::ffi::c_ulonglong
                        & SQLITE_DeferFKs as ::core::ffi::c_ulonglong == 0
                    && (*pParse).pToplevel.is_null() && (*pParse).isMultiWrite == 0)
                {
                    if sqlite3FkLocateIndex(
                        pParse,
                        pTab,
                        pFKey,
                        &raw mut pIdx_0,
                        &raw mut aiCol_0,
                    ) != 0
                    {
                        if isIgnoreErrors == 0
                            || (*db).mallocFailed as ::core::ffi::c_int != 0
                        {
                            return;
                        }
                    } else {
                        pSrc = sqlite3SrcListAppend(
                            pParse,
                            ::core::ptr::null_mut::<SrcList>(),
                            ::core::ptr::null_mut::<Token>(),
                            ::core::ptr::null_mut::<Token>(),
                        );
                        if !pSrc.is_null() {
                            let mut pItem: *mut SrcItem = &raw mut (*pSrc).a
                                as *mut SrcItem;
                            (*pItem).pSTab = (*pFKey).pFrom;
                            (*pItem).zName = (*(*pFKey).pFrom).zName;
                            (*(*pItem).pSTab).nTabRef = (*(*pItem).pSTab)
                                .nTabRef
                                .wrapping_add(1);
                            let c2rust_fresh0 = (*pParse).nTab;
                            (*pParse).nTab = (*pParse).nTab + 1;
                            (*pItem).iCursor = c2rust_fresh0;
                            if regNew != 0 as ::core::ffi::c_int {
                                fkScanChildren(
                                    pParse,
                                    pSrc,
                                    pTab,
                                    pIdx_0,
                                    pFKey,
                                    aiCol_0,
                                    regNew,
                                    -1 as ::core::ffi::c_int,
                                );
                            }
                            if regOld != 0 as ::core::ffi::c_int {
                                let mut eAction: ::core::ffi::c_int = (*pFKey)
                                    .aAction[!aChange.is_null() as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int;
                                if (*db).flags & SQLITE_FkNoAction != 0 {
                                    eAction = OE_None;
                                }
                                fkScanChildren(
                                    pParse,
                                    pSrc,
                                    pTab,
                                    pIdx_0,
                                    pFKey,
                                    aiCol_0,
                                    regOld,
                                    1 as ::core::ffi::c_int,
                                );
                                if (*pFKey).isDeferred == 0 && eAction != OE_Cascade
                                    && eAction != OE_SetNull
                                {
                                    sqlite3MayAbort(pParse);
                                }
                            }
                            (*pItem).zName = ::core::ptr::null_mut::<
                                ::core::ffi::c_char,
                            >();
                            sqlite3SrcListDelete(db, pSrc);
                        }
                        sqlite3DbFree(db, aiCol_0 as *mut ::core::ffi::c_void);
                    }
                }
            }
            pFKey = (*pFKey).pNextTo;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FkOldmask(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
) -> u32_0 {
    unsafe {
        let mut mask: u32_0 = 0 as u32_0;
        if (*(*pParse).db).flags as ::core::ffi::c_ulonglong
            & SQLITE_ForeignKeys as ::core::ffi::c_ulonglong != 0
            && (*pTab).eTabType as ::core::ffi::c_int == TABTYP_NORM
        {
            let mut p: *mut FKey = ::core::ptr::null_mut::<FKey>();
            let mut i: ::core::ffi::c_int = 0;
            p = (*pTab).u.tab.pFKey;
            while !p.is_null() {
                i = 0 as ::core::ffi::c_int;
                while i < (*p).nCol {
                    mask
                        |= if (*(&raw mut (*p).aCol as *mut sColMap).offset(i as isize))
                            .iFrom > 31 as ::core::ffi::c_int
                        {
                            0xffffffff as ::core::ffi::c_uint
                        } else {
                            (1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                << (*(&raw mut (*p).aCol as *mut sColMap)
                                    .offset(i as isize))
                                    .iFrom
                        };
                    i += 1;
                }
                p = (*p).pNextFrom;
            }
            p = sqlite3FkReferences(pTab);
            while !p.is_null() {
                let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
                sqlite3FkLocateIndex(
                    pParse,
                    pTab,
                    p,
                    &raw mut pIdx,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_int>(),
                );
                if !pIdx.is_null() {
                    i = 0 as ::core::ffi::c_int;
                    while i < (*pIdx).nKeyCol as ::core::ffi::c_int {
                        mask
                            |= if *(*pIdx).aiColumn.offset(i as isize)
                                as ::core::ffi::c_int > 31 as ::core::ffi::c_int
                            {
                                0xffffffff as ::core::ffi::c_uint
                            } else {
                                (1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                    << *(*pIdx).aiColumn.offset(i as isize)
                                        as ::core::ffi::c_int
                            };
                        i += 1;
                    }
                }
                p = (*p).pNextTo;
            }
        }
        return mask;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FkRequired(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut aChange: *mut ::core::ffi::c_int,
    mut chngRowid: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eRet: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut bHaveFK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*(*pParse).db).flags as ::core::ffi::c_ulonglong
            & SQLITE_ForeignKeys as ::core::ffi::c_ulonglong != 0
            && (*pTab).eTabType as ::core::ffi::c_int == TABTYP_NORM
        {
            if aChange.is_null() {
                bHaveFK = (!sqlite3FkReferences(pTab).is_null()
                    || !(*pTab).u.tab.pFKey.is_null()) as ::core::ffi::c_int;
            } else {
                let mut p: *mut FKey = ::core::ptr::null_mut::<FKey>();
                p = (*pTab).u.tab.pFKey;
                while !p.is_null() {
                    if fkChildIsModified(pTab, p, aChange, chngRowid) != 0 {
                        if 0 as ::core::ffi::c_int
                            == sqlite3_stricmp((*pTab).zName, (*p).zTo)
                        {
                            eRet = 2 as ::core::ffi::c_int;
                        }
                        bHaveFK = 1 as ::core::ffi::c_int;
                    }
                    p = (*p).pNextFrom;
                }
                p = sqlite3FkReferences(pTab);
                while !p.is_null() {
                    if fkParentIsModified(pTab, p, aChange, chngRowid) != 0 {
                        if (*(*pParse).db).flags & SQLITE_FkNoAction
                            == 0 as ::core::ffi::c_ulonglong
                            && (*p).aAction[1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int != OE_None
                        {
                            return 2 as ::core::ffi::c_int;
                        }
                        bHaveFK = 1 as ::core::ffi::c_int;
                    }
                    p = (*p).pNextTo;
                }
            }
        }
        return if bHaveFK != 0 { eRet } else { 0 as ::core::ffi::c_int };
    }
}
unsafe extern "C" fn fkActionTrigger(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pFKey: *mut FKey,
    mut pChanges: *mut ExprList,
) -> *mut Trigger {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut action: ::core::ffi::c_int = 0;
        let mut pTrigger: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        let mut iAction: ::core::ffi::c_int = !pChanges.is_null() as ::core::ffi::c_int;
        action = (*pFKey).aAction[iAction as usize] as ::core::ffi::c_int;
        if (*db).flags & SQLITE_FkNoAction != 0 {
            action = OE_None;
        }
        if action == OE_Restrict
            && (*db).flags as ::core::ffi::c_ulonglong
                & SQLITE_DeferFKs as ::core::ffi::c_ulonglong != 0
        {
            return ::core::ptr::null_mut::<Trigger>();
        }
        pTrigger = (*pFKey).apTrigger[iAction as usize];
        if action != OE_None && pTrigger.is_null() {
            let mut zFrom: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut nFrom: ::core::ffi::c_int = 0;
            let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
            let mut aiCol: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
                ::core::ffi::c_int,
            >();
            let mut pStep: *mut TriggerStep = ::core::ptr::null_mut::<TriggerStep>();
            let mut pWhere: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            let mut pSelect: *mut Select = ::core::ptr::null_mut::<Select>();
            let mut i: ::core::ffi::c_int = 0;
            let mut pWhen: *mut Expr = ::core::ptr::null_mut::<Expr>();
            if sqlite3FkLocateIndex(pParse, pTab, pFKey, &raw mut pIdx, &raw mut aiCol)
                != 0
            {
                return ::core::ptr::null_mut::<Trigger>();
            }
            i = 0 as ::core::ffi::c_int;
            while i < (*pFKey).nCol {
                let mut tOld: Token = Token {
                    z: b"old\0".as_ptr() as *const ::core::ffi::c_char,
                    n: 3 as ::core::ffi::c_uint

                };
                let mut tNew: Token = Token {
                    z: b"new\0".as_ptr() as *const ::core::ffi::c_char,
                    n: 3 as ::core::ffi::c_uint

                };
                let mut tFromCol: Token = Token {
                    z: ::core::ptr::null::<::core::ffi::c_char>(),
                    n: 0

                };
                let mut tToCol: Token = Token {
                    z: ::core::ptr::null::<::core::ffi::c_char>(),
                    n: 0

                };
                let mut iFromCol: ::core::ffi::c_int = 0;
                let mut pEq: *mut Expr = ::core::ptr::null_mut::<Expr>();
                iFromCol = if !aiCol.is_null() {
                    *aiCol.offset(i as isize)
                } else {
                    (*(&raw mut (*pFKey).aCol as *mut sColMap)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .iFrom
                };
                sqlite3TokenInit(
                    &raw mut tToCol,
                    (*(*pTab)
                        .aCol
                        .offset(
                            (if !pIdx.is_null() {
                                *(*pIdx).aiColumn.offset(i as isize) as ::core::ffi::c_int
                            } else {
                                (*pTab).iPKey as ::core::ffi::c_int
                            }) as isize,
                        ))
                        .zCnName,
                );
                sqlite3TokenInit(
                    &raw mut tFromCol,
                    (*(*(*pFKey).pFrom).aCol.offset(iFromCol as isize)).zCnName,
                );
                pEq = sqlite3PExpr(
                    pParse,
                    TK_EQ,
                    sqlite3PExpr(
                        pParse,
                        TK_DOT,
                        sqlite3ExprAlloc(
                            db,
                            TK_ID,
                            &raw mut tOld,
                            0 as ::core::ffi::c_int,
                        ),
                        sqlite3ExprAlloc(
                            db,
                            TK_ID,
                            &raw mut tToCol,
                            0 as ::core::ffi::c_int,
                        ),
                    ),
                    sqlite3ExprAlloc(
                        db,
                        TK_ID,
                        &raw mut tFromCol,
                        0 as ::core::ffi::c_int,
                    ),
                );
                pWhere = sqlite3ExprAnd(pParse, pWhere, pEq);
                if !pChanges.is_null() {
                    pEq = sqlite3PExpr(
                        pParse,
                        TK_IS,
                        sqlite3PExpr(
                            pParse,
                            TK_DOT,
                            sqlite3ExprAlloc(
                                db,
                                TK_ID,
                                &raw mut tOld,
                                0 as ::core::ffi::c_int,
                            ),
                            sqlite3ExprAlloc(
                                db,
                                TK_ID,
                                &raw mut tToCol,
                                0 as ::core::ffi::c_int,
                            ),
                        ),
                        sqlite3PExpr(
                            pParse,
                            TK_DOT,
                            sqlite3ExprAlloc(
                                db,
                                TK_ID,
                                &raw mut tNew,
                                0 as ::core::ffi::c_int,
                            ),
                            sqlite3ExprAlloc(
                                db,
                                TK_ID,
                                &raw mut tToCol,
                                0 as ::core::ffi::c_int,
                            ),
                        ),
                    );
                    pWhen = sqlite3ExprAnd(pParse, pWhen, pEq);
                }
                if action != OE_Restrict && (action != OE_Cascade || !pChanges.is_null())
                {
                    let mut pNew: *mut Expr = ::core::ptr::null_mut::<Expr>();
                    if action == OE_Cascade {
                        pNew = sqlite3PExpr(
                            pParse,
                            TK_DOT,
                            sqlite3ExprAlloc(
                                db,
                                TK_ID,
                                &raw mut tNew,
                                0 as ::core::ffi::c_int,
                            ),
                            sqlite3ExprAlloc(
                                db,
                                TK_ID,
                                &raw mut tToCol,
                                0 as ::core::ffi::c_int,
                            ),
                        );
                    } else if action == OE_SetDflt {
                        let mut pCol: *mut Column = (*(*pFKey).pFrom)
                            .aCol
                            .offset(iFromCol as isize);
                        let mut pDflt: *mut Expr = ::core::ptr::null_mut::<Expr>();
                        if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_GENERATED
                            != 0
                        {
                            pDflt = ::core::ptr::null_mut::<Expr>();
                        } else {
                            pDflt = sqlite3ColumnExpr((*pFKey).pFrom, pCol);
                        }
                        if !pDflt.is_null() {
                            pNew = sqlite3ExprDup(db, pDflt, 0 as ::core::ffi::c_int);
                        } else {
                            pNew = sqlite3ExprAlloc(
                                db,
                                TK_NULL,
                                ::core::ptr::null::<Token>(),
                                0 as ::core::ffi::c_int,
                            );
                        }
                    } else {
                        pNew = sqlite3ExprAlloc(
                            db,
                            TK_NULL,
                            ::core::ptr::null::<Token>(),
                            0 as ::core::ffi::c_int,
                        );
                    }
                    pList = sqlite3ExprListAppend(pParse, pList, pNew);
                    sqlite3ExprListSetName(
                        pParse,
                        pList,
                        &raw mut tFromCol,
                        0 as ::core::ffi::c_int,
                    );
                }
                i += 1;
            }
            sqlite3DbFree(db, aiCol as *mut ::core::ffi::c_void);
            zFrom = (*(*pFKey).pFrom).zName;
            nFrom = sqlite3Strlen30(zFrom);
            if action == OE_Restrict {
                let mut iDb: ::core::ffi::c_int = sqlite3SchemaToIndex(
                    db,
                    (*pTab).pSchema,
                );
                let mut pSrc: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
                let mut pRaise: *mut Expr = ::core::ptr::null_mut::<Expr>();
                pRaise = sqlite3Expr(
                    db,
                    TK_STRING,
                    b"FOREIGN KEY constraint failed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                pRaise = sqlite3PExpr(
                    pParse,
                    TK_RAISE,
                    pRaise,
                    ::core::ptr::null_mut::<Expr>(),
                );
                if !pRaise.is_null() {
                    (*pRaise).affExpr = OE_Abort as ::core::ffi::c_char;
                }
                pSrc = sqlite3SrcListAppend(
                    pParse,
                    ::core::ptr::null_mut::<SrcList>(),
                    ::core::ptr::null_mut::<Token>(),
                    ::core::ptr::null_mut::<Token>(),
                );
                if !pSrc.is_null() {
                    let ref mut c2rust_fresh1 = (*(&raw mut (*pSrc).a as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .zName;
                    *c2rust_fresh1 = sqlite3DbStrDup(db, zFrom);
                    let ref mut c2rust_fresh2 = (*(&raw mut (*pSrc).a as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .u4
                        .zDatabase;
                    *c2rust_fresh2 = sqlite3DbStrDup(
                        db,
                        (*(*db).aDb.offset(iDb as isize)).zDbSName,
                    );
                }
                pSelect = sqlite3SelectNew(
                    pParse,
                    sqlite3ExprListAppend(
                        pParse,
                        ::core::ptr::null_mut::<ExprList>(),
                        pRaise,
                    ),
                    pSrc,
                    pWhere,
                    ::core::ptr::null_mut::<ExprList>(),
                    ::core::ptr::null_mut::<Expr>(),
                    ::core::ptr::null_mut::<ExprList>(),
                    0 as u32_0,
                    ::core::ptr::null_mut::<Expr>(),
                );
                pWhere = ::core::ptr::null_mut::<Expr>();
            }
            (*db).lookaside.bDisable = (*db).lookaside.bDisable.wrapping_add(1);
            (*db).lookaside.sz = 0 as u16_0;
            pTrigger = sqlite3DbMallocZero(
                db,
                (::core::mem::size_of::<Trigger>() as usize)
                    .wrapping_add(::core::mem::size_of::<TriggerStep>() as usize)
                    .wrapping_add(nFrom as usize)
                    .wrapping_add(1 as usize) as u64_0,
            ) as *mut Trigger;
            if !pTrigger.is_null() {
                (*pTrigger).step_list = pTrigger.offset(1 as ::core::ffi::c_int as isize)
                    as *mut Trigger as *mut TriggerStep;
                pStep = (*pTrigger).step_list;
                (*pStep).zTarget = pStep.offset(1 as ::core::ffi::c_int as isize)
                    as *mut TriggerStep as *mut ::core::ffi::c_char;
                memcpy(
                    (*pStep).zTarget as *mut ::core::ffi::c_void,
                    zFrom as *const ::core::ffi::c_void,
                    nFrom as size_t,
                );
                (*pStep).pWhere = sqlite3ExprDup(db, pWhere, EXPRDUP_REDUCE);
                (*pStep).pExprList = sqlite3ExprListDup(db, pList, EXPRDUP_REDUCE);
                (*pStep).pSelect = sqlite3SelectDup(db, pSelect, EXPRDUP_REDUCE);
                if !pWhen.is_null() {
                    pWhen = sqlite3PExpr(
                        pParse,
                        TK_NOT,
                        pWhen,
                        ::core::ptr::null_mut::<Expr>(),
                    );
                    (*pTrigger).pWhen = sqlite3ExprDup(db, pWhen, EXPRDUP_REDUCE);
                }
            }
            (*db).lookaside.bDisable = (*db).lookaside.bDisable.wrapping_sub(1);
            (*db).lookaside.sz = (if (*db).lookaside.bDisable != 0 {
                0 as ::core::ffi::c_int
            } else {
                (*db).lookaside.szTrue as ::core::ffi::c_int
            }) as u16_0;
            sqlite3ExprDelete(db, pWhere);
            sqlite3ExprDelete(db, pWhen);
            sqlite3ExprListDelete(db, pList);
            sqlite3SelectDelete(db, pSelect);
            if (*db).mallocFailed as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                fkTriggerDelete(db, pTrigger);
                return ::core::ptr::null_mut::<Trigger>();
            }
            let mut c2rust_current_block_96: u64;
            match action {
                OE_Restrict => {
                    (*pStep).op = TK_SELECT as u8_0;
                    c2rust_current_block_96 = 17711149709958600598;
                }
                OE_Cascade => {
                    if pChanges.is_null() {
                        (*pStep).op = TK_DELETE as u8_0;
                        c2rust_current_block_96 = 17711149709958600598;
                    } else {
                        c2rust_current_block_96 = 13391073458903047924;
                    }
                }
                _ => {
                    c2rust_current_block_96 = 13391073458903047924;
                }
            }
            match c2rust_current_block_96 {
                13391073458903047924 => {
                    (*pStep).op = TK_UPDATE as u8_0;
                }
                _ => {}
            }
            (*pStep).pTrig = pTrigger;
            (*pTrigger).pSchema = (*pTab).pSchema;
            (*pTrigger).pTabSchema = (*pTab).pSchema;
            (*pFKey).apTrigger[iAction as usize] = pTrigger;
            (*pTrigger).op = (if !pChanges.is_null() { TK_UPDATE } else { TK_DELETE })
                as u8_0;
        }
        return pTrigger;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FkActions(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pChanges: *mut ExprList,
    mut regOld: ::core::ffi::c_int,
    mut aChange: *mut ::core::ffi::c_int,
    mut bChngRowid: ::core::ffi::c_int,
) {
    unsafe {
        if (*(*pParse).db).flags as ::core::ffi::c_ulonglong
            & SQLITE_ForeignKeys as ::core::ffi::c_ulonglong != 0
        {
            let mut pFKey: *mut FKey = ::core::ptr::null_mut::<FKey>();
            pFKey = sqlite3FkReferences(pTab);
            while !pFKey.is_null() {
                if aChange.is_null()
                    || fkParentIsModified(pTab, pFKey, aChange, bChngRowid) != 0
                {
                    let mut pAct: *mut Trigger = fkActionTrigger(
                        pParse,
                        pTab,
                        pFKey,
                        pChanges,
                    );
                    if !pAct.is_null() {
                        sqlite3CodeRowTriggerDirect(
                            pParse,
                            pAct,
                            pTab,
                            regOld,
                            OE_Abort,
                            0 as ::core::ffi::c_int,
                        );
                    }
                }
                pFKey = (*pFKey).pNextTo;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FkDelete(mut db: *mut sqlite3, mut pTab: *mut Table) {
    unsafe {
        let mut pFKey: *mut FKey = ::core::ptr::null_mut::<FKey>();
        let mut pNext: *mut FKey = ::core::ptr::null_mut::<FKey>();
        pFKey = (*pTab).u.tab.pFKey;
        while !pFKey.is_null() {
            if (*db).pnBytesFreed.is_null() {
                if !(*pFKey).pPrevTo.is_null() {
                    (*(*pFKey).pPrevTo).pNextTo = (*pFKey).pNextTo;
                } else {
                    let mut z: *const ::core::ffi::c_char = if !(*pFKey)
                        .pNextTo
                        .is_null()
                    {
                        (*(*pFKey).pNextTo).zTo
                    } else {
                        (*pFKey).zTo
                    };
                    sqlite3HashInsert(
                        &raw mut (*(*pTab).pSchema).fkeyHash,
                        z,
                        (*pFKey).pNextTo as *mut ::core::ffi::c_void,
                    );
                }
                if !(*pFKey).pNextTo.is_null() {
                    (*(*pFKey).pNextTo).pPrevTo = (*pFKey).pPrevTo;
                }
            }
            fkTriggerDelete(db, (*pFKey).apTrigger[0 as ::core::ffi::c_int as usize]);
            fkTriggerDelete(db, (*pFKey).apTrigger[1 as ::core::ffi::c_int as usize]);
            pNext = (*pFKey).pNextFrom;
            sqlite3DbFree(db, pFKey as *mut ::core::ffi::c_void);
            pFKey = pNext;
        }
    }
}
pub const TK_NOT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const TK_IS: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const TK_NE: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const TK_EQ: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const TK_ID: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const TK_RAISE: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const TK_STRING: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_DELETE: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const TK_UPDATE: ::core::ffi::c_int = 130 as ::core::ffi::c_int;
pub const TK_SELECT: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const TK_DOT: ::core::ffi::c_int = 142 as ::core::ffi::c_int;
pub const TK_COLUMN: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
pub const TK_REGISTER: ::core::ffi::c_int = 176 as ::core::ffi::c_int;
pub const SQLITE_ForeignKeys: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_DeferFKs: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_FkNoAction: u64_0 = (0x8 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const COLFLAG_PRIMKEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const COLFLAG_GENERATED: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const SQLITE_AFF_INTEGER: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;
pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_NOTNULL: ::core::ffi::c_int = 0x90 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TABTYP_NORM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OE_None: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Restrict: ::core::ffi::c_int = 7;
pub const OE_SetNull: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const OE_SetDflt: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OE_Cascade: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_IDXTYPE_PRIMARYKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXPRDUP_REDUCE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
