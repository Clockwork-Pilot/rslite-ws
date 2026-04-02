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
    pub type sqlite3_pcache;
    pub type CheckOnCtx;
    pub type RenameCtx;
    pub type WhereConst;
    pub type WindowRewrite;
    pub type IdxCover;
    pub type RefSrcList;
    pub type CCurHint;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
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
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3WhereExplainOneScan(
        pParse: *mut Parse,
        pTabList: *mut SrcList,
        pLevel: *mut WhereLevel,
        wctrlFlags: u16_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3WhereExplainBloomFilter(
        pParse: *const Parse,
        pWInfo: *const WhereInfo,
        pLevel: *const WhereLevel,
    ) -> ::core::ffi::c_int;
    fn sqlite3WhereAddExplainText(
        pParse: *mut Parse,
        addr: ::core::ffi::c_int,
        pTabList: *mut SrcList,
        pLevel: *mut WhereLevel,
        wctrlFlags: u16_0,
    );
    fn sqlite3WhereCodeOneLoopStart(
        pParse: *mut Parse,
        v: *mut Vdbe,
        pWInfo: *mut WhereInfo,
        iLevel: ::core::ffi::c_int,
        pLevel: *mut WhereLevel,
        notReady: Bitmask,
    ) -> Bitmask;
    fn sqlite3WhereRightJoinLoop(
        pWInfo: *mut WhereInfo,
        iLevel: ::core::ffi::c_int,
        pLevel: *mut WhereLevel,
    );
    fn sqlite3WhereClauseInit(_: *mut WhereClause, _: *mut WhereInfo);
    fn sqlite3WhereClauseClear(_: *mut WhereClause);
    fn sqlite3WhereSplit(_: *mut WhereClause, _: *mut Expr, _: u8_0);
    fn sqlite3WhereAddLimit(_: *mut WhereClause, _: *mut Select);
    fn sqlite3WhereExprUsage(_: *mut WhereMaskSet, _: *mut Expr) -> Bitmask;
    fn sqlite3WhereExprListUsage(_: *mut WhereMaskSet, _: *mut ExprList) -> Bitmask;
    fn sqlite3WhereExprAnalyze(_: *mut SrcList, _: *mut WhereClause);
    fn sqlite3WhereTabFuncArgs(_: *mut Parse, _: *mut SrcItem, _: *mut WhereClause);
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
    fn sqlite3VdbeAppendP4(
        _: *mut Vdbe,
        pP4: *mut ::core::ffi::c_void,
        p4type: ::core::ffi::c_int,
    );
    fn sqlite3VdbeSetP4KeyInfo(_: *mut Parse, _: *mut Index);
    fn sqlite3VdbeGetOp(_: *mut Vdbe, _: ::core::ffi::c_int) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3VdbeResolveLabel(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeCurrentAddr(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3WalkExpr(_: *mut Walker, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3WalkSelect(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3SelectWalkNoop(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3SelectWalkFail(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3MisuseError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRaw(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbNNFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3ProgressCheck(_: *mut Parse);
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3GetTempReg(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempReg(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3GetTempRange(_: *mut Parse, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempRange(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprAnd(_: *mut Parse, _: *mut Expr, _: *mut Expr) -> *mut Expr;
    fn sqlite3ExprDelete(_: *mut sqlite3, _: *mut Expr);
    fn sqlite3ColumnExpr(_: *mut Table, _: *mut Column) -> *mut Expr;
    fn sqlite3ColumnColl(_: *mut Column) -> *const ::core::ffi::c_char;
    fn sqlite3PrimaryKeyIndex(_: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(
        _: *mut Index,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3StorageColumnToTable(_: *mut Table, _: i16_0) -> i16_0;
    fn sqlite3FaultSim(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3AllocateIndexObject(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut Index;
    fn sqlite3OpenTable(
        _: *mut Parse,
        iCur: ::core::ffi::c_int,
        iDb: ::core::ffi::c_int,
        _: *mut Table,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprCodeLoadIndexColumn(
        _: *mut Parse,
        _: *mut Index,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
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
    fn sqlite3ExprCompareSkip(
        _: *mut Expr,
        _: *mut Expr,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprImpliesExpr(
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
    fn sqlite3CodeVerifySchema(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3ExprIsConstant(_: *mut Parse, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3ExprIsSingleTableConstraint(
        _: *mut Expr,
        _: *const SrcList,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprIsInteger(
        _: *const Expr,
        _: *mut ::core::ffi::c_int,
        _: *mut Parse,
    ) -> ::core::ffi::c_int;
    fn sqlite3GenerateIndexKey(
        _: *mut Parse,
        _: *mut Index,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut Index,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BeginWriteOperation(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprDup(
        _: *mut sqlite3,
        _: *const Expr,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3LogEst(_: u64_0) -> LogEst;
    fn sqlite3LogEstAdd(_: LogEst, _: LogEst) -> LogEst;
    fn sqlite3LogEstFromDouble(_: ::core::ffi::c_double) -> LogEst;
    fn sqlite3LogEstToInt(_: LogEst) -> u64_0;
    fn sqlite3IndexAffinityStr(
        _: *mut sqlite3,
        _: *mut Index,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3CompareAffinity(
        pExpr: *const Expr,
        aff2: ::core::ffi::c_char,
    ) -> ::core::ffi::c_char;
    fn sqlite3IndexAffinityOk(
        pExpr: *const Expr,
        idx_affinity: ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3TableColumnAffinity(
        _: *const Table,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_char;
    fn sqlite3ExprAffinity(pExpr: *const Expr) -> ::core::ffi::c_char;
    fn sqlite3ErrStr(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3IsBinary(_: *const CollSeq) -> ::core::ffi::c_int;
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
    static sqlite3StrBINARY: [::core::ffi::c_char; 0];
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
    fn sqlite3KeyInfoAlloc(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut KeyInfo;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3TableLock(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: Pgno,
        _: u8_0,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3GetVTable(_: *mut sqlite3, _: *mut Table) -> *mut VTable;
    fn sqlite3ParserAddCleanup(
        _: *mut Parse,
        _: Option<unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> ()>,
        _: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3ExprCompareCollSeq(_: *mut Parse, _: *const Expr) -> *mut CollSeq;
    fn sqlite3BinaryCompareCollSeq(
        _: *mut Parse,
        _: *const Expr,
        _: *const Expr,
    ) -> *mut CollSeq;
    fn sqlite3ExprVectorSize(pExpr: *const Expr) -> ::core::ffi::c_int;
    fn sqlite3ExprIsVector(pExpr: *const Expr) -> ::core::ffi::c_int;
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
    pub x: C2Rust_Unnamed_28,
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
    pub u: C2Rust_Unnamed_26,
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
    pub u: C2Rust_Unnamed_23,
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
pub union C2Rust_Unnamed_23 {
    pub btree: C2Rust_Unnamed_25,
    pub vtab: C2Rust_Unnamed_24

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
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
pub type i8_0 = int8_t;
pub type int8_t = __int8_t;
pub type __int8_t = i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub nEq: u16_0,
    pub nBtm: u16_0,
    pub nTop: u16_0,
    pub nDistinctCol: u16_0,
    pub pIndex: *mut Index,
    pub pOrderBy: *mut ExprList

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_26 {
    pub in_0: C2Rust_Unnamed_27,
    pub pCoveringIdx: *mut Index

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
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
pub struct WhereOrInfo {
    pub wc: WhereClause,
    pub indexable: Bitmask

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub leftColumn: ::core::ffi::c_int,
    pub iField: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HiddenIndexInfo {
    pub pWC: *mut WhereClause,
    pub pParse: *mut Parse,
    pub eDistinct: ::core::ffi::c_int,
    pub mIn: u32_0,
    pub mHandleIn: u32_0,
    pub aRhs: [*mut sqlite3_value; 0]

}
pub type intptr_t = isize;
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
    pub u: C2Rust_Unnamed_29

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_29 {
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
pub struct CoveringIndexCheck {
    pub pIdx: *mut Index,
    pub iTabCur: ::core::ffi::c_int,
    pub bExpr: u8_0,
    pub bUnidx: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameContext {
    pub pParse: *mut Parse,
    pub pSrcList: *mut SrcList,
    pub uNC: C2Rust_Unnamed_30,
    pub pNext: *mut NameContext,
    pub nRef: ::core::ffi::c_int,
    pub nNcErr: ::core::ffi::c_int,
    pub ncFlags: ::core::ffi::c_int,
    pub nNestedSelect: u32_0,
    pub pWinSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_30 {
    pub pEList: *mut ExprList,
    pub pAggInfo: *mut AggInfo,
    pub pUpsert: *mut Upsert,
    pub iBaseReg: ::core::ffi::c_int

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WherePath {
    pub maskLoop: Bitmask,
    pub revLoop: Bitmask,
    pub nRow: LogEst,
    pub rCost: LogEst,
    pub rUnsort: LogEst,
    pub isOrdered: i8_0,
    pub aLoop: *mut *mut WhereLoop

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereScan {
    pub pOrigWC: *mut WhereClause,
    pub pWC: *mut WhereClause,
    pub zCollName: *const ::core::ffi::c_char,
    pub pIdxExpr: *mut Expr,
    pub k: ::core::ffi::c_int,
    pub opMask: u32_0,
    pub idxaff: ::core::ffi::c_char,
    pub iEquiv: ::core::ffi::c_uchar,
    pub nEquiv: ::core::ffi::c_uchar,
    pub aiCur: [::core::ffi::c_int; 11],
    pub aiColumn: [i16_0; 11]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereLoopBuilder {
    pub pWInfo: *mut WhereInfo,
    pub pWC: *mut WhereClause,
    pub pNew: *mut WhereLoop,
    pub pOrSet: *mut WhereOrSet,
    pub bldFlags1: ::core::ffi::c_uchar,
    pub bldFlags2: ::core::ffi::c_uchar,
    pub iPlanLimit: ::core::ffi::c_uint

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereOrSet {
    pub n: u16_0,
    pub a: [WhereOrCost; 3]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereOrCost {
    pub prereq: Bitmask,
    pub rRun: LogEst,
    pub nOut: LogEst

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_INTERNAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_WARNING: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_WARNING_AUTOINDEX: ::core::ffi::c_int = SQLITE_WARNING
    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_INDEX_SCAN_UNIQUE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_INDEX_SCAN_HEX: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_ISNULL: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_IS: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LIMIT: ::core::ffi::c_int = 73 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_OFFSET: ::core::ffi::c_int = 74 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_AUTOINDEX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const OP_Goto: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OP_Gosub: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const OP_InitCoroutine: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const OP_Yield: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const OP_Once: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const OP_IfNullRow: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const OP_SeekLT: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const OP_SeekGT: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const OP_IfNotOpen: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const OP_IfNoHope: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const OP_IfEmpty: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const OP_Prev: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const OP_IfPos: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const OP_DecrJumpZero: ::core::ffi::c_int = 62 as ::core::ffi::c_int;
pub const OP_Return: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const OP_Blob: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
pub const OP_Copy: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_ReopenIdx: ::core::ffi::c_int = 102 as ::core::ffi::c_int;
pub const OP_OpenRead: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const OP_OpenWrite: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const OP_OpenAutoindex: ::core::ffi::c_int = 117 as ::core::ffi::c_int;
pub const OP_OpenEphemeral: ::core::ffi::c_int = 119 as ::core::ffi::c_int;
pub const OP_Sequence: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const OP_Rowid: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const OP_NullRow: ::core::ffi::c_int = 137 as ::core::ffi::c_int;
pub const OP_IdxInsert: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const OP_IdxRowid: ::core::ffi::c_int = 143 as ::core::ffi::c_int;
pub const OP_VOpen: ::core::ffi::c_int = 174 as ::core::ffi::c_int;
pub const OP_FilterAdd: ::core::ffi::c_int = 184 as ::core::ffi::c_int;
pub const OP_Noop: ::core::ffi::c_int = 188 as ::core::ffi::c_int;
pub const WHERE_LOOP_XFER_SZ: ::core::ffi::c_ulong = 56 as ::core::ffi::c_ulong;
pub const N_OR_COST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const TERM_VIRTUAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const TERM_CODED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const TERM_OK: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const TERM_VNULL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TERM_LIKEOPT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const TERM_HEURTRUTH: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const TERM_HIGHTRUTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TERM_SLICE: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_BLDF1_INDEXED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_BLDF1_UNIQUE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_QUERY_PLANNER_LIMIT: ::core::ffi::c_int = 20000 as ::core::ffi::c_int;
pub const SQLITE_QUERY_PLANNER_LIMIT_INCR: ::core::ffi::c_int = 1000
    as ::core::ffi::c_int;
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
pub const WO_ALL: ::core::ffi::c_int = 0x3fff as ::core::ffi::c_int;
pub const WHERE_COLUMN_EQ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const WHERE_COLUMN_RANGE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WHERE_COLUMN_IN: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const WHERE_COLUMN_NULL: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WHERE_CONSTRAINT: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;
pub const WHERE_TOP_LIMIT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const WHERE_BTM_LIMIT: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const WHERE_IDX_ONLY: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const WHERE_IPK: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const WHERE_INDEXED: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const WHERE_VIRTUALTABLE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const WHERE_IN_ABLE: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const WHERE_ONEROW: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const WHERE_MULTI_OR: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const WHERE_AUTO_INDEX: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const WHERE_SKIPSCAN: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const WHERE_UNQ_WANTED: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const WHERE_PARTIALIDX: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const WHERE_IN_EARLYOUT: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const WHERE_BIGNULL_SORT: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const WHERE_IN_SEEKSCAN: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const WHERE_TRANSCONS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const WHERE_BLOOMFILTER: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
pub const WHERE_SELFCULL: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
pub const WHERE_COROUTINE: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const WHERE_EXPRIDX: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const SQLITE_BIG_DBL: ::core::ffi::c_double = 1e99f64;
pub const LOGEST_MIN: ::core::ffi::c_int = -32768 as ::core::ffi::c_int;
pub const BMS: ::core::ffi::c_int = (::core::mem::size_of::<Bitmask>() as usize)
    .wrapping_mul(8 as usize) as ::core::ffi::c_int;
pub const ALLBITS: Bitmask = -1 as ::core::ffi::c_int as Bitmask;
pub const TOPBIT: Bitmask = (1 as ::core::ffi::c_int as Bitmask)
    << BMS - 1 as ::core::ffi::c_int;
pub const P4_INT32: ::core::ffi::c_int = -3 as ::core::ffi::c_int;
pub const P4_KEYINFO: ::core::ffi::c_int = -8 as ::core::ffi::c_int;
pub const P4_VTAB: ::core::ffi::c_int = -11 as ::core::ffi::c_int;
pub const TK_AND: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const TK_IS: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const TK_EQ: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const TK_GT: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
pub const TK_LE: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const TK_LT: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
pub const TK_GE: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
pub const TK_COLLATE: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const TK_COLUMN: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
pub const TK_AGG_COLUMN: ::core::ffi::c_int = 170 as ::core::ffi::c_int;
pub const TK_FUNCTION: ::core::ffi::c_int = 172 as ::core::ffi::c_int;
pub const SQLITE_ReverseOrder: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const SQLITE_AutoIndex: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const COLFLAG_PRIMKEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const COLFLAG_VIRTUAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_AFF_BLOB: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const SQLITE_AFF_TEXT: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const TF_HasStat1: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const TF_HasGenerated: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_MaybeReanalyze: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const TF_Ephemeral: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TABTYP_VIEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_None: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OE_Replace: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_DESC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_BIGNULL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_IDXTYPE_PRIMARYKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_IDXTYPE_IPK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const XN_ROWID: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const XN_EXPR: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const EP_xIsSelect: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const JT_CROSS: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const JT_LEFT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const JT_RIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const JT_OUTER: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const JT_LTORJ: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const WHERE_ORDERBY_MIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const WHERE_ORDERBY_MAX: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WHERE_ONEPASS_DESIRED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const WHERE_ONEPASS_MULTIROW: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WHERE_DUPLICATES_OK: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const WHERE_OR_SUBCLAUSE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const WHERE_GROUPBY: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const WHERE_DISTINCTBY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const WHERE_WANT_DISTINCT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const WHERE_SORTBYGROUP: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const WHERE_AGG_DISTINCT: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const WHERE_ORDERBY_LIMIT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const WHERE_RIGHT_JOIN: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const WHERE_KEEP_ALL_JOINS: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const WHERE_USE_LIMIT: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const WHERE_DISTINCT_NOOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WHERE_DISTINCT_UNIQUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WHERE_DISTINCT_ORDERED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WHERE_DISTINCT_UNORDERED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SF_MultiValue: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const OPFLAG_USESEEKRESULT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const OPFLAG_SEEKEQ: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPFLAG_FORDELETE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WRC_Continue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WRC_Prune: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WRC_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const M10d_Yes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const ONEPASS_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ONEPASS_SINGLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ONEPASS_MULTI: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereOutputRowCount(
    mut pWInfo: *mut WhereInfo,
) -> LogEst {
    unsafe {
        return (*pWInfo).nRowOut;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereIsDistinct(
    mut pWInfo: *mut WhereInfo,
) -> ::core::ffi::c_int {
    unsafe {
        return (*pWInfo).eDistinct as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereIsOrdered(
    mut pWInfo: *mut WhereInfo,
) -> ::core::ffi::c_int {
    unsafe {
        return if ((*pWInfo).nOBSat as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            (*pWInfo).nOBSat as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereOrderByLimitOptLabel(
    mut pWInfo: *mut WhereInfo,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pInner: *mut WhereLevel = ::core::ptr::null_mut::<WhereLevel>();
        if (*pWInfo).bOrderedInnerLoop() == 0 {
            return (*pWInfo).iContinue;
        }
        pInner = (&raw mut (*pWInfo).a as *mut WhereLevel)
            .offset(
                ((*pWInfo).nLevel as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as isize,
            ) as *mut WhereLevel;
        return if !(*pInner).pRJ.is_null() {
            (*pWInfo).iContinue
        } else {
            (*pInner).addrNxt
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereMinMaxOptEarlyOut(
    mut v: *mut Vdbe,
    mut pWInfo: *mut WhereInfo,
) {
    unsafe {
        let mut pInner: *mut WhereLevel = ::core::ptr::null_mut::<WhereLevel>();
        let mut i: ::core::ffi::c_int = 0;
        if (*pWInfo).bOrderedInnerLoop() == 0 {
            return;
        }
        if (*pWInfo).nOBSat as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return;
        }
        i = (*pWInfo).nLevel as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            pInner = (&raw mut (*pWInfo).a as *mut WhereLevel).offset(i as isize)
                as *mut WhereLevel;
            if (*(*pInner).pWLoop).wsFlags & WHERE_COLUMN_IN as u32_0 != 0 as u32_0 {
                sqlite3VdbeGoto(v, (*pInner).addrNxt);
                return;
            }
            i -= 1;
        }
        sqlite3VdbeGoto(v, (*pWInfo).iBreak);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereContinueLabel(
    mut pWInfo: *mut WhereInfo,
) -> ::core::ffi::c_int {
    unsafe {
        return (*pWInfo).iContinue;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereBreakLabel(
    mut pWInfo: *mut WhereInfo,
) -> ::core::ffi::c_int {
    unsafe {
        return (*pWInfo).iBreak;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereOkOnePass(
    mut pWInfo: *mut WhereInfo,
    mut aiCur: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        memcpy(
            aiCur as *mut ::core::ffi::c_void,
            &raw mut (*pWInfo).aiCurOnePass as *mut ::core::ffi::c_int
                as *const ::core::ffi::c_void,
            (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
                .wrapping_mul(2 as size_t),
        );
        return (*pWInfo).eOnePass as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereUsesDeferredSeek(
    mut pWInfo: *mut WhereInfo,
) -> ::core::ffi::c_int {
    unsafe {
        return (*pWInfo).bDeferredSeek() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn whereOrMove(mut pDest: *mut WhereOrSet, mut pSrc: *mut WhereOrSet) {
    unsafe {
        (*pDest).n = (*pSrc).n;
        memcpy(
            &raw mut (*pDest).a as *mut WhereOrCost as *mut ::core::ffi::c_void,
            &raw mut (*pSrc).a as *mut WhereOrCost as *const ::core::ffi::c_void,
            ((*pDest).n as size_t)
                .wrapping_mul(::core::mem::size_of::<WhereOrCost>() as size_t),
        );
    }
}
unsafe extern "C" fn whereOrInsert(
    mut pSet: *mut WhereOrSet,
    mut prereq: Bitmask,
    mut rRun: LogEst,
    mut nOut: LogEst,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut i: u16_0 = 0;
        let mut p: *mut WhereOrCost = ::core::ptr::null_mut::<WhereOrCost>();
        i = (*pSet).n;
        p = &raw mut (*pSet).a as *mut WhereOrCost;
        loop {
            if !(i as ::core::ffi::c_int > 0 as ::core::ffi::c_int) {
                c2rust_current_block = 2473556513754201174;
                break;
            }
            if rRun as ::core::ffi::c_int <= (*p).rRun as ::core::ffi::c_int
                && prereq & (*p).prereq == prereq
            {
                c2rust_current_block = 311719474393970613;
                break;
            }
            if (*p).rRun as ::core::ffi::c_int <= rRun as ::core::ffi::c_int
                && (*p).prereq & prereq == (*p).prereq
            {
                return 0 as ::core::ffi::c_int;
            }
            i = i.wrapping_sub(1);
            p = p.offset(1);
        }
        match c2rust_current_block {
            2473556513754201174 => {
                if ((*pSet).n as ::core::ffi::c_int) < N_OR_COST {
                    let c2rust_fresh13 = (*pSet).n;
                    (*pSet).n = (*pSet).n.wrapping_add(1);
                    p = (&raw mut (*pSet).a as *mut WhereOrCost)
                        .offset(c2rust_fresh13 as isize) as *mut WhereOrCost;
                    (*p).nOut = nOut;
                } else {
                    p = &raw mut (*pSet).a as *mut WhereOrCost;
                    i = 1 as u16_0;
                    while (i as ::core::ffi::c_int) < (*pSet).n as ::core::ffi::c_int {
                        if (*p).rRun as ::core::ffi::c_int
                            > (*pSet).a[i as usize].rRun as ::core::ffi::c_int
                        {
                            p = (&raw mut (*pSet).a as *mut WhereOrCost)
                                .offset(i as ::core::ffi::c_int as isize);
                        }
                        i = i.wrapping_add(1);
                    }
                    if (*p).rRun as ::core::ffi::c_int <= rRun as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
            _ => {}
        }
        (*p).prereq = prereq;
        (*p).rRun = rRun;
        if (*p).nOut as ::core::ffi::c_int > nOut as ::core::ffi::c_int {
            (*p).nOut = nOut;
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereGetMask(
    mut pMaskSet: *mut WhereMaskSet,
    mut iCursor: ::core::ffi::c_int,
) -> Bitmask {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if (*pMaskSet).ix[0 as ::core::ffi::c_int as usize] == iCursor {
            return 1 as Bitmask;
        }
        i = 1 as ::core::ffi::c_int;
        while i < (*pMaskSet).n {
            if (*pMaskSet).ix[i as usize] == iCursor {
                return (1 as ::core::ffi::c_int as Bitmask) << i;
            }
            i += 1;
        }
        return 0 as Bitmask;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereMalloc(
    mut pWInfo: *mut WhereInfo,
    mut nByte: u64_0,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pBlock: *mut WhereMemBlock = ::core::ptr::null_mut::<WhereMemBlock>();
        pBlock = sqlite3DbMallocRawNN(
            (*(*pWInfo).pParse).db,
            nByte.wrapping_add(::core::mem::size_of::<WhereMemBlock>() as u64_0),
        ) as *mut WhereMemBlock;
        if !pBlock.is_null() {
            (*pBlock).pNext = (*pWInfo).pMemToFree;
            (*pBlock).sz = nByte;
            (*pWInfo).pMemToFree = pBlock;
            pBlock = pBlock.offset(1);
        }
        return pBlock as *mut ::core::ffi::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereRealloc(
    mut pWInfo: *mut WhereInfo,
    mut pOld: *mut ::core::ffi::c_void,
    mut nByte: u64_0,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pNew: *mut ::core::ffi::c_void = sqlite3WhereMalloc(pWInfo, nByte);
        if !pNew.is_null() && !pOld.is_null() {
            let mut pOldBlk: *mut WhereMemBlock = pOld as *mut WhereMemBlock;
            pOldBlk = pOldBlk.offset(-1);
            memcpy(pNew, pOld, (*pOldBlk).sz as size_t);
        }
        return pNew;
    }
}
unsafe extern "C" fn createMask(
    mut pMaskSet: *mut WhereMaskSet,
    mut iCursor: ::core::ffi::c_int,
) {
    unsafe {
        let c2rust_fresh26 = (*pMaskSet).n;
        (*pMaskSet).n = (*pMaskSet).n + 1;
        (*pMaskSet).ix[c2rust_fresh26 as usize] = iCursor;
    }
}
unsafe extern "C" fn whereRightSubexprIsColumn(mut p: *mut Expr) -> *mut Expr {
    unsafe {
        p = sqlite3ExprSkipCollateAndLikely((*p).pRight);
        if !p.is_null() && (*p).op as ::core::ffi::c_int == TK_COLUMN
            && !((*p).flags & 0x20 as ::core::ffi::c_int as u32_0 != 0 as u32_0)
        {
            return p;
        }
        return ::core::ptr::null_mut::<Expr>();
    }
}
#[inline(never)]
unsafe extern "C" fn indexInAffinityOk(
    mut pParse: *mut Parse,
    mut pTerm: *mut WhereTerm,
    mut idxaff: u8_0,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut pX: *mut Expr = (*pTerm).pExpr;
        let mut inexpr: Expr = Expr {
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
        if sqlite3ExprIsVector((*pX).pLeft) != 0 {
            let mut iField: ::core::ffi::c_int = (*pTerm).u.x.iField
                - 1 as ::core::ffi::c_int;
            inexpr.flags = 0 as u32_0;
            inexpr.op = TK_EQ as u8_0;
            inexpr.pLeft = (*(&raw mut (*(*(*pX).pLeft).x.pList).a as *mut ExprList_item)
                .offset(iField as isize))
                .pExpr;
            inexpr.pRight = (*(&raw mut (*(*(*pX).x.pSelect).pEList).a
                as *mut ExprList_item)
                .offset(iField as isize))
                .pExpr;
            pX = &raw mut inexpr;
        }
        if sqlite3IndexAffinityOk(pX, idxaff as ::core::ffi::c_char) != 0 {
            let mut pRet: *mut CollSeq = sqlite3ExprCompareCollSeq(pParse, pX);
            return if !pRet.is_null() {
                (*pRet).zName as *const ::core::ffi::c_char
            } else {
                &raw const sqlite3StrBINARY as *const ::core::ffi::c_char
            };
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn whereScanNext(mut pScan: *mut WhereScan) -> *mut WhereTerm {
    unsafe {
        let mut iCur: ::core::ffi::c_int = 0;
        let mut iColumn: i16_0 = 0;
        let mut pX: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pWC: *mut WhereClause = ::core::ptr::null_mut::<WhereClause>();
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut k: ::core::ffi::c_int = (*pScan).k;
        pWC = (*pScan).pWC;
        loop {
            iColumn = (*pScan)
                .aiColumn[((*pScan).iEquiv as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize];
            iCur = (*pScan)
                .aiCur[((*pScan).iEquiv as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as usize];
            loop {
                let mut c2rust_current_block_25: u64;
                pTerm = (*pWC).a.offset(k as isize);
                while k < (*pWC).nTerm {
                    if (*pTerm).leftCursor == iCur
                        && (*pTerm).u.x.leftColumn == iColumn as ::core::ffi::c_int
                        && (iColumn as ::core::ffi::c_int != XN_EXPR
                            || sqlite3ExprCompareSkip(
                                (*(*pTerm).pExpr).pLeft,
                                (*pScan).pIdxExpr,
                                iCur,
                            ) == 0 as ::core::ffi::c_int)
                        && ((*pScan).iEquiv as ::core::ffi::c_int
                            <= 1 as ::core::ffi::c_int
                            || !((*(*pTerm).pExpr).flags
                                & 0x1 as ::core::ffi::c_int as u32_0 != 0 as u32_0))
                    {
                        if (*pTerm).eOperator as ::core::ffi::c_int & WO_EQUIV
                            != 0 as ::core::ffi::c_int
                            && ((*pScan).nEquiv as ::core::ffi::c_int)
                                < (::core::mem::size_of::<[::core::ffi::c_int; 11]>()
                                    as usize)
                                    .wrapping_div(
                                        ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                    ) as ::core::ffi::c_int
                            && {
                                pX = whereRightSubexprIsColumn((*pTerm).pExpr);
                                !pX.is_null()
                            }
                        {
                            let mut j: ::core::ffi::c_int = 0;
                            j = 0 as ::core::ffi::c_int;
                            while j < (*pScan).nEquiv as ::core::ffi::c_int {
                                if (*pScan).aiCur[j as usize] == (*pX).iTable
                                    && (*pScan).aiColumn[j as usize] as ::core::ffi::c_int
                                        == (*pX).iColumn as ::core::ffi::c_int
                                {
                                    break;
                                }
                                j += 1;
                            }
                            if j == (*pScan).nEquiv as ::core::ffi::c_int {
                                (*pScan).aiCur[j as usize] = (*pX).iTable;
                                (*pScan).aiColumn[j as usize] = (*pX).iColumn as i16_0;
                                (*pScan).nEquiv = (*pScan).nEquiv.wrapping_add(1);
                            }
                        }
                        if (*pTerm).eOperator as u32_0 & (*pScan).opMask != 0 as u32_0 {
                            if !(*pScan).zCollName.is_null()
                                && (*pTerm).eOperator as ::core::ffi::c_int & WO_ISNULL
                                    == 0 as ::core::ffi::c_int
                            {
                                let mut zCollName: *const ::core::ffi::c_char = ::core::ptr::null::<
                                    ::core::ffi::c_char,
                                >();
                                let mut pParse: *mut Parse = (*(*pWC).pWInfo).pParse;
                                pX = (*pTerm).pExpr;
                                if (*pTerm).eOperator as ::core::ffi::c_int & WO_IN != 0 {
                                    zCollName = indexInAffinityOk(
                                        pParse,
                                        pTerm,
                                        (*pScan).idxaff as u8_0,
                                    );
                                    if zCollName.is_null() {
                                        c2rust_current_block_25 = 14523784380283086299;
                                    } else {
                                        c2rust_current_block_25 = 17788412896529399552;
                                    }
                                } else {
                                    let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<
                                        CollSeq,
                                    >();
                                    if sqlite3IndexAffinityOk(pX, (*pScan).idxaff) == 0 {
                                        c2rust_current_block_25 = 14523784380283086299;
                                    } else {
                                        pColl = sqlite3ExprCompareCollSeq(pParse, pX);
                                        zCollName = if !pColl.is_null() {
                                            (*pColl).zName as *const ::core::ffi::c_char
                                        } else {
                                            &raw const sqlite3StrBINARY as *const ::core::ffi::c_char
                                        };
                                        c2rust_current_block_25 = 17788412896529399552;
                                    }
                                }
                                match c2rust_current_block_25 {
                                    14523784380283086299 => {}
                                    _ => {
                                        if sqlite3StrICmp(zCollName, (*pScan).zCollName) != 0 {
                                            c2rust_current_block_25 = 14523784380283086299;
                                        } else {
                                            c2rust_current_block_25 = 18377268871191777778;
                                        }
                                    }
                                }
                            } else {
                                c2rust_current_block_25 = 18377268871191777778;
                            }
                            match c2rust_current_block_25 {
                                14523784380283086299 => {}
                                _ => {
                                    if !((*pTerm).eOperator as ::core::ffi::c_int
                                        & (WO_EQ | WO_IS) != 0 as ::core::ffi::c_int
                                        && {
                                            pX = (*(*pTerm).pExpr).pRight;
                                            !pX.is_null() as ::core::ffi::c_int != 0
                                        } && (*pX).op as ::core::ffi::c_int == TK_COLUMN
                                        && (*pX).iTable
                                            == (*pScan).aiCur[0 as ::core::ffi::c_int as usize]
                                        && (*pX).iColumn as ::core::ffi::c_int
                                            == (*pScan).aiColumn[0 as ::core::ffi::c_int as usize]
                                                as ::core::ffi::c_int)
                                    {
                                        (*pScan).pWC = pWC;
                                        (*pScan).k = k + 1 as ::core::ffi::c_int;
                                        return pTerm;
                                    }
                                }
                            }
                        }
                    }
                    k += 1;
                    pTerm = pTerm.offset(1);
                }
                pWC = (*pWC).pOuter;
                k = 0 as ::core::ffi::c_int;
                if pWC.is_null() {
                    break;
                }
            }
            if (*pScan).iEquiv as ::core::ffi::c_int
                >= (*pScan).nEquiv as ::core::ffi::c_int
            {
                break;
            }
            pWC = (*pScan).pOrigWC;
            k = 0 as ::core::ffi::c_int;
            (*pScan).iEquiv = (*pScan).iEquiv.wrapping_add(1);
        }
        return ::core::ptr::null_mut::<WhereTerm>();
    }
}
#[inline(never)]
unsafe extern "C" fn whereScanInitIndexExpr(
    mut pScan: *mut WhereScan,
) -> *mut WhereTerm {
    unsafe {
        (*pScan).idxaff = sqlite3ExprAffinity((*pScan).pIdxExpr);
        return whereScanNext(pScan);
    }
}
unsafe extern "C" fn whereScanInit(
    mut pScan: *mut WhereScan,
    mut pWC: *mut WhereClause,
    mut iCur: ::core::ffi::c_int,
    mut iColumn: ::core::ffi::c_int,
    mut opMask: u32_0,
    mut pIdx: *mut Index,
) -> *mut WhereTerm {
    unsafe {
        (*pScan).pOrigWC = pWC;
        (*pScan).pWC = pWC;
        (*pScan).pIdxExpr = ::core::ptr::null_mut::<Expr>();
        (*pScan).idxaff = 0 as ::core::ffi::c_char;
        (*pScan).zCollName = ::core::ptr::null::<::core::ffi::c_char>();
        (*pScan).opMask = opMask;
        (*pScan).k = 0 as ::core::ffi::c_int;
        (*pScan).aiCur[0 as ::core::ffi::c_int as usize] = iCur;
        (*pScan).nEquiv = 1 as ::core::ffi::c_uchar;
        (*pScan).iEquiv = 1 as ::core::ffi::c_uchar;
        if !pIdx.is_null() {
            let mut j: ::core::ffi::c_int = iColumn;
            iColumn = *(*pIdx).aiColumn.offset(j as isize) as ::core::ffi::c_int;
            if iColumn == (*(*pIdx).pTable).iPKey as ::core::ffi::c_int {
                iColumn = XN_ROWID;
            } else if iColumn >= 0 as ::core::ffi::c_int {
                (*pScan).idxaff = (*(*(*pIdx).pTable).aCol.offset(iColumn as isize))
                    .affinity;
                (*pScan).zCollName = *(*pIdx).azColl.offset(j as isize);
            } else if iColumn == XN_EXPR {
                (*pScan).pIdxExpr = (*(&raw mut (*(*pIdx).aColExpr).a
                    as *mut ExprList_item)
                    .offset(j as isize))
                    .pExpr;
                (*pScan).zCollName = *(*pIdx).azColl.offset(j as isize);
                (*pScan).aiColumn[0 as ::core::ffi::c_int as usize] = XN_EXPR as i16_0;
                return whereScanInitIndexExpr(pScan);
            }
        } else if iColumn == XN_EXPR {
            return ::core::ptr::null_mut::<WhereTerm>()
        }
        (*pScan).aiColumn[0 as ::core::ffi::c_int as usize] = iColumn as i16_0;
        return whereScanNext(pScan);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereFindTerm(
    mut pWC: *mut WhereClause,
    mut iCur: ::core::ffi::c_int,
    mut iColumn: ::core::ffi::c_int,
    mut notReady: Bitmask,
    mut op: u32_0,
    mut pIdx: *mut Index,
) -> *mut WhereTerm {
    unsafe {
        let mut pResult: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut p: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut scan: WhereScan = WhereScan {
            pOrigWC: ::core::ptr::null_mut::<WhereClause>(),
            pWC: ::core::ptr::null_mut::<WhereClause>(),
            zCollName: ::core::ptr::null::<::core::ffi::c_char>(),
            pIdxExpr: ::core::ptr::null_mut::<Expr>(),
            k: 0,
            opMask: 0,
            idxaff: 0,
            iEquiv: 0,
            nEquiv: 0,
            aiCur: [0; 11],
            aiColumn: [0; 11]

        };
        p = whereScanInit(&raw mut scan, pWC, iCur, iColumn, op, pIdx);
        op = (op as ::core::ffi::c_uint & (WO_EQ | WO_IS) as ::core::ffi::c_uint)
            as u32_0;
        while !p.is_null() {
            if (*p).prereqRight & notReady == 0 as ::core::ffi::c_ulonglong {
                if (*p).prereqRight == 0 as ::core::ffi::c_ulonglong
                    && (*p).eOperator as u32_0 & op != 0 as u32_0
                {
                    return p;
                }
                if pResult.is_null() {
                    pResult = p;
                }
            }
            p = whereScanNext(&raw mut scan);
        }
        return pResult;
    }
}
unsafe extern "C" fn findIndexCol(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut iBase: ::core::ffi::c_int,
    mut pIdx: *mut Index,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut zColl: *const ::core::ffi::c_char = *(*pIdx)
            .azColl
            .offset(iCol as isize);
        i = 0 as ::core::ffi::c_int;
        while i < (*pList).nExpr {
            let mut p: *mut Expr = sqlite3ExprSkipCollateAndLikely(
                (*(&raw mut (*pList).a as *mut ExprList_item).offset(i as isize)).pExpr,
            );
            if !p.is_null()
                && ((*p).op as ::core::ffi::c_int == TK_COLUMN
                    || (*p).op as ::core::ffi::c_int == TK_AGG_COLUMN)
                && (*p).iColumn as ::core::ffi::c_int
                    == *(*pIdx).aiColumn.offset(iCol as isize) as ::core::ffi::c_int
                && (*p).iTable == iBase
            {
                let mut pColl: *mut CollSeq = sqlite3ExprNNCollSeq(
                    pParse,
                    (*(&raw mut (*pList).a as *mut ExprList_item).offset(i as isize))
                        .pExpr,
                );
                if 0 as ::core::ffi::c_int == sqlite3StrICmp((*pColl).zName, zColl) {
                    return i;
                }
            }
            i += 1;
        }
        return -1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn indexColumnNotNull(
    mut pIdx: *mut Index,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut j: ::core::ffi::c_int = 0;
        j = *(*pIdx).aiColumn.offset(iCol as isize) as ::core::ffi::c_int;
        if j >= 0 as ::core::ffi::c_int {
            return (*(*(*pIdx).pTable).aCol.offset(j as isize)).notNull()
                as ::core::ffi::c_int
        } else if j == -1 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int
        } else {
            return 0 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn isDistinctRedundant(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pWC: *mut WhereClause,
    mut pDistinct: *mut ExprList,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut i: ::core::ffi::c_int = 0;
        let mut iBase: ::core::ffi::c_int = 0;
        if (*pTabList).nSrc != 1 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        iBase = (*(&raw mut (*pTabList).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .iCursor;
        pTab = (*(&raw mut (*pTabList).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .pSTab;
        i = 0 as ::core::ffi::c_int;
        while i < (*pDistinct).nExpr {
            let mut p: *mut Expr = sqlite3ExprSkipCollateAndLikely(
                (*(&raw mut (*pDistinct).a as *mut ExprList_item).offset(i as isize))
                    .pExpr,
            );
            if !p.is_null() {
                if !((*p).op as ::core::ffi::c_int != TK_COLUMN
                    && (*p).op as ::core::ffi::c_int != TK_AGG_COLUMN)
                {
                    if (*p).iTable == iBase
                        && ((*p).iColumn as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    {
                        return 1 as ::core::ffi::c_int;
                    }
                }
            }
            i += 1;
        }
        pIdx = (*pTab).pIndex;
        while !pIdx.is_null() {
            if (*pIdx).onError as ::core::ffi::c_int != OE_None {
                if (*pIdx).pPartIdxWhere.is_null() {
                    i = 0 as ::core::ffi::c_int;
                    while i < (*pIdx).nKeyCol as ::core::ffi::c_int {
                        if sqlite3WhereFindTerm(
                                pWC,
                                iBase,
                                i,
                                !(0 as ::core::ffi::c_int as Bitmask),
                                WO_EQ as u32_0,
                                pIdx,
                            )
                            .is_null()
                        {
                            if findIndexCol(pParse, pDistinct, iBase, pIdx, i)
                                < 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                            if indexColumnNotNull(pIdx, i) == 0 as ::core::ffi::c_int {
                                break;
                            }
                        }
                        i += 1;
                    }
                    if i == (*pIdx).nKeyCol as ::core::ffi::c_int {
                        return 1 as ::core::ffi::c_int;
                    }
                }
            }
            pIdx = (*pIdx).pNext;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn estLog(mut N: LogEst) -> LogEst {
    unsafe {
        return (if N as ::core::ffi::c_int <= 10 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            sqlite3LogEst(N as u64_0) as ::core::ffi::c_int - 33 as ::core::ffi::c_int
        }) as LogEst;
    }
}
unsafe extern "C" fn translateColumnToCopy(
    mut pParse: *mut Parse,
    mut iStart: ::core::ffi::c_int,
    mut iTabCur: ::core::ffi::c_int,
    mut iRegister: ::core::ffi::c_int,
    mut iAutoidxCur: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut pOp: *mut VdbeOp = sqlite3VdbeGetOp(v, iStart);
        let mut iEnd: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(v);
        if (*(*pParse).db).mallocFailed != 0 {
            return;
        }
        while iStart < iEnd {
            if !((*pOp).p1 != iTabCur) {
                if (*pOp).opcode as ::core::ffi::c_int == OP_Column {
                    (*pOp).opcode = OP_Copy as u8_0;
                    (*pOp).p1 = (*pOp).p2 + iRegister;
                    (*pOp).p2 = (*pOp).p3;
                    (*pOp).p3 = 0 as ::core::ffi::c_int;
                    (*pOp).p5 = 2 as u16_0;
                } else if (*pOp).opcode as ::core::ffi::c_int == OP_Rowid {
                    (*pOp).opcode = OP_Sequence as u8_0;
                    (*pOp).p1 = iAutoidxCur;
                }
            }
            iStart += 1;
            pOp = pOp.offset(1);
        }
    }
}
unsafe extern "C" fn constraintCompatibleWithOuterJoin(
    mut pTerm: *const WhereTerm,
    mut pSrc: *const SrcItem,
) -> ::core::ffi::c_int {
    unsafe {
        if !((*(*pTerm).pExpr).flags
            & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int) as u32_0
            != 0 as u32_0) || (*(*pTerm).pExpr).w.iJoin != (*pSrc).iCursor
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSrc).fg.jointype as ::core::ffi::c_int & (JT_LEFT | JT_RIGHT)
            != 0 as ::core::ffi::c_int
            && (*(*pTerm).pExpr).flags & 0x2 as ::core::ffi::c_int as u32_0 != 0 as u32_0
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn columnIsGoodIndexCandidate(
    mut pTab: *const Table,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pIdx: *const Index = ::core::ptr::null::<Index>();
        pIdx = (*pTab).pIndex;
        while !pIdx.is_null() {
            let mut j: ::core::ffi::c_int = 0;
            j = 0 as ::core::ffi::c_int;
            while j < (*pIdx).nKeyCol as ::core::ffi::c_int {
                if *(*pIdx).aiColumn.offset(j as isize) as ::core::ffi::c_int == iCol {
                    if j == 0 as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    }
                    if (*pIdx).hasStat1() as ::core::ffi::c_int != 0
                        && *(*pIdx)
                            .aiRowLogEst
                            .offset((j + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int > 20 as ::core::ffi::c_int
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    break;
                } else {
                    j += 1;
                }
            }
            pIdx = (*pIdx).pNext;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn termCanDriveIndex(
    mut pTerm: *const WhereTerm,
    mut pSrc: *const SrcItem,
    notReady: Bitmask,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aff: ::core::ffi::c_char = 0;
        let mut leftCol: ::core::ffi::c_int = 0;
        if (*pTerm).leftCursor != (*pSrc).iCursor {
            return 0 as ::core::ffi::c_int;
        }
        if (*pTerm).eOperator as ::core::ffi::c_int & (WO_EQ | WO_IS)
            == 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSrc).fg.jointype as ::core::ffi::c_int & (JT_LEFT | JT_LTORJ | JT_RIGHT)
            != 0 as ::core::ffi::c_int
            && constraintCompatibleWithOuterJoin(pTerm, pSrc) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pTerm).prereqRight & notReady != 0 as ::core::ffi::c_ulonglong {
            return 0 as ::core::ffi::c_int;
        }
        leftCol = (*pTerm).u.x.leftColumn;
        if leftCol < 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        aff = (*(*(*pSrc).pSTab).aCol.offset(leftCol as isize)).affinity;
        if sqlite3IndexAffinityOk((*pTerm).pExpr, aff) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        return columnIsGoodIndexCandidate((*pSrc).pSTab, leftCol);
    }
}
#[inline(never)]
unsafe extern "C" fn constructAutomaticIndex(
    mut pParse: *mut Parse,
    mut pWC: *mut WhereClause,
    notReady: Bitmask,
    mut pLevel: *mut WhereLevel,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut nKeyCol: ::core::ffi::c_int = 0;
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut pWCEnd: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut addrInit: ::core::ffi::c_int = 0;
        let mut pTable: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut addrTop: ::core::ffi::c_int = 0;
        let mut regRecord: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut mxBitCol: ::core::ffi::c_int = 0;
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        let mut pLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut zNotUsed: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut idxCols: Bitmask = 0;
        let mut extraCols: Bitmask = 0;
        let mut sentWarning: u8_0 = 0 as u8_0;
        let mut useBloomFilter: u8_0 = 0 as u8_0;
        let mut pPartial: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut iContinue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pTabList: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut pSrc: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut addrCounter: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regBase: ::core::ffi::c_int = 0;
        v = (*pParse).pVdbe;
        addrInit = sqlite3VdbeAddOp0(v, OP_Once);
        nKeyCol = 0 as ::core::ffi::c_int;
        pTabList = (*(*pWC).pWInfo).pTabList;
        pSrc = (&raw mut (*pTabList).a as *mut SrcItem).offset((*pLevel).iFrom as isize)
            as *mut SrcItem;
        pTable = (*pSrc).pSTab;
        pWCEnd = (*pWC).a.offset((*pWC).nTerm as isize) as *mut WhereTerm;
        pLoop = (*pLevel).pWLoop as *mut WhereLoop;
        idxCols = 0 as Bitmask;
        pTerm = (*pWC).a;
        loop {
            if !(pTerm < pWCEnd) {
                c2rust_current_block = 11385396242402735691;
                break;
            }
            let mut pExpr: *mut Expr = (*pTerm).pExpr;
            if (*pTerm).wtFlags as ::core::ffi::c_int & TERM_VIRTUAL
                == 0 as ::core::ffi::c_int
                && sqlite3ExprIsSingleTableConstraint(
                    pExpr,
                    pTabList,
                    (*pLevel).iFrom as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                ) != 0
            {
                pPartial = sqlite3ExprAnd(
                    pParse,
                    pPartial,
                    sqlite3ExprDup((*pParse).db, pExpr, 0 as ::core::ffi::c_int),
                );
            }
            if termCanDriveIndex(pTerm, pSrc, notReady) != 0 {
                let mut iCol: ::core::ffi::c_int = 0;
                let mut cMask: Bitmask = 0;
                iCol = (*pTerm).u.x.leftColumn;
                cMask = (if iCol >= BMS {
                    (1 as ::core::ffi::c_int as ::core::ffi::c_ulonglong)
                        << (::core::mem::size_of::<Bitmask>() as usize)
                            .wrapping_mul(8 as usize) as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                } else {
                    (1 as ::core::ffi::c_int as ::core::ffi::c_ulonglong) << iCol
                }) as Bitmask;
                if sentWarning == 0 {
                    sqlite3_log(
                        SQLITE_WARNING_AUTOINDEX,
                        b"automatic index on %s(%s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*pTable).zName,
                        (*(*pTable).aCol.offset(iCol as isize)).zCnName,
                    );
                    sentWarning = 1 as u8_0;
                }
                if idxCols & cMask == 0 as ::core::ffi::c_ulonglong {
                    if whereLoopResize(
                        (*pParse).db,
                        pLoop,
                        nKeyCol + 1 as ::core::ffi::c_int,
                    ) != 0
                    {
                        c2rust_current_block = 1352400130569879833;
                        break;
                    }
                    let c2rust_fresh3 = nKeyCol;
                    nKeyCol = nKeyCol + 1;
                    let ref mut c2rust_fresh4 = *(*pLoop)
                        .aLTerm
                        .offset(c2rust_fresh3 as isize);
                    *c2rust_fresh4 = pTerm;
                    idxCols |= cMask as ::core::ffi::c_ulonglong;
                }
            }
            pTerm = pTerm.offset(1);
        }
        match c2rust_current_block {
            11385396242402735691 => {
                (*pLoop).nLTerm = nKeyCol as u16_0;
                (*pLoop).u.btree.nEq = (*pLoop).nLTerm;
                (*pLoop).wsFlags = (WHERE_COLUMN_EQ | WHERE_IDX_ONLY | WHERE_INDEXED
                    | WHERE_AUTO_INDEX) as u32_0;
                if (*pTable).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
                    extraCols = ALLBITS & !idxCols;
                } else {
                    extraCols = (*pSrc).colUsed
                        & (!idxCols
                            | (1 as ::core::ffi::c_int as Bitmask)
                                << (::core::mem::size_of::<Bitmask>() as usize)
                                    .wrapping_mul(8 as usize) as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int);
                }
                if !((*pTable).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0) {
                    i = 0 as ::core::ffi::c_int;
                    while i < (*pTable).nCol as ::core::ffi::c_int {
                        if !((*(*pTable).aCol.offset(i as isize)).colFlags
                            as ::core::ffi::c_int & COLFLAG_PRIMKEY
                            == 0 as ::core::ffi::c_int)
                        {
                            if i >= BMS - 1 as ::core::ffi::c_int {
                                extraCols
                                    |= ((1 as ::core::ffi::c_int as Bitmask)
                                        << (::core::mem::size_of::<Bitmask>() as usize)
                                            .wrapping_mul(8 as usize) as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
                                break;
                            } else if !(idxCols
                                & (1 as ::core::ffi::c_int as Bitmask) << i != 0)
                            {
                                extraCols
                                    |= ((1 as ::core::ffi::c_int as Bitmask) << i)
                                        as ::core::ffi::c_ulonglong;
                            }
                        }
                        i += 1;
                    }
                }
                mxBitCol = if ((::core::mem::size_of::<Bitmask>() as usize)
                    .wrapping_mul(8 as usize) as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) < (*pTable).nCol as ::core::ffi::c_int
                {
                    (::core::mem::size_of::<Bitmask>() as usize).wrapping_mul(8 as usize)
                        as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                } else {
                    (*pTable).nCol as ::core::ffi::c_int
                };
                i = 0 as ::core::ffi::c_int;
                while i < mxBitCol {
                    if extraCols & (1 as ::core::ffi::c_int as Bitmask) << i != 0 {
                        nKeyCol += 1;
                    }
                    i += 1;
                }
                if (*pSrc).colUsed
                    & (1 as ::core::ffi::c_int as Bitmask)
                        << (::core::mem::size_of::<Bitmask>() as usize)
                            .wrapping_mul(8 as usize) as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int != 0
                {
                    nKeyCol
                        += (*pTable).nCol as ::core::ffi::c_int - BMS
                            + 1 as ::core::ffi::c_int;
                }
                pIdx = sqlite3AllocateIndexObject(
                    (*pParse).db,
                    nKeyCol
                        + ((*pTable).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0)
                            as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    &raw mut zNotUsed,
                );
                if !pIdx.is_null() {
                    (*pLoop).u.btree.pIndex = pIdx;
                    (*pIdx).zName = b"auto-index\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                    (*pIdx).pTable = pTable;
                    n = 0 as ::core::ffi::c_int;
                    idxCols = 0 as Bitmask;
                    pTerm = (*pWC).a;
                    while pTerm < pWCEnd {
                        if termCanDriveIndex(pTerm, pSrc, notReady) != 0 {
                            let mut iCol_0: ::core::ffi::c_int = 0;
                            let mut cMask_0: Bitmask = 0;
                            iCol_0 = (*pTerm).u.x.leftColumn;
                            cMask_0 = (if iCol_0 >= BMS {
                                (1 as ::core::ffi::c_int as ::core::ffi::c_ulonglong)
                                    << (::core::mem::size_of::<Bitmask>() as usize)
                                        .wrapping_mul(8 as usize) as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int
                            } else {
                                (1 as ::core::ffi::c_int as ::core::ffi::c_ulonglong)
                                    << iCol_0
                            }) as Bitmask;
                            if idxCols & cMask_0 == 0 as ::core::ffi::c_ulonglong {
                                let mut pX: *mut Expr = (*pTerm).pExpr;
                                idxCols |= cMask_0 as ::core::ffi::c_ulonglong;
                                *(*pIdx).aiColumn.offset(n as isize) = (*pTerm)
                                    .u
                                    .x
                                    .leftColumn as i16_0;
                                pColl = sqlite3ExprCompareCollSeq(pParse, pX);
                                let ref mut c2rust_fresh5 = *(*pIdx)
                                    .azColl
                                    .offset(n as isize);
                                *c2rust_fresh5 = if !pColl.is_null() {
                                    (*pColl).zName as *const ::core::ffi::c_char
                                } else {
                                    &raw const sqlite3StrBINARY as *const ::core::ffi::c_char
                                };
                                n += 1;
                                if !(*pX).pLeft.is_null()
                                    && sqlite3ExprAffinity((*pX).pLeft) as ::core::ffi::c_int
                                        != SQLITE_AFF_TEXT
                                {
                                    useBloomFilter = 1 as u8_0;
                                }
                            }
                        }
                        pTerm = pTerm.offset(1);
                    }
                    i = 0 as ::core::ffi::c_int;
                    while i < mxBitCol {
                        if extraCols & (1 as ::core::ffi::c_int as Bitmask) << i != 0 {
                            *(*pIdx).aiColumn.offset(n as isize) = i as i16_0;
                            let ref mut c2rust_fresh6 = *(*pIdx)
                                .azColl
                                .offset(n as isize);
                            *c2rust_fresh6 = &raw const sqlite3StrBINARY
                                as *const ::core::ffi::c_char;
                            n += 1;
                        }
                        i += 1;
                    }
                    if (*pSrc).colUsed
                        & (1 as ::core::ffi::c_int as Bitmask)
                            << (::core::mem::size_of::<Bitmask>() as usize)
                                .wrapping_mul(8 as usize) as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int != 0
                    {
                        i = BMS - 1 as ::core::ffi::c_int;
                        while i < (*pTable).nCol as ::core::ffi::c_int {
                            *(*pIdx).aiColumn.offset(n as isize) = i as i16_0;
                            let ref mut c2rust_fresh7 = *(*pIdx)
                                .azColl
                                .offset(n as isize);
                            *c2rust_fresh7 = &raw const sqlite3StrBINARY
                                as *const ::core::ffi::c_char;
                            n += 1;
                            i += 1;
                        }
                    }
                    if (*pTable).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0 {
                        *(*pIdx).aiColumn.offset(n as isize) = XN_ROWID as i16_0;
                        let ref mut c2rust_fresh8 = *(*pIdx).azColl.offset(n as isize);
                        *c2rust_fresh8 = &raw const sqlite3StrBINARY
                            as *const ::core::ffi::c_char;
                    }
                    let c2rust_fresh9 = (*pParse).nTab;
                    (*pParse).nTab = (*pParse).nTab + 1;
                    (*pLevel).iIdxCur = c2rust_fresh9;
                    sqlite3VdbeAddOp2(
                        v,
                        OP_OpenAutoindex,
                        (*pLevel).iIdxCur,
                        nKeyCol + 1 as ::core::ffi::c_int,
                    );
                    sqlite3VdbeSetP4KeyInfo(pParse, pIdx);
                    if (*(*pParse).db).dbOptFlags & 0x80000 as u32_0 == 0 as u32_0
                        && useBloomFilter as ::core::ffi::c_int != 0
                    {
                        sqlite3WhereExplainBloomFilter(pParse, (*pWC).pWInfo, pLevel);
                        (*pParse).nMem += 1;
                        (*pLevel).regFilter = (*pParse).nMem;
                        sqlite3VdbeAddOp2(
                            v,
                            OP_Blob,
                            10000 as ::core::ffi::c_int,
                            (*pLevel).regFilter,
                        );
                    }
                    if (*pSrc).fg.viaCoroutine() != 0 {
                        let mut regYield: ::core::ffi::c_int = 0;
                        let mut pSubq: *mut Subquery = ::core::ptr::null_mut::<
                            Subquery,
                        >();
                        pSubq = (*pSrc).u4.pSubq;
                        regYield = (*pSubq).regReturn;
                        addrCounter = sqlite3VdbeAddOp2(
                            v,
                            OP_Integer,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        );
                        sqlite3VdbeAddOp3(
                            v,
                            OP_InitCoroutine,
                            regYield,
                            0 as ::core::ffi::c_int,
                            (*pSubq).addrFillSub,
                        );
                        addrTop = sqlite3VdbeAddOp1(v, OP_Yield, regYield);
                    } else {
                        addrTop = sqlite3VdbeAddOp2(
                            v,
                            OP_Rewind,
                            (*pLevel).iTabCur,
                            (*pLevel).addrHalt,
                        );
                    }
                    if !pPartial.is_null() {
                        iContinue = sqlite3VdbeMakeLabel(pParse);
                        sqlite3ExprIfFalse(
                            pParse,
                            pPartial,
                            iContinue,
                            SQLITE_JUMPIFNULL,
                        );
                        (*pLoop).wsFlags = ((*pLoop).wsFlags as ::core::ffi::c_uint
                            | WHERE_PARTIALIDX as ::core::ffi::c_uint) as u32_0;
                    }
                    regRecord = sqlite3GetTempReg(pParse);
                    regBase = sqlite3GenerateIndexKey(
                        pParse,
                        pIdx,
                        (*pLevel).iTabCur,
                        regRecord,
                        0 as ::core::ffi::c_int,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        ::core::ptr::null_mut::<Index>(),
                        0 as ::core::ffi::c_int,
                    );
                    if (*pLevel).regFilter != 0 {
                        sqlite3VdbeAddOp4Int(
                            v,
                            OP_FilterAdd,
                            (*pLevel).regFilter,
                            0 as ::core::ffi::c_int,
                            regBase,
                            (*pLoop).u.btree.nEq as ::core::ffi::c_int,
                        );
                    }
                    sqlite3VdbeAddOp2(v, OP_IdxInsert, (*pLevel).iIdxCur, regRecord);
                    sqlite3VdbeChangeP5(v, OPFLAG_USESEEKRESULT as u16_0);
                    if !pPartial.is_null() {
                        sqlite3VdbeResolveLabel(v, iContinue);
                    }
                    if (*pSrc).fg.viaCoroutine() != 0 {
                        sqlite3VdbeChangeP2(v, addrCounter, regBase + n);
                        translateColumnToCopy(
                            pParse,
                            addrTop,
                            (*pLevel).iTabCur,
                            (*(*pSrc).u4.pSubq).regResult,
                            (*pLevel).iIdxCur,
                        );
                        sqlite3VdbeGoto(v, addrTop);
                        (*pSrc)
                            .fg
                            .set_viaCoroutine(
                                0 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                        sqlite3VdbeJumpHere(v, addrTop);
                    } else {
                        sqlite3VdbeAddOp2(
                            v,
                            OP_Next,
                            (*pLevel).iTabCur,
                            addrTop + 1 as ::core::ffi::c_int,
                        );
                        sqlite3VdbeChangeP5(v, SQLITE_STMTSTATUS_AUTOINDEX as u16_0);
                        if (*pSrc).fg.jointype as ::core::ffi::c_int & JT_LEFT
                            != 0 as ::core::ffi::c_int
                        {
                            sqlite3VdbeJumpHere(v, addrTop);
                        }
                    }
                    sqlite3ReleaseTempReg(pParse, regRecord);
                    sqlite3VdbeJumpHere(v, addrInit);
                }
            }
            _ => {}
        }
        sqlite3ExprDelete((*pParse).db, pPartial);
    }
}
#[inline(never)]
unsafe extern "C" fn sqlite3ConstructBloomFilter(
    mut pWInfo: *mut WhereInfo,
    mut iLevel: ::core::ffi::c_int,
    mut pLevel: *mut WhereLevel,
    mut notReady: Bitmask,
) {
    unsafe {
        let mut addrOnce: ::core::ffi::c_int = 0;
        let mut addrTop: ::core::ffi::c_int = 0;
        let mut addrCont: ::core::ffi::c_int = 0;
        let mut pTerm: *const WhereTerm = ::core::ptr::null::<WhereTerm>();
        let mut pWCEnd: *const WhereTerm = ::core::ptr::null::<WhereTerm>();
        let mut pParse: *mut Parse = (*pWInfo).pParse;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut pLoop: *mut WhereLoop = (*pLevel).pWLoop as *mut WhereLoop;
        let mut iCur: ::core::ffi::c_int = 0;
        let mut saved_pIdxEpr: *mut IndexedExpr = ::core::ptr::null_mut::<IndexedExpr>();
        let mut saved_pIdxPartExpr: *mut IndexedExpr = ::core::ptr::null_mut::<
            IndexedExpr,
        >();
        saved_pIdxEpr = (*pParse).pIdxEpr;
        saved_pIdxPartExpr = (*pParse).pIdxPartExpr;
        (*pParse).pIdxEpr = ::core::ptr::null_mut::<IndexedExpr>();
        (*pParse).pIdxPartExpr = ::core::ptr::null_mut::<IndexedExpr>();
        addrOnce = sqlite3VdbeAddOp0(v, OP_Once);
        loop {
            let mut pTabList: *const SrcList = ::core::ptr::null::<SrcList>();
            let mut pItem: *const SrcItem = ::core::ptr::null::<SrcItem>();
            let mut pTab: *const Table = ::core::ptr::null::<Table>();
            let mut sz: u64_0 = 0;
            let mut iSrc: ::core::ffi::c_int = 0;
            sqlite3WhereExplainBloomFilter(pParse, pWInfo, pLevel);
            addrCont = sqlite3VdbeMakeLabel(pParse);
            iCur = (*pLevel).iTabCur;
            (*pParse).nMem += 1;
            (*pLevel).regFilter = (*pParse).nMem;
            pTabList = (*pWInfo).pTabList;
            iSrc = (*pLevel).iFrom as ::core::ffi::c_int;
            pItem = (&raw const (*pTabList).a as *const SrcItem).offset(iSrc as isize)
                as *const SrcItem;
            pTab = (*pItem).pSTab;
            sz = sqlite3LogEstToInt((*pTab).nRowLogEst);
            if sz < 10000 as ::core::ffi::c_ulonglong {
                sz = 10000 as u64_0;
            } else if sz > 10000000 as ::core::ffi::c_ulonglong {
                sz = 10000000 as u64_0;
            }
            sqlite3VdbeAddOp2(v, OP_Blob, sz as ::core::ffi::c_int, (*pLevel).regFilter);
            addrTop = sqlite3VdbeAddOp1(v, OP_Rewind, iCur);
            pWCEnd = (*pWInfo).sWC.a.offset((*pWInfo).sWC.nTerm as isize)
                as *mut WhereTerm;
            pTerm = (*pWInfo).sWC.a;
            while pTerm < pWCEnd {
                let mut pExpr: *mut Expr = (*pTerm).pExpr;
                if (*pTerm).wtFlags as ::core::ffi::c_int & TERM_VIRTUAL
                    == 0 as ::core::ffi::c_int
                    && sqlite3ExprIsSingleTableConstraint(
                        pExpr,
                        pTabList,
                        iSrc,
                        0 as ::core::ffi::c_int,
                    ) != 0
                {
                    sqlite3ExprIfFalse(
                        pParse,
                        (*pTerm).pExpr,
                        addrCont,
                        SQLITE_JUMPIFNULL,
                    );
                }
                pTerm = pTerm.offset(1);
            }
            if (*pLoop).wsFlags & WHERE_IPK as u32_0 != 0 {
                let mut r1: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                sqlite3VdbeAddOp2(v, OP_Rowid, iCur, r1);
                sqlite3VdbeAddOp4Int(
                    v,
                    OP_FilterAdd,
                    (*pLevel).regFilter,
                    0 as ::core::ffi::c_int,
                    r1,
                    1 as ::core::ffi::c_int,
                );
                sqlite3ReleaseTempReg(pParse, r1);
            } else {
                let mut pIdx: *mut Index = (*pLoop).u.btree.pIndex;
                let mut n: ::core::ffi::c_int = (*pLoop).u.btree.nEq
                    as ::core::ffi::c_int;
                let mut r1_0: ::core::ffi::c_int = sqlite3GetTempRange(pParse, n);
                let mut jj: ::core::ffi::c_int = 0;
                jj = 0 as ::core::ffi::c_int;
                while jj < n {
                    sqlite3ExprCodeLoadIndexColumn(pParse, pIdx, iCur, jj, r1_0 + jj);
                    jj += 1;
                }
                sqlite3VdbeAddOp4Int(
                    v,
                    OP_FilterAdd,
                    (*pLevel).regFilter,
                    0 as ::core::ffi::c_int,
                    r1_0,
                    n,
                );
                sqlite3ReleaseTempRange(pParse, r1_0, n);
            }
            sqlite3VdbeResolveLabel(v, addrCont);
            sqlite3VdbeAddOp2(
                v,
                OP_Next,
                (*pLevel).iTabCur,
                addrTop + 1 as ::core::ffi::c_int,
            );
            sqlite3VdbeJumpHere(v, addrTop);
            (*pLoop).wsFlags = ((*pLoop).wsFlags as ::core::ffi::c_uint
                & !WHERE_BLOOMFILTER as ::core::ffi::c_uint) as u32_0;
            if (*(*pParse).db).dbOptFlags & 0x100000 as u32_0 != 0 as u32_0 {
                break;
            }
            loop {
                iLevel += 1;
                if !(iLevel < (*pWInfo).nLevel as ::core::ffi::c_int) {
                    break;
                }
                let mut pTabItem: *const SrcItem = ::core::ptr::null::<SrcItem>();
                pLevel = (&raw mut (*pWInfo).a as *mut WhereLevel)
                    .offset(iLevel as isize) as *mut WhereLevel;
                pTabItem = (&raw mut (*(*pWInfo).pTabList).a as *mut SrcItem)
                    .offset((*pLevel).iFrom as isize) as *mut SrcItem;
                if (*pTabItem).fg.jointype as ::core::ffi::c_int & (JT_LEFT | JT_LTORJ)
                    != 0
                {
                    continue;
                }
                pLoop = (*pLevel).pWLoop as *mut WhereLoop;
                if pLoop.is_null() {
                    continue;
                }
                if (*pLoop).prereq & notReady != 0 {
                    continue;
                }
                if (*pLoop).wsFlags & (WHERE_BLOOMFILTER | WHERE_COLUMN_IN) as u32_0
                    == WHERE_BLOOMFILTER as u32_0
                {
                    break;
                }
            }
            if !(iLevel < (*pWInfo).nLevel as ::core::ffi::c_int) {
                break;
            }
        }
        sqlite3VdbeJumpHere(v, addrOnce);
        (*pParse).pIdxEpr = saved_pIdxEpr;
        (*pParse).pIdxPartExpr = saved_pIdxPartExpr;
    }
}
unsafe extern "C" fn termFromWhereClause(
    mut pWC: *mut WhereClause,
    mut iTerm: ::core::ffi::c_int,
) -> *mut WhereTerm {
    unsafe {
        let mut p: *mut WhereClause = ::core::ptr::null_mut::<WhereClause>();
        p = pWC;
        while !p.is_null() {
            if iTerm < (*p).nTerm {
                return (*p).a.offset(iTerm as isize) as *mut WhereTerm;
            }
            iTerm -= (*p).nTerm;
            p = (*p).pOuter;
        }
        return ::core::ptr::null_mut::<WhereTerm>();
    }
}
unsafe extern "C" fn allocateIndexInfo(
    mut pWInfo: *mut WhereInfo,
    mut pWC: *mut WhereClause,
    mut mUnusable: Bitmask,
    mut pSrc: *mut SrcItem,
    mut pmNoOmit: *mut u16_0,
) -> *mut sqlite3_index_info {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut nTerm: ::core::ffi::c_int = 0;
        let mut pParse: *mut Parse = (*pWInfo).pParse;
        let mut pIdxCons: *mut sqlite3_index_constraint = ::core::ptr::null_mut::<
            sqlite3_index_constraint,
        >();
        let mut pIdxOrderBy: *mut sqlite3_index_orderby = ::core::ptr::null_mut::<
            sqlite3_index_orderby,
        >();
        let mut pUsage: *mut sqlite3_index_constraint_usage = ::core::ptr::null_mut::<
            sqlite3_index_constraint_usage,
        >();
        let mut pHidden: *mut HiddenIndexInfo = ::core::ptr::null_mut::<
            HiddenIndexInfo,
        >();
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut nOrderBy: ::core::ffi::c_int = 0;
        let mut pIdxInfo: *mut sqlite3_index_info = ::core::ptr::null_mut::<
            sqlite3_index_info,
        >();
        let mut mNoOmit: u16_0 = 0 as u16_0;
        let mut pTab: *const Table = ::core::ptr::null::<Table>();
        let mut eDistinct: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pOrderBy: *mut ExprList = (*pWInfo).pOrderBy;
        let mut p: *mut WhereClause = ::core::ptr::null_mut::<WhereClause>();
        pTab = (*pSrc).pSTab;
        p = pWC;
        nTerm = 0 as ::core::ffi::c_int;
        while !p.is_null() {
            i = 0 as ::core::ffi::c_int;
            pTerm = (*p).a;
            while i < (*p).nTerm {
                (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int & !TERM_OK)
                    as u16_0;
                if !((*pTerm).leftCursor != (*pSrc).iCursor) {
                    if !((*pTerm).prereqRight & mUnusable != 0) {
                        if !((*pTerm).eOperator as ::core::ffi::c_int
                            & !(0x800 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int)
                        {
                            if !((*pTerm).wtFlags as ::core::ffi::c_int & TERM_VNULL
                                != 0)
                            {
                                if !((*pSrc).fg.jointype as ::core::ffi::c_int
                                    & (JT_LEFT | JT_LTORJ | JT_RIGHT) != 0 as ::core::ffi::c_int
                                    && constraintCompatibleWithOuterJoin(pTerm, pSrc) == 0)
                                {
                                    nTerm += 1;
                                    (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int
                                        | TERM_OK) as u16_0;
                                }
                            }
                        }
                    }
                }
                i += 1;
                pTerm = pTerm.offset(1);
            }
            p = (*p).pOuter;
        }
        nOrderBy = 0 as ::core::ffi::c_int;
        if !pOrderBy.is_null() {
            let mut n: ::core::ffi::c_int = (*pOrderBy).nExpr;
            i = 0 as ::core::ffi::c_int;
            while i < n {
                let mut pExpr: *mut Expr = (*(&raw mut (*pOrderBy).a
                    as *mut ExprList_item)
                    .offset(i as isize))
                    .pExpr;
                let mut pE2: *mut Expr = ::core::ptr::null_mut::<Expr>();
                if !(sqlite3ExprIsConstant(::core::ptr::null_mut::<Parse>(), pExpr) != 0)
                {
                    if (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                        .offset(i as isize))
                        .fg
                        .sortFlags as ::core::ffi::c_int & KEYINFO_ORDER_BIGNULL != 0
                    {
                        break;
                    }
                    if !((*pExpr).op as ::core::ffi::c_int == TK_COLUMN
                        && (*pExpr).iTable == (*pSrc).iCursor)
                    {
                        if !((*pExpr).op as ::core::ffi::c_int == TK_COLLATE
                            && {
                                pE2 = (*pExpr).pLeft;
                                (*pE2).op as ::core::ffi::c_int == TK_COLUMN
                            } && (*pE2).iTable == (*pSrc).iCursor)
                        {
                            break;
                        }
                        let mut zColl: *const ::core::ffi::c_char = ::core::ptr::null::<
                            ::core::ffi::c_char,
                        >();
                        (*pExpr).iColumn = (*pE2).iColumn;
                        if !(((*pE2).iColumn as ::core::ffi::c_int)
                            < 0 as ::core::ffi::c_int)
                        {
                            zColl = sqlite3ColumnColl(
                                (*pTab).aCol.offset((*pE2).iColumn as isize) as *mut Column,
                            );
                            if zColl.is_null() {
                                zColl = &raw const sqlite3StrBINARY
                                    as *const ::core::ffi::c_char;
                            }
                            if !(sqlite3_stricmp((*pExpr).u.zToken, zColl)
                                == 0 as ::core::ffi::c_int)
                            {
                                break;
                            }
                        }
                    }
                }
                i += 1;
            }
            if i == n {
                nOrderBy = n;
                if (*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_DISTINCTBY != 0
                    && (*pSrc).fg.rowidUsed() == 0
                {
                    eDistinct = 2 as ::core::ffi::c_int
                        + ((*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_SORTBYGROUP
                            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                } else if (*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_GROUPBY != 0
                {
                    eDistinct = 1 as ::core::ffi::c_int;
                }
            }
        }
        pIdxInfo = sqlite3DbMallocZero(
            (*pParse).db,
            (::core::mem::size_of::<sqlite3_index_info>() as usize)
                .wrapping_add(
                    (::core::mem::size_of::<sqlite3_index_constraint>() as usize)
                        .wrapping_add(
                            ::core::mem::size_of::<sqlite3_index_constraint_usage>()
                                as usize,
                        )
                        .wrapping_mul(nTerm as usize),
                )
                .wrapping_add(
                    (::core::mem::size_of::<sqlite3_index_orderby>() as usize)
                        .wrapping_mul(nOrderBy as usize),
                )
                .wrapping_add(
                    (32 as usize)
                        .wrapping_add(
                            (nTerm as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut sqlite3_value>() as usize,
                                ),
                        ),
                ) as u64_0,
        ) as *mut sqlite3_index_info;
        if pIdxInfo.is_null() {
            sqlite3ErrorMsg(
                pParse,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<sqlite3_index_info>();
        }
        pHidden = pIdxInfo.offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_index_info as *mut HiddenIndexInfo;
        pIdxCons = (&raw mut (*pHidden).aRhs as *mut *mut sqlite3_value)
            .offset(nTerm as isize) as *mut *mut sqlite3_value
            as *mut sqlite3_index_constraint;
        pIdxOrderBy = pIdxCons.offset(nTerm as isize) as *mut sqlite3_index_constraint
            as *mut sqlite3_index_orderby;
        pUsage = pIdxOrderBy.offset(nOrderBy as isize) as *mut sqlite3_index_orderby
            as *mut sqlite3_index_constraint_usage;
        (*pIdxInfo).aConstraint = pIdxCons as *mut sqlite3_index_constraint;
        (*pIdxInfo).aOrderBy = pIdxOrderBy as *mut sqlite3_index_orderby;
        (*pIdxInfo).aConstraintUsage = pUsage as *mut sqlite3_index_constraint_usage;
        (*pIdxInfo).colUsed = (*pSrc).colUsed as sqlite3_int64 as sqlite3_uint64;
        if ((*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0)
            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            let mut pPk: *mut Index = sqlite3PrimaryKeyIndex(pTab as *mut Table);
            i = 0 as ::core::ffi::c_int;
            while i < (*pPk).nKeyCol as ::core::ffi::c_int {
                let mut iCol: ::core::ffi::c_int = *(*pPk).aiColumn.offset(i as isize)
                    as ::core::ffi::c_int;
                if iCol >= BMS - 1 as ::core::ffi::c_int {
                    iCol = BMS - 1 as ::core::ffi::c_int;
                }
                (*pIdxInfo).colUsed
                    |= ((1 as ::core::ffi::c_int as Bitmask) << iCol)
                        as ::core::ffi::c_ulonglong;
                i += 1;
            }
        }
        (*pHidden).pWC = pWC;
        (*pHidden).pParse = pParse;
        (*pHidden).eDistinct = eDistinct;
        (*pHidden).mIn = 0 as u32_0;
        p = pWC;
        j = 0 as ::core::ffi::c_int;
        i = j;
        while !p.is_null() {
            let mut nLast: ::core::ffi::c_int = i + (*p).nTerm;
            pTerm = (*p).a;
            while i < nLast {
                let mut op: u16_0 = 0;
                if !((*pTerm).wtFlags as ::core::ffi::c_int & TERM_OK
                    == 0 as ::core::ffi::c_int)
                {
                    (*pIdxCons.offset(j as isize)).iColumn = (*pTerm).u.x.leftColumn;
                    (*pIdxCons.offset(j as isize)).iTermOffset = i;
                    op = ((*pTerm).eOperator as ::core::ffi::c_int & WO_ALL) as u16_0;
                    if op as ::core::ffi::c_int == WO_IN {
                        if (*pTerm).wtFlags as ::core::ffi::c_int & TERM_SLICE
                            == 0 as ::core::ffi::c_int
                        {
                            (*pHidden).mIn = ((*pHidden).mIn as ::core::ffi::c_uint
                                | if j <= 31 as ::core::ffi::c_int {
                                    (1 as ::core::ffi::c_int as ::core::ffi::c_uint) << j
                                } else {
                                    0 as ::core::ffi::c_uint
                                }) as u32_0;
                        }
                        op = WO_EQ as u16_0;
                    }
                    if op as ::core::ffi::c_int == WO_AUX {
                        (*pIdxCons.offset(j as isize)).op = (*pTerm).eMatchOp
                            as ::core::ffi::c_uchar;
                    } else if op as ::core::ffi::c_int & (WO_ISNULL | WO_IS) != 0 {
                        if op as ::core::ffi::c_int == WO_ISNULL {
                            (*pIdxCons.offset(j as isize)).op = SQLITE_INDEX_CONSTRAINT_ISNULL
                                as ::core::ffi::c_uchar;
                        } else {
                            (*pIdxCons.offset(j as isize)).op = SQLITE_INDEX_CONSTRAINT_IS
                                as ::core::ffi::c_uchar;
                        }
                    } else {
                        (*pIdxCons.offset(j as isize)).op = op as u8_0
                            as ::core::ffi::c_uchar;
                        if op as ::core::ffi::c_int & (WO_LT | WO_LE | WO_GT | WO_GE)
                            != 0 && sqlite3ExprIsVector((*(*pTerm).pExpr).pRight) != 0
                        {
                            if j < 16 as ::core::ffi::c_int {
                                mNoOmit = (mNoOmit as ::core::ffi::c_int
                                    | (1 as ::core::ffi::c_int) << j) as u16_0;
                            }
                            if op as ::core::ffi::c_int == WO_LT {
                                (*pIdxCons.offset(j as isize)).op = WO_LE
                                    as ::core::ffi::c_uchar;
                            }
                            if op as ::core::ffi::c_int == WO_GT {
                                (*pIdxCons.offset(j as isize)).op = WO_GE
                                    as ::core::ffi::c_uchar;
                            }
                        }
                    }
                    j += 1;
                }
                i += 1;
                pTerm = pTerm.offset(1);
            }
            p = (*p).pOuter;
        }
        (*pIdxInfo).nConstraint = j;
        j = 0 as ::core::ffi::c_int;
        i = j;
        while i < nOrderBy {
            let mut pExpr_0: *mut Expr = (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                .offset(i as isize))
                .pExpr;
            if !(sqlite3ExprIsConstant(::core::ptr::null_mut::<Parse>(), pExpr_0) != 0) {
                (*pIdxOrderBy.offset(j as isize)).iColumn = (*pExpr_0).iColumn
                    as ::core::ffi::c_int;
                (*pIdxOrderBy.offset(j as isize)).desc = ((*(&raw mut (*pOrderBy).a
                    as *mut ExprList_item)
                    .offset(i as isize))
                    .fg
                    .sortFlags as ::core::ffi::c_int & KEYINFO_ORDER_DESC)
                    as ::core::ffi::c_uchar;
                j += 1;
            }
            i += 1;
        }
        (*pIdxInfo).nOrderBy = j;
        *pmNoOmit = mNoOmit;
        return pIdxInfo;
    }
}
unsafe extern "C" fn freeIdxStr(mut pIdxInfo: *mut sqlite3_index_info) {
    unsafe {
        if (*pIdxInfo).needToFreeIdxStr != 0 {
            sqlite3_free((*pIdxInfo).idxStr as *mut ::core::ffi::c_void);
            (*pIdxInfo).idxStr = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*pIdxInfo).needToFreeIdxStr = 0 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn freeIndexInfo(
    mut db: *mut sqlite3,
    mut pIdxInfo: *mut sqlite3_index_info,
) {
    unsafe {
        let mut pHidden: *mut HiddenIndexInfo = ::core::ptr::null_mut::<
            HiddenIndexInfo,
        >();
        let mut i: ::core::ffi::c_int = 0;
        pHidden = pIdxInfo.offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_index_info as *mut HiddenIndexInfo;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdxInfo).nConstraint {
            sqlite3ValueFree(
                *(&raw mut (*pHidden).aRhs as *mut *mut sqlite3_value).offset(i as isize),
            );
            let ref mut c2rust_fresh21 = *(&raw mut (*pHidden).aRhs
                as *mut *mut sqlite3_value)
                .offset(i as isize);
            *c2rust_fresh21 = ::core::ptr::null_mut::<sqlite3_value>();
            i += 1;
        }
        freeIdxStr(pIdxInfo);
        sqlite3DbFree(db, pIdxInfo as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn vtabBestIndex(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut p: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pVtab: *mut sqlite3_vtab = ::core::ptr::null_mut::<sqlite3_vtab>();
        pVtab = (*sqlite3GetVTable((*pParse).db, pTab)).pVtab;
        (*(*pParse).db).nSchemaLock = (*(*pParse).db).nSchemaLock.wrapping_add(1);
        rc = (*(*pVtab).pModule)
            .xBestIndex
            .expect("non-null function pointer")(pVtab, p);
        (*(*pParse).db).nSchemaLock = (*(*pParse).db).nSchemaLock.wrapping_sub(1);
        if rc != SQLITE_OK && rc != SQLITE_CONSTRAINT {
            if rc == SQLITE_NOMEM {
                sqlite3OomFault((*pParse).db);
            } else if (*pVtab).zErrMsg.is_null() {
                sqlite3ErrorMsg(
                    pParse,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3ErrStr(rc),
                );
            } else {
                sqlite3ErrorMsg(
                    pParse,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pVtab).zErrMsg,
                );
            }
        }
        if (*(*pTab).u.vtab.p).bAllSchemas != 0 {
            sqlite3VtabUsesAllSchemas(pParse);
        }
        sqlite3_free((*pVtab).zErrMsg as *mut ::core::ffi::c_void);
        (*pVtab).zErrMsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return rc;
    }
}
unsafe extern "C" fn whereRangeAdjust(
    mut pTerm: *mut WhereTerm,
    mut nNew: LogEst,
) -> LogEst {
    unsafe {
        let mut nRet: LogEst = nNew;
        if !pTerm.is_null() {
            if (*pTerm).truthProb as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                nRet = (nRet as ::core::ffi::c_int
                    + (*pTerm).truthProb as ::core::ffi::c_int) as LogEst;
            } else if (*pTerm).wtFlags as ::core::ffi::c_int & TERM_VNULL
                == 0 as ::core::ffi::c_int
            {
                nRet = (nRet as ::core::ffi::c_int - 20 as ::core::ffi::c_int) as LogEst;
            }
        }
        return nRet;
    }
}
unsafe extern "C" fn whereRangeScanEst(
    mut pParse: *mut Parse,
    mut pBuilder: *mut WhereLoopBuilder,
    mut pLower: *mut WhereTerm,
    mut pUpper: *mut WhereTerm,
    mut pLoop: *mut WhereLoop,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nOut: ::core::ffi::c_int = (*pLoop).nOut as ::core::ffi::c_int;
        let mut nNew: LogEst = 0;
        nNew = whereRangeAdjust(pLower, nOut as LogEst);
        nNew = whereRangeAdjust(pUpper, nNew);
        if !pLower.is_null()
            && (*pLower).truthProb as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && !pUpper.is_null()
            && (*pUpper).truthProb as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            nNew = (nNew as ::core::ffi::c_int - 20 as ::core::ffi::c_int) as LogEst;
        }
        nOut
            -= !pLower.is_null() as ::core::ffi::c_int
                + !pUpper.is_null() as ::core::ffi::c_int;
        if (nNew as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
            nNew = 10 as LogEst;
        }
        if (nNew as ::core::ffi::c_int) < nOut {
            nOut = nNew as ::core::ffi::c_int;
        }
        (*pLoop).nOut = nOut as LogEst;
        return rc;
    }
}
unsafe extern "C" fn whereLoopInit(mut p: *mut WhereLoop) {
    unsafe {
        (*p).aLTerm = &raw mut (*p).aLTermSpace as *mut *mut WhereTerm;
        (*p).nLTerm = 0 as u16_0;
        (*p).nLSlot = (::core::mem::size_of::<[*mut WhereTerm; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut WhereTerm>() as usize)
            as ::core::ffi::c_int as u16_0;
        (*p).wsFlags = 0 as u32_0;
    }
}
unsafe extern "C" fn whereLoopClearUnion(mut db: *mut sqlite3, mut p: *mut WhereLoop) {
    unsafe {
        if (*p).wsFlags & (WHERE_VIRTUALTABLE | WHERE_AUTO_INDEX) as u32_0 != 0 {
            if (*p).wsFlags & WHERE_VIRTUALTABLE as u32_0 != 0 as u32_0
                && (*p).u.vtab.needFree() as ::core::ffi::c_int != 0
            {
                sqlite3_free((*p).u.vtab.idxStr as *mut ::core::ffi::c_void);
                (*p).u.vtab.set_needFree(0 as u32_0 as u32_0);
                (*p).u.vtab.idxStr = ::core::ptr::null_mut::<::core::ffi::c_char>();
            } else if (*p).wsFlags & WHERE_AUTO_INDEX as u32_0 != 0 as u32_0
                && !(*p).u.btree.pIndex.is_null()
            {
                sqlite3DbFree(
                    db,
                    (*(*p).u.btree.pIndex).zColAff as *mut ::core::ffi::c_void,
                );
                sqlite3DbFreeNN(db, (*p).u.btree.pIndex as *mut ::core::ffi::c_void);
                (*p).u.btree.pIndex = ::core::ptr::null_mut::<Index>();
            }
        }
    }
}
unsafe extern "C" fn whereLoopClear(mut db: *mut sqlite3, mut p: *mut WhereLoop) {
    unsafe {
        if (*p).aLTerm != &raw mut (*p).aLTermSpace as *mut *mut WhereTerm {
            sqlite3DbFreeNN(db, (*p).aLTerm as *mut ::core::ffi::c_void);
            (*p).aLTerm = &raw mut (*p).aLTermSpace as *mut *mut WhereTerm;
            (*p).nLSlot = (::core::mem::size_of::<[*mut WhereTerm; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<*mut WhereTerm>() as usize)
                as ::core::ffi::c_int as u16_0;
        }
        whereLoopClearUnion(db, p);
        (*p).nLTerm = 0 as u16_0;
        (*p).wsFlags = 0 as u32_0;
    }
}
unsafe extern "C" fn whereLoopResize(
    mut db: *mut sqlite3,
    mut p: *mut WhereLoop,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut paNew: *mut *mut WhereTerm = ::core::ptr::null_mut::<*mut WhereTerm>();
        if (*p).nLSlot as ::core::ffi::c_int >= n {
            return SQLITE_OK;
        }
        n = n + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int);
        paNew = sqlite3DbMallocRawNN(
            db,
            (::core::mem::size_of::<*mut WhereTerm>() as usize).wrapping_mul(n as usize)
                as u64_0,
        ) as *mut *mut WhereTerm;
        if paNew.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        memcpy(
            paNew as *mut ::core::ffi::c_void,
            (*p).aLTerm as *const ::core::ffi::c_void,
            (::core::mem::size_of::<*mut WhereTerm>() as size_t)
                .wrapping_mul((*p).nLSlot as size_t),
        );
        if (*p).aLTerm != &raw mut (*p).aLTermSpace as *mut *mut WhereTerm {
            sqlite3DbFreeNN(db, (*p).aLTerm as *mut ::core::ffi::c_void);
        }
        (*p).aLTerm = paNew;
        (*p).nLSlot = n as u16_0;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn whereLoopXfer(
    mut db: *mut sqlite3,
    mut pTo: *mut WhereLoop,
    mut pFrom: *mut WhereLoop,
) -> ::core::ffi::c_int {
    unsafe {
        whereLoopClearUnion(db, pTo);
        if (*pFrom).nLTerm as ::core::ffi::c_int > (*pTo).nLSlot as ::core::ffi::c_int
            && whereLoopResize(db, pTo, (*pFrom).nLTerm as ::core::ffi::c_int) != 0
        {
            memset(
                pTo as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                WHERE_LOOP_XFER_SZ as size_t,
            );
            return SQLITE_NOMEM_BKPT;
        }
        memcpy(
            pTo as *mut ::core::ffi::c_void,
            pFrom as *const ::core::ffi::c_void,
            WHERE_LOOP_XFER_SZ as size_t,
        );
        memcpy(
            (*pTo).aLTerm as *mut ::core::ffi::c_void,
            (*pFrom).aLTerm as *const ::core::ffi::c_void,
            ((*pTo).nLTerm as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut WhereTerm>() as size_t),
        );
        if (*pFrom).wsFlags & WHERE_VIRTUALTABLE as u32_0 != 0 {
            (*pFrom).u.vtab.set_needFree(0 as u32_0 as u32_0);
        } else if (*pFrom).wsFlags & WHERE_AUTO_INDEX as u32_0 != 0 as u32_0 {
            (*pFrom).u.btree.pIndex = ::core::ptr::null_mut::<Index>();
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn whereLoopDelete(mut db: *mut sqlite3, mut p: *mut WhereLoop) {
    unsafe {
        whereLoopClear(db, p);
        sqlite3DbNNFreeNN(db, p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn whereInfoFree(mut db: *mut sqlite3, mut pWInfo: *mut WhereInfo) {
    unsafe {
        sqlite3WhereClauseClear(&raw mut (*pWInfo).sWC);
        while !(*pWInfo).pLoops.is_null() {
            let mut p: *mut WhereLoop = (*pWInfo).pLoops;
            (*pWInfo).pLoops = (*p).pNextLoop;
            whereLoopDelete(db, p);
        }
        while !(*pWInfo).pMemToFree.is_null() {
            let mut pNext: *mut WhereMemBlock = (*(*pWInfo).pMemToFree).pNext;
            sqlite3DbNNFreeNN(db, (*pWInfo).pMemToFree as *mut ::core::ffi::c_void);
            (*pWInfo).pMemToFree = pNext;
        }
        sqlite3DbNNFreeNN(db, pWInfo as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn whereLoopCheaperProperSubset(
    mut pX: *const WhereLoop,
    mut pY: *const WhereLoop,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        if (*pX).rRun as ::core::ffi::c_int > (*pY).rRun as ::core::ffi::c_int
            && (*pX).nOut as ::core::ffi::c_int > (*pY).nOut as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if ((*pX).u.btree.nEq as ::core::ffi::c_int)
            < (*pY).u.btree.nEq as ::core::ffi::c_int
            && (*pX).u.btree.pIndex == (*pY).u.btree.pIndex
            && (*pX).nSkip as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*pY).nSkip as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        if (*pX).nLTerm as ::core::ffi::c_int - (*pX).nSkip as ::core::ffi::c_int
            >= (*pY).nLTerm as ::core::ffi::c_int - (*pY).nSkip as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pY).nSkip as ::core::ffi::c_int > (*pX).nSkip as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        i = (*pX).nLTerm as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            if !(*(*pX).aLTerm.offset(i as isize)).is_null() {
                j = (*pY).nLTerm as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
                while j >= 0 as ::core::ffi::c_int {
                    if *(*pY).aLTerm.offset(j as isize)
                        == *(*pX).aLTerm.offset(i as isize)
                    {
                        break;
                    }
                    j -= 1;
                }
                if j < 0 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
            }
            i -= 1;
        }
        if (*pX).wsFlags & WHERE_IDX_ONLY as u32_0 != 0 as u32_0
            && (*pY).wsFlags & WHERE_IDX_ONLY as u32_0 == 0 as u32_0
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn whereLoopAdjustCost(
    mut p: *const WhereLoop,
    mut pTemplate: *mut WhereLoop,
) {
    unsafe {
        if (*pTemplate).wsFlags & WHERE_INDEXED as u32_0 == 0 as u32_0 {
            return;
        }
        while !p.is_null() {
            if !((*p).iTab as ::core::ffi::c_int
                != (*pTemplate).iTab as ::core::ffi::c_int)
            {
                if !((*p).wsFlags & WHERE_INDEXED as u32_0 == 0 as u32_0) {
                    if whereLoopCheaperProperSubset(p, pTemplate) != 0 {
                        (*pTemplate).rRun = (if ((*p).rRun as ::core::ffi::c_int)
                            < (*pTemplate).rRun as ::core::ffi::c_int
                        {
                            (*p).rRun as ::core::ffi::c_int
                        } else {
                            (*pTemplate).rRun as ::core::ffi::c_int
                        }) as LogEst;
                        (*pTemplate).nOut = (if ((*p).nOut as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            < (*pTemplate).nOut as ::core::ffi::c_int
                        {
                            (*p).nOut as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        } else {
                            (*pTemplate).nOut as ::core::ffi::c_int
                        }) as LogEst;
                    } else if whereLoopCheaperProperSubset(pTemplate, p) != 0 {
                        (*pTemplate).rRun = (if (*p).rRun as ::core::ffi::c_int
                            > (*pTemplate).rRun as ::core::ffi::c_int
                        {
                            (*p).rRun as ::core::ffi::c_int
                        } else {
                            (*pTemplate).rRun as ::core::ffi::c_int
                        }) as LogEst;
                        (*pTemplate).nOut = (if (*p).nOut as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            > (*pTemplate).nOut as ::core::ffi::c_int
                        {
                            (*p).nOut as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        } else {
                            (*pTemplate).nOut as ::core::ffi::c_int
                        }) as LogEst;
                    }
                }
            }
            p = (*p).pNextLoop;
        }
    }
}
unsafe extern "C" fn whereLoopFindLesser(
    mut ppPrev: *mut *mut WhereLoop,
    mut pTemplate: *const WhereLoop,
) -> *mut *mut WhereLoop {
    unsafe {
        let mut p: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        p = *ppPrev;
        while !p.is_null() {
            if !((*p).iTab as ::core::ffi::c_int
                != (*pTemplate).iTab as ::core::ffi::c_int
                || (*p).iSortIdx as ::core::ffi::c_int
                    != (*pTemplate).iSortIdx as ::core::ffi::c_int)
            {
                if (*p).wsFlags & WHERE_AUTO_INDEX as u32_0 != 0 as u32_0
                    && (*pTemplate).nSkip as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    && (*pTemplate).wsFlags & WHERE_INDEXED as u32_0 != 0 as u32_0
                    && (*pTemplate).wsFlags & WHERE_COLUMN_EQ as u32_0 != 0 as u32_0
                    && (*p).prereq & (*pTemplate).prereq == (*pTemplate).prereq
                {
                    break;
                }
                if (*p).prereq & (*pTemplate).prereq == (*p).prereq
                    && (*p).rSetup as ::core::ffi::c_int
                        <= (*pTemplate).rSetup as ::core::ffi::c_int
                    && (*p).rRun as ::core::ffi::c_int
                        <= (*pTemplate).rRun as ::core::ffi::c_int
                    && (*p).nOut as ::core::ffi::c_int
                        <= (*pTemplate).nOut as ::core::ffi::c_int
                {
                    return ::core::ptr::null_mut::<*mut WhereLoop>();
                }
                if (*p).prereq & (*pTemplate).prereq == (*pTemplate).prereq
                    && (*p).rRun as ::core::ffi::c_int
                        >= (*pTemplate).rRun as ::core::ffi::c_int
                    && (*p).nOut as ::core::ffi::c_int
                        >= (*pTemplate).nOut as ::core::ffi::c_int
                {
                    break;
                }
            }
            ppPrev = &raw mut (*p).pNextLoop;
            p = *ppPrev;
        }
        return ppPrev;
    }
}
unsafe extern "C" fn whereLoopInsert(
    mut pBuilder: *mut WhereLoopBuilder,
    mut pTemplate: *mut WhereLoop,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ppPrev: *mut *mut WhereLoop = ::core::ptr::null_mut::<*mut WhereLoop>();
        let mut p: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut pWInfo: *mut WhereInfo = (*pBuilder).pWInfo;
        let mut db: *mut sqlite3 = (*(*pWInfo).pParse).db;
        let mut rc: ::core::ffi::c_int = 0;
        if (*pBuilder).iPlanLimit == 0 as ::core::ffi::c_uint {
            if !(*pBuilder).pOrSet.is_null() {
                (*(*pBuilder).pOrSet).n = 0 as u16_0;
            }
            return SQLITE_DONE;
        }
        (*pBuilder).iPlanLimit = (*pBuilder).iPlanLimit.wrapping_sub(1);
        whereLoopAdjustCost((*pWInfo).pLoops, pTemplate);
        if !(*pBuilder).pOrSet.is_null() {
            if (*pTemplate).nLTerm != 0 {
                whereOrInsert(
                    (*pBuilder).pOrSet,
                    (*pTemplate).prereq,
                    (*pTemplate).rRun,
                    (*pTemplate).nOut,
                );
            }
            return SQLITE_OK;
        }
        ppPrev = whereLoopFindLesser(&raw mut (*pWInfo).pLoops, pTemplate);
        if ppPrev.is_null() {
            return SQLITE_OK
        } else {
            p = *ppPrev;
        }
        if p.is_null() {
            p = sqlite3DbMallocRawNN(db, ::core::mem::size_of::<WhereLoop>() as u64_0)
                as *mut WhereLoop;
            *ppPrev = p;
            if p.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            whereLoopInit(p);
            (*p).pNextLoop = ::core::ptr::null_mut::<WhereLoop>();
        } else {
            let mut ppTail: *mut *mut WhereLoop = &raw mut (*p).pNextLoop;
            let mut pToDel: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
            while !(*ppTail).is_null() {
                ppTail = whereLoopFindLesser(ppTail, pTemplate);
                if ppTail.is_null() {
                    break;
                }
                pToDel = *ppTail;
                if pToDel.is_null() {
                    break;
                }
                *ppTail = (*pToDel).pNextLoop;
                whereLoopDelete(db, pToDel);
            }
        }
        rc = whereLoopXfer(db, p, pTemplate);
        if (*p).wsFlags & WHERE_VIRTUALTABLE as u32_0 == 0 as u32_0 {
            let mut pIndex: *mut Index = (*p).u.btree.pIndex;
            if !pIndex.is_null()
                && (*pIndex).idxType() as ::core::ffi::c_int == SQLITE_IDXTYPE_IPK
            {
                (*p).u.btree.pIndex = ::core::ptr::null_mut::<Index>();
            }
        }
        return rc;
    }
}
unsafe extern "C" fn whereLoopOutputAdjust(
    mut pWC: *mut WhereClause,
    mut pLoop: *mut WhereLoop,
    mut nRow: LogEst,
) {
    unsafe {
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut pX: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut notAllowed: Bitmask = !((*pLoop).prereq | (*pLoop).maskSelf);
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut iReduce: LogEst = 0 as LogEst;
        i = (*pWC).nBase;
        pTerm = (*pWC).a;
        while i > 0 as ::core::ffi::c_int {
            if !((*pTerm).prereqAll & notAllowed != 0 as ::core::ffi::c_ulonglong) {
                if !((*pTerm).prereqAll & (*pLoop).maskSelf
                    == 0 as ::core::ffi::c_ulonglong)
                {
                    if !((*pTerm).wtFlags as ::core::ffi::c_int & TERM_VIRTUAL
                        != 0 as ::core::ffi::c_int)
                    {
                        j = (*pLoop).nLTerm as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int;
                        while j >= 0 as ::core::ffi::c_int {
                            pX = *(*pLoop).aLTerm.offset(j as isize);
                            if !pX.is_null() {
                                if pX == pTerm {
                                    break;
                                }
                                if (*pX).iParent >= 0 as ::core::ffi::c_int
                                    && (*pWC).a.offset((*pX).iParent as isize) as *mut WhereTerm
                                        == pTerm
                                {
                                    break;
                                }
                            }
                            j -= 1;
                        }
                        if j < 0 as ::core::ffi::c_int {
                            sqlite3ProgressCheck((*(*pWC).pWInfo).pParse);
                            if (*pLoop).maskSelf == (*pTerm).prereqAll {
                                if (*pTerm).eOperator as ::core::ffi::c_int
                                    & 0x3f as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                    || (*(&raw mut (*(*(*pWC).pWInfo).pTabList).a
                                        as *mut SrcItem)
                                        .offset((*pLoop).iTab as isize))
                                        .fg
                                        .jointype as ::core::ffi::c_int & (JT_LEFT | JT_LTORJ)
                                        == 0 as ::core::ffi::c_int
                                {
                                    (*pLoop).wsFlags = ((*pLoop).wsFlags as ::core::ffi::c_uint
                                        | WHERE_SELFCULL as ::core::ffi::c_uint) as u32_0;
                                }
                            }
                            if (*pTerm).truthProb as ::core::ffi::c_int
                                <= 0 as ::core::ffi::c_int
                            {
                                (*pLoop).nOut = ((*pLoop).nOut as ::core::ffi::c_int
                                    + (*pTerm).truthProb as ::core::ffi::c_int) as LogEst;
                            } else {
                                (*pLoop).nOut -= 1;
                                if (*pTerm).eOperator as ::core::ffi::c_int
                                    & (WO_EQ | WO_IS) != 0 as ::core::ffi::c_int
                                    && (*pTerm).wtFlags as ::core::ffi::c_int & TERM_HIGHTRUTH
                                        == 0 as ::core::ffi::c_int
                                {
                                    let mut pRight: *mut Expr = (*(*pTerm).pExpr).pRight;
                                    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    if sqlite3ExprIsInteger(
                                        pRight,
                                        &raw mut k,
                                        ::core::ptr::null_mut::<Parse>(),
                                    ) != 0 && k >= -1 as ::core::ffi::c_int
                                        && k <= 1 as ::core::ffi::c_int
                                    {
                                        k = 10 as ::core::ffi::c_int;
                                    } else {
                                        k = 20 as ::core::ffi::c_int;
                                    }
                                    if (iReduce as ::core::ffi::c_int) < k {
                                        (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int
                                            | TERM_HEURTRUTH) as u16_0;
                                        iReduce = k as LogEst;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            i -= 1;
            pTerm = pTerm.offset(1);
        }
        if (*pLoop).nOut as ::core::ffi::c_int
            > nRow as ::core::ffi::c_int - iReduce as ::core::ffi::c_int
        {
            (*pLoop).nOut = (nRow as ::core::ffi::c_int - iReduce as ::core::ffi::c_int)
                as LogEst;
        }
    }
}
unsafe extern "C" fn whereRangeVectorLen(
    mut pParse: *mut Parse,
    mut iCur: ::core::ffi::c_int,
    mut pIdx: *mut Index,
    mut nEq: ::core::ffi::c_int,
    mut pTerm: *mut WhereTerm,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nCmp: ::core::ffi::c_int = sqlite3ExprVectorSize(
            (*(*pTerm).pExpr).pLeft,
        );
        let mut i: ::core::ffi::c_int = 0;
        nCmp = if nCmp < (*pIdx).nColumn as ::core::ffi::c_int - nEq {
            nCmp
        } else {
            (*pIdx).nColumn as ::core::ffi::c_int - nEq
        };
        i = 1 as ::core::ffi::c_int;
        while i < nCmp {
            let mut aff: ::core::ffi::c_char = 0;
            let mut idxaff: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
            let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
            let mut pLhs: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pRhs: *mut Expr = ::core::ptr::null_mut::<Expr>();
            pLhs = (*(&raw mut (*(*(*(*pTerm).pExpr).pLeft).x.pList).a
                as *mut ExprList_item)
                .offset(i as isize))
                .pExpr;
            pRhs = (*(*pTerm).pExpr).pRight;
            if (*pRhs).flags & EP_xIsSelect as u32_0 != 0 as u32_0 {
                pRhs = (*(&raw mut (*(*(*pRhs).x.pSelect).pEList).a
                    as *mut ExprList_item)
                    .offset(i as isize))
                    .pExpr;
            } else {
                pRhs = (*(&raw mut (*(*pRhs).x.pList).a as *mut ExprList_item)
                    .offset(i as isize))
                    .pExpr;
            }
            if (*pLhs).op as ::core::ffi::c_int != TK_COLUMN || (*pLhs).iTable != iCur
                || (*pLhs).iColumn as ::core::ffi::c_int
                    != *(*pIdx).aiColumn.offset((i + nEq) as isize) as ::core::ffi::c_int
                || *(*pIdx).aSortOrder.offset((i + nEq) as isize) as ::core::ffi::c_int
                    != *(*pIdx).aSortOrder.offset(nEq as isize) as ::core::ffi::c_int
            {
                break;
            }
            aff = sqlite3CompareAffinity(pRhs, sqlite3ExprAffinity(pLhs));
            idxaff = sqlite3TableColumnAffinity(
                (*pIdx).pTable,
                (*pLhs).iColumn as ::core::ffi::c_int,
            );
            if aff as ::core::ffi::c_int != idxaff as ::core::ffi::c_int {
                break;
            }
            pColl = sqlite3BinaryCompareCollSeq(pParse, pLhs, pRhs);
            if pColl.is_null() {
                break;
            }
            if sqlite3StrICmp((*pColl).zName, *(*pIdx).azColl.offset((i + nEq) as isize))
                != 0
            {
                break;
            }
            i += 1;
        }
        return i;
    }
}
unsafe extern "C" fn whereLoopAddBtreeIndex(
    mut pBuilder: *mut WhereLoopBuilder,
    mut pSrc: *mut SrcItem,
    mut pProbe: *mut Index,
    mut nInMul: LogEst,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pWInfo: *mut WhereInfo = (*pBuilder).pWInfo;
        let mut pParse: *mut Parse = (*pWInfo).pParse;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pNew: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut opMask: ::core::ffi::c_int = 0;
        let mut scan: WhereScan = WhereScan {
            pOrigWC: ::core::ptr::null_mut::<WhereClause>(),
            pWC: ::core::ptr::null_mut::<WhereClause>(),
            zCollName: ::core::ptr::null::<::core::ffi::c_char>(),
            pIdxExpr: ::core::ptr::null_mut::<Expr>(),
            k: 0,
            opMask: 0,
            idxaff: 0,
            iEquiv: 0,
            nEquiv: 0,
            aiCur: [0; 11],
            aiColumn: [0; 11]

        };
        let mut saved_prereq: Bitmask = 0;
        let mut saved_nLTerm: u16_0 = 0;
        let mut saved_nEq: u16_0 = 0;
        let mut saved_nBtm: u16_0 = 0;
        let mut saved_nTop: u16_0 = 0;
        let mut saved_nSkip: u16_0 = 0;
        let mut saved_wsFlags: u32_0 = 0;
        let mut saved_nOut: LogEst = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut rSize: LogEst = 0;
        let mut rLogSize: LogEst = 0;
        let mut pTop: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut pBtm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        pNew = (*pBuilder).pNew;
        if (*pParse).nErr != 0 {
            return (*pParse).rc;
        }
        if (*pNew).wsFlags & WHERE_BTM_LIMIT as u32_0 != 0 {
            opMask = WO_LT | WO_LE;
        } else {
            opMask = WO_EQ | WO_IN | WO_GT | WO_GE | WO_LT | WO_LE | WO_ISNULL | WO_IS;
        }
        if (*pProbe).bUnordered() != 0 {
            opMask &= !(WO_GT | WO_GE | WO_LT | WO_LE);
        }
        saved_nEq = (*pNew).u.btree.nEq;
        saved_nBtm = (*pNew).u.btree.nBtm;
        saved_nTop = (*pNew).u.btree.nTop;
        saved_nSkip = (*pNew).nSkip;
        saved_nLTerm = (*pNew).nLTerm;
        saved_wsFlags = (*pNew).wsFlags;
        saved_prereq = (*pNew).prereq;
        saved_nOut = (*pNew).nOut;
        pTerm = whereScanInit(
            &raw mut scan,
            (*pBuilder).pWC,
            (*pSrc).iCursor,
            saved_nEq as ::core::ffi::c_int,
            opMask as u32_0,
            pProbe,
        );
        (*pNew).rSetup = 0 as LogEst;
        rSize = *(*pProbe).aiRowLogEst.offset(0 as ::core::ffi::c_int as isize);
        rLogSize = estLog(rSize);
        let mut c2rust_current_block_151: u64;
        while rc == SQLITE_OK && !pTerm.is_null() {
            let mut eOp: u16_0 = (*pTerm).eOperator;
            let mut rCostIdx: LogEst = 0;
            let mut nOutUnadjusted: LogEst = 0;
            let mut nIn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if !((eOp as ::core::ffi::c_int == WO_ISNULL
                || (*pTerm).wtFlags as ::core::ffi::c_int & TERM_VNULL
                    != 0 as ::core::ffi::c_int)
                && indexColumnNotNull(pProbe, saved_nEq as ::core::ffi::c_int) != 0)
            {
                if !((*pTerm).prereqRight & (*pNew).maskSelf != 0) {
                    if !((*pTerm).wtFlags as ::core::ffi::c_int & TERM_LIKEOPT != 0
                        && (*pTerm).eOperator as ::core::ffi::c_int == WO_LT)
                    {
                        if !((*pSrc).fg.jointype as ::core::ffi::c_int
                            & (JT_LEFT | JT_LTORJ | JT_RIGHT) != 0 as ::core::ffi::c_int
                            && constraintCompatibleWithOuterJoin(pTerm, pSrc) == 0)
                        {
                            if (*pProbe).onError as ::core::ffi::c_int != OE_None
                                && saved_nEq as ::core::ffi::c_int
                                    == (*pProbe).nKeyCol as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int
                            {
                                (*pBuilder).bldFlags1 = ((*pBuilder).bldFlags1
                                    as ::core::ffi::c_int | SQLITE_BLDF1_UNIQUE)
                                    as ::core::ffi::c_uchar;
                            } else {
                                (*pBuilder).bldFlags1 = ((*pBuilder).bldFlags1
                                    as ::core::ffi::c_int | SQLITE_BLDF1_INDEXED)
                                    as ::core::ffi::c_uchar;
                            }
                            (*pNew).wsFlags = saved_wsFlags;
                            (*pNew).u.btree.nEq = saved_nEq;
                            (*pNew).u.btree.nBtm = saved_nBtm;
                            (*pNew).u.btree.nTop = saved_nTop;
                            (*pNew).nLTerm = saved_nLTerm;
                            if (*pNew).nLTerm as ::core::ffi::c_int
                                >= (*pNew).nLSlot as ::core::ffi::c_int
                                && whereLoopResize(
                                    db,
                                    pNew,
                                    (*pNew).nLTerm as ::core::ffi::c_int
                                        + 1 as ::core::ffi::c_int,
                                ) != 0
                            {
                                break;
                            }
                            let c2rust_fresh15 = (*pNew).nLTerm;
                            (*pNew).nLTerm = (*pNew).nLTerm.wrapping_add(1);
                            let ref mut c2rust_fresh16 = *(*pNew)
                                .aLTerm
                                .offset(c2rust_fresh15 as isize);
                            *c2rust_fresh16 = pTerm;
                            (*pNew).prereq = (saved_prereq | (*pTerm).prereqRight)
                                & !(*pNew).maskSelf;
                            if eOp as ::core::ffi::c_int & WO_IN != 0 {
                                let mut pExpr: *mut Expr = (*pTerm).pExpr;
                                if (*pExpr).flags & EP_xIsSelect as u32_0 != 0 as u32_0 {
                                    let mut i: ::core::ffi::c_int = 0;
                                    let mut bRedundant: ::core::ffi::c_int = 0
                                        as ::core::ffi::c_int;
                                    nIn = 46 as ::core::ffi::c_int;
                                    i = 0 as ::core::ffi::c_int;
                                    while i
                                        < (*pNew).nLTerm as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int
                                    {
                                        if !(*(*pNew).aLTerm.offset(i as isize)).is_null()
                                            && (**(*pNew).aLTerm.offset(i as isize)).pExpr == pExpr
                                        {
                                            nIn = 0 as ::core::ffi::c_int;
                                            if (**(*pNew).aLTerm.offset(i as isize)).u.x.iField
                                                == (*pTerm).u.x.iField
                                            {
                                                bRedundant = 1 as ::core::ffi::c_int;
                                            }
                                        }
                                        i += 1;
                                    }
                                    if bRedundant != 0 {
                                        (*pNew).nLTerm = (*pNew).nLTerm.wrapping_sub(1);
                                        c2rust_current_block_151 = 1608152415753874203;
                                    } else {
                                        c2rust_current_block_151 = 4888910987971495881;
                                    }
                                } else {
                                    if !(*pExpr).x.pList.is_null()
                                        && (*(*pExpr).x.pList).nExpr != 0
                                    {
                                        nIn = sqlite3LogEst((*(*pExpr).x.pList).nExpr as u64_0)
                                            as ::core::ffi::c_int;
                                    }
                                    c2rust_current_block_151 = 4888910987971495881;
                                }
                                match c2rust_current_block_151 {
                                    1608152415753874203 => {}
                                    _ => {
                                        if (*pProbe).hasStat1() as ::core::ffi::c_int != 0
                                            && rLogSize as ::core::ffi::c_int
                                                >= 10 as ::core::ffi::c_int
                                        {
                                            let mut M: LogEst = 0;
                                            let mut logK: LogEst = 0;
                                            let mut x: LogEst = 0;
                                            M = *(*pProbe).aiRowLogEst.offset(saved_nEq as isize);
                                            logK = estLog(nIn as LogEst);
                                            x = (M as ::core::ffi::c_int + logK as ::core::ffi::c_int
                                                + 10 as ::core::ffi::c_int
                                                - (nIn + rLogSize as ::core::ffi::c_int)) as LogEst;
                                            if x as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                                                c2rust_current_block_151 = 16029476503615101993;
                                            } else if (nInMul as ::core::ffi::c_int)
                                                < 2 as ::core::ffi::c_int
                                                && (*db).dbOptFlags & 0x20000 as u32_0 == 0 as u32_0
                                            {
                                                (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                                                    | WHERE_IN_SEEKSCAN as ::core::ffi::c_uint) as u32_0;
                                                c2rust_current_block_151 = 16029476503615101993;
                                            } else {
                                                c2rust_current_block_151 = 1608152415753874203;
                                            }
                                        } else {
                                            c2rust_current_block_151 = 16029476503615101993;
                                        }
                                        match c2rust_current_block_151 {
                                            1608152415753874203 => {}
                                            _ => {
                                                (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                                                    | WHERE_COLUMN_IN as ::core::ffi::c_uint) as u32_0;
                                                c2rust_current_block_151 = 8140372313878014523;
                                            }
                                        }
                                    }
                                }
                            } else {
                                if eOp as ::core::ffi::c_int & (WO_EQ | WO_IS) != 0 {
                                    let mut iCol: ::core::ffi::c_int = *(*pProbe)
                                        .aiColumn
                                        .offset(saved_nEq as isize) as ::core::ffi::c_int;
                                    (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                                        | WHERE_COLUMN_EQ as ::core::ffi::c_uint) as u32_0;
                                    if iCol == XN_ROWID
                                        || iCol >= 0 as ::core::ffi::c_int
                                            && nInMul as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                            && saved_nEq as ::core::ffi::c_int
                                                == (*pProbe).nKeyCol as ::core::ffi::c_int
                                                    - 1 as ::core::ffi::c_int
                                    {
                                        if iCol == XN_ROWID
                                            || (*pProbe).uniqNotNull() as ::core::ffi::c_int != 0
                                            || (*pProbe).nKeyCol as ::core::ffi::c_int
                                                == 1 as ::core::ffi::c_int
                                                && (*pProbe).onError as ::core::ffi::c_int != 0
                                                && eOp as ::core::ffi::c_int & WO_EQ != 0
                                        {
                                            (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                                                | WHERE_ONEROW as ::core::ffi::c_uint) as u32_0;
                                        } else {
                                            (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                                                | WHERE_UNQ_WANTED as ::core::ffi::c_uint) as u32_0;
                                        }
                                    }
                                    if scan.iEquiv as ::core::ffi::c_int
                                        > 1 as ::core::ffi::c_int
                                    {
                                        (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                                            | WHERE_TRANSCONS as ::core::ffi::c_uint) as u32_0;
                                    }
                                } else if eOp as ::core::ffi::c_int & WO_ISNULL != 0 {
                                    (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                                        | WHERE_COLUMN_NULL as ::core::ffi::c_uint) as u32_0;
                                } else {
                                    let mut nVecLen: ::core::ffi::c_int = whereRangeVectorLen(
                                        pParse,
                                        (*pSrc).iCursor,
                                        pProbe,
                                        saved_nEq as ::core::ffi::c_int,
                                        pTerm,
                                    );
                                    if eOp as ::core::ffi::c_int & (WO_GT | WO_GE) != 0 {
                                        (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                                            | (WHERE_COLUMN_RANGE | WHERE_BTM_LIMIT)
                                                as ::core::ffi::c_uint) as u32_0;
                                        (*pNew).u.btree.nBtm = nVecLen as u16_0;
                                        pBtm = pTerm;
                                        pTop = ::core::ptr::null_mut::<WhereTerm>();
                                        if (*pTerm).wtFlags as ::core::ffi::c_int & TERM_LIKEOPT
                                            != 0
                                        {
                                            pTop = pTerm.offset(1 as ::core::ffi::c_int as isize)
                                                as *mut WhereTerm;
                                            if whereLoopResize(
                                                db,
                                                pNew,
                                                (*pNew).nLTerm as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int,
                                            ) != 0
                                            {
                                                break;
                                            }
                                            let c2rust_fresh17 = (*pNew).nLTerm;
                                            (*pNew).nLTerm = (*pNew).nLTerm.wrapping_add(1);
                                            let ref mut c2rust_fresh18 = *(*pNew)
                                                .aLTerm
                                                .offset(c2rust_fresh17 as isize);
                                            *c2rust_fresh18 = pTop;
                                            (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                                                | WHERE_TOP_LIMIT as ::core::ffi::c_uint) as u32_0;
                                            (*pNew).u.btree.nTop = 1 as u16_0;
                                        }
                                    } else {
                                        (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                                            | (WHERE_COLUMN_RANGE | WHERE_TOP_LIMIT)
                                                as ::core::ffi::c_uint) as u32_0;
                                        (*pNew).u.btree.nTop = nVecLen as u16_0;
                                        pTop = pTerm;
                                        pBtm = if (*pNew).wsFlags & WHERE_BTM_LIMIT as u32_0
                                            != 0 as u32_0
                                        {
                                            *(*pNew)
                                                .aLTerm
                                                .offset(
                                                    ((*pNew).nLTerm as ::core::ffi::c_int
                                                        - 2 as ::core::ffi::c_int) as isize,
                                                )
                                        } else {
                                            ::core::ptr::null_mut::<WhereTerm>()
                                        };
                                    }
                                }
                                c2rust_current_block_151 = 8140372313878014523;
                            }
                            match c2rust_current_block_151 {
                                1608152415753874203 => {}
                                _ => {
                                    if (*pNew).wsFlags & WHERE_COLUMN_RANGE as u32_0 != 0 {
                                        whereRangeScanEst(pParse, pBuilder, pBtm, pTop, pNew);
                                    } else {
                                        (*pNew).u.btree.nEq = (*pNew).u.btree.nEq.wrapping_add(1);
                                        let mut nEq: ::core::ffi::c_int = (*pNew).u.btree.nEq
                                            as ::core::ffi::c_int;
                                        if (*pTerm).truthProb as ::core::ffi::c_int
                                            <= 0 as ::core::ffi::c_int
                                            && *(*pProbe).aiColumn.offset(saved_nEq as isize)
                                                as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                                        {
                                            (*pNew).nOut = ((*pNew).nOut as ::core::ffi::c_int
                                                + (*pTerm).truthProb as ::core::ffi::c_int) as LogEst;
                                            (*pNew).nOut = ((*pNew).nOut as ::core::ffi::c_int - nIn)
                                                as LogEst;
                                        } else {
                                            (*pNew).nOut = ((*pNew).nOut as ::core::ffi::c_int
                                                + (*(*pProbe).aiRowLogEst.offset(nEq as isize)
                                                    as ::core::ffi::c_int
                                                    - *(*pProbe)
                                                        .aiRowLogEst
                                                        .offset((nEq - 1 as ::core::ffi::c_int) as isize)
                                                        as ::core::ffi::c_int)) as LogEst;
                                            if eOp as ::core::ffi::c_int & WO_ISNULL != 0 {
                                                (*pNew).nOut = ((*pNew).nOut as ::core::ffi::c_int
                                                    + 10 as ::core::ffi::c_int) as LogEst;
                                            }
                                        }
                                    }
                                    if (*pProbe).idxType() as ::core::ffi::c_int
                                        == SQLITE_IDXTYPE_IPK
                                    {
                                        rCostIdx = ((*pNew).nOut as ::core::ffi::c_int
                                            + 16 as ::core::ffi::c_int) as LogEst;
                                    } else {
                                        rCostIdx = ((*pNew).nOut as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int
                                            + 15 as ::core::ffi::c_int
                                                * (*pProbe).szIdxRow as ::core::ffi::c_int
                                                / (*(*pSrc).pSTab).szTabRow as ::core::ffi::c_int)
                                            as LogEst;
                                    }
                                    rCostIdx = sqlite3LogEstAdd(rLogSize, rCostIdx);
                                    (*pNew).rRun = rCostIdx;
                                    if (*pNew).wsFlags
                                        & (WHERE_IDX_ONLY | WHERE_IPK | WHERE_EXPRIDX) as u32_0
                                        == 0 as u32_0
                                    {
                                        (*pNew).rRun = sqlite3LogEstAdd(
                                            (*pNew).rRun,
                                            ((*pNew).nOut as ::core::ffi::c_int
                                                + 16 as ::core::ffi::c_int) as LogEst,
                                        );
                                    }
                                    nOutUnadjusted = (*pNew).nOut;
                                    (*pNew).rRun = ((*pNew).rRun as ::core::ffi::c_int
                                        + (nInMul as ::core::ffi::c_int + nIn)) as LogEst;
                                    (*pNew).nOut = ((*pNew).nOut as ::core::ffi::c_int
                                        + (nInMul as ::core::ffi::c_int + nIn)) as LogEst;
                                    whereLoopOutputAdjust((*pBuilder).pWC, pNew, rSize);
                                    rc = whereLoopInsert(pBuilder, pNew);
                                    if (*pNew).wsFlags & WHERE_COLUMN_RANGE as u32_0 != 0 {
                                        (*pNew).nOut = saved_nOut;
                                    } else {
                                        (*pNew).nOut = nOutUnadjusted;
                                    }
                                    if (*pNew).wsFlags & WHERE_TOP_LIMIT as u32_0 == 0 as u32_0
                                        && ((*pNew).u.btree.nEq as ::core::ffi::c_int)
                                            < (*pProbe).nColumn as ::core::ffi::c_int
                                        && (((*pNew).u.btree.nEq as ::core::ffi::c_int)
                                            < (*pProbe).nKeyCol as ::core::ffi::c_int
                                            || (*pProbe).idxType() as ::core::ffi::c_int
                                                != SQLITE_IDXTYPE_PRIMARYKEY)
                                    {
                                        if (*pNew).u.btree.nEq as ::core::ffi::c_int
                                            > 3 as ::core::ffi::c_int
                                        {
                                            sqlite3ProgressCheck(pParse);
                                        }
                                        whereLoopAddBtreeIndex(
                                            pBuilder,
                                            pSrc,
                                            pProbe,
                                            (nInMul as ::core::ffi::c_int + nIn) as LogEst,
                                        );
                                    }
                                    (*pNew).nOut = saved_nOut;
                                }
                            }
                        }
                    }
                }
            }
            pTerm = whereScanNext(&raw mut scan);
        }
        (*pNew).prereq = saved_prereq;
        (*pNew).u.btree.nEq = saved_nEq;
        (*pNew).u.btree.nBtm = saved_nBtm;
        (*pNew).u.btree.nTop = saved_nTop;
        (*pNew).nSkip = saved_nSkip;
        (*pNew).wsFlags = saved_wsFlags;
        (*pNew).nOut = saved_nOut;
        (*pNew).nLTerm = saved_nLTerm;
        if saved_nEq as ::core::ffi::c_int == saved_nSkip as ::core::ffi::c_int
            && (saved_nEq as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                < (*pProbe).nKeyCol as ::core::ffi::c_int
            && saved_nEq as ::core::ffi::c_int == (*pNew).nLTerm as ::core::ffi::c_int
            && (*pProbe).noSkipScan() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*pProbe).hasStat1() as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            && (*db).dbOptFlags & 0x4000 as u32_0 == 0 as u32_0
            && *(*pProbe)
                .aiRowLogEst
                .offset(
                    (saved_nEq as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int >= 42 as ::core::ffi::c_int
            && (*pSrc).fg.fromExists() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && {
                rc = whereLoopResize(
                    db,
                    pNew,
                    (*pNew).nLTerm as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                );
                rc == SQLITE_OK
            }
        {
            let mut nIter: LogEst = 0;
            (*pNew).u.btree.nEq = (*pNew).u.btree.nEq.wrapping_add(1);
            (*pNew).nSkip = (*pNew).nSkip.wrapping_add(1);
            let c2rust_fresh19 = (*pNew).nLTerm;
            (*pNew).nLTerm = (*pNew).nLTerm.wrapping_add(1);
            let ref mut c2rust_fresh20 = *(*pNew).aLTerm.offset(c2rust_fresh19 as isize);
            *c2rust_fresh20 = ::core::ptr::null_mut::<WhereTerm>();
            (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                | WHERE_SKIPSCAN as ::core::ffi::c_uint) as u32_0;
            nIter = (*(*pProbe).aiRowLogEst.offset(saved_nEq as isize)
                as ::core::ffi::c_int
                - *(*pProbe)
                    .aiRowLogEst
                    .offset(
                        (saved_nEq as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int) as LogEst;
            (*pNew).nOut = ((*pNew).nOut as ::core::ffi::c_int
                - nIter as ::core::ffi::c_int) as LogEst;
            nIter = (nIter as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as LogEst;
            whereLoopAddBtreeIndex(
                pBuilder,
                pSrc,
                pProbe,
                (nIter as ::core::ffi::c_int + nInMul as ::core::ffi::c_int) as LogEst,
            );
            (*pNew).nOut = saved_nOut;
            (*pNew).u.btree.nEq = saved_nEq;
            (*pNew).nSkip = saved_nSkip;
            (*pNew).wsFlags = saved_wsFlags;
        }
        return rc;
    }
}
unsafe extern "C" fn indexMightHelpWithOrderBy(
    mut pBuilder: *mut WhereLoopBuilder,
    mut pIndex: *mut Index,
    mut iCursor: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pOB: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut aColExpr: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut ii: ::core::ffi::c_int = 0;
        let mut jj: ::core::ffi::c_int = 0;
        if (*pIndex).bUnordered() != 0 {
            return 0 as ::core::ffi::c_int;
        }
        pOB = (*(*pBuilder).pWInfo).pOrderBy;
        if pOB.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pOB).nExpr {
            let mut pExpr: *mut Expr = sqlite3ExprSkipCollateAndLikely(
                (*(&raw mut (*pOB).a as *mut ExprList_item).offset(ii as isize)).pExpr,
            );
            if !pExpr.is_null() {
                if ((*pExpr).op as ::core::ffi::c_int == TK_COLUMN
                    || (*pExpr).op as ::core::ffi::c_int == TK_AGG_COLUMN)
                    && (*pExpr).iTable == iCursor
                {
                    if ((*pExpr).iColumn as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    {
                        return 1 as ::core::ffi::c_int;
                    }
                    jj = 0 as ::core::ffi::c_int;
                    while jj < (*pIndex).nKeyCol as ::core::ffi::c_int {
                        if (*pExpr).iColumn as ::core::ffi::c_int
                            == *(*pIndex).aiColumn.offset(jj as isize)
                                as ::core::ffi::c_int
                        {
                            return 1 as ::core::ffi::c_int;
                        }
                        jj += 1;
                    }
                } else {
                    aColExpr = (*pIndex).aColExpr;
                    if !aColExpr.is_null() {
                        jj = 0 as ::core::ffi::c_int;
                        while jj < (*pIndex).nKeyCol as ::core::ffi::c_int {
                            if !(*(*pIndex).aiColumn.offset(jj as isize)
                                as ::core::ffi::c_int != XN_EXPR)
                            {
                                if sqlite3ExprCompareSkip(
                                    pExpr,
                                    (*(&raw mut (*aColExpr).a as *mut ExprList_item)
                                        .offset(jj as isize))
                                        .pExpr,
                                    iCursor,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    return 1 as ::core::ffi::c_int;
                                }
                            }
                            jj += 1;
                        }
                    }
                }
            }
            ii += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn whereUsablePartialIndex(
    mut iTab: ::core::ffi::c_int,
    mut jointype: u8_0,
    mut pWC: *mut WhereClause,
    mut pWhere: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut pParse: *mut Parse = ::core::ptr::null_mut::<Parse>();
        if jointype as ::core::ffi::c_int & JT_LTORJ != 0 {
            return 0 as ::core::ffi::c_int;
        }
        pParse = (*(*pWC).pWInfo).pParse;
        while (*pWhere).op as ::core::ffi::c_int == TK_AND {
            if whereUsablePartialIndex(iTab, jointype, pWC, (*pWhere).pLeft) == 0 {
                return 0 as ::core::ffi::c_int;
            }
            pWhere = (*pWhere).pRight;
        }
        i = 0 as ::core::ffi::c_int;
        pTerm = (*pWC).a;
        while i < (*pWC).nTerm {
            let mut pExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
            pExpr = (*pTerm).pExpr;
            if (!((*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0 != 0 as u32_0)
                || (*pExpr).w.iJoin == iTab)
                && (jointype as ::core::ffi::c_int & JT_OUTER == 0 as ::core::ffi::c_int
                    || (*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0 != 0 as u32_0)
                && sqlite3ExprImpliesExpr(pParse, pExpr, pWhere, iTab) != 0
                && sqlite3ExprImpliesExpr(
                    pParse,
                    pExpr,
                    pWhere,
                    -1 as ::core::ffi::c_int,
                ) == 0
                && (*pTerm).wtFlags as ::core::ffi::c_int & TERM_VNULL
                    == 0 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
            pTerm = pTerm.offset(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn exprIsCoveredByIndex(
    mut pExpr: *const Expr,
    mut pIdx: *const Index,
    mut iTabCur: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdx).nColumn as ::core::ffi::c_int {
            if *(*pIdx).aiColumn.offset(i as isize) as ::core::ffi::c_int == XN_EXPR
                && sqlite3ExprCompare(
                    ::core::ptr::null::<Parse>(),
                    pExpr,
                    (*(&raw mut (*(*pIdx).aColExpr).a as *mut ExprList_item)
                        .offset(i as isize))
                        .pExpr,
                    iTabCur,
                ) == 0 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn whereIsCoveringIndexWalkCallback(
    mut pWalk: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pIdx: *const Index = ::core::ptr::null::<Index>();
        let mut aiColumn: *const i16_0 = ::core::ptr::null::<i16_0>();
        let mut nColumn: u16_0 = 0;
        let mut pCk: *mut CoveringIndexCheck = ::core::ptr::null_mut::<
            CoveringIndexCheck,
        >();
        pCk = (*pWalk).u.pCovIdxCk as *mut CoveringIndexCheck;
        pIdx = (*pCk).pIdx;
        if (*pExpr).op as ::core::ffi::c_int == TK_COLUMN
            || (*pExpr).op as ::core::ffi::c_int == TK_AGG_COLUMN
        {
            if (*pExpr).iTable != (*pCk).iTabCur {
                return WRC_Continue;
            }
            pIdx = (*(*pWalk).u.pCovIdxCk).pIdx;
            aiColumn = (*pIdx).aiColumn;
            nColumn = (*pIdx).nColumn;
            i = 0 as ::core::ffi::c_int;
            while i < nColumn as ::core::ffi::c_int {
                if *aiColumn.offset(i as isize) as ::core::ffi::c_int
                    == (*pExpr).iColumn as ::core::ffi::c_int
                {
                    return WRC_Continue;
                }
                i += 1;
            }
            (*pCk).bUnidx = 1 as u8_0;
            return WRC_Abort;
        } else if (*pIdx).bHasExpr() as ::core::ffi::c_int != 0
            && exprIsCoveredByIndex(pExpr, pIdx, (*(*pWalk).u.pCovIdxCk).iTabCur) != 0
        {
            (*pCk).bExpr = 1 as u8_0;
            return WRC_Prune;
        }
        return WRC_Continue;
    }
}
#[inline(never)]
unsafe extern "C" fn whereIsCoveringIndex(
    mut pWInfo: *mut WhereInfo,
    mut pIdx: *mut Index,
    mut iTabCur: ::core::ffi::c_int,
) -> u32_0 {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut ck: CoveringIndexCheck = CoveringIndexCheck {
            pIdx: ::core::ptr::null_mut::<Index>(),
            iTabCur: 0,
            bExpr: 0,
            bUnidx: 0

        };
        let mut w: Walker = Walker {
            pParse: ::core::ptr::null_mut::<Parse>(),
            xExprCallback: None,
            xSelectCallback: None,
            xSelectCallback2: None,
            walkerDepth: 0,
            eCode: 0,
            mWFlags: 0,
            u: C2Rust_Unnamed_29 {
                pNC: ::core::ptr::null_mut::<NameContext>()

            }

        };
        if (*pWInfo).pSelect.is_null() {
            return 0 as u32_0;
        }
        if (*pIdx).bHasExpr() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            i = 0 as ::core::ffi::c_int;
            while i < (*pIdx).nColumn as ::core::ffi::c_int {
                if *(*pIdx).aiColumn.offset(i as isize) as ::core::ffi::c_int
                    >= BMS - 1 as ::core::ffi::c_int
                {
                    break;
                }
                i += 1;
            }
            if i >= (*pIdx).nColumn as ::core::ffi::c_int {
                return 0 as u32_0;
            }
        }
        ck.pIdx = pIdx;
        ck.iTabCur = iTabCur;
        ck.bExpr = 0 as u8_0;
        ck.bUnidx = 0 as u8_0;
        memset(
            &raw mut w as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        w.xExprCallback = Some(
            whereIsCoveringIndexWalkCallback
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = Some(
            sqlite3SelectWalkNoop
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        w.u.pCovIdxCk = &raw mut ck as *mut CoveringIndexCheck;
        sqlite3WalkSelect(&raw mut w, (*pWInfo).pSelect);
        if ck.bUnidx != 0 {
            rc = 0 as ::core::ffi::c_int;
        } else if ck.bExpr != 0 {
            rc = WHERE_EXPRIDX;
        } else {
            rc = WHERE_IDX_ONLY;
        }
        return rc as u32_0;
    }
}
unsafe extern "C" fn whereIndexedExprCleanup(
    mut db: *mut sqlite3,
    mut pObject: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut pp: *mut *mut IndexedExpr = pObject as *mut *mut IndexedExpr;
        while !(*pp).is_null() {
            let mut p: *mut IndexedExpr = *pp;
            *pp = (*p).pIENext;
            sqlite3ExprDelete(db, (*p).pExpr);
            sqlite3DbFreeNN(db, p as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn wherePartIdxExpr(
    mut pParse: *mut Parse,
    mut pIdx: *mut Index,
    mut pPart: *mut Expr,
    mut pMask: *mut Bitmask,
    mut iIdxCur: ::core::ffi::c_int,
    mut pItem: *mut SrcItem,
) {
    unsafe {
        if (*pPart).op as ::core::ffi::c_int == TK_AND {
            wherePartIdxExpr(pParse, pIdx, (*pPart).pRight, pMask, iIdxCur, pItem);
            pPart = (*pPart).pLeft;
        }
        if (*pPart).op as ::core::ffi::c_int == TK_EQ
            || (*pPart).op as ::core::ffi::c_int == TK_IS
        {
            let mut pLeft: *mut Expr = (*pPart).pLeft;
            let mut pRight: *mut Expr = (*pPart).pRight;
            let mut aff: u8_0 = 0;
            if (*pLeft).op as ::core::ffi::c_int != TK_COLUMN {
                return;
            }
            if sqlite3ExprIsConstant(::core::ptr::null_mut::<Parse>(), pRight) == 0 {
                return;
            }
            if sqlite3IsBinary(sqlite3ExprCompareCollSeq(pParse, pPart)) == 0 {
                return;
            }
            if ((*pLeft).iColumn as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                return;
            }
            aff = (*(*(*pIdx).pTable).aCol.offset((*pLeft).iColumn as isize)).affinity
                as u8_0;
            if aff as ::core::ffi::c_int >= SQLITE_AFF_TEXT {
                if !pItem.is_null() {
                    let mut db: *mut sqlite3 = (*pParse).db;
                    let mut p: *mut IndexedExpr = sqlite3DbMallocRaw(
                        db,
                        ::core::mem::size_of::<IndexedExpr>() as u64_0,
                    ) as *mut IndexedExpr;
                    if !p.is_null() {
                        let mut bNullRow: ::core::ffi::c_int = ((*pItem).fg.jointype
                            as ::core::ffi::c_int & (JT_LEFT | JT_LTORJ)
                            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                        (*p).pExpr = sqlite3ExprDup(db, pRight, 0 as ::core::ffi::c_int);
                        (*p).iDataCur = (*pItem).iCursor;
                        (*p).iIdxCur = iIdxCur;
                        (*p).iIdxCol = (*pLeft).iColumn as ::core::ffi::c_int;
                        (*p).bMaybeNullRow = bNullRow as u8_0;
                        (*p).pIENext = (*pParse).pIdxPartExpr;
                        (*p).aff = aff;
                        (*pParse).pIdxPartExpr = p;
                        if (*p).pIENext.is_null() {
                            let mut pArg: *mut ::core::ffi::c_void = &raw mut (*pParse)
                                .pIdxPartExpr as *mut ::core::ffi::c_void;
                            sqlite3ParserAddCleanup(
                                pParse,
                                Some(
                                    whereIndexedExprCleanup
                                        as unsafe extern "C" fn(
                                            *mut sqlite3,
                                            *mut ::core::ffi::c_void,
                                        ) -> (),
                                ),
                                pArg,
                            );
                        }
                    }
                } else if ((*pLeft).iColumn as ::core::ffi::c_int)
                    < BMS - 1 as ::core::ffi::c_int
                {
                    *pMask
                        &= !((1 as ::core::ffi::c_int as Bitmask)
                            << (*pLeft).iColumn as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong;
                }
            }
        }
    }
}
unsafe extern "C" fn whereLoopAddBtree(
    mut pBuilder: *mut WhereLoopBuilder,
    mut mPrereq: Bitmask,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pWInfo: *mut WhereInfo = ::core::ptr::null_mut::<WhereInfo>();
        let mut pProbe: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut sPk: Index = Index {
            zName: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            aiColumn: ::core::ptr::null_mut::<i16_0>(),
            aiRowLogEst: ::core::ptr::null_mut::<LogEst>(),
            pTable: ::core::ptr::null_mut::<Table>(),
            zColAff: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pNext: ::core::ptr::null_mut::<Index>(),
            pSchema: ::core::ptr::null_mut::<Schema>(),
            aSortOrder: ::core::ptr::null_mut::<u8_0>(),
            azColl: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            pPartIdxWhere: ::core::ptr::null_mut::<Expr>(),
            aColExpr: ::core::ptr::null_mut::<ExprList>(),
            tnum: 0,
            szIdxRow: 0,
            nKeyCol: 0,
            nColumn: 0,
            onError: 0,
            idxType_bUnordered_uniqNotNull_isResized_isCovering_noSkipScan_hasStat1_bNoQuery_bAscKeyBug_bHasVCol_bHasExpr: [0; 2],
            c2rust_padding: [0; 3],
            colNotIdxed: 0

        };
        let mut aiRowEstPk: [LogEst; 2] = [0; 2];
        let mut aiColumnPk: i16_0 = -1 as ::core::ffi::c_int as i16_0;
        let mut pTabList: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut pSrc: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut pNew: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut iSortIdx: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut b: ::core::ffi::c_int = 0;
        let mut rSize: LogEst = 0;
        let mut pWC: *mut WhereClause = ::core::ptr::null_mut::<WhereClause>();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        pNew = (*pBuilder).pNew;
        pWInfo = (*pBuilder).pWInfo;
        pTabList = (*pWInfo).pTabList;
        pSrc = (&raw mut (*pTabList).a as *mut SrcItem)
            .offset((*pNew).iTab as ::core::ffi::c_int as isize);
        pTab = (*pSrc).pSTab;
        pWC = (*pBuilder).pWC;
        if (*pSrc).fg.isIndexedBy() != 0 {
            pProbe = (*pSrc).u2.pIBIndex;
        } else if !((*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0) {
            pProbe = (*pTab).pIndex;
        } else {
            let mut pFirst: *mut Index = ::core::ptr::null_mut::<Index>();
            memset(
                &raw mut sPk as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Index>() as size_t,
            );
            sPk.nKeyCol = 1 as u16_0;
            sPk.nColumn = 1 as u16_0;
            sPk.aiColumn = &raw mut aiColumnPk;
            sPk.aiRowLogEst = &raw mut aiRowEstPk as *mut LogEst;
            sPk.onError = OE_Replace as u8_0;
            sPk.pTable = pTab;
            sPk.szIdxRow = 3 as LogEst;
            sPk.set_idxType(
                SQLITE_IDXTYPE_IPK as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
            aiRowEstPk[0 as ::core::ffi::c_int as usize] = (*pTab).nRowLogEst;
            aiRowEstPk[1 as ::core::ffi::c_int as usize] = 0 as LogEst;
            pFirst = (*(*pSrc).pSTab).pIndex;
            if (*pSrc).fg.notIndexed() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                sPk.pNext = pFirst;
            }
            pProbe = &raw mut sPk;
        }
        rSize = (*pTab).nRowLogEst;
        if (*pBuilder).pOrSet.is_null()
            && (*pWInfo).wctrlFlags as ::core::ffi::c_int
                & (WHERE_RIGHT_JOIN | WHERE_OR_SUBCLAUSE) == 0 as ::core::ffi::c_int
            && (*(*(*pWInfo).pParse).db).flags as ::core::ffi::c_ulonglong
                & SQLITE_AutoIndex as ::core::ffi::c_ulonglong
                != 0 as ::core::ffi::c_ulonglong && (*pSrc).fg.isIndexedBy() == 0
            && (*pSrc).fg.notIndexed() == 0 && (*pSrc).fg.isCorrelated() == 0
            && (*pSrc).fg.isRecursive() == 0
            && (*pSrc).fg.jointype as ::core::ffi::c_int & JT_RIGHT
                == 0 as ::core::ffi::c_int
        {
            let mut rLogSize: LogEst = 0;
            let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
            let mut pWCEnd: *mut WhereTerm = (*pWC).a.offset((*pWC).nTerm as isize);
            rLogSize = estLog(rSize);
            pTerm = (*pWC).a;
            while rc == SQLITE_OK && pTerm < pWCEnd {
                if !((*pTerm).prereqRight & (*pNew).maskSelf != 0) {
                    if termCanDriveIndex(pTerm, pSrc, 0 as Bitmask) != 0 {
                        (*pNew).u.btree.nEq = 1 as u16_0;
                        (*pNew).nSkip = 0 as u16_0;
                        (*pNew).u.btree.pIndex = ::core::ptr::null_mut::<Index>();
                        (*pNew).nLTerm = 1 as u16_0;
                        let ref mut c2rust_fresh14 = *(*pNew)
                            .aLTerm
                            .offset(0 as ::core::ffi::c_int as isize);
                        *c2rust_fresh14 = pTerm;
                        (*pNew).rSetup = (rLogSize as ::core::ffi::c_int
                            + rSize as ::core::ffi::c_int) as LogEst;
                        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW)
                            && (*pTab).tabFlags & TF_Ephemeral as u32_0 == 0 as u32_0
                        {
                            (*pNew).rSetup = ((*pNew).rSetup as ::core::ffi::c_int
                                + 28 as ::core::ffi::c_int) as LogEst;
                        } else {
                            (*pNew).rSetup = ((*pNew).rSetup as ::core::ffi::c_int
                                - 25 as ::core::ffi::c_int) as LogEst;
                        }
                        if ((*pNew).rSetup as ::core::ffi::c_int)
                            < 0 as ::core::ffi::c_int
                        {
                            (*pNew).rSetup = 0 as LogEst;
                        }
                        (*pNew).nOut = 43 as LogEst;
                        (*pNew).rRun = sqlite3LogEstAdd(rLogSize, (*pNew).nOut);
                        (*pNew).wsFlags = WHERE_AUTO_INDEX as u32_0;
                        (*pNew).prereq = mPrereq | (*pTerm).prereqRight;
                        rc = whereLoopInsert(pBuilder, pNew);
                    }
                }
                pTerm = pTerm.offset(1);
            }
        }
        while rc == SQLITE_OK && !pProbe.is_null() {
            if !(!(*pProbe).pPartIdxWhere.is_null()
                && whereUsablePartialIndex(
                    (*pSrc).iCursor,
                    (*pSrc).fg.jointype,
                    pWC,
                    (*pProbe).pPartIdxWhere,
                ) == 0)
            {
                if !((*pProbe).bNoQuery() != 0) {
                    rSize = *(*pProbe)
                        .aiRowLogEst
                        .offset(0 as ::core::ffi::c_int as isize);
                    (*pNew).u.btree.nEq = 0 as u16_0;
                    (*pNew).u.btree.nBtm = 0 as u16_0;
                    (*pNew).u.btree.nTop = 0 as u16_0;
                    (*pNew).u.btree.nDistinctCol = 0 as u16_0;
                    (*pNew).nSkip = 0 as u16_0;
                    (*pNew).nLTerm = 0 as u16_0;
                    (*pNew).iSortIdx = 0 as u8_0;
                    (*pNew).rSetup = 0 as LogEst;
                    (*pNew).prereq = mPrereq;
                    (*pNew).nOut = rSize;
                    (*pNew).u.btree.pIndex = pProbe;
                    (*pNew).u.btree.pOrderBy = ::core::ptr::null_mut::<ExprList>();
                    b = indexMightHelpWithOrderBy(pBuilder, pProbe, (*pSrc).iCursor);
                    if (*pProbe).idxType() as ::core::ffi::c_int == SQLITE_IDXTYPE_IPK {
                        (*pNew).wsFlags = WHERE_IPK as u32_0;
                        (*pNew).iSortIdx = (if b != 0 {
                            iSortIdx
                        } else {
                            0 as ::core::ffi::c_int
                        }) as u8_0;
                        (*pNew).rRun = (rSize as ::core::ffi::c_int
                            + 16 as ::core::ffi::c_int) as LogEst;
                        whereLoopOutputAdjust(pWC, pNew, rSize);
                        if (*pSrc).fg.isSubquery() != 0 {
                            if (*pSrc).fg.viaCoroutine() != 0 {
                                (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                                    | WHERE_COROUTINE as ::core::ffi::c_uint) as u32_0;
                            }
                            (*pNew).u.btree.pOrderBy = (*(*(*pSrc).u4.pSubq).pSelect)
                                .pOrderBy;
                        }
                        rc = whereLoopInsert(pBuilder, pNew);
                        (*pNew).nOut = rSize;
                        if rc != 0 {
                            break;
                        }
                    } else {
                        let mut m: Bitmask = 0;
                        if (*pProbe).isCovering() != 0 {
                            m = 0 as Bitmask;
                            (*pNew).wsFlags = (WHERE_IDX_ONLY | WHERE_INDEXED) as u32_0;
                        } else {
                            m = (*pSrc).colUsed & (*pProbe).colNotIdxed;
                            if !(*pProbe).pPartIdxWhere.is_null() {
                                wherePartIdxExpr(
                                    (*pWInfo).pParse,
                                    pProbe,
                                    (*pProbe).pPartIdxWhere,
                                    &raw mut m,
                                    0 as ::core::ffi::c_int,
                                    ::core::ptr::null_mut::<SrcItem>(),
                                );
                            }
                            (*pNew).wsFlags = WHERE_INDEXED as u32_0;
                            if m == TOPBIT
                                || (*pProbe).bHasExpr() as ::core::ffi::c_int != 0
                                    && (*pProbe).bHasVCol() == 0
                                    && m != 0 as ::core::ffi::c_ulonglong
                            {
                                let mut isCov: u32_0 = whereIsCoveringIndex(
                                    pWInfo,
                                    pProbe,
                                    (*pSrc).iCursor,
                                );
                                if !(isCov == 0 as u32_0) {
                                    m = 0 as Bitmask;
                                    (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                                        | isCov as ::core::ffi::c_uint) as u32_0;
                                    isCov & WHERE_IDX_ONLY as u32_0 != 0;
                                }
                            } else if m == 0 as ::core::ffi::c_ulonglong
                                && ((*pTab).tabFlags & TF_WithoutRowid as u32_0
                                    == 0 as u32_0 || !(*pWInfo).pSelect.is_null()
                                    || sqlite3FaultSim(700 as ::core::ffi::c_int) != 0)
                            {
                                (*pNew).wsFlags = (WHERE_IDX_ONLY | WHERE_INDEXED) as u32_0;
                            }
                        }
                        if b != 0
                            || !((*pTab).tabFlags & TF_WithoutRowid as u32_0
                                == 0 as u32_0) || !(*pProbe).pPartIdxWhere.is_null()
                            || (*pSrc).fg.isIndexedBy() as ::core::ffi::c_int != 0
                            || m == 0 as ::core::ffi::c_ulonglong
                                && (*pProbe).bUnordered() as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                && ((*pProbe).szIdxRow as ::core::ffi::c_int)
                                    < (*pTab).szTabRow as ::core::ffi::c_int
                                && (*pWInfo).wctrlFlags as ::core::ffi::c_int
                                    & WHERE_ONEPASS_DESIRED == 0 as ::core::ffi::c_int
                                && sqlite3Config.bUseCis as ::core::ffi::c_int != 0
                                && (*(*(*pWInfo).pParse).db).dbOptFlags & 0x20 as u32_0
                                    == 0 as u32_0
                        {
                            (*pNew).iSortIdx = (if b != 0 {
                                iSortIdx
                            } else {
                                0 as ::core::ffi::c_int
                            }) as u8_0;
                            (*pNew).rRun = (rSize as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                + 15 as ::core::ffi::c_int
                                    * (*pProbe).szIdxRow as ::core::ffi::c_int
                                    / (*pTab).szTabRow as ::core::ffi::c_int) as LogEst;
                            if m != 0 as ::core::ffi::c_ulonglong {
                                let mut nLookup: LogEst = (rSize as ::core::ffi::c_int
                                    + 16 as ::core::ffi::c_int) as LogEst;
                                let mut ii: ::core::ffi::c_int = 0;
                                let mut iCur: ::core::ffi::c_int = (*pSrc).iCursor;
                                let mut pWC2: *mut WhereClause = &raw mut (*pWInfo).sWC;
                                ii = 0 as ::core::ffi::c_int;
                                while ii < (*pWC2).nTerm {
                                    let mut pTerm_0: *mut WhereTerm = (*pWC2)
                                        .a
                                        .offset(ii as isize) as *mut WhereTerm;
                                    if sqlite3ExprCoveredByIndex((*pTerm_0).pExpr, iCur, pProbe)
                                        == 0
                                    {
                                        break;
                                    }
                                    if (*pTerm_0).truthProb as ::core::ffi::c_int
                                        <= 0 as ::core::ffi::c_int
                                    {
                                        nLookup = (nLookup as ::core::ffi::c_int
                                            + (*pTerm_0).truthProb as ::core::ffi::c_int) as LogEst;
                                    } else {
                                        nLookup -= 1;
                                        if (*pTerm_0).eOperator as ::core::ffi::c_int
                                            & (WO_EQ | WO_IS) != 0
                                        {
                                            nLookup = (nLookup as ::core::ffi::c_int
                                                - 19 as ::core::ffi::c_int) as LogEst;
                                        }
                                    }
                                    ii += 1;
                                }
                                (*pNew).rRun = sqlite3LogEstAdd((*pNew).rRun, nLookup);
                            }
                            whereLoopOutputAdjust(pWC, pNew, rSize);
                            if !((*pSrc).fg.jointype as ::core::ffi::c_int & JT_RIGHT
                                != 0 as ::core::ffi::c_int && !(*pProbe).aColExpr.is_null())
                            {
                                rc = whereLoopInsert(pBuilder, pNew);
                            }
                            (*pNew).nOut = rSize;
                            if rc != 0 {
                                break;
                            }
                        }
                    }
                    (*pBuilder).bldFlags1 = 0 as ::core::ffi::c_uchar;
                    rc = whereLoopAddBtreeIndex(pBuilder, pSrc, pProbe, 0 as LogEst);
                    if (*pBuilder).bldFlags1 as ::core::ffi::c_int
                        == SQLITE_BLDF1_INDEXED
                    {
                        (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                            | TF_MaybeReanalyze as ::core::ffi::c_uint) as u32_0;
                    }
                }
            }
            pProbe = (if (*pSrc).fg.isIndexedBy() as ::core::ffi::c_int != 0 {
                ::core::ptr::null_mut::<Index>()
            } else {
                (*pProbe).pNext
            });
            iSortIdx += 1;
        }
        return rc;
    }
}
unsafe extern "C" fn isLimitTerm(mut pTerm: *mut WhereTerm) -> ::core::ffi::c_int {
    unsafe {
        return ((*pTerm).eMatchOp as ::core::ffi::c_int >= SQLITE_INDEX_CONSTRAINT_LIMIT
            && (*pTerm).eMatchOp as ::core::ffi::c_int <= SQLITE_INDEX_CONSTRAINT_OFFSET)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn allConstraintsUsed(
    mut aUsage: *mut sqlite3_index_constraint_usage,
    mut nCons: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        ii = 0 as ::core::ffi::c_int;
        while ii < nCons {
            if (*aUsage.offset(ii as isize)).argvIndex <= 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            ii += 1;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn whereLoopAddVirtualOne(
    mut pBuilder: *mut WhereLoopBuilder,
    mut mPrereq: Bitmask,
    mut mUsable: Bitmask,
    mut mExclude: u16_0,
    mut pIdxInfo: *mut sqlite3_index_info,
    mut mNoOmit: u16_0,
    mut pbIn: *mut ::core::ffi::c_int,
    mut pbRetryLimit: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pWC: *mut WhereClause = (*pBuilder).pWC;
        let mut pHidden: *mut HiddenIndexInfo = pIdxInfo
            .offset(1 as ::core::ffi::c_int as isize) as *mut sqlite3_index_info
            as *mut HiddenIndexInfo;
        let mut pIdxCons: *mut sqlite3_index_constraint = ::core::ptr::null_mut::<
            sqlite3_index_constraint,
        >();
        let mut pUsage: *mut sqlite3_index_constraint_usage = (*pIdxInfo)
            .aConstraintUsage as *mut sqlite3_index_constraint_usage;
        let mut i: ::core::ffi::c_int = 0;
        let mut mxTerm: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pNew: *mut WhereLoop = (*pBuilder).pNew;
        let mut pParse: *mut Parse = (*(*pBuilder).pWInfo).pParse;
        let mut pSrc: *mut SrcItem = (&raw mut (*(*(*pBuilder).pWInfo).pTabList).a
            as *mut SrcItem)
            .offset((*pNew).iTab as isize) as *mut SrcItem;
        let mut nConstraint: ::core::ffi::c_int = (*pIdxInfo).nConstraint;
        *pbIn = 0 as ::core::ffi::c_int;
        (*pNew).prereq = mPrereq;
        pIdxCons = *(&raw mut (*pIdxInfo).aConstraint
            as *mut *mut sqlite3_index_constraint);
        i = 0 as ::core::ffi::c_int;
        while i < nConstraint {
            let mut pTerm: *mut WhereTerm = termFromWhereClause(
                pWC,
                (*pIdxCons).iTermOffset,
            );
            (*pIdxCons).usable = 0 as ::core::ffi::c_uchar;
            if (*pTerm).prereqRight & mUsable == (*pTerm).prereqRight
                && (*pTerm).eOperator as ::core::ffi::c_int
                    & mExclude as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (!pbRetryLimit.is_null() || isLimitTerm(pTerm) == 0)
            {
                (*pIdxCons).usable = 1 as ::core::ffi::c_uchar;
            }
            i += 1;
            pIdxCons = pIdxCons.offset(1);
        }
        memset(
            pUsage as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<sqlite3_index_constraint_usage>() as size_t)
                .wrapping_mul(nConstraint as size_t),
        );
        (*pIdxInfo).idxStr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*pIdxInfo).idxNum = 0 as ::core::ffi::c_int;
        (*pIdxInfo).orderByConsumed = 0 as ::core::ffi::c_int;
        (*pIdxInfo).estimatedCost = SQLITE_BIG_DBL
            / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*pIdxInfo).estimatedRows = 25 as sqlite3_int64;
        (*pIdxInfo).idxFlags = 0 as ::core::ffi::c_int;
        (*pHidden).mHandleIn = 0 as u32_0;
        rc = vtabBestIndex(pParse, (*pSrc).pSTab, pIdxInfo);
        if rc != 0 {
            if rc == SQLITE_CONSTRAINT {
                freeIdxStr(pIdxInfo);
                return SQLITE_OK;
            }
            return rc;
        }
        mxTerm = -1 as ::core::ffi::c_int;
        memset(
            (*pNew).aLTerm as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<*mut WhereTerm>() as size_t)
                .wrapping_mul(nConstraint as size_t),
        );
        memset(
            &raw mut (*pNew).u.vtab as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<C2Rust_Unnamed_24>() as size_t,
        );
        pIdxCons = *(&raw mut (*pIdxInfo).aConstraint
            as *mut *mut sqlite3_index_constraint);
        i = 0 as ::core::ffi::c_int;
        while i < nConstraint {
            let mut iTerm: ::core::ffi::c_int = 0;
            iTerm = (*pUsage.offset(i as isize)).argvIndex - 1 as ::core::ffi::c_int;
            if iTerm >= 0 as ::core::ffi::c_int {
                let mut pTerm_0: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
                let mut j: ::core::ffi::c_int = (*pIdxCons).iTermOffset;
                if iTerm >= nConstraint || j < 0 as ::core::ffi::c_int
                    || {
                        pTerm_0 = termFromWhereClause(pWC, j);
                        pTerm_0.is_null()
                    } || !(*(*pNew).aLTerm.offset(iTerm as isize)).is_null()
                    || (*pIdxCons).usable as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    sqlite3ErrorMsg(
                        pParse,
                        b"%s.xBestIndex malfunction\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*(*pSrc).pSTab).zName,
                    );
                    freeIdxStr(pIdxInfo);
                    return SQLITE_ERROR;
                }
                (*pNew).prereq |= (*pTerm_0).prereqRight as ::core::ffi::c_ulonglong;
                let ref mut c2rust_fresh22 = *(*pNew).aLTerm.offset(iTerm as isize);
                *c2rust_fresh22 = pTerm_0;
                if iTerm > mxTerm {
                    mxTerm = iTerm;
                }
                if (*pUsage.offset(i as isize)).omit != 0 {
                    if i < 16 as ::core::ffi::c_int
                        && (1 as ::core::ffi::c_int) << i & mNoOmit as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        (*pNew).u.vtab.omitMask = ((*pNew).u.vtab.omitMask
                            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << iTerm)
                            as u16_0;
                    }
                    if (*pTerm_0).eMatchOp as ::core::ffi::c_int
                        == SQLITE_INDEX_CONSTRAINT_OFFSET
                    {
                        (*pNew).u.vtab.set_bOmitOffset(1 as u32_0 as u32_0);
                    }
                }
                if (if i <= 31 as ::core::ffi::c_int {
                    (1 as ::core::ffi::c_int as u32_0) << i
                } else {
                    0 as u32_0
                }) & (*pHidden).mHandleIn != 0
                {
                    (*pNew).u.vtab.mHandleIn = ((*pNew).u.vtab.mHandleIn
                        as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_int as ::core::ffi::c_uint) << iTerm)
                        as u32_0;
                } else if (*pTerm_0).eOperator as ::core::ffi::c_int & WO_IN
                    != 0 as ::core::ffi::c_int
                {
                    (*pIdxInfo).orderByConsumed = 0 as ::core::ffi::c_int;
                    (*pIdxInfo).idxFlags &= !SQLITE_INDEX_SCAN_UNIQUE;
                    *pbIn = 1 as ::core::ffi::c_int;
                }
                if isLimitTerm(pTerm_0) != 0
                    && (*pbIn != 0 || allConstraintsUsed(pUsage, i) == 0)
                {
                    freeIdxStr(pIdxInfo);
                    *pbRetryLimit = 1 as ::core::ffi::c_int;
                    return SQLITE_OK;
                }
            }
            i += 1;
            pIdxCons = pIdxCons.offset(1);
        }
        (*pNew).nLTerm = (mxTerm + 1 as ::core::ffi::c_int) as u16_0;
        i = 0 as ::core::ffi::c_int;
        while i <= mxTerm {
            if (*(*pNew).aLTerm.offset(i as isize)).is_null() {
                sqlite3ErrorMsg(
                    pParse,
                    b"%s.xBestIndex malfunction\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(*pSrc).pSTab).zName,
                );
                freeIdxStr(pIdxInfo);
                return SQLITE_ERROR;
            }
            i += 1;
        }
        (*pNew).u.vtab.idxNum = (*pIdxInfo).idxNum;
        (*pNew).u.vtab.set_needFree((*pIdxInfo).needToFreeIdxStr as u32_0 as u32_0);
        (*pIdxInfo).needToFreeIdxStr = 0 as ::core::ffi::c_int;
        (*pNew).u.vtab.idxStr = (*pIdxInfo).idxStr;
        (*pNew).u.vtab.isOrdered = (if (*pIdxInfo).orderByConsumed != 0 {
            (*pIdxInfo).nOrderBy
        } else {
            0 as ::core::ffi::c_int
        }) as i8_0;
        (*pNew)
            .u
            .vtab
            .set_bIdxNumHex(
                ((*pIdxInfo).idxFlags & SQLITE_INDEX_SCAN_HEX != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as u32_0 as u32_0,
            );
        (*pNew).rSetup = 0 as LogEst;
        (*pNew).rRun = sqlite3LogEstFromDouble((*pIdxInfo).estimatedCost);
        (*pNew).nOut = sqlite3LogEst((*pIdxInfo).estimatedRows as u64_0);
        if (*pIdxInfo).idxFlags & SQLITE_INDEX_SCAN_UNIQUE != 0 {
            (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                | WHERE_ONEROW as ::core::ffi::c_uint) as u32_0;
        } else {
            (*pNew).wsFlags = ((*pNew).wsFlags as ::core::ffi::c_uint
                & !WHERE_ONEROW as ::core::ffi::c_uint) as u32_0;
        }
        rc = whereLoopInsert(pBuilder, pNew);
        if (*pNew).u.vtab.needFree() != 0 {
            sqlite3_free((*pNew).u.vtab.idxStr as *mut ::core::ffi::c_void);
            (*pNew).u.vtab.set_needFree(0 as u32_0 as u32_0);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vtab_collation(
    mut pIdxInfo: *mut sqlite3_index_info,
    mut iCons: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut pHidden: *mut HiddenIndexInfo = pIdxInfo
            .offset(1 as ::core::ffi::c_int as isize) as *mut sqlite3_index_info
            as *mut HiddenIndexInfo;
        let mut zRet: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if iCons >= 0 as ::core::ffi::c_int && iCons < (*pIdxInfo).nConstraint {
            let mut pC: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
            let mut iTerm: ::core::ffi::c_int = (*(*pIdxInfo)
                .aConstraint
                .offset(iCons as isize))
                .iTermOffset;
            let mut pX: *mut Expr = (*termFromWhereClause((*pHidden).pWC, iTerm)).pExpr;
            if !(*pX).pLeft.is_null() {
                pC = sqlite3ExprCompareCollSeq((*pHidden).pParse, pX);
            }
            zRet = if !pC.is_null() {
                (*pC).zName as *const ::core::ffi::c_char
            } else {
                &raw const sqlite3StrBINARY as *const ::core::ffi::c_char
            };
        }
        return zRet;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vtab_in(
    mut pIdxInfo: *mut sqlite3_index_info,
    mut iCons: ::core::ffi::c_int,
    mut bHandle: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pHidden: *mut HiddenIndexInfo = pIdxInfo
            .offset(1 as ::core::ffi::c_int as isize) as *mut sqlite3_index_info
            as *mut HiddenIndexInfo;
        let mut m: u32_0 = if iCons <= 31 as ::core::ffi::c_int {
            (1 as ::core::ffi::c_int as u32_0) << iCons
        } else {
            0 as u32_0
        };
        if m & (*pHidden).mIn != 0 {
            if bHandle == 0 as ::core::ffi::c_int {
                (*pHidden).mHandleIn = ((*pHidden).mHandleIn as ::core::ffi::c_uint
                    & !m as ::core::ffi::c_uint) as u32_0;
            } else if bHandle > 0 as ::core::ffi::c_int {
                (*pHidden).mHandleIn = ((*pHidden).mHandleIn as ::core::ffi::c_uint
                    | m as ::core::ffi::c_uint) as u32_0;
            }
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vtab_rhs_value(
    mut pIdxInfo: *mut sqlite3_index_info,
    mut iCons: ::core::ffi::c_int,
    mut ppVal: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pH: *mut HiddenIndexInfo = pIdxInfo
            .offset(1 as ::core::ffi::c_int as isize) as *mut sqlite3_index_info
            as *mut HiddenIndexInfo;
        let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if iCons < 0 as ::core::ffi::c_int || iCons >= (*pIdxInfo).nConstraint {
            rc = sqlite3MisuseError(4499 as ::core::ffi::c_int);
        } else {
            if (*(&raw mut (*pH).aRhs as *mut *mut sqlite3_value).offset(iCons as isize))
                .is_null()
            {
                let mut pTerm: *mut WhereTerm = termFromWhereClause(
                    (*pH).pWC,
                    (*(*pIdxInfo).aConstraint.offset(iCons as isize)).iTermOffset,
                );
                rc = sqlite3ValueFromExpr(
                    (*(*pH).pParse).db,
                    (*(*pTerm).pExpr).pRight,
                    (*(*(*pH).pParse).db).enc,
                    SQLITE_AFF_BLOB as u8_0,
                    (&raw mut (*pH).aRhs as *mut *mut sqlite3_value)
                        .offset(iCons as isize) as *mut *mut sqlite3_value,
                );
            }
            pVal = *(&raw mut (*pH).aRhs as *mut *mut sqlite3_value)
                .offset(iCons as isize);
        }
        *ppVal = pVal;
        if rc == SQLITE_OK && pVal.is_null() {
            rc = SQLITE_NOTFOUND;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vtab_distinct(
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pHidden: *mut HiddenIndexInfo = pIdxInfo
            .offset(1 as ::core::ffi::c_int as isize) as *mut sqlite3_index_info
            as *mut HiddenIndexInfo;
        return (*pHidden).eDistinct;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabUsesAllSchemas(mut pParse: *mut Parse) {
    unsafe {
        let mut nDb: ::core::ffi::c_int = (*(*pParse).db).nDb;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < nDb {
            sqlite3CodeVerifySchema(pParse, i);
            i += 1;
        }
        if (*pParse).writeMask != 0 as ::core::ffi::c_uint {
            i = 0 as ::core::ffi::c_int;
            while i < nDb {
                sqlite3BeginWriteOperation(pParse, 0 as ::core::ffi::c_int, i);
                i += 1;
            }
        }
    }
}
unsafe extern "C" fn whereLoopAddVirtual(
    mut pBuilder: *mut WhereLoopBuilder,
    mut mPrereq: Bitmask,
    mut mUnusable: Bitmask,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pWInfo: *mut WhereInfo = ::core::ptr::null_mut::<WhereInfo>();
        let mut pParse: *mut Parse = ::core::ptr::null_mut::<Parse>();
        let mut pWC: *mut WhereClause = ::core::ptr::null_mut::<WhereClause>();
        let mut pSrc: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut p: *mut sqlite3_index_info = ::core::ptr::null_mut::<
            sqlite3_index_info,
        >();
        let mut nConstraint: ::core::ffi::c_int = 0;
        let mut bIn: ::core::ffi::c_int = 0;
        let mut pNew: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut mBest: Bitmask = 0;
        let mut mNoOmit: u16_0 = 0;
        let mut bRetry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pWInfo = (*pBuilder).pWInfo;
        pParse = (*pWInfo).pParse;
        pWC = (*pBuilder).pWC;
        pNew = (*pBuilder).pNew;
        pSrc = (&raw mut (*(*pWInfo).pTabList).a as *mut SrcItem)
            .offset((*pNew).iTab as isize) as *mut SrcItem;
        p = allocateIndexInfo(pWInfo, pWC, mUnusable, pSrc, &raw mut mNoOmit);
        if p.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        (*pNew).rSetup = 0 as LogEst;
        (*pNew).wsFlags = WHERE_VIRTUALTABLE as u32_0;
        (*pNew).nLTerm = 0 as u16_0;
        (*pNew).u.vtab.set_needFree(0 as u32_0 as u32_0);
        nConstraint = (*p).nConstraint;
        if whereLoopResize((*pParse).db, pNew, nConstraint) != 0 {
            freeIndexInfo((*pParse).db, p);
            return SQLITE_NOMEM_BKPT;
        }
        rc = whereLoopAddVirtualOne(
            pBuilder,
            mPrereq,
            ALLBITS,
            0 as u16_0,
            p,
            mNoOmit,
            &raw mut bIn,
            &raw mut bRetry,
        );
        if bRetry != 0 {
            rc = whereLoopAddVirtualOne(
                pBuilder,
                mPrereq,
                ALLBITS,
                0 as u16_0,
                p,
                mNoOmit,
                &raw mut bIn,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        }
        if rc == SQLITE_OK
            && {
                mBest = (*pNew).prereq & !mPrereq;
                mBest != 0 as ::core::ffi::c_ulonglong || bIn != 0
            }
        {
            let mut seenZero: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut seenZeroNoIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut mPrev: Bitmask = 0 as Bitmask;
            let mut mBestNoIn: Bitmask = 0 as Bitmask;
            if bIn != 0 {
                rc = whereLoopAddVirtualOne(
                    pBuilder,
                    mPrereq,
                    ALLBITS,
                    WO_IN as u16_0,
                    p,
                    mNoOmit,
                    &raw mut bIn,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                mBestNoIn = (*pNew).prereq & !mPrereq;
                if mBestNoIn == 0 as ::core::ffi::c_ulonglong {
                    seenZero = 1 as ::core::ffi::c_int;
                    seenZeroNoIN = 1 as ::core::ffi::c_int;
                }
            }
            while rc == SQLITE_OK {
                let mut i: ::core::ffi::c_int = 0;
                let mut mNext: Bitmask = ALLBITS;
                i = 0 as ::core::ffi::c_int;
                while i < nConstraint {
                    let mut iTerm: ::core::ffi::c_int = (*(*p)
                        .aConstraint
                        .offset(i as isize))
                        .iTermOffset;
                    let mut mThis: Bitmask = (*termFromWhereClause(pWC, iTerm))
                        .prereqRight & !mPrereq;
                    if mThis > mPrev && mThis < mNext {
                        mNext = mThis;
                    }
                    i += 1;
                }
                mPrev = mNext;
                if mNext == ALLBITS {
                    break;
                }
                if mNext == mBest || mNext == mBestNoIn {
                    continue;
                }
                rc = whereLoopAddVirtualOne(
                    pBuilder,
                    mPrereq,
                    mNext | mPrereq,
                    0 as u16_0,
                    p,
                    mNoOmit,
                    &raw mut bIn,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if (*pNew).prereq == mPrereq {
                    seenZero = 1 as ::core::ffi::c_int;
                    if bIn == 0 as ::core::ffi::c_int {
                        seenZeroNoIN = 1 as ::core::ffi::c_int;
                    }
                }
            }
            if rc == SQLITE_OK && seenZero == 0 as ::core::ffi::c_int {
                rc = whereLoopAddVirtualOne(
                    pBuilder,
                    mPrereq,
                    mPrereq,
                    0 as u16_0,
                    p,
                    mNoOmit,
                    &raw mut bIn,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if bIn == 0 as ::core::ffi::c_int {
                    seenZeroNoIN = 1 as ::core::ffi::c_int;
                }
            }
            if rc == SQLITE_OK && seenZeroNoIN == 0 as ::core::ffi::c_int {
                rc = whereLoopAddVirtualOne(
                    pBuilder,
                    mPrereq,
                    mPrereq,
                    WO_IN as u16_0,
                    p,
                    mNoOmit,
                    &raw mut bIn,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
            }
        }
        freeIndexInfo((*pParse).db, p);
        return rc;
    }
}
unsafe extern "C" fn whereLoopAddOr(
    mut pBuilder: *mut WhereLoopBuilder,
    mut mPrereq: Bitmask,
    mut mUnusable: Bitmask,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pWInfo: *mut WhereInfo = (*pBuilder).pWInfo;
        let mut pWC: *mut WhereClause = ::core::ptr::null_mut::<WhereClause>();
        let mut pNew: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut pWCEnd: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut iCur: ::core::ffi::c_int = 0;
        let mut tempWC: WhereClause = WhereClause {
            pWInfo: ::core::ptr::null_mut::<WhereInfo>(),
            pOuter: ::core::ptr::null_mut::<WhereClause>(),
            op: 0,
            hasOr: 0,
            nTerm: 0,
            nSlot: 0,
            nBase: 0,
            a: ::core::ptr::null_mut::<WhereTerm>(),
            aStatic: [WhereTerm {
                pExpr: ::core::ptr::null_mut::<Expr>(),
                pWC: ::core::ptr::null_mut::<WhereClause>(),
                truthProb: 0,
                wtFlags: 0,
                eOperator: 0,
                nChild: 0,
                eMatchOp: 0,
                iParent: 0,
                leftCursor: 0,
                u: C2Rust_Unnamed_22 {
                    x: C2Rust_Unnamed_28 {
                        leftColumn: 0,
                        iField: 0

                    }

                },
                prereqRight: 0,
                prereqAll: 0

            }; 8]

        };
        let mut sSubBuild: WhereLoopBuilder = WhereLoopBuilder {
            pWInfo: ::core::ptr::null_mut::<WhereInfo>(),
            pWC: ::core::ptr::null_mut::<WhereClause>(),
            pNew: ::core::ptr::null_mut::<WhereLoop>(),
            pOrSet: ::core::ptr::null_mut::<WhereOrSet>(),
            bldFlags1: 0,
            bldFlags2: 0,
            iPlanLimit: 0

        };
        let mut sSum: WhereOrSet = WhereOrSet {
            n: 0,
            a: [WhereOrCost {
                prereq: 0,
                rRun: 0,
                nOut: 0

            }; 3]

        };
        let mut sCur: WhereOrSet = WhereOrSet {
            n: 0,
            a: [WhereOrCost {
                prereq: 0,
                rRun: 0,
                nOut: 0

            }; 3]

        };
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        pWC = (*pBuilder).pWC;
        pWCEnd = (*pWC).a.offset((*pWC).nTerm as isize);
        pNew = (*pBuilder).pNew;
        memset(
            &raw mut sSum as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<WhereOrSet>() as size_t,
        );
        pItem = (&raw mut (*(*pWInfo).pTabList).a as *mut SrcItem)
            .offset((*pNew).iTab as ::core::ffi::c_int as isize);
        iCur = (*pItem).iCursor;
        if (*pItem).fg.jointype as ::core::ffi::c_int & JT_RIGHT != 0 {
            return SQLITE_OK;
        }
        pTerm = (*pWC).a;
        while pTerm < pWCEnd && rc == SQLITE_OK {
            if (*pTerm).eOperator as ::core::ffi::c_int & WO_OR
                != 0 as ::core::ffi::c_int
                && (*(*pTerm).u.pOrInfo).indexable & (*pNew).maskSelf
                    != 0 as ::core::ffi::c_ulonglong
            {
                let pOrWC: *mut WhereClause = &raw mut (*(*pTerm).u.pOrInfo).wc;
                let pOrWCEnd: *mut WhereTerm = (*pOrWC).a.offset((*pOrWC).nTerm as isize)
                    as *mut WhereTerm;
                let mut pOrTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
                let mut once: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                let mut i: ::core::ffi::c_int = 0;
                let mut j: ::core::ffi::c_int = 0;
                sSubBuild = *pBuilder;
                sSubBuild.pOrSet = &raw mut sCur;
                let mut c2rust_current_block_48: u64;
                pOrTerm = (*pOrWC).a;
                while pOrTerm < pOrWCEnd {
                    if (*pOrTerm).eOperator as ::core::ffi::c_int & WO_AND
                        != 0 as ::core::ffi::c_int
                    {
                        sSubBuild.pWC = &raw mut (*(*pOrTerm).u.pAndInfo).wc;
                        c2rust_current_block_48 = 18386322304582297246;
                    } else if (*pOrTerm).leftCursor == iCur {
                        tempWC.pWInfo = (*pWC).pWInfo;
                        tempWC.pOuter = pWC;
                        tempWC.op = TK_AND as u8_0;
                        tempWC.nTerm = 1 as ::core::ffi::c_int;
                        tempWC.nBase = 1 as ::core::ffi::c_int;
                        tempWC.a = pOrTerm;
                        sSubBuild.pWC = &raw mut tempWC;
                        c2rust_current_block_48 = 18386322304582297246;
                    } else {
                        c2rust_current_block_48 = 13056961889198038528;
                    }
                    match c2rust_current_block_48 {
                        18386322304582297246 => {
                            sCur.n = 0 as u16_0;
                            if (*(*pItem).pSTab).eTabType as ::core::ffi::c_int
                                == TABTYP_VTAB
                            {
                                rc = whereLoopAddVirtual(
                                    &raw mut sSubBuild,
                                    mPrereq,
                                    mUnusable,
                                );
                            } else {
                                rc = whereLoopAddBtree(&raw mut sSubBuild, mPrereq);
                            }
                            if rc == SQLITE_OK {
                                rc = whereLoopAddOr(&raw mut sSubBuild, mPrereq, mUnusable);
                            }
                            if sCur.n as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                sSum.n = 0 as u16_0;
                                break;
                            } else if once != 0 {
                                whereOrMove(&raw mut sSum, &raw mut sCur);
                                once = 0 as ::core::ffi::c_int;
                            } else {
                                let mut sPrev: WhereOrSet = WhereOrSet {
                                    n: 0,
                                    a: [WhereOrCost {
                                        prereq: 0,
                                        rRun: 0,
                                        nOut: 0

                                    }; 3]

                                };
                                whereOrMove(&raw mut sPrev, &raw mut sSum);
                                sSum.n = 0 as u16_0;
                                i = 0 as ::core::ffi::c_int;
                                while i < sPrev.n as ::core::ffi::c_int {
                                    j = 0 as ::core::ffi::c_int;
                                    while j < sCur.n as ::core::ffi::c_int {
                                        whereOrInsert(
                                            &raw mut sSum,
                                            sPrev.a[i as usize].prereq | sCur.a[j as usize].prereq,
                                            sqlite3LogEstAdd(
                                                sPrev.a[i as usize].rRun,
                                                sCur.a[j as usize].rRun,
                                            ),
                                            sqlite3LogEstAdd(
                                                sPrev.a[i as usize].nOut,
                                                sCur.a[j as usize].nOut,
                                            ),
                                        );
                                        j += 1;
                                    }
                                    i += 1;
                                }
                            }
                        }
                        _ => {}
                    }
                    pOrTerm = pOrTerm.offset(1);
                }
                (*pNew).nLTerm = 1 as u16_0;
                let ref mut c2rust_fresh12 = *(*pNew)
                    .aLTerm
                    .offset(0 as ::core::ffi::c_int as isize);
                *c2rust_fresh12 = pTerm;
                (*pNew).wsFlags = WHERE_MULTI_OR as u32_0;
                (*pNew).rSetup = 0 as LogEst;
                (*pNew).iSortIdx = 0 as u8_0;
                memset(
                    &raw mut (*pNew).u as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<C2Rust_Unnamed_23>() as size_t,
                );
                i = 0 as ::core::ffi::c_int;
                while rc == SQLITE_OK && i < sSum.n as ::core::ffi::c_int {
                    (*pNew).rRun = (sSum.a[i as usize].rRun as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as LogEst;
                    (*pNew).nOut = sSum.a[i as usize].nOut;
                    (*pNew).prereq = sSum.a[i as usize].prereq;
                    rc = whereLoopInsert(pBuilder, pNew);
                    i += 1;
                }
            }
            pTerm = pTerm.offset(1);
        }
        return rc;
    }
}
unsafe extern "C" fn whereLoopAddAll(
    mut pBuilder: *mut WhereLoopBuilder,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pWInfo: *mut WhereInfo = (*pBuilder).pWInfo;
        let mut mPrereq: Bitmask = 0 as Bitmask;
        let mut mPrior: Bitmask = 0 as Bitmask;
        let mut iTab: ::core::ffi::c_int = 0;
        let mut pTabList: *mut SrcList = (*pWInfo).pTabList;
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut pEnd: *mut SrcItem = (&raw mut (*pTabList).a as *mut SrcItem)
            .offset((*pWInfo).nLevel as isize) as *mut SrcItem;
        let mut db: *mut sqlite3 = (*(*pWInfo).pParse).db;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut bFirstPastRJ: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut hasRightJoin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pNew: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        pNew = (*pBuilder).pNew;
        (*pBuilder).iPlanLimit = SQLITE_QUERY_PLANNER_LIMIT as ::core::ffi::c_uint;
        iTab = 0 as ::core::ffi::c_int;
        pItem = &raw mut (*pTabList).a as *mut SrcItem;
        while pItem < pEnd {
            let mut mUnusable: Bitmask = 0 as Bitmask;
            (*pNew).iTab = iTab as u8_0;
            (*pBuilder).iPlanLimit = (*pBuilder)
                .iPlanLimit
                .wrapping_add(SQLITE_QUERY_PLANNER_LIMIT_INCR as ::core::ffi::c_uint);
            (*pNew).maskSelf = sqlite3WhereGetMask(
                &raw mut (*pWInfo).sMaskSet,
                (*pItem).iCursor,
            );
            if bFirstPastRJ != 0
                || (*pItem).fg.jointype as ::core::ffi::c_int
                    & (JT_OUTER | JT_CROSS | JT_LTORJ) != 0 as ::core::ffi::c_int
            {
                if (*pItem).fg.jointype as ::core::ffi::c_int & JT_LTORJ != 0 {
                    hasRightJoin = 1 as ::core::ffi::c_int;
                }
                mPrereq |= mPrior as ::core::ffi::c_ulonglong;
                bFirstPastRJ = ((*pItem).fg.jointype as ::core::ffi::c_int & JT_RIGHT
                    != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            } else if hasRightJoin == 0 {
                mPrereq = 0 as Bitmask;
            }
            if (*(*pItem).pSTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                let mut p: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
                p = pItem.offset(1 as ::core::ffi::c_int as isize) as *mut SrcItem;
                while p < pEnd {
                    if mUnusable != 0
                        || (*p).fg.jointype as ::core::ffi::c_int & (JT_OUTER | JT_CROSS)
                            != 0
                    {
                        mUnusable
                            |= sqlite3WhereGetMask(
                                &raw mut (*pWInfo).sMaskSet,
                                (*p).iCursor,
                            ) as ::core::ffi::c_ulonglong;
                    }
                    p = p.offset(1);
                }
                rc = whereLoopAddVirtual(pBuilder, mPrereq, mUnusable);
            } else {
                rc = whereLoopAddBtree(pBuilder, mPrereq);
            }
            if rc == SQLITE_OK && (*(*pBuilder).pWC).hasOr as ::core::ffi::c_int != 0 {
                rc = whereLoopAddOr(pBuilder, mPrereq, mUnusable);
            }
            mPrior |= (*pNew).maskSelf as ::core::ffi::c_ulonglong;
            if rc != 0 || (*db).mallocFailed as ::core::ffi::c_int != 0 {
                if !(rc == SQLITE_DONE) {
                    break;
                }
                sqlite3_log(
                    SQLITE_WARNING,
                    b"abbreviated query algorithm search\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                rc = SQLITE_OK;
            }
            iTab += 1;
            pItem = pItem.offset(1);
        }
        whereLoopClear(db, pNew);
        return rc;
    }
}
#[inline(never)]
unsafe extern "C" fn wherePathMatchSubqueryOB(
    mut pWInfo: *mut WhereInfo,
    mut pLoop: *mut WhereLoop,
    mut iLoop: ::core::ffi::c_int,
    mut iCur: ::core::ffi::c_int,
    mut pOrderBy: *mut ExprList,
    mut pRevMask: *mut Bitmask,
    mut pOBSat: *mut Bitmask,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iOB: ::core::ffi::c_int = 0;
        let mut jSub: ::core::ffi::c_int = 0;
        let mut rev: u8_0 = 0 as u8_0;
        let mut revIdx: u8_0 = 0 as u8_0;
        let mut pOBExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pSubOB: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        pSubOB = (*pLoop).u.btree.pOrderBy;
        iOB = 0 as ::core::ffi::c_int;
        while (1 as ::core::ffi::c_int as Bitmask) << iOB & *pOBSat
            != 0 as ::core::ffi::c_ulonglong
        {
            iOB += 1;
        }
        jSub = 0 as ::core::ffi::c_int;
        while jSub < (*pSubOB).nExpr && iOB < (*pOrderBy).nExpr {
            if (*(&raw mut (*pSubOB).a as *mut ExprList_item).offset(jSub as isize))
                .u
                .x
                .iOrderByCol as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                break;
            }
            pOBExpr = (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                .offset(iOB as isize))
                .pExpr;
            if (*pOBExpr).op as ::core::ffi::c_int != TK_COLUMN
                && (*pOBExpr).op as ::core::ffi::c_int != TK_AGG_COLUMN
            {
                break;
            }
            if (*pOBExpr).iTable != iCur {
                break;
            }
            if (*pOBExpr).iColumn as ::core::ffi::c_int
                != (*(&raw mut (*pSubOB).a as *mut ExprList_item).offset(jSub as isize))
                    .u
                    .x
                    .iOrderByCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            {
                break;
            }
            if (*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_GROUPBY
                == 0 as ::core::ffi::c_int
            {
                let mut sfOB: u8_0 = (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                    .offset(iOB as isize))
                    .fg
                    .sortFlags;
                let mut sfSub: u8_0 = (*(&raw mut (*pSubOB).a as *mut ExprList_item)
                    .offset(jSub as isize))
                    .fg
                    .sortFlags;
                if sfSub as ::core::ffi::c_int & KEYINFO_ORDER_BIGNULL
                    != sfOB as ::core::ffi::c_int & KEYINFO_ORDER_BIGNULL
                {
                    break;
                }
                revIdx = (sfSub as ::core::ffi::c_int & KEYINFO_ORDER_DESC) as u8_0;
                if jSub > 0 as ::core::ffi::c_int {
                    if rev as ::core::ffi::c_int ^ revIdx as ::core::ffi::c_int
                        != sfOB as ::core::ffi::c_int & KEYINFO_ORDER_DESC
                    {
                        break;
                    }
                } else {
                    rev = (revIdx as ::core::ffi::c_int
                        ^ sfOB as ::core::ffi::c_int & KEYINFO_ORDER_DESC) as u8_0;
                    if rev != 0 {
                        if (*pLoop).wsFlags & WHERE_COROUTINE as u32_0 != 0 as u32_0 {
                            break;
                        }
                        *pRevMask
                            |= ((1 as ::core::ffi::c_int as Bitmask) << iLoop)
                                as ::core::ffi::c_ulonglong;
                    }
                }
            }
            *pOBSat
                |= ((1 as ::core::ffi::c_int as Bitmask) << iOB)
                    as ::core::ffi::c_ulonglong;
            jSub += 1;
            iOB += 1;
        }
        return (jSub > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn wherePathSatisfiesOrderBy(
    mut pWInfo: *mut WhereInfo,
    mut pOrderBy: *mut ExprList,
    mut pPath: *mut WherePath,
    mut wctrlFlags: u16_0,
    mut nLoop: u16_0,
    mut pLast: *mut WhereLoop,
    mut pRevMask: *mut Bitmask,
) -> i8_0 {
    unsafe {
        let mut revSet: u8_0 = 0;
        let mut rev: u8_0 = 0;
        let mut revIdx: u8_0 = 0;
        let mut isOrderDistinct: u8_0 = 0;
        let mut distinctColumns: u8_0 = 0;
        let mut isMatch: u8_0 = 0;
        let mut eqOpMask: u16_0 = 0;
        let mut nKeyCol: u16_0 = 0;
        let mut nColumn: u16_0 = 0;
        let mut nOrderBy: u16_0 = 0;
        let mut iLoop: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut iCur: ::core::ffi::c_int = 0;
        let mut iColumn: ::core::ffi::c_int = 0;
        let mut pLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut pOBExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        let mut pIndex: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut db: *mut sqlite3 = (*(*pWInfo).pParse).db;
        let mut obSat: Bitmask = 0 as Bitmask;
        let mut obDone: Bitmask = 0;
        let mut orderDistinctMask: Bitmask = 0;
        let mut ready: Bitmask = 0;
        if nLoop as ::core::ffi::c_int != 0
            && (*db).dbOptFlags & 0x40 as u32_0 != 0 as u32_0
        {
            return 0 as i8_0;
        }
        nOrderBy = (*pOrderBy).nExpr as u16_0;
        if nOrderBy as ::core::ffi::c_int > BMS - 1 as ::core::ffi::c_int {
            return 0 as i8_0;
        }
        isOrderDistinct = 1 as u8_0;
        obDone = ((1 as ::core::ffi::c_int as ::core::ffi::c_ulonglong)
            << nOrderBy as ::core::ffi::c_int)
            .wrapping_sub(1 as ::core::ffi::c_ulonglong) as Bitmask;
        orderDistinctMask = 0 as Bitmask;
        ready = 0 as Bitmask;
        eqOpMask = (WO_EQ | WO_IS | WO_ISNULL) as u16_0;
        if wctrlFlags as ::core::ffi::c_int
            & (WHERE_ORDERBY_LIMIT | WHERE_ORDERBY_MAX | WHERE_ORDERBY_MIN) != 0
        {
            eqOpMask = (eqOpMask as ::core::ffi::c_int | WO_IN) as u16_0;
        }
        let mut c2rust_current_block_148: u64;
        iLoop = 0 as ::core::ffi::c_int;
        while isOrderDistinct as ::core::ffi::c_int != 0 && obSat < obDone
            && iLoop <= nLoop as ::core::ffi::c_int
        {
            if iLoop > 0 as ::core::ffi::c_int {
                ready |= (*pLoop).maskSelf as ::core::ffi::c_ulonglong;
            }
            if iLoop < nLoop as ::core::ffi::c_int {
                pLoop = *(*pPath).aLoop.offset(iLoop as isize);
                if wctrlFlags as ::core::ffi::c_int & WHERE_ORDERBY_LIMIT != 0 {
                    c2rust_current_block_148 = 4808432441040389987;
                } else {
                    c2rust_current_block_148 = 14763689060501151050;
                }
            } else {
                pLoop = pLast;
                c2rust_current_block_148 = 14763689060501151050;
            }
            match c2rust_current_block_148 {
                14763689060501151050 => {
                    if (*pLoop).wsFlags & WHERE_VIRTUALTABLE as u32_0 != 0 {
                        if (*pLoop).u.vtab.isOrdered as ::core::ffi::c_int != 0
                            && wctrlFlags as ::core::ffi::c_int
                                & (WHERE_DISTINCTBY | WHERE_SORTBYGROUP) != WHERE_DISTINCTBY
                        {
                            obSat = obDone;
                        } else {
                            isOrderDistinct = 0 as u8_0;
                        }
                        break;
                    } else {
                        iCur = (*(&raw mut (*(*pWInfo).pTabList).a as *mut SrcItem)
                            .offset((*pLoop).iTab as isize))
                            .iCursor;
                        let mut c2rust_current_block_36: u64;
                        i = 0 as ::core::ffi::c_int;
                        while i < nOrderBy as ::core::ffi::c_int {
                            if !((1 as ::core::ffi::c_int as Bitmask) << i & obSat != 0)
                            {
                                pOBExpr = sqlite3ExprSkipCollateAndLikely(
                                    (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                                        .offset(i as isize))
                                        .pExpr,
                                );
                                if !pOBExpr.is_null() {
                                    if !((*pOBExpr).op as ::core::ffi::c_int != TK_COLUMN
                                        && (*pOBExpr).op as ::core::ffi::c_int != TK_AGG_COLUMN)
                                    {
                                        if !((*pOBExpr).iTable != iCur) {
                                            pTerm = sqlite3WhereFindTerm(
                                                &raw mut (*pWInfo).sWC,
                                                iCur,
                                                (*pOBExpr).iColumn as ::core::ffi::c_int,
                                                !ready,
                                                eqOpMask as u32_0,
                                                ::core::ptr::null_mut::<Index>(),
                                            );
                                            if !pTerm.is_null() {
                                                if (*pTerm).eOperator as ::core::ffi::c_int == WO_IN {
                                                    j = 0 as ::core::ffi::c_int;
                                                    while j < (*pLoop).nLTerm as ::core::ffi::c_int
                                                        && pTerm != *(*pLoop).aLTerm.offset(j as isize)
                                                    {
                                                        j += 1;
                                                    }
                                                    if j >= (*pLoop).nLTerm as ::core::ffi::c_int {
                                                        c2rust_current_block_36 = 18153031941552419006;
                                                    } else {
                                                        c2rust_current_block_36 = 14775119014532381840;
                                                    }
                                                } else {
                                                    c2rust_current_block_36 = 14775119014532381840;
                                                }
                                                match c2rust_current_block_36 {
                                                    18153031941552419006 => {}
                                                    _ => {
                                                        if (*pTerm).eOperator as ::core::ffi::c_int
                                                            & (WO_EQ | WO_IS) != 0 as ::core::ffi::c_int
                                                            && (*pOBExpr).iColumn as ::core::ffi::c_int
                                                                >= 0 as ::core::ffi::c_int
                                                        {
                                                            let mut pParse: *mut Parse = (*pWInfo).pParse;
                                                            let mut pColl1: *mut CollSeq = sqlite3ExprNNCollSeq(
                                                                pParse,
                                                                (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                                                                    .offset(i as isize))
                                                                    .pExpr,
                                                            );
                                                            let mut pColl2: *mut CollSeq = sqlite3ExprCompareCollSeq(
                                                                pParse,
                                                                (*pTerm).pExpr,
                                                            );
                                                            if pColl2.is_null()
                                                                || sqlite3StrICmp((*pColl1).zName, (*pColl2).zName) != 0
                                                            {
                                                                c2rust_current_block_36 = 18153031941552419006;
                                                            } else {
                                                                c2rust_current_block_36 = 5892776923941496671;
                                                            }
                                                        } else {
                                                            c2rust_current_block_36 = 5892776923941496671;
                                                        }
                                                        match c2rust_current_block_36 {
                                                            18153031941552419006 => {}
                                                            _ => {
                                                                obSat
                                                                    |= ((1 as ::core::ffi::c_int as Bitmask) << i)
                                                                        as ::core::ffi::c_ulonglong;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            i += 1;
                        }
                        if (*pLoop).wsFlags & WHERE_ONEROW as u32_0 == 0 as u32_0 {
                            if (*pLoop).wsFlags & WHERE_IPK as u32_0 != 0 {
                                if !(*pLoop).u.btree.pOrderBy.is_null()
                                    && (*db).dbOptFlags & 0x10000000 as u32_0 == 0 as u32_0
                                    && wherePathMatchSubqueryOB(
                                        pWInfo,
                                        pLoop,
                                        iLoop,
                                        iCur,
                                        pOrderBy,
                                        pRevMask,
                                        &raw mut obSat,
                                    ) != 0
                                {
                                    nColumn = 0 as u16_0;
                                    isOrderDistinct = 0 as u8_0;
                                } else {
                                    nColumn = 1 as u16_0;
                                }
                                pIndex = ::core::ptr::null_mut::<Index>();
                                nKeyCol = 0 as u16_0;
                            } else {
                                pIndex = (*pLoop).u.btree.pIndex;
                                if pIndex.is_null()
                                    || (*pIndex).bUnordered() as ::core::ffi::c_int != 0
                                {
                                    return 0 as i8_0
                                } else {
                                    nKeyCol = (*pIndex).nKeyCol;
                                    nColumn = (*pIndex).nColumn;
                                    isOrderDistinct = ((*pIndex).onError as ::core::ffi::c_int
                                        != OE_None
                                        && (*pLoop).wsFlags & WHERE_SKIPSCAN as u32_0 == 0 as u32_0)
                                        as ::core::ffi::c_int as u8_0;
                                }
                            }
                            revSet = 0 as u8_0;
                            rev = revSet;
                            distinctColumns = 0 as u8_0;
                            let mut c2rust_current_block_131: u64;
                            j = 0 as ::core::ffi::c_int;
                            while j < nColumn as ::core::ffi::c_int {
                                let mut bOnce: u8_0 = 1 as u8_0;
                                if j < (*pLoop).u.btree.nEq as ::core::ffi::c_int
                                    && j >= (*pLoop).nSkip as ::core::ffi::c_int
                                {
                                    let mut eOp: u16_0 = (**(*pLoop).aLTerm.offset(j as isize))
                                        .eOperator;
                                    if eOp as ::core::ffi::c_int
                                        & eqOpMask as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                    {
                                        if eOp as ::core::ffi::c_int & (WO_ISNULL | WO_IS) != 0 {
                                            isOrderDistinct = 0 as u8_0;
                                        }
                                        c2rust_current_block_131 = 13484060386966298149;
                                    } else {
                                        if eOp as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
                                            != 0
                                        {
                                            let mut pX: *mut Expr = (**(*pLoop)
                                                .aLTerm
                                                .offset(j as isize))
                                                .pExpr;
                                            i = j + 1 as ::core::ffi::c_int;
                                            while i < (*pLoop).u.btree.nEq as ::core::ffi::c_int {
                                                if (**(*pLoop).aLTerm.offset(i as isize)).pExpr == pX {
                                                    bOnce = 0 as u8_0;
                                                    break;
                                                } else {
                                                    i += 1;
                                                }
                                            }
                                        }
                                        c2rust_current_block_131 = 8102658916883067714;
                                    }
                                } else {
                                    c2rust_current_block_131 = 8102658916883067714;
                                }
                                match c2rust_current_block_131 {
                                    8102658916883067714 => {
                                        if !pIndex.is_null() {
                                            iColumn = *(*pIndex).aiColumn.offset(j as isize)
                                                as ::core::ffi::c_int;
                                            revIdx = (*(*pIndex).aSortOrder.offset(j as isize)
                                                as ::core::ffi::c_int & KEYINFO_ORDER_DESC) as u8_0;
                                            if iColumn
                                                == (*(*pIndex).pTable).iPKey as ::core::ffi::c_int
                                            {
                                                iColumn = XN_ROWID;
                                            }
                                        } else {
                                            iColumn = XN_ROWID;
                                            revIdx = 0 as u8_0;
                                        }
                                        if isOrderDistinct != 0 {
                                            if iColumn >= 0 as ::core::ffi::c_int
                                                && j >= (*pLoop).u.btree.nEq as ::core::ffi::c_int
                                                && (*(*(*pIndex).pTable).aCol.offset(iColumn as isize))
                                                    .notNull() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                            {
                                                isOrderDistinct = 0 as u8_0;
                                            }
                                            if iColumn == XN_EXPR {
                                                isOrderDistinct = 0 as u8_0;
                                            }
                                        }
                                        isMatch = 0 as u8_0;
                                        let mut c2rust_current_block_101: u64;
                                        i = 0 as ::core::ffi::c_int;
                                        while bOnce as ::core::ffi::c_int != 0
                                            && i < nOrderBy as ::core::ffi::c_int
                                        {
                                            if !((1 as ::core::ffi::c_int as Bitmask) << i & obSat != 0)
                                            {
                                                pOBExpr = sqlite3ExprSkipCollateAndLikely(
                                                    (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                                                        .offset(i as isize))
                                                        .pExpr,
                                                );
                                                if !pOBExpr.is_null() {
                                                    if wctrlFlags as ::core::ffi::c_int
                                                        & (WHERE_GROUPBY | WHERE_DISTINCTBY)
                                                        == 0 as ::core::ffi::c_int
                                                    {
                                                        bOnce = 0 as u8_0;
                                                    }
                                                    if iColumn >= XN_ROWID {
                                                        if (*pOBExpr).op as ::core::ffi::c_int != TK_COLUMN
                                                            && (*pOBExpr).op as ::core::ffi::c_int != TK_AGG_COLUMN
                                                        {
                                                            c2rust_current_block_101 = 2220405792722996547;
                                                        } else if (*pOBExpr).iTable != iCur {
                                                            c2rust_current_block_101 = 2220405792722996547;
                                                        } else if (*pOBExpr).iColumn as ::core::ffi::c_int
                                                            != iColumn
                                                        {
                                                            c2rust_current_block_101 = 2220405792722996547;
                                                        } else {
                                                            c2rust_current_block_101 = 18002345992382212654;
                                                        }
                                                    } else {
                                                        let mut pIxExpr: *mut Expr = (*(&raw mut (*(*pIndex)
                                                            .aColExpr)
                                                            .a as *mut ExprList_item)
                                                            .offset(j as isize))
                                                            .pExpr;
                                                        if sqlite3ExprCompareSkip(pOBExpr, pIxExpr, iCur) != 0 {
                                                            c2rust_current_block_101 = 2220405792722996547;
                                                        } else {
                                                            c2rust_current_block_101 = 18002345992382212654;
                                                        }
                                                    }
                                                    match c2rust_current_block_101 {
                                                        2220405792722996547 => {}
                                                        _ => {
                                                            if iColumn != XN_ROWID {
                                                                pColl = sqlite3ExprNNCollSeq(
                                                                    (*pWInfo).pParse,
                                                                    (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                                                                        .offset(i as isize))
                                                                        .pExpr,
                                                                );
                                                                if sqlite3StrICmp(
                                                                    (*pColl).zName,
                                                                    *(*pIndex).azColl.offset(j as isize),
                                                                ) != 0 as ::core::ffi::c_int
                                                                {
                                                                    c2rust_current_block_101 = 2220405792722996547;
                                                                } else {
                                                                    c2rust_current_block_101 = 13823707972521062695;
                                                                }
                                                            } else {
                                                                c2rust_current_block_101 = 13823707972521062695;
                                                            }
                                                            match c2rust_current_block_101 {
                                                                2220405792722996547 => {}
                                                                _ => {
                                                                    if wctrlFlags as ::core::ffi::c_int & WHERE_DISTINCTBY != 0
                                                                    {
                                                                        (*pLoop).u.btree.nDistinctCol = (j
                                                                            + 1 as ::core::ffi::c_int) as u16_0;
                                                                    }
                                                                    isMatch = 1 as u8_0;
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            i += 1;
                                        }
                                        if isMatch as ::core::ffi::c_int != 0
                                            && wctrlFlags as ::core::ffi::c_int & WHERE_GROUPBY
                                                == 0 as ::core::ffi::c_int
                                        {
                                            if revSet != 0 {
                                                if rev as ::core::ffi::c_int ^ revIdx as ::core::ffi::c_int
                                                    != (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                                                        .offset(i as isize))
                                                        .fg
                                                        .sortFlags as ::core::ffi::c_int & KEYINFO_ORDER_DESC
                                                {
                                                    isMatch = 0 as u8_0;
                                                }
                                            } else {
                                                rev = (revIdx as ::core::ffi::c_int
                                                    ^ (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                                                        .offset(i as isize))
                                                        .fg
                                                        .sortFlags as ::core::ffi::c_int & KEYINFO_ORDER_DESC)
                                                    as u8_0;
                                                if rev != 0 {
                                                    *pRevMask
                                                        |= ((1 as ::core::ffi::c_int as Bitmask) << iLoop)
                                                            as ::core::ffi::c_ulonglong;
                                                }
                                                revSet = 1 as u8_0;
                                            }
                                        }
                                        if isMatch as ::core::ffi::c_int != 0
                                            && (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                                                .offset(i as isize))
                                                .fg
                                                .sortFlags as ::core::ffi::c_int & KEYINFO_ORDER_BIGNULL
                                                != 0
                                        {
                                            if j == (*pLoop).u.btree.nEq as ::core::ffi::c_int {
                                                (*pLoop).wsFlags = ((*pLoop).wsFlags as ::core::ffi::c_uint
                                                    | WHERE_BIGNULL_SORT as ::core::ffi::c_uint) as u32_0;
                                            } else {
                                                isMatch = 0 as u8_0;
                                            }
                                        }
                                        if isMatch != 0 {
                                            if iColumn == XN_ROWID {
                                                distinctColumns = 1 as u8_0;
                                            }
                                            obSat
                                                |= ((1 as ::core::ffi::c_int as Bitmask) << i)
                                                    as ::core::ffi::c_ulonglong;
                                        } else {
                                            if j == 0 as ::core::ffi::c_int
                                                || j < nKeyCol as ::core::ffi::c_int
                                            {
                                                isOrderDistinct = 0 as u8_0;
                                            }
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                                j += 1;
                            }
                            if distinctColumns != 0 {
                                isOrderDistinct = 1 as u8_0;
                            }
                        }
                        if isOrderDistinct != 0 {
                            orderDistinctMask
                                |= (*pLoop).maskSelf as ::core::ffi::c_ulonglong;
                            i = 0 as ::core::ffi::c_int;
                            while i < nOrderBy as ::core::ffi::c_int {
                                let mut p: *mut Expr = ::core::ptr::null_mut::<Expr>();
                                let mut mTerm: Bitmask = 0;
                                if !((1 as ::core::ffi::c_int as Bitmask) << i & obSat != 0)
                                {
                                    p = (*(&raw mut (*pOrderBy).a as *mut ExprList_item)
                                        .offset(i as isize))
                                        .pExpr;
                                    mTerm = sqlite3WhereExprUsage(
                                        &raw mut (*pWInfo).sMaskSet,
                                        p,
                                    );
                                    if !(mTerm == 0 as ::core::ffi::c_ulonglong
                                        && sqlite3ExprIsConstant(
                                            ::core::ptr::null_mut::<Parse>(),
                                            p,
                                        ) == 0)
                                    {
                                        if mTerm & !orderDistinctMask
                                            == 0 as ::core::ffi::c_ulonglong
                                        {
                                            obSat
                                                |= ((1 as ::core::ffi::c_int as Bitmask) << i)
                                                    as ::core::ffi::c_ulonglong;
                                        }
                                    }
                                }
                                i += 1;
                            }
                        }
                    }
                }
                _ => {}
            }
            iLoop += 1;
        }
        if obSat == obDone {
            return nOrderBy as i8_0;
        }
        if isOrderDistinct == 0 {
            i = nOrderBy as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            while i > 0 as ::core::ffi::c_int {
                let mut m: Bitmask = if i
                    < (::core::mem::size_of::<Bitmask>() as usize)
                        .wrapping_mul(8 as usize) as ::core::ffi::c_int
                {
                    ((1 as ::core::ffi::c_int as Bitmask) << i)
                        .wrapping_sub(1 as Bitmask)
                } else {
                    0 as Bitmask
                };
                if obSat & m == m {
                    return i as i8_0;
                }
                i -= 1;
            }
            return 0 as i8_0;
        }
        return -1 as ::core::ffi::c_int as i8_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereIsSorted(
    mut pWInfo: *mut WhereInfo,
) -> ::core::ffi::c_int {
    unsafe {
        return (*pWInfo).sorted() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn whereSortingCost(
    mut pWInfo: *mut WhereInfo,
    mut nRow: LogEst,
    mut nOrderBy: ::core::ffi::c_int,
    mut nSorted: ::core::ffi::c_int,
) -> LogEst {
    unsafe {
        let mut rSortCost: LogEst = 0;
        let mut nCol: LogEst = 0;
        nCol = sqlite3LogEst(
            (((*(*(*pWInfo).pSelect).pEList).nExpr + 59 as ::core::ffi::c_int)
                / 30 as ::core::ffi::c_int) as u64_0,
        );
        rSortCost = (nRow as ::core::ffi::c_int + nCol as ::core::ffi::c_int) as LogEst;
        if nSorted > 0 as ::core::ffi::c_int {
            rSortCost = (rSortCost as ::core::ffi::c_int
                + (sqlite3LogEst(
                    ((nOrderBy - nSorted) * 100 as ::core::ffi::c_int / nOrderBy)
                        as u64_0,
                ) as ::core::ffi::c_int - 66 as ::core::ffi::c_int)) as LogEst;
        }
        if (*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_USE_LIMIT
            != 0 as ::core::ffi::c_int
        {
            rSortCost = (rSortCost as ::core::ffi::c_int + 10 as ::core::ffi::c_int)
                as LogEst;
            if nSorted != 0 as ::core::ffi::c_int {
                rSortCost = (rSortCost as ::core::ffi::c_int + 6 as ::core::ffi::c_int)
                    as LogEst;
            }
            if ((*pWInfo).iLimit as ::core::ffi::c_int) < nRow as ::core::ffi::c_int {
                nRow = (*pWInfo).iLimit;
            }
        } else if (*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_WANT_DISTINCT != 0 {
            if nRow as ::core::ffi::c_int > 10 as ::core::ffi::c_int {
                nRow = (nRow as ::core::ffi::c_int - 10 as ::core::ffi::c_int) as LogEst;
            }
        }
        rSortCost = (rSortCost as ::core::ffi::c_int
            + estLog(nRow) as ::core::ffi::c_int) as LogEst;
        return rSortCost;
    }
}
unsafe extern "C" fn computeMxChoice(mut pWInfo: *mut WhereInfo) -> ::core::ffi::c_int {
    unsafe {
        let mut nLoop: ::core::ffi::c_int = (*pWInfo).nLevel as ::core::ffi::c_int;
        let mut pWLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        if nLoop >= 5 as ::core::ffi::c_int && (*pWInfo).bStarDone() == 0
            && (*(*(*pWInfo).pParse).db).dbOptFlags & 0x20000000 as u32_0 == 0 as u32_0
        {
            let mut aFromTabs: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
            let mut iFromIdx: ::core::ffi::c_int = 0;
            let mut m: Bitmask = 0;
            let mut mSelfJoin: Bitmask = 0 as Bitmask;
            let mut pStart: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
            (*pWInfo).set_bStarDone(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            aFromTabs = &raw mut (*(*pWInfo).pTabList).a as *mut SrcItem;
            pStart = (*pWInfo).pLoops;
            iFromIdx = 0 as ::core::ffi::c_int;
            m = 1 as Bitmask;
            while iFromIdx < nLoop {
                let mut nDep: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut mxRun: LogEst = 0;
                let mut mSeen: Bitmask = 0 as Bitmask;
                let mut pFactTab: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
                pFactTab = aFromTabs.offset(iFromIdx as isize);
                if (*pFactTab).fg.jointype as ::core::ffi::c_int & (JT_OUTER | JT_CROSS)
                    != 0 as ::core::ffi::c_int
                {
                    if iFromIdx + 4 as ::core::ffi::c_int > nLoop {
                        break;
                    }
                    while !pStart.is_null()
                        && (*pStart).iTab as ::core::ffi::c_int <= iFromIdx
                    {
                        pStart = (*pStart).pNextLoop;
                    }
                }
                pWLoop = pStart;
                while !pWLoop.is_null() {
                    if (*aFromTabs.offset((*pWLoop).iTab as isize)).fg.jointype
                        as ::core::ffi::c_int & (JT_OUTER | JT_CROSS)
                        != 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                    if (*pWLoop).prereq & m != 0 as ::core::ffi::c_ulonglong
                        && (*pWLoop).maskSelf & mSeen == 0 as ::core::ffi::c_ulonglong
                        && (*pWLoop).maskSelf & mSelfJoin
                            == 0 as ::core::ffi::c_ulonglong
                    {
                        if (*aFromTabs.offset((*pWLoop).iTab as isize)).pSTab
                            == (*pFactTab).pSTab
                        {
                            mSelfJoin |= m as ::core::ffi::c_ulonglong;
                        } else {
                            nDep += 1;
                            mSeen |= (*pWLoop).maskSelf as ::core::ffi::c_ulonglong;
                        }
                    }
                    pWLoop = (*pWLoop).pNextLoop;
                }
                if !(nDep <= 3 as ::core::ffi::c_int) {
                    (*pWInfo)
                        .set_bStarUsed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    mxRun = LOGEST_MIN as LogEst;
                    pWLoop = pStart;
                    while !pWLoop.is_null() {
                        if !(((*pWLoop).iTab as ::core::ffi::c_int) < iFromIdx) {
                            if (*pWLoop).iTab as ::core::ffi::c_int > iFromIdx {
                                break;
                            }
                            if (*pWLoop).rRun as ::core::ffi::c_int
                                > mxRun as ::core::ffi::c_int
                            {
                                mxRun = (*pWLoop).rRun;
                            }
                        }
                        pWLoop = (*pWLoop).pNextLoop;
                    }
                    if (mxRun as ::core::ffi::c_int) < 32767 as ::core::ffi::c_int {
                        mxRun += 1;
                    }
                    pWLoop = pStart;
                    while !pWLoop.is_null() {
                        if !((*pWLoop).maskSelf & mSeen == 0 as ::core::ffi::c_ulonglong)
                        {
                            if !((*pWLoop).nLTerm != 0) {
                                if ((*pWLoop).rRun as ::core::ffi::c_int)
                                    < mxRun as ::core::ffi::c_int
                                {
                                    (*pWLoop).rRun = mxRun;
                                }
                            }
                        }
                        pWLoop = (*pWLoop).pNextLoop;
                    }
                }
                iFromIdx += 1;
                m <<= 1 as ::core::ffi::c_int;
            }
        }
        return if (*pWInfo).bStarUsed() as ::core::ffi::c_int != 0 {
            18 as ::core::ffi::c_int
        } else {
            12 as ::core::ffi::c_int
        };
    }
}
#[inline(never)]
unsafe extern "C" fn whereLoopIsNoBetter(
    mut pCandidate: *const WhereLoop,
    mut pBaseline: *const WhereLoop,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pCandidate).wsFlags & WHERE_INDEXED as u32_0 == 0 as u32_0 {
            return 1 as ::core::ffi::c_int;
        }
        if (*pBaseline).wsFlags & WHERE_INDEXED as u32_0 == 0 as u32_0 {
            return 1 as ::core::ffi::c_int;
        }
        if ((*(*pCandidate).u.btree.pIndex).szIdxRow as ::core::ffi::c_int)
            < (*(*pBaseline).u.btree.pIndex).szIdxRow as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn wherePathSolver(
    mut pWInfo: *mut WhereInfo,
    mut nRowEst: LogEst,
) -> ::core::ffi::c_int {
    unsafe {
        let mut mxChoice: ::core::ffi::c_int = 0;
        let mut nLoop: ::core::ffi::c_int = 0;
        let mut pParse: *mut Parse = ::core::ptr::null_mut::<Parse>();
        let mut iLoop: ::core::ffi::c_int = 0;
        let mut ii: ::core::ffi::c_int = 0;
        let mut jj: ::core::ffi::c_int = 0;
        let mut mxI: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nOrderBy: ::core::ffi::c_int = 0;
        let mut mxCost: LogEst = 0 as LogEst;
        let mut mxUnsort: LogEst = 0 as LogEst;
        let mut nTo: ::core::ffi::c_int = 0;
        let mut nFrom: ::core::ffi::c_int = 0;
        let mut aFrom: *mut WherePath = ::core::ptr::null_mut::<WherePath>();
        let mut aTo: *mut WherePath = ::core::ptr::null_mut::<WherePath>();
        let mut pFrom: *mut WherePath = ::core::ptr::null_mut::<WherePath>();
        let mut pTo: *mut WherePath = ::core::ptr::null_mut::<WherePath>();
        let mut pWLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut pX: *mut *mut WhereLoop = ::core::ptr::null_mut::<*mut WhereLoop>();
        let mut aSortCost: *mut LogEst = ::core::ptr::null_mut::<LogEst>();
        let mut pSpace: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nSpace: ::core::ffi::c_int = 0;
        pParse = (*pWInfo).pParse;
        nLoop = (*pWInfo).nLevel as ::core::ffi::c_int;
        if nLoop <= 1 as ::core::ffi::c_int {
            mxChoice = 1 as ::core::ffi::c_int;
        } else if nLoop == 2 as ::core::ffi::c_int {
            mxChoice = 5 as ::core::ffi::c_int;
        } else if (*pParse).nErr != 0 {
            mxChoice = 1 as ::core::ffi::c_int;
        } else {
            mxChoice = computeMxChoice(pWInfo);
        }
        if (*pWInfo).pOrderBy.is_null()
            || nRowEst as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            nOrderBy = 0 as ::core::ffi::c_int;
        } else {
            nOrderBy = (*(*pWInfo).pOrderBy).nExpr;
        }
        nSpace = (::core::mem::size_of::<WherePath>() as usize)
            .wrapping_add(
                (::core::mem::size_of::<*mut WhereLoop>() as usize)
                    .wrapping_mul(nLoop as usize),
            )
            .wrapping_mul(mxChoice as usize)
            .wrapping_mul(2 as usize) as ::core::ffi::c_int;
        nSpace = (nSpace as ::core::ffi::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<LogEst>() as usize)
                    .wrapping_mul(nOrderBy as usize) as ::core::ffi::c_ulong,
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
        pSpace = sqlite3DbMallocRawNN((*pParse).db, nSpace as u64_0)
            as *mut ::core::ffi::c_char;
        if pSpace.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        aTo = pSpace as *mut WherePath;
        aFrom = aTo.offset(mxChoice as isize);
        memset(
            aFrom as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<WherePath>() as size_t,
        );
        pX = aFrom.offset(mxChoice as isize) as *mut *mut WhereLoop;
        ii = mxChoice * 2 as ::core::ffi::c_int;
        pFrom = aTo;
        while ii > 0 as ::core::ffi::c_int {
            (*pFrom).aLoop = pX;
            ii -= 1;
            pFrom = pFrom.offset(1);
            pX = pX.offset(nLoop as isize);
        }
        if nOrderBy != 0 {
            aSortCost = pX as *mut LogEst;
            memset(
                aSortCost as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<LogEst>() as size_t)
                    .wrapping_mul(nOrderBy as size_t),
            );
        }
        (*aFrom.offset(0 as ::core::ffi::c_int as isize)).nRow = (if ((*pParse)
            .nQueryLoop as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            (*pParse).nQueryLoop as ::core::ffi::c_int
        } else {
            48 as ::core::ffi::c_int
        }) as LogEst;
        nFrom = 1 as ::core::ffi::c_int;
        if nOrderBy != 0 {
            (*aFrom.offset(0 as ::core::ffi::c_int as isize)).isOrdered = (if nLoop
                > 0 as ::core::ffi::c_int
            {
                -1 as ::core::ffi::c_int
            } else {
                nOrderBy
            }) as i8_0;
        }
        iLoop = 0 as ::core::ffi::c_int;
        while iLoop < nLoop {
            nTo = 0 as ::core::ffi::c_int;
            ii = 0 as ::core::ffi::c_int;
            pFrom = aFrom;
            while ii < nFrom {
                let mut c2rust_current_block_107: u64;
                pWLoop = (*pWInfo).pLoops;
                while !pWLoop.is_null() {
                    let mut nOut: LogEst = 0;
                    let mut rCost: LogEst = 0;
                    let mut rUnsort: LogEst = 0;
                    let mut isOrdered: i8_0 = 0;
                    let mut maskNew: Bitmask = 0;
                    let mut revMask: Bitmask = 0;
                    if !((*pWLoop).prereq & !(*pFrom).maskLoop
                        != 0 as ::core::ffi::c_ulonglong)
                    {
                        if !((*pWLoop).maskSelf & (*pFrom).maskLoop
                            != 0 as ::core::ffi::c_ulonglong)
                        {
                            if !((*pWLoop).wsFlags & WHERE_AUTO_INDEX as u32_0
                                != 0 as u32_0
                                && ((*pFrom).nRow as ::core::ffi::c_int)
                                    < 3 as ::core::ffi::c_int)
                            {
                                rUnsort = ((*pWLoop).rRun as ::core::ffi::c_int
                                    + (*pFrom).nRow as ::core::ffi::c_int) as LogEst;
                                if (*pWLoop).rSetup != 0 {
                                    rUnsort = sqlite3LogEstAdd((*pWLoop).rSetup, rUnsort);
                                }
                                rUnsort = sqlite3LogEstAdd(rUnsort, (*pFrom).rUnsort);
                                nOut = ((*pFrom).nRow as ::core::ffi::c_int
                                    + (*pWLoop).nOut as ::core::ffi::c_int) as LogEst;
                                maskNew = (*pFrom).maskLoop | (*pWLoop).maskSelf;
                                isOrdered = (*pFrom).isOrdered;
                                if (isOrdered as ::core::ffi::c_int)
                                    < 0 as ::core::ffi::c_int
                                {
                                    revMask = 0 as Bitmask;
                                    isOrdered = wherePathSatisfiesOrderBy(
                                        pWInfo,
                                        (*pWInfo).pOrderBy,
                                        pFrom,
                                        (*pWInfo).wctrlFlags,
                                        iLoop as u16_0,
                                        pWLoop,
                                        &raw mut revMask,
                                    );
                                } else {
                                    revMask = (*pFrom).revLoop;
                                }
                                if isOrdered as ::core::ffi::c_int
                                    >= 0 as ::core::ffi::c_int
                                    && (isOrdered as ::core::ffi::c_int) < nOrderBy
                                {
                                    if *aSortCost.offset(isOrdered as isize)
                                        as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                    {
                                        *aSortCost.offset(isOrdered as isize) = whereSortingCost(
                                            pWInfo,
                                            nRowEst,
                                            nOrderBy,
                                            isOrdered as ::core::ffi::c_int,
                                        );
                                    }
                                    rCost = (sqlite3LogEstAdd(
                                        rUnsort,
                                        *aSortCost.offset(isOrdered as isize),
                                    ) as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                        as LogEst;
                                } else {
                                    rCost = rUnsort;
                                    rUnsort = (rUnsort as ::core::ffi::c_int
                                        - 2 as ::core::ffi::c_int) as LogEst;
                                }
                                jj = 0 as ::core::ffi::c_int;
                                pTo = aTo;
                                while jj < nTo {
                                    if (*pTo).maskLoop == maskNew
                                        && (((*pTo).isOrdered as ::core::ffi::c_int
                                            ^ isOrdered as ::core::ffi::c_int)
                                            & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                            || iLoop == nLoop - 1 as ::core::ffi::c_int)
                                    {
                                        break;
                                    }
                                    jj += 1;
                                    pTo = pTo.offset(1);
                                }
                                if jj >= nTo {
                                    if nTo >= mxChoice
                                        && (rCost as ::core::ffi::c_int
                                            > mxCost as ::core::ffi::c_int
                                            || rCost as ::core::ffi::c_int
                                                == mxCost as ::core::ffi::c_int
                                                && rUnsort as ::core::ffi::c_int
                                                    >= mxUnsort as ::core::ffi::c_int)
                                    {
                                        c2rust_current_block_107 = 2122094917359643297;
                                    } else {
                                        if nTo < mxChoice {
                                            let c2rust_fresh10 = nTo;
                                            nTo = nTo + 1;
                                            jj = c2rust_fresh10;
                                        } else {
                                            jj = mxI;
                                        }
                                        pTo = aTo.offset(jj as isize) as *mut WherePath;
                                        c2rust_current_block_107 = 15864857819291709765;
                                    }
                                } else if ((*pTo).rCost as ::core::ffi::c_int)
                                    < rCost as ::core::ffi::c_int
                                    || (*pTo).rCost as ::core::ffi::c_int
                                        == rCost as ::core::ffi::c_int
                                        && ((*pTo).nRow as ::core::ffi::c_int)
                                            < nOut as ::core::ffi::c_int
                                    || (*pTo).rCost as ::core::ffi::c_int
                                        == rCost as ::core::ffi::c_int
                                        && (*pTo).nRow as ::core::ffi::c_int
                                            == nOut as ::core::ffi::c_int
                                        && ((*pTo).rUnsort as ::core::ffi::c_int)
                                            < rUnsort as ::core::ffi::c_int
                                    || (*pTo).rCost as ::core::ffi::c_int
                                        == rCost as ::core::ffi::c_int
                                        && (*pTo).nRow as ::core::ffi::c_int
                                            == nOut as ::core::ffi::c_int
                                        && (*pTo).rUnsort as ::core::ffi::c_int
                                            == rUnsort as ::core::ffi::c_int
                                        && whereLoopIsNoBetter(
                                            pWLoop,
                                            *(*pTo).aLoop.offset(iLoop as isize),
                                        ) != 0
                                {
                                    c2rust_current_block_107 = 2122094917359643297;
                                } else {
                                    c2rust_current_block_107 = 15864857819291709765;
                                }
                                match c2rust_current_block_107 {
                                    2122094917359643297 => {}
                                    _ => {
                                        (*pTo).maskLoop = (*pFrom).maskLoop | (*pWLoop).maskSelf;
                                        (*pTo).revLoop = revMask;
                                        (*pTo).nRow = nOut;
                                        (*pTo).rCost = rCost;
                                        (*pTo).rUnsort = rUnsort;
                                        (*pTo).isOrdered = isOrdered;
                                        memcpy(
                                            (*pTo).aLoop as *mut ::core::ffi::c_void,
                                            (*pFrom).aLoop as *const ::core::ffi::c_void,
                                            (::core::mem::size_of::<*mut WhereLoop>() as size_t)
                                                .wrapping_mul(iLoop as size_t),
                                        );
                                        let ref mut c2rust_fresh11 = *(*pTo)
                                            .aLoop
                                            .offset(iLoop as isize);
                                        *c2rust_fresh11 = pWLoop;
                                        if nTo >= mxChoice {
                                            mxI = 0 as ::core::ffi::c_int;
                                            mxCost = (*aTo.offset(0 as ::core::ffi::c_int as isize))
                                                .rCost;
                                            mxUnsort = (*aTo.offset(0 as ::core::ffi::c_int as isize))
                                                .nRow;
                                            jj = 1 as ::core::ffi::c_int;
                                            pTo = aTo.offset(1 as ::core::ffi::c_int as isize)
                                                as *mut WherePath;
                                            while jj < mxChoice {
                                                if (*pTo).rCost as ::core::ffi::c_int
                                                    > mxCost as ::core::ffi::c_int
                                                    || (*pTo).rCost as ::core::ffi::c_int
                                                        == mxCost as ::core::ffi::c_int
                                                        && (*pTo).rUnsort as ::core::ffi::c_int
                                                            > mxUnsort as ::core::ffi::c_int
                                                {
                                                    mxCost = (*pTo).rCost;
                                                    mxUnsort = (*pTo).rUnsort;
                                                    mxI = jj;
                                                }
                                                jj += 1;
                                                pTo = pTo.offset(1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    pWLoop = (*pWLoop).pNextLoop;
                }
                ii += 1;
                pFrom = pFrom.offset(1);
            }
            pFrom = aTo;
            aTo = aFrom;
            aFrom = pFrom;
            nFrom = nTo;
            iLoop += 1;
        }
        if nFrom == 0 as ::core::ffi::c_int {
            sqlite3ErrorMsg(
                pParse,
                b"no query solution\0".as_ptr() as *const ::core::ffi::c_char,
            );
            sqlite3DbFreeNN((*pParse).db, pSpace as *mut ::core::ffi::c_void);
            return SQLITE_ERROR;
        }
        pFrom = aFrom;
        iLoop = 0 as ::core::ffi::c_int;
        while iLoop < nLoop {
            let mut pLevel: *mut WhereLevel = (&raw mut (*pWInfo).a as *mut WhereLevel)
                .offset(iLoop as isize);
            pWLoop = *(*pFrom).aLoop.offset(iLoop as isize);
            (*pLevel).pWLoop = pWLoop as *mut WhereLoop;
            (*pLevel).iFrom = (*pWLoop).iTab;
            (*pLevel).iTabCur = (*(&raw mut (*(*pWInfo).pTabList).a as *mut SrcItem)
                .offset((*pLevel).iFrom as isize))
                .iCursor;
            iLoop += 1;
        }
        if (*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_WANT_DISTINCT
            != 0 as ::core::ffi::c_int
            && (*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_DISTINCTBY
                == 0 as ::core::ffi::c_int
            && (*pWInfo).eDistinct as ::core::ffi::c_int == WHERE_DISTINCT_NOOP
            && nRowEst as ::core::ffi::c_int != 0
        {
            let mut notUsed: Bitmask = 0;
            let mut rc: ::core::ffi::c_int = wherePathSatisfiesOrderBy(
                pWInfo,
                (*pWInfo).pResultSet,
                pFrom,
                WHERE_DISTINCTBY as u16_0,
                (nLoop - 1 as ::core::ffi::c_int) as u16_0,
                *(*pFrom).aLoop.offset((nLoop - 1 as ::core::ffi::c_int) as isize),
                &raw mut notUsed,
            ) as ::core::ffi::c_int;
            if rc == (*(*pWInfo).pResultSet).nExpr {
                (*pWInfo).eDistinct = WHERE_DISTINCT_ORDERED as u8_0;
            }
        }
        (*pWInfo).set_bOrderedInnerLoop(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if !(*pWInfo).pOrderBy.is_null() {
            (*pWInfo).nOBSat = (*pFrom).isOrdered;
            if (*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_DISTINCTBY != 0 {
                if (*pFrom).isOrdered as ::core::ffi::c_int
                    == (*(*pWInfo).pOrderBy).nExpr
                {
                    (*pWInfo).eDistinct = WHERE_DISTINCT_ORDERED as u8_0;
                }
            } else {
                (*pWInfo).revMask = (*pFrom).revLoop;
                if (*pWInfo).nOBSat as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                    (*pWInfo).nOBSat = 0 as i8_0;
                    if nLoop > 0 as ::core::ffi::c_int {
                        let mut wsFlags: u32_0 = (**(*pFrom)
                            .aLoop
                            .offset((nLoop - 1 as ::core::ffi::c_int) as isize))
                            .wsFlags;
                        if wsFlags & WHERE_ONEROW as u32_0 == 0 as u32_0
                            && wsFlags & (WHERE_IPK | WHERE_COLUMN_IN) as u32_0
                                != (WHERE_IPK | WHERE_COLUMN_IN) as u32_0
                        {
                            let mut m: Bitmask = 0 as Bitmask;
                            let mut rc_0: ::core::ffi::c_int = wherePathSatisfiesOrderBy(
                                pWInfo,
                                (*pWInfo).pOrderBy,
                                pFrom,
                                WHERE_ORDERBY_LIMIT as u16_0,
                                (nLoop - 1 as ::core::ffi::c_int) as u16_0,
                                *(*pFrom)
                                    .aLoop
                                    .offset((nLoop - 1 as ::core::ffi::c_int) as isize),
                                &raw mut m,
                            ) as ::core::ffi::c_int;
                            if rc_0 == (*(*pWInfo).pOrderBy).nExpr {
                                (*pWInfo)
                                    .set_bOrderedInnerLoop(
                                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                    );
                                (*pWInfo).revMask = m;
                            }
                        }
                    }
                } else if nLoop != 0
                    && (*pWInfo).nOBSat as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                    && (*pWInfo).wctrlFlags as ::core::ffi::c_int
                        & (WHERE_ORDERBY_MIN | WHERE_ORDERBY_MAX)
                        != 0 as ::core::ffi::c_int
                {
                    (*pWInfo)
                        .set_bOrderedInnerLoop(
                            1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                        );
                }
            }
            if (*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_SORTBYGROUP != 0
                && (*pWInfo).nOBSat as ::core::ffi::c_int == (*(*pWInfo).pOrderBy).nExpr
                && nLoop > 0 as ::core::ffi::c_int
            {
                let mut revMask_0: Bitmask = 0 as Bitmask;
                let mut nOrder: ::core::ffi::c_int = wherePathSatisfiesOrderBy(
                    pWInfo,
                    (*pWInfo).pOrderBy,
                    pFrom,
                    0 as u16_0,
                    (nLoop - 1 as ::core::ffi::c_int) as u16_0,
                    *(*pFrom).aLoop.offset((nLoop - 1 as ::core::ffi::c_int) as isize),
                    &raw mut revMask_0,
                ) as ::core::ffi::c_int;
                if nOrder == (*(*pWInfo).pOrderBy).nExpr {
                    (*pWInfo)
                        .set_sorted(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*pWInfo).revMask = revMask_0;
                }
            }
        }
        (*pWInfo).nRowOut = (*pFrom).nRow;
        sqlite3DbFreeNN((*pParse).db, pSpace as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
#[inline(never)]
unsafe extern "C" fn whereInterstageHeuristic(mut pWInfo: *mut WhereInfo) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pWInfo).nLevel as ::core::ffi::c_int {
            let mut p: *mut WhereLoop = (*(&raw mut (*pWInfo).a as *mut WhereLevel)
                .offset(i as isize))
                .pWLoop as *mut WhereLoop;
            if p.is_null() {
                break;
            }
            if (*p).wsFlags & WHERE_VIRTUALTABLE as u32_0 != 0 as u32_0 {
                break;
            }
            if !((*p).wsFlags
                & (WHERE_COLUMN_EQ | WHERE_COLUMN_NULL | WHERE_COLUMN_IN) as u32_0
                != 0 as u32_0)
            {
                break;
            }
            let mut iTab: u8_0 = (*p).iTab;
            let mut pLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
            pLoop = (*pWInfo).pLoops;
            while !pLoop.is_null() {
                if !((*pLoop).iTab as ::core::ffi::c_int != iTab as ::core::ffi::c_int) {
                    if !((*pLoop).wsFlags
                        & (WHERE_CONSTRAINT | WHERE_AUTO_INDEX) as u32_0 != 0 as u32_0)
                    {
                        (*pLoop).prereq = ALLBITS;
                    }
                }
                pLoop = (*pLoop).pNextLoop;
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn whereShortCut(
    mut pBuilder: *mut WhereLoopBuilder,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pWInfo: *mut WhereInfo = ::core::ptr::null_mut::<WhereInfo>();
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut pWC: *mut WhereClause = ::core::ptr::null_mut::<WhereClause>();
        let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
        let mut pLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut iCur: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut scan: WhereScan = WhereScan {
            pOrigWC: ::core::ptr::null_mut::<WhereClause>(),
            pWC: ::core::ptr::null_mut::<WhereClause>(),
            zCollName: ::core::ptr::null::<::core::ffi::c_char>(),
            pIdxExpr: ::core::ptr::null_mut::<Expr>(),
            k: 0,
            opMask: 0,
            idxaff: 0,
            iEquiv: 0,
            nEquiv: 0,
            aiCur: [0; 11],
            aiColumn: [0; 11]

        };
        pWInfo = (*pBuilder).pWInfo;
        if (*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_OR_SUBCLAUSE != 0 {
            return 0 as ::core::ffi::c_int;
        }
        pItem = &raw mut (*(*pWInfo).pTabList).a as *mut SrcItem;
        pTab = (*pItem).pSTab;
        if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
            return 0 as ::core::ffi::c_int;
        }
        if (*pItem).fg.isIndexedBy() as ::core::ffi::c_int != 0
            || (*pItem).fg.notIndexed() as ::core::ffi::c_int != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        iCur = (*pItem).iCursor;
        pWC = &raw mut (*pWInfo).sWC;
        pLoop = (*pBuilder).pNew;
        (*pLoop).wsFlags = 0 as u32_0;
        (*pLoop).nSkip = 0 as u16_0;
        pTerm = whereScanInit(
            &raw mut scan,
            pWC,
            iCur,
            -1 as ::core::ffi::c_int,
            (WO_EQ | WO_IS) as u32_0,
            ::core::ptr::null_mut::<Index>(),
        );
        while !pTerm.is_null() && (*pTerm).prereqRight != 0 {
            pTerm = whereScanNext(&raw mut scan);
        }
        if !pTerm.is_null() {
            (*pLoop).wsFlags = (WHERE_COLUMN_EQ | WHERE_IPK | WHERE_ONEROW) as u32_0;
            let ref mut c2rust_fresh23 = *(*pLoop)
                .aLTerm
                .offset(0 as ::core::ffi::c_int as isize);
            *c2rust_fresh23 = pTerm;
            (*pLoop).nLTerm = 1 as u16_0;
            (*pLoop).u.btree.nEq = 1 as u16_0;
            (*pLoop).rRun = 33 as LogEst;
        } else {
            pIdx = (*pTab).pIndex;
            while !pIdx.is_null() {
                let mut opMask: ::core::ffi::c_int = 0;
                if !(!((*pIdx).onError as ::core::ffi::c_int != OE_None)
                    || !(*pIdx).pPartIdxWhere.is_null()
                    || (*pIdx).nKeyCol as ::core::ffi::c_int
                        > (::core::mem::size_of::<[*mut WhereTerm; 3]>() as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut WhereTerm>() as usize,
                            ) as ::core::ffi::c_int)
                {
                    opMask = if (*pIdx).uniqNotNull() as ::core::ffi::c_int != 0 {
                        WO_EQ | WO_IS
                    } else {
                        WO_EQ
                    };
                    j = 0 as ::core::ffi::c_int;
                    while j < (*pIdx).nKeyCol as ::core::ffi::c_int {
                        pTerm = whereScanInit(
                            &raw mut scan,
                            pWC,
                            iCur,
                            j,
                            opMask as u32_0,
                            pIdx,
                        );
                        while !pTerm.is_null() && (*pTerm).prereqRight != 0 {
                            pTerm = whereScanNext(&raw mut scan);
                        }
                        if pTerm.is_null() {
                            break;
                        }
                        let ref mut c2rust_fresh24 = *(*pLoop).aLTerm.offset(j as isize);
                        *c2rust_fresh24 = pTerm;
                        j += 1;
                    }
                    if !(j != (*pIdx).nKeyCol as ::core::ffi::c_int) {
                        (*pLoop).wsFlags = (WHERE_COLUMN_EQ | WHERE_ONEROW
                            | WHERE_INDEXED) as u32_0;
                        if (*pIdx).isCovering() as ::core::ffi::c_int != 0
                            || (*pItem).colUsed & (*pIdx).colNotIdxed
                                == 0 as ::core::ffi::c_ulonglong
                        {
                            (*pLoop).wsFlags = ((*pLoop).wsFlags as ::core::ffi::c_uint
                                | WHERE_IDX_ONLY as ::core::ffi::c_uint) as u32_0;
                        }
                        (*pLoop).nLTerm = j as u16_0;
                        (*pLoop).u.btree.nEq = j as u16_0;
                        (*pLoop).u.btree.pIndex = pIdx;
                        (*pLoop).rRun = 39 as LogEst;
                        break;
                    }
                }
                pIdx = (*pIdx).pNext;
            }
        }
        if (*pLoop).wsFlags != 0 {
            (*pLoop).nOut = 1 as ::core::ffi::c_int as LogEst;
            let ref mut c2rust_fresh25 = (*(&raw mut (*pWInfo).a as *mut WhereLevel)
                .offset(0 as ::core::ffi::c_int as isize))
                .pWLoop;
            *c2rust_fresh25 = pLoop as *mut WhereLoop;
            (*pLoop).maskSelf = 1 as Bitmask;
            (*(&raw mut (*pWInfo).a as *mut WhereLevel)
                .offset(0 as ::core::ffi::c_int as isize))
                .iTabCur = iCur;
            (*pWInfo).nRowOut = 1 as LogEst;
            if !(*pWInfo).pOrderBy.is_null() {
                (*pWInfo).nOBSat = (*(*pWInfo).pOrderBy).nExpr as i8_0;
            }
            if (*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_WANT_DISTINCT != 0 {
                (*pWInfo).eDistinct = WHERE_DISTINCT_UNIQUE as u8_0;
            }
            if scan.iEquiv as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                (*pLoop).wsFlags = ((*pLoop).wsFlags as ::core::ffi::c_uint
                    | WHERE_TRANSCONS as ::core::ffi::c_uint) as u32_0;
            }
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn exprNodeIsDeterministic(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int == TK_FUNCTION
            && ((*pExpr).flags & 0x100000 as ::core::ffi::c_int as u32_0 != 0 as u32_0)
                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*pWalker).eCode = 0 as u16_0;
            return WRC_Abort;
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn exprIsDeterministic(mut p: *mut Expr) -> ::core::ffi::c_int {
    unsafe {
        let mut w: Walker = Walker {
            pParse: ::core::ptr::null_mut::<Parse>(),
            xExprCallback: None,
            xSelectCallback: None,
            xSelectCallback2: None,
            walkerDepth: 0,
            eCode: 0,
            mWFlags: 0,
            u: C2Rust_Unnamed_29 {
                pNC: ::core::ptr::null_mut::<NameContext>()

            }

        };
        memset(
            &raw mut w as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        w.eCode = 1 as u16_0;
        w.xExprCallback = Some(
            exprNodeIsDeterministic
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = Some(
            sqlite3SelectWalkFail
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        sqlite3WalkExpr(&raw mut w, p);
        return w.eCode as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn whereOmitNoopJoin(
    mut pWInfo: *mut WhereInfo,
    mut notReady: Bitmask,
) -> Bitmask {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut tabUsed: Bitmask = 0;
        let mut hasRightJoin: ::core::ffi::c_int = 0;
        tabUsed = sqlite3WhereExprListUsage(
            &raw mut (*pWInfo).sMaskSet,
            (*pWInfo).pResultSet,
        );
        if !(*pWInfo).pOrderBy.is_null() {
            tabUsed
                |= sqlite3WhereExprListUsage(
                    &raw mut (*pWInfo).sMaskSet,
                    (*pWInfo).pOrderBy,
                ) as ::core::ffi::c_ulonglong;
        }
        hasRightJoin = ((*(&raw mut (*(*pWInfo).pTabList).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .fg
            .jointype as ::core::ffi::c_int & JT_LTORJ != 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        i = (*pWInfo).nLevel as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i >= 1 as ::core::ffi::c_int {
            let mut pTerm: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
            let mut pEnd: *mut WhereTerm = ::core::ptr::null_mut::<WhereTerm>();
            let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
            let mut pLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
            let mut m1: Bitmask = 0;
            pLoop = (*(&raw mut (*pWInfo).a as *mut WhereLevel).offset(i as isize))
                .pWLoop as *mut WhereLoop;
            pItem = (&raw mut (*(*pWInfo).pTabList).a as *mut SrcItem)
                .offset((*pLoop).iTab as isize) as *mut SrcItem;
            if !((*pItem).fg.jointype as ::core::ffi::c_int & (JT_LEFT | JT_RIGHT)
                != JT_LEFT)
            {
                if !((*pWInfo).wctrlFlags as ::core::ffi::c_int & WHERE_WANT_DISTINCT
                    == 0 as ::core::ffi::c_int
                    && (*pLoop).wsFlags & WHERE_ONEROW as u32_0 == 0 as u32_0)
                {
                    if !(tabUsed & (*pLoop).maskSelf != 0 as ::core::ffi::c_ulonglong) {
                        pEnd = (*pWInfo).sWC.a.offset((*pWInfo).sWC.nTerm as isize);
                        pTerm = (*pWInfo).sWC.a;
                        while pTerm < pEnd {
                            if (*pTerm).prereqAll & (*pLoop).maskSelf
                                != 0 as ::core::ffi::c_ulonglong
                            {
                                if !((*(*pTerm).pExpr).flags
                                    & 0x1 as ::core::ffi::c_int as u32_0 != 0 as u32_0)
                                    || (*(*pTerm).pExpr).w.iJoin != (*pItem).iCursor
                                {
                                    break;
                                }
                            }
                            if hasRightJoin != 0
                                && (*(*pTerm).pExpr).flags
                                    & 0x2 as ::core::ffi::c_int as u32_0 != 0 as u32_0
                                && (*(*pTerm).pExpr).w.iJoin == (*pItem).iCursor
                            {
                                break;
                            }
                            pTerm = pTerm.offset(1);
                        }
                        if !(pTerm < pEnd) {
                            m1 = ((1 as ::core::ffi::c_int as ::core::ffi::c_ulonglong)
                                << i)
                                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as Bitmask;
                            (*pWInfo).revMask = m1 & (*pWInfo).revMask
                                | (*pWInfo).revMask >> 1 as ::core::ffi::c_int & !m1;
                            notReady &= !(*pLoop).maskSelf as ::core::ffi::c_ulonglong;
                            pTerm = (*pWInfo).sWC.a;
                            while pTerm < pEnd {
                                if (*pTerm).prereqAll & (*pLoop).maskSelf
                                    != 0 as ::core::ffi::c_ulonglong
                                {
                                    (*pTerm).wtFlags = ((*pTerm).wtFlags as ::core::ffi::c_int
                                        | TERM_CODED) as u16_0;
                                }
                                pTerm = pTerm.offset(1);
                            }
                            if i
                                != (*pWInfo).nLevel as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int
                            {
                                let mut nByte: ::core::ffi::c_int = (((*pWInfo).nLevel
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int - i)
                                    as usize)
                                    .wrapping_mul(::core::mem::size_of::<WhereLevel>() as usize)
                                    as ::core::ffi::c_int;
                                memmove(
                                    (&raw mut (*pWInfo).a as *mut WhereLevel).offset(i as isize)
                                        as *mut WhereLevel as *mut ::core::ffi::c_void,
                                    (&raw mut (*pWInfo).a as *mut WhereLevel)
                                        .offset((i + 1 as ::core::ffi::c_int) as isize)
                                        as *mut WhereLevel as *const ::core::ffi::c_void,
                                    nByte as size_t,
                                );
                            }
                            (*pWInfo).nLevel = (*pWInfo).nLevel.wrapping_sub(1);
                        }
                    }
                }
            }
            i -= 1;
        }
        return notReady;
    }
}
#[inline(never)]
unsafe extern "C" fn whereCheckIfBloomFilterIsUseful(mut pWInfo: *const WhereInfo) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut nSearch: LogEst = 0 as LogEst;
        i = 0 as ::core::ffi::c_int;
        while i < (*pWInfo).nLevel as ::core::ffi::c_int {
            let mut pLoop: *mut WhereLoop = (*(&raw const (*pWInfo).a
                as *const WhereLevel)
                .offset(i as isize))
                .pWLoop as *mut WhereLoop;
            let reqFlags: ::core::ffi::c_uint = (WHERE_SELFCULL | WHERE_COLUMN_EQ)
                as ::core::ffi::c_uint;
            let mut pItem: *mut SrcItem = (&raw mut (*(*pWInfo).pTabList).a
                as *mut SrcItem)
                .offset((*pLoop).iTab as isize) as *mut SrcItem;
            let mut pTab: *mut Table = (*pItem).pSTab;
            if (*pTab).tabFlags & TF_HasStat1 as u32_0 == 0 as u32_0 {
                break;
            }
            (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                | TF_MaybeReanalyze as ::core::ffi::c_uint) as u32_0;
            if i >= 1 as ::core::ffi::c_int
                && (*pLoop).wsFlags & reqFlags as u32_0 == reqFlags as u32_0
                && (*pLoop).wsFlags
                    & (0x100 as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int)
                        as u32_0 != 0 as u32_0
            {
                if nSearch as ::core::ffi::c_int
                    > (*pTab).nRowLogEst as ::core::ffi::c_int
                {
                    (*pLoop).wsFlags = ((*pLoop).wsFlags as ::core::ffi::c_uint
                        | WHERE_BLOOMFILTER as ::core::ffi::c_uint) as u32_0;
                    (*pLoop).wsFlags = ((*pLoop).wsFlags as ::core::ffi::c_uint
                        & !WHERE_IDX_ONLY as ::core::ffi::c_uint) as u32_0;
                }
            }
            nSearch = (nSearch as ::core::ffi::c_int
                + (*pLoop).nOut as ::core::ffi::c_int) as LogEst;
            i += 1;
        }
    }
}
#[inline(never)]
unsafe extern "C" fn whereAddIndexedExpr(
    mut pParse: *mut Parse,
    mut pIdx: *mut Index,
    mut iIdxCur: ::core::ffi::c_int,
    mut pTabItem: *mut SrcItem,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut p: *mut IndexedExpr = ::core::ptr::null_mut::<IndexedExpr>();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        pTab = (*pIdx).pTable;
        let mut c2rust_current_block_20: u64;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdx).nColumn as ::core::ffi::c_int {
            let mut pExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut j: ::core::ffi::c_int = *(*pIdx).aiColumn.offset(i as isize)
                as ::core::ffi::c_int;
            if j == XN_EXPR {
                pExpr = (*(&raw mut (*(*pIdx).aColExpr).a as *mut ExprList_item)
                    .offset(i as isize))
                    .pExpr;
                c2rust_current_block_20 = 1917311967535052937;
            } else if j >= 0 as ::core::ffi::c_int
                && (*(*pTab).aCol.offset(j as isize)).colFlags as ::core::ffi::c_int
                    & COLFLAG_VIRTUAL != 0 as ::core::ffi::c_int
            {
                pExpr = sqlite3ColumnExpr(
                    pTab,
                    (*pTab).aCol.offset(j as isize) as *mut Column,
                );
                c2rust_current_block_20 = 1917311967535052937;
            } else {
                c2rust_current_block_20 = 12517898123489920830;
            }
            match c2rust_current_block_20 {
                1917311967535052937 => {
                    if !(sqlite3ExprIsConstant(::core::ptr::null_mut::<Parse>(), pExpr)
                        != 0)
                    {
                        p = sqlite3DbMallocRaw(
                            (*pParse).db,
                            ::core::mem::size_of::<IndexedExpr>() as u64_0,
                        ) as *mut IndexedExpr;
                        if p.is_null() {
                            break;
                        }
                        (*p).pIENext = (*pParse).pIdxEpr;
                        (*p).pExpr = sqlite3ExprDup(
                            (*pParse).db,
                            pExpr,
                            0 as ::core::ffi::c_int,
                        );
                        (*p).iDataCur = (*pTabItem).iCursor;
                        (*p).iIdxCur = iIdxCur;
                        (*p).iIdxCol = i;
                        (*p).bMaybeNullRow = ((*pTabItem).fg.jointype
                            as ::core::ffi::c_int & (JT_LEFT | JT_LTORJ | JT_RIGHT)
                            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as u8_0;
                        if !sqlite3IndexAffinityStr((*pParse).db, pIdx).is_null() {
                            (*p).aff = *(*pIdx).zColAff.offset(i as isize) as u8_0;
                        }
                        (*pParse).pIdxEpr = p;
                        if (*p).pIENext.is_null() {
                            let mut pArg: *mut ::core::ffi::c_void = &raw mut (*pParse)
                                .pIdxEpr as *mut ::core::ffi::c_void;
                            sqlite3ParserAddCleanup(
                                pParse,
                                Some(
                                    whereIndexedExprCleanup
                                        as unsafe extern "C" fn(
                                            *mut sqlite3,
                                            *mut ::core::ffi::c_void,
                                        ) -> (),
                                ),
                                pArg,
                            );
                        }
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }
}
#[inline(never)]
unsafe extern "C" fn whereReverseScanOrder(mut pWInfo: *mut WhereInfo) {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*(*pWInfo).pTabList).nSrc {
            let mut pItem: *mut SrcItem = (&raw mut (*(*pWInfo).pTabList).a
                as *mut SrcItem)
                .offset(ii as isize) as *mut SrcItem;
            if (*pItem).fg.isCte() == 0
                || (*(*pItem).u2.pCteUse).eM10d as ::core::ffi::c_int != M10d_Yes
                || (*pItem).fg.isSubquery() as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                || (*(*(*pItem).u4.pSubq).pSelect).pOrderBy.is_null()
            {
                (*pWInfo).revMask
                    |= ((1 as ::core::ffi::c_int as Bitmask) << ii)
                        as ::core::ffi::c_ulonglong;
            }
            ii += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereBegin(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pWhere: *mut Expr,
    mut pOrderBy: *mut ExprList,
    mut pResultSet: *mut ExprList,
    mut pSelect: *mut Select,
    mut wctrlFlags: u16_0,
    mut iAuxArg: ::core::ffi::c_int,
) -> *mut WhereInfo {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut nByteWInfo: ::core::ffi::c_int = 0;
        let mut nTabList: ::core::ffi::c_int = 0;
        let mut pWInfo: *mut WhereInfo = ::core::ptr::null_mut::<WhereInfo>();
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut notReady: Bitmask = 0;
        let mut sWLB: WhereLoopBuilder = WhereLoopBuilder {
            pWInfo: ::core::ptr::null_mut::<WhereInfo>(),
            pWC: ::core::ptr::null_mut::<WhereClause>(),
            pNew: ::core::ptr::null_mut::<WhereLoop>(),
            pOrSet: ::core::ptr::null_mut::<WhereOrSet>(),
            bldFlags1: 0,
            bldFlags2: 0,
            iPlanLimit: 0

        };
        let mut pMaskSet: *mut WhereMaskSet = ::core::ptr::null_mut::<WhereMaskSet>();
        let mut pLevel: *mut WhereLevel = ::core::ptr::null_mut::<WhereLevel>();
        let mut pLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut ii: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut bFordelete: u8_0 = 0 as u8_0;
        db = (*pParse).db;
        memset(
            &raw mut sWLB as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<WhereLoopBuilder>() as size_t,
        );
        if !pOrderBy.is_null() && (*pOrderBy).nExpr >= BMS {
            pOrderBy = ::core::ptr::null_mut::<ExprList>();
            wctrlFlags = (wctrlFlags as ::core::ffi::c_int & !WHERE_WANT_DISTINCT)
                as u16_0;
            wctrlFlags = (wctrlFlags as ::core::ffi::c_int | WHERE_KEEP_ALL_JOINS)
                as u16_0;
        }
        if (*pTabList).nSrc > BMS {
            sqlite3ErrorMsg(
                pParse,
                b"at most %d tables in a join\0".as_ptr() as *const ::core::ffi::c_char,
                BMS,
            );
            return ::core::ptr::null_mut::<WhereInfo>();
        }
        nTabList = if wctrlFlags as ::core::ffi::c_int & WHERE_OR_SUBCLAUSE != 0 {
            1 as ::core::ffi::c_int
        } else {
            (*pTabList).nSrc
        };
        nByteWInfo = ((856 as usize)
            .wrapping_add(
                (nTabList as usize)
                    .wrapping_mul(::core::mem::size_of::<WhereLevel>() as usize),
            )
            .wrapping_add(7 as usize) & !(7 as ::core::ffi::c_int) as usize)
            as ::core::ffi::c_int;
        pWInfo = sqlite3DbMallocRawNN(
            db,
            (nByteWInfo as usize)
                .wrapping_add(::core::mem::size_of::<WhereLoop>() as usize) as u64_0,
        ) as *mut WhereInfo;
        if (*db).mallocFailed != 0 {
            sqlite3DbFree(db, pWInfo as *mut ::core::ffi::c_void);
            pWInfo = ::core::ptr::null_mut::<WhereInfo>();
        } else {
            (*pWInfo).pParse = pParse;
            (*pWInfo).pTabList = pTabList;
            (*pWInfo).pOrderBy = pOrderBy;
            (*pWInfo).pResultSet = pResultSet;
            (*pWInfo).aiCurOnePass[1 as ::core::ffi::c_int as usize] = -1
                as ::core::ffi::c_int;
            (*pWInfo).aiCurOnePass[0 as ::core::ffi::c_int as usize] = (*pWInfo)
                .aiCurOnePass[1 as ::core::ffi::c_int as usize];
            (*pWInfo).nLevel = nTabList as u8_0;
            (*pWInfo).iContinue = sqlite3VdbeMakeLabel(pParse);
            (*pWInfo).iBreak = (*pWInfo).iContinue;
            (*pWInfo).wctrlFlags = wctrlFlags;
            (*pWInfo).iLimit = iAuxArg as LogEst;
            (*pWInfo).savedNQueryLoop = (*pParse).nQueryLoop as ::core::ffi::c_int;
            (*pWInfo).pSelect = pSelect;
            memset(
                &raw mut (*pWInfo).nOBSat as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (104 as size_t).wrapping_sub(65 as size_t),
            );
            memset(
                (&raw mut (*pWInfo).a as *mut WhereLevel)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut WhereLevel
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<WhereLoop>() as size_t)
                    .wrapping_add(
                        (nTabList as size_t)
                            .wrapping_mul(::core::mem::size_of::<WhereLevel>() as size_t),
                    ),
            );
            pMaskSet = &raw mut (*pWInfo).sMaskSet;
            (*pMaskSet).n = 0 as ::core::ffi::c_int;
            (*pMaskSet).ix[0 as ::core::ffi::c_int as usize] = -99 as ::core::ffi::c_int;
            sWLB.pWInfo = pWInfo;
            sWLB.pWC = &raw mut (*pWInfo).sWC;
            sWLB.pNew = (pWInfo as *mut ::core::ffi::c_char).offset(nByteWInfo as isize)
                as *mut WhereLoop;
            whereLoopInit(sWLB.pNew);
            sqlite3WhereClauseInit(&raw mut (*pWInfo).sWC, pWInfo);
            sqlite3WhereSplit(&raw mut (*pWInfo).sWC, pWhere, TK_AND as u8_0);
            if nTabList == 0 as ::core::ffi::c_int {
                if !pOrderBy.is_null() {
                    (*pWInfo).nOBSat = (*pOrderBy).nExpr as i8_0;
                }
                if wctrlFlags as ::core::ffi::c_int & WHERE_WANT_DISTINCT
                    != 0 as ::core::ffi::c_int
                    && (*db).dbOptFlags & 0x10 as u32_0 == 0 as u32_0
                {
                    (*pWInfo).eDistinct = WHERE_DISTINCT_UNIQUE as u8_0;
                }
                if !(*pWInfo).pSelect.is_null()
                    && (*(*pWInfo).pSelect).selFlags & SF_MultiValue as u32_0
                        == 0 as u32_0
                {
                    sqlite3VdbeExplain(
                        pParse,
                        0 as u8_0,
                        b"SCAN CONSTANT ROW\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            } else {
                ii = 0 as ::core::ffi::c_int;
                loop {
                    createMask(
                        pMaskSet,
                        (*(&raw mut (*pTabList).a as *mut SrcItem).offset(ii as isize))
                            .iCursor,
                    );
                    sqlite3WhereTabFuncArgs(
                        pParse,
                        (&raw mut (*pTabList).a as *mut SrcItem).offset(ii as isize)
                            as *mut SrcItem,
                        &raw mut (*pWInfo).sWC,
                    );
                    ii += 1;
                    if !(ii < (*pTabList).nSrc) {
                        break;
                    }
                }
            }
            sqlite3WhereExprAnalyze(pTabList, &raw mut (*pWInfo).sWC);
            if !pSelect.is_null() && !(*pSelect).pLimit.is_null() {
                sqlite3WhereAddLimit(&raw mut (*pWInfo).sWC, pSelect);
            }
            if !((*pParse).nErr != 0) {
                ii = 0 as ::core::ffi::c_int;
                while ii < (*sWLB.pWC).nBase {
                    let mut pT: *mut WhereTerm = (*sWLB.pWC).a.offset(ii as isize)
                        as *mut WhereTerm;
                    let mut pX: *mut Expr = ::core::ptr::null_mut::<Expr>();
                    if !((*pT).wtFlags as ::core::ffi::c_int & TERM_VIRTUAL != 0) {
                        pX = (*pT).pExpr;
                        if (*pT).prereqAll == 0 as ::core::ffi::c_ulonglong
                            && (nTabList == 0 as ::core::ffi::c_int
                                || exprIsDeterministic(pX) != 0)
                            && !((*pX).flags & 0x2 as ::core::ffi::c_int as u32_0
                                != 0 as u32_0
                                && (*(&raw mut (*pTabList).a as *mut SrcItem)
                                    .offset(0 as ::core::ffi::c_int as isize))
                                    .fg
                                    .jointype as ::core::ffi::c_int & JT_LTORJ
                                    != 0 as ::core::ffi::c_int)
                        {
                            sqlite3ExprIfFalse(
                                pParse,
                                pX,
                                (*pWInfo).iBreak,
                                SQLITE_JUMPIFNULL,
                            );
                            (*pT).wtFlags = ((*pT).wtFlags as ::core::ffi::c_int
                                | TERM_CODED) as u16_0;
                        }
                    }
                    ii += 1;
                }
                if wctrlFlags as ::core::ffi::c_int & WHERE_WANT_DISTINCT != 0 {
                    if (*db).dbOptFlags & 0x10 as u32_0 != 0 as u32_0 {
                        wctrlFlags = (wctrlFlags as ::core::ffi::c_int
                            & !WHERE_WANT_DISTINCT) as u16_0;
                        (*pWInfo).wctrlFlags = ((*pWInfo).wctrlFlags
                            as ::core::ffi::c_int & !WHERE_WANT_DISTINCT) as u16_0;
                    } else if isDistinctRedundant(
                        pParse,
                        pTabList,
                        &raw mut (*pWInfo).sWC,
                        pResultSet,
                    ) != 0
                    {
                        (*pWInfo).eDistinct = WHERE_DISTINCT_UNIQUE as u8_0;
                    } else if pOrderBy.is_null() {
                        (*pWInfo).wctrlFlags = ((*pWInfo).wctrlFlags
                            as ::core::ffi::c_int | WHERE_DISTINCTBY) as u16_0;
                        (*pWInfo).pOrderBy = pResultSet;
                    }
                }
                if nTabList != 1 as ::core::ffi::c_int
                    || whereShortCut(&raw mut sWLB) == 0 as ::core::ffi::c_int
                {
                    rc = whereLoopAddAll(&raw mut sWLB);
                    if rc != 0 {
                        c2rust_current_block = 682105650400477983;
                    } else {
                        wherePathSolver(pWInfo, 0 as LogEst);
                        if (*db).mallocFailed != 0 {
                            c2rust_current_block = 682105650400477983;
                        } else {
                            if !(*pWInfo).pOrderBy.is_null() {
                                whereInterstageHeuristic(pWInfo);
                                wherePathSolver(
                                    pWInfo,
                                    (if ((*pWInfo).nRowOut as ::core::ffi::c_int)
                                        < 0 as ::core::ffi::c_int
                                    {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        (*pWInfo).nRowOut as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int
                                    }) as LogEst,
                                );
                                if (*db).mallocFailed != 0 {
                                    c2rust_current_block = 682105650400477983;
                                } else {
                                    c2rust_current_block = 7318352876044315808;
                                }
                            } else {
                                c2rust_current_block = 7318352876044315808;
                            }
                            match c2rust_current_block {
                                682105650400477983 => {}
                                _ => {
                                    if (*pWInfo).wctrlFlags as ::core::ffi::c_int
                                        & WHERE_WANT_DISTINCT != 0 as ::core::ffi::c_int
                                    {
                                        (*pWInfo).nRowOut = ((*pWInfo).nRowOut as ::core::ffi::c_int
                                            - 30 as ::core::ffi::c_int) as LogEst;
                                    }
                                    c2rust_current_block = 3879520548144599102;
                                }
                            }
                        }
                    }
                } else {
                    c2rust_current_block = 3879520548144599102;
                }
                match c2rust_current_block {
                    682105650400477983 => {}
                    _ => {
                        if (*pWInfo).pOrderBy.is_null()
                            && (*db).flags as ::core::ffi::c_ulonglong
                                & SQLITE_ReverseOrder as ::core::ffi::c_ulonglong
                                != 0 as ::core::ffi::c_ulonglong
                        {
                            whereReverseScanOrder(pWInfo);
                        }
                        if !((*pParse).nErr != 0) {
                            notReady = !(0 as ::core::ffi::c_int as Bitmask);
                            if (*pWInfo).nLevel as ::core::ffi::c_int
                                >= 2 as ::core::ffi::c_int && !pResultSet.is_null()
                                && 0 as ::core::ffi::c_int
                                    == wctrlFlags as ::core::ffi::c_int
                                        & (WHERE_AGG_DISTINCT | WHERE_KEEP_ALL_JOINS)
                                && (*db).dbOptFlags & 0x100 as u32_0 == 0 as u32_0
                            {
                                notReady = whereOmitNoopJoin(pWInfo, notReady);
                                nTabList = (*pWInfo).nLevel as ::core::ffi::c_int;
                            }
                            if (*pWInfo).nLevel as ::core::ffi::c_int
                                >= 2 as ::core::ffi::c_int
                                && (*db).dbOptFlags & 0x80000 as u32_0 == 0 as u32_0
                            {
                                whereCheckIfBloomFilterIsUseful(pWInfo);
                            }
                            (*(*pWInfo).pParse).nQueryLoop = ((*(*pWInfo).pParse)
                                .nQueryLoop as ::core::ffi::c_int
                                + (*pWInfo).nRowOut as ::core::ffi::c_int) as LogEst;
                            if wctrlFlags as ::core::ffi::c_int & WHERE_ONEPASS_DESIRED
                                != 0 as ::core::ffi::c_int
                            {
                                let mut wsFlags: ::core::ffi::c_int = (*(*(&raw mut (*pWInfo)
                                    .a as *mut WhereLevel)
                                    .offset(0 as ::core::ffi::c_int as isize))
                                    .pWLoop)
                                    .wsFlags as ::core::ffi::c_int;
                                let mut bOnerow: ::core::ffi::c_int = (wsFlags
                                    & WHERE_ONEROW != 0 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int;
                                if bOnerow != 0
                                    || 0 as ::core::ffi::c_int
                                        != wctrlFlags as ::core::ffi::c_int & WHERE_ONEPASS_MULTIROW
                                        && !((*(*(&raw mut (*pTabList).a as *mut SrcItem)
                                            .offset(0 as ::core::ffi::c_int as isize))
                                            .pSTab)
                                            .eTabType as ::core::ffi::c_int == TABTYP_VTAB)
                                        && (0 as ::core::ffi::c_int == wsFlags & WHERE_MULTI_OR
                                            || wctrlFlags as ::core::ffi::c_int & WHERE_DUPLICATES_OK
                                                != 0) && (*db).dbOptFlags & 0x8000000 as u32_0 == 0 as u32_0
                                {
                                    (*pWInfo).eOnePass = (if bOnerow != 0 {
                                        ONEPASS_SINGLE
                                    } else {
                                        ONEPASS_MULTI
                                    }) as u8_0;
                                    if (*(*(&raw mut (*pTabList).a as *mut SrcItem)
                                        .offset(0 as ::core::ffi::c_int as isize))
                                        .pSTab)
                                        .tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0
                                        && wsFlags & WHERE_IDX_ONLY != 0
                                    {
                                        if wctrlFlags as ::core::ffi::c_int & WHERE_ONEPASS_MULTIROW
                                            != 0
                                        {
                                            bFordelete = OPFLAG_FORDELETE as u8_0;
                                        }
                                        (*(*(&raw mut (*pWInfo).a as *mut WhereLevel)
                                            .offset(0 as ::core::ffi::c_int as isize))
                                            .pWLoop)
                                            .wsFlags = (wsFlags & !WHERE_IDX_ONLY) as u32_0;
                                    }
                                }
                            }
                            ii = 0 as ::core::ffi::c_int;
                            pLevel = &raw mut (*pWInfo).a as *mut WhereLevel;
                            while ii < nTabList {
                                let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
                                let mut iDb: ::core::ffi::c_int = 0;
                                let mut pTabItem: *mut SrcItem = ::core::ptr::null_mut::<
                                    SrcItem,
                                >();
                                pTabItem = (&raw mut (*pTabList).a as *mut SrcItem)
                                    .offset((*pLevel).iFrom as isize) as *mut SrcItem;
                                pTab = (*pTabItem).pSTab;
                                iDb = sqlite3SchemaToIndex(db, (*pTab).pSchema);
                                pLoop = (*pLevel).pWLoop as *mut WhereLoop;
                                (*pLevel).addrBrk = sqlite3VdbeMakeLabel(pParse);
                                if ii == 0 as ::core::ffi::c_int
                                    || (*pTabItem.offset(0 as ::core::ffi::c_int as isize))
                                        .fg
                                        .jointype as ::core::ffi::c_int & JT_LEFT
                                        != 0 as ::core::ffi::c_int
                                {
                                    (*pLevel).addrHalt = (*pLevel).addrBrk;
                                } else if !(*(&raw mut (*pWInfo).a as *mut WhereLevel)
                                    .offset((ii - 1 as ::core::ffi::c_int) as isize))
                                    .pRJ
                                    .is_null()
                                {
                                    (*pLevel).addrHalt = (*(&raw mut (*pWInfo).a
                                        as *mut WhereLevel)
                                        .offset((ii - 1 as ::core::ffi::c_int) as isize))
                                        .addrBrk;
                                } else {
                                    (*pLevel).addrHalt = (*(&raw mut (*pWInfo).a
                                        as *mut WhereLevel)
                                        .offset((ii - 1 as ::core::ffi::c_int) as isize))
                                        .addrHalt;
                                }
                                if !((*pTab).tabFlags & TF_Ephemeral as u32_0 != 0 as u32_0
                                    || (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW)
                                {
                                    if (*pLoop).wsFlags & WHERE_VIRTUALTABLE as u32_0
                                        != 0 as u32_0
                                    {
                                        let mut pVTab: *const ::core::ffi::c_char = sqlite3GetVTable(
                                            db,
                                            pTab,
                                        ) as *const ::core::ffi::c_char;
                                        let mut iCur: ::core::ffi::c_int = (*pTabItem).iCursor;
                                        sqlite3VdbeAddOp4(
                                            v,
                                            OP_VOpen,
                                            iCur,
                                            0 as ::core::ffi::c_int,
                                            0 as ::core::ffi::c_int,
                                            pVTab,
                                            P4_VTAB,
                                        );
                                    } else if !((*pTab).eTabType as ::core::ffi::c_int
                                        == TABTYP_VTAB)
                                    {
                                        if (*pLoop).wsFlags & WHERE_IDX_ONLY as u32_0 == 0 as u32_0
                                            && wctrlFlags as ::core::ffi::c_int & WHERE_OR_SUBCLAUSE
                                                == 0 as ::core::ffi::c_int
                                            || (*pTabItem).fg.jointype as ::core::ffi::c_int
                                                & (JT_LTORJ | JT_RIGHT) != 0 as ::core::ffi::c_int
                                        {
                                            let mut op: ::core::ffi::c_int = OP_OpenRead;
                                            if (*pWInfo).eOnePass as ::core::ffi::c_int != ONEPASS_OFF {
                                                op = OP_OpenWrite;
                                                (*pWInfo).aiCurOnePass[0 as ::core::ffi::c_int as usize] = (*pTabItem)
                                                    .iCursor;
                                            }
                                            sqlite3OpenTable(
                                                pParse,
                                                (*pTabItem).iCursor,
                                                iDb,
                                                pTab,
                                                op,
                                            );
                                            if (*pWInfo).eOnePass as ::core::ffi::c_int == ONEPASS_OFF
                                                && ((*pTab).nCol as ::core::ffi::c_int) < BMS
                                                && (*pTab).tabFlags
                                                    & (TF_HasGenerated | TF_WithoutRowid) as u32_0 == 0 as u32_0
                                                && (*pLoop).wsFlags
                                                    & (WHERE_AUTO_INDEX | WHERE_BLOOMFILTER) as u32_0
                                                    == 0 as u32_0
                                            {
                                                let mut b: Bitmask = (*pTabItem).colUsed;
                                                let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                while b != 0 {
                                                    b = b >> 1 as ::core::ffi::c_int;
                                                    n += 1;
                                                }
                                                sqlite3VdbeChangeP4(
                                                    v,
                                                    -1 as ::core::ffi::c_int,
                                                    n as intptr_t as *mut ::core::ffi::c_void
                                                        as *const ::core::ffi::c_char,
                                                    P4_INT32,
                                                );
                                            }
                                            sqlite3VdbeChangeP5(v, bFordelete as u16_0);
                                            if ii >= 2 as ::core::ffi::c_int
                                                && (*pTabItem.offset(0 as ::core::ffi::c_int as isize))
                                                    .fg
                                                    .jointype as ::core::ffi::c_int & (JT_LTORJ | JT_LEFT)
                                                    == 0 as ::core::ffi::c_int
                                                && (*pLevel).addrHalt
                                                    == (*(&raw mut (*pWInfo).a as *mut WhereLevel)
                                                        .offset(0 as ::core::ffi::c_int as isize))
                                                        .addrHalt
                                            {
                                                sqlite3VdbeAddOp2(
                                                    v,
                                                    OP_IfEmpty,
                                                    (*pTabItem).iCursor,
                                                    (*pWInfo).iBreak,
                                                );
                                            }
                                        } else {
                                            sqlite3TableLock(
                                                pParse,
                                                iDb,
                                                (*pTab).tnum,
                                                0 as u8_0,
                                                (*pTab).zName,
                                            );
                                        }
                                    }
                                }
                                if (*pLoop).wsFlags & WHERE_INDEXED as u32_0 != 0 {
                                    let mut pIx: *mut Index = (*pLoop).u.btree.pIndex;
                                    let mut iIndexCur: ::core::ffi::c_int = 0;
                                    let mut op_0: ::core::ffi::c_int = OP_OpenRead;
                                    if !((*pTab).tabFlags & TF_WithoutRowid as u32_0
                                        == 0 as u32_0)
                                        && (*pIx).idxType() as ::core::ffi::c_int
                                            == SQLITE_IDXTYPE_PRIMARYKEY
                                        && wctrlFlags as ::core::ffi::c_int & WHERE_OR_SUBCLAUSE
                                            != 0 as ::core::ffi::c_int
                                    {
                                        iIndexCur = (*pLevel).iTabCur;
                                        op_0 = 0 as ::core::ffi::c_int;
                                    } else if (*pWInfo).eOnePass as ::core::ffi::c_int
                                        != ONEPASS_OFF
                                    {
                                        let mut pJ: *mut Index = (*(*pTabItem).pSTab).pIndex;
                                        iIndexCur = iAuxArg;
                                        while !pJ.is_null() && pJ != pIx {
                                            iIndexCur += 1;
                                            pJ = (*pJ).pNext;
                                        }
                                        op_0 = OP_OpenWrite;
                                        (*pWInfo).aiCurOnePass[1 as ::core::ffi::c_int as usize] = iIndexCur;
                                    } else if iAuxArg != 0
                                        && wctrlFlags as ::core::ffi::c_int & WHERE_OR_SUBCLAUSE
                                            != 0 as ::core::ffi::c_int
                                    {
                                        iIndexCur = iAuxArg;
                                        op_0 = OP_ReopenIdx;
                                    } else {
                                        let c2rust_fresh0 = (*pParse).nTab;
                                        (*pParse).nTab = (*pParse).nTab + 1;
                                        iIndexCur = c2rust_fresh0;
                                        if (*pIx).bHasExpr() as ::core::ffi::c_int != 0
                                            && (*db).dbOptFlags & 0x1000000 as u32_0 == 0 as u32_0
                                        {
                                            whereAddIndexedExpr(pParse, pIx, iIndexCur, pTabItem);
                                        }
                                        if !(*pIx).pPartIdxWhere.is_null()
                                            && (*pTabItem).fg.jointype as ::core::ffi::c_int & JT_RIGHT
                                                == 0 as ::core::ffi::c_int
                                        {
                                            wherePartIdxExpr(
                                                pParse,
                                                pIx,
                                                (*pIx).pPartIdxWhere,
                                                ::core::ptr::null_mut::<Bitmask>(),
                                                iIndexCur,
                                                pTabItem,
                                            );
                                        }
                                    }
                                    (*pLevel).iIdxCur = iIndexCur;
                                    if op_0 != 0 {
                                        sqlite3VdbeAddOp3(
                                            v,
                                            op_0,
                                            iIndexCur,
                                            (*pIx).tnum as ::core::ffi::c_int,
                                            iDb,
                                        );
                                        sqlite3VdbeSetP4KeyInfo(pParse, pIx);
                                        if (*pLoop).wsFlags & WHERE_CONSTRAINT as u32_0
                                            != 0 as u32_0
                                            && (*pLoop).wsFlags
                                                & (WHERE_COLUMN_RANGE | WHERE_SKIPSCAN) as u32_0
                                                == 0 as u32_0
                                            && (*pLoop).wsFlags & WHERE_BIGNULL_SORT as u32_0
                                                == 0 as u32_0
                                            && (*pLoop).wsFlags & WHERE_IN_SEEKSCAN as u32_0
                                                == 0 as u32_0
                                            && (*pWInfo).wctrlFlags as ::core::ffi::c_int
                                                & WHERE_ORDERBY_MIN == 0 as ::core::ffi::c_int
                                            && (*pWInfo).eDistinct as ::core::ffi::c_int
                                                != WHERE_DISTINCT_ORDERED
                                        {
                                            sqlite3VdbeChangeP5(v, OPFLAG_SEEKEQ as u16_0);
                                        }
                                    }
                                }
                                if iDb >= 0 as ::core::ffi::c_int {
                                    sqlite3CodeVerifySchema(pParse, iDb);
                                }
                                if (*pTabItem).fg.jointype as ::core::ffi::c_int & JT_RIGHT
                                    != 0 as ::core::ffi::c_int
                                    && {
                                        (*pLevel).pRJ = sqlite3WhereMalloc(
                                            pWInfo,
                                            ::core::mem::size_of::<WhereRightJoin>() as u64_0,
                                        ) as *mut WhereRightJoin;
                                        !(*pLevel).pRJ.is_null()
                                    }
                                {
                                    let mut pRJ: *mut WhereRightJoin = (*pLevel).pRJ;
                                    let c2rust_fresh1 = (*pParse).nTab;
                                    (*pParse).nTab = (*pParse).nTab + 1;
                                    (*pRJ).iMatch = c2rust_fresh1;
                                    (*pParse).nMem += 1;
                                    (*pRJ).regBloom = (*pParse).nMem;
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_Blob,
                                        65536 as ::core::ffi::c_int,
                                        (*pRJ).regBloom,
                                    );
                                    (*pParse).nMem += 1;
                                    (*pRJ).regReturn = (*pParse).nMem;
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_Null,
                                        0 as ::core::ffi::c_int,
                                        (*pRJ).regReturn,
                                    );
                                    if (*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0
                                    {
                                        let mut pInfo: *mut KeyInfo = ::core::ptr::null_mut::<
                                            KeyInfo,
                                        >();
                                        sqlite3VdbeAddOp2(
                                            v,
                                            OP_OpenEphemeral,
                                            (*pRJ).iMatch,
                                            1 as ::core::ffi::c_int,
                                        );
                                        pInfo = sqlite3KeyInfoAlloc(
                                            (*pParse).db,
                                            1 as ::core::ffi::c_int,
                                            0 as ::core::ffi::c_int,
                                        );
                                        if !pInfo.is_null() {
                                            let ref mut c2rust_fresh2 = *(&raw mut (*pInfo).aColl
                                                as *mut *mut CollSeq)
                                                .offset(0 as ::core::ffi::c_int as isize);
                                            *c2rust_fresh2 = ::core::ptr::null_mut::<CollSeq>();
                                            *(*pInfo)
                                                .aSortFlags
                                                .offset(0 as ::core::ffi::c_int as isize) = 0 as u8_0;
                                            sqlite3VdbeAppendP4(
                                                v,
                                                pInfo as *mut ::core::ffi::c_void,
                                                P4_KEYINFO,
                                            );
                                        }
                                    } else {
                                        let mut pPk: *mut Index = sqlite3PrimaryKeyIndex(pTab);
                                        sqlite3VdbeAddOp2(
                                            v,
                                            OP_OpenEphemeral,
                                            (*pRJ).iMatch,
                                            (*pPk).nKeyCol as ::core::ffi::c_int,
                                        );
                                        sqlite3VdbeSetP4KeyInfo(pParse, pPk);
                                    }
                                    (*pLoop).wsFlags = ((*pLoop).wsFlags as ::core::ffi::c_uint
                                        & !WHERE_IDX_ONLY as ::core::ffi::c_uint) as u32_0;
                                    (*pWInfo).nOBSat = 0 as i8_0;
                                    (*pWInfo).eDistinct = WHERE_DISTINCT_UNORDERED as u8_0;
                                }
                                ii += 1;
                                pLevel = pLevel.offset(1);
                            }
                            (*pWInfo).iTop = sqlite3VdbeCurrentAddr(v);
                            if !((*db).mallocFailed != 0) {
                                ii = 0 as ::core::ffi::c_int;
                                loop {
                                    if !(ii < nTabList) {
                                        c2rust_current_block = 17342404680090366851;
                                        break;
                                    }
                                    let mut addrExplain: ::core::ffi::c_int = 0;
                                    let mut wsFlags_0: ::core::ffi::c_int = 0;
                                    let mut pSrc: *mut SrcItem = ::core::ptr::null_mut::<
                                        SrcItem,
                                    >();
                                    if (*pParse).nErr != 0 {
                                        c2rust_current_block = 682105650400477983;
                                        break;
                                    }
                                    pLevel = (&raw mut (*pWInfo).a as *mut WhereLevel)
                                        .offset(ii as isize) as *mut WhereLevel;
                                    wsFlags_0 = (*(*pLevel).pWLoop).wsFlags
                                        as ::core::ffi::c_int;
                                    pSrc = (&raw mut (*pTabList).a as *mut SrcItem)
                                        .offset((*pLevel).iFrom as isize) as *mut SrcItem;
                                    if (*pSrc).fg.isMaterialized() != 0 {
                                        let mut pSubq: *mut Subquery = ::core::ptr::null_mut::<
                                            Subquery,
                                        >();
                                        let mut iOnce: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                        pSubq = (*pSrc).u4.pSubq;
                                        if (*pSrc).fg.isCorrelated() as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                        {
                                            iOnce = sqlite3VdbeAddOp0(v, OP_Once);
                                        } else {
                                            iOnce = 0 as ::core::ffi::c_int;
                                        }
                                        sqlite3VdbeAddOp2(
                                            v,
                                            OP_Gosub,
                                            (*pSubq).regReturn,
                                            (*pSubq).addrFillSub,
                                        );
                                        if iOnce != 0 {
                                            sqlite3VdbeJumpHere(v, iOnce);
                                        }
                                    }
                                    if wsFlags_0 & (WHERE_AUTO_INDEX | WHERE_BLOOMFILTER)
                                        != 0 as ::core::ffi::c_int
                                    {
                                        if wsFlags_0 & WHERE_AUTO_INDEX != 0 as ::core::ffi::c_int {
                                            constructAutomaticIndex(
                                                pParse,
                                                &raw mut (*pWInfo).sWC,
                                                notReady,
                                                pLevel,
                                            );
                                        } else {
                                            sqlite3ConstructBloomFilter(pWInfo, ii, pLevel, notReady);
                                        }
                                        if (*db).mallocFailed != 0 {
                                            c2rust_current_block = 682105650400477983;
                                            break;
                                        }
                                    }
                                    addrExplain = sqlite3WhereExplainOneScan(
                                        pParse,
                                        pTabList,
                                        pLevel,
                                        wctrlFlags,
                                    );
                                    (*pLevel).addrBody = sqlite3VdbeCurrentAddr(v);
                                    notReady = sqlite3WhereCodeOneLoopStart(
                                        pParse,
                                        v,
                                        pWInfo,
                                        ii,
                                        pLevel,
                                        notReady,
                                    );
                                    (*pWInfo).iContinue = (*pLevel).addrCont;
                                    wsFlags_0 & WHERE_MULTI_OR == 0 as ::core::ffi::c_int
                                        && wctrlFlags as ::core::ffi::c_int & WHERE_OR_SUBCLAUSE
                                            == 0 as ::core::ffi::c_int;
                                    ii += 1;
                                }
                                match c2rust_current_block {
                                    682105650400477983 => {}
                                    _ => {
                                        (*pWInfo).iEndWhere = sqlite3VdbeCurrentAddr(v);
                                        return pWInfo;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if !pWInfo.is_null() {
            (*pParse).nQueryLoop = (*pWInfo).savedNQueryLoop as LogEst;
            whereInfoFree(db, pWInfo);
        }
        return ::core::ptr::null_mut::<WhereInfo>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WhereEnd(mut pWInfo: *mut WhereInfo) {
    unsafe {
        let mut pParse: *mut Parse = (*pWInfo).pParse;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut i: ::core::ffi::c_int = 0;
        let mut pLevel: *mut WhereLevel = ::core::ptr::null_mut::<WhereLevel>();
        let mut pLoop: *mut WhereLoop = ::core::ptr::null_mut::<WhereLoop>();
        let mut pTabList: *mut SrcList = (*pWInfo).pTabList;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut iEnd: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(v);
        let mut nRJ: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addrSeek: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = (*pWInfo).nLevel as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            let mut addr: ::core::ffi::c_int = 0;
            pLevel = (&raw mut (*pWInfo).a as *mut WhereLevel).offset(i as isize)
                as *mut WhereLevel;
            if !(*pLevel).pRJ.is_null() {
                let mut pRJ: *mut WhereRightJoin = (*pLevel).pRJ;
                sqlite3VdbeResolveLabel(v, (*pLevel).addrCont);
                (*pLevel).addrCont = sqlite3VdbeMakeLabel(pParse);
                (*pRJ).endSubrtn = sqlite3VdbeCurrentAddr(v);
                sqlite3VdbeAddOp3(
                    v,
                    OP_Return,
                    (*pRJ).regReturn,
                    (*pRJ).addrSubrtn,
                    1 as ::core::ffi::c_int,
                );
                nRJ += 1;
            }
            pLoop = (*pLevel).pWLoop as *mut WhereLoop;
            if (*pLevel).op as ::core::ffi::c_int != OP_Noop {
                let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
                let mut n: ::core::ffi::c_int = 0;
                if (*pWInfo).eDistinct as ::core::ffi::c_int == WHERE_DISTINCT_ORDERED
                    && i
                        == (*pWInfo).nLevel as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                    && (*pLoop).wsFlags & WHERE_INDEXED as u32_0 != 0 as u32_0
                    && {
                        pIdx = (*pLoop).u.btree.pIndex;
                        (*pIdx).hasStat1() as ::core::ffi::c_int != 0
                    }
                    && {
                        n = (*pLoop).u.btree.nDistinctCol as ::core::ffi::c_int;
                        n > 0 as ::core::ffi::c_int
                    }
                    && *(*pIdx).aiRowLogEst.offset(n as isize) as ::core::ffi::c_int
                        >= 36 as ::core::ffi::c_int
                {
                    let mut r1: ::core::ffi::c_int = (*pParse).nMem
                        + 1 as ::core::ffi::c_int;
                    let mut j: ::core::ffi::c_int = 0;
                    let mut op: ::core::ffi::c_int = 0;
                    j = 0 as ::core::ffi::c_int;
                    while j < n {
                        sqlite3VdbeAddOp3(v, OP_Column, (*pLevel).iIdxCur, j, r1 + j);
                        j += 1;
                    }
                    (*pParse).nMem += n + 1 as ::core::ffi::c_int;
                    op = if (*pLevel).op as ::core::ffi::c_int == OP_Prev {
                        OP_SeekLT
                    } else {
                        OP_SeekGT
                    };
                    addrSeek = sqlite3VdbeAddOp4Int(
                        v,
                        op,
                        (*pLevel).iIdxCur,
                        0 as ::core::ffi::c_int,
                        r1,
                        n,
                    );
                    sqlite3VdbeAddOp2(v, OP_Goto, 1 as ::core::ffi::c_int, (*pLevel).p2);
                }
            }
            if (*(&raw mut (*pTabList).a as *mut SrcItem)
                .offset((*pLevel).iFrom as isize))
                .fg
                .fromExists() as ::core::ffi::c_int != 0
                && i == (*pWInfo).nLevel as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            {
                let mut nOuter: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while nOuter < i {
                    if (*(&raw mut (*pTabList).a as *mut SrcItem)
                        .offset(
                            (*pLevel
                                .offset((-nOuter - 1 as ::core::ffi::c_int) as isize))
                                .iFrom as isize,
                        ))
                        .fg
                        .fromExists() == 0
                    {
                        break;
                    }
                    nOuter += 1;
                }
                sqlite3VdbeAddOp2(
                    v,
                    OP_Goto,
                    0 as ::core::ffi::c_int,
                    (*pLevel.offset(-nOuter as isize)).addrBrk,
                );
            }
            sqlite3VdbeResolveLabel(v, (*pLevel).addrCont);
            if (*pLevel).op as ::core::ffi::c_int != OP_Noop {
                sqlite3VdbeAddOp3(
                    v,
                    (*pLevel).op as ::core::ffi::c_int,
                    (*pLevel).p1,
                    (*pLevel).p2,
                    (*pLevel).p3 as ::core::ffi::c_int,
                );
                sqlite3VdbeChangeP5(v, (*pLevel).p5 as u16_0);
                if (*pLevel).regBignull != 0 {
                    sqlite3VdbeResolveLabel(v, (*pLevel).addrBignull);
                    sqlite3VdbeAddOp2(
                        v,
                        OP_DecrJumpZero,
                        (*pLevel).regBignull,
                        (*pLevel).p2 - 1 as ::core::ffi::c_int,
                    );
                }
                if addrSeek != 0 {
                    sqlite3VdbeJumpHere(v, addrSeek);
                    addrSeek = 0 as ::core::ffi::c_int;
                }
            }
            if (*pLoop).wsFlags & WHERE_IN_ABLE as u32_0 != 0 as u32_0
                && (*pLevel).u.in_0.nIn > 0 as ::core::ffi::c_int
            {
                let mut pIn: *mut InLoop = ::core::ptr::null_mut::<InLoop>();
                let mut j_0: ::core::ffi::c_int = 0;
                sqlite3VdbeResolveLabel(v, (*pLevel).addrNxt);
                j_0 = (*pLevel).u.in_0.nIn;
                pIn = (*pLevel)
                    .u
                    .in_0
                    .aInLoop
                    .offset((j_0 - 1 as ::core::ffi::c_int) as isize) as *mut InLoop
                    as *mut InLoop;
                while j_0 > 0 as ::core::ffi::c_int {
                    sqlite3VdbeJumpHere(v, (*pIn).addrInTop + 1 as ::core::ffi::c_int);
                    if (*pIn).eEndLoopOp as ::core::ffi::c_int != OP_Noop {
                        if (*pIn).nPrefix != 0 {
                            let mut bEarlyOut: ::core::ffi::c_int = ((*pLoop).wsFlags
                                & WHERE_VIRTUALTABLE as u32_0 == 0 as u32_0
                                && (*pLoop).wsFlags & WHERE_IN_EARLYOUT as u32_0
                                    != 0 as u32_0) as ::core::ffi::c_int;
                            if (*pLevel).iLeftJoin != 0 {
                                sqlite3VdbeAddOp2(
                                    v,
                                    OP_IfNotOpen,
                                    (*pIn).iCur,
                                    sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int
                                        + bEarlyOut,
                                );
                            }
                            if bEarlyOut != 0 {
                                sqlite3VdbeAddOp4Int(
                                    v,
                                    OP_IfNoHope,
                                    (*pLevel).iIdxCur,
                                    sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
                                    (*pIn).iBase,
                                    (*pIn).nPrefix,
                                );
                                sqlite3VdbeJumpHere(
                                    v,
                                    (*pIn).addrInTop + 1 as ::core::ffi::c_int,
                                );
                            }
                        }
                        sqlite3VdbeAddOp2(
                            v,
                            (*pIn).eEndLoopOp as ::core::ffi::c_int,
                            (*pIn).iCur,
                            (*pIn).addrInTop,
                        );
                    }
                    sqlite3VdbeJumpHere(v, (*pIn).addrInTop - 1 as ::core::ffi::c_int);
                    j_0 -= 1;
                    pIn = pIn.offset(-1);
                }
            }
            sqlite3VdbeResolveLabel(v, (*pLevel).addrBrk);
            if !(*pLevel).pRJ.is_null() {
                sqlite3VdbeAddOp3(
                    v,
                    OP_Return,
                    (*(*pLevel).pRJ).regReturn,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            }
            if (*pLevel).addrSkip != 0 {
                sqlite3VdbeGoto(v, (*pLevel).addrSkip);
                sqlite3VdbeJumpHere(v, (*pLevel).addrSkip);
                sqlite3VdbeJumpHere(v, (*pLevel).addrSkip - 2 as ::core::ffi::c_int);
            }
            if (*pLevel).addrLikeRep != 0 {
                sqlite3VdbeAddOp2(
                    v,
                    OP_DecrJumpZero,
                    ((*pLevel).iLikeRepCntr >> 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_int,
                    (*pLevel).addrLikeRep,
                );
            }
            if (*pLevel).iLeftJoin != 0 {
                let mut ws: ::core::ffi::c_int = (*pLoop).wsFlags as ::core::ffi::c_int;
                addr = sqlite3VdbeAddOp1(v, OP_IfPos, (*pLevel).iLeftJoin);
                if ws & WHERE_IDX_ONLY == 0 as ::core::ffi::c_int {
                    let mut pSrc: *mut SrcItem = (&raw mut (*pTabList).a as *mut SrcItem)
                        .offset((*pLevel).iFrom as isize) as *mut SrcItem;
                    if (*pSrc).fg.viaCoroutine() != 0 {
                        let mut m: ::core::ffi::c_int = 0;
                        let mut n_0: ::core::ffi::c_int = 0;
                        n_0 = (*(*pSrc).u4.pSubq).regResult;
                        m = (*(*pSrc).pSTab).nCol as ::core::ffi::c_int;
                        sqlite3VdbeAddOp3(
                            v,
                            OP_Null,
                            0 as ::core::ffi::c_int,
                            n_0,
                            n_0 + m - 1 as ::core::ffi::c_int,
                        );
                    }
                    sqlite3VdbeAddOp1(v, OP_NullRow, (*pLevel).iTabCur);
                }
                if ws & WHERE_INDEXED != 0
                    || ws & WHERE_MULTI_OR != 0 && !(*pLevel).u.pCoveringIdx.is_null()
                {
                    if ws & WHERE_MULTI_OR != 0 {
                        let mut pIx: *mut Index = (*pLevel).u.pCoveringIdx;
                        let mut iDb: ::core::ffi::c_int = sqlite3SchemaToIndex(
                            db,
                            (*pIx).pSchema,
                        );
                        sqlite3VdbeAddOp3(
                            v,
                            OP_ReopenIdx,
                            (*pLevel).iIdxCur,
                            (*pIx).tnum as ::core::ffi::c_int,
                            iDb,
                        );
                        sqlite3VdbeSetP4KeyInfo(pParse, pIx);
                    }
                    sqlite3VdbeAddOp1(v, OP_NullRow, (*pLevel).iIdxCur);
                }
                if (*pLevel).op as ::core::ffi::c_int == OP_Return {
                    sqlite3VdbeAddOp2(v, OP_Gosub, (*pLevel).p1, (*pLevel).addrFirst);
                } else {
                    sqlite3VdbeGoto(v, (*pLevel).addrFirst);
                }
                sqlite3VdbeJumpHere(v, addr);
            }
            i -= 1;
        }
        i = 0 as ::core::ffi::c_int;
        pLevel = &raw mut (*pWInfo).a as *mut WhereLevel;
        while i < (*pWInfo).nLevel as ::core::ffi::c_int {
            let mut k: ::core::ffi::c_int = 0;
            let mut last: ::core::ffi::c_int = 0;
            let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
            let mut pLastOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
            let mut pIdx_0: *mut Index = ::core::ptr::null_mut::<Index>();
            let mut pTabItem: *mut SrcItem = (&raw mut (*pTabList).a as *mut SrcItem)
                .offset((*pLevel).iFrom as isize) as *mut SrcItem;
            let mut pTab: *mut Table = (*pTabItem).pSTab;
            pLoop = (*pLevel).pWLoop as *mut WhereLoop;
            if !(*pLevel).pRJ.is_null() {
                sqlite3WhereRightJoinLoop(pWInfo, i, pLevel);
            } else if (*pTabItem).fg.viaCoroutine() != 0 {
                translateColumnToCopy(
                    pParse,
                    (*pLevel).addrBody,
                    (*pLevel).iTabCur,
                    (*(*pTabItem).u4.pSubq).regResult,
                    0 as ::core::ffi::c_int,
                );
            } else {
                if (*pLoop).wsFlags & (WHERE_INDEXED | WHERE_IDX_ONLY) as u32_0 != 0 {
                    pIdx_0 = (*pLoop).u.btree.pIndex;
                } else if (*pLoop).wsFlags & WHERE_MULTI_OR as u32_0 != 0 {
                    pIdx_0 = (*pLevel).u.pCoveringIdx;
                }
                if !pIdx_0.is_null() && (*db).mallocFailed == 0 {
                    if (*pWInfo).eOnePass as ::core::ffi::c_int == ONEPASS_OFF
                        || !((*(*pIdx_0).pTable).tabFlags & TF_WithoutRowid as u32_0
                            == 0 as u32_0)
                    {
                        last = iEnd;
                    } else {
                        last = (*pWInfo).iEndWhere;
                    }
                    if (*pIdx_0).bHasExpr() != 0 {
                        let mut p: *mut IndexedExpr = (*pParse).pIdxEpr;
                        while !p.is_null() {
                            if (*p).iIdxCur == (*pLevel).iIdxCur {
                                (*p).iDataCur = -1 as ::core::ffi::c_int;
                                (*p).iIdxCur = -1 as ::core::ffi::c_int;
                            }
                            p = (*p).pIENext;
                        }
                    }
                    k = (*pLevel).addrBody + 1 as ::core::ffi::c_int;
                    pOp = sqlite3VdbeGetOp(v, k);
                    pLastOp = pOp.offset((last - k) as isize);
                    loop {
                        if !((*pOp).p1 != (*pLevel).iTabCur) {
                            if (*pOp).opcode as ::core::ffi::c_int == OP_Column {
                                let mut x: ::core::ffi::c_int = (*pOp).p2;
                                if !((*pTab).tabFlags & TF_WithoutRowid as u32_0
                                    == 0 as u32_0)
                                {
                                    let mut pPk: *mut Index = sqlite3PrimaryKeyIndex(pTab);
                                    x = *(*pPk).aiColumn.offset(x as isize)
                                        as ::core::ffi::c_int;
                                } else {
                                    x = sqlite3StorageColumnToTable(pTab, x as i16_0)
                                        as ::core::ffi::c_int;
                                }
                                x = sqlite3TableColumnToIndex(pIdx_0, x);
                                if x >= 0 as ::core::ffi::c_int {
                                    (*pOp).p2 = x;
                                    (*pOp).p1 = (*pLevel).iIdxCur;
                                } else if (*pLoop).wsFlags
                                    & (WHERE_IDX_ONLY | WHERE_EXPRIDX) as u32_0 != 0
                                {
                                    if (*pLoop).wsFlags & WHERE_IDX_ONLY as u32_0 != 0 {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            b"internal query planner error\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        (*pParse).rc = SQLITE_INTERNAL;
                                    } else {
                                        (*pLoop).wsFlags = ((*pLoop).wsFlags as ::core::ffi::c_uint
                                            & !WHERE_EXPRIDX as ::core::ffi::c_uint) as u32_0;
                                        sqlite3WhereAddExplainText(
                                            pParse,
                                            (*pLevel).addrBody - 1 as ::core::ffi::c_int,
                                            pTabList,
                                            pLevel,
                                            (*pWInfo).wctrlFlags,
                                        );
                                    }
                                }
                            } else if (*pOp).opcode as ::core::ffi::c_int == OP_Rowid {
                                (*pOp).p1 = (*pLevel).iIdxCur;
                                (*pOp).opcode = OP_IdxRowid as u8_0;
                            } else if (*pOp).opcode as ::core::ffi::c_int == OP_IfNullRow
                            {
                                (*pOp).p1 = (*pLevel).iIdxCur;
                            }
                        }
                        pOp = pOp.offset(1);
                        if !(pOp < pLastOp) {
                            break;
                        }
                    }
                }
            }
            i += 1;
            pLevel = pLevel.offset(1);
        }
        sqlite3VdbeResolveLabel(v, (*pWInfo).iBreak);
        (*pParse).nQueryLoop = (*pWInfo).savedNQueryLoop as LogEst;
        whereInfoFree(db, pWInfo);
        (*pParse).withinRJSubrtn = ((*pParse).withinRJSubrtn as ::core::ffi::c_int - nRJ)
            as u8_0;
    }
}
