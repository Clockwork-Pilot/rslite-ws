extern "C" {
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3MallocZero(_: u64_0) -> *mut ::core::ffi::c_void;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> ::core::ffi::c_int;
    fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> ::core::ffi::c_int;
    fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type u64_0 = sqlite_uint64;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_mutex {
    pub mutex: pthread_mutex_t

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __elision: ::core::ffi::c_short,
    pub __list: __pthread_list_t

}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list

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
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int

}
pub const PTHREAD_MUTEX_TIMED_NP: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_RECURSIVE: C2Rust_Unnamed = 1;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_NORMAL: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2Rust_Unnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2Rust_Unnamed = 1;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_FAST: ::core::ffi::c_int = 0;
pub const SQLITE_MUTEX_RECURSIVE: ::core::ffi::c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn sqlite3MemoryBarrier() {
    unsafe {
        
    }
}
unsafe extern "C" fn pthreadMutexInit() -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pthreadMutexEnd() -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pthreadMutexAlloc(
    mut iType: ::core::ffi::c_int,
) -> *mut sqlite3_mutex {
    unsafe {
        static mut staticMutexes: [sqlite3_mutex; 12] = [
            sqlite3_mutex {
                mutex: pthread_mutex_t {
                    __data: __pthread_mutex_s {
                        __lock: 0 as ::core::ffi::c_int,
                        __count: 0 as ::core::ffi::c_uint,
                        __owner: 0 as ::core::ffi::c_int,
                        __nusers: 0 as ::core::ffi::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                        __spins: 0 as ::core::ffi::c_short,
                        __elision: 0 as ::core::ffi::c_short,
                        __list: __pthread_internal_list {
                            __prev: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list,
                            __next: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list

                        }

                    }

                }

            },
            sqlite3_mutex {
                mutex: pthread_mutex_t {
                    __data: __pthread_mutex_s {
                        __lock: 0 as ::core::ffi::c_int,
                        __count: 0 as ::core::ffi::c_uint,
                        __owner: 0 as ::core::ffi::c_int,
                        __nusers: 0 as ::core::ffi::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                        __spins: 0 as ::core::ffi::c_short,
                        __elision: 0 as ::core::ffi::c_short,
                        __list: __pthread_internal_list {
                            __prev: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list,
                            __next: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list

                        }

                    }

                }

            },
            sqlite3_mutex {
                mutex: pthread_mutex_t {
                    __data: __pthread_mutex_s {
                        __lock: 0 as ::core::ffi::c_int,
                        __count: 0 as ::core::ffi::c_uint,
                        __owner: 0 as ::core::ffi::c_int,
                        __nusers: 0 as ::core::ffi::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                        __spins: 0 as ::core::ffi::c_short,
                        __elision: 0 as ::core::ffi::c_short,
                        __list: __pthread_internal_list {
                            __prev: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list,
                            __next: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list

                        }

                    }

                }

            },
            sqlite3_mutex {
                mutex: pthread_mutex_t {
                    __data: __pthread_mutex_s {
                        __lock: 0 as ::core::ffi::c_int,
                        __count: 0 as ::core::ffi::c_uint,
                        __owner: 0 as ::core::ffi::c_int,
                        __nusers: 0 as ::core::ffi::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                        __spins: 0 as ::core::ffi::c_short,
                        __elision: 0 as ::core::ffi::c_short,
                        __list: __pthread_internal_list {
                            __prev: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list,
                            __next: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list

                        }

                    }

                }

            },
            sqlite3_mutex {
                mutex: pthread_mutex_t {
                    __data: __pthread_mutex_s {
                        __lock: 0 as ::core::ffi::c_int,
                        __count: 0 as ::core::ffi::c_uint,
                        __owner: 0 as ::core::ffi::c_int,
                        __nusers: 0 as ::core::ffi::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                        __spins: 0 as ::core::ffi::c_short,
                        __elision: 0 as ::core::ffi::c_short,
                        __list: __pthread_internal_list {
                            __prev: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list,
                            __next: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list

                        }

                    }

                }

            },
            sqlite3_mutex {
                mutex: pthread_mutex_t {
                    __data: __pthread_mutex_s {
                        __lock: 0 as ::core::ffi::c_int,
                        __count: 0 as ::core::ffi::c_uint,
                        __owner: 0 as ::core::ffi::c_int,
                        __nusers: 0 as ::core::ffi::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                        __spins: 0 as ::core::ffi::c_short,
                        __elision: 0 as ::core::ffi::c_short,
                        __list: __pthread_internal_list {
                            __prev: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list,
                            __next: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list

                        }

                    }

                }

            },
            sqlite3_mutex {
                mutex: pthread_mutex_t {
                    __data: __pthread_mutex_s {
                        __lock: 0 as ::core::ffi::c_int,
                        __count: 0 as ::core::ffi::c_uint,
                        __owner: 0 as ::core::ffi::c_int,
                        __nusers: 0 as ::core::ffi::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                        __spins: 0 as ::core::ffi::c_short,
                        __elision: 0 as ::core::ffi::c_short,
                        __list: __pthread_internal_list {
                            __prev: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list,
                            __next: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list

                        }

                    }

                }

            },
            sqlite3_mutex {
                mutex: pthread_mutex_t {
                    __data: __pthread_mutex_s {
                        __lock: 0 as ::core::ffi::c_int,
                        __count: 0 as ::core::ffi::c_uint,
                        __owner: 0 as ::core::ffi::c_int,
                        __nusers: 0 as ::core::ffi::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                        __spins: 0 as ::core::ffi::c_short,
                        __elision: 0 as ::core::ffi::c_short,
                        __list: __pthread_internal_list {
                            __prev: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list,
                            __next: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list

                        }

                    }

                }

            },
            sqlite3_mutex {
                mutex: pthread_mutex_t {
                    __data: __pthread_mutex_s {
                        __lock: 0 as ::core::ffi::c_int,
                        __count: 0 as ::core::ffi::c_uint,
                        __owner: 0 as ::core::ffi::c_int,
                        __nusers: 0 as ::core::ffi::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                        __spins: 0 as ::core::ffi::c_short,
                        __elision: 0 as ::core::ffi::c_short,
                        __list: __pthread_internal_list {
                            __prev: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list,
                            __next: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list

                        }

                    }

                }

            },
            sqlite3_mutex {
                mutex: pthread_mutex_t {
                    __data: __pthread_mutex_s {
                        __lock: 0 as ::core::ffi::c_int,
                        __count: 0 as ::core::ffi::c_uint,
                        __owner: 0 as ::core::ffi::c_int,
                        __nusers: 0 as ::core::ffi::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                        __spins: 0 as ::core::ffi::c_short,
                        __elision: 0 as ::core::ffi::c_short,
                        __list: __pthread_internal_list {
                            __prev: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list,
                            __next: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list

                        }

                    }

                }

            },
            sqlite3_mutex {
                mutex: pthread_mutex_t {
                    __data: __pthread_mutex_s {
                        __lock: 0 as ::core::ffi::c_int,
                        __count: 0 as ::core::ffi::c_uint,
                        __owner: 0 as ::core::ffi::c_int,
                        __nusers: 0 as ::core::ffi::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                        __spins: 0 as ::core::ffi::c_short,
                        __elision: 0 as ::core::ffi::c_short,
                        __list: __pthread_internal_list {
                            __prev: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list,
                            __next: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list

                        }

                    }

                }

            },
            sqlite3_mutex {
                mutex: pthread_mutex_t {
                    __data: __pthread_mutex_s {
                        __lock: 0 as ::core::ffi::c_int,
                        __count: 0 as ::core::ffi::c_uint,
                        __owner: 0 as ::core::ffi::c_int,
                        __nusers: 0 as ::core::ffi::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
                        __spins: 0 as ::core::ffi::c_short,
                        __elision: 0 as ::core::ffi::c_short,
                        __list: __pthread_internal_list {
                            __prev: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list,
                            __next: ::core::ptr::null::<__pthread_internal_list>()
                                as *mut __pthread_internal_list

                        }

                    }

                }

            },
        ];
        let mut p: *mut sqlite3_mutex = ::core::ptr::null_mut::<sqlite3_mutex>();
        match iType {
            SQLITE_MUTEX_RECURSIVE => {
                p = sqlite3MallocZero(::core::mem::size_of::<sqlite3_mutex>() as u64_0)
                    as *mut sqlite3_mutex;
                if !p.is_null() {
                    let mut recursiveAttr: pthread_mutexattr_t = pthread_mutexattr_t {
                        __size: [0; 4]

                    };
                    pthread_mutexattr_init(&raw mut recursiveAttr);
                    pthread_mutexattr_settype(
                        &raw mut recursiveAttr,
                        PTHREAD_MUTEX_RECURSIVE as ::core::ffi::c_int,
                    );
                    pthread_mutex_init(&raw mut (*p).mutex, &raw mut recursiveAttr);
                    pthread_mutexattr_destroy(&raw mut recursiveAttr);
                }
            }
            SQLITE_MUTEX_FAST => {
                p = sqlite3MallocZero(::core::mem::size_of::<sqlite3_mutex>() as u64_0)
                    as *mut sqlite3_mutex;
                if !p.is_null() {
                    pthread_mutex_init(
                        &raw mut (*p).mutex,
                        ::core::ptr::null::<pthread_mutexattr_t>(),
                    );
                }
            }
            _ => {
                p = (&raw mut staticMutexes as *mut sqlite3_mutex)
                    .offset((iType - 2 as ::core::ffi::c_int) as isize)
                    as *mut sqlite3_mutex;
            }
        }
        return p;
    }
}
unsafe extern "C" fn pthreadMutexFree(mut p: *mut sqlite3_mutex) {
    unsafe {
        pthread_mutex_destroy(&raw mut (*p).mutex);
        sqlite3_free(p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn pthreadMutexEnter(mut p: *mut sqlite3_mutex) {
    unsafe {
        pthread_mutex_lock(&raw mut (*p).mutex);
    }
}
unsafe extern "C" fn pthreadMutexTry(mut p: *mut sqlite3_mutex) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if pthread_mutex_trylock(&raw mut (*p).mutex) == 0 as ::core::ffi::c_int {
            rc = SQLITE_OK;
        } else {
            rc = SQLITE_BUSY;
        }
        return rc;
    }
}
unsafe extern "C" fn pthreadMutexLeave(mut p: *mut sqlite3_mutex) {
    unsafe {
        pthread_mutex_unlock(&raw mut (*p).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DefaultMutex() -> *const sqlite3_mutex_methods {
    unsafe {
        static mut sMutex: sqlite3_mutex_methods = unsafe {
            sqlite3_mutex_methods {
                xMutexInit: Some(
                    pthreadMutexInit as unsafe extern "C" fn() -> ::core::ffi::c_int,
                ),
                xMutexEnd: Some(
                    pthreadMutexEnd as unsafe extern "C" fn() -> ::core::ffi::c_int,
                ),
                xMutexAlloc: Some(
                    pthreadMutexAlloc
                        as unsafe extern "C" fn(::core::ffi::c_int) -> *mut sqlite3_mutex,
                ),
                xMutexFree: Some(
                    pthreadMutexFree as unsafe extern "C" fn(*mut sqlite3_mutex) -> (),
                ),
                xMutexEnter: Some(
                    pthreadMutexEnter as unsafe extern "C" fn(*mut sqlite3_mutex) -> (),
                ),
                xMutexTry: Some(
                    pthreadMutexTry
                        as unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
                ),
                xMutexLeave: Some(
                    pthreadMutexLeave as unsafe extern "C" fn(*mut sqlite3_mutex) -> (),
                ),
                xMutexHeld: None,
                xMutexNotheld: None

            }
        };
        return &raw const sMutex;
    }
}
