extern "C" {
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3Malloc(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3FaultSim(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
pub type u64_0 = sqlite_uint64;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type size_t = usize;
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SQLiteThread {
    pub tid: pthread_t,
    pub done: ::core::ffi::c_int,
    pub pOut: *mut ::core::ffi::c_void,
    pub xTask: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    >,
    pub pIn: *mut ::core::ffi::c_void

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3ThreadCreate(
    mut ppThread: *mut *mut SQLiteThread,
    mut xTask: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    >,
    mut pIn: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut SQLiteThread = ::core::ptr::null_mut::<SQLiteThread>();
        let mut rc: ::core::ffi::c_int = 0;
        *ppThread = ::core::ptr::null_mut::<SQLiteThread>();
        p = sqlite3Malloc(::core::mem::size_of::<SQLiteThread>() as u64_0)
            as *mut SQLiteThread;
        if p.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<SQLiteThread>() as size_t,
        );
        (*p).xTask = xTask;
        (*p).pIn = pIn;
        if sqlite3FaultSim(200 as ::core::ffi::c_int) != 0 {
            rc = 1 as ::core::ffi::c_int;
        } else {
            rc = pthread_create(
                &raw mut (*p).tid,
                ::core::ptr::null::<pthread_attr_t>(),
                xTask,
                pIn,
            );
        }
        if rc != 0 {
            (*p).done = 1 as ::core::ffi::c_int;
            (*p).pOut = xTask.expect("non-null function pointer")(pIn);
        }
        *ppThread = p;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ThreadJoin(
    mut p: *mut SQLiteThread,
    mut ppOut: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if p.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        if (*p).done != 0 {
            *ppOut = (*p).pOut;
            rc = SQLITE_OK;
        } else {
            rc = if pthread_join((*p).tid, ppOut) != 0 {
                SQLITE_ERROR
            } else {
                SQLITE_OK
            };
        }
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return rc;
    }
}
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
