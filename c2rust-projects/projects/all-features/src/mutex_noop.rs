extern "C" {
    pub type sqlite3_mutex;
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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn noopMutexInit() -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn noopMutexEnd() -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn noopMutexAlloc(mut id: ::core::ffi::c_int) -> *mut sqlite3_mutex {
    unsafe {
        return 8 as ::core::ffi::c_int as *mut sqlite3_mutex;
    }
}
unsafe extern "C" fn noopMutexFree(mut p: *mut sqlite3_mutex) {}
unsafe extern "C" fn noopMutexEnter(mut p: *mut sqlite3_mutex) {}
unsafe extern "C" fn noopMutexTry(mut p: *mut sqlite3_mutex) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn noopMutexLeave(mut p: *mut sqlite3_mutex) {}
#[no_mangle]
pub unsafe extern "C" fn sqlite3NoopMutex() -> *const sqlite3_mutex_methods {
    unsafe {
        static mut sMutex: sqlite3_mutex_methods = unsafe {
            sqlite3_mutex_methods {
                xMutexInit: Some(
                    noopMutexInit as unsafe extern "C" fn() -> ::core::ffi::c_int,
                ),
                xMutexEnd: Some(
                    noopMutexEnd as unsafe extern "C" fn() -> ::core::ffi::c_int,
                ),
                xMutexAlloc: Some(
                    noopMutexAlloc
                        as unsafe extern "C" fn(::core::ffi::c_int) -> *mut sqlite3_mutex,
                ),
                xMutexFree: Some(
                    noopMutexFree as unsafe extern "C" fn(*mut sqlite3_mutex) -> (),
                ),
                xMutexEnter: Some(
                    noopMutexEnter as unsafe extern "C" fn(*mut sqlite3_mutex) -> (),
                ),
                xMutexTry: Some(
                    noopMutexTry
                        as unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
                ),
                xMutexLeave: Some(
                    noopMutexLeave as unsafe extern "C" fn(*mut sqlite3_mutex) -> (),
                ),
                xMutexHeld: None,
                xMutexNotheld: None

            }
        };
        return &raw const sMutex;
    }
}
