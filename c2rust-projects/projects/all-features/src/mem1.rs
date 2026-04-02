extern "C" {
    fn sqlite3_config(_: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn sqlite3_log(
        iErrCode: ::core::ffi::c_int,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
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
pub type size_t = usize;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_MALLOC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn sqlite3MemMalloc(
    mut nByte: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut p: *mut sqlite3_int64 = ::core::ptr::null_mut::<sqlite3_int64>();
        p = malloc((nByte + 8 as ::core::ffi::c_int) as size_t) as *mut sqlite3_int64;
        if !p.is_null() {
            *p.offset(0 as ::core::ffi::c_int as isize) = nByte as sqlite3_int64;
            p = p.offset(1);
        } else {
            sqlite3_log(
                SQLITE_NOMEM,
                b"failed to allocate %u bytes of memory\0".as_ptr()
                    as *const ::core::ffi::c_char,
                nByte,
            );
        }
        return p as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn sqlite3MemFree(mut pPrior: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut sqlite3_int64 = pPrior as *mut sqlite3_int64;
        p = p.offset(-1);
        free(p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn sqlite3MemSize(
    mut pPrior: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut sqlite3_int64 = ::core::ptr::null_mut::<sqlite3_int64>();
        p = pPrior as *mut sqlite3_int64;
        p = p.offset(-1);
        return *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn sqlite3MemRealloc(
    mut pPrior: *mut ::core::ffi::c_void,
    mut nByte: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut p: *mut sqlite3_int64 = pPrior as *mut sqlite3_int64;
        p = p.offset(-1);
        p = realloc(
            p as *mut ::core::ffi::c_void,
            (nByte + 8 as ::core::ffi::c_int) as size_t,
        ) as *mut sqlite3_int64;
        if !p.is_null() {
            *p.offset(0 as ::core::ffi::c_int as isize) = nByte as sqlite3_int64;
            p = p.offset(1);
        } else {
            sqlite3_log(
                SQLITE_NOMEM,
                b"failed memory resize %u to %u bytes\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3MemSize(pPrior),
                nByte,
            );
        }
        return p as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn sqlite3MemRoundup(mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return n + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn sqlite3MemInit(
    mut NotUsed: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn sqlite3MemShutdown(mut NotUsed: *mut ::core::ffi::c_void) {}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MemSetDefault() {
    unsafe {
        static mut defaultMethods: sqlite3_mem_methods = unsafe {
            sqlite3_mem_methods {
                xMalloc: Some(
                    sqlite3MemMalloc
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                        ) -> *mut ::core::ffi::c_void,
                ),
                xFree: Some(
                    sqlite3MemFree
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
                xRealloc: Some(
                    sqlite3MemRealloc
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                        ) -> *mut ::core::ffi::c_void,
                ),
                xSize: Some(
                    sqlite3MemSize
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                xRoundup: Some(
                    sqlite3MemRoundup
                        as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
                ),
                xInit: Some(
                    sqlite3MemInit
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                xShutdown: Some(
                    sqlite3MemShutdown
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
                pAppData: ::core::ptr::null::<::core::ffi::c_void>()
                    as *mut ::core::ffi::c_void

            }
        };
        sqlite3_config(SQLITE_CONFIG_MALLOC, &raw const defaultMethods);
    }
}
