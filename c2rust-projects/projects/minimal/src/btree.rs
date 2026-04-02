use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Bitvec;
    pub type sqlite3_mutex;
    pub type Pager;
    pub type PCache;
    pub type VtabCtx;
    pub type RenameToken;
    pub type Vdbe;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn sqlite3_str_appendf(
        _: *mut sqlite3_str,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3_str_vappendf(
        _: *mut sqlite3_str,
        zFormat: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    );
    fn sqlite3_str_append(
        _: *mut sqlite3_str,
        zIn: *const ::core::ffi::c_char,
        N: ::core::ffi::c_int,
    );
    fn sqlite3_str_reset(_: *mut sqlite3_str);
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsRead(
        _: *mut sqlite3_file,
        _: *mut ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsFileControlHint(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    );
    fn sqlite3PagerOpen(
        _: *mut sqlite3_vfs,
        ppPager: *mut *mut Pager,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut DbPage) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerClose(pPager: *mut Pager, _: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3PagerReadFileheader(
        _: *mut Pager,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerSetBusyHandler(
        _: *mut Pager,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        _: *mut ::core::ffi::c_void,
    );
    fn sqlite3PagerSetPagesize(
        _: *mut Pager,
        _: *mut u32_0,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerMaxPageCount(_: *mut Pager, _: Pgno) -> Pgno;
    fn sqlite3PagerSetCachesize(_: *mut Pager, _: ::core::ffi::c_int);
    fn sqlite3PagerSetSpillsize(
        _: *mut Pager,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerSetMmapLimit(_: *mut Pager, _: sqlite3_int64);
    fn sqlite3PagerSetFlags(_: *mut Pager, _: ::core::ffi::c_uint);
    fn sqlite3PagerGet(
        pPager: *mut Pager,
        pgno: Pgno,
        ppPage: *mut *mut DbPage,
        clrFlag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerLookup(pPager: *mut Pager, pgno: Pgno) -> *mut DbPage;
    fn sqlite3PagerRef(_: *mut DbPage);
    fn sqlite3PagerUnref(_: *mut DbPage);
    fn sqlite3PagerUnrefNotNull(_: *mut DbPage);
    fn sqlite3PagerUnrefPageOne(_: *mut DbPage);
    fn sqlite3PagerWrite(_: *mut DbPage) -> ::core::ffi::c_int;
    fn sqlite3PagerDontWrite(_: *mut DbPage);
    fn sqlite3PagerMovepage(
        _: *mut Pager,
        _: *mut DbPage,
        _: Pgno,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerPageRefcount(_: *mut DbPage) -> ::core::ffi::c_int;
    fn sqlite3PagerGetData(_: *mut DbPage) -> *mut ::core::ffi::c_void;
    fn sqlite3PagerGetExtra(_: *mut DbPage) -> *mut ::core::ffi::c_void;
    fn sqlite3PagerPagecount(_: *mut Pager, _: *mut ::core::ffi::c_int);
    fn sqlite3PagerBegin(
        _: *mut Pager,
        exFlag: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerCommitPhaseOne(
        _: *mut Pager,
        zSuper: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerCommitPhaseTwo(_: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerRollback(_: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerOpenSavepoint(
        pPager: *mut Pager,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerSavepoint(
        pPager: *mut Pager,
        op: ::core::ffi::c_int,
        iSavepoint: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerSharedLock(pPager: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerCheckpoint(
        pPager: *mut Pager,
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerOpenWal(
        pPager: *mut Pager,
        pisOpen: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerDirectReadOk(pPager: *mut Pager, pgno: Pgno) -> ::core::ffi::c_int;
    fn sqlite3PagerIsreadonly(_: *mut Pager) -> u8_0;
    fn sqlite3PagerDataVersion(_: *mut Pager) -> u32_0;
    fn sqlite3PagerFilename(
        _: *const Pager,
        _: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3PagerFile(_: *mut Pager) -> *mut sqlite3_file;
    fn sqlite3PagerJournalname(_: *mut Pager) -> *const ::core::ffi::c_char;
    fn sqlite3PagerTempSpace(_: *mut Pager) -> *mut ::core::ffi::c_void;
    fn sqlite3PagerClearCache(_: *mut Pager);
    fn sqlite3PagerTruncateImage(_: *mut Pager, _: Pgno);
    fn sqlite3PagerRekey(_: *mut DbPage, _: Pgno, _: u16_0);
    fn sqlite3VdbeRecordUnpack(
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: *mut UnpackedRecord,
    );
    fn sqlite3VdbeRecordCompare(
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: *mut UnpackedRecord,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAllocUnpackedRecord(_: *mut KeyInfo) -> *mut UnpackedRecord;
    fn sqlite3VdbeFindCompare(_: *mut UnpackedRecord) -> RecordCompare;
    fn sqlite3MemSetArrayInt64(
        aMem: *mut sqlite3_value,
        iIdx: ::core::ffi::c_int,
        val: i64_0,
    );
    fn sqlite3CorruptError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3Malloc(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3MallocZero(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRaw(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3Realloc(_: *mut ::core::ffi::c_void, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MallocSize(_: *const ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn sqlite3PageMalloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3PageFree(_: *mut ::core::ffi::c_void);
    fn sqlite3FaultSim(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3BitvecCreate(_: u32_0) -> *mut Bitvec;
    fn sqlite3BitvecTestNotNull(_: *mut Bitvec, _: u32_0) -> ::core::ffi::c_int;
    fn sqlite3BitvecSet(_: *mut Bitvec, _: u32_0) -> ::core::ffi::c_int;
    fn sqlite3BitvecDestroy(_: *mut Bitvec);
    fn sqlite3BitvecSize(_: *mut Bitvec) -> u32_0;
    fn sqlite3PutVarint(_: *mut ::core::ffi::c_uchar, _: u64_0) -> ::core::ffi::c_int;
    fn sqlite3GetVarint(_: *const ::core::ffi::c_uchar, _: *mut u64_0) -> u8_0;
    fn sqlite3WritableSchema(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3AbsInt32(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut sqlite3PendingByte: ::core::ffi::c_int;
    fn sqlite3InvokeBusyHandler(_: *mut BusyHandler) -> ::core::ffi::c_int;
    fn sqlite3StrAccumInit(
        _: *mut StrAccum,
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3StrAccumFinish(_: *mut StrAccum) -> *mut ::core::ffi::c_char;
    fn sqlite3TempInMemory(_: *const sqlite3) -> ::core::ffi::c_int;
    fn sqlite3Get4byte(_: *const u8_0) -> u32_0;
    fn sqlite3Put4byte(_: *mut u8_0, _: u32_0);
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
pub struct Btree {
    pub db: *mut sqlite3,
    pub pBt: *mut BtShared,
    pub inTrans: u8_0,
    pub sharable: u8_0,
    pub locked: u8_0,
    pub hasIncrblobCur: u8_0,
    pub wantToLock: ::core::ffi::c_int,
    pub nBackup: ::core::ffi::c_int,
    pub iBDataVersion: u32_0,
    pub pNext: *mut Btree,
    pub pPrev: *mut Btree

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BtShared {
    pub pPager: *mut Pager,
    pub db: *mut sqlite3,
    pub pCursor: *mut BtCursor,
    pub pPage1: *mut MemPage,
    pub openFlags: u8_0,
    pub autoVacuum: u8_0,
    pub incrVacuum: u8_0,
    pub bDoTruncate: u8_0,
    pub inTransaction: u8_0,
    pub max1bytePayload: u8_0,
    pub nReserveWanted: u8_0,
    pub btsFlags: u16_0,
    pub maxLocal: u16_0,
    pub minLocal: u16_0,
    pub maxLeaf: u16_0,
    pub minLeaf: u16_0,
    pub pageSize: u32_0,
    pub usableSize: u32_0,
    pub nTransaction: ::core::ffi::c_int,
    pub nPage: u32_0,
    pub pSchema: *mut ::core::ffi::c_void,
    pub xFreeSchema: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub mutex: *mut sqlite3_mutex,
    pub pHasContent: *mut Bitvec,
    pub pTmpSpace: *mut u8_0,
    pub nPreformatSize: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemPage {
    pub isInit: u8_0,
    pub intKey: u8_0,
    pub intKeyLeaf: u8_0,
    pub pgno: Pgno,
    pub leaf: u8_0,
    pub hdrOffset: u8_0,
    pub childPtrSize: u8_0,
    pub max1bytePayload: u8_0,
    pub nOverflow: u8_0,
    pub maxLocal: u16_0,
    pub minLocal: u16_0,
    pub cellOffset: u16_0,
    pub nFree: ::core::ffi::c_int,
    pub nCell: u16_0,
    pub maskPage: u16_0,
    pub aiOvfl: [u16_0; 4],
    pub apOvfl: [*mut u8_0; 4],
    pub pBt: *mut BtShared,
    pub aData: *mut u8_0,
    pub aDataEnd: *mut u8_0,
    pub aCellIdx: *mut u8_0,
    pub aDataOfst: *mut u8_0,
    pub pDbPage: *mut DbPage,
    pub xCellSize: Option<unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0>,
    pub xParseCell: Option<
        unsafe extern "C" fn(*mut MemPage, *mut u8_0, *mut CellInfo) -> ()
    >

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CellInfo {
    pub nKey: i64_0,
    pub pPayload: *mut u8_0,
    pub nPayload: u32_0,
    pub nLocal: u16_0,
    pub nSize: u16_0

}
pub type DbPage = PgHdr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PgHdr {
    pub pPage: *mut sqlite3_pcache_page,
    pub pData: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void,
    pub pCache: *mut PCache,
    pub pDirty: *mut PgHdr,
    pub pPager: *mut Pager,
    pub pgno: Pgno,
    pub flags: u16_0,
    pub nRef: i64_0,
    pub pDirtyNext: *mut PgHdr,
    pub pDirtyPrev: *mut PgHdr

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BtCursor {
    pub eState: u8_0,
    pub curFlags: u8_0,
    pub curPagerFlags: u8_0,
    pub hints: u8_0,
    pub skipNext: ::core::ffi::c_int,
    pub pBtree: *mut Btree,
    pub aOverflow: *mut Pgno,
    pub pKey: *mut ::core::ffi::c_void,
    pub pBt: *mut BtShared,
    pub pNext: *mut BtCursor,
    pub info: CellInfo,
    pub nKey: i64_0,
    pub pgnoRoot: Pgno,
    pub iPage: i8_0,
    pub curIntKey: u8_0,
    pub ix: u16_0,
    pub aiIdx: [u16_0; 19],
    pub pKeyInfo: *mut KeyInfo,
    pub pPage: *mut MemPage,
    pub apPage: [*mut MemPage; 19]

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
pub type i8_0 = ::core::ffi::c_schar;
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
pub type uptr = u64_0;
pub type StrAccum = sqlite3_str;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnpackedRecord {
    pub pKeyInfo: *mut KeyInfo,
    pub aMem: *mut Mem,
    pub u: C2Rust_Unnamed_22,
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
pub union C2Rust_Unnamed_22 {
    pub z: *mut ::core::ffi::c_char,
    pub i: i64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BtreePayload {
    pub pKey: *const ::core::ffi::c_void,
    pub nKey: sqlite3_int64,
    pub pData: *const ::core::ffi::c_void,
    pub aMem: *mut sqlite3_value,
    pub nMem: u16_0,
    pub nData: ::core::ffi::c_int,
    pub nZero: ::core::ffi::c_int

}
pub type RecordCompare = Option<
    unsafe extern "C" fn(
        ::core::ffi::c_int,
        *const ::core::ffi::c_void,
        *mut UnpackedRecord,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CellArray {
    pub nCell: ::core::ffi::c_int,
    pub pRef: *mut MemPage,
    pub apCell: *mut *mut u8_0,
    pub szCell: *mut u16_0,
    pub apEnd: [*mut u8_0; 6],
    pub ixNx: [::core::ffi::c_int; 6]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IntegrityCk {
    pub pBt: *mut BtShared,
    pub pPager: *mut Pager,
    pub aPgRef: *mut u8_0,
    pub nCkPage: Pgno,
    pub mxErr: ::core::ffi::c_int,
    pub nErr: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub nStep: u32_0,
    pub zPfx: *const ::core::ffi::c_char,
    pub v0: Pgno,
    pub v1: Pgno,
    pub v2: ::core::ffi::c_int,
    pub errMsg: StrAccum,
    pub heap: *mut u32_0,
    pub db: *mut sqlite3,
    pub nRow: i64_0

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ABORT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_LOCKED: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_READONLY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_INTERRUPT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_EMPTY: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_NOTADB: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_BUSY_SNAPSHOT: ::core::ffi::c_int = SQLITE_BUSY
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_PINNED: ::core::ffi::c_int = SQLITE_CONSTRAINT
    | (11 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MEMORY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_OPEN_TEMP_DB: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_PDB: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
static mut zMagicHeader: [::core::ffi::c_char; 16] = SQLITE_FILE_HEADER;
pub const BTALLOC_ANY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BTALLOC_EXACT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTALLOC_LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn invalidateAllOverflowCache(mut pBt: *mut BtShared) {
    unsafe {
        let mut p: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        p = (*pBt).pCursor;
        while !p.is_null() {
            (*p).curFlags = ((*p).curFlags as ::core::ffi::c_int & !BTCF_ValidOvfl)
                as u8_0;
            p = (*p).pNext;
        }
    }
}
unsafe extern "C" fn invalidateIncrblobCursors(
    mut pBtree: *mut Btree,
    mut pgnoRoot: Pgno,
    mut iRow: i64_0,
    mut isClearTable: ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        (*pBtree).hasIncrblobCur = 0 as u8_0;
        p = (*(*pBtree).pBt).pCursor;
        while !p.is_null() {
            if (*p).curFlags as ::core::ffi::c_int & BTCF_Incrblob
                != 0 as ::core::ffi::c_int
            {
                (*pBtree).hasIncrblobCur = 1 as u8_0;
                if (*p).pgnoRoot == pgnoRoot
                    && (isClearTable != 0 || (*p).info.nKey == iRow)
                {
                    (*p).eState = CURSOR_INVALID as u8_0;
                }
            }
            p = (*p).pNext;
        }
    }
}
unsafe extern "C" fn btreeSetHasContent(
    mut pBt: *mut BtShared,
    mut pgno: Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pBt).pHasContent.is_null() {
            (*pBt).pHasContent = sqlite3BitvecCreate((*pBt).nPage);
            if (*pBt).pHasContent.is_null() {
                rc = SQLITE_NOMEM_BKPT;
            }
        }
        if rc == SQLITE_OK && pgno <= sqlite3BitvecSize((*pBt).pHasContent) {
            rc = sqlite3BitvecSet((*pBt).pHasContent, pgno as u32_0);
        }
        return rc;
    }
}
unsafe extern "C" fn btreeGetHasContent(
    mut pBt: *mut BtShared,
    mut pgno: Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Bitvec = (*pBt).pHasContent;
        return (!p.is_null()
            && (pgno > sqlite3BitvecSize(p)
                || sqlite3BitvecTestNotNull(p, pgno as u32_0) != 0))
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn btreeClearHasContent(mut pBt: *mut BtShared) {
    unsafe {
        sqlite3BitvecDestroy((*pBt).pHasContent);
        (*pBt).pHasContent = ::core::ptr::null_mut::<Bitvec>();
    }
}
unsafe extern "C" fn btreeReleaseAllCursorPages(mut pCur: *mut BtCursor) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if (*pCur).iPage as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            i = 0 as ::core::ffi::c_int;
            while i < (*pCur).iPage as ::core::ffi::c_int {
                releasePageNotNull((*pCur).apPage[i as usize]);
                i += 1;
            }
            releasePageNotNull((*pCur).pPage);
            (*pCur).iPage = -1 as ::core::ffi::c_int as i8_0;
        }
    }
}
unsafe extern "C" fn saveCursorKey(mut pCur: *mut BtCursor) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pCur).curIntKey != 0 {
            (*pCur).nKey = sqlite3BtreeIntegerKey(pCur);
        } else {
            let mut pKey: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            (*pCur).nKey = sqlite3BtreePayloadSize(pCur) as i64_0;
            pKey = sqlite3Malloc(
                ((*pCur).nKey + 9 as ::core::ffi::c_longlong
                    + 8 as ::core::ffi::c_longlong) as u64_0,
            );
            if !pKey.is_null() {
                rc = sqlite3BtreePayload(
                    pCur,
                    0 as u32_0,
                    (*pCur).nKey as ::core::ffi::c_int as u32_0,
                    pKey,
                );
                if rc == SQLITE_OK {
                    memset(
                        (pKey as *mut u8_0).offset((*pCur).nKey as isize)
                            as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as size_t,
                    );
                    (*pCur).pKey = pKey;
                } else {
                    sqlite3_free(pKey);
                }
            } else {
                rc = SQLITE_NOMEM_BKPT;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn saveCursorPosition(mut pCur: *mut BtCursor) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if (*pCur).curFlags as ::core::ffi::c_int & BTCF_Pinned != 0 {
            return SQLITE_CONSTRAINT_PINNED;
        }
        if (*pCur).eState as ::core::ffi::c_int == CURSOR_SKIPNEXT {
            (*pCur).eState = CURSOR_VALID as u8_0;
        } else {
            (*pCur).skipNext = 0 as ::core::ffi::c_int;
        }
        rc = saveCursorKey(pCur);
        if rc == SQLITE_OK {
            btreeReleaseAllCursorPages(pCur);
            (*pCur).eState = CURSOR_REQUIRESEEK as u8_0;
        }
        (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
            & !(BTCF_ValidNKey | BTCF_ValidOvfl | BTCF_AtLast)) as u8_0;
        return rc;
    }
}
unsafe extern "C" fn saveAllCursors(
    mut pBt: *mut BtShared,
    mut iRoot: Pgno,
    mut pExcept: *mut BtCursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        p = (*pBt).pCursor;
        while !p.is_null() {
            if p != pExcept
                && (0 as ::core::ffi::c_uint == iRoot || (*p).pgnoRoot == iRoot)
            {
                break;
            }
            p = (*p).pNext;
        }
        if !p.is_null() {
            return saveCursorsOnList(p, iRoot, pExcept);
        }
        if !pExcept.is_null() {
            (*pExcept).curFlags = ((*pExcept).curFlags as ::core::ffi::c_int
                & !BTCF_Multiple) as u8_0;
        }
        return SQLITE_OK;
    }
}
#[inline(never)]
unsafe extern "C" fn saveCursorsOnList(
    mut p: *mut BtCursor,
    mut iRoot: Pgno,
    mut pExcept: *mut BtCursor,
) -> ::core::ffi::c_int {
    unsafe {
        loop {
            if p != pExcept
                && (0 as ::core::ffi::c_uint == iRoot || (*p).pgnoRoot == iRoot)
            {
                if (*p).eState as ::core::ffi::c_int == CURSOR_VALID
                    || (*p).eState as ::core::ffi::c_int == CURSOR_SKIPNEXT
                {
                    let mut rc: ::core::ffi::c_int = saveCursorPosition(p);
                    if SQLITE_OK != rc {
                        return rc;
                    }
                } else {
                    btreeReleaseAllCursorPages(p);
                }
            }
            p = (*p).pNext;
            if p.is_null() {
                break;
            }
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeClearCursor(mut pCur: *mut BtCursor) {
    unsafe {
        sqlite3_free((*pCur).pKey);
        (*pCur).pKey = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*pCur).eState = CURSOR_INVALID as u8_0;
    }
}
unsafe extern "C" fn btreeMoveto(
    mut pCur: *mut BtCursor,
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: i64_0,
    mut bias: ::core::ffi::c_int,
    mut pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pIdxKey: *mut UnpackedRecord = ::core::ptr::null_mut::<UnpackedRecord>();
        if !pKey.is_null() {
            let mut pKeyInfo: *mut KeyInfo = (*pCur).pKeyInfo as *mut KeyInfo;
            pIdxKey = sqlite3VdbeAllocUnpackedRecord(pKeyInfo);
            if pIdxKey.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            sqlite3VdbeRecordUnpack(nKey as ::core::ffi::c_int, pKey, pIdxKey);
            if (*pIdxKey).nField as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*pIdxKey).nField as ::core::ffi::c_int
                    > (*pKeyInfo).nAllField as ::core::ffi::c_int
            {
                rc = sqlite3CorruptError(877 as ::core::ffi::c_int);
            } else {
                rc = sqlite3BtreeIndexMoveto(pCur, pIdxKey, pRes);
            }
            sqlite3DbFree((*(*pCur).pKeyInfo).db, pIdxKey as *mut ::core::ffi::c_void);
        } else {
            pIdxKey = ::core::ptr::null_mut::<UnpackedRecord>();
            rc = sqlite3BtreeTableMoveto(pCur, nKey, bias, pRes);
        }
        return rc;
    }
}
unsafe extern "C" fn btreeRestoreCursorPosition(
    mut pCur: *mut BtCursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut skipNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*pCur).eState as ::core::ffi::c_int == CURSOR_FAULT {
            return (*pCur).skipNext;
        }
        (*pCur).eState = CURSOR_INVALID as u8_0;
        if sqlite3FaultSim(410 as ::core::ffi::c_int) != 0 {
            rc = SQLITE_IOERR;
        } else {
            rc = btreeMoveto(
                pCur,
                (*pCur).pKey,
                (*pCur).nKey,
                0 as ::core::ffi::c_int,
                &raw mut skipNext,
            );
        }
        if rc == SQLITE_OK {
            sqlite3_free((*pCur).pKey);
            (*pCur).pKey = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if skipNext != 0 {
                (*pCur).skipNext = skipNext;
            }
            if (*pCur).skipNext != 0
                && (*pCur).eState as ::core::ffi::c_int == CURSOR_VALID
            {
                (*pCur).eState = CURSOR_SKIPNEXT as u8_0;
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCursorHasMoved(
    mut pCur: *mut BtCursor,
) -> ::core::ffi::c_int {
    unsafe {
        return (CURSOR_VALID != *(pCur as *mut u8_0) as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeFakeValidCursor() -> *mut BtCursor {
    unsafe {
        static mut fakeCursor: u8_0 = CURSOR_VALID as u8_0;
        return &raw mut fakeCursor as *mut BtCursor;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCursorRestore(
    mut pCur: *mut BtCursor,
    mut pDifferentRow: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = if (*pCur).eState as ::core::ffi::c_int >= CURSOR_REQUIRESEEK {
            btreeRestoreCursorPosition(pCur)
        } else {
            SQLITE_OK
        };
        if rc != 0 {
            *pDifferentRow = 1 as ::core::ffi::c_int;
            return rc;
        }
        if (*pCur).eState as ::core::ffi::c_int != CURSOR_VALID {
            *pDifferentRow = 1 as ::core::ffi::c_int;
        } else {
            *pDifferentRow = 0 as ::core::ffi::c_int;
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCursorHintFlags(
    mut pCur: *mut BtCursor,
    mut x: ::core::ffi::c_uint,
) {
    unsafe {
        (*pCur).hints = x as u8_0;
    }
}
unsafe extern "C" fn ptrmapPageno(mut pBt: *mut BtShared, mut pgno: Pgno) -> Pgno {
    unsafe {
        let mut nPagesPerMapPage: ::core::ffi::c_int = 0;
        let mut iPtrMap: Pgno = 0;
        let mut ret: Pgno = 0;
        if pgno < 2 as ::core::ffi::c_uint {
            return 0 as Pgno;
        }
        nPagesPerMapPage = ((*pBt).usableSize as ::core::ffi::c_uint)
            .wrapping_div(5 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        iPtrMap = (pgno as ::core::ffi::c_uint)
            .wrapping_sub(2 as ::core::ffi::c_uint)
            .wrapping_div(nPagesPerMapPage as ::core::ffi::c_uint) as Pgno;
        ret = (iPtrMap as ::core::ffi::c_uint)
            .wrapping_mul(nPagesPerMapPage as ::core::ffi::c_uint)
            .wrapping_add(2 as ::core::ffi::c_uint) as Pgno;
        if ret
            == (sqlite3PendingByte as ::core::ffi::c_uint)
                .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint)
        {
            ret = ret.wrapping_add(1);
        }
        return ret;
    }
}
unsafe extern "C" fn ptrmapPut(
    mut pBt: *mut BtShared,
    mut key: Pgno,
    mut eType: u8_0,
    mut parent: Pgno,
    mut pRC: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut pDbPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
        let mut pPtrmap: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut iPtrmap: Pgno = 0;
        let mut offset: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if *pRC != 0 {
            return;
        }
        if key == 0 as ::core::ffi::c_uint {
            *pRC = sqlite3CorruptError(1075 as ::core::ffi::c_int);
            return;
        }
        iPtrmap = ptrmapPageno(pBt, key);
        rc = sqlite3PagerGet(
            (*pBt).pPager,
            iPtrmap,
            &raw mut pDbPage,
            0 as ::core::ffi::c_int,
        );
        if rc != SQLITE_OK {
            *pRC = rc;
            return;
        }
        if *(sqlite3PagerGetExtra(pDbPage) as *mut ::core::ffi::c_char)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            *pRC = sqlite3CorruptError(1088 as ::core::ffi::c_int);
        } else {
            offset = (5 as ::core::ffi::c_uint)
                .wrapping_mul(
                    (key as ::core::ffi::c_uint)
                        .wrapping_sub(iPtrmap as ::core::ffi::c_uint)
                        .wrapping_sub(1 as ::core::ffi::c_uint),
                ) as ::core::ffi::c_int;
            if offset < 0 as ::core::ffi::c_int {
                *pRC = sqlite3CorruptError(1093 as ::core::ffi::c_int);
            } else {
                pPtrmap = sqlite3PagerGetData(pDbPage) as *mut u8_0;
                if eType as ::core::ffi::c_int
                    != *pPtrmap.offset(offset as isize) as ::core::ffi::c_int
                    || sqlite3Get4byte(
                        pPtrmap.offset((offset + 1 as ::core::ffi::c_int) as isize)
                            as *mut u8_0,
                    ) != parent
                {
                    rc = sqlite3PagerWrite(pDbPage);
                    *pRC = rc;
                    if rc == SQLITE_OK {
                        *pPtrmap.offset(offset as isize) = eType;
                        sqlite3Put4byte(
                            pPtrmap.offset((offset + 1 as ::core::ffi::c_int) as isize)
                                as *mut u8_0,
                            parent as u32_0,
                        );
                    }
                }
            }
        }
        sqlite3PagerUnref(pDbPage);
    }
}
unsafe extern "C" fn ptrmapGet(
    mut pBt: *mut BtShared,
    mut key: Pgno,
    mut pEType: *mut u8_0,
    mut pPgno: *mut Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pDbPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
        let mut iPtrmap: ::core::ffi::c_int = 0;
        let mut pPtrmap: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut offset: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        iPtrmap = ptrmapPageno(pBt, key) as ::core::ffi::c_int;
        rc = sqlite3PagerGet(
            (*pBt).pPager,
            iPtrmap as Pgno,
            &raw mut pDbPage,
            0 as ::core::ffi::c_int,
        );
        if rc != 0 as ::core::ffi::c_int {
            return rc;
        }
        pPtrmap = sqlite3PagerGetData(pDbPage) as *mut u8_0;
        offset = (5 as ::core::ffi::c_uint)
            .wrapping_mul(
                (key as ::core::ffi::c_uint)
                    .wrapping_sub(iPtrmap as ::core::ffi::c_uint)
                    .wrapping_sub(1 as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int;
        if offset < 0 as ::core::ffi::c_int {
            sqlite3PagerUnref(pDbPage);
            return sqlite3CorruptError(1138 as ::core::ffi::c_int);
        }
        *pEType = *pPtrmap.offset(offset as isize);
        if !pPgno.is_null() {
            *pPgno = sqlite3Get4byte(
                pPtrmap.offset((offset + 1 as ::core::ffi::c_int) as isize) as *mut u8_0,
            ) as Pgno;
        }
        sqlite3PagerUnref(pDbPage);
        if (*pEType as ::core::ffi::c_int) < 1 as ::core::ffi::c_int
            || *pEType as ::core::ffi::c_int > 5 as ::core::ffi::c_int
        {
            return sqlite3CorruptError(1146 as ::core::ffi::c_int);
        }
        return SQLITE_OK;
    }
}
#[inline(never)]
unsafe extern "C" fn btreeParseCellAdjustSizeForOverflow(
    mut pPage: *mut MemPage,
    mut pCell: *mut u8_0,
    mut pInfo: *mut CellInfo,
) {
    unsafe {
        let mut minLocal: ::core::ffi::c_int = 0;
        let mut maxLocal: ::core::ffi::c_int = 0;
        let mut surplus: ::core::ffi::c_int = 0;
        minLocal = (*pPage).minLocal as ::core::ffi::c_int;
        maxLocal = (*pPage).maxLocal as ::core::ffi::c_int;
        surplus = (minLocal as ::core::ffi::c_uint)
            .wrapping_add(
                ((*pInfo).nPayload as ::core::ffi::c_uint)
                    .wrapping_sub(minLocal as ::core::ffi::c_uint)
                    .wrapping_rem(
                        ((*(*pPage).pBt).usableSize as ::core::ffi::c_uint)
                            .wrapping_sub(4 as ::core::ffi::c_uint),
                    ),
            ) as ::core::ffi::c_int;
        if surplus <= maxLocal {
            (*pInfo).nLocal = surplus as u16_0;
        } else {
            (*pInfo).nLocal = minLocal as u16_0;
        }
        (*pInfo).nSize = (((*pInfo).pPayload.offset((*pInfo).nLocal as isize)
            as *mut u8_0)
            .offset_from(pCell) as ::core::ffi::c_long as u16_0 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int) as u16_0;
    }
}
unsafe extern "C" fn btreePayloadToLocal(
    mut pPage: *mut MemPage,
    mut nPayload: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut maxLocal: ::core::ffi::c_int = 0;
        maxLocal = (*pPage).maxLocal as ::core::ffi::c_int;
        if nPayload <= maxLocal as ::core::ffi::c_longlong {
            return nPayload as ::core::ffi::c_int
        } else {
            let mut minLocal: ::core::ffi::c_int = 0;
            let mut surplus: ::core::ffi::c_int = 0;
            minLocal = (*pPage).minLocal as ::core::ffi::c_int;
            surplus = (minLocal as ::core::ffi::c_longlong
                + (nPayload as ::core::ffi::c_longlong
                    - minLocal as ::core::ffi::c_longlong)
                    % ((*(*pPage).pBt).usableSize as ::core::ffi::c_uint)
                        .wrapping_sub(4 as ::core::ffi::c_uint)
                        as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            return if surplus <= maxLocal { surplus } else { minLocal };
        };
    }
}
unsafe extern "C" fn btreeParseCellPtrNoPayload(
    mut pPage: *mut MemPage,
    mut pCell: *mut u8_0,
    mut pInfo: *mut CellInfo,
) {
    unsafe {
        (*pInfo).nSize = (4 as ::core::ffi::c_int
            + sqlite3GetVarint(
                pCell.offset(4 as ::core::ffi::c_int as isize) as *mut u8_0,
                &raw mut (*pInfo).nKey as *mut u64_0,
            ) as ::core::ffi::c_int) as u16_0;
        (*pInfo).nPayload = 0 as u32_0;
        (*pInfo).nLocal = 0 as u16_0;
        (*pInfo).pPayload = ::core::ptr::null_mut::<u8_0>();
    }
}
unsafe extern "C" fn btreeParseCellPtr(
    mut pPage: *mut MemPage,
    mut pCell: *mut u8_0,
    mut pInfo: *mut CellInfo,
) {
    unsafe {
        let mut pIter: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut nPayload: u32_0 = 0;
        let mut iKey: u64_0 = 0;
        pIter = pCell;
        nPayload = *pIter as u32_0;
        if nPayload >= 0x80 as ::core::ffi::c_uint {
            let mut pEnd: *mut u8_0 = pIter.offset(8 as ::core::ffi::c_int as isize)
                as *mut u8_0;
            nPayload &= 0x7f as ::core::ffi::c_uint;
            loop {
                pIter = pIter.offset(1);
                nPayload = ((nPayload as ::core::ffi::c_uint) << 7 as ::core::ffi::c_int
                    | (*pIter as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                        as ::core::ffi::c_uint) as u32_0;
                if !(*pIter as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int
                    && pIter < pEnd)
                {
                    break;
                }
            }
        }
        pIter = pIter.offset(1);
        iKey = *pIter as u64_0;
        if iKey >= 0x80 as ::core::ffi::c_ulonglong {
            let mut x: u8_0 = 0;
            pIter = pIter.offset(1);
            x = *pIter;
            iKey = ((iKey as ::core::ffi::c_ulonglong) << 7 as ::core::ffi::c_int
                ^ x as ::core::ffi::c_ulonglong) as u64_0;
            if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                pIter = pIter.offset(1);
                x = *pIter;
                iKey = ((iKey as ::core::ffi::c_ulonglong) << 7 as ::core::ffi::c_int
                    ^ x as ::core::ffi::c_ulonglong) as u64_0;
                if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                    pIter = pIter.offset(1);
                    x = *pIter;
                    iKey = ((iKey as ::core::ffi::c_ulonglong) << 7 as ::core::ffi::c_int
                        ^ 0x10204000 as ::core::ffi::c_ulonglong
                        ^ x as ::core::ffi::c_ulonglong) as u64_0;
                    if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                        pIter = pIter.offset(1);
                        x = *pIter;
                        iKey = ((iKey as ::core::ffi::c_ulonglong)
                            << 7 as ::core::ffi::c_int
                            ^ 0x4000 as ::core::ffi::c_ulonglong
                            ^ x as ::core::ffi::c_ulonglong) as u64_0;
                        if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                            pIter = pIter.offset(1);
                            x = *pIter;
                            iKey = ((iKey as ::core::ffi::c_ulonglong)
                                << 7 as ::core::ffi::c_int
                                ^ 0x4000 as ::core::ffi::c_ulonglong
                                ^ x as ::core::ffi::c_ulonglong) as u64_0;
                            if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                                pIter = pIter.offset(1);
                                x = *pIter;
                                iKey = ((iKey as ::core::ffi::c_ulonglong)
                                    << 7 as ::core::ffi::c_int
                                    ^ 0x4000 as ::core::ffi::c_ulonglong
                                    ^ x as ::core::ffi::c_ulonglong) as u64_0;
                                if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                                    pIter = pIter.offset(1);
                                    x = *pIter;
                                    iKey = ((iKey as ::core::ffi::c_ulonglong)
                                        << 7 as ::core::ffi::c_int
                                        ^ 0x4000 as ::core::ffi::c_ulonglong
                                        ^ x as ::core::ffi::c_ulonglong) as u64_0;
                                    if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                                        pIter = pIter.offset(1);
                                        iKey = ((iKey as ::core::ffi::c_ulonglong)
                                            << 8 as ::core::ffi::c_int
                                            ^ 0x8000 as ::core::ffi::c_ulonglong
                                            ^ *pIter as ::core::ffi::c_ulonglong) as u64_0;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    iKey ^= 0x204000 as ::core::ffi::c_ulonglong;
                }
            } else {
                iKey ^= 0x4000 as ::core::ffi::c_ulonglong;
            }
        }
        pIter = pIter.offset(1);
        (*pInfo).nKey = *(&raw mut iKey as *mut i64_0);
        (*pInfo).nPayload = nPayload;
        (*pInfo).pPayload = pIter;
        if nPayload <= (*pPage).maxLocal as ::core::ffi::c_uint {
            (*pInfo).nSize = (nPayload as u16_0 as ::core::ffi::c_int
                + pIter.offset_from(pCell) as ::core::ffi::c_long as u16_0
                    as ::core::ffi::c_int) as u16_0;
            if ((*pInfo).nSize as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
                (*pInfo).nSize = 4 as u16_0;
            }
            (*pInfo).nLocal = nPayload as u16_0;
        } else {
            btreeParseCellAdjustSizeForOverflow(pPage, pCell, pInfo);
        };
    }
}
unsafe extern "C" fn btreeParseCellPtrIndex(
    mut pPage: *mut MemPage,
    mut pCell: *mut u8_0,
    mut pInfo: *mut CellInfo,
) {
    unsafe {
        let mut pIter: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut nPayload: u32_0 = 0;
        pIter = pCell.offset((*pPage).childPtrSize as ::core::ffi::c_int as isize);
        nPayload = *pIter as u32_0;
        if nPayload >= 0x80 as ::core::ffi::c_uint {
            let mut pEnd: *mut u8_0 = pIter.offset(8 as ::core::ffi::c_int as isize)
                as *mut u8_0;
            nPayload &= 0x7f as ::core::ffi::c_uint;
            loop {
                pIter = pIter.offset(1);
                nPayload = ((nPayload as ::core::ffi::c_uint) << 7 as ::core::ffi::c_int
                    | (*pIter as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                        as ::core::ffi::c_uint) as u32_0;
                if !(*pIter as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int
                    && pIter < pEnd)
                {
                    break;
                }
            }
        }
        pIter = pIter.offset(1);
        (*pInfo).nKey = nPayload as i64_0;
        (*pInfo).nPayload = nPayload;
        (*pInfo).pPayload = pIter;
        if nPayload <= (*pPage).maxLocal as ::core::ffi::c_uint {
            (*pInfo).nSize = (nPayload as u16_0 as ::core::ffi::c_int
                + pIter.offset_from(pCell) as ::core::ffi::c_long as u16_0
                    as ::core::ffi::c_int) as u16_0;
            if ((*pInfo).nSize as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
                (*pInfo).nSize = 4 as u16_0;
            }
            (*pInfo).nLocal = nPayload as u16_0;
        } else {
            btreeParseCellAdjustSizeForOverflow(pPage, pCell, pInfo);
        };
    }
}
unsafe extern "C" fn btreeParseCell(
    mut pPage: *mut MemPage,
    mut iCell: ::core::ffi::c_int,
    mut pInfo: *mut CellInfo,
) {
    unsafe {
        (*pPage)
            .xParseCell
            .expect(
                "non-null function pointer",
            )(
            pPage,
            (*pPage)
                .aData
                .offset(
                    ((*pPage).maskPage as ::core::ffi::c_int
                        & ((*((*pPage)
                            .aCellIdx
                            .offset((2 as ::core::ffi::c_int * iCell) as isize)
                            as *mut u8_0)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                            | *((*pPage)
                                .aCellIdx
                                .offset((2 as ::core::ffi::c_int * iCell) as isize)
                                as *mut u8_0)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)) as isize,
                ),
            pInfo,
        );
    }
}
unsafe extern "C" fn cellSizePtr(
    mut pPage: *mut MemPage,
    mut pCell: *mut u8_0,
) -> u16_0 {
    unsafe {
        let mut pIter: *mut u8_0 = pCell.offset(4 as ::core::ffi::c_int as isize);
        let mut pEnd: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut nSize: u32_0 = 0;
        nSize = *pIter as u32_0;
        if nSize >= 0x80 as ::core::ffi::c_uint {
            pEnd = pIter.offset(8 as ::core::ffi::c_int as isize) as *mut u8_0;
            nSize &= 0x7f as ::core::ffi::c_uint;
            loop {
                pIter = pIter.offset(1);
                nSize = ((nSize as ::core::ffi::c_uint) << 7 as ::core::ffi::c_int
                    | (*pIter as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                        as ::core::ffi::c_uint) as u32_0;
                if !(*pIter as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int
                    && pIter < pEnd)
                {
                    break;
                }
            }
        }
        pIter = pIter.offset(1);
        if nSize <= (*pPage).maxLocal as ::core::ffi::c_uint {
            nSize = (nSize as ::core::ffi::c_uint)
                .wrapping_add(
                    pIter.offset_from(pCell) as ::core::ffi::c_long as u32_0
                        as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
        } else {
            let mut minLocal: ::core::ffi::c_int = (*pPage).minLocal
                as ::core::ffi::c_int;
            nSize = (minLocal as ::core::ffi::c_uint)
                .wrapping_add(
                    (nSize as ::core::ffi::c_uint)
                        .wrapping_sub(minLocal as ::core::ffi::c_uint)
                        .wrapping_rem(
                            ((*(*pPage).pBt).usableSize as ::core::ffi::c_uint)
                                .wrapping_sub(4 as ::core::ffi::c_uint),
                        ),
                ) as u32_0;
            if nSize > (*pPage).maxLocal as ::core::ffi::c_uint {
                nSize = minLocal as u32_0;
            }
            nSize = (nSize as ::core::ffi::c_uint)
                .wrapping_add(
                    (4 as ::core::ffi::c_int
                        + pIter.offset_from(pCell) as ::core::ffi::c_long as u16_0
                            as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
        }
        return nSize as u16_0;
    }
}
unsafe extern "C" fn cellSizePtrIdxLeaf(
    mut pPage: *mut MemPage,
    mut pCell: *mut u8_0,
) -> u16_0 {
    unsafe {
        let mut pIter: *mut u8_0 = pCell;
        let mut pEnd: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut nSize: u32_0 = 0;
        nSize = *pIter as u32_0;
        if nSize >= 0x80 as ::core::ffi::c_uint {
            pEnd = pIter.offset(8 as ::core::ffi::c_int as isize) as *mut u8_0;
            nSize &= 0x7f as ::core::ffi::c_uint;
            loop {
                pIter = pIter.offset(1);
                nSize = ((nSize as ::core::ffi::c_uint) << 7 as ::core::ffi::c_int
                    | (*pIter as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                        as ::core::ffi::c_uint) as u32_0;
                if !(*pIter as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int
                    && pIter < pEnd)
                {
                    break;
                }
            }
        }
        pIter = pIter.offset(1);
        if nSize <= (*pPage).maxLocal as ::core::ffi::c_uint {
            nSize = (nSize as ::core::ffi::c_uint)
                .wrapping_add(
                    pIter.offset_from(pCell) as ::core::ffi::c_long as u32_0
                        as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            if nSize < 4 as ::core::ffi::c_uint {
                nSize = 4 as u32_0;
            }
        } else {
            let mut minLocal: ::core::ffi::c_int = (*pPage).minLocal
                as ::core::ffi::c_int;
            nSize = (minLocal as ::core::ffi::c_uint)
                .wrapping_add(
                    (nSize as ::core::ffi::c_uint)
                        .wrapping_sub(minLocal as ::core::ffi::c_uint)
                        .wrapping_rem(
                            ((*(*pPage).pBt).usableSize as ::core::ffi::c_uint)
                                .wrapping_sub(4 as ::core::ffi::c_uint),
                        ),
                ) as u32_0;
            if nSize > (*pPage).maxLocal as ::core::ffi::c_uint {
                nSize = minLocal as u32_0;
            }
            nSize = (nSize as ::core::ffi::c_uint)
                .wrapping_add(
                    (4 as ::core::ffi::c_int
                        + pIter.offset_from(pCell) as ::core::ffi::c_long as u16_0
                            as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
        }
        return nSize as u16_0;
    }
}
unsafe extern "C" fn cellSizePtrNoPayload(
    mut pPage: *mut MemPage,
    mut pCell: *mut u8_0,
) -> u16_0 {
    unsafe {
        let mut pIter: *mut u8_0 = pCell.offset(4 as ::core::ffi::c_int as isize);
        let mut pEnd: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        pEnd = pIter.offset(9 as ::core::ffi::c_int as isize);
        loop {
            let c2rust_fresh0 = pIter;
            pIter = pIter.offset(1);
            if !(*c2rust_fresh0 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
                && pIter < pEnd)
            {
                break;
            }
        }
        return pIter.offset_from(pCell) as ::core::ffi::c_long as u16_0;
    }
}
unsafe extern "C" fn cellSizePtrTableLeaf(
    mut pPage: *mut MemPage,
    mut pCell: *mut u8_0,
) -> u16_0 {
    unsafe {
        let mut pIter: *mut u8_0 = pCell;
        let mut pEnd: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut nSize: u32_0 = 0;
        nSize = *pIter as u32_0;
        if nSize >= 0x80 as ::core::ffi::c_uint {
            pEnd = pIter.offset(8 as ::core::ffi::c_int as isize) as *mut u8_0;
            nSize &= 0x7f as ::core::ffi::c_uint;
            loop {
                pIter = pIter.offset(1);
                nSize = ((nSize as ::core::ffi::c_uint) << 7 as ::core::ffi::c_int
                    | (*pIter as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                        as ::core::ffi::c_uint) as u32_0;
                if !(*pIter as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int
                    && pIter < pEnd)
                {
                    break;
                }
            }
        }
        pIter = pIter.offset(1);
        let c2rust_fresh1 = pIter;
        pIter = pIter.offset(1);
        if *c2rust_fresh1 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
            && {
                let c2rust_fresh2 = pIter;
                pIter = pIter.offset(1);
                *c2rust_fresh2 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
            }
            && {
                let c2rust_fresh3 = pIter;
                pIter = pIter.offset(1);
                *c2rust_fresh3 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
            }
            && {
                let c2rust_fresh4 = pIter;
                pIter = pIter.offset(1);
                *c2rust_fresh4 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
            }
            && {
                let c2rust_fresh5 = pIter;
                pIter = pIter.offset(1);
                *c2rust_fresh5 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
            }
            && {
                let c2rust_fresh6 = pIter;
                pIter = pIter.offset(1);
                *c2rust_fresh6 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
            }
            && {
                let c2rust_fresh7 = pIter;
                pIter = pIter.offset(1);
                *c2rust_fresh7 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
            }
            && {
                let c2rust_fresh8 = pIter;
                pIter = pIter.offset(1);
                *c2rust_fresh8 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
            }
        {
            pIter = pIter.offset(1);
        }
        if nSize <= (*pPage).maxLocal as ::core::ffi::c_uint {
            nSize = (nSize as ::core::ffi::c_uint)
                .wrapping_add(
                    pIter.offset_from(pCell) as ::core::ffi::c_long as u32_0
                        as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            if nSize < 4 as ::core::ffi::c_uint {
                nSize = 4 as u32_0;
            }
        } else {
            let mut minLocal: ::core::ffi::c_int = (*pPage).minLocal
                as ::core::ffi::c_int;
            nSize = (minLocal as ::core::ffi::c_uint)
                .wrapping_add(
                    (nSize as ::core::ffi::c_uint)
                        .wrapping_sub(minLocal as ::core::ffi::c_uint)
                        .wrapping_rem(
                            ((*(*pPage).pBt).usableSize as ::core::ffi::c_uint)
                                .wrapping_sub(4 as ::core::ffi::c_uint),
                        ),
                ) as u32_0;
            if nSize > (*pPage).maxLocal as ::core::ffi::c_uint {
                nSize = minLocal as u32_0;
            }
            nSize = (nSize as ::core::ffi::c_uint)
                .wrapping_add(
                    (4 as ::core::ffi::c_int
                        + pIter.offset_from(pCell) as ::core::ffi::c_long as u16_0
                            as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
        }
        return nSize as u16_0;
    }
}
unsafe extern "C" fn ptrmapPutOvflPtr(
    mut pPage: *mut MemPage,
    mut pSrc: *mut MemPage,
    mut pCell: *mut u8_0,
    mut pRC: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut info: CellInfo = CellInfo {
            nKey: 0,
            pPayload: ::core::ptr::null_mut::<u8_0>(),
            nPayload: 0,
            nLocal: 0,
            nSize: 0

        };
        if *pRC != 0 {
            return;
        }
        (*pPage)
            .xParseCell
            .expect("non-null function pointer")(pPage, pCell, &raw mut info);
        if (info.nLocal as ::core::ffi::c_uint) < info.nPayload {
            let mut ovfl: Pgno = 0;
            if (pCell as uptr) < (*pSrc).aDataEnd as uptr
                && pCell.offset(info.nLocal as ::core::ffi::c_int as isize) as uptr
                    > (*pSrc).aDataEnd as uptr
            {
                *pRC = sqlite3CorruptError(1591 as ::core::ffi::c_int);
                return;
            }
            ovfl = sqlite3Get4byte(
                pCell
                    .offset(
                        (info.nSize as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut u8_0,
            ) as Pgno;
            ptrmapPut((*pPage).pBt, ovfl, PTRMAP_OVERFLOW1 as u8_0, (*pPage).pgno, pRC);
        }
    }
}
unsafe extern "C" fn defragmentPage(
    mut pPage: *mut MemPage,
    mut nMaxFrag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut i: ::core::ffi::c_int = 0;
        let mut pc: ::core::ffi::c_int = 0;
        let mut hdr: ::core::ffi::c_int = 0;
        let mut size: ::core::ffi::c_int = 0;
        let mut usableSize: ::core::ffi::c_int = 0;
        let mut cellOffset: ::core::ffi::c_int = 0;
        let mut cbrk: ::core::ffi::c_int = 0;
        let mut nCell: ::core::ffi::c_int = 0;
        let mut data: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut temp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut src: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut iCellFirst: ::core::ffi::c_int = 0;
        let mut iCellLast: ::core::ffi::c_int = 0;
        let mut iCellStart: ::core::ffi::c_int = 0;
        data = (*pPage).aData as *mut ::core::ffi::c_uchar;
        hdr = (*pPage).hdrOffset as ::core::ffi::c_int;
        cellOffset = (*pPage).cellOffset as ::core::ffi::c_int;
        nCell = (*pPage).nCell as ::core::ffi::c_int;
        iCellFirst = cellOffset + 2 as ::core::ffi::c_int * nCell;
        usableSize = (*(*pPage).pBt).usableSize as ::core::ffi::c_int;
        if *data.offset((hdr + 7 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            <= nMaxFrag
        {
            let mut iFree: ::core::ffi::c_int = (*(data
                .offset((hdr + 1 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_uchar)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(data.offset((hdr + 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_uchar)
                    .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            if iFree > usableSize - 4 as ::core::ffi::c_int {
                return sqlite3CorruptError(1649 as ::core::ffi::c_int);
            }
            if iFree != 0 {
                let mut iFree2: ::core::ffi::c_int = (*(data.offset(iFree as isize)
                    as *mut ::core::ffi::c_uchar)
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(data.offset(iFree as isize) as *mut ::core::ffi::c_uchar)
                        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                if iFree2 > usableSize - 4 as ::core::ffi::c_int {
                    return sqlite3CorruptError(1652 as ::core::ffi::c_int);
                }
                if 0 as ::core::ffi::c_int == iFree2
                    || *data.offset(iFree2 as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && *data.offset((iFree2 + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    let mut pEnd: *mut u8_0 = data
                        .offset((cellOffset + nCell * 2 as ::core::ffi::c_int) as isize)
                        as *mut u8_0;
                    let mut pAddr: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                    let mut sz2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut sz: ::core::ffi::c_int = (*(data
                        .offset((iFree + 2 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_uchar)
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *(data.offset((iFree + 2 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_uchar)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int;
                    let mut top: ::core::ffi::c_int = (*(data
                        .offset((hdr + 5 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_uchar)
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_uchar)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int;
                    if top >= iFree {
                        return sqlite3CorruptError(1660 as ::core::ffi::c_int);
                    }
                    if iFree2 != 0 {
                        if iFree + sz > iFree2 {
                            return sqlite3CorruptError(1663 as ::core::ffi::c_int);
                        }
                        sz2 = (*(data.offset((iFree2 + 2 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_uchar)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                            | *(data.offset((iFree2 + 2 as ::core::ffi::c_int) as isize)
                                as *mut ::core::ffi::c_uchar)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int;
                        if iFree2 + sz2 > usableSize {
                            return sqlite3CorruptError(1665 as ::core::ffi::c_int);
                        }
                        memmove(
                            data.offset((iFree + sz + sz2) as isize)
                                as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                            data.offset((iFree + sz) as isize)
                                as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                            (iFree2 - (iFree + sz)) as size_t,
                        );
                        sz += sz2;
                    } else if iFree + sz > usableSize {
                        return sqlite3CorruptError(1669 as ::core::ffi::c_int)
                    }
                    cbrk = top + sz;
                    memmove(
                        data.offset(cbrk as isize) as *mut ::core::ffi::c_uchar
                            as *mut ::core::ffi::c_void,
                        data.offset(top as isize) as *mut ::core::ffi::c_uchar
                            as *const ::core::ffi::c_void,
                        (iFree - top) as size_t,
                    );
                    pAddr = data.offset(cellOffset as isize) as *mut ::core::ffi::c_uchar
                        as *mut u8_0;
                    while pAddr < pEnd {
                        pc = (*pAddr.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                            | *pAddr.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int;
                        if pc < iFree {
                            *pAddr.offset(0 as ::core::ffi::c_int as isize) = (pc + sz
                                >> 8 as ::core::ffi::c_int) as u8_0;
                            *pAddr.offset(1 as ::core::ffi::c_int as isize) = (pc + sz)
                                as u8_0;
                        } else if pc < iFree2 {
                            *pAddr.offset(0 as ::core::ffi::c_int as isize) = (pc + sz2
                                >> 8 as ::core::ffi::c_int) as u8_0;
                            *pAddr.offset(1 as ::core::ffi::c_int as isize) = (pc + sz2)
                                as u8_0;
                        }
                        pAddr = pAddr.offset(2 as ::core::ffi::c_int as isize);
                    }
                    c2rust_current_block = 13707613154239713890;
                } else {
                    c2rust_current_block = 12997042908615822766;
                }
            } else {
                c2rust_current_block = 12997042908615822766;
            }
        } else {
            c2rust_current_block = 12997042908615822766;
        }
        match c2rust_current_block {
            12997042908615822766 => {
                cbrk = usableSize;
                iCellLast = usableSize - 4 as ::core::ffi::c_int;
                iCellStart = (*(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_uchar)
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_uchar)
                        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                if nCell > 0 as ::core::ffi::c_int {
                    temp = sqlite3PagerTempSpace((*(*pPage).pBt).pPager)
                        as *mut ::core::ffi::c_uchar;
                    memcpy(
                        temp as *mut ::core::ffi::c_void,
                        data as *const ::core::ffi::c_void,
                        usableSize as size_t,
                    );
                    src = temp;
                    i = 0 as ::core::ffi::c_int;
                    while i < nCell {
                        let mut pAddr_0: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                        pAddr_0 = data
                            .offset((cellOffset + i * 2 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_uchar as *mut u8_0;
                        pc = (*pAddr_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                            | *pAddr_0.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int;
                        if pc > iCellLast {
                            return sqlite3CorruptError(1702 as ::core::ffi::c_int);
                        }
                        size = (*pPage)
                            .xCellSize
                            .expect(
                                "non-null function pointer",
                            )(pPage, src.offset(pc as isize) as *mut u8_0)
                            as ::core::ffi::c_int;
                        cbrk -= size;
                        if cbrk < iCellStart || pc + size > usableSize {
                            return sqlite3CorruptError(1708 as ::core::ffi::c_int);
                        }
                        *pAddr_0.offset(0 as ::core::ffi::c_int as isize) = (cbrk
                            >> 8 as ::core::ffi::c_int) as u8_0;
                        *pAddr_0.offset(1 as ::core::ffi::c_int as isize) = cbrk as u8_0;
                        memcpy(
                            data.offset(cbrk as isize) as *mut ::core::ffi::c_uchar
                                as *mut ::core::ffi::c_void,
                            src.offset(pc as isize) as *mut ::core::ffi::c_uchar
                                as *const ::core::ffi::c_void,
                            size as size_t,
                        );
                        i += 1;
                    }
                }
                *data.offset((hdr + 7 as ::core::ffi::c_int) as isize) = 0
                    as ::core::ffi::c_uchar;
            }
            _ => {}
        }
        if *data.offset((hdr + 7 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + cbrk - iCellFirst != (*pPage).nFree
        {
            return sqlite3CorruptError(1722 as ::core::ffi::c_int);
        }
        *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_uchar)
            .offset(0 as ::core::ffi::c_int as isize) = (cbrk >> 8 as ::core::ffi::c_int)
            as u8_0 as ::core::ffi::c_uchar;
        *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) = cbrk as u8_0
            as ::core::ffi::c_uchar;
        *data.offset((hdr + 1 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_uchar;
        *data.offset((hdr + 2 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_uchar;
        memset(
            data.offset(iCellFirst as isize) as *mut ::core::ffi::c_uchar
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (cbrk - iCellFirst) as size_t,
        );
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pageFindSlot(
    mut pPg: *mut MemPage,
    mut nByte: ::core::ffi::c_int,
    mut pRc: *mut ::core::ffi::c_int,
) -> *mut u8_0 {
    unsafe {
        let hdr: ::core::ffi::c_int = (*pPg).hdrOffset as ::core::ffi::c_int;
        let aData: *mut u8_0 = (*pPg).aData;
        let mut iAddr: ::core::ffi::c_int = hdr + 1 as ::core::ffi::c_int;
        let mut pTmp: *mut u8_0 = aData.offset(iAddr as isize) as *mut u8_0;
        let mut pc: ::core::ffi::c_int = (*pTmp.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | *pTmp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut x: ::core::ffi::c_int = 0;
        let mut maxPC: ::core::ffi::c_int = ((*(*pPg).pBt).usableSize
            as ::core::ffi::c_uint)
            .wrapping_sub(nByte as ::core::ffi::c_uint) as ::core::ffi::c_int;
        let mut size: ::core::ffi::c_int = 0;
        while pc <= maxPC {
            pTmp = aData.offset((pc + 2 as ::core::ffi::c_int) as isize) as *mut u8_0;
            size = (*pTmp.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *pTmp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            x = size - nByte;
            if x >= 0 as ::core::ffi::c_int {
                if x < 4 as ::core::ffi::c_int {
                    if *aData.offset((hdr + 7 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int > 57 as ::core::ffi::c_int
                    {
                        return ::core::ptr::null_mut::<u8_0>();
                    }
                    memcpy(
                        aData.offset(iAddr as isize) as *mut u8_0
                            as *mut ::core::ffi::c_void,
                        aData.offset(pc as isize) as *mut u8_0
                            as *const ::core::ffi::c_void,
                        2 as size_t,
                    );
                    let ref mut c2rust_fresh20 = *aData
                        .offset((hdr + 7 as ::core::ffi::c_int) as isize);
                    *c2rust_fresh20 = (*c2rust_fresh20 as ::core::ffi::c_int
                        + x as u8_0 as ::core::ffi::c_int) as u8_0;
                    return aData.offset(pc as isize) as *mut u8_0;
                } else if x + pc > maxPC {
                    *pRc = sqlite3CorruptError(1779 as ::core::ffi::c_int);
                    return ::core::ptr::null_mut::<u8_0>();
                } else {
                    *(aData.offset((pc + 2 as ::core::ffi::c_int) as isize) as *mut u8_0)
                        .offset(0 as ::core::ffi::c_int as isize) = (x
                        >> 8 as ::core::ffi::c_int) as u8_0;
                    *(aData.offset((pc + 2 as ::core::ffi::c_int) as isize) as *mut u8_0)
                        .offset(1 as ::core::ffi::c_int as isize) = x as u8_0;
                }
                return aData.offset((pc + x) as isize) as *mut u8_0;
            }
            iAddr = pc;
            pTmp = aData.offset(pc as isize) as *mut u8_0;
            pc = (*pTmp.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *pTmp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            if pc <= iAddr {
                if pc != 0 {
                    *pRc = sqlite3CorruptError(1794 as ::core::ffi::c_int);
                }
                return ::core::ptr::null_mut::<u8_0>();
            }
        }
        if pc > maxPC + nByte - 4 as ::core::ffi::c_int {
            *pRc = sqlite3CorruptError(1801 as ::core::ffi::c_int);
        }
        return ::core::ptr::null_mut::<u8_0>();
    }
}
#[inline(always)]
unsafe extern "C" fn allocateSpace(
    mut pPage: *mut MemPage,
    mut nByte: ::core::ffi::c_int,
    mut pIdx: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let hdr: ::core::ffi::c_int = (*pPage).hdrOffset as ::core::ffi::c_int;
        let data: *mut u8_0 = (*pPage).aData;
        let mut top: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pTmp: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut gap: ::core::ffi::c_int = 0;
        gap = (*pPage).cellOffset as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * (*pPage).nCell as ::core::ffi::c_int;
        pTmp = data.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut u8_0;
        top = (*pTmp.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *pTmp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        if gap > top {
            if top == 0 as ::core::ffi::c_int
                && (*(*pPage).pBt).usableSize
                    == 65536 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                top = 65536 as ::core::ffi::c_int;
            } else {
                return sqlite3CorruptError(1849 as ::core::ffi::c_int)
            }
        } else if top > (*(*pPage).pBt).usableSize as ::core::ffi::c_int {
            return sqlite3CorruptError(1852 as ::core::ffi::c_int)
        }
        if (*data.offset((hdr + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            != 0
            || *data.offset((hdr + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int != 0) && gap + 2 as ::core::ffi::c_int <= top
        {
            let mut pSpace: *mut u8_0 = pageFindSlot(pPage, nByte, &raw mut rc);
            if !pSpace.is_null() {
                let mut g2: ::core::ffi::c_int = 0;
                g2 = pSpace.offset_from(data) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
                *pIdx = g2;
                if g2 <= gap {
                    return sqlite3CorruptError(1869 as ::core::ffi::c_int)
                } else {
                    return SQLITE_OK
                }
            } else if rc != 0 {
                return rc
            }
        }
        if gap + 2 as ::core::ffi::c_int + nByte > top {
            rc = defragmentPage(
                pPage,
                if (4 as ::core::ffi::c_int)
                    < (*pPage).nFree - (2 as ::core::ffi::c_int + nByte)
                {
                    4 as ::core::ffi::c_int
                } else {
                    (*pPage).nFree - (2 as ::core::ffi::c_int + nByte)
                },
            );
            if rc != 0 {
                return rc;
            }
            top = (((*(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                as *mut u8_0)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut u8_0)
                    .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int;
        }
        top -= nByte;
        *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut u8_0)
            .offset(0 as ::core::ffi::c_int as isize) = (top >> 8 as ::core::ffi::c_int)
            as u8_0;
        *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut u8_0)
            .offset(1 as ::core::ffi::c_int as isize) = top as u8_0;
        *pIdx = top;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn freeSpace(
    mut pPage: *mut MemPage,
    mut iStart: ::core::ffi::c_int,
    mut iSize: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iPtr: ::core::ffi::c_int = 0;
        let mut iFreeBlk: ::core::ffi::c_int = 0;
        let mut hdr: u8_0 = 0;
        let mut nFrag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iOrigSize: ::core::ffi::c_int = iSize;
        let mut x: ::core::ffi::c_int = 0;
        let mut iEnd: ::core::ffi::c_int = iStart + iSize;
        let mut data: *mut ::core::ffi::c_uchar = (*pPage).aData
            as *mut ::core::ffi::c_uchar;
        let mut pTmp: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        hdr = (*pPage).hdrOffset;
        iPtr = hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
        if *data.offset((iPtr + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && *data.offset(iPtr as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            iFreeBlk = 0 as ::core::ffi::c_int;
        } else {
            loop {
                iFreeBlk = (*(data.offset(iPtr as isize) as *mut ::core::ffi::c_uchar)
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(data.offset(iPtr as isize) as *mut ::core::ffi::c_uchar)
                        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                if !(iFreeBlk < iStart) {
                    break;
                }
                if iFreeBlk <= iPtr {
                    if iFreeBlk == 0 as ::core::ffi::c_int {
                        break;
                    }
                    return sqlite3CorruptError(1948 as ::core::ffi::c_int);
                } else {
                    iPtr = iFreeBlk;
                }
            }
            if iFreeBlk
                > (*(*pPage).pBt).usableSize as ::core::ffi::c_int
                    - 4 as ::core::ffi::c_int
            {
                return sqlite3CorruptError(1953 as ::core::ffi::c_int);
            }
            if iFreeBlk != 0 && iEnd + 3 as ::core::ffi::c_int >= iFreeBlk {
                nFrag = iFreeBlk - iEnd;
                if iEnd > iFreeBlk {
                    return sqlite3CorruptError(1965 as ::core::ffi::c_int);
                }
                iEnd = iFreeBlk
                    + ((*(data.offset((iFreeBlk + 2 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_uchar)
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *(data.offset((iFreeBlk + 2 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_uchar)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int);
                if iEnd > (*(*pPage).pBt).usableSize as ::core::ffi::c_int {
                    return sqlite3CorruptError(1968 as ::core::ffi::c_int);
                }
                iSize = iEnd - iStart;
                iFreeBlk = (*(data.offset(iFreeBlk as isize)
                    as *mut ::core::ffi::c_uchar)
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(data.offset(iFreeBlk as isize) as *mut ::core::ffi::c_uchar)
                        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            }
            if iPtr > hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
                let mut iPtrEnd: ::core::ffi::c_int = iPtr
                    + ((*(data.offset((iPtr + 2 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_uchar)
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *(data.offset((iPtr + 2 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_uchar)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int);
                if iPtrEnd + 3 as ::core::ffi::c_int >= iStart {
                    if iPtrEnd > iStart {
                        return sqlite3CorruptError(1981 as ::core::ffi::c_int);
                    }
                    nFrag += iStart - iPtrEnd;
                    iSize = iEnd - iPtr;
                    iStart = iPtr;
                }
            }
            if nFrag
                > *data
                    .offset(
                        (hdr as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
            {
                return sqlite3CorruptError(1987 as ::core::ffi::c_int);
            }
            let ref mut c2rust_fresh21 = *data
                .offset((hdr as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize);
            *c2rust_fresh21 = (*c2rust_fresh21 as ::core::ffi::c_int
                - nFrag as u8_0 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        }
        pTmp = data
            .offset((hdr as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_uchar as *mut u8_0;
        x = (*pTmp.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *pTmp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        if (*(*pPage).pBt).btsFlags as ::core::ffi::c_int & BTS_FAST_SECURE != 0 {
            memset(
                data.offset(iStart as isize) as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                iSize as size_t,
            );
        }
        if iStart <= x {
            if iStart < x {
                return sqlite3CorruptError(2001 as ::core::ffi::c_int);
            }
            if iPtr != hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
                return sqlite3CorruptError(2002 as ::core::ffi::c_int);
            }
            *(data.offset((hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_uchar)
                .offset(0 as ::core::ffi::c_int as isize) = (iFreeBlk
                >> 8 as ::core::ffi::c_int) as u8_0 as ::core::ffi::c_uchar;
            *(data.offset((hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_uchar)
                .offset(1 as ::core::ffi::c_int as isize) = iFreeBlk as u8_0
                as ::core::ffi::c_uchar;
            *(data.offset((hdr as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_uchar)
                .offset(0 as ::core::ffi::c_int as isize) = (iEnd
                >> 8 as ::core::ffi::c_int) as u8_0 as ::core::ffi::c_uchar;
            *(data.offset((hdr as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_uchar)
                .offset(1 as ::core::ffi::c_int as isize) = iEnd as u8_0
                as ::core::ffi::c_uchar;
        } else {
            *(data.offset(iPtr as isize) as *mut ::core::ffi::c_uchar)
                .offset(0 as ::core::ffi::c_int as isize) = (iStart
                >> 8 as ::core::ffi::c_int) as u8_0 as ::core::ffi::c_uchar;
            *(data.offset(iPtr as isize) as *mut ::core::ffi::c_uchar)
                .offset(1 as ::core::ffi::c_int as isize) = iStart as u8_0
                as ::core::ffi::c_uchar;
            *(data.offset(iStart as isize) as *mut ::core::ffi::c_uchar)
                .offset(0 as ::core::ffi::c_int as isize) = (iFreeBlk
                >> 8 as ::core::ffi::c_int) as u8_0 as ::core::ffi::c_uchar;
            *(data.offset(iStart as isize) as *mut ::core::ffi::c_uchar)
                .offset(1 as ::core::ffi::c_int as isize) = iFreeBlk as u8_0
                as ::core::ffi::c_uchar;
            *(data.offset((iStart + 2 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_uchar)
                .offset(0 as ::core::ffi::c_int as isize) = (iSize as u16_0
                as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as u8_0
                as ::core::ffi::c_uchar;
            *(data.offset((iStart + 2 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_uchar)
                .offset(1 as ::core::ffi::c_int as isize) = iSize as u16_0 as u8_0
                as ::core::ffi::c_uchar;
        }
        (*pPage).nFree += iOrigSize;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn decodeFlags(
    mut pPage: *mut MemPage,
    mut flagByte: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = ::core::ptr::null_mut::<BtShared>();
        pBt = (*pPage).pBt;
        (*pPage).max1bytePayload = (*pBt).max1bytePayload;
        if flagByte >= PTF_ZERODATA | PTF_LEAF {
            (*pPage).childPtrSize = 0 as u8_0;
            (*pPage).leaf = 1 as u8_0;
            if flagByte == PTF_LEAFDATA | PTF_INTKEY | PTF_LEAF {
                (*pPage).intKeyLeaf = 1 as u8_0;
                (*pPage).xCellSize = Some(
                    cellSizePtrTableLeaf
                        as unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0,
                ) as Option<unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0>;
                (*pPage).xParseCell = Some(
                    btreeParseCellPtr
                        as unsafe extern "C" fn(
                            *mut MemPage,
                            *mut u8_0,
                            *mut CellInfo,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut MemPage,
                            *mut u8_0,
                            *mut CellInfo,
                        ) -> (),
                    >;
                (*pPage).intKey = 1 as u8_0;
                (*pPage).maxLocal = (*pBt).maxLeaf;
                (*pPage).minLocal = (*pBt).minLeaf;
            } else if flagByte == PTF_ZERODATA | PTF_LEAF {
                (*pPage).intKey = 0 as u8_0;
                (*pPage).intKeyLeaf = 0 as u8_0;
                (*pPage).xCellSize = Some(
                    cellSizePtrIdxLeaf
                        as unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0,
                ) as Option<unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0>;
                (*pPage).xParseCell = Some(
                    btreeParseCellPtrIndex
                        as unsafe extern "C" fn(
                            *mut MemPage,
                            *mut u8_0,
                            *mut CellInfo,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut MemPage,
                            *mut u8_0,
                            *mut CellInfo,
                        ) -> (),
                    >;
                (*pPage).maxLocal = (*pBt).maxLocal;
                (*pPage).minLocal = (*pBt).minLocal;
            } else {
                (*pPage).intKey = 0 as u8_0;
                (*pPage).intKeyLeaf = 0 as u8_0;
                (*pPage).xCellSize = Some(
                    cellSizePtrIdxLeaf
                        as unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0,
                ) as Option<unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0>;
                (*pPage).xParseCell = Some(
                    btreeParseCellPtrIndex
                        as unsafe extern "C" fn(
                            *mut MemPage,
                            *mut u8_0,
                            *mut CellInfo,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut MemPage,
                            *mut u8_0,
                            *mut CellInfo,
                        ) -> (),
                    >;
                return sqlite3CorruptError(2057 as ::core::ffi::c_int);
            }
        } else {
            (*pPage).childPtrSize = 4 as u8_0;
            (*pPage).leaf = 0 as u8_0;
            if flagByte == 0x2 as ::core::ffi::c_int {
                (*pPage).intKey = 0 as u8_0;
                (*pPage).intKeyLeaf = 0 as u8_0;
                (*pPage).xCellSize = Some(
                    cellSizePtr as unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0,
                ) as Option<unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0>;
                (*pPage).xParseCell = Some(
                    btreeParseCellPtrIndex
                        as unsafe extern "C" fn(
                            *mut MemPage,
                            *mut u8_0,
                            *mut CellInfo,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut MemPage,
                            *mut u8_0,
                            *mut CellInfo,
                        ) -> (),
                    >;
                (*pPage).maxLocal = (*pBt).maxLocal;
                (*pPage).minLocal = (*pBt).minLocal;
            } else if flagByte == PTF_LEAFDATA | PTF_INTKEY {
                (*pPage).intKeyLeaf = 0 as u8_0;
                (*pPage).xCellSize = Some(
                    cellSizePtrNoPayload
                        as unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0,
                ) as Option<unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0>;
                (*pPage).xParseCell = Some(
                    btreeParseCellPtrNoPayload
                        as unsafe extern "C" fn(
                            *mut MemPage,
                            *mut u8_0,
                            *mut CellInfo,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut MemPage,
                            *mut u8_0,
                            *mut CellInfo,
                        ) -> (),
                    >;
                (*pPage).intKey = 1 as u8_0;
                (*pPage).maxLocal = (*pBt).maxLeaf;
                (*pPage).minLocal = (*pBt).minLeaf;
            } else {
                (*pPage).intKey = 0 as u8_0;
                (*pPage).intKeyLeaf = 0 as u8_0;
                (*pPage).xCellSize = Some(
                    cellSizePtr as unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0,
                ) as Option<unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0>;
                (*pPage).xParseCell = Some(
                    btreeParseCellPtrIndex
                        as unsafe extern "C" fn(
                            *mut MemPage,
                            *mut u8_0,
                            *mut CellInfo,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut MemPage,
                            *mut u8_0,
                            *mut CellInfo,
                        ) -> (),
                    >;
                return sqlite3CorruptError(2081 as ::core::ffi::c_int);
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn btreeComputeFreeSpace(
    mut pPage: *mut MemPage,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pc: ::core::ffi::c_int = 0;
        let mut hdr: u8_0 = 0;
        let mut data: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut usableSize: ::core::ffi::c_int = 0;
        let mut nFree: ::core::ffi::c_int = 0;
        let mut top: ::core::ffi::c_int = 0;
        let mut iCellFirst: ::core::ffi::c_int = 0;
        let mut iCellLast: ::core::ffi::c_int = 0;
        usableSize = (*(*pPage).pBt).usableSize as ::core::ffi::c_int;
        hdr = (*pPage).hdrOffset;
        data = (*pPage).aData;
        top = (((*(data
            .offset((hdr as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
            as *mut u8_0)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *(data
                .offset((hdr as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                as *mut u8_0)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int;
        iCellFirst = hdr as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + (*pPage).childPtrSize as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * (*pPage).nCell as ::core::ffi::c_int;
        iCellLast = usableSize - 4 as ::core::ffi::c_int;
        pc = (*(data
            .offset((hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            as *mut u8_0)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *(data
                .offset((hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                as *mut u8_0)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        nFree = *data
            .offset((hdr as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int + top;
        if pc > 0 as ::core::ffi::c_int {
            let mut next: u32_0 = 0;
            let mut size: u32_0 = 0;
            if pc < top {
                return sqlite3CorruptError(2132 as ::core::ffi::c_int);
            }
            loop {
                if pc > iCellLast {
                    return sqlite3CorruptError(2137 as ::core::ffi::c_int);
                }
                next = ((*(data.offset(pc as isize) as *mut u8_0)
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(data.offset(pc as isize) as *mut u8_0)
                        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    as u32_0;
                size = ((*(data.offset((pc + 2 as ::core::ffi::c_int) as isize)
                    as *mut u8_0)
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(data.offset((pc + 2 as ::core::ffi::c_int) as isize)
                        as *mut u8_0)
                        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    as u32_0;
                nFree = (nFree as u32_0).wrapping_add(size) as ::core::ffi::c_int;
                if next
                    <= (pc as ::core::ffi::c_uint)
                        .wrapping_add(size as ::core::ffi::c_uint)
                        .wrapping_add(3 as ::core::ffi::c_uint)
                {
                    break;
                }
                pc = next as ::core::ffi::c_int;
            }
            if next > 0 as ::core::ffi::c_uint {
                return sqlite3CorruptError(2147 as ::core::ffi::c_int);
            }
            if (pc as u32_0).wrapping_add(size) > usableSize as ::core::ffi::c_uint {
                return sqlite3CorruptError(2151 as ::core::ffi::c_int);
            }
        }
        if nFree > usableSize || nFree < iCellFirst {
            return sqlite3CorruptError(2163 as ::core::ffi::c_int);
        }
        (*pPage).nFree = (nFree - iCellFirst) as u16_0 as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
#[inline(never)]
unsafe extern "C" fn btreeCellSizeCheck(mut pPage: *mut MemPage) -> ::core::ffi::c_int {
    unsafe {
        let mut iCellFirst: ::core::ffi::c_int = 0;
        let mut iCellLast: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut sz: ::core::ffi::c_int = 0;
        let mut pc: ::core::ffi::c_int = 0;
        let mut data: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut usableSize: ::core::ffi::c_int = 0;
        let mut cellOffset: ::core::ffi::c_int = 0;
        iCellFirst = (*pPage).cellOffset as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * (*pPage).nCell as ::core::ffi::c_int;
        usableSize = (*(*pPage).pBt).usableSize as ::core::ffi::c_int;
        iCellLast = usableSize - 4 as ::core::ffi::c_int;
        data = (*pPage).aData;
        cellOffset = (*pPage).cellOffset as ::core::ffi::c_int;
        if (*pPage).leaf == 0 {
            iCellLast -= 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pPage).nCell as ::core::ffi::c_int {
            pc = (*(data.offset((cellOffset + i * 2 as ::core::ffi::c_int) as isize)
                as *mut u8_0)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(data.offset((cellOffset + i * 2 as ::core::ffi::c_int) as isize)
                    as *mut u8_0)
                    .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            if pc < iCellFirst || pc > iCellLast {
                return sqlite3CorruptError(2194 as ::core::ffi::c_int);
            }
            sz = (*pPage)
                .xCellSize
                .expect(
                    "non-null function pointer",
                )(pPage, data.offset(pc as isize) as *mut u8_0) as ::core::ffi::c_int;
            if pc + sz > usableSize {
                return sqlite3CorruptError(2199 as ::core::ffi::c_int);
            }
            i += 1;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn btreeInitPage(mut pPage: *mut MemPage) -> ::core::ffi::c_int {
    unsafe {
        let mut data: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pBt: *mut BtShared = ::core::ptr::null_mut::<BtShared>();
        pBt = (*pPage).pBt;
        data = (*pPage).aData.offset((*pPage).hdrOffset as ::core::ffi::c_int as isize);
        if decodeFlags(
            pPage,
            *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) != 0
        {
            return sqlite3CorruptError(2231 as ::core::ffi::c_int);
        }
        (*pPage).maskPage = ((*pBt).pageSize as ::core::ffi::c_uint)
            .wrapping_sub(1 as ::core::ffi::c_uint) as u16_0;
        (*pPage).nOverflow = 0 as u8_0;
        (*pPage).cellOffset = ((*pPage).hdrOffset as ::core::ffi::c_int
            + 8 as ::core::ffi::c_int + (*pPage).childPtrSize as ::core::ffi::c_int)
            as u16_0;
        (*pPage).aCellIdx = data
            .offset((*pPage).childPtrSize as ::core::ffi::c_int as isize)
            .offset(8 as ::core::ffi::c_int as isize);
        (*pPage).aDataEnd = (*pPage).aData.offset((*pBt).pageSize as isize);
        (*pPage).aDataOfst = (*pPage)
            .aData
            .offset((*pPage).childPtrSize as ::core::ffi::c_int as isize);
        (*pPage).nCell = ((*(data.offset(3 as ::core::ffi::c_int as isize) as *mut u8_0)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *(data.offset(3 as ::core::ffi::c_int as isize) as *mut u8_0)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as u16_0;
        if (*pPage).nCell as ::core::ffi::c_uint
            > ((*pBt).pageSize as ::core::ffi::c_uint)
                .wrapping_sub(8 as ::core::ffi::c_uint)
                .wrapping_div(6 as ::core::ffi::c_uint)
        {
            return sqlite3CorruptError(2245 as ::core::ffi::c_int);
        }
        (*pPage).nFree = -1 as ::core::ffi::c_int;
        (*pPage).isInit = 1 as u8_0;
        if (*(*pBt).db).flags as ::core::ffi::c_ulonglong
            & SQLITE_CellSizeCk as ::core::ffi::c_ulonglong != 0
        {
            return btreeCellSizeCheck(pPage);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn zeroPage(mut pPage: *mut MemPage, mut flags: ::core::ffi::c_int) {
    unsafe {
        let mut data: *mut ::core::ffi::c_uchar = (*pPage).aData
            as *mut ::core::ffi::c_uchar;
        let mut pBt: *mut BtShared = (*pPage).pBt;
        let mut hdr: ::core::ffi::c_int = (*pPage).hdrOffset as ::core::ffi::c_int;
        let mut first: ::core::ffi::c_int = 0;
        if (*pBt).btsFlags as ::core::ffi::c_int & BTS_FAST_SECURE != 0 {
            memset(
                data.offset(hdr as isize) as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*pBt).usableSize as ::core::ffi::c_uint)
                    .wrapping_sub(hdr as ::core::ffi::c_uint) as size_t,
            );
        }
        *data.offset(hdr as isize) = flags as ::core::ffi::c_char
            as ::core::ffi::c_uchar;
        first = hdr
            + (if flags & PTF_LEAF == 0 as ::core::ffi::c_int {
                12 as ::core::ffi::c_int
            } else {
                8 as ::core::ffi::c_int
            });
        memset(
            data.offset((hdr + 1 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            4 as size_t,
        );
        *data.offset((hdr + 7 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_uchar;
        *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_uchar)
            .offset(0 as ::core::ffi::c_int as isize) = ((*pBt).usableSize
            >> 8 as ::core::ffi::c_int) as u8_0 as ::core::ffi::c_uchar;
        *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) = (*pBt).usableSize as u8_0
            as ::core::ffi::c_uchar;
        (*pPage).nFree = ((*pBt).usableSize as ::core::ffi::c_uint)
            .wrapping_sub(first as ::core::ffi::c_uint) as u16_0 as ::core::ffi::c_int;
        decodeFlags(pPage, flags);
        (*pPage).cellOffset = first as u16_0;
        (*pPage).aDataEnd = data.offset((*pBt).pageSize as isize)
            as *mut ::core::ffi::c_uchar as *mut u8_0;
        (*pPage).aCellIdx = data.offset(first as isize) as *mut ::core::ffi::c_uchar
            as *mut u8_0;
        (*pPage).aDataOfst = data.offset((*pPage).childPtrSize as isize)
            as *mut ::core::ffi::c_uchar as *mut u8_0;
        (*pPage).nOverflow = 0 as u8_0;
        (*pPage).maskPage = ((*pBt).pageSize as ::core::ffi::c_uint)
            .wrapping_sub(1 as ::core::ffi::c_uint) as u16_0;
        (*pPage).nCell = 0 as u16_0;
        (*pPage).isInit = 1 as u8_0;
    }
}
unsafe extern "C" fn btreePageFromDbPage(
    mut pDbPage: *mut DbPage,
    mut pgno: Pgno,
    mut pBt: *mut BtShared,
) -> *mut MemPage {
    unsafe {
        let mut pPage: *mut MemPage = sqlite3PagerGetExtra(pDbPage) as *mut MemPage;
        if pgno != (*pPage).pgno {
            (*pPage).aData = sqlite3PagerGetData(pDbPage) as *mut u8_0;
            (*pPage).pDbPage = pDbPage;
            (*pPage).pBt = pBt;
            (*pPage).pgno = pgno;
            (*pPage).hdrOffset = (if pgno == 1 as ::core::ffi::c_uint {
                100 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as u8_0;
        }
        return pPage;
    }
}
unsafe extern "C" fn btreeGetPage(
    mut pBt: *mut BtShared,
    mut pgno: Pgno,
    mut ppPage: *mut *mut MemPage,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pDbPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
        rc = sqlite3PagerGet((*pBt).pPager, pgno, &raw mut pDbPage, flags);
        if rc != 0 {
            return rc;
        }
        *ppPage = btreePageFromDbPage(pDbPage, pgno, pBt);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn btreePageLookup(
    mut pBt: *mut BtShared,
    mut pgno: Pgno,
) -> *mut MemPage {
    unsafe {
        let mut pDbPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
        pDbPage = sqlite3PagerLookup((*pBt).pPager, pgno);
        if !pDbPage.is_null() {
            return btreePageFromDbPage(pDbPage, pgno, pBt);
        }
        return ::core::ptr::null_mut::<MemPage>();
    }
}
unsafe extern "C" fn btreePagecount(mut pBt: *mut BtShared) -> Pgno {
    unsafe {
        return (*pBt).nPage as Pgno;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeLastPage(mut p: *mut Btree) -> Pgno {
    unsafe {
        return btreePagecount((*p).pBt);
    }
}
unsafe extern "C" fn getAndInitPage(
    mut pBt: *mut BtShared,
    mut pgno: Pgno,
    mut ppPage: *mut *mut MemPage,
    mut bReadOnly: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pDbPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        if pgno > btreePagecount(pBt) {
            *ppPage = ::core::ptr::null_mut::<MemPage>();
            return sqlite3CorruptError(2388 as ::core::ffi::c_int);
        }
        rc = sqlite3PagerGet((*pBt).pPager, pgno, &raw mut pDbPage, bReadOnly);
        if rc != 0 {
            *ppPage = ::core::ptr::null_mut::<MemPage>();
            return rc;
        }
        pPage = sqlite3PagerGetExtra(pDbPage) as *mut MemPage;
        if (*pPage).isInit as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            btreePageFromDbPage(pDbPage, pgno, pBt);
            rc = btreeInitPage(pPage);
            if rc != SQLITE_OK {
                releasePage(pPage);
                *ppPage = ::core::ptr::null_mut::<MemPage>();
                return rc;
            }
        }
        *ppPage = pPage;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn releasePageNotNull(mut pPage: *mut MemPage) {
    unsafe {
        sqlite3PagerUnrefNotNull((*pPage).pDbPage);
    }
}
unsafe extern "C" fn releasePage(mut pPage: *mut MemPage) {
    unsafe {
        if !pPage.is_null() {
            releasePageNotNull(pPage);
        }
    }
}
unsafe extern "C" fn releasePageOne(mut pPage: *mut MemPage) {
    unsafe {
        sqlite3PagerUnrefPageOne((*pPage).pDbPage);
    }
}
unsafe extern "C" fn btreeGetUnusedPage(
    mut pBt: *mut BtShared,
    mut pgno: Pgno,
    mut ppPage: *mut *mut MemPage,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = btreeGetPage(pBt, pgno, ppPage, flags);
        if rc == SQLITE_OK {
            if sqlite3PagerPageRefcount((**ppPage).pDbPage) > 1 as ::core::ffi::c_int {
                releasePage(*ppPage);
                *ppPage = ::core::ptr::null_mut::<MemPage>();
                return sqlite3CorruptError(2460 as ::core::ffi::c_int);
            }
            (**ppPage).isInit = 0 as u8_0;
        } else {
            *ppPage = ::core::ptr::null_mut::<MemPage>();
        }
        return rc;
    }
}
unsafe extern "C" fn pageReinit(mut pData: *mut DbPage) {
    unsafe {
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        pPage = sqlite3PagerGetExtra(pData) as *mut MemPage;
        if (*pPage).isInit != 0 {
            (*pPage).isInit = 0 as u8_0;
            if sqlite3PagerPageRefcount(pData) > 1 as ::core::ffi::c_int {
                btreeInitPage(pPage);
            }
        }
    }
}
unsafe extern "C" fn btreeInvokeBusyHandler(
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = pArg as *mut BtShared;
        return sqlite3InvokeBusyHandler(&raw mut (*(*pBt).db).busyHandler);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zFilename: *const ::core::ffi::c_char,
    mut db: *mut sqlite3,
    mut ppBtree: *mut *mut Btree,
    mut flags: ::core::ffi::c_int,
    mut vfsFlags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pBt: *mut BtShared = ::core::ptr::null_mut::<BtShared>();
        let mut p: *mut Btree = ::core::ptr::null_mut::<Btree>();
        let mut mutexOpen: *mut sqlite3_mutex = ::core::ptr::null_mut::<sqlite3_mutex>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nReserve: u8_0 = 0;
        let mut zDbHeader: [::core::ffi::c_uchar; 100] = [0; 100];
        let isTempDb: ::core::ffi::c_int = (zFilename.is_null()
            || *zFilename.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let isMemdb: ::core::ffi::c_int = (!zFilename.is_null()
            && strcmp(zFilename, b":memory:\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || isTempDb != 0 && sqlite3TempInMemory(db) != 0
            || vfsFlags & SQLITE_OPEN_MEMORY != 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        if isMemdb != 0 {
            flags |= BTREE_MEMORY;
        }
        if vfsFlags & SQLITE_OPEN_MAIN_DB != 0 as ::core::ffi::c_int
            && (isMemdb != 0 || isTempDb != 0)
        {
            vfsFlags = vfsFlags & !SQLITE_OPEN_MAIN_DB | SQLITE_OPEN_TEMP_DB;
        }
        p = sqlite3MallocZero(::core::mem::size_of::<Btree>() as u64_0) as *mut Btree;
        if p.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        (*p).inTrans = TRANS_NONE as u8_0;
        (*p).db = db;
        if pBt.is_null() {
            memset(
                (&raw mut zDbHeader as *mut ::core::ffi::c_uchar)
                    .offset(16 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                8 as size_t,
            );
            pBt = sqlite3MallocZero(::core::mem::size_of::<BtShared>() as u64_0)
                as *mut BtShared;
            if pBt.is_null() {
                rc = SQLITE_NOMEM_BKPT;
                c2rust_current_block = 5790547362422950074;
            } else {
                rc = sqlite3PagerOpen(
                    pVfs,
                    &raw mut (*pBt).pPager,
                    zFilename,
                    ::core::mem::size_of::<MemPage>() as ::core::ffi::c_int,
                    flags,
                    vfsFlags,
                    Some(pageReinit as unsafe extern "C" fn(*mut DbPage) -> ()),
                );
                if rc == SQLITE_OK {
                    sqlite3PagerSetMmapLimit(
                        (*pBt).pPager,
                        (*db).szMmap as sqlite3_int64,
                    );
                    rc = sqlite3PagerReadFileheader(
                        (*pBt).pPager,
                        ::core::mem::size_of::<[::core::ffi::c_uchar; 100]>()
                            as ::core::ffi::c_int,
                        &raw mut zDbHeader as *mut ::core::ffi::c_uchar,
                    );
                }
                if rc != SQLITE_OK {
                    c2rust_current_block = 5790547362422950074;
                } else {
                    (*pBt).openFlags = flags as u8_0;
                    (*pBt).db = db;
                    sqlite3PagerSetBusyHandler(
                        (*pBt).pPager,
                        Some(
                            btreeInvokeBusyHandler
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                ) -> ::core::ffi::c_int,
                        ),
                        pBt as *mut ::core::ffi::c_void,
                    );
                    (*p).pBt = pBt;
                    (*pBt).pCursor = ::core::ptr::null_mut::<BtCursor>();
                    (*pBt).pPage1 = ::core::ptr::null_mut::<MemPage>();
                    if sqlite3PagerIsreadonly((*pBt).pPager) != 0 {
                        (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int
                            | BTS_READ_ONLY) as u16_0;
                    }
                    (*pBt).pageSize = ((zDbHeader[16 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                        | (zDbHeader[17 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32_0;
                    if (*pBt).pageSize < 512 as ::core::ffi::c_uint
                        || (*pBt).pageSize > SQLITE_MAX_PAGE_SIZE as ::core::ffi::c_uint
                        || (*pBt).pageSize.wrapping_sub(1 as u32_0) & (*pBt).pageSize
                            != 0 as ::core::ffi::c_uint
                    {
                        (*pBt).pageSize = 0 as u32_0;
                        if !zFilename.is_null() && isMemdb == 0 {
                            (*pBt).autoVacuum = (if SQLITE_DEFAULT_AUTOVACUUM != 0 {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            }) as u8_0;
                            (*pBt).incrVacuum = (if SQLITE_DEFAULT_AUTOVACUUM
                                == 2 as ::core::ffi::c_int
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            }) as u8_0;
                        }
                        nReserve = 0 as u8_0;
                    } else {
                        nReserve = zDbHeader[20 as ::core::ffi::c_int as usize] as u8_0;
                        (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int
                            | BTS_PAGESIZE_FIXED) as u16_0;
                        (*pBt).autoVacuum = (if sqlite3Get4byte(
                            (&raw mut zDbHeader as *mut ::core::ffi::c_uchar)
                                .offset(
                                    (36 as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut ::core::ffi::c_uchar,
                        ) != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as u8_0;
                        (*pBt).incrVacuum = (if sqlite3Get4byte(
                            (&raw mut zDbHeader as *mut ::core::ffi::c_uchar)
                                .offset(
                                    (36 as ::core::ffi::c_int
                                        + 7 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut ::core::ffi::c_uchar,
                        ) != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as u8_0;
                    }
                    rc = sqlite3PagerSetPagesize(
                        (*pBt).pPager,
                        &raw mut (*pBt).pageSize,
                        nReserve as ::core::ffi::c_int,
                    );
                    if rc != 0 {
                        c2rust_current_block = 5790547362422950074;
                    } else {
                        (*pBt).usableSize = ((*pBt).pageSize as ::core::ffi::c_uint)
                            .wrapping_sub(nReserve as ::core::ffi::c_uint) as u32_0;
                        c2rust_current_block = 11626999923138678822;
                    }
                }
            }
        } else {
            c2rust_current_block = 11626999923138678822;
        }
        match c2rust_current_block {
            11626999923138678822 => {
                *ppBtree = p;
            }
            _ => {}
        }
        if rc != SQLITE_OK {
            if !pBt.is_null() && !(*pBt).pPager.is_null() {
                sqlite3PagerClose((*pBt).pPager, ::core::ptr::null_mut::<sqlite3>());
            }
            sqlite3_free(pBt as *mut ::core::ffi::c_void);
            sqlite3_free(p as *mut ::core::ffi::c_void);
            *ppBtree = ::core::ptr::null_mut::<Btree>();
        } else {
            let mut pFile: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
            if sqlite3BtreeSchema(p, 0 as ::core::ffi::c_int, None).is_null() {
                sqlite3BtreeSetCacheSize(p, SQLITE_DEFAULT_CACHE_SIZE);
            }
            pFile = sqlite3PagerFile((*pBt).pPager);
            if !(*pFile).pMethods.is_null() {
                sqlite3OsFileControlHint(
                    pFile,
                    SQLITE_FCNTL_PDB,
                    &raw mut (*pBt).db as *mut ::core::ffi::c_void,
                );
            }
        }
        if !mutexOpen.is_null() {
            sqlite3_mutex_leave(mutexOpen);
        }
        return rc;
    }
}
unsafe extern "C" fn removeFromSharingList(
    mut pBt: *mut BtShared,
) -> ::core::ffi::c_int {
    unsafe {
        return 1 as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn allocateTempSpace(mut pBt: *mut BtShared) -> ::core::ffi::c_int {
    unsafe {
        (*pBt).pTmpSpace = sqlite3PageMalloc((*pBt).pageSize as ::core::ffi::c_int)
            as *mut u8_0;
        if (*pBt).pTmpSpace.is_null() {
            let mut pCur: *mut BtCursor = (*pBt).pCursor;
            (*pBt).pCursor = (*pCur).pNext;
            memset(
                pCur as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<BtCursor>() as size_t,
            );
            return SQLITE_NOMEM_BKPT;
        }
        memset(
            (*pBt).pTmpSpace as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            8 as size_t,
        );
        (*pBt).pTmpSpace = (*pBt).pTmpSpace.offset(4 as ::core::ffi::c_int as isize);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn freeTempSpace(mut pBt: *mut BtShared) {
    unsafe {
        if !(*pBt).pTmpSpace.is_null() {
            (*pBt).pTmpSpace = (*pBt)
                .pTmpSpace
                .offset(-(4 as ::core::ffi::c_int as isize));
            sqlite3PageFree((*pBt).pTmpSpace as *mut ::core::ffi::c_void);
            (*pBt).pTmpSpace = ::core::ptr::null_mut::<u8_0>();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeClose(mut p: *mut Btree) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        sqlite3BtreeRollback(p, SQLITE_OK, 0 as ::core::ffi::c_int);
        if (*p).sharable == 0 || removeFromSharingList(pBt) != 0 {
            sqlite3PagerClose((*pBt).pPager, (*p).db);
            if (*pBt).xFreeSchema.is_some() && !(*pBt).pSchema.is_null() {
                (*pBt).xFreeSchema.expect("non-null function pointer")((*pBt).pSchema);
            }
            sqlite3DbFree(::core::ptr::null_mut::<sqlite3>(), (*pBt).pSchema);
            freeTempSpace(pBt);
            sqlite3_free(pBt as *mut ::core::ffi::c_void);
        }
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeSetCacheSize(
    mut p: *mut Btree,
    mut mxPage: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        sqlite3PagerSetCachesize((*pBt).pPager, mxPage);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeSetSpillSize(
    mut p: *mut Btree,
    mut mxPage: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        let mut res: ::core::ffi::c_int = 0;
        res = sqlite3PagerSetSpillsize((*pBt).pPager, mxPage);
        return res;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeSetMmapLimit(
    mut p: *mut Btree,
    mut szMmap: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        sqlite3PagerSetMmapLimit((*pBt).pPager, szMmap);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeSetPagerFlags(
    mut p: *mut Btree,
    mut pgFlags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        sqlite3PagerSetFlags((*pBt).pPager, pgFlags);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeSetPageSize(
    mut p: *mut Btree,
    mut pageSize: ::core::ffi::c_int,
    mut nReserve: ::core::ffi::c_int,
    mut iFix: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut x: ::core::ffi::c_int = 0;
        let mut pBt: *mut BtShared = (*p).pBt;
        (*pBt).nReserveWanted = nReserve as u8_0;
        x = (*pBt).pageSize.wrapping_sub((*pBt).usableSize) as ::core::ffi::c_int;
        if x == nReserve
            && (pageSize == 0 as ::core::ffi::c_int
                || pageSize as u32_0 == (*pBt).pageSize)
        {
            return SQLITE_OK;
        }
        if nReserve < x {
            nReserve = x;
        }
        if (*pBt).btsFlags as ::core::ffi::c_int & BTS_PAGESIZE_FIXED != 0 {
            return SQLITE_READONLY;
        }
        if pageSize >= 512 as ::core::ffi::c_int && pageSize <= SQLITE_MAX_PAGE_SIZE
            && pageSize - 1 as ::core::ffi::c_int & pageSize == 0 as ::core::ffi::c_int
        {
            if nReserve > 32 as ::core::ffi::c_int
                && pageSize == 512 as ::core::ffi::c_int
            {
                pageSize = 1024 as ::core::ffi::c_int;
            }
            (*pBt).pageSize = pageSize as u32_0;
            freeTempSpace(pBt);
        }
        rc = sqlite3PagerSetPagesize((*pBt).pPager, &raw mut (*pBt).pageSize, nReserve);
        (*pBt).usableSize = ((*pBt).pageSize as ::core::ffi::c_uint)
            .wrapping_sub(nReserve as u16_0 as ::core::ffi::c_uint) as u32_0;
        if iFix != 0 {
            (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int
                | BTS_PAGESIZE_FIXED) as u16_0;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeGetPageSize(
    mut p: *mut Btree,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*p).pBt).pageSize as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeGetReserveNoMutex(
    mut p: *mut Btree,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        n = (*(*p).pBt).pageSize.wrapping_sub((*(*p).pBt).usableSize)
            as ::core::ffi::c_int;
        return n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeGetRequestedReserve(
    mut p: *mut Btree,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n1: ::core::ffi::c_int = 0;
        let mut n2: ::core::ffi::c_int = 0;
        n1 = (*(*p).pBt).nReserveWanted as ::core::ffi::c_int;
        n2 = sqlite3BtreeGetReserveNoMutex(p);
        return if n1 > n2 { n1 } else { n2 };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeMaxPageCount(
    mut p: *mut Btree,
    mut mxPage: Pgno,
) -> Pgno {
    unsafe {
        let mut n: Pgno = 0;
        n = sqlite3PagerMaxPageCount((*(*p).pBt).pPager, mxPage);
        return n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeSecureDelete(
    mut p: *mut Btree,
    mut newFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut b: ::core::ffi::c_int = 0;
        if p.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if newFlag >= 0 as ::core::ffi::c_int {
            (*(*p).pBt).btsFlags = ((*(*p).pBt).btsFlags as ::core::ffi::c_int
                & !BTS_FAST_SECURE) as u16_0;
            (*(*p).pBt).btsFlags = ((*(*p).pBt).btsFlags as ::core::ffi::c_int
                | (BTS_SECURE_DELETE * newFlag) as u16_0 as ::core::ffi::c_int) as u16_0;
        }
        b = ((*(*p).pBt).btsFlags as ::core::ffi::c_int & BTS_FAST_SECURE)
            / BTS_SECURE_DELETE;
        return b;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeSetAutoVacuum(
    mut p: *mut Btree,
    mut autoVacuum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut av: u8_0 = autoVacuum as u8_0;
        if (*pBt).btsFlags as ::core::ffi::c_int & BTS_PAGESIZE_FIXED
            != 0 as ::core::ffi::c_int
            && (if av as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) != (*pBt).autoVacuum as ::core::ffi::c_int
        {
            rc = SQLITE_READONLY;
        } else {
            (*pBt).autoVacuum = (if av as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as u8_0;
            (*pBt).incrVacuum = (if av as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as u8_0;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeGetAutoVacuum(
    mut p: *mut Btree,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = if (*(*p).pBt).autoVacuum == 0 {
            BTREE_AUTOVACUUM_NONE
        } else if (*(*p).pBt).incrVacuum == 0 {
            BTREE_AUTOVACUUM_FULL
        } else {
            BTREE_AUTOVACUUM_INCR
        };
        return rc;
    }
}
unsafe extern "C" fn lockBtree(mut pBt: *mut BtShared) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pPage1: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut nPage: u32_0 = 0;
        let mut nPageFile: u32_0 = 0 as u32_0;
        rc = sqlite3PagerSharedLock((*pBt).pPager);
        if rc != SQLITE_OK {
            return rc;
        }
        rc = btreeGetPage(pBt, 1 as Pgno, &raw mut pPage1, 0 as ::core::ffi::c_int);
        if rc != SQLITE_OK {
            return rc;
        }
        nPage = sqlite3Get4byte(
            (*pPage1).aData.offset(28 as ::core::ffi::c_int as isize),
        );
        sqlite3PagerPagecount(
            (*pBt).pPager,
            &raw mut nPageFile as *mut ::core::ffi::c_int,
        );
        if nPage == 0 as ::core::ffi::c_uint
            || memcmp(
                (*pPage1).aData.offset(24 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                (*pPage1).aData.offset(92 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                4 as size_t,
            ) != 0 as ::core::ffi::c_int
        {
            nPage = nPageFile;
        }
        if (*(*pBt).db).flags as ::core::ffi::c_ulonglong
            & SQLITE_ResetDatabase as ::core::ffi::c_ulonglong
            != 0 as ::core::ffi::c_ulonglong
        {
            nPage = 0 as u32_0;
        }
        if nPage > 0 as ::core::ffi::c_uint {
            let mut pageSize: u32_0 = 0;
            let mut usableSize: u32_0 = 0;
            let mut page1: *mut u8_0 = (*pPage1).aData;
            rc = SQLITE_NOTADB;
            if memcmp(
                page1 as *const ::core::ffi::c_void,
                &raw const zMagicHeader as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                16 as size_t,
            ) != 0 as ::core::ffi::c_int
            {
                c2rust_current_block = 14561165556731603446;
            } else {
                if *page1.offset(18 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    > 2 as ::core::ffi::c_int
                {
                    (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int
                        | BTS_READ_ONLY) as u16_0;
                }
                if *page1.offset(19 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    > 2 as ::core::ffi::c_int
                {
                    c2rust_current_block = 14561165556731603446;
                } else {
                    if *page1.offset(19 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                        && (*pBt).btsFlags as ::core::ffi::c_int & BTS_NO_WAL
                            == 0 as ::core::ffi::c_int
                    {
                        let mut isOpen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        rc = sqlite3PagerOpenWal((*pBt).pPager, &raw mut isOpen);
                        if rc != SQLITE_OK {
                            c2rust_current_block = 14561165556731603446;
                        } else {
                            if isOpen == 0 as ::core::ffi::c_int {
                                releasePageOne(pPage1);
                                return SQLITE_OK;
                            }
                            rc = SQLITE_NOTADB;
                            c2rust_current_block = 17788412896529399552;
                        }
                    } else {
                        c2rust_current_block = 17788412896529399552;
                    }
                    match c2rust_current_block {
                        14561165556731603446 => {}
                        _ => {
                            if memcmp(
                                page1.offset(21 as ::core::ffi::c_int as isize) as *mut u8_0
                                    as *const ::core::ffi::c_void,
                                b"@  \0".as_ptr() as *const ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                3 as size_t,
                            ) != 0 as ::core::ffi::c_int
                            {
                                c2rust_current_block = 14561165556731603446;
                            } else {
                                pageSize = ((*page1
                                    .offset(16 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                    | (*page1.offset(17 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                                    as u32_0;
                                if pageSize.wrapping_sub(1 as u32_0) & pageSize
                                    != 0 as ::core::ffi::c_uint
                                    || pageSize > SQLITE_MAX_PAGE_SIZE as ::core::ffi::c_uint
                                    || pageSize <= 256 as ::core::ffi::c_uint
                                {
                                    c2rust_current_block = 14561165556731603446;
                                } else {
                                    usableSize = (pageSize as ::core::ffi::c_uint)
                                        .wrapping_sub(
                                            *page1.offset(20 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uint,
                                        ) as u32_0;
                                    if pageSize != (*pBt).pageSize {
                                        releasePageOne(pPage1);
                                        (*pBt).usableSize = usableSize;
                                        (*pBt).pageSize = pageSize;
                                        (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int
                                            | BTS_PAGESIZE_FIXED) as u16_0;
                                        freeTempSpace(pBt);
                                        rc = sqlite3PagerSetPagesize(
                                            (*pBt).pPager,
                                            &raw mut (*pBt).pageSize,
                                            pageSize.wrapping_sub(usableSize) as ::core::ffi::c_int,
                                        );
                                        return rc;
                                    }
                                    if nPage > nPageFile {
                                        if sqlite3WritableSchema((*pBt).db)
                                            == 0 as ::core::ffi::c_int
                                        {
                                            rc = sqlite3CorruptError(3403 as ::core::ffi::c_int);
                                            c2rust_current_block = 14561165556731603446;
                                        } else {
                                            nPage = nPageFile;
                                            c2rust_current_block = 6717214610478484138;
                                        }
                                    } else {
                                        c2rust_current_block = 6717214610478484138;
                                    }
                                    match c2rust_current_block {
                                        14561165556731603446 => {}
                                        _ => {
                                            if usableSize < 480 as ::core::ffi::c_uint {
                                                c2rust_current_block = 14561165556731603446;
                                            } else {
                                                (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int
                                                    | BTS_PAGESIZE_FIXED) as u16_0;
                                                (*pBt).pageSize = pageSize;
                                                (*pBt).usableSize = usableSize;
                                                (*pBt).autoVacuum = (if sqlite3Get4byte(
                                                    page1
                                                        .offset(
                                                            (36 as ::core::ffi::c_int
                                                                + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                                                as isize,
                                                        ) as *mut u8_0,
                                                ) != 0
                                                {
                                                    1 as ::core::ffi::c_int
                                                } else {
                                                    0 as ::core::ffi::c_int
                                                }) as u8_0;
                                                (*pBt).incrVacuum = (if sqlite3Get4byte(
                                                    page1
                                                        .offset(
                                                            (36 as ::core::ffi::c_int
                                                                + 7 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                                                as isize,
                                                        ) as *mut u8_0,
                                                ) != 0
                                                {
                                                    1 as ::core::ffi::c_int
                                                } else {
                                                    0 as ::core::ffi::c_int
                                                }) as u8_0;
                                                c2rust_current_block = 11793792312832361944;
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
                11793792312832361944 => {}
                _ => {
                    releasePageOne(pPage1);
                    (*pBt).pPage1 = ::core::ptr::null_mut::<MemPage>();
                    return rc;
                }
            }
        }
        (*pBt).maxLocal = ((*pBt).usableSize as ::core::ffi::c_uint)
            .wrapping_sub(12 as ::core::ffi::c_uint)
            .wrapping_mul(64 as ::core::ffi::c_uint)
            .wrapping_div(255 as ::core::ffi::c_uint)
            .wrapping_sub(23 as ::core::ffi::c_uint) as u16_0;
        (*pBt).minLocal = ((*pBt).usableSize as ::core::ffi::c_uint)
            .wrapping_sub(12 as ::core::ffi::c_uint)
            .wrapping_mul(32 as ::core::ffi::c_uint)
            .wrapping_div(255 as ::core::ffi::c_uint)
            .wrapping_sub(23 as ::core::ffi::c_uint) as u16_0;
        (*pBt).maxLeaf = ((*pBt).usableSize as ::core::ffi::c_uint)
            .wrapping_sub(35 as ::core::ffi::c_uint) as u16_0;
        (*pBt).minLeaf = ((*pBt).usableSize as ::core::ffi::c_uint)
            .wrapping_sub(12 as ::core::ffi::c_uint)
            .wrapping_mul(32 as ::core::ffi::c_uint)
            .wrapping_div(255 as ::core::ffi::c_uint)
            .wrapping_sub(23 as ::core::ffi::c_uint) as u16_0;
        if (*pBt).maxLocal as ::core::ffi::c_int > 127 as ::core::ffi::c_int {
            (*pBt).max1bytePayload = 127 as u8_0;
        } else {
            (*pBt).max1bytePayload = (*pBt).maxLocal as u8_0;
        }
        (*pBt).pPage1 = pPage1;
        (*pBt).nPage = nPage;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn unlockBtreeIfUnused(mut pBt: *mut BtShared) {
    unsafe {
        if (*pBt).inTransaction as ::core::ffi::c_int == TRANS_NONE
            && !(*pBt).pPage1.is_null()
        {
            let mut pPage1: *mut MemPage = (*pBt).pPage1;
            (*pBt).pPage1 = ::core::ptr::null_mut::<MemPage>();
            releasePageOne(pPage1);
        }
    }
}
unsafe extern "C" fn newDatabase(mut pBt: *mut BtShared) -> ::core::ffi::c_int {
    unsafe {
        let mut pP1: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut data: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        if (*pBt).nPage > 0 as ::core::ffi::c_uint {
            return SQLITE_OK;
        }
        pP1 = (*pBt).pPage1;
        data = (*pP1).aData as *mut ::core::ffi::c_uchar;
        rc = sqlite3PagerWrite((*pP1).pDbPage);
        if rc != 0 {
            return rc;
        }
        memcpy(
            data as *mut ::core::ffi::c_void,
            &raw const zMagicHeader as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
        );
        *data.offset(16 as ::core::ffi::c_int as isize) = ((*pBt).pageSize
            as ::core::ffi::c_uint >> 8 as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_uint) as u8_0 as ::core::ffi::c_uchar;
        *data.offset(17 as ::core::ffi::c_int as isize) = ((*pBt).pageSize
            as ::core::ffi::c_uint >> 16 as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_uint) as u8_0 as ::core::ffi::c_uchar;
        *data.offset(18 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_uchar;
        *data.offset(19 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_uchar;
        *data.offset(20 as ::core::ffi::c_int as isize) = (*pBt)
            .pageSize
            .wrapping_sub((*pBt).usableSize) as u8_0 as ::core::ffi::c_uchar;
        *data.offset(21 as ::core::ffi::c_int as isize) = 64 as ::core::ffi::c_uchar;
        *data.offset(22 as ::core::ffi::c_int as isize) = 32 as ::core::ffi::c_uchar;
        *data.offset(23 as ::core::ffi::c_int as isize) = 32 as ::core::ffi::c_uchar;
        memset(
            data.offset(24 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (100 as ::core::ffi::c_int - 24 as ::core::ffi::c_int) as size_t,
        );
        zeroPage(pP1, PTF_INTKEY | PTF_LEAF | PTF_LEAFDATA);
        (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int | BTS_PAGESIZE_FIXED)
            as u16_0;
        sqlite3Put4byte(
            data
                .offset(
                    (36 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                ) as *mut u8_0,
            (*pBt).autoVacuum as u32_0,
        );
        sqlite3Put4byte(
            data
                .offset(
                    (36 as ::core::ffi::c_int
                        + 7 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                ) as *mut u8_0,
            (*pBt).incrVacuum as u32_0,
        );
        (*pBt).nPage = 1 as u32_0;
        *data.offset(31 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_uchar;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeNewDb(mut p: *mut Btree) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        (*(*p).pBt).nPage = 0 as u32_0;
        rc = newDatabase((*p).pBt);
        return rc;
    }
}
#[inline(never)]
unsafe extern "C" fn btreeBeginTrans(
    mut p: *mut Btree,
    mut wrflag: ::core::ffi::c_int,
    mut pSchemaVersion: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        let mut pPager: *mut Pager = (*pBt).pPager;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !((*p).inTrans as ::core::ffi::c_int == TRANS_WRITE
            || (*p).inTrans as ::core::ffi::c_int == TRANS_READ && wrflag == 0)
        {
            if (*(*p).db).flags as ::core::ffi::c_ulonglong
                & SQLITE_ResetDatabase as ::core::ffi::c_ulonglong != 0
                && sqlite3PagerIsreadonly(pPager) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int
                    & !BTS_READ_ONLY) as u16_0;
            }
            if (*pBt).btsFlags as ::core::ffi::c_int & BTS_READ_ONLY
                != 0 as ::core::ffi::c_int && wrflag != 0
            {
                rc = SQLITE_READONLY;
            } else {
                rc = 0 as ::core::ffi::c_int;
                if !(SQLITE_OK != rc) {
                    (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int
                        & !BTS_INITIALLY_EMPTY) as u16_0;
                    if (*pBt).nPage == 0 as ::core::ffi::c_uint {
                        (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int
                            | BTS_INITIALLY_EMPTY) as u16_0;
                    }
                    loop {
                        while (*pBt).pPage1.is_null()
                            && {
                                rc = lockBtree(pBt);
                                SQLITE_OK == rc
                            }
                        {}
                        if rc == SQLITE_OK && wrflag != 0 {
                            if (*pBt).btsFlags as ::core::ffi::c_int & BTS_READ_ONLY
                                != 0 as ::core::ffi::c_int
                            {
                                rc = SQLITE_READONLY;
                            } else {
                                rc = sqlite3PagerBegin(
                                    pPager,
                                    (wrflag > 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                    sqlite3TempInMemory((*p).db),
                                );
                                if rc == SQLITE_OK {
                                    rc = newDatabase(pBt);
                                } else if rc == SQLITE_BUSY_SNAPSHOT
                                    && (*pBt).inTransaction as ::core::ffi::c_int == TRANS_NONE
                                {
                                    rc = SQLITE_BUSY;
                                }
                            }
                        }
                        if rc != SQLITE_OK {
                            unlockBtreeIfUnused(pBt);
                        }
                        if !(rc & 0xff as ::core::ffi::c_int == SQLITE_BUSY
                            && (*pBt).inTransaction as ::core::ffi::c_int == TRANS_NONE
                            && btreeInvokeBusyHandler(pBt as *mut ::core::ffi::c_void)
                                != 0)
                        {
                            break;
                        }
                    }
                    if rc == SQLITE_OK {
                        if (*p).inTrans as ::core::ffi::c_int == TRANS_NONE {
                            (*pBt).nTransaction += 1;
                        }
                        (*p).inTrans = (if wrflag != 0 {
                            TRANS_WRITE
                        } else {
                            TRANS_READ
                        }) as u8_0;
                        if (*p).inTrans as ::core::ffi::c_int
                            > (*pBt).inTransaction as ::core::ffi::c_int
                        {
                            (*pBt).inTransaction = (*p).inTrans;
                        }
                        if wrflag != 0 {
                            let mut pPage1: *mut MemPage = (*pBt).pPage1;
                            if (*pBt).nPage
                                != sqlite3Get4byte(
                                    (*pPage1).aData.offset(28 as ::core::ffi::c_int as isize)
                                        as *mut u8_0,
                                )
                            {
                                rc = sqlite3PagerWrite((*pPage1).pDbPage);
                                if rc == SQLITE_OK {
                                    sqlite3Put4byte(
                                        (*pPage1).aData.offset(28 as ::core::ffi::c_int as isize)
                                            as *mut u8_0,
                                        (*pBt).nPage,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        if rc == SQLITE_OK {
            if !pSchemaVersion.is_null() {
                *pSchemaVersion = sqlite3Get4byte(
                    (*(*pBt).pPage1).aData.offset(40 as ::core::ffi::c_int as isize)
                        as *mut u8_0,
                ) as ::core::ffi::c_int;
            }
            if wrflag != 0 {
                rc = sqlite3PagerOpenSavepoint(pPager, (*(*p).db).nSavepoint);
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeBeginTrans(
    mut p: *mut Btree,
    mut wrflag: ::core::ffi::c_int,
    mut pSchemaVersion: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = ::core::ptr::null_mut::<BtShared>();
        if (*p).sharable as ::core::ffi::c_int != 0
            || (*p).inTrans as ::core::ffi::c_int == TRANS_NONE
            || (*p).inTrans as ::core::ffi::c_int == TRANS_READ
                && wrflag != 0 as ::core::ffi::c_int
        {
            return btreeBeginTrans(p, wrflag, pSchemaVersion);
        }
        pBt = (*p).pBt;
        if !pSchemaVersion.is_null() {
            *pSchemaVersion = sqlite3Get4byte(
                (*(*pBt).pPage1).aData.offset(40 as ::core::ffi::c_int as isize)
                    as *mut u8_0,
            ) as ::core::ffi::c_int;
        }
        if wrflag != 0 {
            return sqlite3PagerOpenSavepoint((*pBt).pPager, (*(*p).db).nSavepoint)
        } else {
            return SQLITE_OK
        };
    }
}
unsafe extern "C" fn setChildPtrmaps(mut pPage: *mut MemPage) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut nCell: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pBt: *mut BtShared = (*pPage).pBt;
        let mut pgno: Pgno = (*pPage).pgno;
        rc = if (*pPage).isInit as ::core::ffi::c_int != 0 {
            SQLITE_OK
        } else {
            btreeInitPage(pPage)
        };
        if rc != SQLITE_OK {
            return rc;
        }
        nCell = (*pPage).nCell as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < nCell {
            let mut pCell: *mut u8_0 = (*pPage)
                .aData
                .offset(
                    ((*pPage).maskPage as ::core::ffi::c_int
                        & ((*((*pPage)
                            .aCellIdx
                            .offset((2 as ::core::ffi::c_int * i) as isize) as *mut u8_0)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                            | *((*pPage)
                                .aCellIdx
                                .offset((2 as ::core::ffi::c_int * i) as isize)
                                as *mut u8_0)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)) as isize,
                );
            ptrmapPutOvflPtr(pPage, pPage, pCell, &raw mut rc);
            if (*pPage).leaf == 0 {
                let mut childPgno: Pgno = sqlite3Get4byte(pCell) as Pgno;
                ptrmapPut(pBt, childPgno, PTRMAP_BTREE as u8_0, pgno, &raw mut rc);
            }
            i += 1;
        }
        if (*pPage).leaf == 0 {
            let mut childPgno_0: Pgno = sqlite3Get4byte(
                (*pPage)
                    .aData
                    .offset(
                        ((*pPage).hdrOffset as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int) as isize,
                    ) as *mut u8_0,
            ) as Pgno;
            ptrmapPut(pBt, childPgno_0, PTRMAP_BTREE as u8_0, pgno, &raw mut rc);
        }
        return rc;
    }
}
unsafe extern "C" fn modifyPagePointer(
    mut pPage: *mut MemPage,
    mut iFrom: Pgno,
    mut iTo: Pgno,
    mut eType: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        if eType as ::core::ffi::c_int == PTRMAP_OVERFLOW2 {
            if sqlite3Get4byte((*pPage).aData) != iFrom {
                return sqlite3CorruptError(3858 as ::core::ffi::c_int);
            }
            sqlite3Put4byte((*pPage).aData, iTo as u32_0);
        } else {
            let mut i: ::core::ffi::c_int = 0;
            let mut nCell: ::core::ffi::c_int = 0;
            let mut rc: ::core::ffi::c_int = 0;
            rc = if (*pPage).isInit as ::core::ffi::c_int != 0 {
                SQLITE_OK
            } else {
                btreeInitPage(pPage)
            };
            if rc != 0 {
                return rc;
            }
            nCell = (*pPage).nCell as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < nCell {
                let mut pCell: *mut u8_0 = (*pPage)
                    .aData
                    .offset(
                        ((*pPage).maskPage as ::core::ffi::c_int
                            & ((*((*pPage)
                                .aCellIdx
                                .offset((2 as ::core::ffi::c_int * i) as isize)
                                as *mut u8_0)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                | *((*pPage)
                                    .aCellIdx
                                    .offset((2 as ::core::ffi::c_int * i) as isize)
                                    as *mut u8_0)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int)) as isize,
                    );
                if eType as ::core::ffi::c_int == PTRMAP_OVERFLOW1 {
                    let mut info: CellInfo = CellInfo {
                        nKey: 0,
                        pPayload: ::core::ptr::null_mut::<u8_0>(),
                        nPayload: 0,
                        nLocal: 0,
                        nSize: 0

                    };
                    (*pPage)
                        .xParseCell
                        .expect(
                            "non-null function pointer",
                        )(pPage, pCell, &raw mut info);
                    if (info.nLocal as ::core::ffi::c_uint) < info.nPayload {
                        if pCell.offset(info.nSize as ::core::ffi::c_int as isize)
                            > (*pPage).aData.offset((*(*pPage).pBt).usableSize as isize)
                        {
                            return sqlite3CorruptError(3877 as ::core::ffi::c_int);
                        }
                        if iFrom
                            == sqlite3Get4byte(
                                pCell
                                    .offset(info.nSize as ::core::ffi::c_int as isize)
                                    .offset(-(4 as ::core::ffi::c_int as isize)),
                            )
                        {
                            sqlite3Put4byte(
                                pCell
                                    .offset(info.nSize as ::core::ffi::c_int as isize)
                                    .offset(-(4 as ::core::ffi::c_int as isize)),
                                iTo as u32_0,
                            );
                            break;
                        }
                    }
                } else {
                    if pCell.offset(4 as ::core::ffi::c_int as isize)
                        > (*pPage).aData.offset((*(*pPage).pBt).usableSize as isize)
                    {
                        return sqlite3CorruptError(3886 as ::core::ffi::c_int);
                    }
                    if sqlite3Get4byte(pCell) == iFrom {
                        sqlite3Put4byte(pCell, iTo as u32_0);
                        break;
                    }
                }
                i += 1;
            }
            if i == nCell {
                if eType as ::core::ffi::c_int != PTRMAP_BTREE
                    || sqlite3Get4byte(
                        (*pPage)
                            .aData
                            .offset(
                                ((*pPage).hdrOffset as ::core::ffi::c_int
                                    + 8 as ::core::ffi::c_int) as isize,
                            ) as *mut u8_0,
                    ) != iFrom
                {
                    return sqlite3CorruptError(3898 as ::core::ffi::c_int);
                }
                sqlite3Put4byte(
                    (*pPage)
                        .aData
                        .offset(
                            ((*pPage).hdrOffset as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int) as isize,
                        ) as *mut u8_0,
                    iTo as u32_0,
                );
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn relocatePage(
    mut pBt: *mut BtShared,
    mut pDbPage: *mut MemPage,
    mut eType: u8_0,
    mut iPtrPage: Pgno,
    mut iFreePage: Pgno,
    mut isCommit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPtrPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut iDbPage: Pgno = (*pDbPage).pgno;
        let mut pPager: *mut Pager = (*pBt).pPager;
        let mut rc: ::core::ffi::c_int = 0;
        if iDbPage < 3 as ::core::ffi::c_uint {
            return sqlite3CorruptError(3933 as ::core::ffi::c_int);
        }
        rc = sqlite3PagerMovepage(pPager, (*pDbPage).pDbPage, iFreePage, isCommit);
        if rc != SQLITE_OK {
            return rc;
        }
        (*pDbPage).pgno = iFreePage;
        if eType as ::core::ffi::c_int == PTRMAP_BTREE
            || eType as ::core::ffi::c_int == PTRMAP_ROOTPAGE
        {
            rc = setChildPtrmaps(pDbPage);
            if rc != SQLITE_OK {
                return rc;
            }
        } else {
            let mut nextOvfl: Pgno = sqlite3Get4byte((*pDbPage).aData) as Pgno;
            if nextOvfl != 0 as ::core::ffi::c_uint {
                ptrmapPut(
                    pBt,
                    nextOvfl,
                    PTRMAP_OVERFLOW2 as u8_0,
                    iFreePage,
                    &raw mut rc,
                );
                if rc != SQLITE_OK {
                    return rc;
                }
            }
        }
        if eType as ::core::ffi::c_int != PTRMAP_ROOTPAGE {
            rc = btreeGetPage(pBt, iPtrPage, &raw mut pPtrPage, 0 as ::core::ffi::c_int);
            if rc != SQLITE_OK {
                return rc;
            }
            rc = sqlite3PagerWrite((*pPtrPage).pDbPage);
            if rc != SQLITE_OK {
                releasePage(pPtrPage);
                return rc;
            }
            rc = modifyPagePointer(pPtrPage, iDbPage, iFreePage, eType);
            releasePage(pPtrPage);
            if rc == SQLITE_OK {
                ptrmapPut(pBt, iFreePage, eType, iPtrPage, &raw mut rc);
            }
        }
        return rc;
    }
}
unsafe extern "C" fn incrVacuumStep(
    mut pBt: *mut BtShared,
    mut nFin: Pgno,
    mut iLastPg: Pgno,
    mut bCommit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nFreeList: Pgno = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if !(ptrmapPageno(pBt, iLastPg) == iLastPg)
            && iLastPg
                != (sqlite3PendingByte as ::core::ffi::c_uint)
                    .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
        {
            let mut eType: u8_0 = 0;
            let mut iPtrPage: Pgno = 0;
            nFreeList = sqlite3Get4byte(
                (*(*pBt).pPage1).aData.offset(36 as ::core::ffi::c_int as isize)
                    as *mut u8_0,
            ) as Pgno;
            if nFreeList == 0 as ::core::ffi::c_uint {
                return SQLITE_DONE;
            }
            rc = ptrmapGet(pBt, iLastPg, &raw mut eType, &raw mut iPtrPage);
            if rc != SQLITE_OK {
                return rc;
            }
            if eType as ::core::ffi::c_int == PTRMAP_ROOTPAGE {
                return sqlite3CorruptError(4031 as ::core::ffi::c_int);
            }
            if eType as ::core::ffi::c_int == PTRMAP_FREEPAGE {
                if bCommit == 0 as ::core::ffi::c_int {
                    let mut iFreePg: Pgno = 0;
                    let mut pFreePg: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
                    rc = allocateBtreePage(
                        pBt,
                        &raw mut pFreePg,
                        &raw mut iFreePg,
                        iLastPg,
                        BTALLOC_EXACT as u8_0,
                    );
                    if rc != SQLITE_OK {
                        return rc;
                    }
                    releasePage(pFreePg);
                }
            } else {
                let mut iFreePg_0: Pgno = 0;
                let mut pLastPg: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
                let mut eMode: u8_0 = BTALLOC_ANY as u8_0;
                let mut iNear: Pgno = 0 as Pgno;
                rc = btreeGetPage(
                    pBt,
                    iLastPg,
                    &raw mut pLastPg,
                    0 as ::core::ffi::c_int,
                );
                if rc != SQLITE_OK {
                    return rc;
                }
                if bCommit == 0 as ::core::ffi::c_int {
                    eMode = BTALLOC_LE as u8_0;
                    iNear = nFin;
                }
                loop {
                    let mut pFreePg_0: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
                    let mut dbSize: Pgno = btreePagecount(pBt);
                    rc = allocateBtreePage(
                        pBt,
                        &raw mut pFreePg_0,
                        &raw mut iFreePg_0,
                        iNear,
                        eMode,
                    );
                    if rc != SQLITE_OK {
                        releasePage(pLastPg);
                        return rc;
                    }
                    releasePage(pFreePg_0);
                    if iFreePg_0 > dbSize {
                        releasePage(pLastPg);
                        return sqlite3CorruptError(4083 as ::core::ffi::c_int);
                    }
                    if !(bCommit != 0 && iFreePg_0 > nFin) {
                        break;
                    }
                }
                rc = relocatePage(pBt, pLastPg, eType, iPtrPage, iFreePg_0, bCommit);
                releasePage(pLastPg);
                if rc != SQLITE_OK {
                    return rc;
                }
            }
        }
        if bCommit == 0 as ::core::ffi::c_int {
            loop {
                iLastPg = iLastPg.wrapping_sub(1);
                if !(iLastPg
                    == (sqlite3PendingByte as ::core::ffi::c_uint)
                        .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint)
                    || ptrmapPageno(pBt, iLastPg) == iLastPg)
                {
                    break;
                }
            }
            (*pBt).bDoTruncate = 1 as u8_0;
            (*pBt).nPage = iLastPg as u32_0;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn finalDbSize(
    mut pBt: *mut BtShared,
    mut nOrig: Pgno,
    mut nFree: Pgno,
) -> Pgno {
    unsafe {
        let mut nEntry: ::core::ffi::c_int = 0;
        let mut nPtrmap: Pgno = 0;
        let mut nFin: Pgno = 0;
        nEntry = ((*pBt).usableSize as ::core::ffi::c_uint)
            .wrapping_div(5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        nPtrmap = (nFree as ::core::ffi::c_uint)
            .wrapping_sub(nOrig as ::core::ffi::c_uint)
            .wrapping_add(ptrmapPageno(pBt, nOrig) as ::core::ffi::c_uint)
            .wrapping_add(nEntry as ::core::ffi::c_uint)
            .wrapping_div(nEntry as ::core::ffi::c_uint) as Pgno;
        nFin = nOrig.wrapping_sub(nFree).wrapping_sub(nPtrmap);
        if nOrig
            > (sqlite3PendingByte as ::core::ffi::c_uint)
                .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint)
            && nFin
                < (sqlite3PendingByte as ::core::ffi::c_uint)
                    .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
        {
            nFin = nFin.wrapping_sub(1);
        }
        while ptrmapPageno(pBt, nFin) == nFin
            || nFin
                == (sqlite3PendingByte as ::core::ffi::c_uint)
                    .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
        {
            nFin = nFin.wrapping_sub(1);
        }
        return nFin;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeIncrVacuum(
    mut p: *mut Btree,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pBt: *mut BtShared = (*p).pBt;
        if (*pBt).autoVacuum == 0 {
            rc = SQLITE_DONE;
        } else {
            let mut nOrig: Pgno = btreePagecount(pBt);
            let mut nFree: Pgno = sqlite3Get4byte(
                (*(*pBt).pPage1).aData.offset(36 as ::core::ffi::c_int as isize)
                    as *mut u8_0,
            ) as Pgno;
            let mut nFin: Pgno = finalDbSize(pBt, nOrig, nFree);
            if nOrig < nFin || nFree >= nOrig {
                rc = sqlite3CorruptError(4151 as ::core::ffi::c_int);
            } else if nFree > 0 as ::core::ffi::c_uint {
                rc = saveAllCursors(pBt, 0 as Pgno, ::core::ptr::null_mut::<BtCursor>());
                if rc == SQLITE_OK {
                    invalidateAllOverflowCache(pBt);
                    rc = incrVacuumStep(pBt, nFin, nOrig, 0 as ::core::ffi::c_int);
                }
                if rc == SQLITE_OK {
                    rc = sqlite3PagerWrite((*(*pBt).pPage1).pDbPage);
                    sqlite3Put4byte(
                        (*(*pBt).pPage1).aData.offset(28 as ::core::ffi::c_int as isize)
                            as *mut u8_0,
                        (*pBt).nPage,
                    );
                }
            } else {
                rc = SQLITE_DONE;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn autoVacuumCommit(mut p: *mut Btree) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut pBt: *mut BtShared = ::core::ptr::null_mut::<BtShared>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        pBt = (*p).pBt;
        pPager = (*pBt).pPager;
        invalidateAllOverflowCache(pBt);
        if (*pBt).incrVacuum == 0 {
            let mut nFin: Pgno = 0;
            let mut nFree: Pgno = 0;
            let mut nVac: Pgno = 0;
            let mut iFree: Pgno = 0;
            let mut nOrig: Pgno = 0;
            nOrig = btreePagecount(pBt);
            if ptrmapPageno(pBt, nOrig) == nOrig
                || nOrig
                    == (sqlite3PendingByte as ::core::ffi::c_uint)
                        .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint)
            {
                return sqlite3CorruptError(4202 as ::core::ffi::c_int);
            }
            nFree = sqlite3Get4byte(
                (*(*pBt).pPage1).aData.offset(36 as ::core::ffi::c_int as isize)
                    as *mut u8_0,
            ) as Pgno;
            db = (*p).db;
            if (*db).xAutovacPages.is_some() {
                let mut iDb: ::core::ffi::c_int = 0;
                iDb = 0 as ::core::ffi::c_int;
                while iDb < (*db).nDb {
                    if (*(*db).aDb.offset(iDb as isize)).pBt == p {
                        break;
                    }
                    iDb += 1;
                }
                nVac = (*db)
                    .xAutovacPages
                    .expect(
                        "non-null function pointer",
                    )(
                    (*db).pAutovacPagesArg,
                    (*(*db).aDb.offset(iDb as isize)).zDbSName,
                    nOrig as u32_0,
                    nFree as u32_0,
                    (*pBt).pageSize,
                ) as Pgno;
                if nVac > nFree {
                    nVac = nFree;
                }
                if nVac == 0 as ::core::ffi::c_uint {
                    return SQLITE_OK;
                }
            } else {
                nVac = nFree;
            }
            nFin = finalDbSize(pBt, nOrig, nVac);
            if nFin > nOrig {
                return sqlite3CorruptError(4229 as ::core::ffi::c_int);
            }
            if nFin < nOrig {
                rc = saveAllCursors(pBt, 0 as Pgno, ::core::ptr::null_mut::<BtCursor>());
            }
            iFree = nOrig;
            while iFree > nFin && rc == SQLITE_OK {
                rc = incrVacuumStep(
                    pBt,
                    nFin,
                    iFree,
                    (nVac == nFree) as ::core::ffi::c_int,
                );
                iFree = iFree.wrapping_sub(1);
            }
            if (rc == SQLITE_DONE || rc == SQLITE_OK) && nFree > 0 as ::core::ffi::c_uint
            {
                rc = sqlite3PagerWrite((*(*pBt).pPage1).pDbPage);
                if nVac == nFree {
                    sqlite3Put4byte(
                        (*(*pBt).pPage1).aData.offset(32 as ::core::ffi::c_int as isize)
                            as *mut u8_0,
                        0 as u32_0,
                    );
                    sqlite3Put4byte(
                        (*(*pBt).pPage1).aData.offset(36 as ::core::ffi::c_int as isize)
                            as *mut u8_0,
                        0 as u32_0,
                    );
                }
                sqlite3Put4byte(
                    (*(*pBt).pPage1).aData.offset(28 as ::core::ffi::c_int as isize)
                        as *mut u8_0,
                    nFin as u32_0,
                );
                (*pBt).bDoTruncate = 1 as u8_0;
                (*pBt).nPage = nFin as u32_0;
            }
            if rc != SQLITE_OK {
                sqlite3PagerRollback(pPager);
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCommitPhaseOne(
    mut p: *mut Btree,
    mut zSuperJrnl: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*p).inTrans as ::core::ffi::c_int == TRANS_WRITE {
            let mut pBt: *mut BtShared = (*p).pBt;
            if (*pBt).autoVacuum != 0 {
                rc = autoVacuumCommit(p);
                if rc != SQLITE_OK {
                    return rc;
                }
            }
            if (*pBt).bDoTruncate != 0 {
                sqlite3PagerTruncateImage((*pBt).pPager, (*pBt).nPage as Pgno);
            }
            rc = sqlite3PagerCommitPhaseOne(
                (*pBt).pPager,
                zSuperJrnl,
                0 as ::core::ffi::c_int,
            );
        }
        return rc;
    }
}
unsafe extern "C" fn btreeEndTransaction(mut p: *mut Btree) {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        let mut db: *mut sqlite3 = (*p).db;
        (*pBt).bDoTruncate = 0 as u8_0;
        if (*p).inTrans as ::core::ffi::c_int > TRANS_NONE
            && (*db).nVdbeRead > 1 as ::core::ffi::c_int
        {
            (*p).inTrans = TRANS_READ as u8_0;
        } else {
            if (*p).inTrans as ::core::ffi::c_int != TRANS_NONE {
                (*pBt).nTransaction -= 1;
                if 0 as ::core::ffi::c_int == (*pBt).nTransaction {
                    (*pBt).inTransaction = TRANS_NONE as u8_0;
                }
            }
            (*p).inTrans = TRANS_NONE as u8_0;
            unlockBtreeIfUnused(pBt);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCommitPhaseTwo(
    mut p: *mut Btree,
    mut bCleanup: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if (*p).inTrans as ::core::ffi::c_int == TRANS_NONE {
            return SQLITE_OK;
        }
        if (*p).inTrans as ::core::ffi::c_int == TRANS_WRITE {
            let mut rc: ::core::ffi::c_int = 0;
            let mut pBt: *mut BtShared = (*p).pBt;
            rc = sqlite3PagerCommitPhaseTwo((*pBt).pPager);
            if rc != SQLITE_OK && bCleanup == 0 as ::core::ffi::c_int {
                return rc;
            }
            (*p).iBDataVersion = (*p).iBDataVersion.wrapping_sub(1);
            (*pBt).inTransaction = TRANS_READ as u8_0;
            btreeClearHasContent(pBt);
        }
        btreeEndTransaction(p);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCommit(mut p: *mut Btree) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3BtreeCommitPhaseOne(p, ::core::ptr::null::<::core::ffi::c_char>());
        if rc == SQLITE_OK {
            rc = sqlite3BtreeCommitPhaseTwo(p, 0 as ::core::ffi::c_int);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeTripAllCursors(
    mut pBtree: *mut Btree,
    mut errCode: ::core::ffi::c_int,
    mut writeOnly: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !pBtree.is_null() {
            p = (*(*pBtree).pBt).pCursor;
            while !p.is_null() {
                if writeOnly != 0
                    && (*p).curFlags as ::core::ffi::c_int & BTCF_WriteFlag
                        == 0 as ::core::ffi::c_int
                {
                    if (*p).eState as ::core::ffi::c_int == CURSOR_VALID
                        || (*p).eState as ::core::ffi::c_int == CURSOR_SKIPNEXT
                    {
                        rc = saveCursorPosition(p);
                        if rc != SQLITE_OK {
                            sqlite3BtreeTripAllCursors(
                                pBtree,
                                rc,
                                0 as ::core::ffi::c_int,
                            );
                            break;
                        }
                    }
                } else {
                    sqlite3BtreeClearCursor(p);
                    (*p).eState = CURSOR_FAULT as u8_0;
                    (*p).skipNext = errCode;
                }
                btreeReleaseAllCursorPages(p);
                p = (*p).pNext;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn btreeSetNPage(mut pBt: *mut BtShared, mut pPage1: *mut MemPage) {
    unsafe {
        let mut nPage: ::core::ffi::c_int = sqlite3Get4byte(
            (*pPage1).aData.offset(28 as ::core::ffi::c_int as isize) as *mut u8_0,
        ) as ::core::ffi::c_int;
        if nPage == 0 as ::core::ffi::c_int {
            sqlite3PagerPagecount((*pBt).pPager, &raw mut nPage);
        }
        (*pBt).nPage = nPage as u32_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeRollback(
    mut p: *mut Btree,
    mut tripCode: ::core::ffi::c_int,
    mut writeOnly: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pBt: *mut BtShared = (*p).pBt;
        let mut pPage1: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        if tripCode == SQLITE_OK {
            tripCode = saveAllCursors(
                pBt,
                0 as Pgno,
                ::core::ptr::null_mut::<BtCursor>(),
            );
            rc = tripCode;
            if rc != 0 {
                writeOnly = 0 as ::core::ffi::c_int;
            }
        } else {
            rc = SQLITE_OK;
        }
        if tripCode != 0 {
            let mut rc2: ::core::ffi::c_int = sqlite3BtreeTripAllCursors(
                p,
                tripCode,
                writeOnly,
            );
            if rc2 != SQLITE_OK {
                rc = rc2;
            }
        }
        if (*p).inTrans as ::core::ffi::c_int == TRANS_WRITE {
            let mut rc2_0: ::core::ffi::c_int = 0;
            rc2_0 = sqlite3PagerRollback((*pBt).pPager);
            if rc2_0 != SQLITE_OK {
                rc = rc2_0;
            }
            if btreeGetPage(pBt, 1 as Pgno, &raw mut pPage1, 0 as ::core::ffi::c_int)
                == SQLITE_OK
            {
                btreeSetNPage(pBt, pPage1);
                releasePageOne(pPage1);
            }
            (*pBt).inTransaction = TRANS_READ as u8_0;
            btreeClearHasContent(pBt);
        }
        btreeEndTransaction(p);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeBeginStmt(
    mut p: *mut Btree,
    mut iStatement: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pBt: *mut BtShared = (*p).pBt;
        rc = sqlite3PagerOpenSavepoint((*pBt).pPager, iStatement);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeSavepoint(
    mut p: *mut Btree,
    mut op: ::core::ffi::c_int,
    mut iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !p.is_null() && (*p).inTrans as ::core::ffi::c_int == TRANS_WRITE {
            let mut pBt: *mut BtShared = (*p).pBt;
            if op == SAVEPOINT_ROLLBACK {
                rc = saveAllCursors(pBt, 0 as Pgno, ::core::ptr::null_mut::<BtCursor>());
            }
            if rc == SQLITE_OK {
                rc = sqlite3PagerSavepoint((*pBt).pPager, op, iSavepoint);
            }
            if rc == SQLITE_OK {
                if iSavepoint < 0 as ::core::ffi::c_int
                    && (*pBt).btsFlags as ::core::ffi::c_int & BTS_INITIALLY_EMPTY
                        != 0 as ::core::ffi::c_int
                {
                    (*pBt).nPage = 0 as u32_0;
                }
                rc = newDatabase(pBt);
                btreeSetNPage(pBt, (*pBt).pPage1);
            }
        }
        return rc;
    }
}
unsafe extern "C" fn btreeCursor(
    mut p: *mut Btree,
    mut iTable: Pgno,
    mut wrFlag: ::core::ffi::c_int,
    mut pKeyInfo: *mut KeyInfo,
    mut pCur: *mut BtCursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        let mut pX: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        if iTable <= 1 as ::core::ffi::c_uint {
            if iTable < 1 as ::core::ffi::c_uint {
                return sqlite3CorruptError(4693 as ::core::ffi::c_int)
            } else if btreePagecount(pBt) == 0 as ::core::ffi::c_uint {
                iTable = 0 as Pgno;
            }
        }
        (*pCur).pgnoRoot = iTable;
        (*pCur).iPage = -1 as ::core::ffi::c_int as i8_0;
        (*pCur).pKeyInfo = pKeyInfo;
        (*pCur).pBtree = p;
        (*pCur).pBt = pBt;
        (*pCur).curFlags = 0 as u8_0;
        pX = (*pBt).pCursor;
        while !pX.is_null() {
            if (*pX).pgnoRoot == iTable {
                (*pX).curFlags = ((*pX).curFlags as ::core::ffi::c_int | BTCF_Multiple)
                    as u8_0;
                (*pCur).curFlags = BTCF_Multiple as u8_0;
            }
            pX = (*pX).pNext;
        }
        (*pCur).eState = CURSOR_INVALID as u8_0;
        (*pCur).pNext = (*pBt).pCursor;
        (*pBt).pCursor = pCur;
        if wrFlag != 0 {
            (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int | BTCF_WriteFlag)
                as u8_0;
            (*pCur).curPagerFlags = 0 as u8_0;
            if (*pBt).pTmpSpace.is_null() {
                return allocateTempSpace(pBt);
            }
        } else {
            (*pCur).curPagerFlags = PAGER_GET_READONLY as u8_0;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn btreeCursorWithLock(
    mut p: *mut Btree,
    mut iTable: Pgno,
    mut wrFlag: ::core::ffi::c_int,
    mut pKeyInfo: *mut KeyInfo,
    mut pCur: *mut BtCursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = btreeCursor(p, iTable, wrFlag, pKeyInfo, pCur);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCursor(
    mut p: *mut Btree,
    mut iTable: Pgno,
    mut wrFlag: ::core::ffi::c_int,
    mut pKeyInfo: *mut KeyInfo,
    mut pCur: *mut BtCursor,
) -> ::core::ffi::c_int {
    unsafe {
        if (*p).sharable != 0 {
            return btreeCursorWithLock(p, iTable, wrFlag, pKeyInfo, pCur)
        } else {
            return btreeCursor(p, iTable, wrFlag, pKeyInfo, pCur)
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCursorSize() -> ::core::ffi::c_int {
    unsafe {
        return ((::core::mem::size_of::<BtCursor>() as usize).wrapping_add(7 as usize)
            & !(7 as ::core::ffi::c_int) as usize) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCursorZero(mut p: *mut BtCursor) {
    unsafe {
        memset(p as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, 32 as size_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCloseCursor(
    mut pCur: *mut BtCursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBtree: *mut Btree = (*pCur).pBtree;
        if !pBtree.is_null() {
            let mut pBt: *mut BtShared = (*pCur).pBt;
            if (*pBt).pCursor == pCur {
                (*pBt).pCursor = (*pCur).pNext;
            } else {
                let mut pPrev: *mut BtCursor = (*pBt).pCursor;
                loop {
                    if (*pPrev).pNext == pCur {
                        (*pPrev).pNext = (*pCur).pNext;
                        break;
                    } else {
                        pPrev = (*pPrev).pNext;
                        if pPrev.is_null() {
                            break;
                        }
                    }
                }
            }
            btreeReleaseAllCursorPages(pCur);
            unlockBtreeIfUnused(pBt);
            sqlite3_free((*pCur).aOverflow as *mut ::core::ffi::c_void);
            sqlite3_free((*pCur).pKey);
            if (*pBt).openFlags as ::core::ffi::c_int & BTREE_SINGLE != 0
                && (*pBt).pCursor.is_null()
            {
                sqlite3BtreeClose(pBtree);
            }
            (*pCur).pBtree = ::core::ptr::null_mut::<Btree>();
        }
        return SQLITE_OK;
    }
}
#[inline(never)]
unsafe extern "C" fn getCellInfo(mut pCur: *mut BtCursor) {
    unsafe {
        if (*pCur).info.nSize as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int | BTCF_ValidNKey)
                as u8_0;
            btreeParseCell(
                (*pCur).pPage,
                (*pCur).ix as ::core::ffi::c_int,
                &raw mut (*pCur).info,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCursorIsValidNN(
    mut pCur: *mut BtCursor,
) -> ::core::ffi::c_int {
    unsafe {
        return ((*pCur).eState as ::core::ffi::c_int == CURSOR_VALID)
            as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeIntegerKey(mut pCur: *mut BtCursor) -> i64_0 {
    unsafe {
        getCellInfo(pCur);
        return (*pCur).info.nKey;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCursorPin(mut pCur: *mut BtCursor) {
    unsafe {
        (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int | BTCF_Pinned)
            as u8_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCursorUnpin(mut pCur: *mut BtCursor) {
    unsafe {
        (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int & !BTCF_Pinned)
            as u8_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeOffset(mut pCur: *mut BtCursor) -> i64_0 {
    unsafe {
        getCellInfo(pCur);
        return (*(*pCur).pBt).pageSize as i64_0
            * ((*(*pCur).pPage).pgno as i64_0 - 1 as i64_0)
            + (*pCur).info.pPayload.offset_from((*(*pCur).pPage).aData)
                as ::core::ffi::c_long as i64_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreePayloadSize(mut pCur: *mut BtCursor) -> u32_0 {
    unsafe {
        getCellInfo(pCur);
        return (*pCur).info.nPayload;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeMaxRecordSize(
    mut pCur: *mut BtCursor,
) -> sqlite3_int64 {
    unsafe {
        return (*(*pCur).pBt).pageSize as sqlite3_int64
            * (*(*pCur).pBt).nPage as sqlite3_int64;
    }
}
unsafe extern "C" fn getOverflowPage(
    mut pBt: *mut BtShared,
    mut ovfl: Pgno,
    mut ppPage: *mut *mut MemPage,
    mut pPgnoNext: *mut Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        let mut next: Pgno = 0 as Pgno;
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pBt).autoVacuum != 0 {
            let mut pgno: Pgno = 0;
            let mut iGuess: Pgno = ovfl.wrapping_add(1 as Pgno);
            let mut eType: u8_0 = 0;
            while ptrmapPageno(pBt, iGuess) == iGuess
                || iGuess
                    == (sqlite3PendingByte as ::core::ffi::c_uint)
                        .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint)
            {
                iGuess = iGuess.wrapping_add(1);
            }
            if iGuess <= btreePagecount(pBt) {
                rc = ptrmapGet(pBt, iGuess, &raw mut eType, &raw mut pgno);
                if rc == SQLITE_OK && eType as ::core::ffi::c_int == PTRMAP_OVERFLOW2
                    && pgno == ovfl
                {
                    next = iGuess;
                    rc = SQLITE_DONE;
                }
            }
        }
        if rc == SQLITE_OK {
            rc = btreeGetPage(
                pBt,
                ovfl,
                &raw mut pPage,
                if ppPage.is_null() {
                    PAGER_GET_READONLY
                } else {
                    0 as ::core::ffi::c_int
                },
            );
            if rc == SQLITE_OK {
                next = sqlite3Get4byte((*pPage).aData) as Pgno;
            }
        }
        *pPgnoNext = next;
        if !ppPage.is_null() {
            *ppPage = pPage;
        } else {
            releasePage(pPage);
        }
        return if rc == SQLITE_DONE { SQLITE_OK } else { rc };
    }
}
unsafe extern "C" fn copyPayload(
    mut pPayload: *mut ::core::ffi::c_void,
    mut pBuf: *mut ::core::ffi::c_void,
    mut nByte: ::core::ffi::c_int,
    mut eOp: ::core::ffi::c_int,
    mut pDbPage: *mut DbPage,
) -> ::core::ffi::c_int {
    unsafe {
        if eOp != 0 {
            let mut rc: ::core::ffi::c_int = sqlite3PagerWrite(pDbPage);
            if rc != SQLITE_OK {
                return rc;
            }
            memcpy(pPayload, pBuf, nByte as size_t);
        } else {
            memcpy(pBuf, pPayload, nByte as size_t);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn accessPayload(
    mut pCur: *mut BtCursor,
    mut offset: u32_0,
    mut amt: u32_0,
    mut pBuf: *mut ::core::ffi::c_uchar,
    mut eOp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aPayload: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut iIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pPage: *mut MemPage = (*pCur).pPage;
        let mut pBt: *mut BtShared = (*pCur).pBt;
        let pBufStart: *mut ::core::ffi::c_uchar = pBuf;
        if (*pCur).ix as ::core::ffi::c_int >= (*pPage).nCell as ::core::ffi::c_int {
            return sqlite3CorruptError(5117 as ::core::ffi::c_int);
        }
        getCellInfo(pCur);
        aPayload = (*pCur).info.pPayload as *mut ::core::ffi::c_uchar;
        if aPayload.offset_from((*pPage).aData) as ::core::ffi::c_long as uptr
            > ((*pBt).usableSize as ::core::ffi::c_uint)
                .wrapping_sub((*pCur).info.nLocal as ::core::ffi::c_uint)
                as ::core::ffi::c_ulonglong
        {
            return sqlite3CorruptError(5132 as ::core::ffi::c_int);
        }
        if offset < (*pCur).info.nLocal as ::core::ffi::c_uint {
            let mut a: ::core::ffi::c_int = amt as ::core::ffi::c_int;
            if (a as u32_0).wrapping_add(offset)
                > (*pCur).info.nLocal as ::core::ffi::c_uint
            {
                a = ((*pCur).info.nLocal as u32_0).wrapping_sub(offset)
                    as ::core::ffi::c_int;
            }
            rc = copyPayload(
                aPayload.offset(offset as isize) as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_void,
                pBuf as *mut ::core::ffi::c_void,
                a,
                eOp,
                (*pPage).pDbPage,
            );
            offset = 0 as u32_0;
            pBuf = pBuf.offset(a as isize);
            amt = (amt as ::core::ffi::c_uint).wrapping_sub(a as ::core::ffi::c_uint)
                as u32_0 as u32_0;
        } else {
            offset = (offset as ::core::ffi::c_uint)
                .wrapping_sub((*pCur).info.nLocal as ::core::ffi::c_uint) as u32_0
                as u32_0;
        }
        if rc == SQLITE_OK && amt > 0 as ::core::ffi::c_uint {
            let ovflSize: u32_0 = (*pBt).usableSize.wrapping_sub(4 as u32_0);
            let mut nextPage: Pgno = 0;
            nextPage = sqlite3Get4byte(
                aPayload.offset((*pCur).info.nLocal as isize)
                    as *mut ::core::ffi::c_uchar,
            ) as Pgno;
            if (*pCur).curFlags as ::core::ffi::c_int & BTCF_ValidOvfl
                == 0 as ::core::ffi::c_int
            {
                let mut nOvfl: ::core::ffi::c_int = (*pCur)
                    .info
                    .nPayload
                    .wrapping_sub((*pCur).info.nLocal as u32_0)
                    .wrapping_add(ovflSize)
                    .wrapping_sub(1 as u32_0)
                    .wrapping_div(ovflSize) as ::core::ffi::c_int;
                if (*pCur).aOverflow.is_null()
                    || nOvfl * ::core::mem::size_of::<Pgno>() as ::core::ffi::c_int
                        > sqlite3MallocSize(
                            (*pCur).aOverflow as *const ::core::ffi::c_void,
                        )
                {
                    let mut aNew: *mut Pgno = ::core::ptr::null_mut::<Pgno>();
                    if sqlite3FaultSim(413 as ::core::ffi::c_int) != 0 {
                        aNew = ::core::ptr::null_mut::<Pgno>();
                    } else {
                        aNew = sqlite3Realloc(
                            (*pCur).aOverflow as *mut ::core::ffi::c_void,
                            ((nOvfl * 2 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<Pgno>() as usize)
                                as u64_0,
                        ) as *mut Pgno;
                    }
                    if aNew.is_null() {
                        return SQLITE_NOMEM_BKPT
                    } else {
                        (*pCur).aOverflow = aNew;
                    }
                }
                memset(
                    (*pCur).aOverflow as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (nOvfl as size_t)
                        .wrapping_mul(::core::mem::size_of::<Pgno>() as size_t),
                );
                (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
                    | BTCF_ValidOvfl) as u8_0;
            } else if *(*pCur).aOverflow.offset(offset.wrapping_div(ovflSize) as isize)
                != 0
            {
                iIdx = offset.wrapping_div(ovflSize) as ::core::ffi::c_int;
                nextPage = *(*pCur).aOverflow.offset(iIdx as isize);
                offset = offset.wrapping_rem(ovflSize);
            }
            while nextPage != 0 {
                if nextPage > (*pBt).nPage {
                    return sqlite3CorruptError(5203 as ::core::ffi::c_int);
                }
                *(*pCur).aOverflow.offset(iIdx as isize) = nextPage;
                if offset >= ovflSize {
                    if *(*pCur)
                        .aOverflow
                        .offset((iIdx + 1 as ::core::ffi::c_int) as isize) != 0
                    {
                        nextPage = *(*pCur)
                            .aOverflow
                            .offset((iIdx + 1 as ::core::ffi::c_int) as isize);
                    } else {
                        rc = getOverflowPage(
                            pBt,
                            nextPage,
                            ::core::ptr::null_mut::<*mut MemPage>(),
                            &raw mut nextPage,
                        );
                    }
                    offset = (offset as ::core::ffi::c_uint)
                        .wrapping_sub(ovflSize as ::core::ffi::c_uint) as u32_0 as u32_0;
                } else {
                    let mut a_0: ::core::ffi::c_int = amt as ::core::ffi::c_int;
                    if (a_0 as u32_0).wrapping_add(offset) > ovflSize {
                        a_0 = ovflSize.wrapping_sub(offset) as ::core::ffi::c_int;
                    }
                    if eOp == 0 as ::core::ffi::c_int
                        && offset == 0 as ::core::ffi::c_uint
                        && sqlite3PagerDirectReadOk((*pBt).pPager, nextPage) != 0
                        && pBuf.offset(-4 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uchar >= pBufStart
                    {
                        let mut fd: *mut sqlite3_file = sqlite3PagerFile((*pBt).pPager);
                        let mut aSave: [u8_0; 4] = [0; 4];
                        let mut aWrite: *mut u8_0 = pBuf
                            .offset(-4 as ::core::ffi::c_int as isize) as *mut u8_0;
                        memcpy(
                            &raw mut aSave as *mut u8_0 as *mut ::core::ffi::c_void,
                            aWrite as *const ::core::ffi::c_void,
                            4 as size_t,
                        );
                        rc = sqlite3OsRead(
                            fd,
                            aWrite as *mut ::core::ffi::c_void,
                            a_0 + 4 as ::core::ffi::c_int,
                            (*pBt).pageSize as i64_0
                                * (nextPage as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint) as i64_0,
                        );
                        nextPage = sqlite3Get4byte(aWrite) as Pgno;
                        memcpy(
                            aWrite as *mut ::core::ffi::c_void,
                            &raw mut aSave as *mut u8_0 as *const ::core::ffi::c_void,
                            4 as size_t,
                        );
                    } else {
                        let mut pDbPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
                        rc = sqlite3PagerGet(
                            (*pBt).pPager,
                            nextPage,
                            &raw mut pDbPage,
                            if eOp == 0 as ::core::ffi::c_int {
                                PAGER_GET_READONLY
                            } else {
                                0 as ::core::ffi::c_int
                            },
                        );
                        if rc == SQLITE_OK {
                            aPayload = sqlite3PagerGetData(pDbPage)
                                as *mut ::core::ffi::c_uchar;
                            nextPage = sqlite3Get4byte(aPayload) as Pgno;
                            rc = copyPayload(
                                aPayload
                                    .offset(
                                        (offset as ::core::ffi::c_uint)
                                            .wrapping_add(4 as ::core::ffi::c_uint) as isize,
                                    ) as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                                pBuf as *mut ::core::ffi::c_void,
                                a_0,
                                eOp,
                                pDbPage,
                            );
                            sqlite3PagerUnref(pDbPage);
                            offset = 0 as u32_0;
                        }
                    }
                    amt = (amt as ::core::ffi::c_uint)
                        .wrapping_sub(a_0 as ::core::ffi::c_uint) as u32_0 as u32_0;
                    if amt == 0 as ::core::ffi::c_uint {
                        return rc;
                    }
                    pBuf = pBuf.offset(a_0 as isize);
                }
                if rc != 0 {
                    break;
                }
                iIdx += 1;
            }
        }
        if rc == SQLITE_OK && amt > 0 as ::core::ffi::c_uint {
            return sqlite3CorruptError(5287 as ::core::ffi::c_int);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreePayload(
    mut pCur: *mut BtCursor,
    mut offset: u32_0,
    mut amt: u32_0,
    mut pBuf: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        return accessPayload(
            pCur,
            offset,
            amt,
            pBuf as *mut ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int,
        );
    }
}
#[inline(never)]
unsafe extern "C" fn accessPayloadChecked(
    mut pCur: *mut BtCursor,
    mut offset: u32_0,
    mut amt: u32_0,
    mut pBuf: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if (*pCur).eState as ::core::ffi::c_int == CURSOR_INVALID {
            return SQLITE_ABORT;
        }
        rc = btreeRestoreCursorPosition(pCur);
        return if rc != 0 {
            rc
        } else {
            accessPayload(
                pCur,
                offset,
                amt,
                pBuf as *mut ::core::ffi::c_uchar,
                0 as ::core::ffi::c_int,
            )
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreePayloadChecked(
    mut pCur: *mut BtCursor,
    mut offset: u32_0,
    mut amt: u32_0,
    mut pBuf: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pCur).eState as ::core::ffi::c_int == CURSOR_VALID {
            return accessPayload(
                pCur,
                offset,
                amt,
                pBuf as *mut ::core::ffi::c_uchar,
                0 as ::core::ffi::c_int,
            )
        } else {
            return accessPayloadChecked(pCur, offset, amt, pBuf)
        };
    }
}
unsafe extern "C" fn fetchPayload(
    mut pCur: *mut BtCursor,
    mut pAmt: *mut u32_0,
) -> *const ::core::ffi::c_void {
    unsafe {
        let mut amt: ::core::ffi::c_int = 0;
        amt = (*pCur).info.nLocal as ::core::ffi::c_int;
        if amt
            > (*(*pCur).pPage).aDataEnd.offset_from((*pCur).info.pPayload)
                as ::core::ffi::c_long as ::core::ffi::c_int
        {
            amt = if 0 as ::core::ffi::c_int
                > (*(*pCur).pPage).aDataEnd.offset_from((*pCur).info.pPayload)
                    as ::core::ffi::c_long as ::core::ffi::c_int
            {
                0 as ::core::ffi::c_int
            } else {
                (*(*pCur).pPage).aDataEnd.offset_from((*pCur).info.pPayload)
                    as ::core::ffi::c_long as ::core::ffi::c_int
            };
        }
        *pAmt = amt as u32_0;
        return (*pCur).info.pPayload as *mut ::core::ffi::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreePayloadFetch(
    mut pCur: *mut BtCursor,
    mut pAmt: *mut u32_0,
) -> *const ::core::ffi::c_void {
    unsafe {
        return fetchPayload(pCur, pAmt);
    }
}
unsafe extern "C" fn moveToChild(
    mut pCur: *mut BtCursor,
    mut newPgno: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if (*pCur).iPage as ::core::ffi::c_int
            >= BTCURSOR_MAX_DEPTH - 1 as ::core::ffi::c_int
        {
            return sqlite3CorruptError(5425 as ::core::ffi::c_int);
        }
        (*pCur).info.nSize = 0 as u16_0;
        (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
            & !(BTCF_ValidNKey | BTCF_ValidOvfl)) as u8_0;
        (*pCur).aiIdx[(*pCur).iPage as usize] = (*pCur).ix;
        (*pCur).apPage[(*pCur).iPage as usize] = (*pCur).pPage;
        (*pCur).ix = 0 as u16_0;
        (*pCur).iPage += 1;
        rc = getAndInitPage(
            (*pCur).pBt,
            newPgno as Pgno,
            &raw mut (*pCur).pPage,
            (*pCur).curPagerFlags as ::core::ffi::c_int,
        );
        if rc == SQLITE_OK
            && (((*(*pCur).pPage).nCell as ::core::ffi::c_int) < 1 as ::core::ffi::c_int
                || (*(*pCur).pPage).intKey as ::core::ffi::c_int
                    != (*pCur).curIntKey as ::core::ffi::c_int)
        {
            releasePage((*pCur).pPage);
            rc = sqlite3CorruptError(5439 as ::core::ffi::c_int);
        }
        if rc != 0 {
            (*pCur).iPage -= 1;
            (*pCur).pPage = (*pCur).apPage[(*pCur).iPage as usize];
        }
        return rc;
    }
}
unsafe extern "C" fn moveToParent(mut pCur: *mut BtCursor) {
    unsafe {
        let mut pLeaf: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        (*pCur).info.nSize = 0 as u16_0;
        (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
            & !(BTCF_ValidNKey | BTCF_ValidOvfl)) as u8_0;
        (*pCur).ix = (*pCur)
            .aiIdx[((*pCur).iPage as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as usize];
        pLeaf = (*pCur).pPage;
        (*pCur).iPage -= 1;
        (*pCur).pPage = (*pCur).apPage[(*pCur).iPage as usize];
        releasePageNotNull(pLeaf);
    }
}
unsafe extern "C" fn moveToRoot(mut pCur: *mut BtCursor) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pRoot: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pCur).iPage as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            if (*pCur).iPage != 0 {
                releasePageNotNull((*pCur).pPage);
                loop {
                    (*pCur).iPage -= 1;
                    if !((*pCur).iPage != 0) {
                        break;
                    }
                    releasePageNotNull((*pCur).apPage[(*pCur).iPage as usize]);
                }
                (*pCur).pPage = (*pCur).apPage[0 as ::core::ffi::c_int as usize];
                pRoot = (*pCur).pPage;
                c2rust_current_block = 14154740549235462041;
            } else {
                c2rust_current_block = 17478428563724192186;
            }
        } else {
            if (*pCur).pgnoRoot == 0 as ::core::ffi::c_uint {
                (*pCur).eState = CURSOR_INVALID as u8_0;
                return SQLITE_EMPTY;
            } else {
                if (*pCur).eState as ::core::ffi::c_int >= CURSOR_REQUIRESEEK {
                    if (*pCur).eState as ::core::ffi::c_int == CURSOR_FAULT {
                        return (*pCur).skipNext;
                    }
                    sqlite3BtreeClearCursor(pCur);
                }
                rc = getAndInitPage(
                    (*pCur).pBt,
                    (*pCur).pgnoRoot,
                    &raw mut (*pCur).pPage,
                    (*pCur).curPagerFlags as ::core::ffi::c_int,
                );
                if rc != SQLITE_OK {
                    (*pCur).eState = CURSOR_INVALID as u8_0;
                    return rc;
                }
                (*pCur).iPage = 0 as i8_0;
                (*pCur).curIntKey = (*(*pCur).pPage).intKey;
            }
            c2rust_current_block = 17478428563724192186;
        }
        match c2rust_current_block {
            17478428563724192186 => {
                pRoot = (*pCur).pPage;
                if (*pRoot).isInit as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || (*pCur).pKeyInfo.is_null() as ::core::ffi::c_int
                        != (*pRoot).intKey as ::core::ffi::c_int
                {
                    return sqlite3CorruptError(5574 as ::core::ffi::c_int);
                }
            }
            _ => {}
        }
        (*pCur).ix = 0 as u16_0;
        (*pCur).info.nSize = 0 as u16_0;
        (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
            & !(BTCF_AtLast | BTCF_ValidNKey | BTCF_ValidOvfl)) as u8_0;
        if (*pRoot).nCell as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            (*pCur).eState = CURSOR_VALID as u8_0;
        } else if (*pRoot).leaf == 0 {
            let mut subpage: Pgno = 0;
            if (*pRoot).pgno != 1 as ::core::ffi::c_uint {
                return sqlite3CorruptError(5586 as ::core::ffi::c_int);
            }
            subpage = sqlite3Get4byte(
                (*pRoot)
                    .aData
                    .offset(
                        ((*pRoot).hdrOffset as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int) as isize,
                    ) as *mut u8_0,
            ) as Pgno;
            (*pCur).eState = CURSOR_VALID as u8_0;
            rc = moveToChild(pCur, subpage as u32_0);
        } else {
            (*pCur).eState = CURSOR_INVALID as u8_0;
            rc = SQLITE_EMPTY;
        }
        return rc;
    }
}
unsafe extern "C" fn moveToLeftmost(mut pCur: *mut BtCursor) -> ::core::ffi::c_int {
    unsafe {
        let mut pgno: Pgno = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        while rc == SQLITE_OK
            && {
                pPage = (*pCur).pPage;
                (*pPage).leaf == 0
            }
        {
            pgno = sqlite3Get4byte(
                (*pPage)
                    .aData
                    .offset(
                        ((*pPage).maskPage as ::core::ffi::c_int
                            & ((*((*pPage)
                                .aCellIdx
                                .offset(
                                    (2 as ::core::ffi::c_int * (*pCur).ix as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut u8_0)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                | *((*pPage)
                                    .aCellIdx
                                    .offset(
                                        (2 as ::core::ffi::c_int * (*pCur).ix as ::core::ffi::c_int)
                                            as isize,
                                    ) as *mut u8_0)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int)) as isize,
                    ),
            ) as Pgno;
            rc = moveToChild(pCur, pgno as u32_0);
        }
        return rc;
    }
}
unsafe extern "C" fn moveToRightmost(mut pCur: *mut BtCursor) -> ::core::ffi::c_int {
    unsafe {
        let mut pgno: Pgno = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        loop {
            pPage = (*pCur).pPage;
            if !((*pPage).leaf == 0) {
                break;
            }
            pgno = sqlite3Get4byte(
                (*pPage)
                    .aData
                    .offset(
                        ((*pPage).hdrOffset as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int) as isize,
                    ) as *mut u8_0,
            ) as Pgno;
            (*pCur).ix = (*pPage).nCell;
            rc = moveToChild(pCur, pgno as u32_0);
            if rc != 0 {
                return rc;
            }
        }
        (*pCur).ix = ((*pPage).nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as u16_0;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeFirst(
    mut pCur: *mut BtCursor,
    mut pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = moveToRoot(pCur);
        if rc == SQLITE_OK {
            *pRes = 0 as ::core::ffi::c_int;
            rc = moveToLeftmost(pCur);
        } else if rc == SQLITE_EMPTY {
            *pRes = 1 as ::core::ffi::c_int;
            rc = SQLITE_OK;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeIsEmpty(
    mut pCur: *mut BtCursor,
    mut pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if (*pCur).eState as ::core::ffi::c_int == CURSOR_VALID {
            *pRes = 0 as ::core::ffi::c_int;
            return SQLITE_OK;
        }
        rc = moveToRoot(pCur);
        if rc == SQLITE_EMPTY {
            *pRes = 1 as ::core::ffi::c_int;
            rc = SQLITE_OK;
        } else {
            *pRes = 0 as ::core::ffi::c_int;
        }
        return rc;
    }
}
#[inline(never)]
unsafe extern "C" fn btreeLast(
    mut pCur: *mut BtCursor,
    mut pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = moveToRoot(pCur);
        if rc == SQLITE_OK {
            *pRes = 0 as ::core::ffi::c_int;
            rc = moveToRightmost(pCur);
            if rc == SQLITE_OK {
                (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int | BTCF_AtLast)
                    as u8_0;
            } else {
                (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
                    & !BTCF_AtLast) as u8_0;
            }
        } else if rc == SQLITE_EMPTY {
            *pRes = 1 as ::core::ffi::c_int;
            rc = SQLITE_OK;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeLast(
    mut pCur: *mut BtCursor,
    mut pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if CURSOR_VALID == (*pCur).eState as ::core::ffi::c_int
            && (*pCur).curFlags as ::core::ffi::c_int & BTCF_AtLast
                != 0 as ::core::ffi::c_int
        {
            *pRes = 0 as ::core::ffi::c_int;
            return SQLITE_OK;
        }
        return btreeLast(pCur, pRes);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeTableMoveto(
    mut pCur: *mut BtCursor,
    mut intKey: i64_0,
    mut biasRight: ::core::ffi::c_int,
    mut pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = 0;
        if (*pCur).eState as ::core::ffi::c_int == CURSOR_VALID
            && (*pCur).curFlags as ::core::ffi::c_int & BTCF_ValidNKey
                != 0 as ::core::ffi::c_int
        {
            if (*pCur).info.nKey == intKey {
                *pRes = 0 as ::core::ffi::c_int;
                return SQLITE_OK;
            }
            if (*pCur).info.nKey < intKey {
                if (*pCur).curFlags as ::core::ffi::c_int & BTCF_AtLast
                    != 0 as ::core::ffi::c_int
                {
                    *pRes = -1 as ::core::ffi::c_int;
                    return SQLITE_OK;
                }
                if (*pCur).info.nKey as ::core::ffi::c_longlong
                    + 1 as ::core::ffi::c_longlong == intKey
                {
                    *pRes = 0 as ::core::ffi::c_int;
                    rc = sqlite3BtreeNext(pCur, 0 as ::core::ffi::c_int);
                    if rc == SQLITE_OK {
                        getCellInfo(pCur);
                        if (*pCur).info.nKey == intKey {
                            return SQLITE_OK;
                        }
                    } else if rc != SQLITE_DONE {
                        return rc
                    }
                }
            }
        }
        rc = moveToRoot(pCur);
        if rc != 0 {
            if rc == SQLITE_EMPTY {
                *pRes = -1 as ::core::ffi::c_int;
                return SQLITE_OK;
            }
            return rc;
        }
        loop {
            let mut lwr: ::core::ffi::c_int = 0;
            let mut upr: ::core::ffi::c_int = 0;
            let mut idx: ::core::ffi::c_int = 0;
            let mut c: ::core::ffi::c_int = 0;
            let mut chldPg: Pgno = 0;
            let mut pPage: *mut MemPage = (*pCur).pPage;
            let mut pCell: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
            lwr = 0 as ::core::ffi::c_int;
            upr = (*pPage).nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            idx = upr >> 1 as ::core::ffi::c_int - biasRight;
            loop {
                let mut nCellKey: i64_0 = 0;
                pCell = (*pPage)
                    .aDataOfst
                    .offset(
                        ((*pPage).maskPage as ::core::ffi::c_int
                            & ((*((*pPage)
                                .aCellIdx
                                .offset((2 as ::core::ffi::c_int * idx) as isize)
                                as *mut u8_0)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                | *((*pPage)
                                    .aCellIdx
                                    .offset((2 as ::core::ffi::c_int * idx) as isize)
                                    as *mut u8_0)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int)) as isize,
                    );
                if (*pPage).intKeyLeaf != 0 {
                    loop {
                        let c2rust_fresh11 = pCell;
                        pCell = pCell.offset(1);
                        if !(0x80 as ::core::ffi::c_int
                            <= *c2rust_fresh11 as ::core::ffi::c_int)
                        {
                            break;
                        }
                        if pCell >= (*pPage).aDataEnd {
                            return sqlite3CorruptError(5859 as ::core::ffi::c_int);
                        }
                    }
                }
                sqlite3GetVarint(pCell, &raw mut nCellKey as *mut u64_0);
                if nCellKey < intKey {
                    lwr = idx + 1 as ::core::ffi::c_int;
                    if lwr > upr {
                        c = -1 as ::core::ffi::c_int;
                        c2rust_current_block = 981995395831942902;
                        break;
                    }
                } else if nCellKey > intKey {
                    upr = idx - 1 as ::core::ffi::c_int;
                    if lwr > upr {
                        c = 1 as ::core::ffi::c_int;
                        c2rust_current_block = 981995395831942902;
                        break;
                    }
                } else {
                    (*pCur).ix = idx as u16_0;
                    if (*pPage).leaf == 0 {
                        lwr = idx;
                        c2rust_current_block = 11640243930581977563;
                        break;
                    } else {
                        (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
                            | BTCF_ValidNKey) as u8_0;
                        (*pCur).info.nKey = nCellKey;
                        (*pCur).info.nSize = 0 as u16_0;
                        *pRes = 0 as ::core::ffi::c_int;
                        return SQLITE_OK;
                    }
                }
                idx = lwr + upr >> 1 as ::core::ffi::c_int;
            }
            match c2rust_current_block {
                981995395831942902 => {
                    if (*pPage).leaf != 0 {
                        (*pCur).ix = idx as u16_0;
                        *pRes = c;
                        rc = SQLITE_OK;
                        break;
                    }
                }
                _ => {}
            }
            if lwr >= (*pPage).nCell as ::core::ffi::c_int {
                chldPg = sqlite3Get4byte(
                    (*pPage)
                        .aData
                        .offset(
                            ((*pPage).hdrOffset as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int) as isize,
                        ) as *mut u8_0,
                ) as Pgno;
            } else {
                chldPg = sqlite3Get4byte(
                    (*pPage)
                        .aData
                        .offset(
                            ((*pPage).maskPage as ::core::ffi::c_int
                                & ((*((*pPage)
                                    .aCellIdx
                                    .offset((2 as ::core::ffi::c_int * lwr) as isize)
                                    as *mut u8_0)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                    | *((*pPage)
                                        .aCellIdx
                                        .offset((2 as ::core::ffi::c_int * lwr) as isize)
                                        as *mut u8_0)
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int)) as isize,
                        ),
                ) as Pgno;
            }
            (*pCur).ix = lwr as u16_0;
            rc = moveToChild(pCur, chldPg as u32_0);
            if rc != 0 {
                break;
            }
        }
        (*pCur).info.nSize = 0 as u16_0;
        return rc;
    }
}
unsafe extern "C" fn indexCellCompare(
    mut pPage: *mut MemPage,
    mut idx: ::core::ffi::c_int,
    mut pIdxKey: *mut UnpackedRecord,
    mut xRecordCompare: RecordCompare,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: ::core::ffi::c_int = 0;
        let mut nCell: ::core::ffi::c_int = 0;
        let mut pCell: *mut u8_0 = (*pPage)
            .aDataOfst
            .offset(
                ((*pPage).maskPage as ::core::ffi::c_int
                    & ((*((*pPage)
                        .aCellIdx
                        .offset((2 as ::core::ffi::c_int * idx) as isize) as *mut u8_0)
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *((*pPage)
                            .aCellIdx
                            .offset((2 as ::core::ffi::c_int * idx) as isize)
                            as *mut u8_0)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int)) as isize,
            );
        nCell = *pCell.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        if nCell <= (*pPage).max1bytePayload as ::core::ffi::c_int {
            c = xRecordCompare
                .expect(
                    "non-null function pointer",
                )(
                nCell,
                pCell.offset(1 as ::core::ffi::c_int as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void,
                pIdxKey,
            );
        } else if *pCell.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int == 0
            && {
                nCell = ((nCell & 0x7f as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                    + *pCell.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                nCell <= (*pPage).maxLocal as ::core::ffi::c_int
            }
        {
            c = xRecordCompare
                .expect(
                    "non-null function pointer",
                )(
                nCell,
                pCell.offset(2 as ::core::ffi::c_int as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void,
                pIdxKey,
            );
        } else {
            c = 99 as ::core::ffi::c_int;
        }
        return c;
    }
}
unsafe extern "C" fn cursorOnLastPage(mut pCur: *mut BtCursor) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pCur).iPage as ::core::ffi::c_int {
            let mut pPage: *mut MemPage = (*pCur).apPage[i as usize];
            if ((*pCur).aiIdx[i as usize] as ::core::ffi::c_int)
                < (*pPage).nCell as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeIndexMoveto(
    mut pCur: *mut BtCursor,
    mut pIdxKey: *mut UnpackedRecord,
    mut pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = 0;
        let mut xRecordCompare: RecordCompare = None;
        xRecordCompare = sqlite3VdbeFindCompare(pIdxKey);
        (*pIdxKey).errCode = 0 as u8_0;
        if (*pCur).eState as ::core::ffi::c_int == CURSOR_VALID
            && (*(*pCur).pPage).leaf as ::core::ffi::c_int != 0
            && cursorOnLastPage(pCur) != 0
        {
            let mut c: ::core::ffi::c_int = 0;
            if (*pCur).ix as ::core::ffi::c_int
                == (*(*pCur).pPage).nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                && {
                    c = indexCellCompare(
                        (*pCur).pPage,
                        (*pCur).ix as ::core::ffi::c_int,
                        pIdxKey,
                        xRecordCompare,
                    );
                    c <= 0 as ::core::ffi::c_int
                } && (*pIdxKey).errCode as ::core::ffi::c_int == SQLITE_OK
            {
                *pRes = c;
                return SQLITE_OK;
            }
            if (*pCur).iPage as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                && indexCellCompare(
                    (*pCur).pPage,
                    0 as ::core::ffi::c_int,
                    pIdxKey,
                    xRecordCompare,
                ) <= 0 as ::core::ffi::c_int
                && (*pIdxKey).errCode as ::core::ffi::c_int == SQLITE_OK
            {
                (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
                    & !(BTCF_ValidOvfl | BTCF_AtLast)) as u8_0;
                if (*(*pCur).pPage).isInit == 0 {
                    return sqlite3CorruptError(6054 as ::core::ffi::c_int);
                }
                c2rust_current_block = 2719512138335094285;
            } else {
                (*pIdxKey).errCode = SQLITE_OK as u8_0;
                c2rust_current_block = 4166486009154926805;
            }
        } else {
            c2rust_current_block = 4166486009154926805;
        }
        match c2rust_current_block {
            4166486009154926805 => {
                rc = moveToRoot(pCur);
                if rc != 0 {
                    if rc == SQLITE_EMPTY {
                        *pRes = -1 as ::core::ffi::c_int;
                        return SQLITE_OK;
                    }
                    return rc;
                }
            }
            _ => {}
        }
        's_125: loop {
            let mut lwr: ::core::ffi::c_int = 0;
            let mut upr: ::core::ffi::c_int = 0;
            let mut idx: ::core::ffi::c_int = 0;
            let mut c_0: ::core::ffi::c_int = 0;
            let mut chldPg: Pgno = 0;
            let mut pPage: *mut MemPage = (*pCur).pPage;
            let mut pCell: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
            lwr = 0 as ::core::ffi::c_int;
            upr = (*pPage).nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            idx = upr >> 1 as ::core::ffi::c_int;
            loop {
                let mut nCell: ::core::ffi::c_int = 0;
                pCell = (*pPage)
                    .aDataOfst
                    .offset(
                        ((*pPage).maskPage as ::core::ffi::c_int
                            & ((*((*pPage)
                                .aCellIdx
                                .offset((2 as ::core::ffi::c_int * idx) as isize)
                                as *mut u8_0)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                | *((*pPage)
                                    .aCellIdx
                                    .offset((2 as ::core::ffi::c_int * idx) as isize)
                                    as *mut u8_0)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int)) as isize,
                    );
                nCell = *pCell.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
                if nCell <= (*pPage).max1bytePayload as ::core::ffi::c_int {
                    c_0 = xRecordCompare
                        .expect(
                            "non-null function pointer",
                        )(
                        nCell,
                        pCell.offset(1 as ::core::ffi::c_int as isize) as *mut u8_0
                            as *mut ::core::ffi::c_void,
                        pIdxKey,
                    );
                } else if *pCell.offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0
                    && {
                        nCell = ((nCell & 0x7f as ::core::ffi::c_int)
                            << 7 as ::core::ffi::c_int)
                            + *pCell.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int;
                        nCell <= (*pPage).maxLocal as ::core::ffi::c_int
                    }
                {
                    c_0 = xRecordCompare
                        .expect(
                            "non-null function pointer",
                        )(
                        nCell,
                        pCell.offset(2 as ::core::ffi::c_int as isize) as *mut u8_0
                            as *mut ::core::ffi::c_void,
                        pIdxKey,
                    );
                } else {
                    let mut pCellKey: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                        ::core::ffi::c_void,
                    >();
                    let pCellBody: *mut u8_0 = pCell
                        .offset(-((*pPage).childPtrSize as ::core::ffi::c_int as isize));
                    let nOverrun: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
                    (*pPage)
                        .xParseCell
                        .expect(
                            "non-null function pointer",
                        )(pPage, pCellBody, &raw mut (*pCur).info);
                    nCell = (*pCur).info.nKey as ::core::ffi::c_int;
                    if nCell < 2 as ::core::ffi::c_int
                        || (nCell as u32_0).wrapping_div((*(*pCur).pBt).usableSize)
                            > (*(*pCur).pBt).nPage
                    {
                        rc = sqlite3CorruptError(6141 as ::core::ffi::c_int);
                        break 's_125;
                    } else {
                        pCellKey = sqlite3Malloc(
                            (nCell as u64_0).wrapping_add(nOverrun as u64_0),
                        );
                        if pCellKey.is_null() {
                            rc = SQLITE_NOMEM_BKPT;
                            break 's_125;
                        } else {
                            (*pCur).ix = idx as u16_0;
                            rc = accessPayload(
                                pCur,
                                0 as u32_0,
                                nCell as u32_0,
                                pCellKey as *mut ::core::ffi::c_uchar,
                                0 as ::core::ffi::c_int,
                            );
                            memset(
                                (pCellKey as *mut u8_0).offset(nCell as isize)
                                    as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                nOverrun as size_t,
                            );
                            (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
                                & !BTCF_ValidOvfl) as u8_0;
                            if rc != 0 {
                                sqlite3_free(pCellKey);
                                break 's_125;
                            } else {
                                c_0 = sqlite3VdbeRecordCompare(nCell, pCellKey, pIdxKey);
                                sqlite3_free(pCellKey);
                            }
                        }
                    }
                }
                if c_0 < 0 as ::core::ffi::c_int {
                    lwr = idx + 1 as ::core::ffi::c_int;
                } else if c_0 > 0 as ::core::ffi::c_int {
                    upr = idx - 1 as ::core::ffi::c_int;
                } else {
                    *pRes = 0 as ::core::ffi::c_int;
                    rc = SQLITE_OK;
                    (*pCur).ix = idx as u16_0;
                    if (*pIdxKey).errCode != 0 {
                        rc = sqlite3CorruptError(6173 as ::core::ffi::c_int);
                    }
                    break 's_125;
                }
                if lwr > upr {
                    break;
                }
                idx = lwr + upr >> 1 as ::core::ffi::c_int;
            }
            if (*pPage).leaf != 0 {
                (*pCur).ix = idx as u16_0;
                *pRes = c_0;
                rc = SQLITE_OK;
                break;
            } else {
                if lwr >= (*pPage).nCell as ::core::ffi::c_int {
                    chldPg = sqlite3Get4byte(
                        (*pPage)
                            .aData
                            .offset(
                                ((*pPage).hdrOffset as ::core::ffi::c_int
                                    + 8 as ::core::ffi::c_int) as isize,
                            ) as *mut u8_0,
                    ) as Pgno;
                } else {
                    chldPg = sqlite3Get4byte(
                        (*pPage)
                            .aData
                            .offset(
                                ((*pPage).maskPage as ::core::ffi::c_int
                                    & ((*((*pPage)
                                        .aCellIdx
                                        .offset((2 as ::core::ffi::c_int * lwr) as isize)
                                        as *mut u8_0)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                        | *((*pPage)
                                            .aCellIdx
                                            .offset((2 as ::core::ffi::c_int * lwr) as isize)
                                            as *mut u8_0)
                                            .offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int)) as isize,
                            ),
                    ) as Pgno;
                }
                (*pCur).info.nSize = 0 as u16_0;
                (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
                    & !(BTCF_ValidNKey | BTCF_ValidOvfl)) as u8_0;
                if (*pCur).iPage as ::core::ffi::c_int
                    >= BTCURSOR_MAX_DEPTH - 1 as ::core::ffi::c_int
                {
                    return sqlite3CorruptError(6204 as ::core::ffi::c_int);
                }
                (*pCur).aiIdx[(*pCur).iPage as usize] = lwr as u16_0;
                (*pCur).apPage[(*pCur).iPage as usize] = (*pCur).pPage;
                (*pCur).ix = 0 as u16_0;
                (*pCur).iPage += 1;
                rc = getAndInitPage(
                    (*pCur).pBt,
                    chldPg,
                    &raw mut (*pCur).pPage,
                    (*pCur).curPagerFlags as ::core::ffi::c_int,
                );
                if rc == SQLITE_OK
                    && (((*(*pCur).pPage).nCell as ::core::ffi::c_int)
                        < 1 as ::core::ffi::c_int
                        || (*(*pCur).pPage).intKey as ::core::ffi::c_int
                            != (*pCur).curIntKey as ::core::ffi::c_int)
                {
                    releasePage((*pCur).pPage);
                    rc = sqlite3CorruptError(6215 as ::core::ffi::c_int);
                }
                if !(rc != 0) {
                    continue;
                }
                (*pCur).iPage -= 1;
                (*pCur).pPage = (*pCur).apPage[(*pCur).iPage as usize];
                break;
            }
        }
        (*pCur).info.nSize = 0 as u16_0;
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeEof(mut pCur: *mut BtCursor) -> ::core::ffi::c_int {
    unsafe {
        return (CURSOR_VALID != (*pCur).eState as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeRowCountEst(mut pCur: *mut BtCursor) -> i64_0 {
    unsafe {
        let mut n: i64_0 = 0;
        let mut i: u8_0 = 0;
        if (*pCur).eState as ::core::ffi::c_int != CURSOR_VALID {
            return 0 as i64_0;
        }
        if (*(*pCur).pPage).leaf as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int as i64_0;
        }
        n = (*(*pCur).pPage).nCell as i64_0;
        i = 0 as u8_0;
        while (i as ::core::ffi::c_int) < (*pCur).iPage as ::core::ffi::c_int {
            n
                *= ((*(*pCur).apPage[i as usize]).nCell as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
            i = i.wrapping_add(1);
        }
        return n;
    }
}
#[inline(never)]
unsafe extern "C" fn btreeNext(mut pCur: *mut BtCursor) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut idx: ::core::ffi::c_int = 0;
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        if (*pCur).eState as ::core::ffi::c_int != CURSOR_VALID {
            rc = if (*pCur).eState as ::core::ffi::c_int >= CURSOR_REQUIRESEEK {
                btreeRestoreCursorPosition(pCur)
            } else {
                SQLITE_OK
            };
            if rc != SQLITE_OK {
                return rc;
            }
            if CURSOR_INVALID == (*pCur).eState as ::core::ffi::c_int {
                return SQLITE_DONE;
            }
            if (*pCur).eState as ::core::ffi::c_int == CURSOR_SKIPNEXT {
                (*pCur).eState = CURSOR_VALID as u8_0;
                if (*pCur).skipNext > 0 as ::core::ffi::c_int {
                    return SQLITE_OK;
                }
            }
        }
        pPage = (*pCur).pPage;
        (*pCur).ix = (*pCur).ix.wrapping_add(1);
        idx = (*pCur).ix as ::core::ffi::c_int;
        if sqlite3FaultSim(412 as ::core::ffi::c_int) != 0 {
            (*pPage).isInit = 0 as u8_0;
        }
        if (*pPage).isInit == 0 {
            return sqlite3CorruptError(6316 as ::core::ffi::c_int);
        }
        if idx >= (*pPage).nCell as ::core::ffi::c_int {
            if (*pPage).leaf == 0 {
                rc = moveToChild(
                    pCur,
                    sqlite3Get4byte(
                        (*pPage)
                            .aData
                            .offset(
                                ((*pPage).hdrOffset as ::core::ffi::c_int
                                    + 8 as ::core::ffi::c_int) as isize,
                            ) as *mut u8_0,
                    ),
                );
                if rc != 0 {
                    return rc;
                }
                return moveToLeftmost(pCur);
            }
            loop {
                if (*pCur).iPage as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*pCur).eState = CURSOR_INVALID as u8_0;
                    return SQLITE_DONE;
                }
                moveToParent(pCur);
                pPage = (*pCur).pPage;
                if !((*pCur).ix as ::core::ffi::c_int
                    >= (*pPage).nCell as ::core::ffi::c_int)
                {
                    break;
                }
            }
            if (*pPage).intKey != 0 {
                return sqlite3BtreeNext(pCur, 0 as ::core::ffi::c_int)
            } else {
                return SQLITE_OK
            }
        }
        if (*pPage).leaf != 0 { return SQLITE_OK } else { return moveToLeftmost(pCur) };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeNext(
    mut pCur: *mut BtCursor,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        (*pCur).info.nSize = 0 as u16_0;
        (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
            & !(BTCF_ValidNKey | BTCF_ValidOvfl)) as u8_0;
        if (*pCur).eState as ::core::ffi::c_int != CURSOR_VALID {
            return btreeNext(pCur);
        }
        pPage = (*pCur).pPage;
        (*pCur).ix = (*pCur).ix.wrapping_add(1);
        if (*pCur).ix as ::core::ffi::c_int >= (*pPage).nCell as ::core::ffi::c_int {
            (*pCur).ix = (*pCur).ix.wrapping_sub(1);
            return btreeNext(pCur);
        }
        if (*pPage).leaf != 0 { return SQLITE_OK } else { return moveToLeftmost(pCur) };
    }
}
#[inline(never)]
unsafe extern "C" fn btreePrevious(mut pCur: *mut BtCursor) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        if (*pCur).eState as ::core::ffi::c_int != CURSOR_VALID {
            rc = if (*pCur).eState as ::core::ffi::c_int >= CURSOR_REQUIRESEEK {
                btreeRestoreCursorPosition(pCur)
            } else {
                SQLITE_OK
            };
            if rc != SQLITE_OK {
                return rc;
            }
            if CURSOR_INVALID == (*pCur).eState as ::core::ffi::c_int {
                return SQLITE_DONE;
            }
            if CURSOR_SKIPNEXT == (*pCur).eState as ::core::ffi::c_int {
                (*pCur).eState = CURSOR_VALID as u8_0;
                if (*pCur).skipNext < 0 as ::core::ffi::c_int {
                    return SQLITE_OK;
                }
            }
        }
        pPage = (*pCur).pPage;
        if sqlite3FaultSim(412 as ::core::ffi::c_int) != 0 {
            (*pPage).isInit = 0 as u8_0;
        }
        if (*pPage).isInit == 0 {
            return sqlite3CorruptError(6409 as ::core::ffi::c_int);
        }
        if (*pPage).leaf == 0 {
            let mut idx: ::core::ffi::c_int = (*pCur).ix as ::core::ffi::c_int;
            rc = moveToChild(
                pCur,
                sqlite3Get4byte(
                    (*pPage)
                        .aData
                        .offset(
                            ((*pPage).maskPage as ::core::ffi::c_int
                                & ((*((*pPage)
                                    .aCellIdx
                                    .offset((2 as ::core::ffi::c_int * idx) as isize)
                                    as *mut u8_0)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                    | *((*pPage)
                                        .aCellIdx
                                        .offset((2 as ::core::ffi::c_int * idx) as isize)
                                        as *mut u8_0)
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int)) as isize,
                        ),
                ),
            );
            if rc != 0 {
                return rc;
            }
            rc = moveToRightmost(pCur);
        } else {
            while (*pCur).ix as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if (*pCur).iPage as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*pCur).eState = CURSOR_INVALID as u8_0;
                    return SQLITE_DONE;
                }
                moveToParent(pCur);
            }
            (*pCur).ix = (*pCur).ix.wrapping_sub(1);
            pPage = (*pCur).pPage;
            if (*pPage).intKey as ::core::ffi::c_int != 0 && (*pPage).leaf == 0 {
                rc = sqlite3BtreePrevious(pCur, 0 as ::core::ffi::c_int);
            } else {
                rc = SQLITE_OK;
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreePrevious(
    mut pCur: *mut BtCursor,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
            & !(BTCF_AtLast | BTCF_ValidOvfl | BTCF_ValidNKey)) as u8_0;
        (*pCur).info.nSize = 0 as u16_0;
        if (*pCur).eState as ::core::ffi::c_int != CURSOR_VALID
            || (*pCur).ix as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*(*pCur).pPage).leaf as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return btreePrevious(pCur);
        }
        (*pCur).ix = (*pCur).ix.wrapping_sub(1);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn allocateBtreePage(
    mut pBt: *mut BtShared,
    mut ppPage: *mut *mut MemPage,
    mut pPgno: *mut Pgno,
    mut nearby: Pgno,
    mut eMode: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPage1: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut n: u32_0 = 0;
        let mut k: u32_0 = 0;
        let mut pTrunk: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut pPrevTrunk: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut mxPage: Pgno = 0;
        pPage1 = (*pBt).pPage1;
        mxPage = btreePagecount(pBt);
        n = sqlite3Get4byte(
            (*pPage1).aData.offset(36 as ::core::ffi::c_int as isize) as *mut u8_0,
        );
        if n >= mxPage {
            return sqlite3CorruptError(6499 as ::core::ffi::c_int);
        }
        if n > 0 as ::core::ffi::c_uint {
            let mut iTrunk: Pgno = 0;
            let mut searchList: u8_0 = 0 as u8_0;
            let mut nSearch: u32_0 = 0 as u32_0;
            if eMode as ::core::ffi::c_int == BTALLOC_EXACT {
                if nearby <= mxPage {
                    let mut eType: u8_0 = 0;
                    rc = ptrmapGet(
                        pBt,
                        nearby,
                        &raw mut eType,
                        ::core::ptr::null_mut::<Pgno>(),
                    );
                    if rc != 0 {
                        return rc;
                    }
                    if eType as ::core::ffi::c_int == PTRMAP_FREEPAGE {
                        searchList = 1 as u8_0;
                    }
                }
            } else if eMode as ::core::ffi::c_int == BTALLOC_LE {
                searchList = 1 as u8_0;
            }
            rc = sqlite3PagerWrite((*pPage1).pDbPage);
            if rc != 0 {
                return rc;
            }
            sqlite3Put4byte(
                (*pPage1).aData.offset(36 as ::core::ffi::c_int as isize) as *mut u8_0,
                n.wrapping_sub(1 as u32_0),
            );
            loop {
                pPrevTrunk = pTrunk;
                if !pPrevTrunk.is_null() {
                    iTrunk = sqlite3Get4byte(
                        (*pPrevTrunk).aData.offset(0 as ::core::ffi::c_int as isize)
                            as *mut u8_0,
                    ) as Pgno;
                } else {
                    iTrunk = sqlite3Get4byte(
                        (*pPage1).aData.offset(32 as ::core::ffi::c_int as isize)
                            as *mut u8_0,
                    ) as Pgno;
                }
                if iTrunk > mxPage
                    || {
                        let c2rust_fresh9 = nSearch;
                        nSearch = nSearch.wrapping_add(1);
                        c2rust_fresh9 > n
                    }
                {
                    rc = sqlite3CorruptError(6555 as ::core::ffi::c_int);
                } else {
                    rc = btreeGetUnusedPage(
                        pBt,
                        iTrunk,
                        &raw mut pTrunk,
                        0 as ::core::ffi::c_int,
                    );
                }
                if rc != 0 {
                    pTrunk = ::core::ptr::null_mut::<MemPage>();
                    break;
                } else {
                    k = sqlite3Get4byte(
                        (*pTrunk).aData.offset(4 as ::core::ffi::c_int as isize)
                            as *mut u8_0,
                    );
                    if k == 0 as ::core::ffi::c_uint && searchList == 0 {
                        rc = sqlite3PagerWrite((*pTrunk).pDbPage);
                        if rc != 0 {
                            break;
                        }
                        *pPgno = iTrunk;
                        memcpy(
                            (*pPage1).aData.offset(32 as ::core::ffi::c_int as isize)
                                as *mut u8_0 as *mut ::core::ffi::c_void,
                            (*pTrunk).aData.offset(0 as ::core::ffi::c_int as isize)
                                as *mut u8_0 as *const ::core::ffi::c_void,
                            4 as size_t,
                        );
                        *ppPage = pTrunk;
                        pTrunk = ::core::ptr::null_mut::<MemPage>();
                    } else if k
                        > ((*pBt).usableSize as ::core::ffi::c_uint)
                            .wrapping_div(4 as ::core::ffi::c_uint)
                            .wrapping_sub(2 as ::core::ffi::c_uint)
                    {
                        rc = sqlite3CorruptError(6584 as ::core::ffi::c_int);
                        break;
                    } else if searchList as ::core::ffi::c_int != 0
                        && (nearby == iTrunk
                            || iTrunk < nearby
                                && eMode as ::core::ffi::c_int == BTALLOC_LE)
                    {
                        *pPgno = iTrunk;
                        *ppPage = pTrunk;
                        searchList = 0 as u8_0;
                        rc = sqlite3PagerWrite((*pTrunk).pDbPage);
                        if rc != 0 {
                            break;
                        }
                        if k == 0 as ::core::ffi::c_uint {
                            if pPrevTrunk.is_null() {
                                memcpy(
                                    (*pPage1).aData.offset(32 as ::core::ffi::c_int as isize)
                                        as *mut u8_0 as *mut ::core::ffi::c_void,
                                    (*pTrunk).aData.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut u8_0 as *const ::core::ffi::c_void,
                                    4 as size_t,
                                );
                            } else {
                                rc = sqlite3PagerWrite((*pPrevTrunk).pDbPage);
                                if rc != SQLITE_OK {
                                    break;
                                }
                                memcpy(
                                    (*pPrevTrunk).aData.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut u8_0 as *mut ::core::ffi::c_void,
                                    (*pTrunk).aData.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut u8_0 as *const ::core::ffi::c_void,
                                    4 as size_t,
                                );
                            }
                        } else {
                            let mut pNewTrunk: *mut MemPage = ::core::ptr::null_mut::<
                                MemPage,
                            >();
                            let mut iNewTrunk: Pgno = sqlite3Get4byte(
                                (*pTrunk).aData.offset(8 as ::core::ffi::c_int as isize)
                                    as *mut u8_0,
                            ) as Pgno;
                            if iNewTrunk > mxPage {
                                rc = sqlite3CorruptError(6618 as ::core::ffi::c_int);
                                break;
                            } else {
                                rc = btreeGetUnusedPage(
                                    pBt,
                                    iNewTrunk,
                                    &raw mut pNewTrunk,
                                    0 as ::core::ffi::c_int,
                                );
                                if rc != SQLITE_OK {
                                    break;
                                }
                                rc = sqlite3PagerWrite((*pNewTrunk).pDbPage);
                                if rc != SQLITE_OK {
                                    releasePage(pNewTrunk);
                                    break;
                                } else {
                                    memcpy(
                                        (*pNewTrunk).aData.offset(0 as ::core::ffi::c_int as isize)
                                            as *mut u8_0 as *mut ::core::ffi::c_void,
                                        (*pTrunk).aData.offset(0 as ::core::ffi::c_int as isize)
                                            as *mut u8_0 as *const ::core::ffi::c_void,
                                        4 as size_t,
                                    );
                                    sqlite3Put4byte(
                                        (*pNewTrunk).aData.offset(4 as ::core::ffi::c_int as isize)
                                            as *mut u8_0,
                                        k.wrapping_sub(1 as u32_0),
                                    );
                                    memcpy(
                                        (*pNewTrunk).aData.offset(8 as ::core::ffi::c_int as isize)
                                            as *mut u8_0 as *mut ::core::ffi::c_void,
                                        (*pTrunk).aData.offset(12 as ::core::ffi::c_int as isize)
                                            as *mut u8_0 as *const ::core::ffi::c_void,
                                        (k as ::core::ffi::c_uint)
                                            .wrapping_sub(1 as ::core::ffi::c_uint)
                                            .wrapping_mul(4 as ::core::ffi::c_uint) as size_t,
                                    );
                                    releasePage(pNewTrunk);
                                    if pPrevTrunk.is_null() {
                                        sqlite3Put4byte(
                                            (*pPage1).aData.offset(32 as ::core::ffi::c_int as isize)
                                                as *mut u8_0,
                                            iNewTrunk as u32_0,
                                        );
                                    } else {
                                        rc = sqlite3PagerWrite((*pPrevTrunk).pDbPage);
                                        if rc != 0 {
                                            break;
                                        }
                                        sqlite3Put4byte(
                                            (*pPrevTrunk).aData.offset(0 as ::core::ffi::c_int as isize)
                                                as *mut u8_0,
                                            iNewTrunk as u32_0,
                                        );
                                    }
                                }
                            }
                        }
                        pTrunk = ::core::ptr::null_mut::<MemPage>();
                    } else if k > 0 as ::core::ffi::c_uint {
                        let mut closest: u32_0 = 0;
                        let mut iPage: Pgno = 0;
                        let mut aData: *mut ::core::ffi::c_uchar = (*pTrunk).aData
                            as *mut ::core::ffi::c_uchar;
                        if nearby > 0 as ::core::ffi::c_uint {
                            let mut i: u32_0 = 0;
                            closest = 0 as u32_0;
                            if eMode as ::core::ffi::c_int == BTALLOC_LE {
                                i = 0 as u32_0;
                                while i < k {
                                    iPage = sqlite3Get4byte(
                                        aData
                                            .offset(
                                                (8 as ::core::ffi::c_uint)
                                                    .wrapping_add(
                                                        (i as ::core::ffi::c_uint)
                                                            .wrapping_mul(4 as ::core::ffi::c_uint),
                                                    ) as isize,
                                            ) as *mut ::core::ffi::c_uchar,
                                    ) as Pgno;
                                    if iPage <= nearby {
                                        closest = i;
                                        break;
                                    } else {
                                        i = i.wrapping_add(1);
                                    }
                                }
                            } else {
                                let mut dist: ::core::ffi::c_int = 0;
                                dist = sqlite3AbsInt32(
                                    (sqlite3Get4byte(
                                        aData.offset(8 as ::core::ffi::c_int as isize)
                                            as *mut ::core::ffi::c_uchar,
                                    ) as Pgno)
                                        .wrapping_sub(nearby) as ::core::ffi::c_int,
                                );
                                i = 1 as u32_0;
                                while i < k {
                                    let mut d2: ::core::ffi::c_int = sqlite3AbsInt32(
                                        (sqlite3Get4byte(
                                            aData
                                                .offset(
                                                    (8 as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            (i as ::core::ffi::c_uint)
                                                                .wrapping_mul(4 as ::core::ffi::c_uint),
                                                        ) as isize,
                                                ) as *mut ::core::ffi::c_uchar,
                                        ) as Pgno)
                                            .wrapping_sub(nearby) as ::core::ffi::c_int,
                                    );
                                    if d2 < dist {
                                        closest = i;
                                        dist = d2;
                                    }
                                    i = i.wrapping_add(1);
                                }
                            }
                        } else {
                            closest = 0 as u32_0;
                        }
                        iPage = sqlite3Get4byte(
                            aData
                                .offset(
                                    (8 as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            (closest as ::core::ffi::c_uint)
                                                .wrapping_mul(4 as ::core::ffi::c_uint),
                                        ) as isize,
                                ) as *mut ::core::ffi::c_uchar,
                        ) as Pgno;
                        if iPage > mxPage || iPage < 2 as ::core::ffi::c_uint {
                            rc = sqlite3CorruptError(6683 as ::core::ffi::c_int);
                            break;
                        } else if searchList == 0
                            || (iPage == nearby
                                || iPage < nearby
                                    && eMode as ::core::ffi::c_int == BTALLOC_LE)
                        {
                            let mut noContent: ::core::ffi::c_int = 0;
                            *pPgno = iPage;
                            rc = sqlite3PagerWrite((*pTrunk).pDbPage);
                            if rc != 0 {
                                break;
                            }
                            if closest
                                < (k as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                            {
                                memcpy(
                                    aData
                                        .offset(
                                            (8 as ::core::ffi::c_uint)
                                                .wrapping_add(
                                                    (closest as ::core::ffi::c_uint)
                                                        .wrapping_mul(4 as ::core::ffi::c_uint),
                                                ) as isize,
                                        ) as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                                    aData
                                        .offset(
                                            (4 as ::core::ffi::c_uint)
                                                .wrapping_add(
                                                    (k as ::core::ffi::c_uint)
                                                        .wrapping_mul(4 as ::core::ffi::c_uint),
                                                ) as isize,
                                        ) as *mut ::core::ffi::c_uchar
                                        as *const ::core::ffi::c_void,
                                    4 as size_t,
                                );
                            }
                            sqlite3Put4byte(
                                aData.offset(4 as ::core::ffi::c_int as isize) as *mut u8_0,
                                k.wrapping_sub(1 as u32_0),
                            );
                            noContent = if btreeGetHasContent(pBt, *pPgno) == 0 {
                                PAGER_GET_NOCONTENT
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            rc = btreeGetUnusedPage(pBt, *pPgno, ppPage, noContent);
                            if rc == SQLITE_OK {
                                rc = sqlite3PagerWrite((**ppPage).pDbPage);
                                if rc != SQLITE_OK {
                                    releasePage(*ppPage);
                                    *ppPage = ::core::ptr::null_mut::<MemPage>();
                                }
                            }
                            searchList = 0 as u8_0;
                        }
                    }
                    releasePage(pPrevTrunk);
                    pPrevTrunk = ::core::ptr::null_mut::<MemPage>();
                    if !(searchList != 0) {
                        break;
                    }
                }
            }
        } else {
            let mut bNoContent: ::core::ffi::c_int = if 0 as ::core::ffi::c_int
                == (*pBt).bDoTruncate as ::core::ffi::c_int
            {
                PAGER_GET_NOCONTENT
            } else {
                0 as ::core::ffi::c_int
            };
            rc = sqlite3PagerWrite((*(*pBt).pPage1).pDbPage);
            if rc != 0 {
                return rc;
            }
            (*pBt).nPage = (*pBt).nPage.wrapping_add(1);
            if (*pBt).nPage
                == (sqlite3PendingByte as ::core::ffi::c_uint)
                    .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
            {
                (*pBt).nPage = (*pBt).nPage.wrapping_add(1);
            }
            if (*pBt).autoVacuum as ::core::ffi::c_int != 0
                && ptrmapPageno(pBt, (*pBt).nPage as Pgno) == (*pBt).nPage
            {
                let mut pPg: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
                rc = btreeGetUnusedPage(
                    pBt,
                    (*pBt).nPage as Pgno,
                    &raw mut pPg,
                    bNoContent,
                );
                if rc == SQLITE_OK {
                    rc = sqlite3PagerWrite((*pPg).pDbPage);
                    releasePage(pPg);
                }
                if rc != 0 {
                    return rc;
                }
                (*pBt).nPage = (*pBt).nPage.wrapping_add(1);
                if (*pBt).nPage
                    == (sqlite3PendingByte as ::core::ffi::c_uint)
                        .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint)
                {
                    (*pBt).nPage = (*pBt).nPage.wrapping_add(1);
                }
            }
            sqlite3Put4byte(
                (*(*pBt).pPage1).aData.offset(28 as ::core::ffi::c_int as isize),
                (*pBt).nPage,
            );
            *pPgno = (*pBt).nPage as Pgno;
            rc = btreeGetUnusedPage(pBt, *pPgno, ppPage, bNoContent);
            if rc != 0 {
                return rc;
            }
            rc = sqlite3PagerWrite((**ppPage).pDbPage);
            if rc != SQLITE_OK {
                releasePage(*ppPage);
                *ppPage = ::core::ptr::null_mut::<MemPage>();
            }
        }
        releasePage(pTrunk);
        releasePage(pPrevTrunk);
        return rc;
    }
}
unsafe extern "C" fn freePage2(
    mut pBt: *mut BtShared,
    mut pMemPage: *mut MemPage,
    mut iPage: Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pTrunk: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut iTrunk: Pgno = 0 as Pgno;
        let mut pPage1: *mut MemPage = (*pBt).pPage1;
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut nFree: u32_0 = 0;
        if iPage < 2 as ::core::ffi::c_uint || iPage > (*pBt).nPage {
            return sqlite3CorruptError(6810 as ::core::ffi::c_int);
        }
        if !pMemPage.is_null() {
            pPage = pMemPage;
            sqlite3PagerRef((*pPage).pDbPage);
        } else {
            pPage = btreePageLookup(pBt, iPage);
        }
        rc = sqlite3PagerWrite((*pPage1).pDbPage);
        if !(rc != 0) {
            nFree = sqlite3Get4byte(
                (*pPage1).aData.offset(36 as ::core::ffi::c_int as isize) as *mut u8_0,
            );
            sqlite3Put4byte(
                (*pPage1).aData.offset(36 as ::core::ffi::c_int as isize) as *mut u8_0,
                nFree.wrapping_add(1 as u32_0),
            );
            if (*pBt).btsFlags as ::core::ffi::c_int & BTS_SECURE_DELETE != 0 {
                if pPage.is_null()
                    && {
                        rc = btreeGetPage(
                            pBt,
                            iPage,
                            &raw mut pPage,
                            0 as ::core::ffi::c_int,
                        );
                        rc != 0 as ::core::ffi::c_int
                    }
                    || {
                        rc = sqlite3PagerWrite((*pPage).pDbPage);
                        rc != 0 as ::core::ffi::c_int
                    }
                {
                    c2rust_current_block = 16367998888416993180;
                } else {
                    memset(
                        (*pPage).aData as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (*(*pPage).pBt).pageSize as size_t,
                    );
                    c2rust_current_block = 5143058163439228106;
                }
            } else {
                c2rust_current_block = 5143058163439228106;
            }
            match c2rust_current_block {
                16367998888416993180 => {}
                _ => {
                    if (*pBt).autoVacuum != 0 {
                        ptrmapPut(
                            pBt,
                            iPage,
                            PTRMAP_FREEPAGE as u8_0,
                            0 as Pgno,
                            &raw mut rc,
                        );
                        if rc != 0 {
                            c2rust_current_block = 16367998888416993180;
                        } else {
                            c2rust_current_block = 2838571290723028321;
                        }
                    } else {
                        c2rust_current_block = 2838571290723028321;
                    }
                    match c2rust_current_block {
                        16367998888416993180 => {}
                        _ => {
                            if nFree != 0 as ::core::ffi::c_uint {
                                let mut nLeaf: u32_0 = 0;
                                iTrunk = sqlite3Get4byte(
                                    (*pPage1).aData.offset(32 as ::core::ffi::c_int as isize)
                                        as *mut u8_0,
                                ) as Pgno;
                                if iTrunk > btreePagecount(pBt) {
                                    rc = sqlite3CorruptError(6857 as ::core::ffi::c_int);
                                    c2rust_current_block = 16367998888416993180;
                                } else {
                                    rc = btreeGetPage(
                                        pBt,
                                        iTrunk,
                                        &raw mut pTrunk,
                                        0 as ::core::ffi::c_int,
                                    );
                                    if rc != SQLITE_OK {
                                        c2rust_current_block = 16367998888416993180;
                                    } else {
                                        nLeaf = sqlite3Get4byte(
                                            (*pTrunk).aData.offset(4 as ::core::ffi::c_int as isize)
                                                as *mut u8_0,
                                        );
                                        if nLeaf
                                            > (*pBt)
                                                .usableSize
                                                .wrapping_div(4 as ::core::ffi::c_uint)
                                                .wrapping_sub(2 as ::core::ffi::c_uint)
                                        {
                                            rc = sqlite3CorruptError(6868 as ::core::ffi::c_int);
                                            c2rust_current_block = 16367998888416993180;
                                        } else if nLeaf
                                            < (*pBt)
                                                .usableSize
                                                .wrapping_div(4 as ::core::ffi::c_uint)
                                                .wrapping_sub(8 as ::core::ffi::c_uint)
                                        {
                                            rc = sqlite3PagerWrite((*pTrunk).pDbPage);
                                            if rc == SQLITE_OK {
                                                sqlite3Put4byte(
                                                    (*pTrunk).aData.offset(4 as ::core::ffi::c_int as isize)
                                                        as *mut u8_0,
                                                    nLeaf.wrapping_add(1 as u32_0),
                                                );
                                                sqlite3Put4byte(
                                                    (*pTrunk)
                                                        .aData
                                                        .offset(
                                                            (8 as ::core::ffi::c_uint)
                                                                .wrapping_add(
                                                                    (nLeaf as ::core::ffi::c_uint)
                                                                        .wrapping_mul(4 as ::core::ffi::c_uint),
                                                                ) as isize,
                                                        ) as *mut u8_0,
                                                    iPage as u32_0,
                                                );
                                                if !pPage.is_null()
                                                    && (*pBt).btsFlags as ::core::ffi::c_int & BTS_SECURE_DELETE
                                                        == 0 as ::core::ffi::c_int
                                                {
                                                    sqlite3PagerDontWrite((*pPage).pDbPage);
                                                }
                                                rc = btreeSetHasContent(pBt, iPage);
                                            }
                                            c2rust_current_block = 16367998888416993180;
                                        } else {
                                            c2rust_current_block = 4090602189656566074;
                                        }
                                    }
                                }
                            } else {
                                c2rust_current_block = 4090602189656566074;
                            }
                            match c2rust_current_block {
                                16367998888416993180 => {}
                                _ => {
                                    if !(pPage.is_null()
                                        && {
                                            rc = btreeGetPage(
                                                pBt,
                                                iPage,
                                                &raw mut pPage,
                                                0 as ::core::ffi::c_int,
                                            );
                                            SQLITE_OK != rc
                                        })
                                    {
                                        rc = sqlite3PagerWrite((*pPage).pDbPage);
                                        if !(rc != SQLITE_OK) {
                                            sqlite3Put4byte((*pPage).aData, iTrunk as u32_0);
                                            sqlite3Put4byte(
                                                (*pPage).aData.offset(4 as ::core::ffi::c_int as isize)
                                                    as *mut u8_0,
                                                0 as u32_0,
                                            );
                                            sqlite3Put4byte(
                                                (*pPage1).aData.offset(32 as ::core::ffi::c_int as isize)
                                                    as *mut u8_0,
                                                iPage as u32_0,
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
        if !pPage.is_null() {
            (*pPage).isInit = 0 as u8_0;
        }
        releasePage(pPage);
        releasePage(pTrunk);
        return rc;
    }
}
unsafe extern "C" fn freePage(
    mut pPage: *mut MemPage,
    mut pRC: *mut ::core::ffi::c_int,
) {
    unsafe {
        if *pRC == SQLITE_OK {
            *pRC = freePage2((*pPage).pBt, pPage, (*pPage).pgno);
        }
    }
}
#[inline(never)]
unsafe extern "C" fn clearCellOverflow(
    mut pPage: *mut MemPage,
    mut pCell: *mut ::core::ffi::c_uchar,
    mut pInfo: *mut CellInfo,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = ::core::ptr::null_mut::<BtShared>();
        let mut ovflPgno: Pgno = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut nOvfl: ::core::ffi::c_int = 0;
        let mut ovflPageSize: u32_0 = 0;
        if pCell.offset((*pInfo).nSize as ::core::ffi::c_int as isize)
            > (*pPage).aDataEnd
        {
            return sqlite3CorruptError(6957 as ::core::ffi::c_int);
        }
        ovflPgno = sqlite3Get4byte(
            pCell
                .offset((*pInfo).nSize as ::core::ffi::c_int as isize)
                .offset(-(4 as ::core::ffi::c_int as isize)),
        ) as Pgno;
        pBt = (*pPage).pBt;
        ovflPageSize = ((*pBt).usableSize as ::core::ffi::c_uint)
            .wrapping_sub(4 as ::core::ffi::c_uint) as u32_0;
        nOvfl = (*pInfo)
            .nPayload
            .wrapping_sub((*pInfo).nLocal as u32_0)
            .wrapping_add(ovflPageSize)
            .wrapping_sub(1 as u32_0)
            .wrapping_div(ovflPageSize) as ::core::ffi::c_int;
        loop {
            let c2rust_fresh10 = nOvfl;
            nOvfl = nOvfl - 1;
            if !(c2rust_fresh10 != 0) {
                break;
            }
            let mut iNext: Pgno = 0 as Pgno;
            let mut pOvfl: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
            if ovflPgno < 2 as ::core::ffi::c_uint || ovflPgno > btreePagecount(pBt) {
                return sqlite3CorruptError(6974 as ::core::ffi::c_int);
            }
            if nOvfl != 0 {
                rc = getOverflowPage(pBt, ovflPgno, &raw mut pOvfl, &raw mut iNext);
                if rc != 0 {
                    return rc;
                }
            }
            if (!pOvfl.is_null()
                || {
                    pOvfl = btreePageLookup(pBt, ovflPgno);
                    !pOvfl.is_null()
                })
                && sqlite3PagerPageRefcount((*pOvfl).pDbPage) != 1 as ::core::ffi::c_int
            {
                rc = sqlite3CorruptError(6994 as ::core::ffi::c_int);
            } else {
                rc = freePage2(pBt, pOvfl, ovflPgno);
            }
            if !pOvfl.is_null() {
                sqlite3PagerUnref((*pOvfl).pDbPage);
            }
            if rc != 0 {
                return rc;
            }
            ovflPgno = iNext;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fillInCell(
    mut pPage: *mut MemPage,
    mut pCell: *mut ::core::ffi::c_uchar,
    mut pX: *const BtreePayload,
    mut pnSize: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nPayload: ::core::ffi::c_int = 0;
        let mut pSrc: *const u8_0 = ::core::ptr::null::<u8_0>();
        let mut nSrc: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut mn: ::core::ffi::c_int = 0;
        let mut spaceLeft: ::core::ffi::c_int = 0;
        let mut pToRelease: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut pPrior: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut pPayload: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut pBt: *mut BtShared = ::core::ptr::null_mut::<BtShared>();
        let mut pgnoOvfl: Pgno = 0;
        let mut nHeader: ::core::ffi::c_int = 0;
        nHeader = (*pPage).childPtrSize as ::core::ffi::c_int;
        if (*pPage).intKey != 0 {
            nPayload = (*pX).nData + (*pX).nZero;
            pSrc = (*pX).pData as *const u8_0;
            nSrc = (*pX).nData;
            nHeader
                += (if (nPayload as u32_0) < 0x80 as ::core::ffi::c_int as u32_0 {
                    *pCell.offset(nHeader as isize) = nPayload as ::core::ffi::c_uchar;
                    1 as ::core::ffi::c_int
                } else {
                    sqlite3PutVarint(
                        pCell.offset(nHeader as isize) as *mut ::core::ffi::c_uchar,
                        nPayload as u64_0,
                    )
                }) as u8_0 as ::core::ffi::c_int;
            nHeader
                += sqlite3PutVarint(
                    pCell.offset(nHeader as isize) as *mut ::core::ffi::c_uchar,
                    *(&raw const (*pX).nKey as *mut u64_0),
                );
        } else {
            nPayload = (*pX).nKey as ::core::ffi::c_int;
            nSrc = nPayload;
            pSrc = (*pX).pKey as *const u8_0;
            nHeader
                += (if (nPayload as u32_0) < 0x80 as ::core::ffi::c_int as u32_0 {
                    *pCell.offset(nHeader as isize) = nPayload as ::core::ffi::c_uchar;
                    1 as ::core::ffi::c_int
                } else {
                    sqlite3PutVarint(
                        pCell.offset(nHeader as isize) as *mut ::core::ffi::c_uchar,
                        nPayload as u64_0,
                    )
                }) as u8_0 as ::core::ffi::c_int;
        }
        pPayload = pCell.offset(nHeader as isize) as *mut ::core::ffi::c_uchar;
        if nPayload <= (*pPage).maxLocal as ::core::ffi::c_int {
            n = nHeader + nPayload;
            if n < 4 as ::core::ffi::c_int {
                n = 4 as ::core::ffi::c_int;
                *pPayload.offset(nPayload as isize) = 0 as ::core::ffi::c_uchar;
            }
            *pnSize = n;
            memcpy(
                pPayload as *mut ::core::ffi::c_void,
                pSrc as *const ::core::ffi::c_void,
                nSrc as size_t,
            );
            memset(
                pPayload.offset(nSrc as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (nPayload - nSrc) as size_t,
            );
            return SQLITE_OK;
        }
        mn = (*pPage).minLocal as ::core::ffi::c_int;
        n = (mn as ::core::ffi::c_uint)
            .wrapping_add(
                ((nPayload - mn) as ::core::ffi::c_uint)
                    .wrapping_rem(
                        ((*(*pPage).pBt).usableSize as ::core::ffi::c_uint)
                            .wrapping_sub(4 as ::core::ffi::c_uint),
                    ),
            ) as ::core::ffi::c_int;
        if n > (*pPage).maxLocal as ::core::ffi::c_int {
            n = mn;
        }
        spaceLeft = n;
        *pnSize = n + nHeader + 4 as ::core::ffi::c_int;
        pPrior = pCell.offset((nHeader + n) as isize) as *mut ::core::ffi::c_uchar;
        pToRelease = ::core::ptr::null_mut::<MemPage>();
        pgnoOvfl = 0 as Pgno;
        pBt = (*pPage).pBt;
        loop {
            n = nPayload;
            if n > spaceLeft {
                n = spaceLeft;
            }
            if nSrc >= n {
                memcpy(
                    pPayload as *mut ::core::ffi::c_void,
                    pSrc as *const ::core::ffi::c_void,
                    n as size_t,
                );
            } else if nSrc > 0 as ::core::ffi::c_int {
                n = nSrc;
                memcpy(
                    pPayload as *mut ::core::ffi::c_void,
                    pSrc as *const ::core::ffi::c_void,
                    n as size_t,
                );
            } else {
                memset(
                    pPayload as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    n as size_t,
                );
            }
            nPayload -= n;
            if nPayload <= 0 as ::core::ffi::c_int {
                break;
            }
            pPayload = pPayload.offset(n as isize);
            pSrc = pSrc.offset(n as isize);
            nSrc -= n;
            spaceLeft -= n;
            if spaceLeft == 0 as ::core::ffi::c_int {
                let mut pOvfl: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
                let mut pgnoPtrmap: Pgno = pgnoOvfl;
                if (*pBt).autoVacuum != 0 {
                    loop {
                        pgnoOvfl = pgnoOvfl.wrapping_add(1);
                        if !(ptrmapPageno(pBt, pgnoOvfl) == pgnoOvfl
                            || pgnoOvfl
                                == (sqlite3PendingByte as ::core::ffi::c_uint)
                                    .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                                    .wrapping_add(1 as ::core::ffi::c_uint))
                        {
                            break;
                        }
                    }
                }
                rc = allocateBtreePage(
                    pBt,
                    &raw mut pOvfl,
                    &raw mut pgnoOvfl,
                    pgnoOvfl,
                    0 as u8_0,
                );
                if (*pBt).autoVacuum as ::core::ffi::c_int != 0 && rc == SQLITE_OK {
                    let mut eType: u8_0 = (if pgnoPtrmap != 0 {
                        PTRMAP_OVERFLOW2
                    } else {
                        PTRMAP_OVERFLOW1
                    }) as u8_0;
                    ptrmapPut(pBt, pgnoOvfl, eType, pgnoPtrmap, &raw mut rc);
                    if rc != 0 {
                        releasePage(pOvfl);
                    }
                }
                if rc != 0 {
                    releasePage(pToRelease);
                    return rc;
                }
                sqlite3Put4byte(pPrior as *mut u8_0, pgnoOvfl as u32_0);
                releasePage(pToRelease);
                pToRelease = pOvfl;
                pPrior = (*pOvfl).aData as *mut ::core::ffi::c_uchar;
                sqlite3Put4byte(pPrior as *mut u8_0, 0 as u32_0);
                pPayload = (*pOvfl).aData.offset(4 as ::core::ffi::c_int as isize)
                    as *mut u8_0 as *mut ::core::ffi::c_uchar;
                spaceLeft = ((*pBt).usableSize as ::core::ffi::c_uint)
                    .wrapping_sub(4 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            }
        }
        releasePage(pToRelease);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn dropCell(
    mut pPage: *mut MemPage,
    mut idx: ::core::ffi::c_int,
    mut sz: ::core::ffi::c_int,
    mut pRC: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut pc: u32_0 = 0;
        let mut data: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut ptr: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut hdr: ::core::ffi::c_int = 0;
        if *pRC != 0 {
            return;
        }
        data = (*pPage).aData;
        ptr = (*pPage).aCellIdx.offset((2 as ::core::ffi::c_int * idx) as isize)
            as *mut u8_0;
        pc = ((*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as u32_0;
        hdr = (*pPage).hdrOffset as ::core::ffi::c_int;
        if (pc as ::core::ffi::c_uint).wrapping_add(sz as ::core::ffi::c_uint)
            > (*(*pPage).pBt).usableSize
        {
            *pRC = sqlite3CorruptError(7250 as ::core::ffi::c_int);
            return;
        }
        rc = freeSpace(pPage, pc as ::core::ffi::c_int, sz);
        if rc != 0 {
            *pRC = rc;
            return;
        }
        (*pPage).nCell = (*pPage).nCell.wrapping_sub(1);
        if (*pPage).nCell as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            memset(
                data.offset((hdr + 1 as ::core::ffi::c_int) as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                4 as size_t,
            );
            *data.offset((hdr + 7 as ::core::ffi::c_int) as isize) = 0 as u8_0;
            *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut u8_0)
                .offset(0 as ::core::ffi::c_int as isize) = ((*(*pPage).pBt).usableSize
                >> 8 as ::core::ffi::c_int) as u8_0;
            *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut u8_0)
                .offset(1 as ::core::ffi::c_int as isize) = (*(*pPage).pBt).usableSize
                as u8_0;
            (*pPage).nFree = ((*(*pPage).pBt).usableSize as ::core::ffi::c_uint)
                .wrapping_sub((*pPage).hdrOffset as ::core::ffi::c_uint)
                .wrapping_sub((*pPage).childPtrSize as ::core::ffi::c_uint)
                .wrapping_sub(8 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        } else {
            memmove(
                ptr as *mut ::core::ffi::c_void,
                ptr.offset(2 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                (2 as ::core::ffi::c_int * ((*pPage).nCell as ::core::ffi::c_int - idx))
                    as size_t,
            );
            *(data.offset((hdr + 3 as ::core::ffi::c_int) as isize) as *mut u8_0)
                .offset(0 as ::core::ffi::c_int as isize) = ((*pPage).nCell
                as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as u8_0;
            *(data.offset((hdr + 3 as ::core::ffi::c_int) as isize) as *mut u8_0)
                .offset(1 as ::core::ffi::c_int as isize) = (*pPage).nCell as u8_0;
            (*pPage).nFree += 2 as ::core::ffi::c_int;
        };
    }
}
unsafe extern "C" fn insertCell(
    mut pPage: *mut MemPage,
    mut i: ::core::ffi::c_int,
    mut pCell: *mut u8_0,
    mut sz: ::core::ffi::c_int,
    mut pTemp: *mut u8_0,
    mut iChild: Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int = 0;
        let mut data: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pIns: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        if (*pPage).nOverflow as ::core::ffi::c_int != 0
            || sz + 2 as ::core::ffi::c_int > (*pPage).nFree
        {
            if !pTemp.is_null() {
                memcpy(
                    pTemp as *mut ::core::ffi::c_void,
                    pCell as *const ::core::ffi::c_void,
                    sz as size_t,
                );
                pCell = pTemp;
            }
            sqlite3Put4byte(pCell, iChild as u32_0);
            let c2rust_fresh22 = (*pPage).nOverflow;
            (*pPage).nOverflow = (*pPage).nOverflow.wrapping_add(1);
            j = c2rust_fresh22 as ::core::ffi::c_int;
            (*pPage).apOvfl[j as usize] = pCell;
            (*pPage).aiOvfl[j as usize] = i as u16_0;
        } else {
            let mut rc: ::core::ffi::c_int = sqlite3PagerWrite((*pPage).pDbPage);
            if rc != 0 as ::core::ffi::c_int {
                return rc;
            }
            data = (*pPage).aData;
            rc = allocateSpace(pPage, sz, &raw mut idx);
            if rc != 0 {
                return rc;
            }
            (*pPage).nFree
                -= (2 as ::core::ffi::c_int + sz) as u16_0 as ::core::ffi::c_int;
            memcpy(
                data.offset((idx + 4 as ::core::ffi::c_int) as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void,
                pCell.offset(4 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                (sz - 4 as ::core::ffi::c_int) as size_t,
            );
            sqlite3Put4byte(data.offset(idx as isize) as *mut u8_0, iChild as u32_0);
            pIns = (*pPage).aCellIdx.offset((i * 2 as ::core::ffi::c_int) as isize);
            memmove(
                pIns.offset(2 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                pIns as *const ::core::ffi::c_void,
                (2 as ::core::ffi::c_int * ((*pPage).nCell as ::core::ffi::c_int - i))
                    as size_t,
            );
            *pIns.offset(0 as ::core::ffi::c_int as isize) = (idx
                >> 8 as ::core::ffi::c_int) as u8_0;
            *pIns.offset(1 as ::core::ffi::c_int as isize) = idx as u8_0;
            (*pPage).nCell = (*pPage).nCell.wrapping_add(1);
            let ref mut c2rust_fresh23 = *data
                .offset(
                    ((*pPage).hdrOffset as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        as isize,
                );
            *c2rust_fresh23 = (*c2rust_fresh23).wrapping_add(1);
            if *c2rust_fresh23 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let ref mut c2rust_fresh24 = *data
                    .offset(
                        ((*pPage).hdrOffset as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int) as isize,
                    );
                *c2rust_fresh24 = (*c2rust_fresh24).wrapping_add(1);
            }
            if (*(*pPage).pBt).autoVacuum != 0 {
                let mut rc2: ::core::ffi::c_int = SQLITE_OK;
                ptrmapPutOvflPtr(pPage, pPage, pCell, &raw mut rc2);
                if rc2 != 0 {
                    return rc2;
                }
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn insertCellFast(
    mut pPage: *mut MemPage,
    mut i: ::core::ffi::c_int,
    mut pCell: *mut u8_0,
    mut sz: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int = 0;
        let mut data: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pIns: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        if sz + 2 as ::core::ffi::c_int > (*pPage).nFree {
            let c2rust_fresh28 = (*pPage).nOverflow;
            (*pPage).nOverflow = (*pPage).nOverflow.wrapping_add(1);
            j = c2rust_fresh28 as ::core::ffi::c_int;
            (*pPage).apOvfl[j as usize] = pCell;
            (*pPage).aiOvfl[j as usize] = i as u16_0;
        } else {
            let mut rc: ::core::ffi::c_int = sqlite3PagerWrite((*pPage).pDbPage);
            if rc != SQLITE_OK {
                return rc;
            }
            data = (*pPage).aData;
            rc = allocateSpace(pPage, sz, &raw mut idx);
            if rc != 0 {
                return rc;
            }
            (*pPage).nFree
                -= (2 as ::core::ffi::c_int + sz) as u16_0 as ::core::ffi::c_int;
            memcpy(
                data.offset(idx as isize) as *mut u8_0 as *mut ::core::ffi::c_void,
                pCell as *const ::core::ffi::c_void,
                sz as size_t,
            );
            pIns = (*pPage).aCellIdx.offset((i * 2 as ::core::ffi::c_int) as isize);
            memmove(
                pIns.offset(2 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                pIns as *const ::core::ffi::c_void,
                (2 as ::core::ffi::c_int * ((*pPage).nCell as ::core::ffi::c_int - i))
                    as size_t,
            );
            *pIns.offset(0 as ::core::ffi::c_int as isize) = (idx
                >> 8 as ::core::ffi::c_int) as u8_0;
            *pIns.offset(1 as ::core::ffi::c_int as isize) = idx as u8_0;
            (*pPage).nCell = (*pPage).nCell.wrapping_add(1);
            let ref mut c2rust_fresh29 = *data
                .offset(
                    ((*pPage).hdrOffset as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        as isize,
                );
            *c2rust_fresh29 = (*c2rust_fresh29).wrapping_add(1);
            if *c2rust_fresh29 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let ref mut c2rust_fresh30 = *data
                    .offset(
                        ((*pPage).hdrOffset as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int) as isize,
                    );
                *c2rust_fresh30 = (*c2rust_fresh30).wrapping_add(1);
            }
            if (*(*pPage).pBt).autoVacuum != 0 {
                let mut rc2: ::core::ffi::c_int = SQLITE_OK;
                ptrmapPutOvflPtr(pPage, pPage, pCell, &raw mut rc2);
                if rc2 != 0 {
                    return rc2;
                }
            }
        }
        return SQLITE_OK;
    }
}
pub const NB: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
unsafe extern "C" fn populateCellCache(
    mut p: *mut CellArray,
    mut idx: ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
) {
    unsafe {
        let mut pRef: *mut MemPage = (*p).pRef;
        let mut szCell: *mut u16_0 = (*p).szCell;
        while N > 0 as ::core::ffi::c_int {
            if *szCell.offset(idx as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                *szCell.offset(idx as isize) = (*pRef)
                    .xCellSize
                    .expect(
                        "non-null function pointer",
                    )(pRef, *(*p).apCell.offset(idx as isize));
            }
            idx += 1;
            N -= 1;
        }
    }
}
#[inline(never)]
unsafe extern "C" fn computeCellSize(
    mut p: *mut CellArray,
    mut N: ::core::ffi::c_int,
) -> u16_0 {
    unsafe {
        *(*p).szCell.offset(N as isize) = (*(*p).pRef)
            .xCellSize
            .expect(
                "non-null function pointer",
            )((*p).pRef, *(*p).apCell.offset(N as isize));
        return *(*p).szCell.offset(N as isize);
    }
}
unsafe extern "C" fn cachedCellSize(
    mut p: *mut CellArray,
    mut N: ::core::ffi::c_int,
) -> u16_0 {
    unsafe {
        if *(*p).szCell.offset(N as isize) != 0 {
            return *(*p).szCell.offset(N as isize);
        }
        return computeCellSize(p, N);
    }
}
unsafe extern "C" fn rebuildPage(
    mut pCArray: *mut CellArray,
    mut iFirst: ::core::ffi::c_int,
    mut nCell: ::core::ffi::c_int,
    mut pPg: *mut MemPage,
) -> ::core::ffi::c_int {
    unsafe {
        let hdr: ::core::ffi::c_int = (*pPg).hdrOffset as ::core::ffi::c_int;
        let aData: *mut u8_0 = (*pPg).aData;
        let usableSize: ::core::ffi::c_int = (*(*pPg).pBt).usableSize
            as ::core::ffi::c_int;
        let pEnd: *mut u8_0 = aData.offset(usableSize as isize) as *mut u8_0;
        let mut i: ::core::ffi::c_int = iFirst;
        let mut j: u32_0 = 0;
        let mut iEnd: ::core::ffi::c_int = i + nCell;
        let mut pCellptr: *mut u8_0 = (*pPg).aCellIdx;
        let mut pTmp: *mut u8_0 = sqlite3PagerTempSpace((*(*pPg).pBt).pPager)
            as *mut u8_0;
        let mut pData: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut k: ::core::ffi::c_int = 0;
        let mut pSrcEnd: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        j = ((*(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut u8_0)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut u8_0)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as u32_0;
        if j > usableSize as u32_0 {
            j = 0 as u32_0;
        }
        memcpy(
            pTmp.offset(j as isize) as *mut u8_0 as *mut ::core::ffi::c_void,
            aData.offset(j as isize) as *mut u8_0 as *const ::core::ffi::c_void,
            (usableSize as u32_0).wrapping_sub(j) as size_t,
        );
        k = 0 as ::core::ffi::c_int;
        while (*pCArray).ixNx[k as usize] <= i {
            k += 1;
        }
        pSrcEnd = (*pCArray).apEnd[k as usize];
        pData = pEnd;
        loop {
            let mut pCell: *mut u8_0 = *(*pCArray).apCell.offset(i as isize);
            let mut sz: u16_0 = *(*pCArray).szCell.offset(i as isize);
            if pCell as uptr >= aData.offset(j as isize) as uptr
                && (pCell as uptr) < pEnd as uptr
            {
                if pCell.offset(sz as ::core::ffi::c_int as isize) as uptr > pEnd as uptr
                {
                    return sqlite3CorruptError(7640 as ::core::ffi::c_int);
                }
                pCell = pTmp
                    .offset(pCell.offset_from(aData) as ::core::ffi::c_long as isize)
                    as *mut u8_0;
            } else if pCell.offset(sz as ::core::ffi::c_int as isize) as uptr
                > pSrcEnd as uptr && (pCell as uptr) < pSrcEnd as uptr
            {
                return sqlite3CorruptError(7645 as ::core::ffi::c_int)
            }
            pData = pData.offset(-(sz as ::core::ffi::c_int as isize));
            *pCellptr.offset(0 as ::core::ffi::c_int as isize) = (pData
                .offset_from(aData) as ::core::ffi::c_long >> 8 as ::core::ffi::c_int)
                as u8_0;
            *pCellptr.offset(1 as ::core::ffi::c_int as isize) = pData.offset_from(aData)
                as ::core::ffi::c_long as u8_0;
            pCellptr = pCellptr.offset(2 as ::core::ffi::c_int as isize);
            if pData < pCellptr {
                return sqlite3CorruptError(7651 as ::core::ffi::c_int);
            }
            memmove(
                pData as *mut ::core::ffi::c_void,
                pCell as *const ::core::ffi::c_void,
                sz as size_t,
            );
            i += 1;
            if i >= iEnd {
                break;
            }
            if (*pCArray).ixNx[k as usize] <= i {
                k += 1;
                pSrcEnd = (*pCArray).apEnd[k as usize];
            }
        }
        (*pPg).nCell = nCell as u16_0;
        (*pPg).nOverflow = 0 as u8_0;
        *(aData.offset((hdr + 1 as ::core::ffi::c_int) as isize) as *mut u8_0)
            .offset(0 as ::core::ffi::c_int as isize) = (0 as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as u8_0;
        *(aData.offset((hdr + 1 as ::core::ffi::c_int) as isize) as *mut u8_0)
            .offset(1 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int as u8_0;
        *(aData.offset((hdr + 3 as ::core::ffi::c_int) as isize) as *mut u8_0)
            .offset(0 as ::core::ffi::c_int as isize) = ((*pPg).nCell
            as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as u8_0;
        *(aData.offset((hdr + 3 as ::core::ffi::c_int) as isize) as *mut u8_0)
            .offset(1 as ::core::ffi::c_int as isize) = (*pPg).nCell as u8_0;
        *(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut u8_0)
            .offset(0 as ::core::ffi::c_int as isize) = (pData.offset_from(aData)
            as ::core::ffi::c_long >> 8 as ::core::ffi::c_int) as u8_0;
        *(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut u8_0)
            .offset(1 as ::core::ffi::c_int as isize) = pData.offset_from(aData)
            as ::core::ffi::c_long as u8_0;
        *aData.offset((hdr + 7 as ::core::ffi::c_int) as isize) = 0 as u8_0;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pageInsertArray(
    mut pPg: *mut MemPage,
    mut pBegin: *mut u8_0,
    mut ppData: *mut *mut u8_0,
    mut pCellptr: *mut u8_0,
    mut iFirst: ::core::ffi::c_int,
    mut nCell: ::core::ffi::c_int,
    mut pCArray: *mut CellArray,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = iFirst;
        let mut aData: *mut u8_0 = (*pPg).aData;
        let mut pData: *mut u8_0 = *ppData;
        let mut iEnd: ::core::ffi::c_int = iFirst + nCell;
        let mut k: ::core::ffi::c_int = 0;
        let mut pEnd: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        if iEnd <= iFirst {
            return 0 as ::core::ffi::c_int;
        }
        k = 0 as ::core::ffi::c_int;
        while (*pCArray).ixNx[k as usize] <= i {
            k += 1;
        }
        pEnd = (*pCArray).apEnd[k as usize];
        loop {
            let mut sz: ::core::ffi::c_int = 0;
            let mut rc: ::core::ffi::c_int = 0;
            let mut pSlot: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
            sz = *(*pCArray).szCell.offset(i as isize) as ::core::ffi::c_int;
            if *aData.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && *aData.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                || {
                    pSlot = pageFindSlot(pPg, sz, &raw mut rc);
                    pSlot.is_null()
                }
            {
                if (pData.offset_from(pBegin) as ::core::ffi::c_long)
                    < sz as ::core::ffi::c_long
                {
                    return 1 as ::core::ffi::c_int;
                }
                pData = pData.offset(-(sz as isize));
                pSlot = pData;
            }
            if (*(*pCArray).apCell.offset(i as isize)).offset(sz as isize) as uptr
                > pEnd as uptr
                && (*(*pCArray).apCell.offset(i as isize) as uptr) < pEnd as uptr
            {
                sqlite3CorruptError(7738 as ::core::ffi::c_int);
                return 1 as ::core::ffi::c_int;
            }
            memmove(
                pSlot as *mut ::core::ffi::c_void,
                *(*pCArray).apCell.offset(i as isize) as *const ::core::ffi::c_void,
                sz as size_t,
            );
            *pCellptr.offset(0 as ::core::ffi::c_int as isize) = (pSlot
                .offset_from(aData) as ::core::ffi::c_long >> 8 as ::core::ffi::c_int)
                as u8_0;
            *pCellptr.offset(1 as ::core::ffi::c_int as isize) = pSlot.offset_from(aData)
                as ::core::ffi::c_long as u8_0;
            pCellptr = pCellptr.offset(2 as ::core::ffi::c_int as isize);
            i += 1;
            if i >= iEnd {
                break;
            }
            if (*pCArray).ixNx[k as usize] <= i {
                k += 1;
                pEnd = (*pCArray).apEnd[k as usize];
            }
        }
        *ppData = pData;
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn pageFreeArray(
    mut pPg: *mut MemPage,
    mut iFirst: ::core::ffi::c_int,
    mut nCell: ::core::ffi::c_int,
    mut pCArray: *mut CellArray,
) -> ::core::ffi::c_int {
    unsafe {
        let aData: *mut u8_0 = (*pPg).aData;
        let pEnd: *mut u8_0 = aData.offset((*(*pPg).pBt).usableSize as isize)
            as *mut u8_0;
        let pStart: *mut u8_0 = aData
            .offset(
                ((*pPg).hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + (*pPg).childPtrSize as ::core::ffi::c_int) as isize,
            ) as *mut u8_0;
        let mut nRet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut iEnd: ::core::ffi::c_int = iFirst + nCell;
        let mut nFree: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aOfst: [::core::ffi::c_int; 10] = [0; 10];
        let mut aAfter: [::core::ffi::c_int; 10] = [0; 10];
        i = iFirst;
        while i < iEnd {
            let mut pCell: *mut u8_0 = *(*pCArray).apCell.offset(i as isize);
            if pCell as uptr >= pStart as uptr && (pCell as uptr) < pEnd as uptr {
                let mut sz: ::core::ffi::c_int = 0;
                let mut iAfter: ::core::ffi::c_int = 0;
                let mut iOfst: ::core::ffi::c_int = 0;
                sz = *(*pCArray).szCell.offset(i as isize) as ::core::ffi::c_int;
                iOfst = pCell.offset_from(aData) as ::core::ffi::c_long as u16_0
                    as ::core::ffi::c_int;
                iAfter = iOfst + sz;
                j = 0 as ::core::ffi::c_int;
                while j < nFree {
                    if aOfst[j as usize] == iAfter {
                        aOfst[j as usize] = iOfst;
                        break;
                    } else if aAfter[j as usize] == iOfst {
                        aAfter[j as usize] = iAfter;
                        break;
                    } else {
                        j += 1;
                    }
                }
                if j >= nFree {
                    if nFree
                        >= (::core::mem::size_of::<[::core::ffi::c_int; 10]>() as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) as ::core::ffi::c_int
                    {
                        j = 0 as ::core::ffi::c_int;
                        while j < nFree {
                            freeSpace(
                                pPg,
                                aOfst[j as usize],
                                aAfter[j as usize] - aOfst[j as usize],
                            );
                            j += 1;
                        }
                        nFree = 0 as ::core::ffi::c_int;
                    }
                    aOfst[nFree as usize] = iOfst;
                    aAfter[nFree as usize] = iAfter;
                    if aData.offset(iAfter as isize) as *mut u8_0 > pEnd {
                        return 0 as ::core::ffi::c_int;
                    }
                    nFree += 1;
                }
                nRet += 1;
            }
            i += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < nFree {
            freeSpace(pPg, aOfst[j as usize], aAfter[j as usize] - aOfst[j as usize]);
            j += 1;
        }
        return nRet;
    }
}
unsafe extern "C" fn editPage(
    mut pPg: *mut MemPage,
    mut iOld: ::core::ffi::c_int,
    mut iNew: ::core::ffi::c_int,
    mut nNew: ::core::ffi::c_int,
    mut pCArray: *mut CellArray,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let aData: *mut u8_0 = (*pPg).aData;
        let hdr: ::core::ffi::c_int = (*pPg).hdrOffset as ::core::ffi::c_int;
        let mut pBegin: *mut u8_0 = (*pPg)
            .aCellIdx
            .offset((nNew * 2 as ::core::ffi::c_int) as isize) as *mut u8_0;
        let mut nCell: ::core::ffi::c_int = (*pPg).nCell as ::core::ffi::c_int;
        let mut pData: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pCellptr: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut i: ::core::ffi::c_int = 0;
        let mut iOldEnd: ::core::ffi::c_int = iOld + (*pPg).nCell as ::core::ffi::c_int
            + (*pPg).nOverflow as ::core::ffi::c_int;
        let mut iNewEnd: ::core::ffi::c_int = iNew + nNew;
        if iOld < iNew {
            let mut nShift: ::core::ffi::c_int = pageFreeArray(
                pPg,
                iOld,
                iNew - iOld,
                pCArray,
            );
            if nShift > nCell {
                return sqlite3CorruptError(7860 as ::core::ffi::c_int);
            }
            memmove(
                (*pPg).aCellIdx as *mut ::core::ffi::c_void,
                (*pPg).aCellIdx.offset((nShift * 2 as ::core::ffi::c_int) as isize)
                    as *mut u8_0 as *const ::core::ffi::c_void,
                (nCell * 2 as ::core::ffi::c_int) as size_t,
            );
            nCell -= nShift;
        }
        if iNewEnd < iOldEnd {
            let mut nTail: ::core::ffi::c_int = pageFreeArray(
                pPg,
                iNewEnd,
                iOldEnd - iNewEnd,
                pCArray,
            );
            nCell -= nTail;
        }
        pData = aData
            .offset(
                ((*(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut u8_0)
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                        as *mut u8_0)
                        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    as isize,
            ) as *mut u8_0;
        if !(pData < pBegin) {
            if !(pData > (*pPg).aDataEnd) {
                if iNew < iOld {
                    let mut nAdd: ::core::ffi::c_int = if nNew < iOld - iNew {
                        nNew
                    } else {
                        iOld - iNew
                    };
                    pCellptr = (*pPg).aCellIdx;
                    memmove(
                        pCellptr.offset((nAdd * 2 as ::core::ffi::c_int) as isize)
                            as *mut u8_0 as *mut ::core::ffi::c_void,
                        pCellptr as *const ::core::ffi::c_void,
                        (nCell * 2 as ::core::ffi::c_int) as size_t,
                    );
                    if pageInsertArray(
                        pPg,
                        pBegin,
                        &raw mut pData,
                        pCellptr,
                        iNew,
                        nAdd,
                        pCArray,
                    ) != 0
                    {
                        c2rust_current_block = 5741245804949393610;
                    } else {
                        nCell += nAdd;
                        c2rust_current_block = 12147880666119273379;
                    }
                } else {
                    c2rust_current_block = 12147880666119273379;
                }
                match c2rust_current_block {
                    5741245804949393610 => {}
                    _ => {
                        i = 0 as ::core::ffi::c_int;
                        loop {
                            if !(i < (*pPg).nOverflow as ::core::ffi::c_int) {
                                c2rust_current_block = 11459959175219260272;
                                break;
                            }
                            let mut iCell: ::core::ffi::c_int = iOld
                                + (*pPg).aiOvfl[i as usize] as ::core::ffi::c_int - iNew;
                            if iCell >= 0 as ::core::ffi::c_int && iCell < nNew {
                                pCellptr = (*pPg)
                                    .aCellIdx
                                    .offset((iCell * 2 as ::core::ffi::c_int) as isize)
                                    as *mut u8_0;
                                if nCell > iCell {
                                    memmove(
                                        pCellptr.offset(2 as ::core::ffi::c_int as isize)
                                            as *mut u8_0 as *mut ::core::ffi::c_void,
                                        pCellptr as *const ::core::ffi::c_void,
                                        ((nCell - iCell) * 2 as ::core::ffi::c_int) as size_t,
                                    );
                                }
                                nCell += 1;
                                cachedCellSize(pCArray, iCell + iNew);
                                if pageInsertArray(
                                    pPg,
                                    pBegin,
                                    &raw mut pData,
                                    pCellptr,
                                    iCell + iNew,
                                    1 as ::core::ffi::c_int,
                                    pCArray,
                                ) != 0
                                {
                                    c2rust_current_block = 5741245804949393610;
                                    break;
                                }
                            }
                            i += 1;
                        }
                        match c2rust_current_block {
                            5741245804949393610 => {}
                            _ => {
                                pCellptr = (*pPg)
                                    .aCellIdx
                                    .offset((nCell * 2 as ::core::ffi::c_int) as isize)
                                    as *mut u8_0;
                                if !(pageInsertArray(
                                    pPg,
                                    pBegin,
                                    &raw mut pData,
                                    pCellptr,
                                    iNew + nCell,
                                    nNew - nCell,
                                    pCArray,
                                ) != 0)
                                {
                                    (*pPg).nCell = nNew as u16_0;
                                    (*pPg).nOverflow = 0 as u8_0;
                                    *(aData.offset((hdr + 3 as ::core::ffi::c_int) as isize)
                                        as *mut u8_0)
                                        .offset(0 as ::core::ffi::c_int as isize) = ((*pPg).nCell
                                        as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as u8_0;
                                    *(aData.offset((hdr + 3 as ::core::ffi::c_int) as isize)
                                        as *mut u8_0)
                                        .offset(1 as ::core::ffi::c_int as isize) = (*pPg).nCell
                                        as u8_0;
                                    *(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                                        as *mut u8_0)
                                        .offset(0 as ::core::ffi::c_int as isize) = (pData
                                        .offset_from(aData) as ::core::ffi::c_long
                                        >> 8 as ::core::ffi::c_int) as u8_0;
                                    *(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                                        as *mut u8_0)
                                        .offset(1 as ::core::ffi::c_int as isize) = pData
                                        .offset_from(aData) as ::core::ffi::c_long as u8_0;
                                    return SQLITE_OK;
                                }
                            }
                        }
                    }
                }
            }
        }
        if nNew < 1 as ::core::ffi::c_int {
            return sqlite3CorruptError(7938 as ::core::ffi::c_int);
        }
        populateCellCache(pCArray, iNew, nNew);
        return rebuildPage(pCArray, iNew, nNew, pPg);
    }
}
unsafe extern "C" fn balance_quick(
    mut pParent: *mut MemPage,
    mut pPage: *mut MemPage,
    mut pSpace: *mut u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let pBt: *mut BtShared = (*pPage).pBt;
        let mut pNew: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut pgnoNew: Pgno = 0;
        if (*pPage).nCell as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return sqlite3CorruptError(7978 as ::core::ffi::c_int);
        }
        rc = allocateBtreePage(
            pBt,
            &raw mut pNew,
            &raw mut pgnoNew,
            0 as Pgno,
            0 as u8_0,
        );
        if rc == SQLITE_OK {
            let mut pOut: *mut u8_0 = pSpace.offset(4 as ::core::ffi::c_int as isize)
                as *mut u8_0;
            let mut pCell: *mut u8_0 = (*pPage).apOvfl[0 as ::core::ffi::c_int as usize];
            let mut szCell: u16_0 = (*pPage)
                .xCellSize
                .expect("non-null function pointer")(pPage, pCell);
            let mut pStop: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
            let mut b: CellArray = CellArray {
                nCell: 0,
                pRef: ::core::ptr::null_mut::<MemPage>(),
                apCell: ::core::ptr::null_mut::<*mut u8_0>(),
                szCell: ::core::ptr::null_mut::<u16_0>(),
                apEnd: [::core::ptr::null_mut::<u8_0>(); 6],
                ixNx: [0; 6]

            };
            zeroPage(pNew, PTF_INTKEY | PTF_LEAFDATA | PTF_LEAF);
            b.nCell = 1 as ::core::ffi::c_int;
            b.pRef = pPage;
            b.apCell = &raw mut pCell;
            b.szCell = &raw mut szCell;
            b.apEnd[0 as ::core::ffi::c_int as usize] = (*pPage).aDataEnd;
            b.ixNx[0 as ::core::ffi::c_int as usize] = 2 as ::core::ffi::c_int;
            b.ixNx[(NB * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = 0x7fffffff
                as ::core::ffi::c_int;
            rc = rebuildPage(
                &raw mut b,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                pNew,
            );
            if rc != 0 {
                releasePage(pNew);
                return rc;
            }
            (*pNew).nFree = ((*pBt).usableSize as ::core::ffi::c_uint)
                .wrapping_sub((*pNew).cellOffset as ::core::ffi::c_uint)
                .wrapping_sub(2 as ::core::ffi::c_uint)
                .wrapping_sub(szCell as ::core::ffi::c_uint) as ::core::ffi::c_int;
            if (*pBt).autoVacuum != 0 {
                ptrmapPut(
                    pBt,
                    pgnoNew,
                    PTRMAP_BTREE as u8_0,
                    (*pParent).pgno,
                    &raw mut rc,
                );
                if szCell as ::core::ffi::c_int > (*pNew).minLocal as ::core::ffi::c_int
                {
                    ptrmapPutOvflPtr(pNew, pNew, pCell, &raw mut rc);
                }
            }
            pCell = (*pPage)
                .aData
                .offset(
                    ((*pPage).maskPage as ::core::ffi::c_int
                        & ((*((*pPage)
                            .aCellIdx
                            .offset(
                                (2 as ::core::ffi::c_int
                                    * ((*pPage).nCell as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)) as isize,
                            ) as *mut u8_0)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                            | *((*pPage)
                                .aCellIdx
                                .offset(
                                    (2 as ::core::ffi::c_int
                                        * ((*pPage).nCell as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int)) as isize,
                                ) as *mut u8_0)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)) as isize,
                );
            pStop = pCell.offset(9 as ::core::ffi::c_int as isize) as *mut u8_0;
            loop {
                let c2rust_fresh25 = pCell;
                pCell = pCell.offset(1);
                if !(*c2rust_fresh25 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                    != 0 && pCell < pStop)
                {
                    break;
                }
            }
            pStop = pCell.offset(9 as ::core::ffi::c_int as isize) as *mut u8_0;
            loop {
                let c2rust_fresh26 = pCell;
                pCell = pCell.offset(1);
                let c2rust_fresh27 = pOut;
                pOut = pOut.offset(1);
                *c2rust_fresh27 = *c2rust_fresh26;
                if !(*c2rust_fresh27 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                    != 0 && pCell < pStop)
                {
                    break;
                }
            }
            if rc == SQLITE_OK {
                rc = insertCell(
                    pParent,
                    (*pParent).nCell as ::core::ffi::c_int,
                    pSpace,
                    pOut.offset_from(pSpace) as ::core::ffi::c_long
                        as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<u8_0>(),
                    (*pPage).pgno,
                );
            }
            sqlite3Put4byte(
                (*pParent)
                    .aData
                    .offset(
                        ((*pParent).hdrOffset as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int) as isize,
                    ) as *mut u8_0,
                pgnoNew as u32_0,
            );
            releasePage(pNew);
        }
        return rc;
    }
}
unsafe extern "C" fn copyNodeContent(
    mut pFrom: *mut MemPage,
    mut pTo: *mut MemPage,
    mut pRC: *mut ::core::ffi::c_int,
) {
    unsafe {
        if *pRC == SQLITE_OK {
            let pBt: *mut BtShared = (*pFrom).pBt;
            let aFrom: *mut u8_0 = (*pFrom).aData;
            let aTo: *mut u8_0 = (*pTo).aData;
            let iFromHdr: ::core::ffi::c_int = (*pFrom).hdrOffset as ::core::ffi::c_int;
            let iToHdr: ::core::ffi::c_int = if (*pTo).pgno == 1 as ::core::ffi::c_uint {
                100 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            let mut rc: ::core::ffi::c_int = 0;
            let mut iData: ::core::ffi::c_int = 0;
            iData = (*(aFrom.offset((iFromHdr + 5 as ::core::ffi::c_int) as isize)
                as *mut u8_0)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(aFrom.offset((iFromHdr + 5 as ::core::ffi::c_int) as isize)
                    as *mut u8_0)
                    .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            memcpy(
                aTo.offset(iData as isize) as *mut u8_0 as *mut ::core::ffi::c_void,
                aFrom.offset(iData as isize) as *mut u8_0 as *const ::core::ffi::c_void,
                ((*pBt).usableSize as ::core::ffi::c_uint)
                    .wrapping_sub(iData as ::core::ffi::c_uint) as size_t,
            );
            memcpy(
                aTo.offset(iToHdr as isize) as *mut u8_0 as *mut ::core::ffi::c_void,
                aFrom.offset(iFromHdr as isize) as *mut u8_0
                    as *const ::core::ffi::c_void,
                ((*pFrom).cellOffset as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int * (*pFrom).nCell as ::core::ffi::c_int)
                    as size_t,
            );
            (*pTo).isInit = 0 as u8_0;
            rc = btreeInitPage(pTo);
            if rc == SQLITE_OK {
                rc = btreeComputeFreeSpace(pTo);
            }
            if rc != SQLITE_OK {
                *pRC = rc;
                return;
            }
            if (*pBt).autoVacuum != 0 {
                *pRC = setChildPtrmaps(pTo);
            }
        }
    }
}
unsafe extern "C" fn balance_nonroot(
    mut pParent: *mut MemPage,
    mut iParentIdx: ::core::ffi::c_int,
    mut aOvflSpace: *mut u8_0,
    mut isRoot: ::core::ffi::c_int,
    mut bBulk: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pBt: *mut BtShared = ::core::ptr::null_mut::<BtShared>();
        let mut nMaxCells: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nNew: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nOld: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut nxDiv: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut leafCorrection: u16_0 = 0;
        let mut leafData: ::core::ffi::c_int = 0;
        let mut usableSpace: ::core::ffi::c_int = 0;
        let mut pageFlags: ::core::ffi::c_int = 0;
        let mut iSpace1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iOvflSpace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut szScratch: u64_0 = 0;
        let mut apOld: [*mut MemPage; 3] = [::core::ptr::null_mut::<MemPage>(); 3];
        let mut apNew: [*mut MemPage; 5] = [::core::ptr::null_mut::<MemPage>(); 5];
        let mut pRight: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut apDiv: [*mut u8_0; 2] = [::core::ptr::null_mut::<u8_0>(); 2];
        let mut cntNew: [::core::ffi::c_int; 5] = [0; 5];
        let mut cntOld: [::core::ffi::c_int; 5] = [0; 5];
        let mut szNew: [::core::ffi::c_int; 5] = [0; 5];
        let mut aSpace1: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pgno: Pgno = 0;
        let mut abDone: [u8_0; 5] = [0; 5];
        let mut aPgno: [Pgno; 5] = [0; 5];
        let mut b: CellArray = CellArray {
            nCell: 0,
            pRef: ::core::ptr::null_mut::<MemPage>(),
            apCell: ::core::ptr::null_mut::<*mut u8_0>(),
            szCell: ::core::ptr::null_mut::<u16_0>(),
            apEnd: [::core::ptr::null_mut::<u8_0>(); 6],
            ixNx: [0; 6]

        };
        memset(
            &raw mut abDone as *mut u8_0 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[u8_0; 5]>() as size_t,
        );
        memset(
            &raw mut b as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<CellArray>() as size_t)
                .wrapping_sub(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        b.ixNx[(NB * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = 0x7fffffff
            as ::core::ffi::c_int;
        pBt = (*pParent).pBt;
        if aOvflSpace.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        i = (*pParent).nOverflow as ::core::ffi::c_int
            + (*pParent).nCell as ::core::ffi::c_int;
        if i < 2 as ::core::ffi::c_int {
            nxDiv = 0 as ::core::ffi::c_int;
        } else {
            if iParentIdx == 0 as ::core::ffi::c_int {
                nxDiv = 0 as ::core::ffi::c_int;
            } else if iParentIdx == i {
                nxDiv = i - 2 as ::core::ffi::c_int + bBulk;
            } else {
                nxDiv = iParentIdx - 1 as ::core::ffi::c_int;
            }
            i = 2 as ::core::ffi::c_int - bBulk;
        }
        nOld = i + 1 as ::core::ffi::c_int;
        if i + nxDiv - (*pParent).nOverflow as ::core::ffi::c_int
            == (*pParent).nCell as ::core::ffi::c_int
        {
            pRight = (*pParent)
                .aData
                .offset(
                    ((*pParent).hdrOffset as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int) as isize,
                ) as *mut u8_0;
        } else {
            pRight = (*pParent)
                .aData
                .offset(
                    ((*pParent).maskPage as ::core::ffi::c_int
                        & ((*((*pParent)
                            .aCellIdx
                            .offset(
                                (2 as ::core::ffi::c_int
                                    * (i + nxDiv - (*pParent).nOverflow as ::core::ffi::c_int))
                                    as isize,
                            ) as *mut u8_0)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                            | *((*pParent)
                                .aCellIdx
                                .offset(
                                    (2 as ::core::ffi::c_int
                                        * (i + nxDiv - (*pParent).nOverflow as ::core::ffi::c_int))
                                        as isize,
                                ) as *mut u8_0)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)) as isize,
                );
        }
        pgno = sqlite3Get4byte(pRight) as Pgno;
        loop {
            if rc == SQLITE_OK {
                rc = getAndInitPage(
                    pBt,
                    pgno,
                    (&raw mut apOld as *mut *mut MemPage).offset(i as isize)
                        as *mut *mut MemPage,
                    0 as ::core::ffi::c_int,
                );
            }
            if rc != 0 {
                memset(
                    &raw mut apOld as *mut *mut MemPage as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ((i + 1 as ::core::ffi::c_int) as size_t)
                        .wrapping_mul(::core::mem::size_of::<*mut MemPage>() as size_t),
                );
                c2rust_current_block = 11627155437346537937;
                break;
            } else {
                if (*apOld[i as usize]).nFree < 0 as ::core::ffi::c_int {
                    rc = btreeComputeFreeSpace(apOld[i as usize]);
                    if rc != 0 {
                        memset(
                            &raw mut apOld as *mut *mut MemPage
                                as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            (i as size_t)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut MemPage>() as size_t,
                                ),
                        );
                        c2rust_current_block = 11627155437346537937;
                        break;
                    }
                }
                nMaxCells
                    += (*apOld[i as usize]).nCell as ::core::ffi::c_int
                        + (::core::mem::size_of::<[*mut u8_0; 4]>() as usize)
                            .wrapping_div(::core::mem::size_of::<*mut u8_0>() as usize)
                            as ::core::ffi::c_int;
                let c2rust_fresh13 = i;
                i = i - 1;
                if c2rust_fresh13 == 0 as ::core::ffi::c_int {
                    c2rust_current_block = 13303144130133872306;
                    break;
                }
                if (*pParent).nOverflow as ::core::ffi::c_int != 0
                    && i + nxDiv
                        == (*pParent).aiOvfl[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                {
                    apDiv[i as usize] = (*pParent)
                        .apOvfl[0 as ::core::ffi::c_int as usize];
                    pgno = sqlite3Get4byte(apDiv[i as usize]) as Pgno;
                    szNew[i as usize] = (*pParent)
                        .xCellSize
                        .expect("non-null function pointer")(pParent, apDiv[i as usize])
                        as ::core::ffi::c_int;
                    (*pParent).nOverflow = 0 as u8_0;
                } else {
                    apDiv[i as usize] = (*pParent)
                        .aData
                        .offset(
                            ((*pParent).maskPage as ::core::ffi::c_int
                                & ((*((*pParent)
                                    .aCellIdx
                                    .offset(
                                        (2 as ::core::ffi::c_int
                                            * (i + nxDiv - (*pParent).nOverflow as ::core::ffi::c_int))
                                            as isize,
                                    ) as *mut u8_0)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                    | *((*pParent)
                                        .aCellIdx
                                        .offset(
                                            (2 as ::core::ffi::c_int
                                                * (i + nxDiv - (*pParent).nOverflow as ::core::ffi::c_int))
                                                as isize,
                                        ) as *mut u8_0)
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int)) as isize,
                        );
                    pgno = sqlite3Get4byte(apDiv[i as usize]) as Pgno;
                    szNew[i as usize] = (*pParent)
                        .xCellSize
                        .expect("non-null function pointer")(pParent, apDiv[i as usize])
                        as ::core::ffi::c_int;
                    if (*pBt).btsFlags as ::core::ffi::c_int & BTS_FAST_SECURE != 0 {
                        let mut iOff: ::core::ffi::c_int = 0;
                        iOff = apDiv[i as usize] as ::core::ffi::c_long
                            as ::core::ffi::c_int
                            - (*pParent).aData as ::core::ffi::c_long
                                as ::core::ffi::c_int;
                        if iOff + szNew[i as usize]
                            <= (*pBt).usableSize as ::core::ffi::c_int
                        {
                            memcpy(
                                aOvflSpace.offset(iOff as isize) as *mut u8_0
                                    as *mut ::core::ffi::c_void,
                                apDiv[i as usize] as *const ::core::ffi::c_void,
                                szNew[i as usize] as size_t,
                            );
                            apDiv[i as usize] = aOvflSpace
                                .offset(
                                    (*(&raw mut apDiv as *mut *mut u8_0).offset(i as isize))
                                        .offset_from((*pParent).aData) as ::core::ffi::c_long
                                        as isize,
                                ) as *mut u8_0;
                        }
                    }
                    dropCell(
                        pParent,
                        i + nxDiv - (*pParent).nOverflow as ::core::ffi::c_int,
                        szNew[i as usize],
                        &raw mut rc,
                    );
                }
            }
        }
        match c2rust_current_block {
            13303144130133872306 => {
                nMaxCells = nMaxCells + 3 as ::core::ffi::c_int
                    & !(3 as ::core::ffi::c_int);
                szScratch = (nMaxCells as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut u8_0>() as usize)
                    .wrapping_add(
                        (nMaxCells as usize)
                            .wrapping_mul(::core::mem::size_of::<u16_0>() as usize),
                    )
                    .wrapping_add((*pBt).pageSize as usize) as u64_0;
                b.apCell = sqlite3DbMallocRaw(
                    ::core::ptr::null_mut::<sqlite3>(),
                    szScratch,
                ) as *mut *mut u8_0;
                if b.apCell.is_null() {
                    rc = SQLITE_NOMEM_BKPT;
                } else {
                    b.szCell = b.apCell.offset(nMaxCells as isize) as *mut *mut u8_0
                        as *mut u16_0;
                    aSpace1 = b.szCell.offset(nMaxCells as isize) as *mut u16_0
                        as *mut u8_0;
                    b.pRef = apOld[0 as ::core::ffi::c_int as usize];
                    leafCorrection = ((*b.pRef).leaf as ::core::ffi::c_int
                        * 4 as ::core::ffi::c_int) as u16_0;
                    leafData = (*b.pRef).intKeyLeaf as ::core::ffi::c_int;
                    i = 0 as ::core::ffi::c_int;
                    loop {
                        if !(i < nOld) {
                            c2rust_current_block = 14953815020842398287;
                            break;
                        }
                        let mut pOld: *mut MemPage = apOld[i as usize];
                        let mut limit: ::core::ffi::c_int = (*pOld).nCell
                            as ::core::ffi::c_int;
                        let mut aData: *mut u8_0 = (*pOld).aData;
                        let mut maskPage: u16_0 = (*pOld).maskPage;
                        let mut piCell: *mut u8_0 = aData
                            .offset((*pOld).cellOffset as ::core::ffi::c_int as isize);
                        let mut piEnd: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                        if *(*pOld).aData.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            != *(*apOld[0 as ::core::ffi::c_int as usize])
                                .aData
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                        {
                            rc = sqlite3CorruptError(8402 as ::core::ffi::c_int);
                            c2rust_current_block = 11627155437346537937;
                            break;
                        } else {
                            memset(
                                b.szCell.offset(b.nCell as isize) as *mut u16_0
                                    as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                (::core::mem::size_of::<u16_0>() as size_t)
                                    .wrapping_mul(
                                        (limit + (*pOld).nOverflow as ::core::ffi::c_int) as size_t,
                                    ),
                            );
                            if (*pOld).nOverflow as ::core::ffi::c_int
                                > 0 as ::core::ffi::c_int
                            {
                                if limit
                                    < (*pOld).aiOvfl[0 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                {
                                    rc = sqlite3CorruptError(8426 as ::core::ffi::c_int);
                                    c2rust_current_block = 11627155437346537937;
                                    break;
                                } else {
                                    limit = (*pOld).aiOvfl[0 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int;
                                    j = 0 as ::core::ffi::c_int;
                                    while j < limit {
                                        let ref mut c2rust_fresh14 = *b
                                            .apCell
                                            .offset(b.nCell as isize);
                                        *c2rust_fresh14 = aData
                                            .offset(
                                                (maskPage as ::core::ffi::c_int
                                                    & ((*piCell.offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                                        | *piCell.offset(1 as ::core::ffi::c_int as isize)
                                                            as ::core::ffi::c_int)) as isize,
                                            );
                                        piCell = piCell.offset(2 as ::core::ffi::c_int as isize);
                                        b.nCell += 1;
                                        j += 1;
                                    }
                                    k = 0 as ::core::ffi::c_int;
                                    while k < (*pOld).nOverflow as ::core::ffi::c_int {
                                        let ref mut c2rust_fresh15 = *b
                                            .apCell
                                            .offset(b.nCell as isize);
                                        *c2rust_fresh15 = (*pOld).apOvfl[k as usize];
                                        b.nCell += 1;
                                        k += 1;
                                    }
                                }
                            }
                            piEnd = aData
                                .offset((*pOld).cellOffset as ::core::ffi::c_int as isize)
                                .offset(
                                    (2 as ::core::ffi::c_int
                                        * (*pOld).nCell as ::core::ffi::c_int) as isize,
                                );
                            while piCell < piEnd {
                                let ref mut c2rust_fresh16 = *b
                                    .apCell
                                    .offset(b.nCell as isize);
                                *c2rust_fresh16 = aData
                                    .offset(
                                        (maskPage as ::core::ffi::c_int
                                            & ((*piCell.offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                                | *piCell.offset(1 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int)) as isize,
                                    );
                                piCell = piCell.offset(2 as ::core::ffi::c_int as isize);
                                b.nCell += 1;
                            }
                            cntOld[i as usize] = b.nCell;
                            if i < nOld - 1 as ::core::ffi::c_int && leafData == 0 {
                                let mut sz: u16_0 = szNew[i as usize] as u16_0;
                                let mut pTemp: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                                *b.szCell.offset(b.nCell as isize) = sz;
                                pTemp = aSpace1.offset(iSpace1 as isize) as *mut u8_0;
                                iSpace1 += sz as ::core::ffi::c_int;
                                memcpy(
                                    pTemp as *mut ::core::ffi::c_void,
                                    apDiv[i as usize] as *const ::core::ffi::c_void,
                                    sz as size_t,
                                );
                                let ref mut c2rust_fresh17 = *b
                                    .apCell
                                    .offset(b.nCell as isize);
                                *c2rust_fresh17 = pTemp
                                    .offset(leafCorrection as ::core::ffi::c_int as isize);
                                *b.szCell.offset(b.nCell as isize) = (*b
                                    .szCell
                                    .offset(b.nCell as isize) as ::core::ffi::c_int
                                    - leafCorrection as ::core::ffi::c_int) as u16_0;
                                if (*pOld).leaf == 0 {
                                    memcpy(
                                        *b.apCell.offset(b.nCell as isize)
                                            as *mut ::core::ffi::c_void,
                                        (*pOld).aData.offset(8 as ::core::ffi::c_int as isize)
                                            as *mut u8_0 as *const ::core::ffi::c_void,
                                        4 as size_t,
                                    );
                                } else {
                                    while (*b.szCell.offset(b.nCell as isize)
                                        as ::core::ffi::c_int) < 4 as ::core::ffi::c_int
                                    {
                                        let c2rust_fresh18 = iSpace1;
                                        iSpace1 = iSpace1 + 1;
                                        *aSpace1.offset(c2rust_fresh18 as isize) = 0 as u8_0;
                                        let ref mut c2rust_fresh19 = *b
                                            .szCell
                                            .offset(b.nCell as isize);
                                        *c2rust_fresh19 = (*c2rust_fresh19).wrapping_add(1);
                                    }
                                }
                                b.nCell += 1;
                            }
                            i += 1;
                        }
                    }
                    match c2rust_current_block {
                        11627155437346537937 => {}
                        _ => {
                            usableSpace = ((*pBt).usableSize as ::core::ffi::c_uint)
                                .wrapping_sub(12 as ::core::ffi::c_uint)
                                .wrapping_add(leafCorrection as ::core::ffi::c_uint)
                                as ::core::ffi::c_int;
                            k = 0 as ::core::ffi::c_int;
                            i = k;
                            while i < nOld {
                                let mut p: *mut MemPage = apOld[i as usize];
                                b.apEnd[k as usize] = (*p).aDataEnd;
                                b.ixNx[k as usize] = cntOld[i as usize];
                                if k != 0
                                    && b.ixNx[k as usize]
                                        == b.ixNx[(k - 1 as ::core::ffi::c_int) as usize]
                                {
                                    k -= 1;
                                }
                                if leafData == 0 {
                                    k += 1;
                                    b.apEnd[k as usize] = (*pParent).aDataEnd;
                                    b.ixNx[k as usize] = cntOld[i as usize]
                                        + 1 as ::core::ffi::c_int;
                                }
                                szNew[i as usize] = usableSpace - (*p).nFree;
                                j = 0 as ::core::ffi::c_int;
                                while j < (*p).nOverflow as ::core::ffi::c_int {
                                    szNew[i as usize]
                                        += 2 as ::core::ffi::c_int
                                            + (*p)
                                                .xCellSize
                                                .expect(
                                                    "non-null function pointer",
                                                )(p, (*p).apOvfl[j as usize]) as ::core::ffi::c_int;
                                    j += 1;
                                }
                                cntNew[i as usize] = cntOld[i as usize];
                                i += 1;
                                k += 1;
                            }
                            k = nOld;
                            i = 0 as ::core::ffi::c_int;
                            's_661: loop {
                                if !(i < k) {
                                    c2rust_current_block = 4183419379601546972;
                                    break;
                                }
                                let mut sz_0: ::core::ffi::c_int = 0;
                                while szNew[i as usize] > usableSpace {
                                    if i + 1 as ::core::ffi::c_int >= k {
                                        k = i + 2 as ::core::ffi::c_int;
                                        if k > NB + 2 as ::core::ffi::c_int {
                                            rc = sqlite3CorruptError(8527 as ::core::ffi::c_int);
                                            c2rust_current_block = 11627155437346537937;
                                            break 's_661;
                                        } else {
                                            szNew[(k - 1 as ::core::ffi::c_int) as usize] = 0
                                                as ::core::ffi::c_int;
                                            cntNew[(k - 1 as ::core::ffi::c_int) as usize] = b.nCell;
                                        }
                                    }
                                    sz_0 = 2 as ::core::ffi::c_int
                                        + cachedCellSize(
                                            &raw mut b,
                                            cntNew[i as usize] - 1 as ::core::ffi::c_int,
                                        ) as ::core::ffi::c_int;
                                    szNew[i as usize] -= sz_0;
                                    if leafData == 0 {
                                        if cntNew[i as usize] < b.nCell {
                                            sz_0 = 2 as ::core::ffi::c_int
                                                + cachedCellSize(&raw mut b, cntNew[i as usize])
                                                    as ::core::ffi::c_int;
                                        } else {
                                            sz_0 = 0 as ::core::ffi::c_int;
                                        }
                                    }
                                    szNew[(i + 1 as ::core::ffi::c_int) as usize] += sz_0;
                                    cntNew[i as usize] -= 1;
                                }
                                while cntNew[i as usize] < b.nCell {
                                    sz_0 = 2 as ::core::ffi::c_int
                                        + cachedCellSize(&raw mut b, cntNew[i as usize])
                                            as ::core::ffi::c_int;
                                    if szNew[i as usize] + sz_0 > usableSpace {
                                        break;
                                    }
                                    szNew[i as usize] += sz_0;
                                    cntNew[i as usize] += 1;
                                    if leafData == 0 {
                                        if cntNew[i as usize] < b.nCell {
                                            sz_0 = 2 as ::core::ffi::c_int
                                                + cachedCellSize(&raw mut b, cntNew[i as usize])
                                                    as ::core::ffi::c_int;
                                        } else {
                                            sz_0 = 0 as ::core::ffi::c_int;
                                        }
                                    }
                                    szNew[(i + 1 as ::core::ffi::c_int) as usize] -= sz_0;
                                }
                                if cntNew[i as usize] >= b.nCell {
                                    k = i + 1 as ::core::ffi::c_int;
                                } else if cntNew[i as usize]
                                    <= (if i > 0 as ::core::ffi::c_int {
                                        cntNew[(i - 1 as ::core::ffi::c_int) as usize]
                                    } else {
                                        0 as ::core::ffi::c_int
                                    })
                                {
                                    rc = sqlite3CorruptError(8560 as ::core::ffi::c_int);
                                    c2rust_current_block = 11627155437346537937;
                                    break;
                                }
                                i += 1;
                            }
                            match c2rust_current_block {
                                11627155437346537937 => {}
                                _ => {
                                    i = k - 1 as ::core::ffi::c_int;
                                    loop {
                                        if !(i > 0 as ::core::ffi::c_int) {
                                            c2rust_current_block = 18312853480645871422;
                                            break;
                                        }
                                        let mut szRight: ::core::ffi::c_int = szNew[i as usize];
                                        let mut szLeft: ::core::ffi::c_int = szNew[(i
                                            - 1 as ::core::ffi::c_int) as usize];
                                        let mut r: ::core::ffi::c_int = 0;
                                        let mut d: ::core::ffi::c_int = 0;
                                        r = cntNew[(i - 1 as ::core::ffi::c_int) as usize]
                                            - 1 as ::core::ffi::c_int;
                                        d = r + 1 as ::core::ffi::c_int - leafData;
                                        cachedCellSize(&raw mut b, d);
                                        loop {
                                            let mut szR: ::core::ffi::c_int = 0;
                                            let mut szD: ::core::ffi::c_int = 0;
                                            szR = cachedCellSize(&raw mut b, r) as ::core::ffi::c_int;
                                            szD = *b.szCell.offset(d as isize) as ::core::ffi::c_int;
                                            if szRight != 0 as ::core::ffi::c_int
                                                && (bBulk != 0
                                                    || szRight + szD + 2 as ::core::ffi::c_int
                                                        > szLeft
                                                            - (szR
                                                                + (if i == k - 1 as ::core::ffi::c_int {
                                                                    0 as ::core::ffi::c_int
                                                                } else {
                                                                    2 as ::core::ffi::c_int
                                                                })))
                                            {
                                                break;
                                            }
                                            szRight += szD + 2 as ::core::ffi::c_int;
                                            szLeft -= szR + 2 as ::core::ffi::c_int;
                                            cntNew[(i - 1 as ::core::ffi::c_int) as usize] = r;
                                            r -= 1;
                                            d -= 1;
                                            if !(r >= 0 as ::core::ffi::c_int) {
                                                break;
                                            }
                                        }
                                        szNew[i as usize] = szRight;
                                        szNew[(i - 1 as ::core::ffi::c_int) as usize] = szLeft;
                                        if cntNew[(i - 1 as ::core::ffi::c_int) as usize]
                                            <= (if i > 1 as ::core::ffi::c_int {
                                                cntNew[(i - 2 as ::core::ffi::c_int) as usize]
                                            } else {
                                                0 as ::core::ffi::c_int
                                            })
                                        {
                                            rc = sqlite3CorruptError(8604 as ::core::ffi::c_int);
                                            c2rust_current_block = 11627155437346537937;
                                            break;
                                        } else {
                                            i -= 1;
                                        }
                                    }
                                    match c2rust_current_block {
                                        11627155437346537937 => {}
                                        _ => {
                                            pageFlags = *(*apOld[0 as ::core::ffi::c_int as usize])
                                                .aData
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int;
                                            i = 0 as ::core::ffi::c_int;
                                            loop {
                                                if !(i < k) {
                                                    c2rust_current_block = 8499731551232998623;
                                                    break;
                                                }
                                                let mut pNew: *mut MemPage = ::core::ptr::null_mut::<
                                                    MemPage,
                                                >();
                                                if i < nOld {
                                                    apNew[i as usize] = apOld[i as usize];
                                                    pNew = apNew[i as usize];
                                                    apOld[i as usize] = ::core::ptr::null_mut::<MemPage>();
                                                    rc = sqlite3PagerWrite((*pNew).pDbPage);
                                                    nNew += 1;
                                                    if sqlite3PagerPageRefcount((*pNew).pDbPage)
                                                        != 1 as ::core::ffi::c_int
                                                            + (i == iParentIdx - nxDiv) as ::core::ffi::c_int
                                                        && rc == SQLITE_OK
                                                    {
                                                        rc = sqlite3CorruptError(8637 as ::core::ffi::c_int);
                                                    }
                                                    if rc != 0 {
                                                        c2rust_current_block = 11627155437346537937;
                                                        break;
                                                    }
                                                } else {
                                                    rc = allocateBtreePage(
                                                        pBt,
                                                        &raw mut pNew,
                                                        &raw mut pgno,
                                                        if bBulk != 0 { 1 as Pgno } else { pgno },
                                                        0 as u8_0,
                                                    );
                                                    if rc != 0 {
                                                        c2rust_current_block = 11627155437346537937;
                                                        break;
                                                    }
                                                    zeroPage(pNew, pageFlags);
                                                    apNew[i as usize] = pNew;
                                                    nNew += 1;
                                                    cntOld[i as usize] = b.nCell;
                                                    if (*pBt).autoVacuum != 0 {
                                                        ptrmapPut(
                                                            pBt,
                                                            (*pNew).pgno,
                                                            PTRMAP_BTREE as u8_0,
                                                            (*pParent).pgno,
                                                            &raw mut rc,
                                                        );
                                                        if rc != SQLITE_OK {
                                                            c2rust_current_block = 11627155437346537937;
                                                            break;
                                                        }
                                                    }
                                                }
                                                i += 1;
                                            }
                                            match c2rust_current_block {
                                                11627155437346537937 => {}
                                                _ => {
                                                    i = 0 as ::core::ffi::c_int;
                                                    while i < nNew {
                                                        aPgno[i as usize] = (*apNew[i as usize]).pgno;
                                                        i += 1;
                                                    }
                                                    i = 0 as ::core::ffi::c_int;
                                                    while i < nNew - 1 as ::core::ffi::c_int {
                                                        let mut iB: ::core::ffi::c_int = i;
                                                        j = i + 1 as ::core::ffi::c_int;
                                                        while j < nNew {
                                                            if (*apNew[j as usize]).pgno < (*apNew[iB as usize]).pgno {
                                                                iB = j;
                                                            }
                                                            j += 1;
                                                        }
                                                        if iB != i {
                                                            let mut pgnoA: Pgno = (*apNew[i as usize]).pgno;
                                                            let mut pgnoB: Pgno = (*apNew[iB as usize]).pgno;
                                                            let mut pgnoTemp: Pgno = (sqlite3PendingByte as Pgno)
                                                                .wrapping_div((*pBt).pageSize as Pgno)
                                                                .wrapping_add(1 as Pgno);
                                                            let mut fgA: u16_0 = (*(*apNew[i as usize]).pDbPage).flags;
                                                            let mut fgB: u16_0 = (*(*apNew[iB as usize]).pDbPage).flags;
                                                            sqlite3PagerRekey(
                                                                (*apNew[i as usize]).pDbPage,
                                                                pgnoTemp,
                                                                fgB,
                                                            );
                                                            sqlite3PagerRekey(
                                                                (*apNew[iB as usize]).pDbPage,
                                                                pgnoA,
                                                                fgA,
                                                            );
                                                            sqlite3PagerRekey((*apNew[i as usize]).pDbPage, pgnoB, fgB);
                                                            (*apNew[i as usize]).pgno = pgnoB;
                                                            (*apNew[iB as usize]).pgno = pgnoA;
                                                        }
                                                        i += 1;
                                                    }
                                                    sqlite3Put4byte(
                                                        pRight,
                                                        (*apNew[(nNew - 1 as ::core::ffi::c_int) as usize]).pgno
                                                            as u32_0,
                                                    );
                                                    if pageFlags & PTF_LEAF == 0 as ::core::ffi::c_int
                                                        && nOld != nNew
                                                    {
                                                        let mut pOld_0: *mut MemPage = ::core::ptr::null_mut::<
                                                            MemPage,
                                                        >();
                                                        if nNew > nOld {
                                                            pOld_0 = apNew[(nOld - 1 as ::core::ffi::c_int) as usize];
                                                        } else {
                                                            pOld_0 = apOld[(nOld - 1 as ::core::ffi::c_int) as usize];
                                                        }
                                                        memcpy(
                                                            (**(&raw mut apNew as *mut *mut MemPage)
                                                                .offset((nNew - 1 as ::core::ffi::c_int) as isize))
                                                                .aData
                                                                .offset(8 as ::core::ffi::c_int as isize) as *mut u8_0
                                                                as *mut ::core::ffi::c_void,
                                                            (*pOld_0).aData.offset(8 as ::core::ffi::c_int as isize)
                                                                as *mut u8_0 as *const ::core::ffi::c_void,
                                                            4 as size_t,
                                                        );
                                                    }
                                                    if (*pBt).autoVacuum != 0 {
                                                        let mut pOld_1: *mut MemPage = ::core::ptr::null_mut::<
                                                            MemPage,
                                                        >();
                                                        pOld_1 = apNew[0 as ::core::ffi::c_int as usize];
                                                        let mut pNew_0: *mut MemPage = pOld_1;
                                                        let mut cntOldNext: ::core::ffi::c_int = (*pNew_0).nCell
                                                            as ::core::ffi::c_int
                                                            + (*pNew_0).nOverflow as ::core::ffi::c_int;
                                                        let mut iNew: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                        let mut iOld: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                        i = 0 as ::core::ffi::c_int;
                                                        loop {
                                                            if !(i < b.nCell) {
                                                                c2rust_current_block = 14065265019409502504;
                                                                break;
                                                            }
                                                            let mut pCell: *mut u8_0 = *b.apCell.offset(i as isize);
                                                            while i == cntOldNext {
                                                                iOld += 1;
                                                                pOld_1 = if iOld < nNew {
                                                                    apNew[iOld as usize]
                                                                } else {
                                                                    apOld[iOld as usize]
                                                                };
                                                                cntOldNext
                                                                    += (*pOld_1).nCell as ::core::ffi::c_int
                                                                        + (*pOld_1).nOverflow as ::core::ffi::c_int
                                                                        + (leafData == 0) as ::core::ffi::c_int;
                                                            }
                                                            if i == cntNew[iNew as usize] {
                                                                iNew += 1;
                                                                pNew_0 = apNew[iNew as usize];
                                                                if leafData == 0 {
                                                                    c2rust_current_block = 12638391263490919476;
                                                                } else {
                                                                    c2rust_current_block = 18368972393688527475;
                                                                }
                                                            } else {
                                                                c2rust_current_block = 18368972393688527475;
                                                            }
                                                            match c2rust_current_block {
                                                                18368972393688527475 => {
                                                                    if iOld >= nNew || (*pNew_0).pgno != aPgno[iOld as usize]
                                                                        || !(pCell as uptr >= (*pOld_1).aData as uptr
                                                                            && (pCell as uptr) < (*pOld_1).aDataEnd as uptr)
                                                                    {
                                                                        if leafCorrection == 0 {
                                                                            ptrmapPut(
                                                                                pBt,
                                                                                sqlite3Get4byte(pCell) as Pgno,
                                                                                PTRMAP_BTREE as u8_0,
                                                                                (*pNew_0).pgno,
                                                                                &raw mut rc,
                                                                            );
                                                                        }
                                                                        if cachedCellSize(&raw mut b, i) as ::core::ffi::c_int
                                                                            > (*pNew_0).minLocal as ::core::ffi::c_int
                                                                        {
                                                                            ptrmapPutOvflPtr(pNew_0, pOld_1, pCell, &raw mut rc);
                                                                        }
                                                                        if rc != 0 {
                                                                            c2rust_current_block = 11627155437346537937;
                                                                            break;
                                                                        }
                                                                    }
                                                                }
                                                                _ => {}
                                                            }
                                                            i += 1;
                                                        }
                                                    } else {
                                                        c2rust_current_block = 14065265019409502504;
                                                    }
                                                    match c2rust_current_block {
                                                        11627155437346537937 => {}
                                                        _ => {
                                                            i = 0 as ::core::ffi::c_int;
                                                            loop {
                                                                if !(i < nNew - 1 as ::core::ffi::c_int) {
                                                                    c2rust_current_block = 14203403603292796089;
                                                                    break;
                                                                }
                                                                let mut pCell_0: *mut u8_0 = ::core::ptr::null_mut::<
                                                                    u8_0,
                                                                >();
                                                                let mut pTemp_0: *mut u8_0 = ::core::ptr::null_mut::<
                                                                    u8_0,
                                                                >();
                                                                let mut sz_1: ::core::ffi::c_int = 0;
                                                                let mut pSrcEnd: *mut u8_0 = ::core::ptr::null_mut::<
                                                                    u8_0,
                                                                >();
                                                                let mut pNew_1: *mut MemPage = apNew[i as usize];
                                                                j = cntNew[i as usize];
                                                                pCell_0 = *b.apCell.offset(j as isize);
                                                                sz_1 = *b.szCell.offset(j as isize) as ::core::ffi::c_int
                                                                    + leafCorrection as ::core::ffi::c_int;
                                                                pTemp_0 = aOvflSpace.offset(iOvflSpace as isize)
                                                                    as *mut u8_0;
                                                                if (*pNew_1).leaf == 0 {
                                                                    memcpy(
                                                                        (*pNew_1).aData.offset(8 as ::core::ffi::c_int as isize)
                                                                            as *mut u8_0 as *mut ::core::ffi::c_void,
                                                                        pCell_0 as *const ::core::ffi::c_void,
                                                                        4 as size_t,
                                                                    );
                                                                } else if leafData != 0 {
                                                                    let mut info: CellInfo = CellInfo {
                                                                        nKey: 0,
                                                                        pPayload: ::core::ptr::null_mut::<u8_0>(),
                                                                        nPayload: 0,
                                                                        nLocal: 0,
                                                                        nSize: 0

                                                                    };
                                                                    j -= 1;
                                                                    (*pNew_1)
                                                                        .xParseCell
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(pNew_1, *b.apCell.offset(j as isize), &raw mut info);
                                                                    pCell_0 = pTemp_0;
                                                                    sz_1 = 4 as ::core::ffi::c_int
                                                                        + sqlite3PutVarint(
                                                                            pCell_0.offset(4 as ::core::ffi::c_int as isize)
                                                                                as *mut ::core::ffi::c_uchar,
                                                                            info.nKey as u64_0,
                                                                        );
                                                                    pTemp_0 = ::core::ptr::null_mut::<u8_0>();
                                                                } else {
                                                                    pCell_0 = pCell_0
                                                                        .offset(-(4 as ::core::ffi::c_int as isize));
                                                                    if *b.szCell.offset(j as isize) as ::core::ffi::c_int
                                                                        == 4 as ::core::ffi::c_int
                                                                    {
                                                                        sz_1 = (*pParent)
                                                                            .xCellSize
                                                                            .expect("non-null function pointer")(pParent, pCell_0)
                                                                            as ::core::ffi::c_int;
                                                                    }
                                                                }
                                                                iOvflSpace += sz_1;
                                                                k = 0 as ::core::ffi::c_int;
                                                                while b.ixNx[k as usize] <= j {
                                                                    k += 1;
                                                                }
                                                                pSrcEnd = b.apEnd[k as usize];
                                                                if (pCell_0 as uptr) < pSrcEnd as uptr
                                                                    && pCell_0.offset(sz_1 as isize) as uptr > pSrcEnd as uptr
                                                                {
                                                                    rc = sqlite3CorruptError(8843 as ::core::ffi::c_int);
                                                                    c2rust_current_block = 11627155437346537937;
                                                                    break;
                                                                } else {
                                                                    rc = insertCell(
                                                                        pParent,
                                                                        nxDiv + i,
                                                                        pCell_0,
                                                                        sz_1,
                                                                        pTemp_0,
                                                                        (*pNew_1).pgno,
                                                                    );
                                                                    if rc != SQLITE_OK {
                                                                        c2rust_current_block = 11627155437346537937;
                                                                        break;
                                                                    }
                                                                    i += 1;
                                                                }
                                                            }
                                                            match c2rust_current_block {
                                                                11627155437346537937 => {}
                                                                _ => {
                                                                    i = 1 as ::core::ffi::c_int - nNew;
                                                                    loop {
                                                                        if !(i < nNew) {
                                                                            c2rust_current_block = 18302639990213641465;
                                                                            break;
                                                                        }
                                                                        let mut iPg: ::core::ffi::c_int = if i
                                                                            < 0 as ::core::ffi::c_int
                                                                        {
                                                                            -i
                                                                        } else {
                                                                            i
                                                                        };
                                                                        if !(abDone[iPg as usize] != 0) {
                                                                            if i >= 0 as ::core::ffi::c_int
                                                                                || cntOld[(iPg - 1 as ::core::ffi::c_int) as usize]
                                                                                    >= cntNew[(iPg - 1 as ::core::ffi::c_int) as usize]
                                                                            {
                                                                                let mut iNew_0: ::core::ffi::c_int = 0;
                                                                                let mut iOld_0: ::core::ffi::c_int = 0;
                                                                                let mut nNewCell: ::core::ffi::c_int = 0;
                                                                                if iPg == 0 as ::core::ffi::c_int {
                                                                                    iOld_0 = 0 as ::core::ffi::c_int;
                                                                                    iNew_0 = iOld_0;
                                                                                    nNewCell = cntNew[0 as ::core::ffi::c_int as usize];
                                                                                } else {
                                                                                    iOld_0 = if iPg < nOld {
                                                                                        cntOld[(iPg - 1 as ::core::ffi::c_int) as usize]
                                                                                            + (leafData == 0) as ::core::ffi::c_int
                                                                                    } else {
                                                                                        b.nCell
                                                                                    };
                                                                                    iNew_0 = cntNew[(iPg - 1 as ::core::ffi::c_int) as usize]
                                                                                        + (leafData == 0) as ::core::ffi::c_int;
                                                                                    nNewCell = cntNew[iPg as usize] - iNew_0;
                                                                                }
                                                                                rc = editPage(
                                                                                    apNew[iPg as usize],
                                                                                    iOld_0,
                                                                                    iNew_0,
                                                                                    nNewCell,
                                                                                    &raw mut b,
                                                                                );
                                                                                if rc != 0 {
                                                                                    c2rust_current_block = 11627155437346537937;
                                                                                    break;
                                                                                }
                                                                                abDone[iPg as usize] = abDone[iPg as usize].wrapping_add(1);
                                                                                (*apNew[iPg as usize]).nFree = usableSpace
                                                                                    - szNew[iPg as usize];
                                                                            }
                                                                        }
                                                                        i += 1;
                                                                    }
                                                                    match c2rust_current_block {
                                                                        11627155437346537937 => {}
                                                                        _ => {
                                                                            if isRoot != 0
                                                                                && (*pParent).nCell as ::core::ffi::c_int
                                                                                    == 0 as ::core::ffi::c_int
                                                                                && (*pParent).hdrOffset as ::core::ffi::c_int
                                                                                    <= (*apNew[0 as ::core::ffi::c_int as usize]).nFree
                                                                            {
                                                                                rc = defragmentPage(
                                                                                    apNew[0 as ::core::ffi::c_int as usize],
                                                                                    -1 as ::core::ffi::c_int,
                                                                                );
                                                                                copyNodeContent(
                                                                                    apNew[0 as ::core::ffi::c_int as usize],
                                                                                    pParent,
                                                                                    &raw mut rc,
                                                                                );
                                                                                freePage(
                                                                                    apNew[0 as ::core::ffi::c_int as usize],
                                                                                    &raw mut rc,
                                                                                );
                                                                            } else if (*pBt).autoVacuum as ::core::ffi::c_int != 0
                                                                                && leafCorrection == 0
                                                                            {
                                                                                i = 0 as ::core::ffi::c_int;
                                                                                while i < nNew {
                                                                                    let mut key: u32_0 = sqlite3Get4byte(
                                                                                        (**(&raw mut apNew as *mut *mut MemPage).offset(i as isize))
                                                                                            .aData
                                                                                            .offset(8 as ::core::ffi::c_int as isize) as *mut u8_0,
                                                                                    );
                                                                                    ptrmapPut(
                                                                                        pBt,
                                                                                        key as Pgno,
                                                                                        PTRMAP_BTREE as u8_0,
                                                                                        (*apNew[i as usize]).pgno,
                                                                                        &raw mut rc,
                                                                                    );
                                                                                    i += 1;
                                                                                }
                                                                            }
                                                                            i = nNew;
                                                                            while i < nOld {
                                                                                freePage(apOld[i as usize], &raw mut rc);
                                                                                i += 1;
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
                        }
                    }
                }
            }
            _ => {}
        }
        sqlite3DbFree(
            ::core::ptr::null_mut::<sqlite3>(),
            b.apCell as *mut ::core::ffi::c_void,
        );
        i = 0 as ::core::ffi::c_int;
        while i < nOld {
            releasePage(apOld[i as usize]);
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nNew {
            releasePage(apNew[i as usize]);
            i += 1;
        }
        return rc;
    }
}
unsafe extern "C" fn balance_deeper(
    mut pRoot: *mut MemPage,
    mut ppChild: *mut *mut MemPage,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pChild: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut pgnoChild: Pgno = 0 as Pgno;
        let mut pBt: *mut BtShared = (*pRoot).pBt;
        rc = sqlite3PagerWrite((*pRoot).pDbPage);
        if rc == SQLITE_OK {
            rc = allocateBtreePage(
                pBt,
                &raw mut pChild,
                &raw mut pgnoChild,
                (*pRoot).pgno,
                0 as u8_0,
            );
            copyNodeContent(pRoot, pChild, &raw mut rc);
            if (*pBt).autoVacuum != 0 {
                ptrmapPut(
                    pBt,
                    pgnoChild,
                    PTRMAP_BTREE as u8_0,
                    (*pRoot).pgno,
                    &raw mut rc,
                );
            }
        }
        if rc != 0 {
            *ppChild = ::core::ptr::null_mut::<MemPage>();
            releasePage(pChild);
            return rc;
        }
        memcpy(
            &raw mut (*pChild).aiOvfl as *mut u16_0 as *mut ::core::ffi::c_void,
            &raw mut (*pRoot).aiOvfl as *mut u16_0 as *const ::core::ffi::c_void,
            ((*pRoot).nOverflow as size_t)
                .wrapping_mul(::core::mem::size_of::<u16_0>() as size_t),
        );
        memcpy(
            &raw mut (*pChild).apOvfl as *mut *mut u8_0 as *mut ::core::ffi::c_void,
            &raw mut (*pRoot).apOvfl as *mut *mut u8_0 as *const ::core::ffi::c_void,
            ((*pRoot).nOverflow as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut u8_0>() as size_t),
        );
        (*pChild).nOverflow = (*pRoot).nOverflow;
        zeroPage(
            pRoot,
            *(*pChild).aData.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int & !PTF_LEAF,
        );
        sqlite3Put4byte(
            (*pRoot)
                .aData
                .offset(
                    ((*pRoot).hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                        as isize,
                ) as *mut u8_0,
            pgnoChild as u32_0,
        );
        *ppChild = pChild;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn anotherValidCursor(mut pCur: *mut BtCursor) -> ::core::ffi::c_int {
    unsafe {
        let mut pOther: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        pOther = (*(*pCur).pBt).pCursor;
        while !pOther.is_null() {
            if pOther != pCur && (*pOther).eState as ::core::ffi::c_int == CURSOR_VALID
                && (*pOther).pPage == (*pCur).pPage
            {
                return sqlite3CorruptError(9075 as ::core::ffi::c_int);
            }
            pOther = (*pOther).pNext;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn balance(mut pCur: *mut BtCursor) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut aBalanceQuickSpace: [u8_0; 13] = [0; 13];
        let mut pFree: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        loop {
            let mut iPage: ::core::ffi::c_int = 0;
            let mut pPage: *mut MemPage = (*pCur).pPage;
            if (*pPage).nFree < 0 as ::core::ffi::c_int
                && btreeComputeFreeSpace(pPage) != 0
            {
                break;
            }
            if (*pPage).nOverflow as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (*pPage).nFree * 3 as ::core::ffi::c_int
                    <= (*(*pCur).pBt).usableSize as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
            {
                break;
            }
            iPage = (*pCur).iPage as ::core::ffi::c_int;
            if iPage == 0 as ::core::ffi::c_int {
                if !((*pPage).nOverflow as ::core::ffi::c_int != 0
                    && {
                        rc = anotherValidCursor(pCur);
                        rc == SQLITE_OK
                    })
                {
                    break;
                }
                rc = balance_deeper(
                    pPage,
                    (&raw mut (*pCur).apPage as *mut *mut MemPage)
                        .offset(1 as ::core::ffi::c_int as isize) as *mut *mut MemPage,
                );
                if rc == SQLITE_OK {
                    (*pCur).iPage = 1 as i8_0;
                    (*pCur).ix = 0 as u16_0;
                    (*pCur).aiIdx[0 as ::core::ffi::c_int as usize] = 0 as u16_0;
                    (*pCur).apPage[0 as ::core::ffi::c_int as usize] = pPage;
                    (*pCur).pPage = (*pCur).apPage[1 as ::core::ffi::c_int as usize];
                }
            } else if sqlite3PagerPageRefcount((*pPage).pDbPage)
                > 1 as ::core::ffi::c_int
            {
                rc = sqlite3CorruptError(9135 as ::core::ffi::c_int);
            } else {
                let pParent: *mut MemPage = (*pCur)
                    .apPage[(iPage - 1 as ::core::ffi::c_int) as usize];
                let iIdx: ::core::ffi::c_int = (*pCur)
                    .aiIdx[(iPage - 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                rc = sqlite3PagerWrite((*pParent).pDbPage);
                if rc == SQLITE_OK && (*pParent).nFree < 0 as ::core::ffi::c_int {
                    rc = btreeComputeFreeSpace(pParent);
                }
                if rc == SQLITE_OK {
                    if (*pPage).intKeyLeaf as ::core::ffi::c_int != 0
                        && (*pPage).nOverflow as ::core::ffi::c_int
                            == 1 as ::core::ffi::c_int
                        && (*pPage).aiOvfl[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int == (*pPage).nCell as ::core::ffi::c_int
                        && (*pParent).pgno != 1 as ::core::ffi::c_uint
                        && (*pParent).nCell as ::core::ffi::c_int == iIdx
                    {
                        rc = balance_quick(
                            pParent,
                            pPage,
                            &raw mut aBalanceQuickSpace as *mut u8_0,
                        );
                    } else {
                        let mut pSpace: *mut u8_0 = sqlite3PageMalloc(
                            (*(*pCur).pBt).pageSize as ::core::ffi::c_int,
                        ) as *mut u8_0;
                        rc = balance_nonroot(
                            pParent,
                            iIdx,
                            pSpace,
                            (iPage == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                            (*pCur).hints as ::core::ffi::c_int & BTREE_BULKLOAD,
                        );
                        if !pFree.is_null() {
                            sqlite3PageFree(pFree as *mut ::core::ffi::c_void);
                        }
                        pFree = pSpace;
                    }
                }
                (*pPage).nOverflow = 0 as u8_0;
                releasePage(pPage);
                (*pCur).iPage -= 1;
                (*pCur).pPage = (*pCur).apPage[(*pCur).iPage as usize];
            }
            if !(rc == SQLITE_OK) {
                break;
            }
        }
        if !pFree.is_null() {
            sqlite3PageFree(pFree as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn btreeOverwriteContent(
    mut pPage: *mut MemPage,
    mut pDest: *mut u8_0,
    mut pX: *const BtreePayload,
    mut iOffset: ::core::ffi::c_int,
    mut iAmt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nData: ::core::ffi::c_int = (*pX).nData - iOffset;
        if nData <= 0 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < iAmt
                && *pDest.offset(i as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                i += 1;
            }
            if i < iAmt {
                let mut rc: ::core::ffi::c_int = sqlite3PagerWrite((*pPage).pDbPage);
                if rc != 0 {
                    return rc;
                }
                memset(
                    pDest.offset(i as isize) as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (iAmt - i) as size_t,
                );
            }
        } else {
            if nData < iAmt {
                let mut rc_0: ::core::ffi::c_int = btreeOverwriteContent(
                    pPage,
                    pDest.offset(nData as isize),
                    pX,
                    iOffset + nData,
                    iAmt - nData,
                );
                if rc_0 != 0 {
                    return rc_0;
                }
                iAmt = nData;
            }
            if memcmp(
                pDest as *const ::core::ffi::c_void,
                ((*pX).pData as *mut u8_0).offset(iOffset as isize)
                    as *const ::core::ffi::c_void,
                iAmt as size_t,
            ) != 0 as ::core::ffi::c_int
            {
                let mut rc_1: ::core::ffi::c_int = sqlite3PagerWrite((*pPage).pDbPage);
                if rc_1 != 0 {
                    return rc_1;
                }
                memmove(
                    pDest as *mut ::core::ffi::c_void,
                    ((*pX).pData as *mut u8_0).offset(iOffset as isize)
                        as *const ::core::ffi::c_void,
                    iAmt as size_t,
                );
            }
        }
        return SQLITE_OK;
    }
}
#[inline(never)]
unsafe extern "C" fn btreeOverwriteOverflowCell(
    mut pCur: *mut BtCursor,
    mut pX: *const BtreePayload,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iOffset: ::core::ffi::c_int = 0;
        let mut nTotal: ::core::ffi::c_int = (*pX).nData + (*pX).nZero;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pPage: *mut MemPage = (*pCur).pPage;
        let mut pBt: *mut BtShared = ::core::ptr::null_mut::<BtShared>();
        let mut ovflPgno: Pgno = 0;
        let mut ovflPageSize: u32_0 = 0;
        rc = btreeOverwriteContent(
            pPage,
            (*pCur).info.pPayload,
            pX,
            0 as ::core::ffi::c_int,
            (*pCur).info.nLocal as ::core::ffi::c_int,
        );
        if rc != 0 {
            return rc;
        }
        iOffset = (*pCur).info.nLocal as ::core::ffi::c_int;
        ovflPgno = sqlite3Get4byte((*pCur).info.pPayload.offset(iOffset as isize))
            as Pgno;
        pBt = (*pPage).pBt;
        ovflPageSize = ((*pBt).usableSize as ::core::ffi::c_uint)
            .wrapping_sub(4 as ::core::ffi::c_uint) as u32_0;
        loop {
            rc = btreeGetPage(pBt, ovflPgno, &raw mut pPage, 0 as ::core::ffi::c_int);
            if rc != 0 {
                return rc;
            }
            if sqlite3PagerPageRefcount((*pPage).pDbPage) != 1 as ::core::ffi::c_int
                || (*pPage).isInit as ::core::ffi::c_int != 0
            {
                rc = sqlite3CorruptError(9299 as ::core::ffi::c_int);
            } else {
                if (iOffset as u32_0).wrapping_add(ovflPageSize) < nTotal as u32_0 {
                    ovflPgno = sqlite3Get4byte((*pPage).aData) as Pgno;
                } else {
                    ovflPageSize = (nTotal - iOffset) as u32_0;
                }
                rc = btreeOverwriteContent(
                    pPage,
                    (*pPage).aData.offset(4 as ::core::ffi::c_int as isize),
                    pX,
                    iOffset,
                    ovflPageSize as ::core::ffi::c_int,
                );
            }
            sqlite3PagerUnref((*pPage).pDbPage);
            if rc != 0 {
                return rc;
            }
            iOffset = (iOffset as ::core::ffi::c_uint)
                .wrapping_add(ovflPageSize as ::core::ffi::c_uint) as ::core::ffi::c_int
                as ::core::ffi::c_int;
            if !(iOffset < nTotal) {
                break;
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn btreeOverwriteCell(
    mut pCur: *mut BtCursor,
    mut pX: *const BtreePayload,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nTotal: ::core::ffi::c_int = (*pX).nData + (*pX).nZero;
        let mut pPage: *mut MemPage = (*pCur).pPage;
        if (*pCur)
            .info
            .pPayload
            .offset((*pCur).info.nLocal as ::core::ffi::c_int as isize)
            > (*pPage).aDataEnd
            || (*pCur).info.pPayload
                < (*pPage)
                    .aData
                    .offset((*pPage).cellOffset as ::core::ffi::c_int as isize)
        {
            return sqlite3CorruptError(9327 as ::core::ffi::c_int);
        }
        if (*pCur).info.nLocal as ::core::ffi::c_int == nTotal {
            return btreeOverwriteContent(
                pPage,
                (*pCur).info.pPayload,
                pX,
                0 as ::core::ffi::c_int,
                (*pCur).info.nLocal as ::core::ffi::c_int,
            )
        } else {
            return btreeOverwriteOverflowCell(pCur, pX)
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeInsert(
    mut pCur: *mut BtCursor,
    mut pX: *const BtreePayload,
    mut flags: ::core::ffi::c_int,
    mut seekResult: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = 0;
        let mut loc: ::core::ffi::c_int = seekResult;
        let mut szNew: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut idx: ::core::ffi::c_int = 0;
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut p: *mut Btree = (*pCur).pBtree;
        let mut oldCell: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut newCell: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        if (*pCur).curFlags as ::core::ffi::c_int & BTCF_Multiple != 0 {
            rc = saveAllCursors((*p).pBt, (*pCur).pgnoRoot, pCur);
            if rc != 0 {
                return rc;
            }
            if loc != 0
                && ((*pCur).iPage as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            {
                return sqlite3CorruptError(9408 as ::core::ffi::c_int);
            }
        }
        if (*pCur).eState as ::core::ffi::c_int >= CURSOR_REQUIRESEEK {
            rc = moveToRoot(pCur);
            if rc != 0 && rc != SQLITE_EMPTY {
                return rc;
            }
        }
        if (*pCur).pKeyInfo.is_null() {
            if (*p).hasIncrblobCur != 0 {
                invalidateIncrblobCursors(
                    p,
                    (*pCur).pgnoRoot,
                    (*pX).nKey as i64_0,
                    0 as ::core::ffi::c_int,
                );
            }
            if (*pCur).curFlags as ::core::ffi::c_int & BTCF_ValidNKey
                != 0 as ::core::ffi::c_int && (*pX).nKey == (*pCur).info.nKey
            {
                if (*pCur).info.nSize as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    && (*pCur).info.nPayload
                        == ((*pX).nData as ::core::ffi::c_uint)
                            .wrapping_add((*pX).nZero as ::core::ffi::c_uint)
                {
                    return btreeOverwriteCell(pCur, pX);
                }
            } else if loc == 0 as ::core::ffi::c_int {
                rc = sqlite3BtreeTableMoveto(
                    pCur,
                    (*pX).nKey as i64_0,
                    (flags & BTREE_APPEND != 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int,
                    &raw mut loc,
                );
                if rc != 0 {
                    return rc;
                }
            }
        } else {
            if loc == 0 as ::core::ffi::c_int
                && flags & BTREE_SAVEPOSITION == 0 as ::core::ffi::c_int
            {
                if (*pX).nMem != 0 {
                    let mut r: UnpackedRecord = UnpackedRecord {
                        pKeyInfo: ::core::ptr::null_mut::<KeyInfo>(),
                        aMem: ::core::ptr::null_mut::<Mem>(),
                        u: C2Rust_Unnamed_22 {
                            z: ::core::ptr::null_mut::<::core::ffi::c_char>()

                        },
                        n: 0,
                        nField: 0,
                        default_rc: 0,
                        errCode: 0,
                        r1: 0,
                        r2: 0,
                        eqSeen: 0

                    };
                    r.pKeyInfo = (*pCur).pKeyInfo as *mut KeyInfo;
                    r.aMem = (*pX).aMem as *mut Mem;
                    r.nField = (*pX).nMem;
                    r.default_rc = 0 as i8_0;
                    r.eqSeen = 0 as u8_0;
                    rc = sqlite3BtreeIndexMoveto(pCur, &raw mut r, &raw mut loc);
                } else {
                    rc = btreeMoveto(
                        pCur,
                        (*pX).pKey,
                        (*pX).nKey as i64_0,
                        (flags & BTREE_APPEND != 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int,
                        &raw mut loc,
                    );
                }
                if rc != 0 {
                    return rc;
                }
            }
            if loc == 0 as ::core::ffi::c_int {
                getCellInfo(pCur);
                if (*pCur).info.nKey == (*pX).nKey {
                    let mut x2: BtreePayload = BtreePayload {
                        pKey: ::core::ptr::null::<::core::ffi::c_void>(),
                        nKey: 0,
                        pData: ::core::ptr::null::<::core::ffi::c_void>(),
                        aMem: ::core::ptr::null_mut::<sqlite3_value>(),
                        nMem: 0,
                        nData: 0,
                        nZero: 0

                    };
                    x2.pData = (*pX).pKey;
                    x2.nData = (*pX).nKey as ::core::ffi::c_int;
                    x2.nZero = 0 as ::core::ffi::c_int;
                    return btreeOverwriteCell(pCur, &raw mut x2);
                }
            }
        }
        pPage = (*pCur).pPage;
        if (*pPage).nFree < 0 as ::core::ffi::c_int {
            if (*pCur).eState as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                rc = sqlite3CorruptError(9531 as ::core::ffi::c_int);
            } else {
                rc = btreeComputeFreeSpace(pPage);
            }
            if rc != 0 {
                return rc;
            }
        }
        newCell = (*(*p).pBt).pTmpSpace as *mut ::core::ffi::c_uchar;
        if flags & BTREE_PREFORMAT != 0 {
            rc = SQLITE_OK;
            szNew = (*(*p).pBt).nPreformatSize;
            if szNew < 4 as ::core::ffi::c_int {
                szNew = 4 as ::core::ffi::c_int;
                *newCell.offset(3 as ::core::ffi::c_int as isize) = 0
                    as ::core::ffi::c_uchar;
            }
            if (*(*p).pBt).autoVacuum as ::core::ffi::c_int != 0
                && szNew > (*pPage).maxLocal as ::core::ffi::c_int
            {
                let mut info: CellInfo = CellInfo {
                    nKey: 0,
                    pPayload: ::core::ptr::null_mut::<u8_0>(),
                    nPayload: 0,
                    nLocal: 0,
                    nSize: 0

                };
                (*pPage)
                    .xParseCell
                    .expect(
                        "non-null function pointer",
                    )(pPage, newCell as *mut u8_0, &raw mut info);
                if info.nPayload != info.nLocal as ::core::ffi::c_uint {
                    let mut ovfl: Pgno = sqlite3Get4byte(
                        newCell.offset((szNew - 4 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_uchar,
                    ) as Pgno;
                    ptrmapPut(
                        (*p).pBt,
                        ovfl,
                        PTRMAP_OVERFLOW1 as u8_0,
                        (*pPage).pgno,
                        &raw mut rc,
                    );
                    if rc != 0 {
                        c2rust_current_block = 16476170483193316560;
                    } else {
                        c2rust_current_block = 479107131381816815;
                    }
                } else {
                    c2rust_current_block = 479107131381816815;
                }
            } else {
                c2rust_current_block = 479107131381816815;
            }
        } else {
            rc = fillInCell(pPage, newCell, pX, &raw mut szNew);
            if rc != 0 {
                c2rust_current_block = 16476170483193316560;
            } else {
                c2rust_current_block = 479107131381816815;
            }
        }
        match c2rust_current_block {
            479107131381816815 => {
                idx = (*pCur).ix as ::core::ffi::c_int;
                (*pCur).info.nSize = 0 as u16_0;
                if loc == 0 as ::core::ffi::c_int {
                    let mut info_0: CellInfo = CellInfo {
                        nKey: 0,
                        pPayload: ::core::ptr::null_mut::<u8_0>(),
                        nPayload: 0,
                        nLocal: 0,
                        nSize: 0

                    };
                    if idx >= (*pPage).nCell as ::core::ffi::c_int {
                        return sqlite3CorruptError(9573 as ::core::ffi::c_int);
                    }
                    rc = sqlite3PagerWrite((*pPage).pDbPage);
                    if rc != 0 {
                        c2rust_current_block = 16476170483193316560;
                    } else {
                        oldCell = (*pPage)
                            .aData
                            .offset(
                                ((*pPage).maskPage as ::core::ffi::c_int
                                    & ((*((*pPage)
                                        .aCellIdx
                                        .offset((2 as ::core::ffi::c_int * idx) as isize)
                                        as *mut u8_0)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                        | *((*pPage)
                                            .aCellIdx
                                            .offset((2 as ::core::ffi::c_int * idx) as isize)
                                            as *mut u8_0)
                                            .offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int)) as isize,
                            ) as *mut ::core::ffi::c_uchar;
                        if (*pPage).leaf == 0 {
                            memcpy(
                                newCell as *mut ::core::ffi::c_void,
                                oldCell as *const ::core::ffi::c_void,
                                4 as size_t,
                            );
                        }
                        (*pPage)
                            .xParseCell
                            .expect(
                                "non-null function pointer",
                            )(pPage, oldCell as *mut u8_0, &raw mut info_0);
                        if info_0.nLocal as ::core::ffi::c_uint != info_0.nPayload {
                            rc = clearCellOverflow(pPage, oldCell, &raw mut info_0);
                        } else {
                            rc = SQLITE_OK;
                        }
                        (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
                            & !BTCF_ValidOvfl) as u8_0;
                        if info_0.nSize as ::core::ffi::c_int == szNew
                            && info_0.nLocal as ::core::ffi::c_uint == info_0.nPayload
                            && ((*(*p).pBt).autoVacuum == 0
                                || szNew < (*pPage).minLocal as ::core::ffi::c_int)
                        {
                            if oldCell
                                < (*pPage)
                                    .aData
                                    .offset((*pPage).hdrOffset as ::core::ffi::c_int as isize)
                                    .offset(10 as ::core::ffi::c_int as isize)
                            {
                                return sqlite3CorruptError(9600 as ::core::ffi::c_int);
                            }
                            if oldCell.offset(szNew as isize) > (*pPage).aDataEnd {
                                return sqlite3CorruptError(9603 as ::core::ffi::c_int);
                            }
                            memcpy(
                                oldCell as *mut ::core::ffi::c_void,
                                newCell as *const ::core::ffi::c_void,
                                szNew as size_t,
                            );
                            return SQLITE_OK;
                        }
                        dropCell(
                            pPage,
                            idx,
                            info_0.nSize as ::core::ffi::c_int,
                            &raw mut rc,
                        );
                        if rc != 0 {
                            c2rust_current_block = 16476170483193316560;
                        } else {
                            c2rust_current_block = 12065775993741208975;
                        }
                    }
                } else {
                    if loc < 0 as ::core::ffi::c_int
                        && (*pPage).nCell as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    {
                        (*pCur).ix = (*pCur).ix.wrapping_add(1);
                        idx = (*pCur).ix as ::core::ffi::c_int;
                        (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
                            & !(BTCF_ValidNKey | BTCF_ValidOvfl)) as u8_0;
                    }
                    c2rust_current_block = 12065775993741208975;
                }
                match c2rust_current_block {
                    16476170483193316560 => {}
                    _ => {
                        rc = insertCellFast(pPage, idx, newCell as *mut u8_0, szNew);
                        if (*pPage).nOverflow != 0 {
                            (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
                                & !(BTCF_ValidNKey | BTCF_ValidOvfl)) as u8_0;
                            rc = balance(pCur);
                            (*(*pCur).pPage).nOverflow = 0 as u8_0;
                            (*pCur).eState = CURSOR_INVALID as u8_0;
                            if flags & BTREE_SAVEPOSITION != 0 && rc == SQLITE_OK {
                                btreeReleaseAllCursorPages(pCur);
                                if !(*pCur).pKeyInfo.is_null() {
                                    (*pCur).pKey = sqlite3Malloc((*pX).nKey as u64_0);
                                    if (*pCur).pKey.is_null() {
                                        rc = SQLITE_NOMEM;
                                    } else {
                                        memcpy((*pCur).pKey, (*pX).pKey, (*pX).nKey as size_t);
                                    }
                                }
                                (*pCur).eState = CURSOR_REQUIRESEEK as u8_0;
                                (*pCur).nKey = (*pX).nKey as i64_0;
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
pub unsafe extern "C" fn sqlite3BtreeTransferRow(
    mut pDest: *mut BtCursor,
    mut pSrc: *mut BtCursor,
    mut iKey: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = (*pDest).pBt;
        let mut aOut: *mut u8_0 = (*pBt).pTmpSpace;
        let mut aIn: *const u8_0 = ::core::ptr::null::<u8_0>();
        let mut nIn: u32_0 = 0;
        let mut nRem: u32_0 = 0;
        getCellInfo(pSrc);
        if (*pSrc).info.nPayload < 0x80 as ::core::ffi::c_uint {
            let c2rust_fresh34 = aOut;
            aOut = aOut.offset(1);
            *c2rust_fresh34 = (*pSrc).info.nPayload as u8_0;
        } else {
            aOut = aOut
                .offset(
                    sqlite3PutVarint(
                        aOut as *mut ::core::ffi::c_uchar,
                        (*pSrc).info.nPayload as u64_0,
                    ) as isize,
                );
        }
        if (*pDest).pKeyInfo.is_null() {
            aOut = aOut
                .offset(
                    sqlite3PutVarint(aOut as *mut ::core::ffi::c_uchar, iKey as u64_0)
                        as isize,
                );
        }
        nIn = (*pSrc).info.nLocal as u32_0;
        aIn = (*pSrc).info.pPayload;
        if aIn.offset(nIn as isize) > (*(*pSrc).pPage).aDataEnd as *const u8_0 {
            return sqlite3CorruptError(9705 as ::core::ffi::c_int);
        }
        nRem = (*pSrc).info.nPayload;
        if nIn == nRem && nIn < (*(*pDest).pPage).maxLocal as ::core::ffi::c_uint {
            memcpy(
                aOut as *mut ::core::ffi::c_void,
                aIn as *const ::core::ffi::c_void,
                nIn as size_t,
            );
            (*pBt).nPreformatSize = (nIn as ::core::ffi::c_uint)
                .wrapping_add(
                    aOut.offset_from((*pBt).pTmpSpace) as ::core::ffi::c_long
                        as ::core::ffi::c_int as ::core::ffi::c_uint,
                ) as ::core::ffi::c_int;
            return SQLITE_OK;
        } else {
            let mut rc: ::core::ffi::c_int = SQLITE_OK;
            let mut pSrcPager: *mut Pager = (*(*pSrc).pBt).pPager;
            let mut pPgnoOut: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
            let mut ovflIn: Pgno = 0 as Pgno;
            let mut pPageIn: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
            let mut pPageOut: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
            let mut nOut: u32_0 = 0;
            nOut = btreePayloadToLocal((*pDest).pPage, (*pSrc).info.nPayload as i64_0)
                as u32_0;
            (*pBt).nPreformatSize = nOut as ::core::ffi::c_int
                + aOut.offset_from((*pBt).pTmpSpace) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
            if nOut < (*pSrc).info.nPayload {
                pPgnoOut = aOut.offset(nOut as isize) as *mut u8_0;
                (*pBt).nPreformatSize += 4 as ::core::ffi::c_int;
            }
            if nRem > nIn {
                if aIn.offset(nIn as isize).offset(4 as ::core::ffi::c_int as isize)
                    > (*(*pSrc).pPage).aDataEnd as *const u8_0
                {
                    return sqlite3CorruptError(9730 as ::core::ffi::c_int);
                }
                ovflIn = sqlite3Get4byte(
                    (*pSrc).info.pPayload.offset(nIn as isize) as *mut u8_0,
                ) as Pgno;
            }
            loop {
                nRem = (nRem as ::core::ffi::c_uint)
                    .wrapping_sub(nOut as ::core::ffi::c_uint) as u32_0 as u32_0;
                loop {
                    if nIn > 0 as ::core::ffi::c_uint {
                        let mut nCopy: ::core::ffi::c_int = (if nOut < nIn {
                            nOut as ::core::ffi::c_uint
                        } else {
                            nIn as ::core::ffi::c_uint
                        }) as ::core::ffi::c_int;
                        memcpy(
                            aOut as *mut ::core::ffi::c_void,
                            aIn as *const ::core::ffi::c_void,
                            nCopy as size_t,
                        );
                        nOut = (nOut as ::core::ffi::c_uint)
                            .wrapping_sub(nCopy as ::core::ffi::c_uint) as u32_0
                            as u32_0;
                        nIn = (nIn as ::core::ffi::c_uint)
                            .wrapping_sub(nCopy as ::core::ffi::c_uint) as u32_0
                            as u32_0;
                        aOut = aOut.offset(nCopy as isize);
                        aIn = aIn.offset(nCopy as isize);
                    }
                    if nOut > 0 as ::core::ffi::c_uint {
                        sqlite3PagerUnref(pPageIn);
                        pPageIn = ::core::ptr::null_mut::<DbPage>();
                        rc = sqlite3PagerGet(
                            pSrcPager,
                            ovflIn,
                            &raw mut pPageIn,
                            PAGER_GET_READONLY,
                        );
                        if rc == SQLITE_OK {
                            aIn = sqlite3PagerGetData(pPageIn) as *const u8_0;
                            ovflIn = sqlite3Get4byte(aIn) as Pgno;
                            aIn = aIn.offset(4 as ::core::ffi::c_int as isize);
                            nIn = ((*(*pSrc).pBt).usableSize as ::core::ffi::c_uint)
                                .wrapping_sub(4 as ::core::ffi::c_uint) as u32_0;
                        }
                    }
                    if !(rc == SQLITE_OK && nOut > 0 as ::core::ffi::c_uint) {
                        break;
                    }
                }
                if rc == SQLITE_OK && nRem > 0 as ::core::ffi::c_uint
                    && !pPgnoOut.is_null()
                {
                    let mut pgnoNew: Pgno = 0;
                    let mut pNew: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
                    rc = allocateBtreePage(
                        pBt,
                        &raw mut pNew,
                        &raw mut pgnoNew,
                        0 as Pgno,
                        0 as u8_0,
                    );
                    sqlite3Put4byte(pPgnoOut, pgnoNew as u32_0);
                    if (*pBt).autoVacuum as ::core::ffi::c_int != 0
                        && !pPageOut.is_null()
                    {
                        ptrmapPut(
                            pBt,
                            pgnoNew,
                            PTRMAP_OVERFLOW2 as u8_0,
                            (*pPageOut).pgno,
                            &raw mut rc,
                        );
                    }
                    releasePage(pPageOut);
                    pPageOut = pNew;
                    if !pPageOut.is_null() {
                        pPgnoOut = (*pPageOut).aData;
                        sqlite3Put4byte(pPgnoOut, 0 as u32_0);
                        aOut = pPgnoOut.offset(4 as ::core::ffi::c_int as isize)
                            as *mut u8_0;
                        nOut = (if ((*pBt).usableSize as ::core::ffi::c_uint)
                            .wrapping_sub(4 as ::core::ffi::c_uint) < nRem
                        {
                            ((*pBt).usableSize as ::core::ffi::c_uint)
                                .wrapping_sub(4 as ::core::ffi::c_uint)
                        } else {
                            nRem as ::core::ffi::c_uint
                        }) as u32_0;
                    }
                }
                if !(nRem > 0 as ::core::ffi::c_uint && rc == SQLITE_OK) {
                    break;
                }
            }
            releasePage(pPageOut);
            sqlite3PagerUnref(pPageIn);
            return rc;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeDelete(
    mut pCur: *mut BtCursor,
    mut flags: u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Btree = (*pCur).pBtree;
        let mut pBt: *mut BtShared = (*p).pBt;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut pCell: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut iCellIdx: ::core::ffi::c_int = 0;
        let mut iCellDepth: ::core::ffi::c_int = 0;
        let mut info: CellInfo = CellInfo {
            nKey: 0,
            pPayload: ::core::ptr::null_mut::<u8_0>(),
            nPayload: 0,
            nLocal: 0,
            nSize: 0

        };
        let mut bPreserve: u8_0 = 0;
        if (*pCur).eState as ::core::ffi::c_int != CURSOR_VALID {
            if (*pCur).eState as ::core::ffi::c_int >= CURSOR_REQUIRESEEK {
                rc = btreeRestoreCursorPosition(pCur);
                if rc != 0 || (*pCur).eState as ::core::ffi::c_int != CURSOR_VALID {
                    return rc;
                }
            } else {
                return sqlite3CorruptError(9826 as ::core::ffi::c_int)
            }
        }
        iCellDepth = (*pCur).iPage as ::core::ffi::c_int;
        iCellIdx = (*pCur).ix as ::core::ffi::c_int;
        pPage = (*pCur).pPage;
        if (*pPage).nCell as ::core::ffi::c_int <= iCellIdx {
            return sqlite3CorruptError(9835 as ::core::ffi::c_int);
        }
        pCell = (*pPage)
            .aData
            .offset(
                ((*pPage).maskPage as ::core::ffi::c_int
                    & ((*((*pPage)
                        .aCellIdx
                        .offset((2 as ::core::ffi::c_int * iCellIdx) as isize)
                        as *mut u8_0)
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *((*pPage)
                            .aCellIdx
                            .offset((2 as ::core::ffi::c_int * iCellIdx) as isize)
                            as *mut u8_0)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int)) as isize,
            ) as *mut ::core::ffi::c_uchar;
        if (*pPage).nFree < 0 as ::core::ffi::c_int && btreeComputeFreeSpace(pPage) != 0
        {
            return sqlite3CorruptError(9839 as ::core::ffi::c_int);
        }
        if pCell < (*pPage).aCellIdx.offset((*pPage).nCell as isize) as *mut u8_0 {
            return sqlite3CorruptError(9842 as ::core::ffi::c_int);
        }
        bPreserve = (flags as ::core::ffi::c_int & BTREE_SAVEPOSITION
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as u8_0;
        if bPreserve != 0 {
            if (*pPage).leaf == 0
                || (*pPage).nFree
                    + (*pPage)
                        .xCellSize
                        .expect("non-null function pointer")(pPage, pCell as *mut u8_0)
                        as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                    > ((*pBt).usableSize as ::core::ffi::c_uint)
                        .wrapping_mul(2 as ::core::ffi::c_uint)
                        .wrapping_div(3 as ::core::ffi::c_uint) as ::core::ffi::c_int
                || (*pPage).nCell as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            {
                rc = saveCursorKey(pCur);
                if rc != 0 {
                    return rc;
                }
            } else {
                bPreserve = 2 as u8_0;
            }
        }
        if (*pPage).leaf == 0 {
            rc = sqlite3BtreePrevious(pCur, 0 as ::core::ffi::c_int);
            if rc != 0 {
                return rc;
            }
        }
        if (*pCur).curFlags as ::core::ffi::c_int & BTCF_Multiple != 0 {
            rc = saveAllCursors(pBt, (*pCur).pgnoRoot, pCur);
            if rc != 0 {
                return rc;
            }
        }
        if (*pCur).pKeyInfo.is_null() && (*p).hasIncrblobCur as ::core::ffi::c_int != 0 {
            invalidateIncrblobCursors(
                p,
                (*pCur).pgnoRoot,
                (*pCur).info.nKey,
                0 as ::core::ffi::c_int,
            );
        }
        rc = sqlite3PagerWrite((*pPage).pDbPage);
        if rc != 0 {
            return rc;
        }
        (*pPage)
            .xParseCell
            .expect(
                "non-null function pointer",
            )(pPage, pCell as *mut u8_0, &raw mut info);
        if info.nLocal as ::core::ffi::c_uint != info.nPayload {
            rc = clearCellOverflow(pPage, pCell, &raw mut info);
        } else {
            rc = SQLITE_OK;
        }
        dropCell(pPage, iCellIdx, info.nSize as ::core::ffi::c_int, &raw mut rc);
        if rc != 0 {
            return rc;
        }
        if (*pPage).leaf == 0 {
            let mut pLeaf: *mut MemPage = (*pCur).pPage;
            let mut nCell: ::core::ffi::c_int = 0;
            let mut n: Pgno = 0;
            let mut pTmp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
                ::core::ffi::c_uchar,
            >();
            if (*pLeaf).nFree < 0 as ::core::ffi::c_int {
                rc = btreeComputeFreeSpace(pLeaf);
                if rc != 0 {
                    return rc;
                }
            }
            if iCellDepth < (*pCur).iPage as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            {
                n = (*(*pCur).apPage[(iCellDepth + 1 as ::core::ffi::c_int) as usize])
                    .pgno;
            } else {
                n = (*(*pCur).pPage).pgno;
            }
            pCell = (*pLeaf)
                .aData
                .offset(
                    ((*pLeaf).maskPage as ::core::ffi::c_int
                        & ((*((*pLeaf)
                            .aCellIdx
                            .offset(
                                (2 as ::core::ffi::c_int
                                    * ((*pLeaf).nCell as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)) as isize,
                            ) as *mut u8_0)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                            | *((*pLeaf)
                                .aCellIdx
                                .offset(
                                    (2 as ::core::ffi::c_int
                                        * ((*pLeaf).nCell as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int)) as isize,
                                ) as *mut u8_0)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)) as isize,
                ) as *mut ::core::ffi::c_uchar;
            if pCell
                < (*pLeaf).aData.offset(4 as ::core::ffi::c_int as isize) as *mut u8_0
            {
                return sqlite3CorruptError(9933 as ::core::ffi::c_int);
            }
            nCell = (*pLeaf)
                .xCellSize
                .expect("non-null function pointer")(pLeaf, pCell as *mut u8_0)
                as ::core::ffi::c_int;
            pTmp = (*pBt).pTmpSpace as *mut ::core::ffi::c_uchar;
            rc = sqlite3PagerWrite((*pLeaf).pDbPage);
            if rc == SQLITE_OK {
                rc = insertCell(
                    pPage,
                    iCellIdx,
                    pCell.offset(-(4 as ::core::ffi::c_int as isize)),
                    nCell + 4 as ::core::ffi::c_int,
                    pTmp as *mut u8_0,
                    n,
                );
            }
            dropCell(
                pLeaf,
                (*pLeaf).nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                nCell,
                &raw mut rc,
            );
            if rc != 0 {
                return rc;
            }
        }
        if (*(*pCur).pPage).nFree * 3 as ::core::ffi::c_int
            <= (*(*pCur).pBt).usableSize as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        {
            rc = SQLITE_OK;
        } else {
            rc = balance(pCur);
        }
        if rc == SQLITE_OK && (*pCur).iPage as ::core::ffi::c_int > iCellDepth {
            releasePageNotNull((*pCur).pPage);
            (*pCur).iPage -= 1;
            while (*pCur).iPage as ::core::ffi::c_int > iCellDepth {
                let c2rust_fresh12 = (*pCur).iPage;
                (*pCur).iPage = (*pCur).iPage - 1;
                releasePage((*pCur).apPage[c2rust_fresh12 as usize]);
            }
            (*pCur).pPage = (*pCur).apPage[(*pCur).iPage as usize];
            rc = balance(pCur);
        }
        if rc == SQLITE_OK {
            if bPreserve as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                (*pCur).eState = CURSOR_SKIPNEXT as u8_0;
                if iCellIdx >= (*pPage).nCell as ::core::ffi::c_int {
                    (*pCur).skipNext = -1 as ::core::ffi::c_int;
                    (*pCur).ix = ((*pPage).nCell as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as u16_0;
                } else {
                    (*pCur).skipNext = 1 as ::core::ffi::c_int;
                }
            } else {
                rc = moveToRoot(pCur);
                if bPreserve != 0 {
                    btreeReleaseAllCursorPages(pCur);
                    (*pCur).eState = CURSOR_REQUIRESEEK as u8_0;
                }
                if rc == SQLITE_EMPTY {
                    rc = SQLITE_OK;
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn btreeCreateTable(
    mut p: *mut Btree,
    mut piTable: *mut Pgno,
    mut createTabFlags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        let mut pRoot: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut pgnoRoot: Pgno = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut ptfFlags: ::core::ffi::c_int = 0;
        if (*pBt).autoVacuum != 0 {
            let mut pgnoMove: Pgno = 0;
            let mut pPageMove: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
            invalidateAllOverflowCache(pBt);
            sqlite3BtreeGetMeta(p, BTREE_LARGEST_ROOT_PAGE, &raw mut pgnoRoot);
            if pgnoRoot > btreePagecount(pBt) {
                return sqlite3CorruptError(10049 as ::core::ffi::c_int);
            }
            pgnoRoot = pgnoRoot.wrapping_add(1);
            while pgnoRoot == ptrmapPageno(pBt, pgnoRoot)
                || pgnoRoot
                    == (sqlite3PendingByte as ::core::ffi::c_uint)
                        .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint)
            {
                pgnoRoot = pgnoRoot.wrapping_add(1);
            }
            rc = allocateBtreePage(
                pBt,
                &raw mut pPageMove,
                &raw mut pgnoMove,
                pgnoRoot,
                BTALLOC_EXACT as u8_0,
            );
            if rc != SQLITE_OK {
                return rc;
            }
            if pgnoMove != pgnoRoot {
                let mut eType: u8_0 = 0 as u8_0;
                let mut iPtrPage: Pgno = 0 as Pgno;
                rc = saveAllCursors(pBt, 0 as Pgno, ::core::ptr::null_mut::<BtCursor>());
                releasePage(pPageMove);
                if rc != SQLITE_OK {
                    return rc;
                }
                rc = btreeGetPage(
                    pBt,
                    pgnoRoot,
                    &raw mut pRoot,
                    0 as ::core::ffi::c_int,
                );
                if rc != SQLITE_OK {
                    return rc;
                }
                rc = ptrmapGet(pBt, pgnoRoot, &raw mut eType, &raw mut iPtrPage);
                if eType as ::core::ffi::c_int == PTRMAP_ROOTPAGE
                    || eType as ::core::ffi::c_int == PTRMAP_FREEPAGE
                {
                    rc = sqlite3CorruptError(10097 as ::core::ffi::c_int);
                }
                if rc != SQLITE_OK {
                    releasePage(pRoot);
                    return rc;
                }
                rc = relocatePage(
                    pBt,
                    pRoot,
                    eType,
                    iPtrPage,
                    pgnoMove,
                    0 as ::core::ffi::c_int,
                );
                releasePage(pRoot);
                if rc != SQLITE_OK {
                    return rc;
                }
                rc = btreeGetPage(
                    pBt,
                    pgnoRoot,
                    &raw mut pRoot,
                    0 as ::core::ffi::c_int,
                );
                if rc != SQLITE_OK {
                    return rc;
                }
                rc = sqlite3PagerWrite((*pRoot).pDbPage);
                if rc != SQLITE_OK {
                    releasePage(pRoot);
                    return rc;
                }
            } else {
                pRoot = pPageMove;
            }
            ptrmapPut(pBt, pgnoRoot, PTRMAP_ROOTPAGE as u8_0, 0 as Pgno, &raw mut rc);
            if rc != 0 {
                releasePage(pRoot);
                return rc;
            }
            rc = sqlite3BtreeUpdateMeta(p, 4 as ::core::ffi::c_int, pgnoRoot as u32_0);
            if rc != 0 {
                releasePage(pRoot);
                return rc;
            }
        } else {
            rc = allocateBtreePage(
                pBt,
                &raw mut pRoot,
                &raw mut pgnoRoot,
                1 as Pgno,
                0 as u8_0,
            );
            if rc != 0 {
                return rc;
            }
        }
        if createTabFlags & BTREE_INTKEY != 0 {
            ptfFlags = PTF_INTKEY | PTF_LEAFDATA | PTF_LEAF;
        } else {
            ptfFlags = PTF_ZERODATA | PTF_LEAF;
        }
        zeroPage(pRoot, ptfFlags);
        sqlite3PagerUnref((*pRoot).pDbPage);
        *piTable = pgnoRoot;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCreateTable(
    mut p: *mut Btree,
    mut piTable: *mut Pgno,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = btreeCreateTable(p, piTable, flags);
        return rc;
    }
}
unsafe extern "C" fn clearDatabasePage(
    mut pBt: *mut BtShared,
    mut pgno: Pgno,
    mut freePageFlag: ::core::ffi::c_int,
    mut pnChange: *mut i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut pCell: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut hdr: ::core::ffi::c_int = 0;
        let mut info: CellInfo = CellInfo {
            nKey: 0,
            pPayload: ::core::ptr::null_mut::<u8_0>(),
            nPayload: 0,
            nLocal: 0,
            nSize: 0

        };
        if pgno > btreePagecount(pBt) {
            return sqlite3CorruptError(10187 as ::core::ffi::c_int);
        }
        rc = getAndInitPage(pBt, pgno, &raw mut pPage, 0 as ::core::ffi::c_int);
        if rc != 0 {
            return rc;
        }
        if (*pBt).openFlags as ::core::ffi::c_int & BTREE_SINGLE
            == 0 as ::core::ffi::c_int
            && sqlite3PagerPageRefcount((*pPage).pDbPage)
                != 1 as ::core::ffi::c_int
                    + (pgno == 1 as ::core::ffi::c_uint) as ::core::ffi::c_int
        {
            rc = sqlite3CorruptError(10194 as ::core::ffi::c_int);
        } else {
            hdr = (*pPage).hdrOffset as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            loop {
                if !(i < (*pPage).nCell as ::core::ffi::c_int) {
                    c2rust_current_block = 5783071609795492627;
                    break;
                }
                pCell = (*pPage)
                    .aData
                    .offset(
                        ((*pPage).maskPage as ::core::ffi::c_int
                            & ((*((*pPage)
                                .aCellIdx
                                .offset((2 as ::core::ffi::c_int * i) as isize)
                                as *mut u8_0)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                | *((*pPage)
                                    .aCellIdx
                                    .offset((2 as ::core::ffi::c_int * i) as isize)
                                    as *mut u8_0)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int)) as isize,
                    ) as *mut ::core::ffi::c_uchar;
                if (*pPage).leaf == 0 {
                    rc = clearDatabasePage(
                        pBt,
                        sqlite3Get4byte(pCell) as Pgno,
                        1 as ::core::ffi::c_int,
                        pnChange,
                    );
                    if rc != 0 {
                        c2rust_current_block = 10203064613522441271;
                        break;
                    }
                }
                (*pPage)
                    .xParseCell
                    .expect(
                        "non-null function pointer",
                    )(pPage, pCell as *mut u8_0, &raw mut info);
                if info.nLocal as ::core::ffi::c_uint != info.nPayload {
                    rc = clearCellOverflow(pPage, pCell, &raw mut info);
                } else {
                    rc = SQLITE_OK;
                }
                if rc != 0 {
                    c2rust_current_block = 10203064613522441271;
                    break;
                }
                i += 1;
            }
            match c2rust_current_block {
                10203064613522441271 => {}
                _ => {
                    if (*pPage).leaf == 0 {
                        rc = clearDatabasePage(
                            pBt,
                            sqlite3Get4byte(
                                (*pPage)
                                    .aData
                                    .offset((hdr + 8 as ::core::ffi::c_int) as isize)
                                    as *mut u8_0,
                            ) as Pgno,
                            1 as ::core::ffi::c_int,
                            pnChange,
                        );
                        if rc != 0 {
                            c2rust_current_block = 10203064613522441271;
                        } else {
                            if (*pPage).intKey != 0 {
                                pnChange = ::core::ptr::null_mut::<i64_0>();
                            }
                            c2rust_current_block = 4068382217303356765;
                        }
                    } else {
                        c2rust_current_block = 4068382217303356765;
                    }
                    match c2rust_current_block {
                        10203064613522441271 => {}
                        _ => {
                            if !pnChange.is_null() {
                                *pnChange += (*pPage).nCell as ::core::ffi::c_longlong;
                            }
                            if freePageFlag != 0 {
                                freePage(pPage, &raw mut rc);
                            } else {
                                rc = sqlite3PagerWrite((*pPage).pDbPage);
                                if rc == 0 as ::core::ffi::c_int {
                                    zeroPage(
                                        pPage,
                                        *(*pPage).aData.offset(hdr as isize) as ::core::ffi::c_int
                                            | PTF_LEAF,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        releasePage(pPage);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeClearTable(
    mut p: *mut Btree,
    mut iTable: ::core::ffi::c_int,
    mut pnChange: *mut i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pBt: *mut BtShared = (*p).pBt;
        rc = saveAllCursors(pBt, iTable as Pgno, ::core::ptr::null_mut::<BtCursor>());
        if SQLITE_OK == rc {
            if (*p).hasIncrblobCur != 0 {
                invalidateIncrblobCursors(
                    p,
                    iTable as Pgno,
                    0 as i64_0,
                    1 as ::core::ffi::c_int,
                );
            }
            rc = clearDatabasePage(
                pBt,
                iTable as Pgno,
                0 as ::core::ffi::c_int,
                pnChange,
            );
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeClearTableOfCursor(
    mut pCur: *mut BtCursor,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3BtreeClearTable(
            (*pCur).pBtree,
            (*pCur).pgnoRoot as ::core::ffi::c_int,
            ::core::ptr::null_mut::<i64_0>(),
        );
    }
}
unsafe extern "C" fn btreeDropTable(
    mut p: *mut Btree,
    mut iTable: Pgno,
    mut piMoved: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut pBt: *mut BtShared = (*p).pBt;
        if iTable > btreePagecount(pBt) {
            return sqlite3CorruptError(10298 as ::core::ffi::c_int);
        }
        rc = sqlite3BtreeClearTable(
            p,
            iTable as ::core::ffi::c_int,
            ::core::ptr::null_mut::<i64_0>(),
        );
        if rc != 0 {
            return rc;
        }
        rc = btreeGetPage(pBt, iTable, &raw mut pPage, 0 as ::core::ffi::c_int);
        if rc != 0 {
            releasePage(pPage);
            return rc;
        }
        *piMoved = 0 as ::core::ffi::c_int;
        if (*pBt).autoVacuum != 0 {
            let mut maxRootPgno: Pgno = 0;
            sqlite3BtreeGetMeta(p, BTREE_LARGEST_ROOT_PAGE, &raw mut maxRootPgno);
            if iTable == maxRootPgno {
                freePage(pPage, &raw mut rc);
                releasePage(pPage);
                if rc != SQLITE_OK {
                    return rc;
                }
            } else {
                let mut pMove: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
                releasePage(pPage);
                rc = btreeGetPage(
                    pBt,
                    maxRootPgno,
                    &raw mut pMove,
                    0 as ::core::ffi::c_int,
                );
                if rc != SQLITE_OK {
                    return rc;
                }
                rc = relocatePage(
                    pBt,
                    pMove,
                    PTRMAP_ROOTPAGE as u8_0,
                    0 as Pgno,
                    iTable,
                    0 as ::core::ffi::c_int,
                );
                releasePage(pMove);
                if rc != SQLITE_OK {
                    return rc;
                }
                pMove = ::core::ptr::null_mut::<MemPage>();
                rc = btreeGetPage(
                    pBt,
                    maxRootPgno,
                    &raw mut pMove,
                    0 as ::core::ffi::c_int,
                );
                freePage(pMove, &raw mut rc);
                releasePage(pMove);
                if rc != SQLITE_OK {
                    return rc;
                }
                *piMoved = maxRootPgno as ::core::ffi::c_int;
            }
            maxRootPgno = maxRootPgno.wrapping_sub(1);
            while maxRootPgno
                == (sqlite3PendingByte as ::core::ffi::c_uint)
                    .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
                || ptrmapPageno(pBt, maxRootPgno) == maxRootPgno
            {
                maxRootPgno = maxRootPgno.wrapping_sub(1);
            }
            rc = sqlite3BtreeUpdateMeta(
                p,
                4 as ::core::ffi::c_int,
                maxRootPgno as u32_0,
            );
        } else {
            freePage(pPage, &raw mut rc);
            releasePage(pPage);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeDropTable(
    mut p: *mut Btree,
    mut iTable: ::core::ffi::c_int,
    mut piMoved: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = btreeDropTable(p, iTable as Pgno, piMoved);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeGetMeta(
    mut p: *mut Btree,
    mut idx: ::core::ffi::c_int,
    mut pMeta: *mut u32_0,
) {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        if idx == BTREE_DATA_VERSION {
            *pMeta = sqlite3PagerDataVersion((*pBt).pPager)
                .wrapping_add((*p).iBDataVersion);
        } else {
            *pMeta = sqlite3Get4byte(
                (*(*pBt).pPage1)
                    .aData
                    .offset(
                        (36 as ::core::ffi::c_int + idx * 4 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut u8_0,
            );
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeUpdateMeta(
    mut p: *mut Btree,
    mut idx: ::core::ffi::c_int,
    mut iMeta: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        let mut pP1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        pP1 = (*(*pBt).pPage1).aData as *mut ::core::ffi::c_uchar;
        rc = sqlite3PagerWrite((*(*pBt).pPage1).pDbPage);
        if rc == SQLITE_OK {
            sqlite3Put4byte(
                pP1
                    .offset(
                        (36 as ::core::ffi::c_int + idx * 4 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut u8_0,
                iMeta,
            );
            if idx == BTREE_INCR_VACUUM {
                (*pBt).incrVacuum = iMeta as u8_0;
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCount(
    mut db: *mut sqlite3,
    mut pCur: *mut BtCursor,
    mut pnEntry: *mut i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nEntry: i64_0 = 0 as i64_0;
        let mut rc: ::core::ffi::c_int = 0;
        rc = moveToRoot(pCur);
        if rc == SQLITE_EMPTY {
            *pnEntry = 0 as i64_0;
            return SQLITE_OK;
        }
        while rc == SQLITE_OK
            && (*db).u1.isInterrupted
                == 0
        {
            let mut iIdx: ::core::ffi::c_int = 0;
            let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
            pPage = (*pCur).pPage;
            if (*pPage).leaf as ::core::ffi::c_int != 0 || (*pPage).intKey == 0 {
                nEntry += (*pPage).nCell as ::core::ffi::c_longlong;
            }
            if (*pPage).leaf != 0 {
                loop {
                    if (*pCur).iPage as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        *pnEntry = nEntry;
                        return moveToRoot(pCur);
                    }
                    moveToParent(pCur);
                    if !((*pCur).ix as ::core::ffi::c_int
                        >= (*(*pCur).pPage).nCell as ::core::ffi::c_int)
                    {
                        break;
                    }
                }
                (*pCur).ix = (*pCur).ix.wrapping_add(1);
                pPage = (*pCur).pPage;
            }
            iIdx = (*pCur).ix as ::core::ffi::c_int;
            if iIdx == (*pPage).nCell as ::core::ffi::c_int {
                rc = moveToChild(
                    pCur,
                    sqlite3Get4byte(
                        (*pPage)
                            .aData
                            .offset(
                                ((*pPage).hdrOffset as ::core::ffi::c_int
                                    + 8 as ::core::ffi::c_int) as isize,
                            ) as *mut u8_0,
                    ),
                );
            } else {
                rc = moveToChild(
                    pCur,
                    sqlite3Get4byte(
                        (*pPage)
                            .aData
                            .offset(
                                ((*pPage).maskPage as ::core::ffi::c_int
                                    & ((*((*pPage)
                                        .aCellIdx
                                        .offset((2 as ::core::ffi::c_int * iIdx) as isize)
                                        as *mut u8_0)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                        | *((*pPage)
                                            .aCellIdx
                                            .offset((2 as ::core::ffi::c_int * iIdx) as isize)
                                            as *mut u8_0)
                                            .offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int)) as isize,
                            ),
                    ),
                );
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreePager(mut p: *mut Btree) -> *mut Pager {
    unsafe {
        return (*(*p).pBt).pPager as *mut Pager;
    }
}
unsafe extern "C" fn checkOom(mut pCheck: *mut IntegrityCk) {
    unsafe {
        (*pCheck).rc = SQLITE_NOMEM;
        (*pCheck).mxErr = 0 as ::core::ffi::c_int;
        if (*pCheck).nErr == 0 as ::core::ffi::c_int {
            (*pCheck).nErr += 1;
        }
    }
}
unsafe extern "C" fn checkProgress(mut pCheck: *mut IntegrityCk) {
    unsafe {
        let mut db: *mut sqlite3 = (*pCheck).db;
        if (*db).u1.isInterrupted != 0
        {
            (*pCheck).rc = SQLITE_INTERRUPT;
            (*pCheck).nErr += 1;
            (*pCheck).mxErr = 0 as ::core::ffi::c_int;
        }
        if (*db).xProgress.is_some() {
            (*pCheck).nStep = (*pCheck).nStep.wrapping_add(1);
            if ((*pCheck).nStep as ::core::ffi::c_uint).wrapping_rem((*db).nProgressOps)
                == 0 as ::core::ffi::c_uint
                && (*db)
                    .xProgress
                    .expect("non-null function pointer")((*db).pProgressArg) != 0
            {
                (*pCheck).rc = SQLITE_INTERRUPT;
                (*pCheck).nErr += 1;
                (*pCheck).mxErr = 0 as ::core::ffi::c_int;
            }
        }
    }
}
unsafe extern "C" fn checkAppendMsg(
    mut pCheck: *mut IntegrityCk,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        
        checkProgress(pCheck);
        if (*pCheck).mxErr == 0 {
            return;
        }
        (*pCheck).mxErr -= 1;
        (*pCheck).nErr += 1;
        let mut ap = c2rust_args;
        if (*pCheck).errMsg.nChar != 0 {
            sqlite3_str_append(
                &raw mut (*pCheck).errMsg,
                b"\n\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
        }
        if !(*pCheck).zPfx.is_null() {
            sqlite3_str_appendf(
                &raw mut (*pCheck).errMsg,
                (*pCheck).zPfx,
                (*pCheck).v0,
                (*pCheck).v1,
                (*pCheck).v2,
            );
        }
        sqlite3_str_vappendf(&raw mut (*pCheck).errMsg, zFormat, ap);
        if (*pCheck).errMsg.accError as ::core::ffi::c_int == SQLITE_NOMEM {
            checkOom(pCheck);
        }
    }
}
unsafe extern "C" fn getPageReferenced(
    mut pCheck: *mut IntegrityCk,
    mut iPg: Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        return *(*pCheck)
            .aPgRef
            .offset(
                (iPg as ::core::ffi::c_uint).wrapping_div(8 as ::core::ffi::c_uint)
                    as isize,
            ) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << (iPg as ::core::ffi::c_uint & 0x7 as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn setPageReferenced(mut pCheck: *mut IntegrityCk, mut iPg: Pgno) {
    unsafe {
        let ref mut c2rust_fresh33 = *(*pCheck)
            .aPgRef
            .offset(
                (iPg as ::core::ffi::c_uint).wrapping_div(8 as ::core::ffi::c_uint)
                    as isize,
            );
        *c2rust_fresh33 = (*c2rust_fresh33 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int)
                << (iPg as ::core::ffi::c_uint & 0x7 as ::core::ffi::c_uint)) as u8_0;
    }
}
unsafe extern "C" fn checkRef(
    mut pCheck: *mut IntegrityCk,
    mut iPage: Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        if iPage > (*pCheck).nCkPage || iPage == 0 as ::core::ffi::c_uint {
            checkAppendMsg(
                pCheck,
                b"invalid page number %u\0".as_ptr() as *const ::core::ffi::c_char,
                iPage,
            );
            return 1 as ::core::ffi::c_int;
        }
        if getPageReferenced(pCheck, iPage) != 0 {
            checkAppendMsg(
                pCheck,
                b"2nd reference to page %u\0".as_ptr() as *const ::core::ffi::c_char,
                iPage,
            );
            return 1 as ::core::ffi::c_int;
        }
        setPageReferenced(pCheck, iPage);
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn checkPtrmap(
    mut pCheck: *mut IntegrityCk,
    mut iChild: Pgno,
    mut eType: u8_0,
    mut iParent: Pgno,
) {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut ePtrmapType: u8_0 = 0;
        let mut iPtrmapParent: Pgno = 0;
        rc = ptrmapGet(
            (*pCheck).pBt,
            iChild,
            &raw mut ePtrmapType,
            &raw mut iPtrmapParent,
        );
        if rc != SQLITE_OK {
            if rc == SQLITE_NOMEM || rc == SQLITE_IOERR_NOMEM {
                checkOom(pCheck);
            }
            checkAppendMsg(
                pCheck,
                b"Failed to read ptrmap key=%u\0".as_ptr() as *const ::core::ffi::c_char,
                iChild,
            );
            return;
        }
        if ePtrmapType as ::core::ffi::c_int != eType as ::core::ffi::c_int
            || iPtrmapParent != iParent
        {
            checkAppendMsg(
                pCheck,
                b"Bad ptr map entry key=%u expected=(%u,%u) got=(%u,%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iChild,
                eType as ::core::ffi::c_int,
                iParent,
                ePtrmapType as ::core::ffi::c_int,
                iPtrmapParent,
            );
        }
    }
}
unsafe extern "C" fn checkList(
    mut pCheck: *mut IntegrityCk,
    mut isFreeList: ::core::ffi::c_int,
    mut iPage: Pgno,
    mut N: u32_0,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut expected: u32_0 = N;
        let mut nErrAtStart: ::core::ffi::c_int = (*pCheck).nErr;
        while iPage != 0 as ::core::ffi::c_uint && (*pCheck).mxErr != 0 {
            let mut pOvflPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
            let mut pOvflData: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
                ::core::ffi::c_uchar,
            >();
            if checkRef(pCheck, iPage) != 0 {
                break;
            }
            N = N.wrapping_sub(1);
            if sqlite3PagerGet(
                (*pCheck).pPager,
                iPage,
                &raw mut pOvflPage,
                0 as ::core::ffi::c_int,
            ) != 0
            {
                checkAppendMsg(
                    pCheck,
                    b"failed to get page %u\0".as_ptr() as *const ::core::ffi::c_char,
                    iPage,
                );
                break;
            } else {
                pOvflData = sqlite3PagerGetData(pOvflPage) as *mut ::core::ffi::c_uchar;
                if isFreeList != 0 {
                    let mut n: u32_0 = sqlite3Get4byte(
                        pOvflData.offset(4 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uchar,
                    );
                    if (*(*pCheck).pBt).autoVacuum != 0 {
                        checkPtrmap(pCheck, iPage, PTRMAP_FREEPAGE as u8_0, 0 as Pgno);
                    }
                    if n
                        > ((*(*pCheck).pBt).usableSize as ::core::ffi::c_uint)
                            .wrapping_div(4 as ::core::ffi::c_uint)
                            .wrapping_sub(2 as ::core::ffi::c_uint)
                    {
                        checkAppendMsg(
                            pCheck,
                            b"freelist leaf count too big on page %u\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            iPage,
                        );
                        N = N.wrapping_sub(1);
                    } else {
                        i = 0 as ::core::ffi::c_int;
                        while i < n as ::core::ffi::c_int {
                            let mut iFreePage: Pgno = sqlite3Get4byte(
                                pOvflData
                                    .offset(
                                        (8 as ::core::ffi::c_int + i * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *mut ::core::ffi::c_uchar,
                            ) as Pgno;
                            if (*(*pCheck).pBt).autoVacuum != 0 {
                                checkPtrmap(
                                    pCheck,
                                    iFreePage,
                                    PTRMAP_FREEPAGE as u8_0,
                                    0 as Pgno,
                                );
                            }
                            checkRef(pCheck, iFreePage);
                            i += 1;
                        }
                        N = (N as ::core::ffi::c_uint)
                            .wrapping_sub(n as ::core::ffi::c_uint) as u32_0 as u32_0;
                    }
                } else if (*(*pCheck).pBt).autoVacuum as ::core::ffi::c_int != 0
                    && N > 0 as ::core::ffi::c_uint
                {
                    i = sqlite3Get4byte(pOvflData) as ::core::ffi::c_int;
                    checkPtrmap(pCheck, i as Pgno, PTRMAP_OVERFLOW2 as u8_0, iPage);
                }
                iPage = sqlite3Get4byte(pOvflData) as Pgno;
                sqlite3PagerUnref(pOvflPage);
            }
        }
        if N != 0 && nErrAtStart == (*pCheck).nErr {
            checkAppendMsg(
                pCheck,
                b"%s is %u but should be %u\0".as_ptr() as *const ::core::ffi::c_char,
                if isFreeList != 0 {
                    b"size\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"overflow list length\0".as_ptr() as *const ::core::ffi::c_char
                },
                expected.wrapping_sub(N),
                expected,
            );
        }
    }
}
unsafe extern "C" fn btreeHeapInsert(mut aHeap: *mut u32_0, mut x: u32_0) {
    unsafe {
        let mut j: u32_0 = 0;
        let mut i: u32_0 = 0;
        let ref mut c2rust_fresh32 = *aHeap.offset(0 as ::core::ffi::c_int as isize);
        *c2rust_fresh32 = (*c2rust_fresh32).wrapping_add(1);
        i = *c2rust_fresh32;
        *aHeap.offset(i as isize) = x;
        loop {
            j = (i as ::core::ffi::c_uint).wrapping_div(2 as ::core::ffi::c_uint)
                as u32_0;
            if !(j > 0 as ::core::ffi::c_uint
                && *aHeap.offset(j as isize) > *aHeap.offset(i as isize))
            {
                break;
            }
            x = *aHeap.offset(j as isize);
            *aHeap.offset(j as isize) = *aHeap.offset(i as isize);
            *aHeap.offset(i as isize) = x;
            i = j;
        };
    }
}
unsafe extern "C" fn btreeHeapPull(
    mut aHeap: *mut u32_0,
    mut pOut: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut j: u32_0 = 0;
        let mut i: u32_0 = 0;
        let mut x: u32_0 = 0;
        x = *aHeap.offset(0 as ::core::ffi::c_int as isize);
        if x == 0 as ::core::ffi::c_uint {
            return 0 as ::core::ffi::c_int;
        }
        *pOut = *aHeap.offset(1 as ::core::ffi::c_int as isize);
        *aHeap.offset(1 as ::core::ffi::c_int as isize) = *aHeap.offset(x as isize);
        *aHeap.offset(x as isize) = 0xffffffff as ::core::ffi::c_uint as u32_0;
        let ref mut c2rust_fresh31 = *aHeap.offset(0 as ::core::ffi::c_int as isize);
        *c2rust_fresh31 = (*c2rust_fresh31).wrapping_sub(1);
        i = 1 as u32_0;
        loop {
            j = (i as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint)
                as u32_0;
            if !(j <= *aHeap.offset(0 as ::core::ffi::c_int as isize)) {
                break;
            }
            if *aHeap.offset(j as isize)
                > *aHeap
                    .offset(
                        (j as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint)
                            as isize,
                    )
            {
                j = j.wrapping_add(1);
            }
            if *aHeap.offset(i as isize) < *aHeap.offset(j as isize) {
                break;
            }
            x = *aHeap.offset(i as isize);
            *aHeap.offset(i as isize) = *aHeap.offset(j as isize);
            *aHeap.offset(j as isize) = x;
            i = j;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn checkTreePage(
    mut pCheck: *mut IntegrityCk,
    mut iPage: Pgno,
    mut piMinKey: *mut i64_0,
    mut maxKey: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPage: *mut MemPage = ::core::ptr::null_mut::<MemPage>();
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut depth: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut d2: ::core::ffi::c_int = 0;
        let mut pgno: ::core::ffi::c_int = 0;
        let mut nFrag: ::core::ffi::c_int = 0;
        let mut hdr: ::core::ffi::c_int = 0;
        let mut cellStart: ::core::ffi::c_int = 0;
        let mut nCell: ::core::ffi::c_int = 0;
        let mut doCoverageCheck: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut keyCanBeEqual: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut data: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pCell: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pCellIdx: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pBt: *mut BtShared = ::core::ptr::null_mut::<BtShared>();
        let mut pc: u32_0 = 0;
        let mut usableSize: u32_0 = 0;
        let mut contentOffset: u32_0 = 0;
        let mut heap: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
        let mut x: u32_0 = 0;
        let mut prev: u32_0 = 0 as u32_0;
        let mut saved_zPfx: *const ::core::ffi::c_char = (*pCheck).zPfx;
        let mut saved_v1: ::core::ffi::c_int = (*pCheck).v1 as ::core::ffi::c_int;
        let mut saved_v2: ::core::ffi::c_int = (*pCheck).v2;
        let mut savedIsInit: u8_0 = 0 as u8_0;
        checkProgress(pCheck);
        if !((*pCheck).mxErr == 0 as ::core::ffi::c_int) {
            pBt = (*pCheck).pBt;
            usableSize = (*pBt).usableSize;
            if iPage == 0 as ::core::ffi::c_uint {
                return 0 as ::core::ffi::c_int;
            }
            if checkRef(pCheck, iPage) != 0 {
                return 0 as ::core::ffi::c_int;
            }
            (*pCheck).zPfx = b"Tree %u page %u: \0".as_ptr()
                as *const ::core::ffi::c_char;
            (*pCheck).v1 = iPage;
            rc = btreeGetPage(pBt, iPage, &raw mut pPage, 0 as ::core::ffi::c_int);
            if rc != 0 as ::core::ffi::c_int {
                checkAppendMsg(
                    pCheck,
                    b"unable to get the page. error code=%d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    rc,
                );
                if rc == SQLITE_IOERR_NOMEM {
                    (*pCheck).rc = SQLITE_NOMEM;
                }
            } else {
                savedIsInit = (*pPage).isInit;
                (*pPage).isInit = 0 as u8_0;
                rc = btreeInitPage(pPage);
                if rc != 0 as ::core::ffi::c_int {
                    checkAppendMsg(
                        pCheck,
                        b"btreeInitPage() returns error code %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        rc,
                    );
                } else {
                    rc = btreeComputeFreeSpace(pPage);
                    if rc != 0 as ::core::ffi::c_int {
                        checkAppendMsg(
                            pCheck,
                            b"free space corruption\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            rc,
                        );
                    } else {
                        data = (*pPage).aData;
                        hdr = (*pPage).hdrOffset as ::core::ffi::c_int;
                        (*pCheck).zPfx = b"Tree %u page %u cell %u: \0".as_ptr()
                            as *const ::core::ffi::c_char;
                        contentOffset = ((((*(data
                            .offset((hdr + 5 as ::core::ffi::c_int) as isize)
                            as *mut u8_0)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                            | *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                                as *mut u8_0)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
                            & 0xffff as ::core::ffi::c_int) + 1 as ::core::ffi::c_int)
                            as u32_0;
                        nCell = (*(data.offset((hdr + 3 as ::core::ffi::c_int) as isize)
                            as *mut u8_0)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                            | *(data.offset((hdr + 3 as ::core::ffi::c_int) as isize)
                                as *mut u8_0)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int;
                        if (*pPage).leaf as ::core::ffi::c_int != 0
                            || (*pPage).intKey as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            (*pCheck).nRow += nCell as ::core::ffi::c_longlong;
                        }
                        cellStart = hdr + 12 as ::core::ffi::c_int
                            - 4 as ::core::ffi::c_int
                                * (*pPage).leaf as ::core::ffi::c_int;
                        pCellIdx = data
                            .offset(
                                (cellStart
                                    + 2 as ::core::ffi::c_int
                                        * (nCell - 1 as ::core::ffi::c_int)) as isize,
                            ) as *mut u8_0;
                        if (*pPage).leaf == 0 {
                            pgno = sqlite3Get4byte(
                                data.offset((hdr + 8 as ::core::ffi::c_int) as isize)
                                    as *mut u8_0,
                            ) as ::core::ffi::c_int;
                            if (*pBt).autoVacuum != 0 {
                                (*pCheck).zPfx = b"Tree %u page %u right child: \0".as_ptr()
                                    as *const ::core::ffi::c_char;
                                checkPtrmap(
                                    pCheck,
                                    pgno as Pgno,
                                    PTRMAP_BTREE as u8_0,
                                    iPage,
                                );
                            }
                            depth = checkTreePage(
                                pCheck,
                                pgno as Pgno,
                                &raw mut maxKey,
                                maxKey,
                            );
                            keyCanBeEqual = 0 as ::core::ffi::c_int;
                        } else {
                            heap = (*pCheck).heap;
                            *heap.offset(0 as ::core::ffi::c_int as isize) = 0 as u32_0;
                        }
                        i = nCell - 1 as ::core::ffi::c_int;
                        while i >= 0 as ::core::ffi::c_int && (*pCheck).mxErr != 0 {
                            let mut info: CellInfo = CellInfo {
                                nKey: 0,
                                pPayload: ::core::ptr::null_mut::<u8_0>(),
                                nPayload: 0,
                                nLocal: 0,
                                nSize: 0

                            };
                            (*pCheck).v2 = i;
                            pc = ((*pCellIdx.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                | *pCellIdx.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as u32_0;
                            pCellIdx = pCellIdx
                                .offset(-(2 as ::core::ffi::c_int as isize));
                            if pc < contentOffset
                                || pc
                                    > (usableSize as ::core::ffi::c_uint)
                                        .wrapping_sub(4 as ::core::ffi::c_uint)
                            {
                                checkAppendMsg(
                                    pCheck,
                                    b"Offset %u out of range %u..%u\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    pc,
                                    contentOffset,
                                    (usableSize as ::core::ffi::c_uint)
                                        .wrapping_sub(4 as ::core::ffi::c_uint),
                                );
                                doCoverageCheck = 0 as ::core::ffi::c_int;
                            } else {
                                pCell = data.offset(pc as isize) as *mut u8_0;
                                (*pPage)
                                    .xParseCell
                                    .expect(
                                        "non-null function pointer",
                                    )(pPage, pCell, &raw mut info);
                                if (pc as ::core::ffi::c_uint)
                                    .wrapping_add(info.nSize as ::core::ffi::c_uint)
                                    > usableSize
                                {
                                    checkAppendMsg(
                                        pCheck,
                                        b"Extends off end of page\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                    doCoverageCheck = 0 as ::core::ffi::c_int;
                                } else {
                                    if (*pPage).intKey != 0 {
                                        if if keyCanBeEqual != 0 {
                                            (info.nKey > maxKey) as ::core::ffi::c_int
                                        } else {
                                            (info.nKey >= maxKey) as ::core::ffi::c_int
                                        } != 0
                                        {
                                            checkAppendMsg(
                                                pCheck,
                                                b"Rowid %lld out of order\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                info.nKey,
                                            );
                                        }
                                        maxKey = info.nKey;
                                        keyCanBeEqual = 0 as ::core::ffi::c_int;
                                    }
                                    if info.nPayload > info.nLocal as ::core::ffi::c_uint {
                                        let mut nPage: u32_0 = 0;
                                        let mut pgnoOvfl: Pgno = 0;
                                        nPage = (info.nPayload as ::core::ffi::c_uint)
                                            .wrapping_sub(info.nLocal as ::core::ffi::c_uint)
                                            .wrapping_add(usableSize as ::core::ffi::c_uint)
                                            .wrapping_sub(5 as ::core::ffi::c_uint)
                                            .wrapping_div(
                                                (usableSize as ::core::ffi::c_uint)
                                                    .wrapping_sub(4 as ::core::ffi::c_uint),
                                            ) as u32_0;
                                        pgnoOvfl = sqlite3Get4byte(
                                            pCell
                                                .offset(
                                                    (info.nSize as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                                                        as isize,
                                                ) as *mut u8_0,
                                        ) as Pgno;
                                        if (*pBt).autoVacuum != 0 {
                                            checkPtrmap(
                                                pCheck,
                                                pgnoOvfl,
                                                PTRMAP_OVERFLOW1 as u8_0,
                                                iPage,
                                            );
                                        }
                                        checkList(pCheck, 0 as ::core::ffi::c_int, pgnoOvfl, nPage);
                                    }
                                    if (*pPage).leaf == 0 {
                                        pgno = sqlite3Get4byte(pCell) as ::core::ffi::c_int;
                                        if (*pBt).autoVacuum != 0 {
                                            checkPtrmap(
                                                pCheck,
                                                pgno as Pgno,
                                                PTRMAP_BTREE as u8_0,
                                                iPage,
                                            );
                                        }
                                        d2 = checkTreePage(
                                            pCheck,
                                            pgno as Pgno,
                                            &raw mut maxKey,
                                            maxKey,
                                        );
                                        keyCanBeEqual = 0 as ::core::ffi::c_int;
                                        if d2 != depth {
                                            checkAppendMsg(
                                                pCheck,
                                                b"Child page depth differs\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                            depth = d2;
                                        }
                                    } else {
                                        btreeHeapInsert(
                                            heap,
                                            pc << 16 as ::core::ffi::c_int
                                                | pc
                                                    .wrapping_add(info.nSize as u32_0)
                                                    .wrapping_sub(1 as u32_0),
                                        );
                                    }
                                }
                            }
                            i -= 1;
                        }
                        *piMinKey = maxKey;
                        (*pCheck).zPfx = ::core::ptr::null::<::core::ffi::c_char>();
                        if doCoverageCheck != 0
                            && (*pCheck).mxErr > 0 as ::core::ffi::c_int
                        {
                            if (*pPage).leaf == 0 {
                                heap = (*pCheck).heap;
                                *heap.offset(0 as ::core::ffi::c_int as isize) = 0 as u32_0;
                                i = nCell - 1 as ::core::ffi::c_int;
                                while i >= 0 as ::core::ffi::c_int {
                                    let mut size: u32_0 = 0;
                                    pc = ((*(data
                                        .offset((cellStart + i * 2 as ::core::ffi::c_int) as isize)
                                        as *mut u8_0)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                        | *(data
                                            .offset((cellStart + i * 2 as ::core::ffi::c_int) as isize)
                                            as *mut u8_0)
                                            .offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int) as u32_0;
                                    size = (*pPage)
                                        .xCellSize
                                        .expect(
                                            "non-null function pointer",
                                        )(pPage, data.offset(pc as isize) as *mut u8_0) as u32_0;
                                    btreeHeapInsert(
                                        heap,
                                        pc << 16 as ::core::ffi::c_int
                                            | pc.wrapping_add(size).wrapping_sub(1 as u32_0),
                                    );
                                    i -= 1;
                                }
                            }
                            i = (*(data.offset((hdr + 1 as ::core::ffi::c_int) as isize)
                                as *mut u8_0)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                | *(data.offset((hdr + 1 as ::core::ffi::c_int) as isize)
                                    as *mut u8_0)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int;
                            while i > 0 as ::core::ffi::c_int {
                                let mut size_0: ::core::ffi::c_int = 0;
                                let mut j: ::core::ffi::c_int = 0;
                                size_0 = (*(data
                                    .offset((i + 2 as ::core::ffi::c_int) as isize)
                                    as *mut u8_0)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                    | *(data.offset((i + 2 as ::core::ffi::c_int) as isize)
                                        as *mut u8_0)
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int;
                                btreeHeapInsert(
                                    heap,
                                    (i as u32_0) << 16 as ::core::ffi::c_int
                                        | (i + size_0 - 1 as ::core::ffi::c_int) as u32_0,
                                );
                                j = (*(data.offset(i as isize) as *mut u8_0)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                    | *(data.offset(i as isize) as *mut u8_0)
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int;
                                i = j;
                            }
                            nFrag = 0 as ::core::ffi::c_int;
                            prev = (contentOffset as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint) as u32_0;
                            while btreeHeapPull(heap, &raw mut x) != 0 {
                                if prev as ::core::ffi::c_uint
                                    & 0xffff as ::core::ffi::c_uint
                                    >= x >> 16 as ::core::ffi::c_int
                                {
                                    checkAppendMsg(
                                        pCheck,
                                        b"Multiple uses for byte %u of page %u\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        x >> 16 as ::core::ffi::c_int,
                                        iPage,
                                    );
                                    break;
                                } else {
                                    nFrag = (nFrag as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            (x as ::core::ffi::c_uint >> 16 as ::core::ffi::c_int)
                                                .wrapping_sub(
                                                    prev as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint,
                                                )
                                                .wrapping_sub(1 as ::core::ffi::c_uint),
                                        ) as ::core::ffi::c_int as ::core::ffi::c_int;
                                    prev = x;
                                }
                            }
                            nFrag = (nFrag as ::core::ffi::c_uint)
                                .wrapping_add(
                                    (usableSize as ::core::ffi::c_uint)
                                        .wrapping_sub(
                                            prev as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint,
                                        )
                                        .wrapping_sub(1 as ::core::ffi::c_uint),
                                ) as ::core::ffi::c_int as ::core::ffi::c_int;
                            if *heap.offset(0 as ::core::ffi::c_int as isize)
                                == 0 as ::core::ffi::c_uint
                                && nFrag
                                    != *data.offset((hdr + 7 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int
                            {
                                checkAppendMsg(
                                    pCheck,
                                    b"Fragmentation of %u bytes reported as %u on page %u\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    nFrag,
                                    *data.offset((hdr + 7 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int,
                                    iPage,
                                );
                            }
                        }
                    }
                }
            }
        }
        if doCoverageCheck == 0 {
            (*pPage).isInit = savedIsInit;
        }
        releasePage(pPage);
        (*pCheck).zPfx = saved_zPfx;
        (*pCheck).v1 = saved_v1 as Pgno;
        (*pCheck).v2 = saved_v2;
        return depth + 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeIntegrityCheck(
    mut db: *mut sqlite3,
    mut p: *mut Btree,
    mut aRoot: *mut Pgno,
    mut aCnt: *mut Mem,
    mut nRoot: ::core::ffi::c_int,
    mut mxErr: ::core::ffi::c_int,
    mut pnErr: *mut ::core::ffi::c_int,
    mut pzOut: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: Pgno = 0;
        let mut sCheck: IntegrityCk = IntegrityCk {
            pBt: ::core::ptr::null_mut::<BtShared>(),
            pPager: ::core::ptr::null_mut::<Pager>(),
            aPgRef: ::core::ptr::null_mut::<u8_0>(),
            nCkPage: 0,
            mxErr: 0,
            nErr: 0,
            rc: 0,
            nStep: 0,
            zPfx: ::core::ptr::null::<::core::ffi::c_char>(),
            v0: 0,
            v1: 0,
            v2: 0,
            errMsg: StrAccum {
                db: ::core::ptr::null_mut::<sqlite3>(),
                zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                nAlloc: 0,
                mxAlloc: 0,
                nChar: 0,
                accError: 0,
                printfFlags: 0

            },
            heap: ::core::ptr::null_mut::<u32_0>(),
            db: ::core::ptr::null_mut::<sqlite3>(),
            nRow: 0

        };
        let mut pBt: *mut BtShared = (*p).pBt;
        let mut savedDbFlags: u64_0 = (*(*pBt).db).flags;
        let mut zErr: [::core::ffi::c_char; 100] = [0; 100];
        let mut bPartial: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bCkFreelist: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if *aRoot.offset(0 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_uint {
            bPartial = 1 as ::core::ffi::c_int;
            if *aRoot.offset(1 as ::core::ffi::c_int as isize)
                != 1 as ::core::ffi::c_uint
            {
                bCkFreelist = 0 as ::core::ffi::c_int;
            }
        }
        memset(
            &raw mut sCheck as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<IntegrityCk>() as size_t,
        );
        sCheck.db = db;
        sCheck.pBt = pBt;
        sCheck.pPager = (*pBt).pPager;
        sCheck.nCkPage = btreePagecount(sCheck.pBt);
        sCheck.mxErr = mxErr;
        sqlite3StrAccumInit(
            &raw mut sCheck.errMsg,
            ::core::ptr::null_mut::<sqlite3>(),
            &raw mut zErr as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            SQLITE_MAX_LENGTH,
        );
        sCheck.errMsg.printfFlags = SQLITE_PRINTF_INTERNAL as u8_0;
        if !(sCheck.nCkPage == 0 as ::core::ffi::c_uint) {
            sCheck.aPgRef = sqlite3MallocZero(
                (sCheck.nCkPage as ::core::ffi::c_uint)
                    .wrapping_div(8 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint) as u64_0,
            ) as *mut u8_0;
            if sCheck.aPgRef.is_null() {
                checkOom(&raw mut sCheck);
            } else {
                sCheck.heap = sqlite3PageMalloc((*pBt).pageSize as ::core::ffi::c_int)
                    as *mut u32_0;
                if sCheck.heap.is_null() {
                    checkOom(&raw mut sCheck);
                } else {
                    i = (sqlite3PendingByte as ::core::ffi::c_uint)
                        .wrapping_div((*pBt).pageSize as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint);
                    if i <= sCheck.nCkPage {
                        setPageReferenced(&raw mut sCheck, i);
                    }
                    if bCkFreelist != 0 {
                        sCheck.zPfx = b"Freelist: \0".as_ptr()
                            as *const ::core::ffi::c_char;
                        checkList(
                            &raw mut sCheck,
                            1 as ::core::ffi::c_int,
                            sqlite3Get4byte(
                                (*(*pBt).pPage1)
                                    .aData
                                    .offset(32 as ::core::ffi::c_int as isize) as *mut u8_0,
                            ) as Pgno,
                            sqlite3Get4byte(
                                (*(*pBt).pPage1)
                                    .aData
                                    .offset(36 as ::core::ffi::c_int as isize) as *mut u8_0,
                            ),
                        );
                        sCheck.zPfx = ::core::ptr::null::<::core::ffi::c_char>();
                    }
                    if bPartial == 0 {
                        if (*pBt).autoVacuum != 0 {
                            let mut mx: Pgno = 0 as Pgno;
                            let mut mxInHdr: Pgno = 0;
                            i = 0 as Pgno;
                            while (i as ::core::ffi::c_int) < nRoot {
                                if mx < *aRoot.offset(i as isize) {
                                    mx = *aRoot.offset(i as isize);
                                }
                                i = i.wrapping_add(1);
                            }
                            mxInHdr = sqlite3Get4byte(
                                (*(*pBt).pPage1)
                                    .aData
                                    .offset(52 as ::core::ffi::c_int as isize) as *mut u8_0,
                            ) as Pgno;
                            if mx != mxInHdr {
                                checkAppendMsg(
                                    &raw mut sCheck,
                                    b"max rootpage (%u) disagrees with header (%u)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    mx,
                                    mxInHdr,
                                );
                            }
                        } else if sqlite3Get4byte(
                            (*(*pBt).pPage1)
                                .aData
                                .offset(64 as ::core::ffi::c_int as isize) as *mut u8_0,
                        ) != 0 as ::core::ffi::c_uint
                        {
                            checkAppendMsg(
                                &raw mut sCheck,
                                b"incremental_vacuum enabled with a max rootpage of zero\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    (*(*pBt).db).flags
                        &= !(SQLITE_CellSizeCk as u64_0) as ::core::ffi::c_ulonglong;
                    i = 0 as Pgno;
                    while (i as ::core::ffi::c_int) < nRoot && sCheck.mxErr != 0 {
                        sCheck.nRow = 0 as i64_0;
                        if *aRoot.offset(i as isize) != 0 {
                            let mut notUsed: i64_0 = 0;
                            if (*pBt).autoVacuum as ::core::ffi::c_int != 0
                                && *aRoot.offset(i as isize) > 1 as ::core::ffi::c_uint
                                && bPartial == 0
                            {
                                checkPtrmap(
                                    &raw mut sCheck,
                                    *aRoot.offset(i as isize),
                                    PTRMAP_ROOTPAGE as u8_0,
                                    0 as Pgno,
                                );
                            }
                            sCheck.v0 = *aRoot.offset(i as isize);
                            checkTreePage(
                                &raw mut sCheck,
                                *aRoot.offset(i as isize),
                                &raw mut notUsed,
                                LARGEST_INT64,
                            );
                        }
                        sqlite3MemSetArrayInt64(
                            aCnt as *mut sqlite3_value,
                            i as ::core::ffi::c_int,
                            sCheck.nRow,
                        );
                        i = i.wrapping_add(1);
                    }
                    (*(*pBt).db).flags = savedDbFlags;
                    if bPartial == 0 {
                        i = 1 as Pgno;
                        while i <= sCheck.nCkPage && sCheck.mxErr != 0 {
                            if getPageReferenced(&raw mut sCheck, i)
                                == 0 as ::core::ffi::c_int
                                && (ptrmapPageno(pBt, i) != i || (*pBt).autoVacuum == 0)
                            {
                                checkAppendMsg(
                                    &raw mut sCheck,
                                    b"Page %u: never used\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    i,
                                );
                            }
                            if getPageReferenced(&raw mut sCheck, i)
                                != 0 as ::core::ffi::c_int
                                && (ptrmapPageno(pBt, i) == i
                                    && (*pBt).autoVacuum as ::core::ffi::c_int != 0)
                            {
                                checkAppendMsg(
                                    &raw mut sCheck,
                                    b"Page %u: pointer map referenced\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    i,
                                );
                            }
                            i = i.wrapping_add(1);
                        }
                    }
                }
            }
        }
        sqlite3PageFree(sCheck.heap as *mut ::core::ffi::c_void);
        sqlite3_free(sCheck.aPgRef as *mut ::core::ffi::c_void);
        *pnErr = sCheck.nErr;
        if sCheck.nErr == 0 as ::core::ffi::c_int {
            sqlite3_str_reset(&raw mut sCheck.errMsg);
            *pzOut = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            *pzOut = sqlite3StrAccumFinish(&raw mut sCheck.errMsg);
        }
        return sCheck.rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeGetFilename(
    mut p: *mut Btree,
) -> *const ::core::ffi::c_char {
    unsafe {
        return sqlite3PagerFilename((*(*p).pBt).pPager, 1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeGetJournalname(
    mut p: *mut Btree,
) -> *const ::core::ffi::c_char {
    unsafe {
        return sqlite3PagerJournalname((*(*p).pBt).pPager);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeTxnState(mut p: *mut Btree) -> ::core::ffi::c_int {
    unsafe {
        return if !p.is_null() {
            (*p).inTrans as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCheckpoint(
    mut p: *mut Btree,
    mut eMode: ::core::ffi::c_int,
    mut pnLog: *mut ::core::ffi::c_int,
    mut pnCkpt: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !p.is_null() {
            let mut pBt: *mut BtShared = (*p).pBt;
            if (*pBt).inTransaction as ::core::ffi::c_int != TRANS_NONE {
                rc = SQLITE_LOCKED;
            } else {
                rc = sqlite3PagerCheckpoint(
                    (*pBt).pPager,
                    (*p).db,
                    eMode,
                    pnLog,
                    pnCkpt,
                );
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeIsInBackup(
    mut p: *mut Btree,
) -> ::core::ffi::c_int {
    unsafe {
        return ((*p).nBackup != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeSchema(
    mut p: *mut Btree,
    mut nBytes: ::core::ffi::c_int,
    mut xFree: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        if (*pBt).pSchema.is_null() && nBytes != 0 {
            (*pBt).pSchema = sqlite3DbMallocZero(
                ::core::ptr::null_mut::<sqlite3>(),
                nBytes as u64_0,
            );
            (*pBt).xFreeSchema = xFree;
        }
        return (*pBt).pSchema;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeSchemaLocked(
    mut p: *mut Btree,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = 0 as ::core::ffi::c_int;
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreePutData(
    mut pCsr: *mut BtCursor,
    mut offset: u32_0,
    mut amt: u32_0,
    mut z: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = if (*pCsr).eState as ::core::ffi::c_int >= CURSOR_REQUIRESEEK {
            btreeRestoreCursorPosition(pCsr)
        } else {
            SQLITE_OK
        };
        if rc != SQLITE_OK {
            return rc;
        }
        if (*pCsr).eState as ::core::ffi::c_int != CURSOR_VALID {
            return SQLITE_ABORT;
        }
        saveAllCursors((*pCsr).pBt, (*pCsr).pgnoRoot, pCsr);
        if (*pCsr).curFlags as ::core::ffi::c_int & BTCF_WriteFlag
            == 0 as ::core::ffi::c_int
        {
            return SQLITE_READONLY;
        }
        return accessPayload(
            pCsr,
            offset,
            amt,
            z as *mut ::core::ffi::c_uchar,
            1 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeIncrblobCursor(mut pCur: *mut BtCursor) {
    unsafe {
        (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int | BTCF_Incrblob)
            as u8_0;
        (*(*pCur).pBtree).hasIncrblobCur = 1 as u8_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeSetVersion(
    mut pBtree: *mut Btree,
    mut iVersion: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut BtShared = (*pBtree).pBt;
        let mut rc: ::core::ffi::c_int = 0;
        (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int & !BTS_NO_WAL) as u16_0;
        if iVersion == 1 as ::core::ffi::c_int {
            (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int | BTS_NO_WAL)
                as u16_0;
        }
        rc = sqlite3BtreeBeginTrans(
            pBtree,
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if rc == SQLITE_OK {
            let mut aData: *mut u8_0 = (*(*pBt).pPage1).aData;
            if *aData.offset(18 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != iVersion as u8_0 as ::core::ffi::c_int
                || *aData.offset(19 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != iVersion as u8_0 as ::core::ffi::c_int
            {
                rc = sqlite3BtreeBeginTrans(
                    pBtree,
                    2 as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if rc == SQLITE_OK {
                    rc = sqlite3PagerWrite((*(*pBt).pPage1).pDbPage);
                    if rc == SQLITE_OK {
                        *aData.offset(18 as ::core::ffi::c_int as isize) = iVersion
                            as u8_0;
                        *aData.offset(19 as ::core::ffi::c_int as isize) = iVersion
                            as u8_0;
                    }
                }
            }
        }
        (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int & !BTS_NO_WAL) as u16_0;
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeCursorHasHint(
    mut pCsr: *mut BtCursor,
    mut mask: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        return ((*pCsr).hints as ::core::ffi::c_uint & mask != 0 as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeIsReadonly(
    mut p: *mut Btree,
) -> ::core::ffi::c_int {
    unsafe {
        return ((*(*p).pBt).btsFlags as ::core::ffi::c_int & BTS_READ_ONLY
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3HeaderSizeBtree() -> ::core::ffi::c_int {
    unsafe {
        return ((::core::mem::size_of::<MemPage>() as usize).wrapping_add(7 as usize)
            & !(7 as ::core::ffi::c_int) as usize) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BtreeClearCache(mut p: *mut Btree) {
    unsafe {
        let mut pBt: *mut BtShared = (*p).pBt;
        if (*pBt).inTransaction as ::core::ffi::c_int == TRANS_NONE {
            sqlite3PagerClearCache((*pBt).pPager);
        }
    }
}
pub const LARGEST_INT64: i64_0 = 0xffffffff as i64_0
    | (0x7fffffff as ::core::ffi::c_int as i64_0) << 32 as ::core::ffi::c_int;
pub const PAGER_GET_NOCONTENT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PAGER_GET_READONLY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_AUTOVACUUM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BTREE_AUTOVACUUM_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BTREE_AUTOVACUUM_FULL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTREE_AUTOVACUUM_INCR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BTREE_MEMORY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BTREE_SINGLE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BTREE_INTKEY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTREE_LARGEST_ROOT_PAGE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BTREE_INCR_VACUUM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const BTREE_DATA_VERSION: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const BTREE_BULKLOAD: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const BTREE_SAVEPOSITION: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BTREE_APPEND: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BTREE_PREFORMAT: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_CellSizeCk: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SQLITE_ResetDatabase: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const SAVEPOINT_ROLLBACK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_PRINTF_INTERNAL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const SQLITE_FILE_HEADER: [::core::ffi::c_char; 16] = unsafe {
    ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"SQLite format 3\0")
};
pub const PTF_INTKEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PTF_ZERODATA: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PTF_LEAFDATA: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const PTF_LEAF: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const TRANS_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRANS_READ: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TRANS_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BTS_READ_ONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const BTS_PAGESIZE_FIXED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BTS_SECURE_DELETE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const BTS_FAST_SECURE: ::core::ffi::c_int = 0xc as ::core::ffi::c_int;
pub const BTS_INITIALLY_EMPTY: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BTS_NO_WAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const BTCURSOR_MAX_DEPTH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const BTCF_WriteFlag: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const BTCF_ValidNKey: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BTCF_ValidOvfl: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const BTCF_AtLast: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BTCF_Incrblob: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BTCF_Multiple: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const BTCF_Pinned: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const CURSOR_VALID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CURSOR_INVALID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CURSOR_SKIPNEXT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CURSOR_REQUIRESEEK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CURSOR_FAULT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PTRMAP_ROOTPAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PTRMAP_FREEPAGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PTRMAP_OVERFLOW1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PTRMAP_OVERFLOW2: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PTRMAP_BTREE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_MAX_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_CACHE_SIZE: ::core::ffi::c_int = -2000 as ::core::ffi::c_int;
pub const SQLITE_MAX_PAGE_SIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
