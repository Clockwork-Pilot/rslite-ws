extern "C" {
    pub type sqlite3_mutex;
    pub type sqlite3_pcache;
    fn sqlite3DefaultMutex() -> *const sqlite3_mutex_methods;
    fn sqlite3NoopMutex() -> *const sqlite3_mutex_methods;
    fn sqlite3MemoryBarrier();
    static mut sqlite3Config: Sqlite3Config;
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type u8_0 = ::core::ffi::c_uchar;
pub type u32_0 = ::core::ffi::c_uint;
pub type sqlite3_int64 = sqlite_int64;
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
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
    >

}
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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3MutexInit() -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if sqlite3Config.mutex.xMutexAlloc.is_none() {
            let mut pFrom: *const sqlite3_mutex_methods = ::core::ptr::null::<
                sqlite3_mutex_methods,
            >();
            let mut pTo: *mut sqlite3_mutex_methods = &raw mut sqlite3Config.mutex;
            if sqlite3Config.bCoreMutex != 0 {
                pFrom = sqlite3DefaultMutex();
            } else {
                pFrom = sqlite3NoopMutex();
            }
            (*pTo).xMutexInit = (*pFrom).xMutexInit;
            (*pTo).xMutexEnd = (*pFrom).xMutexEnd;
            (*pTo).xMutexFree = (*pFrom).xMutexFree;
            (*pTo).xMutexEnter = (*pFrom).xMutexEnter;
            (*pTo).xMutexTry = (*pFrom).xMutexTry;
            (*pTo).xMutexLeave = (*pFrom).xMutexLeave;
            (*pTo).xMutexHeld = (*pFrom).xMutexHeld;
            (*pTo).xMutexNotheld = (*pFrom).xMutexNotheld;
            sqlite3MemoryBarrier();
            (*pTo).xMutexAlloc = (*pFrom).xMutexAlloc;
        }
        rc = sqlite3Config.mutex.xMutexInit.expect("non-null function pointer")();
        sqlite3MemoryBarrier();
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MutexEnd() -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if sqlite3Config.mutex.xMutexEnd.is_some() {
            rc = sqlite3Config.mutex.xMutexEnd.expect("non-null function pointer")();
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_mutex_alloc(
    mut id: ::core::ffi::c_int,
) -> *mut sqlite3_mutex {
    unsafe {
        return sqlite3Config.mutex.xMutexAlloc.expect("non-null function pointer")(id);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MutexAlloc(
    mut id: ::core::ffi::c_int,
) -> *mut sqlite3_mutex {
    unsafe {
        if sqlite3Config.bCoreMutex == 0 {
            return ::core::ptr::null_mut::<sqlite3_mutex>();
        }
        return sqlite3Config.mutex.xMutexAlloc.expect("non-null function pointer")(id);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_mutex_free(mut p: *mut sqlite3_mutex) {
    unsafe {
        if !p.is_null() {
            sqlite3Config.mutex.xMutexFree.expect("non-null function pointer")(p);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_mutex_enter(mut p: *mut sqlite3_mutex) {
    unsafe {
        if !p.is_null() {
            sqlite3Config.mutex.xMutexEnter.expect("non-null function pointer")(p);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_mutex_try(
    mut p: *mut sqlite3_mutex,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !p.is_null() {
            return sqlite3Config.mutex.xMutexTry.expect("non-null function pointer")(p);
        }
        return rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_mutex_leave(mut p: *mut sqlite3_mutex) {
    unsafe {
        if !p.is_null() {
            sqlite3Config.mutex.xMutexLeave.expect("non-null function pointer")(p);
        }
    }
}
