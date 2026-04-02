use c2rust_bitfields::BitfieldStruct;
use ::libc;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type PreUpdate;
    pub type Vdbe;
    pub type TableLock;
    pub type sqlite3_mutex;
    pub type CheckOnCtx;
    pub type CoveringIndexCheck;
    pub type WhereConst;
    pub type WindowRewrite;
    pub type IdxCover;
    pub type RefSrcList;
    pub type CCurHint;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_error_code(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value);
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3BtreeEnterAll(_: *mut sqlite3);
    fn sqlite3BtreeLeaveAll(_: *mut sqlite3);
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
    fn sqlite3VdbeAddParseSchemaOp(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: u16_0,
    );
    fn sqlite3VdbeChangeP5(_: *mut Vdbe, P5: u16_0);
    fn sqlite3VdbeJumpHere(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeUsesBtree(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeFinalize(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeCurrentAddr(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3WalkExpr(_: *mut Walker, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3WalkExprList(_: *mut Walker, _: *mut ExprList) -> ::core::ffi::c_int;
    fn sqlite3WalkSelect(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3CorruptError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3IsIdChar(_: u8_0) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3Dequote(_: *mut ::core::ffi::c_char);
    fn sqlite3RunParser(
        _: *mut Parse,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3GetTempReg(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempReg(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3ColumnExpr(_: *mut Table, _: *mut Column) -> *mut Expr;
    fn sqlite3PrimaryKeyIndex(_: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(
        _: *mut Index,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ViewGetColumnNames(_: *mut Parse, _: *mut Table) -> ::core::ffi::c_int;
    fn sqlite3DeleteTable(_: *mut sqlite3, _: *mut Table);
    fn sqlite3FreeIndex(_: *mut sqlite3, _: *mut Index);
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
    fn sqlite3ExprCodeGetColumnOfTable(
        _: *mut Vdbe,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
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
    fn sqlite3LocateTableItem(
        _: *mut Parse,
        flags: u32_0,
        _: *mut SrcItem,
    ) -> *mut Table;
    fn sqlite3FindIndex(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut Index;
    fn sqlite3NameFromToken(
        _: *mut sqlite3,
        _: *const Token,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3GetVdbe(_: *mut Parse) -> *mut Vdbe;
    fn sqlite3MayAbort(_: *mut Parse);
    fn sqlite3ExprListDup(
        _: *mut sqlite3,
        _: *const ExprList,
        _: ::core::ffi::c_int,
    ) -> *mut ExprList;
    fn sqlite3InsertBuiltinFuncs(_: *mut FuncDef, _: ::core::ffi::c_int);
    fn sqlite3ChangeCookie(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3WithDup(db: *mut sqlite3, p: *mut With) -> *mut With;
    fn sqlite3DeleteTrigger(_: *mut sqlite3, _: *mut Trigger);
    fn sqlite3TriggerStepSrc(_: *mut Parse, _: *mut TriggerStep) -> *mut SrcList;
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
    fn sqlite3Utf8CharLen(
        pData: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3WritableSchema(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3CheckObjectName(
        _: *mut Parse,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ValueFree(_: *mut sqlite3_value);
    fn sqlite3ValueFromExpr(
        _: *mut sqlite3,
        _: *const Expr,
        _: u8_0,
        _: u8_0,
        _: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    fn sqlite3NestedParse(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3SelectPrep(_: *mut Parse, _: *mut Select, _: *mut NameContext);
    fn sqlite3StrIHash(_: *const ::core::ffi::c_char) -> u8_0;
    fn sqlite3ResolveExprNames(_: *mut NameContext, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3ResolveExprListNames(
        _: *mut NameContext,
        _: *mut ExprList,
    ) -> ::core::ffi::c_int;
    fn sqlite3FindDbName(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
    fn sqlite3GetVTable(_: *mut sqlite3, _: *mut Table) -> *mut VTable;
    fn sqlite3ReadOnlyShadowTables(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3IsShadowTableOf(
        _: *mut sqlite3,
        _: *mut Table,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ParseObjectInit(_: *mut Parse, _: *mut sqlite3);
    fn sqlite3ParseObjectReset(_: *mut Parse);
    fn sqlite3WithPush(_: *mut Parse, _: *mut With, _: u8_0) -> *mut With;
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
pub struct RenameToken {
    pub p: *const ::core::ffi::c_void,
    pub t: Token,
    pub pNext: *mut RenameToken

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
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
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
pub struct RenameCtx {
    pub pList: *mut RenameToken,
    pub nList: ::core::ffi::c_int,
    pub iCol: ::core::ffi::c_int,
    pub pTab: *mut Table,
    pub zOld: *const ::core::ffi::c_char

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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ALTER_TABLE: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const OP_IfPos: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const OP_AddImm: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_MakeRecord: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const OP_ReadCookie: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const OP_SetCookie: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const OP_OpenWrite: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const OP_Insert: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const OP_Rowid: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const OP_IdxInsert: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const OP_VRename: ::core::ffi::c_int = 178 as ::core::ffi::c_int;
pub const BTREE_FILE_FORMAT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const P4_VTAB: ::core::ffi::c_int = -11 as ::core::ffi::c_int;
pub const TK_TRIGGER: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
pub const TK_STRING: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_COLUMN: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
pub const SQLITE_ForeignKeys: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_LegacyAlter: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const SQLITE_DqsDDL: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const SQLITE_DqsDML: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
pub const SQLITE_Comments: u64_0 = (0x40 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const SQLITE_FUNC_CONSTANT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_FUNC_INTERNAL: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_BUILTIN: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
pub const COLFLAG_PRIMKEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const COLFLAG_UNIQUE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const COLFLAG_VIRTUAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const COLFLAG_STORED: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const COLFLAG_GENERATED: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const SQLITE_AFF_BLOB: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const SQLITE_AFF_REAL: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_Shadow: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const TF_Eponymous: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const TF_Strict: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const TABTYP_NORM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TABTYP_VIEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EP_DblQuoted: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const EP_WinFunc: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const EP_Subrtn: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const ENAME_NAME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENAME_SPAN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NC_UUpsert: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SF_Expanded: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SF_View: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SF_CopyCte: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const PARSE_MODE_RENAME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PARSE_MODE_UNMAP: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const OPFLAG_SAVEPOSITION: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const INITFLAG_AlterRename: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const INITFLAG_AlterDrop: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const INITFLAG_AlterAdd: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
pub const WRC_Continue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WRC_Prune: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WRC_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn isAlterableTable(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
) -> ::core::ffi::c_int {
    unsafe {
        if 0 as ::core::ffi::c_int
            == sqlite3_strnicmp(
                (*pTab).zName,
                b"sqlite_\0".as_ptr() as *const ::core::ffi::c_char,
                7 as ::core::ffi::c_int,
            ) || (*pTab).tabFlags & TF_Eponymous as u32_0 != 0 as u32_0
            || (*pTab).tabFlags & TF_Shadow as u32_0 != 0 as u32_0
                && sqlite3ReadOnlyShadowTables((*pParse).db) != 0
        {
            sqlite3ErrorMsg(
                pParse,
                b"table %s may not be altered\0".as_ptr() as *const ::core::ffi::c_char,
                (*pTab).zName,
            );
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn renameTestSchema(
    mut pParse: *mut Parse,
    mut zDb: *const ::core::ffi::c_char,
    mut bTemp: ::core::ffi::c_int,
    mut zWhen: *const ::core::ffi::c_char,
    mut bNoDQS: ::core::ffi::c_int,
) {
    unsafe {
        (*pParse).set_colNamesSet(1 as bft as bft);
        sqlite3NestedParse(
            pParse,
            b"SELECT 1 FROM \"%w\".sqlite_master WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X' AND sql NOT LIKE 'create virtual%%' AND sqlite_rename_test(%Q, sql, type, name, %d, %Q, %d)=NULL \0"
                .as_ptr() as *const ::core::ffi::c_char,
            zDb,
            zDb,
            bTemp,
            zWhen,
            bNoDQS,
        );
        if bTemp == 0 as ::core::ffi::c_int {
            sqlite3NestedParse(
                pParse,
                b"SELECT 1 FROM temp.sqlite_master WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X' AND sql NOT LIKE 'create virtual%%' AND sqlite_rename_test(%Q, sql, type, name, 1, %Q, %d)=NULL \0"
                    .as_ptr() as *const ::core::ffi::c_char,
                zDb,
                zWhen,
                bNoDQS,
            );
        }
    }
}
unsafe extern "C" fn renameFixQuotes(
    mut pParse: *mut Parse,
    mut zDb: *const ::core::ffi::c_char,
    mut bTemp: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3NestedParse(
            pParse,
            b"UPDATE \"%w\".sqlite_master SET sql = sqlite_rename_quotefix(%Q, sql)WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X' AND sql NOT LIKE 'create virtual%%'\0"
                .as_ptr() as *const ::core::ffi::c_char,
            zDb,
            zDb,
        );
        if bTemp == 0 as ::core::ffi::c_int {
            sqlite3NestedParse(
                pParse,
                b"UPDATE temp.sqlite_master SET sql = sqlite_rename_quotefix('temp', sql)WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X' AND sql NOT LIKE 'create virtual%%'\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn renameReloadSchema(
    mut pParse: *mut Parse,
    mut iDb: ::core::ffi::c_int,
    mut p5: u16_0,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        if !v.is_null() {
            sqlite3ChangeCookie(pParse, iDb);
            sqlite3VdbeAddParseSchemaOp(
                (*pParse).pVdbe,
                iDb,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                p5,
            );
            if iDb != 1 as ::core::ffi::c_int {
                sqlite3VdbeAddParseSchemaOp(
                    (*pParse).pVdbe,
                    1 as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    p5,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AlterRenameTable(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pName: *mut Token,
) {
    unsafe {
        let mut iDb: ::core::ffi::c_int = 0;
        let mut zDb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut nTabName: ::core::ffi::c_int = 0;
        let mut zTabName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut pVTab: *mut VTable = ::core::ptr::null_mut::<VTable>();
        if !((*db).mallocFailed != 0) {
            pTab = sqlite3LocateTableItem(
                pParse,
                0 as u32_0,
                (&raw mut (*pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem,
            );
            if !pTab.is_null() {
                iDb = sqlite3SchemaToIndex((*pParse).db, (*pTab).pSchema);
                zDb = (*(*db).aDb.offset(iDb as isize)).zDbSName;
                zName = sqlite3NameFromToken(db, pName);
                if !zName.is_null() {
                    if !sqlite3FindTable(db, zName, zDb).is_null()
                        || !sqlite3FindIndex(db, zName, zDb).is_null()
                        || sqlite3IsShadowTableOf(db, pTab, zName) != 0
                    {
                        sqlite3ErrorMsg(
                            pParse,
                            b"there is already another table or index with this name: %s\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            zName,
                        );
                    } else if !(SQLITE_OK != isAlterableTable(pParse, pTab)) {
                        if !(SQLITE_OK
                            != sqlite3CheckObjectName(
                                pParse,
                                zName,
                                b"table\0".as_ptr() as *const ::core::ffi::c_char,
                                zName,
                            ))
                        {
                            if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"view %s may not be altered\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*pTab).zName,
                                );
                            } else if !(sqlite3AuthCheck(
                                pParse,
                                SQLITE_ALTER_TABLE,
                                zDb,
                                (*pTab).zName,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                            ) != 0)
                            {
                                if !(sqlite3ViewGetColumnNames(pParse, pTab) != 0) {
                                    if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                                        pVTab = sqlite3GetVTable(db, pTab);
                                        if (*(*(*pVTab).pVtab).pModule).xRename.is_none() {
                                            pVTab = ::core::ptr::null_mut::<VTable>();
                                        }
                                    }
                                    v = sqlite3GetVdbe(pParse);
                                    if !v.is_null() {
                                        sqlite3MayAbort(pParse);
                                        zTabName = (*pTab).zName;
                                        nTabName = sqlite3Utf8CharLen(
                                            zTabName,
                                            -1 as ::core::ffi::c_int,
                                        );
                                        sqlite3NestedParse(
                                            pParse,
                                            b"UPDATE \"%w\".sqlite_master SET sql = sqlite_rename_table(%Q, type, name, sql, %Q, %Q, %d) WHERE (type!='index' OR tbl_name=%Q COLLATE nocase)AND   name NOT LIKE 'sqliteX_%%' ESCAPE 'X'\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            zDb,
                                            zDb,
                                            zTabName,
                                            zName,
                                            (iDb == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                            zTabName,
                                        );
                                        sqlite3NestedParse(
                                            pParse,
                                            b"UPDATE %Q.sqlite_master SET tbl_name = %Q, name = CASE WHEN type='table' THEN %Q WHEN name LIKE 'sqliteX_autoindex%%' ESCAPE 'X'      AND type='index' THEN 'sqlite_autoindex_' || %Q || substr(name,%d+18) ELSE name END WHERE tbl_name=%Q COLLATE nocase AND (type='table' OR type='index' OR type='trigger');\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            zDb,
                                            zName,
                                            zName,
                                            zName,
                                            nTabName,
                                            zTabName,
                                        );
                                        if !sqlite3FindTable(
                                                db,
                                                b"sqlite_sequence\0".as_ptr() as *const ::core::ffi::c_char,
                                                zDb,
                                            )
                                            .is_null()
                                        {
                                            sqlite3NestedParse(
                                                pParse,
                                                b"UPDATE \"%w\".sqlite_sequence set name = %Q WHERE name = %Q\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                zDb,
                                                zName,
                                                (*pTab).zName,
                                            );
                                        }
                                        if iDb != 1 as ::core::ffi::c_int {
                                            sqlite3NestedParse(
                                                pParse,
                                                b"UPDATE sqlite_temp_schema SET sql = sqlite_rename_table(%Q, type, name, sql, %Q, %Q, 1), tbl_name = CASE WHEN tbl_name=%Q COLLATE nocase AND   sqlite_rename_test(%Q, sql, type, name, 1, 'after rename', 0) THEN %Q ELSE tbl_name END WHERE type IN ('view', 'trigger')\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                zDb,
                                                zTabName,
                                                zName,
                                                zTabName,
                                                zDb,
                                                zName,
                                            );
                                        }
                                        if !pVTab.is_null() {
                                            (*pParse).nMem += 1;
                                            let mut i: ::core::ffi::c_int = (*pParse).nMem;
                                            sqlite3VdbeLoadString(v, i, zName);
                                            sqlite3VdbeAddOp4(
                                                v,
                                                OP_VRename,
                                                i,
                                                0 as ::core::ffi::c_int,
                                                0 as ::core::ffi::c_int,
                                                pVTab as *const ::core::ffi::c_char,
                                                P4_VTAB,
                                            );
                                        }
                                        renameReloadSchema(
                                            pParse,
                                            iDb,
                                            INITFLAG_AlterRename as u16_0,
                                        );
                                        renameTestSchema(
                                            pParse,
                                            zDb,
                                            (iDb == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                            b"after rename\0".as_ptr() as *const ::core::ffi::c_char,
                                            0 as ::core::ffi::c_int,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        sqlite3SrcListDelete(db, pSrc);
        sqlite3DbFree(db, zName as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn sqlite3ErrorIfNotEmpty(
    mut pParse: *mut Parse,
    mut zDb: *const ::core::ffi::c_char,
    mut zTab: *const ::core::ffi::c_char,
    mut zErr: *const ::core::ffi::c_char,
) {
    unsafe {
        sqlite3NestedParse(
            pParse,
            b"SELECT raise(ABORT,%Q) FROM \"%w\".\"%w\"\0".as_ptr()
                as *const ::core::ffi::c_char,
            zErr,
            zDb,
            zTab,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AlterFinishAddColumn(
    mut pParse: *mut Parse,
    mut pColDef: *mut Token,
) {
    unsafe {
        let mut pNew: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut iDb: ::core::ffi::c_int = 0;
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zTab: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zCol: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        let mut pDflt: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut r1: ::core::ffi::c_int = 0;
        db = (*pParse).db;
        if (*pParse).nErr != 0 {
            return;
        }
        pNew = (*pParse).pNewTable;
        iDb = sqlite3SchemaToIndex(db, (*pNew).pSchema);
        zDb = (*(*db).aDb.offset(iDb as isize)).zDbSName;
        zTab = (*pNew).zName.offset(16 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char;
        pCol = (*pNew)
            .aCol
            .offset(
                ((*pNew).nCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
            ) as *mut Column;
        pDflt = sqlite3ColumnExpr(pNew, pCol);
        pTab = sqlite3FindTable(db, zTab, zDb);
        if sqlite3AuthCheck(
            pParse,
            SQLITE_ALTER_TABLE,
            zDb,
            (*pTab).zName,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) != 0
        {
            return;
        }
        if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_PRIMKEY != 0 {
            sqlite3ErrorMsg(
                pParse,
                b"Cannot add a PRIMARY KEY column\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if !(*pNew).pIndex.is_null() {
            sqlite3ErrorMsg(
                pParse,
                b"Cannot add a UNIQUE column\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_GENERATED
            == 0 as ::core::ffi::c_int
        {
            if !pDflt.is_null() && (*(*pDflt).pLeft).op as ::core::ffi::c_int == TK_NULL
            {
                pDflt = ::core::ptr::null_mut::<Expr>();
            }
            if (*db).flags as ::core::ffi::c_ulonglong
                & SQLITE_ForeignKeys as ::core::ffi::c_ulonglong != 0
                && !(*pNew).u.tab.pFKey.is_null() && !pDflt.is_null()
            {
                sqlite3ErrorIfNotEmpty(
                    pParse,
                    zDb,
                    zTab,
                    b"Cannot add a REFERENCES column with non-NULL default value\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if (*pCol).notNull() as ::core::ffi::c_int != 0 && pDflt.is_null() {
                sqlite3ErrorIfNotEmpty(
                    pParse,
                    zDb,
                    zTab,
                    b"Cannot add a NOT NULL column with default value NULL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if !pDflt.is_null() {
                let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                let mut rc: ::core::ffi::c_int = 0;
                rc = sqlite3ValueFromExpr(
                    db,
                    pDflt,
                    SQLITE_UTF8 as u8_0,
                    SQLITE_AFF_BLOB as u8_0,
                    &raw mut pVal,
                );
                if rc != SQLITE_OK {
                    return;
                }
                if pVal.is_null() {
                    sqlite3ErrorIfNotEmpty(
                        pParse,
                        zDb,
                        zTab,
                        b"Cannot add a column with non-constant default\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                sqlite3ValueFree(pVal);
            }
        } else if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_STORED != 0 {
            sqlite3ErrorIfNotEmpty(
                pParse,
                zDb,
                zTab,
                b"cannot add a STORED column\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        zCol = sqlite3DbStrNDup(
            db,
            (*pColDef).z as *mut ::core::ffi::c_char,
            (*pColDef).n as u64_0,
        );
        if !zCol.is_null() {
            let mut zEnd: *mut ::core::ffi::c_char = zCol
                .offset((*pColDef).n.wrapping_sub(1 as ::core::ffi::c_uint) as isize)
                as *mut ::core::ffi::c_char;
            while zEnd > zCol
                && (*zEnd as ::core::ffi::c_int == ';' as i32
                    || *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(*zEnd as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0)
            {
                let c2rust_fresh1 = zEnd;
                zEnd = zEnd.offset(-1);
                *c2rust_fresh1 = '\0' as i32 as ::core::ffi::c_char;
            }
            sqlite3NestedParse(
                pParse,
                b"UPDATE \"%w\".sqlite_master SET sql = printf('%%.%ds, ',sql) || %Q || substr(sql,1+length(printf('%%.%ds',sql))) WHERE type = 'table' AND name = %Q\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                zDb,
                (*pNew).u.tab.addColOffset,
                zCol,
                (*pNew).u.tab.addColOffset,
                zTab,
            );
            sqlite3DbFree(db, zCol as *mut ::core::ffi::c_void);
        }
        v = sqlite3GetVdbe(pParse);
        if !v.is_null() {
            r1 = sqlite3GetTempReg(pParse);
            sqlite3VdbeAddOp3(v, OP_ReadCookie, iDb, r1, BTREE_FILE_FORMAT);
            sqlite3VdbeUsesBtree(v, iDb);
            sqlite3VdbeAddOp2(v, OP_AddImm, r1, -2 as ::core::ffi::c_int);
            sqlite3VdbeAddOp2(
                v,
                OP_IfPos,
                r1,
                sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp3(
                v,
                OP_SetCookie,
                iDb,
                BTREE_FILE_FORMAT,
                3 as ::core::ffi::c_int,
            );
            sqlite3ReleaseTempReg(pParse, r1);
            renameReloadSchema(pParse, iDb, INITFLAG_AlterAdd as u16_0);
            if !(*pNew).pCheck.is_null()
                || (*pCol).notNull() as ::core::ffi::c_int != 0
                    && (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_GENERATED
                        != 0 as ::core::ffi::c_int
                || (*pTab).tabFlags & TF_Strict as u32_0 != 0 as u32_0
            {
                sqlite3NestedParse(
                    pParse,
                    b"SELECT CASE WHEN quick_check GLOB 'CHECK*' THEN raise(ABORT,'CHECK constraint failed') WHEN quick_check GLOB 'non-* value in*' THEN raise(ABORT,'type mismatch on DEFAULT') ELSE raise(ABORT,'NOT NULL constraint failed') END  FROM pragma_quick_check(%Q,%Q) WHERE quick_check GLOB 'CHECK*' OR quick_check GLOB 'NULL*' OR quick_check GLOB 'non-* value in*'\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    zTab,
                    zDb,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AlterBeginAddColumn(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
) {
    unsafe {
        let mut pNew: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut iDb: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut nAlloc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        if !((*db).mallocFailed != 0) {
            pTab = sqlite3LocateTableItem(
                pParse,
                0 as u32_0,
                (&raw mut (*pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem,
            );
            if !pTab.is_null() {
                if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                    sqlite3ErrorMsg(
                        pParse,
                        b"virtual tables may not be altered\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
                    sqlite3ErrorMsg(
                        pParse,
                        b"Cannot add a column to a view\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else if !(SQLITE_OK != isAlterableTable(pParse, pTab)) {
                    sqlite3MayAbort(pParse);
                    iDb = sqlite3SchemaToIndex(db, (*pTab).pSchema);
                    pNew = sqlite3DbMallocZero(
                        db,
                        ::core::mem::size_of::<Table>() as u64_0,
                    ) as *mut Table;
                    if !pNew.is_null() {
                        (*pParse).pNewTable = pNew;
                        (*pNew).nTabRef = 1 as u32_0;
                        (*pNew).nCol = (*pTab).nCol;
                        nAlloc = ((*pNew).nCol as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int
                            * 8 as ::core::ffi::c_int + 8 as ::core::ffi::c_int;
                        (*pNew).aCol = sqlite3DbMallocZero(
                            db,
                            (::core::mem::size_of::<Column>() as usize)
                                .wrapping_mul(nAlloc as u32_0 as usize) as u64_0,
                        ) as *mut Column;
                        (*pNew).zName = sqlite3MPrintf(
                            db,
                            b"sqlite_altertab_%s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*pTab).zName,
                        );
                        if !((*pNew).aCol.is_null() || (*pNew).zName.is_null()) {
                            memcpy(
                                (*pNew).aCol as *mut ::core::ffi::c_void,
                                (*pTab).aCol as *const ::core::ffi::c_void,
                                (::core::mem::size_of::<Column>() as size_t)
                                    .wrapping_mul((*pNew).nCol as size_t),
                            );
                            i = 0 as ::core::ffi::c_int;
                            while i < (*pNew).nCol as ::core::ffi::c_int {
                                let mut pCol: *mut Column = (*pNew).aCol.offset(i as isize)
                                    as *mut Column;
                                (*pCol).zCnName = sqlite3DbStrDup(db, (*pCol).zCnName);
                                (*pCol).hName = sqlite3StrIHash((*pCol).zCnName);
                                i += 1;
                            }
                            (*pNew).u.tab.pDfltList = sqlite3ExprListDup(
                                db,
                                (*pTab).u.tab.pDfltList,
                                0 as ::core::ffi::c_int,
                            );
                            (*pNew).pSchema = (*(*db).aDb.offset(iDb as isize)).pSchema;
                            (*pNew).u.tab.addColOffset = (*pTab).u.tab.addColOffset;
                        }
                    }
                }
            }
        }
        sqlite3SrcListDelete(db, pSrc);
    }
}
unsafe extern "C" fn isRealTable(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut bDrop: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zType: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
            zType = b"view\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
            zType = b"virtual table\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if !zType.is_null() {
            sqlite3ErrorMsg(
                pParse,
                b"cannot %s %s \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                if bDrop != 0 {
                    b"drop column from\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"rename columns of\0".as_ptr() as *const ::core::ffi::c_char
                },
                zType,
                (*pTab).zName,
            );
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AlterRenameColumn(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pOld: *mut Token,
    mut pNew: *mut Token,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut iCol: ::core::ffi::c_int = 0;
        let mut zOld: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut iSchema: ::core::ffi::c_int = 0;
        let mut bQuote: ::core::ffi::c_int = 0;
        pTab = sqlite3LocateTableItem(
            pParse,
            0 as u32_0,
            (&raw mut (*pSrc).a as *mut SrcItem).offset(0 as ::core::ffi::c_int as isize)
                as *mut SrcItem,
        );
        if !pTab.is_null() {
            if !(SQLITE_OK != isAlterableTable(pParse, pTab)) {
                if !(SQLITE_OK != isRealTable(pParse, pTab, 0 as ::core::ffi::c_int)) {
                    iSchema = sqlite3SchemaToIndex(db, (*pTab).pSchema);
                    zDb = (*(*db).aDb.offset(iSchema as isize)).zDbSName;
                    if !(sqlite3AuthCheck(
                        pParse,
                        SQLITE_ALTER_TABLE,
                        zDb,
                        (*pTab).zName,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    ) != 0)
                    {
                        zOld = sqlite3NameFromToken(db, pOld);
                        if !zOld.is_null() {
                            iCol = sqlite3ColumnIndex(pTab, zOld);
                            if iCol < 0 as ::core::ffi::c_int {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"no such column: \"%T\"\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    pOld,
                                );
                            } else {
                                renameTestSchema(
                                    pParse,
                                    zDb,
                                    (iSchema == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                    b"\0".as_ptr() as *const ::core::ffi::c_char,
                                    0 as ::core::ffi::c_int,
                                );
                                renameFixQuotes(
                                    pParse,
                                    zDb,
                                    (iSchema == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                );
                                sqlite3MayAbort(pParse);
                                zNew = sqlite3NameFromToken(db, pNew);
                                if !zNew.is_null() {
                                    bQuote = *(&raw const sqlite3CtypeMap
                                        as *const ::core::ffi::c_uchar)
                                        .offset(
                                            *(*pNew).z.offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uchar as isize,
                                        ) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int;
                                    sqlite3NestedParse(
                                        pParse,
                                        b"UPDATE \"%w\".sqlite_master SET sql = sqlite_rename_column(sql, type, name, %Q, %Q, %d, %Q, %d, %d) WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X'  AND (type != 'index' OR tbl_name = %Q)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        zDb,
                                        zDb,
                                        (*pTab).zName,
                                        iCol,
                                        zNew,
                                        bQuote,
                                        (iSchema == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                        (*pTab).zName,
                                    );
                                    sqlite3NestedParse(
                                        pParse,
                                        b"UPDATE temp.sqlite_master SET sql = sqlite_rename_column(sql, type, name, %Q, %Q, %d, %Q, %d, 1) WHERE type IN ('trigger', 'view')\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        zDb,
                                        (*pTab).zName,
                                        iCol,
                                        zNew,
                                        bQuote,
                                    );
                                    renameReloadSchema(
                                        pParse,
                                        iSchema,
                                        INITFLAG_AlterRename as u16_0,
                                    );
                                    renameTestSchema(
                                        pParse,
                                        zDb,
                                        (iSchema == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                        b"after rename\0".as_ptr() as *const ::core::ffi::c_char,
                                        1 as ::core::ffi::c_int,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        sqlite3SrcListDelete(db, pSrc);
        sqlite3DbFree(db, zOld as *mut ::core::ffi::c_void);
        sqlite3DbFree(db, zNew as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RenameTokenMap(
    mut pParse: *mut Parse,
    mut pPtr: *const ::core::ffi::c_void,
    mut pToken: *const Token,
) -> *const ::core::ffi::c_void {
    unsafe {
        let mut pNew: *mut RenameToken = ::core::ptr::null_mut::<RenameToken>();
        if (*pParse).eParseMode as ::core::ffi::c_int != 3 as ::core::ffi::c_int {
            pNew = sqlite3DbMallocZero(
                (*pParse).db,
                ::core::mem::size_of::<RenameToken>() as u64_0,
            ) as *mut RenameToken;
            if !pNew.is_null() {
                (*pNew).p = pPtr;
                (*pNew).t = *pToken;
                (*pNew).pNext = (*pParse).pRename;
                (*pParse).pRename = pNew;
            }
        }
        return pPtr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RenameTokenRemap(
    mut pParse: *mut Parse,
    mut pTo: *const ::core::ffi::c_void,
    mut pFrom: *const ::core::ffi::c_void,
) {
    unsafe {
        let mut p: *mut RenameToken = ::core::ptr::null_mut::<RenameToken>();
        p = (*pParse).pRename;
        while !p.is_null() {
            if (*p).p == pFrom {
                (*p).p = pTo;
                break;
            } else {
                p = (*p).pNext;
            }
        }
    }
}
unsafe extern "C" fn renameUnmapExprCb(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pParse: *mut Parse = (*pWalker).pParse;
        sqlite3RenameTokenRemap(
            pParse,
            ::core::ptr::null::<::core::ffi::c_void>(),
            pExpr as *const ::core::ffi::c_void,
        );
        if (*pExpr).flags & (EP_WinFunc | EP_Subrtn) as u32_0 == 0 as u32_0 {
            sqlite3RenameTokenRemap(
                pParse,
                ::core::ptr::null::<::core::ffi::c_void>(),
                &raw mut (*pExpr).y.pTab as *const ::core::ffi::c_void,
            );
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn renameWalkWith(mut pWalker: *mut Walker, mut pSelect: *mut Select) {
    unsafe {
        let mut pWith: *mut With = (*pSelect).pWith;
        if !pWith.is_null() {
            let mut pParse: *mut Parse = (*pWalker).pParse;
            let mut i: ::core::ffi::c_int = 0;
            let mut pCopy: *mut With = ::core::ptr::null_mut::<With>();
            if (*(*(&raw mut (*pWith).a as *mut Cte)
                .offset(0 as ::core::ffi::c_int as isize))
                .pSelect)
                .selFlags & SF_Expanded as u32_0 == 0 as u32_0
            {
                pCopy = sqlite3WithDup((*pParse).db, pWith);
                pCopy = sqlite3WithPush(pParse, pCopy, 1 as u8_0);
            }
            i = 0 as ::core::ffi::c_int;
            while i < (*pWith).nCte {
                let mut p: *mut Select = (*(&raw mut (*pWith).a as *mut Cte)
                    .offset(i as isize))
                    .pSelect;
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
                if !pCopy.is_null() {
                    sqlite3SelectPrep(sNC.pParse, p, &raw mut sNC);
                }
                if (*(*sNC.pParse).db).mallocFailed != 0 {
                    return;
                }
                sqlite3WalkSelect(pWalker, p);
                sqlite3RenameExprlistUnmap(
                    pParse,
                    (*(&raw mut (*pWith).a as *mut Cte).offset(i as isize)).pCols,
                );
                i += 1;
            }
            if !pCopy.is_null() && (*pParse).pWith == pCopy {
                (*pParse).pWith = (*pCopy).pOuter;
            }
        }
    }
}
unsafe extern "C" fn unmapColumnIdlistNames(
    mut pParse: *mut Parse,
    mut pIdList: *const IdList,
) {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pIdList).nId {
            sqlite3RenameTokenRemap(
                pParse,
                ::core::ptr::null::<::core::ffi::c_void>(),
                (*(&raw const (*pIdList).a as *const IdList_item).offset(ii as isize))
                    .zName as *const ::core::ffi::c_void,
            );
            ii += 1;
        }
    }
}
unsafe extern "C" fn renameUnmapSelectCb(
    mut pWalker: *mut Walker,
    mut p: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pParse: *mut Parse = (*pWalker).pParse;
        let mut i: ::core::ffi::c_int = 0;
        if (*pParse).nErr != 0 {
            return WRC_Abort;
        }
        if (*p).selFlags & (SF_View | SF_CopyCte) as u32_0 != 0 {
            return WRC_Prune;
        }
        if !(*p).pEList.is_null() {
            let mut pList: *mut ExprList = (*p).pEList;
            i = 0 as ::core::ffi::c_int;
            while i < (*pList).nExpr {
                if !(*(&raw mut (*pList).a as *mut ExprList_item).offset(i as isize))
                    .zEName
                    .is_null()
                    && (*(&raw mut (*pList).a as *mut ExprList_item).offset(i as isize))
                        .fg
                        .eEName() as ::core::ffi::c_int == ENAME_NAME
                {
                    sqlite3RenameTokenRemap(
                        pParse,
                        ::core::ptr::null::<::core::ffi::c_void>(),
                        (*(&raw mut (*pList).a as *mut ExprList_item).offset(i as isize))
                            .zEName as *mut ::core::ffi::c_void,
                    );
                }
                i += 1;
            }
        }
        if !(*p).pSrc.is_null() {
            let mut pSrc: *mut SrcList = (*p).pSrc;
            i = 0 as ::core::ffi::c_int;
            while i < (*pSrc).nSrc {
                sqlite3RenameTokenRemap(
                    pParse,
                    ::core::ptr::null::<::core::ffi::c_void>(),
                    (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize)).zName
                        as *mut ::core::ffi::c_void,
                );
                if (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize))
                    .fg
                    .isUsing() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    sqlite3WalkExpr(
                        pWalker,
                        (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize)).u3.pOn,
                    );
                } else {
                    unmapColumnIdlistNames(
                        pParse,
                        (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize))
                            .u3
                            .pUsing,
                    );
                }
                i += 1;
            }
        }
        renameWalkWith(pWalker, p);
        return WRC_Continue;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RenameExprUnmap(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
) {
    unsafe {
        let mut eMode: u8_0 = (*pParse).eParseMode;
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
        sWalker.pParse = pParse;
        sWalker.xExprCallback = Some(
            renameUnmapExprCb
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        sWalker.xSelectCallback = Some(
            renameUnmapSelectCb
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        (*pParse).eParseMode = PARSE_MODE_UNMAP as u8_0;
        sqlite3WalkExpr(&raw mut sWalker, pExpr);
        (*pParse).eParseMode = eMode;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RenameExprlistUnmap(
    mut pParse: *mut Parse,
    mut pEList: *mut ExprList,
) {
    unsafe {
        if !pEList.is_null() {
            let mut i: ::core::ffi::c_int = 0;
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
            sWalker.pParse = pParse;
            sWalker.xExprCallback = Some(
                renameUnmapExprCb
                    as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
                >;
            sqlite3WalkExprList(&raw mut sWalker, pEList);
            i = 0 as ::core::ffi::c_int;
            while i < (*pEList).nExpr {
                if (*(&raw mut (*pEList).a as *mut ExprList_item).offset(i as isize))
                    .fg
                    .eEName() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    sqlite3RenameTokenRemap(
                        pParse,
                        ::core::ptr::null::<::core::ffi::c_void>(),
                        (*(&raw mut (*pEList).a as *mut ExprList_item)
                            .offset(i as isize))
                            .zEName as *mut ::core::ffi::c_void,
                    );
                }
                i += 1;
            }
        }
    }
}
unsafe extern "C" fn renameTokenFree(
    mut db: *mut sqlite3,
    mut pToken: *mut RenameToken,
) {
    unsafe {
        let mut pNext: *mut RenameToken = ::core::ptr::null_mut::<RenameToken>();
        let mut p: *mut RenameToken = ::core::ptr::null_mut::<RenameToken>();
        p = pToken;
        while !p.is_null() {
            pNext = (*p).pNext;
            sqlite3DbFree(db, p as *mut ::core::ffi::c_void);
            p = pNext;
        }
    }
}
unsafe extern "C" fn renameTokenFind(
    mut pParse: *mut Parse,
    mut pCtx: *mut RenameCtx,
    mut pPtr: *const ::core::ffi::c_void,
) -> *mut RenameToken {
    unsafe {
        let mut pp: *mut *mut RenameToken = ::core::ptr::null_mut::<*mut RenameToken>();
        if pPtr.is_null() {
            return ::core::ptr::null_mut::<RenameToken>();
        }
        pp = &raw mut (*pParse).pRename;
        while !(*pp).is_null() {
            if (**pp).p == pPtr {
                let mut pToken: *mut RenameToken = *pp;
                if !pCtx.is_null() {
                    *pp = (*pToken).pNext;
                    (*pToken).pNext = (*pCtx).pList;
                    (*pCtx).pList = pToken;
                    (*pCtx).nList += 1;
                }
                return pToken;
            }
            pp = &raw mut (**pp).pNext;
        }
        return ::core::ptr::null_mut::<RenameToken>();
    }
}
unsafe extern "C" fn renameColumnSelectCb(
    mut pWalker: *mut Walker,
    mut p: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        if (*p).selFlags & (SF_View | SF_CopyCte) as u32_0 != 0 {
            return WRC_Prune;
        }
        renameWalkWith(pWalker, p);
        return WRC_Continue;
    }
}
unsafe extern "C" fn renameColumnExprCb(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut RenameCtx = (*pWalker).u.pRename as *mut RenameCtx;
        if (*pExpr).op as ::core::ffi::c_int == TK_TRIGGER
            && (*pExpr).iColumn as ::core::ffi::c_int == (*p).iCol
            && (*(*pWalker).pParse).pTriggerTab == (*p).pTab
        {
            renameTokenFind(
                (*pWalker).pParse,
                p as *mut RenameCtx,
                pExpr as *mut ::core::ffi::c_void,
            );
        } else if (*pExpr).op as ::core::ffi::c_int == TK_COLUMN
            && (*pExpr).iColumn as ::core::ffi::c_int == (*p).iCol
            && (*pExpr).flags
                & (0x1000000 as ::core::ffi::c_int | 0x2000000 as ::core::ffi::c_int)
                    as u32_0 == 0 as u32_0 && (*p).pTab == (*pExpr).y.pTab
        {
            renameTokenFind(
                (*pWalker).pParse,
                p as *mut RenameCtx,
                pExpr as *mut ::core::ffi::c_void,
            );
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn renameColumnTokenNext(
    mut pCtx: *mut RenameCtx,
) -> *mut RenameToken {
    unsafe {
        let mut pBest: *mut RenameToken = (*pCtx).pList;
        let mut pToken: *mut RenameToken = ::core::ptr::null_mut::<RenameToken>();
        let mut pp: *mut *mut RenameToken = ::core::ptr::null_mut::<*mut RenameToken>();
        pToken = (*pBest).pNext;
        while !pToken.is_null() {
            if (*pToken).t.z > (*pBest).t.z {
                pBest = pToken;
            }
            pToken = (*pToken).pNext;
        }
        pp = &raw mut (*pCtx).pList;
        while *pp != pBest {
            pp = &raw mut (**pp).pNext;
        }
        *pp = (*pBest).pNext;
        return pBest;
    }
}
unsafe extern "C" fn renameColumnParseError(
    mut pCtx: *mut sqlite3_context,
    mut zWhen: *const ::core::ffi::c_char,
    mut pType: *mut sqlite3_value,
    mut pObject: *mut sqlite3_value,
    mut pParse: *mut Parse,
) {
    unsafe {
        let mut zT: *const ::core::ffi::c_char = sqlite3_value_text(pType)
            as *const ::core::ffi::c_char;
        let mut zN: *const ::core::ffi::c_char = sqlite3_value_text(pObject)
            as *const ::core::ffi::c_char;
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        zErr = sqlite3MPrintf(
            (*pParse).db,
            b"error in %s %s%s%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            zT,
            zN,
            if *zWhen.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            {
                b" \0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            zWhen,
            (*pParse).zErrMsg,
        );
        sqlite3_result_error(pCtx, zErr, -1 as ::core::ffi::c_int);
        sqlite3DbFree((*pParse).db, zErr as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn renameColumnElistNames(
    mut pParse: *mut Parse,
    mut pCtx: *mut RenameCtx,
    mut pEList: *const ExprList,
    mut zOld: *const ::core::ffi::c_char,
) {
    unsafe {
        if !pEList.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pEList).nExpr {
                let mut zName: *const ::core::ffi::c_char = (*(&raw const (*pEList).a
                    as *const ExprList_item)
                    .offset(i as isize))
                    .zEName;
                if (*(&raw const (*pEList).a as *const ExprList_item).offset(i as isize))
                    .fg
                    .eEName() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && !zName.is_null()
                    && 0 as ::core::ffi::c_int == sqlite3_stricmp(zName, zOld)
                {
                    renameTokenFind(
                        pParse,
                        pCtx as *mut RenameCtx,
                        zName as *const ::core::ffi::c_void,
                    );
                }
                i += 1;
            }
        }
    }
}
unsafe extern "C" fn renameColumnIdlistNames(
    mut pParse: *mut Parse,
    mut pCtx: *mut RenameCtx,
    mut pIdList: *const IdList,
    mut zOld: *const ::core::ffi::c_char,
) {
    unsafe {
        if !pIdList.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pIdList).nId {
                let mut zName: *const ::core::ffi::c_char = (*(&raw const (*pIdList).a
                    as *const IdList_item)
                    .offset(i as isize))
                    .zName;
                if 0 as ::core::ffi::c_int == sqlite3_stricmp(zName, zOld) {
                    renameTokenFind(
                        pParse,
                        pCtx as *mut RenameCtx,
                        zName as *const ::core::ffi::c_void,
                    );
                }
                i += 1;
            }
        }
    }
}
unsafe extern "C" fn renameParseSql(
    mut p: *mut Parse,
    mut zDb: *const ::core::ffi::c_char,
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_char,
    mut bTemp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut flags: u64_0 = 0;
        sqlite3ParseObjectInit(p, db);
        if zSql.is_null() {
            return SQLITE_NOMEM;
        }
        if sqlite3_strnicmp(
            zSql,
            b"CREATE \0".as_ptr() as *const ::core::ffi::c_char,
            7 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int
        {
            return sqlite3CorruptError(1146 as ::core::ffi::c_int);
        }
        if bTemp != 0 {
            (*db).init.iDb = 1 as u8_0;
        } else {
            let mut iDb: ::core::ffi::c_int = sqlite3FindDbName(db, zDb);
            (*db).init.iDb = iDb as u8_0;
        }
        (*p).eParseMode = PARSE_MODE_RENAME as u8_0;
        (*p).db = db;
        (*p).nQueryLoop = 1 as LogEst;
        flags = (*db).flags;
        (*db).flags |= SQLITE_Comments as ::core::ffi::c_ulonglong;
        rc = sqlite3RunParser(p, zSql);
        (*db).flags = flags;
        if (*db).mallocFailed != 0 {
            rc = SQLITE_NOMEM;
        }
        if rc == SQLITE_OK
            && ((*p).pNewTable.is_null() && (*p).pNewIndex.is_null()
                && (*p).pNewTrigger.is_null())
        {
            rc = sqlite3CorruptError(1167 as ::core::ffi::c_int);
        }
        (*db).init.iDb = 0 as u8_0;
        return rc;
    }
}
unsafe extern "C" fn renameEditSql(
    mut pCtx: *mut sqlite3_context,
    mut pRename: *mut RenameCtx,
    mut zSql: *const ::core::ffi::c_char,
    mut zNew: *const ::core::ffi::c_char,
    mut bQuote: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nNew: i64_0 = sqlite3Strlen30(zNew) as i64_0;
        let mut nSql: i64_0 = sqlite3Strlen30(zSql) as i64_0;
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(pCtx);
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zQuot: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nQuot: i64_0 = 0 as i64_0;
        let mut zBuf1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zBuf2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if !zNew.is_null() {
            zQuot = sqlite3MPrintf(
                db,
                b"\"%w\" \0".as_ptr() as *const ::core::ffi::c_char,
                zNew,
            );
            if zQuot.is_null() {
                return SQLITE_NOMEM
            } else {
                nQuot = (sqlite3Strlen30(zQuot) - 1 as ::core::ffi::c_int) as i64_0;
            }
            zOut = sqlite3DbMallocZero(
                db,
                (nSql as u64_0)
                    .wrapping_add(
                        ((*pRename).nList as u64_0).wrapping_mul(nQuot as u64_0),
                    )
                    .wrapping_add(1 as u64_0),
            ) as *mut ::core::ffi::c_char;
        } else {
            zOut = sqlite3DbMallocZero(
                db,
                (2 as u64_0)
                    .wrapping_mul(nSql as u64_0)
                    .wrapping_add(1 as u64_0)
                    .wrapping_mul(3 as u64_0),
            ) as *mut ::core::ffi::c_char;
            if !zOut.is_null() {
                zBuf1 = zOut
                    .offset(
                        (nSql as ::core::ffi::c_longlong * 2 as ::core::ffi::c_longlong
                            + 1 as ::core::ffi::c_longlong) as isize,
                    ) as *mut ::core::ffi::c_char;
                zBuf2 = zOut
                    .offset(
                        (nSql as ::core::ffi::c_longlong * 4 as ::core::ffi::c_longlong
                            + 2 as ::core::ffi::c_longlong) as isize,
                    ) as *mut ::core::ffi::c_char;
            }
        }
        if !zOut.is_null() {
            let mut nOut: i64_0 = nSql;
            memcpy(
                zOut as *mut ::core::ffi::c_void,
                zSql as *const ::core::ffi::c_void,
                nSql as size_t,
            );
            while !(*pRename).pList.is_null() {
                let mut iOff: ::core::ffi::c_int = 0;
                let mut nReplace: i64_0 = 0;
                let mut zReplace: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut pBest: *mut RenameToken = renameColumnTokenNext(pRename);
                if !zNew.is_null() {
                    if bQuote == 0 as ::core::ffi::c_int
                        && sqlite3IsIdChar(*((*pBest).t.z as *mut u8_0)) != 0
                    {
                        nReplace = nNew;
                        zReplace = zNew;
                    } else {
                        nReplace = nQuot;
                        zReplace = zQuot;
                        if *(*pBest).t.z.offset((*pBest).t.n as isize)
                            as ::core::ffi::c_int == '"' as i32
                        {
                            nReplace += 1;
                        }
                    }
                } else {
                    memcpy(
                        zBuf1 as *mut ::core::ffi::c_void,
                        (*pBest).t.z as *const ::core::ffi::c_void,
                        (*pBest).t.n as size_t,
                    );
                    *zBuf1.offset((*pBest).t.n as isize) = 0 as ::core::ffi::c_char;
                    sqlite3Dequote(zBuf1);
                    sqlite3_snprintf(
                        (nSql as ::core::ffi::c_longlong * 2 as ::core::ffi::c_longlong)
                            as ::core::ffi::c_int,
                        zBuf2,
                        b"%Q%s\0".as_ptr() as *const ::core::ffi::c_char,
                        zBuf1,
                        if *(*pBest).t.z.offset((*pBest).t.n as isize)
                            as ::core::ffi::c_int == '\'' as i32
                        {
                            b" \0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                    zReplace = zBuf2;
                    nReplace = sqlite3Strlen30(zReplace) as i64_0;
                }
                iOff = (*pBest).t.z.offset_from(zSql) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
                if (*pBest).t.n as ::core::ffi::c_longlong != nReplace {
                    memmove(
                        zOut.offset((iOff as i64_0 + nReplace) as isize)
                            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        zOut
                            .offset(
                                (iOff as ::core::ffi::c_uint).wrapping_add((*pBest).t.n)
                                    as isize,
                            ) as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                        (nOut as ::core::ffi::c_longlong
                            - (iOff as ::core::ffi::c_uint).wrapping_add((*pBest).t.n)
                                as ::core::ffi::c_longlong) as size_t,
                    );
                    nOut
                        += nReplace as ::core::ffi::c_longlong
                            - (*pBest).t.n as ::core::ffi::c_longlong;
                    *zOut.offset(nOut as isize) = '\0' as i32 as ::core::ffi::c_char;
                }
                memcpy(
                    zOut.offset(iOff as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    zReplace as *const ::core::ffi::c_void,
                    nReplace as size_t,
                );
                sqlite3DbFree(db, pBest as *mut ::core::ffi::c_void);
            }
            sqlite3_result_text(
                pCtx,
                zOut,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3DbFree(db, zOut as *mut ::core::ffi::c_void);
        } else {
            rc = SQLITE_NOMEM;
        }
        sqlite3_free(zQuot as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn renameSetENames(
    mut pEList: *mut ExprList,
    mut val: ::core::ffi::c_int,
) {
    unsafe {
        if !pEList.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pEList).nExpr {
                let ref mut c2rust_fresh0 = (*(&raw mut (*pEList).a
                    as *mut ExprList_item)
                    .offset(i as isize))
                    .fg;
                (*c2rust_fresh0)
                    .set_eEName(
                        (val & 0x3 as ::core::ffi::c_int) as ::core::ffi::c_uint
                            as ::core::ffi::c_uint,
                    );
                i += 1;
            }
        }
    }
}
unsafe extern "C" fn renameResolveTrigger(mut pParse: *mut Parse) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pNew: *mut Trigger = (*pParse).pNewTrigger;
        let mut pStep: *mut TriggerStep = ::core::ptr::null_mut::<TriggerStep>();
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
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        memset(
            &raw mut sNC as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<NameContext>() as size_t,
        );
        sNC.pParse = pParse;
        (*pParse).pTriggerTab = sqlite3FindTable(
            db,
            (*pNew).table,
            (*(*db).aDb.offset(sqlite3SchemaToIndex(db, (*pNew).pTabSchema) as isize))
                .zDbSName,
        );
        (*pParse).eTriggerOp = (*pNew).op;
        if !(*pParse).pTriggerTab.is_null() {
            rc = (sqlite3ViewGetColumnNames(pParse, (*pParse).pTriggerTab)
                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
        if rc == SQLITE_OK && !(*pNew).pWhen.is_null() {
            rc = sqlite3ResolveExprNames(&raw mut sNC, (*pNew).pWhen);
        }
        pStep = (*pNew).step_list;
        while rc == SQLITE_OK && !pStep.is_null() {
            if !(*pStep).pSelect.is_null() {
                sqlite3SelectPrep(pParse, (*pStep).pSelect, &raw mut sNC);
                if (*pParse).nErr != 0 {
                    rc = (*pParse).rc;
                }
            }
            if rc == SQLITE_OK && !(*pStep).zTarget.is_null() {
                let mut pSrc: *mut SrcList = sqlite3TriggerStepSrc(pParse, pStep);
                if !pSrc.is_null() {
                    let mut pSel: *mut Select = sqlite3SelectNew(
                        pParse,
                        (*pStep).pExprList,
                        pSrc,
                        ::core::ptr::null_mut::<Expr>(),
                        ::core::ptr::null_mut::<ExprList>(),
                        ::core::ptr::null_mut::<Expr>(),
                        ::core::ptr::null_mut::<ExprList>(),
                        0 as u32_0,
                        ::core::ptr::null_mut::<Expr>(),
                    );
                    if pSel.is_null() {
                        (*pStep).pExprList = ::core::ptr::null_mut::<ExprList>();
                        pSrc = ::core::ptr::null_mut::<SrcList>();
                        rc = SQLITE_NOMEM;
                    } else {
                        renameSetENames((*pStep).pExprList, ENAME_SPAN);
                        sqlite3SelectPrep(
                            pParse,
                            pSel,
                            ::core::ptr::null_mut::<NameContext>(),
                        );
                        renameSetENames((*pStep).pExprList, ENAME_NAME);
                        rc = if (*pParse).nErr != 0 { SQLITE_ERROR } else { SQLITE_OK };
                        if !(*pStep).pExprList.is_null() {
                            (*pSel).pEList = ::core::ptr::null_mut::<ExprList>();
                        }
                        (*pSel).pSrc = ::core::ptr::null_mut::<SrcList>();
                        sqlite3SelectDelete(db, pSel);
                    }
                    if !(*pStep).pFrom.is_null() {
                        let mut i: ::core::ffi::c_int = 0;
                        i = 0 as ::core::ffi::c_int;
                        while i < (*(*pStep).pFrom).nSrc && rc == SQLITE_OK {
                            let mut p: *mut SrcItem = (&raw mut (*(*pStep).pFrom).a
                                as *mut SrcItem)
                                .offset(i as isize) as *mut SrcItem;
                            if (*p).fg.isSubquery() != 0 {
                                sqlite3SelectPrep(
                                    pParse,
                                    (*(*p).u4.pSubq).pSelect,
                                    ::core::ptr::null_mut::<NameContext>(),
                                );
                            }
                            i += 1;
                        }
                    }
                    if (*db).mallocFailed != 0 {
                        rc = SQLITE_NOMEM;
                    }
                    sNC.pSrcList = pSrc;
                    if rc == SQLITE_OK && !(*pStep).pWhere.is_null() {
                        rc = sqlite3ResolveExprNames(&raw mut sNC, (*pStep).pWhere);
                    }
                    if rc == SQLITE_OK {
                        rc = sqlite3ResolveExprListNames(
                            &raw mut sNC,
                            (*pStep).pExprList,
                        );
                    }
                    if !(*pStep).pUpsert.is_null() && rc == SQLITE_OK {
                        let mut pUpsert: *mut Upsert = (*pStep).pUpsert;
                        (*pUpsert).pUpsertSrc = pSrc;
                        sNC.uNC.pUpsert = pUpsert;
                        sNC.ncFlags = NC_UUpsert;
                        rc = sqlite3ResolveExprListNames(
                            &raw mut sNC,
                            (*pUpsert).pUpsertTarget,
                        );
                        if rc == SQLITE_OK {
                            let mut pUpsertSet: *mut ExprList = (*pUpsert).pUpsertSet;
                            rc = sqlite3ResolveExprListNames(&raw mut sNC, pUpsertSet);
                        }
                        if rc == SQLITE_OK {
                            rc = sqlite3ResolveExprNames(
                                &raw mut sNC,
                                (*pUpsert).pUpsertWhere,
                            );
                        }
                        if rc == SQLITE_OK {
                            rc = sqlite3ResolveExprNames(
                                &raw mut sNC,
                                (*pUpsert).pUpsertTargetWhere,
                            );
                        }
                        sNC.ncFlags = 0 as ::core::ffi::c_int;
                    }
                    sNC.pSrcList = ::core::ptr::null_mut::<SrcList>();
                    sqlite3SrcListDelete(db, pSrc);
                } else {
                    rc = SQLITE_NOMEM;
                }
            }
            pStep = (*pStep).pNext;
        }
        return rc;
    }
}
unsafe extern "C" fn renameWalkTrigger(
    mut pWalker: *mut Walker,
    mut pTrigger: *mut Trigger,
) {
    unsafe {
        let mut pStep: *mut TriggerStep = ::core::ptr::null_mut::<TriggerStep>();
        sqlite3WalkExpr(pWalker, (*pTrigger).pWhen);
        pStep = (*pTrigger).step_list;
        while !pStep.is_null() {
            sqlite3WalkSelect(pWalker, (*pStep).pSelect);
            sqlite3WalkExpr(pWalker, (*pStep).pWhere);
            sqlite3WalkExprList(pWalker, (*pStep).pExprList);
            if !(*pStep).pUpsert.is_null() {
                let mut pUpsert: *mut Upsert = (*pStep).pUpsert;
                sqlite3WalkExprList(pWalker, (*pUpsert).pUpsertTarget);
                sqlite3WalkExprList(pWalker, (*pUpsert).pUpsertSet);
                sqlite3WalkExpr(pWalker, (*pUpsert).pUpsertWhere);
                sqlite3WalkExpr(pWalker, (*pUpsert).pUpsertTargetWhere);
            }
            if !(*pStep).pFrom.is_null() {
                let mut i: ::core::ffi::c_int = 0;
                let mut pFrom: *mut SrcList = (*pStep).pFrom;
                i = 0 as ::core::ffi::c_int;
                while i < (*pFrom).nSrc {
                    if (*(&raw mut (*pFrom).a as *mut SrcItem).offset(i as isize))
                        .fg
                        .isSubquery() != 0
                    {
                        sqlite3WalkSelect(
                            pWalker,
                            (*(*(&raw mut (*pFrom).a as *mut SrcItem).offset(i as isize))
                                .u4
                                .pSubq)
                                .pSelect,
                        );
                    }
                    i += 1;
                }
            }
            pStep = (*pStep).pNext;
        }
    }
}
unsafe extern "C" fn renameParseCleanup(mut pParse: *mut Parse) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        if !(*pParse).pVdbe.is_null() {
            sqlite3VdbeFinalize((*pParse).pVdbe);
        }
        sqlite3DeleteTable(db, (*pParse).pNewTable);
        loop {
            pIdx = (*pParse).pNewIndex;
            if pIdx.is_null() {
                break;
            }
            (*pParse).pNewIndex = (*pIdx).pNext;
            sqlite3FreeIndex(db, pIdx);
        }
        sqlite3DeleteTrigger(db, (*pParse).pNewTrigger);
        sqlite3DbFree(db, (*pParse).zErrMsg as *mut ::core::ffi::c_void);
        renameTokenFree(db, (*pParse).pRename);
        sqlite3ParseObjectReset(pParse);
    }
}
unsafe extern "C" fn renameColumnFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut sCtx: RenameCtx = RenameCtx {
            pList: ::core::ptr::null_mut::<RenameToken>(),
            nList: 0,
            iCol: 0,
            pTab: ::core::ptr::null_mut::<Table>(),
            zOld: ::core::ptr::null::<::core::ffi::c_char>()

        };
        let mut zSql: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut zDb: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(3 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut zTable: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(4 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut iCol: ::core::ffi::c_int = sqlite3_value_int(
            *argv.offset(5 as ::core::ffi::c_int as isize),
        );
        let mut zNew: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(6 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut bQuote: ::core::ffi::c_int = sqlite3_value_int(
            *argv.offset(7 as ::core::ffi::c_int as isize),
        );
        let mut bTemp: ::core::ffi::c_int = sqlite3_value_int(
            *argv.offset(8 as ::core::ffi::c_int as isize),
        );
        let mut zOld: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut sParse: Parse = Parse {
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
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut i: ::core::ffi::c_int = 0;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut xAuth: sqlite3_xauth = (*db).xAuth;
        if zSql.is_null() {
            return;
        }
        if zTable.is_null() {
            return;
        }
        if zNew.is_null() {
            return;
        }
        if iCol < 0 as ::core::ffi::c_int {
            return;
        }
        sqlite3BtreeEnterAll(db);
        pTab = sqlite3FindTable(db, zTable, zDb);
        if pTab.is_null() || iCol >= (*pTab).nCol as ::core::ffi::c_int {
            sqlite3BtreeLeaveAll(db);
            return;
        }
        zOld = (*(*pTab).aCol.offset(iCol as isize)).zCnName;
        memset(
            &raw mut sCtx as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<RenameCtx>() as size_t,
        );
        sCtx.iCol = if iCol == (*pTab).iPKey as ::core::ffi::c_int {
            -1 as ::core::ffi::c_int
        } else {
            iCol
        };
        (*db).xAuth = None;
        rc = renameParseSql(&raw mut sParse, zDb, db, zSql, bTemp);
        memset(
            &raw mut sWalker as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        sWalker.pParse = &raw mut sParse;
        sWalker.xExprCallback = Some(
            renameColumnExprCb
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        sWalker.xSelectCallback = Some(
            renameColumnSelectCb
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        sWalker.u.pRename = &raw mut sCtx as *mut RenameCtx;
        sCtx.pTab = pTab;
        if !(rc != SQLITE_OK) {
            if !sParse.pNewTable.is_null() {
                if (*sParse.pNewTable).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
                    let mut pSelect: *mut Select = (*sParse.pNewTable).u.view.pSelect;
                    (*pSelect).selFlags = ((*pSelect).selFlags as ::core::ffi::c_uint
                        & !(SF_View as u32_0) as ::core::ffi::c_uint) as u32_0;
                    sParse.rc = SQLITE_OK;
                    sqlite3SelectPrep(
                        &raw mut sParse,
                        pSelect,
                        ::core::ptr::null_mut::<NameContext>(),
                    );
                    rc = if (*db).mallocFailed as ::core::ffi::c_int != 0 {
                        SQLITE_NOMEM
                    } else {
                        sParse.rc
                    };
                    if rc == SQLITE_OK {
                        sqlite3WalkSelect(&raw mut sWalker, pSelect);
                    }
                    if rc != SQLITE_OK {
                        c2rust_current_block = 14784926368341588560;
                    } else {
                        c2rust_current_block = 17688141731389699982;
                    }
                } else {
                    if (*sParse.pNewTable).eTabType as ::core::ffi::c_int == TABTYP_NORM
                    {
                        let mut bFKOnly: ::core::ffi::c_int = sqlite3_stricmp(
                            zTable,
                            (*sParse.pNewTable).zName,
                        );
                        let mut pFKey: *mut FKey = ::core::ptr::null_mut::<FKey>();
                        sCtx.pTab = sParse.pNewTable;
                        if bFKOnly == 0 as ::core::ffi::c_int {
                            if iCol < (*sParse.pNewTable).nCol as ::core::ffi::c_int {
                                renameTokenFind(
                                    &raw mut sParse,
                                    &raw mut sCtx,
                                    (*(*sParse.pNewTable).aCol.offset(iCol as isize)).zCnName
                                        as *mut ::core::ffi::c_void,
                                );
                            }
                            if sCtx.iCol < 0 as ::core::ffi::c_int {
                                renameTokenFind(
                                    &raw mut sParse,
                                    &raw mut sCtx,
                                    &raw mut (*sParse.pNewTable).iPKey
                                        as *mut ::core::ffi::c_void,
                                );
                            }
                            sqlite3WalkExprList(
                                &raw mut sWalker,
                                (*sParse.pNewTable).pCheck,
                            );
                            pIdx = (*sParse.pNewTable).pIndex;
                            while !pIdx.is_null() {
                                sqlite3WalkExprList(&raw mut sWalker, (*pIdx).aColExpr);
                                pIdx = (*pIdx).pNext;
                            }
                            pIdx = sParse.pNewIndex;
                            while !pIdx.is_null() {
                                sqlite3WalkExprList(&raw mut sWalker, (*pIdx).aColExpr);
                                pIdx = (*pIdx).pNext;
                            }
                            i = 0 as ::core::ffi::c_int;
                            while i < (*sParse.pNewTable).nCol as ::core::ffi::c_int {
                                let mut pExpr: *mut Expr = sqlite3ColumnExpr(
                                    sParse.pNewTable,
                                    (*sParse.pNewTable).aCol.offset(i as isize) as *mut Column,
                                );
                                sqlite3WalkExpr(&raw mut sWalker, pExpr);
                                i += 1;
                            }
                        }
                        pFKey = (*sParse.pNewTable).u.tab.pFKey;
                        while !pFKey.is_null() {
                            i = 0 as ::core::ffi::c_int;
                            while i < (*pFKey).nCol {
                                if bFKOnly == 0 as ::core::ffi::c_int
                                    && (*(&raw mut (*pFKey).aCol as *mut sColMap)
                                        .offset(i as isize))
                                        .iFrom == iCol
                                {
                                    renameTokenFind(
                                        &raw mut sParse,
                                        &raw mut sCtx,
                                        (&raw mut (*pFKey).aCol as *mut sColMap).offset(i as isize)
                                            as *mut sColMap as *mut ::core::ffi::c_void,
                                    );
                                }
                                if 0 as ::core::ffi::c_int
                                    == sqlite3_stricmp((*pFKey).zTo, zTable)
                                    && 0 as ::core::ffi::c_int
                                        == sqlite3_stricmp(
                                            (*(&raw mut (*pFKey).aCol as *mut sColMap)
                                                .offset(i as isize))
                                                .zCol,
                                            zOld,
                                        )
                                {
                                    renameTokenFind(
                                        &raw mut sParse,
                                        &raw mut sCtx,
                                        (*(&raw mut (*pFKey).aCol as *mut sColMap)
                                            .offset(i as isize))
                                            .zCol as *mut ::core::ffi::c_void,
                                    );
                                }
                                i += 1;
                            }
                            pFKey = (*pFKey).pNextFrom;
                        }
                    }
                    c2rust_current_block = 17688141731389699982;
                }
            } else if !sParse.pNewIndex.is_null() {
                sqlite3WalkExprList(&raw mut sWalker, (*sParse.pNewIndex).aColExpr);
                sqlite3WalkExpr(&raw mut sWalker, (*sParse.pNewIndex).pPartIdxWhere);
                c2rust_current_block = 17688141731389699982;
            } else {
                let mut pStep: *mut TriggerStep = ::core::ptr::null_mut::<TriggerStep>();
                rc = renameResolveTrigger(&raw mut sParse);
                if rc != SQLITE_OK {
                    c2rust_current_block = 14784926368341588560;
                } else {
                    pStep = (*sParse.pNewTrigger).step_list;
                    while !pStep.is_null() {
                        if !(*pStep).zTarget.is_null() {
                            let mut pTarget: *mut Table = sqlite3LocateTable(
                                &raw mut sParse,
                                0 as u32_0,
                                (*pStep).zTarget,
                                zDb,
                            );
                            if pTarget == pTab {
                                if !(*pStep).pUpsert.is_null() {
                                    let mut pUpsertSet: *mut ExprList = (*(*pStep).pUpsert)
                                        .pUpsertSet;
                                    renameColumnElistNames(
                                        &raw mut sParse,
                                        &raw mut sCtx,
                                        pUpsertSet,
                                        zOld,
                                    );
                                }
                                renameColumnIdlistNames(
                                    &raw mut sParse,
                                    &raw mut sCtx,
                                    (*pStep).pIdList,
                                    zOld,
                                );
                                renameColumnElistNames(
                                    &raw mut sParse,
                                    &raw mut sCtx,
                                    (*pStep).pExprList,
                                    zOld,
                                );
                            }
                        }
                        pStep = (*pStep).pNext;
                    }
                    if sParse.pTriggerTab == pTab {
                        renameColumnIdlistNames(
                            &raw mut sParse,
                            &raw mut sCtx,
                            (*sParse.pNewTrigger).pColumns,
                            zOld,
                        );
                    }
                    renameWalkTrigger(&raw mut sWalker, sParse.pNewTrigger);
                    c2rust_current_block = 17688141731389699982;
                }
            }
            match c2rust_current_block {
                14784926368341588560 => {}
                _ => {
                    rc = renameEditSql(context, &raw mut sCtx, zSql, zNew, bQuote);
                }
            }
        }
        if rc != SQLITE_OK {
            if rc == SQLITE_ERROR && sqlite3WritableSchema(db) != 0 {
                sqlite3_result_value(
                    context,
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
            } else if !sParse.zErrMsg.is_null() {
                renameColumnParseError(
                    context,
                    b"\0".as_ptr() as *const ::core::ffi::c_char,
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                    *argv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut sParse,
                );
            } else {
                sqlite3_result_error_code(context, rc);
            }
        }
        renameParseCleanup(&raw mut sParse);
        renameTokenFree(db, sCtx.pList);
        (*db).xAuth = xAuth;
        sqlite3BtreeLeaveAll(db);
    }
}
unsafe extern "C" fn renameTableExprCb(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut RenameCtx = (*pWalker).u.pRename as *mut RenameCtx;
        if (*pExpr).op as ::core::ffi::c_int == TK_COLUMN
            && (*pExpr).flags
                & (0x1000000 as ::core::ffi::c_int | 0x2000000 as ::core::ffi::c_int)
                    as u32_0 == 0 as u32_0 && (*p).pTab == (*pExpr).y.pTab
        {
            renameTokenFind(
                (*pWalker).pParse,
                p as *mut RenameCtx,
                &raw mut (*pExpr).y.pTab as *mut ::core::ffi::c_void,
            );
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn renameTableSelectCb(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut p: *mut RenameCtx = (*pWalker).u.pRename as *mut RenameCtx;
        let mut pSrc: *mut SrcList = (*pSelect).pSrc;
        if (*pSelect).selFlags & (SF_View | SF_CopyCte) as u32_0 != 0 {
            return WRC_Prune;
        }
        if pSrc.is_null() {
            return WRC_Abort;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pSrc).nSrc {
            let mut pItem: *mut SrcItem = (&raw mut (*pSrc).a as *mut SrcItem)
                .offset(i as isize) as *mut SrcItem;
            if (*pItem).pSTab == (*p).pTab {
                renameTokenFind(
                    (*pWalker).pParse,
                    p as *mut RenameCtx,
                    (*pItem).zName as *const ::core::ffi::c_void,
                );
            }
            i += 1;
        }
        renameWalkWith(pWalker, pSelect);
        return WRC_Continue;
    }
}
unsafe extern "C" fn renameTableFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut zDb: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut zInput: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(3 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut zOld: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(4 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut zNew: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(5 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut bTemp: ::core::ffi::c_int = sqlite3_value_int(
            *argv.offset(6 as ::core::ffi::c_int as isize),
        );
        if !zInput.is_null() && !zOld.is_null() && !zNew.is_null() {
            let mut sParse: Parse = Parse {
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
            let mut rc: ::core::ffi::c_int = 0;
            let mut bQuote: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut sCtx: RenameCtx = RenameCtx {
                pList: ::core::ptr::null_mut::<RenameToken>(),
                nList: 0,
                iCol: 0,
                pTab: ::core::ptr::null_mut::<Table>(),
                zOld: ::core::ptr::null::<::core::ffi::c_char>()

            };
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
            let mut xAuth: sqlite3_xauth = (*db).xAuth;
            (*db).xAuth = None;
            sqlite3BtreeEnterAll(db);
            memset(
                &raw mut sCtx as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<RenameCtx>() as size_t,
            );
            sCtx.pTab = sqlite3FindTable(db, zOld, zDb);
            memset(
                &raw mut sWalker as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Walker>() as size_t,
            );
            sWalker.pParse = &raw mut sParse;
            sWalker.xExprCallback = Some(
                renameTableExprCb
                    as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
                >;
            sWalker.xSelectCallback = Some(
                renameTableSelectCb
                    as unsafe extern "C" fn(
                        *mut Walker,
                        *mut Select,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
                >;
            sWalker.u.pRename = &raw mut sCtx as *mut RenameCtx;
            rc = renameParseSql(&raw mut sParse, zDb, db, zInput, bTemp);
            if rc == SQLITE_OK {
                let mut isLegacy: ::core::ffi::c_int = ((*db).flags
                    as ::core::ffi::c_ulonglong
                    & SQLITE_LegacyAlter as ::core::ffi::c_ulonglong)
                    as ::core::ffi::c_int;
                if !sParse.pNewTable.is_null() {
                    let mut pTab: *mut Table = sParse.pNewTable;
                    if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
                        if isLegacy == 0 as ::core::ffi::c_int {
                            let mut pSelect: *mut Select = (*pTab).u.view.pSelect;
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
                            sNC.pParse = &raw mut sParse;
                            (*pSelect).selFlags = ((*pSelect).selFlags
                                as ::core::ffi::c_uint
                                & !(SF_View as u32_0) as ::core::ffi::c_uint) as u32_0;
                            sqlite3SelectPrep(
                                &raw mut sParse,
                                (*pTab).u.view.pSelect,
                                &raw mut sNC,
                            );
                            if sParse.nErr != 0 {
                                rc = sParse.rc;
                            } else {
                                sqlite3WalkSelect(&raw mut sWalker, (*pTab).u.view.pSelect);
                            }
                        }
                    } else {
                        if (isLegacy == 0 as ::core::ffi::c_int
                            || (*db).flags as ::core::ffi::c_ulonglong
                                & SQLITE_ForeignKeys as ::core::ffi::c_ulonglong != 0)
                            && !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB)
                        {
                            let mut pFKey: *mut FKey = ::core::ptr::null_mut::<FKey>();
                            pFKey = (*pTab).u.tab.pFKey;
                            while !pFKey.is_null() {
                                if sqlite3_stricmp((*pFKey).zTo, zOld)
                                    == 0 as ::core::ffi::c_int
                                {
                                    renameTokenFind(
                                        &raw mut sParse,
                                        &raw mut sCtx,
                                        (*pFKey).zTo as *mut ::core::ffi::c_void,
                                    );
                                }
                                pFKey = (*pFKey).pNextFrom;
                            }
                        }
                        if sqlite3_stricmp(zOld, (*pTab).zName)
                            == 0 as ::core::ffi::c_int
                        {
                            sCtx.pTab = pTab;
                            if isLegacy == 0 as ::core::ffi::c_int {
                                sqlite3WalkExprList(&raw mut sWalker, (*pTab).pCheck);
                            }
                            renameTokenFind(
                                &raw mut sParse,
                                &raw mut sCtx,
                                (*pTab).zName as *const ::core::ffi::c_void,
                            );
                        }
                    }
                } else if !sParse.pNewIndex.is_null() {
                    renameTokenFind(
                        &raw mut sParse,
                        &raw mut sCtx,
                        (*sParse.pNewIndex).zName as *const ::core::ffi::c_void,
                    );
                    if isLegacy == 0 as ::core::ffi::c_int {
                        sqlite3WalkExpr(
                            &raw mut sWalker,
                            (*sParse.pNewIndex).pPartIdxWhere,
                        );
                    }
                } else {
                    let mut pTrigger: *mut Trigger = sParse.pNewTrigger;
                    let mut pStep: *mut TriggerStep = ::core::ptr::null_mut::<
                        TriggerStep,
                    >();
                    if 0 as ::core::ffi::c_int
                        == sqlite3_stricmp((*sParse.pNewTrigger).table, zOld)
                        && (*sCtx.pTab).pSchema == (*pTrigger).pTabSchema
                    {
                        renameTokenFind(
                            &raw mut sParse,
                            &raw mut sCtx,
                            (*sParse.pNewTrigger).table as *const ::core::ffi::c_void,
                        );
                    }
                    if isLegacy == 0 as ::core::ffi::c_int {
                        rc = renameResolveTrigger(&raw mut sParse);
                        if rc == SQLITE_OK {
                            renameWalkTrigger(&raw mut sWalker, pTrigger);
                            pStep = (*pTrigger).step_list;
                            while !pStep.is_null() {
                                if !(*pStep).zTarget.is_null()
                                    && 0 as ::core::ffi::c_int
                                        == sqlite3_stricmp((*pStep).zTarget, zOld)
                                {
                                    renameTokenFind(
                                        &raw mut sParse,
                                        &raw mut sCtx,
                                        (*pStep).zTarget as *const ::core::ffi::c_void,
                                    );
                                }
                                if !(*pStep).pFrom.is_null() {
                                    let mut i: ::core::ffi::c_int = 0;
                                    i = 0 as ::core::ffi::c_int;
                                    while i < (*(*pStep).pFrom).nSrc {
                                        let mut pItem: *mut SrcItem = (&raw mut (*(*pStep).pFrom).a
                                            as *mut SrcItem)
                                            .offset(i as isize) as *mut SrcItem;
                                        if 0 as ::core::ffi::c_int
                                            == sqlite3_stricmp((*pItem).zName, zOld)
                                        {
                                            renameTokenFind(
                                                &raw mut sParse,
                                                &raw mut sCtx,
                                                (*pItem).zName as *const ::core::ffi::c_void,
                                            );
                                        }
                                        i += 1;
                                    }
                                }
                                pStep = (*pStep).pNext;
                            }
                        }
                    }
                }
            }
            if rc == SQLITE_OK {
                rc = renameEditSql(context, &raw mut sCtx, zInput, zNew, bQuote);
            }
            if rc != SQLITE_OK {
                if rc == SQLITE_ERROR && sqlite3WritableSchema(db) != 0 {
                    sqlite3_result_value(
                        context,
                        *argv.offset(3 as ::core::ffi::c_int as isize),
                    );
                } else if !sParse.zErrMsg.is_null() {
                    renameColumnParseError(
                        context,
                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                        *argv.offset(1 as ::core::ffi::c_int as isize),
                        *argv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut sParse,
                    );
                } else {
                    sqlite3_result_error_code(context, rc);
                }
            }
            renameParseCleanup(&raw mut sParse);
            renameTokenFree(db, sCtx.pList);
            sqlite3BtreeLeaveAll(db);
            (*db).xAuth = xAuth;
        }
    }
}
unsafe extern "C" fn renameQuotefixExprCb(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int == TK_STRING
            && (*pExpr).flags & EP_DblQuoted as u32_0 != 0
        {
            renameTokenFind(
                (*pWalker).pParse,
                (*pWalker).u.pRename as *mut RenameCtx,
                pExpr as *const ::core::ffi::c_void,
            );
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn renameQuotefixFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut zDb: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut zInput: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(1 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut xAuth: sqlite3_xauth = (*db).xAuth;
        (*db).xAuth = None;
        sqlite3BtreeEnterAll(db);
        if !zDb.is_null() && !zInput.is_null() {
            let mut rc: ::core::ffi::c_int = 0;
            let mut sParse: Parse = Parse {
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
            rc = renameParseSql(
                &raw mut sParse,
                zDb,
                db,
                zInput,
                0 as ::core::ffi::c_int,
            );
            if rc == SQLITE_OK {
                let mut sCtx: RenameCtx = RenameCtx {
                    pList: ::core::ptr::null_mut::<RenameToken>(),
                    nList: 0,
                    iCol: 0,
                    pTab: ::core::ptr::null_mut::<Table>(),
                    zOld: ::core::ptr::null::<::core::ffi::c_char>()

                };
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
                    &raw mut sCtx as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<RenameCtx>() as size_t,
                );
                memset(
                    &raw mut sWalker as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<Walker>() as size_t,
                );
                sWalker.pParse = &raw mut sParse;
                sWalker.xExprCallback = Some(
                    renameQuotefixExprCb
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
                sWalker.xSelectCallback = Some(
                    renameColumnSelectCb
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
                sWalker.u.pRename = &raw mut sCtx as *mut RenameCtx;
                if !sParse.pNewTable.is_null() {
                    if (*sParse.pNewTable).eTabType as ::core::ffi::c_int == TABTYP_VIEW
                    {
                        let mut pSelect: *mut Select = (*sParse.pNewTable)
                            .u
                            .view
                            .pSelect;
                        (*pSelect).selFlags = ((*pSelect).selFlags as ::core::ffi::c_uint
                            & !(SF_View as u32_0) as ::core::ffi::c_uint) as u32_0;
                        sParse.rc = SQLITE_OK;
                        sqlite3SelectPrep(
                            &raw mut sParse,
                            pSelect,
                            ::core::ptr::null_mut::<NameContext>(),
                        );
                        rc = if (*db).mallocFailed as ::core::ffi::c_int != 0 {
                            SQLITE_NOMEM
                        } else {
                            sParse.rc
                        };
                        if rc == SQLITE_OK {
                            sqlite3WalkSelect(&raw mut sWalker, pSelect);
                        }
                    } else {
                        let mut i: ::core::ffi::c_int = 0;
                        sqlite3WalkExprList(
                            &raw mut sWalker,
                            (*sParse.pNewTable).pCheck,
                        );
                        i = 0 as ::core::ffi::c_int;
                        while i < (*sParse.pNewTable).nCol as ::core::ffi::c_int {
                            sqlite3WalkExpr(
                                &raw mut sWalker,
                                sqlite3ColumnExpr(
                                    sParse.pNewTable,
                                    (*sParse.pNewTable).aCol.offset(i as isize) as *mut Column,
                                ),
                            );
                            i += 1;
                        }
                    }
                } else if !sParse.pNewIndex.is_null() {
                    sqlite3WalkExprList(&raw mut sWalker, (*sParse.pNewIndex).aColExpr);
                    sqlite3WalkExpr(&raw mut sWalker, (*sParse.pNewIndex).pPartIdxWhere);
                } else {
                    rc = renameResolveTrigger(&raw mut sParse);
                    if rc == SQLITE_OK {
                        renameWalkTrigger(&raw mut sWalker, sParse.pNewTrigger);
                    }
                }
                if rc == SQLITE_OK {
                    rc = renameEditSql(
                        context,
                        &raw mut sCtx,
                        zInput,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        0 as ::core::ffi::c_int,
                    );
                }
                renameTokenFree(db, sCtx.pList);
            }
            if rc != SQLITE_OK {
                if sqlite3WritableSchema(db) != 0 && rc == SQLITE_ERROR {
                    sqlite3_result_value(
                        context,
                        *argv.offset(1 as ::core::ffi::c_int as isize),
                    );
                } else {
                    sqlite3_result_error_code(context, rc);
                }
            }
            renameParseCleanup(&raw mut sParse);
        }
        (*db).xAuth = xAuth;
        sqlite3BtreeLeaveAll(db);
    }
}
unsafe extern "C" fn renameTableTest(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut zDb: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut zInput: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(1 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut bTemp: ::core::ffi::c_int = sqlite3_value_int(
            *argv.offset(4 as ::core::ffi::c_int as isize),
        );
        let mut isLegacy: ::core::ffi::c_int = ((*db).flags as ::core::ffi::c_ulonglong
            & SQLITE_LegacyAlter as ::core::ffi::c_ulonglong) as ::core::ffi::c_int;
        let mut zWhen: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(5 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut bNoDQS: ::core::ffi::c_int = sqlite3_value_int(
            *argv.offset(6 as ::core::ffi::c_int as isize),
        );
        let mut xAuth: sqlite3_xauth = (*db).xAuth;
        (*db).xAuth = None;
        if !zDb.is_null() && !zInput.is_null() {
            let mut rc: ::core::ffi::c_int = 0;
            let mut sParse: Parse = Parse {
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
            let mut flags: u64_0 = (*db).flags;
            if bNoDQS != 0 {
                (*db).flags
                    &= !(SQLITE_DqsDML | SQLITE_DqsDDL) as ::core::ffi::c_ulonglong;
            }
            rc = renameParseSql(&raw mut sParse, zDb, db, zInput, bTemp);
            (*db).flags = flags;
            if rc == SQLITE_OK {
                if isLegacy == 0 as ::core::ffi::c_int && !sParse.pNewTable.is_null()
                    && (*sParse.pNewTable).eTabType as ::core::ffi::c_int == TABTYP_VIEW
                {
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
                    sNC.pParse = &raw mut sParse;
                    sqlite3SelectPrep(
                        &raw mut sParse,
                        (*sParse.pNewTable).u.view.pSelect,
                        &raw mut sNC,
                    );
                    if sParse.nErr != 0 {
                        rc = sParse.rc;
                    }
                } else if !sParse.pNewTrigger.is_null() {
                    if isLegacy == 0 as ::core::ffi::c_int {
                        rc = renameResolveTrigger(&raw mut sParse);
                    }
                    if rc == SQLITE_OK {
                        let mut i1: ::core::ffi::c_int = sqlite3SchemaToIndex(
                            db,
                            (*sParse.pNewTrigger).pTabSchema,
                        );
                        let mut i2: ::core::ffi::c_int = sqlite3FindDbName(db, zDb);
                        if i1 == i2 {
                            sqlite3_result_int(context, 1 as ::core::ffi::c_int);
                        }
                    }
                }
            }
            if rc != SQLITE_OK && !zWhen.is_null() && sqlite3WritableSchema(db) == 0 {
                renameColumnParseError(
                    context,
                    zWhen,
                    *argv.offset(2 as ::core::ffi::c_int as isize),
                    *argv.offset(3 as ::core::ffi::c_int as isize),
                    &raw mut sParse,
                );
            }
            renameParseCleanup(&raw mut sParse);
        }
        (*db).xAuth = xAuth;
    }
}
unsafe extern "C" fn dropColumnFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut iSchema: ::core::ffi::c_int = sqlite3_value_int(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        let mut zSql: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(1 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut iCol: ::core::ffi::c_int = sqlite3_value_int(
            *argv.offset(2 as ::core::ffi::c_int as isize),
        );
        let mut zDb: *const ::core::ffi::c_char = (*(*db).aDb.offset(iSchema as isize))
            .zDbSName;
        let mut rc: ::core::ffi::c_int = 0;
        let mut sParse: Parse = Parse {
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
        let mut pCol: *mut RenameToken = ::core::ptr::null_mut::<RenameToken>();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut zEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut xAuth: sqlite3_xauth = (*db).xAuth;
        (*db).xAuth = None;
        rc = renameParseSql(
            &raw mut sParse,
            zDb,
            db,
            zSql,
            (iSchema == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
        if !(rc != SQLITE_OK) {
            pTab = sParse.pNewTable;
            if pTab.is_null()
                || (*pTab).nCol as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                || iCol >= (*pTab).nCol as ::core::ffi::c_int
            {
                rc = sqlite3CorruptError(2135 as ::core::ffi::c_int);
            } else {
                pCol = renameTokenFind(
                    &raw mut sParse,
                    ::core::ptr::null_mut::<RenameCtx>(),
                    (*(*pTab).aCol.offset(iCol as isize)).zCnName
                        as *mut ::core::ffi::c_void,
                );
                if iCol < (*pTab).nCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
                    let mut pEnd: *mut RenameToken = ::core::ptr::null_mut::<
                        RenameToken,
                    >();
                    pEnd = renameTokenFind(
                        &raw mut sParse,
                        ::core::ptr::null_mut::<RenameCtx>(),
                        (*(*pTab).aCol.offset((iCol + 1 as ::core::ffi::c_int) as isize))
                            .zCnName as *mut ::core::ffi::c_void,
                    );
                    zEnd = (*pEnd).t.z;
                } else {
                    zEnd = zSql.offset((*pTab).u.tab.addColOffset as isize)
                        as *const ::core::ffi::c_char;
                    while *(*pCol).t.z.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                        && *(*pCol).t.z.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int != ',' as i32
                    {
                        (*pCol).t.z = (*pCol).t.z.offset(-1);
                    }
                }
                zNew = sqlite3MPrintf(
                    db,
                    b"%.*s%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pCol).t.z.offset_from(zSql) as ::core::ffi::c_long,
                    zSql,
                    zEnd,
                );
                sqlite3_result_text(
                    context,
                    zNew,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                sqlite3_free(zNew as *mut ::core::ffi::c_void);
            }
        }
        renameParseCleanup(&raw mut sParse);
        (*db).xAuth = xAuth;
        if rc != SQLITE_OK {
            sqlite3_result_error_code(context, rc);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AlterDropColumn(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pName: *const Token,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut iDb: ::core::ffi::c_int = 0;
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zCol: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut iCol: ::core::ffi::c_int = 0;
        if !((*db).mallocFailed != 0) {
            pTab = sqlite3LocateTableItem(
                pParse,
                0 as u32_0,
                (&raw mut (*pSrc).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem,
            );
            if !pTab.is_null() {
                if !(SQLITE_OK != isAlterableTable(pParse, pTab)) {
                    if !(SQLITE_OK != isRealTable(pParse, pTab, 1 as ::core::ffi::c_int))
                    {
                        zCol = sqlite3NameFromToken(db, pName);
                        if !zCol.is_null() {
                            iCol = sqlite3ColumnIndex(pTab, zCol);
                            if iCol < 0 as ::core::ffi::c_int {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"no such column: \"%T\"\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    pName,
                                );
                            } else if (*(*pTab).aCol.offset(iCol as isize)).colFlags
                                as ::core::ffi::c_int & (COLFLAG_PRIMKEY | COLFLAG_UNIQUE)
                                != 0
                            {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"cannot drop %s column: \"%s\"\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    if (*(*pTab).aCol.offset(iCol as isize)).colFlags
                                        as ::core::ffi::c_int & COLFLAG_PRIMKEY != 0
                                    {
                                        b"PRIMARY KEY\0".as_ptr() as *const ::core::ffi::c_char
                                    } else {
                                        b"UNIQUE\0".as_ptr() as *const ::core::ffi::c_char
                                    },
                                    zCol,
                                );
                            } else if (*pTab).nCol as ::core::ffi::c_int
                                <= 1 as ::core::ffi::c_int
                            {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"cannot drop column \"%s\": no other columns exist\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    zCol,
                                );
                            } else {
                                iDb = sqlite3SchemaToIndex(db, (*pTab).pSchema);
                                zDb = (*(*db).aDb.offset(iDb as isize)).zDbSName;
                                if !(sqlite3AuthCheck(
                                    pParse,
                                    SQLITE_ALTER_TABLE,
                                    zDb,
                                    (*pTab).zName,
                                    zCol,
                                ) != 0)
                                {
                                    renameTestSchema(
                                        pParse,
                                        zDb,
                                        (iDb == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                                        0 as ::core::ffi::c_int,
                                    );
                                    renameFixQuotes(
                                        pParse,
                                        zDb,
                                        (iDb == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                    );
                                    sqlite3NestedParse(
                                        pParse,
                                        b"UPDATE \"%w\".sqlite_master SET sql = sqlite_drop_column(%d, sql, %d) WHERE (type=='table' AND tbl_name=%Q COLLATE nocase)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        zDb,
                                        iDb,
                                        iCol,
                                        (*pTab).zName,
                                    );
                                    renameReloadSchema(
                                        pParse,
                                        iDb,
                                        INITFLAG_AlterDrop as u16_0,
                                    );
                                    renameTestSchema(
                                        pParse,
                                        zDb,
                                        (iDb == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                        b"after drop column\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1 as ::core::ffi::c_int,
                                    );
                                    if (*pParse).nErr == 0 as ::core::ffi::c_int
                                        && (*(*pTab).aCol.offset(iCol as isize)).colFlags
                                            as ::core::ffi::c_int & COLFLAG_VIRTUAL
                                            == 0 as ::core::ffi::c_int
                                    {
                                        let mut i: ::core::ffi::c_int = 0;
                                        let mut addr: ::core::ffi::c_int = 0;
                                        let mut reg: ::core::ffi::c_int = 0;
                                        let mut regRec: ::core::ffi::c_int = 0;
                                        let mut pPk: *mut Index = ::core::ptr::null_mut::<Index>();
                                        let mut nField: ::core::ffi::c_int = 0
                                            as ::core::ffi::c_int;
                                        let mut iCur: ::core::ffi::c_int = 0;
                                        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
                                        let c2rust_fresh2 = (*pParse).nTab;
                                        (*pParse).nTab = (*pParse).nTab + 1;
                                        iCur = c2rust_fresh2;
                                        sqlite3OpenTable(pParse, iCur, iDb, pTab, OP_OpenWrite);
                                        addr = sqlite3VdbeAddOp1(v, OP_Rewind, iCur);
                                        (*pParse).nMem += 1;
                                        reg = (*pParse).nMem;
                                        if (*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0
                                        {
                                            sqlite3VdbeAddOp2(v, OP_Rowid, iCur, reg);
                                            (*pParse).nMem += (*pTab).nCol as ::core::ffi::c_int;
                                        } else {
                                            pPk = sqlite3PrimaryKeyIndex(pTab);
                                            (*pParse).nMem += (*pPk).nColumn as ::core::ffi::c_int;
                                            i = 0 as ::core::ffi::c_int;
                                            while i < (*pPk).nKeyCol as ::core::ffi::c_int {
                                                sqlite3VdbeAddOp3(
                                                    v,
                                                    OP_Column,
                                                    iCur,
                                                    i,
                                                    reg + i + 1 as ::core::ffi::c_int,
                                                );
                                                i += 1;
                                            }
                                            nField = (*pPk).nKeyCol as ::core::ffi::c_int;
                                        }
                                        (*pParse).nMem += 1;
                                        regRec = (*pParse).nMem;
                                        let mut c2rust_current_block_48: u64;
                                        i = 0 as ::core::ffi::c_int;
                                        while i < (*pTab).nCol as ::core::ffi::c_int {
                                            if i != iCol
                                                && (*(*pTab).aCol.offset(i as isize)).colFlags
                                                    as ::core::ffi::c_int & COLFLAG_VIRTUAL
                                                    == 0 as ::core::ffi::c_int
                                            {
                                                let mut regOut: ::core::ffi::c_int = 0;
                                                if !pPk.is_null() {
                                                    let mut iPos: ::core::ffi::c_int = sqlite3TableColumnToIndex(
                                                        pPk,
                                                        i,
                                                    );
                                                    let mut iColPos: ::core::ffi::c_int = sqlite3TableColumnToIndex(
                                                        pPk,
                                                        iCol,
                                                    );
                                                    if iPos < (*pPk).nKeyCol as ::core::ffi::c_int {
                                                        c2rust_current_block_48 = 10891380440665537214;
                                                    } else {
                                                        regOut = reg + 1 as ::core::ffi::c_int + iPos
                                                            - (iPos > iColPos) as ::core::ffi::c_int;
                                                        c2rust_current_block_48 = 2989495919056355252;
                                                    }
                                                } else {
                                                    regOut = reg + 1 as ::core::ffi::c_int + nField;
                                                    c2rust_current_block_48 = 2989495919056355252;
                                                }
                                                match c2rust_current_block_48 {
                                                    10891380440665537214 => {}
                                                    _ => {
                                                        if i == (*pTab).iPKey as ::core::ffi::c_int {
                                                            sqlite3VdbeAddOp2(
                                                                v,
                                                                OP_Null,
                                                                0 as ::core::ffi::c_int,
                                                                regOut,
                                                            );
                                                        } else {
                                                            let mut aff: ::core::ffi::c_char = (*(*pTab)
                                                                .aCol
                                                                .offset(i as isize))
                                                                .affinity;
                                                            if aff as ::core::ffi::c_int == SQLITE_AFF_REAL {
                                                                (*(*pTab).aCol.offset(i as isize)).affinity = SQLITE_AFF_NUMERIC
                                                                    as ::core::ffi::c_char;
                                                            }
                                                            sqlite3ExprCodeGetColumnOfTable(v, pTab, iCur, i, regOut);
                                                            (*(*pTab).aCol.offset(i as isize)).affinity = aff;
                                                        }
                                                        nField += 1;
                                                    }
                                                }
                                            }
                                            i += 1;
                                        }
                                        if nField == 0 as ::core::ffi::c_int {
                                            (*pParse).nMem += 1;
                                            sqlite3VdbeAddOp2(
                                                v,
                                                OP_Null,
                                                0 as ::core::ffi::c_int,
                                                reg + 1 as ::core::ffi::c_int,
                                            );
                                            nField = 1 as ::core::ffi::c_int;
                                        }
                                        sqlite3VdbeAddOp3(
                                            v,
                                            OP_MakeRecord,
                                            reg + 1 as ::core::ffi::c_int,
                                            nField,
                                            regRec,
                                        );
                                        if !pPk.is_null() {
                                            sqlite3VdbeAddOp4Int(
                                                v,
                                                OP_IdxInsert,
                                                iCur,
                                                regRec,
                                                reg + 1 as ::core::ffi::c_int,
                                                (*pPk).nKeyCol as ::core::ffi::c_int,
                                            );
                                        } else {
                                            sqlite3VdbeAddOp3(v, OP_Insert, iCur, regRec, reg);
                                        }
                                        sqlite3VdbeChangeP5(v, OPFLAG_SAVEPOSITION as u16_0);
                                        sqlite3VdbeAddOp2(
                                            v,
                                            OP_Next,
                                            iCur,
                                            addr + 1 as ::core::ffi::c_int,
                                        );
                                        sqlite3VdbeJumpHere(v, addr);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        sqlite3DbFree(db, zCol as *mut ::core::ffi::c_void);
        sqlite3SrcListDelete(db, pSrc);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AlterFunctions() {
    unsafe {
        static mut aAlterTableFuncs: [FuncDef; 5] = unsafe {
            [
                FuncDef {
                    nArg: 9 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_INTERNAL | SQLITE_UTF8
                        | SQLITE_FUNC_CONSTANT) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        renameColumnFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sqlite_rename_column\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 7 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_INTERNAL | SQLITE_UTF8
                        | SQLITE_FUNC_CONSTANT) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        renameTableFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sqlite_rename_table\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 7 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_INTERNAL | SQLITE_UTF8
                        | SQLITE_FUNC_CONSTANT) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        renameTableTest
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sqlite_rename_test\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 3 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_INTERNAL | SQLITE_UTF8
                        | SQLITE_FUNC_CONSTANT) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        dropColumnFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sqlite_drop_column\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
                FuncDef {
                    nArg: 2 as i16_0,
                    funcFlags: (SQLITE_FUNC_BUILTIN | SQLITE_FUNC_INTERNAL | SQLITE_UTF8
                        | SQLITE_FUNC_CONSTANT) as u32_0,
                    pUserData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                    pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
                    xSFunc: Some(
                        renameQuotefixFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    xFinalize: None,
                    xValue: None,
                    xInverse: None,
                    zName: b"sqlite_rename_quotefix\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    u: C2Rust_Unnamed_2 {
                        pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

                    }

                },
            ]
        };
        sqlite3InsertBuiltinFuncs(
            &raw mut aAlterTableFuncs as *mut FuncDef,
            (::core::mem::size_of::<[FuncDef; 5]>() as usize)
                .wrapping_div(::core::mem::size_of::<FuncDef>() as usize)
                as ::core::ffi::c_int,
        );
    }
}
