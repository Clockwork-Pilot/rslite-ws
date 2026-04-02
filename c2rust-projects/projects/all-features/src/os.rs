extern "C" {
    pub type sqlite3_mutex;
    pub type sqlite3_pcache;
    fn sqlite3_initialize() -> ::core::ffi::c_int;
    fn sqlite3_os_init() -> ::core::ffi::c_int;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
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
    fn sqlite3Malloc(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3MallocZero(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3MutexAlloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3JournalIsInMemory(p: *mut sqlite3_file) -> ::core::ffi::c_int;
}
pub type u64_0 = sqlite_uint64;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type i64_0 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type u8_0 = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
pub type u32_0 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
pub type sqlite3_int64 = sqlite_int64;
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
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int
    >

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void

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
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_COMMIT_PHASETWO: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_LOCK_TIMEOUT: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_CKPT_DONE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_CKPT_START: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_STATIC_MAIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_SECTOR_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const SQLITE_IOERR_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_IOERR_NOMEM;
#[no_mangle]
pub static mut sqlite3_io_error_hit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_io_error_hardhit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_io_error_pending: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_io_error_persist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_io_error_benign: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_diskfull_pending: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_diskfull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_open_file_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_memdebug_vfs_oom_test: ::core::ffi::c_int = 1
    as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsClose(mut pId: *mut sqlite3_file) {
    unsafe {
        if !(*pId).pMethods.is_null() {
            (*(*pId).pMethods).xClose.expect("non-null function pointer")(pId);
            (*pId).pMethods = ::core::ptr::null::<sqlite3_io_methods>();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsRead(
    mut id: *mut sqlite3_file,
    mut pBuf: *mut ::core::ffi::c_void,
    mut amt: ::core::ffi::c_int,
    mut offset: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_memdebug_vfs_oom_test != 0
            && (id.is_null() || sqlite3JournalIsInMemory(id) == 0)
        {
            let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
            if pTstAlloc.is_null() {
                return SQLITE_IOERR_NOMEM_BKPT;
            }
            sqlite3_free(pTstAlloc);
        }
        return (*(*id).pMethods)
            .xRead
            .expect("non-null function pointer")(id, pBuf, amt, offset as sqlite3_int64);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsWrite(
    mut id: *mut sqlite3_file,
    mut pBuf: *const ::core::ffi::c_void,
    mut amt: ::core::ffi::c_int,
    mut offset: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_memdebug_vfs_oom_test != 0
            && (id.is_null() || sqlite3JournalIsInMemory(id) == 0)
        {
            let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
            if pTstAlloc.is_null() {
                return SQLITE_IOERR_NOMEM_BKPT;
            }
            sqlite3_free(pTstAlloc);
        }
        return (*(*id).pMethods)
            .xWrite
            .expect("non-null function pointer")(id, pBuf, amt, offset as sqlite3_int64);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsTruncate(
    mut id: *mut sqlite3_file,
    mut size: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*id).pMethods)
            .xTruncate
            .expect("non-null function pointer")(id, size as sqlite3_int64);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsSync(
    mut id: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_memdebug_vfs_oom_test != 0
            && (id.is_null() || sqlite3JournalIsInMemory(id) == 0)
        {
            let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
            if pTstAlloc.is_null() {
                return SQLITE_IOERR_NOMEM_BKPT;
            }
            sqlite3_free(pTstAlloc);
        }
        return if flags != 0 {
            (*(*id).pMethods).xSync.expect("non-null function pointer")(id, flags)
        } else {
            SQLITE_OK
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsFileSize(
    mut id: *mut sqlite3_file,
    mut pSize: *mut i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_memdebug_vfs_oom_test != 0
            && (id.is_null() || sqlite3JournalIsInMemory(id) == 0)
        {
            let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
            if pTstAlloc.is_null() {
                return SQLITE_IOERR_NOMEM_BKPT;
            }
            sqlite3_free(pTstAlloc);
        }
        return (*(*id).pMethods)
            .xFileSize
            .expect("non-null function pointer")(id, pSize as *mut sqlite3_int64);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsLock(
    mut id: *mut sqlite3_file,
    mut lockType: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_memdebug_vfs_oom_test != 0
            && (id.is_null() || sqlite3JournalIsInMemory(id) == 0)
        {
            let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
            if pTstAlloc.is_null() {
                return SQLITE_IOERR_NOMEM_BKPT;
            }
            sqlite3_free(pTstAlloc);
        }
        return (*(*id).pMethods).xLock.expect("non-null function pointer")(id, lockType);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsUnlock(
    mut id: *mut sqlite3_file,
    mut lockType: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*id).pMethods)
            .xUnlock
            .expect("non-null function pointer")(id, lockType);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsCheckReservedLock(
    mut id: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_memdebug_vfs_oom_test != 0
            && (id.is_null() || sqlite3JournalIsInMemory(id) == 0)
        {
            let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
            if pTstAlloc.is_null() {
                return SQLITE_IOERR_NOMEM_BKPT;
            }
            sqlite3_free(pTstAlloc);
        }
        return (*(*id).pMethods)
            .xCheckReservedLock
            .expect("non-null function pointer")(id, pResOut);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsFileControl(
    mut id: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        if (*id).pMethods.is_null() {
            return SQLITE_NOTFOUND;
        }
        if op != SQLITE_FCNTL_COMMIT_PHASETWO && op != SQLITE_FCNTL_LOCK_TIMEOUT
            && op != SQLITE_FCNTL_CKPT_DONE && op != SQLITE_FCNTL_CKPT_START
        {
            if sqlite3_memdebug_vfs_oom_test != 0
                && (id.is_null() || sqlite3JournalIsInMemory(id) == 0)
            {
                let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
                if pTstAlloc.is_null() {
                    return SQLITE_IOERR_NOMEM_BKPT;
                }
                sqlite3_free(pTstAlloc);
            }
        }
        return (*(*id).pMethods)
            .xFileControl
            .expect("non-null function pointer")(id, op, pArg);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsFileControlHint(
    mut id: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) {
    unsafe {
        if !(*id).pMethods.is_null() {
            (*(*id).pMethods)
                .xFileControl
                .expect("non-null function pointer")(id, op, pArg);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsSectorSize(
    mut id: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut xSectorSize: Option<
            unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int
        > = (*(*id).pMethods).xSectorSize;
        return if xSectorSize.is_some() {
            xSectorSize.expect("non-null function pointer")(id)
        } else {
            SQLITE_DEFAULT_SECTOR_SIZE
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsDeviceCharacteristics(
    mut id: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        if (*id).pMethods.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        return (*(*id).pMethods)
            .xDeviceCharacteristics
            .expect("non-null function pointer")(id);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsShmLock(
    mut id: *mut sqlite3_file,
    mut offset: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*id).pMethods)
            .xShmLock
            .expect("non-null function pointer")(id, offset, n, flags);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsShmBarrier(mut id: *mut sqlite3_file) {
    unsafe {
        (*(*id).pMethods).xShmBarrier.expect("non-null function pointer")(id);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsShmUnmap(
    mut id: *mut sqlite3_file,
    mut deleteFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*id).pMethods)
            .xShmUnmap
            .expect("non-null function pointer")(id, deleteFlag);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsShmMap(
    mut id: *mut sqlite3_file,
    mut iPage: ::core::ffi::c_int,
    mut pgsz: ::core::ffi::c_int,
    mut bExtend: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_memdebug_vfs_oom_test != 0
            && (id.is_null() || sqlite3JournalIsInMemory(id) == 0)
        {
            let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
            if pTstAlloc.is_null() {
                return SQLITE_IOERR_NOMEM_BKPT;
            }
            sqlite3_free(pTstAlloc);
        }
        return (*(*id).pMethods)
            .xShmMap
            .expect("non-null function pointer")(id, iPage, pgsz, bExtend, pp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsFetch(
    mut id: *mut sqlite3_file,
    mut iOff: i64_0,
    mut iAmt: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_memdebug_vfs_oom_test != 0
            && (id.is_null() || sqlite3JournalIsInMemory(id) == 0)
        {
            let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
            if pTstAlloc.is_null() {
                return SQLITE_IOERR_NOMEM_BKPT;
            }
            sqlite3_free(pTstAlloc);
        }
        return (*(*id).pMethods)
            .xFetch
            .expect("non-null function pointer")(id, iOff as sqlite3_int64, iAmt, pp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsUnfetch(
    mut id: *mut sqlite3_file,
    mut iOff: i64_0,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*id).pMethods)
            .xUnfetch
            .expect("non-null function pointer")(id, iOff as sqlite3_int64, p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pFlagsOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if sqlite3_memdebug_vfs_oom_test != 0
            && (true
                || sqlite3JournalIsInMemory(::core::ptr::null_mut::<sqlite3_file>())
                    == 0)
        {
            let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
            if pTstAlloc.is_null() {
                return SQLITE_IOERR_NOMEM_BKPT;
            }
            sqlite3_free(pTstAlloc);
        }
        rc = (*pVfs)
            .xOpen
            .expect(
                "non-null function pointer",
            )(
            pVfs,
            zPath as sqlite3_filename,
            pFile,
            flags & 0x1087f7f as ::core::ffi::c_int,
            pFlagsOut,
        );
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsDelete(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut dirSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_memdebug_vfs_oom_test != 0
            && (true
                || sqlite3JournalIsInMemory(::core::ptr::null_mut::<sqlite3_file>())
                    == 0)
        {
            let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
            if pTstAlloc.is_null() {
                return SQLITE_IOERR_NOMEM_BKPT;
            }
            sqlite3_free(pTstAlloc);
        }
        return if (*pVfs).xDelete.is_some() {
            (*pVfs).xDelete.expect("non-null function pointer")(pVfs, zPath, dirSync)
        } else {
            SQLITE_OK
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsAccess(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_memdebug_vfs_oom_test != 0
            && (true
                || sqlite3JournalIsInMemory(::core::ptr::null_mut::<sqlite3_file>())
                    == 0)
        {
            let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
            if pTstAlloc.is_null() {
                return SQLITE_IOERR_NOMEM_BKPT;
            }
            sqlite3_free(pTstAlloc);
        }
        return (*pVfs)
            .xAccess
            .expect("non-null function pointer")(pVfs, zPath, flags, pResOut);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsFullPathname(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut nPathOut: ::core::ffi::c_int,
    mut zPathOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_memdebug_vfs_oom_test != 0
            && (true
                || sqlite3JournalIsInMemory(::core::ptr::null_mut::<sqlite3_file>())
                    == 0)
        {
            let mut pTstAlloc: *mut ::core::ffi::c_void = sqlite3Malloc(10 as u64_0);
            if pTstAlloc.is_null() {
                return SQLITE_IOERR_NOMEM_BKPT;
            }
            sqlite3_free(pTstAlloc);
        }
        *zPathOut.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
        return (*pVfs)
            .xFullPathname
            .expect("non-null function pointer")(pVfs, zPath, nPathOut, zPathOut);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsDlOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*pVfs).xDlOpen.expect("non-null function pointer")(pVfs, zPath);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsDlError(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) {
    unsafe {
        (*pVfs).xDlError.expect("non-null function pointer")(pVfs, nByte, zBufOut);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsDlSym(
    mut pVfs: *mut sqlite3_vfs,
    mut pHdle: *mut ::core::ffi::c_void,
    mut zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    unsafe {
        return (*pVfs).xDlSym.expect("non-null function pointer")(pVfs, pHdle, zSym);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsDlClose(
    mut pVfs: *mut sqlite3_vfs,
    mut pHandle: *mut ::core::ffi::c_void,
) {
    unsafe {
        (*pVfs).xDlClose.expect("non-null function pointer")(pVfs, pHandle);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsRandomness(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3Config.iPrngSeed != 0 {
            memset(
                zBufOut as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                nByte as size_t,
            );
            if nByte
                > ::core::mem::size_of::<::core::ffi::c_uint>() as ::core::ffi::c_int
            {
                nByte = ::core::mem::size_of::<::core::ffi::c_uint>()
                    as ::core::ffi::c_int;
            }
            memcpy(
                zBufOut as *mut ::core::ffi::c_void,
                &raw mut sqlite3Config.iPrngSeed as *const ::core::ffi::c_void,
                nByte as size_t,
            );
            return SQLITE_OK;
        } else {
            return (*pVfs)
                .xRandomness
                .expect("non-null function pointer")(pVfs, nByte, zBufOut)
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsSleep(
    mut pVfs: *mut sqlite3_vfs,
    mut nMicro: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (*pVfs).xSleep.expect("non-null function pointer")(pVfs, nMicro);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsGetLastError(
    mut pVfs: *mut sqlite3_vfs,
) -> ::core::ffi::c_int {
    unsafe {
        return if (*pVfs).xGetLastError.is_some() {
            (*pVfs)
                .xGetLastError
                .expect(
                    "non-null function pointer",
                )(
                pVfs,
                0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            )
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsCurrentTimeInt64(
    mut pVfs: *mut sqlite3_vfs,
    mut pTimeOut: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if (*pVfs).iVersion >= 2 as ::core::ffi::c_int
            && (*pVfs).xCurrentTimeInt64.is_some()
        {
            rc = (*pVfs)
                .xCurrentTimeInt64
                .expect("non-null function pointer")(pVfs, pTimeOut);
        } else {
            let mut r: ::core::ffi::c_double = 0.;
            rc = (*pVfs)
                .xCurrentTime
                .expect("non-null function pointer")(pVfs, &raw mut r);
            *pTimeOut = (r * 86400000.0f64) as sqlite3_int64;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsOpenMalloc(
    mut pVfs: *mut sqlite3_vfs,
    mut zFile: *const ::core::ffi::c_char,
    mut ppFile: *mut *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pFile: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        pFile = sqlite3MallocZero((*pVfs).szOsFile as u64_0) as *mut sqlite3_file;
        if !pFile.is_null() {
            rc = sqlite3OsOpen(pVfs, zFile, pFile, flags, pOutFlags);
            if rc != SQLITE_OK {
                sqlite3_free(pFile as *mut ::core::ffi::c_void);
                *ppFile = ::core::ptr::null_mut::<sqlite3_file>();
            } else {
                *ppFile = pFile;
            }
        } else {
            *ppFile = ::core::ptr::null_mut::<sqlite3_file>();
            rc = SQLITE_NOMEM_BKPT;
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsCloseFree(mut pFile: *mut sqlite3_file) {
    unsafe {
        sqlite3OsClose(pFile);
        sqlite3_free(pFile as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OsInit() -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut ::core::ffi::c_void = sqlite3_malloc(10 as ::core::ffi::c_int);
        if p.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        sqlite3_free(p);
        return sqlite3_os_init();
    }
}
static mut vfsList: *mut sqlite3_vfs = ::core::ptr::null::<sqlite3_vfs>()
    as *mut sqlite3_vfs;
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vfs_find(
    mut zVfs: *const ::core::ffi::c_char,
) -> *mut sqlite3_vfs {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut mutex: *mut sqlite3_mutex = ::core::ptr::null_mut::<sqlite3_mutex>();
        let mut rc: ::core::ffi::c_int = sqlite3_initialize();
        if rc != 0 {
            return ::core::ptr::null_mut::<sqlite3_vfs>();
        }
        mutex = sqlite3MutexAlloc(SQLITE_MUTEX_STATIC_MAIN);
        sqlite3_mutex_enter(mutex);
        pVfs = vfsList;
        while !pVfs.is_null() {
            if zVfs.is_null() {
                break;
            }
            if strcmp(zVfs, (*pVfs).zName) == 0 as ::core::ffi::c_int {
                break;
            }
            pVfs = (*pVfs).pNext;
        }
        sqlite3_mutex_leave(mutex);
        return pVfs;
    }
}
unsafe extern "C" fn vfsUnlink(mut pVfs: *mut sqlite3_vfs) {
    unsafe {
        if !pVfs.is_null() {
            if vfsList == pVfs {
                vfsList = (*pVfs).pNext;
            } else if !vfsList.is_null() {
                let mut p: *mut sqlite3_vfs = vfsList;
                while !(*p).pNext.is_null() && (*p).pNext != pVfs {
                    p = (*p).pNext;
                }
                if (*p).pNext == pVfs {
                    (*p).pNext = (*pVfs).pNext;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vfs_register(
    mut pVfs: *mut sqlite3_vfs,
    mut makeDflt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut mutex: *mut sqlite3_mutex = ::core::ptr::null_mut::<sqlite3_mutex>();
        let mut rc: ::core::ffi::c_int = sqlite3_initialize();
        if rc != 0 {
            return rc;
        }
        mutex = sqlite3MutexAlloc(2 as ::core::ffi::c_int);
        sqlite3_mutex_enter(mutex);
        vfsUnlink(pVfs);
        if makeDflt != 0 || vfsList.is_null() {
            (*pVfs).pNext = vfsList;
            vfsList = pVfs;
        } else {
            (*pVfs).pNext = (*vfsList).pNext;
            (*vfsList).pNext = pVfs;
        }
        sqlite3_mutex_leave(mutex);
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_vfs_unregister(
    mut pVfs: *mut sqlite3_vfs,
) -> ::core::ffi::c_int {
    unsafe {
        let mut mutex: *mut sqlite3_mutex = ::core::ptr::null_mut::<sqlite3_mutex>();
        let mut rc: ::core::ffi::c_int = sqlite3_initialize();
        if rc != 0 {
            return rc;
        }
        mutex = sqlite3MutexAlloc(2 as ::core::ffi::c_int);
        sqlite3_mutex_enter(mutex);
        vfsUnlink(pVfs);
        sqlite3_mutex_leave(mutex);
        return SQLITE_OK;
    }
}
