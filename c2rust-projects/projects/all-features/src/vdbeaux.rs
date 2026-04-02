use c2rust_bitfields::BitfieldStruct;
use ::libc;
extern "C" {
    pub type VdbeSorter;
    pub type BtCursor;
    pub type Btree;
    pub type RenameToken;
    pub type TableLock;
    pub type VtabCtx;
    pub type sqlite3_mutex;
    pub type Pager;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
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
    fn sqlite3OsWrite(
        _: *mut sqlite3_file,
        _: *const ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsSync(_: *mut sqlite3_file, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3OsDeviceCharacteristics(id: *mut sqlite3_file) -> ::core::ffi::c_int;
    fn sqlite3OsDelete(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsAccess(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        pResOut: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsOpenMalloc(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: *mut *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsCloseFree(_: *mut sqlite3_file);
    fn sqlite3PagerGetJournalMode(_: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerExclusiveLock(_: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerIsMemdb(_: *mut Pager) -> ::core::ffi::c_int;
    fn disable_simulated_io_errors();
    fn enable_simulated_io_errors();
    fn sqlite3BtreeCommitPhaseOne(
        _: *mut Btree,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeCommitPhaseTwo(
        _: *mut Btree,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeTxnState(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeSavepoint(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeGetFilename(_: *mut Btree) -> *const ::core::ffi::c_char;
    fn sqlite3BtreeGetJournalname(_: *mut Btree) -> *const ::core::ffi::c_char;
    fn sqlite3BtreeCloseCursor(_: *mut BtCursor) -> ::core::ffi::c_int;
    fn sqlite3BtreeTableMoveto(
        _: *mut BtCursor,
        intKey: i64_0,
        bias: ::core::ffi::c_int,
        pRes: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeCursorHasMoved(_: *mut BtCursor) -> ::core::ffi::c_int;
    fn sqlite3BtreeCursorRestore(
        _: *mut BtCursor,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreePayloadSize(_: *mut BtCursor) -> u32_0;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeEnter(_: *mut Btree);
    fn sqlite3BtreeSharable(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeLeave(_: *mut Btree);
    fn sqlite3CorruptError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    fn sqlite3DbReallocOrFree(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: u64_0,
    ) -> *mut ::core::ffi::c_void;
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
    fn sqlite3IsNaN(_: ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3VMPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3ProgressCheck(_: *mut Parse);
    fn sqlite3CommitInternalChanges(_: *mut sqlite3);
    fn sqlite3PrimaryKeyIndex(_: *mut Table) -> *mut Index;
    fn sqlite3DeleteTable(_: *mut sqlite3, _: *mut Table);
    fn sqlite3RollbackAll(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3CloseSavepoints(_: *mut sqlite3);
    fn sqlite3MayAbort(_: *mut Parse);
    fn sqlite3GetVarint32(_: *const ::core::ffi::c_uchar, _: *mut u32_0) -> u8_0;
    fn sqlite3VarintLen(v: u64_0) -> ::core::ffi::c_int;
    fn sqlite3SystemError(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3ErrStr(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3ValueText(_: *mut sqlite3_value, _: u8_0) -> *const ::core::ffi::c_void;
    fn sqlite3ValueSetStr(
        _: *mut sqlite3_value,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: u8_0,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3ValueSetNull(_: *mut sqlite3_value);
    fn sqlite3ValueFree(_: *mut sqlite3_value);
    fn sqlite3ValueNew(_: *mut sqlite3) -> *mut sqlite3_value;
    fn sqlite3ValueApplyAffinity(_: *mut sqlite3_value, _: u8_0, _: u8_0);
    static sqlite3OpcodeProperty: [::core::ffi::c_uchar; 0];
    fn sqlite3KeyInfoUnref(_: *mut KeyInfo);
    fn sqlite3KeyInfoOfIndex(_: *mut Parse, _: *mut Index) -> *mut KeyInfo;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3RCStrUnref(_: *mut ::core::ffi::c_void);
    fn sqlite3StrAccumInit(
        _: *mut StrAccum,
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3StrAccumFinish(_: *mut StrAccum) -> *mut ::core::ffi::c_char;
    fn sqlite3VtabSync(db: *mut sqlite3, _: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VtabCommit(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3VtabLock(_: *mut VTable);
    fn sqlite3VtabUnlock(_: *mut VTable);
    fn sqlite3VtabSavepoint(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
    fn sqlite3VdbeMemCopy(_: *mut Mem, _: *const Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemShallowCopy(_: *mut Mem, _: *const Mem, _: ::core::ffi::c_int);
    fn sqlite3VdbeMemSetStr(
        _: *mut Mem,
        _: *const ::core::ffi::c_char,
        _: i64_0,
        _: u8_0,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemSetInt64(_: *mut Mem, _: i64_0);
    fn sqlite3VdbeMemInit(_: *mut Mem, _: *mut sqlite3, _: u16_0);
    fn sqlite3VdbeMemSetNull(_: *mut Mem);
    fn sqlite3VdbeMemFromBtreeZeroOffset(
        _: *mut BtCursor,
        _: u32_0,
        _: *mut Mem,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemRelease(p: *mut Mem);
    fn sqlite3VdbeMemReleaseMalloc(p: *mut Mem);
    fn sqlite3OpcodeName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3VdbeMemGrow(
        pMem: *mut Mem,
        n: ::core::ffi::c_int,
        preserve: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeSorterClose(_: *mut sqlite3, _: *mut VdbeCursor);
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
    pub trace: C2Rust_Unnamed_25,
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
    pub u1: C2Rust_Unnamed_22,
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
    pub u: C2Rust_Unnamed_18,
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
    pub fg: C2Rust_Unnamed_17,
    pub iCursor: ::core::ffi::c_int,
    pub colUsed: Bitmask,
    pub u1: C2Rust_Unnamed_16,
    pub u2: C2Rust_Unnamed_15,
    pub u3: C2Rust_Unnamed_14,
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
    pub u: C2Rust_Unnamed_13,
    pub pLeft: *mut Expr,
    pub pRight: *mut Expr,
    pub x: C2Rust_Unnamed_12,
    pub nHeight: ::core::ffi::c_int,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ynVar,
    pub iAgg: i16_0,
    pub w: C2Rust_Unnamed_11,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_value {
    pub u: MemValue,
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub flags: u16_0,
    pub enc: u8_0,
    pub eSubtype: u8_0,
    pub db: *mut sqlite3,
    pub szMalloc: ::core::ffi::c_int,
    pub uTemp: u32_0,
    pub zMalloc: *mut ::core::ffi::c_char,
    pub xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union MemValue {
    pub r: ::core::ffi::c_double,
    pub i: i64_0,
    pub nZero: ::core::ffi::c_int,
    pub zPType: *const ::core::ffi::c_char,
    pub pDef: *mut FuncDef

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_context {
    pub pOut: *mut Mem,
    pub pFunc: *mut FuncDef,
    pub pMem: *mut Mem,
    pub pVdbe: *mut Vdbe,
    pub iOp: ::core::ffi::c_int,
    pub isError: ::core::ffi::c_int,
    pub enc: u8_0,
    pub skipFlag: u8_0,
    pub argc: u16_0,
    pub argv: [*mut sqlite3_value; 0]

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Vdbe {
    pub db: *mut sqlite3,
    pub ppVPrev: *mut *mut Vdbe,
    pub pVNext: *mut Vdbe,
    pub pParse: *mut Parse,
    pub nVar: ynVar,
    pub nMem: ::core::ffi::c_int,
    pub nCursor: ::core::ffi::c_int,
    pub cacheCtr: u32_0,
    pub pc: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub iStatement: ::core::ffi::c_int,
    pub iCurrentTime: i64_0,
    pub nFkConstraint: i64_0,
    pub nStmtDefCons: i64_0,
    pub nStmtDefImmCons: i64_0,
    pub aMem: *mut Mem,
    pub apArg: *mut *mut Mem,
    pub apCsr: *mut *mut VdbeCursor,
    pub aVar: *mut Mem,
    pub aOp: *mut Op,
    pub nOp: ::core::ffi::c_int,
    pub nOpAlloc: ::core::ffi::c_int,
    pub aColName: *mut Mem,
    pub pResultRow: *mut Mem,
    pub zErrMsg: *mut ::core::ffi::c_char,
    pub pVList: *mut VList,
    pub startTime: i64_0,
    pub nResColumn: u16_0,
    pub nResAlloc: u16_0,
    pub errorAction: u8_0,
    pub minWriteFileFormat: u8_0,
    pub prepFlags: u8_0,
    pub eVdbeState: u8_0,
    #[bitfield(name = "expired", ty = "bft", bits = "0..=1")]
    #[bitfield(name = "explain", ty = "bft", bits = "2..=3")]
    #[bitfield(name = "changeCntOn", ty = "bft", bits = "4..=4")]
    #[bitfield(name = "usesStmtJournal", ty = "bft", bits = "5..=5")]
    #[bitfield(name = "readOnly", ty = "bft", bits = "6..=6")]
    #[bitfield(name = "bIsReader", ty = "bft", bits = "7..=7")]
    #[bitfield(name = "haveEqpOps", ty = "bft", bits = "8..=8")]
    pub expired_explain_changeCntOn_usesStmtJournal_readOnly_bIsReader_haveEqpOps: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub btreeMask: yDbMask,
    pub lockMask: yDbMask,
    pub aCounter: [u32_0; 9],
    pub zSql: *mut ::core::ffi::c_char,
    pub pFree: *mut ::core::ffi::c_void,
    pub pFrame: *mut VdbeFrame,
    pub pDelFrame: *mut VdbeFrame,
    pub nFrame: ::core::ffi::c_int,
    pub expmask: u32_0,
    pub pProgram: *mut SubProgram,
    pub pAuxData: *mut AuxData

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AuxData {
    pub iAuxOp: ::core::ffi::c_int,
    pub iAuxArg: ::core::ffi::c_int,
    pub pAux: *mut ::core::ffi::c_void,
    pub xDeleteAux: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pNextAux: *mut AuxData

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
pub type Mem = sqlite3_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeFrame {
    pub v: *mut Vdbe,
    pub pParent: *mut VdbeFrame,
    pub aOp: *mut Op,
    pub aMem: *mut Mem,
    pub apCsr: *mut *mut VdbeCursor,
    pub aOnce: *mut u8_0,
    pub token: *mut ::core::ffi::c_void,
    pub lastRowid: i64_0,
    pub pAuxData: *mut AuxData,
    pub nCursor: ::core::ffi::c_int,
    pub pc: ::core::ffi::c_int,
    pub nOp: ::core::ffi::c_int,
    pub nMem: ::core::ffi::c_int,
    pub nChildMem: ::core::ffi::c_int,
    pub nChildCsr: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub nDbChange: i64_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VdbeCursor {
    pub eCurType: u8_0,
    pub iDb: i8_0,
    pub nullRow: u8_0,
    pub deferredMoveto: u8_0,
    pub isTable: u8_0,
    #[bitfield(name = "isEphemeral", ty = "Bool", bits = "0..=0")]
    #[bitfield(name = "useRandomRowid", ty = "Bool", bits = "1..=1")]
    #[bitfield(name = "isOrdered", ty = "Bool", bits = "2..=2")]
    #[bitfield(name = "noReuse", ty = "Bool", bits = "3..=3")]
    #[bitfield(name = "colCache", ty = "Bool", bits = "4..=4")]
    pub isEphemeral_useRandomRowid_isOrdered_noReuse_colCache: [u8; 1],
    pub seekHit: u16_0,
    pub ub: C2Rust_Unnamed_4,
    pub seqCount: i64_0,
    pub cacheStatus: u32_0,
    pub seekResult: ::core::ffi::c_int,
    pub pAltCursor: *mut VdbeCursor,
    pub uc: C2Rust_Unnamed_3,
    pub pKeyInfo: *mut KeyInfo,
    pub iHdrOffset: u32_0,
    pub pgnoRoot: Pgno,
    pub nField: i16_0,
    pub nHdrParsed: u16_0,
    pub movetoTarget: i64_0,
    pub aOffset: *mut u32_0,
    pub aRow: *const u8_0,
    pub payloadSize: u32_0,
    pub szRow: u32_0,
    pub pCache: *mut VdbeTxtBlbCache,
    pub aType: [u32_0; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeTxtBlbCache {
    pub pCValue: *mut ::core::ffi::c_char,
    pub iOffset: i64_0,
    pub iCol: ::core::ffi::c_int,
    pub cacheStatus: u32_0,
    pub colCacheCtr: u32_0

}
pub type i16_0 = int16_t;
pub type int16_t = __int16_t;
pub type __int16_t = i16;
pub type Pgno = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub pCursor: *mut BtCursor,
    pub pVCur: *mut sqlite3_vtab_cursor,
    pub pSorter: *mut VdbeSorter

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub pBtx: *mut Btree,
    pub aAltMap: *mut u32_0

}
pub type Bool = ::core::ffi::c_uint;
pub type i8_0 = int8_t;
pub type int8_t = __int8_t;
pub type __int8_t = i8;
pub type Op = VdbeOp;
pub type yDbMask = ::core::ffi::c_uint;
pub type bft = ::core::ffi::c_uint;
pub type VList = ::core::ffi::c_int;
pub type ynVar = i16_0;
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
    pub u1: C2Rust_Unnamed_8,
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
    pub fg: C2Rust_Unnamed_7,
    pub u: C2Rust_Unnamed_5

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub x: C2Rust_Unnamed_6,
    pub iConstExprReg: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub iOrderByCol: u16_0,
    pub iAlias: u16_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
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
pub struct Token {
    pub z: *const ::core::ffi::c_char,
    pub n: ::core::ffi::c_uint

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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub cr: C2Rust_Unnamed_10,
    pub d: C2Rust_Unnamed_9

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
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
pub struct C2Rust_Unnamed_10 {
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
pub struct AutoincInfo {
    pub pNext: *mut AutoincInfo,
    pub pTab: *mut Table,
    pub iDb: ::core::ffi::c_int,
    pub regCtr: ::core::ffi::c_int

}
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
pub union C2Rust_Unnamed_11 {
    pub iJoin: ::core::ffi::c_int,
    pub iOfst: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub pList: *mut ExprList,
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_13 {
    pub zToken: *mut ::core::ffi::c_char,
    pub iValue: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_14 {
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
pub union C2Rust_Unnamed_15 {
    pub pIBIndex: *mut Index,
    pub pCteUse: *mut CteUse

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_16 {
    pub zIndexedBy: *mut ::core::ffi::c_char,
    pub pFuncArg: *mut ExprList,
    pub nRow: u32_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
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
pub union C2Rust_Unnamed_18 {
    pub tab: C2Rust_Unnamed_21,
    pub view: C2Rust_Unnamed_20,
    pub vtab: C2Rust_Unnamed_19

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub nArg: ::core::ffi::c_int,
    pub azArg: *mut *mut ::core::ffi::c_char,
    pub p: *mut VTable

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
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
pub union C2Rust_Unnamed_22 {
    pub isInterrupted: ::core::ffi::c_int,
    pub notUsed1: ::core::ffi::c_double

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreUpdate {
    pub v: *mut Vdbe,
    pub pCsr: *mut VdbeCursor,
    pub op: ::core::ffi::c_int,
    pub aRecord: *mut u8_0,
    pub pKeyinfo: *mut KeyInfo,
    pub pUnpacked: *mut UnpackedRecord,
    pub pNewUnpacked: *mut UnpackedRecord,
    pub iNewReg: ::core::ffi::c_int,
    pub iBlobWrite: ::core::ffi::c_int,
    pub iKey1: i64_0,
    pub iKey2: i64_0,
    pub oldipk: Mem,
    pub aNew: *mut Mem,
    pub pTab: *mut Table,
    pub pPk: *mut Index,
    pub apDflt: *mut *mut sqlite3_value,
    pub uKey: C2Rust_Unnamed_23

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub keyinfoSpace: [u8_0; 32]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnpackedRecord {
    pub pKeyInfo: *mut KeyInfo,
    pub aMem: *mut Mem,
    pub u: C2Rust_Unnamed_24,
    pub n: ::core::ffi::c_int,
    pub nField: u16_0,
    pub default_rc: i8_0,
    pub errCode: u8_0,
    pub r1: i8_0,
    pub r2: i8_0,
    pub eqSeen: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_24 {
    pub z: *mut ::core::ffi::c_char,
    pub i: i64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_25 {
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
pub type intptr_t = isize;
pub type size_t = usize;
pub type StrAccum = sqlite3_str;
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
pub struct ReusableSpace {
    pub pSpace: *mut u8_0,
    pub nFree: sqlite3_int64,
    pub nNeeded: sqlite3_int64

}
pub type RecordCompare = Option<
    unsafe extern "C" fn(
        ::core::ffi::c_int,
        *const ::core::ffi::c_void,
        *mut UnpackedRecord,
    ) -> ::core::ffi::c_int,
>;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_ABORT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_INTERRUPT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_FULL: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SQLITE_SCHEMA: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_ABORT_ROLLBACK: ::core::ffi::c_int = SQLITE_ABORT
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_COMMITHOOK: ::core::ffi::c_int = SQLITE_CONSTRAINT
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_FOREIGNKEY: ::core::ffi::c_int = SQLITE_CONSTRAINT
    | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_EXCLUSIVE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_OPEN_SUPER_JOURNAL: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SEQUENTIAL: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SQLITE_SYNC_NORMAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_ACCESS_EXISTS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_UPDATE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_VDBE_OP: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_TXN_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_TXN_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_REPREPARE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const OP_Savepoint: ::core::ffi::c_int = 0;
pub const OP_AutoCommit: ::core::ffi::c_int = 1;
pub const OP_Transaction: ::core::ffi::c_int = 2;
pub const OP_Checkpoint: ::core::ffi::c_int = 3;
pub const OP_JournalMode: ::core::ffi::c_int = 4;
pub const OP_Vacuum: ::core::ffi::c_int = 5;
pub const OP_VFilter: ::core::ffi::c_int = 6;
pub const OP_VUpdate: ::core::ffi::c_int = 7;
pub const OP_Init: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const OP_Goto: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OP_PureFunc: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const OP_Function: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
pub const OP_EndCoroutine: ::core::ffi::c_int = 69 as ::core::ffi::c_int;
pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const OP_ResultRow: ::core::ffi::c_int = 85 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_ReopenIdx: ::core::ffi::c_int = 102 as ::core::ffi::c_int;
pub const OP_OpenRead: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const OP_OpenWrite: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const OP_String8: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const OP_ParseSchema: ::core::ffi::c_int = 150 as ::core::ffi::c_int;
pub const OP_Expire: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const OP_Noop: ::core::ffi::c_int = 188 as ::core::ffi::c_int;
pub const OP_Explain: ::core::ffi::c_int = 189 as ::core::ffi::c_int;
pub const OPFLG_JUMP: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_MX_JUMP_OPCODE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeCreate(mut pParse: *mut Parse) -> *mut Vdbe {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut p: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        p = sqlite3DbMallocRawNN(db, ::core::mem::size_of::<Vdbe>() as u64_0)
            as *mut Vdbe;
        if p.is_null() {
            return ::core::ptr::null_mut::<Vdbe>();
        }
        memset(
            &raw mut (*p).aOp as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<Vdbe>() as size_t).wrapping_sub(136 as size_t),
        );
        (*p).db = db;
        if !(*db).pVdbe.is_null() {
            (*(*db).pVdbe).ppVPrev = &raw mut (*p).pVNext;
        }
        (*p).pVNext = (*db).pVdbe as *mut Vdbe;
        (*p).ppVPrev = &raw mut (*db).pVdbe as *mut *mut Vdbe;
        (*db).pVdbe = p as *mut Vdbe;
        (*p).pParse = pParse;
        (*pParse).pVdbe = p;
        sqlite3VdbeAddOp2(p, OP_Init, 0 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeParser(mut p: *mut Vdbe) -> *mut Parse {
    unsafe {
        return (*p).pParse;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeError(
    mut p: *mut Vdbe,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        
        sqlite3DbFree((*p).db, (*p).zErrMsg as *mut ::core::ffi::c_void);
        let mut ap = c2rust_args;
        (*p).zErrMsg = sqlite3VMPrintf((*p).db, zFormat, ap);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSetSql(
    mut p: *mut Vdbe,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut prepFlags: u8_0,
) {
    unsafe {
        if p.is_null() {
            return;
        }
        (*p).prepFlags = prepFlags;
        if prepFlags as ::core::ffi::c_int & SQLITE_PREPARE_SAVESQL
            == 0 as ::core::ffi::c_int
        {
            (*p).expmask = 0 as u32_0;
        }
        (*p).zSql = sqlite3DbStrNDup((*p).db, z, n as u64_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSwap(mut pA: *mut Vdbe, mut pB: *mut Vdbe) {
    unsafe {
        let mut tmp: Vdbe = Vdbe {
            db: ::core::ptr::null_mut::<sqlite3>(),
            ppVPrev: ::core::ptr::null_mut::<*mut Vdbe>(),
            pVNext: ::core::ptr::null_mut::<Vdbe>(),
            pParse: ::core::ptr::null_mut::<Parse>(),
            nVar: 0,
            nMem: 0,
            nCursor: 0,
            cacheCtr: 0,
            pc: 0,
            rc: 0,
            nChange: 0,
            iStatement: 0,
            iCurrentTime: 0,
            nFkConstraint: 0,
            nStmtDefCons: 0,
            nStmtDefImmCons: 0,
            aMem: ::core::ptr::null_mut::<Mem>(),
            apArg: ::core::ptr::null_mut::<*mut Mem>(),
            apCsr: ::core::ptr::null_mut::<*mut VdbeCursor>(),
            aVar: ::core::ptr::null_mut::<Mem>(),
            aOp: ::core::ptr::null_mut::<Op>(),
            nOp: 0,
            nOpAlloc: 0,
            aColName: ::core::ptr::null_mut::<Mem>(),
            pResultRow: ::core::ptr::null_mut::<Mem>(),
            zErrMsg: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pVList: ::core::ptr::null_mut::<VList>(),
            startTime: 0,
            nResColumn: 0,
            nResAlloc: 0,
            errorAction: 0,
            minWriteFileFormat: 0,
            prepFlags: 0,
            eVdbeState: 0,
            expired_explain_changeCntOn_usesStmtJournal_readOnly_bIsReader_haveEqpOps: [0; 2],
            c2rust_padding: [0; 2],
            btreeMask: 0,
            lockMask: 0,
            aCounter: [0; 9],
            zSql: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pFree: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pFrame: ::core::ptr::null_mut::<VdbeFrame>(),
            pDelFrame: ::core::ptr::null_mut::<VdbeFrame>(),
            nFrame: 0,
            expmask: 0,
            pProgram: ::core::ptr::null_mut::<SubProgram>(),
            pAuxData: ::core::ptr::null_mut::<AuxData>()

        };
        let mut pTmp: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut ppTmp: *mut *mut Vdbe = ::core::ptr::null_mut::<*mut Vdbe>();
        let mut zTmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        tmp = *pA;
        *pA = *pB;
        *pB = tmp;
        pTmp = (*pA).pVNext;
        (*pA).pVNext = (*pB).pVNext;
        (*pB).pVNext = pTmp;
        ppTmp = (*pA).ppVPrev;
        (*pA).ppVPrev = (*pB).ppVPrev;
        (*pB).ppVPrev = ppTmp;
        zTmp = (*pA).zSql;
        (*pA).zSql = (*pB).zSql;
        (*pB).zSql = zTmp;
        (*pB).expmask = (*pA).expmask;
        (*pB).prepFlags = (*pA).prepFlags;
        memcpy(
            &raw mut (*pB).aCounter as *mut u32_0 as *mut ::core::ffi::c_void,
            &raw mut (*pA).aCounter as *mut u32_0 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[u32_0; 9]>() as size_t,
        );
        (*pB).aCounter[SQLITE_STMTSTATUS_REPREPARE as usize] = (*pB)
            .aCounter[SQLITE_STMTSTATUS_REPREPARE as usize]
            .wrapping_add(1);
    }
}
unsafe extern "C" fn growOpArray(
    mut v: *mut Vdbe,
    mut nOp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pNew: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
        let mut p: *mut Parse = (*v).pParse;
        let mut nNew: sqlite3_int64 = if (*v).nOpAlloc != 0 {
            2 as sqlite3_int64 * (*v).nOpAlloc as sqlite3_int64
        } else {
            (1024 as usize).wrapping_div(::core::mem::size_of::<Op>() as usize)
                as sqlite3_int64
        };
        if nNew
            > (*(*p).db).aLimit[SQLITE_LIMIT_VDBE_OP as usize] as ::core::ffi::c_longlong
        {
            sqlite3OomFault((*p).db);
            return SQLITE_NOMEM;
        }
        pNew = sqlite3DbRealloc(
            (*p).db,
            (*v).aOp as *mut ::core::ffi::c_void,
            (nNew as u64_0).wrapping_mul(::core::mem::size_of::<Op>() as u64_0),
        ) as *mut VdbeOp;
        if !pNew.is_null() {
            (*p).szOpAlloc = sqlite3DbMallocSize(
                (*p).db,
                pNew as *const ::core::ffi::c_void,
            );
            (*v).nOpAlloc = ((*p).szOpAlloc as usize)
                .wrapping_div(::core::mem::size_of::<Op>() as usize)
                as ::core::ffi::c_int;
            (*v).aOp = pNew as *mut Op;
        }
        return if !pNew.is_null() { SQLITE_OK } else { SQLITE_NOMEM_BKPT };
    }
}
#[inline(never)]
unsafe extern "C" fn growOp3(
    mut p: *mut Vdbe,
    mut op: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut p2: ::core::ffi::c_int,
    mut p3: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if growOpArray(p, 1 as ::core::ffi::c_int) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return sqlite3VdbeAddOp3(p, op, p1, p2, p3);
    }
}
#[inline(never)]
unsafe extern "C" fn addOp4IntSlow(
    mut p: *mut Vdbe,
    mut op: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut p2: ::core::ffi::c_int,
    mut p3: ::core::ffi::c_int,
    mut p4: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut addr: ::core::ffi::c_int = sqlite3VdbeAddOp3(p, op, p1, p2, p3);
        if (*(*p).db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut pOp: *mut VdbeOp = (*p).aOp.offset(addr as isize) as *mut VdbeOp;
            (*pOp).p4type = P4_INT32 as ::core::ffi::c_schar;
            (*pOp).p4.i = p4;
        }
        return addr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeAddOp0(
    mut p: *mut Vdbe,
    mut op: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3VdbeAddOp3(
            p,
            op,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeAddOp1(
    mut p: *mut Vdbe,
    mut op: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3VdbeAddOp3(
            p,
            op,
            p1,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeAddOp2(
    mut p: *mut Vdbe,
    mut op: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut p2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3VdbeAddOp3(p, op, p1, p2, 0 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeAddOp3(
    mut p: *mut Vdbe,
    mut op: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut p2: ::core::ffi::c_int,
    mut p3: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
        i = (*p).nOp;
        if (*p).nOpAlloc <= i {
            return growOp3(p, op, p1, p2, p3);
        }
        (*p).nOp += 1;
        pOp = (*p).aOp.offset(i as isize) as *mut Op as *mut VdbeOp;
        (*pOp).opcode = op as u8_0;
        (*pOp).p5 = 0 as u16_0;
        (*pOp).p1 = p1;
        (*pOp).p2 = p2;
        (*pOp).p3 = p3;
        (*pOp).p4.p = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*pOp).p4type = P4_NOTUSED as ::core::ffi::c_schar;
        return i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeAddOp4Int(
    mut p: *mut Vdbe,
    mut op: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut p2: ::core::ffi::c_int,
    mut p3: ::core::ffi::c_int,
    mut p4: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
        i = (*p).nOp;
        if (*p).nOpAlloc <= i {
            return addOp4IntSlow(p, op, p1, p2, p3, p4);
        }
        (*p).nOp += 1;
        pOp = (*p).aOp.offset(i as isize) as *mut Op as *mut VdbeOp;
        (*pOp).opcode = op as u8_0;
        (*pOp).p5 = 0 as u16_0;
        (*pOp).p1 = p1;
        (*pOp).p2 = p2;
        (*pOp).p3 = p3;
        (*pOp).p4.i = p4;
        (*pOp).p4type = P4_INT32 as ::core::ffi::c_schar;
        return i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeGoto(
    mut p: *mut Vdbe,
    mut iDest: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3VdbeAddOp3(
            p,
            OP_Goto,
            0 as ::core::ffi::c_int,
            iDest,
            0 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeLoadString(
    mut p: *mut Vdbe,
    mut iDest: ::core::ffi::c_int,
    mut zStr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3VdbeAddOp4(
            p,
            OP_String8,
            0 as ::core::ffi::c_int,
            iDest,
            0 as ::core::ffi::c_int,
            zStr,
            0 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMultiLoad(
    mut p: *mut Vdbe,
    mut iDest: ::core::ffi::c_int,
    mut zTypes: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut c2rust_current_block: u64;
        
        let mut i: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_char = 0;
        let mut ap = c2rust_args;
        i = 0 as ::core::ffi::c_int;
        loop {
            c = *zTypes.offset(i as isize);
            if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                c2rust_current_block = 11812396948646013369;
                break;
            }
            if c as ::core::ffi::c_int == 's' as i32 {
                let mut z: *const ::core::ffi::c_char = ap
                    .arg::<*const ::core::ffi::c_char>();
                sqlite3VdbeAddOp4(
                    p,
                    if z.is_null() { OP_Null } else { OP_String8 },
                    0 as ::core::ffi::c_int,
                    iDest + i,
                    0 as ::core::ffi::c_int,
                    z,
                    0 as ::core::ffi::c_int,
                );
            } else {
                if !(c as ::core::ffi::c_int == 'i' as i32) {
                    c2rust_current_block = 2968425633554183086;
                    break;
                }
                sqlite3VdbeAddOp2(
                    p,
                    OP_Integer,
                    ap.arg::<::core::ffi::c_int>(),
                    iDest + i,
                );
            }
            i += 1;
        }
        match c2rust_current_block {
            11812396948646013369 => {
                sqlite3VdbeAddOp2(p, OP_ResultRow, iDest, i);
            }
            _ => {}
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeAddOp4(
    mut p: *mut Vdbe,
    mut op: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut p2: ::core::ffi::c_int,
    mut p3: ::core::ffi::c_int,
    mut zP4: *const ::core::ffi::c_char,
    mut p4type: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut addr: ::core::ffi::c_int = sqlite3VdbeAddOp3(p, op, p1, p2, p3);
        sqlite3VdbeChangeP4(p, addr, zP4, p4type);
        return addr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeAddFunctionCall(
    mut pParse: *mut Parse,
    mut p1: ::core::ffi::c_int,
    mut p2: ::core::ffi::c_int,
    mut p3: ::core::ffi::c_int,
    mut nArg: ::core::ffi::c_int,
    mut pFunc: *const FuncDef,
    mut eCallCtx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut addr: ::core::ffi::c_int = 0;
        let mut pCtx: *mut sqlite3_context = ::core::ptr::null_mut::<sqlite3_context>();
        pCtx = sqlite3DbMallocRawNN(
            (*pParse).db,
            (48 as usize)
                .wrapping_add(
                    (nArg as usize)
                        .wrapping_mul(
                            ::core::mem::size_of::<*mut sqlite3_value>() as usize,
                        ),
                ) as u64_0,
        ) as *mut sqlite3_context;
        if pCtx.is_null() {
            freeEphemeralFunction((*pParse).db, pFunc as *mut FuncDef);
            return 0 as ::core::ffi::c_int;
        }
        (*pCtx).pOut = ::core::ptr::null_mut::<Mem>();
        (*pCtx).pFunc = pFunc as *mut FuncDef;
        (*pCtx).pVdbe = ::core::ptr::null_mut::<Vdbe>();
        (*pCtx).isError = 0 as ::core::ffi::c_int;
        (*pCtx).argc = nArg as u16_0;
        (*pCtx).iOp = sqlite3VdbeCurrentAddr(v);
        addr = sqlite3VdbeAddOp4(
            v,
            if eCallCtx != 0 { OP_PureFunc } else { OP_Function },
            p1,
            p2,
            p3,
            pCtx as *mut ::core::ffi::c_char,
            P4_FUNCCTX,
        );
        sqlite3VdbeChangeP5(v, (eCallCtx & NC_SelfRef) as u16_0);
        sqlite3MayAbort(pParse);
        return addr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeAddOp4Dup8(
    mut p: *mut Vdbe,
    mut op: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut p2: ::core::ffi::c_int,
    mut p3: ::core::ffi::c_int,
    mut zP4: *const u8_0,
    mut p4type: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p4copy: *mut ::core::ffi::c_char = sqlite3DbMallocRawNN(
            sqlite3VdbeDb(p),
            8 as u64_0,
        ) as *mut ::core::ffi::c_char;
        if !p4copy.is_null() {
            memcpy(
                p4copy as *mut ::core::ffi::c_void,
                zP4 as *const ::core::ffi::c_void,
                8 as size_t,
            );
        }
        return sqlite3VdbeAddOp4(p, op, p1, p2, p3, p4copy, p4type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeExplainParent(
    mut pParse: *mut Parse,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
        if (*pParse).addrExplain == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        pOp = sqlite3VdbeGetOp((*pParse).pVdbe, (*pParse).addrExplain);
        return (*pOp).p2;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeExplain(
    mut pParse: *mut Parse,
    mut bPush: u8_0,
    mut zFmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    unsafe {
        let mut addr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*pParse).explain as ::core::ffi::c_int == 2 as ::core::ffi::c_int || false {
            let mut zMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
            
            let mut iThis: ::core::ffi::c_int = 0;
            let mut ap = c2rust_args;
            zMsg = sqlite3VMPrintf((*pParse).db, zFmt, ap);
            v = (*pParse).pVdbe;
            iThis = (*v).nOp;
            addr = sqlite3VdbeAddOp4(
                v,
                OP_Explain,
                iThis,
                (*pParse).addrExplain,
                0 as ::core::ffi::c_int,
                zMsg,
                P4_DYNAMIC,
            );
            if bPush != 0 {
                (*pParse).addrExplain = iThis;
            }
        }
        return addr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeExplainPop(mut pParse: *mut Parse) {
    unsafe {
        (*pParse).addrExplain = sqlite3VdbeExplainParent(pParse);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeAddParseSchemaOp(
    mut p: *mut Vdbe,
    mut iDb: ::core::ffi::c_int,
    mut zWhere: *mut ::core::ffi::c_char,
    mut p5: u16_0,
) {
    unsafe {
        let mut j: ::core::ffi::c_int = 0;
        sqlite3VdbeAddOp4(
            p,
            OP_ParseSchema,
            iDb,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            zWhere,
            P4_DYNAMIC,
        );
        sqlite3VdbeChangeP5(p, p5);
        j = 0 as ::core::ffi::c_int;
        while j < (*(*p).db).nDb {
            sqlite3VdbeUsesBtree(p, j);
            j += 1;
        }
        sqlite3MayAbort((*p).pParse);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeEndCoroutine(
    mut v: *mut Vdbe,
    mut regYield: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3VdbeAddOp1(v, OP_EndCoroutine, regYield);
        (*(*v).pParse).nTempReg = 0 as u8_0;
        (*(*v).pParse).nRangeReg = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMakeLabel(
    mut pParse: *mut Parse,
) -> ::core::ffi::c_int {
    unsafe {
        (*pParse).nLabel -= 1;
        return (*pParse).nLabel;
    }
}
#[inline(never)]
unsafe extern "C" fn resizeResolveLabel(
    mut p: *mut Parse,
    mut v: *mut Vdbe,
    mut j: ::core::ffi::c_int,
) {
    unsafe {
        let mut nNewSize: ::core::ffi::c_int = 10 as ::core::ffi::c_int - (*p).nLabel;
        (*p).aLabel = sqlite3DbReallocOrFree(
            (*p).db,
            (*p).aLabel as *mut ::core::ffi::c_void,
            (nNewSize as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                as u64_0,
        ) as *mut ::core::ffi::c_int;
        if (*p).aLabel.is_null() {
            (*p).nLabelAlloc = 0 as ::core::ffi::c_int;
        } else {
            if nNewSize >= 100 as ::core::ffi::c_int
                && nNewSize / 100 as ::core::ffi::c_int
                    > (*p).nLabelAlloc / 100 as ::core::ffi::c_int
            {
                sqlite3ProgressCheck(p);
            }
            (*p).nLabelAlloc = nNewSize;
            *(*p).aLabel.offset(j as isize) = (*v).nOp;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeResolveLabel(
    mut v: *mut Vdbe,
    mut x: ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *mut Parse = (*v).pParse;
        let mut j: ::core::ffi::c_int = !x;
        if (*p).nLabelAlloc + (*p).nLabel < 0 as ::core::ffi::c_int {
            resizeResolveLabel(p, v, j);
        } else {
            *(*p).aLabel.offset(j as isize) = (*v).nOp;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeRunOnlyOnce(mut p: *mut Vdbe) {
    unsafe {
        sqlite3VdbeAddOp2(
            p,
            OP_Expire,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeReusable(mut p: *mut Vdbe) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 1 as ::core::ffi::c_int;
        while i < (*p).nOp {
            if (*(*p).aOp.offset(i as isize)).opcode as ::core::ffi::c_int
                == 167 as ::core::ffi::c_int
            {
                (*(*p).aOp.offset(1 as ::core::ffi::c_int as isize)).opcode = OP_Noop
                    as u8_0;
                break;
            } else {
                i += 1;
            }
        }
    }
}
unsafe extern "C" fn resolveP2Values(
    mut p: *mut Vdbe,
    mut pMaxVtabArgs: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut nMaxVtabArgs: ::core::ffi::c_int = *pMaxVtabArgs;
        let mut pOp: *mut Op = ::core::ptr::null_mut::<Op>();
        let mut pParse: *mut Parse = (*p).pParse;
        let mut aLabel: *mut ::core::ffi::c_int = (*pParse).aLabel;
        (*p).set_readOnly(1 as bft as bft);
        (*p).set_bIsReader(0 as bft as bft);
        pOp = (*p).aOp.offset(((*p).nOp - 1 as ::core::ffi::c_int) as isize) as *mut Op;
        loop {
            if (*pOp).opcode as ::core::ffi::c_int <= SQLITE_MX_JUMP_OPCODE {
                match (*pOp).opcode as ::core::ffi::c_int {
                    OP_Transaction => {
                        if (*pOp).p2 != 0 as ::core::ffi::c_int {
                            (*p).set_readOnly(0 as bft as bft);
                        }
                        c2rust_current_block = 11812396948646013369;
                    }
                    OP_AutoCommit | OP_Savepoint => {
                        c2rust_current_block = 11812396948646013369;
                    }
                    OP_Checkpoint | OP_Vacuum | OP_JournalMode => {
                        (*p).set_readOnly(0 as bft as bft);
                        (*p).set_bIsReader(1 as bft as bft);
                        c2rust_current_block = 8704759739624374314;
                    }
                    OP_Init => {
                        break;
                    }
                    OP_VUpdate => {
                        if (*pOp).p2 > nMaxVtabArgs {
                            nMaxVtabArgs = (*pOp).p2;
                        }
                        c2rust_current_block = 8704759739624374314;
                    }
                    OP_VFilter => {
                        let mut n: ::core::ffi::c_int = 0;
                        n = (*pOp.offset(-1 as ::core::ffi::c_int as isize)).p1;
                        if n > nMaxVtabArgs {
                            nMaxVtabArgs = n;
                        }
                        c2rust_current_block = 2719512138335094285;
                    }
                    _ => {
                        c2rust_current_block = 2719512138335094285;
                    }
                }
                match c2rust_current_block {
                    8704759739624374314 => {}
                    _ => {
                        match c2rust_current_block {
                            2719512138335094285 => {
                                if (*pOp).p2 < 0 as ::core::ffi::c_int {
                                    (*pOp).p2 = *aLabel.offset(!(*pOp).p2 as isize);
                                }
                            }
                            _ => {
                                (*p).set_bIsReader(1 as bft as bft);
                            }
                        }
                    }
                }
            }
            pOp = pOp.offset(-1);
        }
        if !aLabel.is_null() {
            sqlite3DbNNFreeNN((*p).db, (*pParse).aLabel as *mut ::core::ffi::c_void);
            (*pParse).aLabel = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        (*pParse).nLabel = 0 as ::core::ffi::c_int;
        *pMaxVtabArgs = nMaxVtabArgs;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeCurrentAddr(mut p: *mut Vdbe) -> ::core::ffi::c_int {
    unsafe {
        return (*p).nOp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeTakeOpArray(
    mut p: *mut Vdbe,
    mut pnOp: *mut ::core::ffi::c_int,
    mut pnMaxArg: *mut ::core::ffi::c_int,
) -> *mut VdbeOp {
    unsafe {
        let mut aOp: *mut VdbeOp = (*p).aOp as *mut VdbeOp;
        resolveP2Values(p, pnMaxArg);
        *pnOp = (*p).nOp;
        (*p).aOp = ::core::ptr::null_mut::<Op>();
        return aOp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeAddOpList(
    mut p: *mut Vdbe,
    mut nOp: ::core::ffi::c_int,
    mut aOp: *const VdbeOpList,
    mut iLineno: ::core::ffi::c_int,
) -> *mut VdbeOp {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pOut: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
        let mut pFirst: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
        if (*p).nOp + nOp > (*p).nOpAlloc && growOpArray(p, nOp) != 0 {
            return ::core::ptr::null_mut::<VdbeOp>();
        }
        pOut = (*p).aOp.offset((*p).nOp as isize) as *mut Op as *mut VdbeOp;
        pFirst = pOut;
        i = 0 as ::core::ffi::c_int;
        while i < nOp {
            (*pOut).opcode = (*aOp).opcode;
            (*pOut).p1 = (*aOp).p1 as ::core::ffi::c_int;
            (*pOut).p2 = (*aOp).p2 as ::core::ffi::c_int;
            if *(&raw const sqlite3OpcodeProperty as *const ::core::ffi::c_uchar)
                .offset((*aOp).opcode as isize) as ::core::ffi::c_int & OPFLG_JUMP
                != 0 as ::core::ffi::c_int
                && (*aOp).p2 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            {
                (*pOut).p2 += (*p).nOp;
            }
            (*pOut).p3 = (*aOp).p3 as ::core::ffi::c_int;
            (*pOut).p4type = P4_NOTUSED as ::core::ffi::c_schar;
            (*pOut).p4.p = ::core::ptr::null_mut::<::core::ffi::c_void>();
            (*pOut).p5 = 0 as u16_0;
            i += 1;
            aOp = aOp.offset(1);
            pOut = pOut.offset(1);
        }
        (*p).nOp += nOp;
        return pFirst;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeChangeOpcode(
    mut p: *mut Vdbe,
    mut addr: ::core::ffi::c_int,
    mut iNewOpcode: u8_0,
) {
    unsafe {
        (*sqlite3VdbeGetOp(p, addr)).opcode = iNewOpcode;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeChangeP1(
    mut p: *mut Vdbe,
    mut addr: ::core::ffi::c_int,
    mut val: ::core::ffi::c_int,
) {
    unsafe {
        (*sqlite3VdbeGetOp(p, addr)).p1 = val;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeChangeP2(
    mut p: *mut Vdbe,
    mut addr: ::core::ffi::c_int,
    mut val: ::core::ffi::c_int,
) {
    unsafe {
        (*sqlite3VdbeGetOp(p, addr)).p2 = val;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeChangeP3(
    mut p: *mut Vdbe,
    mut addr: ::core::ffi::c_int,
    mut val: ::core::ffi::c_int,
) {
    unsafe {
        (*sqlite3VdbeGetOp(p, addr)).p3 = val;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeChangeP5(mut p: *mut Vdbe, mut p5: u16_0) {
    unsafe {
        if (*p).nOp > 0 as ::core::ffi::c_int {
            (*(*p).aOp.offset(((*p).nOp - 1 as ::core::ffi::c_int) as isize)).p5 = p5;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeTypeofColumn(
    mut p: *mut Vdbe,
    mut iDest: ::core::ffi::c_int,
) {
    unsafe {
        let mut pOp: *mut VdbeOp = sqlite3VdbeGetLastOp(p);
        if (*pOp).p3 == iDest && (*pOp).opcode as ::core::ffi::c_int == OP_Column {
            (*pOp).p5 = ((*pOp).p5 as ::core::ffi::c_int | OPFLAG_TYPEOFARG) as u16_0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeJumpHere(
    mut p: *mut Vdbe,
    mut addr: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3VdbeChangeP2(p, addr, (*p).nOp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeJumpHereOrPopInst(
    mut p: *mut Vdbe,
    mut addr: ::core::ffi::c_int,
) {
    unsafe {
        if addr == (*p).nOp - 1 as ::core::ffi::c_int {
            (*p).nOp -= 1;
        } else {
            sqlite3VdbeChangeP2(p, addr, (*p).nOp);
        };
    }
}
unsafe extern "C" fn freeEphemeralFunction(
    mut db: *mut sqlite3,
    mut pDef: *mut FuncDef,
) {
    unsafe {
        if (*pDef).funcFlags & SQLITE_FUNC_EPHEM as u32_0 != 0 as u32_0 {
            sqlite3DbNNFreeNN(db, pDef as *mut ::core::ffi::c_void);
        }
    }
}
#[inline(never)]
unsafe extern "C" fn freeP4Mem(mut db: *mut sqlite3, mut p: *mut Mem) {
    unsafe {
        if (*p).szMalloc != 0 {
            sqlite3DbFree(db, (*p).zMalloc as *mut ::core::ffi::c_void);
        }
        sqlite3DbNNFreeNN(db, p as *mut ::core::ffi::c_void);
    }
}
#[inline(never)]
unsafe extern "C" fn freeP4FuncCtx(mut db: *mut sqlite3, mut p: *mut sqlite3_context) {
    unsafe {
        freeEphemeralFunction(db, (*p).pFunc);
        sqlite3DbNNFreeNN(db, p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn freeP4(
    mut db: *mut sqlite3,
    mut p4type: ::core::ffi::c_int,
    mut p4: *mut ::core::ffi::c_void,
) {
    unsafe {
        match p4type {
            P4_FUNCCTX => {
                freeP4FuncCtx(db, p4 as *mut sqlite3_context);
            }
            P4_REAL | P4_INT64 | P4_DYNAMIC | P4_INTARRAY => {
                if !p4.is_null() {
                    sqlite3DbNNFreeNN(db, p4);
                }
            }
            P4_KEYINFO => {
                if (*db).pnBytesFreed.is_null() {
                    sqlite3KeyInfoUnref(p4 as *mut KeyInfo);
                }
            }
            P4_FUNCDEF => {
                freeEphemeralFunction(db, p4 as *mut FuncDef);
            }
            P4_MEM => {
                if (*db).pnBytesFreed.is_null() {
                    sqlite3ValueFree(p4 as *mut sqlite3_value);
                } else {
                    freeP4Mem(db, p4 as *mut Mem);
                }
            }
            P4_VTAB => {
                if (*db).pnBytesFreed.is_null() {
                    sqlite3VtabUnlock(p4 as *mut VTable);
                }
            }
            P4_TABLEREF => {
                if (*db).pnBytesFreed.is_null() {
                    sqlite3DeleteTable(db, p4 as *mut Table);
                }
            }
            P4_SUBRTNSIG => {
                let mut pSig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                sqlite3DbFree(db, (*pSig).zAff as *mut ::core::ffi::c_void);
                sqlite3DbFree(db, pSig as *mut ::core::ffi::c_void);
            }
            _ => {}
        };
    }
}
unsafe extern "C" fn vdbeFreeOpArray(
    mut db: *mut sqlite3,
    mut aOp: *mut Op,
    mut nOp: ::core::ffi::c_int,
) {
    unsafe {
        if !aOp.is_null() {
            let mut pOp: *mut Op = aOp.offset((nOp - 1 as ::core::ffi::c_int) as isize)
                as *mut Op;
            loop {
                if (*pOp).p4type as ::core::ffi::c_int <= P4_FREE_IF_LE {
                    freeP4(db, (*pOp).p4type as ::core::ffi::c_int, (*pOp).p4.p);
                }
                if pOp == aOp {
                    break;
                }
                pOp = pOp.offset(-1);
            }
            sqlite3DbNNFreeNN(db, aOp as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeLinkSubProgram(
    mut pVdbe: *mut Vdbe,
    mut p: *mut SubProgram,
) {
    unsafe {
        (*p).pNext = (*pVdbe).pProgram;
        (*pVdbe).pProgram = p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeHasSubProgram(
    mut pVdbe: *mut Vdbe,
) -> ::core::ffi::c_int {
    unsafe {
        return !(*pVdbe).pProgram.is_null() as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeChangeToNoop(
    mut p: *mut Vdbe,
    mut addr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
        if (*(*p).db).mallocFailed != 0 {
            return 0 as ::core::ffi::c_int;
        }
        pOp = (*p).aOp.offset(addr as isize) as *mut Op as *mut VdbeOp;
        freeP4((*p).db, (*pOp).p4type as ::core::ffi::c_int, (*pOp).p4.p);
        (*pOp).p4type = P4_NOTUSED as ::core::ffi::c_schar;
        (*pOp).p4.z = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*pOp).opcode = OP_Noop as u8_0;
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeDeletePriorOpcode(
    mut p: *mut Vdbe,
    mut op: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        if (*p).nOp > 0 as ::core::ffi::c_int
            && (*(*p).aOp.offset(((*p).nOp - 1 as ::core::ffi::c_int) as isize)).opcode
                as ::core::ffi::c_int == op as ::core::ffi::c_int
        {
            return sqlite3VdbeChangeToNoop(p, (*p).nOp - 1 as ::core::ffi::c_int)
        } else {
            return 0 as ::core::ffi::c_int
        };
    }
}
#[inline(never)]
unsafe extern "C" fn vdbeChangeP4Full(
    mut p: *mut Vdbe,
    mut pOp: *mut Op,
    mut zP4: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) {
    unsafe {
        if (*pOp).p4type != 0 {
            (*pOp).p4type = 0 as ::core::ffi::c_schar;
            (*pOp).p4.p = ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        if n < 0 as ::core::ffi::c_int {
            sqlite3VdbeChangeP4(
                p,
                pOp.offset_from((*p).aOp) as ::core::ffi::c_long as ::core::ffi::c_int,
                zP4,
                n,
            );
        } else {
            if n == 0 as ::core::ffi::c_int {
                n = sqlite3Strlen30(zP4);
            }
            (*pOp).p4.z = sqlite3DbStrNDup((*p).db, zP4, n as u64_0);
            (*pOp).p4type = P4_DYNAMIC as ::core::ffi::c_schar;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeChangeP4(
    mut p: *mut Vdbe,
    mut addr: ::core::ffi::c_int,
    mut zP4: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) {
    unsafe {
        let mut pOp: *mut Op = ::core::ptr::null_mut::<Op>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        db = (*p).db;
        if (*db).mallocFailed != 0 {
            if n != P4_VTAB {
                freeP4(
                    db,
                    n,
                    *(&raw mut zP4 as *mut *mut ::core::ffi::c_char)
                        as *mut ::core::ffi::c_void,
                );
            }
            return;
        }
        if addr < 0 as ::core::ffi::c_int {
            addr = (*p).nOp - 1 as ::core::ffi::c_int;
        }
        pOp = (*p).aOp.offset(addr as isize) as *mut Op;
        if n >= 0 as ::core::ffi::c_int || (*pOp).p4type as ::core::ffi::c_int != 0 {
            vdbeChangeP4Full(p, pOp, zP4, n);
            return;
        }
        if n == P4_INT32 {
            (*pOp).p4.i = zP4 as intptr_t as ::core::ffi::c_int;
            (*pOp).p4type = P4_INT32 as ::core::ffi::c_schar;
        } else if !zP4.is_null() {
            (*pOp).p4.p = zP4 as *mut ::core::ffi::c_void;
            (*pOp).p4type = n as ::core::ffi::c_schar;
            if n == P4_VTAB {
                sqlite3VtabLock(zP4 as *mut VTable);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeAppendP4(
    mut p: *mut Vdbe,
    mut pP4: *mut ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
) {
    unsafe {
        let mut pOp: *mut VdbeOp = ::core::ptr::null_mut::<VdbeOp>();
        if (*(*p).db).mallocFailed != 0 {
            freeP4((*p).db, n, pP4);
        } else {
            pOp = (*p).aOp.offset(((*p).nOp - 1 as ::core::ffi::c_int) as isize)
                as *mut Op as *mut VdbeOp;
            (*pOp).p4type = n as ::core::ffi::c_schar;
            (*pOp).p4.p = pP4;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSetP4KeyInfo(
    mut pParse: *mut Parse,
    mut pIdx: *mut Index,
) {
    unsafe {
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        let mut pKeyInfo: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        pKeyInfo = sqlite3KeyInfoOfIndex(pParse, pIdx);
        if !pKeyInfo.is_null() {
            sqlite3VdbeAppendP4(v, pKeyInfo as *mut ::core::ffi::c_void, P4_KEYINFO);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeGetOp(
    mut p: *mut Vdbe,
    mut addr: ::core::ffi::c_int,
) -> *mut VdbeOp {
    unsafe {
        static mut dummy: VdbeOp = VdbeOp {
            opcode: 0,
            p4type: 0,
            p5: 0,
            p1: 0,
            p2: 0,
            p3: 0,
            p4: p4union { i: 0 }

        };
        if (*(*p).db).mallocFailed != 0 {
            return &raw mut dummy
        } else {
            return (*p).aOp.offset(addr as isize) as *mut VdbeOp
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeGetLastOp(mut p: *mut Vdbe) -> *mut VdbeOp {
    unsafe {
        return sqlite3VdbeGetOp(p, (*p).nOp - 1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeDisplayP4(
    mut db: *mut sqlite3,
    mut pOp: *mut Op,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zP4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut x: StrAccum = StrAccum {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        sqlite3StrAccumInit(
            &raw mut x,
            ::core::ptr::null_mut::<sqlite3>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            SQLITE_MAX_LENGTH,
        );
        match (*pOp).p4type as ::core::ffi::c_int {
            P4_KEYINFO => {
                let mut j: ::core::ffi::c_int = 0;
                let mut pKeyInfo: *mut KeyInfo = (*pOp).p4.pKeyInfo;
                sqlite3_str_appendf(
                    &raw mut x,
                    b"k(%d\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pKeyInfo).nKeyField as ::core::ffi::c_int,
                );
                j = 0 as ::core::ffi::c_int;
                while j < (*pKeyInfo).nKeyField as ::core::ffi::c_int {
                    let mut pColl: *mut CollSeq = *(&raw mut (*pKeyInfo).aColl
                        as *mut *mut CollSeq)
                        .offset(j as isize);
                    let mut zColl: *const ::core::ffi::c_char = if !pColl.is_null() {
                        (*pColl).zName as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    };
                    if strcmp(zColl, b"BINARY\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        zColl = b"B\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                    sqlite3_str_appendf(
                        &raw mut x,
                        b",%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                        if *(*pKeyInfo).aSortFlags.offset(j as isize)
                            as ::core::ffi::c_int & KEYINFO_ORDER_DESC != 0
                        {
                            b"-\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        if *(*pKeyInfo).aSortFlags.offset(j as isize)
                            as ::core::ffi::c_int & KEYINFO_ORDER_BIGNULL != 0
                        {
                            b"N.\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        zColl,
                    );
                    j += 1;
                }
                sqlite3_str_append(
                    &raw mut x,
                    b")\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                );
            }
            P4_COLLSEQ => {
                static mut encnames: [*const ::core::ffi::c_char; 4] = [
                    b"?\0".as_ptr() as *const ::core::ffi::c_char,
                    b"8\0".as_ptr() as *const ::core::ffi::c_char,
                    b"16LE\0".as_ptr() as *const ::core::ffi::c_char,
                    b"16BE\0".as_ptr() as *const ::core::ffi::c_char,
                ];
                let mut pColl_0: *mut CollSeq = (*pOp).p4.pColl;
                sqlite3_str_appendf(
                    &raw mut x,
                    b"%.18s-%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pColl_0).zName,
                    encnames[(*pColl_0).enc as usize],
                );
            }
            P4_FUNCDEF => {
                let mut pDef: *mut FuncDef = (*pOp).p4.pFunc;
                sqlite3_str_appendf(
                    &raw mut x,
                    b"%s(%d)\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pDef).zName,
                    (*pDef).nArg as ::core::ffi::c_int,
                );
            }
            P4_FUNCCTX => {
                let mut pDef_0: *mut FuncDef = (*(*pOp).p4.pCtx).pFunc;
                sqlite3_str_appendf(
                    &raw mut x,
                    b"%s(%d)\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pDef_0).zName,
                    (*pDef_0).nArg as ::core::ffi::c_int,
                );
            }
            P4_INT64 => {
                sqlite3_str_appendf(
                    &raw mut x,
                    b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
                    *(*pOp).p4.pI64,
                );
            }
            P4_INT32 => {
                sqlite3_str_appendf(
                    &raw mut x,
                    b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pOp).p4.i,
                );
            }
            P4_REAL => {
                sqlite3_str_appendf(
                    &raw mut x,
                    b"%.16g\0".as_ptr() as *const ::core::ffi::c_char,
                    *(*pOp).p4.pReal,
                );
            }
            P4_MEM => {
                let mut pMem: *mut Mem = (*pOp).p4.pMem;
                if (*pMem).flags as ::core::ffi::c_int & MEM_Str != 0 {
                    zP4 = (*pMem).z;
                } else if (*pMem).flags as ::core::ffi::c_int & (MEM_Int | MEM_IntReal)
                    != 0
                {
                    sqlite3_str_appendf(
                        &raw mut x,
                        b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
                        (*pMem).u.i,
                    );
                } else if (*pMem).flags as ::core::ffi::c_int & MEM_Real != 0 {
                    sqlite3_str_appendf(
                        &raw mut x,
                        b"%.16g\0".as_ptr() as *const ::core::ffi::c_char,
                        (*pMem).u.r,
                    );
                } else if (*pMem).flags as ::core::ffi::c_int & MEM_Null != 0 {
                    zP4 = b"NULL\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                } else {
                    zP4 = b"(blob)\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                }
            }
            P4_VTAB => {
                let mut pVtab: *mut sqlite3_vtab = (*(*pOp).p4.pVtab).pVtab;
                sqlite3_str_appendf(
                    &raw mut x,
                    b"vtab:%p\0".as_ptr() as *const ::core::ffi::c_char,
                    pVtab,
                );
            }
            P4_INTARRAY => {
                let mut i: u32_0 = 0;
                let mut ai: *mut u32_0 = (*pOp).p4.ai;
                let mut n: u32_0 = *ai.offset(0 as ::core::ffi::c_int as isize);
                i = 1 as u32_0;
                while i <= n {
                    sqlite3_str_appendf(
                        &raw mut x,
                        b"%c%u\0".as_ptr() as *const ::core::ffi::c_char,
                        if i == 1 as u32_0 { '[' as i32 } else { ',' as i32 },
                        *ai.offset(i as isize),
                    );
                    i = i.wrapping_add(1);
                }
                sqlite3_str_append(
                    &raw mut x,
                    b"]\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                );
            }
            P4_SUBPROGRAM => {
                zP4 = b"program\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            P4_TABLE => {
                zP4 = (*(*pOp).p4.pTab).zName;
            }
            P4_SUBRTNSIG => {
                let mut pSig: *mut SubrtnSig = (*pOp).p4.pSubrtnSig;
                sqlite3_str_appendf(
                    &raw mut x,
                    b"subrtnsig:%d,%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pSig).selId,
                    (*pSig).zAff,
                );
            }
            _ => {
                zP4 = (*pOp).p4.z;
            }
        }
        if !zP4.is_null() {
            sqlite3_str_appendall(&raw mut x, zP4);
        }
        if x.accError as ::core::ffi::c_int & SQLITE_NOMEM != 0 as ::core::ffi::c_int {
            sqlite3OomFault(db);
        }
        return sqlite3StrAccumFinish(&raw mut x);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeUsesBtree(
    mut p: *mut Vdbe,
    mut i: ::core::ffi::c_int,
) {
    unsafe {
        (*p).btreeMask
            |= ((1 as ::core::ffi::c_int as yDbMask) << i) as ::core::ffi::c_uint;
        if i != 1 as ::core::ffi::c_int
            && sqlite3BtreeSharable((*(*(*p).db).aDb.offset(i as isize)).pBt) != 0
        {
            (*p).lockMask
                |= ((1 as ::core::ffi::c_int as yDbMask) << i) as ::core::ffi::c_uint;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeEnter(mut p: *mut Vdbe) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut aDb: *mut Db = ::core::ptr::null_mut::<Db>();
        let mut nDb: ::core::ffi::c_int = 0;
        if (*p).lockMask == 0 as ::core::ffi::c_uint {
            return;
        }
        db = (*p).db;
        aDb = (*db).aDb;
        nDb = (*db).nDb;
        i = 0 as ::core::ffi::c_int;
        while i < nDb {
            if i != 1 as ::core::ffi::c_int
                && (*p).lockMask & (1 as ::core::ffi::c_int as yDbMask) << i
                    != 0 as ::core::ffi::c_uint
                && !(*aDb.offset(i as isize)).pBt.is_null()
            {
                sqlite3BtreeEnter((*aDb.offset(i as isize)).pBt);
            }
            i += 1;
        }
    }
}
#[inline(never)]
unsafe extern "C" fn vdbeLeave(mut p: *mut Vdbe) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut aDb: *mut Db = ::core::ptr::null_mut::<Db>();
        let mut nDb: ::core::ffi::c_int = 0;
        db = (*p).db;
        aDb = (*db).aDb;
        nDb = (*db).nDb;
        i = 0 as ::core::ffi::c_int;
        while i < nDb {
            if i != 1 as ::core::ffi::c_int
                && (*p).lockMask & (1 as ::core::ffi::c_int as yDbMask) << i
                    != 0 as ::core::ffi::c_uint
                && !(*aDb.offset(i as isize)).pBt.is_null()
            {
                sqlite3BtreeLeave((*aDb.offset(i as isize)).pBt);
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeLeave(mut p: *mut Vdbe) {
    unsafe {
        if (*p).lockMask == 0 as ::core::ffi::c_uint {
            return;
        }
        vdbeLeave(p);
    }
}
unsafe extern "C" fn initMemArray(
    mut p: *mut Mem,
    mut N: ::core::ffi::c_int,
    mut db: *mut sqlite3,
    mut flags: u16_0,
) {
    unsafe {
        if N > 0 as ::core::ffi::c_int {
            loop {
                (*p).flags = flags;
                (*p).db = db;
                (*p).szMalloc = 0 as ::core::ffi::c_int;
                p = p.offset(1);
                N -= 1;
                if !(N > 0 as ::core::ffi::c_int) {
                    break;
                }
            }
        }
    }
}
unsafe extern "C" fn releaseMemArray(mut p: *mut Mem, mut N: ::core::ffi::c_int) {
    unsafe {
        if !p.is_null() && N != 0 {
            let mut pEnd: *mut Mem = p.offset(N as isize) as *mut Mem;
            let mut db: *mut sqlite3 = (*p).db;
            if !(*db).pnBytesFreed.is_null() {
                loop {
                    if (*p).szMalloc != 0 {
                        sqlite3DbFree(db, (*p).zMalloc as *mut ::core::ffi::c_void);
                    }
                    p = p.offset(1);
                    if !(p < pEnd) {
                        break;
                    }
                }
                return;
            }
            loop {
                if (*p).flags as ::core::ffi::c_int & (MEM_Agg | MEM_Dyn) != 0 {
                    sqlite3VdbeMemRelease(p);
                    (*p).flags = MEM_Undefined as u16_0;
                } else if (*p).szMalloc != 0 {
                    sqlite3DbNNFreeNN(db, (*p).zMalloc as *mut ::core::ffi::c_void);
                    (*p).szMalloc = 0 as ::core::ffi::c_int;
                    (*p).flags = MEM_Undefined as u16_0;
                }
                p = p.offset(1);
                if !(p < pEnd) {
                    break;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeFrameMemDel(mut pArg: *mut ::core::ffi::c_void) {
    unsafe {
        let mut pFrame: *mut VdbeFrame = pArg as *mut VdbeFrame;
        (*pFrame).pParent = (*(*pFrame).v).pDelFrame;
        (*(*pFrame).v).pDelFrame = pFrame;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeNextOpcode(
    mut p: *mut Vdbe,
    mut pSub: *mut Mem,
    mut eMode: ::core::ffi::c_int,
    mut piPc: *mut ::core::ffi::c_int,
    mut piAddr: *mut ::core::ffi::c_int,
    mut paOp: *mut *mut Op,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nRow: ::core::ffi::c_int = 0;
        let mut nSub: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut apSub: *mut *mut SubProgram = ::core::ptr::null_mut::<*mut SubProgram>();
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut aOp: *mut Op = ::core::ptr::null_mut::<Op>();
        let mut iPc: ::core::ffi::c_int = 0;
        nRow = (*p).nOp;
        if !pSub.is_null() {
            if (*pSub).flags as ::core::ffi::c_int & MEM_Blob != 0 {
                nSub = ((*pSub).n as usize)
                    .wrapping_div(::core::mem::size_of::<*mut Vdbe>() as usize)
                    as ::core::ffi::c_int;
                apSub = (*pSub).z as *mut *mut SubProgram;
            }
            i = 0 as ::core::ffi::c_int;
            while i < nSub {
                nRow += (**apSub.offset(i as isize)).nOp;
                i += 1;
            }
        }
        iPc = *piPc;
        loop {
            let c2rust_fresh1 = iPc;
            iPc = iPc + 1;
            i = c2rust_fresh1;
            if i >= nRow {
                (*p).rc = SQLITE_OK;
                rc = SQLITE_DONE;
                break;
            } else {
                if i < (*p).nOp {
                    aOp = (*p).aOp;
                } else {
                    let mut j: ::core::ffi::c_int = 0;
                    i -= (*p).nOp;
                    j = 0 as ::core::ffi::c_int;
                    while i >= (**apSub.offset(j as isize)).nOp {
                        i -= (**apSub.offset(j as isize)).nOp;
                        j += 1;
                    }
                    aOp = (**apSub.offset(j as isize)).aOp as *mut Op;
                }
                if !pSub.is_null()
                    && (*aOp.offset(i as isize)).p4type as ::core::ffi::c_int
                        == P4_SUBPROGRAM
                {
                    let mut nByte: ::core::ffi::c_int = ((nSub + 1 as ::core::ffi::c_int)
                        as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut SubProgram>() as usize)
                        as ::core::ffi::c_int;
                    let mut j_0: ::core::ffi::c_int = 0;
                    j_0 = 0 as ::core::ffi::c_int;
                    while j_0 < nSub {
                        if *apSub.offset(j_0 as isize)
                            == (*aOp.offset(i as isize)).p4.pProgram
                        {
                            break;
                        }
                        j_0 += 1;
                    }
                    if j_0 == nSub {
                        (*p).rc = sqlite3VdbeMemGrow(
                            pSub,
                            nByte,
                            (nSub != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                        );
                        if (*p).rc != SQLITE_OK {
                            rc = SQLITE_ERROR;
                            break;
                        } else {
                            apSub = (*pSub).z as *mut *mut SubProgram;
                            let c2rust_fresh2 = nSub;
                            nSub = nSub + 1;
                            let ref mut c2rust_fresh3 = *apSub
                                .offset(c2rust_fresh2 as isize);
                            *c2rust_fresh3 = (*aOp.offset(i as isize)).p4.pProgram;
                            (*pSub).flags = ((*pSub).flags as ::core::ffi::c_int
                                & !(MEM_TypeMask | MEM_Zero) | 0x10 as ::core::ffi::c_int)
                                as u16_0;
                            (*pSub).n = (nSub as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut SubProgram>() as usize,
                                ) as ::core::ffi::c_int;
                            nRow += (*(*aOp.offset(i as isize)).p4.pProgram).nOp;
                        }
                    }
                }
                if eMode == 0 as ::core::ffi::c_int {
                    break;
                }
                if eMode == 2 as ::core::ffi::c_int {
                    let mut pOp: *mut Op = aOp.offset(i as isize);
                    if (*pOp).opcode as ::core::ffi::c_int == OP_OpenRead {
                        break;
                    }
                    if (*pOp).opcode as ::core::ffi::c_int == OP_OpenWrite
                        && (*pOp).p5 as ::core::ffi::c_int & OPFLAG_P2ISREG
                            == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                    if (*pOp).opcode as ::core::ffi::c_int == OP_ReopenIdx {
                        break;
                    }
                } else {
                    if (*aOp.offset(i as isize)).opcode as ::core::ffi::c_int
                        == OP_Explain
                    {
                        break;
                    }
                    if (*aOp.offset(i as isize)).opcode as ::core::ffi::c_int == OP_Init
                        && iPc > 1 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
            }
        }
        *piPc = iPc;
        *piAddr = i;
        *paOp = aOp;
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeFrameDelete(mut p: *mut VdbeFrame) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut aMem: *mut Mem = (p as *mut u8_0)
            .offset(
                ((::core::mem::size_of::<VdbeFrame>() as usize).wrapping_add(7 as usize)
                    & !(7 as ::core::ffi::c_int) as usize) as isize,
            ) as *mut u8_0 as *mut Mem;
        let mut apCsr: *mut *mut VdbeCursor = aMem.offset((*p).nChildMem as isize)
            as *mut Mem as *mut *mut VdbeCursor;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nChildCsr {
            if !(*apCsr.offset(i as isize)).is_null() {
                sqlite3VdbeFreeCursorNN((*p).v, *apCsr.offset(i as isize));
            }
            i += 1;
        }
        releaseMemArray(aMem, (*p).nChildMem);
        sqlite3VdbeDeleteAuxData(
            (*(*p).v).db,
            &raw mut (*p).pAuxData,
            -1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        sqlite3DbFree((*(*p).v).db, p as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeList(mut p: *mut Vdbe) -> ::core::ffi::c_int {
    unsafe {
        let mut pSub: *mut Mem = ::core::ptr::null_mut::<Mem>();
        let mut db: *mut sqlite3 = (*p).db;
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pMem: *mut Mem = (*p).aMem.offset(1 as ::core::ffi::c_int as isize)
            as *mut Mem;
        let mut bListSubprogs: ::core::ffi::c_int = ((*p).explain() as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
            || (*db).flags as ::core::ffi::c_ulonglong
                & SQLITE_TriggerEQP as ::core::ffi::c_ulonglong
                != 0 as ::core::ffi::c_ulonglong) as ::core::ffi::c_int;
        let mut aOp: *mut Op = ::core::ptr::null_mut::<Op>();
        let mut pOp: *mut Op = ::core::ptr::null_mut::<Op>();
        releaseMemArray(pMem, 8 as ::core::ffi::c_int);
        if (*p).rc == SQLITE_NOMEM {
            sqlite3OomFault(db);
            return SQLITE_ERROR;
        }
        if bListSubprogs != 0 {
            pSub = (*p).aMem.offset(9 as ::core::ffi::c_int as isize) as *mut Mem;
        } else {
            pSub = ::core::ptr::null_mut::<Mem>();
        }
        rc = sqlite3VdbeNextOpcode(
            p,
            pSub,
            ((*p).explain() as ::core::ffi::c_int == 2 as ::core::ffi::c_int)
                as ::core::ffi::c_int,
            &raw mut (*p).pc,
            &raw mut i,
            &raw mut aOp,
        );
        if rc == SQLITE_OK {
            pOp = aOp.offset(i as isize);
            if (*db).u1.isInterrupted
                != 0
            {
                (*p).rc = SQLITE_INTERRUPT;
                rc = SQLITE_ERROR;
                sqlite3VdbeError(p, sqlite3ErrStr((*p).rc));
            } else {
                let mut zP4: *mut ::core::ffi::c_char = sqlite3VdbeDisplayP4(db, pOp);
                if (*p).explain() as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                    sqlite3VdbeMemSetInt64(pMem, (*pOp).p1 as i64_0);
                    sqlite3VdbeMemSetInt64(
                        pMem.offset(1 as ::core::ffi::c_int as isize),
                        (*pOp).p2 as i64_0,
                    );
                    sqlite3VdbeMemSetInt64(
                        pMem.offset(2 as ::core::ffi::c_int as isize),
                        (*pOp).p3 as i64_0,
                    );
                    sqlite3VdbeMemSetStr(
                        pMem.offset(3 as ::core::ffi::c_int as isize),
                        zP4,
                        -1 as ::core::ffi::c_int as i64_0,
                        SQLITE_UTF8 as u8_0,
                        Some(
                            sqlite3_free
                                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                        ),
                    );
                } else {
                    sqlite3VdbeMemSetInt64(
                        pMem.offset(0 as ::core::ffi::c_int as isize),
                        i as i64_0,
                    );
                    sqlite3VdbeMemSetStr(
                        pMem.offset(1 as ::core::ffi::c_int as isize),
                        sqlite3OpcodeName((*pOp).opcode as ::core::ffi::c_int)
                            as *mut ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int as i64_0,
                        SQLITE_UTF8 as u8_0,
                        SQLITE_STATIC,
                    );
                    sqlite3VdbeMemSetInt64(
                        pMem.offset(2 as ::core::ffi::c_int as isize),
                        (*pOp).p1 as i64_0,
                    );
                    sqlite3VdbeMemSetInt64(
                        pMem.offset(3 as ::core::ffi::c_int as isize),
                        (*pOp).p2 as i64_0,
                    );
                    sqlite3VdbeMemSetInt64(
                        pMem.offset(4 as ::core::ffi::c_int as isize),
                        (*pOp).p3 as i64_0,
                    );
                    sqlite3VdbeMemSetInt64(
                        pMem.offset(6 as ::core::ffi::c_int as isize),
                        (*pOp).p5 as i64_0,
                    );
                    sqlite3VdbeMemSetNull(pMem.offset(7 as ::core::ffi::c_int as isize));
                    sqlite3VdbeMemSetStr(
                        pMem.offset(5 as ::core::ffi::c_int as isize),
                        zP4,
                        -1 as ::core::ffi::c_int as i64_0,
                        SQLITE_UTF8 as u8_0,
                        Some(
                            sqlite3_free
                                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                        ),
                    );
                }
                (*p).pResultRow = pMem;
                if (*db).mallocFailed != 0 {
                    (*p).rc = SQLITE_NOMEM;
                    rc = SQLITE_ERROR;
                } else {
                    (*p).rc = SQLITE_OK;
                    rc = SQLITE_ROW;
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn allocSpace(
    mut p: *mut ReusableSpace,
    mut pBuf: *mut ::core::ffi::c_void,
    mut nByte: sqlite3_int64,
) -> *mut ::core::ffi::c_void {
    unsafe {
        if pBuf.is_null() {
            nByte = nByte;
            if nByte <= (*p).nFree {
                (*p).nFree -= nByte as ::core::ffi::c_longlong;
                pBuf = (*p).pSpace.offset((*p).nFree as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void;
            } else {
                (*p).nNeeded += nByte as ::core::ffi::c_longlong;
            }
        }
        return pBuf;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeRewind(mut p: *mut Vdbe) {
    unsafe {
        (*p).eVdbeState = VDBE_READY_STATE as u8_0;
        (*p).pc = -1 as ::core::ffi::c_int;
        (*p).rc = SQLITE_OK;
        (*p).errorAction = OE_Abort as u8_0;
        (*p).nChange = 0 as i64_0;
        (*p).cacheCtr = 1 as u32_0;
        (*p).minWriteFileFormat = 255 as u8_0;
        (*p).iStatement = 0 as ::core::ffi::c_int;
        (*p).nFkConstraint = 0 as i64_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeMakeReady(mut p: *mut Vdbe, mut pParse: *mut Parse) {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut nVar: ::core::ffi::c_int = 0;
        let mut nMem: ::core::ffi::c_int = 0;
        let mut nCursor: ::core::ffi::c_int = 0;
        let mut nArg: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut x: ReusableSpace = ReusableSpace {
            pSpace: ::core::ptr::null_mut::<u8_0>(),
            nFree: 0,
            nNeeded: 0

        };
        (*p).pVList = (*pParse).pVList;
        (*pParse).pVList = ::core::ptr::null_mut::<VList>();
        db = (*p).db;
        nVar = (*pParse).nVar as ::core::ffi::c_int;
        nMem = (*pParse).nMem;
        nCursor = (*pParse).nTab;
        nArg = (*pParse).nMaxArg;
        nMem += nCursor;
        if nCursor == 0 as ::core::ffi::c_int && nMem > 0 as ::core::ffi::c_int {
            nMem += 1;
        }
        n = (::core::mem::size_of::<Op>() as usize).wrapping_mul((*p).nOp as usize)
            as ::core::ffi::c_int;
        x.pSpace = ((*p).aOp as *mut u8_0).offset(n as isize) as *mut u8_0;
        x.nFree = ((*pParse).szOpAlloc - n & !(7 as ::core::ffi::c_int))
            as sqlite3_int64;
        resolveP2Values(p, &raw mut nArg);
        (*p)
            .set_usesStmtJournal(
                ((*pParse).isMultiWrite as ::core::ffi::c_int != 0
                    && (*pParse).mayAbort as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int as u8_0 as bft as bft,
            );
        if (*pParse).explain != 0 {
            if nMem < 10 as ::core::ffi::c_int {
                nMem = 10 as ::core::ffi::c_int;
            }
            (*p).set_explain((*pParse).explain as bft as bft);
            (*p).nResColumn = (12 as ::core::ffi::c_int
                - 4 as ::core::ffi::c_int * (*p).explain() as ::core::ffi::c_int)
                as u16_0;
        }
        (*p).set_expired(0 as bft as bft);
        x.nNeeded = 0 as sqlite3_int64;
        (*p).aMem = allocSpace(
            &raw mut x,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            (nMem as usize).wrapping_mul(::core::mem::size_of::<Mem>() as usize)
                as sqlite3_int64,
        ) as *mut Mem;
        (*p).aVar = allocSpace(
            &raw mut x,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            (nVar as usize).wrapping_mul(::core::mem::size_of::<Mem>() as usize)
                as sqlite3_int64,
        ) as *mut Mem;
        (*p).apArg = allocSpace(
            &raw mut x,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            (nArg as usize).wrapping_mul(::core::mem::size_of::<*mut Mem>() as usize)
                as sqlite3_int64,
        ) as *mut *mut Mem;
        (*p).apCsr = allocSpace(
            &raw mut x,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            (nCursor as usize)
                .wrapping_mul(::core::mem::size_of::<*mut VdbeCursor>() as usize)
                as sqlite3_int64,
        ) as *mut *mut VdbeCursor;
        if x.nNeeded != 0 {
            (*p).pFree = sqlite3DbMallocRawNN(db, x.nNeeded as u64_0);
            x.pSpace = (*p).pFree as *mut u8_0;
            x.nFree = x.nNeeded;
            if (*db).mallocFailed == 0 {
                (*p).aMem = allocSpace(
                    &raw mut x,
                    (*p).aMem as *mut ::core::ffi::c_void,
                    (nMem as usize).wrapping_mul(::core::mem::size_of::<Mem>() as usize)
                        as sqlite3_int64,
                ) as *mut Mem;
                (*p).aVar = allocSpace(
                    &raw mut x,
                    (*p).aVar as *mut ::core::ffi::c_void,
                    (nVar as usize).wrapping_mul(::core::mem::size_of::<Mem>() as usize)
                        as sqlite3_int64,
                ) as *mut Mem;
                (*p).apArg = allocSpace(
                    &raw mut x,
                    (*p).apArg as *mut ::core::ffi::c_void,
                    (nArg as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut Mem>() as usize)
                        as sqlite3_int64,
                ) as *mut *mut Mem;
                (*p).apCsr = allocSpace(
                    &raw mut x,
                    (*p).apCsr as *mut ::core::ffi::c_void,
                    (nCursor as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut VdbeCursor>() as usize)
                        as sqlite3_int64,
                ) as *mut *mut VdbeCursor;
            }
        }
        if (*db).mallocFailed != 0 {
            (*p).nVar = 0 as ynVar;
            (*p).nCursor = 0 as ::core::ffi::c_int;
            (*p).nMem = 0 as ::core::ffi::c_int;
        } else {
            (*p).nCursor = nCursor;
            (*p).nVar = nVar as ynVar;
            initMemArray((*p).aVar, nVar, db, MEM_Null as u16_0);
            (*p).nMem = nMem;
            initMemArray((*p).aMem, nMem, db, MEM_Undefined as u16_0);
            memset(
                (*p).apCsr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (nCursor as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut VdbeCursor>() as size_t),
            );
        }
        sqlite3VdbeRewind(p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeFreeCursor(
    mut p: *mut Vdbe,
    mut pCx: *mut VdbeCursor,
) {
    unsafe {
        if !pCx.is_null() {
            sqlite3VdbeFreeCursorNN(p, pCx);
        }
    }
}
#[inline(never)]
unsafe extern "C" fn freeCursorWithCache(mut p: *mut Vdbe, mut pCx: *mut VdbeCursor) {
    unsafe {
        let mut pCache: *mut VdbeTxtBlbCache = (*pCx).pCache;
        (*pCx).set_colCache(0 as Bool as Bool);
        (*pCx).pCache = ::core::ptr::null_mut::<VdbeTxtBlbCache>();
        if !(*pCache).pCValue.is_null() {
            sqlite3RCStrUnref((*pCache).pCValue as *mut ::core::ffi::c_void);
            (*pCache).pCValue = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        sqlite3DbFree((*p).db, pCache as *mut ::core::ffi::c_void);
        sqlite3VdbeFreeCursorNN(p, pCx);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeFreeCursorNN(
    mut p: *mut Vdbe,
    mut pCx: *mut VdbeCursor,
) {
    unsafe {
        if (*pCx).colCache() != 0 {
            freeCursorWithCache(p, pCx);
            return;
        }
        match (*pCx).eCurType as ::core::ffi::c_int {
            CURTYPE_SORTER => {
                sqlite3VdbeSorterClose((*p).db, pCx);
            }
            CURTYPE_BTREE => {
                sqlite3BtreeCloseCursor((*pCx).uc.pCursor);
            }
            CURTYPE_VTAB => {
                let mut pVCur: *mut sqlite3_vtab_cursor = (*pCx).uc.pVCur;
                let mut pModule: *const sqlite3_module = (*(*pVCur).pVtab).pModule;
                (*(*pVCur).pVtab).nRef -= 1;
                (*pModule).xClose.expect("non-null function pointer")(pVCur);
            }
            _ => {}
        };
    }
}
unsafe extern "C" fn closeCursorsInFrame(mut p: *mut Vdbe) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nCursor {
            let mut pC: *mut VdbeCursor = *(*p).apCsr.offset(i as isize);
            if !pC.is_null() {
                sqlite3VdbeFreeCursorNN(p, pC);
                let ref mut c2rust_fresh0 = *(*p).apCsr.offset(i as isize);
                *c2rust_fresh0 = ::core::ptr::null_mut::<VdbeCursor>();
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeFrameRestore(
    mut pFrame: *mut VdbeFrame,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: *mut Vdbe = (*pFrame).v;
        closeCursorsInFrame(v);
        (*v).aOp = (*pFrame).aOp;
        (*v).nOp = (*pFrame).nOp;
        (*v).aMem = (*pFrame).aMem;
        (*v).nMem = (*pFrame).nMem;
        (*v).apCsr = (*pFrame).apCsr;
        (*v).nCursor = (*pFrame).nCursor;
        (*(*v).db).lastRowid = (*pFrame).lastRowid;
        (*v).nChange = (*pFrame).nChange;
        (*(*v).db).nChange = (*pFrame).nDbChange;
        sqlite3VdbeDeleteAuxData(
            (*v).db,
            &raw mut (*v).pAuxData,
            -1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        (*v).pAuxData = (*pFrame).pAuxData;
        (*pFrame).pAuxData = ::core::ptr::null_mut::<AuxData>();
        return (*pFrame).pc;
    }
}
unsafe extern "C" fn closeAllCursors(mut p: *mut Vdbe) {
    unsafe {
        if !(*p).pFrame.is_null() {
            let mut pFrame: *mut VdbeFrame = ::core::ptr::null_mut::<VdbeFrame>();
            pFrame = (*p).pFrame;
            while !(*pFrame).pParent.is_null() {
                pFrame = (*pFrame).pParent;
            }
            sqlite3VdbeFrameRestore(pFrame);
            (*p).pFrame = ::core::ptr::null_mut::<VdbeFrame>();
            (*p).nFrame = 0 as ::core::ffi::c_int;
        }
        closeCursorsInFrame(p);
        releaseMemArray((*p).aMem, (*p).nMem);
        while !(*p).pDelFrame.is_null() {
            let mut pDel: *mut VdbeFrame = (*p).pDelFrame;
            (*p).pDelFrame = (*pDel).pParent;
            sqlite3VdbeFrameDelete(pDel);
        }
        if !(*p).pAuxData.is_null() {
            sqlite3VdbeDeleteAuxData(
                (*p).db,
                &raw mut (*p).pAuxData,
                -1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSetNumCols(
    mut p: *mut Vdbe,
    mut nResColumn: ::core::ffi::c_int,
) {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*p).db;
        if (*p).nResAlloc != 0 {
            releaseMemArray(
                (*p).aColName,
                (*p).nResAlloc as ::core::ffi::c_int * COLNAME_N,
            );
            sqlite3DbFree(db, (*p).aColName as *mut ::core::ffi::c_void);
        }
        n = nResColumn * COLNAME_N;
        (*p).nResAlloc = nResColumn as u16_0;
        (*p).nResColumn = (*p).nResAlloc;
        (*p).aColName = sqlite3DbMallocRawNN(
            db,
            (::core::mem::size_of::<Mem>() as usize).wrapping_mul(n as usize) as u64_0,
        ) as *mut Mem;
        if (*p).aColName.is_null() {
            return;
        }
        initMemArray((*p).aColName, n, db, MEM_Null as u16_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSetColName(
    mut p: *mut Vdbe,
    mut idx: ::core::ffi::c_int,
    mut var: ::core::ffi::c_int,
    mut zName: *const ::core::ffi::c_char,
    mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pColName: *mut Mem = ::core::ptr::null_mut::<Mem>();
        if (*(*p).db).mallocFailed != 0 {
            return SQLITE_NOMEM_BKPT;
        }
        pColName = (*p)
            .aColName
            .offset((idx + var * (*p).nResAlloc as ::core::ffi::c_int) as isize)
            as *mut Mem;
        rc = sqlite3VdbeMemSetStr(
            pColName,
            zName,
            -1 as ::core::ffi::c_int as i64_0,
            SQLITE_UTF8 as u8_0,
            xDel,
        );
        return rc;
    }
}
unsafe extern "C" fn vdbeCommit(
    mut db: *mut sqlite3,
    mut p: *mut Vdbe,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut nTrans: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut needXcommit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = sqlite3VtabSync(db, p);
        i = 0 as ::core::ffi::c_int;
        while rc == SQLITE_OK && i < (*db).nDb {
            let mut pBt: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
            if sqlite3BtreeTxnState(pBt) == SQLITE_TXN_WRITE {
                static mut aMJNeeded: [u8_0; 6] = [
                    1 as ::core::ffi::c_int as u8_0,
                    1 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    1 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                ];
                let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
                needXcommit = 1 as ::core::ffi::c_int;
                sqlite3BtreeEnter(pBt);
                pPager = sqlite3BtreePager(pBt) as *mut Pager;
                if (*(*db).aDb.offset(i as isize)).safety_level as ::core::ffi::c_int
                    != PAGER_SYNCHRONOUS_OFF
                    && aMJNeeded[sqlite3PagerGetJournalMode(pPager) as usize]
                        as ::core::ffi::c_int != 0
                    && sqlite3PagerIsMemdb(pPager) == 0 as ::core::ffi::c_int
                {
                    nTrans += 1;
                }
                rc = sqlite3PagerExclusiveLock(pPager);
                sqlite3BtreeLeave(pBt);
            }
            i += 1;
        }
        if rc != SQLITE_OK {
            return rc;
        }
        if needXcommit != 0 && (*db).xCommitCallback.is_some() {
            rc = (*db)
                .xCommitCallback
                .expect("non-null function pointer")((*db).pCommitArg);
            if rc != 0 {
                return SQLITE_CONSTRAINT_COMMITHOOK;
            }
        }
        if 0 as ::core::ffi::c_int
            == sqlite3Strlen30(
                sqlite3BtreeGetFilename(
                    (*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).pBt,
                ),
            ) || nTrans <= 1 as ::core::ffi::c_int
        {
            if needXcommit != 0 {
                i = 0 as ::core::ffi::c_int;
                while rc == SQLITE_OK && i < (*db).nDb {
                    let mut pBt_0: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
                    if sqlite3BtreeTxnState(pBt_0) >= SQLITE_TXN_WRITE {
                        rc = sqlite3BtreeCommitPhaseOne(
                            pBt_0,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                    i += 1;
                }
            }
            i = 0 as ::core::ffi::c_int;
            while rc == SQLITE_OK && i < (*db).nDb {
                let mut pBt_1: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
                let mut txn: ::core::ffi::c_int = sqlite3BtreeTxnState(pBt_1);
                if txn != SQLITE_TXN_NONE {
                    rc = sqlite3BtreeCommitPhaseTwo(pBt_1, 0 as ::core::ffi::c_int);
                }
                i += 1;
            }
            if rc == SQLITE_OK {
                sqlite3VtabCommit(db);
            }
        } else {
            let mut pVfs: *mut sqlite3_vfs = (*db).pVfs;
            let mut zSuper: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut zMainFile: *const ::core::ffi::c_char = sqlite3BtreeGetFilename(
                (*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).pBt,
            );
            let mut pSuperJrnl: *mut sqlite3_file = ::core::ptr::null_mut::<
                sqlite3_file,
            >();
            let mut offset: i64_0 = 0 as i64_0;
            let mut res: ::core::ffi::c_int = 0;
            let mut retryCount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut nMainFile: ::core::ffi::c_int = 0;
            nMainFile = sqlite3Strlen30(zMainFile);
            zSuper = sqlite3MPrintf(
                db,
                b"%.4c%s%.16c\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                zMainFile,
                0 as ::core::ffi::c_int,
            );
            if zSuper.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            zSuper = zSuper.offset(4 as ::core::ffi::c_int as isize);
            loop {
                let mut iRandom: u32_0 = 0;
                if retryCount != 0 {
                    if retryCount > 100 as ::core::ffi::c_int {
                        sqlite3_log(
                            SQLITE_FULL,
                            b"MJ delete: %s\0".as_ptr() as *const ::core::ffi::c_char,
                            zSuper,
                        );
                        sqlite3OsDelete(pVfs, zSuper, 0 as ::core::ffi::c_int);
                        break;
                    } else if retryCount == 1 as ::core::ffi::c_int {
                        sqlite3_log(
                            SQLITE_FULL,
                            b"MJ collide: %s\0".as_ptr() as *const ::core::ffi::c_char,
                            zSuper,
                        );
                    }
                }
                retryCount += 1;
                sqlite3_randomness(
                    ::core::mem::size_of::<u32_0>() as ::core::ffi::c_int,
                    &raw mut iRandom as *mut ::core::ffi::c_void,
                );
                sqlite3_snprintf(
                    13 as ::core::ffi::c_int,
                    zSuper.offset(nMainFile as isize) as *mut ::core::ffi::c_char,
                    b"-mj%06X9%02X\0".as_ptr() as *const ::core::ffi::c_char,
                    iRandom >> 8 as ::core::ffi::c_int & 0xffffff as u32_0,
                    iRandom & 0xff as u32_0,
                );
                rc = sqlite3OsAccess(pVfs, zSuper, SQLITE_ACCESS_EXISTS, &raw mut res);
                if !(rc == SQLITE_OK && res != 0) {
                    break;
                }
            }
            if rc == SQLITE_OK {
                rc = sqlite3OsOpenMalloc(
                    pVfs,
                    zSuper,
                    &raw mut pSuperJrnl,
                    SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_EXCLUSIVE
                        | SQLITE_OPEN_SUPER_JOURNAL,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
            }
            if rc != SQLITE_OK {
                sqlite3DbFree(
                    db,
                    zSuper.offset(-(4 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void,
                );
                return rc;
            }
            i = 0 as ::core::ffi::c_int;
            while i < (*db).nDb {
                let mut pBt_2: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
                if sqlite3BtreeTxnState(pBt_2) == SQLITE_TXN_WRITE {
                    let mut zFile: *const ::core::ffi::c_char = sqlite3BtreeGetJournalname(
                        pBt_2,
                    );
                    if !zFile.is_null() {
                        rc = sqlite3OsWrite(
                            pSuperJrnl,
                            zFile as *const ::core::ffi::c_void,
                            sqlite3Strlen30(zFile) + 1 as ::core::ffi::c_int,
                            offset,
                        );
                        offset
                            += (sqlite3Strlen30(zFile) + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_longlong;
                        if rc != SQLITE_OK {
                            sqlite3OsCloseFree(pSuperJrnl);
                            sqlite3OsDelete(pVfs, zSuper, 0 as ::core::ffi::c_int);
                            sqlite3DbFree(
                                db,
                                zSuper.offset(-(4 as ::core::ffi::c_int as isize))
                                    as *mut ::core::ffi::c_void,
                            );
                            return rc;
                        }
                    }
                }
                i += 1;
            }
            if 0 as ::core::ffi::c_int
                == sqlite3OsDeviceCharacteristics(pSuperJrnl) & SQLITE_IOCAP_SEQUENTIAL
                && {
                    rc = sqlite3OsSync(pSuperJrnl, SQLITE_SYNC_NORMAL);
                    SQLITE_OK != rc
                }
            {
                sqlite3OsCloseFree(pSuperJrnl);
                sqlite3OsDelete(pVfs, zSuper, 0 as ::core::ffi::c_int);
                sqlite3DbFree(
                    db,
                    zSuper.offset(-(4 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void,
                );
                return rc;
            }
            i = 0 as ::core::ffi::c_int;
            while rc == SQLITE_OK && i < (*db).nDb {
                let mut pBt_3: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
                if !pBt_3.is_null() {
                    rc = sqlite3BtreeCommitPhaseOne(pBt_3, zSuper);
                }
                i += 1;
            }
            sqlite3OsCloseFree(pSuperJrnl);
            if rc != SQLITE_OK {
                sqlite3DbFree(
                    db,
                    zSuper.offset(-(4 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void,
                );
                return rc;
            }
            rc = sqlite3OsDelete(pVfs, zSuper, 1 as ::core::ffi::c_int);
            sqlite3DbFree(
                db,
                zSuper.offset(-(4 as ::core::ffi::c_int as isize))
                    as *mut ::core::ffi::c_void,
            );
            zSuper = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if rc != 0 {
                return rc;
            }
            disable_simulated_io_errors();
            sqlite3BeginBenignMalloc();
            i = 0 as ::core::ffi::c_int;
            while i < (*db).nDb {
                let mut pBt_4: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
                if !pBt_4.is_null() {
                    sqlite3BtreeCommitPhaseTwo(pBt_4, 1 as ::core::ffi::c_int);
                }
                i += 1;
            }
            sqlite3EndBenignMalloc();
            enable_simulated_io_errors();
            sqlite3VtabCommit(db);
        }
        return rc;
    }
}
#[inline(never)]
unsafe extern "C" fn vdbeCloseStatement(
    mut p: *mut Vdbe,
    mut eOp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let db: *mut sqlite3 = (*p).db;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut i: ::core::ffi::c_int = 0;
        let iSavepoint: ::core::ffi::c_int = (*p).iStatement - 1 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*db).nDb {
            let mut rc2: ::core::ffi::c_int = SQLITE_OK;
            let mut pBt: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
            if !pBt.is_null() {
                if eOp == SAVEPOINT_ROLLBACK {
                    rc2 = sqlite3BtreeSavepoint(pBt, SAVEPOINT_ROLLBACK, iSavepoint);
                }
                if rc2 == SQLITE_OK {
                    rc2 = sqlite3BtreeSavepoint(pBt, SAVEPOINT_RELEASE, iSavepoint);
                }
                if rc == SQLITE_OK {
                    rc = rc2;
                }
            }
            i += 1;
        }
        (*db).nStatement -= 1;
        (*p).iStatement = 0 as ::core::ffi::c_int;
        if rc == SQLITE_OK {
            if eOp == SAVEPOINT_ROLLBACK {
                rc = sqlite3VtabSavepoint(db, SAVEPOINT_ROLLBACK, iSavepoint);
            }
            if rc == SQLITE_OK {
                rc = sqlite3VtabSavepoint(db, SAVEPOINT_RELEASE, iSavepoint);
            }
        }
        if eOp == SAVEPOINT_ROLLBACK {
            (*db).nDeferredCons = (*p).nStmtDefCons;
            (*db).nDeferredImmCons = (*p).nStmtDefImmCons;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeCloseStatement(
    mut p: *mut Vdbe,
    mut eOp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if (*(*p).db).nStatement != 0 && (*p).iStatement != 0 {
            return vdbeCloseStatement(p, eOp);
        }
        return SQLITE_OK;
    }
}
#[inline(never)]
unsafe extern "C" fn vdbeFkError(mut p: *mut Vdbe) -> ::core::ffi::c_int {
    unsafe {
        (*p).rc = SQLITE_CONSTRAINT_FOREIGNKEY;
        (*p).errorAction = OE_Abort as u8_0;
        sqlite3VdbeError(
            p,
            b"FOREIGN KEY constraint failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if (*p).prepFlags as ::core::ffi::c_int & SQLITE_PREPARE_SAVESQL
            == 0 as ::core::ffi::c_int
        {
            return SQLITE_ERROR;
        }
        return SQLITE_CONSTRAINT_FOREIGNKEY;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeCheckFkImmediate(
    mut p: *mut Vdbe,
) -> ::core::ffi::c_int {
    unsafe {
        if (*p).nFkConstraint == 0 as ::core::ffi::c_longlong {
            return SQLITE_OK;
        }
        return vdbeFkError(p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeCheckFkDeferred(
    mut p: *mut Vdbe,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = (*p).db;
        if (*db).nDeferredCons + (*db).nDeferredImmCons == 0 as ::core::ffi::c_longlong {
            return SQLITE_OK;
        }
        return vdbeFkError(p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeHalt(mut p: *mut Vdbe) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*p).db;
        if (*db).mallocFailed != 0 {
            (*p).rc = SQLITE_NOMEM_BKPT;
        }
        closeAllCursors(p);
        if (*p).bIsReader() != 0 {
            let mut mrc: ::core::ffi::c_int = 0;
            let mut eStatementOp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut isSpecialError: ::core::ffi::c_int = 0;
            sqlite3VdbeEnter(p);
            if (*p).rc != 0 {
                mrc = (*p).rc & 0xff as ::core::ffi::c_int;
                isSpecialError = (mrc == SQLITE_NOMEM || mrc == SQLITE_IOERR
                    || mrc == SQLITE_INTERRUPT || mrc == SQLITE_FULL)
                    as ::core::ffi::c_int;
            } else {
                isSpecialError = 0 as ::core::ffi::c_int;
                mrc = isSpecialError;
            }
            if isSpecialError != 0 {
                if (*p).readOnly() == 0 || mrc != SQLITE_INTERRUPT {
                    if (mrc == SQLITE_NOMEM || mrc == SQLITE_FULL)
                        && (*p).usesStmtJournal() as ::core::ffi::c_int != 0
                    {
                        eStatementOp = SAVEPOINT_ROLLBACK;
                    } else {
                        sqlite3RollbackAll(db, SQLITE_ABORT_ROLLBACK);
                        sqlite3CloseSavepoints(db);
                        (*db).autoCommit = 1 as u8_0;
                        (*p).nChange = 0 as i64_0;
                    }
                }
            }
            if (*p).rc == SQLITE_OK
                || (*p).errorAction as ::core::ffi::c_int == OE_Fail
                    && isSpecialError == 0
            {
                sqlite3VdbeCheckFkImmediate(p);
            }
            if !((*db).nVTrans > 0 as ::core::ffi::c_int && (*db).aVTrans.is_null())
                && (*db).autoCommit as ::core::ffi::c_int != 0
                && (*db).nVdbeWrite
                    == ((*p).readOnly() as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
            {
                if (*p).rc == SQLITE_OK
                    || (*p).errorAction as ::core::ffi::c_int == OE_Fail
                        && isSpecialError == 0
                {
                    rc = sqlite3VdbeCheckFkDeferred(p);
                    if rc != SQLITE_OK {
                        if (*p).readOnly() != 0 {
                            sqlite3VdbeLeave(p);
                            return SQLITE_ERROR;
                        }
                        rc = SQLITE_CONSTRAINT_FOREIGNKEY;
                    } else if (*db).flags & SQLITE_CorruptRdOnly != 0 {
                        rc = SQLITE_CORRUPT;
                        (*db).flags &= !SQLITE_CorruptRdOnly as ::core::ffi::c_ulonglong;
                    } else {
                        rc = vdbeCommit(db, p);
                    }
                    if rc == SQLITE_BUSY && (*p).readOnly() as ::core::ffi::c_int != 0 {
                        sqlite3VdbeLeave(p);
                        return SQLITE_BUSY;
                    } else if rc != SQLITE_OK {
                        sqlite3SystemError(db, rc);
                        (*p).rc = rc;
                        sqlite3RollbackAll(db, SQLITE_OK);
                        (*p).nChange = 0 as i64_0;
                    } else {
                        (*db).nDeferredCons = 0 as i64_0;
                        (*db).nDeferredImmCons = 0 as i64_0;
                        (*db).flags
                            &= !(SQLITE_DeferFKs as u64_0) as ::core::ffi::c_ulonglong;
                        sqlite3CommitInternalChanges(db);
                    }
                } else if (*p).rc == SQLITE_SCHEMA
                    && (*db).nVdbeActive > 1 as ::core::ffi::c_int
                {
                    (*p).nChange = 0 as i64_0;
                } else {
                    sqlite3RollbackAll(db, SQLITE_OK);
                    (*p).nChange = 0 as i64_0;
                }
                (*db).nStatement = 0 as ::core::ffi::c_int;
            } else if eStatementOp == 0 as ::core::ffi::c_int {
                if (*p).rc == SQLITE_OK
                    || (*p).errorAction as ::core::ffi::c_int == OE_Fail
                {
                    eStatementOp = SAVEPOINT_RELEASE;
                } else if (*p).errorAction as ::core::ffi::c_int == OE_Abort {
                    eStatementOp = SAVEPOINT_ROLLBACK;
                } else {
                    sqlite3RollbackAll(db, SQLITE_ABORT_ROLLBACK);
                    sqlite3CloseSavepoints(db);
                    (*db).autoCommit = 1 as u8_0;
                    (*p).nChange = 0 as i64_0;
                }
            }
            if eStatementOp != 0 {
                rc = sqlite3VdbeCloseStatement(p, eStatementOp);
                if rc != 0 {
                    if (*p).rc == SQLITE_OK
                        || (*p).rc & 0xff as ::core::ffi::c_int == SQLITE_CONSTRAINT
                    {
                        (*p).rc = rc;
                        sqlite3DbFree(db, (*p).zErrMsg as *mut ::core::ffi::c_void);
                        (*p).zErrMsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    sqlite3RollbackAll(db, SQLITE_ABORT_ROLLBACK);
                    sqlite3CloseSavepoints(db);
                    (*db).autoCommit = 1 as u8_0;
                    (*p).nChange = 0 as i64_0;
                }
            }
            if (*p).changeCntOn() != 0 {
                if eStatementOp != SAVEPOINT_ROLLBACK {
                    sqlite3VdbeSetChanges(db, (*p).nChange);
                } else {
                    sqlite3VdbeSetChanges(db, 0 as i64_0);
                }
                (*p).nChange = 0 as i64_0;
            }
            sqlite3VdbeLeave(p);
        }
        (*db).nVdbeActive -= 1;
        if (*p).readOnly() == 0 {
            (*db).nVdbeWrite -= 1;
        }
        if (*p).bIsReader() != 0 {
            (*db).nVdbeRead -= 1;
        }
        (*p).eVdbeState = VDBE_HALT_STATE as u8_0;
        if (*db).mallocFailed != 0 {
            (*p).rc = SQLITE_NOMEM_BKPT;
        }
        (*db).autoCommit != 0;
        return if (*p).rc == SQLITE_BUSY { SQLITE_BUSY } else { SQLITE_OK };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeResetStepResult(mut p: *mut Vdbe) {
    unsafe {
        (*p).rc = SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeTransferError(
    mut p: *mut Vdbe,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = (*p).db;
        let mut rc: ::core::ffi::c_int = (*p).rc;
        if !(*p).zErrMsg.is_null() {
            (*db).bBenignMalloc = (*db).bBenignMalloc.wrapping_add(1);
            sqlite3BeginBenignMalloc();
            if (*db).pErr.is_null() {
                (*db).pErr = sqlite3ValueNew(db);
            }
            sqlite3ValueSetStr(
                (*db).pErr,
                -1 as ::core::ffi::c_int,
                (*p).zErrMsg as *const ::core::ffi::c_void,
                SQLITE_UTF8 as u8_0,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3EndBenignMalloc();
            (*db).bBenignMalloc = (*db).bBenignMalloc.wrapping_sub(1);
        } else if !(*db).pErr.is_null() {
            sqlite3ValueSetNull((*db).pErr);
        }
        (*db).errCode = rc;
        (*db).errByteOffset = -1 as ::core::ffi::c_int;
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeReset(mut p: *mut Vdbe) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        db = (*p).db;
        if (*p).eVdbeState as ::core::ffi::c_int == VDBE_RUN_STATE {
            sqlite3VdbeHalt(p);
        }
        if (*p).pc >= 0 as ::core::ffi::c_int {
            if !(*db).pErr.is_null() || !(*p).zErrMsg.is_null() {
                sqlite3VdbeTransferError(p);
            } else {
                (*db).errCode = (*p).rc;
            }
        }
        if !(*p).zErrMsg.is_null() {
            sqlite3DbFree(db, (*p).zErrMsg as *mut ::core::ffi::c_void);
            (*p).zErrMsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        (*p).pResultRow = ::core::ptr::null_mut::<Mem>();
        return (*p).rc & (*db).errMask;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeFinalize(mut p: *mut Vdbe) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*p).eVdbeState as ::core::ffi::c_int >= VDBE_READY_STATE {
            rc = sqlite3VdbeReset(p);
        }
        sqlite3VdbeDelete(p);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeDeleteAuxData(
    mut db: *mut sqlite3,
    mut pp: *mut *mut AuxData,
    mut iOp: ::core::ffi::c_int,
    mut mask: ::core::ffi::c_int,
) {
    unsafe {
        while !(*pp).is_null() {
            let mut pAux: *mut AuxData = *pp;
            if iOp < 0 as ::core::ffi::c_int
                || (*pAux).iAuxOp == iOp && (*pAux).iAuxArg >= 0 as ::core::ffi::c_int
                    && ((*pAux).iAuxArg > 31 as ::core::ffi::c_int
                        || mask as ::core::ffi::c_uint
                            & (1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                << (*pAux).iAuxArg == 0)
            {
                if (*pAux).xDeleteAux.is_some() {
                    (*pAux).xDeleteAux.expect("non-null function pointer")((*pAux).pAux);
                }
                *pp = (*pAux).pNextAux;
                sqlite3DbFree(db, pAux as *mut ::core::ffi::c_void);
            } else {
                pp = &raw mut (*pAux).pNextAux;
            }
        }
    }
}
unsafe extern "C" fn sqlite3VdbeClearObject(mut db: *mut sqlite3, mut p: *mut Vdbe) {
    unsafe {
        let mut pSub: *mut SubProgram = ::core::ptr::null_mut::<SubProgram>();
        let mut pNext: *mut SubProgram = ::core::ptr::null_mut::<SubProgram>();
        if !(*p).aColName.is_null() {
            releaseMemArray(
                (*p).aColName,
                (*p).nResAlloc as ::core::ffi::c_int * COLNAME_N,
            );
            sqlite3DbNNFreeNN(db, (*p).aColName as *mut ::core::ffi::c_void);
        }
        pSub = (*p).pProgram;
        while !pSub.is_null() {
            pNext = (*pSub).pNext;
            vdbeFreeOpArray(db, (*pSub).aOp as *mut Op, (*pSub).nOp);
            sqlite3DbFree(db, pSub as *mut ::core::ffi::c_void);
            pSub = pNext;
        }
        if (*p).eVdbeState as ::core::ffi::c_int != VDBE_INIT_STATE {
            releaseMemArray((*p).aVar, (*p).nVar as ::core::ffi::c_int);
            if !(*p).pVList.is_null() {
                sqlite3DbNNFreeNN(db, (*p).pVList as *mut ::core::ffi::c_void);
            }
            if !(*p).pFree.is_null() {
                sqlite3DbNNFreeNN(db, (*p).pFree);
            }
        }
        vdbeFreeOpArray(db, (*p).aOp, (*p).nOp);
        if !(*p).zSql.is_null() {
            sqlite3DbNNFreeNN(db, (*p).zSql as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeDelete(mut p: *mut Vdbe) {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        db = (*p).db;
        sqlite3VdbeClearObject(db, p);
        if (*db).pnBytesFreed.is_null() {
            *(*p).ppVPrev = (*p).pVNext;
            if !(*p).pVNext.is_null() {
                (*(*p).pVNext).ppVPrev = (*p).ppVPrev;
            }
        }
        sqlite3DbNNFreeNN(db, p as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sqlite3VdbeFinishMoveto(
    mut p: *mut VdbeCursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut res: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        extern "C" {
            static mut sqlite3_search_count: ::core::ffi::c_int;
        }
        rc = sqlite3BtreeTableMoveto(
            (*p).uc.pCursor,
            (*p).movetoTarget,
            0 as ::core::ffi::c_int,
            &raw mut res,
        );
        if rc != 0 {
            return rc;
        }
        if res != 0 as ::core::ffi::c_int {
            return sqlite3CorruptError(3807 as ::core::ffi::c_int);
        }
        sqlite3_search_count += 1;
        (*p).deferredMoveto = 0 as u8_0;
        (*p).cacheStatus = CACHE_STALE as u32_0;
        return SQLITE_OK;
    }
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sqlite3VdbeHandleMovedCursor(
    mut p: *mut VdbeCursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut isDifferentRow: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3BtreeCursorRestore((*p).uc.pCursor, &raw mut isDifferentRow);
        (*p).cacheStatus = CACHE_STALE as u32_0;
        if isDifferentRow != 0 {
            (*p).nullRow = 1 as u8_0;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeCursorRestore(
    mut p: *mut VdbeCursor,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3BtreeCursorHasMoved((*p).uc.pCursor) != 0 {
            return sqlite3VdbeHandleMovedCursor(p);
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub static mut sqlite3SmallTypeSizes: [u8_0; 128] = [
    0 as ::core::ffi::c_int as u8_0,
    1 as ::core::ffi::c_int as u8_0,
    2 as ::core::ffi::c_int as u8_0,
    3 as ::core::ffi::c_int as u8_0,
    4 as ::core::ffi::c_int as u8_0,
    6 as ::core::ffi::c_int as u8_0,
    8 as ::core::ffi::c_int as u8_0,
    8 as ::core::ffi::c_int as u8_0,
    0 as ::core::ffi::c_int as u8_0,
    0 as ::core::ffi::c_int as u8_0,
    0 as ::core::ffi::c_int as u8_0,
    0 as ::core::ffi::c_int as u8_0,
    0 as ::core::ffi::c_int as u8_0,
    0 as ::core::ffi::c_int as u8_0,
    1 as ::core::ffi::c_int as u8_0,
    1 as ::core::ffi::c_int as u8_0,
    2 as ::core::ffi::c_int as u8_0,
    2 as ::core::ffi::c_int as u8_0,
    3 as ::core::ffi::c_int as u8_0,
    3 as ::core::ffi::c_int as u8_0,
    4 as ::core::ffi::c_int as u8_0,
    4 as ::core::ffi::c_int as u8_0,
    5 as ::core::ffi::c_int as u8_0,
    5 as ::core::ffi::c_int as u8_0,
    6 as ::core::ffi::c_int as u8_0,
    6 as ::core::ffi::c_int as u8_0,
    7 as ::core::ffi::c_int as u8_0,
    7 as ::core::ffi::c_int as u8_0,
    8 as ::core::ffi::c_int as u8_0,
    8 as ::core::ffi::c_int as u8_0,
    9 as ::core::ffi::c_int as u8_0,
    9 as ::core::ffi::c_int as u8_0,
    10 as ::core::ffi::c_int as u8_0,
    10 as ::core::ffi::c_int as u8_0,
    11 as ::core::ffi::c_int as u8_0,
    11 as ::core::ffi::c_int as u8_0,
    12 as ::core::ffi::c_int as u8_0,
    12 as ::core::ffi::c_int as u8_0,
    13 as ::core::ffi::c_int as u8_0,
    13 as ::core::ffi::c_int as u8_0,
    14 as ::core::ffi::c_int as u8_0,
    14 as ::core::ffi::c_int as u8_0,
    15 as ::core::ffi::c_int as u8_0,
    15 as ::core::ffi::c_int as u8_0,
    16 as ::core::ffi::c_int as u8_0,
    16 as ::core::ffi::c_int as u8_0,
    17 as ::core::ffi::c_int as u8_0,
    17 as ::core::ffi::c_int as u8_0,
    18 as ::core::ffi::c_int as u8_0,
    18 as ::core::ffi::c_int as u8_0,
    19 as ::core::ffi::c_int as u8_0,
    19 as ::core::ffi::c_int as u8_0,
    20 as ::core::ffi::c_int as u8_0,
    20 as ::core::ffi::c_int as u8_0,
    21 as ::core::ffi::c_int as u8_0,
    21 as ::core::ffi::c_int as u8_0,
    22 as ::core::ffi::c_int as u8_0,
    22 as ::core::ffi::c_int as u8_0,
    23 as ::core::ffi::c_int as u8_0,
    23 as ::core::ffi::c_int as u8_0,
    24 as ::core::ffi::c_int as u8_0,
    24 as ::core::ffi::c_int as u8_0,
    25 as ::core::ffi::c_int as u8_0,
    25 as ::core::ffi::c_int as u8_0,
    26 as ::core::ffi::c_int as u8_0,
    26 as ::core::ffi::c_int as u8_0,
    27 as ::core::ffi::c_int as u8_0,
    27 as ::core::ffi::c_int as u8_0,
    28 as ::core::ffi::c_int as u8_0,
    28 as ::core::ffi::c_int as u8_0,
    29 as ::core::ffi::c_int as u8_0,
    29 as ::core::ffi::c_int as u8_0,
    30 as ::core::ffi::c_int as u8_0,
    30 as ::core::ffi::c_int as u8_0,
    31 as ::core::ffi::c_int as u8_0,
    31 as ::core::ffi::c_int as u8_0,
    32 as ::core::ffi::c_int as u8_0,
    32 as ::core::ffi::c_int as u8_0,
    33 as ::core::ffi::c_int as u8_0,
    33 as ::core::ffi::c_int as u8_0,
    34 as ::core::ffi::c_int as u8_0,
    34 as ::core::ffi::c_int as u8_0,
    35 as ::core::ffi::c_int as u8_0,
    35 as ::core::ffi::c_int as u8_0,
    36 as ::core::ffi::c_int as u8_0,
    36 as ::core::ffi::c_int as u8_0,
    37 as ::core::ffi::c_int as u8_0,
    37 as ::core::ffi::c_int as u8_0,
    38 as ::core::ffi::c_int as u8_0,
    38 as ::core::ffi::c_int as u8_0,
    39 as ::core::ffi::c_int as u8_0,
    39 as ::core::ffi::c_int as u8_0,
    40 as ::core::ffi::c_int as u8_0,
    40 as ::core::ffi::c_int as u8_0,
    41 as ::core::ffi::c_int as u8_0,
    41 as ::core::ffi::c_int as u8_0,
    42 as ::core::ffi::c_int as u8_0,
    42 as ::core::ffi::c_int as u8_0,
    43 as ::core::ffi::c_int as u8_0,
    43 as ::core::ffi::c_int as u8_0,
    44 as ::core::ffi::c_int as u8_0,
    44 as ::core::ffi::c_int as u8_0,
    45 as ::core::ffi::c_int as u8_0,
    45 as ::core::ffi::c_int as u8_0,
    46 as ::core::ffi::c_int as u8_0,
    46 as ::core::ffi::c_int as u8_0,
    47 as ::core::ffi::c_int as u8_0,
    47 as ::core::ffi::c_int as u8_0,
    48 as ::core::ffi::c_int as u8_0,
    48 as ::core::ffi::c_int as u8_0,
    49 as ::core::ffi::c_int as u8_0,
    49 as ::core::ffi::c_int as u8_0,
    50 as ::core::ffi::c_int as u8_0,
    50 as ::core::ffi::c_int as u8_0,
    51 as ::core::ffi::c_int as u8_0,
    51 as ::core::ffi::c_int as u8_0,
    52 as ::core::ffi::c_int as u8_0,
    52 as ::core::ffi::c_int as u8_0,
    53 as ::core::ffi::c_int as u8_0,
    53 as ::core::ffi::c_int as u8_0,
    54 as ::core::ffi::c_int as u8_0,
    54 as ::core::ffi::c_int as u8_0,
    55 as ::core::ffi::c_int as u8_0,
    55 as ::core::ffi::c_int as u8_0,
    56 as ::core::ffi::c_int as u8_0,
    56 as ::core::ffi::c_int as u8_0,
    57 as ::core::ffi::c_int as u8_0,
    57 as ::core::ffi::c_int as u8_0,
];
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSerialTypeLen(mut serial_type: u32_0) -> u32_0 {
    unsafe {
        if serial_type >= 128 as u32_0 {
            return serial_type.wrapping_sub(12 as u32_0).wrapping_div(2 as u32_0)
        } else {
            return sqlite3SmallTypeSizes[serial_type as usize] as u32_0
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeOneByteSerialTypeLen(mut serial_type: u8_0) -> u8_0 {
    unsafe {
        return sqlite3SmallTypeSizes[serial_type as usize];
    }
}
unsafe extern "C" fn serialGet(
    mut buf: *const ::core::ffi::c_uchar,
    mut serial_type: u32_0,
    mut pMem: *mut Mem,
) {
    unsafe {
        let mut x: u64_0 = ((*buf.offset(0 as ::core::ffi::c_int as isize) as u32_0)
            << 24 as ::core::ffi::c_int
            | ((*buf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as u32_0
            | ((*buf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as u32_0
            | *buf.offset(3 as ::core::ffi::c_int as isize) as u32_0) as u64_0;
        let mut y: u32_0 = (*buf
            .offset(4 as ::core::ffi::c_int as isize)
            .offset(0 as ::core::ffi::c_int as isize) as u32_0)
            << 24 as ::core::ffi::c_int
            | ((*buf
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as u32_0
            | ((*buf
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as u32_0
            | *buf
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(3 as ::core::ffi::c_int as isize) as u32_0;
        x = ((x as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int)
            .wrapping_add(y as ::core::ffi::c_ulonglong) as u64_0;
        if serial_type == 6 as u32_0 {
            (*pMem).u.i = *(&raw mut x as *mut i64_0);
            (*pMem).flags = MEM_Int as u16_0;
        } else {
            memcpy(
                &raw mut (*pMem).u.r as *mut ::core::ffi::c_void,
                &raw mut x as *const ::core::ffi::c_void,
                ::core::mem::size_of::<u64_0>() as size_t,
            );
            (*pMem).flags = (if x & EXP754 == EXP754
                && x as ::core::ffi::c_ulonglong & MAN754
                    != 0 as ::core::ffi::c_ulonglong
            {
                MEM_Null
            } else {
                MEM_Real
            }) as u16_0;
        };
    }
}
unsafe extern "C" fn serialGet7(
    mut buf: *const ::core::ffi::c_uchar,
    mut pMem: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut x: u64_0 = ((*buf.offset(0 as ::core::ffi::c_int as isize) as u32_0)
            << 24 as ::core::ffi::c_int
            | ((*buf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as u32_0
            | ((*buf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as u32_0
            | *buf.offset(3 as ::core::ffi::c_int as isize) as u32_0) as u64_0;
        let mut y: u32_0 = (*buf
            .offset(4 as ::core::ffi::c_int as isize)
            .offset(0 as ::core::ffi::c_int as isize) as u32_0)
            << 24 as ::core::ffi::c_int
            | ((*buf
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as u32_0
            | ((*buf
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as u32_0
            | *buf
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(3 as ::core::ffi::c_int as isize) as u32_0;
        x = ((x as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int)
            .wrapping_add(y as ::core::ffi::c_ulonglong) as u64_0;
        memcpy(
            &raw mut (*pMem).u.r as *mut ::core::ffi::c_void,
            &raw mut x as *const ::core::ffi::c_void,
            ::core::mem::size_of::<u64_0>() as size_t,
        );
        if x & EXP754 == EXP754
            && x as ::core::ffi::c_ulonglong & MAN754 != 0 as ::core::ffi::c_ulonglong
        {
            (*pMem).flags = MEM_Null as u16_0;
            return 1 as ::core::ffi::c_int;
        }
        (*pMem).flags = MEM_Real as u16_0;
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSerialGet(
    mut buf: *const ::core::ffi::c_uchar,
    mut serial_type: u32_0,
    mut pMem: *mut Mem,
) {
    unsafe {
        match serial_type {
            10 => {
                (*pMem).flags = (MEM_Null | MEM_Zero) as u16_0;
                (*pMem).n = 0 as ::core::ffi::c_int;
                (*pMem).u.nZero = 0 as ::core::ffi::c_int;
                return;
            }
            11 | 0 => {
                (*pMem).flags = MEM_Null as u16_0;
                return;
            }
            1 => {
                (*pMem).u.i = *buf.offset(0 as ::core::ffi::c_int as isize) as i8_0
                    as i64_0;
                (*pMem).flags = MEM_Int as u16_0;
                return;
            }
            2 => {
                (*pMem).u.i = (256 as ::core::ffi::c_int
                    * *buf.offset(0 as ::core::ffi::c_int as isize) as i8_0
                        as ::core::ffi::c_int
                    | *buf.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as i64_0;
                (*pMem).flags = MEM_Int as u16_0;
                return;
            }
            3 => {
                (*pMem).u.i = (65536 as ::core::ffi::c_int
                    * *buf.offset(0 as ::core::ffi::c_int as isize) as i8_0
                        as ::core::ffi::c_int
                    | (*buf.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                    | *buf.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as i64_0;
                (*pMem).flags = MEM_Int as u16_0;
                return;
            }
            4 => {
                (*pMem).u.i = (16777216 as ::core::ffi::c_int
                    * *buf.offset(0 as ::core::ffi::c_int as isize) as i8_0
                        as ::core::ffi::c_int
                    | (*buf.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                    | (*buf.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                    | *buf.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as i64_0;
                (*pMem).flags = MEM_Int as u16_0;
                return;
            }
            5 => {
                (*pMem).u.i = (((*buf
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as u32_0)
                    << 24 as ::core::ffi::c_int
                    | ((*buf
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 16 as ::core::ffi::c_int) as u32_0
                    | ((*buf
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int) as u32_0
                    | *buf
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(3 as ::core::ffi::c_int as isize) as u32_0)
                    as ::core::ffi::c_longlong
                    + ((1 as ::core::ffi::c_int as ::core::ffi::c_longlong)
                        << 32 as ::core::ffi::c_int)
                        * (256 as ::core::ffi::c_int
                            * *buf.offset(0 as ::core::ffi::c_int as isize) as i8_0
                                as ::core::ffi::c_int
                            | *buf.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as ::core::ffi::c_longlong) as i64_0;
                (*pMem).flags = MEM_Int as u16_0;
                return;
            }
            6 | 7 => {
                serialGet(buf, serial_type, pMem);
                return;
            }
            8 | 9 => {
                (*pMem).u.i = serial_type.wrapping_sub(8 as u32_0) as i64_0;
                (*pMem).flags = MEM_Int as u16_0;
                return;
            }
            _ => {
                static mut aFlag: [u16_0; 2] = [
                    (MEM_Blob | MEM_Ephem) as u16_0,
                    (MEM_Str | MEM_Ephem) as u16_0,
                ];
                (*pMem).z = buf as *mut ::core::ffi::c_char;
                (*pMem).n = serial_type
                    .wrapping_sub(12 as u32_0)
                    .wrapping_div(2 as u32_0) as ::core::ffi::c_int;
                (*pMem).flags = aFlag[(serial_type & 1 as u32_0) as usize];
                return;
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeAllocUnpackedRecord(
    mut pKeyInfo: *mut KeyInfo,
) -> *mut UnpackedRecord {
    unsafe {
        let mut p: *mut UnpackedRecord = ::core::ptr::null_mut::<UnpackedRecord>();
        let mut nByte: u64_0 = 0;
        nByte = (::core::mem::size_of::<UnpackedRecord>() as usize)
            .wrapping_add(
                (::core::mem::size_of::<Mem>() as usize)
                    .wrapping_mul(
                        ((*pKeyInfo).nKeyField as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as usize,
                    ),
            ) as u64_0;
        p = sqlite3DbMallocRaw((*pKeyInfo).db, nByte) as *mut UnpackedRecord;
        if p.is_null() {
            return ::core::ptr::null_mut::<UnpackedRecord>();
        }
        (*p).aMem = (p as *mut ::core::ffi::c_char)
            .offset(::core::mem::size_of::<UnpackedRecord>() as isize)
            as *mut ::core::ffi::c_char as *mut Mem;
        (*p).pKeyInfo = pKeyInfo;
        (*p).nField = ((*pKeyInfo).nKeyField as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as u16_0;
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeRecordUnpack(
    mut nKey: ::core::ffi::c_int,
    mut pKey: *const ::core::ffi::c_void,
    mut p: *mut UnpackedRecord,
) {
    unsafe {
        let mut aKey: *const ::core::ffi::c_uchar = pKey as *const ::core::ffi::c_uchar;
        let mut d: u32_0 = 0;
        let mut idx: u32_0 = 0;
        let mut u: u16_0 = 0;
        let mut szHdr: u32_0 = 0;
        let mut pMem: *mut Mem = (*p).aMem;
        let mut pKeyInfo: *mut KeyInfo = (*p).pKeyInfo;
        (*p).default_rc = 0 as i8_0;
        idx = (if (*aKey as ::core::ffi::c_int)
            < 0x80 as ::core::ffi::c_int as u8_0 as ::core::ffi::c_int
        {
            szHdr = *aKey as u32_0;
            1 as ::core::ffi::c_int
        } else {
            sqlite3GetVarint32(aKey, &raw mut szHdr) as ::core::ffi::c_int
        }) as u8_0 as u32_0;
        d = szHdr;
        u = 0 as u16_0;
        while idx < szHdr && d <= nKey as u32_0 {
            let mut serial_type: u32_0 = 0;
            idx = (idx as ::core::ffi::c_uint)
                .wrapping_add(
                    (if (*aKey.offset(idx as isize) as ::core::ffi::c_int)
                        < 0x80 as ::core::ffi::c_int as u8_0 as ::core::ffi::c_int
                    {
                        serial_type = *aKey.offset(idx as isize) as u32_0;
                        1 as ::core::ffi::c_int
                    } else {
                        sqlite3GetVarint32(
                            aKey.offset(idx as isize) as *const ::core::ffi::c_uchar,
                            &raw mut serial_type,
                        ) as ::core::ffi::c_int
                    }) as u8_0 as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            (*pMem).enc = (*pKeyInfo).enc;
            (*pMem).db = (*pKeyInfo).db;
            (*pMem).szMalloc = 0 as ::core::ffi::c_int;
            (*pMem).z = ::core::ptr::null_mut::<::core::ffi::c_char>();
            sqlite3VdbeSerialGet(
                aKey.offset(d as isize) as *const ::core::ffi::c_uchar,
                serial_type,
                pMem,
            );
            d = (d as ::core::ffi::c_uint)
                .wrapping_add(
                    sqlite3VdbeSerialTypeLen(serial_type) as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            u = u.wrapping_add(1);
            if u as ::core::ffi::c_int >= (*p).nField as ::core::ffi::c_int {
                break;
            }
            pMem = pMem.offset(1);
        }
        if d > nKey as u32_0 && u as ::core::ffi::c_int != 0 {
            sqlite3VdbeMemSetNull(
                pMem
                    .offset(
                        -(((u as ::core::ffi::c_int) < (*p).nField as ::core::ffi::c_int)
                            as ::core::ffi::c_int as isize),
                    ),
            );
        }
        (*p).nField = u;
    }
}
#[inline(never)]
unsafe extern "C" fn vdbeCompareMemStringWithEncodingChange(
    mut pMem1: *const Mem,
    mut pMem2: *const Mem,
    mut pColl: *const CollSeq,
    mut prcErr: *mut u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut v1: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut v2: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut c1: Mem = Mem {
            u: MemValue { r: 0. },
            z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            n: 0,
            flags: 0,
            enc: 0,
            eSubtype: 0,
            db: ::core::ptr::null_mut::<sqlite3>(),
            szMalloc: 0,
            uTemp: 0,
            zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            xDel: None

        };
        let mut c2: Mem = Mem {
            u: MemValue { r: 0. },
            z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            n: 0,
            flags: 0,
            enc: 0,
            eSubtype: 0,
            db: ::core::ptr::null_mut::<sqlite3>(),
            szMalloc: 0,
            uTemp: 0,
            zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            xDel: None

        };
        sqlite3VdbeMemInit(&raw mut c1, (*pMem1).db, MEM_Null as u16_0);
        sqlite3VdbeMemInit(&raw mut c2, (*pMem1).db, MEM_Null as u16_0);
        sqlite3VdbeMemShallowCopy(&raw mut c1, pMem1, MEM_Ephem);
        sqlite3VdbeMemShallowCopy(&raw mut c2, pMem2, MEM_Ephem);
        v1 = sqlite3ValueText(&raw mut c1 as *mut sqlite3_value, (*pColl).enc);
        v2 = sqlite3ValueText(&raw mut c2 as *mut sqlite3_value, (*pColl).enc);
        if v1.is_null() || v2.is_null() {
            if !prcErr.is_null() {
                *prcErr = SQLITE_NOMEM_BKPT as u8_0;
            }
            rc = 0 as ::core::ffi::c_int;
        } else {
            rc = (*pColl)
                .xCmp
                .expect("non-null function pointer")((*pColl).pUser, c1.n, v1, c2.n, v2);
        }
        sqlite3VdbeMemReleaseMalloc(&raw mut c1);
        sqlite3VdbeMemReleaseMalloc(&raw mut c2);
        return rc;
    }
}
unsafe extern "C" fn vdbeCompareMemString(
    mut pMem1: *const Mem,
    mut pMem2: *const Mem,
    mut pColl: *const CollSeq,
    mut prcErr: *mut u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pMem1).enc as ::core::ffi::c_int == (*pColl).enc as ::core::ffi::c_int {
            return (*pColl)
                .xCmp
                .expect(
                    "non-null function pointer",
                )(
                (*pColl).pUser,
                (*pMem1).n,
                (*pMem1).z as *const ::core::ffi::c_void,
                (*pMem2).n,
                (*pMem2).z as *const ::core::ffi::c_void,
            )
        } else {
            return vdbeCompareMemStringWithEncodingChange(pMem1, pMem2, pColl, prcErr)
        };
    }
}
unsafe extern "C" fn isAllZero(
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            if *z.offset(i as isize) != 0 {
                return 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sqlite3BlobCompare(
    mut pB1: *const Mem,
    mut pB2: *const Mem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: ::core::ffi::c_int = 0;
        let mut n1: ::core::ffi::c_int = (*pB1).n;
        let mut n2: ::core::ffi::c_int = (*pB2).n;
        if ((*pB1).flags as ::core::ffi::c_int | (*pB2).flags as ::core::ffi::c_int)
            & MEM_Zero != 0
        {
            if (*pB1).flags as ::core::ffi::c_int & (*pB2).flags as ::core::ffi::c_int
                & MEM_Zero != 0
            {
                return (*pB1).u.nZero - (*pB2).u.nZero
            } else if (*pB1).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                if isAllZero((*pB2).z, (*pB2).n) == 0 {
                    return -1 as ::core::ffi::c_int;
                }
                return (*pB1).u.nZero - n2;
            } else {
                if isAllZero((*pB1).z, (*pB1).n) == 0 {
                    return 1 as ::core::ffi::c_int;
                }
                return n1 - (*pB2).u.nZero;
            }
        }
        c = memcmp(
            (*pB1).z as *const ::core::ffi::c_void,
            (*pB2).z as *const ::core::ffi::c_void,
            (if n1 > n2 { n2 } else { n1 }) as size_t,
        );
        if c != 0 {
            return c;
        }
        return n1 - n2;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IntFloatCompare(
    mut i: i64_0,
    mut r: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3IsNaN(r) != 0 {
            return 1 as ::core::ffi::c_int
        } else {
            let mut y: i64_0 = 0;
            if r < -9223372036854775808.0f64 {
                return 1 as ::core::ffi::c_int;
            }
            if r >= 9223372036854775808.0f64 {
                return -1 as ::core::ffi::c_int;
            }
            y = r as i64_0;
            if i < y {
                return -1 as ::core::ffi::c_int;
            }
            if i > y {
                return 1 as ::core::ffi::c_int;
            }
            return if (i as ::core::ffi::c_double) < r {
                -1 as ::core::ffi::c_int
            } else {
                (i as ::core::ffi::c_double > r) as ::core::ffi::c_int
            };
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MemCompare(
    mut pMem1: *const Mem,
    mut pMem2: *const Mem,
    mut pColl: *const CollSeq,
) -> ::core::ffi::c_int {
    unsafe {
        let mut f1: ::core::ffi::c_int = 0;
        let mut f2: ::core::ffi::c_int = 0;
        let mut combined_flags: ::core::ffi::c_int = 0;
        f1 = (*pMem1).flags as ::core::ffi::c_int;
        f2 = (*pMem2).flags as ::core::ffi::c_int;
        combined_flags = f1 | f2;
        if combined_flags & MEM_Null != 0 {
            return (f2 & MEM_Null) - (f1 & MEM_Null);
        }
        if combined_flags & (MEM_Int | MEM_Real | MEM_IntReal) != 0 {
            if f1 & f2 & (MEM_Int | MEM_IntReal) != 0 as ::core::ffi::c_int {
                if (*pMem1).u.i < (*pMem2).u.i {
                    return -1 as ::core::ffi::c_int;
                }
                if (*pMem1).u.i > (*pMem2).u.i {
                    return 1 as ::core::ffi::c_int;
                }
                return 0 as ::core::ffi::c_int;
            }
            if f1 & f2 & MEM_Real != 0 as ::core::ffi::c_int {
                if (*pMem1).u.r < (*pMem2).u.r {
                    return -1 as ::core::ffi::c_int;
                }
                if (*pMem1).u.r > (*pMem2).u.r {
                    return 1 as ::core::ffi::c_int;
                }
                return 0 as ::core::ffi::c_int;
            }
            if f1 & (MEM_Int | MEM_IntReal) != 0 as ::core::ffi::c_int {
                if f2 & MEM_Real != 0 as ::core::ffi::c_int {
                    return sqlite3IntFloatCompare((*pMem1).u.i, (*pMem2).u.r)
                } else if f2 & (MEM_Int | MEM_IntReal) != 0 as ::core::ffi::c_int {
                    if (*pMem1).u.i < (*pMem2).u.i {
                        return -1 as ::core::ffi::c_int;
                    }
                    if (*pMem1).u.i > (*pMem2).u.i {
                        return 1 as ::core::ffi::c_int;
                    }
                    return 0 as ::core::ffi::c_int;
                } else {
                    return -1 as ::core::ffi::c_int
                }
            }
            if f1 & MEM_Real != 0 as ::core::ffi::c_int {
                if f2 & (MEM_Int | MEM_IntReal) != 0 as ::core::ffi::c_int {
                    return -sqlite3IntFloatCompare((*pMem2).u.i, (*pMem1).u.r)
                } else {
                    return -1 as ::core::ffi::c_int
                }
            }
            return 1 as ::core::ffi::c_int;
        }
        if combined_flags & MEM_Str != 0 {
            if f1 & MEM_Str == 0 as ::core::ffi::c_int {
                return 1 as ::core::ffi::c_int;
            }
            if f2 & MEM_Str == 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            if !pColl.is_null() {
                return vdbeCompareMemString(
                    pMem1,
                    pMem2,
                    pColl,
                    ::core::ptr::null_mut::<u8_0>(),
                );
            }
        }
        return sqlite3BlobCompare(pMem1, pMem2);
    }
}
unsafe extern "C" fn vdbeRecordDecodeInt(
    mut serial_type: u32_0,
    mut aKey: *const u8_0,
) -> i64_0 {
    unsafe {
        let mut y: u32_0 = 0;
        match serial_type {
            0 | 1 => {
                return *aKey.offset(0 as ::core::ffi::c_int as isize) as i8_0 as i64_0;
            }
            2 => {
                return (256 as ::core::ffi::c_int
                    * *aKey.offset(0 as ::core::ffi::c_int as isize) as i8_0
                        as ::core::ffi::c_int
                    | *aKey.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as i64_0;
            }
            3 => {
                return (65536 as ::core::ffi::c_int
                    * *aKey.offset(0 as ::core::ffi::c_int as isize) as i8_0
                        as ::core::ffi::c_int
                    | (*aKey.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                    | *aKey.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as i64_0;
            }
            4 => {
                y = (*aKey.offset(0 as ::core::ffi::c_int as isize) as u32_0)
                    << 24 as ::core::ffi::c_int
                    | ((*aKey.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32_0
                    | ((*aKey.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32_0
                    | *aKey.offset(3 as ::core::ffi::c_int as isize) as u32_0;
                return *(&raw mut y as *mut ::core::ffi::c_int) as i64_0;
            }
            5 => {
                return ((*aKey
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as u32_0)
                    << 24 as ::core::ffi::c_int
                    | ((*aKey
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 16 as ::core::ffi::c_int) as u32_0
                    | ((*aKey
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int) as u32_0
                    | *aKey
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(3 as ::core::ffi::c_int as isize) as u32_0) as i64_0
                    + ((1 as ::core::ffi::c_int as i64_0) << 32 as ::core::ffi::c_int)
                        * (256 as ::core::ffi::c_int
                            * *aKey.offset(0 as ::core::ffi::c_int as isize) as i8_0
                                as ::core::ffi::c_int
                            | *aKey.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as i64_0;
            }
            6 => {
                let mut x: u64_0 = ((*aKey.offset(0 as ::core::ffi::c_int as isize)
                    as u32_0) << 24 as ::core::ffi::c_int
                    | ((*aKey.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32_0
                    | ((*aKey.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32_0
                    | *aKey.offset(3 as ::core::ffi::c_int as isize) as u32_0) as u64_0;
                x = ((x as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int
                    | ((*aKey
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as u32_0)
                        << 24 as ::core::ffi::c_int
                        | ((*aKey
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32_0
                        | ((*aKey
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32_0
                        | *aKey
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(3 as ::core::ffi::c_int as isize) as u32_0)
                        as ::core::ffi::c_ulonglong) as u64_0;
                return *(&raw mut x as *mut i64_0);
            }
            _ => {}
        }
        return serial_type.wrapping_sub(8 as u32_0) as i64_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeRecordCompareWithSkip(
    mut nKey1: ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut pPKey2: *mut UnpackedRecord,
    mut bSkip: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut d1: u32_0 = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut szHdr1: u32_0 = 0;
        let mut idx1: u32_0 = 0;
        let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pRhs: *mut Mem = (*pPKey2).aMem;
        let mut pKeyInfo: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        let mut aKey1: *const ::core::ffi::c_uchar = pKey1
            as *const ::core::ffi::c_uchar;
        let mut mem1: Mem = Mem {
            u: MemValue { r: 0. },
            z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            n: 0,
            flags: 0,
            enc: 0,
            eSubtype: 0,
            db: ::core::ptr::null_mut::<sqlite3>(),
            szMalloc: 0,
            uTemp: 0,
            zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            xDel: None

        };
        if bSkip != 0 {
            let mut s1: u32_0 = *aKey1.offset(1 as ::core::ffi::c_int as isize) as u32_0;
            if s1 < 0x80 as u32_0 {
                idx1 = 2 as u32_0;
            } else {
                idx1 = (1 as ::core::ffi::c_int
                    + sqlite3GetVarint32(
                        aKey1.offset(1 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_uchar,
                        &raw mut s1,
                    ) as ::core::ffi::c_int) as u32_0;
            }
            szHdr1 = *aKey1.offset(0 as ::core::ffi::c_int as isize) as u32_0;
            d1 = szHdr1.wrapping_add(sqlite3VdbeSerialTypeLen(s1));
            i = 1 as ::core::ffi::c_int;
            pRhs = pRhs.offset(1);
        } else {
            szHdr1 = *aKey1.offset(0 as ::core::ffi::c_int as isize) as u32_0;
            if szHdr1 < 0x80 as u32_0 {
                idx1 = 1 as u32_0;
            } else {
                idx1 = sqlite3GetVarint32(aKey1, &raw mut szHdr1) as u32_0;
            }
            d1 = szHdr1;
            i = 0 as ::core::ffi::c_int;
        }
        if d1 > nKey1 as u32_0 {
            (*pPKey2).errCode = sqlite3CorruptError(4768 as ::core::ffi::c_int) as u8_0;
            return 0 as ::core::ffi::c_int;
        }
        loop {
            let mut serial_type: u32_0 = 0;
            if (*pRhs).flags as ::core::ffi::c_int & (MEM_Int | MEM_IntReal) != 0 {
                serial_type = *aKey1.offset(idx1 as isize) as u32_0;
                if serial_type >= 10 as u32_0 {
                    rc = if serial_type == 10 as u32_0 {
                        -1 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    };
                } else if serial_type == 0 as u32_0 {
                    rc = -1 as ::core::ffi::c_int;
                } else if serial_type == 7 as u32_0 {
                    serialGet7(
                        aKey1.offset(d1 as isize) as *const ::core::ffi::c_uchar,
                        &raw mut mem1,
                    );
                    rc = -sqlite3IntFloatCompare((*pRhs).u.i, mem1.u.r);
                } else {
                    let mut lhs: i64_0 = vdbeRecordDecodeInt(
                        serial_type,
                        aKey1.offset(d1 as isize) as *const u8_0,
                    );
                    let mut rhs: i64_0 = (*pRhs).u.i;
                    if lhs < rhs {
                        rc = -1 as ::core::ffi::c_int;
                    } else if lhs > rhs {
                        rc = 1 as ::core::ffi::c_int;
                    }
                }
            } else if (*pRhs).flags as ::core::ffi::c_int & MEM_Real != 0 {
                serial_type = *aKey1.offset(idx1 as isize) as u32_0;
                if serial_type >= 10 as u32_0 {
                    rc = if serial_type == 10 as u32_0 {
                        -1 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    };
                } else if serial_type == 0 as u32_0 {
                    rc = -1 as ::core::ffi::c_int;
                } else if serial_type == 7 as u32_0 {
                    if serialGet7(
                        aKey1.offset(d1 as isize) as *const ::core::ffi::c_uchar,
                        &raw mut mem1,
                    ) != 0
                    {
                        rc = -1 as ::core::ffi::c_int;
                    } else if mem1.u.r < (*pRhs).u.r {
                        rc = -1 as ::core::ffi::c_int;
                    } else if mem1.u.r > (*pRhs).u.r {
                        rc = 1 as ::core::ffi::c_int;
                    }
                } else {
                    sqlite3VdbeSerialGet(
                        aKey1.offset(d1 as isize) as *const ::core::ffi::c_uchar,
                        serial_type,
                        &raw mut mem1,
                    );
                    rc = sqlite3IntFloatCompare(mem1.u.i, (*pRhs).u.r);
                }
            } else if (*pRhs).flags as ::core::ffi::c_int & MEM_Str != 0 {
                serial_type = *aKey1.offset(idx1 as isize) as u32_0;
                if serial_type >= 0x80 as u32_0 {
                    sqlite3GetVarint32(
                        aKey1.offset(idx1 as isize) as *const ::core::ffi::c_uchar,
                        &raw mut serial_type,
                    );
                }
                if serial_type < 12 as u32_0 {
                    rc = -1 as ::core::ffi::c_int;
                } else if serial_type & 0x1 as u32_0 == 0 {
                    rc = 1 as ::core::ffi::c_int;
                } else {
                    mem1.n = serial_type
                        .wrapping_sub(12 as u32_0)
                        .wrapping_div(2 as u32_0) as ::core::ffi::c_int;
                    if d1.wrapping_add(mem1.n as u32_0) > nKey1 as u32_0
                        || {
                            pKeyInfo = (*pPKey2).pKeyInfo;
                            (*pKeyInfo).nAllField as ::core::ffi::c_int <= i
                        }
                    {
                        (*pPKey2).errCode = sqlite3CorruptError(
                            4849 as ::core::ffi::c_int,
                        ) as u8_0;
                        return 0 as ::core::ffi::c_int;
                    } else if !(*(&raw mut (*pKeyInfo).aColl as *mut *mut CollSeq)
                        .offset(i as isize))
                        .is_null()
                    {
                        mem1.enc = (*pKeyInfo).enc;
                        mem1.db = (*pKeyInfo).db;
                        mem1.flags = MEM_Str as u16_0;
                        mem1.z = aKey1.offset(d1 as isize) as *const ::core::ffi::c_uchar
                            as *mut ::core::ffi::c_char;
                        rc = vdbeCompareMemString(
                            &raw mut mem1,
                            pRhs,
                            *(&raw mut (*pKeyInfo).aColl as *mut *mut CollSeq)
                                .offset(i as isize),
                            &raw mut (*pPKey2).errCode,
                        );
                    } else {
                        let mut nCmp: ::core::ffi::c_int = if mem1.n < (*pRhs).n {
                            mem1.n
                        } else {
                            (*pRhs).n
                        };
                        rc = memcmp(
                            aKey1.offset(d1 as isize) as *const ::core::ffi::c_uchar
                                as *const ::core::ffi::c_void,
                            (*pRhs).z as *const ::core::ffi::c_void,
                            nCmp as size_t,
                        );
                        if rc == 0 as ::core::ffi::c_int {
                            rc = mem1.n - (*pRhs).n;
                        }
                    }
                }
            } else if (*pRhs).flags as ::core::ffi::c_int & MEM_Blob != 0 {
                serial_type = *aKey1.offset(idx1 as isize) as u32_0;
                if serial_type >= 0x80 as u32_0 {
                    sqlite3GetVarint32(
                        aKey1.offset(idx1 as isize) as *const ::core::ffi::c_uchar,
                        &raw mut serial_type,
                    );
                }
                if serial_type < 12 as u32_0 || serial_type & 0x1 as u32_0 != 0 {
                    rc = -1 as ::core::ffi::c_int;
                } else {
                    let mut nStr: ::core::ffi::c_int = serial_type
                        .wrapping_sub(12 as u32_0)
                        .wrapping_div(2 as u32_0) as ::core::ffi::c_int;
                    if d1.wrapping_add(nStr as u32_0) > nKey1 as u32_0 {
                        (*pPKey2).errCode = sqlite3CorruptError(
                            4879 as ::core::ffi::c_int,
                        ) as u8_0;
                        return 0 as ::core::ffi::c_int;
                    } else if (*pRhs).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                        if isAllZero(
                            aKey1.offset(d1 as isize) as *const ::core::ffi::c_uchar
                                as *const ::core::ffi::c_char,
                            nStr,
                        ) == 0
                        {
                            rc = 1 as ::core::ffi::c_int;
                        } else {
                            rc = nStr - (*pRhs).u.nZero;
                        }
                    } else {
                        let mut nCmp_0: ::core::ffi::c_int = if nStr < (*pRhs).n {
                            nStr
                        } else {
                            (*pRhs).n
                        };
                        rc = memcmp(
                            aKey1.offset(d1 as isize) as *const ::core::ffi::c_uchar
                                as *const ::core::ffi::c_void,
                            (*pRhs).z as *const ::core::ffi::c_void,
                            nCmp_0 as size_t,
                        );
                        if rc == 0 as ::core::ffi::c_int {
                            rc = nStr - (*pRhs).n;
                        }
                    }
                }
            } else {
                serial_type = *aKey1.offset(idx1 as isize) as u32_0;
                if !(serial_type == 0 as u32_0 || serial_type == 10 as u32_0
                    || serial_type == 7 as u32_0
                        && serialGet7(
                            aKey1.offset(d1 as isize) as *const ::core::ffi::c_uchar,
                            &raw mut mem1,
                        ) != 0 as ::core::ffi::c_int)
                {
                    rc = 1 as ::core::ffi::c_int;
                }
            }
            if rc != 0 as ::core::ffi::c_int {
                let mut sortFlags: ::core::ffi::c_int = *(*(*pPKey2).pKeyInfo)
                    .aSortFlags
                    .offset(i as isize) as ::core::ffi::c_int;
                if sortFlags != 0 {
                    if sortFlags & KEYINFO_ORDER_BIGNULL == 0 as ::core::ffi::c_int
                        || sortFlags & KEYINFO_ORDER_DESC
                            != (serial_type == 0 as u32_0
                                || (*pRhs).flags as ::core::ffi::c_int & MEM_Null != 0)
                                as ::core::ffi::c_int
                    {
                        rc = -rc;
                    }
                }
                return rc;
            }
            i += 1;
            if i == (*pPKey2).nField as ::core::ffi::c_int {
                break;
            }
            pRhs = pRhs.offset(1);
            d1 = (d1 as ::core::ffi::c_uint)
                .wrapping_add(
                    sqlite3VdbeSerialTypeLen(serial_type) as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            if d1 > nKey1 as u32_0 {
                break;
            }
            idx1 = (idx1 as ::core::ffi::c_uint)
                .wrapping_add(
                    sqlite3VarintLen(serial_type as u64_0) as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            if idx1 >= szHdr1 {
                (*pPKey2).errCode = sqlite3CorruptError(4930 as ::core::ffi::c_int)
                    as u8_0;
                return 0 as ::core::ffi::c_int;
            }
        }
        (*pPKey2).eqSeen = 1 as u8_0;
        return (*pPKey2).default_rc as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeRecordCompare(
    mut nKey1: ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut pPKey2: *mut UnpackedRecord,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3VdbeRecordCompareWithSkip(
            nKey1,
            pKey1,
            pPKey2,
            0 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn vdbeRecordCompareInt(
    mut nKey1: ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut pPKey2: *mut UnpackedRecord,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aKey: *const u8_0 = (pKey1 as *const u8_0)
            .offset(
                (*(pKey1 as *const u8_0) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int) as isize,
            ) as *const u8_0;
        let mut serial_type: ::core::ffi::c_int = *(pKey1 as *const u8_0)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut res: ::core::ffi::c_int = 0;
        let mut y: u32_0 = 0;
        let mut x: u64_0 = 0;
        let mut v: i64_0 = 0;
        let mut lhs: i64_0 = 0;
        match serial_type {
            1 => {
                lhs = *aKey.offset(0 as ::core::ffi::c_int as isize) as i8_0 as i64_0;
            }
            2 => {
                lhs = (256 as ::core::ffi::c_int
                    * *aKey.offset(0 as ::core::ffi::c_int as isize) as i8_0
                        as ::core::ffi::c_int
                    | *aKey.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as i64_0;
            }
            3 => {
                lhs = (65536 as ::core::ffi::c_int
                    * *aKey.offset(0 as ::core::ffi::c_int as isize) as i8_0
                        as ::core::ffi::c_int
                    | (*aKey.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                    | *aKey.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as i64_0;
            }
            4 => {
                y = (*aKey.offset(0 as ::core::ffi::c_int as isize) as u32_0)
                    << 24 as ::core::ffi::c_int
                    | ((*aKey.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32_0
                    | ((*aKey.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32_0
                    | *aKey.offset(3 as ::core::ffi::c_int as isize) as u32_0;
                lhs = *(&raw mut y as *mut ::core::ffi::c_int) as i64_0;
            }
            5 => {
                lhs = (((*aKey
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as u32_0)
                    << 24 as ::core::ffi::c_int
                    | ((*aKey
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 16 as ::core::ffi::c_int) as u32_0
                    | ((*aKey
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int) as u32_0
                    | *aKey
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(3 as ::core::ffi::c_int as isize) as u32_0)
                    as ::core::ffi::c_longlong
                    + ((1 as ::core::ffi::c_int as ::core::ffi::c_longlong)
                        << 32 as ::core::ffi::c_int)
                        * (256 as ::core::ffi::c_int
                            * *aKey.offset(0 as ::core::ffi::c_int as isize) as i8_0
                                as ::core::ffi::c_int
                            | *aKey.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as ::core::ffi::c_longlong) as i64_0;
            }
            6 => {
                x = ((*aKey.offset(0 as ::core::ffi::c_int as isize) as u32_0)
                    << 24 as ::core::ffi::c_int
                    | ((*aKey.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32_0
                    | ((*aKey.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32_0
                    | *aKey.offset(3 as ::core::ffi::c_int as isize) as u32_0) as u64_0;
                x = ((x as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int
                    | ((*aKey
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as u32_0)
                        << 24 as ::core::ffi::c_int
                        | ((*aKey
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32_0
                        | ((*aKey
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32_0
                        | *aKey
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(3 as ::core::ffi::c_int as isize) as u32_0)
                        as ::core::ffi::c_ulonglong) as u64_0;
                lhs = *(&raw mut x as *mut i64_0);
            }
            8 => {
                lhs = 0 as i64_0;
            }
            9 => {
                lhs = 1 as i64_0;
            }
            0 | 7 => return sqlite3VdbeRecordCompare(nKey1, pKey1, pPKey2),
            _ => return sqlite3VdbeRecordCompare(nKey1, pKey1, pPKey2)

        }
        v = (*pPKey2).u.i;
        if v > lhs {
            res = (*pPKey2).r1 as ::core::ffi::c_int;
        } else if v < lhs {
            res = (*pPKey2).r2 as ::core::ffi::c_int;
        } else if (*pPKey2).nField as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            res = sqlite3VdbeRecordCompareWithSkip(
                nKey1,
                pKey1,
                pPKey2,
                1 as ::core::ffi::c_int,
            );
        } else {
            res = (*pPKey2).default_rc as ::core::ffi::c_int;
            (*pPKey2).eqSeen = 1 as u8_0;
        }
        return res;
    }
}
unsafe extern "C" fn vdbeRecordCompareString(
    mut nKey1: ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut pPKey2: *mut UnpackedRecord,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aKey1: *const u8_0 = pKey1 as *const u8_0;
        let mut serial_type: ::core::ffi::c_int = 0;
        let mut res: ::core::ffi::c_int = 0;
        serial_type = *aKey1.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_schar as ::core::ffi::c_int;
        let mut c2rust_current_block_39: u64;
        loop {
            if serial_type < 12 as ::core::ffi::c_int {
                if !(serial_type < 0 as ::core::ffi::c_int) {
                    c2rust_current_block_39 = 17216689946888361452;
                    break;
                }
                sqlite3GetVarint32(
                    aKey1.offset(1 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_uchar,
                    &raw mut serial_type as *mut u32_0,
                );
                if !(serial_type >= 12 as ::core::ffi::c_int) {
                    c2rust_current_block_39 = 17216689946888361452;
                    break;
                }
            } else {
                if serial_type & 0x1 as ::core::ffi::c_int == 0 {
                    res = (*pPKey2).r2 as ::core::ffi::c_int;
                } else {
                    let mut nCmp: ::core::ffi::c_int = 0;
                    let mut nStr: ::core::ffi::c_int = 0;
                    let mut szHdr: ::core::ffi::c_int = *aKey1
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                    nStr = (serial_type - 12 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
                    if szHdr + nStr > nKey1 {
                        (*pPKey2).errCode = sqlite3CorruptError(
                            5093 as ::core::ffi::c_int,
                        ) as u8_0;
                        return 0 as ::core::ffi::c_int;
                    }
                    nCmp = if (*pPKey2).n < nStr { (*pPKey2).n } else { nStr };
                    res = memcmp(
                        aKey1.offset(szHdr as isize) as *const u8_0
                            as *const ::core::ffi::c_void,
                        (*pPKey2).u.z as *const ::core::ffi::c_void,
                        nCmp as size_t,
                    );
                    if res > 0 as ::core::ffi::c_int {
                        res = (*pPKey2).r2 as ::core::ffi::c_int;
                    } else if res < 0 as ::core::ffi::c_int {
                        res = (*pPKey2).r1 as ::core::ffi::c_int;
                    } else {
                        res = nStr - (*pPKey2).n;
                        if res == 0 as ::core::ffi::c_int {
                            if (*pPKey2).nField as ::core::ffi::c_int
                                > 1 as ::core::ffi::c_int
                            {
                                res = sqlite3VdbeRecordCompareWithSkip(
                                    nKey1,
                                    pKey1,
                                    pPKey2,
                                    1 as ::core::ffi::c_int,
                                );
                            } else {
                                res = (*pPKey2).default_rc as ::core::ffi::c_int;
                                (*pPKey2).eqSeen = 1 as u8_0;
                            }
                        } else if res > 0 as ::core::ffi::c_int {
                            res = (*pPKey2).r2 as ::core::ffi::c_int;
                        } else {
                            res = (*pPKey2).r1 as ::core::ffi::c_int;
                        }
                    }
                }
                c2rust_current_block_39 = 10692455896603418738;
                break;
            }
        }
        match c2rust_current_block_39 {
            17216689946888361452 => {
                res = (*pPKey2).r1 as ::core::ffi::c_int;
            }
            _ => {}
        }
        return res;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeFindCompare(
    mut p: *mut UnpackedRecord,
) -> RecordCompare {
    unsafe {
        if (*(*p).pKeyInfo).nAllField as ::core::ffi::c_int <= 13 as ::core::ffi::c_int {
            let mut flags: ::core::ffi::c_int = (*(*p)
                .aMem
                .offset(0 as ::core::ffi::c_int as isize))
                .flags as ::core::ffi::c_int;
            if *(*(*p).pKeyInfo).aSortFlags.offset(0 as ::core::ffi::c_int as isize) != 0
            {
                if *(*(*p).pKeyInfo).aSortFlags.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int & KEYINFO_ORDER_BIGNULL != 0
                {
                    return Some(
                        sqlite3VdbeRecordCompare
                            as unsafe extern "C" fn(
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_void,
                                *mut UnpackedRecord,
                            ) -> ::core::ffi::c_int,
                    );
                }
                (*p).r1 = 1 as i8_0;
                (*p).r2 = -1 as ::core::ffi::c_int as i8_0;
            } else {
                (*p).r1 = -1 as ::core::ffi::c_int as i8_0;
                (*p).r2 = 1 as i8_0;
            }
            if flags & MEM_Int != 0 {
                (*p).u.i = (*(*p).aMem.offset(0 as ::core::ffi::c_int as isize)).u.i;
                return Some(
                    vdbeRecordCompareInt
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                            *mut UnpackedRecord,
                        ) -> ::core::ffi::c_int,
                );
            }
            if flags & (MEM_Real | MEM_IntReal | MEM_Null | MEM_Blob)
                == 0 as ::core::ffi::c_int
                && (*(&raw mut (*(*p).pKeyInfo).aColl as *mut *mut CollSeq)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .is_null()
            {
                (*p).u.z = (*(*p).aMem.offset(0 as ::core::ffi::c_int as isize)).z;
                (*p).n = (*(*p).aMem.offset(0 as ::core::ffi::c_int as isize)).n;
                return Some(
                    vdbeRecordCompareString
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                            *mut UnpackedRecord,
                        ) -> ::core::ffi::c_int,
                );
            }
        }
        return Some(
            sqlite3VdbeRecordCompare
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    *mut UnpackedRecord,
                ) -> ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeIdxRowid(
    mut db: *mut sqlite3,
    mut pCur: *mut BtCursor,
    mut rowid: *mut i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nCellKey: i64_0 = 0 as i64_0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut szHdr: u32_0 = 0;
        let mut typeRowid: u32_0 = 0;
        let mut lenRowid: u32_0 = 0;
        let mut m: Mem = Mem {
            u: MemValue { r: 0. },
            z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            n: 0,
            flags: 0,
            enc: 0,
            eSubtype: 0,
            db: ::core::ptr::null_mut::<sqlite3>(),
            szMalloc: 0,
            uTemp: 0,
            zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            xDel: None

        };
        let mut v: Mem = Mem {
            u: MemValue { r: 0. },
            z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            n: 0,
            flags: 0,
            enc: 0,
            eSubtype: 0,
            db: ::core::ptr::null_mut::<sqlite3>(),
            szMalloc: 0,
            uTemp: 0,
            zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            xDel: None

        };
        nCellKey = sqlite3BtreePayloadSize(pCur) as i64_0;
        sqlite3VdbeMemInit(&raw mut m, db, 0 as u16_0);
        rc = sqlite3VdbeMemFromBtreeZeroOffset(pCur, nCellKey as u32_0, &raw mut m);
        if rc != 0 {
            return rc;
        }
        szHdr = *(m.z as *mut u8_0) as u32_0;
        if szHdr >= 0x80 as u32_0 {
            sqlite3GetVarint32(m.z as *mut u8_0, &raw mut szHdr);
        }
        if !(szHdr < 3 as u32_0 || szHdr > m.n as u32_0) {
            typeRowid = *(m.z.offset(szHdr.wrapping_sub(1 as u32_0) as isize)
                as *mut ::core::ffi::c_char as *mut u8_0) as u32_0;
            if typeRowid >= 0x80 as u32_0 {
                sqlite3GetVarint32(
                    m.z.offset(szHdr.wrapping_sub(1 as u32_0) as isize)
                        as *mut ::core::ffi::c_char as *mut u8_0,
                    &raw mut typeRowid,
                );
            }
            if !(typeRowid < 1 as u32_0 || typeRowid > 9 as u32_0
                || typeRowid == 7 as u32_0)
            {
                lenRowid = sqlite3SmallTypeSizes[typeRowid as usize] as u32_0;
                if !((m.n as u32_0) < szHdr.wrapping_add(lenRowid)) {
                    sqlite3VdbeSerialGet(
                        m.z.offset((m.n as u32_0).wrapping_sub(lenRowid) as isize)
                            as *mut ::core::ffi::c_char as *mut u8_0,
                        typeRowid,
                        &raw mut v,
                    );
                    *rowid = v.u.i;
                    sqlite3VdbeMemReleaseMalloc(&raw mut m);
                    return SQLITE_OK;
                }
            }
        }
        sqlite3VdbeMemReleaseMalloc(&raw mut m);
        return sqlite3CorruptError(5252 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeIdxKeyCompare(
    mut db: *mut sqlite3,
    mut pC: *mut VdbeCursor,
    mut pUnpacked: *mut UnpackedRecord,
    mut res: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nCellKey: i64_0 = 0 as i64_0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pCur: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        let mut m: Mem = Mem {
            u: MemValue { r: 0. },
            z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            n: 0,
            flags: 0,
            enc: 0,
            eSubtype: 0,
            db: ::core::ptr::null_mut::<sqlite3>(),
            szMalloc: 0,
            uTemp: 0,
            zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            xDel: None

        };
        pCur = (*pC).uc.pCursor;
        nCellKey = sqlite3BtreePayloadSize(pCur) as i64_0;
        if nCellKey <= 0 as ::core::ffi::c_longlong
            || nCellKey > 0x7fffffff as ::core::ffi::c_longlong
        {
            *res = 0 as ::core::ffi::c_int;
            return sqlite3CorruptError(5285 as ::core::ffi::c_int);
        }
        sqlite3VdbeMemInit(&raw mut m, db, 0 as u16_0);
        rc = sqlite3VdbeMemFromBtreeZeroOffset(pCur, nCellKey as u32_0, &raw mut m);
        if rc != 0 {
            return rc;
        }
        *res = sqlite3VdbeRecordCompareWithSkip(
            m.n,
            m.z as *const ::core::ffi::c_void,
            pUnpacked,
            0 as ::core::ffi::c_int,
        );
        sqlite3VdbeMemReleaseMalloc(&raw mut m);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSetChanges(
    mut db: *mut sqlite3,
    mut nChange: i64_0,
) {
    unsafe {
        (*db).nChange = nChange;
        (*db).nTotalChange += nChange as ::core::ffi::c_longlong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeCountChanges(mut v: *mut Vdbe) {
    unsafe {
        (*v).set_changeCntOn(1 as bft as bft);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ExpirePreparedStatements(
    mut db: *mut sqlite3,
    mut iCode: ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        p = (*db).pVdbe as *mut Vdbe;
        while !p.is_null() {
            (*p).set_expired((iCode + 1 as ::core::ffi::c_int) as bft as bft);
            p = (*p).pVNext;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeDb(mut v: *mut Vdbe) -> *mut sqlite3 {
    unsafe {
        return (*v).db;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbePrepareFlags(mut v: *mut Vdbe) -> u8_0 {
    unsafe {
        return (*v).prepFlags;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeGetBoundValue(
    mut v: *mut Vdbe,
    mut iVar: ::core::ffi::c_int,
    mut aff: u8_0,
) -> *mut sqlite3_value {
    unsafe {
        if !v.is_null() {
            let mut pMem: *mut Mem = (*v)
                .aVar
                .offset((iVar - 1 as ::core::ffi::c_int) as isize) as *mut Mem;
            if 0 as ::core::ffi::c_int == (*pMem).flags as ::core::ffi::c_int & MEM_Null
            {
                let mut pRet: *mut sqlite3_value = sqlite3ValueNew((*v).db);
                if !pRet.is_null() {
                    sqlite3VdbeMemCopy(pRet as *mut Mem, pMem);
                    sqlite3ValueApplyAffinity(pRet, aff, SQLITE_UTF8 as u8_0);
                }
                return pRet;
            }
        }
        return ::core::ptr::null_mut::<sqlite3_value>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeSetVarmask(
    mut v: *mut Vdbe,
    mut iVar: ::core::ffi::c_int,
) {
    unsafe {
        if iVar >= 32 as ::core::ffi::c_int {
            (*v).expmask = ((*v).expmask as ::core::ffi::c_uint
                | 0x80000000 as ::core::ffi::c_uint) as u32_0;
        } else {
            (*v).expmask = ((*v).expmask as ::core::ffi::c_uint
                | ((1 as ::core::ffi::c_int as u32_0) << iVar - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as u32_0;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3NotPureFunc(
    mut pCtx: *mut sqlite3_context,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pOp: *const VdbeOp = ::core::ptr::null::<VdbeOp>();
        pOp = (*(*pCtx).pVdbe).aOp.offset((*pCtx).iOp as isize);
        if (*pOp).opcode as ::core::ffi::c_int == OP_PureFunc {
            let mut zContext: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut zMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            if (*pOp).p5 as ::core::ffi::c_int & NC_IsCheck != 0 {
                zContext = b"a CHECK constraint\0".as_ptr()
                    as *const ::core::ffi::c_char;
            } else if (*pOp).p5 as ::core::ffi::c_int & NC_GenCol != 0 {
                zContext = b"a generated column\0".as_ptr()
                    as *const ::core::ffi::c_char;
            } else {
                zContext = b"an index\0".as_ptr() as *const ::core::ffi::c_char;
            }
            zMsg = sqlite3_mprintf(
                b"non-deterministic use of %s() in %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*(*pCtx).pFunc).zName,
                zContext,
            );
            sqlite3_result_error(pCtx, zMsg, -1 as ::core::ffi::c_int);
            sqlite3_free(zMsg as *mut ::core::ffi::c_void);
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VtabImportErrmsg(
    mut p: *mut Vdbe,
    mut pVtab: *mut sqlite3_vtab,
) {
    unsafe {
        if !(*pVtab).zErrMsg.is_null() {
            let mut db: *mut sqlite3 = (*p).db;
            sqlite3DbFree(db, (*p).zErrMsg as *mut ::core::ffi::c_void);
            (*p).zErrMsg = sqlite3DbStrDup(db, (*pVtab).zErrMsg);
            sqlite3_free((*pVtab).zErrMsg as *mut ::core::ffi::c_void);
            (*pVtab).zErrMsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
}
unsafe extern "C" fn vdbeFreeUnpacked(
    mut db: *mut sqlite3,
    mut nField: ::core::ffi::c_int,
    mut p: *mut UnpackedRecord,
) {
    unsafe {
        if !p.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < nField {
                let mut pMem: *mut Mem = (*p).aMem.offset(i as isize) as *mut Mem;
                if !(*pMem).zMalloc.is_null() {
                    sqlite3VdbeMemReleaseMalloc(pMem);
                }
                i += 1;
            }
            sqlite3DbNNFreeNN(db, p as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbePreUpdateHook(
    mut v: *mut Vdbe,
    mut pCsr: *mut VdbeCursor,
    mut op: ::core::ffi::c_int,
    mut zDb: *const ::core::ffi::c_char,
    mut pTab: *mut Table,
    mut iKey1: i64_0,
    mut iReg: ::core::ffi::c_int,
    mut iBlobWrite: ::core::ffi::c_int,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*v).db;
        let mut iKey2: i64_0 = 0;
        let mut preupdate: PreUpdate = PreUpdate {
            v: ::core::ptr::null_mut::<Vdbe>(),
            pCsr: ::core::ptr::null_mut::<VdbeCursor>(),
            op: 0,
            aRecord: ::core::ptr::null_mut::<u8_0>(),
            pKeyinfo: ::core::ptr::null_mut::<KeyInfo>(),
            pUnpacked: ::core::ptr::null_mut::<UnpackedRecord>(),
            pNewUnpacked: ::core::ptr::null_mut::<UnpackedRecord>(),
            iNewReg: 0,
            iBlobWrite: 0,
            iKey1: 0,
            iKey2: 0,
            oldipk: Mem {
                u: MemValue { r: 0. },
                z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                n: 0,
                flags: 0,
                enc: 0,
                eSubtype: 0,
                db: ::core::ptr::null_mut::<sqlite3>(),
                szMalloc: 0,
                uTemp: 0,
                zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                xDel: None

            },
            aNew: ::core::ptr::null_mut::<Mem>(),
            pTab: ::core::ptr::null_mut::<Table>(),
            pPk: ::core::ptr::null_mut::<Index>(),
            apDflt: ::core::ptr::null_mut::<*mut sqlite3_value>(),
            uKey: C2Rust_Unnamed_23 {
                keyinfoSpace: [0; 32]

            }

        };
        let mut zTbl: *const ::core::ffi::c_char = (*pTab).zName;
        memset(
            &raw mut preupdate as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<PreUpdate>() as size_t,
        );
        if ((*pTab).tabFlags & TF_WithoutRowid as u32_0 == 0 as u32_0)
            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            iKey2 = 0 as i64_0;
            iKey1 = iKey2;
            preupdate.pPk = sqlite3PrimaryKeyIndex(pTab);
        } else if op == SQLITE_UPDATE {
            iKey2 = (*(*v).aMem.offset(iReg as isize)).u.i;
        } else {
            iKey2 = iKey1;
        }
        preupdate.v = v;
        preupdate.pCsr = pCsr;
        preupdate.op = op;
        preupdate.iNewReg = iReg;
        preupdate.pKeyinfo = &raw mut preupdate.uKey as *mut KeyInfo;
        (*preupdate.pKeyinfo).db = db;
        (*preupdate.pKeyinfo).enc = (*db).enc;
        (*preupdate.pKeyinfo).nKeyField = (*pTab).nCol as u16_0;
        (*preupdate.pKeyinfo).aSortFlags = ::core::ptr::null_mut::<u8_0>();
        preupdate.iKey1 = iKey1;
        preupdate.iKey2 = iKey2;
        preupdate.pTab = pTab;
        preupdate.iBlobWrite = iBlobWrite;
        (*db).pPreUpdate = &raw mut preupdate;
        (*db)
            .xPreUpdateCallback
            .expect(
                "non-null function pointer",
            )(
            (*db).pPreUpdateArg,
            db,
            op,
            zDb,
            zTbl,
            iKey1 as sqlite3_int64,
            iKey2 as sqlite3_int64,
        );
        (*db).pPreUpdate = ::core::ptr::null_mut::<PreUpdate>();
        sqlite3DbFree(db, preupdate.aRecord as *mut ::core::ffi::c_void);
        vdbeFreeUnpacked(
            db,
            (*preupdate.pKeyinfo).nKeyField as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int,
            preupdate.pUnpacked,
        );
        vdbeFreeUnpacked(
            db,
            (*preupdate.pKeyinfo).nKeyField as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int,
            preupdate.pNewUnpacked,
        );
        sqlite3VdbeMemRelease(&raw mut preupdate.oldipk);
        if !preupdate.aNew.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pCsr).nField as ::core::ffi::c_int {
                sqlite3VdbeMemRelease(preupdate.aNew.offset(i as isize) as *mut Mem);
                i += 1;
            }
            sqlite3DbNNFreeNN(db, preupdate.aNew as *mut ::core::ffi::c_void);
        }
        if !preupdate.apDflt.is_null() {
            let mut i_0: ::core::ffi::c_int = 0;
            i_0 = 0 as ::core::ffi::c_int;
            while i_0 < (*pTab).nCol as ::core::ffi::c_int {
                sqlite3ValueFree(*preupdate.apDflt.offset(i_0 as isize));
                i_0 += 1;
            }
            sqlite3DbFree(db, preupdate.apDflt as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeFuncName(
    mut pCtx: *const sqlite3_context,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*(*pCtx).pFunc).zName;
    }
}
pub const PAGER_SYNCHRONOUS_OFF: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const P4_NOTUSED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const P4_COLLSEQ: ::core::ffi::c_int = -2;
pub const P4_INT32: ::core::ffi::c_int = -3 as ::core::ffi::c_int;
pub const P4_SUBPROGRAM: ::core::ffi::c_int = -4 as ::core::ffi::c_int;
pub const P4_TABLE: ::core::ffi::c_int = -5;
pub const P4_FREE_IF_LE: ::core::ffi::c_int = -6 as ::core::ffi::c_int;
pub const P4_DYNAMIC: ::core::ffi::c_int = -6 as ::core::ffi::c_int;
pub const P4_FUNCDEF: ::core::ffi::c_int = -7;
pub const P4_KEYINFO: ::core::ffi::c_int = -8;
pub const P4_MEM: ::core::ffi::c_int = -10;
pub const P4_VTAB: ::core::ffi::c_int = -11 as ::core::ffi::c_int;
pub const P4_REAL: ::core::ffi::c_int = -12;
pub const P4_INT64: ::core::ffi::c_int = -13;
pub const P4_INTARRAY: ::core::ffi::c_int = -14;
pub const P4_FUNCCTX: ::core::ffi::c_int = -15;
pub const P4_TABLEREF: ::core::ffi::c_int = -16;
pub const P4_SUBRTNSIG: ::core::ffi::c_int = -17;
pub const COLNAME_N: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_PREPARE_SAVESQL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_DeferFKs: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_TriggerEQP: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const SQLITE_CorruptRdOnly: u64_0 = (0x2 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const SQLITE_FUNC_EPHEM: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SAVEPOINT_RELEASE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SAVEPOINT_ROLLBACK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Fail: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_DESC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_BIGNULL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const NC_IsCheck: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const NC_GenCol: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const NC_SelfRef: ::core::ffi::c_int = 0x2e as ::core::ffi::c_int;
pub const OPFLAG_TYPEOFARG: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const OPFLAG_P2ISREG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const EXP754: u64_0 = (0x7ff as ::core::ffi::c_int as u64_0)
    << 52 as ::core::ffi::c_int;
pub const MAN754: ::core::ffi::c_ulonglong = ((1 as ::core::ffi::c_int
    as ::core::ffi::c_ulonglong) << 52 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_ulonglong);
pub const CURTYPE_BTREE: ::core::ffi::c_int = 0;
pub const CURTYPE_SORTER: ::core::ffi::c_int = 1;
pub const CURTYPE_VTAB: ::core::ffi::c_int = 2;
pub const CACHE_STALE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MEM_Undefined: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MEM_Null: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MEM_Str: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MEM_Int: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MEM_Real: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MEM_Blob: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MEM_IntReal: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MEM_Zero: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const MEM_TypeMask: ::core::ffi::c_int = 0xdbf as ::core::ffi::c_int;
pub const MEM_Dyn: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const MEM_Ephem: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const MEM_Agg: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const VDBE_INIT_STATE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VDBE_READY_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const VDBE_RUN_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const VDBE_HALT_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_MAX_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
