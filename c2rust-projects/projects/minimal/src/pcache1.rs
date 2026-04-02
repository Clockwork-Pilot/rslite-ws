extern "C" {
    pub type sqlite3_mutex;
    pub type sqlite3_pcache;
    fn sqlite3_config(_: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3Malloc(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3MallocZero(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3MallocSize(_: *const ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn sqlite3HeapNearlyFull() -> ::core::ffi::c_int;
    fn sqlite3MutexAlloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    fn sqlite3StatusUp(_: ::core::ffi::c_int, _: ::core::ffi::c_int);
    fn sqlite3StatusDown(_: ::core::ffi::c_int, _: ::core::ffi::c_int);
    fn sqlite3StatusHighwater(_: ::core::ffi::c_int, _: ::core::ffi::c_int);
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
}
pub type u64_0 = sqlite_uint64;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type i64_0 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type u16_0 = ::core::ffi::c_ushort;
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
pub type uptr = u64_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PCacheGlobal {
    pub grp: PGroup,
    pub isInit: ::core::ffi::c_int,
    pub separateCache: ::core::ffi::c_int,
    pub nInitPage: ::core::ffi::c_int,
    pub szSlot: ::core::ffi::c_int,
    pub nSlot: ::core::ffi::c_int,
    pub nReserve: ::core::ffi::c_int,
    pub pStart: *mut ::core::ffi::c_void,
    pub pEnd: *mut ::core::ffi::c_void,
    pub mutex: *mut sqlite3_mutex,
    pub pFree: *mut PgFreeslot,
    pub nFreeSlot: ::core::ffi::c_int,
    pub bUnderPressure: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PgFreeslot {
    pub pNext: *mut PgFreeslot

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PGroup {
    pub mutex: *mut sqlite3_mutex,
    pub nMaxPage: ::core::ffi::c_uint,
    pub nMinPage: ::core::ffi::c_uint,
    pub mxPinned: ::core::ffi::c_uint,
    pub nPurgeable: ::core::ffi::c_uint,
    pub lru: PgHdr1

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PgHdr1 {
    pub page: sqlite3_pcache_page,
    pub iKey: ::core::ffi::c_uint,
    pub isBulkLocal: u16_0,
    pub isAnchor: u16_0,
    pub pNext: *mut PgHdr1,
    pub pCache: *mut PCache1,
    pub pLruNext: *mut PgHdr1,
    pub pLruPrev: *mut PgHdr1

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PCache1 {
    pub pGroup: *mut PGroup,
    pub pnPurgeable: *mut ::core::ffi::c_uint,
    pub szPage: ::core::ffi::c_int,
    pub szExtra: ::core::ffi::c_int,
    pub szAlloc: ::core::ffi::c_int,
    pub bPurgeable: ::core::ffi::c_int,
    pub nMin: ::core::ffi::c_uint,
    pub nMax: ::core::ffi::c_uint,
    pub n90pct: ::core::ffi::c_uint,
    pub iMaxKey: ::core::ffi::c_uint,
    pub nPurgeableDummy: ::core::ffi::c_uint,
    pub nRecyclable: ::core::ffi::c_uint,
    pub nPage: ::core::ffi::c_uint,
    pub nHash: ::core::ffi::c_uint,
    pub apHash: *mut *mut PgHdr1,
    pub pFree: *mut PgHdr1,
    pub pBulk: *mut ::core::ffi::c_void

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
pub const SQLITE_CONFIG_PCACHE2: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_STATIC_LRU: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_STATIC_PMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_STATUS_PAGECACHE_USED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_STATUS_PAGECACHE_OVERFLOW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_STATUS_PAGECACHE_SIZE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
static mut pcache1_g: PCacheGlobal = PCacheGlobal {
    grp: PGroup {
        mutex: ::core::ptr::null::<sqlite3_mutex>() as *mut sqlite3_mutex,
        nMaxPage: 0,
        nMinPage: 0,
        mxPinned: 0,
        nPurgeable: 0,
        lru: PgHdr1 {
            page: sqlite3_pcache_page {
                pBuf: ::core::ptr::null::<::core::ffi::c_void>()
                    as *mut ::core::ffi::c_void,
                pExtra: ::core::ptr::null::<::core::ffi::c_void>()
                    as *mut ::core::ffi::c_void

            },
            iKey: 0,
            isBulkLocal: 0,
            isAnchor: 0,
            pNext: ::core::ptr::null::<PgHdr1>() as *mut PgHdr1,
            pCache: ::core::ptr::null::<PCache1>() as *mut PCache1,
            pLruNext: ::core::ptr::null::<PgHdr1>() as *mut PgHdr1,
            pLruPrev: ::core::ptr::null::<PgHdr1>() as *mut PgHdr1

        }

    },
    isInit: 0,
    separateCache: 0,
    nInitPage: 0,
    szSlot: 0,
    nSlot: 0,
    nReserve: 0,
    pStart: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    pEnd: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    mutex: ::core::ptr::null::<sqlite3_mutex>() as *mut sqlite3_mutex,
    pFree: ::core::ptr::null::<PgFreeslot>() as *mut PgFreeslot,
    nFreeSlot: 0,
    bUnderPressure: 0

};
#[no_mangle]
pub unsafe extern "C" fn sqlite3PCacheBufferSetup(
    mut pBuf: *mut ::core::ffi::c_void,
    mut sz: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) {
    unsafe {
        if pcache1_g.isInit != 0 {
            let mut p: *mut PgFreeslot = ::core::ptr::null_mut::<PgFreeslot>();
            if pBuf.is_null() {
                n = 0 as ::core::ffi::c_int;
                sz = n;
            }
            if n == 0 as ::core::ffi::c_int {
                sz = 0 as ::core::ffi::c_int;
            }
            sz = sz & !(7 as ::core::ffi::c_int);
            pcache1_g.szSlot = sz;
            pcache1_g.nFreeSlot = n;
            pcache1_g.nSlot = pcache1_g.nFreeSlot;
            pcache1_g.nReserve = if n > 90 as ::core::ffi::c_int {
                10 as ::core::ffi::c_int
            } else {
                n / 10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            };
            pcache1_g.pStart = pBuf;
            pcache1_g.pFree = ::core::ptr::null_mut::<PgFreeslot>();
            ;
            loop {
                let c2rust_fresh0 = n;
                n = n - 1;
                if !(c2rust_fresh0 != 0) {
                    break;
                }
                p = pBuf as *mut PgFreeslot;
                (*p).pNext = pcache1_g.pFree;
                pcache1_g.pFree = p;
                pBuf = (pBuf as *mut ::core::ffi::c_char).offset(sz as isize)
                    as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
            }
            pcache1_g.pEnd = pBuf;
        }
    }
}
unsafe extern "C" fn pcache1InitBulk(mut pCache: *mut PCache1) -> ::core::ffi::c_int {
    unsafe {
        let mut szBulk: i64_0 = 0;
        let mut zBulk: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if pcache1_g.nInitPage == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if (*pCache).nMax < 3 as ::core::ffi::c_uint {
            return 0 as ::core::ffi::c_int;
        }
        sqlite3BeginBenignMalloc();
        if pcache1_g.nInitPage > 0 as ::core::ffi::c_int {
            szBulk = (*pCache).szAlloc as i64_0 * pcache1_g.nInitPage as i64_0;
        } else {
            szBulk = -1024 as ::core::ffi::c_int as i64_0 * pcache1_g.nInitPage as i64_0;
        }
        if szBulk > (*pCache).szAlloc as i64_0 * (*pCache).nMax as i64_0 {
            szBulk = (*pCache).szAlloc as i64_0 * (*pCache).nMax as i64_0;
        }
        (*pCache).pBulk = sqlite3Malloc(szBulk as u64_0);
        zBulk = (*pCache).pBulk as *mut ::core::ffi::c_char;
        sqlite3EndBenignMalloc();
        if !zBulk.is_null() {
            let mut nBulk: ::core::ffi::c_int = sqlite3MallocSize(
                zBulk as *const ::core::ffi::c_void,
            ) / (*pCache).szAlloc;
            loop {
                let mut pX: *mut PgHdr1 = zBulk.offset((*pCache).szPage as isize)
                    as *mut ::core::ffi::c_char as *mut PgHdr1;
                (*pX).page.pBuf = zBulk as *mut ::core::ffi::c_void;
                (*pX).page.pExtra = (pX as *mut u8_0)
                    .offset(
                        ((::core::mem::size_of::<PgHdr1>() as usize)
                            .wrapping_add(7 as usize)
                            & !(7 as ::core::ffi::c_int) as usize) as isize,
                    ) as *mut ::core::ffi::c_void;
                (*pX).isBulkLocal = 1 as u16_0;
                (*pX).isAnchor = 0 as u16_0;
                (*pX).pNext = (*pCache).pFree;
                (*pX).pLruPrev = ::core::ptr::null_mut::<PgHdr1>();
                (*pCache).pFree = pX;
                zBulk = zBulk.offset((*pCache).szAlloc as isize);
                nBulk -= 1;
                if !(nBulk != 0) {
                    break;
                }
            }
        }
        return !(*pCache).pFree.is_null() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn pcache1Alloc(
    mut nByte: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        if nByte <= pcache1_g.szSlot {
            sqlite3_mutex_enter(pcache1_g.mutex);
            p = pcache1_g.pFree as *mut PgHdr1 as *mut ::core::ffi::c_void;
            if !p.is_null() {
                pcache1_g.pFree = (*pcache1_g.pFree).pNext;
                pcache1_g.nFreeSlot -= 1;
                ;
                sqlite3StatusHighwater(SQLITE_STATUS_PAGECACHE_SIZE, nByte);
                sqlite3StatusUp(SQLITE_STATUS_PAGECACHE_USED, 1 as ::core::ffi::c_int);
            }
            sqlite3_mutex_leave(pcache1_g.mutex);
        }
        if p.is_null() {
            p = sqlite3Malloc(nByte as u64_0);
            if !p.is_null() {
                let mut sz: ::core::ffi::c_int = sqlite3MallocSize(p);
                sqlite3_mutex_enter(pcache1_g.mutex);
                sqlite3StatusHighwater(SQLITE_STATUS_PAGECACHE_SIZE, nByte);
                sqlite3StatusUp(SQLITE_STATUS_PAGECACHE_OVERFLOW, sz);
                sqlite3_mutex_leave(pcache1_g.mutex);
            }
        }
        return p;
    }
}
unsafe extern "C" fn pcache1Free(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        if p.is_null() {
            return;
        }
        if p as uptr >= pcache1_g.pStart as uptr && (p as uptr) < pcache1_g.pEnd as uptr
        {
            let mut pSlot: *mut PgFreeslot = ::core::ptr::null_mut::<PgFreeslot>();
            sqlite3_mutex_enter(pcache1_g.mutex);
            sqlite3StatusDown(SQLITE_STATUS_PAGECACHE_USED, 1 as ::core::ffi::c_int);
            pSlot = p as *mut PgFreeslot;
            (*pSlot).pNext = pcache1_g.pFree;
            pcache1_g.pFree = pSlot;
            pcache1_g.nFreeSlot += 1;
            ;
            sqlite3_mutex_leave(pcache1_g.mutex);
        } else {
            let mut nFreed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            nFreed = sqlite3MallocSize(p);
            sqlite3_mutex_enter(pcache1_g.mutex);
            sqlite3StatusDown(SQLITE_STATUS_PAGECACHE_OVERFLOW, nFreed);
            sqlite3_mutex_leave(pcache1_g.mutex);
            sqlite3_free(p);
        };
    }
}
unsafe extern "C" fn pcache1AllocPage(
    mut pCache: *mut PCache1,
    mut benignMalloc: ::core::ffi::c_int,
) -> *mut PgHdr1 {
    unsafe {
        let mut p: *mut PgHdr1 = ::core::ptr::null_mut::<PgHdr1>();
        let mut pPg: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        if !(*pCache).pFree.is_null()
            || (*pCache).nPage == 0 as ::core::ffi::c_uint
                && pcache1InitBulk(pCache) != 0
        {
            p = (*pCache).pFree;
            (*pCache).pFree = (*p).pNext;
            (*p).pNext = ::core::ptr::null_mut::<PgHdr1>();
        } else {
            if benignMalloc != 0 {
                sqlite3BeginBenignMalloc();
            }
            pPg = pcache1Alloc((*pCache).szAlloc);
            if benignMalloc != 0 {
                sqlite3EndBenignMalloc();
            }
            if pPg.is_null() {
                return ::core::ptr::null_mut::<PgHdr1>();
            }
            p = (pPg as *mut u8_0).offset((*pCache).szPage as isize) as *mut u8_0
                as *mut PgHdr1;
            (*p).page.pBuf = pPg;
            (*p).page.pExtra = (p as *mut u8_0)
                .offset(
                    ((::core::mem::size_of::<PgHdr1>() as usize).wrapping_add(7 as usize)
                        & !(7 as ::core::ffi::c_int) as usize) as isize,
                ) as *mut ::core::ffi::c_void;
            (*p).isBulkLocal = 0 as u16_0;
            (*p).isAnchor = 0 as u16_0;
            (*p).pLruPrev = ::core::ptr::null_mut::<PgHdr1>();
        }
        *(*pCache).pnPurgeable = (*(*pCache).pnPurgeable).wrapping_add(1);
        return p;
    }
}
unsafe extern "C" fn pcache1FreePage(mut p: *mut PgHdr1) {
    unsafe {
        let mut pCache: *mut PCache1 = ::core::ptr::null_mut::<PCache1>();
        pCache = (*p).pCache;
        if (*p).isBulkLocal != 0 {
            (*p).pNext = (*pCache).pFree;
            (*pCache).pFree = p;
        } else {
            pcache1Free((*p).page.pBuf);
        }
        *(*pCache).pnPurgeable = (*(*pCache).pnPurgeable).wrapping_sub(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PageMalloc(
    mut sz: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return pcache1Alloc(sz);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PageFree(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        pcache1Free(p);
    }
}
unsafe extern "C" fn pcache1UnderMemoryPressure(
    mut pCache: *mut PCache1,
) -> ::core::ffi::c_int {
    unsafe {
        if pcache1_g.nSlot != 0
            && (*pCache).szPage + (*pCache).szExtra <= pcache1_g.szSlot
        {
            return pcache1_g.bUnderPressure

        } else {
            return sqlite3HeapNearlyFull()
        };
    }
}
unsafe extern "C" fn pcache1ResizeHash(mut p: *mut PCache1) {
    unsafe {
        let mut apNew: *mut *mut PgHdr1 = ::core::ptr::null_mut::<*mut PgHdr1>();
        let mut nNew: u64_0 = 0;
        let mut i: u32_0 = 0;
        nNew = (2 as u64_0).wrapping_mul((*p).nHash as u64_0);
        if nNew < 256 as ::core::ffi::c_ulonglong {
            nNew = 256 as u64_0;
        }
        if (*p).nHash != 0 {
            sqlite3BeginBenignMalloc();
        }
        apNew = sqlite3MallocZero(
            (::core::mem::size_of::<*mut PgHdr1>() as u64_0).wrapping_mul(nNew),
        ) as *mut *mut PgHdr1;
        if (*p).nHash != 0 {
            sqlite3EndBenignMalloc();
        }
        if !apNew.is_null() {
            i = 0 as u32_0;
            while i < (*p).nHash {
                let mut pPage: *mut PgHdr1 = ::core::ptr::null_mut::<PgHdr1>();
                let mut pNext: *mut PgHdr1 = *(*p).apHash.offset(i as isize);
                loop {
                    pPage = pNext;
                    if pPage.is_null() {
                        break;
                    }
                    let mut h: ::core::ffi::c_uint = ((*pPage).iKey as u64_0)
                        .wrapping_rem(nNew) as ::core::ffi::c_uint;
                    pNext = (*pPage).pNext;
                    (*pPage).pNext = *apNew.offset(h as isize);
                    let ref mut c2rust_fresh4 = *apNew.offset(h as isize);
                    *c2rust_fresh4 = pPage;
                }
                i = i.wrapping_add(1);
            }
            sqlite3_free((*p).apHash as *mut ::core::ffi::c_void);
            (*p).apHash = apNew;
            (*p).nHash = nNew as ::core::ffi::c_uint;
        }
    }
}
unsafe extern "C" fn pcache1PinPage(mut pPage: *mut PgHdr1) -> *mut PgHdr1 {
    unsafe {
        (*(*pPage).pLruPrev).pLruNext = (*pPage).pLruNext;
        (*(*pPage).pLruNext).pLruPrev = (*pPage).pLruPrev;
        (*pPage).pLruNext = ::core::ptr::null_mut::<PgHdr1>();
        (*(*pPage).pCache).nRecyclable = (*(*pPage).pCache).nRecyclable.wrapping_sub(1);
        return pPage;
    }
}
unsafe extern "C" fn pcache1RemoveFromHash(
    mut pPage: *mut PgHdr1,
    mut freeFlag: ::core::ffi::c_int,
) {
    unsafe {
        let mut h: ::core::ffi::c_uint = 0;
        let mut pCache: *mut PCache1 = (*pPage).pCache;
        let mut pp: *mut *mut PgHdr1 = ::core::ptr::null_mut::<*mut PgHdr1>();
        h = (*pPage).iKey.wrapping_rem((*pCache).nHash);
        pp = (*pCache).apHash.offset(h as isize) as *mut *mut PgHdr1;
        while *pp != pPage {
            pp = &raw mut (**pp).pNext;
        }
        *pp = (**pp).pNext;
        (*pCache).nPage = (*pCache).nPage.wrapping_sub(1);
        if freeFlag != 0 {
            pcache1FreePage(pPage);
        }
    }
}
unsafe extern "C" fn pcache1EnforceMaxPage(mut pCache: *mut PCache1) {
    unsafe {
        let mut pGroup: *mut PGroup = (*pCache).pGroup;
        let mut p: *mut PgHdr1 = ::core::ptr::null_mut::<PgHdr1>();
        while (*pGroup).nPurgeable > (*pGroup).nMaxPage
            && {
                p = (*pGroup).lru.pLruPrev;
                (*p).isAnchor as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            }
        {
            pcache1PinPage(p);
            pcache1RemoveFromHash(p, 1 as ::core::ffi::c_int);
        }
        if (*pCache).nPage == 0 as ::core::ffi::c_uint && !(*pCache).pBulk.is_null() {
            sqlite3_free((*pCache).pBulk);
            (*pCache).pFree = ::core::ptr::null_mut::<PgHdr1>();
            (*pCache).pBulk = (*pCache).pFree as *mut ::core::ffi::c_void;
        }
    }
}
unsafe extern "C" fn pcache1TruncateUnsafe(
    mut pCache: *mut PCache1,
    mut iLimit: ::core::ffi::c_uint,
) {
    unsafe {
        let mut h: ::core::ffi::c_uint = 0;
        let mut iStop: ::core::ffi::c_uint = 0;
        if (*pCache).iMaxKey.wrapping_sub(iLimit) < (*pCache).nHash {
            h = iLimit.wrapping_rem((*pCache).nHash);
            iStop = (*pCache).iMaxKey.wrapping_rem((*pCache).nHash);
        } else {
            h = (*pCache).nHash.wrapping_div(2 as ::core::ffi::c_uint);
            iStop = h.wrapping_sub(1 as ::core::ffi::c_uint);
        }
        loop {
            let mut pp: *mut *mut PgHdr1 = ::core::ptr::null_mut::<*mut PgHdr1>();
            let mut pPage: *mut PgHdr1 = ::core::ptr::null_mut::<PgHdr1>();
            pp = (*pCache).apHash.offset(h as isize) as *mut *mut PgHdr1;
            loop {
                pPage = *pp;
                if pPage.is_null() {
                    break;
                }
                if (*pPage).iKey >= iLimit {
                    (*pCache).nPage = (*pCache).nPage.wrapping_sub(1);
                    *pp = (*pPage).pNext;
                    if !(*pPage).pLruNext.is_null() {
                        pcache1PinPage(pPage);
                    }
                    pcache1FreePage(pPage);
                } else {
                    pp = &raw mut (*pPage).pNext;
                }
            }
            if h == iStop {
                break;
            }
            h = h.wrapping_add(1 as ::core::ffi::c_uint).wrapping_rem((*pCache).nHash);
        };
    }
}
unsafe extern "C" fn pcache1Init(
    mut NotUsed: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        memset(
            &raw mut pcache1_g as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<PCacheGlobal>() as size_t,
        );
        pcache1_g.separateCache = (sqlite3Config.pPage.is_null()
            || sqlite3Config.bCoreMutex as ::core::ffi::c_int > 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        if sqlite3Config.bCoreMutex != 0 {
            pcache1_g.grp.mutex = sqlite3MutexAlloc(SQLITE_MUTEX_STATIC_LRU);
            pcache1_g.mutex = sqlite3MutexAlloc(SQLITE_MUTEX_STATIC_PMEM);
        }
        if pcache1_g.separateCache != 0 && sqlite3Config.nPage != 0 as ::core::ffi::c_int
            && sqlite3Config.pPage.is_null()
        {
            pcache1_g.nInitPage = sqlite3Config.nPage;
        } else {
            pcache1_g.nInitPage = 0 as ::core::ffi::c_int;
        }
        pcache1_g.grp.mxPinned = 10 as ::core::ffi::c_uint;
        pcache1_g.isInit = 1 as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pcache1Shutdown(mut NotUsed: *mut ::core::ffi::c_void) {
    unsafe {
        memset(
            &raw mut pcache1_g as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<PCacheGlobal>() as size_t,
        );
    }
}
unsafe extern "C" fn pcache1Create(
    mut szPage: ::core::ffi::c_int,
    mut szExtra: ::core::ffi::c_int,
    mut bPurgeable: ::core::ffi::c_int,
) -> *mut sqlite3_pcache {
    unsafe {
        let mut pCache: *mut PCache1 = ::core::ptr::null_mut::<PCache1>();
        let mut pGroup: *mut PGroup = ::core::ptr::null_mut::<PGroup>();
        let mut sz: i64_0 = 0;
        sz = (::core::mem::size_of::<PCache1>() as usize)
            .wrapping_add(
                (::core::mem::size_of::<PGroup>() as usize)
                    .wrapping_mul(pcache1_g.separateCache as usize),
            ) as i64_0;
        pCache = sqlite3MallocZero(sz as u64_0) as *mut PCache1;
        if !pCache.is_null() {
            if pcache1_g.separateCache != 0 {
                pGroup = pCache.offset(1 as ::core::ffi::c_int as isize) as *mut PCache1
                    as *mut PGroup;
                (*pGroup).mxPinned = 10 as ::core::ffi::c_uint;
            } else {
                pGroup = &raw mut pcache1_g.grp;
            }
            if (*pGroup).lru.isAnchor as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*pGroup).lru.isAnchor = 1 as u16_0;
                (*pGroup).lru.pLruNext = &raw mut (*pGroup).lru;
                (*pGroup).lru.pLruPrev = (*pGroup).lru.pLruNext;
            }
            (*pCache).pGroup = pGroup;
            (*pCache).szPage = szPage;
            (*pCache).szExtra = szExtra;
            (*pCache).szAlloc = ((szPage + szExtra) as usize)
                .wrapping_add(
                    (::core::mem::size_of::<PgHdr1>() as usize).wrapping_add(7 as usize)
                        & !(7 as ::core::ffi::c_int) as usize,
                ) as ::core::ffi::c_int;
            (*pCache).bPurgeable = if bPurgeable != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            pcache1ResizeHash(pCache);
            if bPurgeable != 0 {
                (*pCache).nMin = 10 as ::core::ffi::c_uint;
                (*pGroup).nMinPage = (*pGroup).nMinPage.wrapping_add((*pCache).nMin);
                (*pGroup).mxPinned = (*pGroup)
                    .nMaxPage
                    .wrapping_add(10 as ::core::ffi::c_uint)
                    .wrapping_sub((*pGroup).nMinPage);
                (*pCache).pnPurgeable = &raw mut (*pGroup).nPurgeable;
            } else {
                (*pCache).pnPurgeable = &raw mut (*pCache).nPurgeableDummy;
            }
            if (*pCache).nHash == 0 as ::core::ffi::c_uint {
                pcache1Destroy(pCache as *mut sqlite3_pcache);
                pCache = ::core::ptr::null_mut::<PCache1>();
            }
        }
        return pCache as *mut sqlite3_pcache;
    }
}
unsafe extern "C" fn pcache1Cachesize(
    mut p: *mut sqlite3_pcache,
    mut nMax: ::core::ffi::c_int,
) {
    unsafe {
        let mut pCache: *mut PCache1 = p as *mut PCache1;
        let mut n: u32_0 = 0;
        if (*pCache).bPurgeable != 0 {
            let mut pGroup: *mut PGroup = (*pCache).pGroup;
            n = nMax as u32_0;
            if n
                > (0x7fff0000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_sub((*pGroup).nMaxPage)
                    .wrapping_add((*pCache).nMax)
            {
                n = (0x7fff0000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_sub((*pGroup).nMaxPage)
                    .wrapping_add((*pCache).nMax) as u32_0;
            }
            (*pGroup).nMaxPage = (*pGroup)
                .nMaxPage
                .wrapping_add((n as ::core::ffi::c_uint).wrapping_sub((*pCache).nMax));
            (*pGroup).mxPinned = (*pGroup)
                .nMaxPage
                .wrapping_add(10 as ::core::ffi::c_uint)
                .wrapping_sub((*pGroup).nMinPage);
            (*pCache).nMax = n as ::core::ffi::c_uint;
            (*pCache).n90pct = (*pCache)
                .nMax
                .wrapping_mul(9 as ::core::ffi::c_uint)
                .wrapping_div(10 as ::core::ffi::c_uint);
            pcache1EnforceMaxPage(pCache);
        }
    }
}
unsafe extern "C" fn pcache1Shrink(mut p: *mut sqlite3_pcache) {
    unsafe {
        let mut pCache: *mut PCache1 = p as *mut PCache1;
        if (*pCache).bPurgeable != 0 {
            let mut pGroup: *mut PGroup = (*pCache).pGroup;
            let mut savedMaxPage: ::core::ffi::c_uint = 0;
            savedMaxPage = (*pGroup).nMaxPage;
            (*pGroup).nMaxPage = 0 as ::core::ffi::c_uint;
            pcache1EnforceMaxPage(pCache);
            (*pGroup).nMaxPage = savedMaxPage;
        }
    }
}
unsafe extern "C" fn pcache1Pagecount(mut p: *mut sqlite3_pcache) -> ::core::ffi::c_int {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut pCache: *mut PCache1 = p as *mut PCache1;
        n = (*pCache).nPage as ::core::ffi::c_int;
        return n;
    }
}
#[inline(never)]
unsafe extern "C" fn pcache1FetchStage2(
    mut pCache: *mut PCache1,
    mut iKey: ::core::ffi::c_uint,
    mut createFlag: ::core::ffi::c_int,
) -> *mut PgHdr1 {
    unsafe {
        let mut nPinned: ::core::ffi::c_uint = 0;
        let mut pGroup: *mut PGroup = (*pCache).pGroup;
        let mut pPage: *mut PgHdr1 = ::core::ptr::null_mut::<PgHdr1>();
        nPinned = (*pCache).nPage.wrapping_sub((*pCache).nRecyclable);
        if createFlag == 1 as ::core::ffi::c_int
            && (nPinned >= (*pGroup).mxPinned || nPinned >= (*pCache).n90pct
                || pcache1UnderMemoryPressure(pCache) != 0
                    && (*pCache).nRecyclable < nPinned)
        {
            return ::core::ptr::null_mut::<PgHdr1>();
        }
        if (*pCache).nPage >= (*pCache).nHash {
            pcache1ResizeHash(pCache);
        }
        if (*pCache).bPurgeable != 0 && (*(*pGroup).lru.pLruPrev).isAnchor == 0
            && ((*pCache).nPage.wrapping_add(1 as ::core::ffi::c_uint) >= (*pCache).nMax
                || pcache1UnderMemoryPressure(pCache) != 0)
        {
            let mut pOther: *mut PCache1 = ::core::ptr::null_mut::<PCache1>();
            pPage = (*pGroup).lru.pLruPrev;
            pcache1RemoveFromHash(pPage, 0 as ::core::ffi::c_int);
            pcache1PinPage(pPage);
            pOther = (*pPage).pCache;
            if (*pOther).szAlloc != (*pCache).szAlloc {
                pcache1FreePage(pPage);
                pPage = ::core::ptr::null_mut::<PgHdr1>();
            } else {
                (*pGroup).nPurgeable = (*pGroup)
                    .nPurgeable
                    .wrapping_sub(
                        ((*pOther).bPurgeable - (*pCache).bPurgeable)
                            as ::core::ffi::c_uint,
                    );
            }
        }
        if pPage.is_null() {
            pPage = pcache1AllocPage(
                pCache,
                (createFlag == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
        }
        if !pPage.is_null() {
            let mut h: ::core::ffi::c_uint = iKey.wrapping_rem((*pCache).nHash);
            (*pCache).nPage = (*pCache).nPage.wrapping_add(1);
            (*pPage).iKey = iKey;
            (*pPage).pNext = *(*pCache).apHash.offset(h as isize);
            (*pPage).pCache = pCache;
            (*pPage).pLruNext = ::core::ptr::null_mut::<PgHdr1>();
            let ref mut c2rust_fresh2 = *((*pPage).page.pExtra
                as *mut *mut ::core::ffi::c_void);
            *c2rust_fresh2 = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let ref mut c2rust_fresh3 = *(*pCache).apHash.offset(h as isize);
            *c2rust_fresh3 = pPage;
            if iKey > (*pCache).iMaxKey {
                (*pCache).iMaxKey = iKey;
            }
        }
        return pPage;
    }
}
unsafe extern "C" fn pcache1FetchNoMutex(
    mut p: *mut sqlite3_pcache,
    mut iKey: ::core::ffi::c_uint,
    mut createFlag: ::core::ffi::c_int,
) -> *mut PgHdr1 {
    unsafe {
        let mut pCache: *mut PCache1 = p as *mut PCache1;
        let mut pPage: *mut PgHdr1 = ::core::ptr::null_mut::<PgHdr1>();
        pPage = *(*pCache).apHash.offset(iKey.wrapping_rem((*pCache).nHash) as isize);
        while !pPage.is_null() && (*pPage).iKey != iKey {
            pPage = (*pPage).pNext;
        }
        if !pPage.is_null() {
            if !(*pPage).pLruNext.is_null() {
                return pcache1PinPage(pPage)
            } else {
                return pPage
            }
        } else if createFlag != 0 {
            return pcache1FetchStage2(pCache, iKey, createFlag)
        } else {
            return ::core::ptr::null_mut::<PgHdr1>()
        };
    }
}
unsafe extern "C" fn pcache1Fetch(
    mut p: *mut sqlite3_pcache,
    mut iKey: ::core::ffi::c_uint,
    mut createFlag: ::core::ffi::c_int,
) -> *mut sqlite3_pcache_page {
    unsafe {
        return pcache1FetchNoMutex(p, iKey, createFlag) as *mut sqlite3_pcache_page;
    }
}
unsafe extern "C" fn pcache1Unpin(
    mut p: *mut sqlite3_pcache,
    mut pPg: *mut sqlite3_pcache_page,
    mut reuseUnlikely: ::core::ffi::c_int,
) {
    unsafe {
        let mut pCache: *mut PCache1 = p as *mut PCache1;
        let mut pPage: *mut PgHdr1 = pPg as *mut PgHdr1;
        let mut pGroup: *mut PGroup = (*pCache).pGroup;
        if reuseUnlikely != 0 || (*pGroup).nPurgeable > (*pGroup).nMaxPage {
            pcache1RemoveFromHash(pPage, 1 as ::core::ffi::c_int);
        } else {
            let mut ppFirst: *mut *mut PgHdr1 = &raw mut (*pGroup).lru.pLruNext;
            (*pPage).pLruPrev = &raw mut (*pGroup).lru;
            (*pPage).pLruNext = *ppFirst;
            (*(*pPage).pLruNext).pLruPrev = pPage;
            *ppFirst = pPage;
            (*pCache).nRecyclable = (*pCache).nRecyclable.wrapping_add(1);
        };
    }
}
unsafe extern "C" fn pcache1Rekey(
    mut p: *mut sqlite3_pcache,
    mut pPg: *mut sqlite3_pcache_page,
    mut iOld: ::core::ffi::c_uint,
    mut iNew: ::core::ffi::c_uint,
) {
    unsafe {
        let mut pCache: *mut PCache1 = p as *mut PCache1;
        let mut pPage: *mut PgHdr1 = pPg as *mut PgHdr1;
        let mut pp: *mut *mut PgHdr1 = ::core::ptr::null_mut::<*mut PgHdr1>();
        let mut hOld: ::core::ffi::c_uint = 0;
        let mut hNew: ::core::ffi::c_uint = 0;
        hOld = iOld.wrapping_rem((*pCache).nHash);
        pp = (*pCache).apHash.offset(hOld as isize) as *mut *mut PgHdr1;
        while *pp != pPage {
            pp = &raw mut (**pp).pNext;
        }
        *pp = (*pPage).pNext;
        hNew = iNew.wrapping_rem((*pCache).nHash);
        (*pPage).iKey = iNew;
        (*pPage).pNext = *(*pCache).apHash.offset(hNew as isize);
        let ref mut c2rust_fresh1 = *(*pCache).apHash.offset(hNew as isize);
        *c2rust_fresh1 = pPage;
        if iNew > (*pCache).iMaxKey {
            (*pCache).iMaxKey = iNew;
        }
    }
}
unsafe extern "C" fn pcache1Truncate(
    mut p: *mut sqlite3_pcache,
    mut iLimit: ::core::ffi::c_uint,
) {
    unsafe {
        let mut pCache: *mut PCache1 = p as *mut PCache1;
        if iLimit <= (*pCache).iMaxKey {
            pcache1TruncateUnsafe(pCache, iLimit);
            (*pCache).iMaxKey = iLimit.wrapping_sub(1 as ::core::ffi::c_uint);
        }
    }
}
unsafe extern "C" fn pcache1Destroy(mut p: *mut sqlite3_pcache) {
    unsafe {
        let mut pCache: *mut PCache1 = p as *mut PCache1;
        let mut pGroup: *mut PGroup = (*pCache).pGroup;
        if (*pCache).nPage != 0 {
            pcache1TruncateUnsafe(pCache, 0 as ::core::ffi::c_uint);
        }
        (*pGroup).nMaxPage = (*pGroup).nMaxPage.wrapping_sub((*pCache).nMax);
        (*pGroup).nMinPage = (*pGroup).nMinPage.wrapping_sub((*pCache).nMin);
        (*pGroup).mxPinned = (*pGroup)
            .nMaxPage
            .wrapping_add(10 as ::core::ffi::c_uint)
            .wrapping_sub((*pGroup).nMinPage);
        pcache1EnforceMaxPage(pCache);
        sqlite3_free((*pCache).pBulk);
        sqlite3_free((*pCache).apHash as *mut ::core::ffi::c_void);
        sqlite3_free(pCache as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PCacheSetDefault() {
    unsafe {
        static mut defaultMethods: sqlite3_pcache_methods2 = unsafe {
            sqlite3_pcache_methods2 {
                iVersion: 1 as ::core::ffi::c_int,
                pArg: ::core::ptr::null::<::core::ffi::c_void>()
                    as *mut ::core::ffi::c_void,
                xInit: Some(
                    pcache1Init
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                xShutdown: Some(
                    pcache1Shutdown
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
                xCreate: Some(
                    pcache1Create
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> *mut sqlite3_pcache,
                ),
                xCachesize: Some(
                    pcache1Cachesize
                        as unsafe extern "C" fn(
                            *mut sqlite3_pcache,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
                xPagecount: Some(
                    pcache1Pagecount
                        as unsafe extern "C" fn(
                            *mut sqlite3_pcache,
                        ) -> ::core::ffi::c_int,
                ),
                xFetch: Some(
                    pcache1Fetch
                        as unsafe extern "C" fn(
                            *mut sqlite3_pcache,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_int,
                        ) -> *mut sqlite3_pcache_page,
                ),
                xUnpin: Some(
                    pcache1Unpin
                        as unsafe extern "C" fn(
                            *mut sqlite3_pcache,
                            *mut sqlite3_pcache_page,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
                xRekey: Some(
                    pcache1Rekey
                        as unsafe extern "C" fn(
                            *mut sqlite3_pcache,
                            *mut sqlite3_pcache_page,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_uint,
                        ) -> (),
                ),
                xTruncate: Some(
                    pcache1Truncate
                        as unsafe extern "C" fn(
                            *mut sqlite3_pcache,
                            ::core::ffi::c_uint,
                        ) -> (),
                ),
                xDestroy: Some(
                    pcache1Destroy as unsafe extern "C" fn(*mut sqlite3_pcache) -> (),
                ),
                xShrink: Some(
                    pcache1Shrink as unsafe extern "C" fn(*mut sqlite3_pcache) -> (),
                )

            }
        };
        sqlite3_config(SQLITE_CONFIG_PCACHE2, &raw const defaultMethods);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3HeaderSizePcache1() -> ::core::ffi::c_int {
    unsafe {
        return ((::core::mem::size_of::<PgHdr1>() as usize).wrapping_add(7 as usize)
            & !(7 as ::core::ffi::c_int) as usize) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Pcache1Mutex() -> *mut sqlite3_mutex {
    unsafe {
        return pcache1_g.mutex;
    }
}
pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
