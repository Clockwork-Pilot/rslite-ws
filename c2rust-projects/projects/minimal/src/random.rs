extern "C" {
    pub type sqlite3_mutex;
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
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
    fn sqlite3OsRandomness(
        _: *mut sqlite3_vfs,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3MutexAlloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type u8_0 = ::core::ffi::c_uchar;
pub type u32_0 = ::core::ffi::c_uint;
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
pub struct sqlite3PrngType {
    pub s: [u32_0; 16],
    pub out: [u8_0; 64],
    pub n: u8_0

}
pub type size_t = usize;
pub const SQLITE_MUTEX_STATIC_PRNG: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
static mut sqlite3Prng: sqlite3PrngType = sqlite3PrngType {
    s: [0; 16],
    out: [0; 64],
    n: 0

};
unsafe extern "C" fn chacha_block(mut out: *mut u32_0, mut in_0: *const u32_0) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut x: [u32_0; 16] = [0; 16];
        memcpy(
            &raw mut x as *mut u32_0 as *mut ::core::ffi::c_void,
            in_0 as *const ::core::ffi::c_void,
            64 as size_t,
        );
        i = 0 as ::core::ffi::c_int;
        while i < 10 as ::core::ffi::c_int {
            x[0 as ::core::ffi::c_int as usize] = (x[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[12 as ::core::ffi::c_int as usize]
                ^= x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[12 as ::core::ffi::c_int as usize] = x[12 as ::core::ffi::c_int as usize]
                << 16 as ::core::ffi::c_int
                | x[12 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
            x[8 as ::core::ffi::c_int as usize] = (x[8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[12 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[4 as ::core::ffi::c_int as usize]
                ^= x[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[4 as ::core::ffi::c_int as usize] = x[4 as ::core::ffi::c_int as usize]
                << 12 as ::core::ffi::c_int
                | x[4 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
            x[0 as ::core::ffi::c_int as usize] = (x[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[12 as ::core::ffi::c_int as usize]
                ^= x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[12 as ::core::ffi::c_int as usize] = x[12 as ::core::ffi::c_int as usize]
                << 8 as ::core::ffi::c_int
                | x[12 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
            x[8 as ::core::ffi::c_int as usize] = (x[8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[12 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[4 as ::core::ffi::c_int as usize]
                ^= x[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[4 as ::core::ffi::c_int as usize] = x[4 as ::core::ffi::c_int as usize]
                << 7 as ::core::ffi::c_int
                | x[4 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            x[1 as ::core::ffi::c_int as usize] = (x[1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[13 as ::core::ffi::c_int as usize]
                ^= x[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[13 as ::core::ffi::c_int as usize] = x[13 as ::core::ffi::c_int as usize]
                << 16 as ::core::ffi::c_int
                | x[13 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
            x[9 as ::core::ffi::c_int as usize] = (x[9 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[13 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[5 as ::core::ffi::c_int as usize]
                ^= x[9 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[5 as ::core::ffi::c_int as usize] = x[5 as ::core::ffi::c_int as usize]
                << 12 as ::core::ffi::c_int
                | x[5 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
            x[1 as ::core::ffi::c_int as usize] = (x[1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[13 as ::core::ffi::c_int as usize]
                ^= x[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[13 as ::core::ffi::c_int as usize] = x[13 as ::core::ffi::c_int as usize]
                << 8 as ::core::ffi::c_int
                | x[13 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
            x[9 as ::core::ffi::c_int as usize] = (x[9 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[13 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[5 as ::core::ffi::c_int as usize]
                ^= x[9 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[5 as ::core::ffi::c_int as usize] = x[5 as ::core::ffi::c_int as usize]
                << 7 as ::core::ffi::c_int
                | x[5 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            x[2 as ::core::ffi::c_int as usize] = (x[2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[6 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[14 as ::core::ffi::c_int as usize]
                ^= x[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[14 as ::core::ffi::c_int as usize] = x[14 as ::core::ffi::c_int as usize]
                << 16 as ::core::ffi::c_int
                | x[14 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
            x[10 as ::core::ffi::c_int as usize] = (x[10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[14 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[6 as ::core::ffi::c_int as usize]
                ^= x[10 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[6 as ::core::ffi::c_int as usize] = x[6 as ::core::ffi::c_int as usize]
                << 12 as ::core::ffi::c_int
                | x[6 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
            x[2 as ::core::ffi::c_int as usize] = (x[2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[6 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[14 as ::core::ffi::c_int as usize]
                ^= x[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[14 as ::core::ffi::c_int as usize] = x[14 as ::core::ffi::c_int as usize]
                << 8 as ::core::ffi::c_int
                | x[14 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
            x[10 as ::core::ffi::c_int as usize] = (x[10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[14 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[6 as ::core::ffi::c_int as usize]
                ^= x[10 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[6 as ::core::ffi::c_int as usize] = x[6 as ::core::ffi::c_int as usize]
                << 7 as ::core::ffi::c_int
                | x[6 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            x[3 as ::core::ffi::c_int as usize] = (x[3 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[15 as ::core::ffi::c_int as usize]
                ^= x[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[15 as ::core::ffi::c_int as usize] = x[15 as ::core::ffi::c_int as usize]
                << 16 as ::core::ffi::c_int
                | x[15 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
            x[11 as ::core::ffi::c_int as usize] = (x[11 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[15 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[7 as ::core::ffi::c_int as usize]
                ^= x[11 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[7 as ::core::ffi::c_int as usize] = x[7 as ::core::ffi::c_int as usize]
                << 12 as ::core::ffi::c_int
                | x[7 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
            x[3 as ::core::ffi::c_int as usize] = (x[3 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[15 as ::core::ffi::c_int as usize]
                ^= x[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[15 as ::core::ffi::c_int as usize] = x[15 as ::core::ffi::c_int as usize]
                << 8 as ::core::ffi::c_int
                | x[15 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
            x[11 as ::core::ffi::c_int as usize] = (x[11 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[15 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[7 as ::core::ffi::c_int as usize]
                ^= x[11 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[7 as ::core::ffi::c_int as usize] = x[7 as ::core::ffi::c_int as usize]
                << 7 as ::core::ffi::c_int
                | x[7 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            x[0 as ::core::ffi::c_int as usize] = (x[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[15 as ::core::ffi::c_int as usize]
                ^= x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[15 as ::core::ffi::c_int as usize] = x[15 as ::core::ffi::c_int as usize]
                << 16 as ::core::ffi::c_int
                | x[15 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
            x[10 as ::core::ffi::c_int as usize] = (x[10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[15 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[5 as ::core::ffi::c_int as usize]
                ^= x[10 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[5 as ::core::ffi::c_int as usize] = x[5 as ::core::ffi::c_int as usize]
                << 12 as ::core::ffi::c_int
                | x[5 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
            x[0 as ::core::ffi::c_int as usize] = (x[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[15 as ::core::ffi::c_int as usize]
                ^= x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[15 as ::core::ffi::c_int as usize] = x[15 as ::core::ffi::c_int as usize]
                << 8 as ::core::ffi::c_int
                | x[15 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
            x[10 as ::core::ffi::c_int as usize] = (x[10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[15 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[5 as ::core::ffi::c_int as usize]
                ^= x[10 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[5 as ::core::ffi::c_int as usize] = x[5 as ::core::ffi::c_int as usize]
                << 7 as ::core::ffi::c_int
                | x[5 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            x[1 as ::core::ffi::c_int as usize] = (x[1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[6 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[12 as ::core::ffi::c_int as usize]
                ^= x[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[12 as ::core::ffi::c_int as usize] = x[12 as ::core::ffi::c_int as usize]
                << 16 as ::core::ffi::c_int
                | x[12 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
            x[11 as ::core::ffi::c_int as usize] = (x[11 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[12 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[6 as ::core::ffi::c_int as usize]
                ^= x[11 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[6 as ::core::ffi::c_int as usize] = x[6 as ::core::ffi::c_int as usize]
                << 12 as ::core::ffi::c_int
                | x[6 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
            x[1 as ::core::ffi::c_int as usize] = (x[1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[6 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[12 as ::core::ffi::c_int as usize]
                ^= x[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[12 as ::core::ffi::c_int as usize] = x[12 as ::core::ffi::c_int as usize]
                << 8 as ::core::ffi::c_int
                | x[12 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
            x[11 as ::core::ffi::c_int as usize] = (x[11 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[12 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[6 as ::core::ffi::c_int as usize]
                ^= x[11 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[6 as ::core::ffi::c_int as usize] = x[6 as ::core::ffi::c_int as usize]
                << 7 as ::core::ffi::c_int
                | x[6 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            x[2 as ::core::ffi::c_int as usize] = (x[2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[13 as ::core::ffi::c_int as usize]
                ^= x[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[13 as ::core::ffi::c_int as usize] = x[13 as ::core::ffi::c_int as usize]
                << 16 as ::core::ffi::c_int
                | x[13 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
            x[8 as ::core::ffi::c_int as usize] = (x[8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[13 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[7 as ::core::ffi::c_int as usize]
                ^= x[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[7 as ::core::ffi::c_int as usize] = x[7 as ::core::ffi::c_int as usize]
                << 12 as ::core::ffi::c_int
                | x[7 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
            x[2 as ::core::ffi::c_int as usize] = (x[2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[13 as ::core::ffi::c_int as usize]
                ^= x[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[13 as ::core::ffi::c_int as usize] = x[13 as ::core::ffi::c_int as usize]
                << 8 as ::core::ffi::c_int
                | x[13 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
            x[8 as ::core::ffi::c_int as usize] = (x[8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[13 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[7 as ::core::ffi::c_int as usize]
                ^= x[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[7 as ::core::ffi::c_int as usize] = x[7 as ::core::ffi::c_int as usize]
                << 7 as ::core::ffi::c_int
                | x[7 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            x[3 as ::core::ffi::c_int as usize] = (x[3 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[14 as ::core::ffi::c_int as usize]
                ^= x[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[14 as ::core::ffi::c_int as usize] = x[14 as ::core::ffi::c_int as usize]
                << 16 as ::core::ffi::c_int
                | x[14 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
            x[9 as ::core::ffi::c_int as usize] = (x[9 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[14 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[4 as ::core::ffi::c_int as usize]
                ^= x[9 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[4 as ::core::ffi::c_int as usize] = x[4 as ::core::ffi::c_int as usize]
                << 12 as ::core::ffi::c_int
                | x[4 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
            x[3 as ::core::ffi::c_int as usize] = (x[3 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(x[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                as u32_0 as u32_0;
            x[14 as ::core::ffi::c_int as usize]
                ^= x[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[14 as ::core::ffi::c_int as usize] = x[14 as ::core::ffi::c_int as usize]
                << 8 as ::core::ffi::c_int
                | x[14 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
            x[9 as ::core::ffi::c_int as usize] = (x[9 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uint)
                .wrapping_add(
                    x[14 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
                ) as u32_0 as u32_0;
            x[4 as ::core::ffi::c_int as usize]
                ^= x[9 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint;
            x[4 as ::core::ffi::c_int as usize] = x[4 as ::core::ffi::c_int as usize]
                << 7 as ::core::ffi::c_int
                | x[4 as ::core::ffi::c_int as usize]
                    >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            *out.offset(i as isize) = x[i as usize]
                .wrapping_add(*in_0.offset(i as isize));
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_randomness(
    mut N: ::core::ffi::c_int,
    mut pBuf: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut zBuf: *mut ::core::ffi::c_uchar = pBuf as *mut ::core::ffi::c_uchar;
        let mut mutex: *mut sqlite3_mutex = ::core::ptr::null_mut::<sqlite3_mutex>();
        mutex = sqlite3MutexAlloc(SQLITE_MUTEX_STATIC_PRNG);
        sqlite3_mutex_enter(mutex);
        if N <= 0 as ::core::ffi::c_int || pBuf.is_null() {
            sqlite3Prng.s[0 as ::core::ffi::c_int as usize] = 0 as u32_0;
            sqlite3_mutex_leave(mutex);
            return;
        }
        if sqlite3Prng.s[0 as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_uint {
            let mut pVfs: *mut sqlite3_vfs = sqlite3_vfs_find(
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            static mut chacha20_init: [u32_0; 4] = [
                0x61707865 as ::core::ffi::c_int as u32_0,
                0x3320646e as ::core::ffi::c_int as u32_0,
                0x79622d32 as ::core::ffi::c_int as u32_0,
                0x6b206574 as ::core::ffi::c_int as u32_0,
            ];
            memcpy(
                (&raw mut sqlite3Prng.s as *mut u32_0)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut u32_0
                    as *mut ::core::ffi::c_void,
                &raw const chacha20_init as *const u32_0 as *const ::core::ffi::c_void,
                16 as size_t,
            );
            if pVfs.is_null() {
                memset(
                    (&raw mut sqlite3Prng.s as *mut u32_0)
                        .offset(4 as ::core::ffi::c_int as isize) as *mut u32_0
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    44 as size_t,
                );
            } else {
                sqlite3OsRandomness(
                    pVfs,
                    44 as ::core::ffi::c_int,
                    (&raw mut sqlite3Prng.s as *mut u32_0)
                        .offset(4 as ::core::ffi::c_int as isize) as *mut u32_0
                        as *mut ::core::ffi::c_char,
                );
            }
            sqlite3Prng.s[15 as ::core::ffi::c_int as usize] = sqlite3Prng
                .s[12 as ::core::ffi::c_int as usize];
            sqlite3Prng.s[12 as ::core::ffi::c_int as usize] = 0 as u32_0;
            sqlite3Prng.n = 0 as u8_0;
        }
        loop {
            if N <= sqlite3Prng.n as ::core::ffi::c_int {
                memcpy(
                    zBuf as *mut ::core::ffi::c_void,
                    (&raw mut sqlite3Prng.out as *mut u8_0)
                        .offset((sqlite3Prng.n as ::core::ffi::c_int - N) as isize)
                        as *mut u8_0 as *const ::core::ffi::c_void,
                    N as size_t,
                );
                sqlite3Prng.n = (sqlite3Prng.n as ::core::ffi::c_int - N) as u8_0;
                break;
            } else {
                if sqlite3Prng.n as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    memcpy(
                        zBuf as *mut ::core::ffi::c_void,
                        &raw mut sqlite3Prng.out as *mut u8_0
                            as *const ::core::ffi::c_void,
                        sqlite3Prng.n as size_t,
                    );
                    N -= sqlite3Prng.n as ::core::ffi::c_int;
                    zBuf = zBuf.offset(sqlite3Prng.n as ::core::ffi::c_int as isize);
                }
                sqlite3Prng.s[12 as ::core::ffi::c_int as usize] = sqlite3Prng
                    .s[12 as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
                chacha_block(
                    &raw mut sqlite3Prng.out as *mut u8_0 as *mut u32_0,
                    &raw mut sqlite3Prng.s as *mut u32_0,
                );
                sqlite3Prng.n = 64 as u8_0;
            }
        }
        sqlite3_mutex_leave(mutex);
    }
}
static mut sqlite3SavedPrng: sqlite3PrngType = sqlite3PrngType {
    s: [0; 16],
    out: [0; 64],
    n: 0

};
#[no_mangle]
pub unsafe extern "C" fn sqlite3PrngSaveState() {
    unsafe {
        memcpy(
            &raw mut sqlite3SavedPrng as *mut ::core::ffi::c_void,
            &raw mut sqlite3Prng as *const ::core::ffi::c_void,
            ::core::mem::size_of::<sqlite3PrngType>() as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PrngRestoreState() {
    unsafe {
        memcpy(
            &raw mut sqlite3Prng as *mut ::core::ffi::c_void,
            &raw mut sqlite3SavedPrng as *const ::core::ffi::c_void,
            ::core::mem::size_of::<sqlite3PrngType>() as size_t,
        );
    }
}
