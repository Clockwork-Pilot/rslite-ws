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
    pub type CCurHint;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
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
    fn sqlite3VdbeParser(_: *mut Vdbe) -> *mut Parse;
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
    fn sqlite3VdbeAddOp4Dup8(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const u8_0,
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
    fn sqlite3VdbeAddFunctionCall(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const FuncDef,
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
    fn sqlite3VdbeChangeP3(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
        P3: ::core::ffi::c_int,
    );
    fn sqlite3VdbeChangeP5(_: *mut Vdbe, P5: u16_0);
    fn sqlite3VdbeTypeofColumn(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeJumpHere(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeChangeToNoop(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3VdbeDb(_: *mut Vdbe) -> *mut sqlite3;
    fn sqlite3VdbeGetBoundValue(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> *mut sqlite3_value;
    fn sqlite3VdbeSetVarmask(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3MemCompare(
        _: *const Mem,
        _: *const Mem,
        _: *const CollSeq,
    ) -> ::core::ffi::c_int;
    fn sqlite3WalkExpr(_: *mut Walker, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3WalkExprList(_: *mut Walker, _: *mut ExprList) -> ::core::ffi::c_int;
    fn sqlite3WalkSelect(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3SelectWalkNoop(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3WalkerDepthIncrease(_: *mut Walker, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3WalkerDepthDecrease(_: *mut Walker, _: *mut Select);
    fn sqlite3WindowDelete(_: *mut sqlite3, _: *mut Window);
    fn sqlite3WindowLink(pSel: *mut Select, pWin: *mut Window);
    fn sqlite3WindowCompare(
        _: *const Parse,
        _: *const Window,
        _: *const Window,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3WindowDup(
        db: *mut sqlite3,
        pOwner: *mut Expr,
        p: *mut Window,
    ) -> *mut Window;
    fn sqlite3WindowListDup(db: *mut sqlite3, p: *mut Window) -> *mut Window;
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRaw(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
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
    fn sqlite3DbRealloc(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: u64_0,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbNNFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbMallocSize(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3Dequote(_: *mut ::core::ffi::c_char);
    fn sqlite3DequoteExpr(_: *mut Expr);
    fn sqlite3TokenInit(_: *mut Token, _: *mut ::core::ffi::c_char);
    fn sqlite3ColumnExpr(_: *mut Table, _: *mut Column) -> *mut Expr;
    fn sqlite3ColumnColl(_: *mut Column) -> *const ::core::ffi::c_char;
    fn sqlite3PrimaryKeyIndex(_: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(
        _: *mut Index,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3TableColumnToStorage(_: *mut Table, _: i16_0) -> i16_0;
    fn sqlite3ArrayAllocate(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3IdListDelete(_: *mut sqlite3, _: *mut IdList);
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
    fn sqlite3OpenTable(
        _: *mut Parse,
        iCur: ::core::ffi::c_int,
        iDb: ::core::ffi::c_int,
        _: *mut Table,
        _: ::core::ffi::c_int,
    );
    fn sqlite3GetVdbe(_: *mut Parse) -> *mut Vdbe;
    fn sqlite3CodeVerifySchema(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3MayAbort(_: *mut Parse);
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
    fn sqlite3AtoF(
        z: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_double,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3GetInt32(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VListAdd(
        _: *mut sqlite3,
        _: *mut VList,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut VList;
    fn sqlite3VListNumToName(
        _: *mut VList,
        _: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3VListNameToNum(
        _: *mut VList,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Atoi64(
        _: *const ::core::ffi::c_char,
        _: *mut i64_0,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3DecOrHexToI64(
        _: *const ::core::ffi::c_char,
        _: *mut i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3HexToBlob(
        _: *mut sqlite3,
        z: *const ::core::ffi::c_char,
        n: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3FindCollSeq(
        _: *mut sqlite3,
        enc: u8_0,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut CollSeq;
    fn sqlite3IsBinary(_: *const CollSeq) -> ::core::ffi::c_int;
    fn sqlite3CheckCollSeq(_: *mut Parse, _: *mut CollSeq) -> ::core::ffi::c_int;
    fn sqlite3ValueFree(_: *mut sqlite3_value);
    fn sqlite3ValueFromExpr(
        _: *mut sqlite3,
        _: *const Expr,
        _: u8_0,
        _: u8_0,
        _: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    fn sqlite3ColumnDefault(
        _: *mut Vdbe,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3RenameTokenMap(
        _: *mut Parse,
        _: *const ::core::ffi::c_void,
        _: *const Token,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3RenameExprUnmap(_: *mut Parse, _: *mut Expr);
    fn sqlite3GetCollSeq(
        _: *mut Parse,
        _: u8_0,
        _: *mut CollSeq,
        _: *const ::core::ffi::c_char,
    ) -> *mut CollSeq;
    fn sqlite3AffinityType(
        _: *const ::core::ffi::c_char,
        _: *mut Column,
    ) -> ::core::ffi::c_char;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
    fn sqlite3KeyInfoAlloc(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut KeyInfo;
    fn sqlite3KeyInfoUnref(_: *mut KeyInfo);
    fn sqlite3SelectDestInit(
        _: *mut SelectDest,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3RecordErrorOffsetOfExpr(_: *mut sqlite3, _: *const Expr);
    fn sqlite3VtabOverloadFunction(
        _: *mut sqlite3,
        _: *mut FuncDef,
        nArg: ::core::ffi::c_int,
        _: *mut Expr,
    ) -> *mut FuncDef;
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
pub struct IdxCover {
    pub pIdx: *mut Index,
    pub iCur: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefSrcList {
    pub db: *mut sqlite3,
    pub pRef: *mut SrcList,
    pub nExclude: i64_0,
    pub aiExclude: *mut ::core::ffi::c_int

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
pub struct EdupBuf {
    pub zAlloc: *mut u8_0

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_TRIGGER: ::core::ffi::c_int = SQLITE_CONSTRAINT
    | (7 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_COLUMN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_EXPR_DEPTH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_FUNCTION_ARG: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_VARIABLE_NUMBER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_SUBTYPE: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const SQLITE_RESULT_SUBTYPE: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const OP_Goto: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OP_Gosub: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const OP_Once: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const OP_If: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const OP_IfNot: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const OP_Not: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const OP_IfNullRow: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const OP_NotFound: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const OP_Found: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const OP_SeekRowid: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const OP_Or: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const OP_And: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const OP_IsNull: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const OP_NotNull: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const OP_Ne: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const OP_Eq: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const OP_ElseEq: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const OP_Filter: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const OP_Return: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const OP_Halt: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const OP_Int64: ::core::ffi::c_int = 73 as ::core::ffi::c_int;
pub const OP_BeginSubrtn: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const OP_Blob: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
pub const OP_Variable: ::core::ffi::c_int = 79 as ::core::ffi::c_int;
pub const OP_Move: ::core::ffi::c_int = 80 as ::core::ffi::c_int;
pub const OP_Copy: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
pub const OP_SCopy: ::core::ffi::c_int = 82 as ::core::ffi::c_int;
pub const OP_CollSeq: ::core::ffi::c_int = 86 as ::core::ffi::c_int;
pub const OP_AddImm: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
pub const OP_RealAffinity: ::core::ffi::c_int = 88 as ::core::ffi::c_int;
pub const OP_Cast: ::core::ffi::c_int = 89 as ::core::ffi::c_int;
pub const OP_IsTrue: ::core::ffi::c_int = 92 as ::core::ffi::c_int;
pub const OP_ZeroOrNull: ::core::ffi::c_int = 93 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_TypeCheck: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const OP_Affinity: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
pub const OP_MakeRecord: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const OP_BitAnd: ::core::ffi::c_int = 103 as ::core::ffi::c_int;
pub const OP_Subtract: ::core::ffi::c_int = 108 as ::core::ffi::c_int;
pub const OP_OpenRead: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const OP_OpenDup: ::core::ffi::c_int = 116 as ::core::ffi::c_int;
pub const OP_OpenEphemeral: ::core::ffi::c_int = 119 as ::core::ffi::c_int;
pub const OP_Rowid: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const OP_NullRow: ::core::ffi::c_int = 137 as ::core::ffi::c_int;
pub const OP_IdxInsert: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const OP_Real: ::core::ffi::c_int = 154 as ::core::ffi::c_int;
pub const OP_Param: ::core::ffi::c_int = 158 as ::core::ffi::c_int;
pub const OP_VColumn: ::core::ffi::c_int = 177 as ::core::ffi::c_int;
pub const OP_ClrSubtype: ::core::ffi::c_int = 181 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3TableColumnAffinity(
    mut pTab: *const Table,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_char {
    unsafe {
        if iCol < 0 as ::core::ffi::c_int || iCol >= (*pTab).nCol as ::core::ffi::c_int {
            return SQLITE_AFF_INTEGER as ::core::ffi::c_char;
        }
        return (*(*pTab).aCol.offset(iCol as isize)).affinity;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprAffinity(
    mut pExpr: *const Expr,
) -> ::core::ffi::c_char {
    unsafe {
        let mut op: ::core::ffi::c_int = 0;
        op = (*pExpr).op as ::core::ffi::c_int;
        loop {
            if op == TK_COLUMN || op == TK_AGG_COLUMN && !(*pExpr).y.pTab.is_null() {
                return sqlite3TableColumnAffinity(
                    (*pExpr).y.pTab,
                    (*pExpr).iColumn as ::core::ffi::c_int,
                );
            }
            if op == TK_SELECT {
                return sqlite3ExprAffinity(
                    (*(&raw mut (*(*(*pExpr).x.pSelect).pEList).a as *mut ExprList_item)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .pExpr,
                );
            }
            if op == TK_CAST {
                return sqlite3AffinityType(
                    (*pExpr).u.zToken,
                    ::core::ptr::null_mut::<Column>(),
                );
            }
            if op == TK_SELECT_COLUMN {
                return sqlite3ExprAffinity(
                    (*(&raw mut (*(*(*(*pExpr).pLeft).x.pSelect).pEList).a
                        as *mut ExprList_item)
                        .offset((*pExpr).iColumn as isize))
                        .pExpr,
                );
            }
            if op == TK_VECTOR
                || op == TK_FUNCTION
                    && (*pExpr).affExpr as ::core::ffi::c_int == SQLITE_AFF_DEFER
            {
                return sqlite3ExprAffinity(
                    (*(&raw mut (*(*pExpr).x.pList).a as *mut ExprList_item)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .pExpr,
                );
            }
            if (*pExpr).flags
                & (0x2000 as ::core::ffi::c_int | 0x40000 as ::core::ffi::c_int) as u32_0
                != 0 as ::core::ffi::c_uint
            {
                pExpr = (*pExpr).pLeft;
                op = (*pExpr).op as ::core::ffi::c_int;
            } else {
                if op != TK_REGISTER {
                    break;
                }
                op = (*pExpr).op2 as ::core::ffi::c_int;
                if op == 176 as ::core::ffi::c_int {
                    break;
                }
            }
        }
        return (*pExpr).affExpr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprDataType(
    mut pExpr: *const Expr,
) -> ::core::ffi::c_int {
    unsafe {
        while !pExpr.is_null() {
            match (*pExpr).op as ::core::ffi::c_int {
                TK_COLLATE | TK_IF_NULL_ROW | TK_UPLUS => {
                    pExpr = (*pExpr).pLeft;
                }
                TK_NULL => {
                    pExpr = ::core::ptr::null::<Expr>();
                }
                TK_STRING => return 0x2 as ::core::ffi::c_int,
                TK_BLOB => return 0x4 as ::core::ffi::c_int,
                TK_CONCAT => return 0x6 as ::core::ffi::c_int,
                TK_VARIABLE | TK_AGG_FUNCTION | TK_FUNCTION => {
                    return 0x7 as ::core::ffi::c_int;
                }
                TK_COLUMN
                | TK_AGG_COLUMN
                | TK_SELECT
                | TK_CAST
                | TK_SELECT_COLUMN
                | TK_VECTOR => {
                    let mut aff: ::core::ffi::c_int = sqlite3ExprAffinity(pExpr)
                        as ::core::ffi::c_int;
                    if aff >= SQLITE_AFF_NUMERIC {
                        return 0x5 as ::core::ffi::c_int;
                    }
                    if aff == SQLITE_AFF_TEXT {
                        return 0x6 as ::core::ffi::c_int;
                    }
                    return 0x7 as ::core::ffi::c_int;
                }
                TK_CASE => {
                    let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut ii: ::core::ffi::c_int = 0;
                    let mut pList: *mut ExprList = (*pExpr).x.pList;
                    ii = 1 as ::core::ffi::c_int;
                    while ii < (*pList).nExpr {
                        res
                            |= sqlite3ExprDataType(
                                (*(&raw mut (*pList).a as *mut ExprList_item)
                                    .offset(ii as isize))
                                    .pExpr,
                            );
                        ii += 2 as ::core::ffi::c_int;
                    }
                    if (*pList).nExpr % 2 as ::core::ffi::c_int != 0 {
                        res
                            |= sqlite3ExprDataType(
                                (*(&raw mut (*pList).a as *mut ExprList_item)
                                    .offset(
                                        ((*pList).nExpr - 1 as ::core::ffi::c_int) as isize,
                                    ))
                                    .pExpr,
                            );
                    }
                    return res;
                }
                _ => return 0x1 as ::core::ffi::c_int

            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprAddCollateToken(
    mut pParse: *const Parse,
    mut pExpr: *mut Expr,
    mut pCollName: *const Token,
    mut dequote: ::core::ffi::c_int,
) -> *mut Expr {
    unsafe {
        if (*pCollName).n > 0 as ::core::ffi::c_uint {
            let mut pNew: *mut Expr = sqlite3ExprAlloc(
                (*pParse).db,
                TK_COLLATE,
                pCollName,
                dequote,
            );
            if !pNew.is_null() {
                (*pNew).pLeft = pExpr;
                (*pNew).flags |= (EP_Collate | EP_Skip) as ::core::ffi::c_uint;
                pExpr = pNew;
            }
        }
        return pExpr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprAddCollateString(
    mut pParse: *const Parse,
    mut pExpr: *mut Expr,
    mut zC: *const ::core::ffi::c_char,
) -> *mut Expr {
    unsafe {
        let mut s: Token = Token {
            z: ::core::ptr::null::<::core::ffi::c_char>(),
            n: 0

        };
        sqlite3TokenInit(&raw mut s, zC as *mut ::core::ffi::c_char);
        return sqlite3ExprAddCollateToken(
            pParse,
            pExpr,
            &raw mut s,
            0 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprSkipCollate(mut pExpr: *mut Expr) -> *mut Expr {
    unsafe {
        while !pExpr.is_null()
            && (*pExpr).flags & 0x2000 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
        {
            pExpr = (*pExpr).pLeft;
        }
        return pExpr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprSkipCollateAndLikely(
    mut pExpr: *mut Expr,
) -> *mut Expr {
    unsafe {
        while !pExpr.is_null()
            && (*pExpr).flags
                & (0x2000 as ::core::ffi::c_int | 0x80000 as ::core::ffi::c_int) as u32_0
                != 0 as ::core::ffi::c_uint
        {
            if (*pExpr).flags & 0x80000 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
            {
                pExpr = (*(&raw mut (*(*pExpr).x.pList).a as *mut ExprList_item)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .pExpr;
            } else {
                if !((*pExpr).op as ::core::ffi::c_int == TK_COLLATE) {
                    break;
                }
                pExpr = (*pExpr).pLeft;
            }
        }
        return pExpr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCollSeq(
    mut pParse: *mut Parse,
    mut pExpr: *const Expr,
) -> *mut CollSeq {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        let mut p: *const Expr = pExpr;
        while !p.is_null() {
            let mut op: ::core::ffi::c_int = (*p).op as ::core::ffi::c_int;
            if op == TK_REGISTER {
                op = (*p).op2 as ::core::ffi::c_int;
            }
            if op == TK_AGG_COLUMN && !(*p).y.pTab.is_null() || op == TK_COLUMN
                || op == TK_TRIGGER
            {
                let mut j: ::core::ffi::c_int = 0;
                j = (*p).iColumn as ::core::ffi::c_int;
                if j >= 0 as ::core::ffi::c_int {
                    let mut zColl: *const ::core::ffi::c_char = sqlite3ColumnColl(
                        (*(*p).y.pTab).aCol.offset(j as isize) as *mut Column,
                    );
                    pColl = sqlite3FindCollSeq(
                        db,
                        (*db).enc,
                        zColl,
                        0 as ::core::ffi::c_int,
                    );
                }
                break;
            } else if op == TK_CAST || op == TK_UPLUS {
                p = (*p).pLeft;
            } else if op == TK_VECTOR
                || op == TK_FUNCTION
                    && (*p).affExpr as ::core::ffi::c_int == SQLITE_AFF_DEFER
            {
                p = (*(&raw mut (*(*p).x.pList).a as *mut ExprList_item)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .pExpr;
            } else if op == TK_COLLATE {
                pColl = sqlite3GetCollSeq(
                    pParse,
                    (*db).enc,
                    ::core::ptr::null_mut::<CollSeq>(),
                    (*p).u.zToken,
                );
                break;
            } else {
                if !((*p).flags as ::core::ffi::c_uint
                    & EP_Collate as ::core::ffi::c_uint != 0)
                {
                    break;
                }
                if !(*p).pLeft.is_null()
                    && (*(*p).pLeft).flags as ::core::ffi::c_uint
                        & EP_Collate as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                {
                    p = (*p).pLeft;
                } else {
                    let mut pNext: *mut Expr = (*p).pRight;
                    if (*p).flags as ::core::ffi::c_uint
                        & EP_xIsSelect as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                        && !(*p).x.pList.is_null() && (*db).mallocFailed == 0
                    {
                        let mut i: ::core::ffi::c_int = 0;
                        i = 0 as ::core::ffi::c_int;
                        while i < (*(*p).x.pList).nExpr {
                            if (*(*(&raw mut (*(*p).x.pList).a as *mut ExprList_item)
                                .offset(i as isize))
                                .pExpr)
                                .flags & 0x200 as ::core::ffi::c_int as u32_0
                                != 0 as ::core::ffi::c_uint
                            {
                                pNext = (*(&raw mut (*(*p).x.pList).a as *mut ExprList_item)
                                    .offset(i as isize))
                                    .pExpr;
                                break;
                            } else {
                                i += 1;
                            }
                        }
                    }
                    p = pNext;
                }
            }
        }
        if sqlite3CheckCollSeq(pParse, pColl) != 0 {
            pColl = ::core::ptr::null_mut::<CollSeq>();
        }
        return pColl;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprNNCollSeq(
    mut pParse: *mut Parse,
    mut pExpr: *const Expr,
) -> *mut CollSeq {
    unsafe {
        let mut p: *mut CollSeq = sqlite3ExprCollSeq(pParse, pExpr);
        if p.is_null() {
            p = (*(*pParse).db).pDfltColl;
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCollSeqMatch(
    mut pParse: *mut Parse,
    mut pE1: *const Expr,
    mut pE2: *const Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pColl1: *mut CollSeq = sqlite3ExprNNCollSeq(pParse, pE1);
        let mut pColl2: *mut CollSeq = sqlite3ExprNNCollSeq(pParse, pE2);
        return (sqlite3StrICmp((*pColl1).zName, (*pColl2).zName)
            == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CompareAffinity(
    mut pExpr: *const Expr,
    mut aff2: ::core::ffi::c_char,
) -> ::core::ffi::c_char {
    unsafe {
        let mut aff1: ::core::ffi::c_char = sqlite3ExprAffinity(pExpr);
        if aff1 as ::core::ffi::c_int > SQLITE_AFF_NONE
            && aff2 as ::core::ffi::c_int > SQLITE_AFF_NONE
        {
            if aff1 as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC
                || aff2 as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC
            {
                return SQLITE_AFF_NUMERIC as ::core::ffi::c_char
            } else {
                return SQLITE_AFF_BLOB as ::core::ffi::c_char
            }
        } else {
            return ((if aff1 as ::core::ffi::c_int <= SQLITE_AFF_NONE {
                aff2 as ::core::ffi::c_int
            } else {
                aff1 as ::core::ffi::c_int
            }) | SQLITE_AFF_NONE) as ::core::ffi::c_char
        };
    }
}
unsafe extern "C" fn comparisonAffinity(mut pExpr: *const Expr) -> ::core::ffi::c_char {
    unsafe {
        let mut aff: ::core::ffi::c_char = 0;
        aff = sqlite3ExprAffinity((*pExpr).pLeft);
        if !(*pExpr).pRight.is_null() {
            aff = sqlite3CompareAffinity((*pExpr).pRight, aff);
        } else if (*pExpr).flags as ::core::ffi::c_uint
            & EP_xIsSelect as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
        {
            aff = sqlite3CompareAffinity(
                (*(&raw mut (*(*(*pExpr).x.pSelect).pEList).a as *mut ExprList_item)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .pExpr,
                aff,
            );
        } else if aff as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            aff = SQLITE_AFF_BLOB as ::core::ffi::c_char;
        }
        return aff;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IndexAffinityOk(
    mut pExpr: *const Expr,
    mut idx_affinity: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aff: ::core::ffi::c_char = comparisonAffinity(pExpr);
        if (aff as ::core::ffi::c_int) < SQLITE_AFF_TEXT {
            return 1 as ::core::ffi::c_int;
        }
        if aff as ::core::ffi::c_int == SQLITE_AFF_TEXT {
            return (idx_affinity as ::core::ffi::c_int == SQLITE_AFF_TEXT)
                as ::core::ffi::c_int;
        }
        return (idx_affinity as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn binaryCompareP5(
    mut pExpr1: *const Expr,
    mut pExpr2: *const Expr,
    mut jumpIfNull: ::core::ffi::c_int,
) -> u8_0 {
    unsafe {
        let mut aff: u8_0 = sqlite3ExprAffinity(pExpr2) as u8_0;
        aff = (sqlite3CompareAffinity(pExpr1, aff as ::core::ffi::c_char) as u8_0
            as ::core::ffi::c_int | jumpIfNull as u8_0 as ::core::ffi::c_int) as u8_0;
        return aff;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BinaryCompareCollSeq(
    mut pParse: *mut Parse,
    mut pLeft: *const Expr,
    mut pRight: *const Expr,
) -> *mut CollSeq {
    unsafe {
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        if (*pLeft).flags as ::core::ffi::c_uint & EP_Collate as ::core::ffi::c_uint != 0
        {
            pColl = sqlite3ExprCollSeq(pParse, pLeft);
        } else if !pRight.is_null()
            && (*pRight).flags as ::core::ffi::c_uint & EP_Collate as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
        {
            pColl = sqlite3ExprCollSeq(pParse, pRight);
        } else {
            pColl = sqlite3ExprCollSeq(pParse, pLeft);
            if pColl.is_null() {
                pColl = sqlite3ExprCollSeq(pParse, pRight);
            }
        }
        return pColl;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCompareCollSeq(
    mut pParse: *mut Parse,
    mut p: *const Expr,
) -> *mut CollSeq {
    unsafe {
        if (*p).flags & 0x400 as ::core::ffi::c_int as u32_0 != 0 as ::core::ffi::c_uint
        {
            return sqlite3BinaryCompareCollSeq(pParse, (*p).pRight, (*p).pLeft)
        } else {
            return sqlite3BinaryCompareCollSeq(pParse, (*p).pLeft, (*p).pRight)
        };
    }
}
unsafe extern "C" fn codeCompare(
    mut pParse: *mut Parse,
    mut pLeft: *mut Expr,
    mut pRight: *mut Expr,
    mut opcode: ::core::ffi::c_int,
    mut in1: ::core::ffi::c_int,
    mut in2: ::core::ffi::c_int,
    mut dest: ::core::ffi::c_int,
    mut jumpIfNull: ::core::ffi::c_int,
    mut isCommuted: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p5: ::core::ffi::c_int = 0;
        let mut addr: ::core::ffi::c_int = 0;
        let mut p4: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        if (*pParse).nErr != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if isCommuted != 0 {
            p4 = sqlite3BinaryCompareCollSeq(pParse, pRight, pLeft);
        } else {
            p4 = sqlite3BinaryCompareCollSeq(pParse, pLeft, pRight);
        }
        p5 = binaryCompareP5(pLeft, pRight, jumpIfNull) as ::core::ffi::c_int;
        addr = sqlite3VdbeAddOp4(
            (*pParse).pVdbe,
            opcode,
            in2,
            dest,
            in1,
            p4 as *mut ::core::ffi::c_void as *const ::core::ffi::c_char,
            P4_COLLSEQ,
        );
        sqlite3VdbeChangeP5((*pParse).pVdbe, p5 as u16_0);
        return addr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprIsVector(
    mut pExpr: *const Expr,
) -> ::core::ffi::c_int {
    unsafe {
        return (sqlite3ExprVectorSize(pExpr) > 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprVectorSize(
    mut pExpr: *const Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut op: u8_0 = (*pExpr).op;
        if op as ::core::ffi::c_int == TK_REGISTER {
            op = (*pExpr).op2;
        }
        if op as ::core::ffi::c_int == TK_VECTOR {
            return (*(*pExpr).x.pList).nExpr
        } else if op as ::core::ffi::c_int == TK_SELECT {
            return (*(*(*pExpr).x.pSelect).pEList).nExpr
        } else {
            return 1 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VectorFieldSubexpr(
    mut pVector: *mut Expr,
    mut i: ::core::ffi::c_int,
) -> *mut Expr {
    unsafe {
        if sqlite3ExprIsVector(pVector) != 0 {
            if (*pVector).op as ::core::ffi::c_int == TK_SELECT
                || (*pVector).op2 as ::core::ffi::c_int == TK_SELECT
            {
                return (*(&raw mut (*(*(*pVector).x.pSelect).pEList).a
                    as *mut ExprList_item)
                    .offset(i as isize))
                    .pExpr
            } else {
                return (*(&raw mut (*(*pVector).x.pList).a as *mut ExprList_item)
                    .offset(i as isize))
                    .pExpr
            }
        }
        return pVector;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprForVectorField(
    mut pParse: *mut Parse,
    mut pVector: *mut Expr,
    mut iField: ::core::ffi::c_int,
    mut nField: ::core::ffi::c_int,
) -> *mut Expr {
    unsafe {
        let mut pRet: *mut Expr = ::core::ptr::null_mut::<Expr>();
        if (*pVector).op as ::core::ffi::c_int == TK_SELECT {
            pRet = sqlite3PExpr(
                pParse,
                TK_SELECT_COLUMN,
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<Expr>(),
            );
            if !pRet.is_null() {
                (*pRet).flags
                    |= 0x20000 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint;
                (*pRet).iTable = nField;
                (*pRet).iColumn = iField as ynVar;
                (*pRet).pLeft = pVector;
            }
        } else {
            if (*pVector).op as ::core::ffi::c_int == TK_VECTOR {
                let mut ppVector: *mut *mut Expr = ::core::ptr::null_mut::<*mut Expr>();
                ppVector = &raw mut (*(&raw mut (*(*pVector).x.pList).a
                    as *mut ExprList_item)
                    .offset(iField as isize))
                    .pExpr;
                pVector = *ppVector;
                if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                    *ppVector = ::core::ptr::null_mut::<Expr>();
                    return pVector;
                }
            }
            pRet = sqlite3ExprDup((*pParse).db, pVector, 0 as ::core::ffi::c_int);
        }
        return pRet;
    }
}
unsafe extern "C" fn exprCodeSubselect(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut reg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*pExpr).op as ::core::ffi::c_int == TK_SELECT {
            reg = sqlite3CodeSubselect(pParse, pExpr);
        }
        return reg;
    }
}
unsafe extern "C" fn exprVectorRegister(
    mut pParse: *mut Parse,
    mut pVector: *mut Expr,
    mut iField: ::core::ffi::c_int,
    mut regSelect: ::core::ffi::c_int,
    mut ppExpr: *mut *mut Expr,
    mut pRegFree: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut op: u8_0 = (*pVector).op;
        if op as ::core::ffi::c_int == TK_REGISTER {
            *ppExpr = sqlite3VectorFieldSubexpr(pVector, iField);
            return (*pVector).iTable + iField;
        }
        if op as ::core::ffi::c_int == TK_SELECT {
            *ppExpr = (*(&raw mut (*(*(*pVector).x.pSelect).pEList).a
                as *mut ExprList_item)
                .offset(iField as isize))
                .pExpr;
            return regSelect + iField;
        }
        if op as ::core::ffi::c_int == TK_VECTOR {
            *ppExpr = (*(&raw mut (*(*pVector).x.pList).a as *mut ExprList_item)
                .offset(iField as isize))
                .pExpr;
            return sqlite3ExprCodeTemp(pParse, *ppExpr, pRegFree);
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn codeVectorCompare(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut dest: ::core::ffi::c_int,
    mut op: u8_0,
    mut p5: u8_0,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut pLeft: *mut Expr = (*pExpr).pLeft;
        let mut pRight: *mut Expr = (*pExpr).pRight;
        let mut nLeft: ::core::ffi::c_int = sqlite3ExprVectorSize(pLeft);
        let mut i: ::core::ffi::c_int = 0;
        let mut regLeft: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regRight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut opx: u8_0 = op;
        let mut addrCmp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addrDone: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
        let mut isCommuted: ::core::ffi::c_int = ((*pExpr).flags
            & 0x400 as ::core::ffi::c_int as u32_0 != 0 as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        if (*pParse).nErr != 0 {
            return;
        }
        if nLeft != sqlite3ExprVectorSize(pRight) {
            sqlite3ErrorMsg(
                pParse,
                b"row value misused\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if op as ::core::ffi::c_int == TK_LE {
            opx = TK_LT as u8_0;
        }
        if op as ::core::ffi::c_int == TK_GE {
            opx = TK_GT as u8_0;
        }
        if op as ::core::ffi::c_int == TK_NE {
            opx = TK_EQ as u8_0;
        }
        regLeft = exprCodeSubselect(pParse, pLeft);
        regRight = exprCodeSubselect(pParse, pRight);
        sqlite3VdbeAddOp2(v, OP_Integer, 1 as ::core::ffi::c_int, dest);
        i = 0 as ::core::ffi::c_int;
        loop {
            let mut regFree1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut regFree2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pL: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pR: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut r1: ::core::ffi::c_int = 0;
            let mut r2: ::core::ffi::c_int = 0;
            if addrCmp != 0 {
                sqlite3VdbeJumpHere(v, addrCmp);
            }
            r1 = exprVectorRegister(
                pParse,
                pLeft,
                i,
                regLeft,
                &raw mut pL,
                &raw mut regFree1,
            );
            r2 = exprVectorRegister(
                pParse,
                pRight,
                i,
                regRight,
                &raw mut pR,
                &raw mut regFree2,
            );
            addrCmp = sqlite3VdbeCurrentAddr(v);
            codeCompare(
                pParse,
                pL,
                pR,
                opx as ::core::ffi::c_int,
                r1,
                r2,
                addrDone,
                p5 as ::core::ffi::c_int,
                isCommuted,
            );
            sqlite3ReleaseTempReg(pParse, regFree1);
            sqlite3ReleaseTempReg(pParse, regFree2);
            if (opx as ::core::ffi::c_int == TK_LT || opx as ::core::ffi::c_int == TK_GT)
                && i < nLeft - 1 as ::core::ffi::c_int
            {
                addrCmp = sqlite3VdbeAddOp0(v, OP_ElseEq);
            }
            if p5 as ::core::ffi::c_int == SQLITE_NULLEQ {
                sqlite3VdbeAddOp2(v, OP_Integer, 0 as ::core::ffi::c_int, dest);
            } else {
                sqlite3VdbeAddOp3(v, OP_ZeroOrNull, r1, dest, r2);
            }
            if i == nLeft - 1 as ::core::ffi::c_int {
                break;
            }
            if opx as ::core::ffi::c_int == TK_EQ {
                sqlite3VdbeAddOp2(v, OP_NotNull, dest, addrDone);
            } else {
                sqlite3VdbeAddOp2(v, OP_Goto, 0 as ::core::ffi::c_int, addrDone);
                if i == nLeft - 2 as ::core::ffi::c_int {
                    opx = op;
                }
            }
            i += 1;
        }
        sqlite3VdbeJumpHere(v, addrCmp);
        sqlite3VdbeResolveLabel(v, addrDone);
        if op as ::core::ffi::c_int == TK_NE {
            sqlite3VdbeAddOp2(v, OP_Not, dest, dest);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCheckHeight(
    mut pParse: *mut Parse,
    mut nHeight: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut mxHeight: ::core::ffi::c_int = (*(*pParse).db)
            .aLimit[SQLITE_LIMIT_EXPR_DEPTH as usize];
        if nHeight > mxHeight {
            sqlite3ErrorMsg(
                pParse,
                b"Expression tree is too large (maximum depth %d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mxHeight,
            );
            rc = SQLITE_ERROR;
        }
        return rc;
    }
}
unsafe extern "C" fn heightOfExpr(
    mut p: *const Expr,
    mut pnHeight: *mut ::core::ffi::c_int,
) {
    unsafe {
        if !p.is_null() {
            if (*p).nHeight > *pnHeight {
                *pnHeight = (*p).nHeight;
            }
        }
    }
}
unsafe extern "C" fn heightOfExprList(
    mut p: *const ExprList,
    mut pnHeight: *mut ::core::ffi::c_int,
) {
    unsafe {
        if !p.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*p).nExpr {
                heightOfExpr(
                    (*(&raw const (*p).a as *const ExprList_item).offset(i as isize))
                        .pExpr,
                    pnHeight,
                );
                i += 1;
            }
        }
    }
}
unsafe extern "C" fn heightOfSelect(
    mut pSelect: *const Select,
    mut pnHeight: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *const Select = ::core::ptr::null::<Select>();
        p = pSelect;
        while !p.is_null() {
            heightOfExpr((*p).pWhere, pnHeight);
            heightOfExpr((*p).pHaving, pnHeight);
            heightOfExpr((*p).pLimit, pnHeight);
            heightOfExprList((*p).pEList, pnHeight);
            heightOfExprList((*p).pGroupBy, pnHeight);
            heightOfExprList((*p).pOrderBy, pnHeight);
            p = (*p).pPrior;
        }
    }
}
unsafe extern "C" fn exprSetHeight(mut p: *mut Expr) {
    unsafe {
        let mut nHeight: ::core::ffi::c_int = if !(*p).pLeft.is_null() {
            (*(*p).pLeft).nHeight
        } else {
            0 as ::core::ffi::c_int
        };
        if !(*p).pRight.is_null() && (*(*p).pRight).nHeight > nHeight {
            nHeight = (*(*p).pRight).nHeight;
        }
        if (*p).flags as ::core::ffi::c_uint & EP_xIsSelect as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            heightOfSelect((*p).x.pSelect, &raw mut nHeight);
        } else if !(*p).x.pList.is_null() {
            heightOfExprList((*p).x.pList, &raw mut nHeight);
            (*p).flags
                |= (EP_Propagate as u32_0 & sqlite3ExprListFlags((*p).x.pList))
                    as ::core::ffi::c_uint;
        }
        (*p).nHeight = nHeight + 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprSetHeightAndFlags(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
) {
    unsafe {
        if (*pParse).nErr != 0 {
            return;
        }
        exprSetHeight(p);
        sqlite3ExprCheckHeight(pParse, (*p).nHeight);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SelectExprHeight(
    mut p: *const Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nHeight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        heightOfSelect(p, &raw mut nHeight);
        return nHeight;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprSetErrorOffset(
    mut pExpr: *mut Expr,
    mut iOfst: ::core::ffi::c_int,
) {
    unsafe {
        if pExpr.is_null() {
            return;
        }
        if (*pExpr).flags as ::core::ffi::c_uint
            & (0x2 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
        {
            return;
        }
        (*pExpr).w.iOfst = iOfst;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprAlloc(
    mut db: *mut sqlite3,
    mut op: ::core::ffi::c_int,
    mut pToken: *const Token,
    mut dequote: ::core::ffi::c_int,
) -> *mut Expr {
    unsafe {
        let mut pNew: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut nExtra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !pToken.is_null() {
            if op != TK_INTEGER || (*pToken).z.is_null()
                || sqlite3GetInt32((*pToken).z, &raw mut iValue)
                    == 0 as ::core::ffi::c_int
            {
                nExtra = (*pToken).n.wrapping_add(1 as ::core::ffi::c_uint)
                    as ::core::ffi::c_int;
            }
        }
        pNew = sqlite3DbMallocRawNN(
            db,
            (::core::mem::size_of::<Expr>() as usize).wrapping_add(nExtra as usize)
                as u64_0,
        ) as *mut Expr;
        if !pNew.is_null() {
            memset(
                pNew as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Expr>() as size_t,
            );
            (*pNew).op = op as u8_0;
            (*pNew).iAgg = -1 as ::core::ffi::c_int as i16_0;
            if !pToken.is_null() {
                if nExtra == 0 as ::core::ffi::c_int {
                    (*pNew).flags
                        |= (EP_IntValue | EP_Leaf
                            | (if iValue != 0 { EP_IsTrue } else { EP_IsFalse }))
                            as ::core::ffi::c_uint;
                    (*pNew).u.iValue = iValue;
                } else {
                    (*pNew).u.zToken = pNew.offset(1 as ::core::ffi::c_int as isize)
                        as *mut Expr as *mut ::core::ffi::c_char;
                    if (*pToken).n != 0 {
                        memcpy(
                            (*pNew).u.zToken as *mut ::core::ffi::c_void,
                            (*pToken).z as *const ::core::ffi::c_void,
                            (*pToken).n as size_t,
                        );
                    }
                    *(*pNew).u.zToken.offset((*pToken).n as isize) = 0
                        as ::core::ffi::c_char;
                    if dequote != 0
                        && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *(*pNew).u.zToken.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
                    {
                        sqlite3DequoteExpr(pNew);
                    }
                }
            }
            (*pNew).nHeight = 1 as ::core::ffi::c_int;
        }
        return pNew;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Expr(
    mut db: *mut sqlite3,
    mut op: ::core::ffi::c_int,
    mut zToken: *const ::core::ffi::c_char,
) -> *mut Expr {
    unsafe {
        let mut x: Token = Token {
            z: ::core::ptr::null::<::core::ffi::c_char>(),
            n: 0

        };
        x.z = zToken;
        x.n = sqlite3Strlen30(zToken) as ::core::ffi::c_uint;
        return sqlite3ExprAlloc(db, op, &raw mut x, 0 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprAttachSubtrees(
    mut db: *mut sqlite3,
    mut pRoot: *mut Expr,
    mut pLeft: *mut Expr,
    mut pRight: *mut Expr,
) {
    unsafe {
        if pRoot.is_null() {
            sqlite3ExprDelete(db, pLeft);
            sqlite3ExprDelete(db, pRight);
        } else {
            if !pRight.is_null() {
                (*pRoot).pRight = pRight;
                (*pRoot).flags
                    |= (EP_Propagate as u32_0 & (*pRight).flags) as ::core::ffi::c_uint;
                (*pRoot).nHeight = (*pRight).nHeight + 1 as ::core::ffi::c_int;
            } else {
                (*pRoot).nHeight = 1 as ::core::ffi::c_int;
            }
            if !pLeft.is_null() {
                (*pRoot).pLeft = pLeft;
                (*pRoot).flags
                    |= (EP_Propagate as u32_0 & (*pLeft).flags) as ::core::ffi::c_uint;
                if (*pLeft).nHeight >= (*pRoot).nHeight {
                    (*pRoot).nHeight = (*pLeft).nHeight + 1 as ::core::ffi::c_int;
                }
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PExpr(
    mut pParse: *mut Parse,
    mut op: ::core::ffi::c_int,
    mut pLeft: *mut Expr,
    mut pRight: *mut Expr,
) -> *mut Expr {
    unsafe {
        let mut p: *mut Expr = ::core::ptr::null_mut::<Expr>();
        p = sqlite3DbMallocRawNN((*pParse).db, ::core::mem::size_of::<Expr>() as u64_0)
            as *mut Expr;
        if !p.is_null() {
            memset(
                p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Expr>() as size_t,
            );
            (*p).op = (op & 0xff as ::core::ffi::c_int) as u8_0;
            (*p).iAgg = -1 as ::core::ffi::c_int as i16_0;
            sqlite3ExprAttachSubtrees((*pParse).db, p, pLeft, pRight);
            sqlite3ExprCheckHeight(pParse, (*p).nHeight);
        } else {
            sqlite3ExprDelete((*pParse).db, pLeft);
            sqlite3ExprDelete((*pParse).db, pRight);
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PExprAddSelect(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pSelect: *mut Select,
) {
    unsafe {
        if !pExpr.is_null() {
            (*pExpr).x.pSelect = pSelect;
            (*pExpr).flags
                |= (0x1000 as ::core::ffi::c_int | 0x400000 as ::core::ffi::c_int)
                    as u32_0 as ::core::ffi::c_uint;
            sqlite3ExprSetHeightAndFlags(pParse, pExpr);
        } else {
            sqlite3SelectDelete((*pParse).db, pSelect);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprListToValues(
    mut pParse: *mut Parse,
    mut nElem: ::core::ffi::c_int,
    mut pEList: *mut ExprList,
) -> *mut Select {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        let mut pRet: *mut Select = ::core::ptr::null_mut::<Select>();
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pEList).nExpr {
            let mut pSel: *mut Select = ::core::ptr::null_mut::<Select>();
            let mut pExpr: *mut Expr = (*(&raw mut (*pEList).a as *mut ExprList_item)
                .offset(ii as isize))
                .pExpr;
            let mut nExprElem: ::core::ffi::c_int = 0;
            if (*pExpr).op as ::core::ffi::c_int == TK_VECTOR {
                nExprElem = (*(*pExpr).x.pList).nExpr;
            } else {
                nExprElem = 1 as ::core::ffi::c_int;
            }
            if nExprElem != nElem {
                sqlite3ErrorMsg(
                    pParse,
                    b"IN(...) element has %d term%s - expected %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    nExprElem,
                    if nExprElem > 1 as ::core::ffi::c_int {
                        b"s\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    nElem,
                );
                break;
            } else {
                pSel = sqlite3SelectNew(
                    pParse,
                    (*pExpr).x.pList,
                    ::core::ptr::null_mut::<SrcList>(),
                    ::core::ptr::null_mut::<Expr>(),
                    ::core::ptr::null_mut::<ExprList>(),
                    ::core::ptr::null_mut::<Expr>(),
                    ::core::ptr::null_mut::<ExprList>(),
                    SF_Values as u32_0,
                    ::core::ptr::null_mut::<Expr>(),
                );
                (*pExpr).x.pList = ::core::ptr::null_mut::<ExprList>();
                if !pSel.is_null() {
                    if !pRet.is_null() {
                        (*pSel).op = TK_ALL as u8_0;
                        (*pSel).pPrior = pRet;
                    }
                    pRet = pSel;
                }
                ii += 1;
            }
        }
        if !pRet.is_null() && !(*pRet).pPrior.is_null() {
            (*pRet).selFlags |= SF_MultiValue as ::core::ffi::c_uint;
        }
        sqlite3ExprListDelete((*pParse).db, pEList);
        return pRet;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprAnd(
    mut pParse: *mut Parse,
    mut pLeft: *mut Expr,
    mut pRight: *mut Expr,
) -> *mut Expr {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        if pLeft.is_null() {
            return pRight
        } else if pRight.is_null() {
            return pLeft
        } else {
            let mut f: u32_0 = (*pLeft).flags | (*pRight).flags;
            if f as ::core::ffi::c_uint
                & (EP_OuterON | EP_InnerON | EP_IsFalse | EP_HasFunc)
                    as ::core::ffi::c_uint == EP_IsFalse as ::core::ffi::c_uint
                && !((*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME)
            {
                sqlite3ExprDeferredDelete(pParse, pLeft);
                sqlite3ExprDeferredDelete(pParse, pRight);
                return sqlite3Expr(
                    db,
                    TK_INTEGER,
                    b"0\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                return sqlite3PExpr(pParse, TK_AND, pLeft, pRight)
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprFunction(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut pToken: *const Token,
    mut eDistinct: ::core::ffi::c_int,
) -> *mut Expr {
    unsafe {
        let mut pNew: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut db: *mut sqlite3 = (*pParse).db;
        pNew = sqlite3ExprAlloc(db, TK_FUNCTION, pToken, 1 as ::core::ffi::c_int);
        if pNew.is_null() {
            sqlite3ExprListDelete(db, pList);
            return ::core::ptr::null_mut::<Expr>();
        }
        (*pNew).w.iOfst = (*pToken).z.offset_from((*pParse).zTail) as ::core::ffi::c_long
            as ::core::ffi::c_int;
        if !pList.is_null()
            && (*pList).nExpr
                > (*(*pParse).db).aLimit[SQLITE_LIMIT_FUNCTION_ARG as usize]
            && (*pParse).nested == 0
        {
            sqlite3ErrorMsg(
                pParse,
                b"too many arguments on function %T\0".as_ptr()
                    as *const ::core::ffi::c_char,
                pToken,
            );
        }
        (*pNew).x.pList = pList;
        (*pNew).flags |= 0x8 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint;
        sqlite3ExprSetHeightAndFlags(pParse, pNew);
        if eDistinct == SF_Distinct {
            (*pNew).flags |= 0x4 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint;
        }
        return pNew;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprOrderByAggregateError(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
) {
    unsafe {
        sqlite3ErrorMsg(
            pParse,
            b"ORDER BY may not be used with non-aggregate %#T()\0".as_ptr()
                as *const ::core::ffi::c_char,
            p,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprAddFunctionOrderBy(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pOrderBy: *mut ExprList,
) {
    unsafe {
        let mut pOB: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut db: *mut sqlite3 = (*pParse).db;
        if pOrderBy.is_null() {
            return;
        }
        if pExpr.is_null() {
            sqlite3ExprListDelete(db, pOrderBy);
            return;
        }
        if (*pExpr).x.pList.is_null()
            || (*(*pExpr).x.pList).nExpr == 0 as ::core::ffi::c_int
        {
            sqlite3ParserAddCleanup(
                pParse,
                Some(
                    sqlite3ExprListDeleteGeneric
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                pOrderBy as *mut ::core::ffi::c_void,
            );
            return;
        }
        if (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint
            && (*(*pExpr).y.pWin).eFrmType as ::core::ffi::c_int != TK_FILTER
        {
            sqlite3ExprOrderByAggregateError(pParse, pExpr);
            sqlite3ExprListDelete(db, pOrderBy);
            return;
        }
        if (*pOrderBy).nExpr > (*db).aLimit[SQLITE_LIMIT_COLUMN as usize] {
            sqlite3ErrorMsg(
                pParse,
                b"too many terms in ORDER BY clause\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            sqlite3ExprListDelete(db, pOrderBy);
            return;
        }
        pOB = sqlite3ExprAlloc(
            db,
            TK_ORDER,
            ::core::ptr::null::<Token>(),
            0 as ::core::ffi::c_int,
        );
        if pOB.is_null() {
            sqlite3ExprListDelete(db, pOrderBy);
            return;
        }
        (*pOB).x.pList = pOrderBy;
        (*pExpr).pLeft = pOB;
        (*pOB).flags |= 0x20000 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprFunctionUsable(
    mut pParse: *mut Parse,
    mut pExpr: *const Expr,
    mut pDef: *const FuncDef,
) {
    unsafe {
        if (*pExpr).flags & 0x40000000 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint
        {
            if (*pDef).funcFlags as ::core::ffi::c_uint
                & SQLITE_FUNC_DIRECT as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                || (*(*pParse).db).flags as ::core::ffi::c_ulonglong
                    & SQLITE_TrustedSchema as ::core::ffi::c_ulonglong
                    == 0 as ::core::ffi::c_ulonglong
            {
                sqlite3ErrorMsg(
                    pParse,
                    b"unsafe use of %#T()\0".as_ptr() as *const ::core::ffi::c_char,
                    pExpr,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprAssignVarNumber(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut n: u32_0,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut x: ynVar = 0;
        if pExpr.is_null() {
            return;
        }
        z = (*pExpr).u.zToken;
        if *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*pParse).nVar += 1;
            x = (*pParse).nVar;
        } else {
            let mut doAdd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '?' as i32
            {
                let mut i: i64_0 = 0;
                let mut bOk: ::core::ffi::c_int = 0;
                if n == 2 as ::core::ffi::c_uint {
                    i = (*z.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int - '0' as i32) as i64_0;
                    bOk = 1 as ::core::ffi::c_int;
                } else {
                    bOk = (0 as ::core::ffi::c_int
                        == sqlite3Atoi64(
                            z.offset(1 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_char,
                            &raw mut i,
                            (n as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                                as ::core::ffi::c_int,
                            SQLITE_UTF8 as u8_0,
                        )) as ::core::ffi::c_int;
                }
                if bOk == 0 as ::core::ffi::c_int || i < 1 as ::core::ffi::c_longlong
                    || i
                        > (*db).aLimit[SQLITE_LIMIT_VARIABLE_NUMBER as usize]
                            as ::core::ffi::c_longlong
                {
                    sqlite3ErrorMsg(
                        pParse,
                        b"variable number must be between ?1 and ?%d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*db).aLimit[SQLITE_LIMIT_VARIABLE_NUMBER as usize],
                    );
                    sqlite3RecordErrorOffsetOfExpr((*pParse).db, pExpr);
                    return;
                }
                x = i as ynVar;
                if x as ::core::ffi::c_int > (*pParse).nVar as ::core::ffi::c_int {
                    (*pParse).nVar = x as ::core::ffi::c_int as ynVar;
                    doAdd = 1 as ::core::ffi::c_int;
                } else if sqlite3VListNumToName(
                        (*pParse).pVList,
                        x as ::core::ffi::c_int,
                    )
                    .is_null()
                {
                    doAdd = 1 as ::core::ffi::c_int;
                }
            } else {
                x = sqlite3VListNameToNum((*pParse).pVList, z, n as ::core::ffi::c_int)
                    as ynVar;
                if x as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*pParse).nVar += 1;
                    x = (*pParse).nVar;
                    doAdd = 1 as ::core::ffi::c_int;
                }
            }
            if doAdd != 0 {
                (*pParse).pVList = sqlite3VListAdd(
                    db,
                    (*pParse).pVList,
                    z,
                    n as ::core::ffi::c_int,
                    x as ::core::ffi::c_int,
                );
            }
        }
        (*pExpr).iColumn = x;
        if x as ::core::ffi::c_int > (*db).aLimit[SQLITE_LIMIT_VARIABLE_NUMBER as usize]
        {
            sqlite3ErrorMsg(
                pParse,
                b"too many SQL variables\0".as_ptr() as *const ::core::ffi::c_char,
            );
            sqlite3RecordErrorOffsetOfExpr((*pParse).db, pExpr);
        }
    }
}
#[inline(never)]
unsafe extern "C" fn sqlite3ExprDeleteNN(mut db: *mut sqlite3, mut p: *mut Expr) {
    unsafe {
        while !((*p).flags
            & (0x10000 as ::core::ffi::c_int | 0x800000 as ::core::ffi::c_int) as u32_0
            != 0 as ::core::ffi::c_uint)
        {
            if !(*p).pRight.is_null() {
                sqlite3ExprDeleteNN(db, (*p).pRight);
            } else if (*p).flags as ::core::ffi::c_uint
                & EP_xIsSelect as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            {
                sqlite3SelectDelete(db, (*p).x.pSelect);
            } else {
                sqlite3ExprListDelete(db, (*p).x.pList);
                if (*p).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint
                {
                    sqlite3WindowDelete(db, (*p).y.pWin);
                }
            }
            if !(!(*p).pLeft.is_null()
                && (*p).op as ::core::ffi::c_int != TK_SELECT_COLUMN)
            {
                break;
            }
            let mut pLeft: *mut Expr = (*p).pLeft;
            if !((*p).flags & 0x8000000 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint)
                && !((*pLeft).flags & 0x8000000 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint)
            {
                sqlite3DbNNFreeNN(db, p as *mut ::core::ffi::c_void);
                p = pLeft;
            } else {
                sqlite3ExprDeleteNN(db, pLeft);
                break;
            }
        }
        if !((*p).flags & 0x8000000 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint)
        {
            sqlite3DbNNFreeNN(db, p as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprDelete(mut db: *mut sqlite3, mut p: *mut Expr) {
    unsafe {
        if !p.is_null() {
            sqlite3ExprDeleteNN(db, p);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprDeleteGeneric(
    mut db: *mut sqlite3,
    mut p: *mut ::core::ffi::c_void,
) {
    unsafe {
        if !p.is_null() {
            sqlite3ExprDeleteNN(db, p as *mut Expr);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ClearOnOrUsing(
    mut db: *mut sqlite3,
    mut p: *mut OnOrUsing,
) {
    unsafe {
        if !p.is_null() {
            if !(*p).pOn.is_null() {
                sqlite3ExprDeleteNN(db, (*p).pOn);
            } else if !(*p).pUsing.is_null() {
                sqlite3IdListDelete(db, (*p).pUsing);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprDeferredDelete(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3ParserAddCleanup(
                pParse,
                Some(
                    sqlite3ExprDeleteGeneric
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                pExpr as *mut ::core::ffi::c_void,
            )
            .is_null() as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprUnmapAndDelete(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
) {
    unsafe {
        if !p.is_null() {
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                sqlite3RenameExprUnmap(pParse, p);
            }
            sqlite3ExprDeleteNN((*pParse).db, p);
        }
    }
}
unsafe extern "C" fn exprStructSize(mut p: *const Expr) -> ::core::ffi::c_int {
    unsafe {
        if (*p).flags & 0x10000 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint
        {
            return EXPR_TOKENONLYSIZE as ::core::ffi::c_int;
        }
        if (*p).flags & 0x4000 as ::core::ffi::c_int as u32_0 != 0 as ::core::ffi::c_uint
        {
            return EXPR_REDUCEDSIZE as ::core::ffi::c_int;
        }
        return EXPR_FULLSIZE as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn dupedExprStructSize(
    mut p: *const Expr,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nSize: ::core::ffi::c_int = 0;
        if 0 as ::core::ffi::c_int == flags
            || (*p).flags & 0x20000 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
        {
            nSize = EXPR_FULLSIZE as ::core::ffi::c_int;
        } else if !(*p).pLeft.is_null() || !(*p).x.pList.is_null() {
            nSize = (EXPR_REDUCEDSIZE | EP_Reduced as ::core::ffi::c_ulong)
                as ::core::ffi::c_int;
        } else {
            nSize = (EXPR_TOKENONLYSIZE | EP_TokenOnly as ::core::ffi::c_ulong)
                as ::core::ffi::c_int;
        }
        return nSize;
    }
}
unsafe extern "C" fn dupedExprNodeSize(
    mut p: *const Expr,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nByte: ::core::ffi::c_int = dupedExprStructSize(p, flags)
            & 0xfff as ::core::ffi::c_int;
        if !((*p).flags & 0x800 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint) && !(*p).u.zToken.is_null()
        {
            nByte = (nByte as ::core::ffi::c_ulong)
                .wrapping_add(
                    (strlen((*p).u.zToken) & 0x3fffffff as ::core::ffi::c_int as size_t)
                        .wrapping_add(1 as size_t) as ::core::ffi::c_ulong,
                ) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        return nByte + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn dupedExprSize(mut p: *const Expr) -> ::core::ffi::c_int {
    unsafe {
        let mut nByte: ::core::ffi::c_int = 0;
        nByte = dupedExprNodeSize(p, EXPRDUP_REDUCE);
        if !(*p).pLeft.is_null() {
            nByte += dupedExprSize((*p).pLeft);
        }
        if !(*p).pRight.is_null() {
            nByte += dupedExprSize((*p).pRight);
        }
        return nByte;
    }
}
unsafe extern "C" fn exprDup(
    mut db: *mut sqlite3,
    mut p: *const Expr,
    mut dupFlags: ::core::ffi::c_int,
    mut pEdupBuf: *mut EdupBuf,
) -> *mut Expr {
    unsafe {
        let mut pNew: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut sEdupBuf: EdupBuf = EdupBuf {
            zAlloc: ::core::ptr::null_mut::<u8_0>()

        };
        let mut staticFlag: u32_0 = 0;
        let mut nToken: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        if !pEdupBuf.is_null() {
            sEdupBuf.zAlloc = (*pEdupBuf).zAlloc;
            staticFlag = EP_Static as u32_0;
        } else {
            let mut nAlloc: ::core::ffi::c_int = 0;
            if dupFlags != 0 {
                nAlloc = dupedExprSize(p);
            } else if !((*p).flags & 0x800 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint) && !(*p).u.zToken.is_null()
            {
                nToken = (strlen((*p).u.zToken)
                    & 0x3fffffff as ::core::ffi::c_int as size_t)
                    .wrapping_add(1 as size_t) as ::core::ffi::c_int;
                nAlloc = ((::core::mem::size_of::<Expr>() as usize)
                    .wrapping_add(nToken as usize)
                    .wrapping_add(7 as usize) & !(7 as ::core::ffi::c_int) as usize)
                    as ::core::ffi::c_int;
            } else {
                nToken = 0 as ::core::ffi::c_int;
                nAlloc = ((::core::mem::size_of::<Expr>() as usize)
                    .wrapping_add(7 as usize) & !(7 as ::core::ffi::c_int) as usize)
                    as ::core::ffi::c_int;
            }
            sEdupBuf.zAlloc = sqlite3DbMallocRawNN(db, nAlloc as u64_0) as *mut u8_0;
            staticFlag = 0 as u32_0;
        }
        pNew = sEdupBuf.zAlloc as *mut Expr;
        if !pNew.is_null() {
            let nStructSize: ::core::ffi::c_uint = dupedExprStructSize(p, dupFlags)
                as ::core::ffi::c_uint;
            let mut nNewSize: ::core::ffi::c_int = (nStructSize
                & 0xfff as ::core::ffi::c_uint) as ::core::ffi::c_int;
            if nToken < 0 as ::core::ffi::c_int {
                if !((*p).flags & 0x800 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint) && !(*p).u.zToken.is_null()
                {
                    nToken = sqlite3Strlen30((*p).u.zToken) + 1 as ::core::ffi::c_int;
                } else {
                    nToken = 0 as ::core::ffi::c_int;
                }
            }
            if dupFlags != 0 {
                memcpy(
                    sEdupBuf.zAlloc as *mut ::core::ffi::c_void,
                    p as *const ::core::ffi::c_void,
                    nNewSize as size_t,
                );
            } else {
                let mut nSize: u32_0 = exprStructSize(p) as u32_0;
                memcpy(
                    sEdupBuf.zAlloc as *mut ::core::ffi::c_void,
                    p as *const ::core::ffi::c_void,
                    nSize as size_t,
                );
                if (nSize as usize) < EXPR_FULLSIZE {
                    memset(
                        sEdupBuf.zAlloc.offset(nSize as isize) as *mut u8_0
                            as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        EXPR_FULLSIZE.wrapping_sub(nSize as size_t),
                    );
                }
                nNewSize = EXPR_FULLSIZE as ::core::ffi::c_int;
            }
            (*pNew).flags
                &= !(EP_Reduced | EP_TokenOnly | EP_Static) as ::core::ffi::c_uint;
            (*pNew).flags
                |= nStructSize & (EP_Reduced | EP_TokenOnly) as ::core::ffi::c_uint;
            (*pNew).flags |= staticFlag as ::core::ffi::c_uint;
            dupFlags != 0;
            if nToken > 0 as ::core::ffi::c_int {
                (*pNew).u.zToken = sEdupBuf.zAlloc.offset(nNewSize as isize) as *mut u8_0
                    as *mut ::core::ffi::c_char;
                let mut zToken: *mut ::core::ffi::c_char = (*pNew).u.zToken;
                memcpy(
                    zToken as *mut ::core::ffi::c_void,
                    (*p).u.zToken as *const ::core::ffi::c_void,
                    nToken as size_t,
                );
                nNewSize += nToken;
            }
            sEdupBuf.zAlloc = sEdupBuf
                .zAlloc
                .offset(
                    (nNewSize + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int))
                        as isize,
                );
            if ((*p).flags as ::core::ffi::c_uint | (*pNew).flags as ::core::ffi::c_uint)
                & (EP_TokenOnly | EP_Leaf) as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
            {
                if (*p).flags as ::core::ffi::c_uint
                    & EP_xIsSelect as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                {
                    (*pNew).x.pSelect = sqlite3SelectDup(db, (*p).x.pSelect, dupFlags);
                } else {
                    (*pNew).x.pList = sqlite3ExprListDup(
                        db,
                        (*p).x.pList,
                        if (*p).op as ::core::ffi::c_int != TK_ORDER {
                            dupFlags
                        } else {
                            0 as ::core::ffi::c_int
                        },
                    );
                }
                if (*p).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint
                {
                    (*pNew).y.pWin = sqlite3WindowDup(db, pNew, (*p).y.pWin);
                }
                if dupFlags != 0 {
                    if (*p).op as ::core::ffi::c_int == TK_SELECT_COLUMN {
                        (*pNew).pLeft = (*p).pLeft;
                    } else {
                        (*pNew).pLeft = if !(*p).pLeft.is_null() {
                            exprDup(db, (*p).pLeft, EXPRDUP_REDUCE, &raw mut sEdupBuf)
                        } else {
                            ::core::ptr::null_mut::<Expr>()
                        };
                    }
                    (*pNew).pRight = if !(*p).pRight.is_null() {
                        exprDup(db, (*p).pRight, EXPRDUP_REDUCE, &raw mut sEdupBuf)
                    } else {
                        ::core::ptr::null_mut::<Expr>()
                    };
                } else {
                    if (*p).op as ::core::ffi::c_int == TK_SELECT_COLUMN {
                        (*pNew).pLeft = (*p).pLeft;
                    } else {
                        (*pNew).pLeft = sqlite3ExprDup(
                            db,
                            (*p).pLeft,
                            0 as ::core::ffi::c_int,
                        );
                    }
                    (*pNew).pRight = sqlite3ExprDup(
                        db,
                        (*p).pRight,
                        0 as ::core::ffi::c_int,
                    );
                }
            }
        }
        if !pEdupBuf.is_null() {
            memcpy(
                pEdupBuf as *mut ::core::ffi::c_void,
                &raw mut sEdupBuf as *const ::core::ffi::c_void,
                ::core::mem::size_of::<EdupBuf>() as size_t,
            );
        }
        return pNew;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WithDup(
    mut db: *mut sqlite3,
    mut p: *mut With,
) -> *mut With {
    unsafe {
        let mut pRet: *mut With = ::core::ptr::null_mut::<With>();
        if !p.is_null() {
            let mut nByte: sqlite3_int64 = (16 as usize)
                .wrapping_add(
                    ((*p).nCte as usize)
                        .wrapping_mul(::core::mem::size_of::<Cte>() as usize),
                ) as sqlite3_int64;
            pRet = sqlite3DbMallocZero(db, nByte as u64_0) as *mut With;
            if !pRet.is_null() {
                let mut i: ::core::ffi::c_int = 0;
                (*pRet).nCte = (*p).nCte;
                i = 0 as ::core::ffi::c_int;
                while i < (*p).nCte {
                    let ref mut c2rust_fresh5 = (*(&raw mut (*pRet).a as *mut Cte)
                        .offset(i as isize))
                        .pSelect;
                    *c2rust_fresh5 = sqlite3SelectDup(
                        db,
                        (*(&raw mut (*p).a as *mut Cte).offset(i as isize)).pSelect,
                        0 as ::core::ffi::c_int,
                    );
                    let ref mut c2rust_fresh6 = (*(&raw mut (*pRet).a as *mut Cte)
                        .offset(i as isize))
                        .pCols;
                    *c2rust_fresh6 = sqlite3ExprListDup(
                        db,
                        (*(&raw mut (*p).a as *mut Cte).offset(i as isize)).pCols,
                        0 as ::core::ffi::c_int,
                    );
                    let ref mut c2rust_fresh7 = (*(&raw mut (*pRet).a as *mut Cte)
                        .offset(i as isize))
                        .zName;
                    *c2rust_fresh7 = sqlite3DbStrDup(
                        db,
                        (*(&raw mut (*p).a as *mut Cte).offset(i as isize)).zName,
                    );
                    (*(&raw mut (*pRet).a as *mut Cte).offset(i as isize)).eM10d = (*(&raw mut (*p)
                        .a as *mut Cte)
                        .offset(i as isize))
                        .eM10d;
                    i += 1;
                }
            }
        }
        return pRet;
    }
}
unsafe extern "C" fn gatherSelectWindowsCallback(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int == TK_FUNCTION
            && (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
        {
            let mut pSelect: *mut Select = (*pWalker).u.pSelect;
            let mut pWin: *mut Window = (*pExpr).y.pWin;
            sqlite3WindowLink(pSelect, pWin);
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn gatherSelectWindowsSelectCallback(
    mut pWalker: *mut Walker,
    mut p: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        return if p == (*pWalker).u.pSelect { WRC_Continue } else { WRC_Prune };
    }
}
unsafe extern "C" fn gatherSelectWindows(mut p: *mut Select) {
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
            gatherSelectWindowsCallback
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = Some(
            gatherSelectWindowsSelectCallback
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        w.xSelectCallback2 = None;
        w.pParse = ::core::ptr::null_mut::<Parse>();
        w.u.pSelect = p;
        sqlite3WalkSelect(&raw mut w, p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprDup(
    mut db: *mut sqlite3,
    mut p: *const Expr,
    mut flags: ::core::ffi::c_int,
) -> *mut Expr {
    unsafe {
        return if !p.is_null() {
            exprDup(db, p, flags, ::core::ptr::null_mut::<EdupBuf>())
        } else {
            ::core::ptr::null_mut::<Expr>()
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprListDup(
    mut db: *mut sqlite3,
    mut p: *const ExprList,
    mut flags: ::core::ffi::c_int,
) -> *mut ExprList {
    unsafe {
        let mut pNew: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        let mut pOldItem: *const ExprList_item = ::core::ptr::null::<ExprList_item>();
        let mut i: ::core::ffi::c_int = 0;
        let mut pPriorSelectColOld: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pPriorSelectColNew: *mut Expr = ::core::ptr::null_mut::<Expr>();
        if p.is_null() {
            return ::core::ptr::null_mut::<ExprList>();
        }
        pNew = sqlite3DbMallocRawNN(
            db,
            sqlite3DbMallocSize(db, p as *const ::core::ffi::c_void) as u64_0,
        ) as *mut ExprList;
        if pNew.is_null() {
            return ::core::ptr::null_mut::<ExprList>();
        }
        (*pNew).nExpr = (*p).nExpr;
        (*pNew).nAlloc = (*p).nAlloc;
        pItem = &raw mut (*pNew).a as *mut ExprList_item as *mut ExprList_item;
        pOldItem = &raw const (*p).a as *const ExprList_item as *const ExprList_item;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nExpr {
            let mut pOldExpr: *mut Expr = (*pOldItem).pExpr;
            let mut pNewExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
            (*pItem).pExpr = sqlite3ExprDup(db, pOldExpr, flags);
            if !pOldExpr.is_null()
                && (*pOldExpr).op as ::core::ffi::c_int == TK_SELECT_COLUMN
                && {
                    pNewExpr = (*pItem).pExpr;
                    !pNewExpr.is_null()
                }
            {
                if !(*pNewExpr).pRight.is_null() {
                    pPriorSelectColOld = (*pOldExpr).pRight;
                    pPriorSelectColNew = (*pNewExpr).pRight;
                    (*pNewExpr).pLeft = (*pNewExpr).pRight;
                } else {
                    if (*pOldExpr).pLeft != pPriorSelectColOld {
                        pPriorSelectColOld = (*pOldExpr).pLeft;
                        pPriorSelectColNew = sqlite3ExprDup(
                            db,
                            pPriorSelectColOld,
                            flags,
                        );
                        (*pNewExpr).pRight = pPriorSelectColNew;
                    }
                    (*pNewExpr).pLeft = pPriorSelectColNew;
                }
            }
            (*pItem).zEName = sqlite3DbStrDup(db, (*pOldItem).zEName);
            (*pItem).fg = (*pOldItem).fg;
            (*pItem).u = (*pOldItem).u;
            i += 1;
            pItem = pItem.offset(1);
            pOldItem = pOldItem.offset(1);
        }
        return pNew;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcListDup(
    mut db: *mut sqlite3,
    mut p: *const SrcList,
    mut flags: ::core::ffi::c_int,
) -> *mut SrcList {
    unsafe {
        let mut pNew: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut i: ::core::ffi::c_int = 0;
        if p.is_null() {
            return ::core::ptr::null_mut::<SrcList>();
        }
        pNew = sqlite3DbMallocRawNN(
            db,
            (8 as usize)
                .wrapping_add(
                    ((*p).nSrc as usize)
                        .wrapping_mul(::core::mem::size_of::<SrcItem>() as usize),
                ) as u64_0,
        ) as *mut SrcList;
        if pNew.is_null() {
            return ::core::ptr::null_mut::<SrcList>();
        }
        (*pNew).nAlloc = (*p).nSrc as u32_0;
        (*pNew).nSrc = (*pNew).nAlloc as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nSrc {
            let mut pNewItem: *mut SrcItem = (&raw mut (*pNew).a as *mut SrcItem)
                .offset(i as isize) as *mut SrcItem;
            let mut pOldItem: *const SrcItem = (&raw const (*p).a as *const SrcItem)
                .offset(i as isize) as *const SrcItem;
            let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
            (*pNewItem).fg = (*pOldItem).fg;
            if (*pOldItem).fg.isSubquery() != 0 {
                let mut pNewSubq: *mut Subquery = sqlite3DbMallocRaw(
                    db,
                    ::core::mem::size_of::<Subquery>() as u64_0,
                ) as *mut Subquery;
                if pNewSubq.is_null() {
                    (*pNewItem)
                        .fg
                        .set_isSubquery(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                } else {
                    memcpy(
                        pNewSubq as *mut ::core::ffi::c_void,
                        (*pOldItem).u4.pSubq as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<Subquery>() as size_t,
                    );
                    (*pNewSubq).pSelect = sqlite3SelectDup(
                        db,
                        (*pNewSubq).pSelect,
                        flags,
                    );
                    if (*pNewSubq).pSelect.is_null() {
                        sqlite3DbFree(db, pNewSubq as *mut ::core::ffi::c_void);
                        pNewSubq = ::core::ptr::null_mut::<Subquery>();
                        (*pNewItem)
                            .fg
                            .set_isSubquery(
                                0 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                    }
                }
                (*pNewItem).u4.pSubq = pNewSubq;
            } else if (*pOldItem).fg.fixedSchema() != 0 {
                (*pNewItem).u4.pSchema = (*pOldItem).u4.pSchema;
            } else {
                (*pNewItem).u4.zDatabase = sqlite3DbStrDup(db, (*pOldItem).u4.zDatabase);
            }
            (*pNewItem).zName = sqlite3DbStrDup(db, (*pOldItem).zName);
            (*pNewItem).zAlias = sqlite3DbStrDup(db, (*pOldItem).zAlias);
            (*pNewItem).iCursor = (*pOldItem).iCursor;
            if (*pNewItem).fg.isIndexedBy() != 0 {
                (*pNewItem).u1.zIndexedBy = sqlite3DbStrDup(
                    db,
                    (*pOldItem).u1.zIndexedBy,
                );
            } else if (*pNewItem).fg.isTabFunc() != 0 {
                (*pNewItem).u1.pFuncArg = sqlite3ExprListDup(
                    db,
                    (*pOldItem).u1.pFuncArg,
                    flags,
                );
            } else {
                (*pNewItem).u1.nRow = (*pOldItem).u1.nRow;
            }
            (*pNewItem).u2 = (*pOldItem).u2;
            if (*pNewItem).fg.isCte() != 0 {
                (*(*pNewItem).u2.pCteUse).nUse += 1;
            }
            (*pNewItem).pSTab = (*pOldItem).pSTab;
            pTab = (*pNewItem).pSTab;
            if !pTab.is_null() {
                (*pTab).nTabRef = (*pTab).nTabRef.wrapping_add(1);
            }
            if (*pOldItem).fg.isUsing() != 0 {
                (*pNewItem).u3.pUsing = sqlite3IdListDup(db, (*pOldItem).u3.pUsing);
            } else {
                (*pNewItem).u3.pOn = sqlite3ExprDup(db, (*pOldItem).u3.pOn, flags);
            }
            (*pNewItem).colUsed = (*pOldItem).colUsed;
            i += 1;
        }
        return pNew;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IdListDup(
    mut db: *mut sqlite3,
    mut p: *const IdList,
) -> *mut IdList {
    unsafe {
        let mut pNew: *mut IdList = ::core::ptr::null_mut::<IdList>();
        let mut i: ::core::ffi::c_int = 0;
        if p.is_null() {
            return ::core::ptr::null_mut::<IdList>();
        }
        pNew = sqlite3DbMallocRawNN(
            db,
            (8 as usize)
                .wrapping_add(
                    ((*p).nId as usize)
                        .wrapping_mul(::core::mem::size_of::<IdList_item>() as usize),
                ) as u64_0,
        ) as *mut IdList;
        if pNew.is_null() {
            return ::core::ptr::null_mut::<IdList>();
        }
        (*pNew).nId = (*p).nId;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nId {
            let mut pNewItem: *mut IdList_item = (&raw mut (*pNew).a as *mut IdList_item)
                .offset(i as isize) as *mut IdList_item;
            let mut pOldItem: *const IdList_item = (&raw const (*p).a
                as *const IdList_item)
                .offset(i as isize) as *const IdList_item;
            (*pNewItem).zName = sqlite3DbStrDup(db, (*pOldItem).zName);
            i += 1;
        }
        return pNew;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SelectDup(
    mut db: *mut sqlite3,
    mut pDup: *const Select,
    mut flags: ::core::ffi::c_int,
) -> *mut Select {
    unsafe {
        let mut pRet: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pNext: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pp: *mut *mut Select = &raw mut pRet;
        let mut p: *const Select = ::core::ptr::null::<Select>();
        p = pDup;
        while !p.is_null() {
            let mut pNew: *mut Select = sqlite3DbMallocRawNN(
                db,
                ::core::mem::size_of::<Select>() as u64_0,
            ) as *mut Select;
            if pNew.is_null() {
                break;
            }
            (*pNew).pEList = sqlite3ExprListDup(db, (*p).pEList, flags);
            (*pNew).pSrc = sqlite3SrcListDup(db, (*p).pSrc, flags);
            (*pNew).pWhere = sqlite3ExprDup(db, (*p).pWhere, flags);
            (*pNew).pGroupBy = sqlite3ExprListDup(db, (*p).pGroupBy, flags);
            (*pNew).pHaving = sqlite3ExprDup(db, (*p).pHaving, flags);
            (*pNew).pOrderBy = sqlite3ExprListDup(db, (*p).pOrderBy, flags);
            (*pNew).op = (*p).op;
            (*pNew).pNext = pNext;
            (*pNew).pPrior = ::core::ptr::null_mut::<Select>();
            (*pNew).pLimit = sqlite3ExprDup(db, (*p).pLimit, flags);
            (*pNew).iLimit = 0 as ::core::ffi::c_int;
            (*pNew).iOffset = 0 as ::core::ffi::c_int;
            (*pNew).selFlags = (*p).selFlags & !(SF_UsesEphemeral as u32_0);
            (*pNew).addrOpenEphm[0 as ::core::ffi::c_int as usize] = -1
                as ::core::ffi::c_int;
            (*pNew).addrOpenEphm[1 as ::core::ffi::c_int as usize] = -1
                as ::core::ffi::c_int;
            (*pNew).nSelectRow = (*p).nSelectRow;
            (*pNew).pWith = sqlite3WithDup(db, (*p).pWith);
            (*pNew).pWin = ::core::ptr::null_mut::<Window>();
            (*pNew).pWinDefn = sqlite3WindowListDup(db, (*p).pWinDefn);
            if !(*p).pWin.is_null()
                && (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                gatherSelectWindows(pNew);
            }
            (*pNew).selId = (*p).selId;
            if (*db).mallocFailed != 0 {
                (*pNew).pNext = ::core::ptr::null_mut::<Select>();
                sqlite3SelectDelete(db, pNew);
                break;
            } else {
                *pp = pNew;
                pp = &raw mut (*pNew).pPrior;
                pNext = pNew;
                p = (*p).pPrior;
            }
        }
        return pRet;
    }
}
static mut zeroItem: ExprList_item = ExprList_item {
    pExpr: ::core::ptr::null::<Expr>() as *mut Expr,
    zEName: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    fg: C2Rust_Unnamed_5 {
        sortFlags: 0,
        eEName_done_reusable_bSorterRef_bNulls_bUsed_bUsingTerm_bNoExpand: [0; 2],
        c2rust_padding: [0; 1]

    },
    u: C2Rust_Unnamed_3 {
        x: C2Rust_Unnamed_4 {
            iOrderByCol: 0,
            iAlias: 0

        }

    }

};
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sqlite3ExprListAppendNew(
    mut db: *mut sqlite3,
    mut pExpr: *mut Expr,
) -> *mut ExprList {
    unsafe {
        let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        pList = sqlite3DbMallocRawNN(
            db,
            (8 as usize)
                .wrapping_add(
                    (4 as usize)
                        .wrapping_mul(::core::mem::size_of::<ExprList_item>() as usize),
                ) as u64_0,
        ) as *mut ExprList;
        if pList.is_null() {
            sqlite3ExprDelete(db, pExpr);
            return ::core::ptr::null_mut::<ExprList>();
        }
        (*pList).nAlloc = 4 as ::core::ffi::c_int;
        (*pList).nExpr = 1 as ::core::ffi::c_int;
        pItem = (&raw mut (*pList).a as *mut ExprList_item)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ExprList_item
            as *mut ExprList_item;
        *pItem = zeroItem;
        (*pItem).pExpr = pExpr;
        return pList;
    }
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sqlite3ExprListAppendGrow(
    mut db: *mut sqlite3,
    mut pList: *mut ExprList,
    mut pExpr: *mut Expr,
) -> *mut ExprList {
    unsafe {
        let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        let mut pNew: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        (*pList).nAlloc *= 2 as ::core::ffi::c_int;
        pNew = sqlite3DbRealloc(
            db,
            pList as *mut ::core::ffi::c_void,
            (8 as usize)
                .wrapping_add(
                    ((*pList).nAlloc as usize)
                        .wrapping_mul(::core::mem::size_of::<ExprList_item>() as usize),
                ) as u64_0,
        ) as *mut ExprList;
        if pNew.is_null() {
            sqlite3ExprListDelete(db, pList);
            sqlite3ExprDelete(db, pExpr);
            return ::core::ptr::null_mut::<ExprList>();
        } else {
            pList = pNew;
        }
        let c2rust_fresh2 = (*pList).nExpr;
        (*pList).nExpr = (*pList).nExpr + 1;
        pItem = (&raw mut (*pList).a as *mut ExprList_item)
            .offset(c2rust_fresh2 as isize) as *mut ExprList_item as *mut ExprList_item;
        *pItem = zeroItem;
        (*pItem).pExpr = pExpr;
        return pList;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprListAppend(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut pExpr: *mut Expr,
) -> *mut ExprList {
    unsafe {
        let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        if pList.is_null() {
            return sqlite3ExprListAppendNew((*pParse).db, pExpr);
        }
        if (*pList).nAlloc < (*pList).nExpr + 1 as ::core::ffi::c_int {
            return sqlite3ExprListAppendGrow((*pParse).db, pList, pExpr);
        }
        let c2rust_fresh1 = (*pList).nExpr;
        (*pList).nExpr = (*pList).nExpr + 1;
        pItem = (&raw mut (*pList).a as *mut ExprList_item)
            .offset(c2rust_fresh1 as isize) as *mut ExprList_item as *mut ExprList_item;
        *pItem = zeroItem;
        (*pItem).pExpr = pExpr;
        return pList;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprListAppendVector(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut pColumns: *mut IdList,
    mut pExpr: *mut Expr,
) -> *mut ExprList {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut n: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut iFirst: ::core::ffi::c_int = if !pList.is_null() {
            (*pList).nExpr
        } else {
            0 as ::core::ffi::c_int
        };
        if !pColumns.is_null() {
            if !pExpr.is_null() {
                if (*pExpr).op as ::core::ffi::c_int != TK_SELECT
                    && {
                        n = sqlite3ExprVectorSize(pExpr);
                        (*pColumns).nId != n
                    }
                {
                    sqlite3ErrorMsg(
                        pParse,
                        b"%d columns assigned %d values\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*pColumns).nId,
                        n,
                    );
                } else {
                    i = 0 as ::core::ffi::c_int;
                    while i < (*pColumns).nId {
                        let mut pSubExpr: *mut Expr = sqlite3ExprForVectorField(
                            pParse,
                            pExpr,
                            i,
                            (*pColumns).nId,
                        );
                        if !pSubExpr.is_null() {
                            pList = sqlite3ExprListAppend(pParse, pList, pSubExpr);
                            if !pList.is_null() {
                                let ref mut c2rust_fresh3 = (*(&raw mut (*pList).a
                                    as *mut ExprList_item)
                                    .offset(
                                        ((*pList).nExpr - 1 as ::core::ffi::c_int) as isize,
                                    ))
                                    .zEName;
                                *c2rust_fresh3 = (*(&raw mut (*pColumns).a
                                    as *mut IdList_item)
                                    .offset(i as isize))
                                    .zName;
                                let ref mut c2rust_fresh4 = (*(&raw mut (*pColumns).a
                                    as *mut IdList_item)
                                    .offset(i as isize))
                                    .zName;
                                *c2rust_fresh4 = ::core::ptr::null_mut::<
                                    ::core::ffi::c_char,
                                >();
                            }
                        }
                        i += 1;
                    }
                    if (*db).mallocFailed == 0
                        && (*pExpr).op as ::core::ffi::c_int == TK_SELECT
                        && !pList.is_null()
                    {
                        let mut pFirst: *mut Expr = (*(&raw mut (*pList).a
                            as *mut ExprList_item)
                            .offset(iFirst as isize))
                            .pExpr;
                        (*pFirst).pRight = pExpr;
                        pExpr = ::core::ptr::null_mut::<Expr>();
                        (*pFirst).iTable = (*pColumns).nId;
                    }
                }
            }
        }
        sqlite3ExprUnmapAndDelete(pParse, pExpr);
        sqlite3IdListDelete(db, pColumns);
        return pList;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprListSetSortOrder(
    mut p: *mut ExprList,
    mut iSortOrder: ::core::ffi::c_int,
    mut eNulls: ::core::ffi::c_int,
) {
    unsafe {
        let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        if p.is_null() {
            return;
        }
        pItem = (&raw mut (*p).a as *mut ExprList_item)
            .offset(((*p).nExpr - 1 as ::core::ffi::c_int) as isize)
            as *mut ExprList_item as *mut ExprList_item;
        if iSortOrder == SQLITE_SO_UNDEFINED {
            iSortOrder = SQLITE_SO_ASC;
        }
        (*pItem).fg.sortFlags = iSortOrder as u8_0;
        if eNulls != SQLITE_SO_UNDEFINED {
            (*pItem).fg.set_bNulls(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            if iSortOrder != eNulls {
                (*pItem).fg.sortFlags = ((*pItem).fg.sortFlags as ::core::ffi::c_int
                    | KEYINFO_ORDER_BIGNULL) as u8_0;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprListSetName(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut pName: *const Token,
    mut dequote: ::core::ffi::c_int,
) {
    unsafe {
        if !pList.is_null() {
            let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
            pItem = (&raw mut (*pList).a as *mut ExprList_item)
                .offset(((*pList).nExpr - 1 as ::core::ffi::c_int) as isize)
                as *mut ExprList_item as *mut ExprList_item;
            (*pItem).zEName = sqlite3DbStrNDup(
                (*pParse).db,
                (*pName).z,
                (*pName).n as u64_0,
            );
            if dequote != 0 {
                sqlite3Dequote((*pItem).zEName);
                if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                    sqlite3RenameTokenMap(
                        pParse,
                        (*pItem).zEName as *const ::core::ffi::c_void,
                        pName,
                    );
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprListSetSpan(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut zStart: *const ::core::ffi::c_char,
    mut zEnd: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        if !pList.is_null() {
            let mut pItem: *mut ExprList_item = (&raw mut (*pList).a
                as *mut ExprList_item)
                .offset(((*pList).nExpr - 1 as ::core::ffi::c_int) as isize)
                as *mut ExprList_item;
            if (*pItem).zEName.is_null() {
                (*pItem).zEName = sqlite3DbSpanDup(db, zStart, zEnd);
                (*pItem)
                    .fg
                    .set_eEName(
                        ENAME_SPAN as ::core::ffi::c_uint as ::core::ffi::c_uint,
                    );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprListCheckLength(
    mut pParse: *mut Parse,
    mut pEList: *mut ExprList,
    mut zObject: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut mx: ::core::ffi::c_int = (*(*pParse).db)
            .aLimit[SQLITE_LIMIT_COLUMN as usize];
        if !pEList.is_null() && (*pEList).nExpr > mx {
            sqlite3ErrorMsg(
                pParse,
                b"too many columns in %s\0".as_ptr() as *const ::core::ffi::c_char,
                zObject,
            );
        }
    }
}
#[inline(never)]
unsafe extern "C" fn exprListDeleteNN(mut db: *mut sqlite3, mut pList: *mut ExprList) {
    unsafe {
        let mut i: ::core::ffi::c_int = (*pList).nExpr;
        let mut pItem: *mut ExprList_item = &raw mut (*pList).a as *mut ExprList_item;
        loop {
            sqlite3ExprDelete(db, (*pItem).pExpr);
            if !(*pItem).zEName.is_null() {
                sqlite3DbNNFreeNN(db, (*pItem).zEName as *mut ::core::ffi::c_void);
            }
            pItem = pItem.offset(1);
            i -= 1;
            if !(i > 0 as ::core::ffi::c_int) {
                break;
            }
        }
        sqlite3DbNNFreeNN(db, pList as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprListDelete(
    mut db: *mut sqlite3,
    mut pList: *mut ExprList,
) {
    unsafe {
        if !pList.is_null() {
            exprListDeleteNN(db, pList);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprListDeleteGeneric(
    mut db: *mut sqlite3,
    mut pList: *mut ::core::ffi::c_void,
) {
    unsafe {
        if !pList.is_null() {
            exprListDeleteNN(db, pList as *mut ExprList);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprListFlags(mut pList: *const ExprList) -> u32_0 {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut m: u32_0 = 0 as u32_0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pList).nExpr {
            let mut pExpr: *mut Expr = (*(&raw const (*pList).a as *const ExprList_item)
                .offset(i as isize))
                .pExpr;
            m |= (*pExpr).flags as ::core::ffi::c_uint;
            i += 1;
        }
        return m;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SelectWalkFail(
    mut pWalker: *mut Walker,
    mut NotUsed: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        (*pWalker).eCode = 0 as u16_0;
        return WRC_Abort;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IsTrueOrFalse(
    mut zIn: *const ::core::ffi::c_char,
) -> u32_0 {
    unsafe {
        if sqlite3StrICmp(zIn, b"true\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return EP_IsTrue as u32_0;
        }
        if sqlite3StrICmp(zIn, b"false\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return EP_IsFalse as u32_0;
        }
        return 0 as u32_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprIdToTrueFalse(
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: u32_0 = 0;
        if !((*pExpr).flags
            & (0x4000000 as ::core::ffi::c_int | 0x800 as ::core::ffi::c_int) as u32_0
            != 0 as ::core::ffi::c_uint)
            && {
                v = sqlite3IsTrueOrFalse((*pExpr).u.zToken);
                v != 0 as ::core::ffi::c_uint
            }
        {
            (*pExpr).op = TK_TRUEFALSE as u8_0;
            (*pExpr).flags |= v as ::core::ffi::c_uint;
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprTruthValue(
    mut pExpr: *const Expr,
) -> ::core::ffi::c_int {
    unsafe {
        pExpr = sqlite3ExprSkipCollateAndLikely(pExpr as *mut Expr);
        return (*(*pExpr).u.zToken.offset(4 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprSimplifiedAndOr(mut pExpr: *mut Expr) -> *mut Expr {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int == TK_AND
            || (*pExpr).op as ::core::ffi::c_int == TK_OR
        {
            let mut pRight: *mut Expr = sqlite3ExprSimplifiedAndOr((*pExpr).pRight);
            let mut pLeft: *mut Expr = sqlite3ExprSimplifiedAndOr((*pExpr).pLeft);
            if (*pLeft).flags as ::core::ffi::c_uint
                & (EP_OuterON | EP_IsTrue) as ::core::ffi::c_uint
                == EP_IsTrue as ::core::ffi::c_uint
                || (*pRight).flags as ::core::ffi::c_uint
                    & (EP_OuterON | EP_IsFalse) as ::core::ffi::c_uint
                    == EP_IsFalse as ::core::ffi::c_uint
            {
                pExpr = if (*pExpr).op as ::core::ffi::c_int == TK_AND {
                    pRight
                } else {
                    pLeft
                };
            } else if (*pRight).flags as ::core::ffi::c_uint
                & (EP_OuterON | EP_IsTrue) as ::core::ffi::c_uint
                == EP_IsTrue as ::core::ffi::c_uint
                || (*pLeft).flags as ::core::ffi::c_uint
                    & (EP_OuterON | EP_IsFalse) as ::core::ffi::c_uint
                    == EP_IsFalse as ::core::ffi::c_uint
            {
                pExpr = if (*pExpr).op as ::core::ffi::c_int == TK_AND {
                    pLeft
                } else {
                    pRight
                };
            }
        }
        return pExpr;
    }
}
unsafe extern "C" fn exprEvalRhsFirst(mut pExpr: *mut Expr) -> ::core::ffi::c_int {
    unsafe {
        if (*(*pExpr).pLeft).flags & 0x400000 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint
            && !((*(*pExpr).pRight).flags & 0x400000 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint)
        {
            return 1 as ::core::ffi::c_int
        } else {
            return 0 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn exprComputeOperands(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pR1: *mut ::core::ffi::c_int,
    mut pR2: *mut ::core::ffi::c_int,
    mut pFree1: *mut ::core::ffi::c_int,
    mut pFree2: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut addrIsNull: ::core::ffi::c_int = 0;
        let mut r1: ::core::ffi::c_int = 0;
        let mut r2: ::core::ffi::c_int = 0;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        if exprEvalRhsFirst(pExpr) != 0 && sqlite3ExprCanBeNull((*pExpr).pRight) != 0 {
            r2 = sqlite3ExprCodeTemp(pParse, (*pExpr).pRight, pFree2);
            addrIsNull = sqlite3VdbeAddOp1(v, OP_IsNull, r2);
        } else {
            r2 = 0 as ::core::ffi::c_int;
            addrIsNull = 0 as ::core::ffi::c_int;
        }
        r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, pFree1);
        if addrIsNull == 0 as ::core::ffi::c_int {
            if (*(*pExpr).pRight).flags & 0x400000 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint && sqlite3ExprCanBeNull((*pExpr).pLeft) != 0
            {
                addrIsNull = sqlite3VdbeAddOp1(v, OP_IsNull, r1);
            }
            r2 = sqlite3ExprCodeTemp(pParse, (*pExpr).pRight, pFree2);
        }
        *pR1 = r1;
        *pR2 = r2;
        return addrIsNull;
    }
}
#[inline(never)]
unsafe extern "C" fn exprNodeIsConstantFunction(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pDef: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if (*pExpr).flags & 0x10000 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint
            || {
                pList = (*pExpr).x.pList;
                pList.is_null()
            }
        {
            n = 0 as ::core::ffi::c_int;
        } else {
            n = (*pList).nExpr;
            sqlite3WalkExprList(pWalker, pList);
            if (*pWalker).eCode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return WRC_Abort;
            }
        }
        db = (*(*pWalker).pParse).db;
        pDef = sqlite3FindFunction(db, (*pExpr).u.zToken, n, (*db).enc, 0 as u8_0);
        if pDef.is_null() || (*pDef).xFinalize.is_some()
            || (*pDef).funcFlags as ::core::ffi::c_uint
                & (SQLITE_FUNC_CONSTANT | SQLITE_FUNC_SLOCHNG) as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
            || (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
        {
            (*pWalker).eCode = 0 as u16_0;
            return WRC_Abort;
        }
        return WRC_Prune;
    }
}
unsafe extern "C" fn exprNodeIsConstant(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pWalker).eCode as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            && (*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
        {
            (*pWalker).eCode = 0 as u16_0;
            return WRC_Abort;
        }
        let mut c2rust_current_block_47: u64;
        match (*pExpr).op as ::core::ffi::c_int {
            TK_FUNCTION => {
                if ((*pWalker).eCode as ::core::ffi::c_int >= 4 as ::core::ffi::c_int
                    || (*pExpr).flags & 0x100000 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint)
                    && !((*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint)
                {
                    if (*pWalker).eCode as ::core::ffi::c_int == 5 as ::core::ffi::c_int
                    {
                        (*pExpr).flags
                            |= 0x40000000 as ::core::ffi::c_int as u32_0
                                as ::core::ffi::c_uint;
                    }
                    return WRC_Continue;
                } else if !(*pWalker).pParse.is_null() {
                    return exprNodeIsConstantFunction(pWalker, pExpr)
                } else {
                    (*pWalker).eCode = 0 as u16_0;
                    return WRC_Abort;
                }
            }
            TK_ID => {
                if sqlite3ExprIdToTrueFalse(pExpr) != 0 {
                    return WRC_Prune;
                }
                c2rust_current_block_47 = 8457315219000651999;
            }
            TK_COLUMN | TK_AGG_FUNCTION | TK_AGG_COLUMN => {
                c2rust_current_block_47 = 8457315219000651999;
            }
            TK_IF_NULL_ROW | TK_REGISTER | TK_DOT | TK_RAISE => {
                c2rust_current_block_47 = 4068382217303356765;
            }
            TK_VARIABLE => {
                if (*pWalker).eCode as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
                    (*pExpr).op = TK_NULL as u8_0;
                } else if (*pWalker).eCode as ::core::ffi::c_int
                    == 4 as ::core::ffi::c_int
                {
                    (*pWalker).eCode = 0 as u16_0;
                    return WRC_Abort;
                }
                c2rust_current_block_47 = 7245201122033322888;
            }
            _ => {
                c2rust_current_block_47 = 7245201122033322888;
            }
        }
        match c2rust_current_block_47 {
            8457315219000651999 => {
                if (*pExpr).flags & 0x20 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint
                    && (*pWalker).eCode as ::core::ffi::c_int != 2 as ::core::ffi::c_int
                {
                    return WRC_Continue;
                }
                if (*pWalker).eCode as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                    && (*pExpr).iTable == (*pWalker).u.iCur
                {
                    return WRC_Continue;
                }
            }
            7245201122033322888 => return WRC_Continue,
            _ => {}
        }
        (*pWalker).eCode = 0 as u16_0;
        return WRC_Abort;
    }
}
unsafe extern "C" fn exprIsConst(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
    mut initFlag: ::core::ffi::c_int,
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
        w.eCode = initFlag as u16_0;
        w.pParse = pParse;
        w.xExprCallback = Some(
            exprNodeIsConstant
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
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprIsConstant(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        return exprIsConst(pParse, p, 1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn sqlite3ExprIsConstantNotJoin(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        return exprIsConst(pParse, p, 2 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn exprSelectWalkTableConstant(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pSelect).selFlags as ::core::ffi::c_uint
            & SF_Correlated as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
        {
            (*pWalker).eCode = 0 as u16_0;
            return WRC_Abort;
        }
        return WRC_Prune;
    }
}
unsafe extern "C" fn sqlite3ExprIsTableConstant(
    mut p: *mut Expr,
    mut iCur: ::core::ffi::c_int,
    mut bAllowSubq: ::core::ffi::c_int,
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
        w.eCode = 3 as u16_0;
        w.pParse = ::core::ptr::null_mut::<Parse>();
        w.xExprCallback = Some(
            exprNodeIsConstant
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        if bAllowSubq != 0 {
            w.xSelectCallback = Some(
                exprSelectWalkTableConstant
                    as unsafe extern "C" fn(
                        *mut Walker,
                        *mut Select,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
                >;
        } else {
            w.xSelectCallback = Some(
                sqlite3SelectWalkFail
                    as unsafe extern "C" fn(
                        *mut Walker,
                        *mut Select,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
                >;
        }
        w.u.iCur = iCur;
        sqlite3WalkExpr(&raw mut w, p);
        return w.eCode as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprIsSingleTableConstraint(
    mut pExpr: *mut Expr,
    mut pSrcList: *const SrcList,
    mut iSrc: ::core::ffi::c_int,
    mut bAllowSubq: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSrc: *const SrcItem = (&raw const (*pSrcList).a as *const SrcItem)
            .offset(iSrc as isize) as *const SrcItem;
        if (*pSrc).fg.jointype as ::core::ffi::c_int & JT_LTORJ != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSrc).fg.jointype as ::core::ffi::c_int & JT_LEFT != 0 {
            if !((*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint)
            {
                return 0 as ::core::ffi::c_int;
            }
            if (*pExpr).w.iJoin != (*pSrc).iCursor {
                return 0 as ::core::ffi::c_int;
            }
        } else if (*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint
        {
            return 0 as ::core::ffi::c_int
        }
        if (*pExpr).flags
            & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int) as u32_0
            != 0 as ::core::ffi::c_uint
            && (*(&raw const (*pSrcList).a as *const SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .fg
                .jointype as ::core::ffi::c_int & JT_LTORJ != 0 as ::core::ffi::c_int
        {
            let mut jj: ::core::ffi::c_int = 0;
            jj = 0 as ::core::ffi::c_int;
            while jj < iSrc {
                if (*pExpr).w.iJoin
                    == (*(&raw const (*pSrcList).a as *const SrcItem)
                        .offset(jj as isize))
                        .iCursor
                {
                    if (*(&raw const (*pSrcList).a as *const SrcItem)
                        .offset(jj as isize))
                        .fg
                        .jointype as ::core::ffi::c_int & JT_LTORJ
                        != 0 as ::core::ffi::c_int
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    break;
                } else {
                    jj += 1;
                }
            }
        }
        return sqlite3ExprIsTableConstant(pExpr, (*pSrc).iCursor, bAllowSubq);
    }
}
unsafe extern "C" fn exprNodeIsConstantOrGroupBy(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pGroupBy: *mut ExprList = (*pWalker).u.pGroupBy;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pGroupBy).nExpr {
            let mut p: *mut Expr = (*(&raw mut (*pGroupBy).a as *mut ExprList_item)
                .offset(i as isize))
                .pExpr;
            if sqlite3ExprCompare(
                ::core::ptr::null::<Parse>(),
                pExpr,
                p,
                -1 as ::core::ffi::c_int,
            ) < 2 as ::core::ffi::c_int
            {
                let mut pColl: *mut CollSeq = sqlite3ExprNNCollSeq((*pWalker).pParse, p);
                if sqlite3IsBinary(pColl) != 0 {
                    return WRC_Prune;
                }
            }
            i += 1;
        }
        if (*pExpr).flags as ::core::ffi::c_uint & EP_xIsSelect as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            (*pWalker).eCode = 0 as u16_0;
            return WRC_Abort;
        }
        return exprNodeIsConstant(pWalker, pExpr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprIsConstantOrGroupBy(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
    mut pGroupBy: *mut ExprList,
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
        w.eCode = 1 as u16_0;
        w.xExprCallback = Some(
            exprNodeIsConstantOrGroupBy
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = None;
        w.u.pGroupBy = pGroupBy;
        w.pParse = pParse;
        sqlite3WalkExpr(&raw mut w, p);
        return w.eCode as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprIsConstantOrFunction(
    mut p: *mut Expr,
    mut isInit: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        return exprIsConst(
            ::core::ptr::null_mut::<Parse>(),
            p,
            4 as ::core::ffi::c_int + isInit as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprIsInteger(
    mut p: *const Expr,
    mut pValue: *mut ::core::ffi::c_int,
    mut pParse: *mut Parse,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if p.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*p).flags as ::core::ffi::c_uint & EP_IntValue as ::core::ffi::c_uint != 0 {
            *pValue = (*p).u.iValue;
            return 1 as ::core::ffi::c_int;
        }
        match (*p).op as ::core::ffi::c_int {
            TK_UPLUS => {
                rc = sqlite3ExprIsInteger(
                    (*p).pLeft,
                    pValue,
                    ::core::ptr::null_mut::<Parse>(),
                );
            }
            TK_UMINUS => {
                let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if sqlite3ExprIsInteger(
                    (*p).pLeft,
                    &raw mut v,
                    ::core::ptr::null_mut::<Parse>(),
                ) != 0
                {
                    *pValue = -v;
                    rc = 1 as ::core::ffi::c_int;
                }
            }
            TK_VARIABLE => {
                let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                if !pParse.is_null() {
                    if !(*pParse).pVdbe.is_null() {
                        if !((*(*pParse).db).flags as ::core::ffi::c_ulonglong
                            & SQLITE_EnableQPSG as ::core::ffi::c_ulonglong
                            != 0 as ::core::ffi::c_ulonglong)
                        {
                            sqlite3VdbeSetVarmask(
                                (*pParse).pVdbe,
                                (*p).iColumn as ::core::ffi::c_int,
                            );
                            pVal = sqlite3VdbeGetBoundValue(
                                (*pParse).pReprepare,
                                (*p).iColumn as ::core::ffi::c_int,
                                SQLITE_AFF_BLOB as u8_0,
                            );
                            if !pVal.is_null() {
                                if sqlite3_value_type(pVal) == SQLITE_INTEGER {
                                    let mut vv: sqlite3_int64 = sqlite3_value_int64(pVal);
                                    if vv
                                        == vv as ::core::ffi::c_longlong
                                            & 0x7fffffff as ::core::ffi::c_longlong
                                    {
                                        *pValue = vv as ::core::ffi::c_int;
                                        rc = 1 as ::core::ffi::c_int;
                                    }
                                }
                                sqlite3ValueFree(pVal);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCanBeNull(mut p: *const Expr) -> ::core::ffi::c_int {
    unsafe {
        let mut op: u8_0 = 0;
        while (*p).op as ::core::ffi::c_int == TK_UPLUS
            || (*p).op as ::core::ffi::c_int == TK_UMINUS
        {
            p = (*p).pLeft;
        }
        op = (*p).op;
        if op as ::core::ffi::c_int == TK_REGISTER {
            op = (*p).op2;
        }
        match op as ::core::ffi::c_int {
            TK_INTEGER | TK_STRING | TK_FLOAT | TK_BLOB => return 0 as ::core::ffi::c_int,
            TK_COLUMN => {
                return ((*p).flags & 0x200000 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint || (*p).y.pTab.is_null()
                    || (*p).iColumn as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                        && !(*(*p).y.pTab).aCol.is_null()
                        && ((*p).iColumn as ::core::ffi::c_int)
                            < (*(*p).y.pTab).nCol as ::core::ffi::c_int
                        && (*(*(*p).y.pTab).aCol.offset((*p).iColumn as isize)).notNull()
                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int

        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprNeedsNoAffinityChange(
    mut p: *const Expr,
    mut aff: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut op: u8_0 = 0;
        let mut unaryMinus: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if aff as ::core::ffi::c_int == SQLITE_AFF_BLOB {
            return 1 as ::core::ffi::c_int;
        }
        while (*p).op as ::core::ffi::c_int == TK_UPLUS
            || (*p).op as ::core::ffi::c_int == TK_UMINUS
        {
            if (*p).op as ::core::ffi::c_int == TK_UMINUS {
                unaryMinus = 1 as ::core::ffi::c_int;
            }
            p = (*p).pLeft;
        }
        op = (*p).op;
        if op as ::core::ffi::c_int == TK_REGISTER {
            op = (*p).op2;
        }
        match op as ::core::ffi::c_int {
            TK_INTEGER => {
                return (aff as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC)
                    as ::core::ffi::c_int;
            }
            TK_FLOAT => {
                return (aff as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC)
                    as ::core::ffi::c_int;
            }
            TK_STRING => {
                return (unaryMinus == 0 && aff as ::core::ffi::c_int == SQLITE_AFF_TEXT)
                    as ::core::ffi::c_int;
            }
            TK_BLOB => return (unaryMinus == 0) as ::core::ffi::c_int,
            TK_COLUMN => {
                return (aff as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC
                    && ((*p).iColumn as ::core::ffi::c_int) < 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            }
            _ => return 0 as ::core::ffi::c_int

        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IsRowid(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3StrICmp(z, b"_ROWID_\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        if sqlite3StrICmp(z, b"ROWID\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        if sqlite3StrICmp(z, b"OID\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RowidAlias(
    mut pTab: *mut Table,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut azOpt: [*const ::core::ffi::c_char; 3] = [
            b"_ROWID_\0".as_ptr() as *const ::core::ffi::c_char,
            b"ROWID\0".as_ptr() as *const ::core::ffi::c_char,
            b"OID\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut ii: ::core::ffi::c_int = 0;
        ii = 0 as ::core::ffi::c_int;
        while ii
            < (::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>() as usize)
                .wrapping_div(
                    ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize,
                ) as ::core::ffi::c_int
        {
            if sqlite3ColumnIndex(pTab, azOpt[ii as usize]) < 0 as ::core::ffi::c_int {
                return azOpt[ii as usize];
            }
            ii += 1;
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn isCandidateForInOpt(mut pX: *const Expr) -> *mut Select {
    unsafe {
        let mut p: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pSrc: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
        let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut i: ::core::ffi::c_int = 0;
        if !((*pX).flags as ::core::ffi::c_uint & EP_xIsSelect as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
        {
            return ::core::ptr::null_mut::<Select>();
        }
        if (*pX).flags & 0x40 as ::core::ffi::c_int as u32_0 != 0 as ::core::ffi::c_uint
        {
            return ::core::ptr::null_mut::<Select>();
        }
        p = (*pX).x.pSelect;
        if !(*p).pPrior.is_null() {
            return ::core::ptr::null_mut::<Select>();
        }
        if (*p).selFlags as ::core::ffi::c_uint
            & (SF_Distinct | SF_Aggregate) as ::core::ffi::c_uint != 0
        {
            return ::core::ptr::null_mut::<Select>();
        }
        if !(*p).pLimit.is_null() {
            return ::core::ptr::null_mut::<Select>();
        }
        if !(*p).pWhere.is_null() {
            return ::core::ptr::null_mut::<Select>();
        }
        pSrc = (*p).pSrc;
        if (*pSrc).nSrc != 1 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<Select>();
        }
        if (*(&raw mut (*pSrc).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .fg
            .isSubquery() != 0
        {
            return ::core::ptr::null_mut::<Select>();
        }
        pTab = (*(&raw mut (*pSrc).a as *mut SrcItem)
            .offset(0 as ::core::ffi::c_int as isize))
            .pSTab;
        if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
            return ::core::ptr::null_mut::<Select>();
        }
        pEList = (*p).pEList;
        i = 0 as ::core::ffi::c_int;
        while i < (*pEList).nExpr {
            let mut pRes: *mut Expr = (*(&raw mut (*pEList).a as *mut ExprList_item)
                .offset(i as isize))
                .pExpr;
            if (*pRes).op as ::core::ffi::c_int != TK_COLUMN {
                return ::core::ptr::null_mut::<Select>();
            }
            i += 1;
        }
        return p;
    }
}
unsafe extern "C" fn sqlite3SetHasNullFlag(
    mut v: *mut Vdbe,
    mut iCur: ::core::ffi::c_int,
    mut regHasNull: ::core::ffi::c_int,
) {
    unsafe {
        let mut addr1: ::core::ffi::c_int = 0;
        sqlite3VdbeAddOp2(v, OP_Integer, 0 as ::core::ffi::c_int, regHasNull);
        addr1 = sqlite3VdbeAddOp1(v, OP_Rewind, iCur);
        sqlite3VdbeAddOp3(v, OP_Column, iCur, 0 as ::core::ffi::c_int, regHasNull);
        sqlite3VdbeChangeP5(v, OPFLAG_TYPEOFARG as u16_0);
        sqlite3VdbeJumpHere(v, addr1);
    }
}
unsafe extern "C" fn sqlite3InRhsIsConstant(
    mut pParse: *mut Parse,
    mut pIn: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pLHS: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut res: ::core::ffi::c_int = 0;
        pLHS = (*pIn).pLeft;
        (*pIn).pLeft = ::core::ptr::null_mut::<Expr>();
        res = sqlite3ExprIsConstant(pParse, pIn);
        (*pIn).pLeft = pLHS;
        return res;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FindInIndex(
    mut pParse: *mut Parse,
    mut pX: *mut Expr,
    mut inFlags: u32_0,
    mut prRhsHasNull: *mut ::core::ffi::c_int,
    mut aiMap: *mut ::core::ffi::c_int,
    mut piTab: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut eType: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iTab: ::core::ffi::c_int = 0;
        let mut mustBeUnique: ::core::ffi::c_int = 0;
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        mustBeUnique = (inFlags as ::core::ffi::c_uint
            & IN_INDEX_LOOP as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        let c2rust_fresh8 = (*pParse).nTab;
        (*pParse).nTab = (*pParse).nTab + 1;
        iTab = c2rust_fresh8;
        if !prRhsHasNull.is_null()
            && (*pX).flags as ::core::ffi::c_uint & EP_xIsSelect as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
        {
            let mut i: ::core::ffi::c_int = 0;
            let mut pEList: *mut ExprList = (*(*pX).x.pSelect).pEList;
            i = 0 as ::core::ffi::c_int;
            while i < (*pEList).nExpr {
                if sqlite3ExprCanBeNull(
                    (*(&raw mut (*pEList).a as *mut ExprList_item).offset(i as isize))
                        .pExpr,
                ) != 0
                {
                    break;
                }
                i += 1;
            }
            if i == (*pEList).nExpr {
                prRhsHasNull = ::core::ptr::null_mut::<::core::ffi::c_int>();
            }
        }
        if (*pParse).nErr == 0 as ::core::ffi::c_int
            && {
                p = isCandidateForInOpt(pX);
                !p.is_null()
            }
        {
            let mut db: *mut sqlite3 = (*pParse).db;
            let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
            let mut iDb: ::core::ffi::c_int = 0;
            let mut pEList_0: *mut ExprList = (*p).pEList;
            let mut nExpr: ::core::ffi::c_int = (*pEList_0).nExpr;
            pTab = (*(&raw mut (*(*p).pSrc).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .pSTab;
            iDb = sqlite3SchemaToIndex(db, (*pTab).pSchema);
            sqlite3CodeVerifySchema(pParse, iDb);
            if nExpr == 1 as ::core::ffi::c_int
                && ((*(*(&raw mut (*pEList_0).a as *mut ExprList_item)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .pExpr)
                    .iColumn as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            {
                let mut iAddr: ::core::ffi::c_int = sqlite3VdbeAddOp0(v, OP_Once);
                sqlite3OpenTable(pParse, iTab, iDb, pTab, OP_OpenRead);
                eType = IN_INDEX_ROWID;
                sqlite3VdbeExplain(
                    pParse,
                    0 as u8_0,
                    b"USING ROWID SEARCH ON TABLE %s FOR IN-OPERATOR\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*pTab).zName,
                );
                sqlite3VdbeJumpHere(v, iAddr);
            } else {
                let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
                let mut affinity_ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                let mut i_0: ::core::ffi::c_int = 0;
                i_0 = 0 as ::core::ffi::c_int;
                while i_0 < nExpr && affinity_ok != 0 {
                    let mut pLhs: *mut Expr = sqlite3VectorFieldSubexpr(
                        (*pX).pLeft,
                        i_0,
                    );
                    let mut iCol: ::core::ffi::c_int = (*(*(&raw mut (*pEList_0).a
                        as *mut ExprList_item)
                        .offset(i_0 as isize))
                        .pExpr)
                        .iColumn as ::core::ffi::c_int;
                    let mut idxaff: ::core::ffi::c_char = sqlite3TableColumnAffinity(
                        pTab,
                        iCol,
                    );
                    let mut cmpaff: ::core::ffi::c_char = sqlite3CompareAffinity(
                        pLhs,
                        idxaff,
                    );
                    match cmpaff as ::core::ffi::c_int {
                        SQLITE_AFF_BLOB | SQLITE_AFF_TEXT => {}
                        _ => {
                            affinity_ok = (idxaff as ::core::ffi::c_int
                                >= SQLITE_AFF_NUMERIC) as ::core::ffi::c_int;
                        }
                    }
                    i_0 += 1;
                }
                if affinity_ok != 0 {
                    let mut c2rust_current_block_63: u64;
                    pIdx = (*pTab).pIndex;
                    while !pIdx.is_null() && eType == 0 as ::core::ffi::c_int {
                        let mut colUsed: Bitmask = 0;
                        let mut mCol: Bitmask = 0;
                        if !(((*pIdx).nColumn as ::core::ffi::c_int) < nExpr) {
                            if (*pIdx).pPartIdxWhere.is_null() {
                                if !((*pIdx).nColumn as ::core::ffi::c_int
                                    >= BMS - 1 as ::core::ffi::c_int)
                                {
                                    if mustBeUnique != 0 {
                                        if (*pIdx).nKeyCol as ::core::ffi::c_int > nExpr
                                            || (*pIdx).nColumn as ::core::ffi::c_int > nExpr
                                                && !((*pIdx).onError as ::core::ffi::c_int != OE_None)
                                        {
                                            c2rust_current_block_63 = 7659304154607701039;
                                        } else {
                                            c2rust_current_block_63 = 10399321362245223758;
                                        }
                                    } else {
                                        c2rust_current_block_63 = 10399321362245223758;
                                    }
                                    match c2rust_current_block_63 {
                                        7659304154607701039 => {}
                                        _ => {
                                            colUsed = 0 as Bitmask;
                                            i_0 = 0 as ::core::ffi::c_int;
                                            while i_0 < nExpr {
                                                let mut pLhs_0: *mut Expr = sqlite3VectorFieldSubexpr(
                                                    (*pX).pLeft,
                                                    i_0,
                                                );
                                                let mut pRhs: *mut Expr = (*(&raw mut (*pEList_0).a
                                                    as *mut ExprList_item)
                                                    .offset(i_0 as isize))
                                                    .pExpr;
                                                let mut pReq: *mut CollSeq = sqlite3BinaryCompareCollSeq(
                                                    pParse,
                                                    pLhs_0,
                                                    pRhs,
                                                );
                                                let mut j: ::core::ffi::c_int = 0;
                                                j = 0 as ::core::ffi::c_int;
                                                while j < nExpr {
                                                    if !(*(*pIdx).aiColumn.offset(j as isize)
                                                        as ::core::ffi::c_int
                                                        != (*pRhs).iColumn as ::core::ffi::c_int)
                                                    {
                                                        if !(!pReq.is_null()
                                                            && sqlite3StrICmp(
                                                                (*pReq).zName,
                                                                *(*pIdx).azColl.offset(j as isize),
                                                            ) != 0 as ::core::ffi::c_int)
                                                        {
                                                            break;
                                                        }
                                                    }
                                                    j += 1;
                                                }
                                                if j == nExpr {
                                                    break;
                                                }
                                                mCol = (1 as ::core::ffi::c_int as Bitmask) << j;
                                                if mCol & colUsed != 0 {
                                                    break;
                                                }
                                                colUsed |= mCol as ::core::ffi::c_ulonglong;
                                                if !aiMap.is_null() {
                                                    *aiMap.offset(i_0 as isize) = j;
                                                }
                                                i_0 += 1;
                                            }
                                            if colUsed
                                                == ((1 as ::core::ffi::c_int as ::core::ffi::c_ulonglong)
                                                    << nExpr)
                                                    .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                                            {
                                                let mut iAddr_0: ::core::ffi::c_int = sqlite3VdbeAddOp0(
                                                    v,
                                                    OP_Once,
                                                );
                                                sqlite3VdbeExplain(
                                                    pParse,
                                                    0 as u8_0,
                                                    b"USING INDEX %s FOR IN-OPERATOR\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    (*pIdx).zName,
                                                );
                                                sqlite3VdbeAddOp3(
                                                    v,
                                                    OP_OpenRead,
                                                    iTab,
                                                    (*pIdx).tnum as ::core::ffi::c_int,
                                                    iDb,
                                                );
                                                sqlite3VdbeSetP4KeyInfo(pParse, pIdx);
                                                eType = IN_INDEX_INDEX_ASC
                                                    + *(*pIdx)
                                                        .aSortOrder
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int;
                                                if !prRhsHasNull.is_null() {
                                                    (*pParse).nMem += 1;
                                                    *prRhsHasNull = (*pParse).nMem;
                                                    if nExpr == 1 as ::core::ffi::c_int {
                                                        sqlite3SetHasNullFlag(v, iTab, *prRhsHasNull);
                                                    }
                                                }
                                                sqlite3VdbeJumpHere(v, iAddr_0);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        pIdx = (*pIdx).pNext;
                    }
                }
            }
        }
        if eType == 0 as ::core::ffi::c_int
            && inFlags as ::core::ffi::c_uint & IN_INDEX_NOOP_OK as ::core::ffi::c_uint
                != 0
            && (*pX).flags as ::core::ffi::c_uint & EP_xIsSelect as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
            && (sqlite3InRhsIsConstant(pParse, pX) == 0
                || (*(*pX).x.pList).nExpr <= 2 as ::core::ffi::c_int)
        {
            (*pParse).nTab -= 1;
            iTab = -1 as ::core::ffi::c_int;
            eType = IN_INDEX_NOOP;
        }
        if eType == 0 as ::core::ffi::c_int {
            let mut savedNQueryLoop: u32_0 = (*pParse).nQueryLoop as u32_0;
            let mut rMayHaveNull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            eType = IN_INDEX_EPH;
            if inFlags as ::core::ffi::c_uint & IN_INDEX_LOOP as ::core::ffi::c_uint != 0
            {
                (*pParse).nQueryLoop = 0 as LogEst;
            } else if !prRhsHasNull.is_null() {
                (*pParse).nMem += 1;
                rMayHaveNull = (*pParse).nMem;
                *prRhsHasNull = rMayHaveNull;
            }
            sqlite3CodeRhsOfIN(pParse, pX, iTab);
            if rMayHaveNull != 0 {
                sqlite3SetHasNullFlag(v, iTab, rMayHaveNull);
            }
            (*pParse).nQueryLoop = savedNQueryLoop as LogEst;
        }
        if !aiMap.is_null() && eType != IN_INDEX_INDEX_ASC
            && eType != IN_INDEX_INDEX_DESC
        {
            let mut i_1: ::core::ffi::c_int = 0;
            let mut n: ::core::ffi::c_int = 0;
            n = sqlite3ExprVectorSize((*pX).pLeft);
            i_1 = 0 as ::core::ffi::c_int;
            while i_1 < n {
                *aiMap.offset(i_1 as isize) = i_1;
                i_1 += 1;
            }
        }
        *piTab = iTab;
        return eType;
    }
}
unsafe extern "C" fn exprINAffinity(
    mut pParse: *mut Parse,
    mut pExpr: *const Expr,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut pLeft: *mut Expr = (*pExpr).pLeft;
        let mut nVal: ::core::ffi::c_int = sqlite3ExprVectorSize(pLeft);
        let mut pSelect: *mut Select = if (*pExpr).flags as ::core::ffi::c_uint
            & EP_xIsSelect as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
        {
            (*pExpr).x.pSelect
        } else {
            ::core::ptr::null_mut::<Select>()
        };
        let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        zRet = sqlite3DbMallocRaw((*pParse).db, (1 as i64_0 + nVal as i64_0) as u64_0)
            as *mut ::core::ffi::c_char;
        if !zRet.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < nVal {
                let mut pA: *mut Expr = sqlite3VectorFieldSubexpr(pLeft, i);
                let mut a: ::core::ffi::c_char = sqlite3ExprAffinity(pA);
                if !pSelect.is_null() {
                    *zRet.offset(i as isize) = sqlite3CompareAffinity(
                        (*(&raw mut (*(*pSelect).pEList).a as *mut ExprList_item)
                            .offset(i as isize))
                            .pExpr,
                        a,
                    );
                } else {
                    *zRet.offset(i as isize) = a;
                }
                i += 1;
            }
            *zRet.offset(nVal as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
        return zRet;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SubselectError(
    mut pParse: *mut Parse,
    mut nActual: ::core::ffi::c_int,
    mut nExpect: ::core::ffi::c_int,
) {
    unsafe {
        if (*pParse).nErr == 0 as ::core::ffi::c_int {
            let mut zFmt: *const ::core::ffi::c_char = b"sub-select returns %d columns - expected %d\0"
                .as_ptr() as *const ::core::ffi::c_char;
            sqlite3ErrorMsg(pParse, zFmt, nActual, nExpect);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VectorErrorMsg(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
) {
    unsafe {
        if (*pExpr).flags as ::core::ffi::c_uint & EP_xIsSelect as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            sqlite3SubselectError(
                pParse,
                (*(*(*pExpr).x.pSelect).pEList).nExpr,
                1 as ::core::ffi::c_int,
            );
        } else {
            sqlite3ErrorMsg(
                pParse,
                b"row value misused\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
    }
}
unsafe extern "C" fn findCompatibleInRhsSubrtn(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pNewSig: *mut SubrtnSig,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
        let mut pEnd: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
        let mut pSig: *mut SubrtnSig = ::core::ptr::null_mut::<SubrtnSig>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        if pNewSig.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*pParse).mSubrtnSig as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << ((*pNewSig).selId & 7 as ::core::ffi::c_int)
            == 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        v = (*pParse).pVdbe;
        pOp = sqlite3VdbeGetOp(v, 1 as ::core::ffi::c_int);
        pEnd = sqlite3VdbeGetLastOp(v);
        while pOp < pEnd {
            if !((*pOp).p4type as ::core::ffi::c_int != P4_SUBRTNSIG) {
                pSig = (*pOp).p4.pSubrtnSig;
                if !((*pSig).bComplete == 0) {
                    if !((*pNewSig).selId != (*pSig).selId) {
                        if !(strcmp((*pNewSig).zAff, (*pSig).zAff)
                            != 0 as ::core::ffi::c_int)
                        {
                            (*pExpr).y.sub.iAddr = (*pSig).iAddr;
                            (*pExpr).y.sub.regReturn = (*pSig).regReturn;
                            (*pExpr).iTable = (*pSig).iTable;
                            (*pExpr).flags
                                |= 0x2000000 as ::core::ffi::c_int as u32_0
                                    as ::core::ffi::c_uint;
                            return 1 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            pOp = pOp.offset(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CodeRhsOfIN(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut iTab: ::core::ffi::c_int,
) {
    unsafe {
        let mut addrOnce: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addr: ::core::ffi::c_int = 0;
        let mut pLeft: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pKeyInfo: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        let mut nVal: ::core::ffi::c_int = 0;
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut pSig: *mut SubrtnSig = ::core::ptr::null_mut::<SubrtnSig>();
        v = (*pParse).pVdbe;
        if !((*pExpr).flags & 0x40 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint) && (*pParse).iSelfTab == 0 as ::core::ffi::c_int
        {
            if (*pExpr).flags as ::core::ffi::c_uint
                & EP_xIsSelect as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                && (*(*pExpr).x.pSelect).selFlags as ::core::ffi::c_uint
                    & SF_All as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            {
                pSig = sqlite3DbMallocRawNN(
                    (*pParse).db,
                    ::core::mem::size_of::<SubrtnSig>() as u64_0,
                ) as *mut SubrtnSig;
                if !pSig.is_null() {
                    (*pSig).selId = (*(*pExpr).x.pSelect).selId as ::core::ffi::c_int;
                    (*pSig).zAff = exprINAffinity(pParse, pExpr);
                }
            }
            if (*pExpr).flags & 0x2000000 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
                || findCompatibleInRhsSubrtn(pParse, pExpr, pSig) != 0
            {
                addrOnce = sqlite3VdbeAddOp0(v, OP_Once);
                if (*pExpr).flags as ::core::ffi::c_uint
                    & EP_xIsSelect as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                {
                    sqlite3VdbeExplain(
                        pParse,
                        0 as u8_0,
                        b"REUSE LIST SUBQUERY %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*(*pExpr).x.pSelect).selId,
                    );
                }
                sqlite3VdbeAddOp2(
                    v,
                    OP_Gosub,
                    (*pExpr).y.sub.regReturn,
                    (*pExpr).y.sub.iAddr,
                );
                sqlite3VdbeAddOp2(v, OP_OpenDup, iTab, (*pExpr).iTable);
                sqlite3VdbeJumpHere(v, addrOnce);
                if !pSig.is_null() {
                    sqlite3DbFree(
                        (*pParse).db,
                        (*pSig).zAff as *mut ::core::ffi::c_void,
                    );
                    sqlite3DbFree((*pParse).db, pSig as *mut ::core::ffi::c_void);
                }
                return;
            }
            (*pExpr).flags
                |= 0x2000000 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint;
            (*pParse).nMem += 1;
            (*pExpr).y.sub.regReturn = (*pParse).nMem;
            (*pExpr).y.sub.iAddr = sqlite3VdbeAddOp2(
                v,
                OP_BeginSubrtn,
                0 as ::core::ffi::c_int,
                (*pExpr).y.sub.regReturn,
            ) + 1 as ::core::ffi::c_int;
            if !pSig.is_null() {
                (*pSig).bComplete = 0 as u8_0;
                (*pSig).iAddr = (*pExpr).y.sub.iAddr;
                (*pSig).regReturn = (*pExpr).y.sub.regReturn;
                (*pSig).iTable = iTab;
                (*pParse).mSubrtnSig = ((1 as ::core::ffi::c_int)
                    << ((*pSig).selId & 7 as ::core::ffi::c_int)) as u8_0;
                sqlite3VdbeChangeP4(
                    v,
                    -1 as ::core::ffi::c_int,
                    pSig as *const ::core::ffi::c_char,
                    P4_SUBRTNSIG,
                );
            }
            addrOnce = sqlite3VdbeAddOp0(v, OP_Once);
        }
        pLeft = (*pExpr).pLeft;
        nVal = sqlite3ExprVectorSize(pLeft);
        (*pExpr).iTable = iTab;
        addr = sqlite3VdbeAddOp2(v, OP_OpenEphemeral, (*pExpr).iTable, nVal);
        pKeyInfo = sqlite3KeyInfoAlloc((*pParse).db, nVal, 1 as ::core::ffi::c_int);
        if (*pExpr).flags as ::core::ffi::c_uint & EP_xIsSelect as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            let mut pSelect: *mut Select = (*pExpr).x.pSelect;
            let mut pEList: *mut ExprList = (*pSelect).pEList;
            sqlite3VdbeExplain(
                pParse,
                1 as u8_0,
                b"%sLIST SUBQUERY %d\0".as_ptr() as *const ::core::ffi::c_char,
                if addrOnce != 0 {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"CORRELATED \0".as_ptr() as *const ::core::ffi::c_char
                },
                (*pSelect).selId,
            );
            if (*pEList).nExpr == nVal {
                let mut pCopy: *mut Select = ::core::ptr::null_mut::<Select>();
                let mut dest: SelectDest = SelectDest {
                    eDest: 0,
                    iSDParm: 0,
                    iSDParm2: 0,
                    iSdst: 0,
                    nSdst: 0,
                    zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    pOrderBy: ::core::ptr::null_mut::<ExprList>()

                };
                let mut i: ::core::ffi::c_int = 0;
                let mut rc: ::core::ffi::c_int = 0;
                let mut addrBloom: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                sqlite3SelectDestInit(&raw mut dest, SRT_Set, iTab);
                dest.zAffSdst = exprINAffinity(pParse, pExpr);
                (*pSelect).iLimit = 0 as ::core::ffi::c_int;
                if addrOnce != 0
                    && (*(*pParse).db).dbOptFlags as ::core::ffi::c_uint
                        & 0x80000 as ::core::ffi::c_int as ::core::ffi::c_uint
                        == 0 as ::core::ffi::c_uint
                {
                    (*pParse).nMem += 1;
                    let mut regBloom: ::core::ffi::c_int = (*pParse).nMem;
                    addrBloom = sqlite3VdbeAddOp2(
                        v,
                        OP_Blob,
                        10000 as ::core::ffi::c_int,
                        regBloom,
                    );
                    dest.iSDParm2 = regBloom;
                }
                pCopy = sqlite3SelectDup((*pParse).db, pSelect, 0 as ::core::ffi::c_int);
                rc = if (*(*pParse).db).mallocFailed as ::core::ffi::c_int != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    sqlite3Select(pParse, pCopy, &raw mut dest)
                };
                sqlite3SelectDelete((*pParse).db, pCopy);
                sqlite3DbFree((*pParse).db, dest.zAffSdst as *mut ::core::ffi::c_void);
                if addrBloom != 0 {
                    (*sqlite3VdbeGetOp(v, addrOnce)).p3 = dest.iSDParm2;
                    if dest.iSDParm2 == 0 as ::core::ffi::c_int {
                        (*sqlite3VdbeGetOp(v, addrBloom)).p1 = 10 as ::core::ffi::c_int;
                    }
                }
                if rc != 0 {
                    sqlite3KeyInfoUnref(pKeyInfo);
                    return;
                }
                i = 0 as ::core::ffi::c_int;
                while i < nVal {
                    let mut p: *mut Expr = sqlite3VectorFieldSubexpr(pLeft, i);
                    let ref mut c2rust_fresh9 = *(&raw mut (*pKeyInfo).aColl
                        as *mut *mut CollSeq)
                        .offset(i as isize);
                    *c2rust_fresh9 = sqlite3BinaryCompareCollSeq(
                        pParse,
                        p,
                        (*(&raw mut (*pEList).a as *mut ExprList_item)
                            .offset(i as isize))
                            .pExpr,
                    );
                    i += 1;
                }
            }
        } else if !(*pExpr).x.pList.is_null() {
            let mut affinity: ::core::ffi::c_char = 0;
            let mut i_0: ::core::ffi::c_int = 0;
            let mut pList: *mut ExprList = (*pExpr).x.pList;
            let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
            let mut r1: ::core::ffi::c_int = 0;
            let mut r2: ::core::ffi::c_int = 0;
            affinity = sqlite3ExprAffinity(pLeft);
            if affinity as ::core::ffi::c_int <= SQLITE_AFF_NONE {
                affinity = SQLITE_AFF_BLOB as ::core::ffi::c_char;
            } else if affinity as ::core::ffi::c_int == SQLITE_AFF_REAL {
                affinity = SQLITE_AFF_NUMERIC as ::core::ffi::c_char;
            }
            if !pKeyInfo.is_null() {
                let ref mut c2rust_fresh10 = *(&raw mut (*pKeyInfo).aColl
                    as *mut *mut CollSeq)
                    .offset(0 as ::core::ffi::c_int as isize);
                *c2rust_fresh10 = sqlite3ExprCollSeq(pParse, (*pExpr).pLeft);
            }
            r1 = sqlite3GetTempReg(pParse);
            r2 = sqlite3GetTempReg(pParse);
            i_0 = (*pList).nExpr;
            pItem = &raw mut (*pList).a as *mut ExprList_item as *mut ExprList_item;
            while i_0 > 0 as ::core::ffi::c_int {
                let mut pE2: *mut Expr = (*pItem).pExpr;
                if addrOnce != 0 && sqlite3ExprIsConstant(pParse, pE2) == 0 {
                    sqlite3VdbeChangeToNoop(v, addrOnce - 1 as ::core::ffi::c_int);
                    sqlite3VdbeChangeToNoop(v, addrOnce);
                    (*pExpr).flags
                        &= !(0x2000000 as ::core::ffi::c_int as u32_0)
                            as ::core::ffi::c_uint;
                    addrOnce = 0 as ::core::ffi::c_int;
                }
                sqlite3ExprCode(pParse, pE2, r1);
                sqlite3VdbeAddOp4(
                    v,
                    OP_MakeRecord,
                    r1,
                    1 as ::core::ffi::c_int,
                    r2,
                    &raw mut affinity,
                    1 as ::core::ffi::c_int,
                );
                sqlite3VdbeAddOp4Int(
                    v,
                    OP_IdxInsert,
                    iTab,
                    r2,
                    r1,
                    1 as ::core::ffi::c_int,
                );
                i_0 -= 1;
                pItem = pItem.offset(1);
            }
            sqlite3ReleaseTempReg(pParse, r1);
            sqlite3ReleaseTempReg(pParse, r2);
        }
        if !pSig.is_null() {
            (*pSig).bComplete = 1 as u8_0;
        }
        if !pKeyInfo.is_null() {
            sqlite3VdbeChangeP4(
                v,
                addr,
                pKeyInfo as *mut ::core::ffi::c_void as *const ::core::ffi::c_char,
                P4_KEYINFO,
            );
        }
        if addrOnce != 0 {
            sqlite3VdbeAddOp1(v, OP_NullRow, iTab);
            sqlite3VdbeJumpHere(v, addrOnce);
            sqlite3VdbeAddOp3(
                v,
                OP_Return,
                (*pExpr).y.sub.regReturn,
                (*pExpr).y.sub.iAddr,
                1 as ::core::ffi::c_int,
            );
            sqlite3ClearTempRegCache(pParse);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CodeSubselect(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut addrOnce: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rReg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pSel: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut dest: SelectDest = SelectDest {
            eDest: 0,
            iSDParm: 0,
            iSDParm2: 0,
            iSdst: 0,
            nSdst: 0,
            zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pOrderBy: ::core::ptr::null_mut::<ExprList>()

        };
        let mut nReg: ::core::ffi::c_int = 0;
        let mut pLimit: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        if (*pParse).nErr != 0 {
            return 0 as ::core::ffi::c_int;
        }
        pSel = (*pExpr).x.pSelect;
        if (*pExpr).flags & 0x2000000 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint
        {
            sqlite3VdbeExplain(
                pParse,
                0 as u8_0,
                b"REUSE SUBQUERY %d\0".as_ptr() as *const ::core::ffi::c_char,
                (*pSel).selId,
            );
            sqlite3VdbeAddOp2(
                v,
                OP_Gosub,
                (*pExpr).y.sub.regReturn,
                (*pExpr).y.sub.iAddr,
            );
            return (*pExpr).iTable;
        }
        (*pExpr).flags
            |= 0x2000000 as ::core::ffi::c_int as u32_0 as ::core::ffi::c_uint;
        (*pParse).nMem += 1;
        (*pExpr).y.sub.regReturn = (*pParse).nMem;
        (*pExpr).y.sub.iAddr = sqlite3VdbeAddOp2(
            v,
            OP_BeginSubrtn,
            0 as ::core::ffi::c_int,
            (*pExpr).y.sub.regReturn,
        ) + 1 as ::core::ffi::c_int;
        if !((*pExpr).flags & 0x40 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint)
        {
            addrOnce = sqlite3VdbeAddOp0(v, OP_Once);
        }
        sqlite3VdbeExplain(
            pParse,
            1 as u8_0,
            b"%sSCALAR SUBQUERY %d\0".as_ptr() as *const ::core::ffi::c_char,
            if addrOnce != 0 {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"CORRELATED \0".as_ptr() as *const ::core::ffi::c_char
            },
            (*pSel).selId,
        );
        nReg = if (*pExpr).op as ::core::ffi::c_int == TK_SELECT {
            (*(*pSel).pEList).nExpr
        } else {
            1 as ::core::ffi::c_int
        };
        sqlite3SelectDestInit(
            &raw mut dest,
            0 as ::core::ffi::c_int,
            (*pParse).nMem + 1 as ::core::ffi::c_int,
        );
        (*pParse).nMem += nReg;
        if (*pExpr).op as ::core::ffi::c_int == TK_SELECT {
            dest.eDest = SRT_Mem as u8_0;
            if (*pSel).selFlags as ::core::ffi::c_uint
                & SF_Distinct as ::core::ffi::c_uint != 0 && !(*pSel).pLimit.is_null()
                && !(*(*pSel).pLimit).pRight.is_null()
            {
                dest.iSdst = (*pParse).nMem + 1 as ::core::ffi::c_int;
                (*pParse).nMem += nReg;
            } else {
                dest.iSdst = dest.iSDParm;
            }
            dest.nSdst = nReg;
            sqlite3VdbeAddOp3(
                v,
                OP_Null,
                0 as ::core::ffi::c_int,
                dest.iSDParm,
                (*pParse).nMem,
            );
        } else {
            dest.eDest = SRT_Exists as u8_0;
            sqlite3VdbeAddOp2(v, OP_Integer, 0 as ::core::ffi::c_int, dest.iSDParm);
        }
        if !(*pSel).pLimit.is_null() {
            let mut pLeft: *mut Expr = (*(*pSel).pLimit).pLeft;
            if ((*pLeft).flags & 0x800 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                || (*pLeft).u.iValue != 1 as ::core::ffi::c_int
                    && (*pLeft).u.iValue != 0 as ::core::ffi::c_int
            {
                let mut db: *mut sqlite3 = (*pParse).db;
                pLimit = sqlite3Expr(
                    db,
                    TK_INTEGER,
                    b"0\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if !pLimit.is_null() {
                    (*pLimit).affExpr = SQLITE_AFF_NUMERIC as ::core::ffi::c_char;
                    pLimit = sqlite3PExpr(
                        pParse,
                        TK_NE,
                        sqlite3ExprDup(db, pLeft, 0 as ::core::ffi::c_int),
                        pLimit,
                    );
                }
                sqlite3ExprDeferredDelete(pParse, pLeft);
                (*(*pSel).pLimit).pLeft = pLimit;
            }
        } else {
            pLimit = sqlite3Expr(
                (*pParse).db,
                TK_INTEGER,
                b"1\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*pSel).pLimit = sqlite3PExpr(
                pParse,
                TK_LIMIT,
                pLimit,
                ::core::ptr::null_mut::<Expr>(),
            );
        }
        (*pSel).iLimit = 0 as ::core::ffi::c_int;
        if sqlite3Select(pParse, pSel, &raw mut dest) != 0 {
            (*pExpr).op2 = (*pExpr).op;
            (*pExpr).op = TK_ERROR as u8_0;
            return 0 as ::core::ffi::c_int;
        }
        rReg = dest.iSDParm;
        (*pExpr).iTable = rReg;
        if addrOnce != 0 {
            sqlite3VdbeJumpHere(v, addrOnce);
        }
        sqlite3VdbeAddOp3(
            v,
            OP_Return,
            (*pExpr).y.sub.regReturn,
            (*pExpr).y.sub.iAddr,
            1 as ::core::ffi::c_int,
        );
        sqlite3ClearTempRegCache(pParse);
        return rReg;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCheckIN(
    mut pParse: *mut Parse,
    mut pIn: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nVector: ::core::ffi::c_int = sqlite3ExprVectorSize((*pIn).pLeft);
        if (*pIn).flags as ::core::ffi::c_uint & EP_xIsSelect as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint && (*(*pParse).db).mallocFailed == 0
        {
            if nVector != (*(*(*pIn).x.pSelect).pEList).nExpr {
                sqlite3SubselectError(
                    pParse,
                    (*(*(*pIn).x.pSelect).pEList).nExpr,
                    nVector,
                );
                return 1 as ::core::ffi::c_int;
            }
        } else if nVector != 1 as ::core::ffi::c_int {
            sqlite3VectorErrorMsg(pParse, (*pIn).pLeft);
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn sqlite3ExprCodeIN(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut destIfFalse: ::core::ffi::c_int,
    mut destIfNull: ::core::ffi::c_int,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rRhsHasNull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut eType: ::core::ffi::c_int = 0;
        let mut rLhs: ::core::ffi::c_int = 0;
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut aiMap: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        let mut zAff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nVector: ::core::ffi::c_int = 0;
        let mut iDummy: ::core::ffi::c_int = 0;
        let mut pLeft: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut i: ::core::ffi::c_int = 0;
        let mut destStep2: ::core::ffi::c_int = 0;
        let mut destStep6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut addrTruthOp: ::core::ffi::c_int = 0;
        let mut destNotNull: ::core::ffi::c_int = 0;
        let mut addrTop: ::core::ffi::c_int = 0;
        let mut iTab: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut okConstFactor: u8_0 = (*pParse).okConstFactor() as u8_0;
        pLeft = (*pExpr).pLeft;
        if sqlite3ExprCheckIN(pParse, pExpr) != 0 {
            return;
        }
        zAff = exprINAffinity(pParse, pExpr);
        nVector = sqlite3ExprVectorSize((*pExpr).pLeft);
        aiMap = sqlite3DbMallocZero(
            (*pParse).db,
            (nVector as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                as u64_0,
        ) as *mut ::core::ffi::c_int;
        if !((*(*pParse).db).mallocFailed != 0) {
            v = (*pParse).pVdbe;
            eType = sqlite3FindInIndex(
                pParse,
                pExpr,
                (IN_INDEX_MEMBERSHIP | IN_INDEX_NOOP_OK) as u32_0,
                if destIfFalse == destIfNull {
                    ::core::ptr::null_mut::<::core::ffi::c_int>()
                } else {
                    &raw mut rRhsHasNull
                },
                aiMap,
                &raw mut iTab,
            );
            (*pParse).set_okConstFactor(0 as bft as bft);
            rLhs = exprCodeVector(pParse, pLeft, &raw mut iDummy);
            (*pParse).set_okConstFactor(okConstFactor as bft as bft);
            if eType == IN_INDEX_NOOP {
                let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
                let mut labelOk: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
                let mut r2: ::core::ffi::c_int = 0;
                let mut regToFree: ::core::ffi::c_int = 0;
                let mut regCkNull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut ii: ::core::ffi::c_int = 0;
                pList = (*pExpr).x.pList;
                pColl = sqlite3ExprCollSeq(pParse, (*pExpr).pLeft);
                if destIfNull != destIfFalse {
                    regCkNull = sqlite3GetTempReg(pParse);
                    sqlite3VdbeAddOp3(v, OP_BitAnd, rLhs, rLhs, regCkNull);
                }
                ii = 0 as ::core::ffi::c_int;
                while ii < (*pList).nExpr {
                    r2 = sqlite3ExprCodeTemp(
                        pParse,
                        (*(&raw mut (*pList).a as *mut ExprList_item)
                            .offset(ii as isize))
                            .pExpr,
                        &raw mut regToFree,
                    );
                    if regCkNull != 0
                        && sqlite3ExprCanBeNull(
                            (*(&raw mut (*pList).a as *mut ExprList_item)
                                .offset(ii as isize))
                                .pExpr,
                        ) != 0
                    {
                        sqlite3VdbeAddOp3(v, OP_BitAnd, regCkNull, r2, regCkNull);
                    }
                    sqlite3ReleaseTempReg(pParse, regToFree);
                    if ii < (*pList).nExpr - 1 as ::core::ffi::c_int
                        || destIfNull != destIfFalse
                    {
                        let mut op: ::core::ffi::c_int = if rLhs != r2 {
                            OP_Eq
                        } else {
                            OP_NotNull
                        };
                        sqlite3VdbeAddOp4(
                            v,
                            op,
                            rLhs,
                            labelOk,
                            r2,
                            pColl as *mut ::core::ffi::c_void
                                as *const ::core::ffi::c_char,
                            P4_COLLSEQ,
                        );
                        sqlite3VdbeChangeP5(
                            v,
                            *zAff.offset(0 as ::core::ffi::c_int as isize) as u16_0,
                        );
                    } else {
                        let mut op_0: ::core::ffi::c_int = if rLhs != r2 {
                            OP_Ne
                        } else {
                            OP_IsNull
                        };
                        sqlite3VdbeAddOp4(
                            v,
                            op_0,
                            rLhs,
                            destIfFalse,
                            r2,
                            pColl as *mut ::core::ffi::c_void
                                as *const ::core::ffi::c_char,
                            P4_COLLSEQ,
                        );
                        sqlite3VdbeChangeP5(
                            v,
                            (*zAff.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int | SQLITE_JUMPIFNULL) as u16_0,
                        );
                    }
                    ii += 1;
                }
                if regCkNull != 0 {
                    sqlite3VdbeAddOp2(v, OP_IsNull, regCkNull, destIfNull);
                    sqlite3VdbeGoto(v, destIfFalse);
                }
                sqlite3VdbeResolveLabel(v, labelOk);
                sqlite3ReleaseTempReg(pParse, regCkNull);
            } else {
                if eType != IN_INDEX_ROWID {
                    sqlite3VdbeAddOp4(
                        v,
                        OP_Affinity,
                        rLhs,
                        nVector,
                        0 as ::core::ffi::c_int,
                        zAff,
                        nVector,
                    );
                    i = 0 as ::core::ffi::c_int;
                    while i < nVector && *aiMap.offset(i as isize) == i {
                        i += 1;
                    }
                    if i != nVector {
                        let mut rLhsOrig: ::core::ffi::c_int = rLhs;
                        rLhs = sqlite3GetTempRange(pParse, nVector);
                        i = 0 as ::core::ffi::c_int;
                        while i < nVector {
                            sqlite3VdbeAddOp3(
                                v,
                                OP_Copy,
                                rLhsOrig + i,
                                rLhs + *aiMap.offset(i as isize),
                                0 as ::core::ffi::c_int,
                            );
                            i += 1;
                        }
                        sqlite3ReleaseTempReg(pParse, rLhsOrig);
                    }
                }
                if destIfNull == destIfFalse {
                    destStep2 = destIfFalse;
                } else {
                    destStep6 = sqlite3VdbeMakeLabel(pParse);
                    destStep2 = destStep6;
                }
                i = 0 as ::core::ffi::c_int;
                loop {
                    if !(i < nVector) {
                        c2rust_current_block = 7178192492338286402;
                        break;
                    }
                    let mut p: *mut Expr = sqlite3VectorFieldSubexpr((*pExpr).pLeft, i);
                    if (*pParse).nErr != 0 {
                        c2rust_current_block = 346688114740144451;
                        break;
                    }
                    if sqlite3ExprCanBeNull(p) != 0 {
                        sqlite3VdbeAddOp2(v, OP_IsNull, rLhs + i, destStep2);
                    }
                    i += 1;
                }
                match c2rust_current_block {
                    346688114740144451 => {}
                    _ => {
                        if eType == IN_INDEX_ROWID {
                            sqlite3VdbeAddOp3(v, OP_SeekRowid, iTab, destIfFalse, rLhs);
                            addrTruthOp = sqlite3VdbeAddOp0(v, OP_Goto);
                            c2rust_current_block = 9199578309995299736;
                        } else if destIfFalse == destIfNull {
                            if (*pExpr).flags & 0x2000000 as ::core::ffi::c_int as u32_0
                                != 0 as ::core::ffi::c_uint
                            {
                                let mut pOp: *const VdbeOp = sqlite3VdbeGetOp(
                                    v,
                                    (*pExpr).y.sub.iAddr,
                                );
                                if (*pOp).opcode as ::core::ffi::c_int == OP_Once
                                    && (*pOp).p3 > 0 as ::core::ffi::c_int
                                {
                                    sqlite3VdbeAddOp4Int(
                                        v,
                                        OP_Filter,
                                        (*pOp).p3,
                                        destIfFalse,
                                        rLhs,
                                        nVector,
                                    );
                                }
                            }
                            sqlite3VdbeAddOp4Int(
                                v,
                                OP_NotFound,
                                iTab,
                                destIfFalse,
                                rLhs,
                                nVector,
                            );
                            c2rust_current_block = 346688114740144451;
                        } else {
                            addrTruthOp = sqlite3VdbeAddOp4Int(
                                v,
                                OP_Found,
                                iTab,
                                0 as ::core::ffi::c_int,
                                rLhs,
                                nVector,
                            );
                            c2rust_current_block = 9199578309995299736;
                        }
                        match c2rust_current_block {
                            346688114740144451 => {}
                            _ => {
                                if rRhsHasNull != 0 && nVector == 1 as ::core::ffi::c_int {
                                    sqlite3VdbeAddOp2(v, OP_NotNull, rRhsHasNull, destIfFalse);
                                }
                                if destIfFalse == destIfNull {
                                    sqlite3VdbeGoto(v, destIfFalse);
                                }
                                if destStep6 != 0 {
                                    sqlite3VdbeResolveLabel(v, destStep6);
                                }
                                addrTop = sqlite3VdbeAddOp2(
                                    v,
                                    OP_Rewind,
                                    iTab,
                                    destIfFalse,
                                );
                                if nVector > 1 as ::core::ffi::c_int {
                                    destNotNull = sqlite3VdbeMakeLabel(pParse);
                                } else {
                                    destNotNull = destIfFalse;
                                }
                                i = 0 as ::core::ffi::c_int;
                                while i < nVector {
                                    let mut p_0: *mut Expr = ::core::ptr::null_mut::<Expr>();
                                    let mut pColl_0: *mut CollSeq = ::core::ptr::null_mut::<
                                        CollSeq,
                                    >();
                                    let mut r3: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                                    p_0 = sqlite3VectorFieldSubexpr(pLeft, i);
                                    pColl_0 = sqlite3ExprCollSeq(pParse, p_0);
                                    sqlite3VdbeAddOp3(v, OP_Column, iTab, i, r3);
                                    sqlite3VdbeAddOp4(
                                        v,
                                        OP_Ne,
                                        rLhs + i,
                                        destNotNull,
                                        r3,
                                        pColl_0 as *mut ::core::ffi::c_void
                                            as *const ::core::ffi::c_char,
                                        P4_COLLSEQ,
                                    );
                                    sqlite3ReleaseTempReg(pParse, r3);
                                    i += 1;
                                }
                                sqlite3VdbeAddOp2(
                                    v,
                                    OP_Goto,
                                    0 as ::core::ffi::c_int,
                                    destIfNull,
                                );
                                if nVector > 1 as ::core::ffi::c_int {
                                    sqlite3VdbeResolveLabel(v, destNotNull);
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_Next,
                                        iTab,
                                        addrTop + 1 as ::core::ffi::c_int,
                                    );
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_Goto,
                                        0 as ::core::ffi::c_int,
                                        destIfFalse,
                                    );
                                }
                                sqlite3VdbeJumpHere(v, addrTruthOp);
                            }
                        }
                    }
                }
            }
        }
        sqlite3DbFree((*pParse).db, aiMap as *mut ::core::ffi::c_void);
        sqlite3DbFree((*pParse).db, zAff as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn codeReal(
    mut v: *mut Vdbe,
    mut z: *const ::core::ffi::c_char,
    mut negateFlag: ::core::ffi::c_int,
    mut iMem: ::core::ffi::c_int,
) {
    unsafe {
        if !z.is_null() {
            let mut value: ::core::ffi::c_double = 0.;
            sqlite3AtoF(z, &raw mut value, sqlite3Strlen30(z), SQLITE_UTF8 as u8_0);
            if negateFlag != 0 {
                value = -value;
            }
            sqlite3VdbeAddOp4Dup8(
                v,
                OP_Real,
                0 as ::core::ffi::c_int,
                iMem,
                0 as ::core::ffi::c_int,
                &raw mut value as *mut u8_0,
                P4_REAL,
            );
        }
    }
}
unsafe extern "C" fn codeInteger(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut negFlag: ::core::ffi::c_int,
    mut iMem: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        if (*pExpr).flags as ::core::ffi::c_uint & EP_IntValue as ::core::ffi::c_uint
            != 0
        {
            let mut i: ::core::ffi::c_int = (*pExpr).u.iValue;
            if negFlag != 0 {
                i = -i;
            }
            sqlite3VdbeAddOp2(v, OP_Integer, i, iMem);
        } else {
            let mut c: ::core::ffi::c_int = 0;
            let mut value: i64_0 = 0;
            let mut z: *const ::core::ffi::c_char = (*pExpr).u.zToken;
            c = sqlite3DecOrHexToI64(z, &raw mut value);
            if c == 3 as ::core::ffi::c_int && negFlag == 0
                || c == 2 as ::core::ffi::c_int
                || negFlag != 0 && value == SMALLEST_INT64
            {
                if sqlite3_strnicmp(
                    z,
                    b"0x\0".as_ptr() as *const ::core::ffi::c_char,
                    2 as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
                {
                    sqlite3ErrorMsg(
                        pParse,
                        b"hex literal too big: %s%#T\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        if negFlag != 0 {
                            b"-\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        pExpr,
                    );
                } else {
                    codeReal(v, z, negFlag, iMem);
                }
            } else {
                if negFlag != 0 {
                    value = (if c == 3 as ::core::ffi::c_int {
                        SMALLEST_INT64
                    } else {
                        -(value as ::core::ffi::c_longlong)
                    }) as i64_0;
                }
                sqlite3VdbeAddOp4Dup8(
                    v,
                    OP_Int64,
                    0 as ::core::ffi::c_int,
                    iMem,
                    0 as ::core::ffi::c_int,
                    &raw mut value as *mut u8_0,
                    P4_INT64,
                );
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCodeLoadIndexColumn(
    mut pParse: *mut Parse,
    mut pIdx: *mut Index,
    mut iTabCur: ::core::ffi::c_int,
    mut iIdxCol: ::core::ffi::c_int,
    mut regOut: ::core::ffi::c_int,
) {
    unsafe {
        let mut iTabCol: i16_0 = *(*pIdx).aiColumn.offset(iIdxCol as isize);
        if iTabCol as ::core::ffi::c_int == XN_EXPR {
            (*pParse).iSelfTab = iTabCur + 1 as ::core::ffi::c_int;
            sqlite3ExprCodeCopy(
                pParse,
                (*(&raw mut (*(*pIdx).aColExpr).a as *mut ExprList_item)
                    .offset(iIdxCol as isize))
                    .pExpr,
                regOut,
            );
            (*pParse).iSelfTab = 0 as ::core::ffi::c_int;
        } else {
            sqlite3ExprCodeGetColumnOfTable(
                (*pParse).pVdbe,
                (*pIdx).pTable,
                iTabCur,
                iTabCol as ::core::ffi::c_int,
                regOut,
            );
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCodeGeneratedColumn(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pCol: *mut Column,
    mut regOut: ::core::ffi::c_int,
) {
    unsafe {
        let mut iAddr: ::core::ffi::c_int = 0;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut nErr: ::core::ffi::c_int = (*pParse).nErr;
        if (*pParse).iSelfTab > 0 as ::core::ffi::c_int {
            iAddr = sqlite3VdbeAddOp3(
                v,
                OP_IfNullRow,
                (*pParse).iSelfTab - 1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                regOut,
            );
        } else {
            iAddr = 0 as ::core::ffi::c_int;
        }
        sqlite3ExprCodeCopy(pParse, sqlite3ColumnExpr(pTab, pCol), regOut);
        if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_VIRTUAL
            != 0 as ::core::ffi::c_int
            && (*pTab).tabFlags as ::core::ffi::c_uint & TF_Strict as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
        {
            let mut p3: ::core::ffi::c_int = 2 as ::core::ffi::c_int
                + pCol.offset_from((*pTab).aCol) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
            sqlite3VdbeAddOp4(
                v,
                OP_TypeCheck,
                regOut,
                1 as ::core::ffi::c_int,
                p3,
                pTab as *mut ::core::ffi::c_char,
                P4_TABLE,
            );
        } else if (*pCol).affinity as ::core::ffi::c_int >= SQLITE_AFF_TEXT {
            sqlite3VdbeAddOp4(
                v,
                OP_Affinity,
                regOut,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                &raw mut (*pCol).affinity,
                1 as ::core::ffi::c_int,
            );
        }
        if iAddr != 0 {
            sqlite3VdbeJumpHere(v, iAddr);
        }
        if (*pParse).nErr > nErr {
            (*(*pParse).db).errByteOffset = -1 as ::core::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCodeGetColumnOfTable(
    mut v: *mut Vdbe,
    mut pTab: *mut Table,
    mut iTabCur: ::core::ffi::c_int,
    mut iCol: ::core::ffi::c_int,
    mut regOut: ::core::ffi::c_int,
) {
    unsafe {
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        if iCol < 0 as ::core::ffi::c_int || iCol == (*pTab).iPKey as ::core::ffi::c_int
        {
            sqlite3VdbeAddOp2(v, OP_Rowid, iTabCur, regOut);
        } else {
            let mut op: ::core::ffi::c_int = 0;
            let mut x: ::core::ffi::c_int = 0;
            if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                op = OP_VColumn;
                x = iCol;
            } else {
                pCol = (*pTab).aCol.offset(iCol as isize) as *mut Column;
                if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_VIRTUAL != 0 {
                    let mut pParse: *mut Parse = sqlite3VdbeParser(v);
                    if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_BUSY != 0 {
                        sqlite3ErrorMsg(
                            pParse,
                            b"generated column loop on \"%s\"\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*pCol).zCnName,
                        );
                    } else {
                        let mut savedSelfTab: ::core::ffi::c_int = (*pParse).iSelfTab;
                        (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                            | COLFLAG_BUSY) as u16_0;
                        (*pParse).iSelfTab = iTabCur + 1 as ::core::ffi::c_int;
                        sqlite3ExprCodeGeneratedColumn(pParse, pTab, pCol, regOut);
                        (*pParse).iSelfTab = savedSelfTab;
                        (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                            & !COLFLAG_BUSY) as u16_0;
                    }
                    return;
                } else if !((*pTab).tabFlags as ::core::ffi::c_uint
                    & TF_WithoutRowid as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint)
                {
                    x = sqlite3TableColumnToIndex(sqlite3PrimaryKeyIndex(pTab), iCol);
                    op = OP_Column;
                } else {
                    x = sqlite3TableColumnToStorage(pTab, iCol as i16_0)
                        as ::core::ffi::c_int;
                    op = OP_Column;
                }
            }
            sqlite3VdbeAddOp3(v, op, iTabCur, x, regOut);
            sqlite3ColumnDefault(v, pTab, iCol, regOut);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCodeGetColumn(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut iColumn: ::core::ffi::c_int,
    mut iTable: ::core::ffi::c_int,
    mut iReg: ::core::ffi::c_int,
    mut p5: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3ExprCodeGetColumnOfTable((*pParse).pVdbe, pTab, iTable, iColumn, iReg);
        if p5 != 0 {
            let mut pOp: *mut VdbeOp = sqlite3VdbeGetLastOp((*pParse).pVdbe);
            if (*pOp).opcode as ::core::ffi::c_int == OP_Column {
                (*pOp).p5 = p5 as u16_0;
            }
            if (*pOp).opcode as ::core::ffi::c_int == OP_VColumn {
                (*pOp).p5 = (p5 as ::core::ffi::c_int & OPFLAG_NOCHNG) as u16_0;
            }
        }
        return iReg;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCodeMove(
    mut pParse: *mut Parse,
    mut iFrom: ::core::ffi::c_int,
    mut iTo: ::core::ffi::c_int,
    mut nReg: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3VdbeAddOp3((*pParse).pVdbe, OP_Move, iFrom, iTo, nReg);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprToRegister(
    mut pExpr: *mut Expr,
    mut iReg: ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *mut Expr = sqlite3ExprSkipCollateAndLikely(pExpr);
        if p.is_null() {
            return;
        }
        if !((*p).op as ::core::ffi::c_int == TK_REGISTER) {
            (*p).op2 = (*p).op;
            (*p).op = TK_REGISTER as u8_0;
            (*p).iTable = iReg;
            (*p).flags
                &= !(0x2000 as ::core::ffi::c_int as u32_0) as ::core::ffi::c_uint;
        }
    }
}
unsafe extern "C" fn exprCodeVector(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
    mut piFreeable: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iResult: ::core::ffi::c_int = 0;
        let mut nResult: ::core::ffi::c_int = sqlite3ExprVectorSize(p);
        if nResult == 1 as ::core::ffi::c_int {
            iResult = sqlite3ExprCodeTemp(pParse, p, piFreeable);
        } else {
            *piFreeable = 0 as ::core::ffi::c_int;
            if (*p).op as ::core::ffi::c_int == TK_SELECT {
                iResult = sqlite3CodeSubselect(pParse, p);
            } else {
                let mut i: ::core::ffi::c_int = 0;
                iResult = (*pParse).nMem + 1 as ::core::ffi::c_int;
                (*pParse).nMem += nResult;
                i = 0 as ::core::ffi::c_int;
                while i < nResult {
                    sqlite3ExprCodeFactorable(
                        pParse,
                        (*(&raw mut (*(*p).x.pList).a as *mut ExprList_item)
                            .offset(i as isize))
                            .pExpr,
                        i + iResult,
                    );
                    i += 1;
                }
            }
        }
        return iResult;
    }
}
unsafe extern "C" fn setDoNotMergeFlagOnCopy(mut v: *mut Vdbe) {
    unsafe {
        if (*sqlite3VdbeGetLastOp(v)).opcode as ::core::ffi::c_int == OP_Copy {
            sqlite3VdbeChangeP5(v, 1 as u16_0);
        }
    }
}
unsafe extern "C" fn exprCodeInlineFunction(
    mut pParse: *mut Parse,
    mut pFarg: *mut ExprList,
    mut iFuncId: ::core::ffi::c_int,
    mut target: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nFarg: ::core::ffi::c_int = 0;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        nFarg = (*pFarg).nExpr;
        match iFuncId {
            INLINEFUNC_coalesce => {
                let mut endCoalesce: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
                let mut i: ::core::ffi::c_int = 0;
                sqlite3ExprCode(
                    pParse,
                    (*(&raw mut (*pFarg).a as *mut ExprList_item)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .pExpr,
                    target,
                );
                i = 1 as ::core::ffi::c_int;
                while i < nFarg {
                    sqlite3VdbeAddOp2(v, OP_NotNull, target, endCoalesce);
                    sqlite3ExprCode(
                        pParse,
                        (*(&raw mut (*pFarg).a as *mut ExprList_item).offset(i as isize))
                            .pExpr,
                        target,
                    );
                    i += 1;
                }
                setDoNotMergeFlagOnCopy(v);
                sqlite3VdbeResolveLabel(v, endCoalesce);
            }
            INLINEFUNC_iif => {
                let mut caseExpr: Expr = Expr {
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
                memset(
                    &raw mut caseExpr as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<Expr>() as size_t,
                );
                caseExpr.op = TK_CASE as u8_0;
                caseExpr.x.pList = pFarg;
                return sqlite3ExprCodeTarget(pParse, &raw mut caseExpr, target);
            }
            INLINEFUNC_expr_compare => {
                sqlite3VdbeAddOp2(
                    v,
                    OP_Integer,
                    sqlite3ExprCompare(
                        ::core::ptr::null::<Parse>(),
                        (*(&raw mut (*pFarg).a as *mut ExprList_item)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .pExpr,
                        (*(&raw mut (*pFarg).a as *mut ExprList_item)
                            .offset(1 as ::core::ffi::c_int as isize))
                            .pExpr,
                        -1 as ::core::ffi::c_int,
                    ),
                    target,
                );
            }
            INLINEFUNC_expr_implies_expr => {
                sqlite3VdbeAddOp2(
                    v,
                    OP_Integer,
                    sqlite3ExprImpliesExpr(
                        pParse,
                        (*(&raw mut (*pFarg).a as *mut ExprList_item)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .pExpr,
                        (*(&raw mut (*pFarg).a as *mut ExprList_item)
                            .offset(1 as ::core::ffi::c_int as isize))
                            .pExpr,
                        -1 as ::core::ffi::c_int,
                    ),
                    target,
                );
            }
            INLINEFUNC_implies_nonnull_row => {
                let mut pA1: *mut Expr = ::core::ptr::null_mut::<Expr>();
                pA1 = (*(&raw mut (*pFarg).a as *mut ExprList_item)
                    .offset(1 as ::core::ffi::c_int as isize))
                    .pExpr;
                if (*pA1).op as ::core::ffi::c_int == TK_COLUMN {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Integer,
                        sqlite3ExprImpliesNonNullRow(
                            (*(&raw mut (*pFarg).a as *mut ExprList_item)
                                .offset(0 as ::core::ffi::c_int as isize))
                                .pExpr,
                            (*pA1).iTable,
                            1 as ::core::ffi::c_int,
                        ),
                        target,
                    );
                } else {
                    sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, target);
                }
            }
            INLINEFUNC_affinity => {
                let mut azAff: [*const ::core::ffi::c_char; 6] = [
                    b"blob\0".as_ptr() as *const ::core::ffi::c_char,
                    b"text\0".as_ptr() as *const ::core::ffi::c_char,
                    b"numeric\0".as_ptr() as *const ::core::ffi::c_char,
                    b"integer\0".as_ptr() as *const ::core::ffi::c_char,
                    b"real\0".as_ptr() as *const ::core::ffi::c_char,
                    b"flexnum\0".as_ptr() as *const ::core::ffi::c_char,
                ];
                let mut aff: ::core::ffi::c_char = 0;
                aff = sqlite3ExprAffinity(
                    (*(&raw mut (*pFarg).a as *mut ExprList_item)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .pExpr,
                );
                sqlite3VdbeLoadString(
                    v,
                    target,
                    if aff as ::core::ffi::c_int <= SQLITE_AFF_NONE {
                        b"none\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        azAff[(aff as ::core::ffi::c_int - SQLITE_AFF_BLOB) as usize]
                    },
                );
            }
            _ => {
                target = sqlite3ExprCodeTarget(
                    pParse,
                    (*(&raw mut (*pFarg).a as *mut ExprList_item)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .pExpr,
                    target,
                );
            }
        }
        return target;
    }
}
unsafe extern "C" fn exprNodeCanReturnSubtype(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut pDef: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if (*pExpr).op as ::core::ffi::c_int != TK_FUNCTION {
            return WRC_Prune;
        }
        db = (*(*pWalker).pParse).db;
        n = if !(*pExpr).x.pList.is_null() {
            (*(*pExpr).x.pList).nExpr
        } else {
            0 as ::core::ffi::c_int
        };
        pDef = sqlite3FindFunction(db, (*pExpr).u.zToken, n, (*db).enc, 0 as u8_0);
        if pDef.is_null()
            || (*pDef).funcFlags as ::core::ffi::c_uint
                & SQLITE_RESULT_SUBTYPE as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
        {
            (*pWalker).eCode = 1 as u16_0;
            return WRC_Prune;
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn sqlite3ExprCanReturnSubtype(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
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
        w.pParse = pParse;
        w.xExprCallback = Some(
            exprNodeCanReturnSubtype
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        sqlite3WalkExpr(&raw mut w, pExpr);
        return w.eCode as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn sqlite3IndexedExprLookup(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut target: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut IndexedExpr = ::core::ptr::null_mut::<IndexedExpr>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut c2rust_current_block_21: u64;
        p = (*pParse).pIdxEpr;
        while !p.is_null() {
            let mut exprAff: u8_0 = 0;
            let mut iDataCur: ::core::ffi::c_int = (*p).iDataCur;
            if !(iDataCur < 0 as ::core::ffi::c_int) {
                if (*pParse).iSelfTab != 0 {
                    if (*p).iDataCur != (*pParse).iSelfTab - 1 as ::core::ffi::c_int {
                        c2rust_current_block_21 = 16668937799742929182;
                    } else {
                        iDataCur = -1 as ::core::ffi::c_int;
                        c2rust_current_block_21 = 11006700562992250127;
                    }
                } else {
                    c2rust_current_block_21 = 11006700562992250127;
                }
                match c2rust_current_block_21 {
                    16668937799742929182 => {}
                    _ => {
                        if !(sqlite3ExprCompare(
                            ::core::ptr::null::<Parse>(),
                            pExpr,
                            (*p).pExpr,
                            iDataCur,
                        ) != 0 as ::core::ffi::c_int)
                        {
                            exprAff = sqlite3ExprAffinity(pExpr) as u8_0;
                            if !(exprAff as ::core::ffi::c_int <= SQLITE_AFF_BLOB
                                && (*p).aff as ::core::ffi::c_int != SQLITE_AFF_BLOB
                                || exprAff as ::core::ffi::c_int == SQLITE_AFF_TEXT
                                    && (*p).aff as ::core::ffi::c_int != SQLITE_AFF_TEXT
                                || exprAff as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC
                                    && (*p).aff as ::core::ffi::c_int != SQLITE_AFF_NUMERIC)
                            {
                                if !((*pExpr).flags & 0x80000000 as ::core::ffi::c_uint
                                    != 0 as ::core::ffi::c_uint
                                    && sqlite3ExprCanReturnSubtype(pParse, pExpr) != 0)
                                {
                                    v = (*pParse).pVdbe;
                                    if (*p).bMaybeNullRow != 0 {
                                        let mut addr: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(
                                            v,
                                        );
                                        sqlite3VdbeAddOp3(
                                            v,
                                            OP_IfNullRow,
                                            (*p).iIdxCur,
                                            addr + 3 as ::core::ffi::c_int,
                                            target,
                                        );
                                        sqlite3VdbeAddOp3(
                                            v,
                                            OP_Column,
                                            (*p).iIdxCur,
                                            (*p).iIdxCol,
                                            target,
                                        );
                                        sqlite3VdbeGoto(v, 0 as ::core::ffi::c_int);
                                        p = (*pParse).pIdxEpr;
                                        (*pParse).pIdxEpr = ::core::ptr::null_mut::<IndexedExpr>();
                                        sqlite3ExprCode(pParse, pExpr, target);
                                        (*pParse).pIdxEpr = p;
                                        sqlite3VdbeJumpHere(v, addr + 2 as ::core::ffi::c_int);
                                    } else {
                                        sqlite3VdbeAddOp3(
                                            v,
                                            OP_Column,
                                            (*p).iIdxCur,
                                            (*p).iIdxCol,
                                            target,
                                        );
                                    }
                                    return target;
                                }
                            }
                        }
                    }
                }
            }
            p = (*p).pIENext;
        }
        return -1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn exprPartidxExprLookup(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut iTarget: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut IndexedExpr = ::core::ptr::null_mut::<IndexedExpr>();
        p = (*pParse).pIdxPartExpr;
        while !p.is_null() {
            if (*pExpr).iColumn as ::core::ffi::c_int == (*p).iIdxCol
                && (*pExpr).iTable == (*p).iDataCur
            {
                let mut v: *mut Vdbe = (*pParse).pVdbe;
                let mut addr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut ret: ::core::ffi::c_int = 0;
                if (*p).bMaybeNullRow != 0 {
                    addr = sqlite3VdbeAddOp1(v, OP_IfNullRow, (*p).iIdxCur);
                }
                ret = sqlite3ExprCodeTarget(pParse, (*p).pExpr, iTarget);
                sqlite3VdbeAddOp4(
                    (*pParse).pVdbe,
                    OP_Affinity,
                    ret,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    &raw mut (*p).aff as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                );
                if addr != 0 {
                    sqlite3VdbeJumpHere(v, addr);
                    sqlite3VdbeChangeP3(v, addr, ret);
                }
                return ret;
            }
            p = (*p).pIENext;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn exprCodeTargetAndOr(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut target: ::core::ffi::c_int,
    mut pTmpReg: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut op: ::core::ffi::c_int = 0;
        let mut skipOp: ::core::ffi::c_int = 0;
        let mut addrSkip: ::core::ffi::c_int = 0;
        let mut regSS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut r1: ::core::ffi::c_int = 0;
        let mut r2: ::core::ffi::c_int = 0;
        let mut pAlt: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        op = (*pExpr).op as ::core::ffi::c_int;
        v = (*pParse).pVdbe;
        pAlt = sqlite3ExprSimplifiedAndOr(pExpr);
        if pAlt != pExpr {
            r1 = sqlite3ExprCodeTarget(pParse, pAlt, target);
            sqlite3VdbeAddOp3(v, OP_And, r1, r1, target);
            return target;
        }
        skipOp = if op == TK_AND { OP_IfNot } else { OP_If };
        if exprEvalRhsFirst(pExpr) != 0 {
            regSS = sqlite3ExprCodeTarget(pParse, (*pExpr).pRight, target);
            r2 = regSS;
            addrSkip = sqlite3VdbeAddOp1(v, skipOp, r2);
            r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, pTmpReg);
        } else {
            r1 = sqlite3ExprCodeTarget(pParse, (*pExpr).pLeft, target);
            if (*(*pExpr).pRight).flags & 0x400000 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
            {
                regSS = r1;
                addrSkip = sqlite3VdbeAddOp1(v, skipOp, r1);
            } else {
                regSS = 0 as ::core::ffi::c_int;
                addrSkip = regSS;
            }
            r2 = sqlite3ExprCodeTemp(pParse, (*pExpr).pRight, pTmpReg);
        }
        sqlite3VdbeAddOp3(v, op, r2, r1, target);
        if addrSkip != 0 {
            sqlite3VdbeAddOp2(
                v,
                OP_Goto,
                0 as ::core::ffi::c_int,
                sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
            );
            sqlite3VdbeJumpHere(v, addrSkip);
            sqlite3VdbeAddOp3(v, OP_Or, regSS, regSS, target);
        }
        return target;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCodeTarget(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut target: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFarg: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut nFarg: ::core::ffi::c_int = 0;
        let mut pDef: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
        let mut zId: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut constMask: u32_0 = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut enc: u8_0 = 0;
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        let mut addrINR: ::core::ffi::c_int = 0;
        let mut okConstFactor: u8_0 = 0;
        let mut pAggInfo_0: *mut AggInfo = ::core::ptr::null_mut::<AggInfo>();
        let mut endLabel: ::core::ffi::c_int = 0;
        let mut nextCase: ::core::ffi::c_int = 0;
        let mut nExpr: ::core::ffi::c_int = 0;
        let mut i_0: ::core::ffi::c_int = 0;
        let mut pEList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        let mut aListelem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        let mut opCompare: Expr = Expr {
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
        let mut pX: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pTest: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut pDel: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut db_0: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pAggInfo: *mut AggInfo = ::core::ptr::null_mut::<AggInfo>();
        let mut pCol: *mut AggInfo_col = ::core::ptr::null_mut::<AggInfo_col>();
        let mut c2rust_current_block: u64;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut op: ::core::ffi::c_int = 0;
        let mut inReg: ::core::ffi::c_int = target;
        let mut regFree1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regFree2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut r1: ::core::ffi::c_int = 0;
        let mut r2: ::core::ffi::c_int = 0;
        let mut tempX: Expr = Expr {
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
        let mut p5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            if pExpr.is_null() {
                op = TK_NULL;
            } else if !(*pParse).pIdxEpr.is_null()
                && !((*pExpr).flags & 0x800000 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint)
                && {
                    r1 = sqlite3IndexedExprLookup(pParse, pExpr, target);
                    r1 >= 0 as ::core::ffi::c_int
                }
            {
                return r1
            } else {
                op = (*pExpr).op as ::core::ffi::c_int;
            }
            match op {
                TK_AGG_COLUMN => {
                    pAggInfo = (*pExpr).pAggInfo;
                    pCol = ::core::ptr::null_mut::<AggInfo_col>();
                    if (*pExpr).iAgg as ::core::ffi::c_int >= (*pAggInfo).nColumn {
                        c2rust_current_block = 9606288038608642794;
                        break;
                    } else {
                        c2rust_current_block = 5143058163439228106;
                        break;
                    }
                }
                TK_COLUMN => {
                    c2rust_current_block = 7890507820729860149;
                    break;
                }
                TK_INTEGER => {
                    codeInteger(pParse, pExpr, 0 as ::core::ffi::c_int, target);
                    return target;
                }
                TK_TRUEFALSE => {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Integer,
                        sqlite3ExprTruthValue(pExpr),
                        target,
                    );
                    return target;
                }
                TK_FLOAT => {
                    codeReal(v, (*pExpr).u.zToken, 0 as ::core::ffi::c_int, target);
                    return target;
                }
                TK_STRING => {
                    sqlite3VdbeLoadString(v, target, (*pExpr).u.zToken);
                    return target;
                }
                TK_NULLS => {
                    sqlite3VdbeAddOp3(
                        v,
                        OP_Null,
                        0 as ::core::ffi::c_int,
                        target,
                        target + (*pExpr).y.nReg - 1 as ::core::ffi::c_int,
                    );
                    return target;
                }
                TK_BLOB => {
                    let mut n: ::core::ffi::c_int = 0;
                    let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
                        ::core::ffi::c_char,
                    >();
                    let mut zBlob: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    z = (*pExpr).u.zToken.offset(2 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char;
                    n = sqlite3Strlen30(z) - 1 as ::core::ffi::c_int;
                    zBlob = sqlite3HexToBlob(sqlite3VdbeDb(v), z, n)
                        as *mut ::core::ffi::c_char;
                    sqlite3VdbeAddOp4(
                        v,
                        OP_Blob,
                        n / 2 as ::core::ffi::c_int,
                        target,
                        0 as ::core::ffi::c_int,
                        zBlob,
                        P4_DYNAMIC,
                    );
                    return target;
                }
                TK_VARIABLE => {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Variable,
                        (*pExpr).iColumn as ::core::ffi::c_int,
                        target,
                    );
                    return target;
                }
                TK_REGISTER => return (*pExpr).iTable,
                TK_CAST => {
                    sqlite3ExprCode(pParse, (*pExpr).pLeft, target);
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Cast,
                        target,
                        sqlite3AffinityType(
                            (*pExpr).u.zToken,
                            ::core::ptr::null_mut::<Column>(),
                        ) as ::core::ffi::c_int,
                    );
                    return inReg;
                }
                TK_IS | TK_ISNOT => {
                    op = if op == TK_IS { TK_EQ } else { TK_NE };
                    p5 = SQLITE_NULLEQ;
                    c2rust_current_block = 5409161009579131794;
                    break;
                }
                TK_LT | TK_LE | TK_GT | TK_GE | TK_NE | TK_EQ => {
                    c2rust_current_block = 5409161009579131794;
                    break;
                }
                TK_AND | TK_OR => {
                    inReg = exprCodeTargetAndOr(
                        pParse,
                        pExpr,
                        target,
                        &raw mut regFree1,
                    );
                    c2rust_current_block = 8004310806444026423;
                    break;
                }
                TK_PLUS
                | TK_STAR
                | TK_MINUS
                | TK_REM
                | TK_BITAND
                | TK_BITOR
                | TK_SLASH
                | TK_LSHIFT
                | TK_RSHIFT
                | TK_CONCAT => {
                    let mut addrIsNull_0: ::core::ffi::c_int = 0;
                    if (*pExpr).flags & 0x400000 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint
                    {
                        addrIsNull_0 = exprComputeOperands(
                            pParse,
                            pExpr,
                            &raw mut r1,
                            &raw mut r2,
                            &raw mut regFree1,
                            &raw mut regFree2,
                        );
                    } else {
                        r1 = sqlite3ExprCodeTemp(
                            pParse,
                            (*pExpr).pLeft,
                            &raw mut regFree1,
                        );
                        r2 = sqlite3ExprCodeTemp(
                            pParse,
                            (*pExpr).pRight,
                            &raw mut regFree2,
                        );
                        addrIsNull_0 = 0 as ::core::ffi::c_int;
                    }
                    sqlite3VdbeAddOp3(v, op, r2, r1, target);
                    if addrIsNull_0 != 0 {
                        sqlite3VdbeAddOp2(
                            v,
                            OP_Goto,
                            0 as ::core::ffi::c_int,
                            sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
                        );
                        sqlite3VdbeJumpHere(v, addrIsNull_0);
                        sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, target);
                    }
                    c2rust_current_block = 8004310806444026423;
                    break;
                }
                TK_UMINUS => {
                    let mut pLeft_0: *mut Expr = (*pExpr).pLeft;
                    if (*pLeft_0).op as ::core::ffi::c_int == TK_INTEGER {
                        codeInteger(pParse, pLeft_0, 1 as ::core::ffi::c_int, target);
                        return target;
                    } else if (*pLeft_0).op as ::core::ffi::c_int == TK_FLOAT {
                        codeReal(
                            v,
                            (*pLeft_0).u.zToken,
                            1 as ::core::ffi::c_int,
                            target,
                        );
                        return target;
                    } else {
                        tempX.op = TK_INTEGER as u8_0;
                        tempX.flags = (EP_IntValue | EP_TokenOnly) as u32_0;
                        tempX.u.iValue = 0 as ::core::ffi::c_int;
                        r1 = sqlite3ExprCodeTemp(
                            pParse,
                            &raw mut tempX,
                            &raw mut regFree1,
                        );
                        r2 = sqlite3ExprCodeTemp(
                            pParse,
                            (*pExpr).pLeft,
                            &raw mut regFree2,
                        );
                        sqlite3VdbeAddOp3(v, OP_Subtract, r2, r1, target);
                    }
                    c2rust_current_block = 8004310806444026423;
                    break;
                }
                TK_BITNOT | TK_NOT => {
                    r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, &raw mut regFree1);
                    sqlite3VdbeAddOp2(v, op, r1, inReg);
                    c2rust_current_block = 8004310806444026423;
                    break;
                }
                TK_TRUTH => {
                    let mut isTrue: ::core::ffi::c_int = 0;
                    let mut bNormal: ::core::ffi::c_int = 0;
                    r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, &raw mut regFree1);
                    isTrue = sqlite3ExprTruthValue((*pExpr).pRight);
                    bNormal = ((*pExpr).op2 as ::core::ffi::c_int == TK_IS)
                        as ::core::ffi::c_int;
                    sqlite3VdbeAddOp4Int(
                        v,
                        OP_IsTrue,
                        r1,
                        inReg,
                        (isTrue == 0) as ::core::ffi::c_int,
                        isTrue ^ bNormal,
                    );
                    c2rust_current_block = 8004310806444026423;
                    break;
                }
                TK_ISNULL | TK_NOTNULL => {
                    let mut addr: ::core::ffi::c_int = 0;
                    sqlite3VdbeAddOp2(v, OP_Integer, 1 as ::core::ffi::c_int, target);
                    r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, &raw mut regFree1);
                    addr = sqlite3VdbeAddOp1(v, op, r1);
                    sqlite3VdbeAddOp2(v, OP_Integer, 0 as ::core::ffi::c_int, target);
                    sqlite3VdbeJumpHere(v, addr);
                    c2rust_current_block = 8004310806444026423;
                    break;
                }
                TK_AGG_FUNCTION => {
                    let mut pInfo: *mut AggInfo = (*pExpr).pAggInfo;
                    if pInfo.is_null()
                        || ((*pExpr).iAgg as ::core::ffi::c_int)
                            < 0 as ::core::ffi::c_int
                        || (*pExpr).iAgg as ::core::ffi::c_int >= (*pInfo).nFunc
                    {
                        sqlite3ErrorMsg(
                            pParse,
                            b"misuse of aggregate: %#T()\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            pExpr,
                        );
                    } else {
                        return (*pInfo).iFirstReg + (*pInfo).nColumn
                            + (*pExpr).iAgg as ::core::ffi::c_int
                    }
                    c2rust_current_block = 8004310806444026423;
                    break;
                }
                TK_FUNCTION => {
                    pFarg = ::core::ptr::null_mut::<ExprList>();
                    nFarg = 0;
                    pDef = ::core::ptr::null_mut::<FuncDef>();
                    zId = ::core::ptr::null::<::core::ffi::c_char>();
                    constMask = 0 as u32_0;
                    i = 0;
                    db = (*pParse).db;
                    enc = (*db).enc;
                    pColl = ::core::ptr::null_mut::<CollSeq>();
                    if (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint
                    {
                        return (*(*pExpr).y.pWin).regResult;
                    }
                    if (*pParse).okConstFactor() as ::core::ffi::c_int != 0
                        && sqlite3ExprIsConstantNotJoin(pParse, pExpr) != 0
                    {
                        return sqlite3ExprCodeRunJustOnce(
                            pParse,
                            pExpr,
                            -1 as ::core::ffi::c_int,
                        );
                    }
                    pFarg = (*pExpr).x.pList;
                    nFarg = if !pFarg.is_null() {
                        (*pFarg).nExpr
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    zId = (*pExpr).u.zToken;
                    pDef = sqlite3FindFunction(db, zId, nFarg, enc, 0 as u8_0);
                    if pDef.is_null() || (*pDef).xFinalize.is_some() {
                        c2rust_current_block = 5577234879133443219;
                        break;
                    } else {
                        c2rust_current_block = 1995330570110937187;
                        break;
                    }
                }
                TK_EXISTS | TK_SELECT => {
                    let mut nCol: ::core::ffi::c_int = 0;
                    if (*(*pParse).db).mallocFailed != 0 {
                        return 0 as ::core::ffi::c_int
                    } else if op == TK_SELECT
                        && (*pExpr).flags as ::core::ffi::c_uint
                            & 0x1000 as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                        && {
                            nCol = (*(*(*pExpr).x.pSelect).pEList).nExpr;
                            nCol != 1 as ::core::ffi::c_int
                        }
                    {
                        sqlite3SubselectError(pParse, nCol, 1 as ::core::ffi::c_int);
                    } else {
                        return sqlite3CodeSubselect(pParse, pExpr)
                    }
                    c2rust_current_block = 8004310806444026423;
                    break;
                }
                TK_SELECT_COLUMN => {
                    let mut n_0: ::core::ffi::c_int = 0;
                    let mut pLeft_1: *mut Expr = (*pExpr).pLeft;
                    if (*pLeft_1).iTable == 0 as ::core::ffi::c_int
                        || (*pParse).withinRJSubrtn as ::core::ffi::c_int
                            > (*pLeft_1).op2 as ::core::ffi::c_int
                    {
                        (*pLeft_1).iTable = sqlite3CodeSubselect(pParse, pLeft_1);
                        (*pLeft_1).op2 = (*pParse).withinRJSubrtn;
                    }
                    n_0 = sqlite3ExprVectorSize(pLeft_1);
                    if (*pExpr).iTable != n_0 {
                        sqlite3ErrorMsg(
                            pParse,
                            b"%d columns assigned %d values\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*pExpr).iTable,
                            n_0,
                        );
                    }
                    return (*pLeft_1).iTable + (*pExpr).iColumn as ::core::ffi::c_int;
                }
                TK_IN => {
                    let mut destIfFalse: ::core::ffi::c_int = sqlite3VdbeMakeLabel(
                        pParse,
                    );
                    let mut destIfNull: ::core::ffi::c_int = sqlite3VdbeMakeLabel(
                        pParse,
                    );
                    sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, target);
                    sqlite3ExprCodeIN(pParse, pExpr, destIfFalse, destIfNull);
                    sqlite3VdbeAddOp2(v, OP_Integer, 1 as ::core::ffi::c_int, target);
                    sqlite3VdbeResolveLabel(v, destIfFalse);
                    sqlite3VdbeAddOp2(v, OP_AddImm, target, 0 as ::core::ffi::c_int);
                    sqlite3VdbeResolveLabel(v, destIfNull);
                    return target;
                }
                TK_BETWEEN => {
                    exprCodeBetween(
                        pParse,
                        pExpr,
                        target,
                        None,
                        0 as ::core::ffi::c_int,
                    );
                    return target;
                }
                TK_COLLATE => {
                    if !((*pExpr).flags & 0x200 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint)
                    {
                        sqlite3ExprCode(pParse, (*pExpr).pLeft, target);
                        sqlite3VdbeAddOp1(v, OP_ClrSubtype, target);
                        return target;
                    } else {
                        pExpr = (*pExpr).pLeft;
                    }
                }
                TK_SPAN | TK_UPLUS => {
                    pExpr = (*pExpr).pLeft;
                }
                TK_TRIGGER => {
                    let mut pTab_1: *mut Table = ::core::ptr::null_mut::<Table>();
                    let mut iCol_0: ::core::ffi::c_int = 0;
                    let mut p1: ::core::ffi::c_int = 0;
                    pTab_1 = (*pExpr).y.pTab;
                    iCol_0 = (*pExpr).iColumn as ::core::ffi::c_int;
                    p1 = (*pExpr).iTable
                        * ((*pTab_1).nCol as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                        + sqlite3TableColumnToStorage(pTab_1, iCol_0 as i16_0)
                            as ::core::ffi::c_int;
                    sqlite3VdbeAddOp2(v, OP_Param, p1, target);
                    if iCol_0 >= 0 as ::core::ffi::c_int
                        && (*(*pTab_1).aCol.offset(iCol_0 as isize)).affinity
                            as ::core::ffi::c_int == SQLITE_AFF_REAL
                    {
                        sqlite3VdbeAddOp1(v, OP_RealAffinity, target);
                    }
                    c2rust_current_block = 8004310806444026423;
                    break;
                }
                TK_VECTOR => {
                    sqlite3ErrorMsg(
                        pParse,
                        b"row value misused\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    c2rust_current_block = 8004310806444026423;
                    break;
                }
                TK_IF_NULL_ROW => {
                    addrINR = 0;
                    okConstFactor = (*pParse).okConstFactor() as u8_0;
                    pAggInfo_0 = (*pExpr).pAggInfo;
                    if !pAggInfo_0.is_null() {
                        c2rust_current_block = 1003765866876145076;
                        break;
                    } else {
                        c2rust_current_block = 3842749721932025293;
                        break;
                    }
                }
                TK_CASE => {
                    endLabel = 0;
                    nextCase = 0;
                    nExpr = 0;
                    i_0 = 0;
                    pEList = ::core::ptr::null_mut::<ExprList>();
                    aListelem = ::core::ptr::null_mut::<ExprList_item>();
                    opCompare = Expr {
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
                    pX = ::core::ptr::null_mut::<Expr>();
                    pTest = ::core::ptr::null_mut::<Expr>();
                    pDel = ::core::ptr::null_mut::<Expr>();
                    db_0 = (*pParse).db;
                    pEList = (*pExpr).x.pList;
                    aListelem = &raw mut (*pEList).a as *mut ExprList_item
                        as *mut ExprList_item;
                    nExpr = (*pEList).nExpr;
                    endLabel = sqlite3VdbeMakeLabel(pParse);
                    pX = (*pExpr).pLeft;
                    if !pX.is_null() {
                        c2rust_current_block = 16280550096250383041;
                        break;
                    } else {
                        c2rust_current_block = 4841263011268485695;
                        break;
                    }
                }
                TK_RAISE => {
                    if (*pParse).pTriggerTab.is_null() && (*pParse).nested == 0 {
                        sqlite3ErrorMsg(
                            pParse,
                            b"RAISE() may only be used within a trigger-program\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        return 0 as ::core::ffi::c_int;
                    }
                    if (*pExpr).affExpr as ::core::ffi::c_int == OE_Abort {
                        sqlite3MayAbort(pParse);
                    }
                    if (*pExpr).affExpr as ::core::ffi::c_int == OE_Ignore {
                        sqlite3VdbeAddOp2(v, OP_Halt, SQLITE_OK, OE_Ignore);
                    } else {
                        r1 = sqlite3ExprCodeTemp(
                            pParse,
                            (*pExpr).pLeft,
                            &raw mut regFree1,
                        );
                        sqlite3VdbeAddOp3(
                            v,
                            OP_Halt,
                            if !(*pParse).pTriggerTab.is_null() {
                                SQLITE_CONSTRAINT_TRIGGER
                            } else {
                                SQLITE_ERROR
                            },
                            (*pExpr).affExpr as ::core::ffi::c_int,
                            r1,
                        );
                    }
                    c2rust_current_block = 8004310806444026423;
                    break;
                }
                _ => {
                    sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, target);
                    return target;
                }
            }
        }
        match c2rust_current_block {
            1995330570110937187 => {
                if (*pDef).funcFlags as ::core::ffi::c_uint
                    & SQLITE_FUNC_INLINE as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint && !pFarg.is_null()
                {
                    return exprCodeInlineFunction(
                        pParse,
                        pFarg,
                        (*pDef).pUserData as ::core::ffi::c_long as ::core::ffi::c_int,
                        target,
                    )
                } else if (*pDef).funcFlags as ::core::ffi::c_uint
                    & (SQLITE_FUNC_DIRECT | SQLITE_FUNC_UNSAFE) as ::core::ffi::c_uint
                    != 0
                {
                    sqlite3ExprFunctionUsable(pParse, pExpr, pDef);
                }
                i = 0 as ::core::ffi::c_int;
                while i < nFarg {
                    if i < 32 as ::core::ffi::c_int
                        && sqlite3ExprIsConstant(
                            pParse,
                            (*(&raw mut (*pFarg).a as *mut ExprList_item)
                                .offset(i as isize))
                                .pExpr,
                        ) != 0
                    {
                        constMask
                            |= (1 as ::core::ffi::c_int as ::core::ffi::c_uint) << i;
                    }
                    if (*pDef).funcFlags as ::core::ffi::c_uint
                        & SQLITE_FUNC_NEEDCOLL as ::core::ffi::c_uint
                        != 0 as ::core::ffi::c_uint && pColl.is_null()
                    {
                        pColl = sqlite3ExprCollSeq(
                            pParse,
                            (*(&raw mut (*pFarg).a as *mut ExprList_item)
                                .offset(i as isize))
                                .pExpr,
                        );
                    }
                    i += 1;
                }
                if !pFarg.is_null() {
                    if constMask != 0 {
                        r1 = (*pParse).nMem + 1 as ::core::ffi::c_int;
                        (*pParse).nMem += nFarg;
                    } else {
                        r1 = sqlite3GetTempRange(pParse, nFarg);
                    }
                    if (*pDef).funcFlags as ::core::ffi::c_uint
                        & (SQLITE_FUNC_LENGTH | SQLITE_FUNC_TYPEOF)
                            as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                    {
                        let mut exprOp: u8_0 = 0;
                        exprOp = (*(*(&raw mut (*pFarg).a as *mut ExprList_item)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .pExpr)
                            .op;
                        if exprOp as ::core::ffi::c_int == TK_COLUMN
                            || exprOp as ::core::ffi::c_int == TK_AGG_COLUMN
                        {
                            (*(*(&raw mut (*pFarg).a as *mut ExprList_item)
                                .offset(0 as ::core::ffi::c_int as isize))
                                .pExpr)
                                .op2 = ((*pDef).funcFlags as ::core::ffi::c_uint
                                & OPFLAG_BYTELENARG as ::core::ffi::c_uint) as u8_0;
                        }
                    }
                    sqlite3ExprCodeExprList(
                        pParse,
                        pFarg,
                        r1,
                        0 as ::core::ffi::c_int,
                        SQLITE_ECEL_FACTOR as u8_0,
                    );
                } else {
                    r1 = 0 as ::core::ffi::c_int;
                }
                if nFarg >= 2 as ::core::ffi::c_int
                    && (*pExpr).flags & 0x100 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint
                {
                    pDef = sqlite3VtabOverloadFunction(
                        db,
                        pDef,
                        nFarg,
                        (*(&raw mut (*pFarg).a as *mut ExprList_item)
                            .offset(1 as ::core::ffi::c_int as isize))
                            .pExpr,
                    );
                } else if nFarg > 0 as ::core::ffi::c_int {
                    pDef = sqlite3VtabOverloadFunction(
                        db,
                        pDef,
                        nFarg,
                        (*(&raw mut (*pFarg).a as *mut ExprList_item)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .pExpr,
                    );
                }
                if (*pDef).funcFlags as ::core::ffi::c_uint
                    & SQLITE_FUNC_NEEDCOLL as ::core::ffi::c_uint != 0
                {
                    if pColl.is_null() {
                        pColl = (*db).pDfltColl;
                    }
                    sqlite3VdbeAddOp4(
                        v,
                        OP_CollSeq,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        pColl as *mut ::core::ffi::c_char,
                        P4_COLLSEQ,
                    );
                }
                sqlite3VdbeAddFunctionCall(
                    pParse,
                    constMask as ::core::ffi::c_int,
                    r1,
                    target,
                    nFarg,
                    pDef,
                    (*pExpr).op2 as ::core::ffi::c_int,
                );
                if nFarg != 0 {
                    if constMask == 0 as ::core::ffi::c_uint {
                        sqlite3ReleaseTempRange(pParse, r1, nFarg);
                    }
                }
                return target;
            }
            1003765866876145076 => {
                if (*pAggInfo_0).directMode == 0 {
                    inReg = (*pAggInfo_0).iFirstReg
                        + (*pExpr).iAgg as ::core::ffi::c_int;
                    c2rust_current_block = 8004310806444026423;
                } else if (*(*pExpr).pAggInfo).useSortingIdx != 0 {
                    sqlite3VdbeAddOp3(
                        v,
                        OP_Column,
                        (*pAggInfo_0).sortingIdxPTab,
                        (*(*pAggInfo_0).aCol.offset((*pExpr).iAgg as isize))
                            .iSorterColumn,
                        target,
                    );
                    inReg = target;
                    c2rust_current_block = 8004310806444026423;
                } else {
                    c2rust_current_block = 3842749721932025293;
                }
            }
            5409161009579131794 => {
                let mut pLeft: *mut Expr = (*pExpr).pLeft;
                let mut addrIsNull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if sqlite3ExprIsVector(pLeft) != 0 {
                    codeVectorCompare(pParse, pExpr, target, op as u8_0, p5 as u8_0);
                } else {
                    if (*pExpr).flags & 0x400000 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint && p5 != SQLITE_NULLEQ
                    {
                        addrIsNull = exprComputeOperands(
                            pParse,
                            pExpr,
                            &raw mut r1,
                            &raw mut r2,
                            &raw mut regFree1,
                            &raw mut regFree2,
                        );
                    } else {
                        r1 = sqlite3ExprCodeTemp(
                            pParse,
                            (*pExpr).pLeft,
                            &raw mut regFree1,
                        );
                        r2 = sqlite3ExprCodeTemp(
                            pParse,
                            (*pExpr).pRight,
                            &raw mut regFree2,
                        );
                    }
                    sqlite3VdbeAddOp2(v, OP_Integer, 1 as ::core::ffi::c_int, inReg);
                    codeCompare(
                        pParse,
                        pLeft,
                        (*pExpr).pRight,
                        op,
                        r1,
                        r2,
                        sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
                        p5,
                        ((*pExpr).flags & 0x400 as ::core::ffi::c_int as u32_0
                            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                    );
                    if p5 == SQLITE_NULLEQ {
                        sqlite3VdbeAddOp2(v, OP_Integer, 0 as ::core::ffi::c_int, inReg);
                    } else {
                        sqlite3VdbeAddOp3(v, OP_ZeroOrNull, r1, inReg, r2);
                        if addrIsNull != 0 {
                            sqlite3VdbeAddOp2(
                                v,
                                OP_Goto,
                                0 as ::core::ffi::c_int,
                                sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
                            );
                            sqlite3VdbeJumpHere(v, addrIsNull);
                            sqlite3VdbeAddOp2(
                                v,
                                OP_Null,
                                0 as ::core::ffi::c_int,
                                inReg,
                            );
                        }
                    }
                }
                c2rust_current_block = 8004310806444026423;
            }
            16280550096250383041 => {
                pDel = sqlite3ExprDup(db_0, pX, 0 as ::core::ffi::c_int);
                if (*db_0).mallocFailed != 0 {
                    sqlite3ExprDelete(db_0, pDel);
                    c2rust_current_block = 8004310806444026423;
                } else {
                    sqlite3ExprToRegister(
                        pDel,
                        exprCodeVector(pParse, pDel, &raw mut regFree1),
                    );
                    memset(
                        &raw mut opCompare as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<Expr>() as size_t,
                    );
                    opCompare.op = TK_EQ as u8_0;
                    opCompare.pLeft = pDel;
                    pTest = &raw mut opCompare;
                    regFree1 = 0 as ::core::ffi::c_int;
                    c2rust_current_block = 4841263011268485695;
                }
            }
            5143058163439228106 => {
                pCol = (*pAggInfo).aCol.offset((*pExpr).iAgg as isize)
                    as *mut AggInfo_col as *mut AggInfo_col;
                if (*pAggInfo).directMode == 0 {
                    return (*pAggInfo).iFirstReg + (*pExpr).iAgg as ::core::ffi::c_int
                } else if (*pAggInfo).useSortingIdx != 0 {
                    let mut pTab: *mut Table = (*pCol).pTab;
                    sqlite3VdbeAddOp3(
                        v,
                        OP_Column,
                        (*pAggInfo).sortingIdxPTab,
                        (*pCol).iSorterColumn,
                        target,
                    );
                    if !pTab.is_null() {
                        if !((*pCol).iColumn < 0 as ::core::ffi::c_int) {
                            if (*(*pTab).aCol.offset((*pCol).iColumn as isize)).affinity
                                as ::core::ffi::c_int == SQLITE_AFF_REAL
                            {
                                sqlite3VdbeAddOp1(v, OP_RealAffinity, target);
                            }
                        }
                    }
                    return target;
                } else if (*pExpr).y.pTab.is_null() {
                    sqlite3VdbeAddOp3(
                        v,
                        OP_Column,
                        (*pExpr).iTable,
                        (*pExpr).iColumn as ::core::ffi::c_int,
                        target,
                    );
                    return target;
                }
                c2rust_current_block = 7890507820729860149;
            }
            9606288038608642794 => {
                sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, target);
                c2rust_current_block = 8004310806444026423;
            }
            5577234879133443219 => {
                sqlite3ErrorMsg(
                    pParse,
                    b"unknown function: %#T()\0".as_ptr() as *const ::core::ffi::c_char,
                    pExpr,
                );
                c2rust_current_block = 8004310806444026423;
            }
            _ => {}
        }
        match c2rust_current_block {
            3842749721932025293 => {
                addrINR = sqlite3VdbeAddOp3(
                    v,
                    OP_IfNullRow,
                    (*pExpr).iTable,
                    0 as ::core::ffi::c_int,
                    target,
                );
                (*pParse).set_okConstFactor(0 as bft as bft);
                sqlite3ExprCode(pParse, (*pExpr).pLeft, target);
                (*pParse).set_okConstFactor(okConstFactor as bft as bft);
                sqlite3VdbeJumpHere(v, addrINR);
            }
            4841263011268485695 => {
                i_0 = 0 as ::core::ffi::c_int;
                while i_0 < nExpr - 1 as ::core::ffi::c_int {
                    if !pX.is_null() {
                        opCompare.pRight = (*aListelem.offset(i_0 as isize)).pExpr;
                    } else {
                        pTest = (*aListelem.offset(i_0 as isize)).pExpr;
                    }
                    nextCase = sqlite3VdbeMakeLabel(pParse);
                    sqlite3ExprIfFalse(pParse, pTest, nextCase, SQLITE_JUMPIFNULL);
                    sqlite3ExprCode(
                        pParse,
                        (*aListelem.offset((i_0 + 1 as ::core::ffi::c_int) as isize))
                            .pExpr,
                        target,
                    );
                    sqlite3VdbeGoto(v, endLabel);
                    sqlite3VdbeResolveLabel(v, nextCase);
                    i_0 = i_0 + 2 as ::core::ffi::c_int;
                }
                if nExpr & 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    sqlite3ExprCode(
                        pParse,
                        (*(&raw mut (*pEList).a as *mut ExprList_item)
                            .offset((nExpr - 1 as ::core::ffi::c_int) as isize))
                            .pExpr,
                        target,
                    );
                } else {
                    sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, target);
                }
                sqlite3ExprDelete(db_0, pDel);
                setDoNotMergeFlagOnCopy(v);
                sqlite3VdbeResolveLabel(v, endLabel);
            }
            7890507820729860149 => {
                let mut iTab: ::core::ffi::c_int = (*pExpr).iTable;
                let mut iReg: ::core::ffi::c_int = 0;
                if (*pExpr).flags & 0x20 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint
                {
                    let mut aff: ::core::ffi::c_int = 0;
                    iReg = sqlite3ExprCodeTarget(pParse, (*pExpr).pLeft, target);
                    aff = sqlite3TableColumnAffinity(
                        (*pExpr).y.pTab,
                        (*pExpr).iColumn as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int;
                    if aff > SQLITE_AFF_BLOB {
                        static mut zAff: [::core::ffi::c_char; 10] = unsafe {
                            ::core::mem::transmute::<
                                [u8; 10],
                                [::core::ffi::c_char; 10],
                            >(*b"B\0C\0D\0E\0F\0")
                        };
                        sqlite3VdbeAddOp4(
                            v,
                            OP_Affinity,
                            iReg,
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            (&raw const zAff as *const ::core::ffi::c_char)
                                .offset(
                                    ((aff - 'B' as i32) * 2 as ::core::ffi::c_int) as isize,
                                ) as *const ::core::ffi::c_char,
                            P4_STATIC,
                        );
                    }
                    return iReg;
                }
                if iTab < 0 as ::core::ffi::c_int {
                    if (*pParse).iSelfTab < 0 as ::core::ffi::c_int {
                        let mut pCol_0: *mut Column = ::core::ptr::null_mut::<Column>();
                        let mut pTab_0: *mut Table = ::core::ptr::null_mut::<Table>();
                        let mut iSrc: ::core::ffi::c_int = 0;
                        let mut iCol: ::core::ffi::c_int = (*pExpr).iColumn
                            as ::core::ffi::c_int;
                        pTab_0 = (*pExpr).y.pTab;
                        if iCol < 0 as ::core::ffi::c_int {
                            return -1 as ::core::ffi::c_int - (*pParse).iSelfTab;
                        }
                        pCol_0 = (*pTab_0).aCol.offset(iCol as isize);
                        iSrc = sqlite3TableColumnToStorage(pTab_0, iCol as i16_0)
                            as ::core::ffi::c_int - (*pParse).iSelfTab;
                        if (*pCol_0).colFlags as ::core::ffi::c_int & COLFLAG_GENERATED
                            != 0
                        {
                            if (*pCol_0).colFlags as ::core::ffi::c_int & COLFLAG_BUSY
                                != 0
                            {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"generated column loop on \"%s\"\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*pCol_0).zCnName,
                                );
                                return 0 as ::core::ffi::c_int;
                            }
                            (*pCol_0).colFlags = ((*pCol_0).colFlags
                                as ::core::ffi::c_int | COLFLAG_BUSY) as u16_0;
                            if (*pCol_0).colFlags as ::core::ffi::c_int
                                & COLFLAG_NOTAVAIL != 0
                            {
                                sqlite3ExprCodeGeneratedColumn(
                                    pParse,
                                    pTab_0,
                                    pCol_0,
                                    iSrc,
                                );
                            }
                            (*pCol_0).colFlags = ((*pCol_0).colFlags
                                as ::core::ffi::c_int & !(COLFLAG_BUSY | COLFLAG_NOTAVAIL))
                                as u16_0;
                            return iSrc;
                        } else if (*pCol_0).affinity as ::core::ffi::c_int
                            == SQLITE_AFF_REAL
                        {
                            sqlite3VdbeAddOp2(v, OP_SCopy, iSrc, target);
                            sqlite3VdbeAddOp1(v, OP_RealAffinity, target);
                            return target;
                        } else {
                            return iSrc
                        }
                    } else {
                        iTab = (*pParse).iSelfTab - 1 as ::core::ffi::c_int;
                    }
                } else if !(*pParse).pIdxPartExpr.is_null()
                    && {
                        r1 = exprPartidxExprLookup(pParse, pExpr, target);
                        0 as ::core::ffi::c_int != r1
                    }
                {
                    return r1
                }
                iReg = sqlite3ExprCodeGetColumn(
                    pParse,
                    (*pExpr).y.pTab,
                    (*pExpr).iColumn as ::core::ffi::c_int,
                    iTab,
                    target,
                    (*pExpr).op2,
                );
                return iReg;
            }
            _ => {}
        }
        sqlite3ReleaseTempReg(pParse, regFree1);
        sqlite3ReleaseTempReg(pParse, regFree2);
        return inReg;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCodeRunJustOnce(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut regDest: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        p = (*pParse).pConstExpr;
        if regDest < 0 as ::core::ffi::c_int && !p.is_null() {
            let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
            let mut i: ::core::ffi::c_int = 0;
            pItem = &raw mut (*p).a as *mut ExprList_item as *mut ExprList_item;
            i = (*p).nExpr;
            while i > 0 as ::core::ffi::c_int {
                if (*pItem).fg.reusable() as ::core::ffi::c_int != 0
                    && sqlite3ExprCompare(
                        ::core::ptr::null::<Parse>(),
                        (*pItem).pExpr,
                        pExpr,
                        -1 as ::core::ffi::c_int,
                    ) == 0 as ::core::ffi::c_int
                {
                    return (*pItem).u.iConstExprReg;
                }
                pItem = pItem.offset(1);
                i -= 1;
            }
        }
        pExpr = sqlite3ExprDup((*pParse).db, pExpr, 0 as ::core::ffi::c_int);
        if !pExpr.is_null()
            && (*pExpr).flags & 0x8 as ::core::ffi::c_int as u32_0
                != 0 as ::core::ffi::c_uint
        {
            let mut v: *mut Vdbe = (*pParse).pVdbe;
            let mut addr: ::core::ffi::c_int = 0;
            addr = sqlite3VdbeAddOp0(v, OP_Once);
            (*pParse).set_okConstFactor(0 as bft as bft);
            if (*(*pParse).db).mallocFailed == 0 {
                if regDest < 0 as ::core::ffi::c_int {
                    (*pParse).nMem += 1;
                    regDest = (*pParse).nMem;
                }
                sqlite3ExprCode(pParse, pExpr, regDest);
            }
            (*pParse).set_okConstFactor(1 as bft as bft);
            sqlite3ExprDelete((*pParse).db, pExpr);
            sqlite3VdbeJumpHere(v, addr);
        } else {
            p = sqlite3ExprListAppend(pParse, p, pExpr);
            if !p.is_null() {
                let mut pItem_0: *mut ExprList_item = (&raw mut (*p).a
                    as *mut ExprList_item)
                    .offset(((*p).nExpr - 1 as ::core::ffi::c_int) as isize)
                    as *mut ExprList_item;
                (*pItem_0)
                    .fg
                    .set_reusable(
                        (regDest < 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                            as ::core::ffi::c_uint as ::core::ffi::c_uint,
                    );
                if regDest < 0 as ::core::ffi::c_int {
                    (*pParse).nMem += 1;
                    regDest = (*pParse).nMem;
                }
                (*pItem_0).u.iConstExprReg = regDest;
            }
            (*pParse).pConstExpr = p;
        }
        return regDest;
    }
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sqlite3ExprNullRegisterRange(
    mut pParse: *mut Parse,
    mut iReg: ::core::ffi::c_int,
    mut nReg: ::core::ffi::c_int,
) {
    unsafe {
        let mut okConstFactor: u8_0 = (*pParse).okConstFactor() as u8_0;
        let mut t: Expr = Expr {
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
        memset(
            &raw mut t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Expr>() as size_t,
        );
        t.op = TK_NULLS as u8_0;
        t.y.nReg = nReg;
        (*pParse).set_okConstFactor(1 as bft as bft);
        sqlite3ExprCodeRunJustOnce(pParse, &raw mut t, iReg);
        (*pParse).set_okConstFactor(okConstFactor as bft as bft);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCodeTemp(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pReg: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut r2: ::core::ffi::c_int = 0;
        pExpr = sqlite3ExprSkipCollateAndLikely(pExpr);
        if (*pParse).okConstFactor() as ::core::ffi::c_int != 0 && !pExpr.is_null()
            && (*pExpr).op as ::core::ffi::c_int != TK_REGISTER
            && sqlite3ExprIsConstantNotJoin(pParse, pExpr) != 0
        {
            *pReg = 0 as ::core::ffi::c_int;
            r2 = sqlite3ExprCodeRunJustOnce(pParse, pExpr, -1 as ::core::ffi::c_int);
        } else {
            let mut r1: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
            r2 = sqlite3ExprCodeTarget(pParse, pExpr, r1);
            if r2 == r1 {
                *pReg = r1;
            } else {
                sqlite3ReleaseTempReg(pParse, r1);
                *pReg = 0 as ::core::ffi::c_int;
            }
        }
        return r2;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCode(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut target: ::core::ffi::c_int,
) {
    unsafe {
        let mut inReg: ::core::ffi::c_int = 0;
        if (*pParse).pVdbe.is_null() {
            return;
        }
        inReg = sqlite3ExprCodeTarget(pParse, pExpr, target);
        if inReg != target {
            let mut op: u8_0 = 0;
            let mut pX: *mut Expr = sqlite3ExprSkipCollateAndLikely(pExpr);
            if !pX.is_null()
                && ((*pX).flags & 0x400000 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint
                    || (*pX).op as ::core::ffi::c_int == TK_REGISTER)
            {
                op = OP_Copy as u8_0;
            } else {
                op = OP_SCopy as u8_0;
            }
            sqlite3VdbeAddOp2((*pParse).pVdbe, op as ::core::ffi::c_int, inReg, target);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCodeCopy(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut target: ::core::ffi::c_int,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        pExpr = sqlite3ExprDup(db, pExpr, 0 as ::core::ffi::c_int);
        if (*db).mallocFailed == 0 {
            sqlite3ExprCode(pParse, pExpr, target);
        }
        sqlite3ExprDelete(db, pExpr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCodeFactorable(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut target: ::core::ffi::c_int,
) {
    unsafe {
        if (*pParse).okConstFactor() as ::core::ffi::c_int != 0
            && sqlite3ExprIsConstantNotJoin(pParse, pExpr) != 0
        {
            sqlite3ExprCodeRunJustOnce(pParse, pExpr, target);
        } else {
            sqlite3ExprCodeCopy(pParse, pExpr, target);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCodeExprList(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut target: ::core::ffi::c_int,
    mut srcReg: ::core::ffi::c_int,
    mut flags: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut copyOp: u8_0 = (if flags as ::core::ffi::c_int & SQLITE_ECEL_DUP != 0 {
            OP_Copy
        } else {
            OP_SCopy
        }) as u8_0;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        n = (*pList).nExpr;
        if (*pParse).okConstFactor() == 0 {
            flags = (flags as ::core::ffi::c_int & !SQLITE_ECEL_FACTOR) as u8_0;
        }
        pItem = &raw mut (*pList).a as *mut ExprList_item as *mut ExprList_item;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            let mut pExpr: *mut Expr = (*pItem).pExpr;
            if flags as ::core::ffi::c_int & SQLITE_ECEL_REF != 0 as ::core::ffi::c_int
                && {
                    j = (*pItem).u.x.iOrderByCol as ::core::ffi::c_int;
                    j > 0 as ::core::ffi::c_int
                }
            {
                if flags as ::core::ffi::c_int & SQLITE_ECEL_OMITREF != 0 {
                    i -= 1;
                    n -= 1;
                } else {
                    sqlite3VdbeAddOp2(
                        v,
                        copyOp as ::core::ffi::c_int,
                        j + srcReg - 1 as ::core::ffi::c_int,
                        target + i,
                    );
                }
            } else if flags as ::core::ffi::c_int & SQLITE_ECEL_FACTOR
                != 0 as ::core::ffi::c_int
                && sqlite3ExprIsConstantNotJoin(pParse, pExpr) != 0
            {
                sqlite3ExprCodeRunJustOnce(pParse, pExpr, target + i);
            } else {
                let mut inReg: ::core::ffi::c_int = sqlite3ExprCodeTarget(
                    pParse,
                    pExpr,
                    target + i,
                );
                if inReg != target + i {
                    let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
                    if copyOp as ::core::ffi::c_int == OP_Copy
                        && {
                            pOp = sqlite3VdbeGetLastOp(v);
                            (*pOp).opcode as ::core::ffi::c_int == OP_Copy
                        } && (*pOp).p1 + (*pOp).p3 + 1 as ::core::ffi::c_int == inReg
                        && (*pOp).p2 + (*pOp).p3 + 1 as ::core::ffi::c_int == target + i
                        && (*pOp).p5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        (*pOp).p3 += 1;
                    } else {
                        sqlite3VdbeAddOp2(
                            v,
                            copyOp as ::core::ffi::c_int,
                            inReg,
                            target + i,
                        );
                    }
                }
            }
            i += 1;
            pItem = pItem.offset(1);
        }
        return n;
    }
}
unsafe extern "C" fn exprCodeBetween(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut dest: ::core::ffi::c_int,
    mut xJump: Option<
        unsafe extern "C" fn(
            *mut Parse,
            *mut Expr,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    mut jumpIfNull: ::core::ffi::c_int,
) {
    unsafe {
        let mut exprAnd: Expr = Expr {
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
        let mut compLeft: Expr = Expr {
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
        let mut compRight: Expr = Expr {
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
        let mut regFree1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pDel: *mut Expr = ::core::ptr::null_mut::<Expr>();
        let mut db: *mut sqlite3 = (*pParse).db;
        memset(
            &raw mut compLeft as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Expr>() as size_t,
        );
        memset(
            &raw mut compRight as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Expr>() as size_t,
        );
        memset(
            &raw mut exprAnd as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Expr>() as size_t,
        );
        pDel = sqlite3ExprDup(db, (*pExpr).pLeft, 0 as ::core::ffi::c_int);
        if (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            exprAnd.op = TK_AND as u8_0;
            exprAnd.pLeft = &raw mut compLeft;
            exprAnd.pRight = &raw mut compRight;
            compLeft.op = TK_GE as u8_0;
            compLeft.pLeft = pDel;
            compLeft.pRight = (*(&raw mut (*(*pExpr).x.pList).a as *mut ExprList_item)
                .offset(0 as ::core::ffi::c_int as isize))
                .pExpr;
            compRight.op = TK_LE as u8_0;
            compRight.pLeft = pDel;
            compRight.pRight = (*(&raw mut (*(*pExpr).x.pList).a as *mut ExprList_item)
                .offset(1 as ::core::ffi::c_int as isize))
                .pExpr;
            sqlite3ExprToRegister(pDel, exprCodeVector(pParse, pDel, &raw mut regFree1));
            if xJump.is_some() {
                xJump
                    .expect(
                        "non-null function pointer",
                    )(pParse, &raw mut exprAnd, dest, jumpIfNull);
            } else {
                (*pDel).flags |= EP_OuterON as ::core::ffi::c_uint;
                sqlite3ExprCodeTarget(pParse, &raw mut exprAnd, dest);
            }
            sqlite3ReleaseTempReg(pParse, regFree1);
        }
        sqlite3ExprDelete(db, pDel);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprIfTrue(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut dest: ::core::ffi::c_int,
    mut jumpIfNull: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut op: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regFree1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regFree2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut r1: ::core::ffi::c_int = 0;
        let mut r2: ::core::ffi::c_int = 0;
        if v.is_null() {
            return;
        }
        if pExpr.is_null() {
            return;
        }
        op = (*pExpr).op as ::core::ffi::c_int;
        let mut c2rust_current_block_108: u64;
        match op {
            TK_AND | TK_OR => {
                let mut pAlt: *mut Expr = sqlite3ExprSimplifiedAndOr(pExpr);
                if pAlt != pExpr {
                    sqlite3ExprIfTrue(pParse, pAlt, dest, jumpIfNull);
                } else {
                    let mut pFirst: *mut Expr = ::core::ptr::null_mut::<Expr>();
                    let mut pSecond: *mut Expr = ::core::ptr::null_mut::<Expr>();
                    if exprEvalRhsFirst(pExpr) != 0 {
                        pFirst = (*pExpr).pRight;
                        pSecond = (*pExpr).pLeft;
                    } else {
                        pFirst = (*pExpr).pLeft;
                        pSecond = (*pExpr).pRight;
                    }
                    if op == TK_AND {
                        let mut d2: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
                        sqlite3ExprIfFalse(
                            pParse,
                            pFirst,
                            d2,
                            jumpIfNull ^ SQLITE_JUMPIFNULL,
                        );
                        sqlite3ExprIfTrue(pParse, pSecond, dest, jumpIfNull);
                        sqlite3VdbeResolveLabel(v, d2);
                    } else {
                        sqlite3ExprIfTrue(pParse, pFirst, dest, jumpIfNull);
                        sqlite3ExprIfTrue(pParse, pSecond, dest, jumpIfNull);
                    }
                }
                c2rust_current_block_108 = 10369920510435091891;
            }
            TK_NOT => {
                sqlite3ExprIfFalse(pParse, (*pExpr).pLeft, dest, jumpIfNull);
                c2rust_current_block_108 = 10369920510435091891;
            }
            TK_TRUTH => {
                let mut isNot: ::core::ffi::c_int = 0;
                let mut isTrue: ::core::ffi::c_int = 0;
                isNot = ((*pExpr).op2 as ::core::ffi::c_int == TK_ISNOT)
                    as ::core::ffi::c_int;
                isTrue = sqlite3ExprTruthValue((*pExpr).pRight);
                if isTrue ^ isNot != 0 {
                    sqlite3ExprIfTrue(
                        pParse,
                        (*pExpr).pLeft,
                        dest,
                        if isNot != 0 {
                            SQLITE_JUMPIFNULL
                        } else {
                            0 as ::core::ffi::c_int
                        },
                    );
                } else {
                    sqlite3ExprIfFalse(
                        pParse,
                        (*pExpr).pLeft,
                        dest,
                        if isNot != 0 {
                            SQLITE_JUMPIFNULL
                        } else {
                            0 as ::core::ffi::c_int
                        },
                    );
                }
                c2rust_current_block_108 = 10369920510435091891;
            }
            TK_IS | TK_ISNOT => {
                op = if op == TK_IS { TK_EQ } else { TK_NE };
                jumpIfNull = SQLITE_NULLEQ;
                c2rust_current_block_108 = 15090052786889560393;
            }
            TK_LT | TK_LE | TK_GT | TK_GE | TK_NE | TK_EQ => {
                c2rust_current_block_108 = 15090052786889560393;
            }
            TK_ISNULL | TK_NOTNULL => {
                r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, &raw mut regFree1);
                if regFree1 != 0 {
                    sqlite3VdbeTypeofColumn(v, r1);
                }
                sqlite3VdbeAddOp2(v, op, r1, dest);
                c2rust_current_block_108 = 10369920510435091891;
            }
            TK_BETWEEN => {
                exprCodeBetween(
                    pParse,
                    pExpr,
                    dest,
                    Some(
                        sqlite3ExprIfTrue
                            as unsafe extern "C" fn(
                                *mut Parse,
                                *mut Expr,
                                ::core::ffi::c_int,
                                ::core::ffi::c_int,
                            ) -> (),
                    ),
                    jumpIfNull,
                );
                c2rust_current_block_108 = 10369920510435091891;
            }
            TK_IN => {
                let mut destIfFalse: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
                let mut destIfNull: ::core::ffi::c_int = if jumpIfNull != 0 {
                    dest
                } else {
                    destIfFalse
                };
                sqlite3ExprCodeIN(pParse, pExpr, destIfFalse, destIfNull);
                sqlite3VdbeGoto(v, dest);
                sqlite3VdbeResolveLabel(v, destIfFalse);
                c2rust_current_block_108 = 10369920510435091891;
            }
            _ => {
                c2rust_current_block_108 = 16032508483029926059;
            }
        }
        match c2rust_current_block_108 {
            15090052786889560393 => {
                let mut addrIsNull: ::core::ffi::c_int = 0;
                if sqlite3ExprIsVector((*pExpr).pLeft) != 0 {
                    c2rust_current_block_108 = 16032508483029926059;
                } else {
                    if (*pExpr).flags & 0x400000 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint && jumpIfNull != SQLITE_NULLEQ
                    {
                        addrIsNull = exprComputeOperands(
                            pParse,
                            pExpr,
                            &raw mut r1,
                            &raw mut r2,
                            &raw mut regFree1,
                            &raw mut regFree2,
                        );
                    } else {
                        r1 = sqlite3ExprCodeTemp(
                            pParse,
                            (*pExpr).pLeft,
                            &raw mut regFree1,
                        );
                        r2 = sqlite3ExprCodeTemp(
                            pParse,
                            (*pExpr).pRight,
                            &raw mut regFree2,
                        );
                        addrIsNull = 0 as ::core::ffi::c_int;
                    }
                    codeCompare(
                        pParse,
                        (*pExpr).pLeft,
                        (*pExpr).pRight,
                        op,
                        r1,
                        r2,
                        dest,
                        jumpIfNull,
                        ((*pExpr).flags & 0x400 as ::core::ffi::c_int as u32_0
                            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                    );
                    if addrIsNull != 0 {
                        if jumpIfNull != 0 {
                            sqlite3VdbeChangeP2(v, addrIsNull, dest);
                        } else {
                            sqlite3VdbeJumpHere(v, addrIsNull);
                        }
                    }
                    c2rust_current_block_108 = 10369920510435091891;
                }
            }
            _ => {}
        }
        match c2rust_current_block_108 {
            16032508483029926059 => {
                if (*pExpr).flags as ::core::ffi::c_uint
                    & (EP_OuterON | EP_IsTrue) as ::core::ffi::c_uint
                    == EP_IsTrue as ::core::ffi::c_uint
                {
                    sqlite3VdbeGoto(v, dest);
                } else if !((*pExpr).flags as ::core::ffi::c_uint
                    & (EP_OuterON | EP_IsFalse) as ::core::ffi::c_uint
                    == EP_IsFalse as ::core::ffi::c_uint)
                {
                    r1 = sqlite3ExprCodeTemp(pParse, pExpr, &raw mut regFree1);
                    sqlite3VdbeAddOp3(
                        v,
                        OP_If,
                        r1,
                        dest,
                        (jumpIfNull != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    );
                }
            }
            _ => {}
        }
        sqlite3ReleaseTempReg(pParse, regFree1);
        sqlite3ReleaseTempReg(pParse, regFree2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprIfFalse(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut dest: ::core::ffi::c_int,
    mut jumpIfNull: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut op: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regFree1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regFree2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut r1: ::core::ffi::c_int = 0;
        let mut r2: ::core::ffi::c_int = 0;
        if v.is_null() {
            return;
        }
        if pExpr.is_null() {
            return;
        }
        op = ((*pExpr).op as ::core::ffi::c_int + (TK_ISNULL & 1 as ::core::ffi::c_int)
            ^ 1 as ::core::ffi::c_int) - (TK_ISNULL & 1 as ::core::ffi::c_int);
        let mut c2rust_current_block_117: u64;
        match (*pExpr).op as ::core::ffi::c_int {
            TK_AND | TK_OR => {
                let mut pAlt: *mut Expr = sqlite3ExprSimplifiedAndOr(pExpr);
                if pAlt != pExpr {
                    sqlite3ExprIfFalse(pParse, pAlt, dest, jumpIfNull);
                } else {
                    let mut pFirst: *mut Expr = ::core::ptr::null_mut::<Expr>();
                    let mut pSecond: *mut Expr = ::core::ptr::null_mut::<Expr>();
                    if exprEvalRhsFirst(pExpr) != 0 {
                        pFirst = (*pExpr).pRight;
                        pSecond = (*pExpr).pLeft;
                    } else {
                        pFirst = (*pExpr).pLeft;
                        pSecond = (*pExpr).pRight;
                    }
                    if (*pExpr).op as ::core::ffi::c_int == TK_AND {
                        sqlite3ExprIfFalse(pParse, pFirst, dest, jumpIfNull);
                        sqlite3ExprIfFalse(pParse, pSecond, dest, jumpIfNull);
                    } else {
                        let mut d2: ::core::ffi::c_int = sqlite3VdbeMakeLabel(pParse);
                        sqlite3ExprIfTrue(
                            pParse,
                            pFirst,
                            d2,
                            jumpIfNull ^ SQLITE_JUMPIFNULL,
                        );
                        sqlite3ExprIfFalse(pParse, pSecond, dest, jumpIfNull);
                        sqlite3VdbeResolveLabel(v, d2);
                    }
                }
                c2rust_current_block_117 = 12369290732426379360;
            }
            TK_NOT => {
                sqlite3ExprIfTrue(pParse, (*pExpr).pLeft, dest, jumpIfNull);
                c2rust_current_block_117 = 12369290732426379360;
            }
            TK_TRUTH => {
                let mut isNot: ::core::ffi::c_int = 0;
                let mut isTrue: ::core::ffi::c_int = 0;
                isNot = ((*pExpr).op2 as ::core::ffi::c_int == TK_ISNOT)
                    as ::core::ffi::c_int;
                isTrue = sqlite3ExprTruthValue((*pExpr).pRight);
                if isTrue ^ isNot != 0 {
                    sqlite3ExprIfFalse(
                        pParse,
                        (*pExpr).pLeft,
                        dest,
                        if isNot != 0 {
                            0 as ::core::ffi::c_int
                        } else {
                            SQLITE_JUMPIFNULL
                        },
                    );
                } else {
                    sqlite3ExprIfTrue(
                        pParse,
                        (*pExpr).pLeft,
                        dest,
                        if isNot != 0 {
                            0 as ::core::ffi::c_int
                        } else {
                            SQLITE_JUMPIFNULL
                        },
                    );
                }
                c2rust_current_block_117 = 12369290732426379360;
            }
            TK_IS | TK_ISNOT => {
                op = if (*pExpr).op as ::core::ffi::c_int == TK_IS {
                    TK_NE
                } else {
                    TK_EQ
                };
                jumpIfNull = SQLITE_NULLEQ;
                c2rust_current_block_117 = 15004371738079956865;
            }
            TK_LT | TK_LE | TK_GT | TK_GE | TK_NE | TK_EQ => {
                c2rust_current_block_117 = 15004371738079956865;
            }
            TK_ISNULL | TK_NOTNULL => {
                r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, &raw mut regFree1);
                if regFree1 != 0 {
                    sqlite3VdbeTypeofColumn(v, r1);
                }
                sqlite3VdbeAddOp2(v, op, r1, dest);
                c2rust_current_block_117 = 12369290732426379360;
            }
            TK_BETWEEN => {
                exprCodeBetween(
                    pParse,
                    pExpr,
                    dest,
                    Some(
                        sqlite3ExprIfFalse
                            as unsafe extern "C" fn(
                                *mut Parse,
                                *mut Expr,
                                ::core::ffi::c_int,
                                ::core::ffi::c_int,
                            ) -> (),
                    ),
                    jumpIfNull,
                );
                c2rust_current_block_117 = 12369290732426379360;
            }
            TK_IN => {
                if jumpIfNull != 0 {
                    sqlite3ExprCodeIN(pParse, pExpr, dest, dest);
                } else {
                    let mut destIfNull: ::core::ffi::c_int = sqlite3VdbeMakeLabel(
                        pParse,
                    );
                    sqlite3ExprCodeIN(pParse, pExpr, dest, destIfNull);
                    sqlite3VdbeResolveLabel(v, destIfNull);
                }
                c2rust_current_block_117 = 12369290732426379360;
            }
            _ => {
                c2rust_current_block_117 = 11899613247835612262;
            }
        }
        match c2rust_current_block_117 {
            15004371738079956865 => {
                let mut addrIsNull: ::core::ffi::c_int = 0;
                if sqlite3ExprIsVector((*pExpr).pLeft) != 0 {
                    c2rust_current_block_117 = 11899613247835612262;
                } else {
                    if (*pExpr).flags & 0x400000 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint && jumpIfNull != SQLITE_NULLEQ
                    {
                        addrIsNull = exprComputeOperands(
                            pParse,
                            pExpr,
                            &raw mut r1,
                            &raw mut r2,
                            &raw mut regFree1,
                            &raw mut regFree2,
                        );
                    } else {
                        r1 = sqlite3ExprCodeTemp(
                            pParse,
                            (*pExpr).pLeft,
                            &raw mut regFree1,
                        );
                        r2 = sqlite3ExprCodeTemp(
                            pParse,
                            (*pExpr).pRight,
                            &raw mut regFree2,
                        );
                        addrIsNull = 0 as ::core::ffi::c_int;
                    }
                    codeCompare(
                        pParse,
                        (*pExpr).pLeft,
                        (*pExpr).pRight,
                        op,
                        r1,
                        r2,
                        dest,
                        jumpIfNull,
                        ((*pExpr).flags & 0x400 as ::core::ffi::c_int as u32_0
                            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                    );
                    if addrIsNull != 0 {
                        if jumpIfNull != 0 {
                            sqlite3VdbeChangeP2(v, addrIsNull, dest);
                        } else {
                            sqlite3VdbeJumpHere(v, addrIsNull);
                        }
                    }
                    c2rust_current_block_117 = 12369290732426379360;
                }
            }
            _ => {}
        }
        match c2rust_current_block_117 {
            11899613247835612262 => {
                if (*pExpr).flags as ::core::ffi::c_uint
                    & (EP_OuterON | EP_IsFalse) as ::core::ffi::c_uint
                    == EP_IsFalse as ::core::ffi::c_uint
                {
                    sqlite3VdbeGoto(v, dest);
                } else if !((*pExpr).flags as ::core::ffi::c_uint
                    & (EP_OuterON | EP_IsTrue) as ::core::ffi::c_uint
                    == EP_IsTrue as ::core::ffi::c_uint)
                {
                    r1 = sqlite3ExprCodeTemp(pParse, pExpr, &raw mut regFree1);
                    sqlite3VdbeAddOp3(
                        v,
                        OP_IfNot,
                        r1,
                        dest,
                        (jumpIfNull != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    );
                }
            }
            _ => {}
        }
        sqlite3ReleaseTempReg(pParse, regFree1);
        sqlite3ReleaseTempReg(pParse, regFree2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprIfFalseDup(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut dest: ::core::ffi::c_int,
    mut jumpIfNull: ::core::ffi::c_int,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pCopy: *mut Expr = sqlite3ExprDup(db, pExpr, 0 as ::core::ffi::c_int);
        if (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            sqlite3ExprIfFalse(pParse, pCopy, dest, jumpIfNull);
        }
        sqlite3ExprDelete(db, pCopy);
    }
}
#[inline(never)]
unsafe extern "C" fn exprCompareVariable(
    mut pParse: *const Parse,
    mut pVar: *const Expr,
    mut pExpr: *const Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut res: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
        let mut iVar: ::core::ffi::c_int = 0;
        let mut pL: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut pR: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        if (*pExpr).op as ::core::ffi::c_int == TK_VARIABLE
            && (*pVar).iColumn as ::core::ffi::c_int
                == (*pExpr).iColumn as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*pParse).db).flags as ::core::ffi::c_ulonglong
            & SQLITE_EnableQPSG as ::core::ffi::c_ulonglong
            != 0 as ::core::ffi::c_ulonglong
        {
            return 2 as ::core::ffi::c_int;
        }
        sqlite3ValueFromExpr(
            (*pParse).db,
            pExpr,
            SQLITE_UTF8 as u8_0,
            SQLITE_AFF_BLOB as u8_0,
            &raw mut pR,
        );
        if !pR.is_null() {
            iVar = (*pVar).iColumn as ::core::ffi::c_int;
            sqlite3VdbeSetVarmask((*pParse).pVdbe, iVar);
            pL = sqlite3VdbeGetBoundValue(
                (*pParse).pReprepare,
                iVar,
                SQLITE_AFF_BLOB as u8_0,
            );
            if !pL.is_null() {
                if sqlite3_value_type(pL) == SQLITE_TEXT {
                    sqlite3_value_text(pL);
                }
                res = if sqlite3MemCompare(pL, pR, ::core::ptr::null::<CollSeq>()) != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                };
            }
            sqlite3ValueFree(pR);
            sqlite3ValueFree(pL);
        }
        return res;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCompare(
    mut pParse: *const Parse,
    mut pA: *const Expr,
    mut pB: *const Expr,
    mut iTab: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut combinedFlags: u32_0 = 0;
        if pA.is_null() || pB.is_null() {
            return if pB == pA {
                0 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            };
        }
        if !pParse.is_null() && (*pA).op as ::core::ffi::c_int == TK_VARIABLE {
            return exprCompareVariable(pParse, pA, pB);
        }
        combinedFlags = (*pA).flags | (*pB).flags;
        if combinedFlags as ::core::ffi::c_uint & EP_IntValue as ::core::ffi::c_uint != 0
        {
            if (*pA).flags as ::core::ffi::c_uint & (*pB).flags as ::core::ffi::c_uint
                & EP_IntValue as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
                && (*pA).u.iValue == (*pB).u.iValue
            {
                return 0 as ::core::ffi::c_int;
            }
            return 2 as ::core::ffi::c_int;
        }
        if (*pA).op as ::core::ffi::c_int != (*pB).op as ::core::ffi::c_int
            || (*pA).op as ::core::ffi::c_int == TK_RAISE
        {
            if (*pA).op as ::core::ffi::c_int == TK_COLLATE
                && sqlite3ExprCompare(pParse, (*pA).pLeft, pB, iTab)
                    < 2 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            if (*pB).op as ::core::ffi::c_int == TK_COLLATE
                && sqlite3ExprCompare(pParse, pA, (*pB).pLeft, iTab)
                    < 2 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            if (*pA).op as ::core::ffi::c_int == TK_AGG_COLUMN
                && (*pB).op as ::core::ffi::c_int == TK_COLUMN
                && (*pB).iTable < 0 as ::core::ffi::c_int && (*pA).iTable == iTab
            {} else {
                return 2 as ::core::ffi::c_int
            }
        }
        if !(*pA).u.zToken.is_null() {
            if (*pA).op as ::core::ffi::c_int == TK_FUNCTION
                || (*pA).op as ::core::ffi::c_int == TK_AGG_FUNCTION
            {
                if sqlite3StrICmp((*pA).u.zToken, (*pB).u.zToken)
                    != 0 as ::core::ffi::c_int
                {
                    return 2 as ::core::ffi::c_int;
                }
                if ((*pA).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
                    != ((*pB).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                        != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
                {
                    return 2 as ::core::ffi::c_int;
                }
                if (*pA).flags & 0x1000000 as ::core::ffi::c_int as u32_0
                    != 0 as ::core::ffi::c_uint
                {
                    if sqlite3WindowCompare(
                        pParse,
                        (*pA).y.pWin,
                        (*pB).y.pWin,
                        1 as ::core::ffi::c_int,
                    ) != 0 as ::core::ffi::c_int
                    {
                        return 2 as ::core::ffi::c_int;
                    }
                }
            } else if (*pA).op as ::core::ffi::c_int == TK_NULL {
                return 0 as ::core::ffi::c_int
            } else if (*pA).op as ::core::ffi::c_int == TK_COLLATE {
                if sqlite3_stricmp((*pA).u.zToken, (*pB).u.zToken)
                    != 0 as ::core::ffi::c_int
                {
                    return 2 as ::core::ffi::c_int;
                }
            } else if !(*pB).u.zToken.is_null()
                && (*pA).op as ::core::ffi::c_int != TK_COLUMN
                && (*pA).op as ::core::ffi::c_int != TK_AGG_COLUMN
                && strcmp((*pA).u.zToken, (*pB).u.zToken) != 0 as ::core::ffi::c_int
            {
                return 2 as ::core::ffi::c_int
            }
        }
        if (*pA).flags as ::core::ffi::c_uint
            & (EP_Distinct | EP_Commuted) as ::core::ffi::c_uint
            != (*pB).flags as ::core::ffi::c_uint
                & (EP_Distinct | EP_Commuted) as ::core::ffi::c_uint
        {
            return 2 as ::core::ffi::c_int;
        }
        if combinedFlags as ::core::ffi::c_uint
            & 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            if combinedFlags as ::core::ffi::c_uint & EP_xIsSelect as ::core::ffi::c_uint
                != 0
            {
                return 2 as ::core::ffi::c_int;
            }
            if combinedFlags as ::core::ffi::c_uint & EP_FixedCol as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
                && sqlite3ExprCompare(pParse, (*pA).pLeft, (*pB).pLeft, iTab) != 0
            {
                return 2 as ::core::ffi::c_int;
            }
            if sqlite3ExprCompare(pParse, (*pA).pRight, (*pB).pRight, iTab) != 0 {
                return 2 as ::core::ffi::c_int;
            }
            if sqlite3ExprListCompare((*pA).x.pList, (*pB).x.pList, iTab) != 0 {
                return 2 as ::core::ffi::c_int;
            }
            if (*pA).op as ::core::ffi::c_int != TK_STRING
                && (*pA).op as ::core::ffi::c_int != TK_TRUEFALSE
                && combinedFlags as ::core::ffi::c_uint & 0x4000 as ::core::ffi::c_uint
                    == 0 as ::core::ffi::c_uint
            {
                if (*pA).iColumn as ::core::ffi::c_int
                    != (*pB).iColumn as ::core::ffi::c_int
                {
                    return 2 as ::core::ffi::c_int;
                }
                if (*pA).op2 as ::core::ffi::c_int != (*pB).op2 as ::core::ffi::c_int
                    && (*pA).op as ::core::ffi::c_int == TK_TRUTH
                {
                    return 2 as ::core::ffi::c_int;
                }
                if (*pA).op as ::core::ffi::c_int != TK_IN
                    && (*pA).iTable != (*pB).iTable && (*pA).iTable != iTab
                {
                    return 2 as ::core::ffi::c_int;
                }
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprListCompare(
    mut pA: *const ExprList,
    mut pB: *const ExprList,
    mut iTab: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if pA.is_null() && pB.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if pA.is_null() || pB.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        if (*pA).nExpr != (*pB).nExpr {
            return 1 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pA).nExpr {
            let mut res: ::core::ffi::c_int = 0;
            let mut pExprA: *mut Expr = (*(&raw const (*pA).a as *const ExprList_item)
                .offset(i as isize))
                .pExpr;
            let mut pExprB: *mut Expr = (*(&raw const (*pB).a as *const ExprList_item)
                .offset(i as isize))
                .pExpr;
            if (*(&raw const (*pA).a as *const ExprList_item).offset(i as isize))
                .fg
                .sortFlags as ::core::ffi::c_int
                != (*(&raw const (*pB).a as *const ExprList_item).offset(i as isize))
                    .fg
                    .sortFlags as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            res = sqlite3ExprCompare(::core::ptr::null::<Parse>(), pExprA, pExprB, iTab);
            if res != 0 {
                return res;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCompareSkip(
    mut pA: *mut Expr,
    mut pB: *mut Expr,
    mut iTab: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3ExprCompare(
            ::core::ptr::null::<Parse>(),
            sqlite3ExprSkipCollate(pA),
            sqlite3ExprSkipCollate(pB),
            iTab,
        );
    }
}
unsafe extern "C" fn exprImpliesNotNull(
    mut pParse: *const Parse,
    mut p: *const Expr,
    mut pNN: *const Expr,
    mut iTab: ::core::ffi::c_int,
    mut seenNot: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3ExprCompare(pParse, p, pNN, iTab) == 0 as ::core::ffi::c_int {
            return ((*pNN).op as ::core::ffi::c_int != TK_NULL) as ::core::ffi::c_int;
        }
        's_167: {
            let mut c2rust_current_block_36: u64;
            match (*p).op as ::core::ffi::c_int {
                TK_IN => {
                    if seenNot != 0
                        && (*p).flags & 0x1000 as ::core::ffi::c_int as u32_0
                            != 0 as ::core::ffi::c_uint
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    return exprImpliesNotNull(
                        pParse,
                        (*p).pLeft,
                        pNN,
                        iTab,
                        1 as ::core::ffi::c_int,
                    );
                }
                TK_BETWEEN => {
                    let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
                    pList = (*p).x.pList;
                    if seenNot != 0 {
                        return 0 as ::core::ffi::c_int;
                    }
                    if exprImpliesNotNull(
                        pParse,
                        (*(&raw mut (*pList).a as *mut ExprList_item)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .pExpr,
                        pNN,
                        iTab,
                        1 as ::core::ffi::c_int,
                    ) != 0
                        || exprImpliesNotNull(
                            pParse,
                            (*(&raw mut (*pList).a as *mut ExprList_item)
                                .offset(1 as ::core::ffi::c_int as isize))
                                .pExpr,
                            pNN,
                            iTab,
                            1 as ::core::ffi::c_int,
                        ) != 0
                    {
                        return 1 as ::core::ffi::c_int;
                    }
                    return exprImpliesNotNull(
                        pParse,
                        (*p).pLeft,
                        pNN,
                        iTab,
                        1 as ::core::ffi::c_int,
                    );
                }
                TK_EQ
                | TK_NE
                | TK_LT
                | TK_LE
                | TK_GT
                | TK_GE
                | TK_PLUS
                | TK_MINUS
                | TK_BITOR
                | TK_LSHIFT
                | TK_RSHIFT
                | TK_CONCAT => {
                    seenNot = 1 as ::core::ffi::c_int;
                    c2rust_current_block_36 = 629688519540168457;
                }
                TK_STAR | TK_REM | TK_BITAND | TK_SLASH => {
                    c2rust_current_block_36 = 629688519540168457;
                }
                TK_SPAN | TK_COLLATE | TK_UPLUS | TK_UMINUS => {
                    c2rust_current_block_36 = 16373565294044275823;
                }
                TK_TRUTH => {
                    if seenNot != 0 {
                        return 0 as ::core::ffi::c_int;
                    }
                    if (*p).op2 as ::core::ffi::c_int != TK_IS {
                        return 0 as ::core::ffi::c_int;
                    }
                    return exprImpliesNotNull(
                        pParse,
                        (*p).pLeft,
                        pNN,
                        iTab,
                        1 as ::core::ffi::c_int,
                    );
                }
                TK_BITNOT | TK_NOT => {
                    return exprImpliesNotNull(
                        pParse,
                        (*p).pLeft,
                        pNN,
                        iTab,
                        1 as ::core::ffi::c_int,
                    );
                }
                _ => {
                    break 's_167;
                }
            }
            match c2rust_current_block_36 {
                629688519540168457 => {
                    if exprImpliesNotNull(pParse, (*p).pRight, pNN, iTab, seenNot) != 0 {
                        return 1 as ::core::ffi::c_int;
                    }
                }
                _ => {}
            }
            return exprImpliesNotNull(pParse, (*p).pLeft, pNN, iTab, seenNot);
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn sqlite3ExprIsNotTrue(mut pExpr: *mut Expr) -> ::core::ffi::c_int {
    unsafe {
        let mut v: ::core::ffi::c_int = 0;
        if (*pExpr).op as ::core::ffi::c_int == TK_NULL {
            return 1 as ::core::ffi::c_int;
        }
        if (*pExpr).op as ::core::ffi::c_int == TK_TRUEFALSE
            && sqlite3ExprTruthValue(pExpr) == 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        v = 1 as ::core::ffi::c_int;
        if sqlite3ExprIsInteger(pExpr, &raw mut v, ::core::ptr::null_mut::<Parse>()) != 0
            && v == 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn sqlite3ExprIsIIF(
    mut db: *mut sqlite3,
    mut pExpr: *const Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        if (*pExpr).op as ::core::ffi::c_int == TK_FUNCTION {
            let mut z: *const ::core::ffi::c_char = (*pExpr).u.zToken;
            let mut pDef: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
            if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'i' as i32
                && *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 'I' as i32
            {
                return 0 as ::core::ffi::c_int;
            }
            if (*pExpr).x.pList.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            pDef = sqlite3FindFunction(
                db,
                z,
                (*(*pExpr).x.pList).nExpr,
                (*db).enc,
                0 as u8_0,
            );
            if pDef.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if (*pDef).funcFlags as ::core::ffi::c_uint
                & SQLITE_FUNC_INLINE as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            {
                return 0 as ::core::ffi::c_int;
            }
            if (*pDef).pUserData as ::core::ffi::c_long as ::core::ffi::c_int
                != INLINEFUNC_iif
            {
                return 0 as ::core::ffi::c_int;
            }
        } else if (*pExpr).op as ::core::ffi::c_int == TK_CASE {
            if !(*pExpr).pLeft.is_null() {
                return 0 as ::core::ffi::c_int;
            }
        } else {
            return 0 as ::core::ffi::c_int
        }
        pList = (*pExpr).x.pList;
        if (*pList).nExpr == 2 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if (*pList).nExpr == 3 as ::core::ffi::c_int
            && sqlite3ExprIsNotTrue(
                (*(&raw mut (*pList).a as *mut ExprList_item)
                    .offset(2 as ::core::ffi::c_int as isize))
                    .pExpr,
            ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprImpliesExpr(
    mut pParse: *const Parse,
    mut pE1: *const Expr,
    mut pE2: *const Expr,
    mut iTab: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3ExprCompare(pParse, pE1, pE2, iTab) == 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if (*pE2).op as ::core::ffi::c_int == TK_OR
            && (sqlite3ExprImpliesExpr(pParse, pE1, (*pE2).pLeft, iTab) != 0
                || sqlite3ExprImpliesExpr(pParse, pE1, (*pE2).pRight, iTab) != 0)
        {
            return 1 as ::core::ffi::c_int;
        }
        if (*pE2).op as ::core::ffi::c_int == TK_NOTNULL
            && exprImpliesNotNull(
                pParse,
                pE1,
                (*pE2).pLeft,
                iTab,
                0 as ::core::ffi::c_int,
            ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        if sqlite3ExprIsIIF((*pParse).db, pE1) != 0 {
            return sqlite3ExprImpliesExpr(
                pParse,
                (*(&raw mut (*(*pE1).x.pList).a as *mut ExprList_item)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .pExpr,
                pE2,
                iTab,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn bothImplyNotNullRow(
    mut pWalker: *mut Walker,
    mut pE1: *mut Expr,
    mut pE2: *mut Expr,
) {
    unsafe {
        if (*pWalker).eCode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            sqlite3WalkExpr(pWalker, pE1);
            if (*pWalker).eCode != 0 {
                (*pWalker).eCode = 0 as u16_0;
                sqlite3WalkExpr(pWalker, pE2);
            }
        }
    }
}
unsafe extern "C" fn impliesNotNullRow(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).flags & 0x1 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint
        {
            return WRC_Prune;
        }
        if (*pExpr).flags & 0x2 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint && (*pWalker).mWFlags as ::core::ffi::c_int != 0
        {
            return WRC_Prune;
        }
        match (*pExpr).op as ::core::ffi::c_int {
            TK_ISNOT
            | TK_ISNULL
            | TK_NOTNULL
            | TK_IS
            | TK_VECTOR
            | TK_FUNCTION
            | TK_TRUTH
            | TK_CASE => return WRC_Prune,
            TK_COLUMN => {
                if (*pWalker).u.iCur == (*pExpr).iTable {
                    (*pWalker).eCode = 1 as u16_0;
                    return WRC_Abort;
                }
                return WRC_Prune;
            }
            TK_OR | TK_AND => {
                bothImplyNotNullRow(pWalker, (*pExpr).pLeft, (*pExpr).pRight);
                return WRC_Prune;
            }
            TK_IN => {
                if (*pExpr).flags as ::core::ffi::c_uint
                    & EP_xIsSelect as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                    && (*(*pExpr).x.pList).nExpr > 0 as ::core::ffi::c_int
                {
                    sqlite3WalkExpr(pWalker, (*pExpr).pLeft);
                }
                return WRC_Prune;
            }
            TK_BETWEEN => {
                sqlite3WalkExpr(pWalker, (*pExpr).pLeft);
                bothImplyNotNullRow(
                    pWalker,
                    (*(&raw mut (*(*pExpr).x.pList).a as *mut ExprList_item)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .pExpr,
                    (*(&raw mut (*(*pExpr).x.pList).a as *mut ExprList_item)
                        .offset(1 as ::core::ffi::c_int as isize))
                        .pExpr,
                );
                return WRC_Prune;
            }
            TK_EQ | TK_NE | TK_LT | TK_LE | TK_GT | TK_GE => {
                let mut pLeft: *mut Expr = (*pExpr).pLeft;
                let mut pRight: *mut Expr = (*pExpr).pRight;
                if (*pLeft).op as ::core::ffi::c_int == TK_COLUMN
                    && !(*pLeft).y.pTab.is_null()
                    && (*(*pLeft).y.pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB
                    || (*pRight).op as ::core::ffi::c_int == TK_COLUMN
                        && !(*pRight).y.pTab.is_null()
                        && (*(*pRight).y.pTab).eTabType as ::core::ffi::c_int
                            == TABTYP_VTAB
                {
                    return WRC_Prune;
                }
            }
            _ => {}
        }
        return WRC_Continue;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprImpliesNonNullRow(
    mut p: *mut Expr,
    mut iTab: ::core::ffi::c_int,
    mut isRJ: ::core::ffi::c_int,
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
        p = sqlite3ExprSkipCollateAndLikely(p);
        if p.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*p).op as ::core::ffi::c_int == TK_NOTNULL {
            p = (*p).pLeft;
        } else {
            while (*p).op as ::core::ffi::c_int == TK_AND {
                if sqlite3ExprImpliesNonNullRow((*p).pLeft, iTab, isRJ) != 0 {
                    return 1 as ::core::ffi::c_int;
                }
                p = (*p).pRight;
            }
        }
        w.xExprCallback = Some(
            impliesNotNullRow
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = None;
        w.xSelectCallback2 = None;
        w.eCode = 0 as u16_0;
        w.mWFlags = (isRJ != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as u16_0;
        w.u.iCur = iTab;
        sqlite3WalkExpr(&raw mut w, p);
        return w.eCode as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn exprIdxCover(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int == TK_COLUMN
            && (*pExpr).iTable == (*(*pWalker).u.pIdxCover).iCur
            && sqlite3TableColumnToIndex(
                (*(*pWalker).u.pIdxCover).pIdx,
                (*pExpr).iColumn as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
        {
            (*pWalker).eCode = 1 as u16_0;
            return WRC_Abort;
        }
        return WRC_Continue;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprCoveredByIndex(
    mut pExpr: *mut Expr,
    mut iCur: ::core::ffi::c_int,
    mut pIdx: *mut Index,
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
        let mut xcov: IdxCover = IdxCover {
            pIdx: ::core::ptr::null_mut::<Index>(),
            iCur: 0

        };
        memset(
            &raw mut w as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        xcov.iCur = iCur;
        xcov.pIdx = pIdx;
        w.xExprCallback = Some(
            exprIdxCover
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.u.pIdxCover = &raw mut xcov as *mut IdxCover;
        sqlite3WalkExpr(&raw mut w, pExpr);
        return (w.eCode == 0) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn selectRefEnter(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut RefSrcList = (*pWalker).u.pRefSrcList as *mut RefSrcList;
        let mut pSrc: *mut SrcList = (*pSelect).pSrc;
        let mut i: i64_0 = 0;
        let mut j: i64_0 = 0;
        let mut piNew: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        if (*pSrc).nSrc == 0 as ::core::ffi::c_int {
            return WRC_Continue;
        }
        j = (*p).nExclude;
        (*p).nExclude += (*pSrc).nSrc as ::core::ffi::c_longlong;
        piNew = sqlite3DbRealloc(
            (*p).db,
            (*p).aiExclude as *mut ::core::ffi::c_void,
            ((*p).nExclude as u64_0)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as u64_0),
        ) as *mut ::core::ffi::c_int;
        if piNew.is_null() {
            (*p).nExclude = 0 as i64_0;
            return WRC_Abort;
        } else {
            (*p).aiExclude = piNew;
        }
        i = 0 as i64_0;
        while i < (*pSrc).nSrc as ::core::ffi::c_longlong {
            *(*p).aiExclude.offset(j as isize) = (*(&raw mut (*pSrc).a as *mut SrcItem)
                .offset(i as isize))
                .iCursor;
            i += 1;
            j += 1;
        }
        return WRC_Continue;
    }
}
unsafe extern "C" fn selectRefLeave(mut pWalker: *mut Walker, mut pSelect: *mut Select) {
    unsafe {
        let mut p: *mut RefSrcList = (*pWalker).u.pRefSrcList as *mut RefSrcList;
        let mut pSrc: *mut SrcList = (*pSelect).pSrc;
        if (*p).nExclude != 0 {
            (*p).nExclude -= (*pSrc).nSrc as ::core::ffi::c_longlong;
        }
    }
}
unsafe extern "C" fn exprRefToSrcList(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pExpr).op as ::core::ffi::c_int == TK_COLUMN
            || (*pExpr).op as ::core::ffi::c_int == TK_AGG_COLUMN
        {
            let mut i: ::core::ffi::c_int = 0;
            let mut p: *mut RefSrcList = (*pWalker).u.pRefSrcList as *mut RefSrcList;
            let mut pSrc: *mut SrcList = (*p).pRef;
            let mut nSrc: ::core::ffi::c_int = if !pSrc.is_null() {
                (*pSrc).nSrc
            } else {
                0 as ::core::ffi::c_int
            };
            i = 0 as ::core::ffi::c_int;
            while i < nSrc {
                if (*pExpr).iTable
                    == (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize)).iCursor
                {
                    (*pWalker).eCode = ((*pWalker).eCode as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_int) as u16_0;
                    return WRC_Continue;
                }
                i += 1;
            }
            i = 0 as ::core::ffi::c_int;
            while (i as ::core::ffi::c_longlong) < (*p).nExclude
                && *(*p).aiExclude.offset(i as isize) != (*pExpr).iTable
            {
                i += 1;
            }
            if i as ::core::ffi::c_longlong >= (*p).nExclude {
                (*pWalker).eCode = ((*pWalker).eCode as ::core::ffi::c_int
                    | 2 as ::core::ffi::c_int) as u16_0;
            }
        }
        return WRC_Continue;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ReferencesSrcList(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pSrcList: *mut SrcList,
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
        let mut x: RefSrcList = RefSrcList {
            db: ::core::ptr::null_mut::<sqlite3>(),
            pRef: ::core::ptr::null_mut::<SrcList>(),
            nExclude: 0,
            aiExclude: ::core::ptr::null_mut::<::core::ffi::c_int>()

        };
        memset(
            &raw mut w as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        memset(
            &raw mut x as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<RefSrcList>() as size_t,
        );
        w.xExprCallback = Some(
            exprRefToSrcList
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = Some(
            selectRefEnter
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        w.xSelectCallback2 = Some(
            selectRefLeave as unsafe extern "C" fn(*mut Walker, *mut Select) -> (),
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Select) -> ()>;
        w.u.pRefSrcList = &raw mut x as *mut RefSrcList;
        x.db = (*pParse).db;
        x.pRef = pSrcList;
        sqlite3WalkExprList(&raw mut w, (*pExpr).x.pList);
        if !(*pExpr).pLeft.is_null() {
            sqlite3WalkExprList(&raw mut w, (*(*pExpr).pLeft).x.pList);
        }
        if (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as u32_0
            != 0 as ::core::ffi::c_uint
        {
            sqlite3WalkExpr(&raw mut w, (*(*pExpr).y.pWin).pFilter);
        }
        if !x.aiExclude.is_null() {
            sqlite3DbNNFreeNN((*pParse).db, x.aiExclude as *mut ::core::ffi::c_void);
        }
        if w.eCode as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
            return 1 as ::core::ffi::c_int
        } else if w.eCode != 0 {
            return 0 as ::core::ffi::c_int
        } else {
            return -1 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn agginfoPersistExprCb(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        if !((*pExpr).flags
            & (0x10000 as ::core::ffi::c_int | 0x4000 as ::core::ffi::c_int) as u32_0
            != 0 as ::core::ffi::c_uint) && !(*pExpr).pAggInfo.is_null()
        {
            let mut pAggInfo: *mut AggInfo = (*pExpr).pAggInfo;
            let mut iAgg: ::core::ffi::c_int = (*pExpr).iAgg as ::core::ffi::c_int;
            let mut pParse: *mut Parse = (*pWalker).pParse;
            let mut db: *mut sqlite3 = (*pParse).db;
            if (*pExpr).op as ::core::ffi::c_int != TK_AGG_FUNCTION {
                if iAgg < (*pAggInfo).nColumn
                    && (*(*pAggInfo).aCol.offset(iAgg as isize)).pCExpr == pExpr
                {
                    pExpr = sqlite3ExprDup(db, pExpr, 0 as ::core::ffi::c_int);
                    if !pExpr.is_null() && sqlite3ExprDeferredDelete(pParse, pExpr) == 0
                    {
                        let ref mut c2rust_fresh11 = (*(*pAggInfo)
                            .aCol
                            .offset(iAgg as isize))
                            .pCExpr;
                        *c2rust_fresh11 = pExpr;
                    }
                }
            } else if iAgg < (*pAggInfo).nFunc
                && (*(*pAggInfo).aFunc.offset(iAgg as isize)).pFExpr == pExpr
            {
                pExpr = sqlite3ExprDup(db, pExpr, 0 as ::core::ffi::c_int);
                if !pExpr.is_null() && sqlite3ExprDeferredDelete(pParse, pExpr) == 0 {
                    let ref mut c2rust_fresh12 = (*(*pAggInfo)
                        .aFunc
                        .offset(iAgg as isize))
                        .pFExpr;
                    *c2rust_fresh12 = pExpr;
                }
            }
        }
        return WRC_Continue;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AggInfoPersistWalkerInit(
    mut pWalker: *mut Walker,
    mut pParse: *mut Parse,
) {
    unsafe {
        memset(
            pWalker as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Walker>() as size_t,
        );
        (*pWalker).pParse = pParse;
        (*pWalker).xExprCallback = Some(
            agginfoPersistExprCb
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        (*pWalker).xSelectCallback = Some(
            sqlite3SelectWalkNoop
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
    }
}
unsafe extern "C" fn addAggInfoColumn(
    mut db: *mut sqlite3,
    mut pInfo: *mut AggInfo,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        (*pInfo).aCol = sqlite3ArrayAllocate(
            db,
            (*pInfo).aCol as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<AggInfo_col>() as ::core::ffi::c_int,
            &raw mut (*pInfo).nColumn,
            &raw mut i,
        ) as *mut AggInfo_col;
        return i;
    }
}
unsafe extern "C" fn addAggInfoFunc(
    mut db: *mut sqlite3,
    mut pInfo: *mut AggInfo,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        (*pInfo).aFunc = sqlite3ArrayAllocate(
            db,
            (*pInfo).aFunc as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<AggInfo_func>() as ::core::ffi::c_int,
            &raw mut (*pInfo).nFunc,
            &raw mut i,
        ) as *mut AggInfo_func;
        return i;
    }
}
unsafe extern "C" fn findOrCreateAggInfoColumn(
    mut pParse: *mut Parse,
    mut pAggInfo: *mut AggInfo,
    mut pExpr: *mut Expr,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pCol: *mut AggInfo_col = ::core::ptr::null_mut::<AggInfo_col>();
        let mut k: ::core::ffi::c_int = 0;
        let mut mxTerm: ::core::ffi::c_int = (*(*pParse).db)
            .aLimit[SQLITE_LIMIT_COLUMN as usize];
        pCol = (*pAggInfo).aCol as *mut AggInfo_col;
        k = 0 as ::core::ffi::c_int;
        loop {
            if !(k < (*pAggInfo).nColumn) {
                c2rust_current_block = 14523784380283086299;
                break;
            }
            if (*pCol).pCExpr == pExpr {
                return;
            }
            if (*pCol).iTable == (*pExpr).iTable
                && (*pCol).iColumn == (*pExpr).iColumn as ::core::ffi::c_int
                && (*pExpr).op as ::core::ffi::c_int != TK_IF_NULL_ROW
            {
                c2rust_current_block = 17281240262373992796;
                break;
            }
            k += 1;
            pCol = pCol.offset(1);
        }
        match c2rust_current_block {
            14523784380283086299 => {
                k = addAggInfoColumn((*pParse).db, pAggInfo);
                if k < 0 as ::core::ffi::c_int {
                    return;
                }
                if k > mxTerm {
                    sqlite3ErrorMsg(
                        pParse,
                        b"more than %d aggregate terms\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        mxTerm,
                    );
                    k = mxTerm;
                }
                pCol = (*pAggInfo).aCol.offset(k as isize) as *mut AggInfo_col
                    as *mut AggInfo_col;
                (*pCol).pTab = (*pExpr).y.pTab;
                (*pCol).iTable = (*pExpr).iTable;
                (*pCol).iColumn = (*pExpr).iColumn as ::core::ffi::c_int;
                (*pCol).iSorterColumn = -1 as ::core::ffi::c_int;
                (*pCol).pCExpr = pExpr;
                if !(*pAggInfo).pGroupBy.is_null()
                    && (*pExpr).op as ::core::ffi::c_int != TK_IF_NULL_ROW
                {
                    let mut j: ::core::ffi::c_int = 0;
                    let mut n: ::core::ffi::c_int = 0;
                    let mut pGB: *mut ExprList = (*pAggInfo).pGroupBy;
                    let mut pTerm: *mut ExprList_item = &raw mut (*pGB).a
                        as *mut ExprList_item;
                    n = (*pGB).nExpr;
                    j = 0 as ::core::ffi::c_int;
                    while j < n {
                        let mut pE: *mut Expr = (*pTerm).pExpr;
                        if (*pE).op as ::core::ffi::c_int == TK_COLUMN
                            && (*pE).iTable == (*pExpr).iTable
                            && (*pE).iColumn as ::core::ffi::c_int
                                == (*pExpr).iColumn as ::core::ffi::c_int
                        {
                            (*pCol).iSorterColumn = j;
                            break;
                        } else {
                            j += 1;
                            pTerm = pTerm.offset(1);
                        }
                    }
                }
                if (*pCol).iSorterColumn < 0 as ::core::ffi::c_int {
                    let c2rust_fresh16 = (*pAggInfo).nSortingColumn;
                    (*pAggInfo).nSortingColumn = (*pAggInfo)
                        .nSortingColumn
                        .wrapping_add(1);
                    (*pCol).iSorterColumn = c2rust_fresh16 as ::core::ffi::c_int;
                }
            }
            _ => {}
        }
        (*pExpr).pAggInfo = pAggInfo;
        if (*pExpr).op as ::core::ffi::c_int == TK_COLUMN {
            (*pExpr).op = TK_AGG_COLUMN as u8_0;
        }
        (*pExpr).iAgg = k as i16_0;
    }
}
unsafe extern "C" fn analyzeAggregate(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pNC: *mut NameContext = (*pWalker).u.pNC;
        let mut pParse: *mut Parse = (*pNC).pParse;
        let mut pSrcList: *mut SrcList = (*pNC).pSrcList;
        let mut pAggInfo: *mut AggInfo = (*pNC).uNC.pAggInfo;
        match (*pExpr).op as ::core::ffi::c_int {
            TK_IF_NULL_ROW | TK_AGG_COLUMN | TK_COLUMN => {
                if !pSrcList.is_null() {
                    let mut pItem: *mut SrcItem = &raw mut (*pSrcList).a as *mut SrcItem;
                    i = 0 as ::core::ffi::c_int;
                    while i < (*pSrcList).nSrc {
                        if (*pExpr).iTable == (*pItem).iCursor {
                            findOrCreateAggInfoColumn(pParse, pAggInfo, pExpr);
                            break;
                        } else {
                            i += 1;
                            pItem = pItem.offset(1);
                        }
                    }
                }
                return WRC_Continue;
            }
            TK_AGG_FUNCTION => {
                if (*pNC).ncFlags & NC_InAggFunc == 0 as ::core::ffi::c_int
                    && (*pWalker).walkerDepth == (*pExpr).op2 as ::core::ffi::c_int
                    && (*pExpr).pAggInfo.is_null()
                {
                    let mut pItem_0: *mut AggInfo_func = (*pAggInfo).aFunc
                        as *mut AggInfo_func;
                    let mut mxTerm: ::core::ffi::c_int = (*(*pParse).db)
                        .aLimit[SQLITE_LIMIT_COLUMN as usize];
                    i = 0 as ::core::ffi::c_int;
                    while i < (*pAggInfo).nFunc {
                        if (*pItem_0).pFExpr == pExpr {
                            break;
                        }
                        if sqlite3ExprCompare(
                            ::core::ptr::null::<Parse>(),
                            (*pItem_0).pFExpr,
                            pExpr,
                            -1 as ::core::ffi::c_int,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        i += 1;
                        pItem_0 = pItem_0.offset(1);
                    }
                    if i > mxTerm {
                        sqlite3ErrorMsg(
                            pParse,
                            b"more than %d aggregate terms\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            mxTerm,
                        );
                        i = mxTerm;
                    } else if i >= (*pAggInfo).nFunc {
                        let mut enc: u8_0 = (*(*pParse).db).enc;
                        i = addAggInfoFunc((*pParse).db, pAggInfo);
                        if i >= 0 as ::core::ffi::c_int {
                            let mut nArg: ::core::ffi::c_int = 0;
                            pItem_0 = (*pAggInfo).aFunc.offset(i as isize)
                                as *mut AggInfo_func as *mut AggInfo_func;
                            (*pItem_0).pFExpr = pExpr;
                            nArg = if !(*pExpr).x.pList.is_null() {
                                (*(*pExpr).x.pList).nExpr
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            (*pItem_0).pFunc = sqlite3FindFunction(
                                (*pParse).db,
                                (*pExpr).u.zToken,
                                nArg,
                                enc,
                                0 as u8_0,
                            );
                            if !(*pExpr).pLeft.is_null()
                                && (*(*pItem_0).pFunc).funcFlags as ::core::ffi::c_uint
                                    & SQLITE_FUNC_NEEDCOLL as ::core::ffi::c_uint
                                    == 0 as ::core::ffi::c_uint
                            {
                                let mut pOBList: *mut ExprList = ::core::ptr::null_mut::<
                                    ExprList,
                                >();
                                let c2rust_fresh14 = (*pParse).nTab;
                                (*pParse).nTab = (*pParse).nTab + 1;
                                (*pItem_0).iOBTab = c2rust_fresh14;
                                pOBList = (*(*pExpr).pLeft).x.pList;
                                if (*pOBList).nExpr == 1 as ::core::ffi::c_int
                                    && nArg == 1 as ::core::ffi::c_int
                                    && sqlite3ExprCompare(
                                        ::core::ptr::null::<Parse>(),
                                        (*(&raw mut (*pOBList).a as *mut ExprList_item)
                                            .offset(0 as ::core::ffi::c_int as isize))
                                            .pExpr,
                                        (*(&raw mut (*(*pExpr).x.pList).a as *mut ExprList_item)
                                            .offset(0 as ::core::ffi::c_int as isize))
                                            .pExpr,
                                        0 as ::core::ffi::c_int,
                                    ) == 0 as ::core::ffi::c_int
                                {
                                    (*pItem_0).bOBPayload = 0 as u8_0;
                                    (*pItem_0).bOBUnique = ((*pExpr).flags
                                        & 0x4 as ::core::ffi::c_int as u32_0
                                        != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as u8_0;
                                } else {
                                    (*pItem_0).bOBPayload = 1 as u8_0;
                                }
                                (*pItem_0).bUseSubtype = ((*(*pItem_0).pFunc).funcFlags
                                    as ::core::ffi::c_uint
                                    & SQLITE_SUBTYPE as ::core::ffi::c_uint
                                    != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as u8_0;
                            } else {
                                (*pItem_0).iOBTab = -1 as ::core::ffi::c_int;
                            }
                            if (*pExpr).flags & 0x4 as ::core::ffi::c_int as u32_0
                                != 0 as ::core::ffi::c_uint && (*pItem_0).bOBUnique == 0
                            {
                                let c2rust_fresh15 = (*pParse).nTab;
                                (*pParse).nTab = (*pParse).nTab + 1;
                                (*pItem_0).iDistinct = c2rust_fresh15;
                            } else {
                                (*pItem_0).iDistinct = -1 as ::core::ffi::c_int;
                            }
                        }
                    }
                    (*pExpr).iAgg = i as i16_0;
                    (*pExpr).pAggInfo = pAggInfo;
                    return WRC_Prune;
                } else {
                    return WRC_Continue
                }
            }
            _ => {
                let mut pIEpr: *mut IndexedExpr = ::core::ptr::null_mut::<IndexedExpr>();
                let mut tmp: Expr = Expr {
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
                if !((*pNC).ncFlags & NC_InAggFunc == 0 as ::core::ffi::c_int) {
                    if !(*pParse).pIdxEpr.is_null() {
                        pIEpr = (*pParse).pIdxEpr;
                        while !pIEpr.is_null() {
                            let mut iDataCur: ::core::ffi::c_int = (*pIEpr).iDataCur;
                            if !(iDataCur < 0 as ::core::ffi::c_int) {
                                if sqlite3ExprCompare(
                                    ::core::ptr::null::<Parse>(),
                                    pExpr,
                                    (*pIEpr).pExpr,
                                    iDataCur,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            pIEpr = (*pIEpr).pIENext;
                        }
                        if !pIEpr.is_null() {
                            if (*pExpr).flags as ::core::ffi::c_uint
                                & (0x1000000 as ::core::ffi::c_int
                                    | 0x2000000 as ::core::ffi::c_int) as ::core::ffi::c_uint
                                == 0 as ::core::ffi::c_uint
                            {
                                i = 0 as ::core::ffi::c_int;
                                while i < (*pSrcList).nSrc {
                                    if (*(&raw mut (*pSrcList).a as *mut SrcItem)
                                        .offset(0 as ::core::ffi::c_int as isize))
                                        .iCursor == (*pIEpr).iDataCur
                                    {
                                        break;
                                    }
                                    i += 1;
                                }
                                if !(i >= (*pSrcList).nSrc) {
                                    if (*pExpr).pAggInfo.is_null() {
                                        if (*pParse).nErr != 0 {
                                            return WRC_Abort;
                                        }
                                        memset(
                                            &raw mut tmp as *mut ::core::ffi::c_void,
                                            0 as ::core::ffi::c_int,
                                            ::core::mem::size_of::<Expr>() as size_t,
                                        );
                                        tmp.op = TK_AGG_COLUMN as u8_0;
                                        tmp.iTable = (*pIEpr).iIdxCur;
                                        tmp.iColumn = (*pIEpr).iIdxCol as ynVar;
                                        findOrCreateAggInfoColumn(pParse, pAggInfo, &raw mut tmp);
                                        if (*pParse).nErr != 0 {
                                            return WRC_Abort;
                                        }
                                        let ref mut c2rust_fresh13 = (*(*pAggInfo)
                                            .aCol
                                            .offset(tmp.iAgg as isize))
                                            .pCExpr;
                                        *c2rust_fresh13 = pExpr;
                                        (*pExpr).pAggInfo = pAggInfo;
                                        (*pExpr).iAgg = tmp.iAgg;
                                        return WRC_Prune;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return WRC_Continue;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprAnalyzeAggregates(
    mut pNC: *mut NameContext,
    mut pExpr: *mut Expr,
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
            analyzeAggregate
                as unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int>;
        w.xSelectCallback = Some(
            sqlite3WalkerDepthIncrease
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
            >;
        w.xSelectCallback2 = Some(
            sqlite3WalkerDepthDecrease
                as unsafe extern "C" fn(*mut Walker, *mut Select) -> (),
        ) as Option<unsafe extern "C" fn(*mut Walker, *mut Select) -> ()>;
        w.walkerDepth = 0 as ::core::ffi::c_int;
        w.u.pNC = pNC;
        w.pParse = ::core::ptr::null_mut::<Parse>();
        sqlite3WalkExpr(&raw mut w, pExpr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExprAnalyzeAggList(
    mut pNC: *mut NameContext,
    mut pList: *mut ExprList,
) {
    unsafe {
        let mut pItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        let mut i: ::core::ffi::c_int = 0;
        if !pList.is_null() {
            pItem = &raw mut (*pList).a as *mut ExprList_item as *mut ExprList_item;
            i = 0 as ::core::ffi::c_int;
            while i < (*pList).nExpr {
                sqlite3ExprAnalyzeAggregates(pNC, (*pItem).pExpr);
                i += 1;
                pItem = pItem.offset(1);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GetTempReg(
    mut pParse: *mut Parse,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pParse).nTempReg as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*pParse).nMem += 1;
            return (*pParse).nMem;
        }
        (*pParse).nTempReg = (*pParse).nTempReg.wrapping_sub(1);
        return (*pParse).aTempReg[(*pParse).nTempReg as usize];
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ReleaseTempReg(
    mut pParse: *mut Parse,
    mut iReg: ::core::ffi::c_int,
) {
    unsafe {
        if iReg != 0 {
            if ((*pParse).nTempReg as ::core::ffi::c_int)
                < (::core::mem::size_of::<[::core::ffi::c_int; 8]>() as usize)
                    .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                    as ::core::ffi::c_int
            {
                let c2rust_fresh0 = (*pParse).nTempReg;
                (*pParse).nTempReg = (*pParse).nTempReg.wrapping_add(1);
                (*pParse).aTempReg[c2rust_fresh0 as usize] = iReg;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GetTempRange(
    mut pParse: *mut Parse,
    mut nReg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        if nReg == 1 as ::core::ffi::c_int {
            return sqlite3GetTempReg(pParse);
        }
        i = (*pParse).iRangeReg;
        n = (*pParse).nRangeReg;
        if nReg <= n {
            (*pParse).iRangeReg += nReg;
            (*pParse).nRangeReg -= nReg;
        } else {
            i = (*pParse).nMem + 1 as ::core::ffi::c_int;
            (*pParse).nMem += nReg;
        }
        return i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ReleaseTempRange(
    mut pParse: *mut Parse,
    mut iReg: ::core::ffi::c_int,
    mut nReg: ::core::ffi::c_int,
) {
    unsafe {
        if nReg == 1 as ::core::ffi::c_int {
            sqlite3ReleaseTempReg(pParse, iReg);
            return;
        }
        if nReg > (*pParse).nRangeReg {
            (*pParse).nRangeReg = nReg;
            (*pParse).iRangeReg = iReg;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ClearTempRegCache(mut pParse: *mut Parse) {
    unsafe {
        (*pParse).nTempReg = 0 as u8_0;
        (*pParse).nRangeReg = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TouchRegister(
    mut pParse: *mut Parse,
    mut iReg: ::core::ffi::c_int,
) {
    unsafe {
        if (*pParse).nMem < iReg {
            (*pParse).nMem = iReg;
        }
    }
}
pub const LARGEST_INT64: i64_0 = 0xffffffff as i64_0
    | (0x7fffffff as ::core::ffi::c_int as i64_0) << 32 as ::core::ffi::c_int;
pub const SMALLEST_INT64: i64_0 = -1 as ::core::ffi::c_int as i64_0 - LARGEST_INT64;
pub const BMS: ::core::ffi::c_int = (::core::mem::size_of::<Bitmask>() as usize)
    .wrapping_mul(8 as usize) as ::core::ffi::c_int;
pub const P4_STATIC: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const P4_COLLSEQ: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const P4_TABLE: ::core::ffi::c_int = -5 as ::core::ffi::c_int;
pub const P4_DYNAMIC: ::core::ffi::c_int = -6 as ::core::ffi::c_int;
pub const P4_KEYINFO: ::core::ffi::c_int = -8 as ::core::ffi::c_int;
pub const P4_REAL: ::core::ffi::c_int = -12 as ::core::ffi::c_int;
pub const P4_INT64: ::core::ffi::c_int = -13 as ::core::ffi::c_int;
pub const P4_SUBRTNSIG: ::core::ffi::c_int = -17 as ::core::ffi::c_int;
pub const TK_NOT: ::core::ffi::c_int = 19;
pub const TK_EXISTS: ::core::ffi::c_int = 20;
pub const TK_CAST: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const TK_OR: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const TK_AND: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const TK_IS: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const TK_ISNOT: ::core::ffi::c_int = 46;
pub const TK_BETWEEN: ::core::ffi::c_int = 49;
pub const TK_IN: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const TK_ISNULL: ::core::ffi::c_int = 51;
pub const TK_NOTNULL: ::core::ffi::c_int = 52;
pub const TK_NE: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const TK_EQ: ::core::ffi::c_int = 54;
pub const TK_GT: ::core::ffi::c_int = 55;
pub const TK_LE: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const TK_LT: ::core::ffi::c_int = 57;
pub const TK_GE: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
pub const TK_ID: ::core::ffi::c_int = 60;
pub const TK_RAISE: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const TK_TRIGGER: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
pub const TK_NULLS: ::core::ffi::c_int = 83;
pub const TK_BITAND: ::core::ffi::c_int = 103;
pub const TK_BITOR: ::core::ffi::c_int = 104;
pub const TK_LSHIFT: ::core::ffi::c_int = 105;
pub const TK_RSHIFT: ::core::ffi::c_int = 106;
pub const TK_PLUS: ::core::ffi::c_int = 107;
pub const TK_MINUS: ::core::ffi::c_int = 108;
pub const TK_STAR: ::core::ffi::c_int = 109;
pub const TK_SLASH: ::core::ffi::c_int = 110;
pub const TK_REM: ::core::ffi::c_int = 111;
pub const TK_CONCAT: ::core::ffi::c_int = 112;
pub const TK_COLLATE: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const TK_BITNOT: ::core::ffi::c_int = 115;
pub const TK_STRING: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_ALL: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const TK_SELECT: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const TK_DOT: ::core::ffi::c_int = 142;
pub const TK_ORDER: ::core::ffi::c_int = 146 as ::core::ffi::c_int;
pub const TK_LIMIT: ::core::ffi::c_int = 149 as ::core::ffi::c_int;
pub const TK_FLOAT: ::core::ffi::c_int = 154;
pub const TK_BLOB: ::core::ffi::c_int = 155;
pub const TK_INTEGER: ::core::ffi::c_int = 156 as ::core::ffi::c_int;
pub const TK_VARIABLE: ::core::ffi::c_int = 157 as ::core::ffi::c_int;
pub const TK_CASE: ::core::ffi::c_int = 158;
pub const TK_FILTER: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const TK_COLUMN: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
pub const TK_AGG_FUNCTION: ::core::ffi::c_int = 169 as ::core::ffi::c_int;
pub const TK_AGG_COLUMN: ::core::ffi::c_int = 170 as ::core::ffi::c_int;
pub const TK_TRUEFALSE: ::core::ffi::c_int = 171 as ::core::ffi::c_int;
pub const TK_FUNCTION: ::core::ffi::c_int = 172 as ::core::ffi::c_int;
pub const TK_UPLUS: ::core::ffi::c_int = 173 as ::core::ffi::c_int;
pub const TK_UMINUS: ::core::ffi::c_int = 174 as ::core::ffi::c_int;
pub const TK_TRUTH: ::core::ffi::c_int = 175 as ::core::ffi::c_int;
pub const TK_REGISTER: ::core::ffi::c_int = 176 as ::core::ffi::c_int;
pub const TK_VECTOR: ::core::ffi::c_int = 177 as ::core::ffi::c_int;
pub const TK_SELECT_COLUMN: ::core::ffi::c_int = 178 as ::core::ffi::c_int;
pub const TK_IF_NULL_ROW: ::core::ffi::c_int = 179;
pub const TK_SPAN: ::core::ffi::c_int = 181;
pub const TK_ERROR: ::core::ffi::c_int = 182 as ::core::ffi::c_int;
pub const SQLITE_TrustedSchema: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_EnableQPSG: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_NEEDCOLL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_FUNC_LENGTH: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_FUNC_TYPEOF: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_FUNC_CONSTANT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_FUNC_SLOCHNG: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_DIRECT: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_UNSAFE: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SQLITE_FUNC_INLINE: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
pub const INLINEFUNC_coalesce: ::core::ffi::c_int = 0;
pub const INLINEFUNC_implies_nonnull_row: ::core::ffi::c_int = 1;
pub const INLINEFUNC_expr_implies_expr: ::core::ffi::c_int = 2;
pub const INLINEFUNC_expr_compare: ::core::ffi::c_int = 3;
pub const INLINEFUNC_affinity: ::core::ffi::c_int = 4;
pub const INLINEFUNC_iif: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const COLFLAG_VIRTUAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const COLFLAG_NOTAVAIL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const COLFLAG_BUSY: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const COLFLAG_GENERATED: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const SQLITE_SO_ASC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_SO_UNDEFINED: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const SQLITE_AFF_NONE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_AFF_BLOB: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const SQLITE_AFF_TEXT: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const SQLITE_AFF_INTEGER: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;
pub const SQLITE_AFF_REAL: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;
pub const SQLITE_AFF_DEFER: ::core::ffi::c_int = 0x58 as ::core::ffi::c_int;
pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_NULLEQ: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_Strict: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OE_None: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Ignore: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_BIGNULL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const XN_EXPR: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const EP_OuterON: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const EP_InnerON: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const EP_Distinct: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const EP_HasFunc: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const EP_FixedCol: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const EP_Collate: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const EP_Commuted: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const EP_IntValue: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const EP_xIsSelect: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const EP_Skip: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const EP_Reduced: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const EP_TokenOnly: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const EP_Subquery: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
pub const EP_Leaf: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
pub const EP_Static: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const EP_IsTrue: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const EP_IsFalse: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const EP_Propagate: ::core::ffi::c_int = EP_Collate | EP_Subquery | EP_HasFunc;
pub const EXPR_FULLSIZE: usize = ::core::mem::size_of::<Expr>();
pub const EXPR_REDUCEDSIZE: ::core::ffi::c_ulong = 44 as ::core::ffi::c_ulong;
pub const EXPR_TOKENONLYSIZE: ::core::ffi::c_ulong = 16 as ::core::ffi::c_ulong;
pub const EXPRDUP_REDUCE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const ENAME_SPAN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JT_LEFT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const JT_LTORJ: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const NC_InAggFunc: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const SF_Distinct: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SF_All: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SF_Aggregate: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SF_UsesEphemeral: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SF_Values: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SF_MultiValue: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SF_Correlated: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const SRT_Exists: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SRT_Mem: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SRT_Set: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const PARSE_MODE_RENAME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OPFLAG_NOCHNG: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPFLAG_TYPEOFARG: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const OPFLAG_BYTELENARG: ::core::ffi::c_int = 0xc0 as ::core::ffi::c_int;
pub const WRC_Continue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WRC_Prune: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WRC_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_ECEL_DUP: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_ECEL_FACTOR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_ECEL_REF: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_ECEL_OMITREF: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const IN_INDEX_ROWID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IN_INDEX_EPH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const IN_INDEX_INDEX_ASC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const IN_INDEX_INDEX_DESC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const IN_INDEX_NOOP: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const IN_INDEX_NOOP_OK: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const IN_INDEX_MEMBERSHIP: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const IN_INDEX_LOOP: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
