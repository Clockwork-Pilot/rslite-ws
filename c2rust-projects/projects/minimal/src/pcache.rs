extern "C" {
    pub type sqlite3_mutex;
    pub type sqlite3_pcache;
    pub type Pager;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3PCacheSetDefault();
    static mut sqlite3Config: Sqlite3Config;
}
pub type i64_0 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type u16_0 = ::core::ffi::c_ushort;
pub type u8_0 = ::core::ffi::c_uchar;
pub type u32_0 = ::core::ffi::c_uint;
pub type Pgno = u32_0;
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
pub struct PCache {
    pub pDirty: *mut PgHdr,
    pub pDirtyTail: *mut PgHdr,
    pub pSynced: *mut PgHdr,
    pub nRefSum: i64_0,
    pub szCache: ::core::ffi::c_int,
    pub szSpill: ::core::ffi::c_int,
    pub szPage: ::core::ffi::c_int,
    pub szExtra: ::core::ffi::c_int,
    pub bPurgeable: u8_0,
    pub eCreate: u8_0,
    pub xStress: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut PgHdr) -> ::core::ffi::c_int,
    >,
    pub pStress: *mut ::core::ffi::c_void,
    pub pCache: *mut sqlite3_pcache

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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const PGHDR_CLEAN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PGHDR_DIRTY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PGHDR_WRITEABLE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const PGHDR_NEED_SYNC: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const PGHDR_DONT_WRITE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const PCACHE_DIRTYLIST_REMOVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PCACHE_DIRTYLIST_ADD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PCACHE_DIRTYLIST_FRONT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
unsafe extern "C" fn pcacheManageDirtyList(mut pPage: *mut PgHdr, mut addRemove: u8_0) {
    unsafe {
        let mut p: *mut PCache = (*pPage).pCache;
        if addRemove as ::core::ffi::c_int & PCACHE_DIRTYLIST_REMOVE != 0 {
            if (*p).pSynced == pPage {
                (*p).pSynced = (*pPage).pDirtyPrev;
            }
            if !(*pPage).pDirtyNext.is_null() {
                (*(*pPage).pDirtyNext).pDirtyPrev = (*pPage).pDirtyPrev;
            } else {
                (*p).pDirtyTail = (*pPage).pDirtyPrev;
            }
            if !(*pPage).pDirtyPrev.is_null() {
                (*(*pPage).pDirtyPrev).pDirtyNext = (*pPage).pDirtyNext;
            } else {
                (*p).pDirty = (*pPage).pDirtyNext;
                if (*p).pDirty.is_null() {
                    (*p).eCreate = 2 as u8_0;
                }
            }
        }
        if addRemove as ::core::ffi::c_int & PCACHE_DIRTYLIST_ADD != 0 {
            (*pPage).pDirtyPrev = ::core::ptr::null_mut::<PgHdr>();
            (*pPage).pDirtyNext = (*p).pDirty;
            if !(*pPage).pDirtyNext.is_null() {
                (*(*pPage).pDirtyNext).pDirtyPrev = pPage;
            } else {
                (*p).pDirtyTail = pPage;
                if (*p).bPurgeable != 0 {
                    (*p).eCreate = 1 as u8_0;
                }
            }
            (*p).pDirty = pPage;
            if (*p).pSynced.is_null()
                && 0 as ::core::ffi::c_int
                    == (*pPage).flags as ::core::ffi::c_int & PGHDR_NEED_SYNC
            {
                (*p).pSynced = pPage;
            }
        }
    }
}
unsafe extern "C" fn pcacheUnpin(mut p: *mut PgHdr) {
    unsafe {
        if (*(*p).pCache).bPurgeable != 0 {
            sqlite3Config
                .pcache2
                .xUnpin
                .expect(
                    "non-null function pointer",
                )((*(*p).pCache).pCache, (*p).pPage, 0 as ::core::ffi::c_int);
        }
    }
}
unsafe extern "C" fn numberOfCachePages(mut p: *mut PCache) -> ::core::ffi::c_int {
    unsafe {
        if (*p).szCache >= 0 as ::core::ffi::c_int {
            return (*p).szCache
        } else {
            let mut n: i64_0 = 0;
            n = (-1024 as ::core::ffi::c_int as ::core::ffi::c_longlong
                * (*p).szCache as ::core::ffi::c_longlong
                / ((*p).szPage + (*p).szExtra) as ::core::ffi::c_longlong) as i64_0;
            if n > 1000000000 as ::core::ffi::c_longlong {
                n = 1000000000 as i64_0;
            }
            return n as ::core::ffi::c_int;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheInitialize() -> ::core::ffi::c_int {
    unsafe {
        if sqlite3Config.pcache2.xInit.is_none() {
            sqlite3PCacheSetDefault();
        }
        return sqlite3Config
            .pcache2
            .xInit
            .expect("non-null function pointer")(sqlite3Config.pcache2.pArg);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheShutdown() {
    unsafe {
        if sqlite3Config.pcache2.xShutdown.is_some() {
            sqlite3Config
                .pcache2
                .xShutdown
                .expect("non-null function pointer")(sqlite3Config.pcache2.pArg);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheSize() -> ::core::ffi::c_int {
    unsafe {
        return ::core::mem::size_of::<PCache>() as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheOpen(
    mut szPage: ::core::ffi::c_int,
    mut szExtra: ::core::ffi::c_int,
    mut bPurgeable: ::core::ffi::c_int,
    mut xStress: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut PgHdr) -> ::core::ffi::c_int,
    >,
    mut pStress: *mut ::core::ffi::c_void,
    mut p: *mut PCache,
) -> ::core::ffi::c_int {
    unsafe {
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<PCache>() as size_t,
        );
        (*p).szPage = 1 as ::core::ffi::c_int;
        (*p).szExtra = szExtra;
        (*p).bPurgeable = bPurgeable as u8_0;
        (*p).eCreate = 2 as u8_0;
        (*p).xStress = xStress;
        (*p).pStress = pStress;
        (*p).szCache = 100 as ::core::ffi::c_int;
        (*p).szSpill = 1 as ::core::ffi::c_int;
        return sqlite3PcacheSetPageSize(p, szPage);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheSetPageSize(
    mut pCache: *mut PCache,
    mut szPage: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pCache).szPage != 0 {
            let mut pNew: *mut sqlite3_pcache = ::core::ptr::null_mut::<
                sqlite3_pcache,
            >();
            pNew = sqlite3Config
                .pcache2
                .xCreate
                .expect(
                    "non-null function pointer",
                )(
                szPage,
                ((*pCache).szExtra as usize)
                    .wrapping_add(
                        (::core::mem::size_of::<PgHdr>() as usize)
                            .wrapping_add(7 as usize)
                            & !(7 as ::core::ffi::c_int) as usize,
                    ) as ::core::ffi::c_int,
                (*pCache).bPurgeable as ::core::ffi::c_int,
            );
            if pNew.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            sqlite3Config
                .pcache2
                .xCachesize
                .expect("non-null function pointer")(pNew, numberOfCachePages(pCache));
            if !(*pCache).pCache.is_null() {
                sqlite3Config
                    .pcache2
                    .xDestroy
                    .expect("non-null function pointer")((*pCache).pCache);
            }
            (*pCache).pCache = pNew;
            (*pCache).szPage = szPage;
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheFetch(
    mut pCache: *mut PCache,
    mut pgno: Pgno,
    mut createFlag: ::core::ffi::c_int,
) -> *mut sqlite3_pcache_page {
    unsafe {
        let mut eCreate: ::core::ffi::c_int = 0;
        let mut pRes: *mut sqlite3_pcache_page = ::core::ptr::null_mut::<
            sqlite3_pcache_page,
        >();
        eCreate = createFlag & (*pCache).eCreate as ::core::ffi::c_int;
        pRes = sqlite3Config
            .pcache2
            .xFetch
            .expect(
                "non-null function pointer",
            )((*pCache).pCache, pgno as ::core::ffi::c_uint, eCreate);
        return pRes;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheFetchStress(
    mut pCache: *mut PCache,
    mut pgno: Pgno,
    mut ppPage: *mut *mut sqlite3_pcache_page,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPg: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        if (*pCache).eCreate as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if sqlite3PcachePagecount(pCache) > (*pCache).szSpill {
            pPg = (*pCache).pSynced;
            while !pPg.is_null()
                && ((*pPg).nRef != 0
                    || (*pPg).flags as ::core::ffi::c_int & PGHDR_NEED_SYNC != 0)
            {
                pPg = (*pPg).pDirtyPrev;
            }
            (*pCache).pSynced = pPg;
            if pPg.is_null() {
                pPg = (*pCache).pDirtyTail;
                while !pPg.is_null() && (*pPg).nRef != 0 {
                    pPg = (*pPg).pDirtyPrev;
                }
            }
            if !pPg.is_null() {
                let mut rc: ::core::ffi::c_int = 0;
                rc = (*pCache)
                    .xStress
                    .expect("non-null function pointer")((*pCache).pStress, pPg);
                if rc != SQLITE_OK && rc != SQLITE_BUSY {
                    return rc;
                }
            }
        }
        *ppPage = sqlite3Config
            .pcache2
            .xFetch
            .expect(
                "non-null function pointer",
            )((*pCache).pCache, pgno as ::core::ffi::c_uint, 2 as ::core::ffi::c_int);
        return if (*ppPage).is_null() { SQLITE_NOMEM_BKPT } else { SQLITE_OK };
    }
}
#[inline(never)]
unsafe extern "C" fn pcacheFetchFinishWithInit(
    mut pCache: *mut PCache,
    mut pgno: Pgno,
    mut pPage: *mut sqlite3_pcache_page,
) -> *mut PgHdr {
    unsafe {
        let mut pPgHdr: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        pPgHdr = (*pPage).pExtra as *mut PgHdr;
        memset(
            &raw mut (*pPgHdr).pDirty as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<PgHdr>() as size_t).wrapping_sub(32 as size_t),
        );
        (*pPgHdr).pPage = pPage;
        (*pPgHdr).pData = (*pPage).pBuf;
        (*pPgHdr).pExtra = pPgHdr.offset(1 as ::core::ffi::c_int as isize) as *mut PgHdr
            as *mut ::core::ffi::c_void;
        memset((*pPgHdr).pExtra, 0 as ::core::ffi::c_int, 8 as size_t);
        (*pPgHdr).pCache = pCache;
        (*pPgHdr).pgno = pgno;
        (*pPgHdr).flags = PGHDR_CLEAN as u16_0;
        return sqlite3PcacheFetchFinish(pCache, pgno, pPage);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheFetchFinish(
    mut pCache: *mut PCache,
    mut pgno: Pgno,
    mut pPage: *mut sqlite3_pcache_page,
) -> *mut PgHdr {
    unsafe {
        let mut pPgHdr: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        pPgHdr = (*pPage).pExtra as *mut PgHdr;
        if (*pPgHdr).pPage.is_null() {
            return pcacheFetchFinishWithInit(pCache, pgno, pPage);
        }
        (*pCache).nRefSum += 1;
        (*pPgHdr).nRef += 1;
        return pPgHdr;
    }
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sqlite3PcacheRelease(mut p: *mut PgHdr) {
    unsafe {
        (*(*p).pCache).nRefSum -= 1;
        (*p).nRef -= 1;
        if (*p).nRef == 0 as ::core::ffi::c_longlong {
            if (*p).flags as ::core::ffi::c_int & PGHDR_CLEAN != 0 {
                pcacheUnpin(p);
            } else {
                pcacheManageDirtyList(p, PCACHE_DIRTYLIST_FRONT as u8_0);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheRef(mut p: *mut PgHdr) {
    unsafe {
        (*p).nRef += 1;
        (*(*p).pCache).nRefSum += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheDrop(mut p: *mut PgHdr) {
    unsafe {
        if (*p).flags as ::core::ffi::c_int & PGHDR_DIRTY != 0 {
            pcacheManageDirtyList(p, PCACHE_DIRTYLIST_REMOVE as u8_0);
        }
        (*(*p).pCache).nRefSum -= 1;
        sqlite3Config
            .pcache2
            .xUnpin
            .expect(
                "non-null function pointer",
            )((*(*p).pCache).pCache, (*p).pPage, 1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheMakeDirty(mut p: *mut PgHdr) {
    unsafe {
        if (*p).flags as ::core::ffi::c_int & (PGHDR_CLEAN | PGHDR_DONT_WRITE) != 0 {
            (*p).flags = ((*p).flags as ::core::ffi::c_int & !PGHDR_DONT_WRITE) as u16_0;
            if (*p).flags as ::core::ffi::c_int & PGHDR_CLEAN != 0 {
                (*p).flags = ((*p).flags as ::core::ffi::c_int
                    ^ (PGHDR_DIRTY | PGHDR_CLEAN)) as u16_0;
                pcacheManageDirtyList(p, PCACHE_DIRTYLIST_ADD as u8_0);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheMakeClean(mut p: *mut PgHdr) {
    unsafe {
        pcacheManageDirtyList(p, PCACHE_DIRTYLIST_REMOVE as u8_0);
        (*p).flags = ((*p).flags as ::core::ffi::c_int
            & !(PGHDR_DIRTY | PGHDR_NEED_SYNC | PGHDR_WRITEABLE)) as u16_0;
        (*p).flags = ((*p).flags as ::core::ffi::c_int | PGHDR_CLEAN) as u16_0;
        if (*p).nRef == 0 as ::core::ffi::c_longlong {
            pcacheUnpin(p);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheCleanAll(mut pCache: *mut PCache) {
    unsafe {
        let mut p: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        loop {
            p = (*pCache).pDirty;
            if p.is_null() {
                break;
            }
            sqlite3PcacheMakeClean(p);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheClearWritable(mut pCache: *mut PCache) {
    unsafe {
        let mut p: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        p = (*pCache).pDirty;
        while !p.is_null() {
            (*p).flags = ((*p).flags as ::core::ffi::c_int
                & !(PGHDR_NEED_SYNC | PGHDR_WRITEABLE)) as u16_0;
            p = (*p).pDirtyNext;
        }
        (*pCache).pSynced = (*pCache).pDirtyTail;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheClearSyncFlags(mut pCache: *mut PCache) {
    unsafe {
        let mut p: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        p = (*pCache).pDirty;
        while !p.is_null() {
            (*p).flags = ((*p).flags as ::core::ffi::c_int & !PGHDR_NEED_SYNC) as u16_0;
            p = (*p).pDirtyNext;
        }
        (*pCache).pSynced = (*pCache).pDirtyTail;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheMove(mut p: *mut PgHdr, mut newPgno: Pgno) {
    unsafe {
        let mut pCache: *mut PCache = (*p).pCache;
        let mut pOther: *mut sqlite3_pcache_page = ::core::ptr::null_mut::<
            sqlite3_pcache_page,
        >();
        pOther = sqlite3Config
            .pcache2
            .xFetch
            .expect(
                "non-null function pointer",
            )((*pCache).pCache, newPgno as ::core::ffi::c_uint, 0 as ::core::ffi::c_int);
        if !pOther.is_null() {
            let mut pXPage: *mut PgHdr = (*pOther).pExtra as *mut PgHdr;
            (*pXPage).nRef += 1;
            (*pCache).nRefSum += 1;
            sqlite3PcacheDrop(pXPage);
        }
        sqlite3Config
            .pcache2
            .xRekey
            .expect(
                "non-null function pointer",
            )(
            (*pCache).pCache,
            (*p).pPage,
            (*p).pgno as ::core::ffi::c_uint,
            newPgno as ::core::ffi::c_uint,
        );
        (*p).pgno = newPgno;
        if (*p).flags as ::core::ffi::c_int & PGHDR_DIRTY != 0
            && (*p).flags as ::core::ffi::c_int & PGHDR_NEED_SYNC != 0
        {
            pcacheManageDirtyList(p, PCACHE_DIRTYLIST_FRONT as u8_0);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheTruncate(mut pCache: *mut PCache, mut pgno: Pgno) {
    unsafe {
        if !(*pCache).pCache.is_null() {
            let mut p: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
            let mut pNext: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
            p = (*pCache).pDirty;
            while !p.is_null() {
                pNext = (*p).pDirtyNext;
                if (*p).pgno > pgno {
                    sqlite3PcacheMakeClean(p);
                }
                p = pNext;
            }
            if pgno == 0 as ::core::ffi::c_uint && (*pCache).nRefSum != 0 {
                let mut pPage1: *mut sqlite3_pcache_page = ::core::ptr::null_mut::<
                    sqlite3_pcache_page,
                >();
                pPage1 = sqlite3Config
                    .pcache2
                    .xFetch
                    .expect(
                        "non-null function pointer",
                    )(
                    (*pCache).pCache,
                    1 as ::core::ffi::c_uint,
                    0 as ::core::ffi::c_int,
                );
                if !pPage1.is_null() {
                    memset(
                        (*pPage1).pBuf,
                        0 as ::core::ffi::c_int,
                        (*pCache).szPage as size_t,
                    );
                    pgno = 1 as Pgno;
                }
            }
            sqlite3Config
                .pcache2
                .xTruncate
                .expect(
                    "non-null function pointer",
                )(
                (*pCache).pCache,
                (pgno as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheClose(mut pCache: *mut PCache) {
    unsafe {
        sqlite3Config
            .pcache2
            .xDestroy
            .expect("non-null function pointer")((*pCache).pCache);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheClear(mut pCache: *mut PCache) {
    unsafe {
        sqlite3PcacheTruncate(pCache, 0 as Pgno);
    }
}
unsafe extern "C" fn pcacheMergeDirtyList(
    mut pA: *mut PgHdr,
    mut pB: *mut PgHdr,
) -> *mut PgHdr {
    unsafe {
        let mut result: PgHdr = PgHdr {
            pPage: ::core::ptr::null_mut::<sqlite3_pcache_page>(),
            pData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pExtra: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pCache: ::core::ptr::null_mut::<PCache>(),
            pDirty: ::core::ptr::null_mut::<PgHdr>(),
            pPager: ::core::ptr::null_mut::<Pager>(),
            pgno: 0,
            flags: 0,
            nRef: 0,
            pDirtyNext: ::core::ptr::null_mut::<PgHdr>(),
            pDirtyPrev: ::core::ptr::null_mut::<PgHdr>()

        };
        let mut pTail: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        pTail = &raw mut result;
        loop {
            if (*pA).pgno < (*pB).pgno {
                (*pTail).pDirty = pA;
                pTail = pA;
                pA = (*pA).pDirty;
                if !pA.is_null() {
                    continue;
                }
                (*pTail).pDirty = pB;
                break;
            } else {
                (*pTail).pDirty = pB;
                pTail = pB;
                pB = (*pB).pDirty;
                if !pB.is_null() {
                    continue;
                }
                (*pTail).pDirty = pA;
                break;
            }
        }
        return result.pDirty;
    }
}
pub const N_SORT_BUCKET: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
unsafe extern "C" fn pcacheSortDirtyList(mut pIn: *mut PgHdr) -> *mut PgHdr {
    unsafe {
        let mut a: [*mut PgHdr; 32] = [::core::ptr::null_mut::<PgHdr>(); 32];
        let mut p: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        let mut i: ::core::ffi::c_int = 0;
        memset(
            &raw mut a as *mut *mut PgHdr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[*mut PgHdr; 32]>() as size_t,
        );
        while !pIn.is_null() {
            p = pIn;
            pIn = (*p).pDirty;
            (*p).pDirty = ::core::ptr::null_mut::<PgHdr>();
            i = 0 as ::core::ffi::c_int;
            while i < 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
                if a[i as usize].is_null() {
                    a[i as usize] = p;
                    break;
                } else {
                    p = pcacheMergeDirtyList(a[i as usize], p);
                    a[i as usize] = ::core::ptr::null_mut::<PgHdr>();
                    i += 1;
                }
            }
            if i == 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
                a[i as usize] = pcacheMergeDirtyList(a[i as usize], p);
            }
        }
        p = a[0 as ::core::ffi::c_int as usize];
        i = 1 as ::core::ffi::c_int;
        while i < N_SORT_BUCKET {
            if !a[i as usize].is_null() {
                p = if !p.is_null() {
                    pcacheMergeDirtyList(p, a[i as usize])
                } else {
                    a[i as usize]
                };
            }
            i += 1;
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheDirtyList(mut pCache: *mut PCache) -> *mut PgHdr {
    unsafe {
        let mut p: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        p = (*pCache).pDirty;
        while !p.is_null() {
            (*p).pDirty = (*p).pDirtyNext;
            p = (*p).pDirtyNext;
        }
        return pcacheSortDirtyList((*pCache).pDirty);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheRefCount(mut pCache: *mut PCache) -> i64_0 {
    unsafe {
        return (*pCache).nRefSum;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcachePageRefcount(mut p: *mut PgHdr) -> i64_0 {
    unsafe {
        return (*p).nRef;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcachePagecount(
    mut pCache: *mut PCache,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3Config
            .pcache2
            .xPagecount
            .expect("non-null function pointer")((*pCache).pCache);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheSetCachesize(
    mut pCache: *mut PCache,
    mut mxPage: ::core::ffi::c_int,
) {
    unsafe {
        (*pCache).szCache = mxPage;
        sqlite3Config
            .pcache2
            .xCachesize
            .expect(
                "non-null function pointer",
            )((*pCache).pCache, numberOfCachePages(pCache));
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheSetSpillsize(
    mut p: *mut PCache,
    mut mxPage: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut res: ::core::ffi::c_int = 0;
        if mxPage != 0 {
            if mxPage < 0 as ::core::ffi::c_int {
                mxPage = (-1024 as ::core::ffi::c_int as ::core::ffi::c_longlong
                    * mxPage as ::core::ffi::c_longlong
                    / ((*p).szPage + (*p).szExtra) as ::core::ffi::c_longlong)
                    as ::core::ffi::c_int;
            }
            (*p).szSpill = mxPage;
        }
        res = numberOfCachePages(p);
        if res < (*p).szSpill {
            res = (*p).szSpill;
        }
        return res;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PcacheShrink(mut pCache: *mut PCache) {
    unsafe {
        sqlite3Config
            .pcache2
            .xShrink
            .expect("non-null function pointer")((*pCache).pCache);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3HeaderSizePcache() -> ::core::ffi::c_int {
    unsafe {
        return ((::core::mem::size_of::<PgHdr>() as usize).wrapping_add(7 as usize)
            & !(7 as ::core::ffi::c_int) as usize) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PCachePercentDirty(
    mut pCache: *mut PCache,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pDirty: *mut PgHdr = ::core::ptr::null_mut::<PgHdr>();
        let mut nDirty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nCache: ::core::ffi::c_int = numberOfCachePages(pCache);
        pDirty = (*pCache).pDirty;
        while !pDirty.is_null() {
            nDirty += 1;
            pDirty = (*pDirty).pDirtyNext;
        }
        return if nCache != 0 {
            (nDirty as ::core::ffi::c_longlong * 100 as ::core::ffi::c_longlong
                / nCache as ::core::ffi::c_longlong) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PCacheIsDirty(
    mut pCache: *mut PCache,
) -> ::core::ffi::c_int {
    unsafe {
        return !(*pCache).pDirty.is_null() as ::core::ffi::c_int;
    }
}
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
