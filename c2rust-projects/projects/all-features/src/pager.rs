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
    pub type Wal;
    pub type PCache;
    pub type Bitvec;
    pub type sqlite3_backup;
    pub type sqlite3_pcache;
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
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_uri_boolean(
        z: sqlite3_filename,
        zParam: *const ::core::ffi::c_char,
        bDefault: ::core::ffi::c_int,
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
    fn sqlite3OsClose(_: *mut sqlite3_file);
    fn sqlite3OsRead(
        _: *mut sqlite3_file,
        _: *mut ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsWrite(
        _: *mut sqlite3_file,
        _: *const ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsTruncate(_: *mut sqlite3_file, size: i64_0) -> ::core::ffi::c_int;
    fn sqlite3OsSync(_: *mut sqlite3_file, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3OsFileSize(_: *mut sqlite3_file, pSize: *mut i64_0) -> ::core::ffi::c_int;
    fn sqlite3OsLock(_: *mut sqlite3_file, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3OsUnlock(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsCheckReservedLock(
        id: *mut sqlite3_file,
        pResOut: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsFileControl(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsFileControlHint(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    );
    fn sqlite3OsSectorSize(id: *mut sqlite3_file) -> ::core::ffi::c_int;
    fn sqlite3OsDeviceCharacteristics(id: *mut sqlite3_file) -> ::core::ffi::c_int;
    fn sqlite3OsFetch(
        id: *mut sqlite3_file,
        _: i64_0,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsUnfetch(
        _: *mut sqlite3_file,
        _: i64_0,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsOpen(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3OsFullPathname(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3PcacheOpen(
        szPage: ::core::ffi::c_int,
        szExtra: ::core::ffi::c_int,
        bPurgeable: ::core::ffi::c_int,
        xStress: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut PgHdr,
            ) -> ::core::ffi::c_int,
        >,
        pStress: *mut ::core::ffi::c_void,
        pToInit: *mut PCache,
    ) -> ::core::ffi::c_int;
    fn sqlite3PcacheSetPageSize(
        _: *mut PCache,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PcacheSize() -> ::core::ffi::c_int;
    fn sqlite3PcacheFetch(
        _: *mut PCache,
        _: Pgno,
        createFlag: ::core::ffi::c_int,
    ) -> *mut sqlite3_pcache_page;
    fn sqlite3PcacheFetchStress(
        _: *mut PCache,
        _: Pgno,
        _: *mut *mut sqlite3_pcache_page,
    ) -> ::core::ffi::c_int;
    fn sqlite3PcacheFetchFinish(
        _: *mut PCache,
        _: Pgno,
        pPage: *mut sqlite3_pcache_page,
    ) -> *mut PgHdr;
    fn sqlite3PcacheRelease(_: *mut PgHdr);
    fn sqlite3PcacheDrop(_: *mut PgHdr);
    fn sqlite3PcacheMakeDirty(_: *mut PgHdr);
    fn sqlite3PcacheMakeClean(_: *mut PgHdr);
    fn sqlite3PcacheCleanAll(_: *mut PCache);
    fn sqlite3PcacheClearWritable(_: *mut PCache);
    fn sqlite3PcacheMove(_: *mut PgHdr, _: Pgno);
    fn sqlite3PcacheTruncate(_: *mut PCache, x: Pgno);
    fn sqlite3PcacheDirtyList(_: *mut PCache) -> *mut PgHdr;
    fn sqlite3PcacheClose(_: *mut PCache);
    fn sqlite3PcacheClearSyncFlags(_: *mut PCache);
    fn sqlite3PcacheClear(_: *mut PCache);
    fn sqlite3PcacheRefCount(_: *mut PCache) -> i64_0;
    fn sqlite3PcacheRef(_: *mut PgHdr);
    fn sqlite3PcachePageRefcount(_: *mut PgHdr) -> i64_0;
    fn sqlite3PcachePagecount(_: *mut PCache) -> ::core::ffi::c_int;
    fn sqlite3PcacheSetCachesize(_: *mut PCache, _: ::core::ffi::c_int);
    fn sqlite3PcacheGetCachesize(_: *mut PCache) -> ::core::ffi::c_int;
    fn sqlite3PcacheSetSpillsize(
        _: *mut PCache,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PcacheShrink(_: *mut PCache);
    fn sqlite3PCachePercentDirty(_: *mut PCache) -> ::core::ffi::c_int;
    fn sqlite3PCacheIsDirty(pCache: *mut PCache) -> ::core::ffi::c_int;
    fn sqlite3WalOpen(
        _: *mut sqlite3_vfs,
        _: *mut sqlite3_file,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: i64_0,
        _: *mut *mut Wal,
    ) -> ::core::ffi::c_int;
    fn sqlite3WalClose(
        pWal: *mut Wal,
        _: *mut sqlite3,
        sync_flags: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3WalLimit(_: *mut Wal, _: i64_0);
    fn sqlite3WalBeginReadTransaction(
        pWal: *mut Wal,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3WalEndReadTransaction(pWal: *mut Wal);
    fn sqlite3WalFindFrame(_: *mut Wal, _: Pgno, _: *mut u32_0) -> ::core::ffi::c_int;
    fn sqlite3WalReadFrame(
        _: *mut Wal,
        _: u32_0,
        _: ::core::ffi::c_int,
        _: *mut u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3WalDbsize(pWal: *mut Wal) -> Pgno;
    fn sqlite3WalBeginWriteTransaction(pWal: *mut Wal) -> ::core::ffi::c_int;
    fn sqlite3WalEndWriteTransaction(pWal: *mut Wal) -> ::core::ffi::c_int;
    fn sqlite3WalUndo(
        pWal: *mut Wal,
        xUndo: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, Pgno) -> ::core::ffi::c_int,
        >,
        pUndoCtx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3WalSavepoint(pWal: *mut Wal, aWalData: *mut u32_0);
    fn sqlite3WalSavepointUndo(
        pWal: *mut Wal,
        aWalData: *mut u32_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3WalFrames(
        pWal: *mut Wal,
        _: ::core::ffi::c_int,
        _: *mut PgHdr,
        _: Pgno,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3WalCheckpoint(
        pWal: *mut Wal,
        db: *mut sqlite3,
        eMode: ::core::ffi::c_int,
        xBusy: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        >,
        pBusyArg: *mut ::core::ffi::c_void,
        sync_flags: ::core::ffi::c_int,
        nBuf: ::core::ffi::c_int,
        zBuf: *mut u8_0,
        pnLog: *mut ::core::ffi::c_int,
        pnCkpt: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3WalCallback(pWal: *mut Wal) -> ::core::ffi::c_int;
    fn sqlite3WalExclusiveMode(
        pWal: *mut Wal,
        op: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3WalHeapMemory(pWal: *mut Wal) -> ::core::ffi::c_int;
    fn sqlite3WalFile(pWal: *mut Wal) -> *mut sqlite3_file;
    fn sqlite3CorruptError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3CantopenError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3Malloc(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3MallocZero(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRaw(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3Realloc(_: *mut ::core::ffi::c_void, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MallocSize(_: *const ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn sqlite3PageMalloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3PageFree(_: *mut ::core::ffi::c_void);
    fn sqlite3FaultSim(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3BitvecCreate(_: u32_0) -> *mut Bitvec;
    fn sqlite3BitvecTest(_: *mut Bitvec, _: u32_0) -> ::core::ffi::c_int;
    fn sqlite3BitvecTestNotNull(_: *mut Bitvec, _: u32_0) -> ::core::ffi::c_int;
    fn sqlite3BitvecSet(_: *mut Bitvec, _: u32_0) -> ::core::ffi::c_int;
    fn sqlite3BitvecClear(_: *mut Bitvec, _: u32_0, _: *mut ::core::ffi::c_void);
    fn sqlite3BitvecDestroy(_: *mut Bitvec);
    fn sqlite3IsMemdb(_: *const sqlite3_vfs) -> ::core::ffi::c_int;
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3PendingByte: ::core::ffi::c_int;
    fn sqlite3BackupRestart(_: *mut sqlite3_backup);
    fn sqlite3BackupUpdate(_: *mut sqlite3_backup, _: Pgno, _: *const u8_0);
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
    fn sqlite3JournalOpen(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3JournalSize(_: *mut sqlite3_vfs) -> ::core::ffi::c_int;
    fn sqlite3JournalIsInMemory(p: *mut sqlite3_file) -> ::core::ffi::c_int;
    fn sqlite3MemJournalOpen(_: *mut sqlite3_file);
    fn sqlite3Get4byte(_: *const u8_0) -> u32_0;
    fn sqlite3Put4byte(_: *mut u8_0, _: u32_0);
    static mut sqlite3_io_error_pending: ::core::ffi::c_int;
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
pub struct Pager {
    pub pVfs: *mut sqlite3_vfs,
    pub exclusiveMode: u8_0,
    pub journalMode: u8_0,
    pub useJournal: u8_0,
    pub noSync: u8_0,
    pub fullSync: u8_0,
    pub extraSync: u8_0,
    pub syncFlags: u8_0,
    pub walSyncFlags: u8_0,
    pub tempFile: u8_0,
    pub noLock: u8_0,
    pub readOnly: u8_0,
    pub memDb: u8_0,
    pub memVfs: u8_0,
    pub eState: u8_0,
    pub eLock: u8_0,
    pub changeCountDone: u8_0,
    pub setSuper: u8_0,
    pub doNotSpill: u8_0,
    pub subjInMemory: u8_0,
    pub bUseFetch: u8_0,
    pub hasHeldSharedLock: u8_0,
    pub dbSize: Pgno,
    pub dbOrigSize: Pgno,
    pub dbFileSize: Pgno,
    pub dbHintSize: Pgno,
    pub errCode: ::core::ffi::c_int,
    pub nRec: ::core::ffi::c_int,
    pub cksumInit: u32_0,
    pub nSubRec: u32_0,
    pub pInJournal: *mut Bitvec,
    pub fd: *mut sqlite3_file,
    pub jfd: *mut sqlite3_file,
    pub sjfd: *mut sqlite3_file,
    pub journalOff: i64_0,
    pub journalHdr: i64_0,
    pub pBackup: *mut sqlite3_backup,
    pub aSavepoint: *mut PagerSavepoint,
    pub nSavepoint: ::core::ffi::c_int,
    pub iDataVersion: u32_0,
    pub dbFileVers: [::core::ffi::c_char; 16],
    pub nMmapOut: ::core::ffi::c_int,
    pub szMmap: sqlite3_int64,
    pub pMmapFreelist: *mut PgHdr,
    pub nExtra: u16_0,
    pub nReserve: i16_0,
    pub vfsFlags: u32_0,
    pub sectorSize: u32_0,
    pub mxPgno: Pgno,
    pub lckPgno: Pgno,
    pub pageSize: i64_0,
    pub journalSizeLimit: i64_0,
    pub zFilename: *mut ::core::ffi::c_char,
    pub zJournal: *mut ::core::ffi::c_char,
    pub xBusyHandler: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub pBusyHandlerArg: *mut ::core::ffi::c_void,
    pub aStat: [u32_0; 4],
    pub nRead: ::core::ffi::c_int,
    pub xReiniter: Option<unsafe extern "C" fn(*mut DbPage) -> ()>,
    pub xGet: Option<
        unsafe extern "C" fn(
            *mut Pager,
            Pgno,
            *mut *mut DbPage,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pTmpSpace: *mut ::core::ffi::c_char,
    pub pPCache: *mut PCache,
    pub pWal: *mut Wal,
    pub zWal: *mut ::core::ffi::c_char

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
pub struct PagerSavepoint {
    pub iOffset: i64_0,
    pub iHdrOffset: i64_0,
    pub pInSavepoint: *mut Bitvec,
    pub nOrig: Pgno,
    pub iSubRec: Pgno,
    pub bTruncateOnRelease: ::core::ffi::c_int,
    pub aWalData: [u32_0; 4]

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
pub type size_t = usize;
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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ABORT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_READONLY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_FULL: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SQLITE_CANTOPEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SQLITE_NOTICE: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_IOERR_SHORT_READ: ::core::ffi::c_int = SQLITE_IOERR
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CANTOPEN_SYMLINK: ::core::ffi::c_int = SQLITE_CANTOPEN
    | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_READONLY_ROLLBACK: ::core::ffi::c_int = SQLITE_READONLY
    | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_READONLY_DBMOVED: ::core::ffi::c_int = SQLITE_READONLY
    | (4 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_NOTICE_RECOVER_ROLLBACK: ::core::ffi::c_int = SQLITE_NOTICE
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OK_SYMLINK: ::core::ffi::c_int = SQLITE_OK
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_DELETEONCLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_EXCLUSIVE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MEMORY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_JOURNAL: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_OPEN_TEMP_JOURNAL: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_SUBJOURNAL: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_SUPER_JOURNAL: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_NOFOLLOW: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SAFE_APPEND: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SEQUENTIAL: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_UNDELETABLE_WHEN_OPEN: ::core::ffi::c_int = 0x800
    as ::core::ffi::c_int;
pub const SQLITE_IOCAP_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 0x1000
    as ::core::ffi::c_int;
pub const SQLITE_IOCAP_IMMUTABLE: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SUBPAGE_READ: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_SYNC_NORMAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_SYNC_FULL: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
pub const SQLITE_SYNC_DATAONLY: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_SIZE_HINT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_BUSYHANDLER: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_MMAP_SIZE: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_HAS_MOVED: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_SYNC: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_COMMIT_PHASETWO: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const SQLITE_ACCESS_EXISTS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_CACHE_HIT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CHECKPOINT_PASSIVE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const NO_LOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SHARED_LOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RESERVED_LOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXCLUSIVE_LOCK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PGHDR_DIRTY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PGHDR_WRITEABLE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const PGHDR_NEED_SYNC: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const PGHDR_DONT_WRITE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const PGHDR_MMAP: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_PTRSIZE: ::core::ffi::c_int = __SIZEOF_POINTER__;
pub const SQLITE_DEFAULT_JOURNAL_SIZE_LIMIT: ::core::ffi::c_int = -1
    as ::core::ffi::c_int;
pub const PAGER_OMIT_JOURNAL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PAGER_MEMORY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_DELETE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_PERSIST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_OFF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_TRUNCATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_MEMORY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_WAL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PAGER_GET_NOCONTENT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PAGER_GET_READONLY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PAGER_SYNCHRONOUS_OFF: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PAGER_SYNCHRONOUS_FULL: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
pub const PAGER_SYNCHRONOUS_EXTRA: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const PAGER_SYNCHRONOUS_MASK: ::core::ffi::c_int = 0x7 as ::core::ffi::c_int;
pub const PAGER_FULLFSYNC: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const PAGER_CKPT_FULLFSYNC: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const PAGER_CACHESPILL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_SYNCHRONOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_NoCkptOnClose: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SAVEPOINT_RELEASE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SAVEPOINT_ROLLBACK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const SQLITE_MAX_PAGE_SIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const SQLITE_MAX_DEFAULT_PAGE_SIZE: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const SQLITE_MAX_PAGE_COUNT: ::core::ffi::c_uint = 0xfffffffe as ::core::ffi::c_uint;
pub const PAGER_OPEN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PAGER_READER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PAGER_WRITER_LOCKED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PAGER_WRITER_CACHEMOD: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PAGER_WRITER_DBMOD: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PAGER_WRITER_FINISHED: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PAGER_ERROR: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const UNKNOWN_LOCK: ::core::ffi::c_int = EXCLUSIVE_LOCK + 1 as ::core::ffi::c_int;
pub const MAX_SECTOR_SIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const SPILLFLAG_OFF: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SPILLFLAG_ROLLBACK: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SPILLFLAG_NOSYNC: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const PAGER_STAT_HIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PAGER_STAT_MISS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PAGER_STAT_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PAGER_STAT_SPILL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_pager_readdb_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_pager_writedb_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_pager_writej_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut aJournalMagic: [::core::ffi::c_uchar; 8] = [
    0xd9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerDirectReadOk(
    mut pPager: *mut Pager,
    mut pgno: Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        if (*(*pPager).fd).pMethods.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if sqlite3PCacheIsDirty((*pPager).pPCache) != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pPager).pWal.is_null() {
            let mut iRead: u32_0 = 0 as u32_0;
            sqlite3WalFindFrame((*pPager).pWal, pgno, &raw mut iRead);
            if iRead != 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        if (*(*(*pPager).fd).pMethods)
            .xDeviceCharacteristics
            .expect("non-null function pointer")((*pPager).fd)
            & SQLITE_IOCAP_SUBPAGE_READ == 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn setGetterMethod(mut pPager: *mut Pager) {
    unsafe {
        if (*pPager).errCode != 0 {
            (*pPager).xGet = Some(
                getPageError
                    as unsafe extern "C" fn(
                        *mut Pager,
                        Pgno,
                        *mut *mut DbPage,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut Pager,
                        Pgno,
                        *mut *mut DbPage,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
        } else if (*pPager).bUseFetch != 0 {
            (*pPager).xGet = Some(
                getPageMMap
                    as unsafe extern "C" fn(
                        *mut Pager,
                        Pgno,
                        *mut *mut DbPage,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut Pager,
                        Pgno,
                        *mut *mut DbPage,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
        } else {
            (*pPager).xGet = Some(
                getPageNormal
                    as unsafe extern "C" fn(
                        *mut Pager,
                        Pgno,
                        *mut *mut DbPage,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut Pager,
                        Pgno,
                        *mut *mut DbPage,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
        };
    }
}
unsafe extern "C" fn subjRequiresPage(mut pPg: *mut PgHdr) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = (*pPg).pPager;
        let mut p: *mut PagerSavepoint = ::core::ptr::null_mut::<PagerSavepoint>();
        let mut pgno: Pgno = (*pPg).pgno;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pPager).nSavepoint {
            p = (*pPager).aSavepoint.offset(i as isize) as *mut PagerSavepoint;
            if (*p).nOrig >= pgno
                && 0 as ::core::ffi::c_int
                    == sqlite3BitvecTestNotNull((*p).pInSavepoint, pgno as u32_0)
            {
                i = i + 1 as ::core::ffi::c_int;
                while i < (*pPager).nSavepoint {
                    (*(*pPager).aSavepoint.offset(i as isize)).bTruncateOnRelease = 0
                        as ::core::ffi::c_int;
                    i += 1;
                }
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn read32bits(
    mut fd: *mut sqlite3_file,
    mut offset: i64_0,
    mut pRes: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ac: [::core::ffi::c_uchar; 4] = [0; 4];
        let mut rc: ::core::ffi::c_int = sqlite3OsRead(
            fd,
            &raw mut ac as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 4]>() as ::core::ffi::c_int,
            offset,
        );
        if rc == SQLITE_OK {
            *pRes = sqlite3Get4byte(&raw mut ac as *mut ::core::ffi::c_uchar);
        }
        return rc;
    }
}
unsafe extern "C" fn write32bits(
    mut fd: *mut sqlite3_file,
    mut offset: i64_0,
    mut val: u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ac: [::core::ffi::c_char; 4] = [0; 4];
        sqlite3Put4byte(&raw mut ac as *mut ::core::ffi::c_char as *mut u8_0, val);
        return sqlite3OsWrite(
            fd,
            &raw mut ac as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as ::core::ffi::c_int,
            offset,
        );
    }
}
unsafe extern "C" fn pagerUnlockDb(
    mut pPager: *mut Pager,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !(*(*pPager).fd).pMethods.is_null() {
            rc = if (*pPager).noLock as ::core::ffi::c_int != 0 {
                SQLITE_OK
            } else {
                sqlite3OsUnlock((*pPager).fd, eLock)
            };
            if (*pPager).eLock as ::core::ffi::c_int != UNKNOWN_LOCK {
                (*pPager).eLock = eLock as u8_0;
            }
        }
        (*pPager).changeCountDone = (*pPager).tempFile;
        return rc;
    }
}
unsafe extern "C" fn pagerLockDb(
    mut pPager: *mut Pager,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if ((*pPager).eLock as ::core::ffi::c_int) < eLock
            || (*pPager).eLock as ::core::ffi::c_int == UNKNOWN_LOCK
        {
            rc = if (*pPager).noLock as ::core::ffi::c_int != 0 {
                SQLITE_OK
            } else {
                sqlite3OsLock((*pPager).fd, eLock)
            };
            if rc == SQLITE_OK
                && ((*pPager).eLock as ::core::ffi::c_int != UNKNOWN_LOCK
                    || eLock == EXCLUSIVE_LOCK)
            {
                (*pPager).eLock = eLock as u8_0;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn jrnlBufferSize(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    unsafe {
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn readSuperJournal(
    mut pJrnl: *mut sqlite3_file,
    mut zSuper: *mut ::core::ffi::c_char,
    mut nSuper: u64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut len: u32_0 = 0;
        let mut szJ: i64_0 = 0;
        let mut cksum: u32_0 = 0;
        let mut u: u32_0 = 0;
        let mut aMagic: [::core::ffi::c_uchar; 8] = [0; 8];
        *zSuper.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32
            as ::core::ffi::c_char;
        rc = sqlite3OsFileSize(pJrnl, &raw mut szJ);
        if SQLITE_OK != rc || szJ < 16 as ::core::ffi::c_longlong
            || {
                rc = read32bits(pJrnl, szJ - 16 as i64_0, &raw mut len);
                SQLITE_OK != rc
            } || len as ::core::ffi::c_ulonglong >= nSuper
            || len as ::core::ffi::c_longlong
                > szJ as ::core::ffi::c_longlong - 16 as ::core::ffi::c_longlong
            || len == 0 as u32_0
            || {
                rc = read32bits(pJrnl, szJ - 12 as i64_0, &raw mut cksum);
                SQLITE_OK != rc
            }
            || {
                rc = sqlite3OsRead(
                    pJrnl,
                    &raw mut aMagic as *mut ::core::ffi::c_uchar
                        as *mut ::core::ffi::c_void,
                    8 as ::core::ffi::c_int,
                    szJ - 8 as i64_0,
                );
                SQLITE_OK != rc
            }
            || memcmp(
                &raw mut aMagic as *mut ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                &raw const aJournalMagic as *const ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                8 as size_t,
            ) != 0
            || {
                rc = sqlite3OsRead(
                    pJrnl,
                    zSuper as *mut ::core::ffi::c_void,
                    len as ::core::ffi::c_int,
                    szJ - 16 as i64_0 - len as i64_0,
                );
                SQLITE_OK != rc
            }
        {
            return rc;
        }
        u = 0 as u32_0;
        while u < len {
            cksum = (cksum as ::core::ffi::c_uint)
                .wrapping_sub(*zSuper.offset(u as isize) as ::core::ffi::c_uint) as u32_0
                as u32_0;
            u = u.wrapping_add(1);
        }
        if cksum != 0 {
            len = 0 as u32_0;
        }
        *zSuper.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
        *zSuper.offset(len.wrapping_add(1 as u32_0) as isize) = '\0' as i32
            as ::core::ffi::c_char;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn journalHdrOffset(mut pPager: *mut Pager) -> i64_0 {
    unsafe {
        let mut offset: i64_0 = 0 as i64_0;
        let mut c: i64_0 = (*pPager).journalOff;
        if c != 0 {
            offset = (((c as ::core::ffi::c_longlong - 1 as ::core::ffi::c_longlong)
                / (*pPager).sectorSize as ::core::ffi::c_longlong
                + 1 as ::core::ffi::c_longlong)
                * (*pPager).sectorSize as ::core::ffi::c_longlong) as i64_0;
        }
        return offset;
    }
}
unsafe extern "C" fn zeroJournalHdr(
    mut pPager: *mut Pager,
    mut doTruncate: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).journalOff != 0 {
            let iLimit: i64_0 = (*pPager).journalSizeLimit;
            if doTruncate != 0 || iLimit == 0 as ::core::ffi::c_longlong {
                rc = sqlite3OsTruncate((*pPager).jfd, 0 as i64_0);
            } else {
                static mut zeroHdr: [::core::ffi::c_char; 28] = [
                    0 as ::core::ffi::c_int as ::core::ffi::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ];
                rc = sqlite3OsWrite(
                    (*pPager).jfd,
                    &raw const zeroHdr as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[::core::ffi::c_char; 28]>()
                        as ::core::ffi::c_int,
                    0 as i64_0,
                );
            }
            if rc == SQLITE_OK && (*pPager).noSync == 0 {
                rc = sqlite3OsSync(
                    (*pPager).jfd,
                    SQLITE_SYNC_DATAONLY | (*pPager).syncFlags as ::core::ffi::c_int,
                );
            }
            if rc == SQLITE_OK && iLimit > 0 as ::core::ffi::c_longlong {
                let mut sz: i64_0 = 0;
                rc = sqlite3OsFileSize((*pPager).jfd, &raw mut sz);
                if rc == SQLITE_OK && sz > iLimit {
                    rc = sqlite3OsTruncate((*pPager).jfd, iLimit);
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn writeJournalHdr(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zHeader: *mut ::core::ffi::c_char = (*pPager).pTmpSpace;
        let mut nHeader: u32_0 = (*pPager).pageSize as u32_0;
        let mut nWrite: u32_0 = 0;
        let mut ii: ::core::ffi::c_int = 0;
        if nHeader > (*pPager).sectorSize {
            nHeader = (*pPager).sectorSize;
        }
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pPager).nSavepoint {
            if (*(*pPager).aSavepoint.offset(ii as isize)).iHdrOffset
                == 0 as ::core::ffi::c_longlong
            {
                (*(*pPager).aSavepoint.offset(ii as isize)).iHdrOffset = (*pPager)
                    .journalOff;
            }
            ii += 1;
        }
        (*pPager).journalOff = journalHdrOffset(pPager);
        (*pPager).journalHdr = (*pPager).journalOff;
        if (*pPager).noSync as ::core::ffi::c_int != 0
            || (*pPager).journalMode as ::core::ffi::c_int == PAGER_JOURNALMODE_MEMORY
            || sqlite3OsDeviceCharacteristics((*pPager).fd) & SQLITE_IOCAP_SAFE_APPEND
                != 0
        {
            memcpy(
                zHeader as *mut ::core::ffi::c_void,
                &raw const aJournalMagic as *const ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as size_t,
            );
            sqlite3Put4byte(
                zHeader
                    .offset(::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as isize)
                    as *mut ::core::ffi::c_char as *mut u8_0,
                0xffffffff as u32_0,
            );
        } else {
            memset(
                zHeader as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as size_t)
                    .wrapping_add(4 as size_t),
            );
        }
        if (*pPager).journalMode as ::core::ffi::c_int != PAGER_JOURNALMODE_MEMORY {
            sqlite3_randomness(
                ::core::mem::size_of::<u32_0>() as ::core::ffi::c_int,
                &raw mut (*pPager).cksumInit as *mut ::core::ffi::c_void,
            );
        }
        sqlite3Put4byte(
            zHeader
                .offset(
                    (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize)
                        .wrapping_add(4 as usize) as isize,
                ) as *mut ::core::ffi::c_char as *mut u8_0,
            (*pPager).cksumInit,
        );
        sqlite3Put4byte(
            zHeader
                .offset(
                    (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize)
                        .wrapping_add(8 as usize) as isize,
                ) as *mut ::core::ffi::c_char as *mut u8_0,
            (*pPager).dbOrigSize as u32_0,
        );
        sqlite3Put4byte(
            zHeader
                .offset(
                    (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize)
                        .wrapping_add(12 as usize) as isize,
                ) as *mut ::core::ffi::c_char as *mut u8_0,
            (*pPager).sectorSize,
        );
        sqlite3Put4byte(
            zHeader
                .offset(
                    (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize)
                        .wrapping_add(16 as usize) as isize,
                ) as *mut ::core::ffi::c_char as *mut u8_0,
            (*pPager).pageSize as u32_0,
        );
        memset(
            zHeader
                .offset(
                    (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize)
                        .wrapping_add(20 as usize) as isize,
                ) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (nHeader as size_t)
                .wrapping_sub(
                    (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as size_t)
                        .wrapping_add(20 as size_t),
                ),
        );
        nWrite = 0 as u32_0;
        while rc == SQLITE_OK && nWrite < (*pPager).sectorSize {
            rc = sqlite3OsWrite(
                (*pPager).jfd,
                zHeader as *const ::core::ffi::c_void,
                nHeader as ::core::ffi::c_int,
                (*pPager).journalOff,
            );
            (*pPager).journalOff += nHeader as ::core::ffi::c_longlong;
            nWrite = (nWrite as ::core::ffi::c_uint)
                .wrapping_add(nHeader as ::core::ffi::c_uint) as u32_0 as u32_0;
        }
        return rc;
    }
}
unsafe extern "C" fn readJournalHdr(
    mut pPager: *mut Pager,
    mut isHot: ::core::ffi::c_int,
    mut journalSize: i64_0,
    mut pNRec: *mut u32_0,
    mut pDbSize: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut aMagic: [::core::ffi::c_uchar; 8] = [0; 8];
        let mut iHdrOff: i64_0 = 0;
        (*pPager).journalOff = journalHdrOffset(pPager);
        if (*pPager).journalOff as ::core::ffi::c_longlong
            + (*pPager).sectorSize as ::core::ffi::c_longlong > journalSize
        {
            return SQLITE_DONE;
        }
        iHdrOff = (*pPager).journalOff;
        if isHot != 0 || iHdrOff != (*pPager).journalHdr {
            rc = sqlite3OsRead(
                (*pPager).jfd,
                &raw mut aMagic as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>()
                    as ::core::ffi::c_int,
                iHdrOff,
            );
            if rc != 0 {
                return rc;
            }
            if memcmp(
                &raw mut aMagic as *mut ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                &raw const aJournalMagic as *const ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as size_t,
            ) != 0 as ::core::ffi::c_int
            {
                return SQLITE_DONE;
            }
        }
        rc = read32bits((*pPager).jfd, iHdrOff + 8 as i64_0, pNRec);
        if SQLITE_OK != rc
            || {
                rc = read32bits(
                    (*pPager).jfd,
                    iHdrOff + 12 as i64_0,
                    &raw mut (*pPager).cksumInit,
                );
                SQLITE_OK != rc
            }
            || {
                rc = read32bits((*pPager).jfd, iHdrOff + 16 as i64_0, pDbSize);
                SQLITE_OK != rc
            }
        {
            return rc;
        }
        if (*pPager).journalOff == 0 as ::core::ffi::c_longlong {
            let mut iPageSize: u32_0 = 0;
            let mut iSectorSize: u32_0 = 0;
            rc = read32bits((*pPager).jfd, iHdrOff + 20 as i64_0, &raw mut iSectorSize);
            if SQLITE_OK != rc
                || {
                    rc = read32bits(
                        (*pPager).jfd,
                        iHdrOff + 24 as i64_0,
                        &raw mut iPageSize,
                    );
                    SQLITE_OK != rc
                }
            {
                return rc;
            }
            if iPageSize == 0 as u32_0 {
                iPageSize = (*pPager).pageSize as u32_0;
            }
            if iPageSize < 512 as u32_0 || iSectorSize < 32 as u32_0
                || iPageSize > SQLITE_MAX_PAGE_SIZE as u32_0
                || iSectorSize > MAX_SECTOR_SIZE as u32_0
                || iPageSize.wrapping_sub(1 as u32_0) & iPageSize != 0 as u32_0
                || iSectorSize.wrapping_sub(1 as u32_0) & iSectorSize != 0 as u32_0
            {
                return SQLITE_DONE;
            }
            rc = sqlite3PagerSetPagesize(
                pPager,
                &raw mut iPageSize,
                -1 as ::core::ffi::c_int,
            );
            (*pPager).sectorSize = iSectorSize;
        }
        (*pPager).journalOff += (*pPager).sectorSize as ::core::ffi::c_longlong;
        return rc;
    }
}
unsafe extern "C" fn writeSuperJournal(
    mut pPager: *mut Pager,
    mut zSuper: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut nSuper: ::core::ffi::c_int = 0;
        let mut iHdrOff: i64_0 = 0;
        let mut jrnlSize: i64_0 = 0;
        let mut cksum: u32_0 = 0 as u32_0;
        if zSuper.is_null()
            || (*pPager).journalMode as ::core::ffi::c_int == PAGER_JOURNALMODE_MEMORY
            || (*(*pPager).jfd).pMethods.is_null()
        {
            return SQLITE_OK;
        }
        (*pPager).setSuper = 1 as u8_0;
        nSuper = 0 as ::core::ffi::c_int;
        while *zSuper.offset(nSuper as isize) != 0 {
            cksum = (cksum as ::core::ffi::c_uint)
                .wrapping_add(*zSuper.offset(nSuper as isize) as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            nSuper += 1;
        }
        if (*pPager).fullSync != 0 {
            (*pPager).journalOff = journalHdrOffset(pPager);
        }
        iHdrOff = (*pPager).journalOff;
        rc = write32bits((*pPager).jfd, iHdrOff, (*pPager).lckPgno as u32_0);
        if 0 as ::core::ffi::c_int != rc
            || {
                rc = sqlite3OsWrite(
                    (*pPager).jfd,
                    zSuper as *const ::core::ffi::c_void,
                    nSuper,
                    iHdrOff + 4 as i64_0,
                );
                0 as ::core::ffi::c_int != rc
            }
            || {
                rc = write32bits(
                    (*pPager).jfd,
                    iHdrOff + 4 as i64_0 + nSuper as i64_0,
                    nSuper as u32_0,
                );
                0 as ::core::ffi::c_int != rc
            }
            || {
                rc = write32bits(
                    (*pPager).jfd,
                    iHdrOff + 4 as i64_0 + nSuper as i64_0 + 4 as i64_0,
                    cksum,
                );
                0 as ::core::ffi::c_int != rc
            }
            || {
                rc = sqlite3OsWrite(
                    (*pPager).jfd,
                    &raw const aJournalMagic as *const ::core::ffi::c_uchar
                        as *const ::core::ffi::c_void,
                    8 as ::core::ffi::c_int,
                    iHdrOff + 4 as i64_0 + nSuper as i64_0 + 8 as i64_0,
                );
                0 as ::core::ffi::c_int != rc
            }
        {
            return rc;
        }
        (*pPager).journalOff
            += (nSuper + 20 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
        rc = sqlite3OsFileSize((*pPager).jfd, &raw mut jrnlSize);
        if SQLITE_OK == rc && jrnlSize > (*pPager).journalOff {
            rc = sqlite3OsTruncate((*pPager).jfd, (*pPager).journalOff);
        }
        return rc;
    }
}
unsafe extern "C" fn pager_reset(mut pPager: *mut Pager) {
    unsafe {
        (*pPager).iDataVersion = (*pPager).iDataVersion.wrapping_add(1);
        sqlite3BackupRestart((*pPager).pBackup);
        sqlite3PcacheClear((*pPager).pPCache);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerDataVersion(mut pPager: *mut Pager) -> u32_0 {
    unsafe {
        return (*pPager).iDataVersion;
    }
}
unsafe extern "C" fn releaseAllSavepoints(mut pPager: *mut Pager) {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pPager).nSavepoint {
            sqlite3BitvecDestroy(
                (*(*pPager).aSavepoint.offset(ii as isize)).pInSavepoint,
            );
            ii += 1;
        }
        if (*pPager).exclusiveMode == 0 || sqlite3JournalIsInMemory((*pPager).sjfd) != 0
        {
            sqlite3OsClose((*pPager).sjfd);
        }
        sqlite3_free((*pPager).aSavepoint as *mut ::core::ffi::c_void);
        (*pPager).aSavepoint = ::core::ptr::null_mut::<PagerSavepoint>();
        (*pPager).nSavepoint = 0 as ::core::ffi::c_int;
        (*pPager).nSubRec = 0 as u32_0;
    }
}
unsafe extern "C" fn addToSavepointBitvecs(
    mut pPager: *mut Pager,
    mut pgno: Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pPager).nSavepoint {
            let mut p: *mut PagerSavepoint = (*pPager).aSavepoint.offset(ii as isize)
                as *mut PagerSavepoint;
            if pgno <= (*p).nOrig {
                rc |= sqlite3BitvecSet((*p).pInSavepoint, pgno as u32_0);
            }
            ii += 1;
        }
        return rc;
    }
}
unsafe extern "C" fn pager_unlock(mut pPager: *mut Pager) {
    unsafe {
        sqlite3BitvecDestroy((*pPager).pInJournal);
        (*pPager).pInJournal = ::core::ptr::null_mut::<Bitvec>();
        releaseAllSavepoints(pPager);
        if !(*pPager).pWal.is_null() {
            if (*pPager).eState as ::core::ffi::c_int == PAGER_ERROR {
                sqlite3WalEndWriteTransaction((*pPager).pWal);
            }
            sqlite3WalEndReadTransaction((*pPager).pWal);
            (*pPager).eState = PAGER_OPEN as u8_0;
        } else if (*pPager).exclusiveMode == 0 {
            let mut rc: ::core::ffi::c_int = 0;
            let mut iDc: ::core::ffi::c_int = if !(*(*pPager).fd).pMethods.is_null() {
                sqlite3OsDeviceCharacteristics((*pPager).fd)
            } else {
                0 as ::core::ffi::c_int
            };
            if 0 as ::core::ffi::c_int == iDc & SQLITE_IOCAP_UNDELETABLE_WHEN_OPEN
                || 1 as ::core::ffi::c_int
                    != (*pPager).journalMode as ::core::ffi::c_int
                        & 5 as ::core::ffi::c_int
            {
                sqlite3OsClose((*pPager).jfd);
            }
            rc = pagerUnlockDb(pPager, NO_LOCK);
            if rc != SQLITE_OK && (*pPager).eState as ::core::ffi::c_int == PAGER_ERROR {
                (*pPager).eLock = UNKNOWN_LOCK as u8_0;
            }
            (*pPager).eState = PAGER_OPEN as u8_0;
        }
        if (*pPager).errCode != 0 {
            if (*pPager).tempFile as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                pager_reset(pPager);
                (*pPager).changeCountDone = 0 as u8_0;
                (*pPager).eState = PAGER_OPEN as u8_0;
            } else {
                (*pPager).eState = (if !(*(*pPager).jfd).pMethods.is_null() {
                    PAGER_OPEN
                } else {
                    PAGER_READER
                }) as u8_0;
            }
            if (*pPager).bUseFetch != 0 {
                sqlite3OsUnfetch(
                    (*pPager).fd,
                    0 as i64_0,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
            }
            (*pPager).errCode = SQLITE_OK;
            setGetterMethod(pPager);
        }
        (*pPager).journalOff = 0 as i64_0;
        (*pPager).journalHdr = 0 as i64_0;
        (*pPager).setSuper = 0 as u8_0;
    }
}
unsafe extern "C" fn pager_error(
    mut pPager: *mut Pager,
    mut rc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc2: ::core::ffi::c_int = rc & 0xff as ::core::ffi::c_int;
        if rc2 == SQLITE_FULL || rc2 == SQLITE_IOERR {
            (*pPager).errCode = rc;
            (*pPager).eState = PAGER_ERROR as u8_0;
            setGetterMethod(pPager);
        }
        return rc;
    }
}
unsafe extern "C" fn pagerFlushOnCommit(
    mut pPager: *mut Pager,
    mut bCommit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pPager).tempFile as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if bCommit == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*pPager).fd).pMethods.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        return (sqlite3PCachePercentDirty((*pPager).pPCache) >= 25 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn pager_end_transaction(
    mut pPager: *mut Pager,
    mut hasSuper: ::core::ffi::c_int,
    mut bCommit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut rc2: ::core::ffi::c_int = SQLITE_OK;
        if ((*pPager).eState as ::core::ffi::c_int) < PAGER_WRITER_LOCKED
            && ((*pPager).eLock as ::core::ffi::c_int) < RESERVED_LOCK
        {
            return SQLITE_OK;
        }
        releaseAllSavepoints(pPager);
        if !(*(*pPager).jfd).pMethods.is_null() {
            if sqlite3JournalIsInMemory((*pPager).jfd) != 0 {
                sqlite3OsClose((*pPager).jfd);
            } else if (*pPager).journalMode as ::core::ffi::c_int
                == PAGER_JOURNALMODE_TRUNCATE
            {
                if (*pPager).journalOff == 0 as ::core::ffi::c_longlong {
                    rc = SQLITE_OK;
                } else {
                    rc = sqlite3OsTruncate((*pPager).jfd, 0 as i64_0);
                    if rc == SQLITE_OK && (*pPager).fullSync as ::core::ffi::c_int != 0 {
                        rc = sqlite3OsSync(
                            (*pPager).jfd,
                            (*pPager).syncFlags as ::core::ffi::c_int,
                        );
                    }
                }
                (*pPager).journalOff = 0 as i64_0;
            } else if (*pPager).journalMode as ::core::ffi::c_int
                == PAGER_JOURNALMODE_PERSIST
                || (*pPager).exclusiveMode as ::core::ffi::c_int != 0
                    && ((*pPager).journalMode as ::core::ffi::c_int)
                        < PAGER_JOURNALMODE_WAL
            {
                rc = zeroJournalHdr(
                    pPager,
                    (hasSuper != 0 || (*pPager).tempFile as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int,
                );
                (*pPager).journalOff = 0 as i64_0;
            } else {
                let mut bDelete: ::core::ffi::c_int = ((*pPager).tempFile == 0)
                    as ::core::ffi::c_int;
                sqlite3OsClose((*pPager).jfd);
                if bDelete != 0 {
                    rc = sqlite3OsDelete(
                        (*pPager).pVfs,
                        (*pPager).zJournal,
                        (*pPager).extraSync as ::core::ffi::c_int,
                    );
                }
            }
        }
        sqlite3BitvecDestroy((*pPager).pInJournal);
        (*pPager).pInJournal = ::core::ptr::null_mut::<Bitvec>();
        (*pPager).nRec = 0 as ::core::ffi::c_int;
        if rc == SQLITE_OK {
            if (*pPager).memDb as ::core::ffi::c_int != 0
                || pagerFlushOnCommit(pPager, bCommit) != 0
            {
                sqlite3PcacheCleanAll((*pPager).pPCache);
            } else {
                sqlite3PcacheClearWritable((*pPager).pPCache);
            }
            sqlite3PcacheTruncate((*pPager).pPCache, (*pPager).dbSize);
        }
        if !(*pPager).pWal.is_null() {
            rc2 = sqlite3WalEndWriteTransaction((*pPager).pWal);
        } else if rc == SQLITE_OK && bCommit != 0
            && (*pPager).dbFileSize > (*pPager).dbSize
        {
            rc = pager_truncate(pPager, (*pPager).dbSize);
        }
        if rc == SQLITE_OK && bCommit != 0 {
            rc = sqlite3OsFileControl(
                (*pPager).fd,
                SQLITE_FCNTL_COMMIT_PHASETWO,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
            if rc == SQLITE_NOTFOUND {
                rc = SQLITE_OK;
            }
        }
        if (*pPager).exclusiveMode == 0
            && ((*pPager).pWal.is_null()
                || sqlite3WalExclusiveMode((*pPager).pWal, 0 as ::core::ffi::c_int) != 0)
        {
            rc2 = pagerUnlockDb(pPager, SHARED_LOCK);
        }
        (*pPager).eState = PAGER_READER as u8_0;
        (*pPager).setSuper = 0 as u8_0;
        return if rc == SQLITE_OK { rc2 } else { rc };
    }
}
unsafe extern "C" fn pagerUnlockAndRollback(mut pPager: *mut Pager) {
    unsafe {
        if (*pPager).eState as ::core::ffi::c_int != PAGER_ERROR
            && (*pPager).eState as ::core::ffi::c_int != PAGER_OPEN
        {
            if (*pPager).eState as ::core::ffi::c_int >= PAGER_WRITER_LOCKED {
                sqlite3BeginBenignMalloc();
                sqlite3PagerRollback(pPager);
                sqlite3EndBenignMalloc();
            } else if (*pPager).exclusiveMode == 0 {
                pager_end_transaction(
                    pPager,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
            }
        } else if (*pPager).eState as ::core::ffi::c_int == PAGER_ERROR
            && (*pPager).journalMode as ::core::ffi::c_int == PAGER_JOURNALMODE_MEMORY
            && !(*(*pPager).jfd).pMethods.is_null()
        {
            let mut errCode: ::core::ffi::c_int = (*pPager).errCode;
            let mut eLock: u8_0 = (*pPager).eLock;
            (*pPager).eState = PAGER_OPEN as u8_0;
            (*pPager).errCode = SQLITE_OK;
            (*pPager).eLock = EXCLUSIVE_LOCK as u8_0;
            pager_playback(pPager, 1 as ::core::ffi::c_int);
            (*pPager).errCode = errCode;
            (*pPager).eLock = eLock;
        }
        pager_unlock(pPager);
    }
}
unsafe extern "C" fn pager_cksum(
    mut pPager: *mut Pager,
    mut aData: *const u8_0,
) -> u32_0 {
    unsafe {
        let mut cksum: u32_0 = (*pPager).cksumInit;
        let mut i: ::core::ffi::c_int = ((*pPager).pageSize as ::core::ffi::c_longlong
            - 200 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        while i > 0 as ::core::ffi::c_int {
            cksum = (cksum as ::core::ffi::c_uint)
                .wrapping_add(*aData.offset(i as isize) as ::core::ffi::c_uint) as u32_0
                as u32_0;
            i -= 200 as ::core::ffi::c_int;
        }
        return cksum;
    }
}
unsafe extern "C" fn pager_playback_one_page(
    mut pPager: *mut Pager,
    mut pOffset: *mut i64_0,
    mut pDone: *mut Bitvec,
    mut isMainJrnl: ::core::ffi::c_int,
    mut isSavepnt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pPg: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        let mut pgno: Pgno = 0;
        let mut cksum: u32_0 = 0;
        let mut aData: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut jfd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut isSynced: ::core::ffi::c_int = 0;
        aData = (*pPager).pTmpSpace;
        jfd = if isMainJrnl != 0 { (*pPager).jfd } else { (*pPager).sjfd };
        rc = read32bits(jfd, *pOffset, &raw mut pgno);
        if rc != SQLITE_OK {
            return rc;
        }
        rc = sqlite3OsRead(
            jfd,
            aData as *mut u8_0 as *mut ::core::ffi::c_void,
            (*pPager).pageSize as ::core::ffi::c_int,
            *pOffset + 4 as i64_0,
        );
        if rc != SQLITE_OK {
            return rc;
        }
        *pOffset
            += (*pPager).pageSize as ::core::ffi::c_longlong
                + 4 as ::core::ffi::c_longlong
                + (isMainJrnl * 4 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
        if pgno == 0 as Pgno || pgno == (*pPager).lckPgno {
            return SQLITE_DONE;
        }
        if pgno > (*pPager).dbSize || sqlite3BitvecTest(pDone, pgno as u32_0) != 0 {
            return SQLITE_OK;
        }
        if isMainJrnl != 0 {
            rc = read32bits(jfd, *pOffset - 4 as i64_0, &raw mut cksum);
            if rc != 0 {
                return rc;
            }
            if isSavepnt == 0 && pager_cksum(pPager, aData as *mut u8_0) != cksum {
                return SQLITE_DONE;
            }
        }
        if !pDone.is_null()
            && {
                rc = sqlite3BitvecSet(pDone, pgno as u32_0);
                rc != SQLITE_OK
            }
        {
            return rc;
        }
        if pgno == 1 as Pgno
            && (*pPager).nReserve as ::core::ffi::c_int
                != *(aData as *mut u8_0).offset(20 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
        {
            (*pPager).nReserve = *(aData as *mut u8_0)
                .offset(20 as ::core::ffi::c_int as isize) as i16_0;
        }
        if !(*pPager).pWal.is_null() {
            pPg = ::core::ptr::null_mut::<PgHdr>();
        } else {
            pPg = sqlite3PagerLookup(pPager, pgno) as *mut PgHdr;
        }
        if isMainJrnl != 0 {
            isSynced = ((*pPager).noSync as ::core::ffi::c_int != 0
                || *pOffset <= (*pPager).journalHdr) as ::core::ffi::c_int;
        } else {
            isSynced = (pPg.is_null()
                || 0 as ::core::ffi::c_int
                    == (*pPg).flags as ::core::ffi::c_int & PGHDR_NEED_SYNC)
                as ::core::ffi::c_int;
        }
        if !(*(*pPager).fd).pMethods.is_null()
            && ((*pPager).eState as ::core::ffi::c_int >= PAGER_WRITER_DBMOD
                || (*pPager).eState as ::core::ffi::c_int == PAGER_OPEN) && isSynced != 0
        {
            let mut ofst: i64_0 = pgno.wrapping_sub(1 as Pgno) as i64_0
                * (*pPager).pageSize;
            rc = sqlite3OsWrite(
                (*pPager).fd,
                aData as *mut u8_0 as *const ::core::ffi::c_void,
                (*pPager).pageSize as ::core::ffi::c_int,
                ofst,
            );
            if pgno > (*pPager).dbFileSize {
                (*pPager).dbFileSize = pgno;
            }
            if !(*pPager).pBackup.is_null() {
                sqlite3BackupUpdate((*pPager).pBackup, pgno, aData as *mut u8_0);
            }
        } else if isMainJrnl == 0 && pPg.is_null() {
            (*pPager).doNotSpill = ((*pPager).doNotSpill as ::core::ffi::c_int
                | SPILLFLAG_ROLLBACK) as u8_0;
            rc = sqlite3PagerGet(pPager, pgno, &raw mut pPg, 1 as ::core::ffi::c_int);
            (*pPager).doNotSpill = ((*pPager).doNotSpill as ::core::ffi::c_int
                & !SPILLFLAG_ROLLBACK) as u8_0;
            if rc != SQLITE_OK {
                return rc;
            }
            sqlite3PcacheMakeDirty(pPg);
        }
        if !pPg.is_null() {
            let mut pData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            pData = (*pPg).pData;
            memcpy(
                pData,
                aData as *mut u8_0 as *const ::core::ffi::c_void,
                (*pPager).pageSize as size_t,
            );
            (*pPager).xReiniter.expect("non-null function pointer")(pPg as *mut DbPage);
            if pgno == 1 as Pgno {
                memcpy(
                    &raw mut (*pPager).dbFileVers as *mut ::core::ffi::c_void,
                    (pData as *mut u8_0).offset(24 as ::core::ffi::c_int as isize)
                        as *mut u8_0 as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
                );
            }
            sqlite3PcacheRelease(pPg);
        }
        return rc;
    }
}
unsafe extern "C" fn pager_delsuper(
    mut pPager: *mut Pager,
    mut zSuper: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pVfs: *mut sqlite3_vfs = (*pPager).pVfs;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pSuper: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut pJournal: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut zSuperJournal: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nSuperJournal: i64_0 = 0;
        let mut zJournal: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zSuperPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zFree: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nSuperPtr: i64_0 = 0;
        pSuper = sqlite3MallocZero((2 as i64_0 * (*pVfs).szOsFile as i64_0) as u64_0)
            as *mut sqlite3_file;
        if pSuper.is_null() {
            rc = SQLITE_NOMEM_BKPT;
            pJournal = ::core::ptr::null_mut::<sqlite3_file>();
        } else {
            let flags: ::core::ffi::c_int = SQLITE_OPEN_READONLY
                | SQLITE_OPEN_SUPER_JOURNAL;
            rc = sqlite3OsOpen(
                pVfs,
                zSuper,
                pSuper,
                flags,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            pJournal = (pSuper as *mut u8_0).offset((*pVfs).szOsFile as isize)
                as *mut sqlite3_file;
        }
        if !(rc != SQLITE_OK) {
            rc = sqlite3OsFileSize(pSuper, &raw mut nSuperJournal);
            if !(rc != SQLITE_OK) {
                nSuperPtr = 1 as i64_0 + (*pVfs).mxPathname as i64_0;
                zFree = sqlite3Malloc(
                    (4 as ::core::ffi::c_longlong
                        + nSuperJournal as ::core::ffi::c_longlong
                        + nSuperPtr as ::core::ffi::c_longlong
                        + 2 as ::core::ffi::c_longlong) as u64_0,
                ) as *mut ::core::ffi::c_char;
                if zFree.is_null() {
                    rc = SQLITE_NOMEM_BKPT;
                } else {
                    let ref mut c2rust_fresh0 = *zFree
                        .offset(3 as ::core::ffi::c_int as isize);
                    *c2rust_fresh0 = 0 as ::core::ffi::c_char;
                    let ref mut c2rust_fresh1 = *zFree
                        .offset(2 as ::core::ffi::c_int as isize);
                    *c2rust_fresh1 = *c2rust_fresh0;
                    let ref mut c2rust_fresh2 = *zFree
                        .offset(1 as ::core::ffi::c_int as isize);
                    *c2rust_fresh2 = *c2rust_fresh1;
                    *zFree.offset(0 as ::core::ffi::c_int as isize) = *c2rust_fresh2;
                    zSuperJournal = zFree.offset(4 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char;
                    zSuperPtr = zSuperJournal
                        .offset(
                            (nSuperJournal as ::core::ffi::c_longlong
                                + 2 as ::core::ffi::c_longlong) as isize,
                        ) as *mut ::core::ffi::c_char;
                    rc = sqlite3OsRead(
                        pSuper,
                        zSuperJournal as *mut ::core::ffi::c_void,
                        nSuperJournal as ::core::ffi::c_int,
                        0 as i64_0,
                    );
                    if !(rc != SQLITE_OK) {
                        *zSuperJournal.offset(nSuperJournal as isize) = 0
                            as ::core::ffi::c_char;
                        *zSuperJournal
                            .offset(
                                (nSuperJournal as ::core::ffi::c_longlong
                                    + 1 as ::core::ffi::c_longlong) as isize,
                            ) = 0 as ::core::ffi::c_char;
                        zJournal = zSuperJournal;
                        loop {
                            if !((zJournal.offset_from(zSuperJournal)
                                as ::core::ffi::c_long as ::core::ffi::c_longlong)
                                < nSuperJournal)
                            {
                                c2rust_current_block = 6450636197030046351;
                                break;
                            }
                            let mut exists: ::core::ffi::c_int = 0;
                            rc = sqlite3OsAccess(
                                pVfs,
                                zJournal,
                                SQLITE_ACCESS_EXISTS,
                                &raw mut exists,
                            );
                            if rc != SQLITE_OK {
                                c2rust_current_block = 14980566233080080208;
                                break;
                            }
                            if exists != 0 {
                                let mut c: ::core::ffi::c_int = 0;
                                let mut flags_0: ::core::ffi::c_int = SQLITE_OPEN_READONLY
                                    | SQLITE_OPEN_SUPER_JOURNAL;
                                rc = sqlite3OsOpen(
                                    pVfs,
                                    zJournal,
                                    pJournal,
                                    flags_0,
                                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                );
                                if rc != SQLITE_OK {
                                    c2rust_current_block = 14980566233080080208;
                                    break;
                                }
                                rc = readSuperJournal(
                                    pJournal,
                                    zSuperPtr,
                                    nSuperPtr as u64_0,
                                );
                                sqlite3OsClose(pJournal);
                                if rc != SQLITE_OK {
                                    c2rust_current_block = 14980566233080080208;
                                    break;
                                }
                                c = (*zSuperPtr.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                    && strcmp(zSuperPtr, zSuper) == 0 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int;
                                if c != 0 {
                                    c2rust_current_block = 14980566233080080208;
                                    break;
                                }
                            }
                            zJournal = zJournal
                                .offset(
                                    (sqlite3Strlen30(zJournal) + 1 as ::core::ffi::c_int)
                                        as isize,
                                );
                        }
                        match c2rust_current_block {
                            14980566233080080208 => {}
                            _ => {
                                sqlite3OsClose(pSuper);
                                rc = sqlite3OsDelete(pVfs, zSuper, 0 as ::core::ffi::c_int);
                            }
                        }
                    }
                }
            }
        }
        sqlite3_free(zFree as *mut ::core::ffi::c_void);
        if !pSuper.is_null() {
            sqlite3OsClose(pSuper);
            sqlite3_free(pSuper as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn pager_truncate(
    mut pPager: *mut Pager,
    mut nPage: Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !(*(*pPager).fd).pMethods.is_null()
            && ((*pPager).eState as ::core::ffi::c_int >= PAGER_WRITER_DBMOD
                || (*pPager).eState as ::core::ffi::c_int == PAGER_OPEN)
        {
            let mut currentSize: i64_0 = 0;
            let mut newSize: i64_0 = 0;
            let mut szPage: ::core::ffi::c_int = (*pPager).pageSize
                as ::core::ffi::c_int;
            rc = sqlite3OsFileSize((*pPager).fd, &raw mut currentSize);
            newSize = szPage as i64_0 * nPage as i64_0;
            if rc == SQLITE_OK && currentSize != newSize {
                if currentSize > newSize {
                    rc = sqlite3OsTruncate((*pPager).fd, newSize);
                } else if currentSize as ::core::ffi::c_longlong
                    + szPage as ::core::ffi::c_longlong <= newSize
                {
                    let mut pTmp: *mut ::core::ffi::c_char = (*pPager).pTmpSpace;
                    memset(
                        pTmp as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        szPage as size_t,
                    );
                    sqlite3OsFileControlHint(
                        (*pPager).fd,
                        SQLITE_FCNTL_SIZE_HINT,
                        &raw mut newSize as *mut ::core::ffi::c_void,
                    );
                    rc = sqlite3OsWrite(
                        (*pPager).fd,
                        pTmp as *const ::core::ffi::c_void,
                        szPage,
                        newSize - szPage as i64_0,
                    );
                }
                if rc == SQLITE_OK {
                    (*pPager).dbFileSize = nPage;
                }
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SectorSize(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iRet: ::core::ffi::c_int = sqlite3OsSectorSize(pFile);
        if iRet < 32 as ::core::ffi::c_int {
            iRet = 512 as ::core::ffi::c_int;
        } else if iRet > MAX_SECTOR_SIZE {
            iRet = MAX_SECTOR_SIZE;
        }
        return iRet;
    }
}
unsafe extern "C" fn setSectorSize(mut pPager: *mut Pager) {
    unsafe {
        if (*pPager).tempFile as ::core::ffi::c_int != 0
            || sqlite3OsDeviceCharacteristics((*pPager).fd)
                & SQLITE_IOCAP_POWERSAFE_OVERWRITE != 0 as ::core::ffi::c_int
        {
            (*pPager).sectorSize = 512 as u32_0;
        } else {
            (*pPager).sectorSize = sqlite3SectorSize((*pPager).fd) as u32_0;
        };
    }
}
unsafe extern "C" fn pager_playback(
    mut pPager: *mut Pager,
    mut isHot: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pPager).pVfs;
        let mut szJ: i64_0 = 0;
        let mut nRec: u32_0 = 0;
        let mut u: u32_0 = 0;
        let mut mxPg: Pgno = 0 as Pgno;
        let mut rc: ::core::ffi::c_int = 0;
        let mut res: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut zSuper: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut needPagerReset: ::core::ffi::c_int = 0;
        let mut nPlayback: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut savedPageSize: u32_0 = (*pPager).pageSize as u32_0;
        rc = sqlite3OsFileSize((*pPager).jfd, &raw mut szJ);
        if !(rc != SQLITE_OK) {
            zSuper = (*pPager).pTmpSpace;
            rc = readSuperJournal(
                (*pPager).jfd,
                zSuper,
                (1 as i64_0 + (*(*pPager).pVfs).mxPathname as i64_0) as u64_0,
            );
            if rc == SQLITE_OK
                && *zSuper.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 0
            {
                rc = sqlite3OsAccess(pVfs, zSuper, SQLITE_ACCESS_EXISTS, &raw mut res);
            }
            zSuper = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !(rc != SQLITE_OK || res == 0) {
                (*pPager).journalOff = 0 as i64_0;
                needPagerReset = isHot;
                's_73: loop {
                    rc = readJournalHdr(
                        pPager,
                        isHot,
                        szJ,
                        &raw mut nRec,
                        &raw mut mxPg,
                    );
                    if rc != SQLITE_OK {
                        if rc == SQLITE_DONE {
                            rc = SQLITE_OK;
                        }
                        break;
                    } else {
                        if nRec == 0xffffffff as u32_0 {
                            nRec = ((szJ as ::core::ffi::c_longlong
                                - (*pPager).sectorSize as ::core::ffi::c_longlong)
                                / ((*pPager).pageSize as ::core::ffi::c_longlong
                                    + 8 as ::core::ffi::c_longlong)) as ::core::ffi::c_int
                                as u32_0;
                        }
                        if nRec == 0 as u32_0 && isHot == 0
                            && (*pPager).journalHdr as ::core::ffi::c_longlong
                                + (*pPager).sectorSize as ::core::ffi::c_longlong
                                == (*pPager).journalOff
                        {
                            nRec = ((szJ as ::core::ffi::c_longlong
                                - (*pPager).journalOff as ::core::ffi::c_longlong)
                                / ((*pPager).pageSize as ::core::ffi::c_longlong
                                    + 8 as ::core::ffi::c_longlong)) as ::core::ffi::c_int
                                as u32_0;
                        }
                        if (*pPager).journalOff
                            == (*pPager).sectorSize as ::core::ffi::c_longlong
                        {
                            rc = pager_truncate(pPager, mxPg);
                            if rc != SQLITE_OK {
                                break;
                            }
                            (*pPager).dbSize = mxPg;
                            if (*pPager).mxPgno < mxPg {
                                (*pPager).mxPgno = mxPg;
                            }
                        }
                        u = 0 as u32_0;
                        while u < nRec {
                            if needPagerReset != 0 {
                                pager_reset(pPager);
                                needPagerReset = 0 as ::core::ffi::c_int;
                            }
                            rc = pager_playback_one_page(
                                pPager,
                                &raw mut (*pPager).journalOff,
                                ::core::ptr::null_mut::<Bitvec>(),
                                1 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                            );
                            if rc == SQLITE_OK {
                                nPlayback += 1;
                                u = u.wrapping_add(1);
                            } else if rc == SQLITE_DONE {
                                (*pPager).journalOff = szJ;
                                break;
                            } else {
                                if !(rc == SQLITE_IOERR_SHORT_READ) {
                                    break 's_73;
                                }
                                rc = SQLITE_OK;
                                break 's_73;
                            }
                        }
                    }
                }
            }
        }
        if rc == SQLITE_OK {
            rc = sqlite3PagerSetPagesize(
                pPager,
                &raw mut savedPageSize,
                -1 as ::core::ffi::c_int,
            );
        }
        (*pPager).changeCountDone = (*pPager).tempFile;
        if rc == SQLITE_OK {
            zSuper = (*pPager).pTmpSpace.offset(4 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char;
            rc = readSuperJournal(
                (*pPager).jfd,
                zSuper,
                (1 as i64_0 + (*(*pPager).pVfs).mxPathname as i64_0) as u64_0,
            );
        }
        if rc == SQLITE_OK
            && ((*pPager).eState as ::core::ffi::c_int >= PAGER_WRITER_DBMOD
                || (*pPager).eState as ::core::ffi::c_int == PAGER_OPEN)
        {
            rc = sqlite3PagerSync(pPager, ::core::ptr::null::<::core::ffi::c_char>());
        }
        if rc == SQLITE_OK {
            rc = pager_end_transaction(
                pPager,
                (*zSuper.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '\0' as i32) as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        if rc == SQLITE_OK
            && *zSuper.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 && res != 0
        {
            memset(
                (*pPager).pTmpSpace as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                4 as size_t,
            );
            rc = pager_delsuper(pPager, zSuper);
        }
        if isHot != 0 && nPlayback != 0 {
            sqlite3_log(
                SQLITE_NOTICE_RECOVER_ROLLBACK,
                b"recovered %d pages from %s\0".as_ptr() as *const ::core::ffi::c_char,
                nPlayback,
                (*pPager).zJournal,
            );
        }
        setSectorSize(pPager);
        return rc;
    }
}
unsafe extern "C" fn readDbPage(mut pPg: *mut PgHdr) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = (*pPg).pPager;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut iFrame: u32_0 = 0 as u32_0;
        if !(*pPager).pWal.is_null() {
            rc = sqlite3WalFindFrame((*pPager).pWal, (*pPg).pgno, &raw mut iFrame);
            if rc != 0 {
                return rc;
            }
        }
        if iFrame != 0 {
            rc = sqlite3WalReadFrame(
                (*pPager).pWal,
                iFrame,
                (*pPager).pageSize as ::core::ffi::c_int,
                (*pPg).pData as *mut u8_0,
            );
        } else {
            let mut iOffset: i64_0 = (*pPg).pgno.wrapping_sub(1 as Pgno) as i64_0
                * (*pPager).pageSize;
            rc = sqlite3OsRead(
                (*pPager).fd,
                (*pPg).pData,
                (*pPager).pageSize as ::core::ffi::c_int,
                iOffset,
            );
            if rc == SQLITE_IOERR_SHORT_READ {
                rc = SQLITE_OK;
            }
        }
        if (*pPg).pgno == 1 as Pgno {
            if rc != 0 {
                memset(
                    &raw mut (*pPager).dbFileVers as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    0xff as ::core::ffi::c_int,
                    ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
                );
            } else {
                let mut dbFileVers: *mut u8_0 = ((*pPg).pData as *mut u8_0)
                    .offset(24 as ::core::ffi::c_int as isize) as *mut u8_0;
                memcpy(
                    &raw mut (*pPager).dbFileVers as *mut ::core::ffi::c_void,
                    dbFileVers as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
                );
            }
        }
        sqlite3_pager_readdb_count += 1;
        (*pPager).nRead += 1;
        return rc;
    }
}
unsafe extern "C" fn pager_write_changecounter(mut pPg: *mut PgHdr) {
    unsafe {
        let mut change_counter: u32_0 = 0;
        if pPg.is_null() {
            return;
        }
        change_counter = sqlite3Get4byte(
                &raw mut (*(*pPg).pPager).dbFileVers as *mut ::core::ffi::c_char
                    as *mut u8_0,
            )
            .wrapping_add(1 as u32_0);
        sqlite3Put4byte(
            ((*pPg).pData as *mut ::core::ffi::c_char as *mut u8_0)
                .offset(24 as ::core::ffi::c_int as isize),
            change_counter,
        );
        sqlite3Put4byte(
            ((*pPg).pData as *mut ::core::ffi::c_char as *mut u8_0)
                .offset(92 as ::core::ffi::c_int as isize),
            change_counter,
        );
        sqlite3Put4byte(
            ((*pPg).pData as *mut ::core::ffi::c_char as *mut u8_0)
                .offset(96 as ::core::ffi::c_int as isize),
            3051002 as u32_0,
        );
    }
}
unsafe extern "C" fn pagerUndoCallback(
    mut pCtx: *mut ::core::ffi::c_void,
    mut iPg: Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pPager: *mut Pager = pCtx as *mut Pager;
        let mut pPg: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        pPg = sqlite3PagerLookup(pPager, iPg) as *mut PgHdr;
        if !pPg.is_null() {
            if sqlite3PcachePageRefcount(pPg) == 1 as ::core::ffi::c_longlong {
                sqlite3PcacheDrop(pPg);
            } else {
                rc = readDbPage(pPg);
                if rc == SQLITE_OK {
                    (*pPager)
                        .xReiniter
                        .expect("non-null function pointer")(pPg as *mut DbPage);
                }
                sqlite3PagerUnrefNotNull(pPg as *mut DbPage);
            }
        }
        sqlite3BackupRestart((*pPager).pBackup);
        return rc;
    }
}
unsafe extern "C" fn pagerRollbackWal(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pList: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        (*pPager).dbSize = (*pPager).dbOrigSize;
        rc = sqlite3WalUndo(
            (*pPager).pWal,
            Some(
                pagerUndoCallback
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        Pgno,
                    ) -> ::core::ffi::c_int,
            ),
            pPager as *mut ::core::ffi::c_void,
        );
        pList = sqlite3PcacheDirtyList((*pPager).pPCache);
        while !pList.is_null() && rc == SQLITE_OK {
            let mut pNext: *mut PgHdr = (*pList).pDirty;
            rc = pagerUndoCallback(pPager as *mut ::core::ffi::c_void, (*pList).pgno);
            pList = pNext;
        }
        return rc;
    }
}
unsafe extern "C" fn pagerWalFrames(
    mut pPager: *mut Pager,
    mut pList: *mut PgHdr,
    mut nTruncate: Pgno,
    mut isCommit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut nList: ::core::ffi::c_int = 0;
        let mut p: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        if isCommit != 0 {
            let mut ppNext: *mut *mut PgHdr = &raw mut pList;
            nList = 0 as ::core::ffi::c_int;
            p = pList;
            loop {
                *ppNext = p;
                if (*ppNext).is_null() {
                    break;
                }
                if (*p).pgno <= nTruncate {
                    ppNext = &raw mut (*p).pDirty;
                    nList += 1;
                }
                p = (*p).pDirty;
            }
        } else {
            nList = 1 as ::core::ffi::c_int;
        }
        (*pPager).aStat[PAGER_STAT_WRITE as usize] = ((*pPager)
            .aStat[PAGER_STAT_WRITE as usize] as ::core::ffi::c_uint)
            .wrapping_add(nList as ::core::ffi::c_uint) as u32_0 as u32_0;
        if (*pList).pgno == 1 as Pgno {
            pager_write_changecounter(pList);
        }
        rc = sqlite3WalFrames(
            (*pPager).pWal,
            (*pPager).pageSize as ::core::ffi::c_int,
            pList,
            nTruncate,
            isCommit,
            (*pPager).walSyncFlags as ::core::ffi::c_int,
        );
        if rc == SQLITE_OK && !(*pPager).pBackup.is_null() {
            p = pList;
            while !p.is_null() {
                sqlite3BackupUpdate(
                    (*pPager).pBackup,
                    (*p).pgno,
                    (*p).pData as *mut u8_0,
                );
                p = (*p).pDirty;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn pagerBeginReadTransaction(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut changed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        sqlite3WalEndReadTransaction((*pPager).pWal);
        rc = sqlite3WalBeginReadTransaction((*pPager).pWal, &raw mut changed);
        if rc != SQLITE_OK || changed != 0 {
            pager_reset(pPager);
            if (*pPager).bUseFetch != 0 {
                sqlite3OsUnfetch(
                    (*pPager).fd,
                    0 as i64_0,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
            }
        }
        return rc;
    }
}
unsafe extern "C" fn pagerPagecount(
    mut pPager: *mut Pager,
    mut pnPage: *mut Pgno,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nPage: Pgno = 0;
        nPage = sqlite3WalDbsize((*pPager).pWal);
        if nPage == 0 as Pgno && !(*(*pPager).fd).pMethods.is_null() {
            let mut n: i64_0 = 0 as i64_0;
            let mut rc: ::core::ffi::c_int = sqlite3OsFileSize((*pPager).fd, &raw mut n);
            if rc != SQLITE_OK {
                return rc;
            }
            nPage = ((n + (*pPager).pageSize - 1 as i64_0) / (*pPager).pageSize) as Pgno;
        }
        if nPage > (*pPager).mxPgno {
            (*pPager).mxPgno = nPage;
        }
        *pnPage = nPage;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pagerOpenWalIfPresent(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).tempFile == 0 {
            let mut isWal: ::core::ffi::c_int = 0;
            rc = sqlite3OsAccess(
                (*pPager).pVfs,
                (*pPager).zWal,
                SQLITE_ACCESS_EXISTS,
                &raw mut isWal,
            );
            if rc == SQLITE_OK {
                if isWal != 0 {
                    let mut nPage: Pgno = 0;
                    rc = pagerPagecount(pPager, &raw mut nPage);
                    if rc != 0 {
                        return rc;
                    }
                    if nPage == 0 as Pgno {
                        rc = sqlite3OsDelete(
                            (*pPager).pVfs,
                            (*pPager).zWal,
                            0 as ::core::ffi::c_int,
                        );
                    } else {
                        rc = sqlite3PagerOpenWal(
                            pPager,
                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        );
                    }
                } else if (*pPager).journalMode as ::core::ffi::c_int
                    == PAGER_JOURNALMODE_WAL
                {
                    (*pPager).journalMode = PAGER_JOURNALMODE_DELETE as u8_0;
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn pagerPlaybackSavepoint(
    mut pPager: *mut Pager,
    mut pSavepoint: *mut PagerSavepoint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut szJ: i64_0 = 0;
        let mut iHdrOff: i64_0 = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pDone: *mut Bitvec = ::core::ptr::null_mut::<Bitvec>();
        if !pSavepoint.is_null() {
            pDone = sqlite3BitvecCreate((*pSavepoint).nOrig as u32_0);
            if pDone.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
        }
        (*pPager).dbSize = if !pSavepoint.is_null() {
            (*pSavepoint).nOrig
        } else {
            (*pPager).dbOrigSize
        };
        (*pPager).changeCountDone = (*pPager).tempFile;
        if pSavepoint.is_null() && !(*pPager).pWal.is_null() {
            return pagerRollbackWal(pPager);
        }
        szJ = (*pPager).journalOff;
        if !pSavepoint.is_null() && (*pPager).pWal.is_null() {
            iHdrOff = (if (*pSavepoint).iHdrOffset != 0 {
                (*pSavepoint).iHdrOffset as ::core::ffi::c_longlong
            } else {
                szJ as ::core::ffi::c_longlong
            }) as i64_0;
            (*pPager).journalOff = (*pSavepoint).iOffset;
            while rc == SQLITE_OK && (*pPager).journalOff < iHdrOff {
                rc = pager_playback_one_page(
                    pPager,
                    &raw mut (*pPager).journalOff,
                    pDone,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            }
        } else {
            (*pPager).journalOff = 0 as i64_0;
        }
        while rc == SQLITE_OK && (*pPager).journalOff < szJ {
            let mut ii: u32_0 = 0;
            let mut nJRec: u32_0 = 0 as u32_0;
            let mut dummy: u32_0 = 0;
            rc = readJournalHdr(
                pPager,
                0 as ::core::ffi::c_int,
                szJ,
                &raw mut nJRec,
                &raw mut dummy,
            );
            if nJRec == 0 as u32_0
                && (*pPager).journalHdr as ::core::ffi::c_longlong
                    + (*pPager).sectorSize as ::core::ffi::c_longlong
                    == (*pPager).journalOff
            {
                nJRec = ((szJ as ::core::ffi::c_longlong
                    - (*pPager).journalOff as ::core::ffi::c_longlong)
                    / ((*pPager).pageSize as ::core::ffi::c_longlong
                        + 8 as ::core::ffi::c_longlong)) as u32_0;
            }
            ii = 0 as u32_0;
            while rc == SQLITE_OK && ii < nJRec && (*pPager).journalOff < szJ {
                rc = pager_playback_one_page(
                    pPager,
                    &raw mut (*pPager).journalOff,
                    pDone,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                ii = ii.wrapping_add(1);
            }
        }
        if !pSavepoint.is_null() {
            let mut ii_0: u32_0 = 0;
            let mut offset: i64_0 = (*pSavepoint).iSubRec as i64_0
                * (4 as i64_0 + (*pPager).pageSize);
            if !(*pPager).pWal.is_null() {
                rc = sqlite3WalSavepointUndo(
                    (*pPager).pWal,
                    &raw mut (*pSavepoint).aWalData as *mut u32_0,
                );
            }
            ii_0 = (*pSavepoint).iSubRec as u32_0;
            while rc == SQLITE_OK && ii_0 < (*pPager).nSubRec {
                rc = pager_playback_one_page(
                    pPager,
                    &raw mut offset,
                    pDone,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                ii_0 = ii_0.wrapping_add(1);
            }
        }
        sqlite3BitvecDestroy(pDone);
        if rc == SQLITE_OK {
            (*pPager).journalOff = szJ;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerSetCachesize(
    mut pPager: *mut Pager,
    mut mxPage: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3PcacheSetCachesize((*pPager).pPCache, mxPage);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerSetSpillsize(
    mut pPager: *mut Pager,
    mut mxPage: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3PcacheSetSpillsize((*pPager).pPCache, mxPage);
    }
}
unsafe extern "C" fn pagerFixMaplimit(mut pPager: *mut Pager) {
    unsafe {
        let mut fd: *mut sqlite3_file = (*pPager).fd;
        if !(*fd).pMethods.is_null()
            && (*(*fd).pMethods).iVersion >= 3 as ::core::ffi::c_int
        {
            let mut sz: sqlite3_int64 = 0;
            sz = (*pPager).szMmap;
            (*pPager).bUseFetch = (sz > 0 as ::core::ffi::c_longlong)
                as ::core::ffi::c_int as u8_0;
            setGetterMethod(pPager);
            sqlite3OsFileControlHint(
                (*pPager).fd,
                SQLITE_FCNTL_MMAP_SIZE,
                &raw mut sz as *mut ::core::ffi::c_void,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerSetMmapLimit(
    mut pPager: *mut Pager,
    mut szMmap: sqlite3_int64,
) {
    unsafe {
        (*pPager).szMmap = szMmap;
        pagerFixMaplimit(pPager);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerShrink(mut pPager: *mut Pager) {
    unsafe {
        sqlite3PcacheShrink((*pPager).pPCache);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerSetFlags(
    mut pPager: *mut Pager,
    mut pgFlags: ::core::ffi::c_uint,
) {
    unsafe {
        let mut level: ::core::ffi::c_uint = pgFlags
            & PAGER_SYNCHRONOUS_MASK as ::core::ffi::c_uint;
        if (*pPager).tempFile as ::core::ffi::c_int != 0
            || level == PAGER_SYNCHRONOUS_OFF as ::core::ffi::c_uint
        {
            (*pPager).noSync = 1 as u8_0;
            (*pPager).fullSync = 0 as u8_0;
            (*pPager).extraSync = 0 as u8_0;
        } else {
            (*pPager).noSync = 0 as u8_0;
            (*pPager).fullSync = (if level
                >= PAGER_SYNCHRONOUS_FULL as ::core::ffi::c_uint
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as u8_0;
            if level == PAGER_SYNCHRONOUS_EXTRA as ::core::ffi::c_uint {
                (*pPager).extraSync = 1 as u8_0;
            } else {
                (*pPager).extraSync = 0 as u8_0;
            }
        }
        if (*pPager).noSync != 0 {
            (*pPager).syncFlags = 0 as u8_0;
        } else if pgFlags & PAGER_FULLFSYNC as ::core::ffi::c_uint != 0 {
            (*pPager).syncFlags = SQLITE_SYNC_FULL as u8_0;
        } else {
            (*pPager).syncFlags = SQLITE_SYNC_NORMAL as u8_0;
        }
        (*pPager).walSyncFlags = (((*pPager).syncFlags as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as u8_0;
        if (*pPager).fullSync != 0 {
            (*pPager).walSyncFlags = ((*pPager).walSyncFlags as ::core::ffi::c_int
                | (*pPager).syncFlags as ::core::ffi::c_int) as u8_0;
        }
        if pgFlags & PAGER_CKPT_FULLFSYNC as ::core::ffi::c_uint != 0
            && (*pPager).noSync == 0
        {
            (*pPager).walSyncFlags = ((*pPager).walSyncFlags as ::core::ffi::c_int
                | SQLITE_SYNC_FULL << 2 as ::core::ffi::c_int) as u8_0;
        }
        if pgFlags & PAGER_CACHESPILL as ::core::ffi::c_uint != 0 {
            (*pPager).doNotSpill = ((*pPager).doNotSpill as ::core::ffi::c_int
                & !SPILLFLAG_OFF) as u8_0;
        } else {
            (*pPager).doNotSpill = ((*pPager).doNotSpill as ::core::ffi::c_int
                | SPILLFLAG_OFF) as u8_0;
        };
    }
}
#[no_mangle]
pub static mut sqlite3_opentemp_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn pagerOpentemp(
    mut pPager: *mut Pager,
    mut pFile: *mut sqlite3_file,
    mut vfsFlags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        sqlite3_opentemp_count += 1;
        vfsFlags
            |= SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_EXCLUSIVE
                | SQLITE_OPEN_DELETEONCLOSE;
        rc = sqlite3OsOpen(
            (*pPager).pVfs,
            ::core::ptr::null::<::core::ffi::c_char>(),
            pFile,
            vfsFlags,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerSetBusyHandler(
    mut pPager: *mut Pager,
    mut xBusyHandler: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    mut pBusyHandlerArg: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut ap: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            *mut ::core::ffi::c_void,
        >();
        (*pPager).xBusyHandler = xBusyHandler;
        (*pPager).pBusyHandlerArg = pBusyHandlerArg;
        ap = &raw mut (*pPager).xBusyHandler as *mut *mut ::core::ffi::c_void;
        sqlite3OsFileControlHint(
            (*pPager).fd,
            SQLITE_FCNTL_BUSYHANDLER,
            ap as *mut ::core::ffi::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerSetPagesize(
    mut pPager: *mut Pager,
    mut pPageSize: *mut u32_0,
    mut nReserve: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pageSize: u32_0 = *pPageSize;
        if ((*pPager).memDb as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*pPager).dbSize == 0 as Pgno)
            && sqlite3PcacheRefCount((*pPager).pPCache) == 0 as ::core::ffi::c_longlong
            && pageSize != 0 && pageSize != (*pPager).pageSize as u32_0
        {
            let mut pNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut nByte: i64_0 = 0 as i64_0;
            if (*pPager).eState as ::core::ffi::c_int > PAGER_OPEN
                && !(*(*pPager).fd).pMethods.is_null()
            {
                rc = sqlite3OsFileSize((*pPager).fd, &raw mut nByte);
            }
            if rc == SQLITE_OK {
                pNew = sqlite3PageMalloc(
                    pageSize.wrapping_add(8 as u32_0) as ::core::ffi::c_int,
                ) as *mut ::core::ffi::c_char;
                if pNew.is_null() {
                    rc = SQLITE_NOMEM_BKPT;
                } else {
                    memset(
                        pNew.offset(pageSize as isize) as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        8 as size_t,
                    );
                }
            }
            if rc == SQLITE_OK {
                pager_reset(pPager);
                rc = sqlite3PcacheSetPageSize(
                    (*pPager).pPCache,
                    pageSize as ::core::ffi::c_int,
                );
            }
            if rc == SQLITE_OK {
                sqlite3PageFree((*pPager).pTmpSpace as *mut ::core::ffi::c_void);
                (*pPager).pTmpSpace = pNew;
                (*pPager).dbSize = ((nByte as ::core::ffi::c_longlong
                    + pageSize as ::core::ffi::c_longlong - 1 as ::core::ffi::c_longlong)
                    / pageSize as ::core::ffi::c_longlong) as Pgno;
                (*pPager).pageSize = pageSize as i64_0;
                (*pPager).lckPgno = (sqlite3PendingByte as u32_0)
                    .wrapping_div(pageSize)
                    .wrapping_add(1 as Pgno);
            } else {
                sqlite3PageFree(pNew as *mut ::core::ffi::c_void);
            }
        }
        *pPageSize = (*pPager).pageSize as u32_0;
        if rc == SQLITE_OK {
            if nReserve < 0 as ::core::ffi::c_int {
                nReserve = (*pPager).nReserve as ::core::ffi::c_int;
            }
            (*pPager).nReserve = nReserve as i16_0;
            pagerFixMaplimit(pPager);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerTempSpace(
    mut pPager: *mut Pager,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*pPager).pTmpSpace as *mut ::core::ffi::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerMaxPageCount(
    mut pPager: *mut Pager,
    mut mxPage: Pgno,
) -> Pgno {
    unsafe {
        if mxPage > 0 as Pgno {
            (*pPager).mxPgno = mxPage;
        }
        return (*pPager).mxPgno;
    }
}
static mut saved_cnt: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn disable_simulated_io_errors() {
    unsafe {
        saved_cnt = sqlite3_io_error_pending;
        sqlite3_io_error_pending = -1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn enable_simulated_io_errors() {
    unsafe {
        sqlite3_io_error_pending = saved_cnt;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerReadFileheader(
    mut pPager: *mut Pager,
    mut N: ::core::ffi::c_int,
    mut pDest: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        memset(pDest as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, N as size_t);
        if !(*(*pPager).fd).pMethods.is_null() {
            rc = sqlite3OsRead(
                (*pPager).fd,
                pDest as *mut ::core::ffi::c_void,
                N,
                0 as i64_0,
            );
            if rc == SQLITE_IOERR_SHORT_READ {
                rc = SQLITE_OK;
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerPagecount(
    mut pPager: *mut Pager,
    mut pnPage: *mut ::core::ffi::c_int,
) {
    unsafe {
        *pnPage = (*pPager).dbSize as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn pager_wait_on_lock(
    mut pPager: *mut Pager,
    mut locktype: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        loop {
            rc = pagerLockDb(pPager, locktype);
            if !(rc == SQLITE_BUSY
                && (*pPager)
                    .xBusyHandler
                    .expect("non-null function pointer")((*pPager).pBusyHandlerArg) != 0)
            {
                break;
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerTruncateImage(
    mut pPager: *mut Pager,
    mut nPage: Pgno,
) {
    unsafe {
        (*pPager).dbSize = nPage;
    }
}
unsafe extern "C" fn pagerSyncHotJournal(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).noSync == 0 {
            rc = sqlite3OsSync((*pPager).jfd, SQLITE_SYNC_NORMAL);
        }
        if rc == SQLITE_OK {
            rc = sqlite3OsFileSize((*pPager).jfd, &raw mut (*pPager).journalHdr);
        }
        return rc;
    }
}
unsafe extern "C" fn pagerAcquireMapPage(
    mut pPager: *mut Pager,
    mut pgno: Pgno,
    mut pData: *mut ::core::ffi::c_void,
    mut ppPage: *mut *mut PgHdr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        if !(*pPager).pMmapFreelist.is_null() {
            p = (*pPager).pMmapFreelist;
            *ppPage = p;
            (*pPager).pMmapFreelist = (*p).pDirty;
            (*p).pDirty = ::core::ptr::null_mut::<PgHdr>();
            memset((*p).pExtra, 0 as ::core::ffi::c_int, 8 as size_t);
        } else {
            p = sqlite3MallocZero(
                (::core::mem::size_of::<PgHdr>() as usize)
                    .wrapping_add((*pPager).nExtra as usize) as u64_0,
            ) as *mut PgHdr;
            *ppPage = p;
            if p.is_null() {
                sqlite3OsUnfetch(
                    (*pPager).fd,
                    pgno.wrapping_sub(1 as Pgno) as i64_0 * (*pPager).pageSize,
                    pData,
                );
                return SQLITE_NOMEM_BKPT;
            }
            (*p).pExtra = p.offset(1 as ::core::ffi::c_int as isize) as *mut PgHdr
                as *mut ::core::ffi::c_void;
            (*p).flags = PGHDR_MMAP as u16_0;
            (*p).nRef = 1 as i64_0;
            (*p).pPager = pPager;
        }
        (*p).pgno = pgno;
        (*p).pData = pData;
        (*pPager).nMmapOut += 1;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pagerReleaseMapPage(mut pPg: *mut PgHdr) {
    unsafe {
        let mut pPager: *mut Pager = (*pPg).pPager;
        (*pPager).nMmapOut -= 1;
        (*pPg).pDirty = (*pPager).pMmapFreelist;
        (*pPager).pMmapFreelist = pPg;
        sqlite3OsUnfetch(
            (*pPager).fd,
            (*pPg).pgno.wrapping_sub(1 as Pgno) as i64_0 * (*pPager).pageSize,
            (*pPg).pData,
        );
    }
}
unsafe extern "C" fn pagerFreeMapHdrs(mut pPager: *mut Pager) {
    unsafe {
        let mut p: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        let mut pNext: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        p = (*pPager).pMmapFreelist;
        while !p.is_null() {
            pNext = (*p).pDirty;
            sqlite3_free(p as *mut ::core::ffi::c_void);
            p = pNext;
        }
    }
}
unsafe extern "C" fn databaseIsUnmoved(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    unsafe {
        let mut bHasMoved: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = 0;
        if (*pPager).tempFile != 0 {
            return SQLITE_OK;
        }
        if (*pPager).dbSize == 0 as Pgno {
            return SQLITE_OK;
        }
        rc = sqlite3OsFileControl(
            (*pPager).fd,
            SQLITE_FCNTL_HAS_MOVED,
            &raw mut bHasMoved as *mut ::core::ffi::c_void,
        );
        if rc == SQLITE_NOTFOUND {
            rc = SQLITE_OK;
        } else if rc == SQLITE_OK && bHasMoved != 0 {
            rc = SQLITE_READONLY_DBMOVED;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerClose(
    mut pPager: *mut Pager,
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTmp: *mut u8_0 = (*pPager).pTmpSpace as *mut u8_0;
        disable_simulated_io_errors();
        sqlite3BeginBenignMalloc();
        pagerFreeMapHdrs(pPager);
        (*pPager).exclusiveMode = 0 as u8_0;
        let mut a: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        if !db.is_null()
            && 0 as ::core::ffi::c_ulonglong
                == (*db).flags as ::core::ffi::c_ulonglong
                    & SQLITE_NoCkptOnClose as ::core::ffi::c_ulonglong
            && SQLITE_OK == databaseIsUnmoved(pPager)
        {
            a = pTmp;
        }
        sqlite3WalClose(
            (*pPager).pWal,
            db,
            (*pPager).walSyncFlags as ::core::ffi::c_int,
            (*pPager).pageSize as ::core::ffi::c_int,
            a,
        );
        (*pPager).pWal = ::core::ptr::null_mut::<Wal>();
        pager_reset(pPager);
        if (*pPager).memDb != 0 {
            pager_unlock(pPager);
        } else {
            if !(*(*pPager).jfd).pMethods.is_null() {
                pager_error(pPager, pagerSyncHotJournal(pPager));
            }
            pagerUnlockAndRollback(pPager);
        }
        sqlite3EndBenignMalloc();
        enable_simulated_io_errors();
        sqlite3OsClose((*pPager).jfd);
        sqlite3OsClose((*pPager).fd);
        sqlite3PageFree(pTmp as *mut ::core::ffi::c_void);
        sqlite3PcacheClose((*pPager).pPCache);
        sqlite3_free(pPager as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerPagenumber(mut pPg: *mut DbPage) -> Pgno {
    unsafe {
        return (*pPg).pgno;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerRef(mut pPg: *mut DbPage) {
    unsafe {
        sqlite3PcacheRef(pPg as *mut PgHdr);
    }
}
unsafe extern "C" fn syncJournal(
    mut pPager: *mut Pager,
    mut newHdr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3PagerExclusiveLock(pPager);
        if rc != SQLITE_OK {
            return rc;
        }
        if (*pPager).noSync == 0 {
            if !(*(*pPager).jfd).pMethods.is_null()
                && (*pPager).journalMode as ::core::ffi::c_int
                    != PAGER_JOURNALMODE_MEMORY
            {
                let iDc: ::core::ffi::c_int = sqlite3OsDeviceCharacteristics(
                    (*pPager).fd,
                ) as ::core::ffi::c_int;
                if 0 as ::core::ffi::c_int == iDc & SQLITE_IOCAP_SAFE_APPEND {
                    let mut iNextHdrOffset: i64_0 = 0;
                    let mut aMagic: [u8_0; 8] = [0; 8];
                    let mut zHeader: [u8_0; 12] = [0; 12];
                    memcpy(
                        &raw mut zHeader as *mut u8_0 as *mut ::core::ffi::c_void,
                        &raw const aJournalMagic as *const ::core::ffi::c_uchar
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as size_t,
                    );
                    sqlite3Put4byte(
                        (&raw mut zHeader as *mut u8_0)
                            .offset(
                                ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as isize,
                            ) as *mut u8_0,
                        (*pPager).nRec as u32_0,
                    );
                    iNextHdrOffset = journalHdrOffset(pPager);
                    rc = sqlite3OsRead(
                        (*pPager).jfd,
                        &raw mut aMagic as *mut u8_0 as *mut ::core::ffi::c_void,
                        8 as ::core::ffi::c_int,
                        iNextHdrOffset,
                    );
                    if rc == SQLITE_OK
                        && 0 as ::core::ffi::c_int
                            == memcmp(
                                &raw mut aMagic as *mut u8_0 as *const ::core::ffi::c_void,
                                &raw const aJournalMagic as *const ::core::ffi::c_uchar
                                    as *const ::core::ffi::c_void,
                                8 as size_t,
                            )
                    {
                        static mut zerobyte: u8_0 = 0 as u8_0;
                        rc = sqlite3OsWrite(
                            (*pPager).jfd,
                            &raw const zerobyte as *const ::core::ffi::c_void,
                            1 as ::core::ffi::c_int,
                            iNextHdrOffset,
                        );
                    }
                    if rc != SQLITE_OK && rc != SQLITE_IOERR_SHORT_READ {
                        return rc;
                    }
                    if (*pPager).fullSync as ::core::ffi::c_int != 0
                        && 0 as ::core::ffi::c_int == iDc & SQLITE_IOCAP_SEQUENTIAL
                    {
                        rc = sqlite3OsSync(
                            (*pPager).jfd,
                            (*pPager).syncFlags as ::core::ffi::c_int,
                        );
                        if rc != SQLITE_OK {
                            return rc;
                        }
                    }
                    rc = sqlite3OsWrite(
                        (*pPager).jfd,
                        &raw mut zHeader as *mut u8_0 as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<[u8_0; 12]>() as ::core::ffi::c_int,
                        (*pPager).journalHdr,
                    );
                    if rc != SQLITE_OK {
                        return rc;
                    }
                }
                if 0 as ::core::ffi::c_int == iDc & SQLITE_IOCAP_SEQUENTIAL {
                    rc = sqlite3OsSync(
                        (*pPager).jfd,
                        (*pPager).syncFlags as ::core::ffi::c_int
                            | (if (*pPager).syncFlags as ::core::ffi::c_int
                                == SQLITE_SYNC_FULL
                            {
                                SQLITE_SYNC_DATAONLY
                            } else {
                                0 as ::core::ffi::c_int
                            }),
                    );
                    if rc != SQLITE_OK {
                        return rc;
                    }
                }
                (*pPager).journalHdr = (*pPager).journalOff;
                if newHdr != 0
                    && 0 as ::core::ffi::c_int == iDc & SQLITE_IOCAP_SAFE_APPEND
                {
                    (*pPager).nRec = 0 as ::core::ffi::c_int;
                    rc = writeJournalHdr(pPager);
                    if rc != SQLITE_OK {
                        return rc;
                    }
                }
            } else {
                (*pPager).journalHdr = (*pPager).journalOff;
            }
        }
        sqlite3PcacheClearSyncFlags((*pPager).pPCache);
        (*pPager).eState = PAGER_WRITER_DBMOD as u8_0;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pager_write_pagelist(
    mut pPager: *mut Pager,
    mut pList: *mut PgHdr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*(*pPager).fd).pMethods.is_null() {
            rc = pagerOpentemp(
                pPager,
                (*pPager).fd,
                (*pPager).vfsFlags as ::core::ffi::c_int,
            );
        }
        if rc == SQLITE_OK && (*pPager).dbHintSize < (*pPager).dbSize
            && (!(*pList).pDirty.is_null() || (*pList).pgno > (*pPager).dbHintSize)
        {
            let mut szFile: sqlite3_int64 = (*pPager).pageSize as sqlite3_int64
                * (*pPager).dbSize as sqlite3_int64;
            sqlite3OsFileControlHint(
                (*pPager).fd,
                SQLITE_FCNTL_SIZE_HINT,
                &raw mut szFile as *mut ::core::ffi::c_void,
            );
            (*pPager).dbHintSize = (*pPager).dbSize;
        }
        while rc == SQLITE_OK && !pList.is_null() {
            let mut pgno: Pgno = (*pList).pgno;
            if pgno <= (*pPager).dbSize
                && 0 as ::core::ffi::c_int
                    == (*pList).flags as ::core::ffi::c_int & PGHDR_DONT_WRITE
            {
                let mut offset: i64_0 = pgno.wrapping_sub(1 as Pgno) as i64_0
                    * (*pPager).pageSize;
                let mut pData: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                if (*pList).pgno == 1 as Pgno {
                    pager_write_changecounter(pList);
                }
                pData = (*pList).pData as *mut ::core::ffi::c_char;
                rc = sqlite3OsWrite(
                    (*pPager).fd,
                    pData as *const ::core::ffi::c_void,
                    (*pPager).pageSize as ::core::ffi::c_int,
                    offset,
                );
                if pgno == 1 as Pgno {
                    memcpy(
                        &raw mut (*pPager).dbFileVers as *mut ::core::ffi::c_void,
                        pData.offset(24 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
                    );
                }
                if pgno > (*pPager).dbFileSize {
                    (*pPager).dbFileSize = pgno;
                }
                (*pPager).aStat[PAGER_STAT_WRITE as usize] = (*pPager)
                    .aStat[PAGER_STAT_WRITE as usize]
                    .wrapping_add(1);
                sqlite3BackupUpdate(
                    (*pPager).pBackup,
                    pgno,
                    (*pList).pData as *mut u8_0,
                );
                sqlite3_pager_writedb_count += 1;
            }
            pList = (*pList).pDirty;
        }
        return rc;
    }
}
unsafe extern "C" fn openSubJournal(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*(*pPager).sjfd).pMethods.is_null() {
            let flags: ::core::ffi::c_int = SQLITE_OPEN_SUBJOURNAL
                | SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_EXCLUSIVE
                | SQLITE_OPEN_DELETEONCLOSE;
            let mut nStmtSpill: ::core::ffi::c_int = sqlite3Config.nStmtSpill;
            if (*pPager).journalMode as ::core::ffi::c_int == PAGER_JOURNALMODE_MEMORY
                || (*pPager).subjInMemory as ::core::ffi::c_int != 0
            {
                nStmtSpill = -1 as ::core::ffi::c_int;
            }
            rc = sqlite3JournalOpen(
                (*pPager).pVfs,
                ::core::ptr::null::<::core::ffi::c_char>(),
                (*pPager).sjfd,
                flags,
                nStmtSpill,
            );
        }
        return rc;
    }
}
unsafe extern "C" fn subjournalPage(mut pPg: *mut PgHdr) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pPager: *mut Pager = (*pPg).pPager;
        if (*pPager).journalMode as ::core::ffi::c_int != PAGER_JOURNALMODE_OFF {
            rc = openSubJournal(pPager);
            if rc == SQLITE_OK {
                let mut pData: *mut ::core::ffi::c_void = (*pPg).pData;
                let mut offset: i64_0 = (*pPager).nSubRec as i64_0
                    * (4 as i64_0 + (*pPager).pageSize);
                let mut pData2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                pData2 = pData as *mut ::core::ffi::c_char;
                rc = write32bits((*pPager).sjfd, offset, (*pPg).pgno as u32_0);
                if rc == SQLITE_OK {
                    rc = sqlite3OsWrite(
                        (*pPager).sjfd,
                        pData2 as *const ::core::ffi::c_void,
                        (*pPager).pageSize as ::core::ffi::c_int,
                        offset + 4 as i64_0,
                    );
                }
            }
        }
        if rc == SQLITE_OK {
            (*pPager).nSubRec = (*pPager).nSubRec.wrapping_add(1);
            rc = addToSavepointBitvecs(pPager, (*pPg).pgno);
        }
        return rc;
    }
}
unsafe extern "C" fn subjournalPageIfRequired(
    mut pPg: *mut PgHdr,
) -> ::core::ffi::c_int {
    unsafe {
        if subjRequiresPage(pPg) != 0 {
            return subjournalPage(pPg)
        } else {
            return SQLITE_OK
        };
    }
}
unsafe extern "C" fn pagerStress(
    mut p: *mut ::core::ffi::c_void,
    mut pPg: *mut PgHdr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = p as *mut Pager;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).errCode != 0 {
            return SQLITE_OK;
        }
        if (*pPager).doNotSpill as ::core::ffi::c_int != 0
            && ((*pPager).doNotSpill as ::core::ffi::c_int
                & (SPILLFLAG_ROLLBACK | SPILLFLAG_OFF) != 0 as ::core::ffi::c_int
                || (*pPg).flags as ::core::ffi::c_int & PGHDR_NEED_SYNC
                    != 0 as ::core::ffi::c_int)
        {
            return SQLITE_OK;
        }
        (*pPager).aStat[PAGER_STAT_SPILL as usize] = (*pPager)
            .aStat[PAGER_STAT_SPILL as usize]
            .wrapping_add(1);
        (*pPg).pDirty = ::core::ptr::null_mut::<PgHdr>();
        if !(*pPager).pWal.is_null() {
            rc = subjournalPageIfRequired(pPg);
            if rc == SQLITE_OK {
                rc = pagerWalFrames(pPager, pPg, 0 as Pgno, 0 as ::core::ffi::c_int);
            }
        } else {
            if (*pPg).flags as ::core::ffi::c_int & PGHDR_NEED_SYNC != 0
                || (*pPager).eState as ::core::ffi::c_int == PAGER_WRITER_CACHEMOD
            {
                rc = syncJournal(pPager, 1 as ::core::ffi::c_int);
            }
            if rc == SQLITE_OK {
                rc = pager_write_pagelist(pPager, pPg);
            }
        }
        if rc == SQLITE_OK {
            sqlite3PcacheMakeClean(pPg);
        }
        return pager_error(pPager, rc);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerFlush(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = (*pPager).errCode;
        if (*pPager).memDb == 0 {
            let mut pList: *mut PgHdr = sqlite3PcacheDirtyList((*pPager).pPCache);
            while rc == SQLITE_OK && !pList.is_null() {
                let mut pNext: *mut PgHdr = (*pList).pDirty;
                if (*pList).nRef == 0 as ::core::ffi::c_longlong {
                    rc = pagerStress(pPager as *mut ::core::ffi::c_void, pList);
                }
                pList = pNext;
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut ppPager: *mut *mut Pager,
    mut zFilename: *const ::core::ffi::c_char,
    mut nExtra: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut vfsFlags: ::core::ffi::c_int,
    mut xReinit: Option<unsafe extern "C" fn(*mut DbPage) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPtr: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut tempFile: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut memDb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut memJM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut readOnly: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut journalFileSize: ::core::ffi::c_int = 0;
        let mut zPathname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nPathname: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut useJournal: ::core::ffi::c_int = (flags & PAGER_OMIT_JOURNAL
            == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let mut pcacheSize: ::core::ffi::c_int = sqlite3PcacheSize();
        let mut szPageDflt: u32_0 = SQLITE_DEFAULT_PAGE_SIZE as u32_0;
        let mut zUri: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nUriByte: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        journalFileSize = sqlite3JournalSize(pVfs) + 7 as ::core::ffi::c_int
            & !(7 as ::core::ffi::c_int);
        *ppPager = ::core::ptr::null_mut::<Pager>();
        if flags & PAGER_MEMORY != 0 {
            memDb = 1 as ::core::ffi::c_int;
            if !zFilename.is_null()
                && *zFilename.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int != 0
            {
                zPathname = sqlite3DbStrDup(
                    ::core::ptr::null_mut::<sqlite3>(),
                    zFilename,
                );
                if zPathname.is_null() {
                    return SQLITE_NOMEM_BKPT;
                }
                nPathname = sqlite3Strlen30(zPathname);
                zFilename = ::core::ptr::null::<::core::ffi::c_char>();
            }
        }
        if !zFilename.is_null()
            && *zFilename.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0
        {
            let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            nPathname = (*pVfs).mxPathname + 1 as ::core::ffi::c_int;
            zPathname = sqlite3DbMallocRaw(
                ::core::ptr::null_mut::<sqlite3>(),
                (2 as i64_0 * nPathname as i64_0) as u64_0,
            ) as *mut ::core::ffi::c_char;
            if zPathname.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            *zPathname.offset(0 as ::core::ffi::c_int as isize) = 0
                as ::core::ffi::c_char;
            rc = sqlite3OsFullPathname(pVfs, zFilename, nPathname, zPathname);
            if rc != SQLITE_OK {
                if rc == SQLITE_OK_SYMLINK {
                    if vfsFlags & SQLITE_OPEN_NOFOLLOW != 0 {
                        rc = SQLITE_CANTOPEN_SYMLINK;
                    } else {
                        rc = SQLITE_OK;
                    }
                }
            }
            nPathname = sqlite3Strlen30(zPathname);
            zUri = zFilename
                .offset(
                    ((sqlite3Strlen30
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int)(zFilename) + 1 as ::core::ffi::c_int)
                        as isize,
                ) as *const ::core::ffi::c_char;
            z = zUri;
            while *z != 0 {
                z = z.offset(strlen(z).wrapping_add(1 as size_t) as isize);
                z = z.offset(strlen(z).wrapping_add(1 as size_t) as isize);
            }
            nUriByte = (z.offset(1 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char)
                .offset_from(zUri) as ::core::ffi::c_long as ::core::ffi::c_int;
            if rc == SQLITE_OK
                && nPathname + 8 as ::core::ffi::c_int > (*pVfs).mxPathname
            {
                rc = sqlite3CantopenError(4813 as ::core::ffi::c_int);
            }
            if rc != SQLITE_OK {
                sqlite3DbFree(
                    ::core::ptr::null_mut::<sqlite3>(),
                    zPathname as *mut ::core::ffi::c_void,
                );
                return rc;
            }
        }
        pPtr = sqlite3MallocZero(
            (((::core::mem::size_of::<Pager>() as usize).wrapping_add(7 as usize)
                & !(7 as ::core::ffi::c_int) as usize)
                .wrapping_add(
                    (pcacheSize + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int))
                        as usize,
                )
                .wrapping_add(
                    ((*pVfs).szOsFile + 7 as ::core::ffi::c_int
                        & !(7 as ::core::ffi::c_int)) as usize,
                ) as u64_0)
                .wrapping_add((journalFileSize as u64_0).wrapping_mul(2 as u64_0))
                .wrapping_add(SQLITE_PTRSIZE as u64_0)
                .wrapping_add(4 as u64_0)
                .wrapping_add(nPathname as u64_0)
                .wrapping_add(1 as u64_0)
                .wrapping_add(nUriByte as u64_0)
                .wrapping_add(nPathname as u64_0)
                .wrapping_add(8 as u64_0)
                .wrapping_add(1 as u64_0)
                .wrapping_add(nPathname as u64_0)
                .wrapping_add(4 as u64_0)
                .wrapping_add(1 as u64_0)
                .wrapping_add(3 as u64_0),
        ) as *mut u8_0;
        if pPtr.is_null() {
            sqlite3DbFree(
                ::core::ptr::null_mut::<sqlite3>(),
                zPathname as *mut ::core::ffi::c_void,
            );
            return SQLITE_NOMEM_BKPT;
        }
        pPager = pPtr as *mut Pager;
        pPtr = pPtr
            .offset(
                ((::core::mem::size_of::<Pager>() as usize).wrapping_add(7 as usize)
                    & !(7 as ::core::ffi::c_int) as usize) as isize,
            );
        (*pPager).pPCache = pPtr as *mut PCache;
        pPtr = pPtr
            .offset(
                (pcacheSize + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int))
                    as isize,
            );
        (*pPager).fd = pPtr as *mut sqlite3_file;
        pPtr = pPtr
            .offset(
                ((*pVfs).szOsFile + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int))
                    as isize,
            );
        (*pPager).sjfd = pPtr as *mut sqlite3_file;
        pPtr = pPtr.offset(journalFileSize as isize);
        (*pPager).jfd = pPtr as *mut sqlite3_file;
        pPtr = pPtr.offset(journalFileSize as isize);
        memcpy(
            pPtr as *mut ::core::ffi::c_void,
            &raw mut pPager as *const ::core::ffi::c_void,
            SQLITE_PTRSIZE as size_t,
        );
        pPtr = pPtr.offset(SQLITE_PTRSIZE as isize);
        pPtr = pPtr.offset(4 as ::core::ffi::c_int as isize);
        (*pPager).zFilename = pPtr as *mut ::core::ffi::c_char;
        if nPathname > 0 as ::core::ffi::c_int {
            memcpy(
                pPtr as *mut ::core::ffi::c_void,
                zPathname as *const ::core::ffi::c_void,
                nPathname as size_t,
            );
            pPtr = pPtr.offset((nPathname + 1 as ::core::ffi::c_int) as isize);
            if !zUri.is_null() {
                memcpy(
                    pPtr as *mut ::core::ffi::c_void,
                    zUri as *const ::core::ffi::c_void,
                    nUriByte as size_t,
                );
                pPtr = pPtr.offset(nUriByte as isize);
            } else {
                pPtr = pPtr.offset(1);
            }
        }
        if nPathname > 0 as ::core::ffi::c_int {
            (*pPager).zJournal = pPtr as *mut ::core::ffi::c_char;
            memcpy(
                pPtr as *mut ::core::ffi::c_void,
                zPathname as *const ::core::ffi::c_void,
                nPathname as size_t,
            );
            pPtr = pPtr.offset(nPathname as isize);
            memcpy(
                pPtr as *mut ::core::ffi::c_void,
                b"-journal\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                8 as size_t,
            );
            pPtr = pPtr
                .offset((8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
        } else {
            (*pPager).zJournal = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if nPathname > 0 as ::core::ffi::c_int {
            (*pPager).zWal = pPtr as *mut ::core::ffi::c_char;
            memcpy(
                pPtr as *mut ::core::ffi::c_void,
                zPathname as *const ::core::ffi::c_void,
                nPathname as size_t,
            );
            pPtr = pPtr.offset(nPathname as isize);
            memcpy(
                pPtr as *mut ::core::ffi::c_void,
                b"-wal\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                4 as size_t,
            );
            pPtr = pPtr
                .offset((4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
        } else {
            (*pPager).zWal = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if nPathname != 0 {
            sqlite3DbFree(
                ::core::ptr::null_mut::<sqlite3>(),
                zPathname as *mut ::core::ffi::c_void,
            );
        }
        (*pPager).pVfs = pVfs;
        (*pPager).vfsFlags = vfsFlags as u32_0;
        let mut c2rust_current_block_121: u64;
        if !zFilename.is_null()
            && *zFilename.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0
        {
            let mut fout: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            rc = sqlite3OsOpen(
                pVfs,
                (*pPager).zFilename,
                (*pPager).fd,
                vfsFlags,
                &raw mut fout,
            );
            memJM = (fout & SQLITE_OPEN_MEMORY != 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            (*pPager).memVfs = memJM as u8_0;
            readOnly = (fout & SQLITE_OPEN_READONLY != 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if rc == SQLITE_OK {
                let mut iDc: ::core::ffi::c_int = sqlite3OsDeviceCharacteristics(
                    (*pPager).fd,
                );
                if readOnly == 0 {
                    setSectorSize(pPager);
                    if szPageDflt < (*pPager).sectorSize {
                        if (*pPager).sectorSize > SQLITE_MAX_DEFAULT_PAGE_SIZE as u32_0 {
                            szPageDflt = SQLITE_MAX_DEFAULT_PAGE_SIZE as u32_0;
                        } else {
                            szPageDflt = (*pPager).sectorSize;
                        }
                    }
                }
                (*pPager).noLock = sqlite3_uri_boolean(
                    (*pPager).zFilename as sqlite3_filename,
                    b"nolock\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                ) as u8_0;
                if iDc & SQLITE_IOCAP_IMMUTABLE != 0 as ::core::ffi::c_int
                    || sqlite3_uri_boolean(
                        (*pPager).zFilename as sqlite3_filename,
                        b"immutable\0".as_ptr() as *const ::core::ffi::c_char,
                        0 as ::core::ffi::c_int,
                    ) != 0
                {
                    vfsFlags |= SQLITE_OPEN_READONLY;
                    c2rust_current_block_121 = 11300258924823861251;
                } else {
                    c2rust_current_block_121 = 6712462580143783635;
                }
            } else {
                c2rust_current_block_121 = 6712462580143783635;
            }
        } else {
            c2rust_current_block_121 = 11300258924823861251;
        }
        match c2rust_current_block_121 {
            11300258924823861251 => {
                tempFile = 1 as ::core::ffi::c_int;
                (*pPager).eState = PAGER_READER as u8_0;
                (*pPager).eLock = EXCLUSIVE_LOCK as u8_0;
                (*pPager).noLock = 1 as u8_0;
                readOnly = vfsFlags & SQLITE_OPEN_READONLY;
            }
            _ => {}
        }
        if rc == SQLITE_OK {
            rc = sqlite3PagerSetPagesize(
                pPager,
                &raw mut szPageDflt,
                -1 as ::core::ffi::c_int,
            );
        }
        if rc == SQLITE_OK {
            nExtra = nExtra + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int);
            rc = sqlite3PcacheOpen(
                szPageDflt as ::core::ffi::c_int,
                nExtra,
                (memDb == 0) as ::core::ffi::c_int,
                if memDb == 0 {
                    Some(
                        pagerStress
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut PgHdr,
                            ) -> ::core::ffi::c_int,
                    )
                } else {
                    None
                },
                pPager as *mut ::core::ffi::c_void,
                (*pPager).pPCache,
            );
        }
        if rc != SQLITE_OK {
            sqlite3OsClose((*pPager).fd);
            sqlite3PageFree((*pPager).pTmpSpace as *mut ::core::ffi::c_void);
            sqlite3_free(pPager as *mut ::core::ffi::c_void);
            return rc;
        }
        (*pPager).useJournal = useJournal as u8_0;
        (*pPager).mxPgno = SQLITE_MAX_PAGE_COUNT as Pgno;
        (*pPager).tempFile = tempFile as u8_0;
        (*pPager).exclusiveMode = tempFile as u8_0;
        (*pPager).changeCountDone = (*pPager).tempFile;
        (*pPager).memDb = memDb as u8_0;
        (*pPager).readOnly = readOnly as u8_0;
        sqlite3PagerSetFlags(
            pPager,
            (SQLITE_DEFAULT_SYNCHRONOUS + 1 as ::core::ffi::c_int | PAGER_CACHESPILL)
                as ::core::ffi::c_uint,
        );
        (*pPager).nExtra = nExtra as u16_0;
        (*pPager).journalSizeLimit = SQLITE_DEFAULT_JOURNAL_SIZE_LIMIT as i64_0;
        setSectorSize(pPager);
        if useJournal == 0 {
            (*pPager).journalMode = PAGER_JOURNALMODE_OFF as u8_0;
        } else if memDb != 0 || memJM != 0 {
            (*pPager).journalMode = PAGER_JOURNALMODE_MEMORY as u8_0;
        }
        (*pPager).xReiniter = xReinit;
        setGetterMethod(pPager);
        *ppPager = pPager;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_database_file_object(
    mut zName: *const ::core::ffi::c_char,
) -> *mut sqlite3_file {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
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
        p = zName
            .offset(-(4 as ::core::ffi::c_int as isize))
            .offset(-(::core::mem::size_of::<*mut Pager>() as usize as isize));
        pPager = *(p as *mut *mut Pager);
        return (*pPager).fd;
    }
}
unsafe extern "C" fn hasHotJournal(
    mut pPager: *mut Pager,
    mut pExists: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let pVfs: *mut sqlite3_vfs = (*pPager).pVfs;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut exists: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut jrnlOpen: ::core::ffi::c_int = !(*(*pPager).jfd).pMethods.is_null()
            as ::core::ffi::c_int;
        *pExists = 0 as ::core::ffi::c_int;
        if jrnlOpen == 0 {
            rc = sqlite3OsAccess(
                pVfs,
                (*pPager).zJournal,
                SQLITE_ACCESS_EXISTS,
                &raw mut exists,
            );
        }
        if rc == SQLITE_OK && exists != 0 {
            let mut locked: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            rc = sqlite3OsCheckReservedLock((*pPager).fd, &raw mut locked);
            if rc == SQLITE_OK && locked == 0 {
                let mut nPage: Pgno = 0;
                rc = pagerPagecount(pPager, &raw mut nPage);
                if rc == SQLITE_OK {
                    if nPage == 0 as Pgno && jrnlOpen == 0 {
                        sqlite3BeginBenignMalloc();
                        if pagerLockDb(pPager, RESERVED_LOCK) == SQLITE_OK {
                            sqlite3OsDelete(
                                pVfs,
                                (*pPager).zJournal,
                                0 as ::core::ffi::c_int,
                            );
                            if (*pPager).exclusiveMode == 0 {
                                pagerUnlockDb(pPager, SHARED_LOCK);
                            }
                        }
                        sqlite3EndBenignMalloc();
                    } else {
                        if jrnlOpen == 0 {
                            let mut f: ::core::ffi::c_int = SQLITE_OPEN_READONLY
                                | SQLITE_OPEN_MAIN_JOURNAL;
                            rc = sqlite3OsOpen(
                                pVfs,
                                (*pPager).zJournal,
                                (*pPager).jfd,
                                f,
                                &raw mut f,
                            );
                        }
                        if rc == SQLITE_OK {
                            let mut first: u8_0 = 0 as u8_0;
                            rc = sqlite3OsRead(
                                (*pPager).jfd,
                                &raw mut first as *mut ::core::ffi::c_void,
                                1 as ::core::ffi::c_int,
                                0 as i64_0,
                            );
                            if rc == SQLITE_IOERR_SHORT_READ {
                                rc = SQLITE_OK;
                            }
                            if jrnlOpen == 0 {
                                sqlite3OsClose((*pPager).jfd);
                            }
                            *pExists = (first as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                        } else if rc == SQLITE_CANTOPEN {
                            *pExists = 1 as ::core::ffi::c_int;
                            rc = SQLITE_OK;
                        }
                    }
                }
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerSharedLock(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).pWal.is_null()
            && (*pPager).eState as ::core::ffi::c_int == PAGER_OPEN
        {
            let mut bHotJournal: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            rc = pager_wait_on_lock(pPager, SHARED_LOCK);
            if rc != SQLITE_OK {
                c2rust_current_block = 6981440318699971291;
            } else {
                if (*pPager).eLock as ::core::ffi::c_int <= SHARED_LOCK {
                    rc = hasHotJournal(pPager, &raw mut bHotJournal);
                }
                if rc != SQLITE_OK {
                    c2rust_current_block = 6981440318699971291;
                } else {
                    if bHotJournal != 0 {
                        if (*pPager).readOnly != 0 {
                            rc = SQLITE_READONLY_ROLLBACK;
                            c2rust_current_block = 6981440318699971291;
                        } else {
                            rc = pagerLockDb(pPager, EXCLUSIVE_LOCK);
                            if rc != SQLITE_OK {
                                c2rust_current_block = 6981440318699971291;
                            } else {
                                if (*(*pPager).jfd).pMethods.is_null()
                                    && (*pPager).journalMode as ::core::ffi::c_int
                                        != PAGER_JOURNALMODE_OFF
                                {
                                    let pVfs: *mut sqlite3_vfs = (*pPager).pVfs;
                                    let mut bExists: ::core::ffi::c_int = 0;
                                    rc = sqlite3OsAccess(
                                        pVfs,
                                        (*pPager).zJournal,
                                        SQLITE_ACCESS_EXISTS,
                                        &raw mut bExists,
                                    );
                                    if rc == SQLITE_OK && bExists != 0 {
                                        let mut fout: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                        let mut f: ::core::ffi::c_int = SQLITE_OPEN_READWRITE
                                            | SQLITE_OPEN_MAIN_JOURNAL;
                                        rc = sqlite3OsOpen(
                                            pVfs,
                                            (*pPager).zJournal,
                                            (*pPager).jfd,
                                            f,
                                            &raw mut fout,
                                        );
                                        if rc == SQLITE_OK && fout & SQLITE_OPEN_READONLY != 0 {
                                            rc = sqlite3CantopenError(5334 as ::core::ffi::c_int);
                                            sqlite3OsClose((*pPager).jfd);
                                        }
                                    }
                                }
                                if !(*(*pPager).jfd).pMethods.is_null() {
                                    rc = pagerSyncHotJournal(pPager);
                                    if rc == SQLITE_OK {
                                        rc = pager_playback(
                                            pPager,
                                            ((*pPager).tempFile == 0) as ::core::ffi::c_int,
                                        );
                                        (*pPager).eState = PAGER_OPEN as u8_0;
                                    }
                                } else if (*pPager).exclusiveMode == 0 {
                                    pagerUnlockDb(pPager, SHARED_LOCK);
                                }
                                if rc != SQLITE_OK {
                                    pager_error(pPager, rc);
                                    c2rust_current_block = 6981440318699971291;
                                } else {
                                    c2rust_current_block = 14072441030219150333;
                                }
                            }
                        }
                    } else {
                        c2rust_current_block = 14072441030219150333;
                    }
                    match c2rust_current_block {
                        6981440318699971291 => {}
                        _ => {
                            if (*pPager).tempFile == 0
                                && (*pPager).hasHeldSharedLock as ::core::ffi::c_int != 0
                            {
                                let mut dbFileVers: [::core::ffi::c_char; 16] = [0; 16];
                                rc = sqlite3OsRead(
                                    (*pPager).fd,
                                    &raw mut dbFileVers as *mut ::core::ffi::c_void,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                                        as ::core::ffi::c_int,
                                    24 as i64_0,
                                );
                                if rc != SQLITE_OK {
                                    if rc != SQLITE_IOERR_SHORT_READ {
                                        c2rust_current_block = 6981440318699971291;
                                    } else {
                                        memset(
                                            &raw mut dbFileVers as *mut ::core::ffi::c_char
                                                as *mut ::core::ffi::c_void,
                                            0 as ::core::ffi::c_int,
                                            ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                                                as size_t,
                                        );
                                        c2rust_current_block = 1622411330066726685;
                                    }
                                } else {
                                    c2rust_current_block = 1622411330066726685;
                                }
                                match c2rust_current_block {
                                    6981440318699971291 => {}
                                    _ => {
                                        if memcmp(
                                            &raw mut (*pPager).dbFileVers as *mut ::core::ffi::c_char
                                                as *const ::core::ffi::c_void,
                                            &raw mut dbFileVers as *mut ::core::ffi::c_char
                                                as *const ::core::ffi::c_void,
                                            ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                                                as size_t,
                                        ) != 0 as ::core::ffi::c_int
                                        {
                                            pager_reset(pPager);
                                            if (*pPager).bUseFetch != 0 {
                                                sqlite3OsUnfetch(
                                                    (*pPager).fd,
                                                    0 as i64_0,
                                                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                                );
                                            }
                                        }
                                        c2rust_current_block = 1423531122933789233;
                                    }
                                }
                            } else {
                                c2rust_current_block = 1423531122933789233;
                            }
                            match c2rust_current_block {
                                6981440318699971291 => {}
                                _ => {
                                    rc = pagerOpenWalIfPresent(pPager);
                                    c2rust_current_block = 7420279277351916581;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            c2rust_current_block = 7420279277351916581;
        }
        match c2rust_current_block {
            7420279277351916581 => {
                if !(*pPager).pWal.is_null() {
                    rc = pagerBeginReadTransaction(pPager);
                }
                if (*pPager).tempFile as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*pPager).eState as ::core::ffi::c_int == PAGER_OPEN
                    && rc == SQLITE_OK
                {
                    rc = pagerPagecount(pPager, &raw mut (*pPager).dbSize);
                }
            }
            _ => {}
        }
        if rc != SQLITE_OK {
            pager_unlock(pPager);
        } else {
            (*pPager).eState = PAGER_READER as u8_0;
            (*pPager).hasHeldSharedLock = 1 as u8_0;
        }
        return rc;
    }
}
unsafe extern "C" fn pagerUnlockIfUnused(mut pPager: *mut Pager) {
    unsafe {
        if sqlite3PcacheRefCount((*pPager).pPCache) == 0 as ::core::ffi::c_longlong {
            pagerUnlockAndRollback(pPager);
        }
    }
}
unsafe extern "C" fn getPageNormal(
    mut pPager: *mut Pager,
    mut pgno: Pgno,
    mut ppPage: *mut *mut DbPage,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pPg: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        let mut noContent: u8_0 = 0;
        let mut pBase: *mut sqlite3_pcache_page = ::core::ptr::null_mut::<
            sqlite3_pcache_page,
        >();
        if pgno == 0 as Pgno {
            return sqlite3CorruptError(5547 as ::core::ffi::c_int);
        }
        pBase = sqlite3PcacheFetch((*pPager).pPCache, pgno, 3 as ::core::ffi::c_int);
        if pBase.is_null() {
            pPg = ::core::ptr::null_mut::<PgHdr>();
            rc = sqlite3PcacheFetchStress((*pPager).pPCache, pgno, &raw mut pBase);
            if rc != SQLITE_OK {
                c2rust_current_block = 3222590281903869779;
            } else if pBase.is_null() {
                rc = SQLITE_NOMEM_BKPT;
                c2rust_current_block = 3222590281903869779;
            } else {
                c2rust_current_block = 7746791466490516765;
            }
        } else {
            c2rust_current_block = 7746791466490516765;
        }
        match c2rust_current_block {
            7746791466490516765 => {
                *ppPage = sqlite3PcacheFetchFinish((*pPager).pPCache, pgno, pBase)
                    as *mut DbPage;
                pPg = *ppPage as *mut PgHdr;
                noContent = (flags & PAGER_GET_NOCONTENT != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as u8_0;
                if !(*pPg).pPager.is_null() && noContent == 0 {
                    (*pPager).aStat[PAGER_STAT_HIT as usize] = (*pPager)
                        .aStat[PAGER_STAT_HIT as usize]
                        .wrapping_add(1);
                    return SQLITE_OK;
                } else if pgno == (*pPager).lckPgno {
                    rc = sqlite3CorruptError(5579 as ::core::ffi::c_int);
                } else {
                    (*pPg).pPager = pPager;
                    if (*(*pPager).fd).pMethods.is_null() || (*pPager).dbSize < pgno
                        || noContent as ::core::ffi::c_int != 0
                    {
                        if pgno > (*pPager).mxPgno {
                            rc = SQLITE_FULL;
                            if pgno <= (*pPager).dbSize {
                                sqlite3PcacheRelease(pPg);
                                pPg = ::core::ptr::null_mut::<PgHdr>();
                            }
                            c2rust_current_block = 3222590281903869779;
                        } else {
                            if noContent != 0 {
                                sqlite3BeginBenignMalloc();
                                if pgno <= (*pPager).dbOrigSize {
                                    sqlite3BitvecSet((*pPager).pInJournal, pgno as u32_0);
                                }
                                addToSavepointBitvecs(pPager, pgno);
                                sqlite3EndBenignMalloc();
                            }
                            memset(
                                (*pPg).pData,
                                0 as ::core::ffi::c_int,
                                (*pPager).pageSize as size_t,
                            );
                            c2rust_current_block = 7427571413727699167;
                        }
                    } else {
                        (*pPager).aStat[PAGER_STAT_MISS as usize] = (*pPager)
                            .aStat[PAGER_STAT_MISS as usize]
                            .wrapping_add(1);
                        rc = readDbPage(pPg);
                        if rc != SQLITE_OK {
                            c2rust_current_block = 3222590281903869779;
                        } else {
                            c2rust_current_block = 7427571413727699167;
                        }
                    }
                    match c2rust_current_block {
                        3222590281903869779 => {}
                        _ => return SQLITE_OK

                    }
                }
            }
            _ => {}
        }
        if !pPg.is_null() {
            sqlite3PcacheDrop(pPg);
        }
        pagerUnlockIfUnused(pPager);
        *ppPage = ::core::ptr::null_mut::<DbPage>();
        return rc;
    }
}
unsafe extern "C" fn getPageMMap(
    mut pPager: *mut Pager,
    mut pgno: Pgno,
    mut ppPage: *mut *mut DbPage,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pPg: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        let mut iFrame: u32_0 = 0 as u32_0;
        let bMmapOk: ::core::ffi::c_int = (pgno > 1 as Pgno
            && ((*pPager).eState as ::core::ffi::c_int == PAGER_READER
                || flags & PAGER_GET_READONLY != 0)) as ::core::ffi::c_int;
        if pgno <= 1 as Pgno && pgno == 0 as Pgno {
            return sqlite3CorruptError(5662 as ::core::ffi::c_int);
        }
        if bMmapOk != 0 && !(*pPager).pWal.is_null() {
            rc = sqlite3WalFindFrame((*pPager).pWal, pgno, &raw mut iFrame);
            if rc != SQLITE_OK {
                *ppPage = ::core::ptr::null_mut::<DbPage>();
                return rc;
            }
        }
        if bMmapOk != 0 && iFrame == 0 as u32_0 {
            let mut pData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            rc = sqlite3OsFetch(
                (*pPager).fd,
                pgno.wrapping_sub(1 as Pgno) as i64_0 * (*pPager).pageSize,
                (*pPager).pageSize as ::core::ffi::c_int,
                &raw mut pData,
            );
            if rc == SQLITE_OK && !pData.is_null() {
                if (*pPager).eState as ::core::ffi::c_int > PAGER_READER
                    || (*pPager).tempFile as ::core::ffi::c_int != 0
                {
                    pPg = sqlite3PagerLookup(pPager, pgno) as *mut PgHdr;
                }
                if pPg.is_null() {
                    rc = pagerAcquireMapPage(pPager, pgno, pData, &raw mut pPg);
                } else {
                    sqlite3OsUnfetch(
                        (*pPager).fd,
                        pgno.wrapping_sub(1 as Pgno) as i64_0 * (*pPager).pageSize,
                        pData,
                    );
                }
                if !pPg.is_null() {
                    *ppPage = pPg as *mut DbPage;
                    return SQLITE_OK;
                }
            }
            if rc != SQLITE_OK {
                *ppPage = ::core::ptr::null_mut::<DbPage>();
                return rc;
            }
        }
        return getPageNormal(pPager, pgno, ppPage, flags);
    }
}
unsafe extern "C" fn getPageError(
    mut pPager: *mut Pager,
    mut pgno: Pgno,
    mut ppPage: *mut *mut DbPage,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        *ppPage = ::core::ptr::null_mut::<DbPage>();
        return (*pPager).errCode;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerGet(
    mut pPager: *mut Pager,
    mut pgno: Pgno,
    mut ppPage: *mut *mut DbPage,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (*pPager)
            .xGet
            .expect("non-null function pointer")(pPager, pgno, ppPage, flags);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerLookup(
    mut pPager: *mut Pager,
    mut pgno: Pgno,
) -> *mut DbPage {
    unsafe {
        let mut pPage: *mut sqlite3_pcache_page = ::core::ptr::null_mut::<
            sqlite3_pcache_page,
        >();
        pPage = sqlite3PcacheFetch((*pPager).pPCache, pgno, 0 as ::core::ffi::c_int);
        if pPage.is_null() {
            return ::core::ptr::null_mut::<DbPage>();
        }
        return sqlite3PcacheFetchFinish((*pPager).pPCache, pgno, pPage) as *mut DbPage;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerUnrefNotNull(mut pPg: *mut DbPage) {
    unsafe {
        if (*pPg).flags as ::core::ffi::c_int & PGHDR_MMAP != 0 {
            pagerReleaseMapPage(pPg as *mut PgHdr);
        } else {
            sqlite3PcacheRelease(pPg as *mut PgHdr);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerUnref(mut pPg: *mut DbPage) {
    unsafe {
        if !pPg.is_null() {
            sqlite3PagerUnrefNotNull(pPg);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerUnrefPageOne(mut pPg: *mut DbPage) {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        pPager = (*pPg).pPager;
        sqlite3PcacheRelease(pPg as *mut PgHdr);
        pagerUnlockIfUnused(pPager);
    }
}
unsafe extern "C" fn pager_open_journal(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let pVfs: *mut sqlite3_vfs = (*pPager).pVfs;
        if (*pPager).errCode != 0 {
            return (*pPager).errCode;
        }
        if (*pPager).pWal.is_null()
            && (*pPager).journalMode as ::core::ffi::c_int != PAGER_JOURNALMODE_OFF
        {
            (*pPager).pInJournal = sqlite3BitvecCreate((*pPager).dbSize as u32_0);
            if (*pPager).pInJournal.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            if (*(*pPager).jfd).pMethods.is_null() {
                if (*pPager).journalMode as ::core::ffi::c_int
                    == PAGER_JOURNALMODE_MEMORY
                {
                    sqlite3MemJournalOpen((*pPager).jfd);
                } else {
                    let mut flags: ::core::ffi::c_int = SQLITE_OPEN_READWRITE
                        | SQLITE_OPEN_CREATE;
                    let mut nSpill: ::core::ffi::c_int = 0;
                    if (*pPager).tempFile != 0 {
                        flags |= SQLITE_OPEN_DELETEONCLOSE | SQLITE_OPEN_TEMP_JOURNAL;
                        flags |= SQLITE_OPEN_EXCLUSIVE;
                        nSpill = sqlite3Config.nStmtSpill;
                    } else {
                        flags |= SQLITE_OPEN_MAIN_JOURNAL;
                        nSpill = jrnlBufferSize(pPager);
                    }
                    rc = databaseIsUnmoved(pPager);
                    if rc == SQLITE_OK {
                        rc = sqlite3JournalOpen(
                            pVfs,
                            (*pPager).zJournal,
                            (*pPager).jfd,
                            flags,
                            nSpill,
                        );
                    }
                }
            }
            if rc == SQLITE_OK {
                (*pPager).nRec = 0 as ::core::ffi::c_int;
                (*pPager).journalOff = 0 as i64_0;
                (*pPager).setSuper = 0 as u8_0;
                (*pPager).journalHdr = 0 as i64_0;
                rc = writeJournalHdr(pPager);
            }
        }
        if rc != SQLITE_OK {
            sqlite3BitvecDestroy((*pPager).pInJournal);
            (*pPager).pInJournal = ::core::ptr::null_mut::<Bitvec>();
            (*pPager).journalOff = 0 as i64_0;
        } else {
            (*pPager).eState = PAGER_WRITER_CACHEMOD as u8_0;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerBegin(
    mut pPager: *mut Pager,
    mut exFlag: ::core::ffi::c_int,
    mut subjInMemory: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).errCode != 0 {
            return (*pPager).errCode;
        }
        (*pPager).subjInMemory = subjInMemory as u8_0;
        if (*pPager).eState as ::core::ffi::c_int == PAGER_READER {
            if !(*pPager).pWal.is_null() {
                if (*pPager).exclusiveMode as ::core::ffi::c_int != 0
                    && sqlite3WalExclusiveMode((*pPager).pWal, -1 as ::core::ffi::c_int)
                        != 0
                {
                    rc = pagerLockDb(pPager, EXCLUSIVE_LOCK);
                    if rc != SQLITE_OK {
                        return rc;
                    }
                    sqlite3WalExclusiveMode((*pPager).pWal, 1 as ::core::ffi::c_int);
                }
                rc = sqlite3WalBeginWriteTransaction((*pPager).pWal);
            } else {
                rc = pagerLockDb(pPager, RESERVED_LOCK);
                if rc == SQLITE_OK && exFlag != 0 {
                    rc = pager_wait_on_lock(pPager, EXCLUSIVE_LOCK);
                }
            }
            if rc == SQLITE_OK {
                (*pPager).eState = PAGER_WRITER_LOCKED as u8_0;
                (*pPager).dbHintSize = (*pPager).dbSize;
                (*pPager).dbFileSize = (*pPager).dbSize;
                (*pPager).dbOrigSize = (*pPager).dbSize;
                (*pPager).journalOff = 0 as i64_0;
            }
        }
        return rc;
    }
}
#[inline(never)]
unsafe extern "C" fn pagerAddPageToRollbackJournal(
    mut pPg: *mut PgHdr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = (*pPg).pPager;
        let mut rc: ::core::ffi::c_int = 0;
        let mut cksum: u32_0 = 0;
        let mut pData2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut iOff: i64_0 = (*pPager).journalOff;
        pData2 = (*pPg).pData as *mut ::core::ffi::c_char;
        cksum = pager_cksum(pPager, pData2 as *mut u8_0);
        (*pPg).flags = ((*pPg).flags as ::core::ffi::c_int | PGHDR_NEED_SYNC) as u16_0;
        rc = write32bits((*pPager).jfd, iOff, (*pPg).pgno as u32_0);
        if rc != SQLITE_OK {
            return rc;
        }
        rc = sqlite3OsWrite(
            (*pPager).jfd,
            pData2 as *const ::core::ffi::c_void,
            (*pPager).pageSize as ::core::ffi::c_int,
            iOff + 4 as i64_0,
        );
        if rc != SQLITE_OK {
            return rc;
        }
        rc = write32bits((*pPager).jfd, iOff + (*pPager).pageSize + 4 as i64_0, cksum);
        if rc != SQLITE_OK {
            return rc;
        }
        sqlite3_pager_writej_count += 1;
        (*pPager).journalOff
            += (8 as i64_0 + (*pPager).pageSize) as ::core::ffi::c_longlong;
        (*pPager).nRec += 1;
        rc = sqlite3BitvecSet((*pPager).pInJournal, (*pPg).pgno as u32_0);
        rc |= addToSavepointBitvecs(pPager, (*pPg).pgno);
        return rc;
    }
}
unsafe extern "C" fn pager_write(mut pPg: *mut PgHdr) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = (*pPg).pPager;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).eState as ::core::ffi::c_int == PAGER_WRITER_LOCKED {
            rc = pager_open_journal(pPager);
            if rc != SQLITE_OK {
                return rc;
            }
        }
        sqlite3PcacheMakeDirty(pPg);
        if !(*pPager).pInJournal.is_null()
            && sqlite3BitvecTestNotNull((*pPager).pInJournal, (*pPg).pgno as u32_0)
                == 0 as ::core::ffi::c_int
        {
            if (*pPg).pgno <= (*pPager).dbOrigSize {
                rc = pagerAddPageToRollbackJournal(pPg);
                if rc != SQLITE_OK {
                    return rc;
                }
            } else if (*pPager).eState as ::core::ffi::c_int != PAGER_WRITER_DBMOD {
                (*pPg).flags = ((*pPg).flags as ::core::ffi::c_int | PGHDR_NEED_SYNC)
                    as u16_0;
            }
        }
        (*pPg).flags = ((*pPg).flags as ::core::ffi::c_int | PGHDR_WRITEABLE) as u16_0;
        if (*pPager).nSavepoint > 0 as ::core::ffi::c_int {
            rc = subjournalPageIfRequired(pPg);
        }
        if (*pPager).dbSize < (*pPg).pgno {
            (*pPager).dbSize = (*pPg).pgno;
        }
        return rc;
    }
}
#[inline(never)]
unsafe extern "C" fn pagerWriteLargeSector(mut pPg: *mut PgHdr) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nPageCount: Pgno = 0;
        let mut pg1: Pgno = 0;
        let mut nPage: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ii: ::core::ffi::c_int = 0;
        let mut needSync: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pPager: *mut Pager = (*pPg).pPager;
        let mut nPagePerSector: Pgno = ((*pPager).sectorSize as i64_0
            / (*pPager).pageSize) as Pgno;
        (*pPager).doNotSpill = ((*pPager).doNotSpill as ::core::ffi::c_int
            | SPILLFLAG_NOSYNC) as u8_0;
        pg1 = ((*pPg).pgno.wrapping_sub(1 as Pgno)
            & !nPagePerSector.wrapping_sub(1 as Pgno))
            .wrapping_add(1 as Pgno);
        nPageCount = (*pPager).dbSize;
        if (*pPg).pgno > nPageCount {
            nPage = (*pPg).pgno.wrapping_sub(pg1).wrapping_add(1 as Pgno)
                as ::core::ffi::c_int;
        } else if pg1.wrapping_add(nPagePerSector).wrapping_sub(1 as Pgno) > nPageCount {
            nPage = nPageCount.wrapping_add(1 as Pgno).wrapping_sub(pg1)
                as ::core::ffi::c_int;
        } else {
            nPage = nPagePerSector as ::core::ffi::c_int;
        }
        ii = 0 as ::core::ffi::c_int;
        while ii < nPage && rc == SQLITE_OK {
            let mut pg: Pgno = pg1.wrapping_add(ii as Pgno);
            let mut pPage: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
            if pg == (*pPg).pgno
                || sqlite3BitvecTest((*pPager).pInJournal, pg as u32_0) == 0
            {
                if pg != (*pPager).lckPgno {
                    rc = sqlite3PagerGet(
                        pPager,
                        pg,
                        &raw mut pPage,
                        0 as ::core::ffi::c_int,
                    );
                    if rc == SQLITE_OK {
                        rc = pager_write(pPage);
                        if (*pPage).flags as ::core::ffi::c_int & PGHDR_NEED_SYNC != 0 {
                            needSync = 1 as ::core::ffi::c_int;
                        }
                        sqlite3PagerUnrefNotNull(pPage as *mut DbPage);
                    }
                }
            } else {
                pPage = sqlite3PagerLookup(pPager, pg) as *mut PgHdr;
                if !pPage.is_null() {
                    if (*pPage).flags as ::core::ffi::c_int & PGHDR_NEED_SYNC != 0 {
                        needSync = 1 as ::core::ffi::c_int;
                    }
                    sqlite3PagerUnrefNotNull(pPage as *mut DbPage);
                }
            }
            ii += 1;
        }
        if rc == SQLITE_OK && needSync != 0 {
            ii = 0 as ::core::ffi::c_int;
            while ii < nPage {
                let mut pPage_0: *mut PgHdr = sqlite3PagerLookup(
                    pPager,
                    pg1.wrapping_add(ii as Pgno),
                ) as *mut PgHdr;
                if !pPage_0.is_null() {
                    (*pPage_0).flags = ((*pPage_0).flags as ::core::ffi::c_int
                        | PGHDR_NEED_SYNC) as u16_0;
                    sqlite3PagerUnrefNotNull(pPage_0 as *mut DbPage);
                }
                ii += 1;
            }
        }
        (*pPager).doNotSpill = ((*pPager).doNotSpill as ::core::ffi::c_int
            & !SPILLFLAG_NOSYNC) as u8_0;
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerWrite(mut pPg: *mut PgHdr) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = (*pPg).pPager;
        if (*pPg).flags as ::core::ffi::c_int & PGHDR_WRITEABLE
            != 0 as ::core::ffi::c_int && (*pPager).dbSize >= (*pPg).pgno
        {
            if (*pPager).nSavepoint != 0 {
                return subjournalPageIfRequired(pPg);
            }
            return SQLITE_OK;
        } else if (*pPager).errCode != 0 {
            return (*pPager).errCode
        } else if (*pPager).sectorSize > (*pPager).pageSize as u32_0 {
            return pagerWriteLargeSector(pPg)
        } else {
            return pager_write(pPg)
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerDontWrite(mut pPg: *mut PgHdr) {
    unsafe {
        let mut pPager: *mut Pager = (*pPg).pPager;
        if (*pPager).tempFile == 0
            && (*pPg).flags as ::core::ffi::c_int & PGHDR_DIRTY != 0
            && (*pPager).nSavepoint == 0 as ::core::ffi::c_int
        {
            (*pPg).flags = ((*pPg).flags as ::core::ffi::c_int | PGHDR_DONT_WRITE)
                as u16_0;
            (*pPg).flags = ((*pPg).flags as ::core::ffi::c_int & !PGHDR_WRITEABLE)
                as u16_0;
        }
    }
}
unsafe extern "C" fn pager_incr_changecounter(
    mut pPager: *mut Pager,
    mut isDirectMode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).changeCountDone == 0 && (*pPager).dbSize > 0 as Pgno {
            let mut pPgHdr: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
            rc = sqlite3PagerGet(
                pPager,
                1 as Pgno,
                &raw mut pPgHdr,
                0 as ::core::ffi::c_int,
            );
            if DIRECT_MODE == 0 && rc == 0 as ::core::ffi::c_int {
                rc = sqlite3PagerWrite(pPgHdr);
            }
            if rc == SQLITE_OK {
                pager_write_changecounter(pPgHdr);
                (*pPager).changeCountDone = 1 as u8_0;
            }
            sqlite3PagerUnref(pPgHdr as *mut DbPage);
        }
        return rc;
    }
}
pub const DIRECT_MODE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerSync(
    mut pPager: *mut Pager,
    mut zSuper: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pArg: *mut ::core::ffi::c_void = zSuper as *mut ::core::ffi::c_void;
        rc = sqlite3OsFileControl((*pPager).fd, SQLITE_FCNTL_SYNC, pArg);
        if rc == SQLITE_NOTFOUND {
            rc = SQLITE_OK;
        }
        if rc == SQLITE_OK && (*pPager).noSync == 0 {
            rc = sqlite3OsSync((*pPager).fd, (*pPager).syncFlags as ::core::ffi::c_int);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerExclusiveLock(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = (*pPager).errCode;
        if rc == SQLITE_OK {
            if 0 as ::core::ffi::c_int == !(*pPager).pWal.is_null() as ::core::ffi::c_int
            {
                rc = pager_wait_on_lock(pPager, EXCLUSIVE_LOCK);
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerCommitPhaseOne(
    mut pPager: *mut Pager,
    mut zSuper: *const ::core::ffi::c_char,
    mut noSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).errCode != 0 {
            return (*pPager).errCode;
        }
        if sqlite3FaultSim(400 as ::core::ffi::c_int) != 0 {
            return SQLITE_IOERR;
        }
        if ((*pPager).eState as ::core::ffi::c_int) < PAGER_WRITER_CACHEMOD {
            return SQLITE_OK;
        }
        if 0 as ::core::ffi::c_int == pagerFlushOnCommit(pPager, 1 as ::core::ffi::c_int)
        {
            sqlite3BackupRestart((*pPager).pBackup);
        } else {
            let mut pList: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
            if !(*pPager).pWal.is_null() {
                let mut pPageOne: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
                pList = sqlite3PcacheDirtyList((*pPager).pPCache);
                if pList.is_null() {
                    rc = sqlite3PagerGet(
                        pPager,
                        1 as Pgno,
                        &raw mut pPageOne,
                        0 as ::core::ffi::c_int,
                    );
                    pList = pPageOne;
                    (*pList).pDirty = ::core::ptr::null_mut::<PgHdr>();
                }
                if !pList.is_null() {
                    rc = pagerWalFrames(
                        pPager,
                        pList,
                        (*pPager).dbSize,
                        1 as ::core::ffi::c_int,
                    );
                }
                sqlite3PagerUnref(pPageOne as *mut DbPage);
                if rc == SQLITE_OK {
                    sqlite3PcacheCleanAll((*pPager).pPCache);
                }
            } else {
                rc = pager_incr_changecounter(pPager, 0 as ::core::ffi::c_int);
                if !(rc != SQLITE_OK) {
                    rc = writeSuperJournal(pPager, zSuper);
                    if !(rc != SQLITE_OK) {
                        rc = syncJournal(pPager, 0 as ::core::ffi::c_int);
                        if !(rc != SQLITE_OK) {
                            pList = sqlite3PcacheDirtyList((*pPager).pPCache);
                            if bBatch == 0 as ::core::ffi::c_int {
                                rc = pager_write_pagelist(pPager, pList);
                            }
                            if !(rc != SQLITE_OK) {
                                sqlite3PcacheCleanAll((*pPager).pPCache);
                                if (*pPager).dbSize > (*pPager).dbFileSize {
                                    let mut nNew: Pgno = (*pPager)
                                        .dbSize
                                        .wrapping_sub(
                                            ((*pPager).dbSize == (*pPager).lckPgno)
                                                as ::core::ffi::c_int as Pgno,
                                        );
                                    rc = pager_truncate(pPager, nNew);
                                    if rc != SQLITE_OK {
                                        c2rust_current_block = 3607274197126565213;
                                    } else {
                                        c2rust_current_block = 6450636197030046351;
                                    }
                                } else {
                                    c2rust_current_block = 6450636197030046351;
                                }
                                match c2rust_current_block {
                                    3607274197126565213 => {}
                                    _ => {
                                        if noSync == 0 {
                                            rc = sqlite3PagerSync(pPager, zSuper);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if rc == SQLITE_OK && (*pPager).pWal.is_null() {
            (*pPager).eState = PAGER_WRITER_FINISHED as u8_0;
        }
        return rc;
    }
}
pub const bBatch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerCommitPhaseTwo(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).errCode != 0 {
            return (*pPager).errCode;
        }
        (*pPager).iDataVersion = (*pPager).iDataVersion.wrapping_add(1);
        if (*pPager).eState as ::core::ffi::c_int == PAGER_WRITER_LOCKED
            && (*pPager).exclusiveMode as ::core::ffi::c_int != 0
            && (*pPager).journalMode as ::core::ffi::c_int == PAGER_JOURNALMODE_PERSIST
        {
            (*pPager).eState = PAGER_READER as u8_0;
            return SQLITE_OK;
        }
        rc = pager_end_transaction(
            pPager,
            (*pPager).setSuper as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        return pager_error(pPager, rc);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerRollback(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).eState as ::core::ffi::c_int == PAGER_ERROR {
            return (*pPager).errCode;
        }
        if (*pPager).eState as ::core::ffi::c_int <= PAGER_READER {
            return SQLITE_OK;
        }
        if !(*pPager).pWal.is_null() {
            let mut rc2: ::core::ffi::c_int = 0;
            rc = sqlite3PagerSavepoint(
                pPager,
                SAVEPOINT_ROLLBACK,
                -1 as ::core::ffi::c_int,
            );
            rc2 = pager_end_transaction(
                pPager,
                (*pPager).setSuper as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if rc == SQLITE_OK {
                rc = rc2;
            }
        } else if (*(*pPager).jfd).pMethods.is_null()
            || (*pPager).eState as ::core::ffi::c_int == PAGER_WRITER_LOCKED
        {
            let mut eState: ::core::ffi::c_int = (*pPager).eState as ::core::ffi::c_int;
            rc = pager_end_transaction(
                pPager,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if (*pPager).memDb == 0 && eState > PAGER_WRITER_LOCKED {
                (*pPager).errCode = SQLITE_ABORT;
                (*pPager).eState = PAGER_ERROR as u8_0;
                setGetterMethod(pPager);
                return rc;
            }
        } else {
            rc = pager_playback(pPager, 0 as ::core::ffi::c_int);
        }
        return pager_error(pPager, rc);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerIsreadonly(mut pPager: *mut Pager) -> u8_0 {
    unsafe {
        return (*pPager).readOnly;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerMemUsed(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        let mut perPageSize: ::core::ffi::c_int = ((*pPager).pageSize
            as ::core::ffi::c_longlong + (*pPager).nExtra as ::core::ffi::c_longlong
            + (::core::mem::size_of::<PgHdr>() as usize)
                .wrapping_add(
                    (5 as usize)
                        .wrapping_mul(
                            ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize,
                        ),
                ) as ::core::ffi::c_int as ::core::ffi::c_longlong)
            as ::core::ffi::c_int;
        return ((perPageSize * sqlite3PcachePagecount((*pPager).pPCache)
            + sqlite3MallocSize(pPager as *const ::core::ffi::c_void)) as i64_0
            + (*pPager).pageSize) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerPageRefcount(
    mut pPage: *mut DbPage,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3PcachePageRefcount(pPage as *mut PgHdr) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerStats(
    mut pPager: *mut Pager,
) -> *mut ::core::ffi::c_int {
    unsafe {
        static mut a: [::core::ffi::c_int; 11] = [0; 11];
        a[0 as ::core::ffi::c_int as usize] = sqlite3PcacheRefCount((*pPager).pPCache)
            as ::core::ffi::c_int;
        a[1 as ::core::ffi::c_int as usize] = sqlite3PcachePagecount((*pPager).pPCache);
        a[2 as ::core::ffi::c_int as usize] = sqlite3PcacheGetCachesize(
            (*pPager).pPCache,
        );
        a[3 as ::core::ffi::c_int as usize] = if (*pPager).eState as ::core::ffi::c_int
            == PAGER_OPEN
        {
            -1 as ::core::ffi::c_int
        } else {
            (*pPager).dbSize as ::core::ffi::c_int
        };
        a[4 as ::core::ffi::c_int as usize] = (*pPager).eState as ::core::ffi::c_int;
        a[5 as ::core::ffi::c_int as usize] = (*pPager).errCode;
        a[6 as ::core::ffi::c_int as usize] = (*pPager).aStat[PAGER_STAT_HIT as usize]
            as ::core::ffi::c_int & 0x7fffffff as ::core::ffi::c_int;
        a[7 as ::core::ffi::c_int as usize] = (*pPager).aStat[PAGER_STAT_MISS as usize]
            as ::core::ffi::c_int & 0x7fffffff as ::core::ffi::c_int;
        a[8 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
        a[9 as ::core::ffi::c_int as usize] = (*pPager).nRead;
        a[10 as ::core::ffi::c_int as usize] = (*pPager).aStat[PAGER_STAT_WRITE as usize]
            as ::core::ffi::c_int & 0x7fffffff as ::core::ffi::c_int;
        return &raw mut a as *mut ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerCacheStat(
    mut pPager: *mut Pager,
    mut eStat: ::core::ffi::c_int,
    mut reset: ::core::ffi::c_int,
    mut pnVal: *mut u64_0,
) {
    unsafe {
        eStat -= SQLITE_DBSTATUS_CACHE_HIT;
        *pnVal = (*pnVal as ::core::ffi::c_ulonglong)
            .wrapping_add((*pPager).aStat[eStat as usize] as ::core::ffi::c_ulonglong)
            as u64_0 as u64_0;
        if reset != 0 {
            (*pPager).aStat[eStat as usize] = 0 as u32_0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerIsMemdb(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        return ((*pPager).tempFile as ::core::ffi::c_int != 0
            || (*pPager).memVfs as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn pagerOpenSavepoint(
    mut pPager: *mut Pager,
    mut nSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nCurrent: ::core::ffi::c_int = (*pPager).nSavepoint;
        let mut ii: ::core::ffi::c_int = 0;
        let mut aNew: *mut PagerSavepoint = ::core::ptr::null_mut::<PagerSavepoint>();
        aNew = sqlite3Realloc(
            (*pPager).aSavepoint as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<PagerSavepoint>() as usize)
                .wrapping_mul(nSavepoint as usize) as u64_0,
        ) as *mut PagerSavepoint;
        if aNew.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        memset(
            aNew.offset(nCurrent as isize) as *mut PagerSavepoint
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((nSavepoint - nCurrent) as size_t)
                .wrapping_mul(::core::mem::size_of::<PagerSavepoint>() as size_t),
        );
        (*pPager).aSavepoint = aNew;
        ii = nCurrent;
        while ii < nSavepoint {
            (*aNew.offset(ii as isize)).nOrig = (*pPager).dbSize;
            if !(*(*pPager).jfd).pMethods.is_null()
                && (*pPager).journalOff > 0 as ::core::ffi::c_longlong
            {
                (*aNew.offset(ii as isize)).iOffset = (*pPager).journalOff;
            } else {
                (*aNew.offset(ii as isize)).iOffset = (*pPager).sectorSize as i64_0;
            }
            (*aNew.offset(ii as isize)).iSubRec = (*pPager).nSubRec as Pgno;
            let ref mut c2rust_fresh3 = (*aNew.offset(ii as isize)).pInSavepoint;
            *c2rust_fresh3 = sqlite3BitvecCreate((*pPager).dbSize as u32_0);
            (*aNew.offset(ii as isize)).bTruncateOnRelease = 1 as ::core::ffi::c_int;
            if (*aNew.offset(ii as isize)).pInSavepoint.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            if !(*pPager).pWal.is_null() {
                sqlite3WalSavepoint(
                    (*pPager).pWal,
                    &raw mut (*aNew.offset(ii as isize)).aWalData as *mut u32_0,
                );
            }
            (*pPager).nSavepoint = ii + 1 as ::core::ffi::c_int;
            ii += 1;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerOpenSavepoint(
    mut pPager: *mut Pager,
    mut nSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if nSavepoint > (*pPager).nSavepoint
            && (*pPager).useJournal as ::core::ffi::c_int != 0
        {
            return pagerOpenSavepoint(pPager, nSavepoint)
        } else {
            return SQLITE_OK
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerSavepoint(
    mut pPager: *mut Pager,
    mut op: ::core::ffi::c_int,
    mut iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = (*pPager).errCode;
        if rc == SQLITE_OK && iSavepoint < (*pPager).nSavepoint {
            let mut ii: ::core::ffi::c_int = 0;
            let mut nNew: ::core::ffi::c_int = 0;
            nNew = iSavepoint
                + (if op == SAVEPOINT_RELEASE {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                });
            ii = nNew;
            while ii < (*pPager).nSavepoint {
                sqlite3BitvecDestroy(
                    (*(*pPager).aSavepoint.offset(ii as isize)).pInSavepoint,
                );
                ii += 1;
            }
            (*pPager).nSavepoint = nNew;
            if op == SAVEPOINT_RELEASE {
                let mut pRel: *mut PagerSavepoint = (*pPager)
                    .aSavepoint
                    .offset(nNew as isize) as *mut PagerSavepoint;
                if (*pRel).bTruncateOnRelease != 0
                    && !(*(*pPager).sjfd).pMethods.is_null()
                {
                    if sqlite3JournalIsInMemory((*pPager).sjfd) != 0 {
                        let mut sz: i64_0 = ((*pPager).pageSize + 4 as i64_0)
                            * (*pRel).iSubRec as i64_0;
                        rc = sqlite3OsTruncate((*pPager).sjfd, sz);
                    }
                    (*pPager).nSubRec = (*pRel).iSubRec as u32_0;
                }
            } else if !(*pPager).pWal.is_null() || !(*(*pPager).jfd).pMethods.is_null() {
                let mut pSavepoint: *mut PagerSavepoint = if nNew
                    == 0 as ::core::ffi::c_int
                {
                    ::core::ptr::null_mut::<PagerSavepoint>()
                } else {
                    (*pPager)
                        .aSavepoint
                        .offset((nNew - 1 as ::core::ffi::c_int) as isize)
                        as *mut PagerSavepoint
                };
                rc = pagerPlaybackSavepoint(pPager, pSavepoint);
            }
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerFilename(
    mut pPager: *const Pager,
    mut nullIfMemDb: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        static mut zFake: [::core::ffi::c_char; 8] = [
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
        ];
        if nullIfMemDb != 0
            && ((*pPager).memDb as ::core::ffi::c_int != 0
                || sqlite3IsMemdb((*pPager).pVfs) != 0)
        {
            return (&raw const zFake as *const ::core::ffi::c_char)
                .offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
        } else {
            return (*pPager).zFilename
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerVfs(mut pPager: *mut Pager) -> *mut sqlite3_vfs {
    unsafe {
        return (*pPager).pVfs;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerFile(mut pPager: *mut Pager) -> *mut sqlite3_file {
    unsafe {
        return (*pPager).fd;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerJrnlFile(
    mut pPager: *mut Pager,
) -> *mut sqlite3_file {
    unsafe {
        return if !(*pPager).pWal.is_null() {
            sqlite3WalFile((*pPager).pWal)
        } else {
            (*pPager).jfd
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerJournalname(
    mut pPager: *mut Pager,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*pPager).zJournal;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerMovepage(
    mut pPager: *mut Pager,
    mut pPg: *mut DbPage,
    mut pgno: Pgno,
    mut isCommit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPgOld: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        let mut needSyncPgno: Pgno = 0 as Pgno;
        let mut rc: ::core::ffi::c_int = 0;
        let mut origPgno: Pgno = 0;
        if (*pPager).tempFile != 0 {
            rc = sqlite3PagerWrite(pPg as *mut PgHdr);
            if rc != 0 {
                return rc;
            }
        }
        if (*pPg).flags as ::core::ffi::c_int & PGHDR_DIRTY != 0 as ::core::ffi::c_int
            && {
                rc = subjournalPageIfRequired(pPg as *mut PgHdr);
                SQLITE_OK != rc
            }
        {
            return rc;
        }
        if (*pPg).flags as ::core::ffi::c_int & PGHDR_NEED_SYNC != 0 && isCommit == 0 {
            needSyncPgno = (*pPg).pgno;
        }
        (*pPg).flags = ((*pPg).flags as ::core::ffi::c_int & !PGHDR_NEED_SYNC) as u16_0;
        pPgOld = sqlite3PagerLookup(pPager, pgno) as *mut PgHdr;
        if !pPgOld.is_null() {
            if (*pPgOld).nRef > 1 as ::core::ffi::c_longlong {
                sqlite3PagerUnrefNotNull(pPgOld as *mut DbPage);
                return sqlite3CorruptError(7228 as ::core::ffi::c_int);
            }
            (*pPg).flags = ((*pPg).flags as ::core::ffi::c_int
                | (*pPgOld).flags as ::core::ffi::c_int & PGHDR_NEED_SYNC) as u16_0;
            if (*pPager).tempFile != 0 {
                sqlite3PcacheMove(pPgOld, (*pPager).dbSize.wrapping_add(1 as Pgno));
            } else {
                sqlite3PcacheDrop(pPgOld);
            }
        }
        origPgno = (*pPg).pgno;
        sqlite3PcacheMove(pPg as *mut PgHdr, pgno);
        sqlite3PcacheMakeDirty(pPg as *mut PgHdr);
        if (*pPager).tempFile as ::core::ffi::c_int != 0 && !pPgOld.is_null() {
            sqlite3PcacheMove(pPgOld, origPgno);
            sqlite3PagerUnrefNotNull(pPgOld as *mut DbPage);
        }
        if needSyncPgno != 0 {
            let mut pPgHdr: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
            rc = sqlite3PagerGet(
                pPager,
                needSyncPgno,
                &raw mut pPgHdr,
                0 as ::core::ffi::c_int,
            );
            if rc != SQLITE_OK {
                if needSyncPgno <= (*pPager).dbOrigSize {
                    sqlite3BitvecClear(
                        (*pPager).pInJournal,
                        needSyncPgno as u32_0,
                        (*pPager).pTmpSpace as *mut ::core::ffi::c_void,
                    );
                }
                return rc;
            }
            (*pPgHdr).flags = ((*pPgHdr).flags as ::core::ffi::c_int | PGHDR_NEED_SYNC)
                as u16_0;
            sqlite3PcacheMakeDirty(pPgHdr);
            sqlite3PagerUnrefNotNull(pPgHdr as *mut DbPage);
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerRekey(
    mut pPg: *mut DbPage,
    mut iNew: Pgno,
    mut flags: u16_0,
) {
    unsafe {
        (*pPg).flags = flags;
        sqlite3PcacheMove(pPg as *mut PgHdr, iNew);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerGetData(
    mut pPg: *mut DbPage,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*pPg).pData;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerGetExtra(
    mut pPg: *mut DbPage,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*pPg).pExtra;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerLockingMode(
    mut pPager: *mut Pager,
    mut eMode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if eMode >= 0 as ::core::ffi::c_int && (*pPager).tempFile == 0
            && sqlite3WalHeapMemory((*pPager).pWal) == 0
        {
            (*pPager).exclusiveMode = eMode as u8_0;
        }
        return (*pPager).exclusiveMode as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerSetJournalMode(
    mut pPager: *mut Pager,
    mut eMode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eOld: u8_0 = (*pPager).journalMode;
        if (*pPager).memDb != 0 {
            if eMode != PAGER_JOURNALMODE_MEMORY && eMode != PAGER_JOURNALMODE_OFF {
                eMode = eOld as ::core::ffi::c_int;
            }
        }
        if eMode != eOld as ::core::ffi::c_int {
            (*pPager).journalMode = eMode as u8_0;
            if (*pPager).exclusiveMode == 0
                && eOld as ::core::ffi::c_int & 5 as ::core::ffi::c_int
                    == 1 as ::core::ffi::c_int
                && eMode & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                sqlite3OsClose((*pPager).jfd);
                if (*pPager).eLock as ::core::ffi::c_int >= RESERVED_LOCK {
                    sqlite3OsDelete(
                        (*pPager).pVfs,
                        (*pPager).zJournal,
                        0 as ::core::ffi::c_int,
                    );
                } else {
                    let mut rc: ::core::ffi::c_int = SQLITE_OK;
                    let mut state: ::core::ffi::c_int = (*pPager).eState
                        as ::core::ffi::c_int;
                    if state == PAGER_OPEN {
                        rc = sqlite3PagerSharedLock(pPager);
                    }
                    if (*pPager).eState as ::core::ffi::c_int == PAGER_READER {
                        rc = pagerLockDb(pPager, RESERVED_LOCK);
                    }
                    if rc == SQLITE_OK {
                        sqlite3OsDelete(
                            (*pPager).pVfs,
                            (*pPager).zJournal,
                            0 as ::core::ffi::c_int,
                        );
                    }
                    if rc == SQLITE_OK && state == PAGER_READER {
                        pagerUnlockDb(pPager, SHARED_LOCK);
                    } else if state == PAGER_OPEN {
                        pager_unlock(pPager);
                    }
                }
            } else if eMode == PAGER_JOURNALMODE_OFF || eMode == PAGER_JOURNALMODE_MEMORY
            {
                sqlite3OsClose((*pPager).jfd);
            }
        }
        return (*pPager).journalMode as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerGetJournalMode(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        return (*pPager).journalMode as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerOkToChangeJournalMode(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pPager).eState as ::core::ffi::c_int >= PAGER_WRITER_CACHEMOD {
            return 0 as ::core::ffi::c_int;
        }
        if !(*(*pPager).jfd).pMethods.is_null()
            && (*pPager).journalOff > 0 as ::core::ffi::c_longlong
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerJournalSizeLimit(
    mut pPager: *mut Pager,
    mut iLimit: i64_0,
) -> i64_0 {
    unsafe {
        if iLimit >= -1 as ::core::ffi::c_int as ::core::ffi::c_longlong {
            (*pPager).journalSizeLimit = iLimit;
            sqlite3WalLimit((*pPager).pWal, iLimit);
        }
        return (*pPager).journalSizeLimit;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerBackupPtr(
    mut pPager: *mut Pager,
) -> *mut *mut sqlite3_backup {
    unsafe {
        return &raw mut (*pPager).pBackup;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerClearCache(mut pPager: *mut Pager) {
    unsafe {
        if (*pPager).tempFile as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            pager_reset(pPager);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerCheckpoint(
    mut pPager: *mut Pager,
    mut db: *mut sqlite3,
    mut eMode: ::core::ffi::c_int,
    mut pnLog: *mut ::core::ffi::c_int,
    mut pnCkpt: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).pWal.is_null()
            && (*pPager).journalMode as ::core::ffi::c_int == PAGER_JOURNALMODE_WAL
        {
            sqlite3_exec(
                db,
                b"PRAGMA table_list\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
        if !(*pPager).pWal.is_null() {
            rc = sqlite3WalCheckpoint(
                (*pPager).pWal,
                db,
                eMode,
                if eMode <= SQLITE_CHECKPOINT_PASSIVE {
                    None
                } else {
                    (*pPager).xBusyHandler
                },
                (*pPager).pBusyHandlerArg,
                (*pPager).walSyncFlags as ::core::ffi::c_int,
                (*pPager).pageSize as ::core::ffi::c_int,
                (*pPager).pTmpSpace as *mut u8_0,
                pnLog,
                pnCkpt,
            );
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerWalCallback(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3WalCallback((*pPager).pWal);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerWalSupported(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pMethods: *const sqlite3_io_methods = (*(*pPager).fd).pMethods
            as *const sqlite3_io_methods;
        if (*pPager).noLock != 0 {
            return 0 as ::core::ffi::c_int;
        }
        return ((*pPager).exclusiveMode as ::core::ffi::c_int != 0
            || (*pMethods).iVersion >= 2 as ::core::ffi::c_int
                && (*pMethods).xShmMap.is_some()) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn pagerExclusiveLock(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut eOrigLock: u8_0 = 0;
        eOrigLock = (*pPager).eLock;
        rc = pagerLockDb(pPager, EXCLUSIVE_LOCK);
        if rc != SQLITE_OK {
            pagerUnlockDb(pPager, eOrigLock as ::core::ffi::c_int);
        }
        return rc;
    }
}
unsafe extern "C" fn pagerOpenWal(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).exclusiveMode != 0 {
            rc = pagerExclusiveLock(pPager);
        }
        if rc == SQLITE_OK {
            rc = sqlite3WalOpen(
                (*pPager).pVfs,
                (*pPager).fd,
                (*pPager).zWal,
                (*pPager).exclusiveMode as ::core::ffi::c_int,
                (*pPager).journalSizeLimit,
                &raw mut (*pPager).pWal,
            );
        }
        pagerFixMaplimit(pPager);
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerOpenWal(
    mut pPager: *mut Pager,
    mut pbOpen: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).tempFile == 0 && (*pPager).pWal.is_null() {
            if sqlite3PagerWalSupported(pPager) == 0 {
                return SQLITE_CANTOPEN;
            }
            sqlite3OsClose((*pPager).jfd);
            rc = pagerOpenWal(pPager);
            if rc == SQLITE_OK {
                (*pPager).journalMode = PAGER_JOURNALMODE_WAL as u8_0;
                (*pPager).eState = PAGER_OPEN as u8_0;
            }
        } else {
            *pbOpen = 1 as ::core::ffi::c_int;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PagerCloseWal(
    mut pPager: *mut Pager,
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pPager).pWal.is_null() {
            let mut logexists: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            rc = pagerLockDb(pPager, SHARED_LOCK);
            if rc == SQLITE_OK {
                rc = sqlite3OsAccess(
                    (*pPager).pVfs,
                    (*pPager).zWal,
                    SQLITE_ACCESS_EXISTS,
                    &raw mut logexists,
                );
            }
            if rc == SQLITE_OK && logexists != 0 {
                rc = pagerOpenWal(pPager);
            }
        }
        if rc == SQLITE_OK && !(*pPager).pWal.is_null() {
            rc = pagerExclusiveLock(pPager);
            if rc == SQLITE_OK {
                rc = sqlite3WalClose(
                    (*pPager).pWal,
                    db,
                    (*pPager).walSyncFlags as ::core::ffi::c_int,
                    (*pPager).pageSize as ::core::ffi::c_int,
                    (*pPager).pTmpSpace as *mut u8_0,
                );
                (*pPager).pWal = ::core::ptr::null_mut::<Wal>();
                pagerFixMaplimit(pPager);
                if rc != 0 && (*pPager).exclusiveMode == 0 {
                    pagerUnlockDb(pPager, SHARED_LOCK);
                }
            }
        }
        return rc;
    }
}
pub const SQLITE_DEFAULT_PAGE_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const __SIZEOF_POINTER__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
