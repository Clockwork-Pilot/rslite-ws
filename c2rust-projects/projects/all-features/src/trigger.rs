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
    fn sqlite3VdbeAddParseSchemaOp(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: u16_0,
    );
    fn sqlite3VdbeChangeP5(_: *mut Vdbe, P5: u16_0);
    fn sqlite3VdbeChangeP4(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
        zP4: *const ::core::ffi::c_char,
        N: ::core::ffi::c_int,
    );
    fn sqlite3VdbeMakeLabel(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3VdbeDelete(_: *mut Vdbe);
    fn sqlite3VdbeResolveLabel(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeTakeOpArray(
        _: *mut Vdbe,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> *mut VdbeOp;
    fn sqlite3VdbeLinkSubProgram(_: *mut Vdbe, _: *mut SubProgram);
    fn sqlite3WalkExprList(_: *mut Walker, _: *mut ExprList) -> ::core::ffi::c_int;
    fn sqlite3ExprWalkNoop(_: *mut Walker, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3SelectWalkNoop(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
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
    fn sqlite3DbSpanDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3Dequote(_: *mut ::core::ffi::c_char);
    fn sqlite3TokenInit(_: *mut Token, _: *mut ::core::ffi::c_char);
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
    fn sqlite3GenerateColumnNames(pParse: *mut Parse, pSelect: *mut Select);
    fn sqlite3Insert(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Select,
        _: *mut IdList,
        _: ::core::ffi::c_int,
        _: *mut Upsert,
    );
    fn sqlite3IdListIndex(
        _: *mut IdList,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3SrcListAppendList(
        pParse: *mut Parse,
        p1: *mut SrcList,
        p2: *mut SrcList,
    ) -> *mut SrcList;
    fn sqlite3SrcListAppend(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Token,
        _: *mut Token,
    ) -> *mut SrcList;
    fn sqlite3SrcListAppendFromTerm(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Token,
        _: *mut Token,
        _: *mut Token,
        _: *mut Select,
        _: *mut OnOrUsing,
    ) -> *mut SrcList;
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
    fn sqlite3DeleteFrom(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Expr,
        _: *mut ExprList,
        _: *mut Expr,
    );
    fn sqlite3Update(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut ExprList,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: *mut ExprList,
        _: *mut Expr,
        _: *mut Upsert,
    );
    fn sqlite3ExprCodeFactorable(_: *mut Parse, _: *mut Expr, _: ::core::ffi::c_int);
    fn sqlite3ExprIfFalse(
        _: *mut Parse,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3NameFromToken(
        _: *mut sqlite3,
        _: *const Token,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3GetVdbe(_: *mut Parse) -> *mut Vdbe;
    fn sqlite3CodeVerifySchema(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3CodeVerifyNamedSchema(_: *mut Parse, zDb: *const ::core::ffi::c_char);
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
    fn sqlite3IdListDup(_: *mut sqlite3, _: *const IdList) -> *mut IdList;
    fn sqlite3SelectDup(
        _: *mut sqlite3,
        _: *const Select,
        _: ::core::ffi::c_int,
    ) -> *mut Select;
    fn sqlite3ChangeCookie(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3AuthCheck(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3DbIsNamed(
        db: *mut sqlite3,
        iDb: ::core::ffi::c_int,
        zName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3FixInit(
        _: *mut DbFixer,
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const Token,
    );
    fn sqlite3FixSrcList(_: *mut DbFixer, _: *mut SrcList) -> ::core::ffi::c_int;
    fn sqlite3FixExpr(_: *mut DbFixer, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3FixTriggerStep(_: *mut DbFixer, _: *mut TriggerStep) -> ::core::ffi::c_int;
    fn sqlite3ExprAffinity(pExpr: *const Expr) -> ::core::ffi::c_char;
    fn sqlite3TwoPartName(
        _: *mut Parse,
        _: *mut Token,
        _: *mut Token,
        _: *mut *mut Token,
    ) -> ::core::ffi::c_int;
    fn sqlite3ReadSchema(pParse: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3CheckObjectName(
        _: *mut Parse,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    fn sqlite3NestedParse(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3SelectPrep(_: *mut Parse, _: *mut Select, _: *mut NameContext);
    fn sqlite3ResolveExprNames(_: *mut NameContext, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3ResolveExprListNames(
        _: *mut NameContext,
        _: *mut ExprList,
    ) -> ::core::ffi::c_int;
    fn sqlite3RenameTokenMap(
        _: *mut Parse,
        _: *const ::core::ffi::c_void,
        _: *const Token,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3RenameTokenRemap(
        _: *mut Parse,
        pTo: *const ::core::ffi::c_void,
        pFrom: *const ::core::ffi::c_void,
    );
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
    fn sqlite3HasExplicitNulls(_: *mut Parse, _: *mut ExprList) -> ::core::ffi::c_int;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3SelectDestInit(
        _: *mut SelectDest,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ReadOnlyShadowTables(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3ShadowTableName(
        db: *mut sqlite3,
        zName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ParseObjectInit(_: *mut Parse, _: *mut sqlite3);
    fn sqlite3ParseObjectReset(_: *mut Parse);
    fn sqlite3UpsertDelete(_: *mut sqlite3, _: *mut Upsert);
    fn sqlite3UpsertDup(_: *mut sqlite3, _: *mut Upsert) -> *mut Upsert;
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
pub struct OnOrUsing {
    pub pOn: *mut Expr,
    pub pUsing: *mut IdList

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
pub union C2Rust_Unnamed_24 {
    pub sSrc: SrcList,
    pub fromSpace: [u8_0; 80]

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CREATE_TEMP_TRIGGER: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_CREATE_TRIGGER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_DELETE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_DROP_TEMP_TRIGGER: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SQLITE_DROP_TRIGGER: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQLITE_INSERT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const OP_Program: ::core::ffi::c_int = 49 as ::core::ffi::c_int;
pub const OP_Halt: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
pub const OP_RealAffinity: ::core::ffi::c_int = 88 as ::core::ffi::c_int;
pub const OP_MakeRecord: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const OP_NewRowid: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const OP_Insert: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const OP_ResetCount: ::core::ffi::c_int = 132 as ::core::ffi::c_int;
pub const OP_DropTrigger: ::core::ffi::c_int = 155 as ::core::ffi::c_int;
pub const OP_Trace: ::core::ffi::c_int = 185 as ::core::ffi::c_int;
pub const OMIT_TEMPDB: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LEGACY_SCHEMA_TABLE: [::core::ffi::c_char; 14] = unsafe {
    ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"sqlite_master\0")
};
pub const LEGACY_TEMP_SCHEMA_TABLE: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<
        [u8; 19],
        [::core::ffi::c_char; 19],
    >(*b"sqlite_temp_master\0")
};
pub const P4_SUBPROGRAM: ::core::ffi::c_int = -4 as ::core::ffi::c_int;
pub const P4_DYNAMIC: ::core::ffi::c_int = -6 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3DeleteTriggerStep(
    mut db: *mut sqlite3,
    mut pTriggerStep: *mut TriggerStep,
) {
    unsafe {
        while !pTriggerStep.is_null() {
            let mut pTmp: *mut TriggerStep = pTriggerStep;
            pTriggerStep = (*pTriggerStep).pNext;
            sqlite3ExprDelete(db, (*pTmp).pWhere);
            sqlite3ExprListDelete(db, (*pTmp).pExprList);
            sqlite3SelectDelete(db, (*pTmp).pSelect);
            sqlite3IdListDelete(db, (*pTmp).pIdList);
            sqlite3UpsertDelete(db, (*pTmp).pUpsert);
            sqlite3SrcListDelete(db, (*pTmp).pFrom);
            sqlite3DbFree(db, (*pTmp).zSpan as *mut ::core::ffi::c_void);
            sqlite3DbFree(db, pTmp as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TriggerList(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
) -> *mut Trigger {
    unsafe {
        let mut pTmpSchema: *mut Schema = ::core::ptr::null_mut::<Schema>();
        let mut pList: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        let mut p: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        pTmpSchema = (*(*(*pParse).db).aDb.offset(1 as ::core::ffi::c_int as isize))
            .pSchema;
        p = (*pTmpSchema).trigHash.first;
        pList = (*pTab).pTrigger;
        while !p.is_null() {
            let mut pTrig: *mut Trigger = (*p).data as *mut Trigger;
            if (*pTrig).pTabSchema == (*pTab).pSchema && !(*pTrig).table.is_null()
                && 0 as ::core::ffi::c_int
                    == sqlite3StrICmp((*pTrig).table, (*pTab).zName)
                && ((*pTrig).pTabSchema != pTmpSchema
                    || (*pTrig).bReturning as ::core::ffi::c_int != 0)
            {
                (*pTrig).pNext = pList;
                pList = pTrig;
            } else if (*pTrig).op as ::core::ffi::c_int == TK_RETURNING {
                (*pTrig).table = (*pTab).zName;
                (*pTrig).pTabSchema = (*pTab).pSchema;
                (*pTrig).pNext = pList;
                pList = pTrig;
            }
            p = (*p).next;
        }
        return pList;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BeginTrigger(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
    mut tr_tm: ::core::ffi::c_int,
    mut op: ::core::ffi::c_int,
    mut pColumns: *mut IdList,
    mut pTableName: *mut SrcList,
    mut pWhen: *mut Expr,
    mut isTemp: ::core::ffi::c_int,
    mut noErr: ::core::ffi::c_int,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pTrigger: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut iDb: ::core::ffi::c_int = 0;
        let mut pName: *mut Token = ::core::ptr::null_mut::<Token>();
        let mut sFix: DbFixer = DbFixer {
            pParse: ::core::ptr::null_mut::<Parse>(),
            w: Walker {
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

            },
            pSchema: ::core::ptr::null_mut::<Schema>(),
            bTemp: 0,
            zDb: ::core::ptr::null::<::core::ffi::c_char>(),
            zType: ::core::ptr::null::<::core::ffi::c_char>(),
            pName: ::core::ptr::null::<Token>()

        };
        if isTemp != 0 {
            if (*pName2).n > 0 as ::core::ffi::c_uint {
                sqlite3ErrorMsg(
                    pParse,
                    b"temporary trigger may not have qualified name\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                c2rust_current_block = 17045246593118011163;
            } else {
                iDb = 1 as ::core::ffi::c_int;
                pName = pName1;
                c2rust_current_block = 1841672684692190573;
            }
        } else {
            iDb = sqlite3TwoPartName(pParse, pName1, pName2, &raw mut pName);
            if iDb < 0 as ::core::ffi::c_int {
                c2rust_current_block = 17045246593118011163;
            } else {
                c2rust_current_block = 1841672684692190573;
            }
        }
        match c2rust_current_block {
            1841672684692190573 => {
                if !(pTableName.is_null()
                    || (*db).mallocFailed as ::core::ffi::c_int != 0)
                {
                    if (*db).init.busy as ::core::ffi::c_int != 0
                        && iDb != 1 as ::core::ffi::c_int
                    {
                        sqlite3DbFree(
                            db,
                            (*(&raw mut (*pTableName).a as *mut SrcItem)
                                .offset(0 as ::core::ffi::c_int as isize))
                                .u4
                                .zDatabase as *mut ::core::ffi::c_void,
                        );
                        let ref mut c2rust_fresh0 = (*(&raw mut (*pTableName).a
                            as *mut SrcItem)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .u4
                            .zDatabase;
                        *c2rust_fresh0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    pTab = sqlite3SrcListLookup(pParse, pTableName);
                    if (*db).init.busy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && (*pName2).n == 0 as ::core::ffi::c_uint && !pTab.is_null()
                        && (*pTab).pSchema
                            == (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize))
                                .pSchema
                    {
                        iDb = 1 as ::core::ffi::c_int;
                    }
                    if !((*db).mallocFailed != 0) {
                        sqlite3FixInit(
                            &raw mut sFix,
                            pParse,
                            iDb,
                            b"trigger\0".as_ptr() as *const ::core::ffi::c_char,
                            pName,
                        );
                        if !(sqlite3FixSrcList(&raw mut sFix, pTableName) != 0) {
                            pTab = sqlite3SrcListLookup(pParse, pTableName);
                            if pTab.is_null() {
                                c2rust_current_block = 7919242485714060775;
                            } else if (*pTab).eTabType as ::core::ffi::c_int
                                == TABTYP_VTAB
                            {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"cannot create triggers on virtual tables\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                c2rust_current_block = 7919242485714060775;
                            } else if (*pTab).tabFlags & TF_Shadow as u32_0 != 0 as u32_0
                                && sqlite3ReadOnlyShadowTables(db) != 0
                            {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"cannot create triggers on shadow tables\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                c2rust_current_block = 7919242485714060775;
                            } else {
                                zName = sqlite3NameFromToken(db, pName);
                                if zName.is_null() {
                                    c2rust_current_block = 17045246593118011163;
                                } else if sqlite3CheckObjectName(
                                    pParse,
                                    zName,
                                    b"trigger\0".as_ptr() as *const ::core::ffi::c_char,
                                    (*pTab).zName,
                                ) != 0
                                {
                                    c2rust_current_block = 17045246593118011163;
                                } else {
                                    if !((*pParse).eParseMode as ::core::ffi::c_int
                                        >= PARSE_MODE_RENAME)
                                    {
                                        if !sqlite3HashFind(
                                                &raw mut (*(*(*db).aDb.offset(iDb as isize)).pSchema)
                                                    .trigHash,
                                                zName,
                                            )
                                            .is_null()
                                        {
                                            if noErr == 0 {
                                                sqlite3ErrorMsg(
                                                    pParse,
                                                    b"trigger %T already exists\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    pName,
                                                );
                                            } else {
                                                sqlite3CodeVerifySchema(pParse, iDb);
                                            }
                                            c2rust_current_block = 17045246593118011163;
                                        } else {
                                            c2rust_current_block = 15512526488502093901;
                                        }
                                    } else {
                                        c2rust_current_block = 15512526488502093901;
                                    }
                                    match c2rust_current_block {
                                        17045246593118011163 => {}
                                        _ => {
                                            if sqlite3_strnicmp(
                                                (*pTab).zName,
                                                b"sqlite_\0".as_ptr() as *const ::core::ffi::c_char,
                                                7 as ::core::ffi::c_int,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                sqlite3ErrorMsg(
                                                    pParse,
                                                    b"cannot create trigger on system table\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                c2rust_current_block = 17045246593118011163;
                                            } else if (*pTab).eTabType as ::core::ffi::c_int
                                                == TABTYP_VIEW && tr_tm != TK_INSTEAD
                                            {
                                                sqlite3ErrorMsg(
                                                    pParse,
                                                    b"cannot create %s trigger on view: %S\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    if tr_tm == TK_BEFORE {
                                                        b"BEFORE\0".as_ptr() as *const ::core::ffi::c_char
                                                    } else {
                                                        b"AFTER\0".as_ptr() as *const ::core::ffi::c_char
                                                    },
                                                    &raw mut (*pTableName).a as *mut SrcItem,
                                                );
                                                c2rust_current_block = 7919242485714060775;
                                            } else if !((*pTab).eTabType as ::core::ffi::c_int
                                                == TABTYP_VIEW) && tr_tm == TK_INSTEAD
                                            {
                                                sqlite3ErrorMsg(
                                                    pParse,
                                                    b"cannot create INSTEAD OF trigger on table: %S\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    &raw mut (*pTableName).a as *mut SrcItem,
                                                );
                                                c2rust_current_block = 7919242485714060775;
                                            } else {
                                                if !((*pParse).eParseMode as ::core::ffi::c_int
                                                    >= PARSE_MODE_RENAME)
                                                {
                                                    let mut iTabDb: ::core::ffi::c_int = sqlite3SchemaToIndex(
                                                        db,
                                                        (*pTab).pSchema,
                                                    );
                                                    let mut code: ::core::ffi::c_int = SQLITE_CREATE_TRIGGER;
                                                    let mut zDb: *const ::core::ffi::c_char = (*(*db)
                                                        .aDb
                                                        .offset(iTabDb as isize))
                                                        .zDbSName;
                                                    let mut zDbTrig: *const ::core::ffi::c_char = if isTemp != 0
                                                    {
                                                        (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize))
                                                            .zDbSName as *const ::core::ffi::c_char
                                                    } else {
                                                        zDb
                                                    };
                                                    if iTabDb == 1 as ::core::ffi::c_int || isTemp != 0 {
                                                        code = SQLITE_CREATE_TEMP_TRIGGER;
                                                    }
                                                    if sqlite3AuthCheck(
                                                        pParse,
                                                        code,
                                                        zName,
                                                        (*pTab).zName,
                                                        zDbTrig,
                                                    ) != 0
                                                    {
                                                        c2rust_current_block = 17045246593118011163;
                                                    } else if sqlite3AuthCheck(
                                                        pParse,
                                                        SQLITE_INSERT,
                                                        if OMIT_TEMPDB == 0 && iTabDb == 1 as ::core::ffi::c_int {
                                                            LEGACY_TEMP_SCHEMA_TABLE.as_ptr()
                                                        } else {
                                                            LEGACY_SCHEMA_TABLE.as_ptr()
                                                        },
                                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                                        zDb,
                                                    ) != 0
                                                    {
                                                        c2rust_current_block = 17045246593118011163;
                                                    } else {
                                                        c2rust_current_block = 7343950298149844727;
                                                    }
                                                } else {
                                                    c2rust_current_block = 7343950298149844727;
                                                }
                                                match c2rust_current_block {
                                                    17045246593118011163 => {}
                                                    _ => {
                                                        if tr_tm == TK_INSTEAD {
                                                            tr_tm = TK_BEFORE;
                                                        }
                                                        pTrigger = sqlite3DbMallocZero(
                                                            db,
                                                            ::core::mem::size_of::<Trigger>() as u64_0,
                                                        ) as *mut Trigger;
                                                        if pTrigger.is_null() {
                                                            c2rust_current_block = 17045246593118011163;
                                                        } else {
                                                            (*pTrigger).zName = zName;
                                                            zName = ::core::ptr::null_mut::<::core::ffi::c_char>();
                                                            (*pTrigger).table = sqlite3DbStrDup(
                                                                db,
                                                                (*(&raw mut (*pTableName).a as *mut SrcItem)
                                                                    .offset(0 as ::core::ffi::c_int as isize))
                                                                    .zName,
                                                            );
                                                            (*pTrigger).pSchema = (*(*db).aDb.offset(iDb as isize))
                                                                .pSchema;
                                                            (*pTrigger).pTabSchema = (*pTab).pSchema;
                                                            (*pTrigger).op = op as u8_0;
                                                            (*pTrigger).tr_tm = (if tr_tm == TK_BEFORE {
                                                                TRIGGER_BEFORE
                                                            } else {
                                                                TRIGGER_AFTER
                                                            }) as u8_0;
                                                            if (*pParse).eParseMode as ::core::ffi::c_int
                                                                >= PARSE_MODE_RENAME
                                                            {
                                                                sqlite3RenameTokenRemap(
                                                                    pParse,
                                                                    (*pTrigger).table as *const ::core::ffi::c_void,
                                                                    (*(&raw mut (*pTableName).a as *mut SrcItem)
                                                                        .offset(0 as ::core::ffi::c_int as isize))
                                                                        .zName as *const ::core::ffi::c_void,
                                                                );
                                                                (*pTrigger).pWhen = pWhen;
                                                                pWhen = ::core::ptr::null_mut::<Expr>();
                                                            } else {
                                                                (*pTrigger).pWhen = sqlite3ExprDup(
                                                                    db,
                                                                    pWhen,
                                                                    EXPRDUP_REDUCE,
                                                                );
                                                            }
                                                            (*pTrigger).pColumns = pColumns;
                                                            pColumns = ::core::ptr::null_mut::<IdList>();
                                                            (*pParse).pNewTrigger = pTrigger;
                                                            c2rust_current_block = 17045246593118011163;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            match c2rust_current_block {
                                17045246593118011163 => {}
                                _ => {
                                    if (*db).init.iDb as ::core::ffi::c_int
                                        == 1 as ::core::ffi::c_int
                                    {
                                        (*db)
                                            .init
                                            .set_orphanTrigger(
                                                1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                            );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        sqlite3DbFree(db, zName as *mut ::core::ffi::c_void);
        sqlite3SrcListDelete(db, pTableName);
        sqlite3IdListDelete(db, pColumns);
        sqlite3ExprDelete(db, pWhen);
        if (*pParse).pNewTrigger.is_null() {
            sqlite3DeleteTrigger(db, pTrigger);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FinishTrigger(
    mut pParse: *mut Parse,
    mut pStepList: *mut TriggerStep,
    mut pAll: *mut Token,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pTrig: *mut Trigger = (*pParse).pNewTrigger;
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut sFix: DbFixer = DbFixer {
            pParse: ::core::ptr::null_mut::<Parse>(),
            w: Walker {
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

            },
            pSchema: ::core::ptr::null_mut::<Schema>(),
            bTemp: 0,
            zDb: ::core::ptr::null::<::core::ffi::c_char>(),
            zType: ::core::ptr::null::<::core::ffi::c_char>(),
            pName: ::core::ptr::null::<Token>()

        };
        let mut iDb: ::core::ffi::c_int = 0;
        let mut nameToken: Token = Token {
            z: ::core::ptr::null::<::core::ffi::c_char>(),
            n: 0

        };
        (*pParse).pNewTrigger = ::core::ptr::null_mut::<Trigger>();
        if !((*pParse).nErr != 0 || pTrig.is_null()) {
            zName = (*pTrig).zName;
            iDb = sqlite3SchemaToIndex((*pParse).db, (*pTrig).pSchema);
            (*pTrig).step_list = pStepList;
            while !pStepList.is_null() {
                (*pStepList).pTrig = pTrig;
                pStepList = (*pStepList).pNext;
            }
            sqlite3TokenInit(&raw mut nameToken, (*pTrig).zName);
            sqlite3FixInit(
                &raw mut sFix,
                pParse,
                iDb,
                b"trigger\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut nameToken,
            );
            if !(sqlite3FixTriggerStep(&raw mut sFix, (*pTrig).step_list) != 0
                || sqlite3FixExpr(&raw mut sFix, (*pTrig).pWhen) != 0)
            {
                if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                    (*pParse).pNewTrigger = pTrig;
                    pTrig = ::core::ptr::null_mut::<Trigger>();
                    c2rust_current_block = 4761528863920922185;
                } else if (*db).init.busy == 0 {
                    let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
                    let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    if sqlite3ReadOnlyShadowTables(db) != 0 {
                        let mut pStep: *mut TriggerStep = ::core::ptr::null_mut::<
                            TriggerStep,
                        >();
                        pStep = (*pTrig).step_list;
                        loop {
                            if pStep.is_null() {
                                c2rust_current_block = 18317007320854588510;
                                break;
                            }
                            if !(*pStep).zTarget.is_null()
                                && sqlite3ShadowTableName(db, (*pStep).zTarget) != 0
                            {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"trigger \"%s\" may not write to shadow table \"%s\"\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*pTrig).zName,
                                    (*pStep).zTarget,
                                );
                                c2rust_current_block = 14476009087953683453;
                                break;
                            } else {
                                pStep = (*pStep).pNext;
                            }
                        }
                    } else {
                        c2rust_current_block = 18317007320854588510;
                    }
                    match c2rust_current_block {
                        14476009087953683453 => {}
                        _ => {
                            v = sqlite3GetVdbe(pParse);
                            if v.is_null() {
                                c2rust_current_block = 14476009087953683453;
                            } else {
                                sqlite3BeginWriteOperation(
                                    pParse,
                                    0 as ::core::ffi::c_int,
                                    iDb,
                                );
                                z = sqlite3DbStrNDup(
                                    db,
                                    (*pAll).z as *mut ::core::ffi::c_char,
                                    (*pAll).n as u64_0,
                                );
                                sqlite3NestedParse(
                                    pParse,
                                    b"INSERT INTO %Q.sqlite_master VALUES('trigger',%Q,%Q,0,'CREATE TRIGGER %q')\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*(*db).aDb.offset(iDb as isize)).zDbSName,
                                    zName,
                                    (*pTrig).table,
                                    z,
                                );
                                sqlite3DbFree(db, z as *mut ::core::ffi::c_void);
                                sqlite3ChangeCookie(pParse, iDb);
                                sqlite3VdbeAddParseSchemaOp(
                                    v,
                                    iDb,
                                    sqlite3MPrintf(
                                        db,
                                        b"type='trigger' AND name='%q'\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        zName,
                                    ),
                                    0 as u16_0,
                                );
                                c2rust_current_block = 4761528863920922185;
                            }
                        }
                    }
                } else {
                    c2rust_current_block = 4761528863920922185;
                }
                match c2rust_current_block {
                    14476009087953683453 => {}
                    _ => {
                        if (*db).init.busy != 0 {
                            let mut pLink: *mut Trigger = pTrig;
                            let mut pHash: *mut Hash = &raw mut (*(*(*db)
                                .aDb
                                .offset(iDb as isize))
                                .pSchema)
                                .trigHash;
                            pTrig = sqlite3HashInsert(
                                pHash,
                                zName,
                                pTrig as *mut ::core::ffi::c_void,
                            ) as *mut Trigger;
                            if !pTrig.is_null() {
                                sqlite3OomFault(db);
                            } else if (*pLink).pSchema == (*pLink).pTabSchema {
                                let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
                                pTab = sqlite3HashFind(
                                    &raw mut (*(*pLink).pTabSchema).tblHash,
                                    (*pLink).table,
                                ) as *mut Table;
                                (*pLink).pNext = (*pTab).pTrigger;
                                (*pTab).pTrigger = pLink;
                            }
                        }
                    }
                }
            }
        }
        sqlite3DeleteTrigger(db, pTrig);
        sqlite3DeleteTriggerStep(db, pStepList);
    }
}
unsafe extern "C" fn triggerSpanDup(
    mut db: *mut sqlite3,
    mut zStart: *const ::core::ffi::c_char,
    mut zEnd: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = sqlite3DbSpanDup(db, zStart, zEnd);
        let mut i: ::core::ffi::c_int = 0;
        if !z.is_null() {
            i = 0 as ::core::ffi::c_int;
            while *z.offset(i as isize) != 0 {
                if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(*z.offset(i as isize) as ::core::ffi::c_uchar as isize)
                    as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
                {
                    *z.offset(i as isize) = ' ' as i32 as ::core::ffi::c_char;
                }
                i += 1;
            }
        }
        return z;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TriggerSelectStep(
    mut db: *mut sqlite3,
    mut pSelect: *mut Select,
    mut zStart: *const ::core::ffi::c_char,
    mut zEnd: *const ::core::ffi::c_char,
) -> *mut TriggerStep {
    unsafe {
        let mut pTriggerStep: *mut TriggerStep = sqlite3DbMallocZero(
            db,
            ::core::mem::size_of::<TriggerStep>() as u64_0,
        ) as *mut TriggerStep;
        if pTriggerStep.is_null() {
            sqlite3SelectDelete(db, pSelect);
            return ::core::ptr::null_mut::<TriggerStep>();
        }
        (*pTriggerStep).op = TK_SELECT as u8_0;
        (*pTriggerStep).pSelect = pSelect;
        (*pTriggerStep).orconf = OE_Default as u8_0;
        (*pTriggerStep).zSpan = triggerSpanDup(db, zStart, zEnd);
        return pTriggerStep;
    }
}
unsafe extern "C" fn triggerStepAllocate(
    mut pParse: *mut Parse,
    mut op: u8_0,
    mut pName: *mut Token,
    mut zStart: *const ::core::ffi::c_char,
    mut zEnd: *const ::core::ffi::c_char,
) -> *mut TriggerStep {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pTriggerStep: *mut TriggerStep = ::core::ptr::null_mut::<TriggerStep>();
        if (*pParse).nErr != 0 {
            return ::core::ptr::null_mut::<TriggerStep>();
        }
        pTriggerStep = sqlite3DbMallocZero(
            db,
            (::core::mem::size_of::<TriggerStep>() as usize)
                .wrapping_add((*pName).n as usize)
                .wrapping_add(1 as usize) as u64_0,
        ) as *mut TriggerStep;
        if !pTriggerStep.is_null() {
            let mut z: *mut ::core::ffi::c_char = pTriggerStep
                .offset(1 as ::core::ffi::c_int as isize) as *mut TriggerStep
                as *mut ::core::ffi::c_char;
            memcpy(
                z as *mut ::core::ffi::c_void,
                (*pName).z as *const ::core::ffi::c_void,
                (*pName).n as size_t,
            );
            sqlite3Dequote(z);
            (*pTriggerStep).zTarget = z;
            (*pTriggerStep).op = op;
            (*pTriggerStep).zSpan = triggerSpanDup(db, zStart, zEnd);
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                sqlite3RenameTokenMap(
                    pParse,
                    (*pTriggerStep).zTarget as *const ::core::ffi::c_void,
                    pName,
                );
            }
        }
        return pTriggerStep;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TriggerInsertStep(
    mut pParse: *mut Parse,
    mut pTableName: *mut Token,
    mut pColumn: *mut IdList,
    mut pSelect: *mut Select,
    mut orconf: u8_0,
    mut pUpsert: *mut Upsert,
    mut zStart: *const ::core::ffi::c_char,
    mut zEnd: *const ::core::ffi::c_char,
) -> *mut TriggerStep {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pTriggerStep: *mut TriggerStep = ::core::ptr::null_mut::<TriggerStep>();
        pTriggerStep = triggerStepAllocate(
            pParse,
            TK_INSERT as u8_0,
            pTableName,
            zStart,
            zEnd,
        );
        if !pTriggerStep.is_null() {
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                (*pTriggerStep).pSelect = pSelect;
                pSelect = ::core::ptr::null_mut::<Select>();
            } else {
                (*pTriggerStep).pSelect = sqlite3SelectDup(db, pSelect, EXPRDUP_REDUCE);
            }
            (*pTriggerStep).pIdList = pColumn;
            (*pTriggerStep).pUpsert = pUpsert;
            (*pTriggerStep).orconf = orconf;
            if !pUpsert.is_null() {
                sqlite3HasExplicitNulls(pParse, (*pUpsert).pUpsertTarget);
            }
        } else {
            sqlite3IdListDelete(db, pColumn);
            sqlite3UpsertDelete(db, pUpsert);
        }
        sqlite3SelectDelete(db, pSelect);
        return pTriggerStep;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TriggerUpdateStep(
    mut pParse: *mut Parse,
    mut pTableName: *mut Token,
    mut pFrom: *mut SrcList,
    mut pEList: *mut ExprList,
    mut pWhere: *mut Expr,
    mut orconf: u8_0,
    mut zStart: *const ::core::ffi::c_char,
    mut zEnd: *const ::core::ffi::c_char,
) -> *mut TriggerStep {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pTriggerStep: *mut TriggerStep = ::core::ptr::null_mut::<TriggerStep>();
        pTriggerStep = triggerStepAllocate(
            pParse,
            TK_UPDATE as u8_0,
            pTableName,
            zStart,
            zEnd,
        );
        if !pTriggerStep.is_null() {
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                (*pTriggerStep).pExprList = pEList;
                (*pTriggerStep).pWhere = pWhere;
                (*pTriggerStep).pFrom = pFrom;
                pEList = ::core::ptr::null_mut::<ExprList>();
                pWhere = ::core::ptr::null_mut::<Expr>();
                pFrom = ::core::ptr::null_mut::<SrcList>();
            } else {
                (*pTriggerStep).pExprList = sqlite3ExprListDup(
                    db,
                    pEList,
                    EXPRDUP_REDUCE,
                );
                (*pTriggerStep).pWhere = sqlite3ExprDup(db, pWhere, EXPRDUP_REDUCE);
                (*pTriggerStep).pFrom = sqlite3SrcListDup(db, pFrom, EXPRDUP_REDUCE);
            }
            (*pTriggerStep).orconf = orconf;
        }
        sqlite3ExprListDelete(db, pEList);
        sqlite3ExprDelete(db, pWhere);
        sqlite3SrcListDelete(db, pFrom);
        return pTriggerStep;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TriggerDeleteStep(
    mut pParse: *mut Parse,
    mut pTableName: *mut Token,
    mut pWhere: *mut Expr,
    mut zStart: *const ::core::ffi::c_char,
    mut zEnd: *const ::core::ffi::c_char,
) -> *mut TriggerStep {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pTriggerStep: *mut TriggerStep = ::core::ptr::null_mut::<TriggerStep>();
        pTriggerStep = triggerStepAllocate(
            pParse,
            TK_DELETE as u8_0,
            pTableName,
            zStart,
            zEnd,
        );
        if !pTriggerStep.is_null() {
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                (*pTriggerStep).pWhere = pWhere;
                pWhere = ::core::ptr::null_mut::<Expr>();
            } else {
                (*pTriggerStep).pWhere = sqlite3ExprDup(db, pWhere, EXPRDUP_REDUCE);
            }
            (*pTriggerStep).orconf = OE_Default as u8_0;
        }
        sqlite3ExprDelete(db, pWhere);
        return pTriggerStep;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DeleteTrigger(
    mut db: *mut sqlite3,
    mut pTrigger: *mut Trigger,
) {
    unsafe {
        if pTrigger.is_null() || (*pTrigger).bReturning as ::core::ffi::c_int != 0 {
            return;
        }
        sqlite3DeleteTriggerStep(db, (*pTrigger).step_list);
        sqlite3DbFree(db, (*pTrigger).zName as *mut ::core::ffi::c_void);
        sqlite3DbFree(db, (*pTrigger).table as *mut ::core::ffi::c_void);
        sqlite3ExprDelete(db, (*pTrigger).pWhen);
        sqlite3IdListDelete(db, (*pTrigger).pColumns);
        sqlite3DbFree(db, pTrigger as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DropTrigger(
    mut pParse: *mut Parse,
    mut pName: *mut SrcList,
    mut noErr: ::core::ffi::c_int,
) {
    unsafe {
        let mut pTrigger: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        let mut i: ::core::ffi::c_int = 0;
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = (*pParse).db;
        if !((*db).mallocFailed != 0) {
            if !(SQLITE_OK != sqlite3ReadSchema(pParse)) {
                zDb = (*(&raw mut (*pName).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .u4
                    .zDatabase;
                zName = (*(&raw mut (*pName).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .zName;
                i = OMIT_TEMPDB;
                while i < (*db).nDb {
                    let mut j: ::core::ffi::c_int = if i < 2 as ::core::ffi::c_int {
                        i ^ 1 as ::core::ffi::c_int
                    } else {
                        i
                    };
                    if !(!zDb.is_null()
                        && sqlite3DbIsNamed(db, j, zDb) == 0 as ::core::ffi::c_int)
                    {
                        pTrigger = sqlite3HashFind(
                            &raw mut (*(*(*db).aDb.offset(j as isize)).pSchema).trigHash,
                            zName,
                        ) as *mut Trigger;
                        if !pTrigger.is_null() {
                            break;
                        }
                    }
                    i += 1;
                }
                if pTrigger.is_null() {
                    if noErr == 0 {
                        sqlite3ErrorMsg(
                            pParse,
                            b"no such trigger: %S\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*pName).a as *mut SrcItem,
                        );
                    } else {
                        sqlite3CodeVerifyNamedSchema(pParse, zDb);
                    }
                    (*pParse).set_checkSchema(1 as bft as bft);
                } else {
                    sqlite3DropTriggerPtr(pParse, pTrigger);
                }
            }
        }
        sqlite3SrcListDelete(db, pName);
    }
}
unsafe extern "C" fn tableOfTrigger(mut pTrigger: *mut Trigger) -> *mut Table {
    unsafe {
        return sqlite3HashFind(
            &raw mut (*(*pTrigger).pTabSchema).tblHash,
            (*pTrigger).table,
        ) as *mut Table;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DropTriggerPtr(
    mut pParse: *mut Parse,
    mut pTrigger: *mut Trigger,
) {
    unsafe {
        let mut pTable: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut iDb: ::core::ffi::c_int = 0;
        iDb = sqlite3SchemaToIndex((*pParse).db, (*pTrigger).pSchema);
        pTable = tableOfTrigger(pTrigger);
        if !pTable.is_null() {
            let mut code: ::core::ffi::c_int = SQLITE_DROP_TRIGGER;
            let mut zDb: *const ::core::ffi::c_char = (*(*db).aDb.offset(iDb as isize))
                .zDbSName;
            let mut zTab: *const ::core::ffi::c_char = if OMIT_TEMPDB == 0
                && iDb == 1 as ::core::ffi::c_int
            {
                LEGACY_TEMP_SCHEMA_TABLE.as_ptr()
            } else {
                LEGACY_SCHEMA_TABLE.as_ptr()
            };
            if iDb == 1 as ::core::ffi::c_int {
                code = SQLITE_DROP_TEMP_TRIGGER;
            }
            if sqlite3AuthCheck(pParse, code, (*pTrigger).zName, (*pTable).zName, zDb)
                != 0
                || sqlite3AuthCheck(
                    pParse,
                    SQLITE_DELETE,
                    zTab,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    zDb,
                ) != 0
            {
                return;
            }
        }
        v = sqlite3GetVdbe(pParse);
        if !v.is_null() {
            sqlite3NestedParse(
                pParse,
                b"DELETE FROM %Q.sqlite_master WHERE name=%Q AND type='trigger'\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*(*db).aDb.offset(iDb as isize)).zDbSName,
                (*pTrigger).zName,
            );
            sqlite3ChangeCookie(pParse, iDb);
            sqlite3VdbeAddOp4(
                v,
                OP_DropTrigger,
                iDb,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                (*pTrigger).zName,
                0 as ::core::ffi::c_int,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3UnlinkAndDeleteTrigger(
    mut db: *mut sqlite3,
    mut iDb: ::core::ffi::c_int,
    mut zName: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut pTrigger: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        let mut pHash: *mut Hash = ::core::ptr::null_mut::<Hash>();
        pHash = &raw mut (*(*(*db).aDb.offset(iDb as isize)).pSchema).trigHash;
        pTrigger = sqlite3HashInsert(
            pHash,
            zName,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut Trigger;
        if !pTrigger.is_null() {
            if (*pTrigger).pSchema == (*pTrigger).pTabSchema {
                let mut pTab: *mut Table = tableOfTrigger(pTrigger);
                if !pTab.is_null() {
                    let mut pp: *mut *mut Trigger = ::core::ptr::null_mut::<
                        *mut Trigger,
                    >();
                    pp = &raw mut (*pTab).pTrigger;
                    while !(*pp).is_null() {
                        if *pp == pTrigger {
                            *pp = (**pp).pNext;
                            break;
                        } else {
                            pp = &raw mut (**pp).pNext;
                        }
                    }
                }
            }
            sqlite3DeleteTrigger(db, pTrigger);
            (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
                | DBFLAG_SchemaChange as ::core::ffi::c_uint) as u32_0;
        }
    }
}
unsafe extern "C" fn checkColumnOverlap(
    mut pIdList: *mut IdList,
    mut pEList: *mut ExprList,
) -> ::core::ffi::c_int {
    unsafe {
        let mut e: ::core::ffi::c_int = 0;
        if pIdList.is_null() || pEList.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        e = 0 as ::core::ffi::c_int;
        while e < (*pEList).nExpr {
            if sqlite3IdListIndex(
                pIdList,
                (*(&raw mut (*pEList).a as *mut ExprList_item).offset(e as isize)).zEName,
            ) >= 0 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            e += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn tempTriggersExist(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        if (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pSchema.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pSchema)
            .trigHash
            .first
            .is_null()
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn triggersReallyExist(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut op: ::core::ffi::c_int,
    mut pChanges: *mut ExprList,
    mut pMask: *mut ::core::ffi::c_int,
) -> *mut Trigger {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut mask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pList: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        let mut p: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        pList = sqlite3TriggerList(pParse, pTab);
        if !pList.is_null() {
            p = pList;
            if (*(*pParse).db).flags as ::core::ffi::c_ulonglong
                & SQLITE_EnableTrigger as ::core::ffi::c_ulonglong
                == 0 as ::core::ffi::c_ulonglong && !(*pTab).pTrigger.is_null()
            {
                if pList == (*pTab).pTrigger {
                    pList = ::core::ptr::null_mut::<Trigger>();
                    c2rust_current_block = 4356140245254519547;
                } else {
                    while !(*p).pNext.is_null() && (*p).pNext != (*pTab).pTrigger {
                        p = (*p).pNext;
                    }
                    (*p).pNext = ::core::ptr::null_mut::<Trigger>();
                    p = pList;
                    c2rust_current_block = 3640593987805443782;
                }
            } else {
                c2rust_current_block = 3640593987805443782;
            }
            match c2rust_current_block {
                4356140245254519547 => {}
                _ => {
                    loop {
                        if (*p).op as ::core::ffi::c_int == op
                            && checkColumnOverlap((*p).pColumns, pChanges) != 0
                        {
                            mask |= (*p).tr_tm as ::core::ffi::c_int;
                        } else if (*p).op as ::core::ffi::c_int == TK_RETURNING {
                            (*p).op = op as u8_0;
                            if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                                if op != TK_INSERT {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        b"%s RETURNING is not available on virtual tables\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        if op == TK_DELETE {
                                            b"DELETE\0".as_ptr() as *const ::core::ffi::c_char
                                        } else {
                                            b"UPDATE\0".as_ptr() as *const ::core::ffi::c_char
                                        },
                                    );
                                }
                                (*p).tr_tm = TRIGGER_BEFORE as u8_0;
                            } else {
                                (*p).tr_tm = TRIGGER_AFTER as u8_0;
                            }
                            mask |= (*p).tr_tm as ::core::ffi::c_int;
                        } else if (*p).bReturning as ::core::ffi::c_int != 0
                            && (*p).op as ::core::ffi::c_int == TK_INSERT
                            && op == TK_UPDATE && (*pParse).pToplevel.is_null()
                        {
                            mask |= (*p).tr_tm as ::core::ffi::c_int;
                        }
                        p = (*p).pNext;
                        if p.is_null() {
                            break;
                        }
                    }
                }
            }
        }
        if !pMask.is_null() {
            *pMask = mask;
        }
        return if mask != 0 { pList } else { ::core::ptr::null_mut::<Trigger>() };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TriggersExist(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut op: ::core::ffi::c_int,
    mut pChanges: *mut ExprList,
    mut pMask: *mut ::core::ffi::c_int,
) -> *mut Trigger {
    unsafe {
        if (*pTab).pTrigger.is_null() && tempTriggersExist((*pParse).db) == 0
            || (*pParse).disableTriggers as ::core::ffi::c_int != 0
        {
            if !pMask.is_null() {
                *pMask = 0 as ::core::ffi::c_int;
            }
            return ::core::ptr::null_mut::<Trigger>();
        }
        return triggersReallyExist(pParse, pTab, op, pChanges, pMask);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TriggerStepSrc(
    mut pParse: *mut Parse,
    mut pStep: *mut TriggerStep,
) -> *mut SrcList {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pSrc: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut zName: *mut ::core::ffi::c_char = sqlite3DbStrDup(db, (*pStep).zTarget);
        pSrc = sqlite3SrcListAppend(
            pParse,
            ::core::ptr::null_mut::<SrcList>(),
            ::core::ptr::null_mut::<Token>(),
            ::core::ptr::null_mut::<Token>(),
        );
        if !pSrc.is_null() {
            let mut pSchema: *mut Schema = (*(*pStep).pTrig).pSchema;
            let ref mut c2rust_fresh4 = (*(&raw mut (*pSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .zName;
            *c2rust_fresh4 = zName;
            if pSchema != (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pSchema {
                let ref mut c2rust_fresh5 = (*(&raw mut (*pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .u4
                    .pSchema;
                *c2rust_fresh5 = pSchema;
                let ref mut c2rust_fresh6 = (*(&raw mut (*pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .fg;
                (*c2rust_fresh6)
                    .set_fixedSchema(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            if !(*pStep).pFrom.is_null() {
                let mut pDup: *mut SrcList = sqlite3SrcListDup(
                    db,
                    (*pStep).pFrom,
                    0 as ::core::ffi::c_int,
                );
                if !pDup.is_null() && (*pDup).nSrc > 1 as ::core::ffi::c_int
                    && !((*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME)
                {
                    let mut pSubquery: *mut Select = ::core::ptr::null_mut::<Select>();
                    let mut as_0: Token = Token {
                        z: ::core::ptr::null::<::core::ffi::c_char>(),
                        n: 0

                    };
                    pSubquery = sqlite3SelectNew(
                        pParse,
                        ::core::ptr::null_mut::<ExprList>(),
                        pDup,
                        ::core::ptr::null_mut::<Expr>(),
                        ::core::ptr::null_mut::<ExprList>(),
                        ::core::ptr::null_mut::<Expr>(),
                        ::core::ptr::null_mut::<ExprList>(),
                        SF_NestedFrom as u32_0,
                        ::core::ptr::null_mut::<Expr>(),
                    );
                    as_0.n = 0 as ::core::ffi::c_uint;
                    as_0.z = ::core::ptr::null::<::core::ffi::c_char>();
                    pDup = sqlite3SrcListAppendFromTerm(
                        pParse,
                        ::core::ptr::null_mut::<SrcList>(),
                        ::core::ptr::null_mut::<Token>(),
                        ::core::ptr::null_mut::<Token>(),
                        &raw mut as_0,
                        pSubquery,
                        ::core::ptr::null_mut::<OnOrUsing>(),
                    );
                }
                pSrc = sqlite3SrcListAppendList(pParse, pSrc, pDup);
            }
        } else {
            sqlite3DbFree(db, zName as *mut ::core::ffi::c_void);
        }
        return pSrc;
    }
}
unsafe extern "C" fn isAsteriskTerm(
    mut pParse: *mut Parse,
    mut pTerm: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pTerm).op as ::core::ffi::c_int == TK_ASTERISK {
            return 1 as ::core::ffi::c_int;
        }
        if (*pTerm).op as ::core::ffi::c_int != TK_DOT {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*pTerm).pRight).op as ::core::ffi::c_int != TK_ASTERISK {
            return 0 as ::core::ffi::c_int;
        }
        sqlite3ErrorMsg(
            pParse,
            b"RETURNING may not use \"TABLE.*\" wildcards\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn sqlite3ExpandReturning(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut pTab: *mut Table,
) -> *mut ExprList {
    unsafe {
        let mut pNew: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pList).nExpr {
            let mut pOldExpr: *mut Expr = (*(&raw mut (*pList).a as *mut ExprList_item)
                .offset(i as isize))
                .pExpr;
            if !pOldExpr.is_null() {
                if isAsteriskTerm(pParse, pOldExpr) != 0 {
                    let mut jj: ::core::ffi::c_int = 0;
                    jj = 0 as ::core::ffi::c_int;
                    while jj < (*pTab).nCol as ::core::ffi::c_int {
                        let mut pNewExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
                        if !((*(*pTab).aCol.offset(jj as isize)).colFlags
                            as ::core::ffi::c_int & COLFLAG_HIDDEN
                            != 0 as ::core::ffi::c_int)
                        {
                            pNewExpr = sqlite3Expr(
                                db,
                                TK_ID,
                                (*(*pTab).aCol.offset(jj as isize)).zCnName,
                            );
                            pNew = sqlite3ExprListAppend(pParse, pNew, pNewExpr);
                            if (*db).mallocFailed == 0 {
                                let mut pItem: *mut ExprList_item = (&raw mut (*pNew).a
                                    as *mut ExprList_item)
                                    .offset(((*pNew).nExpr - 1 as ::core::ffi::c_int) as isize)
                                    as *mut ExprList_item;
                                (*pItem).zEName = sqlite3DbStrDup(
                                    db,
                                    (*(*pTab).aCol.offset(jj as isize)).zCnName,
                                );
                                (*pItem)
                                    .fg
                                    .set_eEName(
                                        ENAME_NAME as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                    );
                            }
                        }
                        jj += 1;
                    }
                } else {
                    let mut pNewExpr_0: *mut Expr = sqlite3ExprDup(
                        db,
                        pOldExpr,
                        0 as ::core::ffi::c_int,
                    );
                    pNew = sqlite3ExprListAppend(pParse, pNew, pNewExpr_0);
                    if (*db).mallocFailed == 0
                        && !(*(&raw mut (*pList).a as *mut ExprList_item)
                            .offset(i as isize))
                            .zEName
                            .is_null()
                    {
                        let mut pItem_0: *mut ExprList_item = (&raw mut (*pNew).a
                            as *mut ExprList_item)
                            .offset(((*pNew).nExpr - 1 as ::core::ffi::c_int) as isize)
                            as *mut ExprList_item;
                        (*pItem_0).zEName = sqlite3DbStrDup(
                            db,
                            (*(&raw mut (*pList).a as *mut ExprList_item)
                                .offset(i as isize))
                                .zEName,
                        );
                        (*pItem_0)
                            .fg
                            .set_eEName(
                                (*(&raw mut (*pList).a as *mut ExprList_item)
                                    .offset(i as isize))
                                    .fg
                                    .eEName() as ::core::ffi::c_uint,
                            );
                    }
                }
            }
            i += 1;
        }
        return pNew;
    }
}
unsafe extern "C" fn sqlite3ReturningSubqueryVarSelect(
    mut NotUsed: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).flags & EP_xIsSelect as u32_0 != 0 as u32_0
            && (*(*pExpr).x.pSelect).selFlags & SF_Correlated as u32_0 != 0 as u32_0
        {
            (*pExpr).flags = ((*pExpr).flags as ::core::ffi::c_uint
                | 0x40 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint) as u32_0;
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn sqlite3ReturningSubqueryCorrelated(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pSrc: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        pSrc = (*pSelect).pSrc;
        i = 0 as ::core::ffi::c_int;
        while i < (*pSrc).nSrc {
            if (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize)).pSTab
                == (*pWalker).u.pTab
            {
                (*pSelect).selFlags = ((*pSelect).selFlags as ::core::ffi::c_uint
                    | SF_Correlated as ::core::ffi::c_uint) as u32_0;
                (*pWalker).eCode = 1 as u16_0;
                break;
            } else {
                i += 1;
            }
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn sqlite3ProcessReturningSubqueries(
    mut pEList: *mut ExprList,
    mut pTab: *mut Table,
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
        memset(
            &raw mut w as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        w.xExprCallback = Some(
            sqlite3ExprWalkNoop
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = Some(
            sqlite3ReturningSubqueryCorrelated
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        w.u.pTab = pTab as *mut Table;
        sqlite3WalkExprList(&raw mut w, pEList);
        if w.eCode != 0 {
            w.xExprCallback = Some(
                sqlite3ReturningSubqueryVarSelect
                    as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
                >;
            w.xSelectCallback = Some(
                sqlite3SelectWalkNoop
                    as unsafe extern "C" fn(
                        *mut Walker,
                        *mut Select,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
                >;
            sqlite3WalkExprList(&raw mut w, pEList);
        }
    }
}
unsafe extern "C" fn codeReturningTrigger(
    mut pParse: *mut Parse,
    mut pTrigger: *mut Trigger,
    mut pTab: *mut Table,
    mut regIn: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pNew: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pReturning: *mut Returning = ::core::ptr::null_mut::<Returning>();
        let mut sSelect: Select = Select {
            op: 0,
            nSelectRow: 0,
            selFlags: 0,
            iLimit: 0,
            iOffset: 0,
            selId: 0,
            addrOpenEphm: [0; 2],
            pEList: ::core::ptr::null_mut::<ExprList>(),
            pSrc: ::core::ptr::null_mut::<SrcList>(),
            pWhere: ::core::ptr::null_mut::<Expr>(),
            pGroupBy: ::core::ptr::null_mut::<ExprList>(),
            pHaving: ::core::ptr::null_mut::<Expr>(),
            pOrderBy: ::core::ptr::null_mut::<ExprList>(),
            pPrior: ::core::ptr::null_mut::<Select>(),
            pNext: ::core::ptr::null_mut::<Select>(),
            pLimit: ::core::ptr::null_mut::<Expr>(),
            pWith: ::core::ptr::null_mut::<With>(),
            pWin: ::core::ptr::null_mut::<Window>(),
            pWinDefn: ::core::ptr::null_mut::<Window>()

        };
        let mut pFrom: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut uSrc: C2Rust_Unnamed_24 = C2Rust_Unnamed_24 {
            sSrc: SrcList {
                nSrc: 0,
                nAlloc: 0,
                a: []

            }

        };
        if (*pParse).bReturning == 0 {
            return;
        }
        pReturning = (*pParse).u1.d.pReturning;
        if pTrigger != &raw mut (*pReturning).retTrig {
            return;
        }
        memset(
            &raw mut sSelect as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Select>() as size_t,
        );
        memset(
            &raw mut uSrc as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<C2Rust_Unnamed_24>() as size_t,
        );
        pFrom = &raw mut uSrc.sSrc;
        sSelect.pEList = sqlite3ExprListDup(
            db,
            (*pReturning).pReturnEL,
            0 as ::core::ffi::c_int,
        );
        sSelect.pSrc = pFrom;
        (*pFrom).nSrc = 1 as ::core::ffi::c_int;
        let ref mut c2rust_fresh1 = (*(&raw mut (*pFrom).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .pSTab;
        *c2rust_fresh1 = pTab;
        let ref mut c2rust_fresh2 = (*(&raw mut (*pFrom).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .zName;
        *c2rust_fresh2 = (*pTab).zName;
        (*(&raw mut (*pFrom).a as *mut SrcItem).offset(0 as ::core::ffi::c_int as isize))
            .iCursor = -1 as ::core::ffi::c_int;
        sqlite3SelectPrep(
            pParse,
            &raw mut sSelect,
            ::core::ptr::null_mut::<NameContext>(),
        );
        if (*pParse).nErr == 0 as ::core::ffi::c_int {
            sqlite3GenerateColumnNames(pParse, &raw mut sSelect);
        }
        sqlite3ExprListDelete(db, sSelect.pEList);
        pNew = sqlite3ExpandReturning(pParse, (*pReturning).pReturnEL, pTab);
        if (*pParse).nErr == 0 as ::core::ffi::c_int {
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
            if (*pReturning).nRetCol == 0 as ::core::ffi::c_int {
                (*pReturning).nRetCol = (*pNew).nExpr;
                let c2rust_fresh3 = (*pParse).nTab;
                (*pParse).nTab = (*pParse).nTab + 1;
                (*pReturning).iRetCur = c2rust_fresh3;
            }
            sNC.pParse = pParse;
            sNC.uNC.iBaseReg = regIn;
            sNC.ncFlags = NC_UBaseReg;
            (*pParse).eTriggerOp = (*pTrigger).op;
            (*pParse).pTriggerTab = pTab;
            if sqlite3ResolveExprListNames(&raw mut sNC, pNew) == SQLITE_OK
                && (*db).mallocFailed == 0
            {
                let mut i: ::core::ffi::c_int = 0;
                let mut nCol: ::core::ffi::c_int = (*pNew).nExpr;
                let mut reg: ::core::ffi::c_int = (*pParse).nMem
                    + 1 as ::core::ffi::c_int;
                sqlite3ProcessReturningSubqueries(pNew, pTab);
                (*pParse).nMem += nCol + 2 as ::core::ffi::c_int;
                (*pReturning).iRetReg = reg;
                i = 0 as ::core::ffi::c_int;
                while i < nCol {
                    let mut pCol: *mut Expr = (*(&raw mut (*pNew).a
                        as *mut ExprList_item)
                        .offset(i as isize))
                        .pExpr;
                    sqlite3ExprCodeFactorable(pParse, pCol, reg + i);
                    if sqlite3ExprAffinity(pCol) as ::core::ffi::c_int == SQLITE_AFF_REAL
                    {
                        sqlite3VdbeAddOp1(v, OP_RealAffinity, reg + i);
                    }
                    i += 1;
                }
                sqlite3VdbeAddOp3(v, OP_MakeRecord, reg, i, reg + i);
                sqlite3VdbeAddOp2(
                    v,
                    OP_NewRowid,
                    (*pReturning).iRetCur,
                    reg + i + 1 as ::core::ffi::c_int,
                );
                sqlite3VdbeAddOp3(
                    v,
                    OP_Insert,
                    (*pReturning).iRetCur,
                    reg + i,
                    reg + i + 1 as ::core::ffi::c_int,
                );
            }
        }
        sqlite3ExprListDelete(db, pNew);
        (*pParse).eTriggerOp = 0 as u8_0;
        (*pParse).pTriggerTab = ::core::ptr::null_mut::<Table>();
    }
}
unsafe extern "C" fn codeTriggerProgram(
    mut pParse: *mut Parse,
    mut pStepList: *mut TriggerStep,
    mut orconf: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStep: *mut TriggerStep = ::core::ptr::null_mut::<TriggerStep>();
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut db: *mut sqlite3 = (*pParse).db;
        pStep = pStepList;
        while !pStep.is_null() {
            (*pParse).eOrconf = (if orconf == OE_Default {
                (*pStep).orconf as ::core::ffi::c_int
            } else {
                orconf as u8_0 as ::core::ffi::c_int
            }) as u8_0;
            if !(*pStep).zSpan.is_null() {
                sqlite3VdbeAddOp4(
                    v,
                    OP_Trace,
                    0x7fffffff as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    sqlite3MPrintf(
                        db,
                        b"-- %s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*pStep).zSpan,
                    ),
                    P4_DYNAMIC,
                );
            }
            match (*pStep).op as ::core::ffi::c_int {
                TK_UPDATE => {
                    sqlite3Update(
                        pParse,
                        sqlite3TriggerStepSrc(pParse, pStep),
                        sqlite3ExprListDup(
                            db,
                            (*pStep).pExprList,
                            0 as ::core::ffi::c_int,
                        ),
                        sqlite3ExprDup(db, (*pStep).pWhere, 0 as ::core::ffi::c_int),
                        (*pParse).eOrconf as ::core::ffi::c_int,
                        ::core::ptr::null_mut::<ExprList>(),
                        ::core::ptr::null_mut::<Expr>(),
                        ::core::ptr::null_mut::<Upsert>(),
                    );
                    sqlite3VdbeAddOp0(v, OP_ResetCount);
                }
                TK_INSERT => {
                    sqlite3Insert(
                        pParse,
                        sqlite3TriggerStepSrc(pParse, pStep),
                        sqlite3SelectDup(db, (*pStep).pSelect, 0 as ::core::ffi::c_int),
                        sqlite3IdListDup(db, (*pStep).pIdList),
                        (*pParse).eOrconf as ::core::ffi::c_int,
                        sqlite3UpsertDup(db, (*pStep).pUpsert),
                    );
                    sqlite3VdbeAddOp0(v, OP_ResetCount);
                }
                TK_DELETE => {
                    sqlite3DeleteFrom(
                        pParse,
                        sqlite3TriggerStepSrc(pParse, pStep),
                        sqlite3ExprDup(db, (*pStep).pWhere, 0 as ::core::ffi::c_int),
                        ::core::ptr::null_mut::<ExprList>(),
                        ::core::ptr::null_mut::<Expr>(),
                    );
                    sqlite3VdbeAddOp0(v, OP_ResetCount);
                }
                _ => {
                    let mut sDest: SelectDest = SelectDest {
                        eDest: 0,
                        iSDParm: 0,
                        iSDParm2: 0,
                        iSdst: 0,
                        nSdst: 0,
                        zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        pOrderBy: ::core::ptr::null_mut::<ExprList>()

                    };
                    let mut pSelect: *mut Select = sqlite3SelectDup(
                        db,
                        (*pStep).pSelect,
                        0 as ::core::ffi::c_int,
                    );
                    sqlite3SelectDestInit(
                        &raw mut sDest,
                        SRT_Discard,
                        0 as ::core::ffi::c_int,
                    );
                    sqlite3Select(pParse, pSelect, &raw mut sDest);
                    sqlite3SelectDelete(db, pSelect);
                }
            }
            pStep = (*pStep).pNext;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn transferParseError(mut pTo: *mut Parse, mut pFrom: *mut Parse) {
    unsafe {
        if (*pTo).nErr == 0 as ::core::ffi::c_int {
            (*pTo).zErrMsg = (*pFrom).zErrMsg;
            (*pTo).nErr = (*pFrom).nErr;
            (*pTo).rc = (*pFrom).rc;
        } else {
            sqlite3DbFree((*pFrom).db, (*pFrom).zErrMsg as *mut ::core::ffi::c_void);
        };
    }
}
unsafe extern "C" fn codeRowTrigger(
    mut pParse: *mut Parse,
    mut pTrigger: *mut Trigger,
    mut pTab: *mut Table,
    mut orconf: ::core::ffi::c_int,
) -> *mut TriggerPrg {
    unsafe {
        let mut pTop: *mut Parse = if !(*pParse).pToplevel.is_null() {
            (*pParse).pToplevel
        } else {
            pParse
        };
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pPrg: *mut TriggerPrg = ::core::ptr::null_mut::<TriggerPrg>();
        let mut pWhen: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
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
        let mut pProgram: *mut SubProgram = ::core::ptr::null_mut::<SubProgram>();
        let mut iEndTrigger: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut sSubParse: Parse = Parse {
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
        pPrg = sqlite3DbMallocZero(db, ::core::mem::size_of::<TriggerPrg>() as u64_0)
            as *mut TriggerPrg;
        if pPrg.is_null() {
            return ::core::ptr::null_mut::<TriggerPrg>();
        }
        (*pPrg).pNext = (*pTop).pTriggerPrg;
        (*pTop).pTriggerPrg = pPrg;
        pProgram = sqlite3DbMallocZero(db, ::core::mem::size_of::<SubProgram>() as u64_0)
            as *mut SubProgram;
        (*pPrg).pProgram = pProgram;
        if pProgram.is_null() {
            return ::core::ptr::null_mut::<TriggerPrg>();
        }
        sqlite3VdbeLinkSubProgram((*pTop).pVdbe, pProgram);
        (*pPrg).pTrigger = pTrigger;
        (*pPrg).orconf = orconf;
        (*pPrg).aColmask[0 as ::core::ffi::c_int as usize] = 0xffffffff
            as ::core::ffi::c_uint as u32_0;
        (*pPrg).aColmask[1 as ::core::ffi::c_int as usize] = 0xffffffff
            as ::core::ffi::c_uint as u32_0;
        sqlite3ParseObjectInit(&raw mut sSubParse, db);
        memset(
            &raw mut sNC as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<NameContext>() as size_t,
        );
        sNC.pParse = &raw mut sSubParse;
        sSubParse.pTriggerTab = pTab;
        sSubParse.pToplevel = pTop;
        sSubParse.zAuthContext = (*pTrigger).zName;
        sSubParse.eTriggerOp = (*pTrigger).op;
        sSubParse.nQueryLoop = (*pParse).nQueryLoop;
        sSubParse.prepFlags = (*pParse).prepFlags;
        sSubParse.oldmask = 0 as u32_0;
        sSubParse.newmask = 0 as u32_0;
        v = sqlite3GetVdbe(&raw mut sSubParse);
        if !v.is_null() {
            if !(*pTrigger).zName.is_null() {
                sqlite3VdbeChangeP4(
                    v,
                    -1 as ::core::ffi::c_int,
                    sqlite3MPrintf(
                        db,
                        b"-- TRIGGER %s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*pTrigger).zName,
                    ),
                    P4_DYNAMIC,
                );
            }
            if !(*pTrigger).pWhen.is_null() {
                pWhen = sqlite3ExprDup(db, (*pTrigger).pWhen, 0 as ::core::ffi::c_int);
                if (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && SQLITE_OK == sqlite3ResolveExprNames(&raw mut sNC, pWhen)
                {
                    iEndTrigger = sqlite3VdbeMakeLabel(&raw mut sSubParse);
                    sqlite3ExprIfFalse(
                        &raw mut sSubParse,
                        pWhen,
                        iEndTrigger,
                        SQLITE_JUMPIFNULL,
                    );
                }
                sqlite3ExprDelete(db, pWhen);
            }
            codeTriggerProgram(&raw mut sSubParse, (*pTrigger).step_list, orconf);
            if iEndTrigger != 0 {
                sqlite3VdbeResolveLabel(v, iEndTrigger);
            }
            sqlite3VdbeAddOp0(v, OP_Halt);
            transferParseError(pParse, &raw mut sSubParse);
            if (*pParse).nErr == 0 as ::core::ffi::c_int {
                (*pProgram).aOp = sqlite3VdbeTakeOpArray(
                    v,
                    &raw mut (*pProgram).nOp,
                    &raw mut (*pTop).nMaxArg,
                );
            }
            (*pProgram).nMem = sSubParse.nMem;
            (*pProgram).nCsr = sSubParse.nTab;
            (*pProgram).token = pTrigger as *mut ::core::ffi::c_void;
            (*pPrg).aColmask[0 as ::core::ffi::c_int as usize] = sSubParse.oldmask;
            (*pPrg).aColmask[1 as ::core::ffi::c_int as usize] = sSubParse.newmask;
            sqlite3VdbeDelete(v);
        } else {
            transferParseError(pParse, &raw mut sSubParse);
        }
        sqlite3ParseObjectReset(&raw mut sSubParse);
        return pPrg;
    }
}
unsafe extern "C" fn getRowTrigger(
    mut pParse: *mut Parse,
    mut pTrigger: *mut Trigger,
    mut pTab: *mut Table,
    mut orconf: ::core::ffi::c_int,
) -> *mut TriggerPrg {
    unsafe {
        let mut pRoot: *mut Parse = if !(*pParse).pToplevel.is_null() {
            (*pParse).pToplevel
        } else {
            pParse
        };
        let mut pPrg: *mut TriggerPrg = ::core::ptr::null_mut::<TriggerPrg>();
        pPrg = (*pRoot).pTriggerPrg;
        while !pPrg.is_null()
            && ((*pPrg).pTrigger != pTrigger || (*pPrg).orconf != orconf)
        {
            pPrg = (*pPrg).pNext;
        }
        if pPrg.is_null() {
            pPrg = codeRowTrigger(pParse, pTrigger, pTab, orconf);
            (*(*pParse).db).errByteOffset = -1 as ::core::ffi::c_int;
        }
        return pPrg;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CodeRowTriggerDirect(
    mut pParse: *mut Parse,
    mut p: *mut Trigger,
    mut pTab: *mut Table,
    mut reg: ::core::ffi::c_int,
    mut orconf: ::core::ffi::c_int,
    mut ignoreJump: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        let mut pPrg: *mut TriggerPrg = ::core::ptr::null_mut::<TriggerPrg>();
        pPrg = getRowTrigger(pParse, p, pTab, orconf);
        if !pPrg.is_null() {
            let mut bRecursive: ::core::ffi::c_int = (!(*p).zName.is_null()
                && 0 as ::core::ffi::c_ulonglong
                    == (*(*pParse).db).flags as ::core::ffi::c_ulonglong
                        & SQLITE_RecTriggers as ::core::ffi::c_ulonglong)
                as ::core::ffi::c_int;
            (*pParse).nMem += 1;
            sqlite3VdbeAddOp4(
                v,
                OP_Program,
                reg,
                ignoreJump,
                (*pParse).nMem,
                (*pPrg).pProgram as *const ::core::ffi::c_char,
                P4_SUBPROGRAM,
            );
            sqlite3VdbeChangeP5(v, bRecursive as u16_0);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CodeRowTrigger(
    mut pParse: *mut Parse,
    mut pTrigger: *mut Trigger,
    mut op: ::core::ffi::c_int,
    mut pChanges: *mut ExprList,
    mut tr_tm: ::core::ffi::c_int,
    mut pTab: *mut Table,
    mut reg: ::core::ffi::c_int,
    mut orconf: ::core::ffi::c_int,
    mut ignoreJump: ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        p = pTrigger;
        while !p.is_null() {
            if ((*p).op as ::core::ffi::c_int == op
                || (*p).bReturning as ::core::ffi::c_int != 0
                    && (*p).op as ::core::ffi::c_int == TK_INSERT && op == TK_UPDATE)
                && (*p).tr_tm as ::core::ffi::c_int == tr_tm
                && checkColumnOverlap((*p).pColumns, pChanges) != 0
            {
                if (*p).bReturning == 0 {
                    sqlite3CodeRowTriggerDirect(
                        pParse,
                        p,
                        pTab,
                        reg,
                        orconf,
                        ignoreJump,
                    );
                } else if (*pParse).pToplevel.is_null() {
                    codeReturningTrigger(pParse, p, pTab, reg);
                }
            }
            p = (*p).pNext;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TriggerColmask(
    mut pParse: *mut Parse,
    mut pTrigger: *mut Trigger,
    mut pChanges: *mut ExprList,
    mut isNew: ::core::ffi::c_int,
    mut tr_tm: ::core::ffi::c_int,
    mut pTab: *mut Table,
    mut orconf: ::core::ffi::c_int,
) -> u32_0 {
    unsafe {
        let op: ::core::ffi::c_int = if !pChanges.is_null() {
            TK_UPDATE
        } else {
            TK_DELETE
        };
        let mut mask: u32_0 = 0 as u32_0;
        let mut p: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
            return 0xffffffff as u32_0;
        }
        p = pTrigger;
        while !p.is_null() {
            if (*p).op as ::core::ffi::c_int == op
                && tr_tm & (*p).tr_tm as ::core::ffi::c_int != 0
                && checkColumnOverlap((*p).pColumns, pChanges) != 0
            {
                if (*p).bReturning != 0 {
                    mask = 0xffffffff as ::core::ffi::c_uint as u32_0;
                } else {
                    let mut pPrg: *mut TriggerPrg = ::core::ptr::null_mut::<
                        TriggerPrg,
                    >();
                    pPrg = getRowTrigger(pParse, p, pTab, orconf);
                    if !pPrg.is_null() {
                        mask = (mask as ::core::ffi::c_uint
                            | (*pPrg).aColmask[isNew as usize] as ::core::ffi::c_uint)
                            as u32_0;
                    }
                }
            }
            p = (*p).pNext;
        }
        return mask;
    }
}
pub const SQLITE_RecTriggers: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SQLITE_EnableTrigger: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const DBFLAG_SchemaChange: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const COLFLAG_HIDDEN: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_AFF_REAL: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;
pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const TF_Shadow: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TABTYP_VIEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Default: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EP_xIsSelect: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const EXPRDUP_REDUCE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const ENAME_NAME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NC_UBaseReg: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SF_NestedFrom: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SF_Correlated: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const SRT_Discard: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PARSE_MODE_RENAME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TRIGGER_BEFORE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TRIGGER_AFTER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WRC_Continue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TK_BEFORE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const TK_ID: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const TK_INSTEAD: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const TK_INSERT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const TK_DELETE: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const TK_UPDATE: ::core::ffi::c_int = 130 as ::core::ffi::c_int;
pub const TK_SELECT: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const TK_DOT: ::core::ffi::c_int = 142 as ::core::ffi::c_int;
pub const TK_RETURNING: ::core::ffi::c_int = 151 as ::core::ffi::c_int;
pub const TK_ASTERISK: ::core::ffi::c_int = 180 as ::core::ffi::c_int;
