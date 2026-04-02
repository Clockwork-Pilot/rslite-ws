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
    fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
        errmsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_str_appendf(
        _: *mut sqlite3_str,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strglob(
        zGlob: *const ::core::ffi::c_char,
        zStr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strlike(
        zGlob: *const ::core::ffi::c_char,
        zStr: *const ::core::ffi::c_char,
        cEsc: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    fn sqlite3VdbeAddFunctionCall(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const FuncDef,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeChangeP5(_: *mut Vdbe, P5: u16_0);
    fn sqlite3VdbeJumpHere(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeChangeP4(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
        zP4: *const ::core::ffi::c_char,
        N: ::core::ffi::c_int,
    );
    fn sqlite3VdbeSetP4KeyInfo(_: *mut Parse, _: *mut Index);
    fn sqlite3VdbeMakeLabel(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3VdbeResolveLabel(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeCurrentAddr(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3TouchRegister(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3PrimaryKeyIndex(_: *mut Table) -> *mut Index;
    fn sqlite3OpenTable(
        _: *mut Parse,
        iCur: ::core::ffi::c_int,
        iDb: ::core::ffi::c_int,
        _: *mut Table,
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
    fn sqlite3BeginWriteOperation(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3AuthCheck(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3LogEst(_: u64_0) -> LogEst;
    fn sqlite3TwoPartName(
        _: *mut Parse,
        _: *mut Token,
        _: *mut Token,
        _: *mut *mut Token,
    ) -> ::core::ffi::c_int;
    fn sqlite3ReadSchema(pParse: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3LocateCollSeq(
        pParse: *mut Parse,
        zName: *const ::core::ffi::c_char,
    ) -> *mut CollSeq;
    fn sqlite3NestedParse(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3FindDb(_: *mut sqlite3, _: *mut Token) -> ::core::ffi::c_int;
    fn sqlite3DefaultRowEst(_: *mut Index);
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3StrAccumInit(
        _: *mut StrAccum,
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ResultStrAccum(_: *mut sqlite3_context, _: *mut StrAccum);
    fn sqlite3TableLock(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: Pgno,
        _: u8_0,
        _: *const ::core::ffi::c_char,
    );
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
pub type tRowcnt = u64_0;
pub type StrAccum = sqlite3_str;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StatSample {
    pub anDLt: *mut tRowcnt

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StatAccum {
    pub db: *mut sqlite3,
    pub nEst: tRowcnt,
    pub nRow: tRowcnt,
    pub nLimit: ::core::ffi::c_int,
    pub nCol: ::core::ffi::c_int,
    pub nKeyCol: ::core::ffi::c_int,
    pub nSkipAhead: u8_0,
    pub current: StatSample

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub zName: *const ::core::ffi::c_char,
    pub zCols: *const ::core::ffi::c_char

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct analysisInfo {
    pub db: *mut sqlite3,
    pub zDatabase: *const ::core::ffi::c_char

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ANALYZE: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OP_Goto: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OP_If: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const OP_IfNot: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const OP_SeekGT: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const OP_IsNull: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const OP_NotNull: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const OP_Ne: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_MakeRecord: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const OP_Count: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
pub const OP_OpenRead: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const OP_OpenWrite: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const OP_NewRowid: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const OP_Insert: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const OP_Clear: ::core::ffi::c_int = 146 as ::core::ffi::c_int;
pub const OP_LoadAnalysis: ::core::ffi::c_int = 151 as ::core::ffi::c_int;
pub const OP_Expire: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const OP_Noop: ::core::ffi::c_int = 188 as ::core::ffi::c_int;
pub const P4_COLLSEQ: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const P4_TABLE: ::core::ffi::c_int = -5 as ::core::ffi::c_int;
pub const P4_DYNAMIC: ::core::ffi::c_int = -6 as ::core::ffi::c_int;
pub const IsStat4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn openStatTable(
    mut pParse: *mut Parse,
    mut iDb: ::core::ffi::c_int,
    mut iStatCur: ::core::ffi::c_int,
    mut zWhere: *const ::core::ffi::c_char,
    mut zWhereType: *const ::core::ffi::c_char,
) {
    unsafe {
        static mut aTable: [C2Rust_Unnamed_22; 3] = [
            C2Rust_Unnamed_22 {
                zName: b"sqlite_stat1\0".as_ptr() as *const ::core::ffi::c_char,
                zCols: b"tbl,idx,stat\0".as_ptr() as *const ::core::ffi::c_char

            },
            C2Rust_Unnamed_22 {
                zName: b"sqlite_stat4\0".as_ptr() as *const ::core::ffi::c_char,
                zCols: ::core::ptr::null::<::core::ffi::c_char>()

            },
            C2Rust_Unnamed_22 {
                zName: b"sqlite_stat3\0".as_ptr() as *const ::core::ffi::c_char,
                zCols: ::core::ptr::null::<::core::ffi::c_char>()

            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pDb: *mut Db = ::core::ptr::null_mut::<Db>();
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        let mut aRoot: [u32_0; 3] = [0; 3];
        let mut aCreateTbl: [u8_0; 3] = [0; 3];
        let nToOpen: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if v.is_null() {
            return;
        }
        pDb = (*db).aDb.offset(iDb as isize) as *mut Db;
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[C2Rust_Unnamed_22; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_22>() as usize)
                as ::core::ffi::c_int
        {
            let mut zTab: *const ::core::ffi::c_char = aTable[i as usize].zName;
            let mut pStat: *mut Table = ::core::ptr::null_mut::<Table>();
            aCreateTbl[i as usize] = 0 as u8_0;
            pStat = sqlite3FindTable(db, zTab, (*pDb).zDbSName);
            if pStat.is_null() {
                if i < nToOpen {
                    sqlite3NestedParse(
                        pParse,
                        b"CREATE TABLE %Q.%s(%s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*pDb).zDbSName,
                        zTab,
                        aTable[i as usize].zCols,
                    );
                    aRoot[i as usize] = (*pParse).u1.cr.regRoot as u32_0;
                    aCreateTbl[i as usize] = OPFLAG_P2ISREG as u8_0;
                }
            } else {
                aRoot[i as usize] = (*pStat).tnum as u32_0;
                sqlite3TableLock(pParse, iDb, aRoot[i as usize], 1 as u8_0, zTab);
                if !zWhere.is_null() {
                    sqlite3NestedParse(
                        pParse,
                        b"DELETE FROM %Q.%s WHERE %s=%Q\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*pDb).zDbSName,
                        zTab,
                        zWhereType,
                        zWhere,
                    );
                } else if (*db).xPreUpdateCallback.is_some() {
                    sqlite3NestedParse(
                        pParse,
                        b"DELETE FROM %Q.%s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*pDb).zDbSName,
                        zTab,
                    );
                } else {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Clear,
                        aRoot[i as usize] as ::core::ffi::c_int,
                        iDb,
                    );
                }
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nToOpen {
            sqlite3VdbeAddOp4Int(
                v,
                OP_OpenWrite,
                iStatCur + i,
                aRoot[i as usize] as ::core::ffi::c_int,
                iDb,
                3 as ::core::ffi::c_int,
            );
            sqlite3VdbeChangeP5(v, aCreateTbl[i as usize] as u16_0);
            i += 1;
        }
    }
}
unsafe extern "C" fn statAccumDestructor(mut pOld: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut StatAccum = pOld as *mut StatAccum;
        sqlite3DbFree((*p).db, p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn statInit(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut StatAccum = ::core::ptr::null_mut::<StatAccum>();
        let mut nCol: ::core::ffi::c_int = 0;
        let mut nKeyCol: ::core::ffi::c_int = 0;
        let mut nColUp: ::core::ffi::c_int = 0;
        let mut n: i64_0 = 0;
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        nCol = sqlite3_value_int(*argv.offset(0 as ::core::ffi::c_int as isize));
        nColUp = if (::core::mem::size_of::<tRowcnt>() as usize) < 8 as usize {
            nCol + 1 as ::core::ffi::c_int & !(1 as ::core::ffi::c_int)
        } else {
            nCol
        };
        nKeyCol = sqlite3_value_int(*argv.offset(1 as ::core::ffi::c_int as isize));
        n = (::core::mem::size_of::<StatAccum>() as usize)
            .wrapping_add(
                (::core::mem::size_of::<tRowcnt>() as usize)
                    .wrapping_mul(nColUp as usize),
            ) as i64_0;
        p = sqlite3DbMallocZero(db, n as u64_0) as *mut StatAccum;
        if p.is_null() {
            sqlite3_result_error_nomem(context);
            return;
        }
        (*p).db = db;
        (*p).nEst = sqlite3_value_int64(*argv.offset(2 as ::core::ffi::c_int as isize))
            as tRowcnt;
        (*p).nRow = 0 as tRowcnt;
        (*p).nLimit = sqlite3_value_int(*argv.offset(3 as ::core::ffi::c_int as isize));
        (*p).nCol = nCol;
        (*p).nKeyCol = nKeyCol;
        (*p).nSkipAhead = 0 as u8_0;
        (*p).current.anDLt = p.offset(1 as ::core::ffi::c_int as isize) as *mut StatAccum
            as *mut tRowcnt;
        sqlite3_result_blob(
            context,
            p as *const ::core::ffi::c_void,
            ::core::mem::size_of::<StatAccum>() as ::core::ffi::c_int,
            Some(
                statAccumDestructor
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
    }
}
static mut statInitFuncdef: FuncDef = unsafe {
    FuncDef {
        nArg: 4 as i16_0,
        funcFlags: SQLITE_UTF8 as u32_0,
        pUserData: ::core::ptr::null::<::core::ffi::c_void>()
            as *mut ::core::ffi::c_void,
        pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
        xSFunc: Some(
            statInit
                as unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
        ),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: b"stat_init\0".as_ptr() as *const ::core::ffi::c_char,
        u: C2Rust_Unnamed_2 {
            pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

        }

    }
};
unsafe extern "C" fn statPush(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut p: *mut StatAccum = sqlite3_value_blob(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) as *mut StatAccum;
        let mut iChng: ::core::ffi::c_int = sqlite3_value_int(
            *argv.offset(1 as ::core::ffi::c_int as isize),
        );
        if !((*p).nRow == 0 as ::core::ffi::c_ulonglong) {
            i = iChng;
            while i < (*p).nCol {
                let ref mut c2rust_fresh11 = *(*p).current.anDLt.offset(i as isize);
                *c2rust_fresh11 = (*c2rust_fresh11).wrapping_add(1);
                i += 1;
            }
        }
        (*p).nRow = (*p).nRow.wrapping_add(1);
        if (*p).nLimit != 0
            && (*p).nRow
                > ((*p).nLimit as ::core::ffi::c_ulonglong)
                    .wrapping_mul(
                        ((*p).nSkipAhead as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    )
        {
            (*p).nSkipAhead = (*p).nSkipAhead.wrapping_add(1);
            sqlite3_result_int(
                context,
                (*(*p).current.anDLt.offset(0 as ::core::ffi::c_int as isize)
                    > 0 as ::core::ffi::c_ulonglong) as ::core::ffi::c_int,
            );
        }
    }
}
static mut statPushFuncdef: FuncDef = unsafe {
    FuncDef {
        nArg: (2 as ::core::ffi::c_int + IsStat4) as i16_0,
        funcFlags: SQLITE_UTF8 as u32_0,
        pUserData: ::core::ptr::null::<::core::ffi::c_void>()
            as *mut ::core::ffi::c_void,
        pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
        xSFunc: Some(
            statPush
                as unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
        ),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: b"stat_push\0".as_ptr() as *const ::core::ffi::c_char,
        u: C2Rust_Unnamed_2 {
            pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

        }

    }
};
pub const STAT_GET_STAT1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn statGet(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut StatAccum = sqlite3_value_blob(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) as *mut StatAccum;
        let mut sStat: sqlite3_str = sqlite3_str {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        let mut i: ::core::ffi::c_int = 0;
        sqlite3StrAccumInit(
            &raw mut sStat,
            ::core::ptr::null_mut::<sqlite3>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            ((*p).nKeyCol + 1 as ::core::ffi::c_int) * 100 as ::core::ffi::c_int,
        );
        sqlite3_str_appendf(
            &raw mut sStat,
            b"%llu\0".as_ptr() as *const ::core::ffi::c_char,
            if (*p).nSkipAhead as ::core::ffi::c_int != 0 {
                (*p).nEst
            } else {
                (*p).nRow
            },
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nKeyCol {
            let mut nDistinct: u64_0 = (*(*p).current.anDLt.offset(i as isize) as u64_0)
                .wrapping_add(1 as u64_0);
            let mut iVal: u64_0 = ((*p).nRow as u64_0)
                .wrapping_add(nDistinct)
                .wrapping_sub(1 as u64_0)
                .wrapping_div(nDistinct);
            if iVal == 2 as ::core::ffi::c_ulonglong
                && ((*p).nRow as ::core::ffi::c_ulonglong)
                    .wrapping_mul(10 as ::core::ffi::c_ulonglong)
                    <= (nDistinct as ::core::ffi::c_ulonglong)
                        .wrapping_mul(11 as ::core::ffi::c_ulonglong)
            {
                iVal = 1 as u64_0;
            }
            sqlite3_str_appendf(
                &raw mut sStat,
                b" %llu\0".as_ptr() as *const ::core::ffi::c_char,
                iVal,
            );
            i += 1;
        }
        sqlite3ResultStrAccum(context, &raw mut sStat);
    }
}
static mut statGetFuncdef: FuncDef = unsafe {
    FuncDef {
        nArg: (1 as ::core::ffi::c_int + IsStat4) as i16_0,
        funcFlags: SQLITE_UTF8 as u32_0,
        pUserData: ::core::ptr::null::<::core::ffi::c_void>()
            as *mut ::core::ffi::c_void,
        pNext: ::core::ptr::null::<FuncDef>() as *mut FuncDef,
        xSFunc: Some(
            statGet
                as unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
        ),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: b"stat_get\0".as_ptr() as *const ::core::ffi::c_char,
        u: C2Rust_Unnamed_2 {
            pHash: ::core::ptr::null::<FuncDef>() as *mut FuncDef

        }

    }
};
unsafe extern "C" fn callStatGet(
    mut pParse: *mut Parse,
    mut regStat: ::core::ffi::c_int,
    mut iParam: ::core::ffi::c_int,
    mut regOut: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3VdbeAddFunctionCall(
            pParse,
            0 as ::core::ffi::c_int,
            regStat,
            regOut,
            1 as ::core::ffi::c_int + IsStat4,
            &raw const statGetFuncdef,
            0 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn analyzeOneTable(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pOnlyIdx: *mut Index,
    mut iStatCur: ::core::ffi::c_int,
    mut iMem: ::core::ffi::c_int,
    mut iTab: ::core::ffi::c_int,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut iIdxCur: ::core::ffi::c_int = 0;
        let mut iTabCur: ::core::ffi::c_int = 0;
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut i: ::core::ffi::c_int = 0;
        let mut jZeroRows: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iDb: ::core::ffi::c_int = 0;
        let mut needTableCnt: u8_0 = 1 as u8_0;
        let c2rust_fresh0 = iMem;
        iMem = iMem + 1;
        let mut regNewRowid: ::core::ffi::c_int = c2rust_fresh0;
        let c2rust_fresh1 = iMem;
        iMem = iMem + 1;
        let mut regStat: ::core::ffi::c_int = c2rust_fresh1;
        let c2rust_fresh2 = iMem;
        iMem = iMem + 1;
        let mut regChng: ::core::ffi::c_int = c2rust_fresh2;
        let c2rust_fresh3 = iMem;
        iMem = iMem + 1;
        let mut regRowid: ::core::ffi::c_int = c2rust_fresh3;
        let c2rust_fresh4 = iMem;
        iMem = iMem + 1;
        let mut regTemp: ::core::ffi::c_int = c2rust_fresh4;
        let c2rust_fresh5 = iMem;
        iMem = iMem + 1;
        let mut regTemp2: ::core::ffi::c_int = c2rust_fresh5;
        let c2rust_fresh6 = iMem;
        iMem = iMem + 1;
        let mut regTabname: ::core::ffi::c_int = c2rust_fresh6;
        let c2rust_fresh7 = iMem;
        iMem = iMem + 1;
        let mut regIdxname: ::core::ffi::c_int = c2rust_fresh7;
        let c2rust_fresh8 = iMem;
        iMem = iMem + 1;
        let mut regStat1: ::core::ffi::c_int = c2rust_fresh8;
        let mut regPrev: ::core::ffi::c_int = iMem;
        let mut pStat1: *mut Table = ::core::ptr::null_mut::<Table>();
        sqlite3TouchRegister(pParse, iMem);
        v = sqlite3GetVdbe(pParse);
        if v.is_null() || pTab.is_null() {
            return;
        }
        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_NORM) {
            return;
        }
        if sqlite3_strlike(
            b"sqlite\\_%\0".as_ptr() as *const ::core::ffi::c_char,
            (*pTab).zName,
            '\\' as i32 as ::core::ffi::c_uint,
        ) == 0 as ::core::ffi::c_int
        {
            return;
        }
        iDb = sqlite3SchemaToIndex(db, (*pTab).pSchema);
        if sqlite3AuthCheck(
            pParse,
            SQLITE_ANALYZE,
            (*pTab).zName,
            ::core::ptr::null::<::core::ffi::c_char>(),
            (*(*db).aDb.offset(iDb as isize)).zDbSName,
        ) != 0
        {
            return;
        }
        if (*db).xPreUpdateCallback.is_some() {
            pStat1 = sqlite3DbMallocZero(
                db,
                (::core::mem::size_of::<Table>() as usize).wrapping_add(13 as usize)
                    as u64_0,
            ) as *mut Table;
            if pStat1.is_null() {
                return;
            }
            (*pStat1).zName = pStat1.offset(1 as ::core::ffi::c_int as isize)
                as *mut Table as *mut ::core::ffi::c_char;
            memcpy(
                (*pStat1).zName as *mut ::core::ffi::c_void,
                b"sqlite_stat1\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                13 as size_t,
            );
            (*pStat1).nCol = 3 as i16_0;
            (*pStat1).iPKey = -1 as ::core::ffi::c_int as i16_0;
            sqlite3VdbeAddOp4(
                (*pParse).pVdbe,
                OP_Noop,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                pStat1 as *mut ::core::ffi::c_char,
                P4_DYNAMIC,
            );
        }
        sqlite3TableLock(pParse, iDb, (*pTab).tnum, 0 as u8_0, (*pTab).zName);
        let c2rust_fresh9 = iTab;
        iTab = iTab + 1;
        iTabCur = c2rust_fresh9;
        let c2rust_fresh10 = iTab;
        iTab = iTab + 1;
        iIdxCur = c2rust_fresh10;
        (*pParse).nTab = if (*pParse).nTab > iTab { (*pParse).nTab } else { iTab };
        sqlite3OpenTable(pParse, iTabCur, iDb, pTab, OP_OpenRead);
        sqlite3VdbeLoadString(v, regTabname, (*pTab).zName);
        let mut c2rust_current_block_123: u64;
        pIdx = (*pTab).pIndex;
        while !pIdx.is_null() {
            let mut nCol: ::core::ffi::c_int = 0;
            let mut addrGotoEnd: ::core::ffi::c_int = 0;
            let mut addrNextRow: ::core::ffi::c_int = 0;
            let mut zIdxName: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut nColTest: ::core::ffi::c_int = 0;
            if !(!pOnlyIdx.is_null() && pOnlyIdx != pIdx) {
                if (*pIdx).pPartIdxWhere.is_null() {
                    needTableCnt = 0 as u8_0;
                }
                if !((*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0)
                    && (*pIdx).idxType() as ::core::ffi::c_int
                        == SQLITE_IDXTYPE_PRIMARYKEY
                {
                    nCol = (*pIdx).nKeyCol as ::core::ffi::c_int;
                    zIdxName = (*pTab).zName;
                    nColTest = nCol - 1 as ::core::ffi::c_int;
                } else {
                    nCol = (*pIdx).nColumn as ::core::ffi::c_int;
                    zIdxName = (*pIdx).zName;
                    nColTest = if (*pIdx).uniqNotNull() as ::core::ffi::c_int != 0 {
                        (*pIdx).nKeyCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    } else {
                        nCol - 1 as ::core::ffi::c_int
                    };
                }
                sqlite3VdbeLoadString(v, regIdxname, zIdxName);
                sqlite3TouchRegister(pParse, regPrev + nColTest);
                sqlite3VdbeAddOp3(
                    v,
                    OP_OpenRead,
                    iIdxCur,
                    (*pIdx).tnum as ::core::ffi::c_int,
                    iDb,
                );
                sqlite3VdbeSetP4KeyInfo(pParse, pIdx);
                sqlite3VdbeAddOp2(v, OP_Integer, (*db).nAnalysisLimit, regTemp2);
                sqlite3VdbeAddOp2(
                    v,
                    OP_Integer,
                    nCol,
                    regStat + 1 as ::core::ffi::c_int,
                );
                sqlite3VdbeAddOp2(
                    v,
                    OP_Integer,
                    (*pIdx).nKeyCol as ::core::ffi::c_int,
                    regRowid,
                );
                sqlite3VdbeAddOp3(
                    v,
                    OP_Count,
                    iIdxCur,
                    regTemp,
                    ((*db).dbOptFlags & 0x800 as u32_0 != 0 as u32_0)
                        as ::core::ffi::c_int,
                );
                sqlite3VdbeAddFunctionCall(
                    pParse,
                    0 as ::core::ffi::c_int,
                    regStat + 1 as ::core::ffi::c_int,
                    regStat,
                    4 as ::core::ffi::c_int,
                    &raw const statInitFuncdef,
                    0 as ::core::ffi::c_int,
                );
                addrGotoEnd = sqlite3VdbeAddOp1(v, OP_Rewind, iIdxCur);
                sqlite3VdbeAddOp2(v, OP_Integer, 0 as ::core::ffi::c_int, regChng);
                addrNextRow = sqlite3VdbeCurrentAddr(v);
                if nColTest > 0 as ::core::ffi::c_int {
                    let mut endDistinctTest: ::core::ffi::c_int = sqlite3VdbeMakeLabel(
                        pParse,
                    );
                    let mut aGotoChng: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
                        ::core::ffi::c_int,
                    >();
                    aGotoChng = sqlite3DbMallocRawNN(
                        db,
                        (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                            .wrapping_mul(nColTest as usize) as u64_0,
                    ) as *mut ::core::ffi::c_int;
                    if aGotoChng.is_null() {
                        c2rust_current_block_123 = 2569451025026770673;
                    } else {
                        sqlite3VdbeAddOp0(v, OP_Goto);
                        addrNextRow = sqlite3VdbeCurrentAddr(v);
                        if nColTest == 1 as ::core::ffi::c_int
                            && (*pIdx).nKeyCol as ::core::ffi::c_int
                                == 1 as ::core::ffi::c_int
                            && (*pIdx).onError as ::core::ffi::c_int != OE_None
                        {
                            sqlite3VdbeAddOp2(v, OP_NotNull, regPrev, endDistinctTest);
                        }
                        i = 0 as ::core::ffi::c_int;
                        while i < nColTest {
                            let mut pColl: *mut ::core::ffi::c_char = sqlite3LocateCollSeq(
                                pParse,
                                *(*pIdx).azColl.offset(i as isize),
                            ) as *mut ::core::ffi::c_char;
                            sqlite3VdbeAddOp2(v, OP_Integer, i, regChng);
                            sqlite3VdbeAddOp3(v, OP_Column, iIdxCur, i, regTemp);
                            *aGotoChng.offset(i as isize) = sqlite3VdbeAddOp4(
                                v,
                                OP_Ne,
                                regTemp,
                                0 as ::core::ffi::c_int,
                                regPrev + i,
                                pColl,
                                P4_COLLSEQ,
                            );
                            sqlite3VdbeChangeP5(v, SQLITE_NULLEQ as u16_0);
                            i += 1;
                        }
                        sqlite3VdbeAddOp2(v, OP_Integer, nColTest, regChng);
                        sqlite3VdbeGoto(v, endDistinctTest);
                        sqlite3VdbeJumpHere(v, addrNextRow - 1 as ::core::ffi::c_int);
                        i = 0 as ::core::ffi::c_int;
                        while i < nColTest {
                            sqlite3VdbeJumpHere(v, *aGotoChng.offset(i as isize));
                            sqlite3VdbeAddOp3(v, OP_Column, iIdxCur, i, regPrev + i);
                            i += 1;
                        }
                        sqlite3VdbeResolveLabel(v, endDistinctTest);
                        sqlite3DbFree(db, aGotoChng as *mut ::core::ffi::c_void);
                        c2rust_current_block_123 = 5854763015135596753;
                    }
                } else {
                    c2rust_current_block_123 = 5854763015135596753;
                }
                match c2rust_current_block_123 {
                    2569451025026770673 => {}
                    _ => {
                        sqlite3VdbeAddFunctionCall(
                            pParse,
                            1 as ::core::ffi::c_int,
                            regStat,
                            regTemp,
                            2 as ::core::ffi::c_int + IsStat4,
                            &raw const statPushFuncdef,
                            0 as ::core::ffi::c_int,
                        );
                        if (*db).nAnalysisLimit != 0 {
                            let mut j1: ::core::ffi::c_int = 0;
                            let mut j2: ::core::ffi::c_int = 0;
                            let mut j3: ::core::ffi::c_int = 0;
                            j1 = sqlite3VdbeAddOp1(v, OP_IsNull, regTemp);
                            j2 = sqlite3VdbeAddOp1(v, OP_If, regTemp);
                            j3 = sqlite3VdbeAddOp4Int(
                                v,
                                OP_SeekGT,
                                iIdxCur,
                                0 as ::core::ffi::c_int,
                                regPrev,
                                1 as ::core::ffi::c_int,
                            );
                            sqlite3VdbeJumpHere(v, j1);
                            sqlite3VdbeAddOp2(v, OP_Next, iIdxCur, addrNextRow);
                            sqlite3VdbeJumpHere(v, j2);
                            sqlite3VdbeJumpHere(v, j3);
                        } else {
                            sqlite3VdbeAddOp2(v, OP_Next, iIdxCur, addrNextRow);
                        }
                        if !(*pIdx).pPartIdxWhere.is_null() {
                            sqlite3VdbeJumpHere(v, addrGotoEnd);
                            addrGotoEnd = 0 as ::core::ffi::c_int;
                        }
                        callStatGet(pParse, regStat, STAT_GET_STAT1, regStat1);
                        sqlite3VdbeAddOp4(
                            v,
                            OP_MakeRecord,
                            regTabname,
                            3 as ::core::ffi::c_int,
                            regTemp,
                            b"BBB\0".as_ptr() as *const ::core::ffi::c_char,
                            0 as ::core::ffi::c_int,
                        );
                        sqlite3VdbeAddOp2(v, OP_NewRowid, iStatCur, regNewRowid);
                        sqlite3VdbeAddOp3(v, OP_Insert, iStatCur, regTemp, regNewRowid);
                        sqlite3VdbeChangeP4(
                            v,
                            -1 as ::core::ffi::c_int,
                            pStat1 as *mut ::core::ffi::c_char,
                            P4_TABLE,
                        );
                        sqlite3VdbeChangeP5(v, OPFLAG_APPEND as u16_0);
                        if addrGotoEnd != 0 {
                            sqlite3VdbeJumpHere(v, addrGotoEnd);
                        }
                    }
                }
            }
            pIdx = (*pIdx).pNext;
        }
        if pOnlyIdx.is_null() && needTableCnt as ::core::ffi::c_int != 0 {
            sqlite3VdbeAddOp2(v, OP_Count, iTabCur, regStat1);
            jZeroRows = sqlite3VdbeAddOp1(v, OP_IfNot, regStat1);
            sqlite3VdbeAddOp2(v, OP_Null, 0 as ::core::ffi::c_int, regIdxname);
            sqlite3VdbeAddOp4(
                v,
                OP_MakeRecord,
                regTabname,
                3 as ::core::ffi::c_int,
                regTemp,
                b"BBB\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            sqlite3VdbeAddOp2(v, OP_NewRowid, iStatCur, regNewRowid);
            sqlite3VdbeAddOp3(v, OP_Insert, iStatCur, regTemp, regNewRowid);
            sqlite3VdbeChangeP5(v, OPFLAG_APPEND as u16_0);
            sqlite3VdbeChangeP4(
                v,
                -1 as ::core::ffi::c_int,
                pStat1 as *mut ::core::ffi::c_char,
                P4_TABLE,
            );
            sqlite3VdbeJumpHere(v, jZeroRows);
        }
    }
}
unsafe extern "C" fn loadAnalysis(mut pParse: *mut Parse, mut iDb: ::core::ffi::c_int) {
    unsafe {
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        if !v.is_null() {
            sqlite3VdbeAddOp1(v, OP_LoadAnalysis, iDb);
        }
    }
}
unsafe extern "C" fn analyzeDatabase(
    mut pParse: *mut Parse,
    mut iDb: ::core::ffi::c_int,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pSchema: *mut Schema = (*(*db).aDb.offset(iDb as isize)).pSchema;
        let mut k: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut iStatCur: ::core::ffi::c_int = 0;
        let mut iMem: ::core::ffi::c_int = 0;
        let mut iTab: ::core::ffi::c_int = 0;
        sqlite3BeginWriteOperation(pParse, 0 as ::core::ffi::c_int, iDb);
        iStatCur = (*pParse).nTab;
        (*pParse).nTab += 3 as ::core::ffi::c_int;
        openStatTable(
            pParse,
            iDb,
            iStatCur,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        iMem = (*pParse).nMem + 1 as ::core::ffi::c_int;
        iTab = (*pParse).nTab;
        k = (*pSchema).tblHash.first;
        while !k.is_null() {
            let mut pTab: *mut Table = (*k).data as *mut Table;
            analyzeOneTable(
                pParse,
                pTab,
                ::core::ptr::null_mut::<Index>(),
                iStatCur,
                iMem,
                iTab,
            );
            k = (*k).next;
        }
        loadAnalysis(pParse, iDb);
    }
}
unsafe extern "C" fn analyzeTable(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pOnlyIdx: *mut Index,
) {
    unsafe {
        let mut iDb: ::core::ffi::c_int = 0;
        let mut iStatCur: ::core::ffi::c_int = 0;
        iDb = sqlite3SchemaToIndex((*pParse).db, (*pTab).pSchema);
        sqlite3BeginWriteOperation(pParse, 0 as ::core::ffi::c_int, iDb);
        iStatCur = (*pParse).nTab;
        (*pParse).nTab += 3 as ::core::ffi::c_int;
        if !pOnlyIdx.is_null() {
            openStatTable(
                pParse,
                iDb,
                iStatCur,
                (*pOnlyIdx).zName,
                b"idx\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            openStatTable(
                pParse,
                iDb,
                iStatCur,
                (*pTab).zName,
                b"tbl\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        analyzeOneTable(
            pParse,
            pTab,
            pOnlyIdx,
            iStatCur,
            (*pParse).nMem + 1 as ::core::ffi::c_int,
            (*pParse).nTab,
        );
        loadAnalysis(pParse, iDb);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Analyze(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut iDb: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zDb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut pTableName: *mut Token = ::core::ptr::null_mut::<Token>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        if SQLITE_OK != sqlite3ReadSchema(pParse) {
            return;
        }
        if pName1.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*db).nDb {
                if !(i == 1 as ::core::ffi::c_int) {
                    analyzeDatabase(pParse, i);
                }
                i += 1;
            }
        } else if (*pName2).n == 0 as ::core::ffi::c_uint
            && {
                iDb = sqlite3FindDb(db, pName1);
                iDb >= 0 as ::core::ffi::c_int
            }
        {
            analyzeDatabase(pParse, iDb);
        } else {
            iDb = sqlite3TwoPartName(pParse, pName1, pName2, &raw mut pTableName);
            if iDb >= 0 as ::core::ffi::c_int {
                zDb = if (*pName2).n != 0 {
                    (*(*db).aDb.offset(iDb as isize)).zDbSName
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                };
                z = sqlite3NameFromToken(db, pTableName);
                if !z.is_null() {
                    pIdx = sqlite3FindIndex(db, z, zDb);
                    if !pIdx.is_null() {
                        analyzeTable(pParse, (*pIdx).pTable, pIdx);
                    } else {
                        pTab = sqlite3LocateTable(pParse, 0 as u32_0, z, zDb);
                        if !pTab.is_null() {
                            analyzeTable(pParse, pTab, ::core::ptr::null_mut::<Index>());
                        }
                    }
                    sqlite3DbFree(db, z as *mut ::core::ffi::c_void);
                }
            }
        }
        if (*db).nSqlExec as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && {
                v = sqlite3GetVdbe(pParse);
                !v.is_null()
            }
        {
            sqlite3VdbeAddOp0(v, OP_Expire);
        }
    }
}
unsafe extern "C" fn decodeIntArray(
    mut zIntArray: *mut ::core::ffi::c_char,
    mut nOut: ::core::ffi::c_int,
    mut aOut: *mut tRowcnt,
    mut aLog: *mut LogEst,
    mut pIndex: *mut Index,
) {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = zIntArray;
        let mut c: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut v: tRowcnt = 0;
        i = 0 as ::core::ffi::c_int;
        while *z as ::core::ffi::c_int != 0 && i < nOut {
            v = 0 as tRowcnt;
            loop {
                c = *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                if !(c >= '0' as i32 && c <= '9' as i32) {
                    break;
                }
                v = (v as ::core::ffi::c_ulonglong)
                    .wrapping_mul(10 as ::core::ffi::c_ulonglong)
                    .wrapping_add(c as ::core::ffi::c_ulonglong)
                    .wrapping_sub('0' as i32 as ::core::ffi::c_ulonglong) as tRowcnt;
                z = z.offset(1);
            }
            *aLog.offset(i as isize) = sqlite3LogEst(v as u64_0);
            if *z as ::core::ffi::c_int == ' ' as i32 {
                z = z.offset(1);
            }
            i += 1;
        }
        (*pIndex).set_bUnordered(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*pIndex).set_noSkipScan(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        while *z.offset(0 as ::core::ffi::c_int as isize) != 0 {
            if sqlite3_strglob(b"unordered*\0".as_ptr() as *const ::core::ffi::c_char, z)
                == 0 as ::core::ffi::c_int
            {
                (*pIndex)
                    .set_bUnordered(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else if sqlite3_strglob(
                b"sz=[0-9]*\0".as_ptr() as *const ::core::ffi::c_char,
                z,
            ) == 0 as ::core::ffi::c_int
            {
                let mut sz: ::core::ffi::c_int = sqlite3Atoi(
                    z.offset(3 as ::core::ffi::c_int as isize),
                );
                if sz < 2 as ::core::ffi::c_int {
                    sz = 2 as ::core::ffi::c_int;
                }
                (*pIndex).szIdxRow = sqlite3LogEst(sz as u64_0);
            } else if sqlite3_strglob(
                b"noskipscan*\0".as_ptr() as *const ::core::ffi::c_char,
                z,
            ) == 0 as ::core::ffi::c_int
            {
                (*pIndex)
                    .set_noSkipScan(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            while *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
                && *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != ' ' as i32
            {
                z = z.offset(1);
            }
            while *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ' ' as i32
            {
                z = z.offset(1);
            }
        }
    }
}
unsafe extern "C" fn analysisLoader(
    mut pData: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut NotUsed: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pInfo: *mut analysisInfo = pData as *mut analysisInfo;
        let mut pIndex: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut pTable: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if argv.is_null() || (*argv.offset(0 as ::core::ffi::c_int as isize)).is_null()
            || (*argv.offset(2 as ::core::ffi::c_int as isize)).is_null()
        {
            return 0 as ::core::ffi::c_int;
        }
        pTable = sqlite3FindTable(
            (*pInfo).db,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            (*pInfo).zDatabase,
        );
        if pTable.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*argv.offset(1 as ::core::ffi::c_int as isize)).is_null() {
            pIndex = ::core::ptr::null_mut::<Index>();
        } else if sqlite3_stricmp(
            *argv.offset(0 as ::core::ffi::c_int as isize),
            *argv.offset(1 as ::core::ffi::c_int as isize),
        ) == 0 as ::core::ffi::c_int
        {
            pIndex = sqlite3PrimaryKeyIndex(pTable);
        } else {
            pIndex = sqlite3FindIndex(
                (*pInfo).db,
                *argv.offset(1 as ::core::ffi::c_int as isize),
                (*pInfo).zDatabase,
            );
        }
        z = *argv.offset(2 as ::core::ffi::c_int as isize);
        if !pIndex.is_null() {
            let mut aiRowEst: *mut tRowcnt = ::core::ptr::null_mut::<tRowcnt>();
            let mut nCol: ::core::ffi::c_int = (*pIndex).nKeyCol as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int;
            (*pIndex).set_bUnordered(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            decodeIntArray(
                z as *mut ::core::ffi::c_char,
                nCol,
                aiRowEst,
                (*pIndex).aiRowLogEst,
                pIndex,
            );
            (*pIndex).set_hasStat1(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            if (*pIndex).pPartIdxWhere.is_null() {
                (*pTable).nRowLogEst = *(*pIndex)
                    .aiRowLogEst
                    .offset(0 as ::core::ffi::c_int as isize);
                (*pTable).tabFlags = ((*pTable).tabFlags as ::core::ffi::c_uint
                    | TF_HasStat1 as ::core::ffi::c_uint) as u32_0;
            }
        } else {
            let mut fakeIdx: Index = Index {
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
            fakeIdx.szIdxRow = (*pTable).szTabRow;
            decodeIntArray(
                z as *mut ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<tRowcnt>(),
                &raw mut (*pTable).nRowLogEst,
                &raw mut fakeIdx,
            );
            (*pTable).szTabRow = fakeIdx.szIdxRow;
            (*pTable).tabFlags = ((*pTable).tabFlags as ::core::ffi::c_uint
                | TF_HasStat1 as ::core::ffi::c_uint) as u32_0;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DeleteIndexSamples(
    mut db: *mut sqlite3,
    mut pIdx: *mut Index,
) {}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AnalysisLoad(
    mut db: *mut sqlite3,
    mut iDb: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sInfo: analysisInfo = analysisInfo {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zDatabase: ::core::ptr::null::<::core::ffi::c_char>()

        };
        let mut i: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pSchema: *mut Schema = (*(*db).aDb.offset(iDb as isize)).pSchema;
        let mut pStat1: *const Table = ::core::ptr::null::<Table>();
        i = (*pSchema).tblHash.first;
        while !i.is_null() {
            let mut pTab: *mut Table = (*i).data as *mut Table;
            (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                & !TF_HasStat1 as ::core::ffi::c_uint) as u32_0;
            i = (*i).next;
        }
        i = (*pSchema).idxHash.first;
        while !i.is_null() {
            let mut pIdx: *mut Index = (*i).data as *mut Index;
            (*pIdx).set_hasStat1(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            i = (*i).next;
        }
        sInfo.db = db;
        sInfo.zDatabase = (*(*db).aDb.offset(iDb as isize)).zDbSName;
        pStat1 = sqlite3FindTable(
            db,
            b"sqlite_stat1\0".as_ptr() as *const ::core::ffi::c_char,
            sInfo.zDatabase,
        );
        if !pStat1.is_null() && (*pStat1).eTabType as ::core::ffi::c_int == TABTYP_NORM {
            zSql = sqlite3MPrintf(
                db,
                b"SELECT tbl,idx,stat FROM %Q.sqlite_stat1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sInfo.zDatabase,
            );
            if zSql.is_null() {
                rc = SQLITE_NOMEM_BKPT;
            } else {
                rc = sqlite3_exec(
                    db,
                    zSql,
                    Some(
                        analysisLoader
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    &raw mut sInfo as *mut ::core::ffi::c_void,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                sqlite3DbFree(db, zSql as *mut ::core::ffi::c_void);
            }
        }
        i = (*pSchema).idxHash.first;
        while !i.is_null() {
            let mut pIdx_0: *mut Index = (*i).data as *mut Index;
            if (*pIdx_0).hasStat1() == 0 {
                sqlite3DefaultRowEst(pIdx_0);
            }
            i = (*i).next;
        }
        if rc == SQLITE_NOMEM {
            sqlite3OomFault(db);
        }
        return rc;
    }
}
pub const SQLITE_NULLEQ: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_HasStat1: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TABTYP_NORM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OE_None: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_IDXTYPE_PRIMARYKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OPFLAG_APPEND: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPFLAG_P2ISREG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
