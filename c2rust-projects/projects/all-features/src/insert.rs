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
    pub type WindowRewrite;
    pub type IdxCover;
    pub type RefSrcList;
    pub type CCurHint;
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
    fn sqlite3VdbeLoadString(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
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
    fn sqlite3VdbeEndCoroutine(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeAddOpList(
        _: *mut Vdbe,
        nOp: ::core::ffi::c_int,
        aOp: *const VdbeOpList,
        iLineno: ::core::ffi::c_int,
    ) -> *mut VdbeOp;
    fn sqlite3VdbeExplain(
        _: *mut Parse,
        _: u8_0,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    fn sqlite3VdbeGetLastOp(_: *mut Vdbe) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3VdbeResolveLabel(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeCurrentAddr(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeCountChanges(_: *mut Vdbe);
    fn sqlite3VdbeDb(_: *mut Vdbe) -> *mut sqlite3;
    fn sqlite3VdbeHasSubProgram(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3WalkExpr(_: *mut Walker, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRaw(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbNNFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3GetTempReg(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempReg(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3GetTempRange(_: *mut Parse, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempRange(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprDelete(_: *mut sqlite3, _: *mut Expr);
    fn sqlite3ExprListDelete(_: *mut sqlite3, _: *mut ExprList);
    fn sqlite3ColumnExpr(_: *mut Table, _: *mut Column) -> *mut Expr;
    fn sqlite3ColumnColl(_: *mut Column) -> *const ::core::ffi::c_char;
    fn sqlite3PrimaryKeyIndex(_: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(
        _: *mut Index,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3TableColumnToStorage(_: *mut Table, _: i16_0) -> i16_0;
    fn sqlite3FaultSim(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3ViewGetColumnNames(_: *mut Parse, _: *mut Table) -> ::core::ffi::c_int;
    fn sqlite3SrcItemAttachSubquery(
        _: *mut Parse,
        _: *mut SrcItem,
        _: *mut Select,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3IdListDelete(_: *mut sqlite3, _: *mut IdList);
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
    fn sqlite3SrcListLookup(_: *mut Parse, _: *mut SrcList) -> *mut Table;
    fn sqlite3IsReadOnly(
        _: *mut Parse,
        _: *mut Table,
        _: *mut Trigger,
    ) -> ::core::ffi::c_int;
    fn sqlite3CodeChangeCount(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3ExprCode(_: *mut Parse, _: *mut Expr, _: ::core::ffi::c_int);
    fn sqlite3ExprCodeGeneratedColumn(
        _: *mut Parse,
        _: *mut Table,
        _: *mut Column,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprCodeCopy(_: *mut Parse, _: *mut Expr, _: ::core::ffi::c_int);
    fn sqlite3ExprCodeFactorable(_: *mut Parse, _: *mut Expr, _: ::core::ffi::c_int);
    fn sqlite3ExprCodeTarget(
        _: *mut Parse,
        _: *mut Expr,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprCodeExprList(
        _: *mut Parse,
        _: *mut ExprList,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprIfTrue(
        _: *mut Parse,
        _: *mut Expr,
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
    fn sqlite3GetVdbe(_: *mut Parse) -> *mut Vdbe;
    fn sqlite3CodeVerifySchema(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3ExprIsConstant(_: *mut Parse, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3IsRowid(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3GenerateRowDelete(
        _: *mut Parse,
        _: *mut Table,
        _: *mut Trigger,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: i16_0,
        _: u8_0,
        _: u8_0,
        _: u8_0,
        _: ::core::ffi::c_int,
    );
    fn sqlite3GenerateRowIndexDelete(
        _: *mut Parse,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3BeginWriteOperation(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3MultiWrite(_: *mut Parse);
    fn sqlite3MayAbort(_: *mut Parse);
    fn sqlite3HaltConstraint(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: i8_0,
        _: u8_0,
    );
    fn sqlite3UniqueConstraint(_: *mut Parse, _: ::core::ffi::c_int, _: *mut Index);
    fn sqlite3RowidConstraint(_: *mut Parse, _: ::core::ffi::c_int, _: *mut Table);
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
    fn sqlite3ColumnIndex(
        pTab: *mut Table,
        zCol: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3AuthCheck(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ExprAffinity(pExpr: *const Expr) -> ::core::ffi::c_char;
    fn sqlite3ReadSchema(pParse: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3LocateCollSeq(
        pParse: *mut Parse,
        zName: *const ::core::ffi::c_char,
    ) -> *mut CollSeq;
    static sqlite3OpcodeProperty: [::core::ffi::c_uchar; 0];
    static sqlite3StrBINARY: [::core::ffi::c_char; 0];
    fn sqlite3SelectWrongNumTermsError(pParse: *mut Parse, p: *mut Select);
    fn sqlite3ResolveExprListNames(
        _: *mut NameContext,
        _: *mut ExprList,
    ) -> ::core::ffi::c_int;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
    fn sqlite3HasExplicitNulls(_: *mut Parse, _: *mut ExprList) -> ::core::ffi::c_int;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3SelectDestInit(
        _: *mut SelectDest,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3TableLock(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: Pgno,
        _: u8_0,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3GetVTable(_: *mut sqlite3, _: *mut Table) -> *mut VTable;
    fn sqlite3VtabMakeWritable(_: *mut Parse, _: *mut Table);
    fn sqlite3ParserAddCleanup(
        _: *mut Parse,
        _: Option<unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> ()>,
        _: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3UpsertDelete(_: *mut sqlite3, _: *mut Upsert);
    fn sqlite3UpsertAnalyzeTarget(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Upsert,
        _: *mut Upsert,
    ) -> ::core::ffi::c_int;
    fn sqlite3UpsertDoUpdate(
        _: *mut Parse,
        _: *mut Upsert,
        _: *mut Table,
        _: *mut Index,
        _: ::core::ffi::c_int,
    );
    fn sqlite3UpsertOfIndex(_: *mut Upsert, _: *mut Index) -> *mut Upsert;
    fn sqlite3UpsertNextIsIPK(_: *mut Upsert) -> ::core::ffi::c_int;
    fn sqlite3FkCheck(
        _: *mut Parse,
        _: *mut Table,
        _: ::core::ffi::c_int,
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
pub type __int8_t = i8;
pub type int8_t = __int8_t;
pub type intptr_t = isize;
pub type size_t = usize;
pub type i8_0 = int8_t;
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
pub struct SelectDest {
    pub eDest: u8_0,
    pub iSDParm: ::core::ffi::c_int,
    pub iSDParm2: ::core::ffi::c_int,
    pub iSdst: ::core::ffi::c_int,
    pub nSdst: ::core::ffi::c_int,
    pub zAffSdst: *mut ::core::ffi::c_char,
    pub pOrderBy: *mut ExprList

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeOpList {
    pub opcode: u8_0,
    pub p1: ::core::ffi::c_schar,
    pub p2: ::core::ffi::c_schar,
    pub p3: ::core::ffi::c_schar

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexIterator {
    pub eType: ::core::ffi::c_int,
    pub i: ::core::ffi::c_int,
    pub u: C2Rust_Unnamed_24

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_24 {
    pub lx: C2Rust_Unnamed_26,
    pub ax: C2Rust_Unnamed_25

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub nIdx: ::core::ffi::c_int,
    pub aIdx: *mut IndexListTerm

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexListTerm {
    pub p: *mut Index,
    pub ix: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub pIdx: *mut Index

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT_SEQUENCE: ::core::ffi::c_int = SQLITE_CORRUPT
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_CHECK: ::core::ffi::c_int = SQLITE_CONSTRAINT
    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_NOTNULL: ::core::ffi::c_int = SQLITE_CONSTRAINT
    | (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_INSERT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const OP_VUpdate: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const OP_Goto: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OP_InitCoroutine: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const OP_Yield: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const OP_MustBeInt: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const OP_IfNot: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const OP_NoConflict: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const OP_NotExists: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const OP_IsNull: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const OP_NotNull: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const OP_Ne: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const OP_Eq: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const OP_Le: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const OP_HaltIfNull: ::core::ffi::c_int = 70 as ::core::ffi::c_int;
pub const OP_Halt: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const OP_SoftNull: ::core::ffi::c_int = 77 as ::core::ffi::c_int;
pub const OP_Copy: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
pub const OP_SCopy: ::core::ffi::c_int = 82 as ::core::ffi::c_int;
pub const OP_IntCopy: ::core::ffi::c_int = 83 as ::core::ffi::c_int;
pub const OP_AddImm: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_TypeCheck: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const OP_Affinity: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
pub const OP_MakeRecord: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const OP_OpenRead: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const OP_OpenWrite: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const OP_OpenEphemeral: ::core::ffi::c_int = 119 as ::core::ffi::c_int;
pub const OP_Close: ::core::ffi::c_int = 123 as ::core::ffi::c_int;
pub const OP_NewRowid: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const OP_Insert: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const OP_RowCell: ::core::ffi::c_int = 130 as ::core::ffi::c_int;
pub const OP_Delete: ::core::ffi::c_int = 131 as ::core::ffi::c_int;
pub const OP_RowData: ::core::ffi::c_int = 135 as ::core::ffi::c_int;
pub const OP_Rowid: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const OP_SeekEnd: ::core::ffi::c_int = 138 as ::core::ffi::c_int;
pub const OP_IdxInsert: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const OP_IdxRowid: ::core::ffi::c_int = 143 as ::core::ffi::c_int;
pub const OP_MemMax: ::core::ffi::c_int = 160 as ::core::ffi::c_int;
pub const OP_CursorLock: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
pub const OP_CursorUnlock: ::core::ffi::c_int = 169 as ::core::ffi::c_int;
pub const OP_VOpen: ::core::ffi::c_int = 174 as ::core::ffi::c_int;
pub const OPFLG_JUMP: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const P4_TRANSIENT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const P4_COLLSEQ: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const P4_INT32: ::core::ffi::c_int = -3 as ::core::ffi::c_int;
pub const P4_TABLE: ::core::ffi::c_int = -5 as ::core::ffi::c_int;
pub const P4_DYNAMIC: ::core::ffi::c_int = -6 as ::core::ffi::c_int;
pub const P4_VTAB: ::core::ffi::c_int = -11 as ::core::ffi::c_int;
pub const P5_ConstraintNotNull: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const P5_ConstraintCheck: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3OpenTable(
    mut pParse: *mut Parse,
    mut iCur: ::core::ffi::c_int,
    mut iDb: ::core::ffi::c_int,
    mut pTab: *mut Table,
    mut opcode: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        v = (*pParse).pVdbe;
        if (*(*pParse).db).noSharedCache == 0 {
            sqlite3TableLock(
                pParse,
                iDb,
                (*pTab).tnum,
                (if opcode == OP_OpenWrite {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as u8_0,
                (*pTab).zName,
            );
        }
        if (*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0 {
            sqlite3VdbeAddOp4Int(
                v,
                opcode,
                iCur,
                (*pTab).tnum as ::core::ffi::c_int,
                iDb,
                (*pTab).nNVCol as ::core::ffi::c_int,
            );
        } else {
            let mut pPk: *mut Index = sqlite3PrimaryKeyIndex(pTab);
            sqlite3VdbeAddOp3(v, opcode, iCur, (*pPk).tnum as ::core::ffi::c_int, iDb);
            sqlite3VdbeSetP4KeyInfo(pParse, pPk);
        };
    }
}
#[inline(never)]
unsafe extern "C" fn computeIndexAffStr(
    mut db: *mut sqlite3,
    mut pIdx: *mut Index,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut pTab: *mut Table = (*pIdx).pTable;
        (*pIdx).zColAff = sqlite3DbMallocRaw(
            ::core::ptr::null_mut::<sqlite3>(),
            ((*pIdx).nColumn as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u64_0,
        ) as *mut ::core::ffi::c_char;
        if (*pIdx).zColAff.is_null() {
            sqlite3OomFault(db);
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        n = 0 as ::core::ffi::c_int;
        while n < (*pIdx).nColumn as ::core::ffi::c_int {
            let mut x: i16_0 = *(*pIdx).aiColumn.offset(n as isize);
            let mut aff: ::core::ffi::c_char = 0;
            if x as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                aff = (*(*pTab).aCol.offset(x as isize)).affinity;
            } else if x as ::core::ffi::c_int == XN_ROWID {
                aff = SQLITE_AFF_INTEGER as ::core::ffi::c_char;
            } else {
                aff = sqlite3ExprAffinity(
                    (*(&raw mut (*(*pIdx).aColExpr).a as *mut ExprList_item)
                        .offset(n as isize))
                        .pExpr,
                );
            }
            if (aff as ::core::ffi::c_int) < SQLITE_AFF_BLOB {
                aff = SQLITE_AFF_BLOB as ::core::ffi::c_char;
            }
            if aff as ::core::ffi::c_int > SQLITE_AFF_NUMERIC {
                aff = SQLITE_AFF_NUMERIC as ::core::ffi::c_char;
            }
            *(*pIdx).zColAff.offset(n as isize) = aff;
            n += 1;
        }
        *(*pIdx).zColAff.offset(n as isize) = 0 as ::core::ffi::c_char;
        return (*pIdx).zColAff;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IndexAffinityStr(
    mut db: *mut sqlite3,
    mut pIdx: *mut Index,
) -> *const ::core::ffi::c_char {
    unsafe {
        if (*pIdx).zColAff.is_null() {
            return computeIndexAffStr(db, pIdx);
        }
        return (*pIdx).zColAff;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TableAffinityStr(
    mut db: *mut sqlite3,
    mut pTab: *const Table,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zColAff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        zColAff = sqlite3DbMallocRaw(
            db,
            ((*pTab).nCol as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u64_0,
        ) as *mut ::core::ffi::c_char;
        if !zColAff.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            let mut j: ::core::ffi::c_int = 0;
            j = 0 as ::core::ffi::c_int;
            i = j;
            while i < (*pTab).nCol as ::core::ffi::c_int {
                if (*(*pTab).aCol.offset(i as isize)).colFlags as ::core::ffi::c_int
                    & COLFLAG_VIRTUAL == 0 as ::core::ffi::c_int
                {
                    let c2rust_fresh3 = j;
                    j = j + 1;
                    *zColAff.offset(c2rust_fresh3 as isize) = (*(*pTab)
                        .aCol
                        .offset(i as isize))
                        .affinity;
                }
                i += 1;
            }
            loop {
                let c2rust_fresh4 = j;
                j = j - 1;
                *zColAff.offset(c2rust_fresh4 as isize) = 0 as ::core::ffi::c_char;
                if !(j >= 0 as ::core::ffi::c_int
                    && *zColAff.offset(j as isize) as ::core::ffi::c_int
                        <= SQLITE_AFF_BLOB)
                {
                    break;
                }
            }
        }
        return zColAff;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TableAffinity(
    mut v: *mut Vdbe,
    mut pTab: *mut Table,
    mut iReg: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut zColAff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if (*pTab).tabFlags & TF_Strict as u32_0 != 0 {
            if iReg == 0 as ::core::ffi::c_int {
                let mut pPrev: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
                let mut p3: ::core::ffi::c_int = 0;
                sqlite3VdbeAppendP4(v, pTab as *mut ::core::ffi::c_void, P4_TABLE);
                pPrev = sqlite3VdbeGetLastOp(v);
                (*pPrev).opcode = OP_TypeCheck as u8_0;
                p3 = (*pPrev).p3;
                (*pPrev).p3 = 0 as ::core::ffi::c_int;
                sqlite3VdbeAddOp3(v, OP_MakeRecord, (*pPrev).p1, (*pPrev).p2, p3);
            } else {
                sqlite3VdbeAddOp2(
                    v,
                    OP_TypeCheck,
                    iReg,
                    (*pTab).nNVCol as ::core::ffi::c_int,
                );
                sqlite3VdbeAppendP4(v, pTab as *mut ::core::ffi::c_void, P4_TABLE);
            }
            return;
        }
        zColAff = (*pTab).zColAff;
        if zColAff.is_null() {
            zColAff = sqlite3TableAffinityStr(::core::ptr::null_mut::<sqlite3>(), pTab);
            if zColAff.is_null() {
                sqlite3OomFault(sqlite3VdbeDb(v));
                return;
            }
            (*pTab).zColAff = zColAff;
        }
        i = (strlen(zColAff) & 0x3fffffff as ::core::ffi::c_int as size_t)
            as ::core::ffi::c_int;
        if i != 0 {
            if iReg != 0 {
                sqlite3VdbeAddOp4(
                    v,
                    OP_Affinity,
                    iReg,
                    i,
                    0 as ::core::ffi::c_int,
                    zColAff,
                    i,
                );
            } else {
                sqlite3VdbeChangeP4(v, -1 as ::core::ffi::c_int, zColAff, i);
            }
        }
    }
}
unsafe extern "C" fn readsTable(
    mut p: *mut Parse,
    mut iDb: ::core::ffi::c_int,
    mut pTab: *mut Table,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: *mut Vdbe = sqlite3GetVdbe(p);
        let mut i: ::core::ffi::c_int = 0;
        let mut iEnd: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(v);
        let mut pVTab: *mut VTable = if (*pTab).eTabType as ::core::ffi::c_int
            == TABTYP_VTAB
        {
            sqlite3GetVTable((*p).db, pTab)
        } else {
            ::core::ptr::null_mut::<VTable>()
        };
        i = 1 as ::core::ffi::c_int;
        while i < iEnd {
            let mut pOp: *mut VdbeOp = sqlite3VdbeGetOp(v, i);
            if (*pOp).opcode as ::core::ffi::c_int == OP_OpenRead && (*pOp).p3 == iDb {
                let mut pIndex: *mut Index = ::core::ptr::null_mut::<Index>();
                let mut tnum: Pgno = (*pOp).p2 as Pgno;
                if tnum == (*pTab).tnum {
                    return 1 as ::core::ffi::c_int;
                }
                pIndex = (*pTab).pIndex;
                while !pIndex.is_null() {
                    if tnum == (*pIndex).tnum {
                        return 1 as ::core::ffi::c_int;
                    }
                    pIndex = (*pIndex).pNext;
                }
            }
            if (*pOp).opcode as ::core::ffi::c_int == OP_VOpen
                && (*pOp).p4.pVtab == pVTab
            {
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn exprColumnFlagUnion(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int == TK_COLUMN
            && (*pExpr).iColumn as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        {
            (*pWalker).eCode = ((*pWalker).eCode as ::core::ffi::c_int
                | (*(*(*pWalker).u.pTab).aCol.offset((*pExpr).iColumn as isize)).colFlags
                    as ::core::ffi::c_int) as u16_0;
        }
        return WRC_Continue;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ComputeGeneratedColumns(
    mut pParse: *mut Parse,
    mut iRegStore: ::core::ffi::c_int,
    mut pTab: *mut Table,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
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
        let mut pRedo: *mut Column = ::core::ptr::null_mut::<Column>();
        let mut eProgress: ::core::ffi::c_int = 0;
        let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
        sqlite3TableAffinity((*pParse).pVdbe, pTab, iRegStore);
        if (*pTab).tabFlags & TF_HasStored as u32_0 != 0 as u32_0 {
            pOp = sqlite3VdbeGetLastOp((*pParse).pVdbe);
            if (*pOp).opcode as ::core::ffi::c_int == OP_Affinity {
                let mut ii: ::core::ffi::c_int = 0;
                let mut jj: ::core::ffi::c_int = 0;
                let mut zP4: *mut ::core::ffi::c_char = (*pOp).p4.z;
                jj = 0 as ::core::ffi::c_int;
                ii = jj;
                while *zP4.offset(jj as isize) != 0 {
                    if !((*(*pTab).aCol.offset(ii as isize)).colFlags
                        as ::core::ffi::c_int & COLFLAG_VIRTUAL != 0)
                    {
                        if (*(*pTab).aCol.offset(ii as isize)).colFlags
                            as ::core::ffi::c_int & COLFLAG_STORED != 0
                        {
                            *zP4.offset(jj as isize) = SQLITE_AFF_NONE
                                as ::core::ffi::c_char;
                        }
                        jj += 1;
                    }
                    ii += 1;
                }
            } else if (*pOp).opcode as ::core::ffi::c_int == OP_TypeCheck {
                (*pOp).p3 = 1 as ::core::ffi::c_int;
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pTab).nCol as ::core::ffi::c_int {
            if (*(*pTab).aCol.offset(i as isize)).colFlags as ::core::ffi::c_int
                & COLFLAG_GENERATED != 0
            {
                let ref mut c2rust_fresh5 = (*(*pTab).aCol.offset(i as isize)).colFlags;
                *c2rust_fresh5 = (*c2rust_fresh5 as ::core::ffi::c_int
                    | COLFLAG_NOTAVAIL) as u16_0;
            }
            i += 1;
        }
        w.u.pTab = pTab as *mut Table;
        w.xExprCallback = Some(
            exprColumnFlagUnion
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = None;
        w.xSelectCallback2 = None;
        (*pParse).iSelfTab = -iRegStore;
        loop {
            eProgress = 0 as ::core::ffi::c_int;
            pRedo = ::core::ptr::null_mut::<Column>();
            i = 0 as ::core::ffi::c_int;
            while i < (*pTab).nCol as ::core::ffi::c_int {
                let mut pCol: *mut Column = (*pTab).aCol.offset(i as isize);
                if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_NOTAVAIL
                    != 0 as ::core::ffi::c_int
                {
                    let mut x: ::core::ffi::c_int = 0;
                    (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                        | COLFLAG_BUSY) as u16_0;
                    w.eCode = 0 as u16_0;
                    sqlite3WalkExpr(&raw mut w, sqlite3ColumnExpr(pTab, pCol));
                    (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                        & !COLFLAG_BUSY) as u16_0;
                    if w.eCode as ::core::ffi::c_int & COLFLAG_NOTAVAIL != 0 {
                        pRedo = pCol;
                    } else {
                        eProgress = 1 as ::core::ffi::c_int;
                        x = sqlite3TableColumnToStorage(pTab, i as i16_0)
                            as ::core::ffi::c_int + iRegStore;
                        sqlite3ExprCodeGeneratedColumn(pParse, pTab, pCol, x);
                        (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                            & !COLFLAG_NOTAVAIL) as u16_0;
                    }
                }
                i += 1;
            }
            if !(!pRedo.is_null() && eProgress != 0) {
                break;
            }
        }
        if !pRedo.is_null() {
            sqlite3ErrorMsg(
                pParse,
                b"generated column loop on \"%s\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pRedo).zCnName,
            );
        }
        (*pParse).iSelfTab = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn autoIncBegin(
    mut pParse: *mut Parse,
    mut iDb: ::core::ffi::c_int,
    mut pTab: *mut Table,
) -> ::core::ffi::c_int {
    unsafe {
        let mut memId: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*pTab).tabFlags & TF_Autoincrement as u32_0 != 0 as u32_0
            && (*(*pParse).db).mDbFlags & DBFLAG_Vacuum as u32_0 == 0 as u32_0
        {
            let mut pToplevel: *mut Parse = if !(*pParse).pToplevel.is_null() {
                (*pParse).pToplevel
            } else {
                pParse
            };
            let mut pInfo: *mut AutoincInfo = ::core::ptr::null_mut::<AutoincInfo>();
            let mut pSeqTab: *mut Table = (*(*(*(*pParse).db).aDb.offset(iDb as isize))
                .pSchema)
                .pSeqTab;
            if pSeqTab.is_null()
                || !((*pSeqTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0)
                || (*pSeqTab).eTabType as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                || (*pSeqTab).nCol as ::core::ffi::c_int != 2 as ::core::ffi::c_int
            {
                (*pParse).nErr += 1;
                (*pParse).rc = SQLITE_CORRUPT_SEQUENCE;
                return 0 as ::core::ffi::c_int;
            }
            pInfo = (*pToplevel).pAinc;
            while !pInfo.is_null() && (*pInfo).pTab != pTab {
                pInfo = (*pInfo).pNext;
            }
            if pInfo.is_null() {
                pInfo = sqlite3DbMallocRawNN(
                    (*pParse).db,
                    ::core::mem::size_of::<AutoincInfo>() as u64_0,
                ) as *mut AutoincInfo;
                sqlite3ParserAddCleanup(
                    pToplevel,
                    Some(
                        sqlite3DbFree
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    pInfo as *mut ::core::ffi::c_void,
                );
                if (*(*pParse).db).mallocFailed != 0 {
                    return 0 as ::core::ffi::c_int;
                }
                (*pInfo).pNext = (*pToplevel).pAinc;
                (*pToplevel).pAinc = pInfo;
                (*pInfo).pTab = pTab;
                (*pInfo).iDb = iDb;
                (*pToplevel).nMem += 1;
                (*pToplevel).nMem += 1;
                (*pInfo).regCtr = (*pToplevel).nMem;
                (*pToplevel).nMem += 2 as ::core::ffi::c_int;
            }
            memId = (*pInfo).regCtr;
        }
        return memId;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AutoincrementBegin(mut pParse: *mut Parse) {
    unsafe {
        let mut p: *mut AutoincInfo = ::core::ptr::null_mut::<AutoincInfo>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pDb: *mut Db = ::core::ptr::null_mut::<Db>();
        let mut memId: ::core::ffi::c_int = 0;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        p = (*pParse).pAinc;
        while !p.is_null() {
            static mut iLn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            static mut autoInc: [VdbeOpList; 12] = [
                VdbeOpList {
                    opcode: OP_Null as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 0 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_Rewind as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 10 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_Column as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 0 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_Ne as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 9 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_Rowid as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 0 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_Column as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 1 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_AddImm as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 0 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_Copy as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 0 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_Goto as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 11 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_Next as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 2 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_Integer as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 0 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_Close as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 0 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
            ];
            let mut aOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
            pDb = (*db).aDb.offset((*p).iDb as isize) as *mut Db;
            memId = (*p).regCtr;
            sqlite3OpenTable(
                pParse,
                0 as ::core::ffi::c_int,
                (*p).iDb,
                (*(*pDb).pSchema).pSeqTab,
                OP_OpenRead,
            );
            sqlite3VdbeLoadString(
                v,
                memId - 1 as ::core::ffi::c_int,
                (*(*p).pTab).zName,
            );
            aOp = sqlite3VdbeAddOpList(
                v,
                (::core::mem::size_of::<[VdbeOpList; 12]>() as usize)
                    .wrapping_div(::core::mem::size_of::<VdbeOpList>() as usize)
                    as ::core::ffi::c_int,
                &raw const autoInc as *const VdbeOpList,
                iLn,
            );
            if aOp.is_null() {
                break;
            }
            (*aOp.offset(0 as ::core::ffi::c_int as isize)).p2 = memId;
            (*aOp.offset(0 as ::core::ffi::c_int as isize)).p3 = memId
                + 2 as ::core::ffi::c_int;
            (*aOp.offset(2 as ::core::ffi::c_int as isize)).p3 = memId;
            (*aOp.offset(3 as ::core::ffi::c_int as isize)).p1 = memId
                - 1 as ::core::ffi::c_int;
            (*aOp.offset(3 as ::core::ffi::c_int as isize)).p3 = memId;
            (*aOp.offset(3 as ::core::ffi::c_int as isize)).p5 = SQLITE_JUMPIFNULL
                as u16_0;
            (*aOp.offset(4 as ::core::ffi::c_int as isize)).p2 = memId
                + 1 as ::core::ffi::c_int;
            (*aOp.offset(5 as ::core::ffi::c_int as isize)).p3 = memId;
            (*aOp.offset(6 as ::core::ffi::c_int as isize)).p1 = memId;
            (*aOp.offset(7 as ::core::ffi::c_int as isize)).p2 = memId
                + 2 as ::core::ffi::c_int;
            (*aOp.offset(7 as ::core::ffi::c_int as isize)).p1 = memId;
            (*aOp.offset(10 as ::core::ffi::c_int as isize)).p2 = memId;
            if (*pParse).nTab == 0 as ::core::ffi::c_int {
                (*pParse).nTab = 1 as ::core::ffi::c_int;
            }
            p = (*p).pNext;
        }
    }
}
unsafe extern "C" fn autoIncStep(
    mut pParse: *mut Parse,
    mut memId: ::core::ffi::c_int,
    mut regRowid: ::core::ffi::c_int,
) {
    unsafe {
        if memId > 0 as ::core::ffi::c_int {
            sqlite3VdbeAddOp2((*pParse).pVdbe, OP_MemMax, memId, regRowid);
        }
    }
}
#[inline(never)]
unsafe extern "C" fn autoIncrementEnd(mut pParse: *mut Parse) {
    unsafe {
        let mut p: *mut AutoincInfo = ::core::ptr::null_mut::<AutoincInfo>();
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut db: *mut sqlite3 = (*pParse).db;
        p = (*pParse).pAinc;
        while !p.is_null() {
            static mut iLn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            static mut autoIncEnd: [VdbeOpList; 5] = [
                VdbeOpList {
                    opcode: OP_NotNull as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 2 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_NewRowid as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 0 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_MakeRecord as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 2 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_Insert as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 0 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
                VdbeOpList {
                    opcode: OP_Close as u8_0,
                    p1: 0 as ::core::ffi::c_schar,
                    p2: 0 as ::core::ffi::c_schar,
                    p3: 0 as ::core::ffi::c_schar

                },
            ];
            let mut aOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
            let mut pDb: *mut Db = (*db).aDb.offset((*p).iDb as isize) as *mut Db;
            let mut iRec: ::core::ffi::c_int = 0;
            let mut memId: ::core::ffi::c_int = (*p).regCtr;
            iRec = sqlite3GetTempReg(pParse);
            sqlite3VdbeAddOp3(
                v,
                OP_Le,
                memId + 2 as ::core::ffi::c_int,
                sqlite3VdbeCurrentAddr(v) + 7 as ::core::ffi::c_int,
                memId,
            );
            sqlite3OpenTable(
                pParse,
                0 as ::core::ffi::c_int,
                (*p).iDb,
                (*(*pDb).pSchema).pSeqTab,
                OP_OpenWrite,
            );
            aOp = sqlite3VdbeAddOpList(
                v,
                (::core::mem::size_of::<[VdbeOpList; 5]>() as usize)
                    .wrapping_div(::core::mem::size_of::<VdbeOpList>() as usize)
                    as ::core::ffi::c_int,
                &raw const autoIncEnd as *const VdbeOpList,
                iLn,
            );
            if aOp.is_null() {
                break;
            }
            (*aOp.offset(0 as ::core::ffi::c_int as isize)).p1 = memId
                + 1 as ::core::ffi::c_int;
            (*aOp.offset(1 as ::core::ffi::c_int as isize)).p2 = memId
                + 1 as ::core::ffi::c_int;
            (*aOp.offset(2 as ::core::ffi::c_int as isize)).p1 = memId
                - 1 as ::core::ffi::c_int;
            (*aOp.offset(2 as ::core::ffi::c_int as isize)).p3 = iRec;
            (*aOp.offset(3 as ::core::ffi::c_int as isize)).p2 = iRec;
            (*aOp.offset(3 as ::core::ffi::c_int as isize)).p3 = memId
                + 1 as ::core::ffi::c_int;
            (*aOp.offset(3 as ::core::ffi::c_int as isize)).p5 = OPFLAG_APPEND as u16_0;
            sqlite3ReleaseTempReg(pParse, iRec);
            p = (*p).pNext;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AutoincrementEnd(mut pParse: *mut Parse) {
    unsafe {
        if !(*pParse).pAinc.is_null() {
            autoIncrementEnd(pParse);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MultiValuesEnd(
    mut pParse: *mut Parse,
    mut pVal: *mut Select,
) {
    unsafe {
        if !pVal.is_null() && (*(*pVal).pSrc).nSrc > 0 as ::core::ffi::c_int {
            let mut pItem: *mut SrcItem = (&raw mut (*(*pVal).pSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem;
            if (*pItem).fg.isSubquery() != 0 {
                sqlite3VdbeEndCoroutine((*pParse).pVdbe, (*(*pItem).u4.pSubq).regReturn);
                sqlite3VdbeJumpHere(
                    (*pParse).pVdbe,
                    (*(*pItem).u4.pSubq).addrFillSub - 1 as ::core::ffi::c_int,
                );
            }
        }
    }
}
unsafe extern "C" fn exprListIsConstant(
    mut pParse: *mut Parse,
    mut pRow: *mut ExprList,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pRow).nExpr {
            if 0 as ::core::ffi::c_int
                == sqlite3ExprIsConstant(
                    pParse,
                    (*(&raw mut (*pRow).a as *mut ExprList_item).offset(ii as isize))
                        .pExpr,
                )
            {
                return 0 as ::core::ffi::c_int;
            }
            ii += 1;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn exprListIsNoAffinity(
    mut pParse: *mut Parse,
    mut pRow: *mut ExprList,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        if exprListIsConstant(pParse, pRow) == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pRow).nExpr {
            let mut pExpr: *mut Expr = (*(&raw mut (*pRow).a as *mut ExprList_item)
                .offset(ii as isize))
                .pExpr;
            if 0 as ::core::ffi::c_int
                != sqlite3ExprAffinity(pExpr) as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            ii += 1;
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MultiValues(
    mut pParse: *mut Parse,
    mut pLeft: *mut Select,
    mut pRow: *mut ExprList,
) -> *mut Select {
    unsafe {
        if (*pParse).bHasWith() as ::core::ffi::c_int != 0
            || (*(*pParse).db).init.busy as ::core::ffi::c_int != 0
            || exprListIsConstant(pParse, pRow) == 0 as ::core::ffi::c_int
            || (*(*pLeft).pSrc).nSrc == 0 as ::core::ffi::c_int
                && exprListIsNoAffinity(pParse, (*pLeft).pEList)
                    == 0 as ::core::ffi::c_int
            || (*pParse).eParseMode as ::core::ffi::c_int != PARSE_MODE_NORMAL
        {
            let mut pSelect: *mut Select = ::core::ptr::null_mut::<Select>();
            let mut f: ::core::ffi::c_int = SF_Values | SF_MultiValue;
            if (*(*pLeft).pSrc).nSrc != 0 {
                sqlite3MultiValuesEnd(pParse, pLeft);
                f = SF_Values;
            } else if !(*pLeft).pPrior.is_null() {
                f = (f as u32_0 & (*pLeft).selFlags) as ::core::ffi::c_int;
            }
            pSelect = sqlite3SelectNew(
                pParse,
                pRow,
                ::core::ptr::null_mut::<SrcList>(),
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<ExprList>(),
                f as u32_0,
                ::core::ptr::null_mut::<Expr>(),
            );
            (*pLeft).selFlags = ((*pLeft).selFlags as ::core::ffi::c_uint
                & !(SF_MultiValue as u32_0) as ::core::ffi::c_uint) as u32_0;
            if !pSelect.is_null() {
                (*pSelect).op = TK_ALL as u8_0;
                (*pSelect).pPrior = pLeft;
                pLeft = pSelect;
            }
        } else {
            let mut p: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
            if (*(*pLeft).pSrc).nSrc == 0 as ::core::ffi::c_int {
                let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
                let mut pRet: *mut Select = sqlite3SelectNew(
                    pParse,
                    ::core::ptr::null_mut::<ExprList>(),
                    ::core::ptr::null_mut::<SrcList>(),
                    ::core::ptr::null_mut::<Expr>(),
                    ::core::ptr::null_mut::<ExprList>(),
                    ::core::ptr::null_mut::<Expr>(),
                    ::core::ptr::null_mut::<ExprList>(),
                    0 as u32_0,
                    ::core::ptr::null_mut::<Expr>(),
                );
                if (*(*pParse).db).mDbFlags & DBFLAG_SchemaKnownOk as u32_0 == 0 as u32_0
                {
                    sqlite3ReadSchema(pParse);
                }
                if !pRet.is_null() {
                    let mut dest: SelectDest = SelectDest {
                        eDest: 0,
                        iSDParm: 0,
                        iSDParm2: 0,
                        iSdst: 0,
                        nSdst: 0,
                        zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        pOrderBy: ::core::ptr::null_mut::<ExprList>()

                    };
                    let mut pSubq: *mut Subquery = ::core::ptr::null_mut::<Subquery>();
                    (*(*pRet).pSrc).nSrc = 1 as ::core::ffi::c_int;
                    (*pRet).pPrior = (*pLeft).pPrior;
                    (*pRet).op = (*pLeft).op;
                    if !(*pRet).pPrior.is_null() {
                        (*pRet).selFlags = ((*pRet).selFlags as ::core::ffi::c_uint
                            | SF_Values as ::core::ffi::c_uint) as u32_0;
                    }
                    (*pLeft).pPrior = ::core::ptr::null_mut::<Select>();
                    (*pLeft).op = TK_SELECT as u8_0;
                    p = (&raw mut (*(*pRet).pSrc).a as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem;
                    (*p)
                        .fg
                        .set_viaCoroutine(
                            1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                        );
                    (*p).iCursor = -1 as ::core::ffi::c_int;
                    (*p).u1.nRow = 2 as u32_0;
                    if sqlite3SrcItemAttachSubquery(
                        pParse,
                        p,
                        pLeft,
                        0 as ::core::ffi::c_int,
                    ) != 0
                    {
                        pSubq = (*p).u4.pSubq;
                        (*pSubq).addrFillSub = sqlite3VdbeCurrentAddr(v)
                            + 1 as ::core::ffi::c_int;
                        (*pParse).nMem += 1;
                        (*pSubq).regReturn = (*pParse).nMem;
                        sqlite3VdbeAddOp3(
                            v,
                            OP_InitCoroutine,
                            (*pSubq).regReturn,
                            0 as ::core::ffi::c_int,
                            (*pSubq).addrFillSub,
                        );
                        sqlite3SelectDestInit(
                            &raw mut dest,
                            SRT_Coroutine,
                            (*pSubq).regReturn,
                        );
                        dest.iSdst = (*pParse).nMem + 3 as ::core::ffi::c_int;
                        dest.nSdst = (*(*pLeft).pEList).nExpr;
                        (*pParse).nMem += 2 as ::core::ffi::c_int + dest.nSdst;
                        (*pLeft).selFlags = ((*pLeft).selFlags as ::core::ffi::c_uint
                            | SF_MultiValue as ::core::ffi::c_uint) as u32_0;
                        sqlite3Select(pParse, pLeft, &raw mut dest);
                        (*pSubq).regResult = dest.iSdst;
                    }
                    pLeft = pRet;
                }
            } else {
                p = (&raw mut (*(*pLeft).pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem;
                (*p).u1.nRow = (*p).u1.nRow.wrapping_add(1);
            }
            if (*pParse).nErr == 0 as ::core::ffi::c_int {
                let mut pSubq_0: *mut Subquery = ::core::ptr::null_mut::<Subquery>();
                pSubq_0 = (*p).u4.pSubq;
                if (*(*(*pSubq_0).pSelect).pEList).nExpr != (*pRow).nExpr {
                    sqlite3SelectWrongNumTermsError(pParse, (*pSubq_0).pSelect);
                } else {
                    sqlite3ExprCodeExprList(
                        pParse,
                        pRow,
                        (*pSubq_0).regResult,
                        0 as ::core::ffi::c_int,
                        0 as u8_0,
                    );
                    sqlite3VdbeAddOp1((*pParse).pVdbe, OP_Yield, (*pSubq_0).regReturn);
                }
            }
            sqlite3ExprListDelete((*pParse).db, pRow);
        }
        return pLeft;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Insert(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pSelect: *mut Select,
    mut pColumn: *mut IdList,
    mut onError: ::core::ffi::c_int,
    mut pUpsert: *mut Upsert,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut nColumn: ::core::ffi::c_int = 0;
        let mut nHidden: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iDataCur: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iIdxCur: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ipkColumn: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut endOfLoop: ::core::ffi::c_int = 0;
        let mut srcTab: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addrInsTop: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addrCont: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut dest: SelectDest = SelectDest {
            eDest: 0,
            iSDParm: 0,
            iSDParm2: 0,
            iSdst: 0,
            nSdst: 0,
            zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pOrderBy: ::core::ptr::null_mut::<ExprList>()

        };
        let mut iDb: ::core::ffi::c_int = 0;
        let mut useTempTable: u8_0 = 0 as u8_0;
        let mut appendFlag: u8_0 = 0 as u8_0;
        let mut withoutRowid: u8_0 = 0;
        let mut bIdListInOrder: u8_0 = 0;
        let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut iRegStore: ::core::ffi::c_int = 0;
        let mut regFromSelect: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regAutoinc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regRowCount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regIns: ::core::ffi::c_int = 0;
        let mut regRowid: ::core::ffi::c_int = 0;
        let mut regData: ::core::ffi::c_int = 0;
        let mut aRegIdx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        let mut aTabColMap: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        let mut isView: ::core::ffi::c_int = 0;
        let mut pTrigger: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        let mut tmask: ::core::ffi::c_int = 0;
        db = (*pParse).db;
        if !((*pParse).nErr != 0) {
            dest.iSDParm = 0 as ::core::ffi::c_int;
            if !pSelect.is_null()
                && (*pSelect).selFlags & SF_Values as u32_0 != 0 as u32_0
                && (*pSelect).pPrior.is_null()
            {
                pList = (*pSelect).pEList;
                (*pSelect).pEList = ::core::ptr::null_mut::<ExprList>();
                sqlite3SelectDelete(db, pSelect);
                pSelect = ::core::ptr::null_mut::<Select>();
            }
            pTab = sqlite3SrcListLookup(pParse, pTabList);
            if !pTab.is_null() {
                iDb = sqlite3SchemaToIndex(db, (*pTab).pSchema);
                if !(sqlite3AuthCheck(
                    pParse,
                    SQLITE_INSERT,
                    (*pTab).zName,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    (*(*db).aDb.offset(iDb as isize)).zDbSName,
                ) != 0)
                {
                    withoutRowid = !((*pTab).tabFlags & TF_WithoutRowid as u32_0
                        == 0 as u32_0) as ::core::ffi::c_int as u8_0;
                    pTrigger = sqlite3TriggersExist(
                        pParse,
                        pTab,
                        TK_INSERT,
                        ::core::ptr::null_mut::<ExprList>(),
                        &raw mut tmask,
                    );
                    isView = ((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW)
                        as ::core::ffi::c_int;
                    if !(sqlite3ViewGetColumnNames(pParse, pTab) != 0) {
                        if !(sqlite3IsReadOnly(pParse, pTab, pTrigger) != 0) {
                            v = sqlite3GetVdbe(pParse);
                            if !v.is_null() {
                                if (*pParse).nested as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    sqlite3VdbeCountChanges(v);
                                }
                                sqlite3BeginWriteOperation(
                                    pParse,
                                    (!pSelect.is_null() || !pTrigger.is_null())
                                        as ::core::ffi::c_int,
                                    iDb,
                                );
                                if pColumn.is_null() && !pSelect.is_null()
                                    && pTrigger.is_null()
                                    && xferOptimization(pParse, pTab, pSelect, onError, iDb)
                                        != 0
                                {
                                    c2rust_current_block = 16497991052681986260;
                                } else {
                                    regAutoinc = autoIncBegin(pParse, iDb, pTab);
                                    regIns = (*pParse).nMem + 1 as ::core::ffi::c_int;
                                    regRowid = regIns;
                                    (*pParse).nMem
                                        += (*pTab).nCol as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int;
                                    if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                                        regRowid += 1;
                                        (*pParse).nMem += 1;
                                    }
                                    regData = regRowid + 1 as ::core::ffi::c_int;
                                    bIdListInOrder = ((*pTab).tabFlags
                                        & (TF_OOOHidden | TF_HasStored) as u32_0 == 0 as u32_0)
                                        as ::core::ffi::c_int as u8_0;
                                    if !pColumn.is_null() {
                                        aTabColMap = sqlite3DbMallocZero(
                                            db,
                                            ((*pTab).nCol as usize)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                                ) as u64_0,
                                        ) as *mut ::core::ffi::c_int;
                                        if aTabColMap.is_null() {
                                            c2rust_current_block = 14505076636074796664;
                                        } else {
                                            i = 0 as ::core::ffi::c_int;
                                            loop {
                                                if !(i < (*pColumn).nId) {
                                                    c2rust_current_block = 307447392441238883;
                                                    break;
                                                }
                                                j = sqlite3ColumnIndex(
                                                    pTab,
                                                    (*(&raw mut (*pColumn).a as *mut IdList_item)
                                                        .offset(i as isize))
                                                        .zName,
                                                );
                                                if j >= 0 as ::core::ffi::c_int {
                                                    if *aTabColMap.offset(j as isize) == 0 as ::core::ffi::c_int
                                                    {
                                                        *aTabColMap.offset(j as isize) = i
                                                            + 1 as ::core::ffi::c_int;
                                                    }
                                                    if i != j {
                                                        bIdListInOrder = 0 as u8_0;
                                                    }
                                                    if j == (*pTab).iPKey as ::core::ffi::c_int {
                                                        ipkColumn = i;
                                                    }
                                                    if (*(*pTab).aCol.offset(j as isize)).colFlags
                                                        as ::core::ffi::c_int & (COLFLAG_STORED | COLFLAG_VIRTUAL)
                                                        != 0
                                                    {
                                                        sqlite3ErrorMsg(
                                                            pParse,
                                                            b"cannot INSERT into generated column \"%s\"\0".as_ptr()
                                                                as *const ::core::ffi::c_char,
                                                            (*(*pTab).aCol.offset(j as isize)).zCnName,
                                                        );
                                                        c2rust_current_block = 14505076636074796664;
                                                        break;
                                                    }
                                                } else if sqlite3IsRowid(
                                                    (*(&raw mut (*pColumn).a as *mut IdList_item)
                                                        .offset(i as isize))
                                                        .zName,
                                                ) != 0 && withoutRowid == 0
                                                {
                                                    ipkColumn = i;
                                                    bIdListInOrder = 0 as u8_0;
                                                } else {
                                                    sqlite3ErrorMsg(
                                                        pParse,
                                                        b"table %S has no column named %s\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        &raw mut (*pTabList).a as *mut SrcItem,
                                                        (*(&raw mut (*pColumn).a as *mut IdList_item)
                                                            .offset(i as isize))
                                                            .zName,
                                                    );
                                                    (*pParse).set_checkSchema(1 as bft as bft);
                                                    c2rust_current_block = 14505076636074796664;
                                                    break;
                                                }
                                                i += 1;
                                            }
                                        }
                                    } else {
                                        c2rust_current_block = 307447392441238883;
                                    }
                                    match c2rust_current_block {
                                        14505076636074796664 => {}
                                        _ => {
                                            if !pSelect.is_null() {
                                                let mut rc: ::core::ffi::c_int = 0;
                                                if (*(*pSelect).pSrc).nSrc == 1 as ::core::ffi::c_int
                                                    && (*(&raw mut (*(*pSelect).pSrc).a as *mut SrcItem)
                                                        .offset(0 as ::core::ffi::c_int as isize))
                                                        .fg
                                                        .viaCoroutine() as ::core::ffi::c_int != 0
                                                    && (*pSelect).pPrior.is_null()
                                                {
                                                    let mut pItem: *mut SrcItem = (&raw mut (*(*pSelect).pSrc).a
                                                        as *mut SrcItem)
                                                        .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem;
                                                    let mut pSubq: *mut Subquery = ::core::ptr::null_mut::<
                                                        Subquery,
                                                    >();
                                                    pSubq = (*pItem).u4.pSubq;
                                                    dest.iSDParm = (*pSubq).regReturn;
                                                    regFromSelect = (*pSubq).regResult;
                                                    nColumn = (*(*(*pSubq).pSelect).pEList).nExpr;
                                                    sqlite3VdbeExplain(
                                                        pParse,
                                                        0 as u8_0,
                                                        b"SCAN %S\0".as_ptr() as *const ::core::ffi::c_char,
                                                        pItem,
                                                    );
                                                    if bIdListInOrder as ::core::ffi::c_int != 0
                                                        && nColumn == (*pTab).nCol as ::core::ffi::c_int
                                                    {
                                                        regData = regFromSelect;
                                                        regRowid = regData - 1 as ::core::ffi::c_int;
                                                        regIns = regRowid
                                                            - (if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB
                                                            {
                                                                1 as ::core::ffi::c_int
                                                            } else {
                                                                0 as ::core::ffi::c_int
                                                            });
                                                    }
                                                    c2rust_current_block = 5005389895767293342;
                                                } else {
                                                    let mut addrTop: ::core::ffi::c_int = 0;
                                                    (*pParse).nMem += 1;
                                                    let mut regYield: ::core::ffi::c_int = (*pParse).nMem;
                                                    addrTop = sqlite3VdbeCurrentAddr(v)
                                                        + 1 as ::core::ffi::c_int;
                                                    sqlite3VdbeAddOp3(
                                                        v,
                                                        OP_InitCoroutine,
                                                        regYield,
                                                        0 as ::core::ffi::c_int,
                                                        addrTop,
                                                    );
                                                    sqlite3SelectDestInit(
                                                        &raw mut dest,
                                                        SRT_Coroutine,
                                                        regYield,
                                                    );
                                                    dest.iSdst = if bIdListInOrder as ::core::ffi::c_int != 0 {
                                                        regData
                                                    } else {
                                                        0 as ::core::ffi::c_int
                                                    };
                                                    dest.nSdst = (*pTab).nCol as ::core::ffi::c_int;
                                                    rc = sqlite3Select(pParse, pSelect, &raw mut dest);
                                                    regFromSelect = dest.iSdst;
                                                    if rc != 0 || (*pParse).nErr != 0 {
                                                        c2rust_current_block = 14505076636074796664;
                                                    } else {
                                                        sqlite3VdbeEndCoroutine(v, regYield);
                                                        sqlite3VdbeJumpHere(v, addrTop - 1 as ::core::ffi::c_int);
                                                        nColumn = (*(*pSelect).pEList).nExpr;
                                                        c2rust_current_block = 5005389895767293342;
                                                    }
                                                }
                                                match c2rust_current_block {
                                                    14505076636074796664 => {}
                                                    _ => {
                                                        if !pTrigger.is_null() || readsTable(pParse, iDb, pTab) != 0
                                                        {
                                                            useTempTable = 1 as u8_0;
                                                        }
                                                        if useTempTable != 0 {
                                                            let mut regRec: ::core::ffi::c_int = 0;
                                                            let mut regTempRowid: ::core::ffi::c_int = 0;
                                                            let mut addrL: ::core::ffi::c_int = 0;
                                                            let c2rust_fresh0 = (*pParse).nTab;
                                                            (*pParse).nTab = (*pParse).nTab + 1;
                                                            srcTab = c2rust_fresh0;
                                                            regRec = sqlite3GetTempReg(pParse);
                                                            regTempRowid = sqlite3GetTempReg(pParse);
                                                            sqlite3VdbeAddOp2(v, OP_OpenEphemeral, srcTab, nColumn);
                                                            addrL = sqlite3VdbeAddOp1(v, OP_Yield, dest.iSDParm);
                                                            sqlite3VdbeAddOp3(
                                                                v,
                                                                OP_MakeRecord,
                                                                regFromSelect,
                                                                nColumn,
                                                                regRec,
                                                            );
                                                            sqlite3VdbeAddOp2(v, OP_NewRowid, srcTab, regTempRowid);
                                                            sqlite3VdbeAddOp3(
                                                                v,
                                                                OP_Insert,
                                                                srcTab,
                                                                regRec,
                                                                regTempRowid,
                                                            );
                                                            sqlite3VdbeGoto(v, addrL);
                                                            sqlite3VdbeJumpHere(v, addrL);
                                                            sqlite3ReleaseTempReg(pParse, regRec);
                                                            sqlite3ReleaseTempReg(pParse, regTempRowid);
                                                        }
                                                        c2rust_current_block = 16590946904645350046;
                                                    }
                                                }
                                            } else {
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
                                                memset(
                                                    &raw mut sNC as *mut ::core::ffi::c_void,
                                                    0 as ::core::ffi::c_int,
                                                    ::core::mem::size_of::<NameContext>() as size_t,
                                                );
                                                sNC.pParse = pParse;
                                                srcTab = -1 as ::core::ffi::c_int;
                                                if !pList.is_null() {
                                                    nColumn = (*pList).nExpr;
                                                    if sqlite3ResolveExprListNames(&raw mut sNC, pList) != 0 {
                                                        c2rust_current_block = 14505076636074796664;
                                                    } else {
                                                        c2rust_current_block = 16590946904645350046;
                                                    }
                                                } else {
                                                    nColumn = 0 as ::core::ffi::c_int;
                                                    c2rust_current_block = 16590946904645350046;
                                                }
                                            }
                                            match c2rust_current_block {
                                                14505076636074796664 => {}
                                                _ => {
                                                    if pColumn.is_null() && nColumn > 0 as ::core::ffi::c_int {
                                                        ipkColumn = (*pTab).iPKey as ::core::ffi::c_int;
                                                        if ipkColumn >= 0 as ::core::ffi::c_int
                                                            && (*pTab).tabFlags & TF_HasGenerated as u32_0 != 0 as u32_0
                                                        {
                                                            i = ipkColumn - 1 as ::core::ffi::c_int;
                                                            while i >= 0 as ::core::ffi::c_int {
                                                                if (*(*pTab).aCol.offset(i as isize)).colFlags
                                                                    as ::core::ffi::c_int & COLFLAG_GENERATED != 0
                                                                {
                                                                    ipkColumn -= 1;
                                                                }
                                                                i -= 1;
                                                            }
                                                        }
                                                        if (*pTab).tabFlags
                                                            & (TF_HasGenerated | TF_HasHidden) as u32_0 != 0 as u32_0
                                                        {
                                                            i = 0 as ::core::ffi::c_int;
                                                            while i < (*pTab).nCol as ::core::ffi::c_int {
                                                                if (*(*pTab).aCol.offset(i as isize)).colFlags
                                                                    as ::core::ffi::c_int & COLFLAG_NOINSERT != 0
                                                                {
                                                                    nHidden += 1;
                                                                }
                                                                i += 1;
                                                            }
                                                        }
                                                        if nColumn != (*pTab).nCol as ::core::ffi::c_int - nHidden {
                                                            sqlite3ErrorMsg(
                                                                pParse,
                                                                b"table %S has %d columns but %d values were supplied\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                &raw mut (*pTabList).a as *mut SrcItem,
                                                                (*pTab).nCol as ::core::ffi::c_int - nHidden,
                                                                nColumn,
                                                            );
                                                            c2rust_current_block = 14505076636074796664;
                                                        } else {
                                                            c2rust_current_block = 12890877304563811856;
                                                        }
                                                    } else {
                                                        c2rust_current_block = 12890877304563811856;
                                                    }
                                                    match c2rust_current_block {
                                                        14505076636074796664 => {}
                                                        _ => {
                                                            if !pColumn.is_null() && nColumn != (*pColumn).nId {
                                                                sqlite3ErrorMsg(
                                                                    pParse,
                                                                    b"%d values for %d columns\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    nColumn,
                                                                    (*pColumn).nId,
                                                                );
                                                                c2rust_current_block = 14505076636074796664;
                                                            } else {
                                                                if (*db).flags & SQLITE_CountRows
                                                                    != 0 as ::core::ffi::c_ulonglong && (*pParse).nested == 0
                                                                    && (*pParse).pTriggerTab.is_null()
                                                                    && (*pParse).bReturning == 0
                                                                {
                                                                    (*pParse).nMem += 1;
                                                                    regRowCount = (*pParse).nMem;
                                                                    sqlite3VdbeAddOp2(
                                                                        v,
                                                                        OP_Integer,
                                                                        0 as ::core::ffi::c_int,
                                                                        regRowCount,
                                                                    );
                                                                }
                                                                if isView == 0 {
                                                                    let mut nIdx: ::core::ffi::c_int = 0;
                                                                    nIdx = sqlite3OpenTableAndIndices(
                                                                        pParse,
                                                                        pTab,
                                                                        OP_OpenWrite,
                                                                        0 as u8_0,
                                                                        -1 as ::core::ffi::c_int,
                                                                        ::core::ptr::null_mut::<u8_0>(),
                                                                        &raw mut iDataCur,
                                                                        &raw mut iIdxCur,
                                                                    );
                                                                    aRegIdx = sqlite3DbMallocRawNN(
                                                                        db,
                                                                        (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                                            .wrapping_mul((nIdx + 2 as ::core::ffi::c_int) as usize)
                                                                            as u64_0,
                                                                    ) as *mut ::core::ffi::c_int;
                                                                    if aRegIdx.is_null() {
                                                                        c2rust_current_block = 14505076636074796664;
                                                                    } else {
                                                                        i = 0 as ::core::ffi::c_int;
                                                                        pIdx = (*pTab).pIndex;
                                                                        while i < nIdx {
                                                                            (*pParse).nMem += 1;
                                                                            *aRegIdx.offset(i as isize) = (*pParse).nMem;
                                                                            (*pParse).nMem += (*pIdx).nColumn as ::core::ffi::c_int;
                                                                            pIdx = (*pIdx).pNext;
                                                                            i += 1;
                                                                        }
                                                                        (*pParse).nMem += 1;
                                                                        *aRegIdx.offset(i as isize) = (*pParse).nMem;
                                                                        c2rust_current_block = 2884634553824165030;
                                                                    }
                                                                } else {
                                                                    c2rust_current_block = 2884634553824165030;
                                                                }
                                                                match c2rust_current_block {
                                                                    14505076636074796664 => {}
                                                                    _ => {
                                                                        if !pUpsert.is_null() {
                                                                            let mut pNx: *mut Upsert = ::core::ptr::null_mut::<
                                                                                Upsert,
                                                                            >();
                                                                            if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                                                                                sqlite3ErrorMsg(
                                                                                    pParse,
                                                                                    b"UPSERT not implemented for virtual table \"%s\"\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                    (*pTab).zName,
                                                                                );
                                                                                c2rust_current_block = 14505076636074796664;
                                                                            } else if (*pTab).eTabType as ::core::ffi::c_int
                                                                                == TABTYP_VIEW
                                                                            {
                                                                                sqlite3ErrorMsg(
                                                                                    pParse,
                                                                                    b"cannot UPSERT a view\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                );
                                                                                c2rust_current_block = 14505076636074796664;
                                                                            } else if sqlite3HasExplicitNulls(
                                                                                pParse,
                                                                                (*pUpsert).pUpsertTarget,
                                                                            ) != 0
                                                                            {
                                                                                c2rust_current_block = 14505076636074796664;
                                                                            } else {
                                                                                (*(&raw mut (*pTabList).a as *mut SrcItem)
                                                                                    .offset(0 as ::core::ffi::c_int as isize))
                                                                                    .iCursor = iDataCur;
                                                                                pNx = pUpsert;
                                                                                loop {
                                                                                    (*pNx).pUpsertSrc = pTabList;
                                                                                    (*pNx).regData = regData;
                                                                                    (*pNx).iDataCur = iDataCur;
                                                                                    (*pNx).iIdxCur = iIdxCur;
                                                                                    if !(*pNx).pUpsertTarget.is_null() {
                                                                                        if sqlite3UpsertAnalyzeTarget(
                                                                                            pParse,
                                                                                            pTabList,
                                                                                            pNx,
                                                                                            pUpsert,
                                                                                        ) != 0
                                                                                        {
                                                                                            c2rust_current_block = 14505076636074796664;
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    pNx = (*pNx).pNextUpsert;
                                                                                    if pNx.is_null() {
                                                                                        c2rust_current_block = 919396821984190499;
                                                                                        break;
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            c2rust_current_block = 919396821984190499;
                                                                        }
                                                                        match c2rust_current_block {
                                                                            14505076636074796664 => {}
                                                                            _ => {
                                                                                if useTempTable != 0 {
                                                                                    addrInsTop = sqlite3VdbeAddOp1(v, OP_Rewind, srcTab);
                                                                                    addrCont = sqlite3VdbeCurrentAddr(v);
                                                                                } else if !pSelect.is_null() {
                                                                                    addrCont = sqlite3VdbeAddOp1(v, OP_Yield, dest.iSDParm);
                                                                                    addrInsTop = addrCont;
                                                                                    if ipkColumn >= 0 as ::core::ffi::c_int {
                                                                                        sqlite3VdbeAddOp2(
                                                                                            v,
                                                                                            OP_Copy,
                                                                                            regFromSelect + ipkColumn,
                                                                                            regRowid,
                                                                                        );
                                                                                    }
                                                                                }
                                                                                nHidden = 0 as ::core::ffi::c_int;
                                                                                iRegStore = regData;
                                                                                let mut c2rust_current_block_192: u64;
                                                                                i = 0 as ::core::ffi::c_int;
                                                                                while i < (*pTab).nCol as ::core::ffi::c_int {
                                                                                    let mut k: ::core::ffi::c_int = 0;
                                                                                    let mut colFlags: u32_0 = 0;
                                                                                    if i == (*pTab).iPKey as ::core::ffi::c_int {
                                                                                        sqlite3VdbeAddOp1(v, OP_SoftNull, iRegStore);
                                                                                    } else {
                                                                                        colFlags = (*(*pTab).aCol.offset(i as isize)).colFlags
                                                                                            as u32_0;
                                                                                        if colFlags & COLFLAG_NOINSERT as u32_0 != 0 as u32_0 {
                                                                                            nHidden += 1;
                                                                                            if colFlags & COLFLAG_VIRTUAL as u32_0 != 0 as u32_0 {
                                                                                                iRegStore -= 1;
                                                                                                c2rust_current_block_192 = 654039154479240366;
                                                                                            } else if colFlags & COLFLAG_STORED as u32_0 != 0 as u32_0 {
                                                                                                if tmask & TRIGGER_BEFORE != 0 {
                                                                                                    sqlite3VdbeAddOp1(v, OP_SoftNull, iRegStore);
                                                                                                }
                                                                                                c2rust_current_block_192 = 654039154479240366;
                                                                                            } else if pColumn.is_null() {
                                                                                                sqlite3ExprCodeFactorable(
                                                                                                    pParse,
                                                                                                    sqlite3ColumnExpr(
                                                                                                        pTab,
                                                                                                        (*pTab).aCol.offset(i as isize) as *mut Column,
                                                                                                    ),
                                                                                                    iRegStore,
                                                                                                );
                                                                                                c2rust_current_block_192 = 654039154479240366;
                                                                                            } else {
                                                                                                c2rust_current_block_192 = 11202235766349324107;
                                                                                            }
                                                                                        } else {
                                                                                            c2rust_current_block_192 = 11202235766349324107;
                                                                                        }
                                                                                        match c2rust_current_block_192 {
                                                                                            654039154479240366 => {}
                                                                                            _ => {
                                                                                                if !pColumn.is_null() {
                                                                                                    j = *aTabColMap.offset(i as isize);
                                                                                                    if j == 0 as ::core::ffi::c_int {
                                                                                                        sqlite3ExprCodeFactorable(
                                                                                                            pParse,
                                                                                                            sqlite3ColumnExpr(
                                                                                                                pTab,
                                                                                                                (*pTab).aCol.offset(i as isize) as *mut Column,
                                                                                                            ),
                                                                                                            iRegStore,
                                                                                                        );
                                                                                                        c2rust_current_block_192 = 654039154479240366;
                                                                                                    } else {
                                                                                                        k = j - 1 as ::core::ffi::c_int;
                                                                                                        c2rust_current_block_192 = 12153365054289215322;
                                                                                                    }
                                                                                                } else if nColumn == 0 as ::core::ffi::c_int {
                                                                                                    sqlite3ExprCodeFactorable(
                                                                                                        pParse,
                                                                                                        sqlite3ColumnExpr(
                                                                                                            pTab,
                                                                                                            (*pTab).aCol.offset(i as isize) as *mut Column,
                                                                                                        ),
                                                                                                        iRegStore,
                                                                                                    );
                                                                                                    c2rust_current_block_192 = 654039154479240366;
                                                                                                } else {
                                                                                                    k = i - nHidden;
                                                                                                    c2rust_current_block_192 = 12153365054289215322;
                                                                                                }
                                                                                                match c2rust_current_block_192 {
                                                                                                    654039154479240366 => {}
                                                                                                    _ => {
                                                                                                        if useTempTable != 0 {
                                                                                                            sqlite3VdbeAddOp3(v, OP_Column, srcTab, k, iRegStore);
                                                                                                        } else if !pSelect.is_null() {
                                                                                                            if regFromSelect != regData {
                                                                                                                sqlite3VdbeAddOp2(
                                                                                                                    v,
                                                                                                                    OP_SCopy,
                                                                                                                    regFromSelect + k,
                                                                                                                    iRegStore,
                                                                                                                );
                                                                                                            }
                                                                                                        } else {
                                                                                                            let mut pX: *mut Expr = (*(&raw mut (*pList).a
                                                                                                                as *mut ExprList_item)
                                                                                                                .offset(k as isize))
                                                                                                                .pExpr;
                                                                                                            let mut y: ::core::ffi::c_int = sqlite3ExprCodeTarget(
                                                                                                                pParse,
                                                                                                                pX,
                                                                                                                iRegStore,
                                                                                                            );
                                                                                                            if y != iRegStore {
                                                                                                                sqlite3VdbeAddOp2(
                                                                                                                    v,
                                                                                                                    if (*pX).flags & 0x400000 as ::core::ffi::c_int as u32_0
                                                                                                                        != 0 as u32_0
                                                                                                                    {
                                                                                                                        OP_Copy
                                                                                                                    } else {
                                                                                                                        OP_SCopy
                                                                                                                    },
                                                                                                                    y,
                                                                                                                    iRegStore,
                                                                                                                );
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    i += 1;
                                                                                    iRegStore += 1;
                                                                                }
                                                                                endOfLoop = sqlite3VdbeMakeLabel(pParse);
                                                                                if tmask & TRIGGER_BEFORE != 0 {
                                                                                    let mut regCols: ::core::ffi::c_int = sqlite3GetTempRange(
                                                                                        pParse,
                                                                                        (*pTab).nCol as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                    );
                                                                                    if ipkColumn < 0 as ::core::ffi::c_int {
                                                                                        sqlite3VdbeAddOp2(
                                                                                            v,
                                                                                            OP_Integer,
                                                                                            -1 as ::core::ffi::c_int,
                                                                                            regCols,
                                                                                        );
                                                                                    } else {
                                                                                        let mut addr1: ::core::ffi::c_int = 0;
                                                                                        if useTempTable != 0 {
                                                                                            sqlite3VdbeAddOp3(v, OP_Column, srcTab, ipkColumn, regCols);
                                                                                        } else {
                                                                                            sqlite3ExprCode(
                                                                                                pParse,
                                                                                                (*(&raw mut (*pList).a as *mut ExprList_item)
                                                                                                    .offset(ipkColumn as isize))
                                                                                                    .pExpr,
                                                                                                regCols,
                                                                                            );
                                                                                        }
                                                                                        addr1 = sqlite3VdbeAddOp1(v, OP_NotNull, regCols);
                                                                                        sqlite3VdbeAddOp2(
                                                                                            v,
                                                                                            OP_Integer,
                                                                                            -1 as ::core::ffi::c_int,
                                                                                            regCols,
                                                                                        );
                                                                                        sqlite3VdbeJumpHere(v, addr1);
                                                                                        sqlite3VdbeAddOp1(v, OP_MustBeInt, regCols);
                                                                                    }
                                                                                    sqlite3VdbeAddOp3(
                                                                                        v,
                                                                                        OP_Copy,
                                                                                        regRowid + 1 as ::core::ffi::c_int,
                                                                                        regCols + 1 as ::core::ffi::c_int,
                                                                                        (*pTab).nNVCol as ::core::ffi::c_int
                                                                                            - 1 as ::core::ffi::c_int,
                                                                                    );
                                                                                    if (*pTab).tabFlags & TF_HasGenerated as u32_0 != 0 {
                                                                                        sqlite3ComputeGeneratedColumns(
                                                                                            pParse,
                                                                                            regCols + 1 as ::core::ffi::c_int,
                                                                                            pTab,
                                                                                        );
                                                                                    }
                                                                                    if isView == 0 {
                                                                                        sqlite3TableAffinity(
                                                                                            v,
                                                                                            pTab,
                                                                                            regCols + 1 as ::core::ffi::c_int,
                                                                                        );
                                                                                    }
                                                                                    sqlite3CodeRowTrigger(
                                                                                        pParse,
                                                                                        pTrigger,
                                                                                        TK_INSERT,
                                                                                        ::core::ptr::null_mut::<ExprList>(),
                                                                                        TRIGGER_BEFORE,
                                                                                        pTab,
                                                                                        regCols - (*pTab).nCol as ::core::ffi::c_int
                                                                                            - 1 as ::core::ffi::c_int,
                                                                                        onError,
                                                                                        endOfLoop,
                                                                                    );
                                                                                    sqlite3ReleaseTempRange(
                                                                                        pParse,
                                                                                        regCols,
                                                                                        (*pTab).nCol as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                    );
                                                                                }
                                                                                if isView == 0 {
                                                                                    if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                                                                                        sqlite3VdbeAddOp2(
                                                                                            v,
                                                                                            OP_Null,
                                                                                            0 as ::core::ffi::c_int,
                                                                                            regIns,
                                                                                        );
                                                                                    }
                                                                                    if ipkColumn >= 0 as ::core::ffi::c_int {
                                                                                        if useTempTable != 0 {
                                                                                            sqlite3VdbeAddOp3(
                                                                                                v,
                                                                                                OP_Column,
                                                                                                srcTab,
                                                                                                ipkColumn,
                                                                                                regRowid,
                                                                                            );
                                                                                        } else if pSelect.is_null() {
                                                                                            let mut pIpk: *mut Expr = (*(&raw mut (*pList).a
                                                                                                as *mut ExprList_item)
                                                                                                .offset(ipkColumn as isize))
                                                                                                .pExpr;
                                                                                            if (*pIpk).op as ::core::ffi::c_int == TK_NULL
                                                                                                && !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB)
                                                                                            {
                                                                                                sqlite3VdbeAddOp3(
                                                                                                    v,
                                                                                                    OP_NewRowid,
                                                                                                    iDataCur,
                                                                                                    regRowid,
                                                                                                    regAutoinc,
                                                                                                );
                                                                                                appendFlag = 1 as u8_0;
                                                                                            } else {
                                                                                                sqlite3ExprCode(
                                                                                                    pParse,
                                                                                                    (*(&raw mut (*pList).a as *mut ExprList_item)
                                                                                                        .offset(ipkColumn as isize))
                                                                                                        .pExpr,
                                                                                                    regRowid,
                                                                                                );
                                                                                            }
                                                                                        }
                                                                                        if appendFlag == 0 {
                                                                                            let mut addr1_0: ::core::ffi::c_int = 0;
                                                                                            if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB)
                                                                                            {
                                                                                                addr1_0 = sqlite3VdbeAddOp1(v, OP_NotNull, regRowid);
                                                                                                sqlite3VdbeAddOp3(
                                                                                                    v,
                                                                                                    OP_NewRowid,
                                                                                                    iDataCur,
                                                                                                    regRowid,
                                                                                                    regAutoinc,
                                                                                                );
                                                                                                sqlite3VdbeJumpHere(v, addr1_0);
                                                                                            } else {
                                                                                                addr1_0 = sqlite3VdbeCurrentAddr(v);
                                                                                                sqlite3VdbeAddOp2(
                                                                                                    v,
                                                                                                    OP_IsNull,
                                                                                                    regRowid,
                                                                                                    addr1_0 + 2 as ::core::ffi::c_int,
                                                                                                );
                                                                                            }
                                                                                            sqlite3VdbeAddOp1(v, OP_MustBeInt, regRowid);
                                                                                        }
                                                                                    } else if (*pTab).eTabType as ::core::ffi::c_int
                                                                                        == TABTYP_VTAB || withoutRowid as ::core::ffi::c_int != 0
                                                                                    {
                                                                                        sqlite3VdbeAddOp2(
                                                                                            v,
                                                                                            OP_Null,
                                                                                            0 as ::core::ffi::c_int,
                                                                                            regRowid,
                                                                                        );
                                                                                    } else {
                                                                                        sqlite3VdbeAddOp3(
                                                                                            v,
                                                                                            OP_NewRowid,
                                                                                            iDataCur,
                                                                                            regRowid,
                                                                                            regAutoinc,
                                                                                        );
                                                                                        appendFlag = 1 as u8_0;
                                                                                    }
                                                                                    autoIncStep(pParse, regAutoinc, regRowid);
                                                                                    if (*pTab).tabFlags & TF_HasGenerated as u32_0 != 0 {
                                                                                        sqlite3ComputeGeneratedColumns(
                                                                                            pParse,
                                                                                            regRowid + 1 as ::core::ffi::c_int,
                                                                                            pTab,
                                                                                        );
                                                                                    }
                                                                                    if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                                                                                        let mut pVTab: *const ::core::ffi::c_char = sqlite3GetVTable(
                                                                                            db,
                                                                                            pTab,
                                                                                        ) as *const ::core::ffi::c_char;
                                                                                        sqlite3VtabMakeWritable(pParse, pTab);
                                                                                        sqlite3VdbeAddOp4(
                                                                                            v,
                                                                                            OP_VUpdate,
                                                                                            1 as ::core::ffi::c_int,
                                                                                            (*pTab).nCol as ::core::ffi::c_int
                                                                                                + 2 as ::core::ffi::c_int,
                                                                                            regIns,
                                                                                            pVTab,
                                                                                            P4_VTAB,
                                                                                        );
                                                                                        sqlite3VdbeChangeP5(
                                                                                            v,
                                                                                            (if onError == OE_Default { OE_Abort } else { onError })
                                                                                                as u16_0,
                                                                                        );
                                                                                        sqlite3MayAbort(pParse);
                                                                                    } else {
                                                                                        let mut isReplace: ::core::ffi::c_int = 0
                                                                                            as ::core::ffi::c_int;
                                                                                        let mut bUseSeek: ::core::ffi::c_int = 0;
                                                                                        sqlite3GenerateConstraintChecks(
                                                                                            pParse,
                                                                                            pTab,
                                                                                            aRegIdx,
                                                                                            iDataCur,
                                                                                            iIdxCur,
                                                                                            regIns,
                                                                                            0 as ::core::ffi::c_int,
                                                                                            (ipkColumn >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                                                                                                as u8_0,
                                                                                            onError as u8_0,
                                                                                            endOfLoop,
                                                                                            &raw mut isReplace,
                                                                                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                                                                            pUpsert,
                                                                                        );
                                                                                        if (*db).flags as ::core::ffi::c_ulonglong
                                                                                            & SQLITE_ForeignKeys as ::core::ffi::c_ulonglong != 0
                                                                                        {
                                                                                            sqlite3FkCheck(
                                                                                                pParse,
                                                                                                pTab,
                                                                                                0 as ::core::ffi::c_int,
                                                                                                regIns,
                                                                                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                                                                                0 as ::core::ffi::c_int,
                                                                                            );
                                                                                        }
                                                                                        bUseSeek = (isReplace == 0 as ::core::ffi::c_int
                                                                                            || sqlite3VdbeHasSubProgram(v) == 0) as ::core::ffi::c_int;
                                                                                        sqlite3CompleteInsertion(
                                                                                            pParse,
                                                                                            pTab,
                                                                                            iDataCur,
                                                                                            iIdxCur,
                                                                                            regIns,
                                                                                            aRegIdx,
                                                                                            0 as ::core::ffi::c_int,
                                                                                            appendFlag as ::core::ffi::c_int,
                                                                                            bUseSeek,
                                                                                        );
                                                                                    }
                                                                                }
                                                                                if regRowCount != 0 {
                                                                                    sqlite3VdbeAddOp2(
                                                                                        v,
                                                                                        OP_AddImm,
                                                                                        regRowCount,
                                                                                        1 as ::core::ffi::c_int,
                                                                                    );
                                                                                }
                                                                                if !pTrigger.is_null() {
                                                                                    sqlite3CodeRowTrigger(
                                                                                        pParse,
                                                                                        pTrigger,
                                                                                        TK_INSERT,
                                                                                        ::core::ptr::null_mut::<ExprList>(),
                                                                                        TRIGGER_AFTER,
                                                                                        pTab,
                                                                                        regData - 2 as ::core::ffi::c_int
                                                                                            - (*pTab).nCol as ::core::ffi::c_int,
                                                                                        onError,
                                                                                        endOfLoop,
                                                                                    );
                                                                                }
                                                                                sqlite3VdbeResolveLabel(v, endOfLoop);
                                                                                if useTempTable != 0 {
                                                                                    sqlite3VdbeAddOp2(v, OP_Next, srcTab, addrCont);
                                                                                    sqlite3VdbeJumpHere(v, addrInsTop);
                                                                                    sqlite3VdbeAddOp1(v, OP_Close, srcTab);
                                                                                } else if !pSelect.is_null() {
                                                                                    sqlite3VdbeGoto(v, addrCont);
                                                                                    sqlite3VdbeJumpHere(v, addrInsTop);
                                                                                }
                                                                                c2rust_current_block = 16497991052681986260;
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
                                    }
                                }
                                match c2rust_current_block {
                                    14505076636074796664 => {}
                                    _ => {
                                        if (*pParse).nested as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                            && (*pParse).pTriggerTab.is_null()
                                        {
                                            sqlite3AutoincrementEnd(pParse);
                                        }
                                        if regRowCount != 0 {
                                            sqlite3CodeChangeCount(
                                                v,
                                                regRowCount,
                                                b"rows inserted\0".as_ptr() as *const ::core::ffi::c_char,
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
        sqlite3SrcListDelete(db, pTabList);
        sqlite3ExprListDelete(db, pList);
        sqlite3UpsertDelete(db, pUpsert);
        sqlite3SelectDelete(db, pSelect);
        if !pColumn.is_null() {
            sqlite3IdListDelete(db, pColumn);
            sqlite3DbFree(db, aTabColMap as *mut ::core::ffi::c_void);
        }
        if !aRegIdx.is_null() {
            sqlite3DbNNFreeNN(db, aRegIdx as *mut ::core::ffi::c_void);
        }
    }
}
pub const CKCNSTRNT_COLUMN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const CKCNSTRNT_ROWID: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
unsafe extern "C" fn checkConstraintExprNode(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int == TK_COLUMN {
            if (*pExpr).iColumn as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                if *(*pWalker).u.aiCol.offset((*pExpr).iColumn as isize)
                    >= 0 as ::core::ffi::c_int
                {
                    (*pWalker).eCode = ((*pWalker).eCode as ::core::ffi::c_int
                        | CKCNSTRNT_COLUMN) as u16_0;
                }
            } else {
                (*pWalker).eCode = ((*pWalker).eCode as ::core::ffi::c_int
                    | CKCNSTRNT_ROWID) as u16_0;
            }
        }
        return WRC_Continue;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprReferencesUpdatedColumn(
    mut pExpr: *mut Expr,
    mut aiChng: *mut ::core::ffi::c_int,
    mut chngRowid: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
        memset(
            &raw mut w as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        w.eCode = 0 as u16_0;
        w.xExprCallback = Some(
            checkConstraintExprNode
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.u.aiCol = aiChng;
        sqlite3WalkExpr(&raw mut w, pExpr);
        if chngRowid == 0 {
            w.eCode = (w.eCode as ::core::ffi::c_int & !CKCNSTRNT_ROWID) as u16_0;
        }
        return (w.eCode as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn indexIteratorFirst(
    mut pIter: *mut IndexIterator,
    mut pIx: *mut ::core::ffi::c_int,
) -> *mut Index {
    unsafe {
        if (*pIter).eType != 0 {
            *pIx = (*(*pIter).u.ax.aIdx.offset(0 as ::core::ffi::c_int as isize)).ix;
            return (*(*pIter).u.ax.aIdx.offset(0 as ::core::ffi::c_int as isize)).p;
        } else {
            *pIx = 0 as ::core::ffi::c_int;
            return (*pIter).u.lx.pIdx;
        };
    }
}
unsafe extern "C" fn indexIteratorNext(
    mut pIter: *mut IndexIterator,
    mut pIx: *mut ::core::ffi::c_int,
) -> *mut Index {
    unsafe {
        if (*pIter).eType != 0 {
            (*pIter).i += 1;
            let mut i: ::core::ffi::c_int = (*pIter).i;
            if i >= (*pIter).u.ax.nIdx {
                *pIx = i;
                return ::core::ptr::null_mut::<Index>();
            }
            *pIx = (*(*pIter).u.ax.aIdx.offset(i as isize)).ix;
            return (*(*pIter).u.ax.aIdx.offset(i as isize)).p;
        } else {
            *pIx += 1;
            (*pIter).u.lx.pIdx = (*(*pIter).u.lx.pIdx).pNext;
            return (*pIter).u.lx.pIdx;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GenerateConstraintChecks(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut aRegIdx: *mut ::core::ffi::c_int,
    mut iDataCur: ::core::ffi::c_int,
    mut iIdxCur: ::core::ffi::c_int,
    mut regNewData: ::core::ffi::c_int,
    mut regOldData: ::core::ffi::c_int,
    mut pkChng: u8_0,
    mut overrideError: u8_0,
    mut ignoreDest: ::core::ffi::c_int,
    mut pbMayReplace: *mut ::core::ffi::c_int,
    mut aiChng: *mut ::core::ffi::c_int,
    mut pUpsert: *mut Upsert,
) {
    unsafe {
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut pPk: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut i: ::core::ffi::c_int = 0;
        let mut ix: ::core::ffi::c_int = 0;
        let mut nCol: ::core::ffi::c_int = 0;
        let mut onError: ::core::ffi::c_int = 0;
        let mut seenReplace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nPkField: ::core::ffi::c_int = 0;
        let mut pUpsertClause: *mut Upsert = ::core::ptr::null_mut::<Upsert>();
        let mut isUpdate: u8_0 = 0;
        let mut bAffinityDone: u8_0 = 0 as u8_0;
        let mut upsertIpkReturn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut upsertIpkDelay: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ipkTop: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ipkBottom: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regTrigCnt: ::core::ffi::c_int = 0;
        let mut addrRecheck: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut lblRecheckOk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pTrigger: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        let mut nReplaceTrig: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut sIdxIter: IndexIterator = IndexIterator {
            eType: 0,
            i: 0,
            u: C2Rust_Unnamed_24 {
                lx: C2Rust_Unnamed_26 {
                    pIdx: ::core::ptr::null_mut::<Index>()

                }

            }

        };
        isUpdate = (regOldData != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as u8_0;
        db = (*pParse).db;
        v = (*pParse).pVdbe;
        nCol = (*pTab).nCol as ::core::ffi::c_int;
        if (*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0 {
            pPk = ::core::ptr::null_mut::<Index>();
            nPkField = 1 as ::core::ffi::c_int;
        } else {
            pPk = sqlite3PrimaryKeyIndex(pTab);
            nPkField = (*pPk).nKeyCol as ::core::ffi::c_int;
        }
        if (*pTab).tabFlags & TF_HasNotNull as u32_0 != 0 {
            let mut b2ndPass: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut nSeenReplace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut nGenerated: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            loop {
                let mut c2rust_current_block_51: u64;
                i = 0 as ::core::ffi::c_int;
                while i < nCol {
                    let mut iReg: ::core::ffi::c_int = 0;
                    let mut pCol: *mut Column = (*pTab).aCol.offset(i as isize)
                        as *mut Column;
                    let mut isGenerated: ::core::ffi::c_int = 0;
                    onError = (*pCol).notNull() as ::core::ffi::c_int;
                    if !(onError == OE_None) {
                        if !(i == (*pTab).iPKey as ::core::ffi::c_int) {
                            isGenerated = (*pCol).colFlags as ::core::ffi::c_int
                                & COLFLAG_GENERATED;
                            if isGenerated != 0 && b2ndPass == 0 {
                                nGenerated += 1;
                            } else if !(!aiChng.is_null()
                                && *aiChng.offset(i as isize) < 0 as ::core::ffi::c_int
                                && isGenerated == 0)
                            {
                                if overrideError as ::core::ffi::c_int != OE_Default {
                                    onError = overrideError as ::core::ffi::c_int;
                                } else if onError == OE_Default {
                                    onError = OE_Abort;
                                }
                                if onError == OE_Replace {
                                    if b2ndPass != 0
                                        || (*pCol).iDflt as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                    {
                                        onError = OE_Abort;
                                    }
                                    c2rust_current_block_51 = 2122094917359643297;
                                } else if b2ndPass != 0 && isGenerated == 0 {
                                    c2rust_current_block_51 = 5783071609795492627;
                                } else {
                                    c2rust_current_block_51 = 2122094917359643297;
                                }
                                match c2rust_current_block_51 {
                                    5783071609795492627 => {}
                                    _ => {
                                        iReg = sqlite3TableColumnToStorage(pTab, i as i16_0)
                                            as ::core::ffi::c_int + regNewData
                                            + 1 as ::core::ffi::c_int;
                                        let mut c2rust_current_block_50: u64;
                                        match onError {
                                            OE_Replace => {
                                                let mut addr1: ::core::ffi::c_int = sqlite3VdbeAddOp1(
                                                    v,
                                                    OP_NotNull,
                                                    iReg,
                                                );
                                                nSeenReplace += 1;
                                                sqlite3ExprCodeCopy(
                                                    pParse,
                                                    sqlite3ColumnExpr(pTab, pCol),
                                                    iReg,
                                                );
                                                sqlite3VdbeJumpHere(v, addr1);
                                                c2rust_current_block_50 = 7252614138838059896;
                                            }
                                            OE_Abort => {
                                                sqlite3MayAbort(pParse);
                                                c2rust_current_block_50 = 2290177392965769716;
                                            }
                                            OE_Rollback | OE_Fail => {
                                                c2rust_current_block_50 = 2290177392965769716;
                                            }
                                            _ => {
                                                sqlite3VdbeAddOp2(v, OP_IsNull, iReg, ignoreDest);
                                                c2rust_current_block_50 = 7252614138838059896;
                                            }
                                        }
                                        match c2rust_current_block_50 {
                                            2290177392965769716 => {
                                                let mut zMsg: *mut ::core::ffi::c_char = sqlite3MPrintf(
                                                    db,
                                                    b"%s.%s\0".as_ptr() as *const ::core::ffi::c_char,
                                                    (*pTab).zName,
                                                    (*pCol).zCnName,
                                                );
                                                sqlite3VdbeAddOp3(
                                                    v,
                                                    OP_HaltIfNull,
                                                    SQLITE_CONSTRAINT_NOTNULL,
                                                    onError,
                                                    iReg,
                                                );
                                                sqlite3VdbeAppendP4(
                                                    v,
                                                    zMsg as *mut ::core::ffi::c_void,
                                                    P4_DYNAMIC,
                                                );
                                                sqlite3VdbeChangeP5(v, P5_ConstraintNotNull as u16_0);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                    i += 1;
                }
                if nGenerated == 0 as ::core::ffi::c_int
                    && nSeenReplace == 0 as ::core::ffi::c_int
                {
                    break;
                }
                if b2ndPass != 0 {
                    break;
                }
                b2ndPass = 1 as ::core::ffi::c_int;
                if nSeenReplace > 0 as ::core::ffi::c_int
                    && (*pTab).tabFlags & TF_HasGenerated as u32_0 != 0 as u32_0
                {
                    sqlite3ComputeGeneratedColumns(
                        pParse,
                        regNewData + 1 as ::core::ffi::c_int,
                        pTab,
                    );
                }
            }
        }
        if !(*pTab).pCheck.is_null()
            && (*db).flags as ::core::ffi::c_ulonglong
                & SQLITE_IgnoreChecks as ::core::ffi::c_ulonglong
                == 0 as ::core::ffi::c_ulonglong
        {
            let mut pCheck: *mut ExprList = (*pTab).pCheck;
            (*pParse).iSelfTab = -(regNewData + 1 as ::core::ffi::c_int);
            onError = if overrideError as ::core::ffi::c_int != OE_Default {
                overrideError as ::core::ffi::c_int
            } else {
                OE_Abort
            };
            i = 0 as ::core::ffi::c_int;
            while i < (*pCheck).nExpr {
                let mut allOk: ::core::ffi::c_int = 0;
                let mut pCopy: *mut Expr = ::core::ptr::null_mut::<Expr>();
                let mut pExpr: *mut Expr = (*(&raw mut (*pCheck).a as *mut ExprList_item)
                    .offset(i as isize))
                    .pExpr;
                if !(!aiChng.is_null()
                    && sqlite3ExprReferencesUpdatedColumn(
                        pExpr,
                        aiChng,
                        pkChng as ::core::ffi::c_int,
                    ) == 0)
                {
                    if bAffinityDone as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        sqlite3TableAffinity(
                            v,
                            pTab,
                            regNewData + 1 as ::core::ffi::c_int,
                        );
                        bAffinityDone = 1 as u8_0;
                    }
                    allOk = sqlite3VdbeMakeLabel(pParse);
                    pCopy = sqlite3ExprDup(db, pExpr, 0 as ::core::ffi::c_int);
                    if (*db).mallocFailed == 0 {
                        sqlite3ExprIfTrue(pParse, pCopy, allOk, SQLITE_JUMPIFNULL);
                    }
                    sqlite3ExprDelete(db, pCopy);
                    if onError == OE_Ignore {
                        sqlite3VdbeGoto(v, ignoreDest);
                    } else {
                        let mut zName: *mut ::core::ffi::c_char = (*(&raw mut (*pCheck).a
                            as *mut ExprList_item)
                            .offset(i as isize))
                            .zEName;
                        if onError == OE_Replace {
                            onError = OE_Abort;
                        }
                        sqlite3HaltConstraint(
                            pParse,
                            SQLITE_CONSTRAINT_CHECK,
                            onError,
                            zName,
                            P4_TRANSIENT as i8_0,
                            P5_ConstraintCheck as u8_0,
                        );
                    }
                    sqlite3VdbeResolveLabel(v, allOk);
                }
                i += 1;
            }
            (*pParse).iSelfTab = 0 as ::core::ffi::c_int;
        }
        sIdxIter.eType = 0 as ::core::ffi::c_int;
        sIdxIter.i = 0 as ::core::ffi::c_int;
        sIdxIter.u.ax.aIdx = ::core::ptr::null_mut::<IndexListTerm>();
        sIdxIter.u.lx.pIdx = (*pTab).pIndex;
        if !pUpsert.is_null() {
            if (*pUpsert).pUpsertTarget.is_null() {
                if (*pUpsert).isDoUpdate as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    overrideError = OE_Ignore as u8_0;
                    pUpsert = ::core::ptr::null_mut::<Upsert>();
                } else {
                    overrideError = OE_Update as u8_0;
                }
            } else if !(*pTab).pIndex.is_null() {
                let mut nIdx: ::core::ffi::c_int = 0;
                let mut jj: ::core::ffi::c_int = 0;
                let mut nByte: u64_0 = 0;
                let mut pTerm: *mut Upsert = ::core::ptr::null_mut::<Upsert>();
                let mut bUsed: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                nIdx = 0 as ::core::ffi::c_int;
                pIdx = (*pTab).pIndex;
                while !pIdx.is_null() {
                    pIdx = (*pIdx).pNext;
                    nIdx += 1;
                }
                sIdxIter.eType = 1 as ::core::ffi::c_int;
                sIdxIter.u.ax.nIdx = nIdx;
                nByte = (::core::mem::size_of::<IndexListTerm>() as usize)
                    .wrapping_add(1 as usize)
                    .wrapping_mul(nIdx as usize)
                    .wrapping_add(nIdx as usize) as u64_0;
                sIdxIter.u.ax.aIdx = sqlite3DbMallocZero(db, nByte)
                    as *mut IndexListTerm;
                if sIdxIter.u.ax.aIdx.is_null() {
                    return;
                }
                bUsed = sIdxIter.u.ax.aIdx.offset(nIdx as isize) as *mut IndexListTerm
                    as *mut u8_0;
                (*pUpsert).pToFree = sIdxIter.u.ax.aIdx as *mut ::core::ffi::c_void;
                i = 0 as ::core::ffi::c_int;
                pTerm = pUpsert;
                while !pTerm.is_null() {
                    if (*pTerm).pUpsertTarget.is_null() {
                        break;
                    }
                    if !(*pTerm).pUpsertIdx.is_null() {
                        jj = 0 as ::core::ffi::c_int;
                        pIdx = (*pTab).pIndex;
                        while !pIdx.is_null() && pIdx != (*pTerm).pUpsertIdx {
                            pIdx = (*pIdx).pNext;
                            jj += 1;
                        }
                        if !(*bUsed.offset(jj as isize) != 0) {
                            *bUsed.offset(jj as isize) = 1 as u8_0;
                            let ref mut c2rust_fresh1 = (*sIdxIter
                                .u
                                .ax
                                .aIdx
                                .offset(i as isize))
                                .p;
                            *c2rust_fresh1 = pIdx;
                            (*sIdxIter.u.ax.aIdx.offset(i as isize)).ix = jj;
                            i += 1;
                        }
                    }
                    pTerm = (*pTerm).pNextUpsert;
                }
                jj = 0 as ::core::ffi::c_int;
                pIdx = (*pTab).pIndex;
                while !pIdx.is_null() {
                    if !(*bUsed.offset(jj as isize) != 0) {
                        let ref mut c2rust_fresh2 = (*sIdxIter
                            .u
                            .ax
                            .aIdx
                            .offset(i as isize))
                            .p;
                        *c2rust_fresh2 = pIdx;
                        (*sIdxIter.u.ax.aIdx.offset(i as isize)).ix = jj;
                        i += 1;
                    }
                    pIdx = (*pIdx).pNext;
                    jj += 1;
                }
            }
        }
        if (*db).flags as ::core::ffi::c_ulonglong
            & (SQLITE_RecTriggers | SQLITE_ForeignKeys) as ::core::ffi::c_ulonglong
            == 0 as ::core::ffi::c_ulonglong
        {
            pTrigger = ::core::ptr::null_mut::<Trigger>();
            regTrigCnt = 0 as ::core::ffi::c_int;
        } else {
            if (*db).flags as ::core::ffi::c_ulonglong
                & SQLITE_RecTriggers as ::core::ffi::c_ulonglong != 0
            {
                pTrigger = sqlite3TriggersExist(
                    pParse,
                    pTab,
                    TK_DELETE,
                    ::core::ptr::null_mut::<ExprList>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                regTrigCnt = (!pTrigger.is_null()
                    || sqlite3FkRequired(
                        pParse,
                        pTab,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        0 as ::core::ffi::c_int,
                    ) != 0) as ::core::ffi::c_int;
            } else {
                pTrigger = ::core::ptr::null_mut::<Trigger>();
                regTrigCnt = sqlite3FkRequired(
                    pParse,
                    pTab,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    0 as ::core::ffi::c_int,
                );
            }
            if regTrigCnt != 0 {
                (*pParse).nMem += 1;
                regTrigCnt = (*pParse).nMem;
                sqlite3VdbeAddOp2(v, OP_Integer, 0 as ::core::ffi::c_int, regTrigCnt);
                lblRecheckOk = sqlite3VdbeMakeLabel(pParse);
                addrRecheck = lblRecheckOk;
            }
        }
        if pkChng as ::core::ffi::c_int != 0 && pPk.is_null() {
            let mut addrRowidOk: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
            onError = (*pTab).keyConf as ::core::ffi::c_int;
            if overrideError as ::core::ffi::c_int != OE_Default {
                onError = overrideError as ::core::ffi::c_int;
            } else if onError == OE_Default {
                onError = OE_Abort;
            }
            if !pUpsert.is_null() {
                pUpsertClause = sqlite3UpsertOfIndex(
                    pUpsert,
                    ::core::ptr::null_mut::<Index>(),
                );
                if !pUpsertClause.is_null() {
                    if (*pUpsertClause).isDoUpdate as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        onError = OE_Ignore;
                    } else {
                        onError = OE_Update;
                    }
                }
                if pUpsertClause != pUpsert {
                    upsertIpkDelay = sqlite3VdbeAddOp0(v, OP_Goto);
                }
            }
            if onError == OE_Replace && onError != overrideError as ::core::ffi::c_int
                && !(*pTab).pIndex.is_null() && upsertIpkDelay == 0
            {
                ipkTop = sqlite3VdbeAddOp0(v, OP_Goto) + 1 as ::core::ffi::c_int;
            }
            if isUpdate != 0 {
                sqlite3VdbeAddOp3(v, OP_Eq, regNewData, addrRowidOk, regOldData);
                sqlite3VdbeChangeP5(v, SQLITE_NOTNULL as u16_0);
            }
            sqlite3VdbeAddOp3(v, OP_NotExists, iDataCur, addrRowidOk, regNewData);
            let mut c2rust_current_block_212: u64;
            match onError {
                OE_Rollback | OE_Abort | OE_Fail => {
                    c2rust_current_block_212 = 12045739402850935335;
                }
                OE_Replace => {
                    if regTrigCnt != 0 {
                        sqlite3MultiWrite(pParse);
                        sqlite3GenerateRowDelete(
                            pParse,
                            pTab,
                            pTrigger,
                            iDataCur,
                            iIdxCur,
                            regNewData,
                            1 as i16_0,
                            0 as u8_0,
                            OE_Replace as u8_0,
                            1 as u8_0,
                            -1 as ::core::ffi::c_int,
                        );
                        sqlite3VdbeAddOp2(
                            v,
                            OP_AddImm,
                            regTrigCnt,
                            1 as ::core::ffi::c_int,
                        );
                        nReplaceTrig += 1;
                    } else {
                        sqlite3VdbeAddOp2(v, OP_Delete, iDataCur, OPFLAG_ISNOOP);
                        sqlite3VdbeAppendP4(
                            v,
                            pTab as *mut ::core::ffi::c_void,
                            P4_TABLE,
                        );
                        if !(*pTab).pIndex.is_null() {
                            sqlite3MultiWrite(pParse);
                            sqlite3GenerateRowIndexDelete(
                                pParse,
                                pTab,
                                iDataCur,
                                iIdxCur,
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                -1 as ::core::ffi::c_int,
                            );
                        }
                    }
                    seenReplace = 1 as ::core::ffi::c_int;
                    c2rust_current_block_212 = 8147588656546899898;
                }
                OE_Update => {
                    sqlite3UpsertDoUpdate(
                        pParse,
                        pUpsert,
                        pTab,
                        ::core::ptr::null_mut::<Index>(),
                        iDataCur,
                    );
                    c2rust_current_block_212 = 12153365054289215322;
                }
                OE_Ignore => {
                    c2rust_current_block_212 = 12153365054289215322;
                }
                _ => {
                    onError = OE_Abort;
                    c2rust_current_block_212 = 12045739402850935335;
                }
            }
            match c2rust_current_block_212 {
                12045739402850935335 => {
                    sqlite3RowidConstraint(pParse, onError, pTab);
                }
                12153365054289215322 => {
                    sqlite3VdbeGoto(v, ignoreDest);
                }
                _ => {}
            }
            sqlite3VdbeResolveLabel(v, addrRowidOk);
            if !pUpsert.is_null() && pUpsertClause != pUpsert {
                upsertIpkReturn = sqlite3VdbeAddOp0(v, OP_Goto);
            } else if ipkTop != 0 {
                ipkBottom = sqlite3VdbeAddOp0(v, OP_Goto);
                sqlite3VdbeJumpHere(v, ipkTop - 1 as ::core::ffi::c_int);
            }
        }
        pIdx = indexIteratorFirst(&raw mut sIdxIter, &raw mut ix);
        while !pIdx.is_null() {
            let mut regIdx: ::core::ffi::c_int = 0;
            let mut regR: ::core::ffi::c_int = 0;
            let mut iThisCur: ::core::ffi::c_int = 0;
            let mut addrUniqueOk: ::core::ffi::c_int = 0;
            let mut addrConflictCk: ::core::ffi::c_int = 0;
            if !(*aRegIdx.offset(ix as isize) == 0 as ::core::ffi::c_int) {
                if !pUpsert.is_null() {
                    pUpsertClause = sqlite3UpsertOfIndex(pUpsert, pIdx);
                    if upsertIpkDelay != 0 && pUpsertClause == pUpsert {
                        sqlite3VdbeJumpHere(v, upsertIpkDelay);
                    }
                }
                addrUniqueOk = sqlite3VdbeMakeLabel(pParse);
                if bAffinityDone as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    sqlite3TableAffinity(v, pTab, regNewData + 1 as ::core::ffi::c_int);
                    bAffinityDone = 1 as u8_0;
                }
                iThisCur = iIdxCur + ix;
                if !(*pIdx).pPartIdxWhere.is_null() {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Null,
                        0 as ::core::ffi::c_int,
                        *aRegIdx.offset(ix as isize),
                    );
                    (*pParse).iSelfTab = -(regNewData + 1 as ::core::ffi::c_int);
                    sqlite3ExprIfFalseDup(
                        pParse,
                        (*pIdx).pPartIdxWhere,
                        addrUniqueOk,
                        SQLITE_JUMPIFNULL,
                    );
                    (*pParse).iSelfTab = 0 as ::core::ffi::c_int;
                }
                regIdx = *aRegIdx.offset(ix as isize) + 1 as ::core::ffi::c_int;
                i = 0 as ::core::ffi::c_int;
                while i < (*pIdx).nColumn as ::core::ffi::c_int {
                    let mut iField: ::core::ffi::c_int = *(*pIdx)
                        .aiColumn
                        .offset(i as isize) as ::core::ffi::c_int;
                    let mut x: ::core::ffi::c_int = 0;
                    if iField == XN_EXPR {
                        (*pParse).iSelfTab = -(regNewData + 1 as ::core::ffi::c_int);
                        sqlite3ExprCodeCopy(
                            pParse,
                            (*(&raw mut (*(*pIdx).aColExpr).a as *mut ExprList_item)
                                .offset(i as isize))
                                .pExpr,
                            regIdx + i,
                        );
                        (*pParse).iSelfTab = 0 as ::core::ffi::c_int;
                    } else if iField == XN_ROWID
                        || iField == (*pTab).iPKey as ::core::ffi::c_int
                    {
                        x = regNewData;
                        sqlite3VdbeAddOp2(v, OP_IntCopy, x, regIdx + i);
                    } else {
                        x = sqlite3TableColumnToStorage(pTab, iField as i16_0)
                            as ::core::ffi::c_int + regNewData + 1 as ::core::ffi::c_int;
                        sqlite3VdbeAddOp2(v, OP_SCopy, x, regIdx + i);
                    }
                    i += 1;
                }
                sqlite3VdbeAddOp3(
                    v,
                    OP_MakeRecord,
                    regIdx,
                    (*pIdx).nColumn as ::core::ffi::c_int,
                    *aRegIdx.offset(ix as isize),
                );
                if isUpdate as ::core::ffi::c_int != 0 && pPk == pIdx
                    && pkChng as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    sqlite3VdbeResolveLabel(v, addrUniqueOk);
                } else {
                    onError = (*pIdx).onError as ::core::ffi::c_int;
                    if onError == OE_None {
                        sqlite3VdbeResolveLabel(v, addrUniqueOk);
                    } else {
                        if overrideError as ::core::ffi::c_int != OE_Default {
                            onError = overrideError as ::core::ffi::c_int;
                        } else if onError == OE_Default {
                            onError = OE_Abort;
                        }
                        if !pUpsertClause.is_null() {
                            if (*pUpsertClause).isDoUpdate as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                onError = OE_Ignore;
                            } else {
                                onError = OE_Update;
                            }
                        }
                        addrConflictCk = sqlite3VdbeAddOp4Int(
                            v,
                            OP_NoConflict,
                            iThisCur,
                            addrUniqueOk,
                            regIdx,
                            (*pIdx).nKeyCol as ::core::ffi::c_int,
                        );
                        regR = if pIdx == pPk {
                            regIdx
                        } else {
                            sqlite3GetTempRange(pParse, nPkField)
                        };
                        if isUpdate as ::core::ffi::c_int != 0 || onError == OE_Replace {
                            if (*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0
                            {
                                sqlite3VdbeAddOp2(v, OP_IdxRowid, iThisCur, regR);
                                if isUpdate != 0 {
                                    sqlite3VdbeAddOp3(v, OP_Eq, regR, addrUniqueOk, regOldData);
                                    sqlite3VdbeChangeP5(v, SQLITE_NOTNULL as u16_0);
                                }
                            } else {
                                let mut x_0: ::core::ffi::c_int = 0;
                                if pIdx != pPk {
                                    i = 0 as ::core::ffi::c_int;
                                    while i < (*pPk).nKeyCol as ::core::ffi::c_int {
                                        x_0 = sqlite3TableColumnToIndex(
                                            pIdx,
                                            *(*pPk).aiColumn.offset(i as isize) as ::core::ffi::c_int,
                                        );
                                        sqlite3VdbeAddOp3(v, OP_Column, iThisCur, x_0, regR + i);
                                        i += 1;
                                    }
                                }
                                if isUpdate != 0 {
                                    let mut addrJump: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(
                                        v,
                                    ) + (*pPk).nKeyCol as ::core::ffi::c_int;
                                    let mut op: ::core::ffi::c_int = OP_Ne;
                                    let mut regCmp: ::core::ffi::c_int = if (*pIdx).idxType()
                                        as ::core::ffi::c_int == SQLITE_IDXTYPE_PRIMARYKEY
                                    {
                                        regIdx
                                    } else {
                                        regR
                                    };
                                    i = 0 as ::core::ffi::c_int;
                                    while i < (*pPk).nKeyCol as ::core::ffi::c_int {
                                        let mut p4: *mut ::core::ffi::c_char = sqlite3LocateCollSeq(
                                            pParse,
                                            *(*pPk).azColl.offset(i as isize),
                                        ) as *mut ::core::ffi::c_char;
                                        x_0 = *(*pPk).aiColumn.offset(i as isize)
                                            as ::core::ffi::c_int;
                                        if i
                                            == (*pPk).nKeyCol as ::core::ffi::c_int
                                                - 1 as ::core::ffi::c_int
                                        {
                                            addrJump = addrUniqueOk;
                                            op = OP_Eq;
                                        }
                                        x_0 = sqlite3TableColumnToStorage(pTab, x_0 as i16_0)
                                            as ::core::ffi::c_int;
                                        sqlite3VdbeAddOp4(
                                            v,
                                            op,
                                            regOldData + 1 as ::core::ffi::c_int + x_0,
                                            addrJump,
                                            regCmp + i,
                                            p4,
                                            P4_COLLSEQ,
                                        );
                                        sqlite3VdbeChangeP5(v, SQLITE_NOTNULL as u16_0);
                                        i += 1;
                                    }
                                }
                            }
                        }
                        let mut c2rust_current_block_379: u64;
                        match onError {
                            OE_Rollback | OE_Abort | OE_Fail => {
                                sqlite3UniqueConstraint(pParse, onError, pIdx);
                                c2rust_current_block_379 = 10648164479545198704;
                            }
                            OE_Update => {
                                sqlite3UpsertDoUpdate(
                                    pParse,
                                    pUpsert,
                                    pTab,
                                    pIdx,
                                    iIdxCur + ix,
                                );
                                c2rust_current_block_379 = 16070719095729554596;
                            }
                            OE_Ignore => {
                                c2rust_current_block_379 = 16070719095729554596;
                            }
                            _ => {
                                let mut nConflictCk: ::core::ffi::c_int = 0;
                                nConflictCk = sqlite3VdbeCurrentAddr(v) - addrConflictCk;
                                if regTrigCnt != 0 {
                                    sqlite3MultiWrite(pParse);
                                    nReplaceTrig += 1;
                                }
                                if !pTrigger.is_null()
                                    && isUpdate as ::core::ffi::c_int != 0
                                {
                                    sqlite3VdbeAddOp1(v, OP_CursorLock, iDataCur);
                                }
                                sqlite3GenerateRowDelete(
                                    pParse,
                                    pTab,
                                    pTrigger,
                                    iDataCur,
                                    iIdxCur,
                                    regR,
                                    nPkField as i16_0,
                                    0 as u8_0,
                                    OE_Replace as u8_0,
                                    (if pIdx == pPk { ONEPASS_SINGLE } else { ONEPASS_OFF })
                                        as u8_0,
                                    iThisCur,
                                );
                                if !pTrigger.is_null()
                                    && isUpdate as ::core::ffi::c_int != 0
                                {
                                    sqlite3VdbeAddOp1(v, OP_CursorUnlock, iDataCur);
                                }
                                if regTrigCnt != 0 {
                                    let mut addrBypass: ::core::ffi::c_int = 0;
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_AddImm,
                                        regTrigCnt,
                                        1 as ::core::ffi::c_int,
                                    );
                                    addrBypass = sqlite3VdbeAddOp0(v, OP_Goto);
                                    sqlite3VdbeResolveLabel(v, lblRecheckOk);
                                    lblRecheckOk = sqlite3VdbeMakeLabel(pParse);
                                    if !(*pIdx).pPartIdxWhere.is_null() {
                                        sqlite3VdbeAddOp2(
                                            v,
                                            OP_IsNull,
                                            regIdx - 1 as ::core::ffi::c_int,
                                            lblRecheckOk,
                                        );
                                    }
                                    while nConflictCk > 0 as ::core::ffi::c_int {
                                        let mut x_1: VdbeOp = VdbeOp {
                                            opcode: 0,
                                            p4type: 0,
                                            p5: 0,
                                            p1: 0,
                                            p2: 0,
                                            p3: 0,
                                            p4: p4union { i: 0 }

                                        };
                                        x_1 = *sqlite3VdbeGetOp(v, addrConflictCk);
                                        if x_1.opcode as ::core::ffi::c_int != OP_IdxRowid {
                                            let mut p2: ::core::ffi::c_int = 0;
                                            let mut zP4: *const ::core::ffi::c_char = ::core::ptr::null::<
                                                ::core::ffi::c_char,
                                            >();
                                            if *(&raw const sqlite3OpcodeProperty
                                                as *const ::core::ffi::c_uchar)
                                                .offset(x_1.opcode as isize) as ::core::ffi::c_int
                                                & OPFLG_JUMP != 0
                                            {
                                                p2 = lblRecheckOk;
                                            } else {
                                                p2 = x_1.p2;
                                            }
                                            zP4 = (if x_1.p4type as ::core::ffi::c_int == P4_INT32 {
                                                x_1.p4.i as intptr_t as *mut ::core::ffi::c_void
                                            } else {
                                                x_1.p4.z as *mut ::core::ffi::c_void
                                            }) as *const ::core::ffi::c_char;
                                            sqlite3VdbeAddOp4(
                                                v,
                                                x_1.opcode as ::core::ffi::c_int,
                                                x_1.p1,
                                                p2,
                                                x_1.p3,
                                                zP4,
                                                x_1.p4type as ::core::ffi::c_int,
                                            );
                                            sqlite3VdbeChangeP5(v, x_1.p5);
                                        }
                                        nConflictCk -= 1;
                                        addrConflictCk += 1;
                                    }
                                    sqlite3UniqueConstraint(pParse, OE_Abort, pIdx);
                                    sqlite3VdbeJumpHere(v, addrBypass);
                                }
                                seenReplace = 1 as ::core::ffi::c_int;
                                c2rust_current_block_379 = 10648164479545198704;
                            }
                        }
                        match c2rust_current_block_379 {
                            16070719095729554596 => {
                                sqlite3VdbeGoto(v, ignoreDest);
                            }
                            _ => {}
                        }
                        sqlite3VdbeResolveLabel(v, addrUniqueOk);
                        if regR != regIdx {
                            sqlite3ReleaseTempRange(pParse, regR, nPkField);
                        }
                        if !pUpsertClause.is_null() && upsertIpkReturn != 0
                            && sqlite3UpsertNextIsIPK(pUpsertClause) != 0
                        {
                            sqlite3VdbeGoto(v, upsertIpkDelay + 1 as ::core::ffi::c_int);
                            sqlite3VdbeJumpHere(v, upsertIpkReturn);
                            upsertIpkReturn = 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            pIdx = indexIteratorNext(&raw mut sIdxIter, &raw mut ix);
        }
        if ipkTop != 0 {
            sqlite3VdbeGoto(v, ipkTop);
            sqlite3VdbeJumpHere(v, ipkBottom);
        }
        if nReplaceTrig != 0 {
            sqlite3VdbeAddOp2(v, OP_IfNot, regTrigCnt, lblRecheckOk);
            if pPk.is_null() {
                if isUpdate != 0 {
                    sqlite3VdbeAddOp3(v, OP_Eq, regNewData, addrRecheck, regOldData);
                    sqlite3VdbeChangeP5(v, SQLITE_NOTNULL as u16_0);
                }
                sqlite3VdbeAddOp3(v, OP_NotExists, iDataCur, addrRecheck, regNewData);
                sqlite3RowidConstraint(pParse, OE_Abort, pTab);
            } else {
                sqlite3VdbeGoto(v, addrRecheck);
            }
            sqlite3VdbeResolveLabel(v, lblRecheckOk);
        }
        if (*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0 {
            let mut regRec: ::core::ffi::c_int = *aRegIdx.offset(ix as isize);
            sqlite3VdbeAddOp3(
                v,
                OP_MakeRecord,
                regNewData + 1 as ::core::ffi::c_int,
                (*pTab).nNVCol as ::core::ffi::c_int,
                regRec,
            );
            if bAffinityDone == 0 {
                sqlite3TableAffinity(v, pTab, 0 as ::core::ffi::c_int);
            }
        }
        *pbMayReplace = seenReplace;
    }
}
unsafe extern "C" fn codeWithoutRowidPreupdate(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut iCur: ::core::ffi::c_int,
    mut regData: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut r: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
        sqlite3VdbeAddOp2(v, OP_Integer, 0 as ::core::ffi::c_int, r);
        sqlite3VdbeAddOp4(
            v,
            OP_Insert,
            iCur,
            regData,
            r,
            pTab as *mut ::core::ffi::c_char,
            P4_TABLE,
        );
        sqlite3VdbeChangeP5(v, OPFLAG_ISNOOP as u16_0);
        sqlite3ReleaseTempReg(pParse, r);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CompleteInsertion(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut iDataCur: ::core::ffi::c_int,
    mut iIdxCur: ::core::ffi::c_int,
    mut regNewData: ::core::ffi::c_int,
    mut aRegIdx: *mut ::core::ffi::c_int,
    mut update_flags: ::core::ffi::c_int,
    mut appendBias: ::core::ffi::c_int,
    mut useSeekResult: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut pik_flags: u8_0 = 0;
        let mut i: ::core::ffi::c_int = 0;
        v = (*pParse).pVdbe;
        i = 0 as ::core::ffi::c_int;
        pIdx = (*pTab).pIndex;
        while !pIdx.is_null() {
            if !(*aRegIdx.offset(i as isize) == 0 as ::core::ffi::c_int) {
                if !(*pIdx).pPartIdxWhere.is_null() {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_IsNull,
                        *aRegIdx.offset(i as isize),
                        sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
                    );
                }
                pik_flags = (if useSeekResult != 0 {
                    OPFLAG_USESEEKRESULT
                } else {
                    0 as ::core::ffi::c_int
                }) as u8_0;
                if (*pIdx).idxType() as ::core::ffi::c_int == SQLITE_IDXTYPE_PRIMARYKEY
                    && !((*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0)
                {
                    pik_flags = (pik_flags as ::core::ffi::c_int | OPFLAG_NCHANGE)
                        as u8_0;
                    pik_flags = (pik_flags as ::core::ffi::c_int
                        | update_flags & OPFLAG_SAVEPOSITION) as u8_0;
                    if update_flags == 0 as ::core::ffi::c_int {
                        codeWithoutRowidPreupdate(
                            pParse,
                            pTab,
                            iIdxCur + i,
                            *aRegIdx.offset(i as isize),
                        );
                    }
                }
                sqlite3VdbeAddOp4Int(
                    v,
                    OP_IdxInsert,
                    iIdxCur + i,
                    *aRegIdx.offset(i as isize),
                    *aRegIdx.offset(i as isize) + 1 as ::core::ffi::c_int,
                    if (*pIdx).uniqNotNull() as ::core::ffi::c_int != 0 {
                        (*pIdx).nKeyCol as ::core::ffi::c_int
                    } else {
                        (*pIdx).nColumn as ::core::ffi::c_int
                    },
                );
                sqlite3VdbeChangeP5(v, pik_flags as u16_0);
            }
            pIdx = (*pIdx).pNext;
            i += 1;
        }
        if !((*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0) {
            return;
        }
        if (*pParse).nested != 0 {
            pik_flags = 0 as u8_0;
        } else {
            pik_flags = OPFLAG_NCHANGE as u8_0;
            pik_flags = (pik_flags as ::core::ffi::c_int
                | if update_flags != 0 { update_flags } else { OPFLAG_LASTROWID })
                as u8_0;
        }
        if appendBias != 0 {
            pik_flags = (pik_flags as ::core::ffi::c_int | OPFLAG_APPEND) as u8_0;
        }
        if useSeekResult != 0 {
            pik_flags = (pik_flags as ::core::ffi::c_int | OPFLAG_USESEEKRESULT) as u8_0;
        }
        sqlite3VdbeAddOp3(
            v,
            OP_Insert,
            iDataCur,
            *aRegIdx.offset(i as isize),
            regNewData,
        );
        if (*pParse).nested == 0 {
            sqlite3VdbeAppendP4(v, pTab as *mut ::core::ffi::c_void, P4_TABLE);
        }
        sqlite3VdbeChangeP5(v, pik_flags as u16_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OpenTableAndIndices(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut op: ::core::ffi::c_int,
    mut p5: u8_0,
    mut iBase: ::core::ffi::c_int,
    mut aToOpen: *mut u8_0,
    mut piDataCur: *mut ::core::ffi::c_int,
    mut piIdxCur: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut iDb: ::core::ffi::c_int = 0;
        let mut iDataCur: ::core::ffi::c_int = 0;
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
            *piIdxCur = -999 as ::core::ffi::c_int;
            *piDataCur = *piIdxCur;
            return 0 as ::core::ffi::c_int;
        }
        iDb = sqlite3SchemaToIndex((*pParse).db, (*pTab).pSchema);
        v = (*pParse).pVdbe;
        if iBase < 0 as ::core::ffi::c_int {
            iBase = (*pParse).nTab;
        }
        let c2rust_fresh6 = iBase;
        iBase = iBase + 1;
        iDataCur = c2rust_fresh6;
        *piDataCur = iDataCur;
        if (*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0
            && (aToOpen.is_null()
                || *aToOpen.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int != 0)
        {
            sqlite3OpenTable(pParse, iDataCur, iDb, pTab, op);
        } else if (*(*pParse).db).noSharedCache as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            sqlite3TableLock(
                pParse,
                iDb,
                (*pTab).tnum,
                (op == OP_OpenWrite) as ::core::ffi::c_int as u8_0,
                (*pTab).zName,
            );
        }
        *piIdxCur = iBase;
        i = 0 as ::core::ffi::c_int;
        pIdx = (*pTab).pIndex;
        while !pIdx.is_null() {
            let c2rust_fresh7 = iBase;
            iBase = iBase + 1;
            let mut iIdxCur: ::core::ffi::c_int = c2rust_fresh7;
            if (*pIdx).idxType() as ::core::ffi::c_int == SQLITE_IDXTYPE_PRIMARYKEY
                && !((*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0)
            {
                *piDataCur = iIdxCur;
                p5 = 0 as u8_0;
            }
            if aToOpen.is_null()
                || *aToOpen.offset((i + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int != 0
            {
                sqlite3VdbeAddOp3(
                    v,
                    op,
                    iIdxCur,
                    (*pIdx).tnum as ::core::ffi::c_int,
                    iDb,
                );
                sqlite3VdbeSetP4KeyInfo(pParse, pIdx);
                sqlite3VdbeChangeP5(v, p5 as u16_0);
            }
            pIdx = (*pIdx).pNext;
            i += 1;
        }
        if iBase > (*pParse).nTab {
            (*pParse).nTab = iBase;
        }
        return i;
    }
}
#[no_mangle]
pub static mut sqlite3_xferopt_count: ::core::ffi::c_int = 0;
unsafe extern "C" fn xferCompatibleIndex(
    mut pDest: *mut Index,
    mut pSrc: *mut Index,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if (*pDest).nKeyCol as ::core::ffi::c_int
            != (*pSrc).nKeyCol as ::core::ffi::c_int
            || (*pDest).nColumn as ::core::ffi::c_int
                != (*pSrc).nColumn as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pDest).onError as ::core::ffi::c_int
            != (*pSrc).onError as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pSrc).nKeyCol as ::core::ffi::c_int {
            if *(*pSrc).aiColumn.offset(i as isize) as ::core::ffi::c_int
                != *(*pDest).aiColumn.offset(i as isize) as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            if *(*pSrc).aiColumn.offset(i as isize) as ::core::ffi::c_int == XN_EXPR {
                if sqlite3ExprCompare(
                    ::core::ptr::null::<Parse>(),
                    (*(&raw mut (*(*pSrc).aColExpr).a as *mut ExprList_item)
                        .offset(i as isize))
                        .pExpr,
                    (*(&raw mut (*(*pDest).aColExpr).a as *mut ExprList_item)
                        .offset(i as isize))
                        .pExpr,
                    -1 as ::core::ffi::c_int,
                ) != 0 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if *(*pSrc).aSortOrder.offset(i as isize) as ::core::ffi::c_int
                != *(*pDest).aSortOrder.offset(i as isize) as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            if sqlite3_stricmp(
                *(*pSrc).azColl.offset(i as isize),
                *(*pDest).azColl.offset(i as isize),
            ) != 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
        if sqlite3ExprCompare(
            ::core::ptr::null::<Parse>(),
            (*pSrc).pPartIdxWhere,
            (*pDest).pPartIdxWhere,
            -1 as ::core::ffi::c_int,
        ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn xferOptimization(
    mut pParse: *mut Parse,
    mut pDest: *mut Table,
    mut pSelect: *mut Select,
    mut onError: ::core::ffi::c_int,
    mut iDbDest: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pSrc: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pSrcIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut pDestIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut i: ::core::ffi::c_int = 0;
        let mut iDbSrc: ::core::ffi::c_int = 0;
        let mut iSrc: ::core::ffi::c_int = 0;
        let mut iDest: ::core::ffi::c_int = 0;
        let mut addr1: ::core::ffi::c_int = 0;
        let mut addr2: ::core::ffi::c_int = 0;
        let mut emptyDestTest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut emptySrcTest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut regAutoinc: ::core::ffi::c_int = 0;
        let mut destHasUniqueIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regData: ::core::ffi::c_int = 0;
        let mut regRowid: ::core::ffi::c_int = 0;
        if !(*pParse).pWith.is_null() || !(*pSelect).pWith.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*pDest).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
            return 0 as ::core::ffi::c_int;
        }
        if onError == OE_Default {
            if (*pDest).iPKey as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                onError = (*pDest).keyConf as ::core::ffi::c_int;
            }
            if onError == OE_Default {
                onError = OE_Abort;
            }
        }
        if (*(*pSelect).pSrc).nSrc != 1 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if (*(&raw mut (*(*pSelect).pSrc).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .fg
            .isSubquery() != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSelect).pWhere.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSelect).pOrderBy.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSelect).pGroupBy.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSelect).pLimit.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSelect).pPrior.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSelect).selFlags & SF_Distinct as u32_0 != 0 {
            return 0 as ::core::ffi::c_int;
        }
        pEList = (*pSelect).pEList;
        if (*pEList).nExpr != 1 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*(&raw mut (*pEList).a as *mut ExprList_item)
            .offset(0 as ::core::ffi::c_int as isize))
            .pExpr)
            .op as ::core::ffi::c_int != TK_ASTERISK
        {
            return 0 as ::core::ffi::c_int;
        }
        pItem = &raw mut (*(*pSelect).pSrc).a as *mut SrcItem;
        pSrc = sqlite3LocateTableItem(pParse, 0 as u32_0, pItem);
        if pSrc.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSrc).tnum == (*pDest).tnum && (*pSrc).pSchema == (*pDest).pSchema {
            return 0 as ::core::ffi::c_int;
        }
        if ((*pDest).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0)
            as ::core::ffi::c_int
            != ((*pSrc).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0)
                as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if !((*pSrc).eTabType as ::core::ffi::c_int == TABTYP_NORM) {
            return 0 as ::core::ffi::c_int;
        }
        if (*pDest).nCol as ::core::ffi::c_int != (*pSrc).nCol as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if (*pDest).iPKey as ::core::ffi::c_int != (*pSrc).iPKey as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if (*pDest).tabFlags & TF_Strict as u32_0 != 0 as u32_0
            && (*pSrc).tabFlags & TF_Strict as u32_0 == 0 as u32_0
        {
            return 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pDest).nCol as ::core::ffi::c_int {
            let mut pDestCol: *mut Column = (*pDest).aCol.offset(i as isize)
                as *mut Column;
            let mut pSrcCol: *mut Column = (*pSrc).aCol.offset(i as isize)
                as *mut Column;
            if (*pDestCol).colFlags as ::core::ffi::c_int & COLFLAG_GENERATED
                != (*pSrcCol).colFlags as ::core::ffi::c_int & COLFLAG_GENERATED
            {
                return 0 as ::core::ffi::c_int;
            }
            if (*pDestCol).colFlags as ::core::ffi::c_int & COLFLAG_GENERATED
                != 0 as ::core::ffi::c_int
            {
                if sqlite3ExprCompare(
                    ::core::ptr::null::<Parse>(),
                    sqlite3ColumnExpr(pSrc, pSrcCol),
                    sqlite3ColumnExpr(pDest, pDestCol),
                    -1 as ::core::ffi::c_int,
                ) != 0 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if (*pDestCol).affinity as ::core::ffi::c_int
                != (*pSrcCol).affinity as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            if sqlite3_stricmp(sqlite3ColumnColl(pDestCol), sqlite3ColumnColl(pSrcCol))
                != 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            if (*pDestCol).notNull() as ::core::ffi::c_int != 0
                && (*pSrcCol).notNull() == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            if (*pDestCol).colFlags as ::core::ffi::c_int & COLFLAG_GENERATED
                == 0 as ::core::ffi::c_int && i > 0 as ::core::ffi::c_int
            {
                let mut pDestExpr: *mut Expr = sqlite3ColumnExpr(pDest, pDestCol);
                let mut pSrcExpr: *mut Expr = sqlite3ColumnExpr(pSrc, pSrcCol);
                if pDestExpr.is_null() as ::core::ffi::c_int
                    != pSrcExpr.is_null() as ::core::ffi::c_int
                    || !pDestExpr.is_null()
                        && strcmp((*pDestExpr).u.zToken, (*pSrcExpr).u.zToken)
                            != 0 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
            i += 1;
        }
        pDestIdx = (*pDest).pIndex;
        while !pDestIdx.is_null() {
            if (*pDestIdx).onError as ::core::ffi::c_int != OE_None {
                destHasUniqueIdx = 1 as ::core::ffi::c_int;
            }
            pSrcIdx = (*pSrc).pIndex;
            while !pSrcIdx.is_null() {
                if xferCompatibleIndex(pDestIdx, pSrcIdx) != 0 {
                    break;
                }
                pSrcIdx = (*pSrcIdx).pNext;
            }
            if pSrcIdx.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if (*pSrcIdx).tnum == (*pDestIdx).tnum && (*pSrc).pSchema == (*pDest).pSchema
                && sqlite3FaultSim(411 as ::core::ffi::c_int) == SQLITE_OK
            {
                return 0 as ::core::ffi::c_int;
            }
            pDestIdx = (*pDestIdx).pNext;
        }
        if !(*pDest).pCheck.is_null()
            && (*db).mDbFlags & DBFLAG_Vacuum as u32_0 == 0 as u32_0
            && sqlite3ExprListCompare(
                (*pSrc).pCheck,
                (*pDest).pCheck,
                -1 as ::core::ffi::c_int,
            ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*db).flags as ::core::ffi::c_ulonglong
            & SQLITE_ForeignKeys as ::core::ffi::c_ulonglong
            != 0 as ::core::ffi::c_ulonglong && !(*pDest).u.tab.pFKey.is_null()
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*db).flags & SQLITE_CountRows != 0 as ::core::ffi::c_ulonglong {
            return 0 as ::core::ffi::c_int;
        }
        sqlite3_xferopt_count += 1;
        iDbSrc = sqlite3SchemaToIndex(db, (*pSrc).pSchema);
        v = sqlite3GetVdbe(pParse);
        sqlite3CodeVerifySchema(pParse, iDbSrc);
        let c2rust_fresh8 = (*pParse).nTab;
        (*pParse).nTab = (*pParse).nTab + 1;
        iSrc = c2rust_fresh8;
        let c2rust_fresh9 = (*pParse).nTab;
        (*pParse).nTab = (*pParse).nTab + 1;
        iDest = c2rust_fresh9;
        regAutoinc = autoIncBegin(pParse, iDbDest, pDest);
        regData = sqlite3GetTempReg(pParse);
        sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, regData);
        regRowid = sqlite3GetTempReg(pParse);
        sqlite3OpenTable(pParse, iDest, iDbDest, pDest, OP_OpenWrite);
        if (*db).mDbFlags & DBFLAG_Vacuum as u32_0 == 0 as u32_0
            && (((*pDest).iPKey as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                && !(*pDest).pIndex.is_null() || destHasUniqueIdx != 0
                || onError != OE_Abort && onError != OE_Rollback)
        {
            addr1 = sqlite3VdbeAddOp2(v, OP_Rewind, iDest, 0 as ::core::ffi::c_int);
            emptyDestTest = sqlite3VdbeAddOp0(v, OP_Goto);
            sqlite3VdbeJumpHere(v, addr1);
        }
        if (*pSrc).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0 {
            let mut insFlags: u8_0 = 0;
            sqlite3OpenTable(pParse, iSrc, iDbSrc, pSrc, OP_OpenRead);
            emptySrcTest = sqlite3VdbeAddOp2(
                v,
                OP_Rewind,
                iSrc,
                0 as ::core::ffi::c_int,
            );
            if (*pDest).iPKey as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                addr1 = sqlite3VdbeAddOp2(v, OP_Rowid, iSrc, regRowid);
                if (*db).mDbFlags & DBFLAG_Vacuum as u32_0 == 0 as u32_0 {
                    addr2 = sqlite3VdbeAddOp3(
                        v,
                        OP_NotExists,
                        iDest,
                        0 as ::core::ffi::c_int,
                        regRowid,
                    );
                    sqlite3RowidConstraint(pParse, onError, pDest);
                    sqlite3VdbeJumpHere(v, addr2);
                }
                autoIncStep(pParse, regAutoinc, regRowid);
            } else if (*pDest).pIndex.is_null()
                && (*db).mDbFlags & DBFLAG_VacuumInto as u32_0 == 0
            {
                addr1 = sqlite3VdbeAddOp2(v, OP_NewRowid, iDest, regRowid);
            } else {
                addr1 = sqlite3VdbeAddOp2(v, OP_Rowid, iSrc, regRowid);
            }
            if (*db).mDbFlags & DBFLAG_Vacuum as u32_0 != 0 {
                sqlite3VdbeAddOp1(v, OP_SeekEnd, iDest);
                insFlags = (OPFLAG_APPEND | OPFLAG_USESEEKRESULT | OPFLAG_PREFORMAT)
                    as u8_0;
            } else {
                insFlags = (OPFLAG_NCHANGE | OPFLAG_LASTROWID | OPFLAG_APPEND
                    | OPFLAG_PREFORMAT) as u8_0;
            }
            if (*db).mDbFlags & DBFLAG_Vacuum as u32_0 == 0 as u32_0 {
                sqlite3VdbeAddOp3(v, OP_RowData, iSrc, regData, 1 as ::core::ffi::c_int);
                insFlags = (insFlags as ::core::ffi::c_int & !OPFLAG_PREFORMAT) as u8_0;
            } else {
                sqlite3VdbeAddOp3(v, OP_RowCell, iDest, iSrc, regRowid);
            }
            sqlite3VdbeAddOp3(v, OP_Insert, iDest, regData, regRowid);
            if (*db).mDbFlags & DBFLAG_Vacuum as u32_0 == 0 as u32_0 {
                sqlite3VdbeChangeP4(
                    v,
                    -1 as ::core::ffi::c_int,
                    pDest as *mut ::core::ffi::c_char,
                    P4_TABLE,
                );
            }
            sqlite3VdbeChangeP5(v, insFlags as u16_0);
            sqlite3VdbeAddOp2(v, OP_Next, iSrc, addr1);
            sqlite3VdbeAddOp2(v, OP_Close, iSrc, 0 as ::core::ffi::c_int);
            sqlite3VdbeAddOp2(v, OP_Close, iDest, 0 as ::core::ffi::c_int);
        } else {
            sqlite3TableLock(pParse, iDbDest, (*pDest).tnum, 1 as u8_0, (*pDest).zName);
            sqlite3TableLock(pParse, iDbSrc, (*pSrc).tnum, 0 as u8_0, (*pSrc).zName);
        }
        pDestIdx = (*pDest).pIndex;
        while !pDestIdx.is_null() {
            let mut idxInsFlags: u8_0 = 0 as u8_0;
            pSrcIdx = (*pSrc).pIndex;
            while !pSrcIdx.is_null() {
                if xferCompatibleIndex(pDestIdx, pSrcIdx) != 0 {
                    break;
                }
                pSrcIdx = (*pSrcIdx).pNext;
            }
            sqlite3VdbeAddOp3(
                v,
                OP_OpenRead,
                iSrc,
                (*pSrcIdx).tnum as ::core::ffi::c_int,
                iDbSrc,
            );
            sqlite3VdbeSetP4KeyInfo(pParse, pSrcIdx);
            sqlite3VdbeAddOp3(
                v,
                OP_OpenWrite,
                iDest,
                (*pDestIdx).tnum as ::core::ffi::c_int,
                iDbDest,
            );
            sqlite3VdbeSetP4KeyInfo(pParse, pDestIdx);
            sqlite3VdbeChangeP5(v, OPFLAG_BULKCSR as u16_0);
            addr1 = sqlite3VdbeAddOp2(v, OP_Rewind, iSrc, 0 as ::core::ffi::c_int);
            if (*db).mDbFlags & DBFLAG_Vacuum as u32_0 != 0 {
                i = 0 as ::core::ffi::c_int;
                while i < (*pSrcIdx).nColumn as ::core::ffi::c_int {
                    let mut zColl: *const ::core::ffi::c_char = *(*pSrcIdx)
                        .azColl
                        .offset(i as isize);
                    if sqlite3_stricmp(
                        &raw const sqlite3StrBINARY as *const ::core::ffi::c_char,
                        zColl,
                    ) != 0
                    {
                        break;
                    }
                    i += 1;
                }
                if i == (*pSrcIdx).nColumn as ::core::ffi::c_int {
                    idxInsFlags = (OPFLAG_USESEEKRESULT | OPFLAG_PREFORMAT) as u8_0;
                    sqlite3VdbeAddOp1(v, OP_SeekEnd, iDest);
                    sqlite3VdbeAddOp2(v, OP_RowCell, iDest, iSrc);
                }
            } else if !((*pSrc).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0)
                && (*pDestIdx).idxType() as ::core::ffi::c_int
                    == SQLITE_IDXTYPE_PRIMARYKEY
            {
                idxInsFlags = (idxInsFlags as ::core::ffi::c_int | OPFLAG_NCHANGE)
                    as u8_0;
            }
            if idxInsFlags as ::core::ffi::c_int
                != OPFLAG_USESEEKRESULT | OPFLAG_PREFORMAT
            {
                sqlite3VdbeAddOp3(v, OP_RowData, iSrc, regData, 1 as ::core::ffi::c_int);
                if (*db).mDbFlags & DBFLAG_Vacuum as u32_0 == 0 as u32_0
                    && !((*pDest).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0)
                    && (*pDestIdx).idxType() as ::core::ffi::c_int
                        == SQLITE_IDXTYPE_PRIMARYKEY
                {
                    codeWithoutRowidPreupdate(pParse, pDest, iDest, regData);
                }
            }
            sqlite3VdbeAddOp2(v, OP_IdxInsert, iDest, regData);
            sqlite3VdbeChangeP5(
                v,
                (idxInsFlags as ::core::ffi::c_int | OPFLAG_APPEND) as u16_0,
            );
            sqlite3VdbeAddOp2(v, OP_Next, iSrc, addr1 + 1 as ::core::ffi::c_int);
            sqlite3VdbeJumpHere(v, addr1);
            sqlite3VdbeAddOp2(v, OP_Close, iSrc, 0 as ::core::ffi::c_int);
            sqlite3VdbeAddOp2(v, OP_Close, iDest, 0 as ::core::ffi::c_int);
            pDestIdx = (*pDestIdx).pNext;
        }
        if emptySrcTest != 0 {
            sqlite3VdbeJumpHere(v, emptySrcTest);
        }
        sqlite3ReleaseTempReg(pParse, regRowid);
        sqlite3ReleaseTempReg(pParse, regData);
        if emptyDestTest != 0 {
            sqlite3AutoincrementEnd(pParse);
            sqlite3VdbeAddOp2(v, OP_Halt, SQLITE_OK, 0 as ::core::ffi::c_int);
            sqlite3VdbeJumpHere(v, emptyDestTest);
            sqlite3VdbeAddOp2(v, OP_Close, iDest, 0 as ::core::ffi::c_int);
            return 0 as ::core::ffi::c_int;
        } else {
            return 1 as ::core::ffi::c_int
        };
    }
}
pub const SQLITE_IgnoreChecks: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SQLITE_RecTriggers: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SQLITE_ForeignKeys: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_CountRows: u64_0 = (0x1 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const DBFLAG_Vacuum: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const DBFLAG_VacuumInto: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const DBFLAG_SchemaKnownOk: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const COLFLAG_VIRTUAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const COLFLAG_STORED: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const COLFLAG_NOTAVAIL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const COLFLAG_BUSY: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const COLFLAG_GENERATED: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const COLFLAG_NOINSERT: ::core::ffi::c_int = 0x62 as ::core::ffi::c_int;
pub const SQLITE_AFF_NONE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_AFF_BLOB: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const SQLITE_AFF_INTEGER: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;
pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_NOTNULL: ::core::ffi::c_int = 0x90 as ::core::ffi::c_int;
pub const TF_HasHidden: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const TF_Autoincrement: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const TF_HasStored: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const TF_HasGenerated: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_OOOHidden: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const TF_HasNotNull: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const TF_Strict: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const TABTYP_NORM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TABTYP_VIEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_None: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OE_Rollback: ::core::ffi::c_int = 1;
pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Fail: ::core::ffi::c_int = 3;
pub const OE_Ignore: ::core::ffi::c_int = 4;
pub const OE_Replace: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const OE_Update: ::core::ffi::c_int = 6;
pub const OE_Default: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_IDXTYPE_PRIMARYKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XN_ROWID: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const XN_EXPR: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const SF_Distinct: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SF_Values: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SF_MultiValue: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SRT_Coroutine: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const PARSE_MODE_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OPFLAG_NCHANGE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPFLAG_LASTROWID: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const OPFLAG_APPEND: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPFLAG_USESEEKRESULT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const OPFLAG_ISNOOP: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const OPFLAG_BULKCSR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPFLAG_SAVEPOSITION: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPFLAG_PREFORMAT: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TRIGGER_BEFORE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TRIGGER_AFTER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WRC_Continue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ONEPASS_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ONEPASS_SINGLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_INSERT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const TK_DELETE: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const TK_ALL: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const TK_SELECT: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const TK_COLUMN: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
pub const TK_ASTERISK: ::core::ffi::c_int = 180 as ::core::ffi::c_int;
