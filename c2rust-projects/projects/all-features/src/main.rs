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
    pub type Pager;
    fn sqlite3_os_end() -> ::core::ffi::c_int;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_text16(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_reset_auto_extension();
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_mutex_free(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_log(
        iErrCode: ::core::ffi::c_int,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3HashInit(_: *mut Hash);
    fn sqlite3HashFind(
        _: *const Hash,
        pKey: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3HashClear(_: *mut Hash);
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3OsInit() -> ::core::ffi::c_int;
    fn sqlite3OsFileControl(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsSleep(_: *mut sqlite3_vfs, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3PcacheInitialize() -> ::core::ffi::c_int;
    fn sqlite3PcacheShutdown();
    fn sqlite3PCacheBufferSetup(
        _: *mut ::core::ffi::c_void,
        sz: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    );
    fn sqlite3PCacheSetDefault();
    fn sqlite3HeaderSizePcache() -> ::core::ffi::c_int;
    fn sqlite3HeaderSizePcache1() -> ::core::ffi::c_int;
    static mut sqlite3TreeTrace: u32_0;
    static mut sqlite3WhereTrace: u32_0;
    fn sqlite3PagerShrink(_: *mut Pager);
    fn sqlite3PagerFlush(_: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerDataVersion(_: *mut Pager) -> u32_0;
    fn sqlite3PagerVfs(_: *mut Pager) -> *mut sqlite3_vfs;
    fn sqlite3PagerFile(_: *mut Pager) -> *mut sqlite3_file;
    fn sqlite3PagerJrnlFile(_: *mut Pager) -> *mut sqlite3_file;
    fn sqlite3BtreeOpen(
        pVfs: *mut sqlite3_vfs,
        zFilename: *const ::core::ffi::c_char,
        db: *mut sqlite3,
        ppBtree: *mut *mut Btree,
        flags: ::core::ffi::c_int,
        vfsFlags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeClose(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeSetPageSize(
        p: *mut Btree,
        nPagesize: ::core::ffi::c_int,
        nReserve: ::core::ffi::c_int,
        eFix: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeGetRequestedReserve(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeRollback(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeTxnState(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeIsInBackup(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeCheckpoint(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeGetFilename(_: *mut Btree) -> *const ::core::ffi::c_char;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeIsReadonly(pBt: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3HeaderSizeBtree() -> ::core::ffi::c_int;
    fn sqlite3BtreeClearCache(_: *mut Btree);
    fn sqlite3BtreeEnter(_: *mut Btree);
    fn sqlite3BtreeEnterAll(_: *mut sqlite3);
    fn sqlite3BtreeLeave(_: *mut Btree);
    fn sqlite3BtreeLeaveAll(_: *mut sqlite3);
    fn sqlite3IsIdChar(_: u8_0) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3ColumnType(
        _: *mut Column,
        _: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3MallocInit() -> ::core::ffi::c_int;
    fn sqlite3MallocEnd();
    fn sqlite3Malloc(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3MallocZero(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MallocSize(_: *const ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn sqlite3MemSetDefault();
    fn sqlite3BenignMallocHooks(
        _: Option<unsafe extern "C" fn() -> ()>,
        _: Option<unsafe extern "C" fn() -> ()>,
    );
    fn sqlite3MutexAlloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    fn sqlite3MutexInit() -> ::core::ffi::c_int;
    fn sqlite3MutexEnd() -> ::core::ffi::c_int;
    fn sqlite3MemoryBarrier();
    fn sqlite3LookasideUsed(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3Init(
        _: *mut sqlite3,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3ResetAllSchemasOfConnection(_: *mut sqlite3);
    fn sqlite3CollapseDatabaseArray(_: *mut sqlite3);
    fn sqlite3ColumnColl(_: *mut Column) -> *const ::core::ffi::c_char;
    fn sqlite3FaultSim(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3BitvecBuiltinTest(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3FindTable(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut Table;
    fn sqlite3PrngSaveState();
    fn sqlite3PrngRestoreState();
    fn sqlite3IsRowid(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3FindFunction(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: u8_0,
        _: u8_0,
    ) -> *mut FuncDef;
    fn sqlite3RegisterBuiltinFunctions();
    fn sqlite3RegisterPerConnectionBuiltinFunctions(_: *mut sqlite3);
    fn sqlite3SafetyCheckOk(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3SafetyCheckSickOrOk(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3ColumnIndex(
        pTab: *mut Table,
        zCol: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3LogEst(_: u64_0) -> LogEst;
    fn sqlite3LogEstFromDouble(_: ::core::ffi::c_double) -> LogEst;
    fn sqlite3LogEstToInt(_: LogEst) -> u64_0;
    fn sqlite3DecOrHexToI64(
        _: *const ::core::ffi::c_char,
        _: *mut i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3ErrorWithMsg(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3Error(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3HexToInt(h: ::core::ffi::c_int) -> u8_0;
    fn sqlite3MemdbInit() -> ::core::ffi::c_int;
    fn sqlite3FindCollSeq(
        _: *mut sqlite3,
        enc: u8_0,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut CollSeq;
    fn sqlite3SetTextEncoding(db: *mut sqlite3, _: u8_0);
    fn sqlite3GetBoolean(z: *const ::core::ffi::c_char, _: u8_0) -> u8_0;
    fn sqlite3ValueText(_: *mut sqlite3_value, _: u8_0) -> *const ::core::ffi::c_void;
    fn sqlite3ValueSetStr(
        _: *mut sqlite3_value,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: u8_0,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3ValueFree(_: *mut sqlite3_value);
    fn sqlite3ResultIntReal(_: *mut sqlite3_context);
    fn sqlite3ValueNew(_: *mut sqlite3) -> *mut sqlite3_value;
    fn sqlite3Utf16to8(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> *mut ::core::ffi::c_char;
    static sqlite3StrBINARY: [::core::ffi::c_char; 0];
    static mut sqlite3StdType: [*const ::core::ffi::c_char; 0];
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3BuiltinFunctions: FuncDefHash;
    static mut sqlite3PendingByte: ::core::ffi::c_int;
    fn sqlite3ExpirePreparedStatements(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3FindDbName(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3SchemaClear(_: *mut ::core::ffi::c_void);
    fn sqlite3SchemaGet(_: *mut sqlite3, _: *mut Btree) -> *mut Schema;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3OomClear(_: *mut sqlite3);
    fn sqlite3ApiExit(db: *mut sqlite3, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3AutoLoadExtensions(_: *mut sqlite3);
    fn sqlite3CloseExtensions(_: *mut sqlite3);
    fn sqlite3VtabDisconnect(db: *mut sqlite3, p: *mut Table);
    fn sqlite3VtabRollback(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3VtabModuleUnref(_: *mut sqlite3, _: *mut Module);
    fn sqlite3VtabUnlockList(_: *mut sqlite3);
    fn sqlite3VtabEponymousTableClear(_: *mut sqlite3, _: *mut Module);
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
    fn sqlite3CompileOptions(
        pnOpt: *mut ::core::ffi::c_int,
    ) -> *mut *const ::core::ffi::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void

}
pub type va_list = __builtin_va_list;
pub type u8_0 = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
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
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void

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
pub struct FuncDefHash {
    pub a: [*mut FuncDef; 23]

}
pub type size_t = usize;
pub type LOGFUNC_t = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub op: ::core::ffi::c_int,
    pub mask: u64_0

}
pub type intptr_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpenMode {
    pub z: *const ::core::ffi::c_char,
    pub mode: ::core::ffi::c_int

}
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type sqlite3LocaltimeType = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type void_function = Option<unsafe extern "C" fn() -> ()>;
pub type sqlite3FaultFuncType = Option<
    unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
>;
pub const SQLITE_VERSION: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"3.51.2\0")
};
pub const SQLITE_VERSION_NUMBER: ::core::ffi::c_int = 3051002 as ::core::ffi::c_int;
pub const SQLITE_SOURCE_ID: [::core::ffi::c_char; 85] = unsafe {
    ::core::mem::transmute::<
        [u8; 85],
        [::core::ffi::c_char; 85],
    >(
        *b"2026-01-09 17:27:48 b270f8339eb13b504d0b2ba154ebca966b7dde08e40c3ed7d559749818cbalt1\0",
    )
};
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_INTERNAL: ::core::ffi::c_int = 2;
pub const SQLITE_PERM: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_ABORT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_LOCKED: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_READONLY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_INTERRUPT: ::core::ffi::c_int = 9;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12;
pub const SQLITE_FULL: ::core::ffi::c_int = 13;
pub const SQLITE_CANTOPEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SQLITE_PROTOCOL: ::core::ffi::c_int = 15;
pub const SQLITE_EMPTY: ::core::ffi::c_int = 16;
pub const SQLITE_SCHEMA: ::core::ffi::c_int = 17;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_MISMATCH: ::core::ffi::c_int = 20;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_NOLFS: ::core::ffi::c_int = 22;
pub const SQLITE_AUTH: ::core::ffi::c_int = 23;
pub const SQLITE_FORMAT: ::core::ffi::c_int = 24;
pub const SQLITE_RANGE: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const SQLITE_NOTADB: ::core::ffi::c_int = 26;
pub const SQLITE_NOTICE: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const SQLITE_WARNING: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100;
pub const SQLITE_DONE: ::core::ffi::c_int = 101;
pub const SQLITE_ERROR_MISSING_COLLSEQ: ::core::ffi::c_int = 257;
pub const SQLITE_ERROR_RETRY: ::core::ffi::c_int = 513;
pub const SQLITE_ERROR_SNAPSHOT: ::core::ffi::c_int = 769;
pub const SQLITE_IOERR_READ: ::core::ffi::c_int = 266;
pub const SQLITE_IOERR_SHORT_READ: ::core::ffi::c_int = 522;
pub const SQLITE_IOERR_WRITE: ::core::ffi::c_int = 778;
pub const SQLITE_IOERR_FSYNC: ::core::ffi::c_int = 1034;
pub const SQLITE_IOERR_DIR_FSYNC: ::core::ffi::c_int = 1290;
pub const SQLITE_IOERR_TRUNCATE: ::core::ffi::c_int = 1546;
pub const SQLITE_IOERR_FSTAT: ::core::ffi::c_int = 1802;
pub const SQLITE_IOERR_UNLOCK: ::core::ffi::c_int = 2058;
pub const SQLITE_IOERR_RDLOCK: ::core::ffi::c_int = 2314;
pub const SQLITE_IOERR_DELETE: ::core::ffi::c_int = 2570;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_ACCESS: ::core::ffi::c_int = 3338;
pub const SQLITE_IOERR_CHECKRESERVEDLOCK: ::core::ffi::c_int = 3594;
pub const SQLITE_IOERR_LOCK: ::core::ffi::c_int = 3850;
pub const SQLITE_IOERR_CLOSE: ::core::ffi::c_int = 4106;
pub const SQLITE_IOERR_DIR_CLOSE: ::core::ffi::c_int = 4362;
pub const SQLITE_IOERR_SHMOPEN: ::core::ffi::c_int = 4618;
pub const SQLITE_IOERR_SHMSIZE: ::core::ffi::c_int = 4874;
pub const SQLITE_IOERR_SHMLOCK: ::core::ffi::c_int = 5130;
pub const SQLITE_IOERR_SHMMAP: ::core::ffi::c_int = 5386;
pub const SQLITE_IOERR_SEEK: ::core::ffi::c_int = 5642;
pub const SQLITE_IOERR_DELETE_NOENT: ::core::ffi::c_int = 5898;
pub const SQLITE_IOERR_MMAP: ::core::ffi::c_int = 6154;
pub const SQLITE_IOERR_GETTEMPPATH: ::core::ffi::c_int = 6410;
pub const SQLITE_IOERR_CONVPATH: ::core::ffi::c_int = 6666;
pub const SQLITE_LOCKED_SHAREDCACHE: ::core::ffi::c_int = 262;
pub const SQLITE_BUSY_RECOVERY: ::core::ffi::c_int = 261;
pub const SQLITE_BUSY_SNAPSHOT: ::core::ffi::c_int = 517;
pub const SQLITE_CANTOPEN_NOTEMPDIR: ::core::ffi::c_int = 270;
pub const SQLITE_CANTOPEN_ISDIR: ::core::ffi::c_int = 526;
pub const SQLITE_CANTOPEN_FULLPATH: ::core::ffi::c_int = 782;
pub const SQLITE_CANTOPEN_CONVPATH: ::core::ffi::c_int = 1038;
pub const SQLITE_CANTOPEN_SYMLINK: ::core::ffi::c_int = 1550;
pub const SQLITE_CORRUPT_VTAB: ::core::ffi::c_int = 267;
pub const SQLITE_READONLY_RECOVERY: ::core::ffi::c_int = 264;
pub const SQLITE_READONLY_ROLLBACK: ::core::ffi::c_int = 776;
pub const SQLITE_READONLY_DBMOVED: ::core::ffi::c_int = 1032;
pub const SQLITE_READONLY_CANTINIT: ::core::ffi::c_int = 1288;
pub const SQLITE_READONLY_DIRECTORY: ::core::ffi::c_int = 1544;
pub const SQLITE_ABORT_ROLLBACK: ::core::ffi::c_int = 516;
pub const SQLITE_CONSTRAINT_CHECK: ::core::ffi::c_int = 275;
pub const SQLITE_CONSTRAINT_COMMITHOOK: ::core::ffi::c_int = 531;
pub const SQLITE_CONSTRAINT_FOREIGNKEY: ::core::ffi::c_int = 787;
pub const SQLITE_CONSTRAINT_FUNCTION: ::core::ffi::c_int = 1043;
pub const SQLITE_CONSTRAINT_NOTNULL: ::core::ffi::c_int = 1299;
pub const SQLITE_CONSTRAINT_PRIMARYKEY: ::core::ffi::c_int = 1555;
pub const SQLITE_CONSTRAINT_TRIGGER: ::core::ffi::c_int = 1811;
pub const SQLITE_CONSTRAINT_UNIQUE: ::core::ffi::c_int = 2067;
pub const SQLITE_CONSTRAINT_VTAB: ::core::ffi::c_int = 2323;
pub const SQLITE_CONSTRAINT_ROWID: ::core::ffi::c_int = 2579;
pub const SQLITE_NOTICE_RECOVER_WAL: ::core::ffi::c_int = 283;
pub const SQLITE_NOTICE_RECOVER_ROLLBACK: ::core::ffi::c_int = 539;
pub const SQLITE_NOTICE_RBU: ::core::ffi::c_int = 795;
pub const SQLITE_WARNING_AUTOINDEX: ::core::ffi::c_int = 284;
pub const SQLITE_OPEN_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_DELETEONCLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_EXCLUSIVE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_OPEN_URI: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MEMORY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_OPEN_TEMP_DB: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SQLITE_OPEN_TRANSIENT_DB: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_JOURNAL: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_OPEN_TEMP_JOURNAL: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_SUBJOURNAL: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_SUPER_JOURNAL: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_NOMUTEX: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_FULLMUTEX: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_SHAREDCACHE: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_PRIVATECACHE: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_WAL: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_EXRESCODE: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_FILE_POINTER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_VFS_POINTER: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_JOURNAL_POINTER: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_DATA_VERSION: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_RESERVE_BYTES: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_RESET_CACHE: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_SINGLETHREAD: ::core::ffi::c_int = 1;
pub const SQLITE_CONFIG_MULTITHREAD: ::core::ffi::c_int = 2;
pub const SQLITE_CONFIG_SERIALIZED: ::core::ffi::c_int = 3;
pub const SQLITE_CONFIG_MALLOC: ::core::ffi::c_int = 4;
pub const SQLITE_CONFIG_GETMALLOC: ::core::ffi::c_int = 5;
pub const SQLITE_CONFIG_PAGECACHE: ::core::ffi::c_int = 7;
pub const SQLITE_CONFIG_MEMSTATUS: ::core::ffi::c_int = 9;
pub const SQLITE_CONFIG_MUTEX: ::core::ffi::c_int = 10;
pub const SQLITE_CONFIG_GETMUTEX: ::core::ffi::c_int = 11;
pub const SQLITE_CONFIG_LOOKASIDE: ::core::ffi::c_int = 13;
pub const SQLITE_CONFIG_PCACHE: ::core::ffi::c_int = 14;
pub const SQLITE_CONFIG_GETPCACHE: ::core::ffi::c_int = 15;
pub const SQLITE_CONFIG_LOG: ::core::ffi::c_int = 16;
pub const SQLITE_CONFIG_URI: ::core::ffi::c_int = 17;
pub const SQLITE_CONFIG_PCACHE2: ::core::ffi::c_int = 18;
pub const SQLITE_CONFIG_GETPCACHE2: ::core::ffi::c_int = 19;
pub const SQLITE_CONFIG_COVERING_INDEX_SCAN: ::core::ffi::c_int = 20;
pub const SQLITE_CONFIG_MMAP_SIZE: ::core::ffi::c_int = 22;
pub const SQLITE_CONFIG_PCACHE_HDRSZ: ::core::ffi::c_int = 24;
pub const SQLITE_CONFIG_PMASZ: ::core::ffi::c_int = 25;
pub const SQLITE_CONFIG_STMTJRNL_SPILL: ::core::ffi::c_int = 26;
pub const SQLITE_CONFIG_SMALL_MALLOC: ::core::ffi::c_int = 27;
pub const SQLITE_CONFIG_MEMDB_MAXSIZE: ::core::ffi::c_int = 29;
pub const SQLITE_CONFIG_ROWID_IN_VIEW: ::core::ffi::c_int = 30;
pub const SQLITE_DBCONFIG_MAINDBNAME: ::core::ffi::c_int = 1000;
pub const SQLITE_DBCONFIG_LOOKASIDE: ::core::ffi::c_int = 1001;
pub const SQLITE_DBCONFIG_ENABLE_FKEY: ::core::ffi::c_int = 1002 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_TRIGGER: ::core::ffi::c_int = 1003
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER: ::core::ffi::c_int = 1004
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION: ::core::ffi::c_int = 1005
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE: ::core::ffi::c_int = 1006
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_QPSG: ::core::ffi::c_int = 1007 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_TRIGGER_EQP: ::core::ffi::c_int = 1008 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_RESET_DATABASE: ::core::ffi::c_int = 1009
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_DEFENSIVE: ::core::ffi::c_int = 1010 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_WRITABLE_SCHEMA: ::core::ffi::c_int = 1011
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_LEGACY_ALTER_TABLE: ::core::ffi::c_int = 1012
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_DQS_DML: ::core::ffi::c_int = 1013 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_DQS_DDL: ::core::ffi::c_int = 1014 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_VIEW: ::core::ffi::c_int = 1015 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_LEGACY_FILE_FORMAT: ::core::ffi::c_int = 1016
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_TRUSTED_SCHEMA: ::core::ffi::c_int = 1017
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_STMT_SCANSTATUS: ::core::ffi::c_int = 1018
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_REVERSE_SCANORDER: ::core::ffi::c_int = 1019
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_ATTACH_CREATE: ::core::ffi::c_int = 1020
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_ATTACH_WRITE: ::core::ffi::c_int = 1021
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_COMMENTS: ::core::ffi::c_int = 1022
    as ::core::ffi::c_int;
pub const SQLITE_TRACE_CLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LENGTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_WORKER_THREADS: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_UTF16LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_UTF16BE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_UTF16: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_ANY: ::core::ffi::c_int = 5;
pub const SQLITE_UTF16_ALIGNED: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_DIRECTONLY: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_SUBTYPE: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const SQLITE_INNOCUOUS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SQLITE_RESULT_SUBTYPE: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const SQLITE_SELFORDER1: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_TXN_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_TXN_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_RECURSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_PRNG_SAVE: ::core::ffi::c_int = 5;
pub const SQLITE_TESTCTRL_PRNG_RESTORE: ::core::ffi::c_int = 6;
pub const SQLITE_TESTCTRL_FK_NO_ACTION: ::core::ffi::c_int = 7;
pub const SQLITE_TESTCTRL_BITVEC_TEST: ::core::ffi::c_int = 8;
pub const SQLITE_TESTCTRL_FAULT_INSTALL: ::core::ffi::c_int = 9;
pub const SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS: ::core::ffi::c_int = 10;
pub const SQLITE_TESTCTRL_PENDING_BYTE: ::core::ffi::c_int = 11;
pub const SQLITE_TESTCTRL_ASSERT: ::core::ffi::c_int = 12;
pub const SQLITE_TESTCTRL_ALWAYS: ::core::ffi::c_int = 13;
pub const SQLITE_TESTCTRL_JSON_SELFCHECK: ::core::ffi::c_int = 14;
pub const SQLITE_TESTCTRL_OPTIMIZATIONS: ::core::ffi::c_int = 15;
pub const SQLITE_TESTCTRL_GETOPT: ::core::ffi::c_int = 16;
pub const SQLITE_TESTCTRL_INTERNAL_FUNCTIONS: ::core::ffi::c_int = 17;
pub const SQLITE_TESTCTRL_LOCALTIME_FAULT: ::core::ffi::c_int = 18;
pub const SQLITE_TESTCTRL_ONCE_RESET_THRESHOLD: ::core::ffi::c_int = 19;
pub const SQLITE_TESTCTRL_NEVER_CORRUPT: ::core::ffi::c_int = 20;
pub const SQLITE_TESTCTRL_VDBE_COVERAGE: ::core::ffi::c_int = 21;
pub const SQLITE_TESTCTRL_BYTEORDER: ::core::ffi::c_int = 22;
pub const SQLITE_TESTCTRL_ISINIT: ::core::ffi::c_int = 23;
pub const SQLITE_TESTCTRL_SORTER_MMAP: ::core::ffi::c_int = 24;
pub const SQLITE_TESTCTRL_IMPOSTER: ::core::ffi::c_int = 25;
pub const SQLITE_TESTCTRL_RESULT_INTREAL: ::core::ffi::c_int = 27;
pub const SQLITE_TESTCTRL_PRNG_SEED: ::core::ffi::c_int = 28;
pub const SQLITE_TESTCTRL_EXTRA_SCHEMA_CHECKS: ::core::ffi::c_int = 29;
pub const SQLITE_TESTCTRL_SEEK_COUNT: ::core::ffi::c_int = 30;
pub const SQLITE_TESTCTRL_TRACEFLAGS: ::core::ffi::c_int = 31;
pub const SQLITE_TESTCTRL_LOGEST: ::core::ffi::c_int = 33;
pub const SQLITE_CHECKPOINT_PASSIVE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CHECKPOINT_TRUNCATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_MAX_WORKER_THREADS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_WORKER_THREADS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_BYTEORDER: ::core::ffi::c_int = 1234 as ::core::ffi::c_int;
pub const SQLITE_BIGENDIAN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_LITTLEENDIAN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_UTF16NATIVE: ::core::ffi::c_int = SQLITE_UTF16LE;
pub const SQLITE_MAX_MMAP_SIZE: ::core::ffi::c_int = 0x7fff0000 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_MMAP_SIZE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PAGER_SYNCHRONOUS_OFF: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_MAX_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
pub const SQLITE_MIN_LENGTH: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const SQLITE_MAX_COLUMN: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
pub const SQLITE_MAX_SQL_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
pub const SQLITE_MAX_EXPR_DEPTH: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const SQLITE_MAX_COMPOUND_SELECT: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const SQLITE_MAX_VDBE_OP: ::core::ffi::c_int = 250000000 as ::core::ffi::c_int;
pub const SQLITE_MAX_FUNCTION_ARG: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_WAL_AUTOCHECKPOINT: ::core::ffi::c_int = 1000
    as ::core::ffi::c_int;
pub const SQLITE_MAX_ATTACHED: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_MAX_VARIABLE_NUMBER: ::core::ffi::c_int = 32766 as ::core::ffi::c_int;
pub const SQLITE_MAX_LIKE_PATTERN_LENGTH: ::core::ffi::c_int = 50000
    as ::core::ffi::c_int;
pub const SQLITE_MAX_TRIGGER_DEPTH: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_SYNCHRONOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_N_LIMIT: ::core::ffi::c_int = SQLITE_LIMIT_WORKER_THREADS
    + 1 as ::core::ffi::c_int;
pub const LOOKASIDE_SMALL: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const SQLITE_TRACE_LEGACY: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_TRACE_XPROFILE: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_TRACE_NONLEGACY_MASK: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;
pub const SQLITE_MAX_DB: ::core::ffi::c_int = SQLITE_MAX_ATTACHED
    + 2 as ::core::ffi::c_int;
pub const SQLITE_WriteSchema: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_LegacyFileFmt: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_CacheSpill: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_ShortColNames: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_TrustedSchema: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_StmtScanStatus: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SQLITE_NoCkptOnClose: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_ReverseOrder: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const SQLITE_ForeignKeys: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_AutoIndex: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_LoadExtension: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const SQLITE_EnableTrigger: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const SQLITE_DeferFKs: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_Fts3Tokenizer: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
pub const SQLITE_EnableQPSG: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
pub const SQLITE_TriggerEQP: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const SQLITE_ResetDatabase: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const SQLITE_LegacyAlter: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const SQLITE_NoSchemaError: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const SQLITE_Defensive: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SQLITE_DqsDDL: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const SQLITE_DqsDML: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
pub const SQLITE_EnableView: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const SQLITE_CorruptRdOnly: u64_0 = (0x2 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const SQLITE_FkNoAction: u64_0 = (0x8 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const SQLITE_AttachCreate: u64_0 = (0x10 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const SQLITE_AttachWrite: u64_0 = (0x20 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const SQLITE_Comments: u64_0 = (0x40 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const DBFLAG_SchemaChange: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const DBFLAG_InternalFunc: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_STATE_OPEN: ::core::ffi::c_int = 0x76 as ::core::ffi::c_int;
pub const SQLITE_STATE_CLOSED: ::core::ffi::c_int = 0xce as ::core::ffi::c_int;
pub const SQLITE_STATE_SICK: ::core::ffi::c_int = 0xba as ::core::ffi::c_int;
pub const SQLITE_STATE_BUSY: ::core::ffi::c_int = 0x6d as ::core::ffi::c_int;
pub const SQLITE_STATE_ERROR: ::core::ffi::c_int = 0xd5 as ::core::ffi::c_int;
pub const SQLITE_STATE_ZOMBIE: ::core::ffi::c_int = 0xa7 as ::core::ffi::c_int;
pub const SQLITE_FUNC_ENCMASK: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
pub const SQLITE_FUNC_UNSAFE: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const COLFLAG_PRIMKEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const TF_Autoincrement: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TABTYP_VIEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
#[no_mangle]
pub static mut sqlite3_version: [::core::ffi::c_char; 7] = SQLITE_VERSION;
#[no_mangle]
pub unsafe extern "C" fn sqlite3_libversion() -> *const ::core::ffi::c_char {
    unsafe {
        return &raw const sqlite3_version as *const ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_sourceid() -> *const ::core::ffi::c_char {
    unsafe {
        return SQLITE_SOURCE_ID.as_ptr();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_libversion_number() -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_VERSION_NUMBER;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_threadsafe() -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_THREADSAFE;
    }
}
pub const SQLITE_DEBUG_OS_TRACE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3OSTrace: ::core::ffi::c_int = SQLITE_DEBUG_OS_TRACE;
#[no_mangle]
pub static mut sqlite3_temp_directory: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut sqlite3_data_directory: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn sqlite3_initialize() -> ::core::ffi::c_int {
    unsafe {
        let mut pMainMtx: *mut sqlite3_mutex = ::core::ptr::null_mut::<sqlite3_mutex>();
        let mut rc: ::core::ffi::c_int = 0;
        if sqlite3Config.isInit != 0 {
            sqlite3MemoryBarrier();
            return SQLITE_OK;
        }
        rc = sqlite3MutexInit();
        if rc != 0 {
            return rc;
        }
        pMainMtx = sqlite3MutexAlloc(2 as ::core::ffi::c_int);
        sqlite3_mutex_enter(pMainMtx);
        sqlite3Config.isMutexInit = 1 as ::core::ffi::c_int;
        if sqlite3Config.isMallocInit == 0 {
            rc = sqlite3MallocInit();
        }
        if rc == SQLITE_OK {
            sqlite3Config.isMallocInit = 1 as ::core::ffi::c_int;
            if sqlite3Config.pInitMutex.is_null() {
                sqlite3Config.pInitMutex = sqlite3MutexAlloc(SQLITE_MUTEX_RECURSIVE);
                if sqlite3Config.bCoreMutex as ::core::ffi::c_int != 0
                    && sqlite3Config.pInitMutex.is_null()
                {
                    rc = SQLITE_NOMEM_BKPT;
                }
            }
        }
        if rc == SQLITE_OK {
            sqlite3Config.nRefInitMutex += 1;
        }
        sqlite3_mutex_leave(pMainMtx);
        if rc != SQLITE_OK {
            return rc;
        }
        sqlite3_mutex_enter(sqlite3Config.pInitMutex);
        if sqlite3Config.isInit == 0 as ::core::ffi::c_int
            && sqlite3Config.inProgress == 0 as ::core::ffi::c_int
        {
            sqlite3Config.inProgress = 1 as ::core::ffi::c_int;
            memset(
                &raw mut sqlite3BuiltinFunctions as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<FuncDefHash>() as size_t,
            );
            sqlite3RegisterBuiltinFunctions();
            if sqlite3Config.isPCacheInit == 0 as ::core::ffi::c_int {
                rc = sqlite3PcacheInitialize();
            }
            if rc == SQLITE_OK {
                sqlite3Config.isPCacheInit = 1 as ::core::ffi::c_int;
                rc = sqlite3OsInit();
            }
            if rc == SQLITE_OK {
                rc = sqlite3MemdbInit();
            }
            if rc == SQLITE_OK {
                sqlite3PCacheBufferSetup(
                    sqlite3Config.pPage,
                    sqlite3Config.szPage,
                    sqlite3Config.nPage,
                );
            }
            if rc == SQLITE_OK {
                sqlite3MemoryBarrier();
                sqlite3Config.isInit = 1 as ::core::ffi::c_int;
            }
            sqlite3Config.inProgress = 0 as ::core::ffi::c_int;
        }
        sqlite3_mutex_leave(sqlite3Config.pInitMutex);
        sqlite3_mutex_enter(pMainMtx);
        sqlite3Config.nRefInitMutex -= 1;
        if sqlite3Config.nRefInitMutex <= 0 as ::core::ffi::c_int {
            sqlite3_mutex_free(sqlite3Config.pInitMutex);
            sqlite3Config.pInitMutex = ::core::ptr::null_mut::<sqlite3_mutex>();
        }
        sqlite3_mutex_leave(pMainMtx);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_shutdown() -> ::core::ffi::c_int {
    unsafe {
        if sqlite3Config.isInit != 0 {
            sqlite3_os_end();
            sqlite3_reset_auto_extension();
            sqlite3Config.isInit = 0 as ::core::ffi::c_int;
        }
        if sqlite3Config.isPCacheInit != 0 {
            sqlite3PcacheShutdown();
            sqlite3Config.isPCacheInit = 0 as ::core::ffi::c_int;
        }
        if sqlite3Config.isMallocInit != 0 {
            sqlite3MallocEnd();
            sqlite3Config.isMallocInit = 0 as ::core::ffi::c_int;
            sqlite3_data_directory = ::core::ptr::null_mut::<::core::ffi::c_char>();
            sqlite3_temp_directory = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if sqlite3Config.isMutexInit != 0 {
            sqlite3MutexEnd();
            sqlite3Config.isMutexInit = 0 as ::core::ffi::c_int;
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_config(
    mut op: ::core::ffi::c_int,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    unsafe {
        
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if sqlite3Config.isInit != 0 {
            static mut mAnytimeConfigOption: u64_0 = 0 as u64_0
                | (1 as ::core::ffi::c_int as u64_0) << 16 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int as u64_0) << 24 as ::core::ffi::c_int;
            if op < 0 as ::core::ffi::c_int || op > 63 as ::core::ffi::c_int
                || (1 as ::core::ffi::c_int as u64_0) << op & mAnytimeConfigOption
                    == 0 as ::core::ffi::c_ulonglong
            {
                return sqlite3MisuseError(440 as ::core::ffi::c_int);
            }
        }
        let mut ap = c2rust_args;
        match op {
            SQLITE_CONFIG_SINGLETHREAD => {
                sqlite3Config.bCoreMutex = 0 as u8_0;
                sqlite3Config.bFullMutex = 0 as u8_0;
            }
            SQLITE_CONFIG_MULTITHREAD => {
                sqlite3Config.bCoreMutex = 1 as u8_0;
                sqlite3Config.bFullMutex = 0 as u8_0;
            }
            SQLITE_CONFIG_SERIALIZED => {
                sqlite3Config.bCoreMutex = 1 as u8_0;
                sqlite3Config.bFullMutex = 1 as u8_0;
            }
            SQLITE_CONFIG_MUTEX => {
                sqlite3Config.mutex = *ap.arg::<*mut sqlite3_mutex_methods>();
            }
            SQLITE_CONFIG_GETMUTEX => {
                *ap.arg::<*mut sqlite3_mutex_methods>() = sqlite3Config.mutex;
            }
            SQLITE_CONFIG_MALLOC => {
                sqlite3Config.m = *ap.arg::<*mut sqlite3_mem_methods>();
            }
            SQLITE_CONFIG_GETMALLOC => {
                if sqlite3Config.m.xMalloc.is_none() {
                    sqlite3MemSetDefault();
                }
                *ap.arg::<*mut sqlite3_mem_methods>() = sqlite3Config.m;
            }
            SQLITE_CONFIG_MEMSTATUS => {
                sqlite3Config.bMemstat = ap.arg::<::core::ffi::c_int>();
            }
            SQLITE_CONFIG_SMALL_MALLOC => {
                sqlite3Config.bSmallMalloc = ap.arg::<::core::ffi::c_int>() as u8_0;
            }
            SQLITE_CONFIG_PAGECACHE => {
                sqlite3Config.pPage = ap.arg::<*mut ::core::ffi::c_void>();
                sqlite3Config.szPage = ap.arg::<::core::ffi::c_int>();
                sqlite3Config.nPage = ap.arg::<::core::ffi::c_int>();
            }
            SQLITE_CONFIG_PCACHE_HDRSZ => {
                *ap.arg::<*mut ::core::ffi::c_int>() = sqlite3HeaderSizeBtree()
                    + sqlite3HeaderSizePcache() + sqlite3HeaderSizePcache1();
            }
            SQLITE_CONFIG_PCACHE => {}
            SQLITE_CONFIG_GETPCACHE => {
                rc = SQLITE_ERROR;
            }
            SQLITE_CONFIG_PCACHE2 => {
                sqlite3Config.pcache2 = *ap.arg::<*mut sqlite3_pcache_methods2>();
            }
            SQLITE_CONFIG_GETPCACHE2 => {
                if sqlite3Config.pcache2.xInit.is_none() {
                    sqlite3PCacheSetDefault();
                }
                *ap.arg::<*mut sqlite3_pcache_methods2>() = sqlite3Config.pcache2;
            }
            SQLITE_CONFIG_LOOKASIDE => {
                sqlite3Config.szLookaside = ap.arg::<::core::ffi::c_int>();
                sqlite3Config.nLookaside = ap.arg::<::core::ffi::c_int>();
            }
            SQLITE_CONFIG_LOG => {
                let mut xLog: LOGFUNC_t = ::core::mem::transmute(
                    ap
                        .arg::<
                            *mut unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_char,
                            ) -> (),
                        >(),
                );
                let mut pLogArg: *mut ::core::ffi::c_void = ap
                    .arg::<*mut ::core::ffi::c_void>();
                ;
                ;
            }
            SQLITE_CONFIG_URI => {
                let mut bOpenUri: ::core::ffi::c_int = ap.arg::<::core::ffi::c_int>();
                ;
            }
            SQLITE_CONFIG_COVERING_INDEX_SCAN => {
                sqlite3Config.bUseCis = ap.arg::<::core::ffi::c_int>() as u8_0;
            }
            SQLITE_CONFIG_MMAP_SIZE => {
                let mut szMmap: sqlite3_int64 = ap.arg::<sqlite3_int64>();
                let mut mxMmap: sqlite3_int64 = ap.arg::<sqlite3_int64>();
                if mxMmap < 0 as ::core::ffi::c_longlong
                    || mxMmap > SQLITE_MAX_MMAP_SIZE as ::core::ffi::c_longlong
                {
                    mxMmap = SQLITE_MAX_MMAP_SIZE as sqlite3_int64;
                }
                if szMmap < 0 as ::core::ffi::c_longlong {
                    szMmap = SQLITE_DEFAULT_MMAP_SIZE as sqlite3_int64;
                }
                if szMmap > mxMmap {
                    szMmap = mxMmap;
                }
                sqlite3Config.mxMmap = mxMmap;
                sqlite3Config.szMmap = szMmap;
            }
            SQLITE_CONFIG_PMASZ => {
                sqlite3Config.szPma = ap.arg::<::core::ffi::c_uint>() as u32_0;
            }
            SQLITE_CONFIG_STMTJRNL_SPILL => {
                sqlite3Config.nStmtSpill = ap.arg::<::core::ffi::c_int>();
            }
            SQLITE_CONFIG_MEMDB_MAXSIZE => {
                sqlite3Config.mxMemdbSize = ap.arg::<sqlite3_int64>();
            }
            SQLITE_CONFIG_ROWID_IN_VIEW => {
                let mut pVal: *mut ::core::ffi::c_int = ap
                    .arg::<*mut ::core::ffi::c_int>();
                *pVal = 0 as ::core::ffi::c_int;
            }
            _ => {
                rc = SQLITE_ERROR;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn setupLookaside(
    mut db: *mut sqlite3,
    mut pBuf: *mut ::core::ffi::c_void,
    mut sz: ::core::ffi::c_int,
    mut cnt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStart: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut szAlloc: sqlite3_int64 = 0;
        let mut nBig: ::core::ffi::c_int = 0;
        let mut nSm: ::core::ffi::c_int = 0;
        if sqlite3LookasideUsed(db, ::core::ptr::null_mut::<::core::ffi::c_int>())
            > 0 as ::core::ffi::c_int
        {
            return SQLITE_BUSY;
        }
        if (*db).lookaside.bMalloced != 0 {
            sqlite3_free((*db).lookaside.pStart);
        }
        sz = sz & !(7 as ::core::ffi::c_int);
        if sz <= ::core::mem::size_of::<*mut LookasideSlot>() as ::core::ffi::c_int {
            sz = 0 as ::core::ffi::c_int;
        }
        if sz > 65528 as ::core::ffi::c_int {
            sz = 65528 as ::core::ffi::c_int;
        }
        if cnt < 1 as ::core::ffi::c_int {
            cnt = 0 as ::core::ffi::c_int;
        }
        if sz > 0 as ::core::ffi::c_int && cnt > 0x7fff0000 as ::core::ffi::c_int / sz {
            cnt = 0x7fff0000 as ::core::ffi::c_int / sz;
        }
        szAlloc = (sz as i64_0 * cnt as i64_0) as sqlite3_int64;
        if szAlloc == 0 as ::core::ffi::c_longlong {
            sz = 0 as ::core::ffi::c_int;
            pStart = ::core::ptr::null_mut::<::core::ffi::c_void>();
        } else if pBuf.is_null() {
            sqlite3BeginBenignMalloc();
            pStart = sqlite3Malloc(szAlloc as u64_0);
            sqlite3EndBenignMalloc();
            if !pStart.is_null() {
                szAlloc = sqlite3MallocSize(pStart) as sqlite3_int64;
            }
        } else {
            pStart = pBuf;
        }
        if sz >= LOOKASIDE_SMALL * 3 as ::core::ffi::c_int {
            nBig = (szAlloc as ::core::ffi::c_longlong
                / (3 as ::core::ffi::c_int * LOOKASIDE_SMALL + sz)
                    as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            nSm = ((szAlloc as ::core::ffi::c_longlong
                - sz as ::core::ffi::c_longlong * nBig as ::core::ffi::c_longlong)
                / LOOKASIDE_SMALL as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        } else if sz >= LOOKASIDE_SMALL * 2 as ::core::ffi::c_int {
            nBig = (szAlloc as ::core::ffi::c_longlong
                / (LOOKASIDE_SMALL + sz) as ::core::ffi::c_longlong)
                as ::core::ffi::c_int;
            nSm = ((szAlloc as ::core::ffi::c_longlong
                - sz as ::core::ffi::c_longlong * nBig as ::core::ffi::c_longlong)
                / LOOKASIDE_SMALL as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        } else if sz > 0 as ::core::ffi::c_int {
            nBig = (szAlloc as ::core::ffi::c_longlong / sz as ::core::ffi::c_longlong)
                as ::core::ffi::c_int;
            nSm = 0 as ::core::ffi::c_int;
        } else {
            nSm = 0 as ::core::ffi::c_int;
            nBig = nSm;
        }
        (*db).lookaside.pStart = pStart;
        (*db).lookaside.pInit = ::core::ptr::null_mut::<LookasideSlot>();
        (*db).lookaside.pFree = ::core::ptr::null_mut::<LookasideSlot>();
        (*db).lookaside.sz = sz as u16_0;
        (*db).lookaside.szTrue = sz as u16_0;
        if !pStart.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            let mut p: *mut LookasideSlot = ::core::ptr::null_mut::<LookasideSlot>();
            p = pStart as *mut LookasideSlot;
            i = 0 as ::core::ffi::c_int;
            while i < nBig {
                (*p).pNext = (*db).lookaside.pInit;
                (*db).lookaside.pInit = p;
                p = (p as *mut u8_0).offset(sz as isize) as *mut u8_0
                    as *mut LookasideSlot;
                i += 1;
            }
            (*db).lookaside.pSmallInit = ::core::ptr::null_mut::<LookasideSlot>();
            (*db).lookaside.pSmallFree = ::core::ptr::null_mut::<LookasideSlot>();
            (*db).lookaside.pMiddle = p as *mut ::core::ffi::c_void;
            i = 0 as ::core::ffi::c_int;
            while i < nSm {
                (*p).pNext = (*db).lookaside.pSmallInit;
                (*db).lookaside.pSmallInit = p;
                p = (p as *mut u8_0).offset(LOOKASIDE_SMALL as isize) as *mut u8_0
                    as *mut LookasideSlot;
                i += 1;
            }
            (*db).lookaside.pEnd = p as *mut ::core::ffi::c_void;
            (*db).lookaside.bDisable = 0 as u32_0;
            (*db).lookaside.bMalloced = (if pBuf.is_null() {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as u8_0;
            (*db).lookaside.nSlot = (nBig + nSm) as u32_0;
        } else {
            (*db).lookaside.pStart = ::core::ptr::null_mut::<::core::ffi::c_void>();
            (*db).lookaside.pSmallInit = ::core::ptr::null_mut::<LookasideSlot>();
            (*db).lookaside.pSmallFree = ::core::ptr::null_mut::<LookasideSlot>();
            (*db).lookaside.pMiddle = ::core::ptr::null_mut::<::core::ffi::c_void>();
            (*db).lookaside.pEnd = ::core::ptr::null_mut::<::core::ffi::c_void>();
            (*db).lookaside.bDisable = 1 as u32_0;
            (*db).lookaside.sz = 0 as u16_0;
            (*db).lookaside.bMalloced = 0 as u8_0;
            (*db).lookaside.nSlot = 0 as u32_0;
        }
        (*db).lookaside.pTrueEnd = (*db).lookaside.pEnd;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_db_mutex(mut db: *mut sqlite3) -> *mut sqlite3_mutex {
    unsafe {
        return (*db).mutex;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_db_release_memory(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        sqlite3_mutex_enter((*db).mutex);
        sqlite3BtreeEnterAll(db);
        i = 0 as ::core::ffi::c_int;
        while i < (*db).nDb {
            let mut pBt: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
            if !pBt.is_null() {
                let mut pPager: *mut Pager = sqlite3BtreePager(pBt) as *mut Pager;
                sqlite3PagerShrink(pPager);
            }
            i += 1;
        }
        sqlite3BtreeLeaveAll(db);
        sqlite3_mutex_leave((*db).mutex);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_db_cacheflush(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut bSeenBusy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        sqlite3_mutex_enter((*db).mutex);
        sqlite3BtreeEnterAll(db);
        i = 0 as ::core::ffi::c_int;
        while rc == SQLITE_OK && i < (*db).nDb {
            let mut pBt: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
            if !pBt.is_null() && sqlite3BtreeTxnState(pBt) == SQLITE_TXN_WRITE {
                let mut pPager: *mut Pager = sqlite3BtreePager(pBt) as *mut Pager;
                rc = sqlite3PagerFlush(pPager);
                if rc == SQLITE_BUSY {
                    bSeenBusy = 1 as ::core::ffi::c_int;
                    rc = SQLITE_OK;
                }
            }
            i += 1;
        }
        sqlite3BtreeLeaveAll(db);
        sqlite3_mutex_leave((*db).mutex);
        return if rc == SQLITE_OK && bSeenBusy != 0 { SQLITE_BUSY } else { rc };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_db_config(
    mut db: *mut sqlite3,
    mut op: ::core::ffi::c_int,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    unsafe {
        
        let mut rc: ::core::ffi::c_int = 0;
        sqlite3_mutex_enter((*db).mutex);
        let mut ap = c2rust_args;
        match op {
            SQLITE_DBCONFIG_MAINDBNAME => {
                let ref mut c2rust_fresh0 = (*(*db)
                    .aDb
                    .offset(0 as ::core::ffi::c_int as isize))
                    .zDbSName;
                *c2rust_fresh0 = ap.arg::<*mut ::core::ffi::c_char>();
                rc = SQLITE_OK;
            }
            SQLITE_DBCONFIG_LOOKASIDE => {
                let mut pBuf: *mut ::core::ffi::c_void = ap
                    .arg::<*mut ::core::ffi::c_void>();
                let mut sz: ::core::ffi::c_int = ap.arg::<::core::ffi::c_int>();
                let mut cnt: ::core::ffi::c_int = ap.arg::<::core::ffi::c_int>();
                rc = setupLookaside(db, pBuf, sz, cnt);
            }
            _ => {
                static mut aFlagOp: [C2Rust_Unnamed_22; 21] = [
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_ENABLE_FKEY,
                        mask: SQLITE_ForeignKeys as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_ENABLE_TRIGGER,
                        mask: SQLITE_EnableTrigger as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_ENABLE_VIEW,
                        mask: SQLITE_EnableView as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER,
                        mask: SQLITE_Fts3Tokenizer as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION,
                        mask: SQLITE_LoadExtension as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE,
                        mask: SQLITE_NoCkptOnClose as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_ENABLE_QPSG,
                        mask: SQLITE_EnableQPSG as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_TRIGGER_EQP,
                        mask: SQLITE_TriggerEQP as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_RESET_DATABASE,
                        mask: SQLITE_ResetDatabase as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_DEFENSIVE,
                        mask: SQLITE_Defensive as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_WRITABLE_SCHEMA,
                        mask: (SQLITE_WriteSchema | SQLITE_NoSchemaError) as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_LEGACY_ALTER_TABLE,
                        mask: SQLITE_LegacyAlter as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_DQS_DDL,
                        mask: SQLITE_DqsDDL as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_DQS_DML,
                        mask: SQLITE_DqsDML as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_LEGACY_FILE_FORMAT,
                        mask: SQLITE_LegacyFileFmt as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_TRUSTED_SCHEMA,
                        mask: SQLITE_TrustedSchema as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_STMT_SCANSTATUS,
                        mask: SQLITE_StmtScanStatus as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_REVERSE_SCANORDER,
                        mask: SQLITE_ReverseOrder as u64_0

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_ENABLE_ATTACH_CREATE,
                        mask: SQLITE_AttachCreate

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_ENABLE_ATTACH_WRITE,
                        mask: SQLITE_AttachWrite

                    },
                    C2Rust_Unnamed_22 {
                        op: SQLITE_DBCONFIG_ENABLE_COMMENTS,
                        mask: SQLITE_Comments

                    },
                ];
                let mut i: ::core::ffi::c_uint = 0;
                rc = SQLITE_ERROR;
                i = 0 as ::core::ffi::c_uint;
                while i
                    < (::core::mem::size_of::<[C2Rust_Unnamed_22; 21]>() as usize)
                        .wrapping_div(
                            ::core::mem::size_of::<C2Rust_Unnamed_22>() as usize,
                        ) as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    if aFlagOp[i as usize].op == op {
                        let mut onoff: ::core::ffi::c_int = ap
                            .arg::<::core::ffi::c_int>();
                        let mut pRes: *mut ::core::ffi::c_int = ap
                            .arg::<*mut ::core::ffi::c_int>();
                        let mut oldFlags: u64_0 = (*db).flags;
                        if onoff > 0 as ::core::ffi::c_int {
                            (*db).flags
                                |= aFlagOp[i as usize].mask as ::core::ffi::c_ulonglong;
                        } else if onoff == 0 as ::core::ffi::c_int {
                            (*db).flags
                                &= !aFlagOp[i as usize].mask as ::core::ffi::c_ulonglong;
                        }
                        if oldFlags != (*db).flags {
                            sqlite3ExpirePreparedStatements(db, 0 as ::core::ffi::c_int);
                        }
                        if !pRes.is_null() {
                            *pRes = ((*db).flags & aFlagOp[i as usize].mask
                                != 0 as ::core::ffi::c_ulonglong) as ::core::ffi::c_int;
                        }
                        rc = SQLITE_OK;
                        break;
                    } else {
                        i = i.wrapping_add(1);
                    }
                }
            }
        }
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
unsafe extern "C" fn binCollFunc(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut nKey1: ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut nKey2: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        n = if nKey1 < nKey2 { nKey1 } else { nKey2 };
        rc = memcmp(pKey1, pKey2, n as size_t);
        if rc == 0 as ::core::ffi::c_int {
            rc = nKey1 - nKey2;
        }
        return rc;
    }
}
unsafe extern "C" fn rtrimCollFunc(
    mut pUser: *mut ::core::ffi::c_void,
    mut nKey1: ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut nKey2: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pK1: *const u8_0 = pKey1 as *const u8_0;
        let mut pK2: *const u8_0 = pKey2 as *const u8_0;
        while nKey1 != 0
            && *pK1.offset((nKey1 - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == ' ' as i32
        {
            nKey1 -= 1;
        }
        while nKey2 != 0
            && *pK2.offset((nKey2 - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == ' ' as i32
        {
            nKey2 -= 1;
        }
        return binCollFunc(pUser, nKey1, pKey1, nKey2, pKey2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IsBinary(mut p: *const CollSeq) -> ::core::ffi::c_int {
    unsafe {
        return (p.is_null()
            || (*p).xCmp
                == Some(
                    binCollFunc
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                )) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn nocaseCollatingFunc(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut nKey1: ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut nKey2: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut r: ::core::ffi::c_int = sqlite3_strnicmp(
            pKey1 as *const ::core::ffi::c_char,
            pKey2 as *const ::core::ffi::c_char,
            if nKey1 < nKey2 { nKey1 } else { nKey2 },
        );
        if 0 as ::core::ffi::c_int == r {
            r = nKey1 - nKey2;
        }
        return r;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_last_insert_rowid(
    mut db: *mut sqlite3,
) -> sqlite3_int64 {
    unsafe {
        return (*db).lastRowid as sqlite3_int64;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_set_last_insert_rowid(
    mut db: *mut sqlite3,
    mut iRowid: sqlite3_int64,
) {
    unsafe {
        sqlite3_mutex_enter((*db).mutex);
        (*db).lastRowid = iRowid as i64_0;
        sqlite3_mutex_leave((*db).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_changes64(mut db: *mut sqlite3) -> sqlite3_int64 {
    unsafe {
        return (*db).nChange as sqlite3_int64;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_changes(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3_changes64(db) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_total_changes64(mut db: *mut sqlite3) -> sqlite3_int64 {
    unsafe {
        return (*db).nTotalChange as sqlite3_int64;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_total_changes(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3_total_changes64(db) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CloseSavepoints(mut db: *mut sqlite3) {
    unsafe {
        while !(*db).pSavepoint.is_null() {
            let mut pTmp: *mut Savepoint = (*db).pSavepoint;
            (*db).pSavepoint = (*pTmp).pNext;
            sqlite3DbFree(db, pTmp as *mut ::core::ffi::c_void);
        }
        (*db).nSavepoint = 0 as ::core::ffi::c_int;
        (*db).nStatement = 0 as ::core::ffi::c_int;
        (*db).isTransactionSavepoint = 0 as u8_0;
    }
}
unsafe extern "C" fn functionDestroy(mut db: *mut sqlite3, mut p: *mut FuncDef) {
    unsafe {
        let mut pDestructor: *mut FuncDestructor = ::core::ptr::null_mut::<
            FuncDestructor,
        >();
        pDestructor = (*p).u.pDestructor;
        if !pDestructor.is_null() {
            (*pDestructor).nRef -= 1;
            if (*pDestructor).nRef == 0 as ::core::ffi::c_int {
                (*pDestructor)
                    .xDestroy
                    .expect("non-null function pointer")((*pDestructor).pUserData);
                sqlite3DbFree(db, pDestructor as *mut ::core::ffi::c_void);
            }
        }
    }
}
unsafe extern "C" fn disconnectAllVtab(mut db: *mut sqlite3) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut p: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        sqlite3BtreeEnterAll(db);
        i = 0 as ::core::ffi::c_int;
        while i < (*db).nDb {
            let mut pSchema: *mut Schema = (*(*db).aDb.offset(i as isize)).pSchema;
            if !pSchema.is_null() {
                p = (*pSchema).tblHash.first;
                while !p.is_null() {
                    let mut pTab: *mut Table = (*p).data as *mut Table;
                    if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
                        sqlite3VtabDisconnect(db, pTab);
                    }
                    p = (*p).next;
                }
            }
            i += 1;
        }
        p = (*db).aModule.first;
        while !p.is_null() {
            let mut pMod: *mut Module = (*p).data as *mut Module;
            if !(*pMod).pEpoTab.is_null() {
                sqlite3VtabDisconnect(db, (*pMod).pEpoTab);
            }
            p = (*p).next;
        }
        sqlite3VtabUnlockList(db);
        sqlite3BtreeLeaveAll(db);
    }
}
unsafe extern "C" fn connectionIsBusy(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        let mut j: ::core::ffi::c_int = 0;
        if !(*db).pVdbe.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        j = 0 as ::core::ffi::c_int;
        while j < (*db).nDb {
            let mut pBt: *mut Btree = (*(*db).aDb.offset(j as isize)).pBt;
            if !pBt.is_null() && sqlite3BtreeIsInBackup(pBt) != 0 {
                return 1 as ::core::ffi::c_int;
            }
            j += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn sqlite3Close(
    mut db: *mut sqlite3,
    mut forceZombie: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if db.is_null() {
            return SQLITE_OK;
        }
        if sqlite3SafetyCheckSickOrOk(db) == 0 {
            return sqlite3MisuseError(1253 as ::core::ffi::c_int);
        }
        sqlite3_mutex_enter((*db).mutex);
        if (*db).mTrace as ::core::ffi::c_int & SQLITE_TRACE_CLOSE != 0 {
            (*db)
                .trace
                .xV2
                .expect(
                    "non-null function pointer",
                )(
                SQLITE_TRACE_CLOSE as u32_0,
                (*db).pTraceArg,
                db as *mut ::core::ffi::c_void,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        disconnectAllVtab(db);
        sqlite3VtabRollback(db);
        if forceZombie == 0 && connectionIsBusy(db) != 0 {
            sqlite3ErrorWithMsg(
                db,
                SQLITE_BUSY,
                b"unable to close due to unfinalized statements or unfinished backups\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            sqlite3_mutex_leave((*db).mutex);
            return SQLITE_BUSY;
        }
        while !(*db).pDbData.is_null() {
            let mut p: *mut DbClientData = (*db).pDbData;
            (*db).pDbData = (*p).pNext;
            if (*p).xDestructor.is_some() {
                (*p).xDestructor.expect("non-null function pointer")((*p).pData);
            }
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
        (*db).eOpenState = SQLITE_STATE_ZOMBIE as u8_0;
        sqlite3LeaveMutexAndCloseZombie(db);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_txn_state(
    mut db: *mut sqlite3,
    mut zSchema: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iDb: ::core::ffi::c_int = 0;
        let mut nDb: ::core::ffi::c_int = 0;
        let mut iTxn: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        sqlite3_mutex_enter((*db).mutex);
        if !zSchema.is_null() {
            iDb = sqlite3FindDbName(db, zSchema);
            nDb = iDb;
            if iDb < 0 as ::core::ffi::c_int {
                nDb -= 1;
            }
        } else {
            iDb = 0 as ::core::ffi::c_int;
            nDb = (*db).nDb - 1 as ::core::ffi::c_int;
        }
        while iDb <= nDb {
            let mut pBt: *mut Btree = (*(*db).aDb.offset(iDb as isize)).pBt;
            let mut x: ::core::ffi::c_int = if !pBt.is_null() {
                sqlite3BtreeTxnState(pBt)
            } else {
                SQLITE_TXN_NONE
            };
            if x > iTxn {
                iTxn = x;
            }
            iDb += 1;
        }
        sqlite3_mutex_leave((*db).mutex);
        return iTxn;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_close(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3Close(db, 0 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_close_v2(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3Close(db, 1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3LeaveMutexAndCloseZombie(mut db: *mut sqlite3) {
    unsafe {
        let mut i: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut j: ::core::ffi::c_int = 0;
        if (*db).eOpenState as ::core::ffi::c_int != SQLITE_STATE_ZOMBIE
            || connectionIsBusy(db) != 0
        {
            sqlite3_mutex_leave((*db).mutex);
            return;
        }
        sqlite3RollbackAll(db, SQLITE_OK);
        sqlite3CloseSavepoints(db);
        j = 0 as ::core::ffi::c_int;
        while j < (*db).nDb {
            let mut pDb: *mut Db = (*db).aDb.offset(j as isize) as *mut Db;
            if !(*pDb).pBt.is_null() {
                sqlite3BtreeClose((*pDb).pBt);
                (*pDb).pBt = ::core::ptr::null_mut::<Btree>();
                if j != 1 as ::core::ffi::c_int {
                    (*pDb).pSchema = ::core::ptr::null_mut::<Schema>();
                }
            }
            j += 1;
        }
        if !(*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pSchema.is_null() {
            sqlite3SchemaClear(
                (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pSchema
                    as *mut ::core::ffi::c_void,
            );
        }
        sqlite3VtabUnlockList(db);
        sqlite3CollapseDatabaseArray(db);
        i = (*db).aFunc.first;
        while !i.is_null() {
            let mut pNext: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
            let mut p: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
            p = (*i).data as *mut FuncDef;
            loop {
                functionDestroy(db, p);
                pNext = (*p).pNext;
                sqlite3DbFree(db, p as *mut ::core::ffi::c_void);
                p = pNext;
                if p.is_null() {
                    break;
                }
            }
            i = (*i).next;
        }
        sqlite3HashClear(&raw mut (*db).aFunc);
        i = (*db).aCollSeq.first;
        while !i.is_null() {
            let mut pColl: *mut CollSeq = (*i).data as *mut CollSeq;
            j = 0 as ::core::ffi::c_int;
            while j < 3 as ::core::ffi::c_int {
                if (*pColl.offset(j as isize)).xDel.is_some() {
                    (*pColl.offset(j as isize))
                        .xDel
                        .expect(
                            "non-null function pointer",
                        )((*pColl.offset(j as isize)).pUser);
                }
                j += 1;
            }
            sqlite3DbFree(db, pColl as *mut ::core::ffi::c_void);
            i = (*i).next;
        }
        sqlite3HashClear(&raw mut (*db).aCollSeq);
        i = (*db).aModule.first;
        while !i.is_null() {
            let mut pMod: *mut Module = (*i).data as *mut Module;
            sqlite3VtabEponymousTableClear(db, pMod);
            sqlite3VtabModuleUnref(db, pMod);
            i = (*i).next;
        }
        sqlite3HashClear(&raw mut (*db).aModule);
        sqlite3Error(db, SQLITE_OK);
        sqlite3ValueFree((*db).pErr);
        sqlite3CloseExtensions(db);
        (*db).eOpenState = SQLITE_STATE_ERROR as u8_0;
        sqlite3DbFree(
            db,
            (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pSchema
                as *mut ::core::ffi::c_void,
        );
        if (*db).xAutovacDestr.is_some() {
            (*db)
                .xAutovacDestr
                .expect("non-null function pointer")((*db).pAutovacPagesArg);
        }
        sqlite3_mutex_leave((*db).mutex);
        (*db).eOpenState = SQLITE_STATE_CLOSED as u8_0;
        sqlite3_mutex_free((*db).mutex);
        if (*db).lookaside.bMalloced != 0 {
            sqlite3_free((*db).lookaside.pStart);
        }
        sqlite3_free(db as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RollbackAll(
    mut db: *mut sqlite3,
    mut tripCode: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut inTrans: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut schemaChange: ::core::ffi::c_int = 0;
        sqlite3BeginBenignMalloc();
        sqlite3BtreeEnterAll(db);
        schemaChange = ((*db).mDbFlags & DBFLAG_SchemaChange as u32_0 != 0 as u32_0
            && (*db).init.busy as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*db).nDb {
            let mut p: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
            if !p.is_null() {
                if sqlite3BtreeTxnState(p) == SQLITE_TXN_WRITE {
                    inTrans = 1 as ::core::ffi::c_int;
                }
                sqlite3BtreeRollback(
                    p,
                    tripCode,
                    (schemaChange == 0) as ::core::ffi::c_int,
                );
            }
            i += 1;
        }
        sqlite3VtabRollback(db);
        sqlite3EndBenignMalloc();
        if schemaChange != 0 {
            sqlite3ExpirePreparedStatements(db, 0 as ::core::ffi::c_int);
            sqlite3ResetAllSchemasOfConnection(db);
        }
        sqlite3BtreeLeaveAll(db);
        (*db).nDeferredCons = 0 as i64_0;
        (*db).nDeferredImmCons = 0 as i64_0;
        (*db).flags
            &= !(SQLITE_DeferFKs as u64_0 | SQLITE_CorruptRdOnly)
                as ::core::ffi::c_ulonglong;
        if (*db).xRollbackCallback.is_some() && (inTrans != 0 || (*db).autoCommit == 0) {
            (*db)
                .xRollbackCallback
                .expect("non-null function pointer")((*db).pRollbackArg);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ErrName(
    mut rc: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut zName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut origRc: ::core::ffi::c_int = rc;
        i = 0 as ::core::ffi::c_int;
        while i < 2 as ::core::ffi::c_int && zName.is_null() {
            match rc {
                SQLITE_OK => {
                    zName = b"SQLITE_OK\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_ERROR => {
                    zName = b"SQLITE_ERROR\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_ERROR_SNAPSHOT => {
                    zName = b"SQLITE_ERROR_SNAPSHOT\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_ERROR_RETRY => {
                    zName = b"SQLITE_ERROR_RETRY\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_ERROR_MISSING_COLLSEQ => {
                    zName = b"SQLITE_ERROR_MISSING_COLLSEQ\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_INTERNAL => {
                    zName = b"SQLITE_INTERNAL\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_PERM => {
                    zName = b"SQLITE_PERM\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_ABORT => {
                    zName = b"SQLITE_ABORT\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_ABORT_ROLLBACK => {
                    zName = b"SQLITE_ABORT_ROLLBACK\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_BUSY => {
                    zName = b"SQLITE_BUSY\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_BUSY_RECOVERY => {
                    zName = b"SQLITE_BUSY_RECOVERY\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_BUSY_SNAPSHOT => {
                    zName = b"SQLITE_BUSY_SNAPSHOT\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_LOCKED => {
                    zName = b"SQLITE_LOCKED\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_LOCKED_SHAREDCACHE => {
                    zName = b"SQLITE_LOCKED_SHAREDCACHE\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_NOMEM => {
                    zName = b"SQLITE_NOMEM\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_READONLY => {
                    zName = b"SQLITE_READONLY\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_READONLY_RECOVERY => {
                    zName = b"SQLITE_READONLY_RECOVERY\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_READONLY_CANTINIT => {
                    zName = b"SQLITE_READONLY_CANTINIT\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_READONLY_ROLLBACK => {
                    zName = b"SQLITE_READONLY_ROLLBACK\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_READONLY_DBMOVED => {
                    zName = b"SQLITE_READONLY_DBMOVED\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_READONLY_DIRECTORY => {
                    zName = b"SQLITE_READONLY_DIRECTORY\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_INTERRUPT => {
                    zName = b"SQLITE_INTERRUPT\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR => {
                    zName = b"SQLITE_IOERR\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_READ => {
                    zName = b"SQLITE_IOERR_READ\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_SHORT_READ => {
                    zName = b"SQLITE_IOERR_SHORT_READ\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_WRITE => {
                    zName = b"SQLITE_IOERR_WRITE\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_FSYNC => {
                    zName = b"SQLITE_IOERR_FSYNC\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_DIR_FSYNC => {
                    zName = b"SQLITE_IOERR_DIR_FSYNC\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_TRUNCATE => {
                    zName = b"SQLITE_IOERR_TRUNCATE\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_FSTAT => {
                    zName = b"SQLITE_IOERR_FSTAT\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_UNLOCK => {
                    zName = b"SQLITE_IOERR_UNLOCK\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_RDLOCK => {
                    zName = b"SQLITE_IOERR_RDLOCK\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_DELETE => {
                    zName = b"SQLITE_IOERR_DELETE\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_NOMEM => {
                    zName = b"SQLITE_IOERR_NOMEM\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_ACCESS => {
                    zName = b"SQLITE_IOERR_ACCESS\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_CHECKRESERVEDLOCK => {
                    zName = b"SQLITE_IOERR_CHECKRESERVEDLOCK\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_LOCK => {
                    zName = b"SQLITE_IOERR_LOCK\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_CLOSE => {
                    zName = b"SQLITE_IOERR_CLOSE\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_DIR_CLOSE => {
                    zName = b"SQLITE_IOERR_DIR_CLOSE\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_SHMOPEN => {
                    zName = b"SQLITE_IOERR_SHMOPEN\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_SHMSIZE => {
                    zName = b"SQLITE_IOERR_SHMSIZE\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_SHMLOCK => {
                    zName = b"SQLITE_IOERR_SHMLOCK\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_SHMMAP => {
                    zName = b"SQLITE_IOERR_SHMMAP\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_SEEK => {
                    zName = b"SQLITE_IOERR_SEEK\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_DELETE_NOENT => {
                    zName = b"SQLITE_IOERR_DELETE_NOENT\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_MMAP => {
                    zName = b"SQLITE_IOERR_MMAP\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_GETTEMPPATH => {
                    zName = b"SQLITE_IOERR_GETTEMPPATH\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_IOERR_CONVPATH => {
                    zName = b"SQLITE_IOERR_CONVPATH\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CORRUPT => {
                    zName = b"SQLITE_CORRUPT\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_CORRUPT_VTAB => {
                    zName = b"SQLITE_CORRUPT_VTAB\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_NOTFOUND => {
                    zName = b"SQLITE_NOTFOUND\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_FULL => {
                    zName = b"SQLITE_FULL\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_CANTOPEN => {
                    zName = b"SQLITE_CANTOPEN\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_CANTOPEN_NOTEMPDIR => {
                    zName = b"SQLITE_CANTOPEN_NOTEMPDIR\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CANTOPEN_ISDIR => {
                    zName = b"SQLITE_CANTOPEN_ISDIR\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CANTOPEN_FULLPATH => {
                    zName = b"SQLITE_CANTOPEN_FULLPATH\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CANTOPEN_CONVPATH => {
                    zName = b"SQLITE_CANTOPEN_CONVPATH\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CANTOPEN_SYMLINK => {
                    zName = b"SQLITE_CANTOPEN_SYMLINK\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_PROTOCOL => {
                    zName = b"SQLITE_PROTOCOL\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_EMPTY => {
                    zName = b"SQLITE_EMPTY\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_SCHEMA => {
                    zName = b"SQLITE_SCHEMA\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_TOOBIG => {
                    zName = b"SQLITE_TOOBIG\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_CONSTRAINT => {
                    zName = b"SQLITE_CONSTRAINT\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CONSTRAINT_UNIQUE => {
                    zName = b"SQLITE_CONSTRAINT_UNIQUE\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CONSTRAINT_TRIGGER => {
                    zName = b"SQLITE_CONSTRAINT_TRIGGER\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CONSTRAINT_FOREIGNKEY => {
                    zName = b"SQLITE_CONSTRAINT_FOREIGNKEY\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CONSTRAINT_CHECK => {
                    zName = b"SQLITE_CONSTRAINT_CHECK\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CONSTRAINT_PRIMARYKEY => {
                    zName = b"SQLITE_CONSTRAINT_PRIMARYKEY\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CONSTRAINT_NOTNULL => {
                    zName = b"SQLITE_CONSTRAINT_NOTNULL\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CONSTRAINT_COMMITHOOK => {
                    zName = b"SQLITE_CONSTRAINT_COMMITHOOK\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CONSTRAINT_VTAB => {
                    zName = b"SQLITE_CONSTRAINT_VTAB\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CONSTRAINT_FUNCTION => {
                    zName = b"SQLITE_CONSTRAINT_FUNCTION\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_CONSTRAINT_ROWID => {
                    zName = b"SQLITE_CONSTRAINT_ROWID\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_MISMATCH => {
                    zName = b"SQLITE_MISMATCH\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_MISUSE => {
                    zName = b"SQLITE_MISUSE\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_NOLFS => {
                    zName = b"SQLITE_NOLFS\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_AUTH => {
                    zName = b"SQLITE_AUTH\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_FORMAT => {
                    zName = b"SQLITE_FORMAT\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_RANGE => {
                    zName = b"SQLITE_RANGE\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_NOTADB => {
                    zName = b"SQLITE_NOTADB\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_ROW => {
                    zName = b"SQLITE_ROW\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_NOTICE => {
                    zName = b"SQLITE_NOTICE\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_NOTICE_RECOVER_WAL => {
                    zName = b"SQLITE_NOTICE_RECOVER_WAL\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_NOTICE_RECOVER_ROLLBACK => {
                    zName = b"SQLITE_NOTICE_RECOVER_ROLLBACK\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_NOTICE_RBU => {
                    zName = b"SQLITE_NOTICE_RBU\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_WARNING => {
                    zName = b"SQLITE_WARNING\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_WARNING_AUTOINDEX => {
                    zName = b"SQLITE_WARNING_AUTOINDEX\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                SQLITE_DONE => {
                    zName = b"SQLITE_DONE\0".as_ptr() as *const ::core::ffi::c_char;
                }
                _ => {}
            }
            i += 1;
            rc &= 0xff as ::core::ffi::c_int;
        }
        if zName.is_null() {
            static mut zBuf: [::core::ffi::c_char; 50] = [0; 50];
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 50]>()
                    as ::core::ffi::c_int,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                b"SQLITE_UNKNOWN(%d)\0".as_ptr() as *const ::core::ffi::c_char,
                origRc,
            );
            zName = &raw mut zBuf as *mut ::core::ffi::c_char;
        }
        return zName;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ErrStr(
    mut rc: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        static mut aMsg: [*const ::core::ffi::c_char; 29] = [
            b"not an error\0".as_ptr() as *const ::core::ffi::c_char,
            b"SQL logic error\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"access permission denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"query aborted\0".as_ptr() as *const ::core::ffi::c_char,
            b"database is locked\0".as_ptr() as *const ::core::ffi::c_char,
            b"database table is locked\0".as_ptr() as *const ::core::ffi::c_char,
            b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            b"attempt to write a readonly database\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"interrupted\0".as_ptr() as *const ::core::ffi::c_char,
            b"disk I/O error\0".as_ptr() as *const ::core::ffi::c_char,
            b"database disk image is malformed\0".as_ptr() as *const ::core::ffi::c_char,
            b"unknown operation\0".as_ptr() as *const ::core::ffi::c_char,
            b"database or disk is full\0".as_ptr() as *const ::core::ffi::c_char,
            b"unable to open database file\0".as_ptr() as *const ::core::ffi::c_char,
            b"locking protocol\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"database schema has changed\0".as_ptr() as *const ::core::ffi::c_char,
            b"string or blob too big\0".as_ptr() as *const ::core::ffi::c_char,
            b"constraint failed\0".as_ptr() as *const ::core::ffi::c_char,
            b"datatype mismatch\0".as_ptr() as *const ::core::ffi::c_char,
            b"bad parameter or other API misuse\0".as_ptr()
                as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"authorization denied\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"column index out of range\0".as_ptr() as *const ::core::ffi::c_char,
            b"file is not a database\0".as_ptr() as *const ::core::ffi::c_char,
            b"notification message\0".as_ptr() as *const ::core::ffi::c_char,
            b"warning message\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut zErr: *const ::core::ffi::c_char = b"unknown error\0".as_ptr()
            as *const ::core::ffi::c_char;
        match rc {
            SQLITE_ABORT_ROLLBACK => {
                zErr = b"abort due to ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_ROW => {
                zErr = b"another row available\0".as_ptr() as *const ::core::ffi::c_char;
            }
            SQLITE_DONE => {
                zErr = b"no more rows available\0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
            _ => {
                rc &= 0xff as ::core::ffi::c_int;
                if rc >= 0 as ::core::ffi::c_int
                    && rc
                        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 29]>()
                            as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<*const ::core::ffi::c_char>()
                                    as usize,
                            ) as ::core::ffi::c_int && !aMsg[rc as usize].is_null()
                {
                    zErr = aMsg[rc as usize];
                }
            }
        }
        return zErr;
    }
}
unsafe extern "C" fn sqliteDefaultBusyCallback(
    mut ptr: *mut ::core::ffi::c_void,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        static mut delays: [u8_0; 12] = [
            1 as ::core::ffi::c_int as u8_0,
            2 as ::core::ffi::c_int as u8_0,
            5 as ::core::ffi::c_int as u8_0,
            10 as ::core::ffi::c_int as u8_0,
            15 as ::core::ffi::c_int as u8_0,
            20 as ::core::ffi::c_int as u8_0,
            25 as ::core::ffi::c_int as u8_0,
            25 as ::core::ffi::c_int as u8_0,
            25 as ::core::ffi::c_int as u8_0,
            50 as ::core::ffi::c_int as u8_0,
            50 as ::core::ffi::c_int as u8_0,
            100 as ::core::ffi::c_int as u8_0,
        ];
        static mut totals: [u8_0; 12] = [
            0 as ::core::ffi::c_int as u8_0,
            1 as ::core::ffi::c_int as u8_0,
            3 as ::core::ffi::c_int as u8_0,
            8 as ::core::ffi::c_int as u8_0,
            18 as ::core::ffi::c_int as u8_0,
            33 as ::core::ffi::c_int as u8_0,
            53 as ::core::ffi::c_int as u8_0,
            78 as ::core::ffi::c_int as u8_0,
            103 as ::core::ffi::c_int as u8_0,
            128 as ::core::ffi::c_int as u8_0,
            178 as ::core::ffi::c_int as u8_0,
            228 as ::core::ffi::c_int as u8_0,
        ];
        let mut db: *mut sqlite3 = ptr as *mut sqlite3;
        let mut tmout: ::core::ffi::c_int = (*db).busyTimeout;
        let mut delay: ::core::ffi::c_int = 0;
        let mut prior: ::core::ffi::c_int = 0;
        if count < NDELAY {
            delay = delays[count as usize] as ::core::ffi::c_int;
            prior = totals[count as usize] as ::core::ffi::c_int;
        } else {
            delay = delays[(NDELAY - 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int;
            prior = totals[(NDELAY - 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + delay * (count - (NDELAY - 1 as ::core::ffi::c_int));
        }
        if prior + delay > tmout {
            delay = tmout - prior;
            if delay <= 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
        }
        sqlite3OsSleep((*db).pVfs, delay * 1000 as ::core::ffi::c_int);
        return 1 as ::core::ffi::c_int;
    }
}
pub const NDELAY: ::core::ffi::c_int = (::core::mem::size_of::<[u8_0; 12]>() as usize)
    .wrapping_div(::core::mem::size_of::<u8_0>() as usize) as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3InvokeBusyHandler(
    mut p: *mut BusyHandler,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if (*p).xBusyHandler.is_none() || (*p).nBusy < 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        rc = (*p)
            .xBusyHandler
            .expect("non-null function pointer")((*p).pBusyArg, (*p).nBusy);
        if rc == 0 as ::core::ffi::c_int {
            (*p).nBusy = -1 as ::core::ffi::c_int;
        } else {
            (*p).nBusy += 1;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_busy_handler(
    mut db: *mut sqlite3,
    mut xBusy: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_mutex_enter((*db).mutex);
        (*db).busyHandler.xBusyHandler = xBusy;
        (*db).busyHandler.pBusyArg = pArg;
        (*db).busyHandler.nBusy = 0 as ::core::ffi::c_int;
        (*db).busyTimeout = 0 as ::core::ffi::c_int;
        sqlite3_mutex_leave((*db).mutex);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_progress_handler(
    mut db: *mut sqlite3,
    mut nOps: ::core::ffi::c_int,
    mut xProgress: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    mut pArg: *mut ::core::ffi::c_void,
) {
    unsafe {
        sqlite3_mutex_enter((*db).mutex);
        if nOps > 0 as ::core::ffi::c_int {
            (*db).xProgress = xProgress;
            (*db).nProgressOps = nOps as ::core::ffi::c_uint;
            (*db).pProgressArg = pArg;
        } else {
            (*db).xProgress = None;
            (*db).nProgressOps = 0 as ::core::ffi::c_uint;
            (*db).pProgressArg = ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        sqlite3_mutex_leave((*db).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_busy_timeout(
    mut db: *mut sqlite3,
    mut ms: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if ms > 0 as ::core::ffi::c_int {
            sqlite3_busy_handler(
                db,
                Some(
                    sqliteDefaultBusyCallback
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                db as *mut ::core::ffi::c_void,
            );
            (*db).busyTimeout = ms;
        } else {
            sqlite3_busy_handler(
                db,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_setlk_timeout(
    mut db: *mut sqlite3,
    mut ms: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if ms < -1 as ::core::ffi::c_int {
            return SQLITE_RANGE;
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_interrupt(mut db: *mut sqlite3) {
    unsafe {
        ;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_is_interrupted(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        return ((*db).u1.isInterrupted
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CreateFunc(
    mut db: *mut sqlite3,
    mut zFunctionName: *const ::core::ffi::c_char,
    mut nArg: ::core::ffi::c_int,
    mut enc: ::core::ffi::c_int,
    mut pUserData: *mut ::core::ffi::c_void,
    mut xSFunc: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xStep: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xFinal: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    mut xValue: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    mut xInverse: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut pDestructor: *mut FuncDestructor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut FuncDef = ::core::ptr::null_mut::<FuncDef>();
        let mut extraFlags: ::core::ffi::c_int = 0;
        if zFunctionName.is_null() || xSFunc.is_some() && xFinal.is_some()
            || xFinal.is_none() as ::core::ffi::c_int
                != xStep.is_none() as ::core::ffi::c_int
            || xValue.is_none() as ::core::ffi::c_int
                != xInverse.is_none() as ::core::ffi::c_int
            || (nArg < -1 as ::core::ffi::c_int || nArg > SQLITE_MAX_FUNCTION_ARG)
            || (255 as ::core::ffi::c_int) < sqlite3Strlen30(zFunctionName)
        {
            return sqlite3MisuseError(1948 as ::core::ffi::c_int);
        }
        extraFlags = enc
            & (SQLITE_DETERMINISTIC | SQLITE_DIRECTONLY | SQLITE_SUBTYPE
                | SQLITE_INNOCUOUS | SQLITE_RESULT_SUBTYPE | SQLITE_SELFORDER1);
        enc &= SQLITE_FUNC_ENCMASK | SQLITE_ANY;
        extraFlags ^= SQLITE_FUNC_UNSAFE;
        match enc {
            SQLITE_UTF16 => {
                enc = SQLITE_UTF16NATIVE;
            }
            SQLITE_ANY => {
                let mut rc: ::core::ffi::c_int = 0;
                rc = sqlite3CreateFunc(
                    db,
                    zFunctionName,
                    nArg,
                    (SQLITE_UTF8 | extraFlags) ^ SQLITE_FUNC_UNSAFE,
                    pUserData,
                    xSFunc,
                    xStep,
                    xFinal,
                    xValue,
                    xInverse,
                    pDestructor,
                );
                if rc == SQLITE_OK {
                    rc = sqlite3CreateFunc(
                        db,
                        zFunctionName,
                        nArg,
                        (SQLITE_UTF16LE | extraFlags) ^ SQLITE_FUNC_UNSAFE,
                        pUserData,
                        xSFunc,
                        xStep,
                        xFinal,
                        xValue,
                        xInverse,
                        pDestructor,
                    );
                }
                if rc != SQLITE_OK {
                    return rc;
                }
                enc = SQLITE_UTF16BE;
            }
            SQLITE_UTF8 | SQLITE_UTF16LE | SQLITE_UTF16BE => {}
            _ => {
                enc = SQLITE_UTF8;
            }
        }
        p = sqlite3FindFunction(db, zFunctionName, nArg, enc as u8_0, 0 as u8_0);
        if !p.is_null() && (*p).funcFlags & SQLITE_FUNC_ENCMASK as u32_0 == enc as u32_0
            && (*p).nArg as ::core::ffi::c_int == nArg
        {
            if (*db).nVdbeActive != 0 {
                sqlite3ErrorWithMsg(
                    db,
                    SQLITE_BUSY,
                    b"unable to delete/modify user-function due to active statements\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                return SQLITE_BUSY;
            } else {
                sqlite3ExpirePreparedStatements(db, 0 as ::core::ffi::c_int);
            }
        } else if xSFunc.is_none() && xFinal.is_none() {
            return SQLITE_OK
        }
        p = sqlite3FindFunction(db, zFunctionName, nArg, enc as u8_0, 1 as u8_0);
        if p.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        functionDestroy(db, p);
        if !pDestructor.is_null() {
            (*pDestructor).nRef += 1;
        }
        (*p).u.pDestructor = pDestructor;
        (*p).funcFlags = (*p).funcFlags & SQLITE_FUNC_ENCMASK as u32_0
            | extraFlags as u32_0;
        (*p).xSFunc = if xSFunc.is_some() { xSFunc } else { xStep };
        (*p).xFinalize = xFinal;
        (*p).xValue = xValue;
        (*p).xInverse = xInverse;
        (*p).pUserData = pUserData;
        (*p).nArg = nArg as u16_0 as i16_0;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn createFunctionApi(
    mut db: *mut sqlite3,
    mut zFunc: *const ::core::ffi::c_char,
    mut nArg: ::core::ffi::c_int,
    mut enc: ::core::ffi::c_int,
    mut p: *mut ::core::ffi::c_void,
    mut xSFunc: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xStep: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xFinal: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    mut xValue: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    mut xInverse: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = SQLITE_ERROR;
        let mut pArg: *mut FuncDestructor = ::core::ptr::null_mut::<FuncDestructor>();
        sqlite3_mutex_enter((*db).mutex);
        if xDestroy.is_some() {
            pArg = sqlite3Malloc(::core::mem::size_of::<FuncDestructor>() as u64_0)
                as *mut FuncDestructor;
            if pArg.is_null() {
                sqlite3OomFault(db);
                xDestroy.expect("non-null function pointer")(p);
                c2rust_current_block = 17603154382996925082;
            } else {
                (*pArg).nRef = 0 as ::core::ffi::c_int;
                (*pArg).xDestroy = xDestroy;
                (*pArg).pUserData = p;
                c2rust_current_block = 7815301370352969686;
            }
        } else {
            c2rust_current_block = 7815301370352969686;
        }
        match c2rust_current_block {
            7815301370352969686 => {
                rc = sqlite3CreateFunc(
                    db,
                    zFunc,
                    nArg,
                    enc,
                    p,
                    xSFunc,
                    xStep,
                    xFinal,
                    xValue,
                    xInverse,
                    pArg,
                );
                if !pArg.is_null() && (*pArg).nRef == 0 as ::core::ffi::c_int {
                    xDestroy.expect("non-null function pointer")(p);
                    sqlite3_free(pArg as *mut ::core::ffi::c_void);
                }
            }
            _ => {}
        }
        rc = sqlite3ApiExit(db, rc);
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_function(
    mut db: *mut sqlite3,
    mut zFunc: *const ::core::ffi::c_char,
    mut nArg: ::core::ffi::c_int,
    mut enc: ::core::ffi::c_int,
    mut p: *mut ::core::ffi::c_void,
    mut xSFunc: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xStep: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xFinal: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        return createFunctionApi(
            db,
            zFunc,
            nArg,
            enc,
            p,
            xSFunc,
            xStep,
            xFinal,
            None,
            None,
            None,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_function_v2(
    mut db: *mut sqlite3,
    mut zFunc: *const ::core::ffi::c_char,
    mut nArg: ::core::ffi::c_int,
    mut enc: ::core::ffi::c_int,
    mut p: *mut ::core::ffi::c_void,
    mut xSFunc: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xStep: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xFinal: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    mut xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        return createFunctionApi(
            db,
            zFunc,
            nArg,
            enc,
            p,
            xSFunc,
            xStep,
            xFinal,
            None,
            None,
            xDestroy,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_window_function(
    mut db: *mut sqlite3,
    mut zFunc: *const ::core::ffi::c_char,
    mut nArg: ::core::ffi::c_int,
    mut enc: ::core::ffi::c_int,
    mut p: *mut ::core::ffi::c_void,
    mut xStep: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xFinal: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    mut xValue: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    mut xInverse: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        return createFunctionApi(
            db,
            zFunc,
            nArg,
            enc,
            p,
            None,
            xStep,
            xFinal,
            xValue,
            xInverse,
            xDestroy,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_function16(
    mut db: *mut sqlite3,
    mut zFunctionName: *const ::core::ffi::c_void,
    mut nArg: ::core::ffi::c_int,
    mut eTextRep: ::core::ffi::c_int,
    mut p: *mut ::core::ffi::c_void,
    mut xSFunc: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xStep: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut xFinal: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut zFunc8: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        sqlite3_mutex_enter((*db).mutex);
        zFunc8 = sqlite3Utf16to8(
            db,
            zFunctionName,
            -1 as ::core::ffi::c_int,
            SQLITE_UTF16NATIVE as u8_0,
        );
        rc = sqlite3CreateFunc(
            db,
            zFunc8,
            nArg,
            eTextRep,
            p,
            xSFunc,
            xStep,
            xFinal,
            None,
            None,
            ::core::ptr::null_mut::<FuncDestructor>(),
        );
        sqlite3DbFree(db, zFunc8 as *mut ::core::ffi::c_void);
        rc = sqlite3ApiExit(db, rc);
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
unsafe extern "C" fn sqlite3InvalidFunction(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zName: *const ::core::ffi::c_char = sqlite3_user_data(context)
            as *const ::core::ffi::c_char;
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        zErr = sqlite3_mprintf(
            b"unable to use function %s in the requested context\0".as_ptr()
                as *const ::core::ffi::c_char,
            zName,
        );
        sqlite3_result_error(context, zErr, -1 as ::core::ffi::c_int);
        sqlite3_free(zErr as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_overload_function(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut nArg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut zCopy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        sqlite3_mutex_enter((*db).mutex);
        rc = !sqlite3FindFunction(db, zName, nArg, SQLITE_UTF8 as u8_0, 0 as u8_0)
            .is_null() as ::core::ffi::c_int;
        sqlite3_mutex_leave((*db).mutex);
        if rc != 0 {
            return SQLITE_OK;
        }
        zCopy = sqlite3_mprintf(b"%s\0".as_ptr() as *const ::core::ffi::c_char, zName);
        if zCopy.is_null() {
            return SQLITE_NOMEM;
        }
        return sqlite3_create_function_v2(
            db,
            zName,
            nArg,
            SQLITE_UTF8,
            zCopy as *mut ::core::ffi::c_void,
            Some(
                sqlite3InvalidFunction
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
            Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_trace(
    mut db: *mut sqlite3,
    mut xTrace: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> (),
    >,
    mut pArg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pOld: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        sqlite3_mutex_enter((*db).mutex);
        pOld = (*db).pTraceArg;
        (*db).mTrace = (if xTrace.is_some() {
            SQLITE_TRACE_LEGACY
        } else {
            0 as ::core::ffi::c_int
        }) as u8_0;
        (*db).trace.xLegacy = xTrace;
        (*db).pTraceArg = pArg;
        sqlite3_mutex_leave((*db).mutex);
        return pOld;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_trace_v2(
    mut db: *mut sqlite3,
    mut mTrace: ::core::ffi::c_uint,
    mut xTrace: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_uint,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_mutex_enter((*db).mutex);
        if mTrace == 0 as ::core::ffi::c_uint {
            xTrace = None;
        }
        if xTrace.is_none() {
            mTrace = 0 as ::core::ffi::c_uint;
        }
        (*db).mTrace = mTrace as u8_0;
        (*db).trace.xV2 = xTrace
            as Option<
                unsafe extern "C" fn(
                    u32_0,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        (*db).pTraceArg = pArg;
        sqlite3_mutex_leave((*db).mutex);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_profile(
    mut db: *mut sqlite3,
    mut xProfile: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            sqlite_uint64,
        ) -> (),
    >,
    mut pArg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pOld: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        sqlite3_mutex_enter((*db).mutex);
        pOld = (*db).pProfileArg;
        (*db).xProfile = xProfile
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    u64_0,
                ) -> (),
            >;
        (*db).pProfileArg = pArg;
        (*db).mTrace = ((*db).mTrace as ::core::ffi::c_int & SQLITE_TRACE_NONLEGACY_MASK)
            as u8_0;
        if (*db).xProfile.is_some() {
            (*db).mTrace = ((*db).mTrace as ::core::ffi::c_int | SQLITE_TRACE_XPROFILE)
                as u8_0;
        }
        sqlite3_mutex_leave((*db).mutex);
        return pOld;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_commit_hook(
    mut db: *mut sqlite3,
    mut xCallback: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    mut pArg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pOld: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        sqlite3_mutex_enter((*db).mutex);
        pOld = (*db).pCommitArg;
        (*db).xCommitCallback = xCallback;
        (*db).pCommitArg = pArg;
        sqlite3_mutex_leave((*db).mutex);
        return pOld;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_update_hook(
    mut db: *mut sqlite3,
    mut xCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            sqlite_int64,
        ) -> (),
    >,
    mut pArg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pRet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        sqlite3_mutex_enter((*db).mutex);
        pRet = (*db).pUpdateArg;
        (*db).xUpdateCallback = xCallback;
        (*db).pUpdateArg = pArg;
        sqlite3_mutex_leave((*db).mutex);
        return pRet;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_rollback_hook(
    mut db: *mut sqlite3,
    mut xCallback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    mut pArg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pRet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        sqlite3_mutex_enter((*db).mutex);
        pRet = (*db).pRollbackArg;
        (*db).xRollbackCallback = xCallback;
        (*db).pRollbackArg = pArg;
        sqlite3_mutex_leave((*db).mutex);
        return pRet;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_preupdate_hook(
    mut db: *mut sqlite3,
    mut xCallback: Option<
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
    mut pArg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pRet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        sqlite3_mutex_enter((*db).mutex);
        pRet = (*db).pPreUpdateArg;
        (*db).xPreUpdateCallback = xCallback;
        (*db).pPreUpdateArg = pArg;
        sqlite3_mutex_leave((*db).mutex);
        return pRet;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_autovacuum_pages(
    mut db: *mut sqlite3,
    mut xCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            u32_0,
            u32_0,
            u32_0,
        ) -> ::core::ffi::c_uint,
    >,
    mut pArg: *mut ::core::ffi::c_void,
    mut xDestructor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_mutex_enter((*db).mutex);
        if (*db).xAutovacDestr.is_some() {
            (*db)
                .xAutovacDestr
                .expect("non-null function pointer")((*db).pAutovacPagesArg);
        }
        (*db).xAutovacPages = xCallback;
        (*db).pAutovacPagesArg = pArg;
        (*db).xAutovacDestr = xDestructor;
        sqlite3_mutex_leave((*db).mutex);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WalDefaultHook(
    mut pClientData: *mut ::core::ffi::c_void,
    mut db: *mut sqlite3,
    mut zDb: *const ::core::ffi::c_char,
    mut nFrame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if nFrame >= pClientData as intptr_t as ::core::ffi::c_int {
            sqlite3BeginBenignMalloc();
            sqlite3_wal_checkpoint(db, zDb);
            sqlite3EndBenignMalloc();
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_wal_autocheckpoint(
    mut db: *mut sqlite3,
    mut nFrame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if nFrame > 0 as ::core::ffi::c_int {
            sqlite3_wal_hook(
                db,
                Some(
                    sqlite3WalDefaultHook
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut sqlite3,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                nFrame as intptr_t as *mut ::core::ffi::c_void,
            );
        } else {
            sqlite3_wal_hook(db, None, ::core::ptr::null_mut::<::core::ffi::c_void>());
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_wal_hook(
    mut db: *mut sqlite3,
    mut xCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    mut pArg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pRet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        sqlite3_mutex_enter((*db).mutex);
        pRet = (*db).pWalArg;
        (*db).xWalCallback = xCallback;
        (*db).pWalArg = pArg;
        sqlite3_mutex_leave((*db).mutex);
        return pRet;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_wal_checkpoint_v2(
    mut db: *mut sqlite3,
    mut zDb: *const ::core::ffi::c_char,
    mut eMode: ::core::ffi::c_int,
    mut pnLog: *mut ::core::ffi::c_int,
    mut pnCkpt: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut iDb: ::core::ffi::c_int = 0;
        if !pnLog.is_null() {
            *pnLog = -1 as ::core::ffi::c_int;
        }
        if !pnCkpt.is_null() {
            *pnCkpt = -1 as ::core::ffi::c_int;
        }
        if eMode < SQLITE_CHECKPOINT_PASSIVE || eMode > SQLITE_CHECKPOINT_TRUNCATE {
            return sqlite3MisuseError(2564 as ::core::ffi::c_int);
        }
        sqlite3_mutex_enter((*db).mutex);
        if !zDb.is_null()
            && *zDb.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        {
            iDb = sqlite3FindDbName(db, zDb);
        } else {
            iDb = SQLITE_MAX_DB;
        }
        if iDb < 0 as ::core::ffi::c_int {
            rc = SQLITE_ERROR;
            sqlite3ErrorWithMsg(
                db,
                SQLITE_ERROR,
                b"unknown database: %s\0".as_ptr() as *const ::core::ffi::c_char,
                zDb,
            );
        } else {
            (*db).busyHandler.nBusy = 0 as ::core::ffi::c_int;
            rc = sqlite3Checkpoint(db, iDb, eMode, pnLog, pnCkpt);
            sqlite3Error(db, rc);
        }
        rc = sqlite3ApiExit(db, rc);
        if (*db).nVdbeActive == 0 as ::core::ffi::c_int {
            ;
        }
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_wal_checkpoint(
    mut db: *mut sqlite3,
    mut zDb: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3_wal_checkpoint_v2(
            db,
            zDb,
            SQLITE_CHECKPOINT_PASSIVE,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Checkpoint(
    mut db: *mut sqlite3,
    mut iDb: ::core::ffi::c_int,
    mut eMode: ::core::ffi::c_int,
    mut pnLog: *mut ::core::ffi::c_int,
    mut pnCkpt: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut i: ::core::ffi::c_int = 0;
        let mut bBusy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*db).nDb && rc == SQLITE_OK {
            if i == iDb || iDb == SQLITE_MAX_DB {
                rc = sqlite3BtreeCheckpoint(
                    (*(*db).aDb.offset(i as isize)).pBt,
                    eMode,
                    pnLog,
                    pnCkpt,
                );
                pnLog = ::core::ptr::null_mut::<::core::ffi::c_int>();
                pnCkpt = ::core::ptr::null_mut::<::core::ffi::c_int>();
                if rc == SQLITE_BUSY {
                    bBusy = 1 as ::core::ffi::c_int;
                    rc = SQLITE_OK;
                }
            }
            i += 1;
        }
        return if rc == SQLITE_OK && bBusy != 0 { SQLITE_BUSY } else { rc };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TempInMemory(
    mut db: *const sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        return ((*db).temp_store as ::core::ffi::c_int == 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_errmsg(
    mut db: *mut sqlite3,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if db.is_null() {
            return sqlite3ErrStr(SQLITE_NOMEM_BKPT);
        }
        if sqlite3SafetyCheckSickOrOk(db) == 0 {
            return sqlite3ErrStr(sqlite3MisuseError(2700 as ::core::ffi::c_int));
        }
        sqlite3_mutex_enter((*db).mutex);
        if (*db).mallocFailed != 0 {
            z = sqlite3ErrStr(SQLITE_NOMEM_BKPT);
        } else {
            z = if (*db).errCode != 0 {
                sqlite3_value_text((*db).pErr) as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
            if z.is_null() {
                z = sqlite3ErrStr((*db).errCode);
            }
        }
        sqlite3_mutex_leave((*db).mutex);
        return z;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_set_errmsg(
    mut db: *mut sqlite3,
    mut errcode: ::core::ffi::c_int,
    mut zMsg: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if sqlite3SafetyCheckOk(db) == 0 {
            return sqlite3MisuseError(2727 as ::core::ffi::c_int);
        }
        sqlite3_mutex_enter((*db).mutex);
        if !zMsg.is_null() {
            sqlite3ErrorWithMsg(
                db,
                errcode,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                zMsg,
            );
        } else {
            sqlite3Error(db, errcode);
        }
        rc = sqlite3ApiExit(db, rc);
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_error_offset(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iOffset: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        if !db.is_null() && sqlite3SafetyCheckSickOrOk(db) != 0 && (*db).errCode != 0 {
            sqlite3_mutex_enter((*db).mutex);
            iOffset = (*db).errByteOffset;
            sqlite3_mutex_leave((*db).mutex);
        }
        return iOffset;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_errmsg16(
    mut db: *mut sqlite3,
) -> *const ::core::ffi::c_void {
    unsafe {
        static mut outOfMem: [u16_0; 14] = [
            'o' as i32 as u16_0,
            'u' as i32 as u16_0,
            't' as i32 as u16_0,
            ' ' as i32 as u16_0,
            'o' as i32 as u16_0,
            'f' as i32 as u16_0,
            ' ' as i32 as u16_0,
            'm' as i32 as u16_0,
            'e' as i32 as u16_0,
            'm' as i32 as u16_0,
            'o' as i32 as u16_0,
            'r' as i32 as u16_0,
            'y' as i32 as u16_0,
            0 as ::core::ffi::c_int as u16_0,
        ];
        static mut misuse: [u16_0; 34] = [
            'b' as i32 as u16_0,
            'a' as i32 as u16_0,
            'd' as i32 as u16_0,
            ' ' as i32 as u16_0,
            'p' as i32 as u16_0,
            'a' as i32 as u16_0,
            'r' as i32 as u16_0,
            'a' as i32 as u16_0,
            'm' as i32 as u16_0,
            'e' as i32 as u16_0,
            't' as i32 as u16_0,
            'e' as i32 as u16_0,
            'r' as i32 as u16_0,
            ' ' as i32 as u16_0,
            'o' as i32 as u16_0,
            'r' as i32 as u16_0,
            ' ' as i32 as u16_0,
            'o' as i32 as u16_0,
            't' as i32 as u16_0,
            'h' as i32 as u16_0,
            'e' as i32 as u16_0,
            'r' as i32 as u16_0,
            ' ' as i32 as u16_0,
            'A' as i32 as u16_0,
            'P' as i32 as u16_0,
            'I' as i32 as u16_0,
            ' ' as i32 as u16_0,
            'm' as i32 as u16_0,
            'i' as i32 as u16_0,
            's' as i32 as u16_0,
            'u' as i32 as u16_0,
            's' as i32 as u16_0,
            'e' as i32 as u16_0,
            0 as ::core::ffi::c_int as u16_0,
        ];
        let mut z: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        if db.is_null() {
            return &raw const outOfMem as *const u16_0 as *mut ::core::ffi::c_void;
        }
        if sqlite3SafetyCheckSickOrOk(db) == 0 {
            return &raw const misuse as *const u16_0 as *mut ::core::ffi::c_void;
        }
        sqlite3_mutex_enter((*db).mutex);
        if (*db).mallocFailed != 0 {
            z = &raw const outOfMem as *const u16_0 as *mut ::core::ffi::c_void;
        } else {
            z = sqlite3_value_text16((*db).pErr);
            if z.is_null() {
                sqlite3ErrorWithMsg(db, (*db).errCode, sqlite3ErrStr((*db).errCode));
                z = sqlite3_value_text16((*db).pErr);
            }
            sqlite3OomClear(db);
        }
        sqlite3_mutex_leave((*db).mutex);
        return z;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_errcode(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        if !db.is_null() && sqlite3SafetyCheckSickOrOk(db) == 0 {
            return sqlite3MisuseError(2802 as ::core::ffi::c_int);
        }
        if db.is_null() || (*db).mallocFailed as ::core::ffi::c_int != 0 {
            return SQLITE_NOMEM_BKPT;
        }
        return (*db).errCode & (*db).errMask;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_extended_errcode(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        if !db.is_null() && sqlite3SafetyCheckSickOrOk(db) == 0 {
            return sqlite3MisuseError(2811 as ::core::ffi::c_int);
        }
        if db.is_null() || (*db).mallocFailed as ::core::ffi::c_int != 0 {
            return SQLITE_NOMEM_BKPT;
        }
        return (*db).errCode;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_system_errno(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        return if !db.is_null() { (*db).iSysErrno } else { 0 as ::core::ffi::c_int };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_errstr(
    mut rc: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        return sqlite3ErrStr(rc);
    }
}
unsafe extern "C" fn createCollation(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut enc: u8_0,
    mut pCtx: *mut ::core::ffi::c_void,
    mut xCompare: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        let mut enc2: ::core::ffi::c_int = 0;
        enc2 = enc as ::core::ffi::c_int;
        if enc2 == SQLITE_UTF16 || enc2 == SQLITE_UTF16_ALIGNED {
            enc2 = SQLITE_UTF16NATIVE;
        }
        if enc2 < SQLITE_UTF8 || enc2 > SQLITE_UTF16BE {
            return sqlite3MisuseError(2859 as ::core::ffi::c_int);
        }
        pColl = sqlite3FindCollSeq(db, enc2 as u8_0, zName, 0 as ::core::ffi::c_int);
        if !pColl.is_null() && (*pColl).xCmp.is_some() {
            if (*db).nVdbeActive != 0 {
                sqlite3ErrorWithMsg(
                    db,
                    SQLITE_BUSY,
                    b"unable to delete/modify collation sequence due to active statements\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                return SQLITE_BUSY;
            }
            sqlite3ExpirePreparedStatements(db, 0 as ::core::ffi::c_int);
            if (*pColl).enc as ::core::ffi::c_int & !SQLITE_UTF16_ALIGNED == enc2 {
                let mut aColl: *mut CollSeq = sqlite3HashFind(
                    &raw mut (*db).aCollSeq,
                    zName,
                ) as *mut CollSeq;
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < 3 as ::core::ffi::c_int {
                    let mut p: *mut CollSeq = aColl.offset(j as isize) as *mut CollSeq;
                    if (*p).enc as ::core::ffi::c_int
                        == (*pColl).enc as ::core::ffi::c_int
                    {
                        if (*p).xDel.is_some() {
                            (*p).xDel.expect("non-null function pointer")((*p).pUser);
                        }
                        (*p).xCmp = None;
                    }
                    j += 1;
                }
            }
        }
        pColl = sqlite3FindCollSeq(db, enc2 as u8_0, zName, 1 as ::core::ffi::c_int);
        if pColl.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        (*pColl).xCmp = xCompare;
        (*pColl).pUser = pCtx;
        (*pColl).xDel = xDel;
        (*pColl).enc = (enc2 | enc as ::core::ffi::c_int & SQLITE_UTF16_ALIGNED) as u8_0;
        sqlite3Error(db, SQLITE_OK);
        return SQLITE_OK;
    }
}
static mut aHardLimit: [::core::ffi::c_int; 12] = [
    SQLITE_MAX_LENGTH,
    SQLITE_MAX_SQL_LENGTH,
    SQLITE_MAX_COLUMN,
    SQLITE_MAX_EXPR_DEPTH,
    SQLITE_MAX_COMPOUND_SELECT,
    SQLITE_MAX_VDBE_OP,
    SQLITE_MAX_FUNCTION_ARG,
    SQLITE_MAX_ATTACHED,
    SQLITE_MAX_LIKE_PATTERN_LENGTH,
    SQLITE_MAX_VARIABLE_NUMBER,
    SQLITE_MAX_TRIGGER_DEPTH,
    SQLITE_MAX_WORKER_THREADS,
];
#[no_mangle]
pub unsafe extern "C" fn sqlite3_limit(
    mut db: *mut sqlite3,
    mut limitId: ::core::ffi::c_int,
    mut newLimit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut oldLimit: ::core::ffi::c_int = 0;
        if limitId < 0 as ::core::ffi::c_int || limitId >= SQLITE_N_LIMIT {
            return -1 as ::core::ffi::c_int;
        }
        oldLimit = (*db).aLimit[limitId as usize];
        if newLimit >= 0 as ::core::ffi::c_int {
            if newLimit > aHardLimit[limitId as usize] {
                newLimit = aHardLimit[limitId as usize];
            } else if newLimit < SQLITE_MIN_LENGTH && limitId == SQLITE_LIMIT_LENGTH {
                newLimit = SQLITE_MIN_LENGTH;
            }
            (*db).aLimit[limitId as usize] = newLimit;
        }
        return oldLimit;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ParseUri(
    mut zDefaultVfs: *const ::core::ffi::c_char,
    mut zUri: *const ::core::ffi::c_char,
    mut pFlags: *mut ::core::ffi::c_uint,
    mut ppVfs: *mut *mut sqlite3_vfs,
    mut pzFile: *mut *mut ::core::ffi::c_char,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut flags: ::core::ffi::c_uint = *pFlags;
        let mut zVfs: *const ::core::ffi::c_char = zDefaultVfs;
        let mut zFile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut c: ::core::ffi::c_char = 0;
        let mut nUri: ::core::ffi::c_int = sqlite3Strlen30(zUri);
        if (flags & SQLITE_OPEN_URI as ::core::ffi::c_uint != 0
            || sqlite3Config.bOpenUri
                as ::core::ffi::c_int != 0) && nUri >= 5 as ::core::ffi::c_int
            && memcmp(
                zUri as *const ::core::ffi::c_void,
                b"file:\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                5 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut zOpt: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut eState: ::core::ffi::c_int = 0;
            let mut iIn: ::core::ffi::c_int = 0;
            let mut iOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut nByte: u64_0 = (nUri + 8 as ::core::ffi::c_int) as u64_0;
            flags |= SQLITE_OPEN_URI as ::core::ffi::c_uint;
            iIn = 0 as ::core::ffi::c_int;
            while iIn < nUri {
                nByte = (nByte as ::core::ffi::c_ulonglong)
                    .wrapping_add(
                        (*zUri.offset(iIn as isize) as ::core::ffi::c_int == '&' as i32)
                            as ::core::ffi::c_int as ::core::ffi::c_ulonglong,
                    ) as u64_0 as u64_0;
                iIn += 1;
            }
            zFile = sqlite3_malloc64(nByte as sqlite3_uint64)
                as *mut ::core::ffi::c_char;
            if zFile.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            memset(
                zFile as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                4 as size_t,
            );
            zFile = zFile.offset(4 as ::core::ffi::c_int as isize);
            iIn = 5 as ::core::ffi::c_int;
            if *zUri.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '/' as i32
                && *zUri.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '/' as i32
            {
                iIn = 7 as ::core::ffi::c_int;
                while *zUri.offset(iIn as isize) as ::core::ffi::c_int != 0
                    && *zUri.offset(iIn as isize) as ::core::ffi::c_int != '/' as i32
                {
                    iIn += 1;
                }
                if iIn != 7 as ::core::ffi::c_int
                    && (iIn != 16 as ::core::ffi::c_int
                        || memcmp(
                            b"localhost\0".as_ptr() as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            zUri.offset(7 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                            9 as size_t,
                        ) != 0)
                {
                    *pzErrMsg = sqlite3_mprintf(
                        b"invalid uri authority: %.*s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        iIn - 7 as ::core::ffi::c_int,
                        zUri.offset(7 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_char,
                    );
                    rc = SQLITE_ERROR;
                    c2rust_current_block = 18240643025223241126;
                } else {
                    c2rust_current_block = 13242334135786603907;
                }
            } else {
                c2rust_current_block = 13242334135786603907;
            }
            match c2rust_current_block {
                18240643025223241126 => {}
                _ => {
                    eState = 0 as ::core::ffi::c_int;
                    loop {
                        c = *zUri.offset(iIn as isize);
                        if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                            && c as ::core::ffi::c_int != '#' as i32)
                        {
                            break;
                        }
                        iIn += 1;
                        if c as ::core::ffi::c_int == '%' as i32
                            && *(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(
                                    *zUri.offset(iIn as isize) as ::core::ffi::c_uchar as isize,
                                ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                            && *(&raw const sqlite3CtypeMap
                                as *const ::core::ffi::c_uchar)
                                .offset(
                                    *zUri.offset((iIn + 1 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_uchar as isize,
                                ) as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                        {
                            let c2rust_fresh5 = iIn;
                            iIn = iIn + 1;
                            let mut octet: ::core::ffi::c_int = (sqlite3HexToInt(
                                *zUri.offset(c2rust_fresh5 as isize) as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
                            let c2rust_fresh6 = iIn;
                            iIn = iIn + 1;
                            octet
                                += sqlite3HexToInt(
                                    *zUri.offset(c2rust_fresh6 as isize) as ::core::ffi::c_int,
                                ) as ::core::ffi::c_int;
                            if octet == 0 as ::core::ffi::c_int {
                                loop {
                                    c = *zUri.offset(iIn as isize);
                                    if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                        && c as ::core::ffi::c_int != '#' as i32
                                        && (eState != 0 as ::core::ffi::c_int
                                            || c as ::core::ffi::c_int != '?' as i32)
                                        && (eState != 1 as ::core::ffi::c_int
                                            || c as ::core::ffi::c_int != '=' as i32
                                                && c as ::core::ffi::c_int != '&' as i32)
                                        && (eState != 2 as ::core::ffi::c_int
                                            || c as ::core::ffi::c_int != '&' as i32))
                                    {
                                        break;
                                    }
                                    iIn += 1;
                                }
                                continue;
                            } else {
                                c = octet as ::core::ffi::c_char;
                            }
                        } else if eState == 1 as ::core::ffi::c_int
                            && (c as ::core::ffi::c_int == '&' as i32
                                || c as ::core::ffi::c_int == '=' as i32)
                        {
                            if *zFile.offset((iOut - 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            {
                                while *zUri.offset(iIn as isize) as ::core::ffi::c_int != 0
                                    && *zUri.offset(iIn as isize) as ::core::ffi::c_int
                                        != '#' as i32
                                    && *zUri.offset((iIn - 1 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int != '&' as i32
                                {
                                    iIn += 1;
                                }
                                continue;
                            } else {
                                if c as ::core::ffi::c_int == '&' as i32 {
                                    let c2rust_fresh7 = iOut;
                                    iOut = iOut + 1;
                                    *zFile.offset(c2rust_fresh7 as isize) = '\0' as i32
                                        as ::core::ffi::c_char;
                                } else {
                                    eState = 2 as ::core::ffi::c_int;
                                }
                                c = 0 as ::core::ffi::c_char;
                            }
                        } else if eState == 0 as ::core::ffi::c_int
                            && c as ::core::ffi::c_int == '?' as i32
                            || eState == 2 as ::core::ffi::c_int
                                && c as ::core::ffi::c_int == '&' as i32
                        {
                            c = 0 as ::core::ffi::c_char;
                            eState = 1 as ::core::ffi::c_int;
                        }
                        let c2rust_fresh8 = iOut;
                        iOut = iOut + 1;
                        *zFile.offset(c2rust_fresh8 as isize) = c;
                    }
                    if eState == 1 as ::core::ffi::c_int {
                        let c2rust_fresh9 = iOut;
                        iOut = iOut + 1;
                        *zFile.offset(c2rust_fresh9 as isize) = '\0' as i32
                            as ::core::ffi::c_char;
                    }
                    memset(
                        zFile.offset(iOut as isize) as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        4 as size_t,
                    );
                    zOpt = zFile
                        .offset(
                            ((sqlite3Strlen30
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int)(zFile) + 1 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut ::core::ffi::c_char;
                    loop {
                        if !(*zOpt.offset(0 as ::core::ffi::c_int as isize) != 0) {
                            c2rust_current_block = 12027283704867122503;
                            break;
                        }
                        let mut nOpt: ::core::ffi::c_int = sqlite3Strlen30(zOpt);
                        let mut zVal: *mut ::core::ffi::c_char = zOpt
                            .offset((nOpt + 1 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_char;
                        let mut nVal: ::core::ffi::c_int = sqlite3Strlen30(zVal);
                        if nOpt == 3 as ::core::ffi::c_int
                            && memcmp(
                                b"vfs\0".as_ptr() as *const ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                zOpt as *const ::core::ffi::c_void,
                                3 as size_t,
                            ) == 0 as ::core::ffi::c_int
                        {
                            zVfs = zVal;
                        } else {
                            let mut aMode: *mut OpenMode = ::core::ptr::null_mut::<
                                OpenMode,
                            >();
                            let mut zModeType: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                ::core::ffi::c_char,
                            >();
                            let mut mask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            let mut limit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            if nOpt == 5 as ::core::ffi::c_int
                                && memcmp(
                                    b"cache\0".as_ptr() as *const ::core::ffi::c_char
                                        as *const ::core::ffi::c_void,
                                    zOpt as *const ::core::ffi::c_void,
                                    5 as size_t,
                                ) == 0 as ::core::ffi::c_int
                            {
                                static mut aCacheMode: [OpenMode; 3] = [
                                    OpenMode {
                                        z: b"shared\0".as_ptr() as *const ::core::ffi::c_char,
                                        mode: SQLITE_OPEN_SHAREDCACHE

                                    },
                                    OpenMode {
                                        z: b"private\0".as_ptr() as *const ::core::ffi::c_char,
                                        mode: SQLITE_OPEN_PRIVATECACHE

                                    },
                                    OpenMode {
                                        z: ::core::ptr::null::<::core::ffi::c_char>(),
                                        mode: 0 as ::core::ffi::c_int

                                    },
                                ];
                                mask = SQLITE_OPEN_SHAREDCACHE | SQLITE_OPEN_PRIVATECACHE;
                                aMode = &raw mut aCacheMode as *mut OpenMode
                                    as *mut OpenMode;
                                limit = mask;
                                zModeType = b"cache\0".as_ptr()
                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                            }
                            if nOpt == 4 as ::core::ffi::c_int
                                && memcmp(
                                    b"mode\0".as_ptr() as *const ::core::ffi::c_char
                                        as *const ::core::ffi::c_void,
                                    zOpt as *const ::core::ffi::c_void,
                                    4 as size_t,
                                ) == 0 as ::core::ffi::c_int
                            {
                                static mut aOpenMode: [OpenMode; 5] = [
                                    OpenMode {
                                        z: b"ro\0".as_ptr() as *const ::core::ffi::c_char,
                                        mode: SQLITE_OPEN_READONLY

                                    },
                                    OpenMode {
                                        z: b"rw\0".as_ptr() as *const ::core::ffi::c_char,
                                        mode: SQLITE_OPEN_READWRITE

                                    },
                                    OpenMode {
                                        z: b"rwc\0".as_ptr() as *const ::core::ffi::c_char,
                                        mode: SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE

                                    },
                                    OpenMode {
                                        z: b"memory\0".as_ptr() as *const ::core::ffi::c_char,
                                        mode: SQLITE_OPEN_MEMORY

                                    },
                                    OpenMode {
                                        z: ::core::ptr::null::<::core::ffi::c_char>(),
                                        mode: 0 as ::core::ffi::c_int

                                    },
                                ];
                                mask = SQLITE_OPEN_READONLY | SQLITE_OPEN_READWRITE
                                    | SQLITE_OPEN_CREATE | SQLITE_OPEN_MEMORY;
                                aMode = &raw mut aOpenMode as *mut OpenMode
                                    as *mut OpenMode;
                                limit = (mask as ::core::ffi::c_uint & flags)
                                    as ::core::ffi::c_int;
                                zModeType = b"access\0".as_ptr()
                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                            }
                            if !aMode.is_null() {
                                let mut i: ::core::ffi::c_int = 0;
                                let mut mode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                i = 0 as ::core::ffi::c_int;
                                while !(*aMode.offset(i as isize)).z.is_null() {
                                    let mut z: *const ::core::ffi::c_char = (*aMode
                                        .offset(i as isize))
                                        .z;
                                    if nVal == sqlite3Strlen30(z)
                                        && 0 as ::core::ffi::c_int
                                            == memcmp(
                                                zVal as *const ::core::ffi::c_void,
                                                z as *const ::core::ffi::c_void,
                                                nVal as size_t,
                                            )
                                    {
                                        mode = (*aMode.offset(i as isize)).mode;
                                        break;
                                    } else {
                                        i += 1;
                                    }
                                }
                                if mode == 0 as ::core::ffi::c_int {
                                    *pzErrMsg = sqlite3_mprintf(
                                        b"no such %s mode: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        zModeType,
                                        zVal,
                                    );
                                    rc = SQLITE_ERROR;
                                    c2rust_current_block = 18240643025223241126;
                                    break;
                                } else if mode & !SQLITE_OPEN_MEMORY > limit {
                                    *pzErrMsg = sqlite3_mprintf(
                                        b"%s mode not allowed: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        zModeType,
                                        zVal,
                                    );
                                    rc = SQLITE_PERM;
                                    c2rust_current_block = 18240643025223241126;
                                    break;
                                } else {
                                    flags = flags & !mask as ::core::ffi::c_uint
                                        | mode as ::core::ffi::c_uint;
                                }
                            }
                        }
                        zOpt = zVal.offset((nVal + 1 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_char;
                    }
                }
            }
        } else {
            zFile = sqlite3_malloc64((nUri + 8 as ::core::ffi::c_int) as sqlite3_uint64)
                as *mut ::core::ffi::c_char;
            if zFile.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            memset(
                zFile as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                4 as size_t,
            );
            zFile = zFile.offset(4 as ::core::ffi::c_int as isize);
            if nUri != 0 {
                memcpy(
                    zFile as *mut ::core::ffi::c_void,
                    zUri as *const ::core::ffi::c_void,
                    nUri as size_t,
                );
            }
            memset(
                zFile.offset(nUri as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                4 as size_t,
            );
            flags &= !SQLITE_OPEN_URI as ::core::ffi::c_uint;
            c2rust_current_block = 12027283704867122503;
        }
        match c2rust_current_block {
            12027283704867122503 => {
                *ppVfs = sqlite3_vfs_find(zVfs);
                if (*ppVfs).is_null() {
                    *pzErrMsg = sqlite3_mprintf(
                        b"no such vfs: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        zVfs,
                    );
                    rc = SQLITE_ERROR;
                }
            }
            _ => {}
        }
        if rc != SQLITE_OK {
            sqlite3_free_filename(zFile as *const ::core::ffi::c_char);
            zFile = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        *pFlags = flags;
        *pzFile = zFile;
        return rc;
    }
}
unsafe extern "C" fn uriParameter(
    mut zFilename: *const ::core::ffi::c_char,
    mut zParam: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        zFilename = zFilename
            .offset((sqlite3Strlen30(zFilename) + 1 as ::core::ffi::c_int) as isize);
        while !zFilename.is_null()
            && *zFilename.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0
        {
            let mut x: ::core::ffi::c_int = strcmp(zFilename, zParam);
            zFilename = zFilename
                .offset((sqlite3Strlen30(zFilename) + 1 as ::core::ffi::c_int) as isize);
            if x == 0 as ::core::ffi::c_int {
                return zFilename;
            }
            zFilename = zFilename
                .offset((sqlite3Strlen30(zFilename) + 1 as ::core::ffi::c_int) as isize);
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn openDatabase(
    mut zFilename: *const ::core::ffi::c_char,
    mut ppDb: *mut *mut sqlite3,
    mut flags: ::core::ffi::c_uint,
    mut zVfs: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut isThreadsafe: ::core::ffi::c_int = 0;
        let mut zOpen: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zErrMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        *ppDb = ::core::ptr::null_mut::<sqlite3>();
        rc = sqlite3_initialize();
        if rc != 0 {
            return rc;
        }
        if sqlite3Config.bCoreMutex as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            isThreadsafe = 0 as ::core::ffi::c_int;
        } else if flags & SQLITE_OPEN_NOMUTEX as ::core::ffi::c_uint != 0 {
            isThreadsafe = 0 as ::core::ffi::c_int;
        } else if flags & SQLITE_OPEN_FULLMUTEX as ::core::ffi::c_uint != 0 {
            isThreadsafe = 1 as ::core::ffi::c_int;
        } else {
            isThreadsafe = sqlite3Config.bFullMutex as ::core::ffi::c_int;
        }
        if flags & SQLITE_OPEN_PRIVATECACHE as ::core::ffi::c_uint != 0 {
            flags &= !SQLITE_OPEN_SHAREDCACHE as ::core::ffi::c_uint;
        } else if sqlite3Config.sharedCacheEnabled != 0 {
            flags |= SQLITE_OPEN_SHAREDCACHE as ::core::ffi::c_uint;
        }
        flags
            &= !(SQLITE_OPEN_DELETEONCLOSE | SQLITE_OPEN_EXCLUSIVE | SQLITE_OPEN_MAIN_DB
                | SQLITE_OPEN_TEMP_DB | SQLITE_OPEN_TRANSIENT_DB
                | SQLITE_OPEN_MAIN_JOURNAL | SQLITE_OPEN_TEMP_JOURNAL
                | SQLITE_OPEN_SUBJOURNAL | SQLITE_OPEN_SUPER_JOURNAL
                | SQLITE_OPEN_NOMUTEX | SQLITE_OPEN_FULLMUTEX | SQLITE_OPEN_WAL)
                as ::core::ffi::c_uint;
        db = sqlite3MallocZero(::core::mem::size_of::<sqlite3>() as u64_0)
            as *mut sqlite3;
        if !db.is_null() {
            if isThreadsafe != 0 {
                (*db).mutex = sqlite3MutexAlloc(SQLITE_MUTEX_RECURSIVE);
                if (*db).mutex.is_null() {
                    sqlite3_free(db as *mut ::core::ffi::c_void);
                    db = ::core::ptr::null_mut::<sqlite3>();
                    c2rust_current_block = 6589548103820849419;
                } else {
                    isThreadsafe == 0 as ::core::ffi::c_int;
                    c2rust_current_block = 14648156034262866959;
                }
            } else {
                c2rust_current_block = 14648156034262866959;
            }
            match c2rust_current_block {
                6589548103820849419 => {}
                _ => {
                    sqlite3_mutex_enter((*db).mutex);
                    (*db).errMask = (if flags
                        & SQLITE_OPEN_EXRESCODE as ::core::ffi::c_uint
                        != 0 as ::core::ffi::c_uint
                    {
                        0xffffffff as ::core::ffi::c_uint
                    } else {
                        0xff as ::core::ffi::c_uint
                    }) as ::core::ffi::c_int;
                    (*db).nDb = 2 as ::core::ffi::c_int;
                    (*db).eOpenState = SQLITE_STATE_BUSY as u8_0;
                    (*db).aDb = &raw mut (*db).aDbStatic as *mut Db;
                    (*db).lookaside.bDisable = 1 as u32_0;
                    (*db).lookaside.sz = 0 as u16_0;
                    memcpy(
                        &raw mut (*db).aLimit as *mut ::core::ffi::c_int
                            as *mut ::core::ffi::c_void,
                        &raw const aHardLimit as *const ::core::ffi::c_int
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<[::core::ffi::c_int; 12]>() as size_t,
                    );
                    (*db).aLimit[SQLITE_LIMIT_WORKER_THREADS as usize] = SQLITE_DEFAULT_WORKER_THREADS;
                    (*db).autoCommit = 1 as u8_0;
                    (*db).nextAutovac = -1 as ::core::ffi::c_int as ::core::ffi::c_schar;
                    (*db).szMmap = sqlite3Config.szMmap as i64_0;
                    (*db).nextPagesize = 0 as ::core::ffi::c_int;
                    (*db).init.azInit = &raw mut sqlite3StdType
                        as *mut *const ::core::ffi::c_char;
                    (*db).flags
                        |= ((SQLITE_ShortColNames | SQLITE_EnableTrigger)
                            as ::core::ffi::c_uint | SQLITE_EnableView
                            | SQLITE_CacheSpill as ::core::ffi::c_uint)
                            as ::core::ffi::c_ulonglong | SQLITE_AttachCreate
                            | SQLITE_AttachWrite | SQLITE_Comments
                            | SQLITE_TrustedSchema as ::core::ffi::c_ulonglong
                            | SQLITE_DqsDML as ::core::ffi::c_ulonglong
                            | SQLITE_DqsDDL as ::core::ffi::c_ulonglong
                            | SQLITE_AutoIndex as ::core::ffi::c_ulonglong;
                    sqlite3HashInit(&raw mut (*db).aCollSeq);
                    sqlite3HashInit(&raw mut (*db).aModule);
                    createCollation(
                        db,
                        &raw const sqlite3StrBINARY as *const ::core::ffi::c_char,
                        SQLITE_UTF8 as u8_0,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        Some(
                            binCollFunc
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *const ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *const ::core::ffi::c_void,
                                ) -> ::core::ffi::c_int,
                        ),
                        None,
                    );
                    createCollation(
                        db,
                        &raw const sqlite3StrBINARY as *const ::core::ffi::c_char,
                        SQLITE_UTF16BE as u8_0,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        Some(
                            binCollFunc
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *const ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *const ::core::ffi::c_void,
                                ) -> ::core::ffi::c_int,
                        ),
                        None,
                    );
                    createCollation(
                        db,
                        &raw const sqlite3StrBINARY as *const ::core::ffi::c_char,
                        SQLITE_UTF16LE as u8_0,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        Some(
                            binCollFunc
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *const ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *const ::core::ffi::c_void,
                                ) -> ::core::ffi::c_int,
                        ),
                        None,
                    );
                    createCollation(
                        db,
                        b"NOCASE\0".as_ptr() as *const ::core::ffi::c_char,
                        SQLITE_UTF8 as u8_0,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        Some(
                            nocaseCollatingFunc
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *const ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *const ::core::ffi::c_void,
                                ) -> ::core::ffi::c_int,
                        ),
                        None,
                    );
                    createCollation(
                        db,
                        b"RTRIM\0".as_ptr() as *const ::core::ffi::c_char,
                        SQLITE_UTF8 as u8_0,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        Some(
                            rtrimCollFunc
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *const ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *const ::core::ffi::c_void,
                                ) -> ::core::ffi::c_int,
                        ),
                        None,
                    );
                    if !((*db).mallocFailed != 0) {
                        (*db).openFlags = flags;
                        if (1 as ::core::ffi::c_int)
                            << (flags & 7 as ::core::ffi::c_uint)
                            & 0x46 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            rc = sqlite3MisuseError(3534 as ::core::ffi::c_int);
                        } else {
                            if zFilename.is_null() {
                                zFilename = b":memory:\0".as_ptr()
                                    as *const ::core::ffi::c_char;
                            }
                            rc = sqlite3ParseUri(
                                zVfs,
                                zFilename,
                                &raw mut flags,
                                &raw mut (*db).pVfs,
                                &raw mut zOpen,
                                &raw mut zErrMsg,
                            );
                        }
                        if rc != SQLITE_OK {
                            if rc == SQLITE_NOMEM {
                                sqlite3OomFault(db);
                            }
                            sqlite3ErrorWithMsg(
                                db,
                                rc,
                                if !zErrMsg.is_null() {
                                    b"%s\0".as_ptr() as *const ::core::ffi::c_char
                                } else {
                                    ::core::ptr::null::<::core::ffi::c_char>()
                                },
                                zErrMsg,
                            );
                            sqlite3_free(zErrMsg as *mut ::core::ffi::c_void);
                        } else {
                            rc = sqlite3BtreeOpen(
                                (*db).pVfs,
                                zOpen,
                                db,
                                &raw mut (*(*db)
                                    .aDb
                                    .offset(0 as ::core::ffi::c_int as isize))
                                    .pBt,
                                0 as ::core::ffi::c_int,
                                (flags | SQLITE_OPEN_MAIN_DB as ::core::ffi::c_uint)
                                    as ::core::ffi::c_int,
                            );
                            if rc != SQLITE_OK {
                                if rc == SQLITE_IOERR_NOMEM {
                                    rc = SQLITE_NOMEM_BKPT;
                                }
                                sqlite3Error(db, rc);
                            } else {
                                sqlite3BtreeEnter(
                                    (*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).pBt,
                                );
                                let ref mut c2rust_fresh1 = (*(*db)
                                    .aDb
                                    .offset(0 as ::core::ffi::c_int as isize))
                                    .pSchema;
                                *c2rust_fresh1 = sqlite3SchemaGet(
                                    db,
                                    (*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).pBt,
                                );
                                if (*db).mallocFailed == 0 {
                                    sqlite3SetTextEncoding(
                                        db,
                                        (*(*(*db).aDb.offset(0 as ::core::ffi::c_int as isize))
                                            .pSchema)
                                            .enc,
                                    );
                                }
                                sqlite3BtreeLeave(
                                    (*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).pBt,
                                );
                                let ref mut c2rust_fresh2 = (*(*db)
                                    .aDb
                                    .offset(1 as ::core::ffi::c_int as isize))
                                    .pSchema;
                                *c2rust_fresh2 = sqlite3SchemaGet(
                                    db,
                                    ::core::ptr::null_mut::<Btree>(),
                                );
                                let ref mut c2rust_fresh3 = (*(*db)
                                    .aDb
                                    .offset(0 as ::core::ffi::c_int as isize))
                                    .zDbSName;
                                *c2rust_fresh3 = b"main\0".as_ptr()
                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                (*(*db).aDb.offset(0 as ::core::ffi::c_int as isize))
                                    .safety_level = (SQLITE_DEFAULT_SYNCHRONOUS
                                    + 1 as ::core::ffi::c_int) as u8_0;
                                let ref mut c2rust_fresh4 = (*(*db)
                                    .aDb
                                    .offset(1 as ::core::ffi::c_int as isize))
                                    .zDbSName;
                                *c2rust_fresh4 = b"temp\0".as_ptr()
                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize))
                                    .safety_level = PAGER_SYNCHRONOUS_OFF as u8_0;
                                (*db).eOpenState = SQLITE_STATE_OPEN as u8_0;
                                if !((*db).mallocFailed != 0) {
                                    sqlite3Error(db, SQLITE_OK);
                                    sqlite3RegisterPerConnectionBuiltinFunctions(db);
                                    rc = sqlite3_errcode(db);
                                    if rc == SQLITE_OK {
                                        sqlite3AutoLoadExtensions(db);
                                        rc = sqlite3_errcode(db);
                                        if rc != SQLITE_OK {
                                            c2rust_current_block = 6589548103820849419;
                                        } else {
                                            c2rust_current_block = 15514718523126015390;
                                        }
                                    } else {
                                        c2rust_current_block = 15514718523126015390;
                                    }
                                    match c2rust_current_block {
                                        6589548103820849419 => {}
                                        _ => {
                                            if rc != 0 {
                                                sqlite3Error(db, rc);
                                            }
                                            setupLookaside(
                                                db,
                                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                                sqlite3Config.szLookaside,
                                                sqlite3Config.nLookaside,
                                            );
                                            sqlite3_wal_autocheckpoint(
                                                db,
                                                SQLITE_DEFAULT_WAL_AUTOCHECKPOINT,
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
        if !db.is_null() {
            sqlite3_mutex_leave((*db).mutex);
        }
        rc = sqlite3_errcode(db);
        if rc & 0xff as ::core::ffi::c_int == SQLITE_NOMEM {
            sqlite3_close(db);
            db = ::core::ptr::null_mut::<sqlite3>();
        } else if rc != SQLITE_OK {
            (*db).eOpenState = SQLITE_STATE_SICK as u8_0;
        }
        *ppDb = db;
        sqlite3_free_filename(zOpen as *const ::core::ffi::c_char);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_open(
    mut zFilename: *const ::core::ffi::c_char,
    mut ppDb: *mut *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        return openDatabase(
            zFilename,
            ppDb,
            (SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE) as ::core::ffi::c_uint,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_open_v2(
    mut filename: *const ::core::ffi::c_char,
    mut ppDb: *mut *mut sqlite3,
    mut flags: ::core::ffi::c_int,
    mut zVfs: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return openDatabase(filename, ppDb, flags as ::core::ffi::c_uint, zVfs);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_open16(
    mut zFilename: *const ::core::ffi::c_void,
    mut ppDb: *mut *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zFilename8: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut rc: ::core::ffi::c_int = 0;
        *ppDb = ::core::ptr::null_mut::<sqlite3>();
        rc = sqlite3_initialize();
        if rc != 0 {
            return rc;
        }
        if zFilename.is_null() {
            zFilename = b"\0\0\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void;
        }
        pVal = sqlite3ValueNew(::core::ptr::null_mut::<sqlite3>());
        sqlite3ValueSetStr(
            pVal,
            -1 as ::core::ffi::c_int,
            zFilename,
            SQLITE_UTF16NATIVE as u8_0,
            SQLITE_STATIC,
        );
        zFilename8 = sqlite3ValueText(pVal, SQLITE_UTF8 as u8_0)
            as *const ::core::ffi::c_char;
        if !zFilename8.is_null() {
            rc = openDatabase(
                zFilename8,
                ppDb,
                (SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE) as ::core::ffi::c_uint,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            if rc == SQLITE_OK
                && !((*(*(**ppDb).aDb.offset(0 as ::core::ffi::c_int as isize)).pSchema)
                    .schemaFlags as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
                    == 0x1 as ::core::ffi::c_int)
            {
                (**ppDb).enc = SQLITE_UTF16NATIVE as u8_0;
                (*(*(**ppDb).aDb.offset(0 as ::core::ffi::c_int as isize)).pSchema)
                    .enc = (**ppDb).enc;
            }
        } else {
            rc = SQLITE_NOMEM_BKPT;
        }
        sqlite3ValueFree(pVal);
        return rc & 0xff as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_collation(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut enc: ::core::ffi::c_int,
    mut pCtx: *mut ::core::ffi::c_void,
    mut xCompare: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3_create_collation_v2(db, zName, enc, pCtx, xCompare, None);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_collation_v2(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut enc: ::core::ffi::c_int,
    mut pCtx: *mut ::core::ffi::c_void,
    mut xCompare: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        sqlite3_mutex_enter((*db).mutex);
        rc = createCollation(db, zName, enc as u8_0, pCtx, xCompare, xDel);
        rc = sqlite3ApiExit(db, rc);
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_collation16(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_void,
    mut enc: ::core::ffi::c_int,
    mut pCtx: *mut ::core::ffi::c_void,
    mut xCompare: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zName8: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        sqlite3_mutex_enter((*db).mutex);
        zName8 = sqlite3Utf16to8(
            db,
            zName,
            -1 as ::core::ffi::c_int,
            SQLITE_UTF16NATIVE as u8_0,
        );
        if !zName8.is_null() {
            rc = createCollation(db, zName8, enc as u8_0, pCtx, xCompare, None);
            sqlite3DbFree(db, zName8 as *mut ::core::ffi::c_void);
        }
        rc = sqlite3ApiExit(db, rc);
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_collation_needed(
    mut db: *mut sqlite3,
    mut pCollNeededArg: *mut ::core::ffi::c_void,
    mut xCollNeeded: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_mutex_enter((*db).mutex);
        (*db).xCollNeeded = xCollNeeded;
        (*db).xCollNeeded16 = None;
        (*db).pCollNeededArg = pCollNeededArg;
        sqlite3_mutex_leave((*db).mutex);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_collation_needed16(
    mut db: *mut sqlite3,
    mut pCollNeededArg: *mut ::core::ffi::c_void,
    mut xCollNeeded16: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> (),
    >,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_mutex_enter((*db).mutex);
        (*db).xCollNeeded = None;
        (*db).xCollNeeded16 = xCollNeeded16;
        (*db).pCollNeededArg = pCollNeededArg;
        sqlite3_mutex_leave((*db).mutex);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_get_clientdata(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut p: *mut DbClientData = ::core::ptr::null_mut::<DbClientData>();
        sqlite3_mutex_enter((*db).mutex);
        p = (*db).pDbData;
        while !p.is_null() {
            if strcmp(&raw mut (*p).zName as *mut ::core::ffi::c_char, zName)
                == 0 as ::core::ffi::c_int
            {
                let mut pResult: *mut ::core::ffi::c_void = (*p).pData;
                sqlite3_mutex_leave((*db).mutex);
                return pResult;
            }
            p = (*p).pNext;
        }
        sqlite3_mutex_leave((*db).mutex);
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_set_clientdata(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut pData: *mut ::core::ffi::c_void,
    mut xDestructor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut DbClientData = ::core::ptr::null_mut::<DbClientData>();
        let mut pp: *mut *mut DbClientData = ::core::ptr::null_mut::<
            *mut DbClientData,
        >();
        sqlite3_mutex_enter((*db).mutex);
        pp = &raw mut (*db).pDbData;
        p = (*db).pDbData;
        while !p.is_null()
            && strcmp(&raw mut (*p).zName as *mut ::core::ffi::c_char, zName) != 0
        {
            pp = &raw mut (*p).pNext;
            p = (*p).pNext;
        }
        if !p.is_null() {
            if (*p).xDestructor.is_some() {
                (*p).xDestructor.expect("non-null function pointer")((*p).pData);
            }
            if pData.is_null() {
                *pp = (*p).pNext;
                sqlite3_free(p as *mut ::core::ffi::c_void);
                sqlite3_mutex_leave((*db).mutex);
                return SQLITE_OK;
            }
        } else if pData.is_null() {
            sqlite3_mutex_leave((*db).mutex);
            return SQLITE_OK;
        } else {
            let mut n: size_t = strlen(zName);
            p = sqlite3_malloc64(
                (24 as size_t).wrapping_add(n.wrapping_add(1 as size_t))
                    as sqlite3_uint64,
            ) as *mut DbClientData;
            if p.is_null() {
                if xDestructor.is_some() {
                    xDestructor.expect("non-null function pointer")(pData);
                }
                sqlite3_mutex_leave((*db).mutex);
                return SQLITE_NOMEM;
            }
            memcpy(
                &raw mut (*p).zName as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                zName as *const ::core::ffi::c_void,
                n.wrapping_add(1 as size_t),
            );
            (*p).pNext = (*db).pDbData;
            (*db).pDbData = p;
        }
        (*p).pData = pData;
        (*p).xDestructor = xDestructor;
        sqlite3_mutex_leave((*db).mutex);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_global_recover() -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_get_autocommit(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        return (*db).autoCommit as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ReportError(
    mut iErr: ::core::ffi::c_int,
    mut lineno: ::core::ffi::c_int,
    mut zType: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_log(
            iErr,
            b"%s at line %d of [%.10s]\0".as_ptr() as *const ::core::ffi::c_char,
            zType,
            lineno,
            sqlite3_sourceid().offset(20 as ::core::ffi::c_int as isize),
        );
        return iErr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CorruptError(
    mut lineno: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3ReportError(
            SQLITE_CORRUPT,
            lineno,
            b"database corruption\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MisuseError(
    mut lineno: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3ReportError(
            SQLITE_MISUSE,
            lineno,
            b"misuse\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CantopenError(
    mut lineno: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3ReportError(
            SQLITE_CANTOPEN,
            lineno,
            b"cannot open file\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_thread_cleanup() {}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_table_column_metadata(
    mut db: *mut sqlite3,
    mut zDbName: *const ::core::ffi::c_char,
    mut zTableName: *const ::core::ffi::c_char,
    mut zColumnName: *const ::core::ffi::c_char,
    mut pzDataType: *mut *const ::core::ffi::c_char,
    mut pzCollSeq: *mut *const ::core::ffi::c_char,
    mut pNotNull: *mut ::core::ffi::c_int,
    mut pPrimaryKey: *mut ::core::ffi::c_int,
    mut pAutoinc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = 0;
        let mut zErrMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        let mut iCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zDataType: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zCollSeq: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut notnull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut primarykey: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut autoinc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        sqlite3_mutex_enter((*db).mutex);
        sqlite3BtreeEnterAll(db);
        rc = sqlite3Init(db, &raw mut zErrMsg);
        if !(SQLITE_OK != rc) {
            pTab = sqlite3FindTable(db, zTableName, zDbName);
            if pTab.is_null() || (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
                pTab = ::core::ptr::null_mut::<Table>();
            } else {
                if zColumnName.is_null() {
                    c2rust_current_block = 2838571290723028321;
                } else {
                    iCol = sqlite3ColumnIndex(pTab, zColumnName);
                    if iCol >= 0 as ::core::ffi::c_int {
                        pCol = (*pTab).aCol.offset(iCol as isize) as *mut Column;
                        c2rust_current_block = 2838571290723028321;
                    } else if (*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0
                        && sqlite3IsRowid(zColumnName) != 0
                    {
                        iCol = (*pTab).iPKey as ::core::ffi::c_int;
                        pCol = if iCol >= 0 as ::core::ffi::c_int {
                            (*pTab).aCol.offset(iCol as isize) as *mut Column
                        } else {
                            ::core::ptr::null_mut::<Column>()
                        };
                        c2rust_current_block = 2838571290723028321;
                    } else {
                        pTab = ::core::ptr::null_mut::<Table>();
                        c2rust_current_block = 6231146550366258145;
                    }
                }
                match c2rust_current_block {
                    6231146550366258145 => {}
                    _ => {
                        if !pCol.is_null() {
                            zDataType = sqlite3ColumnType(
                                pCol,
                                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            );
                            zCollSeq = sqlite3ColumnColl(pCol);
                            notnull = ((*pCol).notNull() as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                            primarykey = ((*pCol).colFlags as ::core::ffi::c_int
                                & COLFLAG_PRIMKEY != 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int;
                            autoinc = ((*pTab).iPKey as ::core::ffi::c_int == iCol
                                && (*pTab).tabFlags & TF_Autoincrement as u32_0
                                    != 0 as u32_0) as ::core::ffi::c_int;
                        } else {
                            zDataType = b"INTEGER\0".as_ptr()
                                as *const ::core::ffi::c_char;
                            primarykey = 1 as ::core::ffi::c_int;
                        }
                        if zCollSeq.is_null() {
                            zCollSeq = &raw const sqlite3StrBINARY
                                as *const ::core::ffi::c_char;
                        }
                    }
                }
            }
        }
        sqlite3BtreeLeaveAll(db);
        if !pzDataType.is_null() {
            *pzDataType = zDataType;
        }
        if !pzCollSeq.is_null() {
            *pzCollSeq = zCollSeq;
        }
        if !pNotNull.is_null() {
            *pNotNull = notnull;
        }
        if !pPrimaryKey.is_null() {
            *pPrimaryKey = primarykey;
        }
        if !pAutoinc.is_null() {
            *pAutoinc = autoinc;
        }
        if SQLITE_OK == rc && pTab.is_null() {
            sqlite3DbFree(db, zErrMsg as *mut ::core::ffi::c_void);
            zErrMsg = sqlite3MPrintf(
                db,
                b"no such table column: %s.%s\0".as_ptr() as *const ::core::ffi::c_char,
                zTableName,
                zColumnName,
            );
            rc = SQLITE_ERROR;
        }
        sqlite3ErrorWithMsg(
            db,
            rc,
            if !zErrMsg.is_null() {
                b"%s\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            },
            zErrMsg,
        );
        sqlite3DbFree(db, zErrMsg as *mut ::core::ffi::c_void);
        rc = sqlite3ApiExit(db, rc);
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_sleep(
    mut ms: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut rc: ::core::ffi::c_int = 0;
        pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        if pVfs.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        rc = sqlite3OsSleep(
            pVfs,
            (if ms < 0 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int
            } else {
                1000 as ::core::ffi::c_int * ms
            }),
        ) / 1000 as ::core::ffi::c_int;
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_extended_result_codes(
    mut db: *mut sqlite3,
    mut onoff: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_mutex_enter((*db).mutex);
        (*db).errMask = (if onoff != 0 {
            0xffffffff as ::core::ffi::c_uint
        } else {
            0xff as ::core::ffi::c_uint
        }) as ::core::ffi::c_int;
        sqlite3_mutex_leave((*db).mutex);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_file_control(
    mut db: *mut sqlite3,
    mut zDbName: *const ::core::ffi::c_char,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_ERROR;
        let mut pBtree: *mut Btree = ::core::ptr::null_mut::<Btree>();
        sqlite3_mutex_enter((*db).mutex);
        pBtree = sqlite3DbNameToBtree(db, zDbName);
        if !pBtree.is_null() {
            let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
            let mut fd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
            sqlite3BtreeEnter(pBtree);
            pPager = sqlite3BtreePager(pBtree) as *mut Pager;
            fd = sqlite3PagerFile(pPager);
            if op == SQLITE_FCNTL_FILE_POINTER {
                let ref mut c2rust_fresh14 = *(pArg as *mut *mut sqlite3_file);
                *c2rust_fresh14 = fd;
                rc = SQLITE_OK;
            } else if op == SQLITE_FCNTL_VFS_POINTER {
                let ref mut c2rust_fresh15 = *(pArg as *mut *mut sqlite3_vfs);
                *c2rust_fresh15 = sqlite3PagerVfs(pPager);
                rc = SQLITE_OK;
            } else if op == SQLITE_FCNTL_JOURNAL_POINTER {
                let ref mut c2rust_fresh16 = *(pArg as *mut *mut sqlite3_file);
                *c2rust_fresh16 = sqlite3PagerJrnlFile(pPager);
                rc = SQLITE_OK;
            } else if op == SQLITE_FCNTL_DATA_VERSION {
                *(pArg as *mut ::core::ffi::c_uint) = sqlite3PagerDataVersion(pPager)
                    as ::core::ffi::c_uint;
                rc = SQLITE_OK;
            } else if op == SQLITE_FCNTL_RESERVE_BYTES {
                let mut iNew: ::core::ffi::c_int = *(pArg as *mut ::core::ffi::c_int);
                *(pArg as *mut ::core::ffi::c_int) = sqlite3BtreeGetRequestedReserve(
                    pBtree,
                );
                if iNew >= 0 as ::core::ffi::c_int && iNew <= 255 as ::core::ffi::c_int {
                    sqlite3BtreeSetPageSize(
                        pBtree,
                        0 as ::core::ffi::c_int,
                        iNew,
                        0 as ::core::ffi::c_int,
                    );
                }
                rc = SQLITE_OK;
            } else if op == SQLITE_FCNTL_RESET_CACHE {
                sqlite3BtreeClearCache(pBtree);
                rc = SQLITE_OK;
            } else {
                let mut nSave: ::core::ffi::c_int = (*db).busyHandler.nBusy;
                rc = sqlite3OsFileControl(fd, op, pArg);
                (*db).busyHandler.nBusy = nSave;
            }
            sqlite3BtreeLeave(pBtree);
        }
        sqlite3_mutex_leave((*db).mutex);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_test_control(
    mut op: ::core::ffi::c_int,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        
        let mut ap = c2rust_args;
        match op {
            SQLITE_TESTCTRL_PRNG_SAVE => {
                sqlite3PrngSaveState();
            }
            SQLITE_TESTCTRL_PRNG_RESTORE => {
                sqlite3PrngRestoreState();
            }
            SQLITE_TESTCTRL_PRNG_SEED => {
                let mut x: ::core::ffi::c_int = ap.arg::<::core::ffi::c_int>();
                let mut y: ::core::ffi::c_int = 0;
                let mut db: *mut sqlite3 = ap.arg::<*mut sqlite3>();
                if !db.is_null()
                    && {
                        y = (*(*(*db).aDb.offset(0 as ::core::ffi::c_int as isize))
                            .pSchema)
                            .schema_cookie;
                        y != 0 as ::core::ffi::c_int
                    }
                {
                    x = y;
                }
                sqlite3Config.iPrngSeed = x as ::core::ffi::c_uint;
                sqlite3_randomness(
                    0 as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
            }
            SQLITE_TESTCTRL_FK_NO_ACTION => {
                let mut db_0: *mut sqlite3 = ap.arg::<*mut sqlite3>();
                let mut b: ::core::ffi::c_int = ap.arg::<::core::ffi::c_int>();
                if b != 0 {
                    (*db_0).flags |= SQLITE_FkNoAction as ::core::ffi::c_ulonglong;
                } else {
                    (*db_0).flags &= !SQLITE_FkNoAction as ::core::ffi::c_ulonglong;
                }
            }
            SQLITE_TESTCTRL_BITVEC_TEST => {
                let mut sz: ::core::ffi::c_int = ap.arg::<::core::ffi::c_int>();
                let mut aProg: *mut ::core::ffi::c_int = ap
                    .arg::<*mut ::core::ffi::c_int>();
                rc = sqlite3BitvecBuiltinTest(sz, aProg);
            }
            SQLITE_TESTCTRL_FAULT_INSTALL => {
                sqlite3Config.xTestCallback = ::core::mem::transmute::<_, Option<
                        unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
                    >>(
                    ap
                        .arg::<
                            *mut unsafe extern "C" fn(
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                        >(),
                );
                rc = sqlite3FaultSim(0 as ::core::ffi::c_int);
            }
            SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS => {
                let mut xBenignBegin: void_function = None;
                let mut xBenignEnd: void_function = None;
                xBenignBegin = ::core::mem::transmute(
                    ap.arg::<*mut unsafe extern "C" fn() -> ()>(),
                );
                xBenignEnd = ::core::mem::transmute(
                    ap.arg::<*mut unsafe extern "C" fn() -> ()>(),
                );
                sqlite3BenignMallocHooks(
                    xBenignBegin as Option<unsafe extern "C" fn() -> ()>,
                    xBenignEnd as Option<unsafe extern "C" fn() -> ()>,
                );
            }
            SQLITE_TESTCTRL_PENDING_BYTE => {
                rc = sqlite3PendingByte;
                let mut newVal: ::core::ffi::c_uint = ap.arg::<::core::ffi::c_uint>();
                if newVal != 0 {
                    sqlite3PendingByte = newVal as ::core::ffi::c_int;
                }
            }
            SQLITE_TESTCTRL_ASSERT => {
                let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                rc = x_0;
            }
            SQLITE_TESTCTRL_ALWAYS => {
                let mut x_1: ::core::ffi::c_int = ap.arg::<::core::ffi::c_int>();
                rc = if x_1 != 0 { x_1 } else { 0 as ::core::ffi::c_int };
            }
            SQLITE_TESTCTRL_BYTEORDER => {
                rc = SQLITE_BYTEORDER * 100 as ::core::ffi::c_int
                    + SQLITE_LITTLEENDIAN * 10 as ::core::ffi::c_int + SQLITE_BIGENDIAN;
            }
            SQLITE_TESTCTRL_OPTIMIZATIONS => {
                let mut db_1: *mut sqlite3 = ap.arg::<*mut sqlite3>();
                (*db_1).dbOptFlags = ap.arg::<u32_0>();
            }
            SQLITE_TESTCTRL_GETOPT => {
                let mut db_2: *mut sqlite3 = ap.arg::<*mut sqlite3>();
                let mut pN: *mut ::core::ffi::c_int = ap
                    .arg::<*mut ::core::ffi::c_int>();
                *pN = (*db_2).dbOptFlags as ::core::ffi::c_int;
            }
            SQLITE_TESTCTRL_LOCALTIME_FAULT => {
                sqlite3Config.bLocaltimeFault = ap.arg::<::core::ffi::c_int>();
                if sqlite3Config.bLocaltimeFault == 2 as ::core::ffi::c_int {
                    sqlite3Config.xAltLocaltime = ::core::mem::transmute::<_, Option<
                            unsafe extern "C" fn(
                                *const ::core::ffi::c_void,
                                *mut ::core::ffi::c_void,
                            ) -> ::core::ffi::c_int,
                        >>(
                        ap
                            .arg::<
                                *mut unsafe extern "C" fn(
                                    *const ::core::ffi::c_void,
                                    *mut ::core::ffi::c_void,
                                ) -> ::core::ffi::c_int,
                            >(),
                    );
                } else {
                    sqlite3Config.xAltLocaltime = None;
                }
            }
            SQLITE_TESTCTRL_INTERNAL_FUNCTIONS => {
                let mut db_3: *mut sqlite3 = ap.arg::<*mut sqlite3>();
                (*db_3).mDbFlags = ((*db_3).mDbFlags as ::core::ffi::c_uint
                    ^ DBFLAG_InternalFunc as ::core::ffi::c_uint) as u32_0;
            }
            SQLITE_TESTCTRL_NEVER_CORRUPT => {
                sqlite3Config.neverCorrupt = ap.arg::<::core::ffi::c_int>();
            }
            SQLITE_TESTCTRL_EXTRA_SCHEMA_CHECKS => {
                sqlite3Config.bExtraSchemaChecks = ap.arg::<::core::ffi::c_int>()
                    as u8_0;
            }
            SQLITE_TESTCTRL_ONCE_RESET_THRESHOLD => {
                sqlite3Config.iOnceResetThreshold = ap.arg::<::core::ffi::c_int>();
            }
            SQLITE_TESTCTRL_SORTER_MMAP => {
                let mut db_4: *mut sqlite3 = ap.arg::<*mut sqlite3>();
                (*db_4).nMaxSorterMmap = ap.arg::<::core::ffi::c_int>();
            }
            SQLITE_TESTCTRL_ISINIT => {
                if sqlite3Config.isInit == 0 as ::core::ffi::c_int {
                    rc = SQLITE_ERROR;
                }
            }
            SQLITE_TESTCTRL_IMPOSTER => {
                let mut db_5: *mut sqlite3 = ap.arg::<*mut sqlite3>();
                let mut iDb: ::core::ffi::c_int = 0;
                sqlite3_mutex_enter((*db_5).mutex);
                iDb = sqlite3FindDbName(db_5, ap.arg::<*const ::core::ffi::c_char>());
                if iDb >= 0 as ::core::ffi::c_int {
                    (*db_5).init.iDb = iDb as u8_0;
                    (*db_5)
                        .init
                        .set_imposterTable(
                            ap.arg::<::core::ffi::c_int>() as ::core::ffi::c_uint
                                as ::core::ffi::c_uint,
                        );
                    (*db_5).init.busy = (*db_5).init.imposterTable() as u8_0;
                    (*db_5).init.newTnum = ap.arg::<::core::ffi::c_int>() as Pgno;
                    if (*db_5).init.busy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && (*db_5).init.newTnum > 0 as Pgno
                    {
                        sqlite3ResetAllSchemasOfConnection(db_5);
                    }
                }
                sqlite3_mutex_leave((*db_5).mutex);
            }
            SQLITE_TESTCTRL_RESULT_INTREAL => {
                let mut pCtx: *mut sqlite3_context = ap.arg::<*mut ::core::ffi::c_void>()
                    as *mut sqlite3_context;
                sqlite3ResultIntReal(pCtx);
            }
            SQLITE_TESTCTRL_SEEK_COUNT => {
                let mut db_6: *mut sqlite3 = ap.arg::<*mut sqlite3>();
                let mut pn: *mut u64_0 = ap.arg::<*mut sqlite3_uint64>();
                *pn = 0 as u64_0;
            }
            SQLITE_TESTCTRL_TRACEFLAGS => {
                let mut opTrace: ::core::ffi::c_int = ap.arg::<::core::ffi::c_int>();
                let mut ptr: *mut u32_0 = ap.arg::<*mut u32_0>();
                match opTrace {
                    0 => {
                        *ptr = sqlite3TreeTrace;
                    }
                    1 => {
                        sqlite3TreeTrace = *ptr;
                    }
                    2 => {
                        *ptr = sqlite3WhereTrace;
                    }
                    3 => {
                        sqlite3WhereTrace = *ptr;
                    }
                    _ => {}
                }
            }
            SQLITE_TESTCTRL_LOGEST => {
                let mut rIn: ::core::ffi::c_double = ap.arg::<::core::ffi::c_double>();
                let mut rLogEst: LogEst = sqlite3LogEstFromDouble(rIn);
                let mut pI1: *mut ::core::ffi::c_int = ap
                    .arg::<*mut ::core::ffi::c_int>();
                let mut pU64: *mut u64_0 = ap.arg::<*mut u64_0>();
                let mut pI2: *mut ::core::ffi::c_int = ap
                    .arg::<*mut ::core::ffi::c_int>();
                *pI1 = rLogEst as ::core::ffi::c_int;
                *pU64 = sqlite3LogEstToInt(rLogEst);
                *pI2 = sqlite3LogEst(*pU64) as ::core::ffi::c_int;
            }
            SQLITE_TESTCTRL_VDBE_COVERAGE | SQLITE_TESTCTRL_JSON_SELFCHECK | _ => {}
        }
        return rc;
    }
}
unsafe extern "C" fn databaseName(
    mut zName: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        while *zName.offset(-1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
            || *zName.offset(-2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            || *zName.offset(-3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            || *zName.offset(-4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            zName = zName.offset(-1);
        }
        return zName;
    }
}
unsafe extern "C" fn appendText(
    mut p: *mut ::core::ffi::c_char,
    mut z: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut n: size_t = strlen(z);
        memcpy(
            p as *mut ::core::ffi::c_void,
            z as *const ::core::ffi::c_void,
            n.wrapping_add(1 as size_t),
        );
        return p.offset(n as isize).offset(1 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_create_filename(
    mut zDatabase: *const ::core::ffi::c_char,
    mut zJournal: *const ::core::ffi::c_char,
    mut zWal: *const ::core::ffi::c_char,
    mut nParam: ::core::ffi::c_int,
    mut azParam: *mut *const ::core::ffi::c_char,
) -> sqlite3_filename {
    unsafe {
        let mut nByte: sqlite3_int64 = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut pResult: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        nByte = strlen(zDatabase)
            .wrapping_add(strlen(zJournal))
            .wrapping_add(strlen(zWal))
            .wrapping_add(10 as size_t) as sqlite3_int64;
        i = 0 as ::core::ffi::c_int;
        while i < nParam * 2 as ::core::ffi::c_int {
            nByte = (nByte as ::core::ffi::c_ulonglong)
                .wrapping_add(
                    strlen(*azParam.offset(i as isize)).wrapping_add(1 as size_t)
                        as ::core::ffi::c_ulonglong,
                ) as sqlite3_int64 as sqlite3_int64;
            i += 1;
        }
        p = sqlite3_malloc64(nByte as sqlite3_uint64) as *mut ::core::ffi::c_char;
        pResult = p;
        if p.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        memset(p as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, 4 as size_t);
        p = p.offset(4 as ::core::ffi::c_int as isize);
        p = appendText(p, zDatabase);
        i = 0 as ::core::ffi::c_int;
        while i < nParam * 2 as ::core::ffi::c_int {
            p = appendText(p, *azParam.offset(i as isize));
            i += 1;
        }
        let c2rust_fresh11 = p;
        p = p.offset(1);
        *c2rust_fresh11 = 0 as ::core::ffi::c_char;
        p = appendText(p, zJournal);
        p = appendText(p, zWal);
        let c2rust_fresh12 = p;
        p = p.offset(1);
        *c2rust_fresh12 = 0 as ::core::ffi::c_char;
        let c2rust_fresh13 = p;
        p = p.offset(1);
        *c2rust_fresh13 = 0 as ::core::ffi::c_char;
        return pResult.offset(4 as ::core::ffi::c_int as isize) as sqlite3_filename;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_free_filename(mut p: *const ::core::ffi::c_char) {
    unsafe {
        if p.is_null() {
            return;
        }
        p = databaseName(p);
        sqlite3_free(
            (p as *mut ::core::ffi::c_char).offset(-(4 as ::core::ffi::c_int as isize))
                as *mut ::core::ffi::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_uri_parameter(
    mut zFilename: *const ::core::ffi::c_char,
    mut zParam: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        if zFilename.is_null() || zParam.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        zFilename = databaseName(zFilename);
        return uriParameter(zFilename, zParam);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_uri_key(
    mut zFilename: *const ::core::ffi::c_char,
    mut N: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        if zFilename.is_null() || N < 0 as ::core::ffi::c_int {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        zFilename = databaseName(zFilename);
        zFilename = zFilename
            .offset((sqlite3Strlen30(zFilename) + 1 as ::core::ffi::c_int) as isize);
        while !zFilename.is_null()
            && *zFilename.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0
            && {
                let c2rust_fresh10 = N;
                N = N - 1;
                c2rust_fresh10 > 0 as ::core::ffi::c_int
            }
        {
            zFilename = zFilename
                .offset((sqlite3Strlen30(zFilename) + 1 as ::core::ffi::c_int) as isize);
            zFilename = zFilename
                .offset((sqlite3Strlen30(zFilename) + 1 as ::core::ffi::c_int) as isize);
        }
        return if *zFilename.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int != 0
        {
            zFilename
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_uri_boolean(
    mut zFilename: *const ::core::ffi::c_char,
    mut zParam: *const ::core::ffi::c_char,
    mut bDflt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut z: *const ::core::ffi::c_char = sqlite3_uri_parameter(zFilename, zParam);
        bDflt = (bDflt != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        return if !z.is_null() {
            sqlite3GetBoolean(z, bDflt as u8_0) as ::core::ffi::c_int
        } else {
            bDflt
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_uri_int64(
    mut zFilename: *const ::core::ffi::c_char,
    mut zParam: *const ::core::ffi::c_char,
    mut bDflt: sqlite3_int64,
) -> sqlite3_int64 {
    unsafe {
        let mut z: *const ::core::ffi::c_char = sqlite3_uri_parameter(zFilename, zParam);
        let mut v: sqlite3_int64 = 0;
        if !z.is_null() && sqlite3DecOrHexToI64(z, &raw mut v) == 0 as ::core::ffi::c_int
        {
            bDflt = v;
        }
        return bDflt;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_filename_database(
    mut zFilename: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        if zFilename.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        return databaseName(zFilename);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_filename_journal(
    mut zFilename: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        if zFilename.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        zFilename = databaseName(zFilename);
        zFilename = zFilename
            .offset((sqlite3Strlen30(zFilename) + 1 as ::core::ffi::c_int) as isize);
        while !zFilename.is_null()
            && *zFilename.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0
        {
            zFilename = zFilename
                .offset((sqlite3Strlen30(zFilename) + 1 as ::core::ffi::c_int) as isize);
            zFilename = zFilename
                .offset((sqlite3Strlen30(zFilename) + 1 as ::core::ffi::c_int) as isize);
        }
        return zFilename.offset(1 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_filename_wal(
    mut zFilename: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        zFilename = sqlite3_filename_journal(zFilename);
        if !zFilename.is_null() {
            zFilename = zFilename
                .offset((sqlite3Strlen30(zFilename) + 1 as ::core::ffi::c_int) as isize);
        }
        return zFilename;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DbNameToBtree(
    mut db: *mut sqlite3,
    mut zDbName: *const ::core::ffi::c_char,
) -> *mut Btree {
    unsafe {
        let mut iDb: ::core::ffi::c_int = if !zDbName.is_null() {
            sqlite3FindDbName(db, zDbName)
        } else {
            0 as ::core::ffi::c_int
        };
        return if iDb < 0 as ::core::ffi::c_int {
            ::core::ptr::null_mut::<Btree>()
        } else {
            (*(*db).aDb.offset(iDb as isize)).pBt
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_db_name(
    mut db: *mut sqlite3,
    mut N: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        if N < 0 as ::core::ffi::c_int || N >= (*db).nDb {
            return ::core::ptr::null::<::core::ffi::c_char>()
        } else {
            return (*(*db).aDb.offset(N as isize)).zDbSName
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_db_filename(
    mut db: *mut sqlite3,
    mut zDbName: *const ::core::ffi::c_char,
) -> sqlite3_filename {
    unsafe {
        let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
        pBt = sqlite3DbNameToBtree(db, zDbName);
        return if !pBt.is_null() {
            sqlite3BtreeGetFilename(pBt) as sqlite3_filename
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_db_readonly(
    mut db: *mut sqlite3,
    mut zDbName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
        pBt = sqlite3DbNameToBtree(db, zDbName);
        return if !pBt.is_null() {
            sqlite3BtreeIsReadonly(pBt)
        } else {
            -1 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_compileoption_used(
    mut zOptName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut nOpt: ::core::ffi::c_int = 0;
        let mut azCompileOpt: *mut *const ::core::ffi::c_char = ::core::ptr::null_mut::<
            *const ::core::ffi::c_char,
        >();
        azCompileOpt = sqlite3CompileOptions(&raw mut nOpt);
        if sqlite3_strnicmp(
            zOptName,
            b"SQLITE_\0".as_ptr() as *const ::core::ffi::c_char,
            7 as ::core::ffi::c_int,
        ) == 0 as ::core::ffi::c_int
        {
            zOptName = zOptName.offset(7 as ::core::ffi::c_int as isize);
        }
        n = sqlite3Strlen30(zOptName);
        i = 0 as ::core::ffi::c_int;
        while i < nOpt {
            if sqlite3_strnicmp(zOptName, *azCompileOpt.offset(i as isize), n)
                == 0 as ::core::ffi::c_int
                && sqlite3IsIdChar(
                    *(*azCompileOpt.offset(i as isize)).offset(n as isize) as u8_0,
                ) == 0 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_compileoption_get(
    mut N: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut nOpt: ::core::ffi::c_int = 0;
        let mut azCompileOpt: *mut *const ::core::ffi::c_char = ::core::ptr::null_mut::<
            *const ::core::ffi::c_char,
        >();
        azCompileOpt = sqlite3CompileOptions(&raw mut nOpt);
        if N >= 0 as ::core::ffi::c_int && N < nOpt {
            return *azCompileOpt.offset(N as isize);
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_THREADSAFE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
