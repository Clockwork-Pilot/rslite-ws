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
    fn sqlite3VdbeChangeToNoop(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeDeletePriorOpcode(_: *mut Vdbe, op: u8_0) -> ::core::ffi::c_int;
    fn sqlite3VdbeAppendP4(
        _: *mut Vdbe,
        pP4: *mut ::core::ffi::c_void,
        p4type: ::core::ffi::c_int,
    );
    fn sqlite3VdbeSetP4KeyInfo(_: *mut Parse, _: *mut Index);
    fn sqlite3VdbeMakeLabel(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3VdbeResolveLabel(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeCurrentAddr(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeSetNumCols(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeSetColName(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeCountChanges(_: *mut Vdbe);
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbNNFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3GetTempRange(_: *mut Parse, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempRange(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprDelete(_: *mut sqlite3, _: *mut Expr);
    fn sqlite3PrimaryKeyIndex(_: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToStorage(_: *mut Table, _: i16_0) -> i16_0;
    fn sqlite3ViewGetColumnNames(_: *mut Parse, _: *mut Table) -> ::core::ffi::c_int;
    fn sqlite3DeleteTable(_: *mut sqlite3, _: *mut Table);
    fn sqlite3AutoincrementEnd(pParse: *mut Parse);
    fn sqlite3SrcListAppend(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Token,
        _: *mut Token,
    ) -> *mut SrcList;
    fn sqlite3IndexedByLookup(_: *mut Parse, _: *mut SrcItem) -> ::core::ffi::c_int;
    fn sqlite3SrcListDelete(_: *mut sqlite3, _: *mut SrcList);
    fn sqlite3Select(
        _: *mut Parse,
        _: *mut Select,
        _: *mut SelectDest,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3WhereOkOnePass(
        _: *mut WhereInfo,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3WhereUsesDeferredSeek(_: *mut WhereInfo) -> ::core::ffi::c_int;
    fn sqlite3ExprCodeLoadIndexColumn(
        _: *mut Parse,
        _: *mut Index,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprCodeGetColumnOfTable(
        _: *mut Vdbe,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprIfFalseDup(
        _: *mut Parse,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3LocateTableItem(
        _: *mut Parse,
        flags: u32_0,
        _: *mut SrcItem,
    ) -> *mut Table;
    fn sqlite3GetVdbe(_: *mut Parse) -> *mut Vdbe;
    fn sqlite3OpenTableAndIndices(
        _: *mut Parse,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: u8_0,
        _: ::core::ffi::c_int,
        _: *mut u8_0,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BeginWriteOperation(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3MultiWrite(_: *mut Parse);
    fn sqlite3MayAbort(_: *mut Parse);
    fn sqlite3ExprDup(
        _: *mut sqlite3,
        _: *const Expr,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3TriggersExist(
        _: *mut Parse,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: *mut ExprList,
        pMask: *mut ::core::ffi::c_int,
    ) -> *mut Trigger;
    fn sqlite3CodeRowTrigger(
        _: *mut Parse,
        _: *mut Trigger,
        _: ::core::ffi::c_int,
        _: *mut ExprList,
        _: ::core::ffi::c_int,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3TriggerColmask(
        _: *mut Parse,
        _: *mut Trigger,
        _: *mut ExprList,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut Table,
        _: ::core::ffi::c_int,
    ) -> u32_0;
    fn sqlite3AuthCheck(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3AuthContextPush(
        _: *mut Parse,
        _: *mut AuthContext,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3AuthContextPop(_: *mut AuthContext);
    fn sqlite3IndexAffinityStr(
        _: *mut sqlite3,
        _: *mut Index,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3WritableSchema(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3ResolveExprNames(_: *mut NameContext, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
    fn sqlite3SelectDestInit(
        _: *mut SelectDest,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3GetVTable(_: *mut sqlite3, _: *mut Table) -> *mut VTable;
    fn sqlite3ReadOnlyShadowTables(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3VtabMakeWritable(_: *mut Parse, _: *mut Table);
    fn sqlite3FkCheck(
        _: *mut Parse,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3FkActions(
        _: *mut Parse,
        _: *mut Table,
        _: *mut ExprList,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3FkRequired(
        _: *mut Parse,
        _: *mut Table,
        _: *mut ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3FkOldmask(_: *mut Parse, _: *mut Table) -> u32_0;
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
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AuthContext {
    pub zAuthContext: *const ::core::ffi::c_char,
    pub pParse: *mut Parse

}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SelectDest {
    pub eDest: u8_0,
    pub iSDParm: ::core::ffi::c_int,
    pub iSDParm2: ::core::ffi::c_int,
    pub iSdst: ::core::ffi::c_int,
    pub nSdst: ::core::ffi::c_int,
    pub zAffSdst: *mut ::core::ffi::c_char,
    pub pOrderBy: *mut ExprList

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_DENY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DELETE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const OP_VUpdate: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const OP_Once: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const OP_NotFound: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const OP_NotExists: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const OP_RowSetRead: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const OP_Copy: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
pub const OP_FkCheck: ::core::ffi::c_int = 84 as ::core::ffi::c_int;
pub const OP_ResultRow: ::core::ffi::c_int = 85 as ::core::ffi::c_int;
pub const OP_AddImm: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
pub const OP_RealAffinity: ::core::ffi::c_int = 88 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_MakeRecord: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const OP_OpenWrite: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const OP_OpenEphemeral: ::core::ffi::c_int = 119 as ::core::ffi::c_int;
pub const OP_Close: ::core::ffi::c_int = 123 as ::core::ffi::c_int;
pub const OP_Delete: ::core::ffi::c_int = 131 as ::core::ffi::c_int;
pub const OP_RowData: ::core::ffi::c_int = 135 as ::core::ffi::c_int;
pub const OP_IdxInsert: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const OP_IdxDelete: ::core::ffi::c_int = 141 as ::core::ffi::c_int;
pub const OP_FinishSeek: ::core::ffi::c_int = 144 as ::core::ffi::c_int;
pub const OP_Clear: ::core::ffi::c_int = 146 as ::core::ffi::c_int;
pub const OP_RowSetAdd: ::core::ffi::c_int = 157 as ::core::ffi::c_int;
pub const P4_STATIC: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const P4_TABLE: ::core::ffi::c_int = -5 as ::core::ffi::c_int;
pub const P4_VTAB: ::core::ffi::c_int = -11 as ::core::ffi::c_int;
pub const COLNAME_NAME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcListLookup(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
) -> *mut Table {
    unsafe {
        let mut pItem: *mut SrcItem = &raw mut (*pSrc).a as *mut SrcItem;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        pTab = sqlite3LocateTableItem(pParse, 0 as u32_0, pItem);
        if !(*pItem).pSTab.is_null() {
            sqlite3DeleteTable((*pParse).db, (*pItem).pSTab);
        }
        (*pItem).pSTab = pTab;
        (*pItem).fg.set_notCte(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if !pTab.is_null() {
            (*pTab).nTabRef = (*pTab).nTabRef.wrapping_add(1);
            if (*pItem).fg.isIndexedBy() as ::core::ffi::c_int != 0
                && sqlite3IndexedByLookup(pParse, pItem) != 0
            {
                pTab = ::core::ptr::null_mut::<Table>();
            }
        }
        return pTab;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CodeChangeCount(
    mut v: *mut Vdbe,
    mut regCounter: ::core::ffi::c_int,
    mut zColName: *const ::core::ffi::c_char,
) {
    unsafe {
        sqlite3VdbeAddOp0(v, OP_FkCheck);
        sqlite3VdbeAddOp2(v, OP_ResultRow, regCounter, 1 as ::core::ffi::c_int);
        sqlite3VdbeSetNumCols(v, 1 as ::core::ffi::c_int);
        sqlite3VdbeSetColName(
            v,
            0 as ::core::ffi::c_int,
            COLNAME_NAME,
            zColName,
            SQLITE_STATIC,
        );
    }
}
unsafe extern "C" fn vtabIsReadOnly(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
) -> ::core::ffi::c_int {
    unsafe {
        if (*(*(*sqlite3GetVTable((*pParse).db, pTab)).pMod).pModule).xUpdate.is_none() {
            return 1 as ::core::ffi::c_int;
        }
        if !(*pParse).pToplevel.is_null()
            && (*(*pTab).u.vtab.p).eVtabRisk as ::core::ffi::c_int
                > ((*(*pParse).db).flags as ::core::ffi::c_ulonglong
                    & SQLITE_TrustedSchema as ::core::ffi::c_ulonglong
                    != 0 as ::core::ffi::c_ulonglong) as ::core::ffi::c_int
        {
            sqlite3ErrorMsg(
                pParse,
                b"unsafe use of virtual table \"%s\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pTab).zName,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn tabIsReadOnly(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
            return vtabIsReadOnly(pParse, pTab);
        }
        if (*pTab).tabFlags as ::core::ffi::c_uint
            & (TF_Readonly | TF_Shadow) as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            return 0 as ::core::ffi::c_int;
        }
        db = (*pParse).db;
        if (*pTab).tabFlags as ::core::ffi::c_uint & TF_Readonly as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            return (sqlite3WritableSchema(db) == 0 as ::core::ffi::c_int
                && (*pParse).nested as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        }
        return sqlite3ReadOnlyShadowTables(db);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IsReadOnly(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pTrigger: *mut Trigger,
) -> ::core::ffi::c_int {
    unsafe {
        if tabIsReadOnly(pParse, pTab) != 0 {
            sqlite3ErrorMsg(
                pParse,
                b"table %s may not be modified\0".as_ptr() as *const ::core::ffi::c_char,
                (*pTab).zName,
            );
            return 1 as ::core::ffi::c_int;
        }
        if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW
            && (pTrigger.is_null()
                || (*pTrigger).bReturning as ::core::ffi::c_int != 0
                    && (*pTrigger).pNext.is_null())
        {
            sqlite3ErrorMsg(
                pParse,
                b"cannot modify %s because it is a view\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pTab).zName,
            );
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MaterializeView(
    mut pParse: *mut Parse,
    mut pView: *mut Table,
    mut pWhere: *mut Expr,
    mut pOrderBy: *mut ExprList,
    mut pLimit: *mut Expr,
    mut iCur: ::core::ffi::c_int,
) {
    unsafe {
        let mut dest: SelectDest = SelectDest {
            eDest: 0,
            iSDParm: 0,
            iSDParm2: 0,
            iSdst: 0,
            nSdst: 0,
            zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pOrderBy: ::core::ptr::null_mut::<ExprList>()

        };
        let mut pSel: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pFrom: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut iDb: ::core::ffi::c_int = sqlite3SchemaToIndex(db, (*pView).pSchema);
        pWhere = sqlite3ExprDup(db, pWhere, 0 as ::core::ffi::c_int);
        pFrom = sqlite3SrcListAppend(
            pParse,
            ::core::ptr::null_mut::<SrcList>(),
            ::core::ptr::null_mut::<Token>(),
            ::core::ptr::null_mut::<Token>(),
        );
        if !pFrom.is_null() {
            let ref mut c2rust_fresh3 = (*(&raw mut (*pFrom).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .zName;
            *c2rust_fresh3 = sqlite3DbStrDup(db, (*pView).zName);
            let ref mut c2rust_fresh4 = (*(&raw mut (*pFrom).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .u4
                .zDatabase;
            *c2rust_fresh4 = sqlite3DbStrDup(
                db,
                (*(*db).aDb.offset(iDb as isize)).zDbSName,
            );
        }
        pSel = sqlite3SelectNew(
            pParse,
            ::core::ptr::null_mut::<ExprList>(),
            pFrom,
            pWhere,
            ::core::ptr::null_mut::<ExprList>(),
            ::core::ptr::null_mut::<Expr>(),
            pOrderBy,
            SF_IncludeHidden as u32_0,
            pLimit,
        );
        sqlite3SelectDestInit(&raw mut dest, SRT_EphemTab, iCur);
        sqlite3Select(pParse, pSel, &raw mut dest);
        sqlite3SelectDelete(db, pSel);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DeleteFrom(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pWhere: *mut Expr,
    mut pOrderBy: *mut ExprList,
    mut pLimit: *mut Expr,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut i: ::core::ffi::c_int = 0;
        let mut pWInfo: *mut WhereInfo = ::core::ptr::null_mut::<WhereInfo>();
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut iTabCur: ::core::ffi::c_int = 0;
        let mut iDataCur: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iIdxCur: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nIdx: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut sContext: AuthContext = AuthContext {
            zAuthContext: ::core::ptr::null::<::core::ffi::c_char>(),
            pParse: ::core::ptr::null_mut::<Parse>()

        };
        let mut sNC: NameContext = NameContext {
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
        let mut iDb: ::core::ffi::c_int = 0;
        let mut memCnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rcauth: ::core::ffi::c_int = 0;
        let mut eOnePass: ::core::ffi::c_int = 0;
        let mut aiCurOnePass: [::core::ffi::c_int; 2] = [0; 2];
        let mut aToOpen: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pPk: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut iPk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nPk: i16_0 = 1 as i16_0;
        let mut iKey: ::core::ffi::c_int = 0;
        let mut nKey: i16_0 = 0;
        let mut iEphCur: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iRowSet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addrBypass: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addrLoop: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addrEphOpen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bComplex: ::core::ffi::c_int = 0;
        let mut isView: ::core::ffi::c_int = 0;
        let mut pTrigger: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        memset(
            &raw mut sContext as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<AuthContext>() as size_t,
        );
        db = (*pParse).db;
        if !((*pParse).nErr != 0) {
            pTab = sqlite3SrcListLookup(pParse, pTabList);
            if !pTab.is_null() {
                pTrigger = sqlite3TriggersExist(
                    pParse,
                    pTab,
                    TK_DELETE,
                    ::core::ptr::null_mut::<ExprList>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                isView = ((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW)
                    as ::core::ffi::c_int;
                bComplex = (!pTrigger.is_null()
                    || sqlite3FkRequired(
                        pParse,
                        pTab,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        0 as ::core::ffi::c_int,
                    ) != 0) as ::core::ffi::c_int;
                if !(sqlite3ViewGetColumnNames(pParse, pTab) != 0) {
                    if !(sqlite3IsReadOnly(pParse, pTab, pTrigger) != 0) {
                        iDb = sqlite3SchemaToIndex(db, (*pTab).pSchema);
                        rcauth = sqlite3AuthCheck(
                            pParse,
                            SQLITE_DELETE,
                            (*pTab).zName,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            (*(*db).aDb.offset(iDb as isize)).zDbSName,
                        );
                        if !(rcauth == SQLITE_DENY) {
                            let c2rust_fresh0 = (*pParse).nTab;
                            (*pParse).nTab = (*pParse).nTab + 1;
                            let ref mut c2rust_fresh1 = (*(&raw mut (*pTabList).a
                                as *mut SrcItem)
                                .offset(0 as ::core::ffi::c_int as isize))
                                .iCursor;
                            *c2rust_fresh1 = c2rust_fresh0;
                            iTabCur = *c2rust_fresh1;
                            nIdx = 0 as ::core::ffi::c_int;
                            pIdx = (*pTab).pIndex;
                            while !pIdx.is_null() {
                                (*pParse).nTab += 1;
                                pIdx = (*pIdx).pNext;
                                nIdx += 1;
                            }
                            if isView != 0 {
                                sqlite3AuthContextPush(
                                    pParse,
                                    &raw mut sContext,
                                    (*pTab).zName,
                                );
                            }
                            v = sqlite3GetVdbe(pParse);
                            if !v.is_null() {
                                if (*pParse).nested as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    sqlite3VdbeCountChanges(v);
                                }
                                sqlite3BeginWriteOperation(pParse, bComplex, iDb);
                                if isView != 0 {
                                    sqlite3MaterializeView(
                                        pParse,
                                        pTab,
                                        pWhere,
                                        pOrderBy,
                                        pLimit,
                                        iTabCur,
                                    );
                                    iIdxCur = iTabCur;
                                    iDataCur = iIdxCur;
                                    pOrderBy = ::core::ptr::null_mut::<ExprList>();
                                    pLimit = ::core::ptr::null_mut::<Expr>();
                                }
                                memset(
                                    &raw mut sNC as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    ::core::mem::size_of::<NameContext>() as size_t,
                                );
                                sNC.pParse = pParse;
                                sNC.pSrcList = pTabList;
                                if !(sqlite3ResolveExprNames(&raw mut sNC, pWhere) != 0) {
                                    if (*db).flags & SQLITE_CountRows
                                        != 0 as ::core::ffi::c_ulonglong && (*pParse).nested == 0
                                        && (*pParse).pTriggerTab.is_null()
                                        && (*pParse).bReturning == 0
                                    {
                                        (*pParse).nMem += 1;
                                        memCnt = (*pParse).nMem;
                                        sqlite3VdbeAddOp2(
                                            v,
                                            OP_Integer,
                                            0 as ::core::ffi::c_int,
                                            memCnt,
                                        );
                                    }
                                    if rcauth == SQLITE_OK && pWhere.is_null() && bComplex == 0
                                        && !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB)
                                    {
                                        if (*pTab).tabFlags as ::core::ffi::c_uint
                                            & TF_WithoutRowid as ::core::ffi::c_uint
                                            == 0 as ::core::ffi::c_uint
                                        {
                                            sqlite3VdbeAddOp4(
                                                v,
                                                OP_Clear,
                                                (*pTab).tnum as ::core::ffi::c_int,
                                                iDb,
                                                if memCnt != 0 { memCnt } else { -1 as ::core::ffi::c_int },
                                                (*pTab).zName,
                                                P4_STATIC,
                                            );
                                        }
                                        pIdx = (*pTab).pIndex;
                                        while !pIdx.is_null() {
                                            if (*pIdx).idxType() as ::core::ffi::c_int
                                                == SQLITE_IDXTYPE_PRIMARYKEY
                                                && !((*pTab).tabFlags as ::core::ffi::c_uint
                                                    & TF_WithoutRowid as ::core::ffi::c_uint
                                                    == 0 as ::core::ffi::c_uint)
                                            {
                                                sqlite3VdbeAddOp3(
                                                    v,
                                                    OP_Clear,
                                                    (*pIdx).tnum as ::core::ffi::c_int,
                                                    iDb,
                                                    if memCnt != 0 { memCnt } else { -1 as ::core::ffi::c_int },
                                                );
                                            } else {
                                                sqlite3VdbeAddOp2(
                                                    v,
                                                    OP_Clear,
                                                    (*pIdx).tnum as ::core::ffi::c_int,
                                                    iDb,
                                                );
                                            }
                                            pIdx = (*pIdx).pNext;
                                        }
                                        c2rust_current_block = 8968043056769084000;
                                    } else {
                                        let mut wcf: u16_0 = (WHERE_ONEPASS_DESIRED
                                            | WHERE_DUPLICATES_OK) as u16_0;
                                        if sNC.ncFlags & NC_Subquery != 0 {
                                            bComplex = 1 as ::core::ffi::c_int;
                                        }
                                        wcf = (wcf as ::core::ffi::c_int
                                            | if bComplex != 0 {
                                                0 as ::core::ffi::c_int
                                            } else {
                                                WHERE_ONEPASS_MULTIROW
                                            }) as u16_0;
                                        if (*pTab).tabFlags as ::core::ffi::c_uint
                                            & TF_WithoutRowid as ::core::ffi::c_uint
                                            == 0 as ::core::ffi::c_uint
                                        {
                                            pPk = ::core::ptr::null_mut::<Index>();
                                            (*pParse).nMem += 1;
                                            iRowSet = (*pParse).nMem;
                                            sqlite3VdbeAddOp2(
                                                v,
                                                OP_Null,
                                                0 as ::core::ffi::c_int,
                                                iRowSet,
                                            );
                                        } else {
                                            pPk = sqlite3PrimaryKeyIndex(pTab);
                                            nPk = (*pPk).nKeyCol as i16_0;
                                            iPk = (*pParse).nMem + 1 as ::core::ffi::c_int;
                                            (*pParse).nMem += nPk as ::core::ffi::c_int;
                                            let c2rust_fresh2 = (*pParse).nTab;
                                            (*pParse).nTab = (*pParse).nTab + 1;
                                            iEphCur = c2rust_fresh2;
                                            addrEphOpen = sqlite3VdbeAddOp2(
                                                v,
                                                OP_OpenEphemeral,
                                                iEphCur,
                                                nPk as ::core::ffi::c_int,
                                            );
                                            sqlite3VdbeSetP4KeyInfo(pParse, pPk);
                                        }
                                        pWInfo = sqlite3WhereBegin(
                                            pParse,
                                            pTabList,
                                            pWhere,
                                            ::core::ptr::null_mut::<ExprList>(),
                                            ::core::ptr::null_mut::<ExprList>(),
                                            ::core::ptr::null_mut::<Select>(),
                                            wcf,
                                            iTabCur + 1 as ::core::ffi::c_int,
                                        );
                                        if pWInfo.is_null() {
                                            c2rust_current_block = 4429012388999056353;
                                        } else {
                                            eOnePass = sqlite3WhereOkOnePass(
                                                pWInfo,
                                                &raw mut aiCurOnePass as *mut ::core::ffi::c_int,
                                            );
                                            if eOnePass != ONEPASS_SINGLE {
                                                sqlite3MultiWrite(pParse);
                                            }
                                            if sqlite3WhereUsesDeferredSeek(pWInfo) != 0 {
                                                sqlite3VdbeAddOp1(v, OP_FinishSeek, iTabCur);
                                            }
                                            if memCnt != 0 {
                                                sqlite3VdbeAddOp2(
                                                    v,
                                                    OP_AddImm,
                                                    memCnt,
                                                    1 as ::core::ffi::c_int,
                                                );
                                            }
                                            if !pPk.is_null() {
                                                i = 0 as ::core::ffi::c_int;
                                                while i < nPk as ::core::ffi::c_int {
                                                    sqlite3ExprCodeGetColumnOfTable(
                                                        v,
                                                        pTab,
                                                        iTabCur,
                                                        *(*pPk).aiColumn.offset(i as isize) as ::core::ffi::c_int,
                                                        iPk + i,
                                                    );
                                                    i += 1;
                                                }
                                                iKey = iPk;
                                            } else {
                                                (*pParse).nMem += 1;
                                                iKey = (*pParse).nMem;
                                                sqlite3ExprCodeGetColumnOfTable(
                                                    v,
                                                    pTab,
                                                    iTabCur,
                                                    -1 as ::core::ffi::c_int,
                                                    iKey,
                                                );
                                            }
                                            if eOnePass != ONEPASS_OFF {
                                                nKey = nPk;
                                                aToOpen = sqlite3DbMallocRawNN(
                                                    db,
                                                    (nIdx + 2 as ::core::ffi::c_int) as u64_0,
                                                ) as *mut u8_0;
                                                if aToOpen.is_null() {
                                                    sqlite3WhereEnd(pWInfo);
                                                    c2rust_current_block = 4429012388999056353;
                                                } else {
                                                    memset(
                                                        aToOpen as *mut ::core::ffi::c_void,
                                                        1 as ::core::ffi::c_int,
                                                        (nIdx + 1 as ::core::ffi::c_int) as size_t,
                                                    );
                                                    *aToOpen
                                                        .offset((nIdx + 1 as ::core::ffi::c_int) as isize) = 0
                                                        as u8_0;
                                                    if aiCurOnePass[0 as ::core::ffi::c_int as usize]
                                                        >= 0 as ::core::ffi::c_int
                                                    {
                                                        *aToOpen
                                                            .offset(
                                                                (aiCurOnePass[0 as ::core::ffi::c_int as usize] - iTabCur)
                                                                    as isize,
                                                            ) = 0 as u8_0;
                                                    }
                                                    if aiCurOnePass[1 as ::core::ffi::c_int as usize]
                                                        >= 0 as ::core::ffi::c_int
                                                    {
                                                        *aToOpen
                                                            .offset(
                                                                (aiCurOnePass[1 as ::core::ffi::c_int as usize] - iTabCur)
                                                                    as isize,
                                                            ) = 0 as u8_0;
                                                    }
                                                    if addrEphOpen != 0 {
                                                        sqlite3VdbeChangeToNoop(v, addrEphOpen);
                                                    }
                                                    addrBypass = sqlite3VdbeMakeLabel(pParse);
                                                    c2rust_current_block = 14913924298693586572;
                                                }
                                            } else {
                                                if !pPk.is_null() {
                                                    (*pParse).nMem += 1;
                                                    iKey = (*pParse).nMem;
                                                    nKey = 0 as i16_0;
                                                    sqlite3VdbeAddOp4(
                                                        v,
                                                        OP_MakeRecord,
                                                        iPk,
                                                        nPk as ::core::ffi::c_int,
                                                        iKey,
                                                        sqlite3IndexAffinityStr((*pParse).db, pPk),
                                                        nPk as ::core::ffi::c_int,
                                                    );
                                                    sqlite3VdbeAddOp4Int(
                                                        v,
                                                        OP_IdxInsert,
                                                        iEphCur,
                                                        iKey,
                                                        iPk,
                                                        nPk as ::core::ffi::c_int,
                                                    );
                                                } else {
                                                    nKey = 1 as i16_0;
                                                    sqlite3VdbeAddOp2(v, OP_RowSetAdd, iRowSet, iKey);
                                                }
                                                sqlite3WhereEnd(pWInfo);
                                                c2rust_current_block = 14913924298693586572;
                                            }
                                            match c2rust_current_block {
                                                4429012388999056353 => {}
                                                _ => {
                                                    if isView == 0 {
                                                        let mut iAddrOnce: ::core::ffi::c_int = 0
                                                            as ::core::ffi::c_int;
                                                        if eOnePass == ONEPASS_MULTI {
                                                            iAddrOnce = sqlite3VdbeAddOp0(v, OP_Once);
                                                        }
                                                        sqlite3OpenTableAndIndices(
                                                            pParse,
                                                            pTab,
                                                            OP_OpenWrite,
                                                            OPFLAG_FORDELETE as u8_0,
                                                            iTabCur,
                                                            aToOpen,
                                                            &raw mut iDataCur,
                                                            &raw mut iIdxCur,
                                                        );
                                                        if eOnePass == ONEPASS_MULTI {
                                                            sqlite3VdbeJumpHereOrPopInst(v, iAddrOnce);
                                                        }
                                                    }
                                                    if eOnePass != ONEPASS_OFF {
                                                        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB)
                                                            && *aToOpen.offset((iDataCur - iTabCur) as isize)
                                                                as ::core::ffi::c_int != 0
                                                        {
                                                            sqlite3VdbeAddOp4Int(
                                                                v,
                                                                OP_NotFound,
                                                                iDataCur,
                                                                addrBypass,
                                                                iKey,
                                                                nKey as ::core::ffi::c_int,
                                                            );
                                                        }
                                                    } else if !pPk.is_null() {
                                                        addrLoop = sqlite3VdbeAddOp1(v, OP_Rewind, iEphCur);
                                                        if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                                                            sqlite3VdbeAddOp3(
                                                                v,
                                                                OP_Column,
                                                                iEphCur,
                                                                0 as ::core::ffi::c_int,
                                                                iKey,
                                                            );
                                                        } else {
                                                            sqlite3VdbeAddOp2(v, OP_RowData, iEphCur, iKey);
                                                        }
                                                    } else {
                                                        addrLoop = sqlite3VdbeAddOp3(
                                                            v,
                                                            OP_RowSetRead,
                                                            iRowSet,
                                                            0 as ::core::ffi::c_int,
                                                            iKey,
                                                        );
                                                    }
                                                    if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                                                        let mut pVTab: *const ::core::ffi::c_char = sqlite3GetVTable(
                                                            db,
                                                            pTab,
                                                        ) as *const ::core::ffi::c_char;
                                                        sqlite3VtabMakeWritable(pParse, pTab);
                                                        sqlite3MayAbort(pParse);
                                                        if eOnePass == ONEPASS_SINGLE {
                                                            sqlite3VdbeAddOp1(v, OP_Close, iTabCur);
                                                            if (*pParse).pToplevel.is_null() {
                                                                (*pParse).isMultiWrite = 0 as u8_0;
                                                            }
                                                        }
                                                        sqlite3VdbeAddOp4(
                                                            v,
                                                            OP_VUpdate,
                                                            0 as ::core::ffi::c_int,
                                                            1 as ::core::ffi::c_int,
                                                            iKey,
                                                            pVTab,
                                                            P4_VTAB,
                                                        );
                                                        sqlite3VdbeChangeP5(v, OE_Abort as u16_0);
                                                    } else {
                                                        let mut count: ::core::ffi::c_int = ((*pParse).nested
                                                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                                                            as ::core::ffi::c_int;
                                                        sqlite3GenerateRowDelete(
                                                            pParse,
                                                            pTab,
                                                            pTrigger,
                                                            iDataCur,
                                                            iIdxCur,
                                                            iKey,
                                                            nKey,
                                                            count as u8_0,
                                                            OE_Default as u8_0,
                                                            eOnePass as u8_0,
                                                            aiCurOnePass[1 as ::core::ffi::c_int as usize],
                                                        );
                                                    }
                                                    if eOnePass != ONEPASS_OFF {
                                                        sqlite3VdbeResolveLabel(v, addrBypass);
                                                        sqlite3WhereEnd(pWInfo);
                                                    } else if !pPk.is_null() {
                                                        sqlite3VdbeAddOp2(
                                                            v,
                                                            OP_Next,
                                                            iEphCur,
                                                            addrLoop + 1 as ::core::ffi::c_int,
                                                        );
                                                        sqlite3VdbeJumpHere(v, addrLoop);
                                                    } else {
                                                        sqlite3VdbeGoto(v, addrLoop);
                                                        sqlite3VdbeJumpHere(v, addrLoop);
                                                    }
                                                    c2rust_current_block = 8968043056769084000;
                                                }
                                            }
                                        }
                                    }
                                    match c2rust_current_block {
                                        4429012388999056353 => {}
                                        _ => {
                                            if (*pParse).nested as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                                && (*pParse).pTriggerTab.is_null()
                                            {
                                                sqlite3AutoincrementEnd(pParse);
                                            }
                                            if memCnt != 0 {
                                                sqlite3CodeChangeCount(
                                                    v,
                                                    memCnt,
                                                    b"rows deleted\0".as_ptr() as *const ::core::ffi::c_char,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        sqlite3AuthContextPop(&raw mut sContext);
        sqlite3SrcListDelete(db, pTabList);
        sqlite3ExprDelete(db, pWhere);
        if !aToOpen.is_null() {
            sqlite3DbNNFreeNN(db, aToOpen as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GenerateRowDelete(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pTrigger: *mut Trigger,
    mut iDataCur: ::core::ffi::c_int,
    mut iIdxCur: ::core::ffi::c_int,
    mut iPk: ::core::ffi::c_int,
    mut nPk: i16_0,
    mut count: u8_0,
    mut onconf: u8_0,
    mut eMode: u8_0,
    mut iIdxNoSeek: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut iOld: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iLabel: ::core::ffi::c_int = 0;
        let mut opSeek: u8_0 = 0;
        iLabel = sqlite3VdbeMakeLabel(pParse);
        opSeek = (if (*pTab).tabFlags as ::core::ffi::c_uint
            & TF_WithoutRowid as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
        {
            OP_NotExists
        } else {
            OP_NotFound
        }) as u8_0;
        if eMode as ::core::ffi::c_int == ONEPASS_OFF {
            sqlite3VdbeAddOp4Int(
                v,
                opSeek as ::core::ffi::c_int,
                iDataCur,
                iLabel,
                iPk,
                nPk as ::core::ffi::c_int,
            );
        }
        if sqlite3FkRequired(
            pParse,
            pTab,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        ) != 0 || !pTrigger.is_null()
        {
            let mut mask: u32_0 = 0;
            let mut iCol: ::core::ffi::c_int = 0;
            let mut addrStart: ::core::ffi::c_int = 0;
            mask = sqlite3TriggerColmask(
                pParse,
                pTrigger,
                ::core::ptr::null_mut::<ExprList>(),
                0 as ::core::ffi::c_int,
                TRIGGER_BEFORE | TRIGGER_AFTER,
                pTab,
                onconf as ::core::ffi::c_int,
            );
            mask |= sqlite3FkOldmask(pParse, pTab) as ::core::ffi::c_uint;
            iOld = (*pParse).nMem + 1 as ::core::ffi::c_int;
            (*pParse).nMem
                += 1 as ::core::ffi::c_int + (*pTab).nCol as ::core::ffi::c_int;
            sqlite3VdbeAddOp2(v, OP_Copy, iPk, iOld);
            iCol = 0 as ::core::ffi::c_int;
            while iCol < (*pTab).nCol as ::core::ffi::c_int {
                if mask == 0xffffffff as ::core::ffi::c_uint
                    || iCol <= 31 as ::core::ffi::c_int
                        && mask as ::core::ffi::c_uint
                            & (1 as ::core::ffi::c_int as ::core::ffi::c_uint) << iCol
                            != 0 as ::core::ffi::c_uint
                {
                    let mut kk: ::core::ffi::c_int = sqlite3TableColumnToStorage(
                        pTab,
                        iCol as i16_0,
                    ) as ::core::ffi::c_int;
                    sqlite3ExprCodeGetColumnOfTable(
                        v,
                        pTab,
                        iDataCur,
                        iCol,
                        iOld + kk + 1 as ::core::ffi::c_int,
                    );
                }
                iCol += 1;
            }
            addrStart = sqlite3VdbeCurrentAddr(v);
            sqlite3CodeRowTrigger(
                pParse,
                pTrigger,
                TK_DELETE,
                ::core::ptr::null_mut::<ExprList>(),
                TRIGGER_BEFORE,
                pTab,
                iOld,
                onconf as ::core::ffi::c_int,
                iLabel,
            );
            if addrStart < sqlite3VdbeCurrentAddr(v) {
                sqlite3VdbeAddOp4Int(
                    v,
                    opSeek as ::core::ffi::c_int,
                    iDataCur,
                    iLabel,
                    iPk,
                    nPk as ::core::ffi::c_int,
                );
                iIdxNoSeek = -1 as ::core::ffi::c_int;
            }
            sqlite3FkCheck(
                pParse,
                pTab,
                iOld,
                0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                0 as ::core::ffi::c_int,
            );
        }
        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW) {
            let mut p5: u8_0 = 0 as u8_0;
            sqlite3GenerateRowIndexDelete(
                pParse,
                pTab,
                iDataCur,
                iIdxCur,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                iIdxNoSeek,
            );
            sqlite3VdbeAddOp2(
                v,
                OP_Delete,
                iDataCur,
                if count as ::core::ffi::c_int != 0 {
                    OPFLAG_NCHANGE
                } else {
                    0 as ::core::ffi::c_int
                },
            );
            if (*pParse).nested as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || 0 as ::core::ffi::c_int
                    == sqlite3_stricmp(
                        (*pTab).zName,
                        b"sqlite_stat1\0".as_ptr() as *const ::core::ffi::c_char,
                    )
            {
                sqlite3VdbeAppendP4(
                    v,
                    pTab as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    P4_TABLE,
                );
            }
            if eMode as ::core::ffi::c_int != ONEPASS_OFF {
                sqlite3VdbeChangeP5(v, OPFLAG_AUXDELETE as u16_0);
            }
            if iIdxNoSeek >= 0 as ::core::ffi::c_int && iIdxNoSeek != iDataCur {
                sqlite3VdbeAddOp1(v, OP_Delete, iIdxNoSeek);
            }
            if eMode as ::core::ffi::c_int == ONEPASS_MULTI {
                p5 = (p5 as ::core::ffi::c_int | OPFLAG_SAVEPOSITION) as u8_0;
            }
            sqlite3VdbeChangeP5(v, p5 as u16_0);
        }
        sqlite3FkActions(
            pParse,
            pTab,
            ::core::ptr::null_mut::<ExprList>(),
            iOld,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        );
        if !pTrigger.is_null() {
            sqlite3CodeRowTrigger(
                pParse,
                pTrigger,
                TK_DELETE,
                ::core::ptr::null_mut::<ExprList>(),
                TRIGGER_AFTER,
                pTab,
                iOld,
                onconf as ::core::ffi::c_int,
                iLabel,
            );
        }
        sqlite3VdbeResolveLabel(v, iLabel);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GenerateRowIndexDelete(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut iDataCur: ::core::ffi::c_int,
    mut iIdxCur: ::core::ffi::c_int,
    mut aRegIdx: *mut ::core::ffi::c_int,
    mut iIdxNoSeek: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut r1: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iPartIdxLabel: ::core::ffi::c_int = 0;
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut pPrior: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut pPk: *mut Index = ::core::ptr::null_mut::<Index>();
        v = (*pParse).pVdbe;
        pPk = if (*pTab).tabFlags as ::core::ffi::c_uint
            & TF_WithoutRowid as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
        {
            ::core::ptr::null_mut::<Index>()
        } else {
            sqlite3PrimaryKeyIndex(pTab)
        };
        i = 0 as ::core::ffi::c_int;
        pIdx = (*pTab).pIndex;
        while !pIdx.is_null() {
            if !(!aRegIdx.is_null()
                && *aRegIdx.offset(i as isize) == 0 as ::core::ffi::c_int)
            {
                if !(pIdx == pPk) {
                    if !(iIdxCur + i == iIdxNoSeek) {
                        r1 = sqlite3GenerateIndexKey(
                            pParse,
                            pIdx,
                            iDataCur,
                            0 as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                            &raw mut iPartIdxLabel,
                            pPrior,
                            r1,
                        );
                        sqlite3VdbeAddOp3(
                            v,
                            OP_IdxDelete,
                            iIdxCur + i,
                            r1,
                            if (*pIdx).uniqNotNull() as ::core::ffi::c_int != 0 {
                                (*pIdx).nKeyCol as ::core::ffi::c_int
                            } else {
                                (*pIdx).nColumn as ::core::ffi::c_int
                            },
                        );
                        sqlite3VdbeChangeP5(v, 1 as u16_0);
                        sqlite3ResolvePartIdxLabel(pParse, iPartIdxLabel);
                        pPrior = pIdx;
                    }
                }
            }
            i += 1;
            pIdx = (*pIdx).pNext;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GenerateIndexKey(
    mut pParse: *mut Parse,
    mut pIdx: *mut Index,
    mut iDataCur: ::core::ffi::c_int,
    mut regOut: ::core::ffi::c_int,
    mut prefixOnly: ::core::ffi::c_int,
    mut piPartIdxLabel: *mut ::core::ffi::c_int,
    mut pPrior: *mut Index,
    mut regPrior: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut j: ::core::ffi::c_int = 0;
        let mut regBase: ::core::ffi::c_int = 0;
        let mut nCol: ::core::ffi::c_int = 0;
        if !piPartIdxLabel.is_null() {
            if !(*pIdx).pPartIdxWhere.is_null() {
                *piPartIdxLabel = sqlite3VdbeMakeLabel(pParse);
                (*pParse).iSelfTab = iDataCur + 1 as ::core::ffi::c_int;
                sqlite3ExprIfFalseDup(
                    pParse,
                    (*pIdx).pPartIdxWhere,
                    *piPartIdxLabel,
                    SQLITE_JUMPIFNULL,
                );
                (*pParse).iSelfTab = 0 as ::core::ffi::c_int;
                pPrior = ::core::ptr::null_mut::<Index>();
            } else {
                *piPartIdxLabel = 0 as ::core::ffi::c_int;
            }
        }
        nCol = if prefixOnly != 0 && (*pIdx).uniqNotNull() as ::core::ffi::c_int != 0 {
            (*pIdx).nKeyCol as ::core::ffi::c_int
        } else {
            (*pIdx).nColumn as ::core::ffi::c_int
        };
        regBase = sqlite3GetTempRange(pParse, nCol);
        if !pPrior.is_null()
            && (regBase != regPrior || !(*pPrior).pPartIdxWhere.is_null())
        {
            pPrior = ::core::ptr::null_mut::<Index>();
        }
        j = 0 as ::core::ffi::c_int;
        while j < nCol {
            if !(!pPrior.is_null()
                && *(*pPrior).aiColumn.offset(j as isize) as ::core::ffi::c_int
                    == *(*pIdx).aiColumn.offset(j as isize) as ::core::ffi::c_int
                && *(*pPrior).aiColumn.offset(j as isize) as ::core::ffi::c_int
                    != XN_EXPR)
            {
                sqlite3ExprCodeLoadIndexColumn(pParse, pIdx, iDataCur, j, regBase + j);
                if *(*pIdx).aiColumn.offset(j as isize) as ::core::ffi::c_int
                    >= 0 as ::core::ffi::c_int
                {
                    sqlite3VdbeDeletePriorOpcode(v, OP_RealAffinity as u8_0);
                }
            }
            j += 1;
        }
        if regOut != 0 {
            sqlite3VdbeAddOp3(v, OP_MakeRecord, regBase, nCol, regOut);
        }
        sqlite3ReleaseTempRange(pParse, regBase, nCol);
        return regBase;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ResolvePartIdxLabel(
    mut pParse: *mut Parse,
    mut iLabel: ::core::ffi::c_int,
) {
    unsafe {
        if iLabel != 0 {
            sqlite3VdbeResolveLabel((*pParse).pVdbe, iLabel);
        }
    }
}
pub const TK_DELETE: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const SQLITE_TrustedSchema: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_CountRows: u64_0 = (0x1 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const TF_Readonly: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_Shadow: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TABTYP_VIEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Default: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_IDXTYPE_PRIMARYKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XN_EXPR: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const WHERE_ONEPASS_DESIRED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const WHERE_ONEPASS_MULTIROW: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WHERE_DUPLICATES_OK: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const NC_Subquery: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SF_IncludeHidden: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const SRT_EphemTab: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const OPFLAG_NCHANGE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPFLAG_FORDELETE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPFLAG_SAVEPOSITION: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPFLAG_AUXDELETE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const TRIGGER_BEFORE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TRIGGER_AFTER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ONEPASS_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ONEPASS_SINGLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ONEPASS_MULTI: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
