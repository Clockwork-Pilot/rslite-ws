use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type RenameToken;
    pub type Vdbe;
    pub type sqlite3_mutex;
    fn sqlite3_str_appendf(
        _: *mut sqlite3_str,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3_str_append(
        _: *mut sqlite3_str,
        zIn: *const ::core::ffi::c_char,
        N: ::core::ffi::c_int,
    );
    fn sqlite3_str_appendall(_: *mut sqlite3_str, zIn: *const ::core::ffi::c_char);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3WhereGetMask(_: *mut WhereMaskSet, _: ::core::ffi::c_int) -> Bitmask;
    fn sqlite3WhereFindTerm(
        pWC: *mut WhereClause,
        iCur: ::core::ffi::c_int,
        iColumn: ::core::ffi::c_int,
        notReady: Bitmask,
        op: u32_0,
        pIdx: *mut Index,
    ) -> *mut WhereTerm;
    fn sqlite3WhereRealloc(
        pWInfo: *mut WhereInfo,
        pOld: *mut ::core::ffi::c_void,
        nByte: u64_0,
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
    fn sqlite3VdbeExplain(
        _: *mut Parse,
        _: u8_0,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeExplainPop(_: *mut Parse);
    fn sqlite3VdbeChangeP1(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
        P1: ::core::ffi::c_int,
    );
    fn sqlite3VdbeChangeP2(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
        P2: ::core::ffi::c_int,
    );
    fn sqlite3VdbeChangeP5(_: *mut Vdbe, P5: u16_0);
    fn sqlite3VdbeJumpHere(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeChangeP4(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
        zP4: *const ::core::ffi::c_char,
        N: ::core::ffi::c_int,
    );
    fn sqlite3VdbeSetP4KeyInfo(_: *mut Parse, _: *mut Index);
    fn sqlite3VdbeGetOp(_: *mut Vdbe, _: ::core::ffi::c_int) -> *mut VdbeOp;
    fn sqlite3VdbeGetLastOp(_: *mut Vdbe) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3VdbeResolveLabel(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeCurrentAddr(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbNNFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3GetTempReg(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempReg(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3GetTempRange(_: *mut Parse, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempRange(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
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
    fn sqlite3ExprListDelete(_: *mut sqlite3, _: *mut ExprList);
    fn sqlite3PrimaryKeyIndex(_: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(
        _: *mut Index,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3TableColumnToStorage(_: *mut Table, _: i16_0) -> i16_0;
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
    fn sqlite3WhereContinueLabel(_: *mut WhereInfo) -> ::core::ffi::c_int;
    fn sqlite3WhereUsesDeferredSeek(_: *mut WhereInfo) -> ::core::ffi::c_int;
    fn sqlite3ExprCodeGetColumnOfTable(
        _: *mut Vdbe,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprCode(_: *mut Parse, _: *mut Expr, _: ::core::ffi::c_int);
    fn sqlite3ExprCodeTemp(
        _: *mut Parse,
        _: *mut Expr,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCodeTarget(
        _: *mut Parse,
        _: *mut Expr,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprIfFalse(
        _: *mut Parse,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprCompare(
        _: *const Parse,
        _: *const Expr,
        _: *const Expr,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCoveredByIndex(
        _: *mut Expr,
        iCur: ::core::ffi::c_int,
        pIdx: *mut Index,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCanBeNull(_: *const Expr) -> ::core::ffi::c_int;
    fn sqlite3ExprNeedsNoAffinityChange(
        _: *const Expr,
        _: ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprDup(
        _: *mut sqlite3,
        _: *const Expr,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3IndexAffinityStr(
        _: *mut sqlite3,
        _: *mut Index,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3CompareAffinity(
        pExpr: *const Expr,
        aff2: ::core::ffi::c_char,
    ) -> ::core::ffi::c_char;
    fn sqlite3CodeRhsOfIN(_: *mut Parse, _: *mut Expr, _: ::core::ffi::c_int);
    fn sqlite3CodeSubselect(_: *mut Parse, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3StrAccumInit(
        _: *mut StrAccum,
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3StrAccumFinish(_: *mut StrAccum) -> *mut ::core::ffi::c_char;
    fn sqlite3FindInIndex(
        _: *mut Parse,
        _: *mut Expr,
        _: u32_0,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprIsVector(pExpr: *const Expr) -> ::core::ffi::c_int;
    fn sqlite3VectorFieldSubexpr(_: *mut Expr, _: ::core::ffi::c_int) -> *mut Expr;
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
pub type i8_0 = ::core::ffi::c_schar;
pub type StrAccum = sqlite3_str;
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
pub union C2Rust_Unnamed_29 {
    pub sSrc: SrcList,
    pub fromSpace: [u8_0; 80]

}
pub const SQLITE_INDEX_CONSTRAINT_OFFSET: ::core::ffi::c_int = 74 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_FULLSCAN_STEP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OP_VFilter: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const OP_Goto: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OP_Gosub: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const OP_InitCoroutine: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const OP_Yield: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const OP_MustBeInt: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const OP_If: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const OP_IfNot: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const OP_SeekLT: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const OP_SeekLE: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const OP_SeekGE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const OP_SeekGT: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const OP_NotFound: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const OP_Found: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const OP_SeekRowid: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const OP_Last: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const OP_Prev: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const OP_IdxLE: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
pub const OP_IdxGT: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const OP_IdxLT: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const OP_IdxGE: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const OP_RowSetTest: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const OP_IsNull: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const OP_Gt: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
pub const OP_Le: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const OP_Lt: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
pub const OP_Ge: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
pub const OP_VNext: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const OP_Filter: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const OP_Return: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const OP_BeginSubrtn: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const OP_Copy: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_Affinity: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
pub const OP_MakeRecord: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const OP_OpenEphemeral: ::core::ffi::c_int = 119 as ::core::ffi::c_int;
pub const OP_SeekScan: ::core::ffi::c_int = 125 as ::core::ffi::c_int;
pub const OP_SeekHit: ::core::ffi::c_int = 126 as ::core::ffi::c_int;
pub const OP_Rowid: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const OP_NullRow: ::core::ffi::c_int = 137 as ::core::ffi::c_int;
pub const OP_IdxInsert: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const OP_DeferredSeek: ::core::ffi::c_int = 142 as ::core::ffi::c_int;
pub const OP_VInitIn: ::core::ffi::c_int = 176 as ::core::ffi::c_int;
pub const OP_FilterAdd: ::core::ffi::c_int = 184 as ::core::ffi::c_int;
pub const OP_Noop: ::core::ffi::c_int = 188 as ::core::ffi::c_int;
pub const OP_Explain: ::core::ffi::c_int = 189 as ::core::ffi::c_int;
pub const TERM_VIRTUAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const TERM_CODED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const TERM_VNULL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TERM_LIKEOPT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const TERM_LIKECOND: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const TERM_LIKE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const TERM_IS: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const TERM_VARSELECT: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const TERM_SLICE: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const WO_IN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const WO_EQ: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WO_LE: ::core::ffi::c_int = WO_EQ << TK_LE - TK_EQ;
pub const WO_GE: ::core::ffi::c_int = WO_EQ << TK_GE - TK_EQ;
pub const WO_IS: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const WO_ISNULL: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const WO_AND: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const WO_EQUIV: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const WO_ROWVAL: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const WO_ALL: ::core::ffi::c_int = 0x3fff as ::core::ffi::c_int;
pub const WHERE_COLUMN_EQ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const WHERE_COLUMN_RANGE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WHERE_COLUMN_IN: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const WHERE_CONSTRAINT: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;
pub const WHERE_TOP_LIMIT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const WHERE_BTM_LIMIT: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const WHERE_BOTH_LIMIT: ::core::ffi::c_int = 0x30 as ::core::ffi::c_int;
pub const WHERE_IDX_ONLY: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const WHERE_IPK: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const WHERE_INDEXED: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const WHERE_VIRTUALTABLE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const WHERE_IN_ABLE: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const WHERE_ONEROW: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const WHERE_MULTI_OR: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const WHERE_AUTO_INDEX: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const WHERE_UNQ_WANTED: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const WHERE_PARTIALIDX: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const WHERE_IN_EARLYOUT: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const WHERE_BIGNULL_SORT: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const WHERE_IN_SEEKSCAN: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const WHERE_TRANSCONS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const WHERE_EXPRIDX: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const P4_STATIC: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const P4_DYNAMIC: ::core::ffi::c_int = -6 as ::core::ffi::c_int;
pub const P4_INTARRAY: ::core::ffi::c_int = -14 as ::core::ffi::c_int;
unsafe extern "C" fn explainIndexColumnName(
    mut pIdx: *mut Index,
    mut i: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        i = *(*pIdx).aiColumn.offset(i as isize) as ::core::ffi::c_int;
        if i == XN_EXPR {
            return b"<expr>\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if i == XN_ROWID {
            return b"rowid\0".as_ptr() as *const ::core::ffi::c_char;
        }
        return (*(*(*pIdx).pTable).aCol.offset(i as isize)).zCnName;
    }
}
unsafe extern "C" fn explainAppendTerm(
    mut pStr: *mut StrAccum,
    mut pIdx: *mut Index,
    mut nTerm: ::core::ffi::c_int,
    mut iTerm: ::core::ffi::c_int,
    mut bAnd: ::core::ffi::c_int,
    mut zOp: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if bAnd != 0 {
            sqlite3_str_append(
                pStr as *mut sqlite3_str,
                b" AND \0".as_ptr() as *const ::core::ffi::c_char,
                5 as ::core::ffi::c_int,
            );
        }
        if nTerm > 1 as ::core::ffi::c_int {
            sqlite3_str_append(
                pStr as *mut sqlite3_str,
                b"(\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
        }
        i = 0 as ::core::ffi::c_int;
        while i < nTerm {
            if i != 0 {
                sqlite3_str_append(
                    pStr as *mut sqlite3_str,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                );
            }
            sqlite3_str_appendall(
                pStr as *mut sqlite3_str,
                explainIndexColumnName(pIdx, iTerm + i),
            );
            i += 1;
        }
        if nTerm > 1 as ::core::ffi::c_int {
            sqlite3_str_append(
                pStr as *mut sqlite3_str,
                b")\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
        }
        sqlite3_str_append(pStr as *mut sqlite3_str, zOp, 1 as ::core::ffi::c_int);
        if nTerm > 1 as ::core::ffi::c_int {
            sqlite3_str_append(
                pStr as *mut sqlite3_str,
                b"(\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
        }
        i = 0 as ::core::ffi::c_int;
        while i < nTerm {
            if i != 0 {
                sqlite3_str_append(
                    pStr as *mut sqlite3_str,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                );
            }
            sqlite3_str_append(
                pStr as *mut sqlite3_str,
                b"?\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            i += 1;
        }
        if nTerm > 1 as ::core::ffi::c_int {
            sqlite3_str_append(
                pStr as *mut sqlite3_str,
                b")\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
        }
    }
}
unsafe extern "C" fn explainIndexRange(
    mut pStr: *mut StrAccum,
    mut pLoop: *mut WhereLoop,
) {
    unsafe {
        let mut pIndex: *mut Index = (*pLoop).u.btree.pIndex;
        let mut nEq: u16_0 = (*pLoop).u.btree.nEq;
        let mut nSkip: u16_0 = (*pLoop).nSkip;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        if nEq as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*pLoop).wsFlags as ::core::ffi::c_uint
                & (WHERE_BTM_LIMIT | WHERE_TOP_LIMIT) as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
        {
            return;
        }
        sqlite3_str_append(
            pStr as *mut sqlite3_str,
            b" (\0".as_ptr() as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
        );
        i = 0 as ::core::ffi::c_int;
        while i < nEq as ::core::ffi::c_int {
            let mut z: *const ::core::ffi::c_char = explainIndexColumnName(pIndex, i);
            if i != 0 {
                sqlite3_str_append(
                    pStr as *mut sqlite3_str,
                    b" AND \0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                );
            }
            sqlite3_str_appendf(
                pStr as *mut sqlite3_str,
                if i >= nSkip as ::core::ffi::c_int {
                    b"%s=?\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"ANY(%s)\0".as_ptr() as *const ::core::ffi::c_char
                },
                z,
            );
            i += 1;
        }
        j = i;
        if (*pLoop).wsFlags as ::core::ffi::c_uint
            & WHERE_BTM_LIMIT as ::core::ffi::c_uint != 0
        {
            explainAppendTerm(
                pStr,
                pIndex,
                (*pLoop).u.btree.nBtm as ::core::ffi::c_int,
                j,
                i,
                b">\0".as_ptr() as *const ::core::ffi::c_char,
            );
            i = 1 as ::core::ffi::c_int;
        }
        if (*pLoop).wsFlags as ::core::ffi::c_uint
            & WHERE_TOP_LIMIT as ::core::ffi::c_uint != 0
        {
            explainAppendTerm(
                pStr,
                pIndex,
                (*pLoop).u.btree.nTop as ::core::ffi::c_int,
                j,
                i,
                b"<\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        sqlite3_str_append(
            pStr as *mut sqlite3_str,
            b")\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereAddExplainText(
    mut pParse: *mut Parse,
    mut addr: ::core::ffi::c_int,
    mut pTabList: *mut SrcList,
    mut pLevel: *mut WhereLevel,
    mut wctrlFlags: u16_0,
) {
    unsafe {
        if (*(if !(*pParse).pToplevel.is_null() { (*pParse).pToplevel } else { pParse }))
            .explain as ::core::ffi::c_int == 2 as ::core::ffi::c_int || false
        {
            let mut pOp: *mut VdbeOp = sqlite3VdbeGetOp((*pParse).pVdbe, addr);
            let mut pItem: *mut SrcItem = (&raw mut (*pTabList).a as *mut SrcItem)
                .offset((*pLevel).iFrom as isize) as *mut SrcItem;
            let mut db: *mut sqlite3 = (*pParse).db;
            let mut isSearch: ::core::ffi::c_int = 0;
            let mut pLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
            let mut flags: u32_0 = 0;
            let mut str: StrAccum = StrAccum {
                db: ::core::ptr::null_mut::<sqlite3>(),
                zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                nAlloc: 0,
                mxAlloc: 0,
                nChar: 0,
                accError: 0,
                printfFlags: 0

            };
            let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
            if (*db).mallocFailed != 0 {
                return;
            }
            pLoop = (*pLevel).pWLoop as *mut WhereLoop;
            flags = (*pLoop).wsFlags;
            isSearch = (flags as ::core::ffi::c_uint
                & (WHERE_BTM_LIMIT | WHERE_TOP_LIMIT) as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
                || flags as ::core::ffi::c_uint
                    & WHERE_VIRTUALTABLE as ::core::ffi::c_uint
                    == 0 as ::core::ffi::c_uint
                    && (*pLoop).u.btree.nEq as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                || wctrlFlags as ::core::ffi::c_int
                    & (WHERE_ORDERBY_MIN | WHERE_ORDERBY_MAX) != 0)
                as ::core::ffi::c_int;
            sqlite3StrAccumInit(
                &raw mut str,
                db,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 100]>()
                    as ::core::ffi::c_int,
                SQLITE_MAX_LENGTH,
            );
            str.printfFlags = SQLITE_PRINTF_INTERNAL as u8_0;
            sqlite3_str_appendf(
                &raw mut str,
                b"%s %S%s\0".as_ptr() as *const ::core::ffi::c_char,
                if isSearch != 0 {
                    b"SEARCH\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"SCAN\0".as_ptr() as *const ::core::ffi::c_char
                },
                pItem,
                if (*pItem).fg.fromExists() as ::core::ffi::c_int != 0 {
                    b" EXISTS\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            if flags as ::core::ffi::c_uint
                & (WHERE_IPK | WHERE_VIRTUALTABLE) as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
            {
                let mut zFmt: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
                pIdx = (*pLoop).u.btree.pIndex;
                if !((*(*pItem).pSTab).tabFlags as ::core::ffi::c_uint
                    & TF_WithoutRowid as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint)
                    && (*pIdx).idxType() as ::core::ffi::c_int
                        == SQLITE_IDXTYPE_PRIMARYKEY
                {
                    if isSearch != 0 {
                        zFmt = b"PRIMARY KEY\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                } else if flags as ::core::ffi::c_uint
                    & WHERE_PARTIALIDX as ::core::ffi::c_uint != 0
                {
                    zFmt = b"AUTOMATIC PARTIAL COVERING INDEX\0".as_ptr()
                        as *const ::core::ffi::c_char;
                } else if flags as ::core::ffi::c_uint
                    & WHERE_AUTO_INDEX as ::core::ffi::c_uint != 0
                {
                    zFmt = b"AUTOMATIC COVERING INDEX\0".as_ptr()
                        as *const ::core::ffi::c_char;
                } else if flags as ::core::ffi::c_uint
                    & (WHERE_IDX_ONLY | WHERE_EXPRIDX) as ::core::ffi::c_uint != 0
                {
                    zFmt = b"COVERING INDEX %s\0".as_ptr() as *const ::core::ffi::c_char;
                } else {
                    zFmt = b"INDEX %s\0".as_ptr() as *const ::core::ffi::c_char;
                }
                if !zFmt.is_null() {
                    sqlite3_str_append(
                        &raw mut str,
                        b" USING \0".as_ptr() as *const ::core::ffi::c_char,
                        7 as ::core::ffi::c_int,
                    );
                    sqlite3_str_appendf(&raw mut str, zFmt, (*pIdx).zName);
                    explainIndexRange(&raw mut str, pLoop);
                }
            } else if flags as ::core::ffi::c_uint & WHERE_IPK as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
                && flags as ::core::ffi::c_uint & WHERE_CONSTRAINT as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
            {
                let mut cRangeOp: ::core::ffi::c_char = 0;
                let mut zRowid: *const ::core::ffi::c_char = b"rowid\0".as_ptr()
                    as *const ::core::ffi::c_char;
                sqlite3_str_appendf(
                    &raw mut str,
                    b" USING INTEGER PRIMARY KEY (%s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    zRowid,
                );
                if flags as ::core::ffi::c_uint
                    & (WHERE_COLUMN_EQ | WHERE_COLUMN_IN) as ::core::ffi::c_uint != 0
                {
                    cRangeOp = '=' as i32 as ::core::ffi::c_char;
                } else if flags as ::core::ffi::c_uint
                    & WHERE_BOTH_LIMIT as ::core::ffi::c_uint
                    == WHERE_BOTH_LIMIT as ::core::ffi::c_uint
                {
                    sqlite3_str_appendf(
                        &raw mut str,
                        b">? AND %s\0".as_ptr() as *const ::core::ffi::c_char,
                        zRowid,
                    );
                    cRangeOp = '<' as i32 as ::core::ffi::c_char;
                } else if flags as ::core::ffi::c_uint
                    & WHERE_BTM_LIMIT as ::core::ffi::c_uint != 0
                {
                    cRangeOp = '>' as i32 as ::core::ffi::c_char;
                } else {
                    cRangeOp = '<' as i32 as ::core::ffi::c_char;
                }
                sqlite3_str_appendf(
                    &raw mut str,
                    b"%c?)\0".as_ptr() as *const ::core::ffi::c_char,
                    cRangeOp as ::core::ffi::c_int,
                );
            } else if flags as ::core::ffi::c_uint
                & WHERE_VIRTUALTABLE as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            {
                sqlite3_str_appendall(
                    &raw mut str,
                    b" VIRTUAL TABLE INDEX \0".as_ptr() as *const ::core::ffi::c_char,
                );
                sqlite3_str_appendf(
                    &raw mut str,
                    if (*pLoop).u.vtab.bIdxNumHex() as ::core::ffi::c_int != 0 {
                        b"0x%x:%s\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"%d:%s\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    (*pLoop).u.vtab.idxNum,
                    (*pLoop).u.vtab.idxStr,
                );
            }
            if (*pItem).fg.jointype as ::core::ffi::c_int & JT_LEFT != 0 {
                sqlite3_str_appendf(
                    &raw mut str,
                    b" LEFT-JOIN\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            sqlite3DbFree(db, (*pOp).p4.z as *mut ::core::ffi::c_void);
            (*pOp).p4type = P4_DYNAMIC as ::core::ffi::c_schar;
            (*pOp).p4.z = sqlite3StrAccumFinish(&raw mut str);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereExplainOneScan(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pLevel: *mut WhereLevel,
    mut wctrlFlags: u16_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*(if !(*pParse).pToplevel.is_null() { (*pParse).pToplevel } else { pParse }))
            .explain as ::core::ffi::c_int == 2 as ::core::ffi::c_int || false
        {
            if (*(*pLevel).pWLoop).wsFlags as ::core::ffi::c_uint
                & WHERE_MULTI_OR as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                && wctrlFlags as ::core::ffi::c_int & WHERE_OR_SUBCLAUSE
                    == 0 as ::core::ffi::c_int
            {
                let mut v: *mut Vdbe = (*pParse).pVdbe;
                let mut addr: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(v);
                ret = sqlite3VdbeAddOp3(
                    v,
                    OP_Explain,
                    addr,
                    (*pParse).addrExplain,
                    (*(*pLevel).pWLoop).rRun as ::core::ffi::c_int,
                );
                sqlite3WhereAddExplainText(pParse, addr, pTabList, pLevel, wctrlFlags);
            }
        }
        return ret;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereExplainBloomFilter(
    mut pParse: *const Parse,
    mut pWInfo: *const WhereInfo,
    mut pLevel: *const WhereLevel,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pItem: *mut SrcItem = (&raw mut (*(*pWInfo).pTabList).a as *mut SrcItem)
            .offset((*pLevel).iFrom as isize) as *mut SrcItem;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut zMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut pLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut str: StrAccum = StrAccum {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        sqlite3StrAccumInit(
            &raw mut str,
            db,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            SQLITE_MAX_LENGTH,
        );
        str.printfFlags = SQLITE_PRINTF_INTERNAL as u8_0;
        sqlite3_str_appendf(
            &raw mut str,
            b"BLOOM FILTER ON %S (\0".as_ptr() as *const ::core::ffi::c_char,
            pItem,
        );
        pLoop = (*pLevel).pWLoop as *mut WhereLoop;
        if (*pLoop).wsFlags as ::core::ffi::c_uint & WHERE_IPK as ::core::ffi::c_uint
            != 0
        {
            let mut pTab: *const Table = (*pItem).pSTab;
            if (*pTab).iPKey as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                sqlite3_str_appendf(
                    &raw mut str,
                    b"%s=?\0".as_ptr() as *const ::core::ffi::c_char,
                    (*(*pTab).aCol.offset((*pTab).iPKey as isize)).zCnName,
                );
            } else {
                sqlite3_str_appendf(
                    &raw mut str,
                    b"rowid=?\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        } else {
            i = (*pLoop).nSkip as ::core::ffi::c_int;
            while i < (*pLoop).u.btree.nEq as ::core::ffi::c_int {
                let mut z: *const ::core::ffi::c_char = explainIndexColumnName(
                    (*pLoop).u.btree.pIndex,
                    i,
                );
                if i > (*pLoop).nSkip as ::core::ffi::c_int {
                    sqlite3_str_append(
                        &raw mut str,
                        b" AND \0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    );
                }
                sqlite3_str_appendf(
                    &raw mut str,
                    b"%s=?\0".as_ptr() as *const ::core::ffi::c_char,
                    z,
                );
                i += 1;
            }
        }
        sqlite3_str_append(
            &raw mut str,
            b")\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        zMsg = sqlite3StrAccumFinish(&raw mut str);
        ret = sqlite3VdbeAddOp4(
            v,
            OP_Explain,
            sqlite3VdbeCurrentAddr(v),
            (*pParse).addrExplain,
            0 as ::core::ffi::c_int,
            zMsg,
            P4_DYNAMIC,
        );
        return ret;
    }
}
unsafe extern "C" fn disableTerm(
    mut pLevel: *mut WhereLevel,
    mut pTerm: *mut WhereTerm,
) {
    unsafe {
        let mut nLoop: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while (*pTerm).wtFlags as ::core::ffi::c_int & TERM_CODED
            == 0 as ::core::ffi::c_int
            && ((*pLevel).iLeftJoin == 0 as ::core::ffi::c_int
                || (*(*pTerm).pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint)
            && (*pLevel).notReady & (*pTerm).prereqAll == 0 as ::core::ffi::c_ulonglong
        {
            if nLoop != 0
                && (*pTerm).wtFlags as ::core::ffi::c_int & TERM_LIKE
                    != 0 as ::core::ffi::c_int
            {
                (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int
                    | TERM_LIKECOND) as u16_0;
            } else {
                (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int | TERM_CODED)
                    as u16_0;
            }
            if (*pTerm).iParent < 0 as ::core::ffi::c_int {
                break;
            }
            pTerm = (*(*pTerm).pWC).a.offset((*pTerm).iParent as isize)
                as *mut WhereTerm;
            (*pTerm).nChild = (*pTerm).nChild.wrapping_sub(1);
            if (*pTerm).nChild as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                break;
            }
            nLoop += 1;
        }
    }
}
unsafe extern "C" fn codeApplyAffinity(
    mut pParse: *mut Parse,
    mut base: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut zAff: *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        if zAff.is_null() {
            return;
        }
        while n > 0 as ::core::ffi::c_int
            && *zAff.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                <= SQLITE_AFF_BLOB
        {
            n -= 1;
            base += 1;
            zAff = zAff.offset(1);
        }
        while n > 1 as ::core::ffi::c_int
            && *zAff.offset((n - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                <= SQLITE_AFF_BLOB
        {
            n -= 1;
        }
        if n > 0 as ::core::ffi::c_int {
            sqlite3VdbeAddOp4(v, OP_Affinity, base, n, 0 as ::core::ffi::c_int, zAff, n);
        }
    }
}
unsafe extern "C" fn updateRangeAffinityStr(
    mut pRight: *mut Expr,
    mut n: ::core::ffi::c_int,
    mut zAff: *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            let mut p: *mut Expr = sqlite3VectorFieldSubexpr(pRight, i);
            if sqlite3CompareAffinity(p, *zAff.offset(i as isize)) as ::core::ffi::c_int
                == SQLITE_AFF_BLOB
                || sqlite3ExprNeedsNoAffinityChange(p, *zAff.offset(i as isize)) != 0
            {
                *zAff.offset(i as isize) = SQLITE_AFF_BLOB as ::core::ffi::c_char;
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn adjustOrderByCol(
    mut pOrderBy: *mut ExprList,
    mut pEList: *mut ExprList,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        if pOrderBy.is_null() {
            return;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pOrderBy).nExpr {
            let mut t: ::core::ffi::c_int = (*(&raw mut (*pOrderBy).a
                as *mut ExprList_item)
                .offset(i as isize))
                .u
                .x
                .iOrderByCol as ::core::ffi::c_int;
            if !(t == 0 as ::core::ffi::c_int) {
                j = 0 as ::core::ffi::c_int;
                while j < (*pEList).nExpr {
                    if (*(&raw mut (*pEList).a as *mut ExprList_item).offset(j as isize))
                        .u
                        .x
                        .iOrderByCol as ::core::ffi::c_int == t
                    {
                        (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                            .offset(i as isize))
                            .u
                            .x
                            .iOrderByCol = (j + 1 as ::core::ffi::c_int) as u16_0;
                        break;
                    } else {
                        j += 1;
                    }
                }
                if j >= (*pEList).nExpr {
                    (*(&raw mut (*pOrderBy).a as *mut ExprList_item).offset(i as isize))
                        .u
                        .x
                        .iOrderByCol = 0 as u16_0;
                }
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn removeUnindexableInClauseTerms(
    mut pParse: *mut Parse,
    mut iEq: ::core::ffi::c_int,
    mut pLoop: *mut WhereLoop,
    mut pX: *mut Expr,
) -> *mut Expr {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pSelect: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pNew: *mut Expr = ::core::ptr::null_mut::<Expr>();
        pNew = sqlite3ExprDup(db, pX, 0 as ::core::ffi::c_int);
        if (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            pSelect = (*pNew).x.pSelect;
            while !pSelect.is_null() {
                let mut pOrigRhs: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                let mut pOrigLhs: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                let mut pRhs: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                let mut pLhs: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                let mut i: ::core::ffi::c_int = 0;
                pOrigRhs = (*pSelect).pEList;
                if pSelect == (*pNew).x.pSelect {
                    pOrigLhs = (*(*pNew).pLeft).x.pList;
                }
                i = iEq;
                while i < (*pLoop).nLTerm as ::core::ffi::c_int {
                    if (**(*pLoop).aLTerm.offset(i as isize)).pExpr == pX {
                        let mut iField: ::core::ffi::c_int = 0;
                        iField = (**(*pLoop).aLTerm.offset(i as isize)).u.x.iField
                            - 1 as ::core::ffi::c_int;
                        if !(*(&raw mut (*pOrigRhs).a as *mut ExprList_item)
                            .offset(iField as isize))
                            .pExpr
                            .is_null()
                        {
                            pRhs = sqlite3ExprListAppend(
                                pParse,
                                pRhs,
                                (*(&raw mut (*pOrigRhs).a as *mut ExprList_item)
                                    .offset(iField as isize))
                                    .pExpr,
                            );
                            let ref mut c2rust_fresh8 = (*(&raw mut (*pOrigRhs).a
                                as *mut ExprList_item)
                                .offset(iField as isize))
                                .pExpr;
                            *c2rust_fresh8 = ::core::ptr::null_mut::<Expr>();
                            if !pRhs.is_null() {
                                (*(&raw mut (*pRhs).a as *mut ExprList_item)
                                    .offset(((*pRhs).nExpr - 1 as ::core::ffi::c_int) as isize))
                                    .u
                                    .x
                                    .iOrderByCol = (iField + 1 as ::core::ffi::c_int) as u16_0;
                            }
                            if !pOrigLhs.is_null() {
                                pLhs = sqlite3ExprListAppend(
                                    pParse,
                                    pLhs,
                                    (*(&raw mut (*pOrigLhs).a as *mut ExprList_item)
                                        .offset(iField as isize))
                                        .pExpr,
                                );
                                let ref mut c2rust_fresh9 = (*(&raw mut (*pOrigLhs).a
                                    as *mut ExprList_item)
                                    .offset(iField as isize))
                                    .pExpr;
                                *c2rust_fresh9 = ::core::ptr::null_mut::<Expr>();
                            }
                        }
                    }
                    i += 1;
                }
                sqlite3ExprListDelete(db, pOrigRhs);
                if !pOrigLhs.is_null() {
                    sqlite3ExprListDelete(db, pOrigLhs);
                    (*(*pNew).pLeft).x.pList = pLhs;
                }
                (*pSelect).pEList = pRhs;
                (*pParse).nSelect += 1;
                (*pSelect).selId = (*pParse).nSelect as u32_0;
                if !pLhs.is_null() && (*pLhs).nExpr == 1 as ::core::ffi::c_int {
                    let mut p: *mut Expr = (*(&raw mut (*pLhs).a as *mut ExprList_item)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .pExpr;
                    let ref mut c2rust_fresh10 = (*(&raw mut (*pLhs).a
                        as *mut ExprList_item)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .pExpr;
                    *c2rust_fresh10 = ::core::ptr::null_mut::<Expr>();
                    sqlite3ExprDelete(db, (*pNew).pLeft);
                    (*pNew).pLeft = p;
                }
                if !pRhs.is_null() {
                    adjustOrderByCol((*pSelect).pOrderBy, pRhs);
                    adjustOrderByCol((*pSelect).pGroupBy, pRhs);
                    i = 0 as ::core::ffi::c_int;
                    while i < (*pRhs).nExpr {
                        (*(&raw mut (*pRhs).a as *mut ExprList_item).offset(i as isize))
                            .u
                            .x
                            .iOrderByCol = 0 as u16_0;
                        i += 1;
                    }
                }
                pSelect = (*pSelect).pPrior;
            }
        }
        return pNew;
    }
}
#[inline(never)]
unsafe extern "C" fn codeINTerm(
    mut pParse: *mut Parse,
    mut pTerm: *mut WhereTerm,
    mut pLevel: *mut WhereLevel,
    mut iEq: ::core::ffi::c_int,
    mut bRev: ::core::ffi::c_int,
    mut iTarget: ::core::ffi::c_int,
) {
    unsafe {
        let mut pX: *mut Expr = (*pTerm).pExpr;
        let mut eType: ::core::ffi::c_int = IN_INDEX_NOOP;
        let mut iTab: ::core::ffi::c_int = 0;
        let mut pIn: *mut InLoop = ::core::ptr::null_mut::<InLoop>();
        let mut pLoop: *mut WhereLoop = (*pLevel).pWLoop as *mut WhereLoop;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut i: ::core::ffi::c_int = 0;
        let mut nEq: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aiMap: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        if (*pLoop).wsFlags as ::core::ffi::c_uint
            & WHERE_VIRTUALTABLE as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            && !(*pLoop).u.btree.pIndex.is_null()
            && *(*(*pLoop).u.btree.pIndex).aSortOrder.offset(iEq as isize)
                as ::core::ffi::c_int != 0
        {
            bRev = (bRev == 0) as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < iEq {
            if !(*(*pLoop).aLTerm.offset(i as isize)).is_null()
                && (**(*pLoop).aLTerm.offset(i as isize)).pExpr == pX
            {
                disableTerm(pLevel, pTerm);
                return;
            }
            i += 1;
        }
        i = iEq;
        while i < (*pLoop).nLTerm as ::core::ffi::c_int {
            if (**(*pLoop).aLTerm.offset(i as isize)).pExpr == pX {
                nEq += 1;
            }
            i += 1;
        }
        iTab = 0 as ::core::ffi::c_int;
        if !((*pX).flags as ::core::ffi::c_uint & EP_xIsSelect as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
            || (*(*(*pX).x.pSelect).pEList).nExpr == 1 as ::core::ffi::c_int
        {
            eType = sqlite3FindInIndex(
                pParse,
                pX,
                IN_INDEX_LOOP as u32_0,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                &raw mut iTab,
            );
        } else {
            let mut db: *mut sqlite3 = (*pParse).db;
            let mut pXMod: *mut Expr = removeUnindexableInClauseTerms(
                pParse,
                iEq,
                pLoop,
                pX,
            );
            if (*db).mallocFailed == 0 {
                aiMap = sqlite3DbMallocZero(
                    db,
                    (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        .wrapping_mul(nEq as usize) as u64_0,
                ) as *mut ::core::ffi::c_int;
                eType = sqlite3FindInIndex(
                    pParse,
                    pXMod,
                    IN_INDEX_LOOP as u32_0,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    aiMap,
                    &raw mut iTab,
                );
            }
            sqlite3ExprDelete(db, pXMod);
        }
        if eType == IN_INDEX_INDEX_DESC {
            bRev = (bRev == 0) as ::core::ffi::c_int;
        }
        sqlite3VdbeAddOp2(
            v,
            if bRev != 0 { OP_Last } else { OP_Rewind },
            iTab,
            0 as ::core::ffi::c_int,
        );
        (*pLoop).wsFlags |= WHERE_IN_ABLE as ::core::ffi::c_uint;
        if (*pLevel).u.in_0.nIn == 0 as ::core::ffi::c_int {
            (*pLevel).addrNxt = sqlite3VdbeMakeLabel(pParse);
        }
        if iEq > 0 as ::core::ffi::c_int
            && (*pLoop).wsFlags as ::core::ffi::c_uint
                & WHERE_IN_SEEKSCAN as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
        {
            (*pLoop).wsFlags |= WHERE_IN_EARLYOUT as ::core::ffi::c_uint;
        }
        i = (*pLevel).u.in_0.nIn;
        (*pLevel).u.in_0.nIn += nEq;
        (*pLevel).u.in_0.aInLoop = sqlite3WhereRealloc(
            (*(*pTerm).pWC).pWInfo,
            (*pLevel).u.in_0.aInLoop as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<InLoop>() as usize)
                .wrapping_mul((*pLevel).u.in_0.nIn as usize) as u64_0,
        ) as *mut InLoop;
        pIn = (*pLevel).u.in_0.aInLoop as *mut InLoop;
        if !pIn.is_null() {
            let mut iMap: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            pIn = pIn.offset(i as isize);
            i = iEq;
            while i < (*pLoop).nLTerm as ::core::ffi::c_int {
                if (**(*pLoop).aLTerm.offset(i as isize)).pExpr == pX {
                    let mut iOut: ::core::ffi::c_int = iTarget + i - iEq;
                    if eType == IN_INDEX_ROWID {
                        (*pIn).addrInTop = sqlite3VdbeAddOp2(v, OP_Rowid, iTab, iOut);
                    } else {
                        let mut iCol: ::core::ffi::c_int = if !aiMap.is_null() {
                            let c2rust_fresh7 = iMap;
                            iMap = iMap + 1;
                            *aiMap.offset(c2rust_fresh7 as isize)
                        } else {
                            0 as ::core::ffi::c_int
                        };
                        (*pIn).addrInTop = sqlite3VdbeAddOp3(
                            v,
                            OP_Column,
                            iTab,
                            iCol,
                            iOut,
                        );
                    }
                    sqlite3VdbeAddOp1(v, OP_IsNull, iOut);
                    if i == iEq {
                        (*pIn).iCur = iTab;
                        (*pIn).eEndLoopOp = (if bRev != 0 { OP_Prev } else { OP_Next })
                            as u8_0;
                        if iEq > 0 as ::core::ffi::c_int {
                            (*pIn).iBase = iTarget - i;
                            (*pIn).nPrefix = i;
                        } else {
                            (*pIn).nPrefix = 0 as ::core::ffi::c_int;
                        }
                    } else {
                        (*pIn).eEndLoopOp = OP_Noop as u8_0;
                    }
                    pIn = pIn.offset(1);
                }
                i += 1;
            }
            if iEq > 0 as ::core::ffi::c_int
                && (*pLoop).wsFlags as ::core::ffi::c_uint
                    & (WHERE_IN_SEEKSCAN | WHERE_VIRTUALTABLE) as ::core::ffi::c_uint
                    == 0 as ::core::ffi::c_uint
            {
                sqlite3VdbeAddOp3(
                    v,
                    OP_SeekHit,
                    (*pLevel).iIdxCur,
                    0 as ::core::ffi::c_int,
                    iEq,
                );
            }
        } else {
            (*pLevel).u.in_0.nIn = 0 as ::core::ffi::c_int;
        }
        sqlite3DbFree((*pParse).db, aiMap as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn codeEqualityTerm(
    mut pParse: *mut Parse,
    mut pTerm: *mut WhereTerm,
    mut pLevel: *mut WhereLevel,
    mut iEq: ::core::ffi::c_int,
    mut bRev: ::core::ffi::c_int,
    mut iTarget: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pX: *mut Expr = (*pTerm).pExpr;
        let mut iReg: ::core::ffi::c_int = 0;
        if (*pX).op as ::core::ffi::c_int == TK_EQ
            || (*pX).op as ::core::ffi::c_int == TK_IS
        {
            iReg = sqlite3ExprCodeTarget(pParse, (*pX).pRight, iTarget);
        } else if (*pX).op as ::core::ffi::c_int == TK_ISNULL {
            iReg = iTarget;
            sqlite3VdbeAddOp2((*pParse).pVdbe, OP_Null, 0 as ::core::ffi::c_int, iReg);
        } else {
            iReg = iTarget;
            codeINTerm(pParse, pTerm, pLevel, iEq, bRev, iTarget);
        }
        if (*(*pLevel).pWLoop).wsFlags as ::core::ffi::c_uint
            & WHERE_TRANSCONS as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            || (*pTerm).eOperator as ::core::ffi::c_int & WO_EQUIV
                == 0 as ::core::ffi::c_int
        {
            disableTerm(pLevel, pTerm);
        }
        return iReg;
    }
}
unsafe extern "C" fn codeAllEqualityTerms(
    mut pParse: *mut Parse,
    mut pLevel: *mut WhereLevel,
    mut bRev: ::core::ffi::c_int,
    mut nExtraReg: ::core::ffi::c_int,
    mut pzAff: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nEq: u16_0 = 0;
        let mut nSkip: u16_0 = 0;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut pLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut j: ::core::ffi::c_int = 0;
        let mut regBase: ::core::ffi::c_int = 0;
        let mut nReg: ::core::ffi::c_int = 0;
        let mut zAff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        pLoop = (*pLevel).pWLoop as *mut WhereLoop;
        nEq = (*pLoop).u.btree.nEq;
        nSkip = (*pLoop).nSkip;
        pIdx = (*pLoop).u.btree.pIndex;
        regBase = (*pParse).nMem + 1 as ::core::ffi::c_int;
        nReg = nEq as ::core::ffi::c_int + nExtraReg;
        (*pParse).nMem += nReg;
        zAff = sqlite3DbStrDup(
            (*pParse).db,
            sqlite3IndexAffinityStr((*pParse).db, pIdx),
        );
        if nSkip != 0 {
            let mut iIdxCur: ::core::ffi::c_int = (*pLevel).iIdxCur;
            sqlite3VdbeAddOp3(
                v,
                OP_Null,
                0 as ::core::ffi::c_int,
                regBase,
                regBase + nSkip as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp1(v, if bRev != 0 { OP_Last } else { OP_Rewind }, iIdxCur);
            j = sqlite3VdbeAddOp0(v, OP_Goto);
            (*pLevel).addrSkip = sqlite3VdbeAddOp4Int(
                v,
                if bRev != 0 { OP_SeekLT } else { OP_SeekGT },
                iIdxCur,
                0 as ::core::ffi::c_int,
                regBase,
                nSkip as ::core::ffi::c_int,
            );
            sqlite3VdbeJumpHere(v, j);
            j = 0 as ::core::ffi::c_int;
            while j < nSkip as ::core::ffi::c_int {
                sqlite3VdbeAddOp3(v, OP_Column, iIdxCur, j, regBase + j);
                j += 1;
            }
        }
        j = nSkip as ::core::ffi::c_int;
        while j < nEq as ::core::ffi::c_int {
            let mut r1: ::core::ffi::c_int = 0;
            pTerm = *(*pLoop).aLTerm.offset(j as isize);
            r1 = codeEqualityTerm(pParse, pTerm, pLevel, j, bRev, regBase + j);
            if r1 != regBase + j {
                if nReg == 1 as ::core::ffi::c_int {
                    sqlite3ReleaseTempReg(pParse, regBase);
                    regBase = r1;
                } else {
                    sqlite3VdbeAddOp2(v, OP_Copy, r1, regBase + j);
                }
            }
            if (*pTerm).eOperator as ::core::ffi::c_int & WO_IN != 0 {
                if (*(*pTerm).pExpr).flags as ::core::ffi::c_uint
                    & EP_xIsSelect as ::core::ffi::c_uint != 0
                {
                    if !zAff.is_null() {
                        *zAff.offset(j as isize) = SQLITE_AFF_BLOB
                            as ::core::ffi::c_char;
                    }
                }
            } else if (*pTerm).eOperator as ::core::ffi::c_int & WO_ISNULL
                == 0 as ::core::ffi::c_int
            {
                let mut pRight: *mut Expr = (*(*pTerm).pExpr).pRight;
                if (*pTerm).wtFlags as ::core::ffi::c_int & TERM_IS
                    == 0 as ::core::ffi::c_int && sqlite3ExprCanBeNull(pRight) != 0
                {
                    sqlite3VdbeAddOp2(v, OP_IsNull, regBase + j, (*pLevel).addrBrk);
                }
                if (*pParse).nErr == 0 as ::core::ffi::c_int {
                    if sqlite3CompareAffinity(pRight, *zAff.offset(j as isize))
                        as ::core::ffi::c_int == SQLITE_AFF_BLOB
                    {
                        *zAff.offset(j as isize) = SQLITE_AFF_BLOB
                            as ::core::ffi::c_char;
                    }
                    if sqlite3ExprNeedsNoAffinityChange(pRight, *zAff.offset(j as isize))
                        != 0
                    {
                        *zAff.offset(j as isize) = SQLITE_AFF_BLOB
                            as ::core::ffi::c_char;
                    }
                }
            }
            j += 1;
        }
        *pzAff = zAff;
        return regBase;
    }
}
unsafe extern "C" fn whereLikeOptimizationStringFixup(
    mut v: *mut Vdbe,
    mut pLevel: *mut WhereLevel,
    mut pTerm: *mut WhereTerm,
) {
    unsafe {
        if (*pTerm).wtFlags as ::core::ffi::c_int & TERM_LIKEOPT != 0 {
            let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
            pOp = sqlite3VdbeGetLastOp(v);
            (*pOp).p3 = ((*pLevel).iLikeRepCntr >> 1 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            (*pOp).p5 = ((*pLevel).iLikeRepCntr as ::core::ffi::c_uint
                & 1 as ::core::ffi::c_uint) as u8_0 as u16_0;
        }
    }
}
unsafe extern "C" fn codeDeferredSeek(
    mut pWInfo: *mut WhereInfo,
    mut pIdx: *mut Index,
    mut iCur: ::core::ffi::c_int,
    mut iIdxCur: ::core::ffi::c_int,
) {
    unsafe {
        let mut pParse: *mut Parse = (*pWInfo).pParse;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        (*pWInfo).set_bDeferredSeek(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        sqlite3VdbeAddOp3(v, OP_DeferredSeek, iIdxCur, 0 as ::core::ffi::c_int, iCur);
        if (*pWInfo).wctrlFlags as ::core::ffi::c_int
            & (WHERE_OR_SUBCLAUSE | WHERE_RIGHT_JOIN) != 0
            && (*(if !(*pParse).pToplevel.is_null() {
                (*pParse).pToplevel
            } else {
                pParse
            }))
                .writeMask == 0 as ::core::ffi::c_uint
        {
            let mut i: ::core::ffi::c_int = 0;
            let mut pTab: *mut Table = (*pIdx).pTable;
            let mut ai: *mut u32_0 = sqlite3DbMallocZero(
                (*pParse).db,
                (::core::mem::size_of::<u32_0>() as usize)
                    .wrapping_mul(
                        ((*pTab).nCol as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as usize,
                    ) as u64_0,
            ) as *mut u32_0;
            if !ai.is_null() {
                *ai.offset(0 as ::core::ffi::c_int as isize) = (*pTab).nCol as u32_0;
                i = 0 as ::core::ffi::c_int;
                while i < (*pIdx).nColumn as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    let mut x1: ::core::ffi::c_int = 0;
                    let mut x2: ::core::ffi::c_int = 0;
                    x1 = *(*pIdx).aiColumn.offset(i as isize) as ::core::ffi::c_int;
                    x2 = sqlite3TableColumnToStorage(pTab, x1 as i16_0)
                        as ::core::ffi::c_int;
                    if x1 >= 0 as ::core::ffi::c_int {
                        *ai.offset((x2 + 1 as ::core::ffi::c_int) as isize) = (i
                            + 1 as ::core::ffi::c_int) as u32_0;
                    }
                    i += 1;
                }
                sqlite3VdbeChangeP4(
                    v,
                    -1 as ::core::ffi::c_int,
                    ai as *mut ::core::ffi::c_char,
                    P4_INTARRAY,
                );
            }
        }
    }
}
unsafe extern "C" fn codeExprOrVector(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
    mut iReg: ::core::ffi::c_int,
    mut nReg: ::core::ffi::c_int,
) {
    unsafe {
        if !p.is_null() && sqlite3ExprIsVector(p) != 0 {
            if (*p).flags as ::core::ffi::c_uint & EP_xIsSelect as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                let mut v: *mut Vdbe = (*pParse).pVdbe;
                let mut iSelect: ::core::ffi::c_int = 0;
                iSelect = sqlite3CodeSubselect(pParse, p);
                sqlite3VdbeAddOp3(
                    v,
                    OP_Copy,
                    iSelect,
                    iReg,
                    nReg - 1 as ::core::ffi::c_int,
                );
            } else {
                let mut i: ::core::ffi::c_int = 0;
                let mut pList: *const ExprList = ::core::ptr::null::<ExprList>();
                pList = (*p).x.pList;
                i = 0 as ::core::ffi::c_int;
                while i < nReg {
                    sqlite3ExprCode(
                        pParse,
                        (*(&raw const (*pList).a as *const ExprList_item)
                            .offset(i as isize))
                            .pExpr,
                        iReg + i,
                    );
                    i += 1;
                }
            }
        } else {
            sqlite3ExprCode(pParse, p, iReg);
        };
    }
}
unsafe extern "C" fn whereApplyPartialIndexConstraints(
    mut pTruth: *mut Expr,
    mut iTabCur: ::core::ffi::c_int,
    mut pWC: *mut WhereClause,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        while (*pTruth).op as ::core::ffi::c_int == TK_AND {
            whereApplyPartialIndexConstraints((*pTruth).pLeft, iTabCur, pWC);
            pTruth = (*pTruth).pRight;
        }
        i = 0 as ::core::ffi::c_int;
        pTerm = (*pWC).a;
        while i < (*pWC).nTerm {
            let mut pExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
            if !((*pTerm).wtFlags as ::core::ffi::c_int & TERM_CODED != 0) {
                pExpr = (*pTerm).pExpr;
                if sqlite3ExprCompare(
                    ::core::ptr::null::<Parse>(),
                    pExpr,
                    pTruth,
                    iTabCur,
                ) == 0 as ::core::ffi::c_int
                {
                    (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int
                        | TERM_CODED) as u16_0;
                }
            }
            i += 1;
            pTerm = pTerm.offset(1);
        }
    }
}
#[inline(never)]
unsafe extern "C" fn filterPullDown(
    mut pParse: *mut Parse,
    mut pWInfo: *mut WhereInfo,
    mut iLevel: ::core::ffi::c_int,
    mut addrNxt: ::core::ffi::c_int,
    mut notReady: Bitmask,
) {
    unsafe {
        let mut saved_addrBrk: ::core::ffi::c_int = 0;
        loop {
            iLevel += 1;
            if !(iLevel < (*pWInfo).nLevel as ::core::ffi::c_int) {
                break;
            }
            let mut pLevel: *mut WhereLevel = (&raw mut (*pWInfo).a as *mut WhereLevel)
                .offset(iLevel as isize) as *mut WhereLevel;
            let mut pLoop: *mut WhereLoop = (*pLevel).pWLoop as *mut WhereLoop;
            if (*pLevel).regFilter == 0 as ::core::ffi::c_int {
                continue;
            }
            if (*(*pLevel).pWLoop).nSkip != 0 {
                continue;
            }
            if (*pLoop).prereq & notReady != 0 {
                continue;
            }
            saved_addrBrk = (*pLevel).addrBrk;
            (*pLevel).addrBrk = addrNxt;
            if (*pLoop).wsFlags as ::core::ffi::c_uint & WHERE_IPK as ::core::ffi::c_uint
                != 0
            {
                let mut pTerm: *mut WhereTerm = *(*pLoop)
                    .aLTerm
                    .offset(0 as ::core::ffi::c_int as isize);
                let mut regRowid: ::core::ffi::c_int = 0;
                regRowid = sqlite3GetTempReg(pParse);
                regRowid = codeEqualityTerm(
                    pParse,
                    pTerm,
                    pLevel,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    regRowid,
                );
                sqlite3VdbeAddOp2((*pParse).pVdbe, OP_MustBeInt, regRowid, addrNxt);
                sqlite3VdbeAddOp4Int(
                    (*pParse).pVdbe,
                    OP_Filter,
                    (*pLevel).regFilter,
                    addrNxt,
                    regRowid,
                    1 as ::core::ffi::c_int,
                );
            } else {
                let mut nEq: u16_0 = (*pLoop).u.btree.nEq;
                let mut r1: ::core::ffi::c_int = 0;
                let mut zStartAff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                r1 = codeAllEqualityTerms(
                    pParse,
                    pLevel,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    &raw mut zStartAff,
                );
                codeApplyAffinity(pParse, r1, nEq as ::core::ffi::c_int, zStartAff);
                sqlite3DbFree((*pParse).db, zStartAff as *mut ::core::ffi::c_void);
                sqlite3VdbeAddOp4Int(
                    (*pParse).pVdbe,
                    OP_Filter,
                    (*pLevel).regFilter,
                    addrNxt,
                    r1,
                    nEq as ::core::ffi::c_int,
                );
            }
            (*pLevel).regFilter = 0 as ::core::ffi::c_int;
            (*pLevel).addrBrk = saved_addrBrk;
        };
    }
}
unsafe extern "C" fn whereLoopIsOneRow(mut pLoop: *mut WhereLoop) -> ::core::ffi::c_int {
    unsafe {
        if (*(*pLoop).u.btree.pIndex).onError as ::core::ffi::c_int != 0
            && (*pLoop).nSkip as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*pLoop).u.btree.nEq as ::core::ffi::c_int
                == (*(*pLoop).u.btree.pIndex).nKeyCol as ::core::ffi::c_int
        {
            let mut ii: ::core::ffi::c_int = 0;
            ii = 0 as ::core::ffi::c_int;
            while ii < (*pLoop).u.btree.nEq as ::core::ffi::c_int {
                if (**(*pLoop).aLTerm.offset(ii as isize)).eOperator
                    as ::core::ffi::c_int & (WO_IS | WO_ISNULL) != 0
                {
                    return 0 as ::core::ffi::c_int;
                }
                ii += 1;
            }
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereCodeOneLoopStart(
    mut pParse: *mut Parse,
    mut v: *mut Vdbe,
    mut pWInfo: *mut WhereInfo,
    mut iLevel: ::core::ffi::c_int,
    mut pLevel: *mut WhereLevel,
    mut notReady: Bitmask,
) -> Bitmask {
    unsafe {
        let mut pRJ_0: *mut WhereRightJoin = ::core::ptr::null_mut::<WhereRightJoin>();
        let mut c2rust_current_block: u64;
        let mut j: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut iCur: ::core::ffi::c_int = 0;
        let mut addrNxt: ::core::ffi::c_int = 0;
        let mut bRev: ::core::ffi::c_int = 0;
        let mut pLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut pWC: *mut WhereClause = ::core::ptr::null_mut::<WhereClause>();
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pTabItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut addrBrk: ::core::ffi::c_int = 0;
        let mut addrCont: ::core::ffi::c_int = 0;
        let mut iRowidReg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iReleaseReg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut iLoop: ::core::ffi::c_int = 0;
        pWC = &raw mut (*pWInfo).sWC;
        db = (*pParse).db;
        pLoop = (*pLevel).pWLoop as *mut WhereLoop;
        pTabItem = (&raw mut (*(*pWInfo).pTabList).a as *mut SrcItem)
            .offset((*pLevel).iFrom as isize) as *mut SrcItem;
        iCur = (*pTabItem).iCursor;
        (*pLevel).notReady = notReady
            & !sqlite3WhereGetMask(&raw mut (*pWInfo).sMaskSet, iCur);
        bRev = ((*pWInfo).revMask as ::core::ffi::c_ulonglong >> iLevel
            & 1 as ::core::ffi::c_ulonglong) as ::core::ffi::c_int;
        (*pLevel).addrNxt = (*pLevel).addrBrk;
        addrBrk = (*pLevel).addrNxt;
        (*pLevel).addrCont = sqlite3VdbeMakeLabel(pParse);
        addrCont = (*pLevel).addrCont;
        if (*pLevel).iFrom as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && (*pTabItem.offset(0 as ::core::ffi::c_int as isize)).fg.jointype
                as ::core::ffi::c_int & JT_LEFT != 0 as ::core::ffi::c_int
        {
            (*pParse).nMem += 1;
            (*pLevel).iLeftJoin = (*pParse).nMem;
            sqlite3VdbeAddOp2(
                v,
                OP_Integer,
                0 as ::core::ffi::c_int,
                (*pLevel).iLeftJoin,
            );
        }
        if (*pTabItem).fg.viaCoroutine() != 0 {
            let mut regYield: ::core::ffi::c_int = 0;
            let mut pSubq: *mut Subquery = ::core::ptr::null_mut::<Subquery>();
            pSubq = (*pTabItem).u4.pSubq;
            regYield = (*pSubq).regReturn;
            sqlite3VdbeAddOp3(
                v,
                OP_InitCoroutine,
                regYield,
                0 as ::core::ffi::c_int,
                (*pSubq).addrFillSub,
            );
            (*pLevel).p2 = sqlite3VdbeAddOp2(v, OP_Yield, regYield, addrBrk);
            (*pLevel).op = OP_Goto as u8_0;
        } else if (*pLoop).wsFlags as ::core::ffi::c_uint
            & WHERE_VIRTUALTABLE as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
        {
            let mut iReg: ::core::ffi::c_int = 0;
            let mut addrNotFound: ::core::ffi::c_int = 0;
            let mut nConstraint: ::core::ffi::c_int = (*pLoop).nLTerm
                as ::core::ffi::c_int;
            iReg = sqlite3GetTempRange(pParse, nConstraint + 2 as ::core::ffi::c_int);
            addrNotFound = (*pLevel).addrBrk;
            j = 0 as ::core::ffi::c_int;
            while j < nConstraint {
                let mut iTarget: ::core::ffi::c_int = iReg + j + 2 as ::core::ffi::c_int;
                pTerm = *(*pLoop).aLTerm.offset(j as isize);
                if !pTerm.is_null() {
                    if (*pTerm).eOperator as ::core::ffi::c_int & WO_IN != 0 {
                        if (if j <= 31 as ::core::ffi::c_int {
                            (1 as ::core::ffi::c_int as u32_0) << j
                        } else {
                            0 as u32_0
                        }) & (*pLoop).u.vtab.mHandleIn != 0
                        {
                            let c2rust_fresh0 = (*pParse).nTab;
                            (*pParse).nTab = (*pParse).nTab + 1;
                            let mut iTab: ::core::ffi::c_int = c2rust_fresh0;
                            (*pParse).nMem += 1;
                            let mut iCache: ::core::ffi::c_int = (*pParse).nMem;
                            sqlite3CodeRhsOfIN(pParse, (*pTerm).pExpr, iTab);
                            sqlite3VdbeAddOp3(v, OP_VInitIn, iTab, iTarget, iCache);
                        } else {
                            codeEqualityTerm(pParse, pTerm, pLevel, j, bRev, iTarget);
                            addrNotFound = (*pLevel).addrNxt;
                        }
                    } else {
                        let mut pRight: *mut Expr = (*(*pTerm).pExpr).pRight;
                        codeExprOrVector(
                            pParse,
                            pRight,
                            iTarget,
                            1 as ::core::ffi::c_int,
                        );
                        if (*pTerm).eMatchOp as ::core::ffi::c_int
                            == SQLITE_INDEX_CONSTRAINT_OFFSET
                            && (*pLoop).u.vtab.bOmitOffset() as ::core::ffi::c_int != 0
                        {
                            sqlite3VdbeAddOp2(
                                v,
                                OP_Integer,
                                0 as ::core::ffi::c_int,
                                (*(*pWInfo).pSelect).iOffset,
                            );
                        }
                    }
                }
                j += 1;
            }
            sqlite3VdbeAddOp2(v, OP_Integer, (*pLoop).u.vtab.idxNum, iReg);
            sqlite3VdbeAddOp2(
                v,
                OP_Integer,
                nConstraint,
                iReg + 1 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp4(
                v,
                OP_VFilter,
                iCur,
                addrNotFound,
                iReg,
                (*pLoop).u.vtab.idxStr,
                if (*pLoop).u.vtab.needFree() as ::core::ffi::c_int != 0 {
                    P4_DYNAMIC
                } else {
                    P4_STATIC
                },
            );
            (*pLoop).u.vtab.set_needFree(0 as u32_0 as u32_0);
            if (*db).mallocFailed != 0 {
                (*pLoop).u.vtab.idxStr = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            (*pLevel).p1 = iCur;
            (*pLevel).op = (if (*pWInfo).eOnePass as ::core::ffi::c_int != 0 {
                OP_Noop
            } else {
                OP_VNext
            }) as u8_0;
            (*pLevel).p2 = sqlite3VdbeCurrentAddr(v);
            j = 0 as ::core::ffi::c_int;
            while j < nConstraint {
                pTerm = *(*pLoop).aLTerm.offset(j as isize);
                if j < 16 as ::core::ffi::c_int
                    && (*pLoop).u.vtab.omitMask as ::core::ffi::c_int >> j
                        & 1 as ::core::ffi::c_int != 0
                {
                    disableTerm(pLevel, pTerm);
                } else if (*pTerm).eOperator as ::core::ffi::c_int & WO_IN
                    != 0 as ::core::ffi::c_int
                    && (if j <= 31 as ::core::ffi::c_int {
                        (1 as ::core::ffi::c_int as u32_0) << j
                    } else {
                        0 as u32_0
                    }) & (*pLoop).u.vtab.mHandleIn == 0 as ::core::ffi::c_uint
                    && (*db).mallocFailed == 0
                {
                    let mut pCompare: *mut Expr = ::core::ptr::null_mut::<Expr>();
                    let mut pRight_0: *mut Expr = ::core::ptr::null_mut::<Expr>();
                    let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
                    let mut iIn: ::core::ffi::c_int = 0;
                    iIn = 0 as ::core::ffi::c_int;
                    while iIn < (*pLevel).u.in_0.nIn {
                        pOp = sqlite3VdbeGetOp(
                            v,
                            (*(*pLevel).u.in_0.aInLoop.offset(iIn as isize)).addrInTop,
                        );
                        if (*pOp).opcode as ::core::ffi::c_int == OP_Column
                            && (*pOp).p3 == iReg + j + 2 as ::core::ffi::c_int
                            || (*pOp).opcode as ::core::ffi::c_int == OP_Rowid
                                && (*pOp).p2 == iReg + j + 2 as ::core::ffi::c_int
                        {
                            sqlite3VdbeAddOp3(
                                v,
                                (*pOp).opcode as ::core::ffi::c_int,
                                (*pOp).p1,
                                (*pOp).p2,
                                (*pOp).p3,
                            );
                            break;
                        } else {
                            iIn += 1;
                        }
                    }
                    pCompare = sqlite3PExpr(
                        pParse,
                        TK_EQ,
                        ::core::ptr::null_mut::<Expr>(),
                        ::core::ptr::null_mut::<Expr>(),
                    );
                    if (*db).mallocFailed == 0 {
                        let mut iFld: ::core::ffi::c_int = (*pTerm).u.x.iField;
                        let mut pLeft: *mut Expr = (*(*pTerm).pExpr).pLeft;
                        if iFld > 0 as ::core::ffi::c_int {
                            (*pCompare).pLeft = (*(&raw mut (*(*pLeft).x.pList).a
                                as *mut ExprList_item)
                                .offset((iFld - 1 as ::core::ffi::c_int) as isize))
                                .pExpr;
                        } else {
                            (*pCompare).pLeft = pLeft;
                        }
                        pRight_0 = sqlite3Expr(
                            db,
                            TK_REGISTER,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                        (*pCompare).pRight = pRight_0;
                        if !pRight_0.is_null() {
                            (*pRight_0).iTable = iReg + j + 2 as ::core::ffi::c_int;
                            sqlite3ExprIfFalse(
                                pParse,
                                pCompare,
                                (*pLevel).addrCont,
                                SQLITE_JUMPIFNULL,
                            );
                        }
                        (*pCompare).pLeft = ::core::ptr::null_mut::<Expr>();
                    }
                    sqlite3ExprDelete(db, pCompare);
                }
                j += 1;
            }
        } else if (*pLoop).wsFlags as ::core::ffi::c_uint
            & WHERE_IPK as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            && (*pLoop).wsFlags as ::core::ffi::c_uint
                & (WHERE_COLUMN_IN | WHERE_COLUMN_EQ) as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
        {
            pTerm = *(*pLoop).aLTerm.offset(0 as ::core::ffi::c_int as isize);
            (*pParse).nMem += 1;
            iReleaseReg = (*pParse).nMem;
            iRowidReg = codeEqualityTerm(
                pParse,
                pTerm,
                pLevel,
                0 as ::core::ffi::c_int,
                bRev,
                iReleaseReg,
            );
            if iRowidReg != iReleaseReg {
                sqlite3ReleaseTempReg(pParse, iReleaseReg);
            }
            addrNxt = (*pLevel).addrNxt;
            if (*pLevel).regFilter != 0 {
                sqlite3VdbeAddOp2(v, OP_MustBeInt, iRowidReg, addrNxt);
                sqlite3VdbeAddOp4Int(
                    v,
                    OP_Filter,
                    (*pLevel).regFilter,
                    addrNxt,
                    iRowidReg,
                    1 as ::core::ffi::c_int,
                );
                filterPullDown(pParse, pWInfo, iLevel, addrNxt, notReady);
            }
            sqlite3VdbeAddOp3(v, OP_SeekRowid, iCur, addrNxt, iRowidReg);
            (*pLevel).op = OP_Noop as u8_0;
        } else if (*pLoop).wsFlags as ::core::ffi::c_uint
            & WHERE_IPK as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            && (*pLoop).wsFlags as ::core::ffi::c_uint
                & WHERE_COLUMN_RANGE as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
        {
            let mut testOp: ::core::ffi::c_int = OP_Noop;
            let mut start: ::core::ffi::c_int = 0;
            let mut memEndValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pStart: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
            let mut pEnd: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
            j = 0 as ::core::ffi::c_int;
            pEnd = ::core::ptr::null_mut::<WhereTerm>();
            pStart = pEnd;
            if (*pLoop).wsFlags as ::core::ffi::c_uint
                & WHERE_BTM_LIMIT as ::core::ffi::c_uint != 0
            {
                let c2rust_fresh1 = j;
                j = j + 1;
                pStart = *(*pLoop).aLTerm.offset(c2rust_fresh1 as isize);
            }
            if (*pLoop).wsFlags as ::core::ffi::c_uint
                & WHERE_TOP_LIMIT as ::core::ffi::c_uint != 0
            {
                let c2rust_fresh2 = j;
                j = j + 1;
                pEnd = *(*pLoop).aLTerm.offset(c2rust_fresh2 as isize);
            }
            if bRev != 0 {
                pTerm = pStart;
                pStart = pEnd;
                pEnd = pTerm;
            }
            if !pStart.is_null() {
                let mut pX: *mut Expr = ::core::ptr::null_mut::<Expr>();
                let mut r1: ::core::ffi::c_int = 0;
                let mut rTemp: ::core::ffi::c_int = 0;
                let mut op: ::core::ffi::c_int = 0;
                let aMoveOp: [u8_0; 4] = [
                    OP_SeekGT as u8_0,
                    OP_SeekLE as u8_0,
                    OP_SeekLT as u8_0,
                    OP_SeekGE as u8_0,
                ];
                pX = (*pStart).pExpr;
                if sqlite3ExprIsVector((*pX).pRight) != 0 {
                    rTemp = sqlite3GetTempReg(pParse);
                    r1 = rTemp;
                    codeExprOrVector(pParse, (*pX).pRight, r1, 1 as ::core::ffi::c_int);
                    op = aMoveOp[((*pX).op as ::core::ffi::c_int - TK_GT
                        - 1 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int
                        | 0x1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
                } else {
                    r1 = sqlite3ExprCodeTemp(pParse, (*pX).pRight, &raw mut rTemp);
                    disableTerm(pLevel, pStart);
                    op = aMoveOp[((*pX).op as ::core::ffi::c_int - TK_GT) as usize]
                        as ::core::ffi::c_int;
                }
                sqlite3VdbeAddOp3(v, op, iCur, addrBrk, r1);
                sqlite3ReleaseTempReg(pParse, rTemp);
            } else {
                sqlite3VdbeAddOp2(
                    v,
                    if bRev != 0 { OP_Last } else { OP_Rewind },
                    iCur,
                    (*pLevel).addrHalt,
                );
            }
            if !pEnd.is_null() {
                let mut pX_0: *mut Expr = ::core::ptr::null_mut::<Expr>();
                pX_0 = (*pEnd).pExpr;
                (*pParse).nMem += 1;
                memEndValue = (*pParse).nMem;
                codeExprOrVector(
                    pParse,
                    (*pX_0).pRight,
                    memEndValue,
                    1 as ::core::ffi::c_int,
                );
                if 0 as ::core::ffi::c_int == sqlite3ExprIsVector((*pX_0).pRight)
                    && ((*pX_0).op as ::core::ffi::c_int == TK_LT
                        || (*pX_0).op as ::core::ffi::c_int == TK_GT)
                {
                    testOp = if bRev != 0 { OP_Le } else { OP_Ge };
                } else {
                    testOp = if bRev != 0 { OP_Lt } else { OP_Gt };
                }
                if 0 as ::core::ffi::c_int == sqlite3ExprIsVector((*pX_0).pRight) {
                    disableTerm(pLevel, pEnd);
                }
            }
            start = sqlite3VdbeCurrentAddr(v);
            (*pLevel).op = (if bRev != 0 { OP_Prev } else { OP_Next }) as u8_0;
            (*pLevel).p1 = iCur;
            (*pLevel).p2 = start;
            if testOp != OP_Noop {
                (*pParse).nMem += 1;
                iRowidReg = (*pParse).nMem;
                sqlite3VdbeAddOp2(v, OP_Rowid, iCur, iRowidReg);
                sqlite3VdbeAddOp3(v, testOp, memEndValue, addrBrk, iRowidReg);
                sqlite3VdbeChangeP5(
                    v,
                    (SQLITE_AFF_NUMERIC | SQLITE_JUMPIFNULL) as u16_0,
                );
            }
        } else if (*pLoop).wsFlags as ::core::ffi::c_uint
            & WHERE_INDEXED as ::core::ffi::c_uint != 0
        {
            static mut aStartOp: [u8_0; 8] = [
                0 as ::core::ffi::c_int as u8_0,
                0 as ::core::ffi::c_int as u8_0,
                OP_Rewind as u8_0,
                OP_Last as u8_0,
                OP_SeekGT as u8_0,
                OP_SeekLT as u8_0,
                OP_SeekGE as u8_0,
                OP_SeekLE as u8_0,
            ];
            static mut aEndOp: [u8_0; 4] = [
                OP_IdxGE as u8_0,
                OP_IdxGT as u8_0,
                OP_IdxLE as u8_0,
                OP_IdxLT as u8_0,
            ];
            let mut nEq: u16_0 = (*pLoop).u.btree.nEq;
            let mut nBtm: u16_0 = (*pLoop).u.btree.nBtm;
            let mut nTop: u16_0 = (*pLoop).u.btree.nTop;
            let mut regBase: ::core::ffi::c_int = 0;
            let mut pRangeStart: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
            let mut pRangeEnd: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
            let mut startEq: ::core::ffi::c_int = 0;
            let mut endEq: ::core::ffi::c_int = 0;
            let mut start_constraints: ::core::ffi::c_int = 0;
            let mut nConstraint_0: ::core::ffi::c_int = 0;
            let mut iIdxCur: ::core::ffi::c_int = 0;
            let mut nExtraReg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut op_0: ::core::ffi::c_int = 0;
            let mut zStartAff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut zEndAff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut bSeekPastNull: u8_0 = 0 as u8_0;
            let mut bStopAtNull: u8_0 = 0 as u8_0;
            let mut omitTable: ::core::ffi::c_int = 0;
            let mut regBignull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut addrSeekScan: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            pIdx = (*pLoop).u.btree.pIndex;
            iIdxCur = (*pLevel).iIdxCur;
            j = nEq as ::core::ffi::c_int;
            if (*pLoop).wsFlags as ::core::ffi::c_uint
                & WHERE_BTM_LIMIT as ::core::ffi::c_uint != 0
            {
                let c2rust_fresh3 = j;
                j = j + 1;
                pRangeStart = *(*pLoop).aLTerm.offset(c2rust_fresh3 as isize);
                nExtraReg = if nExtraReg > (*pLoop).u.btree.nBtm as ::core::ffi::c_int {
                    nExtraReg
                } else {
                    (*pLoop).u.btree.nBtm as ::core::ffi::c_int
                };
            }
            if (*pLoop).wsFlags as ::core::ffi::c_uint
                & WHERE_TOP_LIMIT as ::core::ffi::c_uint != 0
            {
                let c2rust_fresh4 = j;
                j = j + 1;
                pRangeEnd = *(*pLoop).aLTerm.offset(c2rust_fresh4 as isize);
                nExtraReg = if nExtraReg > (*pLoop).u.btree.nTop as ::core::ffi::c_int {
                    nExtraReg
                } else {
                    (*pLoop).u.btree.nTop as ::core::ffi::c_int
                };
                if (*pRangeEnd).wtFlags as ::core::ffi::c_int & TERM_LIKEOPT
                    != 0 as ::core::ffi::c_int
                {
                    (*pParse).nMem += 1;
                    (*pLevel).iLikeRepCntr = (*pParse).nMem as u32_0;
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Integer,
                        1 as ::core::ffi::c_int,
                        (*pLevel).iLikeRepCntr as ::core::ffi::c_int,
                    );
                    (*pLevel).addrLikeRep = sqlite3VdbeCurrentAddr(v);
                    (*pLevel).iLikeRepCntr <<= 1 as ::core::ffi::c_int;
                    (*pLevel).iLikeRepCntr
                        |= (bRev
                            ^ (*(*pIdx).aSortOrder.offset(nEq as isize)
                                as ::core::ffi::c_int == SQLITE_SO_DESC)
                                as ::core::ffi::c_int) as ::core::ffi::c_uint;
                }
                if pRangeStart.is_null() {
                    j = *(*pIdx).aiColumn.offset(nEq as isize) as ::core::ffi::c_int;
                    if j >= 0 as ::core::ffi::c_int
                        && (*(*(*pIdx).pTable).aCol.offset(j as isize)).notNull()
                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        || j == XN_EXPR
                    {
                        bSeekPastNull = 1 as u8_0;
                    }
                }
            }
            if (*pLoop).wsFlags as ::core::ffi::c_uint
                & (WHERE_TOP_LIMIT | WHERE_BTM_LIMIT) as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
                && (*pLoop).wsFlags as ::core::ffi::c_uint
                    & WHERE_BIGNULL_SORT as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
            {
                nExtraReg = 1 as ::core::ffi::c_int;
                bSeekPastNull = 1 as u8_0;
                (*pParse).nMem += 1;
                regBignull = (*pParse).nMem;
                (*pLevel).regBignull = regBignull;
                if (*pLevel).iLeftJoin != 0 {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Integer,
                        0 as ::core::ffi::c_int,
                        regBignull,
                    );
                }
                (*pLevel).addrBignull = sqlite3VdbeMakeLabel(pParse);
            }
            if (nEq as ::core::ffi::c_int) < (*pIdx).nColumn as ::core::ffi::c_int
                && bRev
                    == (*(*pIdx).aSortOrder.offset(nEq as isize) as ::core::ffi::c_int
                        == SQLITE_SO_ASC) as ::core::ffi::c_int
            {
                let mut t: *mut WhereTerm = pRangeEnd;
                pRangeEnd = pRangeStart;
                pRangeStart = t;
                let mut t_0: u8_0 = bSeekPastNull;
                bSeekPastNull = bStopAtNull;
                bStopAtNull = t_0;
                let mut t_1: u8_0 = nBtm as u8_0;
                nBtm = nTop;
                nTop = t_1 as u16_0;
            }
            if iLevel > 0 as ::core::ffi::c_int
                && (*pLoop).wsFlags as ::core::ffi::c_uint
                    & WHERE_IN_SEEKSCAN as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
            {
                sqlite3VdbeAddOp1(v, OP_NullRow, iIdxCur);
            }
            regBase = codeAllEqualityTerms(
                pParse,
                pLevel,
                bRev,
                nExtraReg,
                &raw mut zStartAff,
            );
            if !zStartAff.is_null() && nTop as ::core::ffi::c_int != 0 {
                zEndAff = sqlite3DbStrDup(
                    db,
                    zStartAff.offset(nEq as isize) as *mut ::core::ffi::c_char,
                );
            }
            addrNxt = if regBignull != 0 {
                (*pLevel).addrBignull
            } else {
                (*pLevel).addrNxt
            };
            startEq = (pRangeStart.is_null()
                || (*pRangeStart).eOperator as ::core::ffi::c_int & (WO_LE | WO_GE) != 0)
                as ::core::ffi::c_int;
            endEq = (pRangeEnd.is_null()
                || (*pRangeEnd).eOperator as ::core::ffi::c_int & (WO_LE | WO_GE) != 0)
                as ::core::ffi::c_int;
            start_constraints = (!pRangeStart.is_null()
                || nEq as ::core::ffi::c_int > 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            nConstraint_0 = nEq as ::core::ffi::c_int;
            if !pRangeStart.is_null() {
                let mut pRight_1: *mut Expr = (*(*pRangeStart).pExpr).pRight;
                codeExprOrVector(
                    pParse,
                    pRight_1,
                    regBase + nEq as ::core::ffi::c_int,
                    nBtm as ::core::ffi::c_int,
                );
                whereLikeOptimizationStringFixup(v, pLevel, pRangeStart);
                if (*pRangeStart).wtFlags as ::core::ffi::c_int & TERM_VNULL
                    == 0 as ::core::ffi::c_int && sqlite3ExprCanBeNull(pRight_1) != 0
                {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_IsNull,
                        regBase + nEq as ::core::ffi::c_int,
                        addrNxt,
                    );
                }
                if !zStartAff.is_null() {
                    updateRangeAffinityStr(
                        pRight_1,
                        nBtm as ::core::ffi::c_int,
                        zStartAff.offset(nEq as isize) as *mut ::core::ffi::c_char,
                    );
                }
                nConstraint_0 += nBtm as ::core::ffi::c_int;
                if sqlite3ExprIsVector(pRight_1) == 0 as ::core::ffi::c_int {
                    disableTerm(pLevel, pRangeStart);
                } else {
                    startEq = 1 as ::core::ffi::c_int;
                }
                bSeekPastNull = 0 as u8_0;
            } else if bSeekPastNull != 0 {
                startEq = 0 as ::core::ffi::c_int;
                sqlite3VdbeAddOp2(
                    v,
                    OP_Null,
                    0 as ::core::ffi::c_int,
                    regBase + nEq as ::core::ffi::c_int,
                );
                start_constraints = 1 as ::core::ffi::c_int;
                nConstraint_0 += 1;
            } else if regBignull != 0 {
                sqlite3VdbeAddOp2(
                    v,
                    OP_Null,
                    0 as ::core::ffi::c_int,
                    regBase + nEq as ::core::ffi::c_int,
                );
                start_constraints = 1 as ::core::ffi::c_int;
                nConstraint_0 += 1;
            }
            codeApplyAffinity(
                pParse,
                regBase,
                nConstraint_0 - bSeekPastNull as ::core::ffi::c_int,
                zStartAff,
            );
            if !((*pLoop).nSkip as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                && nConstraint_0 == (*pLoop).nSkip as ::core::ffi::c_int)
            {
                if regBignull != 0 {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Integer,
                        1 as ::core::ffi::c_int,
                        regBignull,
                    );
                }
                if (*pLevel).regFilter != 0 {
                    sqlite3VdbeAddOp4Int(
                        v,
                        OP_Filter,
                        (*pLevel).regFilter,
                        addrNxt,
                        regBase,
                        nEq as ::core::ffi::c_int,
                    );
                    filterPullDown(pParse, pWInfo, iLevel, addrNxt, notReady);
                }
                op_0 = aStartOp[((start_constraints << 2 as ::core::ffi::c_int)
                    + (startEq << 1 as ::core::ffi::c_int) + bRev) as usize]
                    as ::core::ffi::c_int;
                if (*pLoop).wsFlags as ::core::ffi::c_uint
                    & WHERE_IN_SEEKSCAN as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint && op_0 == OP_SeekGE
                {
                    addrSeekScan = sqlite3VdbeAddOp1(
                        v,
                        OP_SeekScan,
                        (*(*pIdx).aiRowLogEst.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int + 9 as ::core::ffi::c_int)
                            / 10 as ::core::ffi::c_int,
                    );
                    if !pRangeStart.is_null() || !pRangeEnd.is_null() {
                        sqlite3VdbeChangeP5(v, 1 as u16_0);
                        sqlite3VdbeChangeP2(
                            v,
                            addrSeekScan,
                            sqlite3VdbeCurrentAddr(v) + 1 as ::core::ffi::c_int,
                        );
                        addrSeekScan = 0 as ::core::ffi::c_int;
                    }
                }
                sqlite3VdbeAddOp4Int(v, op_0, iIdxCur, addrNxt, regBase, nConstraint_0);
                if regBignull != 0 {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Goto,
                        0 as ::core::ffi::c_int,
                        sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
                    );
                    op_0 = aStartOp[((nConstraint_0 > 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_int * 4 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int + bRev) as usize]
                        as ::core::ffi::c_int;
                    sqlite3VdbeAddOp4Int(
                        v,
                        op_0,
                        iIdxCur,
                        addrNxt,
                        regBase,
                        nConstraint_0 - startEq,
                    );
                }
            }
            nConstraint_0 = nEq as ::core::ffi::c_int;
            if !pRangeEnd.is_null() {
                let mut pRight_2: *mut Expr = (*(*pRangeEnd).pExpr).pRight;
                codeExprOrVector(
                    pParse,
                    pRight_2,
                    regBase + nEq as ::core::ffi::c_int,
                    nTop as ::core::ffi::c_int,
                );
                whereLikeOptimizationStringFixup(v, pLevel, pRangeEnd);
                if (*pRangeEnd).wtFlags as ::core::ffi::c_int & TERM_VNULL
                    == 0 as ::core::ffi::c_int && sqlite3ExprCanBeNull(pRight_2) != 0
                {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_IsNull,
                        regBase + nEq as ::core::ffi::c_int,
                        addrNxt,
                    );
                }
                if !zEndAff.is_null() {
                    updateRangeAffinityStr(
                        pRight_2,
                        nTop as ::core::ffi::c_int,
                        zEndAff,
                    );
                    codeApplyAffinity(
                        pParse,
                        regBase + nEq as ::core::ffi::c_int,
                        nTop as ::core::ffi::c_int,
                        zEndAff,
                    );
                }
                nConstraint_0 += nTop as ::core::ffi::c_int;
                if sqlite3ExprIsVector(pRight_2) == 0 as ::core::ffi::c_int {
                    disableTerm(pLevel, pRangeEnd);
                } else {
                    endEq = 1 as ::core::ffi::c_int;
                }
            } else if bStopAtNull != 0 {
                if regBignull == 0 as ::core::ffi::c_int {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Null,
                        0 as ::core::ffi::c_int,
                        regBase + nEq as ::core::ffi::c_int,
                    );
                    endEq = 0 as ::core::ffi::c_int;
                }
                nConstraint_0 += 1;
            }
            if !zStartAff.is_null() {
                sqlite3DbNNFreeNN(db, zStartAff as *mut ::core::ffi::c_void);
            }
            if !zEndAff.is_null() {
                sqlite3DbNNFreeNN(db, zEndAff as *mut ::core::ffi::c_void);
            }
            (*pLevel).p2 = sqlite3VdbeCurrentAddr(v);
            if nConstraint_0 != 0 {
                if regBignull != 0 {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_IfNot,
                        regBignull,
                        sqlite3VdbeCurrentAddr(v) + 3 as ::core::ffi::c_int,
                    );
                }
                op_0 = aEndOp[(bRev * 2 as ::core::ffi::c_int + endEq) as usize]
                    as ::core::ffi::c_int;
                sqlite3VdbeAddOp4Int(v, op_0, iIdxCur, addrNxt, regBase, nConstraint_0);
                if addrSeekScan != 0 {
                    sqlite3VdbeJumpHere(v, addrSeekScan);
                }
            }
            if regBignull != 0 {
                sqlite3VdbeAddOp2(
                    v,
                    OP_If,
                    regBignull,
                    sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
                );
                op_0 = aEndOp[(bRev * 2 as ::core::ffi::c_int
                    + bSeekPastNull as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                sqlite3VdbeAddOp4Int(
                    v,
                    op_0,
                    iIdxCur,
                    addrNxt,
                    regBase,
                    nConstraint_0 + bSeekPastNull as ::core::ffi::c_int,
                );
            }
            if (*pLoop).wsFlags as ::core::ffi::c_uint
                & WHERE_IN_EARLYOUT as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            {
                sqlite3VdbeAddOp3(
                    v,
                    OP_SeekHit,
                    iIdxCur,
                    nEq as ::core::ffi::c_int,
                    nEq as ::core::ffi::c_int,
                );
            }
            omitTable = ((*pLoop).wsFlags as ::core::ffi::c_uint
                & WHERE_IDX_ONLY as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                && (*pWInfo).wctrlFlags as ::core::ffi::c_int
                    & (WHERE_OR_SUBCLAUSE | WHERE_RIGHT_JOIN) == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if !(omitTable != 0) {
                if (*(*pIdx).pTable).tabFlags as ::core::ffi::c_uint
                    & TF_WithoutRowid as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                {
                    codeDeferredSeek(pWInfo, pIdx, iCur, iIdxCur);
                } else if iCur != iIdxCur {
                    let mut pPk: *mut Index = sqlite3PrimaryKeyIndex((*pIdx).pTable);
                    iRowidReg = sqlite3GetTempRange(
                        pParse,
                        (*pPk).nKeyCol as ::core::ffi::c_int,
                    );
                    j = 0 as ::core::ffi::c_int;
                    while j < (*pPk).nKeyCol as ::core::ffi::c_int {
                        k = sqlite3TableColumnToIndex(
                            pIdx,
                            *(*pPk).aiColumn.offset(j as isize) as ::core::ffi::c_int,
                        );
                        sqlite3VdbeAddOp3(v, OP_Column, iIdxCur, k, iRowidReg + j);
                        j += 1;
                    }
                    sqlite3VdbeAddOp4Int(
                        v,
                        OP_NotFound,
                        iCur,
                        addrCont,
                        iRowidReg,
                        (*pPk).nKeyCol as ::core::ffi::c_int,
                    );
                }
            }
            if (*pLevel).iLeftJoin == 0 as ::core::ffi::c_int {
                if !(*pIdx).pPartIdxWhere.is_null() && (*pLevel).pRJ.is_null() {
                    whereApplyPartialIndexConstraints((*pIdx).pPartIdxWhere, iCur, pWC);
                }
            }
            if (*pLoop).wsFlags as ::core::ffi::c_uint
                & WHERE_ONEROW as ::core::ffi::c_uint != 0
                || (*pLevel).u.in_0.nIn != 0 && regBignull == 0 as ::core::ffi::c_int
                    && whereLoopIsOneRow(pLoop) != 0
            {
                (*pLevel).op = OP_Noop as u8_0;
            } else if bRev != 0 {
                (*pLevel).op = OP_Prev as u8_0;
            } else {
                (*pLevel).op = OP_Next as u8_0;
            }
            (*pLevel).p1 = iIdxCur;
            (*pLevel).p3 = (if (*pLoop).wsFlags as ::core::ffi::c_uint
                & WHERE_UNQ_WANTED as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as u8_0;
            if (*pLoop).wsFlags as ::core::ffi::c_uint
                & WHERE_CONSTRAINT as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            {
                (*pLevel).p5 = SQLITE_STMTSTATUS_FULLSCAN_STEP as u8_0;
            }
            if omitTable != 0 {
                pIdx = ::core::ptr::null_mut::<Index>();
            }
        } else if (*pLoop).wsFlags as ::core::ffi::c_uint
            & WHERE_MULTI_OR as ::core::ffi::c_uint != 0
        {
            let mut pOrWc: *mut WhereClause = ::core::ptr::null_mut::<WhereClause>();
            let mut pOrTab: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
            let mut pCov: *mut Index = ::core::ptr::null_mut::<Index>();
            let c2rust_fresh5 = (*pParse).nTab;
            (*pParse).nTab = (*pParse).nTab + 1;
            let mut iCovCur: ::core::ffi::c_int = c2rust_fresh5;
            (*pParse).nMem += 1;
            let mut regReturn: ::core::ffi::c_int = (*pParse).nMem;
            let mut regRowset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut regRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iLoopBody: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
            let mut iRetInit: ::core::ffi::c_int = 0;
            let mut untestedTerms: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut ii: ::core::ffi::c_int = 0;
            let mut pAndExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pTab: *mut Table = (*pTabItem).pSTab;
            pTerm = *(*pLoop).aLTerm.offset(0 as ::core::ffi::c_int as isize);
            pOrWc = &raw mut (*(*pTerm).u.pOrInfo).wc;
            (*pLevel).op = OP_Return as u8_0;
            (*pLevel).p1 = regReturn;
            if (*pWInfo).nLevel as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                let mut nNotReady: ::core::ffi::c_int = 0;
                let mut origSrc: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
                nNotReady = (*pWInfo).nLevel as ::core::ffi::c_int - iLevel
                    - 1 as ::core::ffi::c_int;
                pOrTab = sqlite3DbMallocRawNN(
                    db,
                    (8 as usize)
                        .wrapping_add(
                            ((nNotReady + 1 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<SrcItem>() as usize),
                        ) as u64_0,
                ) as *mut SrcList;
                if pOrTab.is_null() {
                    return notReady;
                }
                (*pOrTab).nAlloc = (nNotReady + 1 as ::core::ffi::c_int) as u8_0
                    as u32_0;
                (*pOrTab).nSrc = (*pOrTab).nAlloc as ::core::ffi::c_int;
                memcpy(
                    &raw mut (*pOrTab).a as *mut SrcItem as *mut ::core::ffi::c_void,
                    pTabItem as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<SrcItem>() as size_t,
                );
                origSrc = &raw mut (*(*pWInfo).pTabList).a as *mut SrcItem;
                k = 1 as ::core::ffi::c_int;
                while k <= nNotReady {
                    memcpy(
                        (&raw mut (*pOrTab).a as *mut SrcItem).offset(k as isize)
                            as *mut SrcItem as *mut ::core::ffi::c_void,
                        origSrc.offset((*pLevel.offset(k as isize)).iFrom as isize)
                            as *mut SrcItem as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<SrcItem>() as size_t,
                    );
                    k += 1;
                }
            } else {
                pOrTab = (*pWInfo).pTabList;
            }
            if (*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_DUPLICATES_OK
                == 0 as ::core::ffi::c_int
            {
                if (*pTab).tabFlags as ::core::ffi::c_uint
                    & TF_WithoutRowid as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                {
                    (*pParse).nMem += 1;
                    regRowset = (*pParse).nMem;
                    sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, regRowset);
                } else {
                    let mut pPk_0: *mut Index = sqlite3PrimaryKeyIndex(pTab);
                    let c2rust_fresh6 = (*pParse).nTab;
                    (*pParse).nTab = (*pParse).nTab + 1;
                    regRowset = c2rust_fresh6;
                    sqlite3VdbeAddOp2(
                        v,
                        OP_OpenEphemeral,
                        regRowset,
                        (*pPk_0).nKeyCol as ::core::ffi::c_int,
                    );
                    sqlite3VdbeSetP4KeyInfo(pParse, pPk_0);
                }
                (*pParse).nMem += 1;
                regRowid = (*pParse).nMem;
            }
            iRetInit = sqlite3VdbeAddOp2(
                v,
                OP_Integer,
                0 as ::core::ffi::c_int,
                regReturn,
            );
            if (*pWC).nTerm > 1 as ::core::ffi::c_int {
                let mut iTerm: ::core::ffi::c_int = 0;
                iTerm = 0 as ::core::ffi::c_int;
                while iTerm < (*pWC).nTerm {
                    let mut pExpr: *mut Expr = (*(*pWC).a.offset(iTerm as isize)).pExpr;
                    if !((*pWC).a.offset(iTerm as isize) as *mut WhereTerm == pTerm) {
                        if !((*(*pWC).a.offset(iTerm as isize)).wtFlags
                            as ::core::ffi::c_int
                            & (TERM_VIRTUAL | TERM_CODED | TERM_SLICE)
                            != 0 as ::core::ffi::c_int)
                        {
                            if !((*(*pWC).a.offset(iTerm as isize)).eOperator
                                as ::core::ffi::c_int & WO_ALL == 0 as ::core::ffi::c_int)
                            {
                                if !((*pExpr).flags
                                    & 0x400000 as ::core::ffi::c_int as u32_0
                                    != 0 as ::core::ffi::c_uint)
                                {
                                    pExpr = sqlite3ExprDup(db, pExpr, 0 as ::core::ffi::c_int);
                                    pAndExpr = sqlite3ExprAnd(pParse, pAndExpr, pExpr);
                                }
                            }
                        }
                    }
                    iTerm += 1;
                }
                if !pAndExpr.is_null() {
                    pAndExpr = sqlite3PExpr(
                        pParse,
                        TK_AND | 0x10000 as ::core::ffi::c_int,
                        ::core::ptr::null_mut::<Expr>(),
                        pAndExpr,
                    );
                }
            }
            sqlite3VdbeExplain(
                pParse,
                1 as u8_0,
                b"MULTI-INDEX OR\0".as_ptr() as *const ::core::ffi::c_char,
            );
            ii = 0 as ::core::ffi::c_int;
            while ii < (*pOrWc).nTerm {
                let mut pOrTerm: *mut WhereTerm = (*pOrWc).a.offset(ii as isize)
                    as *mut WhereTerm;
                if (*pOrTerm).leftCursor == iCur
                    || (*pOrTerm).eOperator as ::core::ffi::c_int & WO_AND
                        != 0 as ::core::ffi::c_int
                {
                    let mut pSubWInfo: *mut WhereInfo = ::core::ptr::null_mut::<
                        WhereInfo,
                    >();
                    let mut pOrExpr: *mut Expr = (*pOrTerm).pExpr;
                    let mut pDelete: *mut Expr = ::core::ptr::null_mut::<Expr>();
                    let mut jmp1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    pOrExpr = sqlite3ExprDup(db, pOrExpr, 0 as ::core::ffi::c_int);
                    pDelete = pOrExpr;
                    if (*db).mallocFailed != 0 {
                        sqlite3ExprDelete(db, pDelete);
                    } else {
                        if !pAndExpr.is_null() {
                            (*pAndExpr).pLeft = pOrExpr;
                            pOrExpr = pAndExpr;
                        }
                        sqlite3VdbeExplain(
                            pParse,
                            1 as u8_0,
                            b"INDEX %d\0".as_ptr() as *const ::core::ffi::c_char,
                            ii + 1 as ::core::ffi::c_int,
                        );
                        pSubWInfo = sqlite3WhereBegin(
                            pParse,
                            pOrTab,
                            pOrExpr,
                            ::core::ptr::null_mut::<ExprList>(),
                            ::core::ptr::null_mut::<ExprList>(),
                            ::core::ptr::null_mut::<Select>(),
                            WHERE_OR_SUBCLAUSE as u16_0,
                            iCovCur,
                        );
                        if !pSubWInfo.is_null() {
                            let mut pSubLoop: *mut WhereLoop = ::core::ptr::null_mut::<
                                WhereLoop,
                            >();
                            let mut addrExplain: ::core::ffi::c_int = sqlite3WhereExplainOneScan(
                                pParse,
                                pOrTab,
                                (&raw mut (*pSubWInfo).a as *mut WhereLevel)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut WhereLevel,
                                0 as u16_0,
                            );
                            if (*pWInfo).wctrlFlags as ::core::ffi::c_int
                                & WHERE_DUPLICATES_OK == 0 as ::core::ffi::c_int
                            {
                                let mut iSet: ::core::ffi::c_int = if ii
                                    == (*pOrWc).nTerm - 1 as ::core::ffi::c_int
                                {
                                    -1 as ::core::ffi::c_int
                                } else {
                                    ii
                                };
                                if (*pTab).tabFlags as ::core::ffi::c_uint
                                    & TF_WithoutRowid as ::core::ffi::c_uint
                                    == 0 as ::core::ffi::c_uint
                                {
                                    sqlite3ExprCodeGetColumnOfTable(
                                        v,
                                        pTab,
                                        iCur,
                                        -1 as ::core::ffi::c_int,
                                        regRowid,
                                    );
                                    jmp1 = sqlite3VdbeAddOp4Int(
                                        v,
                                        OP_RowSetTest,
                                        regRowset,
                                        0 as ::core::ffi::c_int,
                                        regRowid,
                                        iSet,
                                    );
                                } else {
                                    let mut pPk_1: *mut Index = sqlite3PrimaryKeyIndex(pTab);
                                    let mut nPk: ::core::ffi::c_int = (*pPk_1).nKeyCol
                                        as ::core::ffi::c_int;
                                    let mut iPk: ::core::ffi::c_int = 0;
                                    let mut r: ::core::ffi::c_int = 0;
                                    r = sqlite3GetTempRange(pParse, nPk);
                                    iPk = 0 as ::core::ffi::c_int;
                                    while iPk < nPk {
                                        let mut iCol: ::core::ffi::c_int = *(*pPk_1)
                                            .aiColumn
                                            .offset(iPk as isize) as ::core::ffi::c_int;
                                        sqlite3ExprCodeGetColumnOfTable(
                                            v,
                                            pTab,
                                            iCur,
                                            iCol,
                                            r + iPk,
                                        );
                                        iPk += 1;
                                    }
                                    if iSet != 0 {
                                        jmp1 = sqlite3VdbeAddOp4Int(
                                            v,
                                            OP_Found,
                                            regRowset,
                                            0 as ::core::ffi::c_int,
                                            r,
                                            nPk,
                                        );
                                    }
                                    if iSet >= 0 as ::core::ffi::c_int {
                                        sqlite3VdbeAddOp3(v, OP_MakeRecord, r, nPk, regRowid);
                                        sqlite3VdbeAddOp4Int(
                                            v,
                                            OP_IdxInsert,
                                            regRowset,
                                            regRowid,
                                            r,
                                            nPk,
                                        );
                                        if iSet != 0 {
                                            sqlite3VdbeChangeP5(v, OPFLAG_USESEEKRESULT as u16_0);
                                        }
                                    }
                                    sqlite3ReleaseTempRange(pParse, r, nPk);
                                }
                            }
                            sqlite3VdbeAddOp2(v, OP_Gosub, regReturn, iLoopBody);
                            if jmp1 != 0 {
                                sqlite3VdbeJumpHere(v, jmp1);
                            }
                            if (*pSubWInfo).untestedTerms() != 0 {
                                untestedTerms = 1 as ::core::ffi::c_int;
                            }
                            pSubLoop = (*(&raw mut (*pSubWInfo).a as *mut WhereLevel)
                                .offset(0 as ::core::ffi::c_int as isize))
                                .pWLoop as *mut WhereLoop;
                            if (*pSubLoop).wsFlags as ::core::ffi::c_uint
                                & WHERE_INDEXED as ::core::ffi::c_uint
                                != 0 as ::core::ffi::c_uint
                                && (ii == 0 as ::core::ffi::c_int
                                    || (*pSubLoop).u.btree.pIndex == pCov)
                                && ((*pTab).tabFlags as ::core::ffi::c_uint
                                    & TF_WithoutRowid as ::core::ffi::c_uint
                                    == 0 as ::core::ffi::c_uint
                                    || !((*(*pSubLoop).u.btree.pIndex).idxType()
                                        as ::core::ffi::c_int == SQLITE_IDXTYPE_PRIMARYKEY))
                            {
                                pCov = (*pSubLoop).u.btree.pIndex;
                            } else {
                                pCov = ::core::ptr::null_mut::<Index>();
                            }
                            if sqlite3WhereUsesDeferredSeek(pSubWInfo) != 0 {
                                (*pWInfo)
                                    .set_bDeferredSeek(
                                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                    );
                            }
                            sqlite3WhereEnd(pSubWInfo);
                            sqlite3VdbeExplainPop(pParse);
                        }
                        sqlite3ExprDelete(db, pDelete);
                    }
                }
                ii += 1;
            }
            sqlite3VdbeExplainPop(pParse);
            (*pLevel).u.pCoveringIdx = pCov;
            if !pCov.is_null() {
                (*pLevel).iIdxCur = iCovCur;
            }
            if !pAndExpr.is_null() {
                (*pAndExpr).pLeft = ::core::ptr::null_mut::<Expr>();
                sqlite3ExprDelete(db, pAndExpr);
            }
            sqlite3VdbeChangeP1(v, iRetInit, sqlite3VdbeCurrentAddr(v));
            sqlite3VdbeGoto(v, (*pLevel).addrBrk);
            sqlite3VdbeResolveLabel(v, iLoopBody);
            (*pLevel).p2 = sqlite3VdbeCurrentAddr(v);
            if (*pWInfo).nLevel as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                sqlite3DbFreeNN(db, pOrTab as *mut ::core::ffi::c_void);
            }
            if untestedTerms == 0 {
                disableTerm(pLevel, pTerm);
            }
        } else {
            static mut aStep: [u8_0; 2] = [OP_Next as u8_0, OP_Prev as u8_0];
            static mut aStart: [u8_0; 2] = [OP_Rewind as u8_0, OP_Last as u8_0];
            if (*pTabItem).fg.isRecursive() != 0 {
                (*pLevel).op = OP_Noop as u8_0;
            } else {
                (*pLevel).op = aStep[bRev as usize];
                (*pLevel).p1 = iCur;
                (*pLevel).p2 = 1 as ::core::ffi::c_int
                    + sqlite3VdbeAddOp2(
                        v,
                        aStart[bRev as usize] as ::core::ffi::c_int,
                        iCur,
                        (*pLevel).addrHalt,
                    );
                (*pLevel).p5 = SQLITE_STMTSTATUS_FULLSCAN_STEP as u8_0;
            }
        }
        iLoop = if !pIdx.is_null() {
            1 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        };
        loop {
            let mut iNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut c2rust_current_block_647: u64;
            pTerm = (*pWC).a;
            j = (*pWC).nTerm;
            while j > 0 as ::core::ffi::c_int {
                let mut pE: *mut Expr = ::core::ptr::null_mut::<Expr>();
                let mut skipLikeAddr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if !((*pTerm).wtFlags as ::core::ffi::c_int & (TERM_VIRTUAL | TERM_CODED)
                    != 0)
                {
                    if (*pTerm).prereqAll & (*pLevel).notReady
                        != 0 as ::core::ffi::c_ulonglong
                    {
                        (*pWInfo)
                            .set_untestedTerms(
                                1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                    } else {
                        pE = (*pTerm).pExpr;
                        if (*pTabItem).fg.jointype as ::core::ffi::c_int
                            & (JT_LEFT | JT_LTORJ | JT_RIGHT) != 0
                        {
                            if !((*pE).flags
                                & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)
                                    as u32_0 != 0 as ::core::ffi::c_uint)
                            {
                                c2rust_current_block_647 = 16642413284942005565;
                            } else if (*pTabItem).fg.jointype as ::core::ffi::c_int
                                & JT_LEFT == JT_LEFT
                                && !((*pE).flags & 0x1 as ::core::ffi::c_int as u32_0
                                    != 0 as ::core::ffi::c_uint)
                            {
                                c2rust_current_block_647 = 16642413284942005565;
                            } else {
                                let mut m: Bitmask = sqlite3WhereGetMask(
                                    &raw mut (*pWInfo).sMaskSet,
                                    (*pE).w.iJoin,
                                );
                                if m & (*pLevel).notReady != 0 {
                                    c2rust_current_block_647 = 16642413284942005565;
                                } else {
                                    c2rust_current_block_647 = 14526570417385538002;
                                }
                            }
                        } else {
                            c2rust_current_block_647 = 14526570417385538002;
                        }
                        match c2rust_current_block_647 {
                            16642413284942005565 => {}
                            _ => {
                                if iLoop == 1 as ::core::ffi::c_int
                                    && sqlite3ExprCoveredByIndex(pE, (*pLevel).iTabCur, pIdx)
                                        == 0
                                {
                                    iNext = 2 as ::core::ffi::c_int;
                                } else if iLoop < 3 as ::core::ffi::c_int
                                    && (*pTerm).wtFlags as ::core::ffi::c_int & TERM_VARSELECT
                                        != 0
                                {
                                    if iNext == 0 as ::core::ffi::c_int {
                                        iNext = 3 as ::core::ffi::c_int;
                                    }
                                } else {
                                    if (*pTerm).wtFlags as ::core::ffi::c_int & TERM_LIKECOND
                                        != 0 as ::core::ffi::c_int
                                    {
                                        let mut x: u32_0 = (*pLevel).iLikeRepCntr;
                                        if x > 0 as ::core::ffi::c_uint {
                                            skipLikeAddr = sqlite3VdbeAddOp1(
                                                v,
                                                if x as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint != 0
                                                {
                                                    OP_IfNot
                                                } else {
                                                    OP_If
                                                },
                                                (x >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                            );
                                        }
                                    }
                                    sqlite3ExprIfFalse(pParse, pE, addrCont, SQLITE_JUMPIFNULL);
                                    if skipLikeAddr != 0 {
                                        sqlite3VdbeJumpHere(v, skipLikeAddr);
                                    }
                                    (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int
                                        | TERM_CODED) as u16_0;
                                }
                            }
                        }
                    }
                }
                j -= 1;
                pTerm = pTerm.offset(1);
            }
            iLoop = iNext;
            if !(iLoop > 0 as ::core::ffi::c_int) {
                break;
            }
        }
        pTerm = (*pWC).a;
        j = (*pWC).nBase;
        while j > 0 as ::core::ffi::c_int {
            let mut pE_0: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut sEAlt: Expr = Expr {
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
            let mut pAlt: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
            if !((*pTerm).wtFlags as ::core::ffi::c_int & (TERM_VIRTUAL | TERM_CODED)
                != 0)
            {
                if !((*pTerm).eOperator as ::core::ffi::c_int & (WO_EQ | WO_IS)
                    == 0 as ::core::ffi::c_int)
                {
                    if !((*pTerm).eOperator as ::core::ffi::c_int & WO_EQUIV
                        == 0 as ::core::ffi::c_int)
                    {
                        if !((*pTerm).leftCursor != iCur) {
                            if !((*pTabItem).fg.jointype as ::core::ffi::c_int
                                & (JT_LEFT | JT_LTORJ | JT_RIGHT) != 0)
                            {
                                pE_0 = (*pTerm).pExpr;
                                pAlt = sqlite3WhereFindTerm(
                                    pWC,
                                    iCur,
                                    (*pTerm).u.x.leftColumn,
                                    notReady,
                                    (WO_EQ | WO_IN | WO_IS) as u32_0,
                                    ::core::ptr::null_mut::<Index>(),
                                );
                                if !pAlt.is_null() {
                                    if !((*pAlt).wtFlags as ::core::ffi::c_int
                                        & 0x4 as ::core::ffi::c_int != 0)
                                    {
                                        if !((*pAlt).eOperator as ::core::ffi::c_int & WO_IN != 0
                                            && (*(*pAlt).pExpr).flags as ::core::ffi::c_uint
                                                & EP_xIsSelect as ::core::ffi::c_uint
                                                != 0 as ::core::ffi::c_uint
                                            && (*(*(*(*pAlt).pExpr).x.pSelect).pEList).nExpr
                                                > 1 as ::core::ffi::c_int)
                                        {
                                            sEAlt = *(*pAlt).pExpr;
                                            sEAlt.pLeft = (*pE_0).pLeft;
                                            sqlite3ExprIfFalse(
                                                pParse,
                                                &raw mut sEAlt,
                                                addrCont,
                                                SQLITE_JUMPIFNULL,
                                            );
                                            (*pAlt).wtFlags = ((*pAlt).wtFlags as ::core::ffi::c_int
                                                | TERM_CODED) as u16_0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            j -= 1;
            pTerm = pTerm.offset(1);
        }
        if !(*pLevel).pRJ.is_null() {
            let mut pTab_0: *mut Table = ::core::ptr::null_mut::<Table>();
            let mut nPk_0: ::core::ffi::c_int = 0;
            let mut r_0: ::core::ffi::c_int = 0;
            let mut jmp1_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pRJ: *mut WhereRightJoin = (*pLevel).pRJ;
            pTab_0 = (*(&raw mut (*(*pWInfo).pTabList).a as *mut SrcItem)
                .offset((*pLevel).iFrom as isize))
                .pSTab;
            if (*pTab_0).tabFlags as ::core::ffi::c_uint
                & TF_WithoutRowid as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            {
                r_0 = sqlite3GetTempRange(pParse, 2 as ::core::ffi::c_int);
                sqlite3ExprCodeGetColumnOfTable(
                    v,
                    pTab_0,
                    (*pLevel).iTabCur,
                    -1 as ::core::ffi::c_int,
                    r_0 + 1 as ::core::ffi::c_int,
                );
                nPk_0 = 1 as ::core::ffi::c_int;
            } else {
                let mut iPk_0: ::core::ffi::c_int = 0;
                let mut pPk_2: *mut Index = sqlite3PrimaryKeyIndex(pTab_0);
                nPk_0 = (*pPk_2).nKeyCol as ::core::ffi::c_int;
                r_0 = sqlite3GetTempRange(pParse, nPk_0 + 1 as ::core::ffi::c_int);
                iPk_0 = 0 as ::core::ffi::c_int;
                while iPk_0 < nPk_0 {
                    let mut iCol_0: ::core::ffi::c_int = *(*pPk_2)
                        .aiColumn
                        .offset(iPk_0 as isize) as ::core::ffi::c_int;
                    sqlite3ExprCodeGetColumnOfTable(
                        v,
                        pTab_0,
                        iCur,
                        iCol_0,
                        r_0 + 1 as ::core::ffi::c_int + iPk_0,
                    );
                    iPk_0 += 1;
                }
            }
            jmp1_0 = sqlite3VdbeAddOp4Int(
                v,
                OP_Found,
                (*pRJ).iMatch,
                0 as ::core::ffi::c_int,
                r_0 + 1 as ::core::ffi::c_int,
                nPk_0,
            );
            sqlite3VdbeAddOp3(
                v,
                OP_MakeRecord,
                r_0 + 1 as ::core::ffi::c_int,
                nPk_0,
                r_0,
            );
            sqlite3VdbeAddOp4Int(
                v,
                OP_IdxInsert,
                (*pRJ).iMatch,
                r_0,
                r_0 + 1 as ::core::ffi::c_int,
                nPk_0,
            );
            sqlite3VdbeAddOp4Int(
                v,
                OP_FilterAdd,
                (*pRJ).regBloom,
                0 as ::core::ffi::c_int,
                r_0 + 1 as ::core::ffi::c_int,
                nPk_0,
            );
            sqlite3VdbeChangeP5(v, OPFLAG_USESEEKRESULT as u16_0);
            sqlite3VdbeJumpHere(v, jmp1_0);
            sqlite3ReleaseTempRange(pParse, r_0, nPk_0 + 1 as ::core::ffi::c_int);
        }
        if (*pLevel).iLeftJoin != 0 {
            (*pLevel).addrFirst = sqlite3VdbeCurrentAddr(v);
            sqlite3VdbeAddOp2(
                v,
                OP_Integer,
                1 as ::core::ffi::c_int,
                (*pLevel).iLeftJoin,
            );
            if (*pLevel).pRJ.is_null() {
                c2rust_current_block = 16777119596591587926;
            } else {
                c2rust_current_block = 16549556943091836366;
            }
        } else {
            c2rust_current_block = 16549556943091836366;
        }
        match c2rust_current_block {
            16549556943091836366 => {
                if !(*pLevel).pRJ.is_null() {
                    pRJ_0 = (*pLevel).pRJ;
                    sqlite3VdbeAddOp2(
                        v,
                        OP_BeginSubrtn,
                        0 as ::core::ffi::c_int,
                        (*pRJ_0).regReturn,
                    );
                    (*pRJ_0).addrSubrtn = sqlite3VdbeCurrentAddr(v);
                    (*pParse).withinRJSubrtn = (*pParse).withinRJSubrtn.wrapping_add(1);
                    c2rust_current_block = 16777119596591587926;
                } else {
                    c2rust_current_block = 8500432430644993475;
                }
            }
            _ => {}
        }
        match c2rust_current_block {
            16777119596591587926 => {
                pTerm = (*pWC).a;
                j = 0 as ::core::ffi::c_int;
                while j < (*pWC).nBase {
                    if !((*pTerm).wtFlags as ::core::ffi::c_int
                        & (TERM_VIRTUAL | TERM_CODED) != 0)
                    {
                        if !((*pTerm).prereqAll & (*pLevel).notReady
                            != 0 as ::core::ffi::c_ulonglong)
                        {
                            if !((*pTabItem).fg.jointype as ::core::ffi::c_int & JT_LTORJ
                                != 0)
                            {
                                sqlite3ExprIfFalse(
                                    pParse,
                                    (*pTerm).pExpr,
                                    addrCont,
                                    SQLITE_JUMPIFNULL,
                                );
                                (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int
                                    | TERM_CODED) as u16_0;
                            }
                        }
                    }
                    j += 1;
                    pTerm = pTerm.offset(1);
                }
            }
            _ => {}
        }
        return (*pLevel).notReady;
    }
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sqlite3WhereRightJoinLoop(
    mut pWInfo: *mut WhereInfo,
    mut iLevel: ::core::ffi::c_int,
    mut pLevel: *mut WhereLevel,
) {
    unsafe {
        let mut pParse: *mut Parse = (*pWInfo).pParse;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut pRJ: *mut WhereRightJoin = (*pLevel).pRJ;
        let mut pSubWhere: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pWC: *mut WhereClause = &raw mut (*pWInfo).sWC;
        let mut pSubWInfo: *mut WhereInfo = ::core::ptr::null_mut::<WhereInfo>();
        let mut pLoop: *mut WhereLoop = (*pLevel).pWLoop as *mut WhereLoop;
        let mut pTabItem: *mut SrcItem = (&raw mut (*(*pWInfo).pTabList).a
            as *mut SrcItem)
            .offset((*pLevel).iFrom as isize) as *mut SrcItem;
        let mut pFrom: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut uSrc: C2Rust_Unnamed_29 = C2Rust_Unnamed_29 {
            sSrc: SrcList {
                nSrc: 0,
                nAlloc: 0,
                a: []

            }

        };
        let mut mAll: Bitmask = 0 as Bitmask;
        let mut k: ::core::ffi::c_int = 0;
        sqlite3VdbeExplain(
            pParse,
            1 as u8_0,
            b"RIGHT-JOIN %s\0".as_ptr() as *const ::core::ffi::c_char,
            (*(*pTabItem).pSTab).zName,
        );
        k = 0 as ::core::ffi::c_int;
        while k < iLevel {
            let mut iIdxCur: ::core::ffi::c_int = 0;
            let mut pRight: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
            pRight = (&raw mut (*(*pWInfo).pTabList).a as *mut SrcItem)
                .offset(
                    (*(&raw mut (*pWInfo).a as *mut WhereLevel).offset(k as isize)).iFrom
                        as isize,
                ) as *mut SrcItem;
            mAll
                |= (*(*(&raw mut (*pWInfo).a as *mut WhereLevel).offset(k as isize))
                    .pWLoop)
                    .maskSelf as ::core::ffi::c_ulonglong;
            if (*pRight).fg.viaCoroutine() != 0 {
                let mut pSubq: *mut Subquery = ::core::ptr::null_mut::<Subquery>();
                pSubq = (*pRight).u4.pSubq;
                sqlite3VdbeAddOp3(
                    v,
                    OP_Null,
                    0 as ::core::ffi::c_int,
                    (*pSubq).regResult,
                    (*pSubq).regResult + (*(*(*pSubq).pSelect).pEList).nExpr
                        - 1 as ::core::ffi::c_int,
                );
            }
            sqlite3VdbeAddOp1(
                v,
                OP_NullRow,
                (*(&raw mut (*pWInfo).a as *mut WhereLevel).offset(k as isize)).iTabCur,
            );
            iIdxCur = (*(&raw mut (*pWInfo).a as *mut WhereLevel).offset(k as isize))
                .iIdxCur;
            if iIdxCur != 0 {
                sqlite3VdbeAddOp1(v, OP_NullRow, iIdxCur);
            }
            k += 1;
        }
        if (*pTabItem).fg.jointype as ::core::ffi::c_int & JT_LTORJ
            == 0 as ::core::ffi::c_int
        {
            mAll |= (*pLoop).maskSelf as ::core::ffi::c_ulonglong;
            k = 0 as ::core::ffi::c_int;
            while k < (*pWC).nTerm {
                let mut pTerm: *mut WhereTerm = (*pWC).a.offset(k as isize)
                    as *mut WhereTerm;
                if (*pTerm).wtFlags as ::core::ffi::c_int & (TERM_VIRTUAL | TERM_SLICE)
                    != 0 as ::core::ffi::c_int
                    && (*pTerm).eOperator as ::core::ffi::c_int != WO_ROWVAL
                {
                    break;
                }
                if !((*pTerm).prereqAll & !mAll != 0) {
                    if !((*(*pTerm).pExpr).flags
                        & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)
                            as u32_0 != 0 as ::core::ffi::c_uint)
                    {
                        pSubWhere = sqlite3ExprAnd(
                            pParse,
                            pSubWhere,
                            sqlite3ExprDup(
                                (*pParse).db,
                                (*pTerm).pExpr,
                                0 as ::core::ffi::c_int,
                            ),
                        );
                    }
                }
                k += 1;
            }
        }
        pFrom = &raw mut uSrc.sSrc;
        (*pFrom).nSrc = 1 as ::core::ffi::c_int;
        (*pFrom).nAlloc = 1 as u32_0;
        memcpy(
            (&raw mut (*pFrom).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem
                as *mut ::core::ffi::c_void,
            pTabItem as *const ::core::ffi::c_void,
            ::core::mem::size_of::<SrcItem>() as size_t,
        );
        (*(&raw mut (*pFrom).a as *mut SrcItem).offset(0 as ::core::ffi::c_int as isize))
            .fg
            .jointype = 0 as u8_0;
        (*pParse).withinRJSubrtn = (*pParse).withinRJSubrtn.wrapping_add(1);
        pSubWInfo = sqlite3WhereBegin(
            pParse,
            pFrom,
            pSubWhere,
            ::core::ptr::null_mut::<ExprList>(),
            ::core::ptr::null_mut::<ExprList>(),
            ::core::ptr::null_mut::<Select>(),
            WHERE_RIGHT_JOIN as u16_0,
            0 as ::core::ffi::c_int,
        );
        if !pSubWInfo.is_null() {
            let mut iCur: ::core::ffi::c_int = (*pLevel).iTabCur;
            (*pParse).nMem += 1;
            let mut r: ::core::ffi::c_int = (*pParse).nMem;
            let mut nPk: ::core::ffi::c_int = 0;
            let mut jmp: ::core::ffi::c_int = 0;
            let mut addrCont: ::core::ffi::c_int = sqlite3WhereContinueLabel(pSubWInfo);
            let mut pTab: *mut Table = (*pTabItem).pSTab;
            if (*pTab).tabFlags as ::core::ffi::c_uint
                & TF_WithoutRowid as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            {
                sqlite3ExprCodeGetColumnOfTable(
                    v,
                    pTab,
                    iCur,
                    -1 as ::core::ffi::c_int,
                    r,
                );
                nPk = 1 as ::core::ffi::c_int;
            } else {
                let mut iPk: ::core::ffi::c_int = 0;
                let mut pPk: *mut Index = sqlite3PrimaryKeyIndex(pTab);
                nPk = (*pPk).nKeyCol as ::core::ffi::c_int;
                (*pParse).nMem += nPk - 1 as ::core::ffi::c_int;
                iPk = 0 as ::core::ffi::c_int;
                while iPk < nPk {
                    let mut iCol: ::core::ffi::c_int = *(*pPk)
                        .aiColumn
                        .offset(iPk as isize) as ::core::ffi::c_int;
                    sqlite3ExprCodeGetColumnOfTable(v, pTab, iCur, iCol, r + iPk);
                    iPk += 1;
                }
            }
            jmp = sqlite3VdbeAddOp4Int(
                v,
                OP_Filter,
                (*pRJ).regBloom,
                0 as ::core::ffi::c_int,
                r,
                nPk,
            );
            sqlite3VdbeAddOp4Int(v, OP_Found, (*pRJ).iMatch, addrCont, r, nPk);
            sqlite3VdbeJumpHere(v, jmp);
            sqlite3VdbeAddOp2(v, OP_Gosub, (*pRJ).regReturn, (*pRJ).addrSubrtn);
            sqlite3WhereEnd(pSubWInfo);
        }
        sqlite3ExprDelete((*pParse).db, pSubWhere);
        sqlite3VdbeExplainPop(pParse);
        (*pParse).withinRJSubrtn = (*pParse).withinRJSubrtn.wrapping_sub(1);
    }
}
pub const SQLITE_MAX_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
pub const SQLITE_SO_ASC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_SO_DESC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_AFF_BLOB: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_IDXTYPE_PRIMARYKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XN_ROWID: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const XN_EXPR: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const EP_xIsSelect: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const JT_LEFT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const JT_RIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const JT_LTORJ: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const WHERE_ORDERBY_MIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const WHERE_ORDERBY_MAX: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WHERE_DUPLICATES_OK: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const WHERE_OR_SUBCLAUSE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const WHERE_RIGHT_JOIN: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const OPFLAG_USESEEKRESULT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_PRINTF_INTERNAL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const IN_INDEX_ROWID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IN_INDEX_INDEX_DESC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const IN_INDEX_NOOP: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const IN_INDEX_LOOP: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const TK_AND: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const TK_IS: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const TK_ISNULL: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const TK_EQ: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const TK_GT: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
pub const TK_LE: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const TK_LT: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
pub const TK_GE: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
pub const TK_REGISTER: ::core::ffi::c_int = 176 as ::core::ffi::c_int;
