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
    pub type WhereInfo;
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
    fn sqlite3VdbeChangeP5(_: *mut Vdbe, P5: u16_0);
    fn sqlite3VdbeJumpHere(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeJumpHereOrPopInst(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeChangeToNoop(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAppendP4(
        _: *mut Vdbe,
        pP4: *mut ::core::ffi::c_void,
        p4type: ::core::ffi::c_int,
    );
    fn sqlite3VdbeMakeLabel(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3VdbeResolveLabel(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeCurrentAddr(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeCountChanges(_: *mut Vdbe);
    fn sqlite3VdbeDb(_: *mut Vdbe) -> *mut sqlite3;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
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
    fn sqlite3ColumnExpr(_: *mut Table, _: *mut Column) -> *mut Expr;
    fn sqlite3PrimaryKeyIndex(_: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToStorage(_: *mut Table, _: i16_0) -> i16_0;
    fn sqlite3ViewGetColumnNames(_: *mut Parse, _: *mut Table) -> ::core::ffi::c_int;
    fn sqlite3AutoincrementEnd(pParse: *mut Parse);
    fn sqlite3ComputeGeneratedColumns(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *mut Table,
    );
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
    fn sqlite3ExprCodeGetColumnOfTable(
        _: *mut Vdbe,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprCode(_: *mut Parse, _: *mut Expr, _: ::core::ffi::c_int);
    fn sqlite3ExprIfFalse(
        _: *mut Parse,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3GetVdbe(_: *mut Parse) -> *mut Vdbe;
    fn sqlite3IsRowid(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3GenerateRowIndexDelete(
        _: *mut Parse,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprReferencesUpdatedColumn(
        _: *mut Expr,
        _: *mut ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3GenerateConstraintChecks(
        _: *mut Parse,
        _: *mut Table,
        _: *mut ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: u8_0,
        _: u8_0,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut Upsert,
    );
    fn sqlite3CompleteInsertion(
        _: *mut Parse,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
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
    fn sqlite3SrcListDup(
        _: *mut sqlite3,
        _: *const SrcList,
        _: ::core::ffi::c_int,
    ) -> *mut SrcList;
    fn sqlite3MaterializeView(
        _: *mut Parse,
        _: *mut Table,
        _: *mut Expr,
        _: *mut ExprList,
        _: *mut Expr,
        _: ::core::ffi::c_int,
    );
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
    fn sqlite3TableAffinity(_: *mut Vdbe, _: *mut Table, _: ::core::ffi::c_int);
    fn sqlite3ValueFromExpr(
        _: *mut sqlite3,
        _: *const Expr,
        _: u8_0,
        _: u8_0,
        _: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3ResolveExprNames(_: *mut NameContext, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
    fn sqlite3KeyInfoOfIndex(_: *mut Parse, _: *mut Index) -> *mut KeyInfo;
    fn sqlite3SelectDestInit(
        _: *mut SelectDest,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3GetVTable(_: *mut sqlite3, _: *mut Table) -> *mut VTable;
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
pub const SQLITE_DENY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_IGNORE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_UPDATE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const OP_VUpdate: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const OP_MustBeInt: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const OP_Once: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const OP_NotFound: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const OP_NotExists: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const OP_IsNull: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const OP_Copy: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
pub const OP_SCopy: ::core::ffi::c_int = 82 as ::core::ffi::c_int;
pub const OP_AddImm: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
pub const OP_RealAffinity: ::core::ffi::c_int = 88 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_MakeRecord: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const OP_OpenWrite: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const OP_OpenEphemeral: ::core::ffi::c_int = 119 as ::core::ffi::c_int;
pub const OP_Close: ::core::ffi::c_int = 123 as ::core::ffi::c_int;
pub const OP_NewRowid: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const OP_Insert: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const OP_Delete: ::core::ffi::c_int = 131 as ::core::ffi::c_int;
pub const OP_RowData: ::core::ffi::c_int = 135 as ::core::ffi::c_int;
pub const OP_Rowid: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const OP_IdxInsert: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const OP_FinishSeek: ::core::ffi::c_int = 144 as ::core::ffi::c_int;
pub const OP_VColumn: ::core::ffi::c_int = 177 as ::core::ffi::c_int;
pub const ALLBITS: Bitmask = -1 as ::core::ffi::c_int as Bitmask;
pub const P4_TABLE: ::core::ffi::c_int = -5 as ::core::ffi::c_int;
pub const P4_KEYINFO: ::core::ffi::c_int = -8 as ::core::ffi::c_int;
pub const P4_MEM: ::core::ffi::c_int = -10 as ::core::ffi::c_int;
pub const P4_VTAB: ::core::ffi::c_int = -11 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3ColumnDefault(
    mut v: *mut Vdbe,
    mut pTab: *mut Table,
    mut i: ::core::ffi::c_int,
    mut iReg: ::core::ffi::c_int,
) {
    unsafe {
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        pCol = (*pTab).aCol.offset(i as isize) as *mut Column;
        if (*pCol).iDflt != 0 {
            let mut pValue: *mut sqlite3_value = ::core::ptr::null_mut::<
                sqlite3_value,
            >();
            let mut enc: u8_0 = (*sqlite3VdbeDb(v)).enc;
            sqlite3ValueFromExpr(
                sqlite3VdbeDb(v),
                sqlite3ColumnExpr(pTab, pCol),
                enc,
                (*pCol).affinity as u8_0,
                &raw mut pValue,
            );
            if !pValue.is_null() {
                sqlite3VdbeAppendP4(v, pValue as *mut ::core::ffi::c_void, P4_MEM);
            }
        }
        if (*pCol).affinity as ::core::ffi::c_int == SQLITE_AFF_REAL
            && !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB)
        {
            sqlite3VdbeAddOp1(v, OP_RealAffinity, iReg);
        }
    }
}
unsafe extern "C" fn indexColumnIsBeingUpdated(
    mut pIdx: *mut Index,
    mut iCol: ::core::ffi::c_int,
    mut aXRef: *mut ::core::ffi::c_int,
    mut chngRowid: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iIdxCol: i16_0 = *(*pIdx).aiColumn.offset(iCol as isize);
        if iIdxCol as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            return (*aXRef.offset(iIdxCol as isize) >= 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        }
        return sqlite3ExprReferencesUpdatedColumn(
            (*(&raw mut (*(*pIdx).aColExpr).a as *mut ExprList_item)
                .offset(iCol as isize))
                .pExpr,
            aXRef,
            chngRowid,
        );
    }
}
unsafe extern "C" fn indexWhereClauseMightChange(
    mut pIdx: *mut Index,
    mut aXRef: *mut ::core::ffi::c_int,
    mut chngRowid: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pIdx).pPartIdxWhere.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        return sqlite3ExprReferencesUpdatedColumn(
            (*pIdx).pPartIdxWhere,
            aXRef,
            chngRowid,
        );
    }
}
unsafe extern "C" fn exprRowColumn(
    mut pParse: *mut Parse,
    mut iCol: ::core::ffi::c_int,
) -> *mut Expr {
    unsafe {
        let mut pRet: *mut Expr = sqlite3PExpr(
            pParse,
            TK_ROW,
            ::core::ptr::null_mut::<Expr>(),
            ::core::ptr::null_mut::<Expr>(),
        );
        if !pRet.is_null() {
            (*pRet).iColumn = (iCol + 1 as ::core::ffi::c_int) as ynVar;
        }
        return pRet;
    }
}
unsafe extern "C" fn updateFromSelect(
    mut pParse: *mut Parse,
    mut iEph: ::core::ffi::c_int,
    mut pPk: *mut Index,
    mut pChanges: *mut ExprList,
    mut pTabList: *mut SrcList,
    mut pWhere: *mut Expr,
    mut pOrderBy: *mut ExprList,
    mut pLimit: *mut Expr,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut dest: SelectDest = SelectDest {
            eDest: 0,
            iSDParm: 0,
            iSDParm2: 0,
            iSdst: 0,
            nSdst: 0,
            zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pOrderBy: ::core::ptr::null_mut::<ExprList>()

        };
        let mut pSelect: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pGrp: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pLimit2: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pOrderBy2: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pTab: *mut Table = (*(&raw mut (*pTabList).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .pSTab;
        let mut pSrc: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut pWhere2: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut eDest: ::core::ffi::c_int = 0;
        pSrc = sqlite3SrcListDup(db, pTabList, 0 as ::core::ffi::c_int);
        pWhere2 = sqlite3ExprDup(db, pWhere, 0 as ::core::ffi::c_int);
        if !pSrc.is_null() {
            (*(&raw mut (*pSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .iCursor = -1 as ::core::ffi::c_int;
            let ref mut c2rust_fresh3 = (*(*(&raw mut (*pSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .pSTab)
                .nTabRef;
            *c2rust_fresh3 = (*c2rust_fresh3).wrapping_sub(1);
            let ref mut c2rust_fresh4 = (*(&raw mut (*pSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .pSTab;
            *c2rust_fresh4 = ::core::ptr::null_mut::<Table>();
        }
        if !pPk.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*pPk).nKeyCol as ::core::ffi::c_int {
                let mut pNew: *mut Expr = exprRowColumn(
                    pParse,
                    *(*pPk).aiColumn.offset(i as isize) as ::core::ffi::c_int,
                );
                pList = sqlite3ExprListAppend(pParse, pList, pNew);
                i += 1;
            }
            eDest = if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                SRT_Table
            } else {
                SRT_Upfrom
            };
        } else if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
            i = 0 as ::core::ffi::c_int;
            while i < (*pTab).nCol as ::core::ffi::c_int {
                pList = sqlite3ExprListAppend(pParse, pList, exprRowColumn(pParse, i));
                i += 1;
            }
            eDest = SRT_Table;
        } else {
            eDest = if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                SRT_Table
            } else {
                SRT_Upfrom
            };
            pList = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                sqlite3PExpr(
                    pParse,
                    TK_ROW,
                    ::core::ptr::null_mut::<Expr>(),
                    ::core::ptr::null_mut::<Expr>(),
                ),
            );
        }
        if !pChanges.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*pChanges).nExpr {
                pList = sqlite3ExprListAppend(
                    pParse,
                    pList,
                    sqlite3ExprDup(
                        db,
                        (*(&raw mut (*pChanges).a as *mut ExprList_item)
                            .offset(i as isize))
                            .pExpr,
                        0 as ::core::ffi::c_int,
                    ),
                );
                i += 1;
            }
        }
        pSelect = sqlite3SelectNew(
            pParse,
            pList,
            pSrc,
            pWhere2,
            pGrp,
            ::core::ptr::null_mut::<Expr>(),
            pOrderBy2,
            (SF_UFSrcCheck | SF_IncludeHidden | SF_UpdateFrom) as u32_0,
            pLimit2,
        );
        if !pSelect.is_null() {
            (*pSelect).selFlags = ((*pSelect).selFlags as ::core::ffi::c_uint
                | SF_OrderByReqd as ::core::ffi::c_uint) as u32_0;
        }
        sqlite3SelectDestInit(&raw mut dest, eDest, iEph);
        dest.iSDParm2 = if !pPk.is_null() {
            (*pPk).nKeyCol as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
        sqlite3Select(pParse, pSelect, &raw mut dest);
        sqlite3SelectDelete(db, pSelect);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Update(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pChanges: *mut ExprList,
    mut pWhere: *mut Expr,
    mut onError: ::core::ffi::c_int,
    mut pOrderBy: *mut ExprList,
    mut pLimit: *mut Expr,
    mut pUpsert: *mut Upsert,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut addrTop: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pWInfo: *mut WhereInfo = ::core::ptr::null_mut::<WhereInfo>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut pPk: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut nIdx: ::core::ffi::c_int = 0;
        let mut nAllIdx: ::core::ffi::c_int = 0;
        let mut iBaseCur: ::core::ffi::c_int = 0;
        let mut iDataCur: ::core::ffi::c_int = 0;
        let mut iIdxCur: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut aRegIdx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        let mut aXRef: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        let mut aToOpen: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut chngPk: u8_0 = 0;
        let mut chngRowid: u8_0 = 0;
        let mut chngKey: u8_0 = 0;
        let mut pRowidExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut iRowidExpr: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
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
        let mut eOnePass: ::core::ffi::c_int = 0;
        let mut hasFK: ::core::ffi::c_int = 0;
        let mut labelBreak: ::core::ffi::c_int = 0;
        let mut labelContinue: ::core::ffi::c_int = 0;
        let mut flags: ::core::ffi::c_int = 0;
        let mut isView: ::core::ffi::c_int = 0;
        let mut pTrigger: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        let mut tmask: ::core::ffi::c_int = 0;
        let mut newmask: ::core::ffi::c_int = 0;
        let mut iEph: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nKey: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aiCurOnePass: [::core::ffi::c_int; 2] = [0; 2];
        let mut addrOpen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iPk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nPk: i16_0 = 0 as i16_0;
        let mut bReplace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bFinishSeek: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut nChangeFrom: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regRowCount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regOldRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regNewRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regNew: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regOld: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regRowSet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regKey: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        memset(
            &raw mut sContext as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<AuthContext>() as size_t,
        );
        db = (*pParse).db;
        if !((*pParse).nErr != 0) {
            pTab = sqlite3SrcListLookup(pParse, pTabList);
            if !pTab.is_null() {
                iDb = sqlite3SchemaToIndex((*pParse).db, (*pTab).pSchema);
                pTrigger = sqlite3TriggersExist(
                    pParse,
                    pTab,
                    TK_UPDATE,
                    pChanges,
                    &raw mut tmask,
                );
                isView = ((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW)
                    as ::core::ffi::c_int;
                nChangeFrom = if (*pTabList).nSrc > 1 as ::core::ffi::c_int {
                    (*pChanges).nExpr
                } else {
                    0 as ::core::ffi::c_int
                };
                if !(sqlite3ViewGetColumnNames(pParse, pTab) != 0) {
                    if !(sqlite3IsReadOnly(pParse, pTab, pTrigger) != 0) {
                        let c2rust_fresh0 = (*pParse).nTab;
                        (*pParse).nTab = (*pParse).nTab + 1;
                        iDataCur = c2rust_fresh0;
                        iBaseCur = iDataCur;
                        iIdxCur = iDataCur + 1 as ::core::ffi::c_int;
                        pPk = if (*pTab).tabFlags & TF_WithoutRowid as u32_0
                            == 0 as u32_0
                        {
                            ::core::ptr::null_mut::<Index>()
                        } else {
                            sqlite3PrimaryKeyIndex(pTab)
                        };
                        nIdx = 0 as ::core::ffi::c_int;
                        pIdx = (*pTab).pIndex;
                        while !pIdx.is_null() {
                            if pPk == pIdx {
                                iDataCur = (*pParse).nTab;
                            }
                            (*pParse).nTab += 1;
                            pIdx = (*pIdx).pNext;
                            nIdx += 1;
                        }
                        if !pUpsert.is_null() {
                            iDataCur = (*pUpsert).iDataCur;
                            iIdxCur = (*pUpsert).iIdxCur;
                            (*pParse).nTab = iBaseCur;
                        }
                        (*(&raw mut (*pTabList).a as *mut SrcItem)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .iCursor = iDataCur;
                        aXRef = sqlite3DbMallocRawNN(
                            db,
                            (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                .wrapping_mul(
                                    ((*pTab).nCol as ::core::ffi::c_int + nIdx
                                        + 1 as ::core::ffi::c_int) as usize,
                                )
                                .wrapping_add(nIdx as usize)
                                .wrapping_add(2 as usize) as u64_0,
                        ) as *mut ::core::ffi::c_int;
                        if !aXRef.is_null() {
                            aRegIdx = aXRef
                                .offset((*pTab).nCol as ::core::ffi::c_int as isize);
                            aToOpen = aRegIdx
                                .offset(nIdx as isize)
                                .offset(1 as ::core::ffi::c_int as isize) as *mut u8_0;
                            memset(
                                aToOpen as *mut ::core::ffi::c_void,
                                1 as ::core::ffi::c_int,
                                (nIdx + 1 as ::core::ffi::c_int) as size_t,
                            );
                            *aToOpen.offset((nIdx + 1 as ::core::ffi::c_int) as isize) = 0
                                as u8_0;
                            i = 0 as ::core::ffi::c_int;
                            while i < (*pTab).nCol as ::core::ffi::c_int {
                                *aXRef.offset(i as isize) = -1 as ::core::ffi::c_int;
                                i += 1;
                            }
                            memset(
                                &raw mut sNC as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<NameContext>() as size_t,
                            );
                            sNC.pParse = pParse;
                            sNC.pSrcList = pTabList;
                            sNC.uNC.pUpsert = pUpsert;
                            sNC.ncFlags = NC_UUpsert;
                            v = sqlite3GetVdbe(pParse);
                            if !v.is_null() {
                                chngPk = 0 as u8_0;
                                chngRowid = chngPk;
                                i = 0 as ::core::ffi::c_int;
                                loop {
                                    if !(i < (*pChanges).nExpr) {
                                        c2rust_current_block = 15237655884915618618;
                                        break;
                                    }
                                    if nChangeFrom == 0 as ::core::ffi::c_int
                                        && sqlite3ResolveExprNames(
                                            &raw mut sNC,
                                            (*(&raw mut (*pChanges).a as *mut ExprList_item)
                                                .offset(i as isize))
                                                .pExpr,
                                        ) != 0
                                    {
                                        c2rust_current_block = 131595005288832221;
                                        break;
                                    }
                                    j = sqlite3ColumnIndex(
                                        pTab,
                                        (*(&raw mut (*pChanges).a as *mut ExprList_item)
                                            .offset(i as isize))
                                            .zEName,
                                    );
                                    if j >= 0 as ::core::ffi::c_int {
                                        if j == (*pTab).iPKey as ::core::ffi::c_int {
                                            chngRowid = 1 as u8_0;
                                            pRowidExpr = (*(&raw mut (*pChanges).a
                                                as *mut ExprList_item)
                                                .offset(i as isize))
                                                .pExpr;
                                            iRowidExpr = i;
                                        } else if !pPk.is_null()
                                            && (*(*pTab).aCol.offset(j as isize)).colFlags
                                                as ::core::ffi::c_int & COLFLAG_PRIMKEY
                                                != 0 as ::core::ffi::c_int
                                        {
                                            chngPk = 1 as u8_0;
                                        } else if (*(*pTab).aCol.offset(j as isize)).colFlags
                                            as ::core::ffi::c_int & COLFLAG_GENERATED != 0
                                        {
                                            sqlite3ErrorMsg(
                                                pParse,
                                                b"cannot UPDATE generated column \"%s\"\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                (*(*pTab).aCol.offset(j as isize)).zCnName,
                                            );
                                            c2rust_current_block = 131595005288832221;
                                            break;
                                        }
                                        *aXRef.offset(j as isize) = i;
                                    } else if pPk.is_null()
                                        && sqlite3IsRowid(
                                            (*(&raw mut (*pChanges).a as *mut ExprList_item)
                                                .offset(i as isize))
                                                .zEName,
                                        ) != 0
                                    {
                                        j = -1 as ::core::ffi::c_int;
                                        chngRowid = 1 as u8_0;
                                        pRowidExpr = (*(&raw mut (*pChanges).a
                                            as *mut ExprList_item)
                                            .offset(i as isize))
                                            .pExpr;
                                        iRowidExpr = i;
                                    } else {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            b"no such column: %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            (*(&raw mut (*pChanges).a as *mut ExprList_item)
                                                .offset(i as isize))
                                                .zEName,
                                        );
                                        (*pParse).set_checkSchema(1 as bft as bft);
                                        c2rust_current_block = 131595005288832221;
                                        break;
                                    }
                                    let mut rc: ::core::ffi::c_int = 0;
                                    rc = sqlite3AuthCheck(
                                        pParse,
                                        SQLITE_UPDATE,
                                        (*pTab).zName,
                                        if j < 0 as ::core::ffi::c_int {
                                            b"ROWID\0".as_ptr() as *const ::core::ffi::c_char
                                        } else {
                                            (*(*pTab).aCol.offset(j as isize)).zCnName
                                                as *const ::core::ffi::c_char
                                        },
                                        (*(*db).aDb.offset(iDb as isize)).zDbSName,
                                    );
                                    if rc == SQLITE_DENY {
                                        c2rust_current_block = 131595005288832221;
                                        break;
                                    }
                                    if rc == SQLITE_IGNORE {
                                        *aXRef.offset(j as isize) = -1 as ::core::ffi::c_int;
                                    }
                                    i += 1;
                                }
                                match c2rust_current_block {
                                    131595005288832221 => {}
                                    _ => {
                                        chngKey = (chngRowid as ::core::ffi::c_int
                                            + chngPk as ::core::ffi::c_int) as u8_0;
                                        if (*pTab).tabFlags & TF_HasGenerated as u32_0 != 0 {
                                            let mut bProgress: ::core::ffi::c_int = 0;
                                            loop {
                                                bProgress = 0 as ::core::ffi::c_int;
                                                i = 0 as ::core::ffi::c_int;
                                                while i < (*pTab).nCol as ::core::ffi::c_int {
                                                    if !(*aXRef.offset(i as isize) >= 0 as ::core::ffi::c_int) {
                                                        if !((*(*pTab).aCol.offset(i as isize)).colFlags
                                                            as ::core::ffi::c_int & COLFLAG_GENERATED
                                                            == 0 as ::core::ffi::c_int)
                                                        {
                                                            if sqlite3ExprReferencesUpdatedColumn(
                                                                sqlite3ColumnExpr(
                                                                    pTab,
                                                                    (*pTab).aCol.offset(i as isize) as *mut Column,
                                                                ),
                                                                aXRef,
                                                                chngRowid as ::core::ffi::c_int,
                                                            ) != 0
                                                            {
                                                                *aXRef.offset(i as isize) = 99999 as ::core::ffi::c_int;
                                                                bProgress = 1 as ::core::ffi::c_int;
                                                            }
                                                        }
                                                    }
                                                    i += 1;
                                                }
                                                if !(bProgress != 0) {
                                                    break;
                                                }
                                            }
                                        }
                                        (*(&raw mut (*pTabList).a as *mut SrcItem)
                                            .offset(0 as ::core::ffi::c_int as isize))
                                            .colUsed = (if (*pTab).eTabType as ::core::ffi::c_int
                                            == TABTYP_VTAB
                                        {
                                            ALLBITS
                                        } else {
                                            0 as ::core::ffi::c_ulonglong
                                        }) as Bitmask;
                                        hasFK = sqlite3FkRequired(
                                            pParse,
                                            pTab,
                                            aXRef,
                                            chngKey as ::core::ffi::c_int,
                                        );
                                        if onError == OE_Replace {
                                            bReplace = 1 as ::core::ffi::c_int;
                                        }
                                        nAllIdx = 0 as ::core::ffi::c_int;
                                        pIdx = (*pTab).pIndex;
                                        while !pIdx.is_null() {
                                            let mut reg: ::core::ffi::c_int = 0;
                                            if chngKey as ::core::ffi::c_int != 0
                                                || hasFK > 1 as ::core::ffi::c_int || pIdx == pPk
                                                || indexWhereClauseMightChange(
                                                    pIdx,
                                                    aXRef,
                                                    chngRowid as ::core::ffi::c_int,
                                                ) != 0
                                            {
                                                (*pParse).nMem += 1;
                                                reg = (*pParse).nMem;
                                                (*pParse).nMem += (*pIdx).nColumn as ::core::ffi::c_int;
                                            } else {
                                                reg = 0 as ::core::ffi::c_int;
                                                i = 0 as ::core::ffi::c_int;
                                                while i < (*pIdx).nKeyCol as ::core::ffi::c_int {
                                                    if indexColumnIsBeingUpdated(
                                                        pIdx,
                                                        i,
                                                        aXRef,
                                                        chngRowid as ::core::ffi::c_int,
                                                    ) != 0
                                                    {
                                                        (*pParse).nMem += 1;
                                                        reg = (*pParse).nMem;
                                                        (*pParse).nMem += (*pIdx).nColumn as ::core::ffi::c_int;
                                                        if onError == OE_Default
                                                            && (*pIdx).onError as ::core::ffi::c_int == OE_Replace
                                                        {
                                                            bReplace = 1 as ::core::ffi::c_int;
                                                        }
                                                        break;
                                                    } else {
                                                        i += 1;
                                                    }
                                                }
                                            }
                                            if reg == 0 as ::core::ffi::c_int {
                                                *aToOpen
                                                    .offset((nAllIdx + 1 as ::core::ffi::c_int) as isize) = 0
                                                    as u8_0;
                                            }
                                            *aRegIdx.offset(nAllIdx as isize) = reg;
                                            pIdx = (*pIdx).pNext;
                                            nAllIdx += 1;
                                        }
                                        (*pParse).nMem += 1;
                                        *aRegIdx.offset(nAllIdx as isize) = (*pParse).nMem;
                                        if bReplace != 0 {
                                            memset(
                                                aToOpen as *mut ::core::ffi::c_void,
                                                1 as ::core::ffi::c_int,
                                                (nIdx + 1 as ::core::ffi::c_int) as size_t,
                                            );
                                        }
                                        if (*pParse).nested as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                        {
                                            sqlite3VdbeCountChanges(v);
                                        }
                                        sqlite3BeginWriteOperation(
                                            pParse,
                                            (!pTrigger.is_null() || hasFK != 0) as ::core::ffi::c_int,
                                            iDb,
                                        );
                                        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB)
                                        {
                                            regRowSet = *aRegIdx.offset(nAllIdx as isize);
                                            (*pParse).nMem += 1;
                                            regNewRowid = (*pParse).nMem;
                                            regOldRowid = regNewRowid;
                                            if chngPk as ::core::ffi::c_int != 0 || !pTrigger.is_null()
                                                || hasFK != 0
                                            {
                                                regOld = (*pParse).nMem + 1 as ::core::ffi::c_int;
                                                (*pParse).nMem += (*pTab).nCol as ::core::ffi::c_int;
                                            }
                                            if chngKey as ::core::ffi::c_int != 0 || !pTrigger.is_null()
                                                || hasFK != 0
                                            {
                                                (*pParse).nMem += 1;
                                                regNewRowid = (*pParse).nMem;
                                            }
                                            regNew = (*pParse).nMem + 1 as ::core::ffi::c_int;
                                            (*pParse).nMem += (*pTab).nCol as ::core::ffi::c_int;
                                        }
                                        if isView != 0 {
                                            sqlite3AuthContextPush(
                                                pParse,
                                                &raw mut sContext,
                                                (*pTab).zName,
                                            );
                                        }
                                        if nChangeFrom == 0 as ::core::ffi::c_int && isView != 0 {
                                            sqlite3MaterializeView(
                                                pParse,
                                                pTab,
                                                pWhere,
                                                pOrderBy,
                                                pLimit,
                                                iDataCur,
                                            );
                                            pOrderBy = ::core::ptr::null_mut::<ExprList>();
                                            pLimit = ::core::ptr::null_mut::<Expr>();
                                        }
                                        if !(nChangeFrom == 0 as ::core::ffi::c_int
                                            && sqlite3ResolveExprNames(&raw mut sNC, pWhere) != 0)
                                        {
                                            if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                                                updateVirtualTable(
                                                    pParse,
                                                    pTabList,
                                                    pTab,
                                                    pChanges,
                                                    pRowidExpr,
                                                    aXRef,
                                                    pWhere,
                                                    onError,
                                                );
                                            } else {
                                                labelBreak = sqlite3VdbeMakeLabel(pParse);
                                                labelContinue = labelBreak;
                                                if (*db).flags & SQLITE_CountRows
                                                    != 0 as ::core::ffi::c_ulonglong
                                                    && (*pParse).pTriggerTab.is_null() && (*pParse).nested == 0
                                                    && (*pParse).bReturning == 0 && pUpsert.is_null()
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
                                                if nChangeFrom == 0 as ::core::ffi::c_int
                                                    && (*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0
                                                {
                                                    sqlite3VdbeAddOp3(
                                                        v,
                                                        OP_Null,
                                                        0 as ::core::ffi::c_int,
                                                        regRowSet,
                                                        regOldRowid,
                                                    );
                                                    let c2rust_fresh1 = (*pParse).nTab;
                                                    (*pParse).nTab = (*pParse).nTab + 1;
                                                    iEph = c2rust_fresh1;
                                                    addrOpen = sqlite3VdbeAddOp3(
                                                        v,
                                                        OP_OpenEphemeral,
                                                        iEph,
                                                        0 as ::core::ffi::c_int,
                                                        regRowSet,
                                                    );
                                                } else {
                                                    nPk = (if !pPk.is_null() {
                                                        (*pPk).nKeyCol as ::core::ffi::c_int
                                                    } else {
                                                        0 as ::core::ffi::c_int
                                                    }) as i16_0;
                                                    iPk = (*pParse).nMem + 1 as ::core::ffi::c_int;
                                                    (*pParse).nMem += nPk as ::core::ffi::c_int;
                                                    (*pParse).nMem += nChangeFrom;
                                                    (*pParse).nMem += 1;
                                                    regKey = (*pParse).nMem;
                                                    if pUpsert.is_null() {
                                                        let mut nEphCol: ::core::ffi::c_int = nPk
                                                            as ::core::ffi::c_int + nChangeFrom
                                                            + (if isView != 0 {
                                                                (*pTab).nCol as ::core::ffi::c_int
                                                            } else {
                                                                0 as ::core::ffi::c_int
                                                            });
                                                        let c2rust_fresh2 = (*pParse).nTab;
                                                        (*pParse).nTab = (*pParse).nTab + 1;
                                                        iEph = c2rust_fresh2;
                                                        if !pPk.is_null() {
                                                            sqlite3VdbeAddOp3(
                                                                v,
                                                                OP_Null,
                                                                0 as ::core::ffi::c_int,
                                                                iPk,
                                                                iPk + nPk as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                                                            );
                                                        }
                                                        addrOpen = sqlite3VdbeAddOp2(
                                                            v,
                                                            OP_OpenEphemeral,
                                                            iEph,
                                                            nEphCol,
                                                        );
                                                        if !pPk.is_null() {
                                                            let mut pKeyInfo: *mut KeyInfo = sqlite3KeyInfoOfIndex(
                                                                pParse,
                                                                pPk,
                                                            );
                                                            if !pKeyInfo.is_null() {
                                                                (*pKeyInfo).nAllField = nEphCol as u16_0;
                                                                sqlite3VdbeAppendP4(
                                                                    v,
                                                                    pKeyInfo as *mut ::core::ffi::c_void,
                                                                    P4_KEYINFO,
                                                                );
                                                            }
                                                        }
                                                        if nChangeFrom != 0 {
                                                            updateFromSelect(
                                                                pParse,
                                                                iEph,
                                                                pPk,
                                                                pChanges,
                                                                pTabList,
                                                                pWhere,
                                                                pOrderBy,
                                                                pLimit,
                                                            );
                                                            if isView != 0 {
                                                                iDataCur = iEph;
                                                            }
                                                        }
                                                    }
                                                }
                                                if nChangeFrom != 0 {
                                                    sqlite3MultiWrite(pParse);
                                                    eOnePass = ONEPASS_OFF;
                                                    nKey = nPk as ::core::ffi::c_int;
                                                    regKey = iPk;
                                                    c2rust_current_block = 11783992868154864732;
                                                } else {
                                                    if !pUpsert.is_null() {
                                                        pWInfo = ::core::ptr::null_mut::<WhereInfo>();
                                                        eOnePass = ONEPASS_SINGLE;
                                                        sqlite3ExprIfFalse(
                                                            pParse,
                                                            pWhere,
                                                            labelBreak,
                                                            SQLITE_JUMPIFNULL,
                                                        );
                                                        bFinishSeek = 0 as ::core::ffi::c_int;
                                                        c2rust_current_block = 6958737826328148217;
                                                    } else {
                                                        flags = WHERE_ONEPASS_DESIRED;
                                                        if (*pParse).nested == 0 && pTrigger.is_null() && hasFK == 0
                                                            && chngKey == 0 && bReplace == 0
                                                            && (pWhere.is_null()
                                                                || !((*pWhere).flags
                                                                    & 0x400000 as ::core::ffi::c_int as u32_0 != 0 as u32_0))
                                                        {
                                                            flags |= WHERE_ONEPASS_MULTIROW;
                                                        }
                                                        pWInfo = sqlite3WhereBegin(
                                                            pParse,
                                                            pTabList,
                                                            pWhere,
                                                            ::core::ptr::null_mut::<ExprList>(),
                                                            ::core::ptr::null_mut::<ExprList>(),
                                                            ::core::ptr::null_mut::<Select>(),
                                                            flags as u16_0,
                                                            iIdxCur,
                                                        );
                                                        if pWInfo.is_null() {
                                                            c2rust_current_block = 131595005288832221;
                                                        } else {
                                                            eOnePass = sqlite3WhereOkOnePass(
                                                                pWInfo,
                                                                &raw mut aiCurOnePass as *mut ::core::ffi::c_int,
                                                            );
                                                            bFinishSeek = sqlite3WhereUsesDeferredSeek(pWInfo);
                                                            if eOnePass != ONEPASS_SINGLE {
                                                                sqlite3MultiWrite(pParse);
                                                                if eOnePass == ONEPASS_MULTI {
                                                                    let mut iCur: ::core::ffi::c_int = aiCurOnePass[1
                                                                        as ::core::ffi::c_int as usize];
                                                                    if iCur >= 0 as ::core::ffi::c_int && iCur != iDataCur
                                                                        && *aToOpen.offset((iCur - iBaseCur) as isize)
                                                                            as ::core::ffi::c_int != 0
                                                                    {
                                                                        eOnePass = ONEPASS_OFF;
                                                                    }
                                                                }
                                                            }
                                                            c2rust_current_block = 6958737826328148217;
                                                        }
                                                    }
                                                    match c2rust_current_block {
                                                        131595005288832221 => {}
                                                        _ => {
                                                            if (*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0
                                                            {
                                                                sqlite3VdbeAddOp2(v, OP_Rowid, iDataCur, regOldRowid);
                                                                if eOnePass == ONEPASS_OFF {
                                                                    (*pParse).nMem += 1;
                                                                    *aRegIdx.offset(nAllIdx as isize) = (*pParse).nMem;
                                                                    sqlite3VdbeAddOp3(
                                                                        v,
                                                                        OP_Insert,
                                                                        iEph,
                                                                        regRowSet,
                                                                        regOldRowid,
                                                                    );
                                                                } else if addrOpen != 0 {
                                                                    sqlite3VdbeChangeToNoop(v, addrOpen);
                                                                }
                                                            } else {
                                                                i = 0 as ::core::ffi::c_int;
                                                                while i < nPk as ::core::ffi::c_int {
                                                                    sqlite3ExprCodeGetColumnOfTable(
                                                                        v,
                                                                        pTab,
                                                                        iDataCur,
                                                                        *(*pPk).aiColumn.offset(i as isize) as ::core::ffi::c_int,
                                                                        iPk + i,
                                                                    );
                                                                    i += 1;
                                                                }
                                                                if eOnePass != 0 {
                                                                    if addrOpen != 0 {
                                                                        sqlite3VdbeChangeToNoop(v, addrOpen);
                                                                    }
                                                                    nKey = nPk as ::core::ffi::c_int;
                                                                    regKey = iPk;
                                                                } else {
                                                                    sqlite3VdbeAddOp4(
                                                                        v,
                                                                        OP_MakeRecord,
                                                                        iPk,
                                                                        nPk as ::core::ffi::c_int,
                                                                        regKey,
                                                                        sqlite3IndexAffinityStr(db, pPk),
                                                                        nPk as ::core::ffi::c_int,
                                                                    );
                                                                    sqlite3VdbeAddOp4Int(
                                                                        v,
                                                                        OP_IdxInsert,
                                                                        iEph,
                                                                        regKey,
                                                                        iPk,
                                                                        nPk as ::core::ffi::c_int,
                                                                    );
                                                                }
                                                            }
                                                            c2rust_current_block = 11783992868154864732;
                                                        }
                                                    }
                                                }
                                                match c2rust_current_block {
                                                    131595005288832221 => {}
                                                    _ => {
                                                        if pUpsert.is_null() {
                                                            if nChangeFrom == 0 as ::core::ffi::c_int
                                                                && eOnePass != ONEPASS_MULTI
                                                            {
                                                                sqlite3WhereEnd(pWInfo);
                                                            }
                                                            if isView == 0 {
                                                                let mut addrOnce: ::core::ffi::c_int = 0
                                                                    as ::core::ffi::c_int;
                                                                let mut iNotUsed1: ::core::ffi::c_int = 0
                                                                    as ::core::ffi::c_int;
                                                                let mut iNotUsed2: ::core::ffi::c_int = 0
                                                                    as ::core::ffi::c_int;
                                                                if eOnePass != ONEPASS_OFF {
                                                                    if aiCurOnePass[0 as ::core::ffi::c_int as usize]
                                                                        >= 0 as ::core::ffi::c_int
                                                                    {
                                                                        *aToOpen
                                                                            .offset(
                                                                                (aiCurOnePass[0 as ::core::ffi::c_int as usize] - iBaseCur)
                                                                                    as isize,
                                                                            ) = 0 as u8_0;
                                                                    }
                                                                    if aiCurOnePass[1 as ::core::ffi::c_int as usize]
                                                                        >= 0 as ::core::ffi::c_int
                                                                    {
                                                                        *aToOpen
                                                                            .offset(
                                                                                (aiCurOnePass[1 as ::core::ffi::c_int as usize] - iBaseCur)
                                                                                    as isize,
                                                                            ) = 0 as u8_0;
                                                                    }
                                                                }
                                                                if eOnePass == ONEPASS_MULTI
                                                                    && nIdx
                                                                        - (aiCurOnePass[1 as ::core::ffi::c_int as usize]
                                                                            >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                                                                        > 0 as ::core::ffi::c_int
                                                                {
                                                                    addrOnce = sqlite3VdbeAddOp0(v, OP_Once);
                                                                }
                                                                sqlite3OpenTableAndIndices(
                                                                    pParse,
                                                                    pTab,
                                                                    OP_OpenWrite,
                                                                    0 as u8_0,
                                                                    iBaseCur,
                                                                    aToOpen,
                                                                    &raw mut iNotUsed1,
                                                                    &raw mut iNotUsed2,
                                                                );
                                                                if addrOnce != 0 {
                                                                    sqlite3VdbeJumpHereOrPopInst(v, addrOnce);
                                                                }
                                                            }
                                                            if eOnePass != ONEPASS_OFF {
                                                                if aiCurOnePass[0 as ::core::ffi::c_int as usize]
                                                                    != iDataCur
                                                                    && aiCurOnePass[1 as ::core::ffi::c_int as usize]
                                                                        != iDataCur
                                                                {
                                                                    sqlite3VdbeAddOp4Int(
                                                                        v,
                                                                        OP_NotFound,
                                                                        iDataCur,
                                                                        labelBreak,
                                                                        regKey,
                                                                        nKey,
                                                                    );
                                                                }
                                                                if eOnePass != ONEPASS_SINGLE {
                                                                    labelContinue = sqlite3VdbeMakeLabel(pParse);
                                                                }
                                                                sqlite3VdbeAddOp2(
                                                                    v,
                                                                    OP_IsNull,
                                                                    if !pPk.is_null() { regKey } else { regOldRowid },
                                                                    labelBreak,
                                                                );
                                                            } else if !pPk.is_null() || nChangeFrom != 0 {
                                                                labelContinue = sqlite3VdbeMakeLabel(pParse);
                                                                sqlite3VdbeAddOp2(v, OP_Rewind, iEph, labelBreak);
                                                                addrTop = sqlite3VdbeCurrentAddr(v);
                                                                if nChangeFrom != 0 {
                                                                    if isView == 0 {
                                                                        if !pPk.is_null() {
                                                                            i = 0 as ::core::ffi::c_int;
                                                                            while i < nPk as ::core::ffi::c_int {
                                                                                sqlite3VdbeAddOp3(v, OP_Column, iEph, i, iPk + i);
                                                                                i += 1;
                                                                            }
                                                                            sqlite3VdbeAddOp4Int(
                                                                                v,
                                                                                OP_NotFound,
                                                                                iDataCur,
                                                                                labelContinue,
                                                                                iPk,
                                                                                nPk as ::core::ffi::c_int,
                                                                            );
                                                                        } else {
                                                                            sqlite3VdbeAddOp2(v, OP_Rowid, iEph, regOldRowid);
                                                                            sqlite3VdbeAddOp3(
                                                                                v,
                                                                                OP_NotExists,
                                                                                iDataCur,
                                                                                labelContinue,
                                                                                regOldRowid,
                                                                            );
                                                                        }
                                                                    }
                                                                } else {
                                                                    sqlite3VdbeAddOp2(v, OP_RowData, iEph, regKey);
                                                                    sqlite3VdbeAddOp4Int(
                                                                        v,
                                                                        OP_NotFound,
                                                                        iDataCur,
                                                                        labelContinue,
                                                                        regKey,
                                                                        0 as ::core::ffi::c_int,
                                                                    );
                                                                }
                                                            } else {
                                                                sqlite3VdbeAddOp2(v, OP_Rewind, iEph, labelBreak);
                                                                labelContinue = sqlite3VdbeMakeLabel(pParse);
                                                                addrTop = sqlite3VdbeAddOp2(v, OP_Rowid, iEph, regOldRowid);
                                                                sqlite3VdbeAddOp3(
                                                                    v,
                                                                    OP_NotExists,
                                                                    iDataCur,
                                                                    labelContinue,
                                                                    regOldRowid,
                                                                );
                                                            }
                                                        }
                                                        if chngRowid != 0 {
                                                            if nChangeFrom == 0 as ::core::ffi::c_int {
                                                                sqlite3ExprCode(pParse, pRowidExpr, regNewRowid);
                                                            } else {
                                                                sqlite3VdbeAddOp3(
                                                                    v,
                                                                    OP_Column,
                                                                    iEph,
                                                                    iRowidExpr,
                                                                    regNewRowid,
                                                                );
                                                            }
                                                            sqlite3VdbeAddOp1(v, OP_MustBeInt, regNewRowid);
                                                        }
                                                        if chngPk as ::core::ffi::c_int != 0 || hasFK != 0
                                                            || !pTrigger.is_null()
                                                        {
                                                            let mut oldmask: u32_0 = if hasFK != 0 {
                                                                sqlite3FkOldmask(pParse, pTab)
                                                            } else {
                                                                0 as u32_0
                                                            };
                                                            oldmask = (oldmask as ::core::ffi::c_uint
                                                                | sqlite3TriggerColmask(
                                                                    pParse,
                                                                    pTrigger,
                                                                    pChanges,
                                                                    0 as ::core::ffi::c_int,
                                                                    TRIGGER_BEFORE | TRIGGER_AFTER,
                                                                    pTab,
                                                                    onError,
                                                                ) as ::core::ffi::c_uint) as u32_0;
                                                            i = 0 as ::core::ffi::c_int;
                                                            while i < (*pTab).nCol as ::core::ffi::c_int {
                                                                let mut colFlags: u32_0 = (*(*pTab).aCol.offset(i as isize))
                                                                    .colFlags as u32_0;
                                                                k = sqlite3TableColumnToStorage(pTab, i as i16_0)
                                                                    as ::core::ffi::c_int + regOld;
                                                                if oldmask == 0xffffffff as u32_0
                                                                    || i < 32 as ::core::ffi::c_int
                                                                        && oldmask & (1 as ::core::ffi::c_int as u32_0) << i
                                                                            != 0 as u32_0
                                                                    || colFlags & COLFLAG_PRIMKEY as u32_0 != 0 as u32_0
                                                                {
                                                                    sqlite3ExprCodeGetColumnOfTable(v, pTab, iDataCur, i, k);
                                                                } else {
                                                                    sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, k);
                                                                }
                                                                i += 1;
                                                            }
                                                            if chngRowid as ::core::ffi::c_int
                                                                == 0 as ::core::ffi::c_int && pPk.is_null()
                                                            {
                                                                sqlite3VdbeAddOp2(v, OP_Copy, regOldRowid, regNewRowid);
                                                            }
                                                        }
                                                        newmask = sqlite3TriggerColmask(
                                                            pParse,
                                                            pTrigger,
                                                            pChanges,
                                                            1 as ::core::ffi::c_int,
                                                            TRIGGER_BEFORE,
                                                            pTab,
                                                            onError,
                                                        ) as ::core::ffi::c_int;
                                                        i = 0 as ::core::ffi::c_int;
                                                        k = regNew;
                                                        while i < (*pTab).nCol as ::core::ffi::c_int {
                                                            if i == (*pTab).iPKey as ::core::ffi::c_int {
                                                                sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, k);
                                                            } else if (*(*pTab).aCol.offset(i as isize)).colFlags
                                                                as ::core::ffi::c_int & COLFLAG_GENERATED
                                                                != 0 as ::core::ffi::c_int
                                                            {
                                                                if (*(*pTab).aCol.offset(i as isize)).colFlags
                                                                    as ::core::ffi::c_int & COLFLAG_VIRTUAL != 0
                                                                {
                                                                    k -= 1;
                                                                }
                                                            } else {
                                                                j = *aXRef.offset(i as isize);
                                                                if j >= 0 as ::core::ffi::c_int {
                                                                    if nChangeFrom != 0 {
                                                                        let mut nOff: ::core::ffi::c_int = if isView != 0 {
                                                                            (*pTab).nCol as ::core::ffi::c_int
                                                                        } else {
                                                                            nPk as ::core::ffi::c_int
                                                                        };
                                                                        sqlite3VdbeAddOp3(v, OP_Column, iEph, nOff + j, k);
                                                                    } else {
                                                                        sqlite3ExprCode(
                                                                            pParse,
                                                                            (*(&raw mut (*pChanges).a as *mut ExprList_item)
                                                                                .offset(j as isize))
                                                                                .pExpr,
                                                                            k,
                                                                        );
                                                                    }
                                                                } else if 0 as ::core::ffi::c_int == tmask & TRIGGER_BEFORE
                                                                    || i > 31 as ::core::ffi::c_int
                                                                    || newmask as ::core::ffi::c_uint
                                                                        & (1 as ::core::ffi::c_int as ::core::ffi::c_uint) << i != 0
                                                                {
                                                                    sqlite3ExprCodeGetColumnOfTable(v, pTab, iDataCur, i, k);
                                                                    bFinishSeek = 0 as ::core::ffi::c_int;
                                                                } else {
                                                                    sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, k);
                                                                }
                                                            }
                                                            i += 1;
                                                            k += 1;
                                                        }
                                                        if (*pTab).tabFlags & TF_HasGenerated as u32_0 != 0 {
                                                            sqlite3ComputeGeneratedColumns(pParse, regNew, pTab);
                                                        }
                                                        if tmask & TRIGGER_BEFORE != 0 {
                                                            sqlite3TableAffinity(v, pTab, regNew);
                                                            sqlite3CodeRowTrigger(
                                                                pParse,
                                                                pTrigger,
                                                                TK_UPDATE,
                                                                pChanges,
                                                                TRIGGER_BEFORE,
                                                                pTab,
                                                                regOldRowid,
                                                                onError,
                                                                labelContinue,
                                                            );
                                                            if isView == 0 {
                                                                if !pPk.is_null() {
                                                                    sqlite3VdbeAddOp4Int(
                                                                        v,
                                                                        OP_NotFound,
                                                                        iDataCur,
                                                                        labelContinue,
                                                                        regKey,
                                                                        nKey,
                                                                    );
                                                                } else {
                                                                    sqlite3VdbeAddOp3(
                                                                        v,
                                                                        OP_NotExists,
                                                                        iDataCur,
                                                                        labelContinue,
                                                                        regOldRowid,
                                                                    );
                                                                }
                                                                i = 0 as ::core::ffi::c_int;
                                                                k = regNew;
                                                                while i < (*pTab).nCol as ::core::ffi::c_int {
                                                                    if (*(*pTab).aCol.offset(i as isize)).colFlags
                                                                        as ::core::ffi::c_int & COLFLAG_GENERATED != 0
                                                                    {
                                                                        if (*(*pTab).aCol.offset(i as isize)).colFlags
                                                                            as ::core::ffi::c_int & COLFLAG_VIRTUAL != 0
                                                                        {
                                                                            k -= 1;
                                                                        }
                                                                    } else if *aXRef.offset(i as isize)
                                                                        < 0 as ::core::ffi::c_int
                                                                        && i != (*pTab).iPKey as ::core::ffi::c_int
                                                                    {
                                                                        sqlite3ExprCodeGetColumnOfTable(v, pTab, iDataCur, i, k);
                                                                    }
                                                                    i += 1;
                                                                    k += 1;
                                                                }
                                                                if (*pTab).tabFlags & TF_HasGenerated as u32_0 != 0 {
                                                                    sqlite3ComputeGeneratedColumns(pParse, regNew, pTab);
                                                                }
                                                            }
                                                        }
                                                        if isView == 0 {
                                                            sqlite3GenerateConstraintChecks(
                                                                pParse,
                                                                pTab,
                                                                aRegIdx,
                                                                iDataCur,
                                                                iIdxCur,
                                                                regNewRowid,
                                                                regOldRowid,
                                                                chngKey,
                                                                onError as u8_0,
                                                                labelContinue,
                                                                &raw mut bReplace,
                                                                aXRef,
                                                                ::core::ptr::null_mut::<Upsert>(),
                                                            );
                                                            if bReplace != 0 || chngKey as ::core::ffi::c_int != 0 {
                                                                if !pPk.is_null() {
                                                                    sqlite3VdbeAddOp4Int(
                                                                        v,
                                                                        OP_NotFound,
                                                                        iDataCur,
                                                                        labelContinue,
                                                                        regKey,
                                                                        nKey,
                                                                    );
                                                                } else {
                                                                    sqlite3VdbeAddOp3(
                                                                        v,
                                                                        OP_NotExists,
                                                                        iDataCur,
                                                                        labelContinue,
                                                                        regOldRowid,
                                                                    );
                                                                }
                                                            }
                                                            if hasFK != 0 {
                                                                sqlite3FkCheck(
                                                                    pParse,
                                                                    pTab,
                                                                    regOldRowid,
                                                                    0 as ::core::ffi::c_int,
                                                                    aXRef,
                                                                    chngKey as ::core::ffi::c_int,
                                                                );
                                                            }
                                                            sqlite3GenerateRowIndexDelete(
                                                                pParse,
                                                                pTab,
                                                                iDataCur,
                                                                iIdxCur,
                                                                aRegIdx,
                                                                -1 as ::core::ffi::c_int,
                                                            );
                                                            if bFinishSeek != 0 {
                                                                sqlite3VdbeAddOp1(v, OP_FinishSeek, iDataCur);
                                                            }
                                                            sqlite3VdbeAddOp3(
                                                                v,
                                                                OP_Delete,
                                                                iDataCur,
                                                                OPFLAG_ISUPDATE
                                                                    | (if hasFK > 1 as ::core::ffi::c_int
                                                                        || chngKey as ::core::ffi::c_int != 0
                                                                    {
                                                                        0 as ::core::ffi::c_int
                                                                    } else {
                                                                        OPFLAG_ISNOOP
                                                                    }),
                                                                regNewRowid,
                                                            );
                                                            if eOnePass == ONEPASS_MULTI {
                                                                sqlite3VdbeChangeP5(v, OPFLAG_SAVEPOSITION as u16_0);
                                                            }
                                                            if (*pParse).nested == 0 {
                                                                sqlite3VdbeAppendP4(
                                                                    v,
                                                                    pTab as *mut ::core::ffi::c_void,
                                                                    P4_TABLE,
                                                                );
                                                            }
                                                            if hasFK != 0 {
                                                                sqlite3FkCheck(
                                                                    pParse,
                                                                    pTab,
                                                                    0 as ::core::ffi::c_int,
                                                                    regNewRowid,
                                                                    aXRef,
                                                                    chngKey as ::core::ffi::c_int,
                                                                );
                                                            }
                                                            sqlite3CompleteInsertion(
                                                                pParse,
                                                                pTab,
                                                                iDataCur,
                                                                iIdxCur,
                                                                regNewRowid,
                                                                aRegIdx,
                                                                OPFLAG_ISUPDATE
                                                                    | (if eOnePass == ONEPASS_MULTI {
                                                                        OPFLAG_SAVEPOSITION
                                                                    } else {
                                                                        0 as ::core::ffi::c_int
                                                                    }),
                                                                0 as ::core::ffi::c_int,
                                                                0 as ::core::ffi::c_int,
                                                            );
                                                            if hasFK != 0 {
                                                                sqlite3FkActions(
                                                                    pParse,
                                                                    pTab,
                                                                    pChanges,
                                                                    regOldRowid,
                                                                    aXRef,
                                                                    chngKey as ::core::ffi::c_int,
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
                                                                TK_UPDATE,
                                                                pChanges,
                                                                TRIGGER_AFTER,
                                                                pTab,
                                                                regOldRowid,
                                                                onError,
                                                                labelContinue,
                                                            );
                                                        }
                                                        if !(eOnePass == ONEPASS_SINGLE) {
                                                            if eOnePass == ONEPASS_MULTI {
                                                                sqlite3VdbeResolveLabel(v, labelContinue);
                                                                sqlite3WhereEnd(pWInfo);
                                                            } else {
                                                                sqlite3VdbeResolveLabel(v, labelContinue);
                                                                sqlite3VdbeAddOp2(v, OP_Next, iEph, addrTop);
                                                            }
                                                        }
                                                        sqlite3VdbeResolveLabel(v, labelBreak);
                                                        if (*pParse).nested as ::core::ffi::c_int
                                                            == 0 as ::core::ffi::c_int
                                                            && (*pParse).pTriggerTab.is_null() && pUpsert.is_null()
                                                        {
                                                            sqlite3AutoincrementEnd(pParse);
                                                        }
                                                        if regRowCount != 0 {
                                                            sqlite3CodeChangeCount(
                                                                v,
                                                                regRowCount,
                                                                b"rows updated\0".as_ptr() as *const ::core::ffi::c_char,
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
                }
            }
        }
        sqlite3AuthContextPop(&raw mut sContext);
        sqlite3DbFree(db, aXRef as *mut ::core::ffi::c_void);
        sqlite3SrcListDelete(db, pTabList);
        sqlite3ExprListDelete(db, pChanges);
        sqlite3ExprDelete(db, pWhere);
    }
}
unsafe extern "C" fn updateVirtualTable(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pTab: *mut Table,
    mut pChanges: *mut ExprList,
    mut pRowid: *mut Expr,
    mut aXRef: *mut ::core::ffi::c_int,
    mut pWhere: *mut Expr,
    mut onError: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut ephemTab: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pVTab: *const ::core::ffi::c_char = sqlite3GetVTable(db, pTab)
            as *const ::core::ffi::c_char;
        let mut pWInfo: *mut WhereInfo = ::core::ptr::null_mut::<WhereInfo>();
        let mut nArg: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + (*pTab).nCol as ::core::ffi::c_int;
        let mut regArg: ::core::ffi::c_int = 0;
        let mut regRec: ::core::ffi::c_int = 0;
        let mut regRowid: ::core::ffi::c_int = 0;
        let mut iCsr: ::core::ffi::c_int = (*(&raw mut (*pSrc).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .iCursor;
        let mut aDummy: [::core::ffi::c_int; 2] = [0; 2];
        let mut eOnePass: ::core::ffi::c_int = 0;
        let mut addr: ::core::ffi::c_int = 0;
        let c2rust_fresh5 = (*pParse).nTab;
        (*pParse).nTab = (*pParse).nTab + 1;
        ephemTab = c2rust_fresh5;
        addr = sqlite3VdbeAddOp2(v, OP_OpenEphemeral, ephemTab, nArg);
        regArg = (*pParse).nMem + 1 as ::core::ffi::c_int;
        (*pParse).nMem += nArg;
        if (*pSrc).nSrc > 1 as ::core::ffi::c_int {
            let mut pPk: *mut Index = ::core::ptr::null_mut::<Index>();
            let mut pRow: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            if (*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0 {
                if !pRowid.is_null() {
                    pRow = sqlite3ExprDup(db, pRowid, 0 as ::core::ffi::c_int);
                } else {
                    pRow = sqlite3PExpr(
                        pParse,
                        TK_ROW,
                        ::core::ptr::null_mut::<Expr>(),
                        ::core::ptr::null_mut::<Expr>(),
                    );
                }
            } else {
                let mut iPk: i16_0 = 0;
                pPk = sqlite3PrimaryKeyIndex(pTab);
                iPk = *(*pPk).aiColumn.offset(0 as ::core::ffi::c_int as isize);
                if *aXRef.offset(iPk as isize) >= 0 as ::core::ffi::c_int {
                    pRow = sqlite3ExprDup(
                        db,
                        (*(&raw mut (*pChanges).a as *mut ExprList_item)
                            .offset(*aXRef.offset(iPk as isize) as isize))
                            .pExpr,
                        0 as ::core::ffi::c_int,
                    );
                } else {
                    pRow = exprRowColumn(pParse, iPk as ::core::ffi::c_int);
                }
            }
            pList = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                pRow,
            );
            i = 0 as ::core::ffi::c_int;
            while i < (*pTab).nCol as ::core::ffi::c_int {
                if *aXRef.offset(i as isize) >= 0 as ::core::ffi::c_int {
                    pList = sqlite3ExprListAppend(
                        pParse,
                        pList,
                        sqlite3ExprDup(
                            db,
                            (*(&raw mut (*pChanges).a as *mut ExprList_item)
                                .offset(*aXRef.offset(i as isize) as isize))
                                .pExpr,
                            0 as ::core::ffi::c_int,
                        ),
                    );
                } else {
                    let mut pRowExpr: *mut Expr = exprRowColumn(pParse, i);
                    if !pRowExpr.is_null() {
                        (*pRowExpr).op2 = OPFLAG_NOCHNG as u8_0;
                    }
                    pList = sqlite3ExprListAppend(pParse, pList, pRowExpr);
                }
                i += 1;
            }
            updateFromSelect(
                pParse,
                ephemTab,
                pPk,
                pList,
                pSrc,
                pWhere,
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Expr>(),
            );
            sqlite3ExprListDelete(db, pList);
            eOnePass = ONEPASS_OFF;
        } else {
            (*pParse).nMem += 1;
            regRec = (*pParse).nMem;
            (*pParse).nMem += 1;
            regRowid = (*pParse).nMem;
            pWInfo = sqlite3WhereBegin(
                pParse,
                pSrc,
                pWhere,
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Select>(),
                WHERE_ONEPASS_DESIRED as u16_0,
                0 as ::core::ffi::c_int,
            );
            if pWInfo.is_null() {
                return;
            }
            i = 0 as ::core::ffi::c_int;
            while i < (*pTab).nCol as ::core::ffi::c_int {
                if *aXRef.offset(i as isize) >= 0 as ::core::ffi::c_int {
                    sqlite3ExprCode(
                        pParse,
                        (*(&raw mut (*pChanges).a as *mut ExprList_item)
                            .offset(*aXRef.offset(i as isize) as isize))
                            .pExpr,
                        regArg + 2 as ::core::ffi::c_int + i,
                    );
                } else {
                    sqlite3VdbeAddOp3(
                        v,
                        OP_VColumn,
                        iCsr,
                        i,
                        regArg + 2 as ::core::ffi::c_int + i,
                    );
                    sqlite3VdbeChangeP5(v, OPFLAG_NOCHNG as u16_0);
                }
                i += 1;
            }
            if (*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0 {
                sqlite3VdbeAddOp2(v, OP_Rowid, iCsr, regArg);
                if !pRowid.is_null() {
                    sqlite3ExprCode(pParse, pRowid, regArg + 1 as ::core::ffi::c_int);
                } else {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Rowid,
                        iCsr,
                        regArg + 1 as ::core::ffi::c_int,
                    );
                }
            } else {
                let mut pPk_0: *mut Index = ::core::ptr::null_mut::<Index>();
                let mut iPk_0: i16_0 = 0;
                pPk_0 = sqlite3PrimaryKeyIndex(pTab);
                iPk_0 = *(*pPk_0).aiColumn.offset(0 as ::core::ffi::c_int as isize);
                sqlite3VdbeAddOp3(
                    v,
                    OP_VColumn,
                    iCsr,
                    iPk_0 as ::core::ffi::c_int,
                    regArg,
                );
                sqlite3VdbeAddOp2(
                    v,
                    OP_SCopy,
                    regArg + 2 as ::core::ffi::c_int + iPk_0 as ::core::ffi::c_int,
                    regArg + 1 as ::core::ffi::c_int,
                );
            }
            eOnePass = sqlite3WhereOkOnePass(
                pWInfo,
                &raw mut aDummy as *mut ::core::ffi::c_int,
            );
            if eOnePass != 0 {
                sqlite3VdbeChangeToNoop(v, addr);
                sqlite3VdbeAddOp1(v, OP_Close, iCsr);
            } else {
                sqlite3MultiWrite(pParse);
                sqlite3VdbeAddOp3(v, OP_MakeRecord, regArg, nArg, regRec);
                sqlite3VdbeAddOp2(v, OP_NewRowid, ephemTab, regRowid);
                sqlite3VdbeAddOp3(v, OP_Insert, ephemTab, regRec, regRowid);
            }
        }
        if eOnePass == ONEPASS_OFF {
            if (*pSrc).nSrc == 1 as ::core::ffi::c_int {
                sqlite3WhereEnd(pWInfo);
            }
            addr = sqlite3VdbeAddOp1(v, OP_Rewind, ephemTab);
            i = 0 as ::core::ffi::c_int;
            while i < nArg {
                sqlite3VdbeAddOp3(v, OP_Column, ephemTab, i, regArg + i);
                i += 1;
            }
        }
        sqlite3VtabMakeWritable(pParse, pTab);
        sqlite3VdbeAddOp4(
            v,
            OP_VUpdate,
            0 as ::core::ffi::c_int,
            nArg,
            regArg,
            pVTab,
            P4_VTAB,
        );
        sqlite3VdbeChangeP5(
            v,
            (if onError == OE_Default { OE_Abort } else { onError }) as u16_0,
        );
        sqlite3MayAbort(pParse);
        if eOnePass == ONEPASS_OFF {
            sqlite3VdbeAddOp2(v, OP_Next, ephemTab, addr + 1 as ::core::ffi::c_int);
            sqlite3VdbeJumpHere(v, addr);
            sqlite3VdbeAddOp2(v, OP_Close, ephemTab, 0 as ::core::ffi::c_int);
        } else {
            sqlite3WhereEnd(pWInfo);
        };
    }
}
pub const SQLITE_CountRows: u64_0 = (0x1 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const COLFLAG_PRIMKEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const COLFLAG_VIRTUAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const COLFLAG_GENERATED: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const SQLITE_AFF_REAL: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;
pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const TF_HasGenerated: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TABTYP_VIEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Replace: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const OE_Default: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const WHERE_ONEPASS_DESIRED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const WHERE_ONEPASS_MULTIROW: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const NC_UUpsert: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SF_IncludeHidden: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const SF_UFSrcCheck: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
pub const SF_OrderByReqd: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const SF_UpdateFrom: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SRT_Table: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SRT_Upfrom: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const OPFLAG_NOCHNG: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPFLAG_ISUPDATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const OPFLAG_ISNOOP: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const OPFLAG_SAVEPOSITION: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const TRIGGER_BEFORE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TRIGGER_AFTER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ONEPASS_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ONEPASS_SINGLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ONEPASS_MULTI: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TK_ROW: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const TK_UPDATE: ::core::ffi::c_int = 130 as ::core::ffi::c_int;
