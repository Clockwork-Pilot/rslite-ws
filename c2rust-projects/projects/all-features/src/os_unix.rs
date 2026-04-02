extern "C" {
    pub type sqlite3_mutex;
    pub type sqlite3_pcache;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_uri_parameter(
        z: sqlite3_filename,
        zParam: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3_uri_boolean(
        z: sqlite3_filename,
        zParam: *const ::core::ffi::c_char,
        bDefault: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut sqlite3_temp_directory: *mut ::core::ffi::c_char;
    fn sqlite3_vfs_register(
        _: *mut sqlite3_vfs,
        makeDflt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_mutex_alloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    fn sqlite3_mutex_free(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn sqlite3_log(
        iErrCode: ::core::ffi::c_int,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
    static mut sqlite3OSTrace: ::core::ffi::c_int;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fchmod(__fd: ::core::ffi::c_int, __mode: __mode_t) -> ::core::ffi::c_int;
    fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn sqlite3CantopenError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3MutexAlloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    fn sqlite3MemoryBarrier();
    fn sqlite3DebugPrintf(_: *const ::core::ffi::c_char, ...);
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3PendingByte: ::core::ffi::c_int;
    fn fcntl(
        __fd: ::core::ffi::c_int,
        __cmd: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    fn pread64(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
        __offset: __off64_t,
    ) -> ssize_t;
    fn pwrite64(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
        __offset: __off64_t,
    ) -> ssize_t;
    fn fchown(
        __fd: ::core::ffi::c_int,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::core::ffi::c_int;
    fn getcwd(
        __buf: *mut ::core::ffi::c_char,
        __size: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn getpid() -> __pid_t;
    fn geteuid() -> __uid_t;
    fn readlink(
        __path: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn rmdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn ftruncate(__fd: ::core::ffi::c_int, __length: __off64_t) -> ::core::ffi::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
    ) -> *mut ::core::ffi::c_void;
    fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
    fn mremap(
        __addr: *mut ::core::ffi::c_void,
        __old_len: size_t,
        __new_len: size_t,
        __flags: ::core::ffi::c_int,
        ...
    ) -> *mut ::core::ffi::c_void;
    fn utime(
        __file: *const ::core::ffi::c_char,
        __file_times: *const utimbuf,
    ) -> ::core::ffi::c_int;
    static mut sqlite3_io_error_hit: ::core::ffi::c_int;
    static mut sqlite3_io_error_hardhit: ::core::ffi::c_int;
    static mut sqlite3_io_error_pending: ::core::ffi::c_int;
    static mut sqlite3_io_error_persist: ::core::ffi::c_int;
    static mut sqlite3_io_error_benign: ::core::ffi::c_int;
    static mut sqlite3_diskfull_pending: ::core::ffi::c_int;
    static mut sqlite3_diskfull: ::core::ffi::c_int;
    static mut sqlite3_open_file_count: ::core::ffi::c_int;
    fn dlopen(
        __file: *const ::core::ffi::c_char,
        __mode: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn dlclose(__handle: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn dlsym(
        __handle: *mut ::core::ffi::c_void,
        __name: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn dlerror() -> *mut ::core::ffi::c_char;
}
pub type u64_0 = sqlite_uint64;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type i64_0 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type u16_0 = uint16_t;
pub type uint16_t = __uint16_t;
pub type __uint16_t = u16;
pub type u8_0 = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
pub type u32_0 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
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
pub struct unix_syscall {
    pub zName: *const ::core::ffi::c_char,
    pub pCurrent: sqlite3_syscall_ptr,
    pub pDefault: sqlite3_syscall_ptr

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3]

}
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t

}
pub type __time_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __off_t = ::core::ffi::c_long;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __gid_t = ::core::ffi::c_uint;
pub type __uid_t = ::core::ffi::c_uint;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __ino_t = ::core::ffi::c_ulong;
pub type size_t = usize;
pub type ssize_t = __ssize_t;
pub type __ssize_t = ::core::ffi::c_long;
pub const _SC_PAGESIZE: C2Rust_Unnamed = 30;
pub type __off64_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
pub type __suseconds_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t

}
pub type pid_t = __pid_t;
pub type __pid_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbPath {
    pub rc: ::core::ffi::c_int,
    pub nSymlink: ::core::ffi::c_int,
    pub zOut: *mut ::core::ffi::c_char,
    pub nOut: ::core::ffi::c_int,
    pub nUsed: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unixFile {
    pub pMethod: *const sqlite3_io_methods,
    pub pVfs: *mut sqlite3_vfs,
    pub pInode: *mut unixInodeInfo,
    pub h: ::core::ffi::c_int,
    pub eFileLock: ::core::ffi::c_uchar,
    pub ctrlFlags: ::core::ffi::c_ushort,
    pub lastErrno: ::core::ffi::c_int,
    pub lockingContext: *mut ::core::ffi::c_void,
    pub pPreallocatedUnused: *mut UnixUnusedFd,
    pub zPath: *const ::core::ffi::c_char,
    pub pShm: *mut unixShm,
    pub szChunk: ::core::ffi::c_int,
    pub nFetchOut: ::core::ffi::c_int,
    pub mmapSize: sqlite3_int64,
    pub mmapSizeActual: sqlite3_int64,
    pub mmapSizeMax: sqlite3_int64,
    pub pMapRegion: *mut ::core::ffi::c_void,
    pub sectorSize: ::core::ffi::c_int,
    pub deviceCharacteristics: ::core::ffi::c_int,
    pub aPadding: [::core::ffi::c_char; 32]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unixShm {
    pub pShmNode: *mut unixShmNode,
    pub pNext: *mut unixShm,
    pub hasMutex: u8_0,
    pub id: u8_0,
    pub sharedMask: u16_0,
    pub exclMask: u16_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unixShmNode {
    pub pInode: *mut unixInodeInfo,
    pub pShmMutex: *mut sqlite3_mutex,
    pub zFilename: *mut ::core::ffi::c_char,
    pub hShm: ::core::ffi::c_int,
    pub szRegion: ::core::ffi::c_int,
    pub nRegion: u16_0,
    pub isReadonly: u8_0,
    pub isUnlocked: u8_0,
    pub apRegion: *mut *mut ::core::ffi::c_char,
    pub nRef: ::core::ffi::c_int,
    pub pFirst: *mut unixShm,
    pub aLock: [::core::ffi::c_int; 8]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unixInodeInfo {
    pub fileId: unixFileId,
    pub pLockMutex: *mut sqlite3_mutex,
    pub nShared: ::core::ffi::c_int,
    pub nLock: ::core::ffi::c_int,
    pub eFileLock: ::core::ffi::c_uchar,
    pub bProcessLock: ::core::ffi::c_uchar,
    pub pUnused: *mut UnixUnusedFd,
    pub nRef: ::core::ffi::c_int,
    pub pShmNode: *mut unixShmNode,
    pub pNext: *mut unixInodeInfo,
    pub pPrev: *mut unixInodeInfo

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnixUnusedFd {
    pub fd: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub pNext: *mut UnixUnusedFd

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unixFileId {
    pub dev: dev_t,
    pub ino: u64_0

}
pub type dev_t = __dev_t;
pub type off_t = __off64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: ::core::ffi::c_short,
    pub l_whence: ::core::ffi::c_short,
    pub l_start: __off64_t,
    pub l_len: __off64_t,
    pub l_pid: __pid_t

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
pub type off64_t = __off64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t

}
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type finder_type = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        *mut unixFile,
    ) -> *const sqlite3_io_methods,
>;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const _SC_SIGSTKSZ: C2Rust_Unnamed = 250;
pub const _SC_MINSIGSTKSZ: C2Rust_Unnamed = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2Rust_Unnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2Rust_Unnamed = 247;
pub const _SC_XOPEN_STREAMS: C2Rust_Unnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2Rust_Unnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2Rust_Unnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2Rust_Unnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2Rust_Unnamed = 242;
pub const _SC_SS_REPL_MAX: C2Rust_Unnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2Rust_Unnamed = 240;
pub const _SC_V7_LP64_OFF64: C2Rust_Unnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2Rust_Unnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2Rust_Unnamed = 237;
pub const _SC_RAW_SOCKETS: C2Rust_Unnamed = 236;
pub const _SC_IPV6: C2Rust_Unnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2Rust_Unnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2Rust_Unnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2Rust_Unnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2Rust_Unnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2Rust_Unnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2Rust_Unnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2Rust_Unnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2Rust_Unnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2Rust_Unnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2Rust_Unnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2Rust_Unnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2Rust_Unnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2Rust_Unnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2Rust_Unnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2Rust_Unnamed = 185;
pub const _SC_TRACE_LOG: C2Rust_Unnamed = 184;
pub const _SC_TRACE_INHERIT: C2Rust_Unnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2Rust_Unnamed = 182;
pub const _SC_TRACE: C2Rust_Unnamed = 181;
pub const _SC_HOST_NAME_MAX: C2Rust_Unnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2Rust_Unnamed = 179;
pub const _SC_V6_LP64_OFF64: C2Rust_Unnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2Rust_Unnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2Rust_Unnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2Rust_Unnamed = 175;
pub const _SC_STREAMS: C2Rust_Unnamed = 174;
pub const _SC_SYMLOOP_MAX: C2Rust_Unnamed = 173;
pub const _SC_2_PBS_TRACK: C2Rust_Unnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2Rust_Unnamed = 171;
pub const _SC_2_PBS_LOCATE: C2Rust_Unnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2Rust_Unnamed = 169;
pub const _SC_2_PBS: C2Rust_Unnamed = 168;
pub const _SC_USER_GROUPS_R: C2Rust_Unnamed = 167;
pub const _SC_USER_GROUPS: C2Rust_Unnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2Rust_Unnamed = 165;
pub const _SC_TIMEOUTS: C2Rust_Unnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2Rust_Unnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2Rust_Unnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2Rust_Unnamed = 161;
pub const _SC_SPORADIC_SERVER: C2Rust_Unnamed = 160;
pub const _SC_SPAWN: C2Rust_Unnamed = 159;
pub const _SC_SIGNALS: C2Rust_Unnamed = 158;
pub const _SC_SHELL: C2Rust_Unnamed = 157;
pub const _SC_REGEX_VERSION: C2Rust_Unnamed = 156;
pub const _SC_REGEXP: C2Rust_Unnamed = 155;
pub const _SC_SPIN_LOCKS: C2Rust_Unnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2Rust_Unnamed = 153;
pub const _SC_NETWORKING: C2Rust_Unnamed = 152;
pub const _SC_SINGLE_PROCESS: C2Rust_Unnamed = 151;
pub const _SC_MULTI_PROCESS: C2Rust_Unnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2Rust_Unnamed = 149;
pub const _SC_FILE_SYSTEM: C2Rust_Unnamed = 148;
pub const _SC_FILE_LOCKING: C2Rust_Unnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2Rust_Unnamed = 146;
pub const _SC_PIPE: C2Rust_Unnamed = 145;
pub const _SC_FIFO: C2Rust_Unnamed = 144;
pub const _SC_FD_MGMT: C2Rust_Unnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2Rust_Unnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2Rust_Unnamed = 141;
pub const _SC_DEVICE_IO: C2Rust_Unnamed = 140;
pub const _SC_THREAD_CPUTIME: C2Rust_Unnamed = 139;
pub const _SC_CPUTIME: C2Rust_Unnamed = 138;
pub const _SC_CLOCK_SELECTION: C2Rust_Unnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2Rust_Unnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2Rust_Unnamed = 135;
pub const _SC_BASE: C2Rust_Unnamed = 134;
pub const _SC_BARRIERS: C2Rust_Unnamed = 133;
pub const _SC_ADVISORY_INFO: C2Rust_Unnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2Rust_Unnamed = 131;
pub const _SC_XOPEN_REALTIME: C2Rust_Unnamed = 130;
pub const _SC_XOPEN_LEGACY: C2Rust_Unnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2Rust_Unnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2Rust_Unnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2Rust_Unnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2Rust_Unnamed = 125;
pub const _SC_NL_TEXTMAX: C2Rust_Unnamed = 124;
pub const _SC_NL_SETMAX: C2Rust_Unnamed = 123;
pub const _SC_NL_NMAX: C2Rust_Unnamed = 122;
pub const _SC_NL_MSGMAX: C2Rust_Unnamed = 121;
pub const _SC_NL_LANGMAX: C2Rust_Unnamed = 120;
pub const _SC_NL_ARGMAX: C2Rust_Unnamed = 119;
pub const _SC_USHRT_MAX: C2Rust_Unnamed = 118;
pub const _SC_ULONG_MAX: C2Rust_Unnamed = 117;
pub const _SC_UINT_MAX: C2Rust_Unnamed = 116;
pub const _SC_UCHAR_MAX: C2Rust_Unnamed = 115;
pub const _SC_SHRT_MIN: C2Rust_Unnamed = 114;
pub const _SC_SHRT_MAX: C2Rust_Unnamed = 113;
pub const _SC_SCHAR_MIN: C2Rust_Unnamed = 112;
pub const _SC_SCHAR_MAX: C2Rust_Unnamed = 111;
pub const _SC_SSIZE_MAX: C2Rust_Unnamed = 110;
pub const _SC_NZERO: C2Rust_Unnamed = 109;
pub const _SC_MB_LEN_MAX: C2Rust_Unnamed = 108;
pub const _SC_WORD_BIT: C2Rust_Unnamed = 107;
pub const _SC_LONG_BIT: C2Rust_Unnamed = 106;
pub const _SC_INT_MIN: C2Rust_Unnamed = 105;
pub const _SC_INT_MAX: C2Rust_Unnamed = 104;
pub const _SC_CHAR_MIN: C2Rust_Unnamed = 103;
pub const _SC_CHAR_MAX: C2Rust_Unnamed = 102;
pub const _SC_CHAR_BIT: C2Rust_Unnamed = 101;
pub const _SC_XOPEN_XPG4: C2Rust_Unnamed = 100;
pub const _SC_XOPEN_XPG3: C2Rust_Unnamed = 99;
pub const _SC_XOPEN_XPG2: C2Rust_Unnamed = 98;
pub const _SC_2_UPE: C2Rust_Unnamed = 97;
pub const _SC_2_C_VERSION: C2Rust_Unnamed = 96;
pub const _SC_2_CHAR_TERM: C2Rust_Unnamed = 95;
pub const _SC_XOPEN_SHM: C2Rust_Unnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2Rust_Unnamed = 93;
pub const _SC_XOPEN_CRYPT: C2Rust_Unnamed = 92;
pub const _SC_XOPEN_UNIX: C2Rust_Unnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2Rust_Unnamed = 90;
pub const _SC_XOPEN_VERSION: C2Rust_Unnamed = 89;
pub const _SC_PASS_MAX: C2Rust_Unnamed = 88;
pub const _SC_ATEXIT_MAX: C2Rust_Unnamed = 87;
pub const _SC_AVPHYS_PAGES: C2Rust_Unnamed = 86;
pub const _SC_PHYS_PAGES: C2Rust_Unnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2Rust_Unnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2Rust_Unnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2Rust_Unnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2Rust_Unnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2Rust_Unnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2Rust_Unnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2Rust_Unnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2Rust_Unnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2Rust_Unnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2Rust_Unnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2Rust_Unnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2Rust_Unnamed = 73;
pub const _SC_TTY_NAME_MAX: C2Rust_Unnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2Rust_Unnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2Rust_Unnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2Rust_Unnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2Rust_Unnamed = 68;
pub const _SC_THREADS: C2Rust_Unnamed = 67;
pub const _SC_T_IOV_MAX: C2Rust_Unnamed = 66;
pub const _SC_PII_OSI_M: C2Rust_Unnamed = 65;
pub const _SC_PII_OSI_CLTS: C2Rust_Unnamed = 64;
pub const _SC_PII_OSI_COTS: C2Rust_Unnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2Rust_Unnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2Rust_Unnamed = 61;
pub const _SC_IOV_MAX: C2Rust_Unnamed = 60;
pub const _SC_UIO_MAXIOV: C2Rust_Unnamed = 60;
pub const _SC_SELECT: C2Rust_Unnamed = 59;
pub const _SC_POLL: C2Rust_Unnamed = 58;
pub const _SC_PII_OSI: C2Rust_Unnamed = 57;
pub const _SC_PII_INTERNET: C2Rust_Unnamed = 56;
pub const _SC_PII_SOCKET: C2Rust_Unnamed = 55;
pub const _SC_PII_XTI: C2Rust_Unnamed = 54;
pub const _SC_PII: C2Rust_Unnamed = 53;
pub const _SC_2_LOCALEDEF: C2Rust_Unnamed = 52;
pub const _SC_2_SW_DEV: C2Rust_Unnamed = 51;
pub const _SC_2_FORT_RUN: C2Rust_Unnamed = 50;
pub const _SC_2_FORT_DEV: C2Rust_Unnamed = 49;
pub const _SC_2_C_DEV: C2Rust_Unnamed = 48;
pub const _SC_2_C_BIND: C2Rust_Unnamed = 47;
pub const _SC_2_VERSION: C2Rust_Unnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2Rust_Unnamed = 45;
pub const _SC_RE_DUP_MAX: C2Rust_Unnamed = 44;
pub const _SC_LINE_MAX: C2Rust_Unnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2Rust_Unnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2Rust_Unnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2Rust_Unnamed = 40;
pub const _SC_BC_STRING_MAX: C2Rust_Unnamed = 39;
pub const _SC_BC_SCALE_MAX: C2Rust_Unnamed = 38;
pub const _SC_BC_DIM_MAX: C2Rust_Unnamed = 37;
pub const _SC_BC_BASE_MAX: C2Rust_Unnamed = 36;
pub const _SC_TIMER_MAX: C2Rust_Unnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2Rust_Unnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2Rust_Unnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2Rust_Unnamed = 32;
pub const _SC_RTSIG_MAX: C2Rust_Unnamed = 31;
pub const _SC_VERSION: C2Rust_Unnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2Rust_Unnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2Rust_Unnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2Rust_Unnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2Rust_Unnamed = 25;
pub const _SC_AIO_MAX: C2Rust_Unnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2Rust_Unnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2Rust_Unnamed = 22;
pub const _SC_SEMAPHORES: C2Rust_Unnamed = 21;
pub const _SC_MESSAGE_PASSING: C2Rust_Unnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2Rust_Unnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2Rust_Unnamed = 18;
pub const _SC_MEMLOCK: C2Rust_Unnamed = 17;
pub const _SC_MAPPED_FILES: C2Rust_Unnamed = 16;
pub const _SC_FSYNC: C2Rust_Unnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2Rust_Unnamed = 14;
pub const _SC_PRIORITIZED_IO: C2Rust_Unnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2Rust_Unnamed = 12;
pub const _SC_TIMERS: C2Rust_Unnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2Rust_Unnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2Rust_Unnamed = 9;
pub const _SC_SAVED_IDS: C2Rust_Unnamed = 8;
pub const _SC_JOB_CONTROL: C2Rust_Unnamed = 7;
pub const _SC_TZNAME_MAX: C2Rust_Unnamed = 6;
pub const _SC_STREAM_MAX: C2Rust_Unnamed = 5;
pub const _SC_OPEN_MAX: C2Rust_Unnamed = 4;
pub const _SC_NGROUPS_MAX: C2Rust_Unnamed = 3;
pub const _SC_CLK_TCK: C2Rust_Unnamed = 2;
pub const _SC_CHILD_MAX: C2Rust_Unnamed = 1;
pub const _SC_ARG_MAX: C2Rust_Unnamed = 0;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_PERM: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_READONLY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_FULL: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SQLITE_WARNING: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const SQLITE_IOERR_READ: ::core::ffi::c_int = SQLITE_IOERR
    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_SHORT_READ: ::core::ffi::c_int = SQLITE_IOERR
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_WRITE: ::core::ffi::c_int = SQLITE_IOERR
    | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_FSTAT: ::core::ffi::c_int = SQLITE_IOERR
    | (7 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_UNLOCK: ::core::ffi::c_int = SQLITE_IOERR
    | (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_RDLOCK: ::core::ffi::c_int = SQLITE_IOERR
    | (9 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_CHECKRESERVEDLOCK: ::core::ffi::c_int = SQLITE_IOERR
    | (14 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_LOCK: ::core::ffi::c_int = SQLITE_IOERR
    | (15 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_CLOSE: ::core::ffi::c_int = SQLITE_IOERR
    | (16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_SHMSIZE: ::core::ffi::c_int = SQLITE_IOERR
    | (19 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_SHMLOCK: ::core::ffi::c_int = SQLITE_IOERR
    | (20 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_DELETE_NOENT: ::core::ffi::c_int = SQLITE_IOERR
    | (23 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_GETTEMPPATH: ::core::ffi::c_int = SQLITE_IOERR
    | (25 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_CORRUPTFS: ::core::ffi::c_int = SQLITE_IOERR
    | (33 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_READONLY_CANTINIT: ::core::ffi::c_int = SQLITE_READONLY
    | (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_READONLY_DIRECTORY: ::core::ffi::c_int = SQLITE_READONLY
    | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OK_SYMLINK: ::core::ffi::c_int = SQLITE_OK
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_DELETEONCLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_EXCLUSIVE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_OPEN_URI: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_JOURNAL: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_OPEN_SUPER_JOURNAL: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_WAL: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 0x1000
    as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SUBPAGE_READ: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_SYNC_FULL: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
pub const SQLITE_SYNC_DATAONLY: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_LOCKSTATE: ::core::ffi::c_int = 1;
pub const SQLITE_FCNTL_LAST_ERRNO: ::core::ffi::c_int = 4;
pub const SQLITE_FCNTL_SIZE_HINT: ::core::ffi::c_int = 5;
pub const SQLITE_FCNTL_CHUNK_SIZE: ::core::ffi::c_int = 6;
pub const SQLITE_FCNTL_PERSIST_WAL: ::core::ffi::c_int = 10;
pub const SQLITE_FCNTL_VFSNAME: ::core::ffi::c_int = 12;
pub const SQLITE_FCNTL_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 13;
pub const SQLITE_FCNTL_TEMPFILENAME: ::core::ffi::c_int = 16;
pub const SQLITE_FCNTL_MMAP_SIZE: ::core::ffi::c_int = 18;
pub const SQLITE_FCNTL_HAS_MOVED: ::core::ffi::c_int = 20;
pub const SQLITE_FCNTL_EXTERNAL_READER: ::core::ffi::c_int = 40;
pub const SQLITE_FCNTL_NULL_IO: ::core::ffi::c_int = 43;
pub const SQLITE_ACCESS_EXISTS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_SHM_UNLOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_SHM_LOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_SHM_SHARED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_SHM_EXCLUSIVE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_SHM_NLOCK: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_FAST: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_STATIC_VFS1: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_STATIC_TEMPDIR: ::core::ffi::c_int = SQLITE_MUTEX_STATIC_VFS1;
pub const SQLITE_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const ENOLCK: ::core::ffi::c_int = 37;
pub const ETIMEDOUT: ::core::ffi::c_int = 110;
pub const SQLITE_MAX_SYMLINK: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_SECTOR_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const NO_LOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SHARED_LOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RESERVED_LOCK: ::core::ffi::c_int = 2;
pub const PENDING_LOCK: ::core::ffi::c_int = 3;
pub const EXCLUSIVE_LOCK: ::core::ffi::c_int = 4;
pub const SHARED_SIZE: ::core::ffi::c_int = 510 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const SQLITE_IOERR_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_IOERR_NOMEM;
pub const EPERM: ::core::ffi::c_int = 1;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5;
pub const ENXIO: ::core::ffi::c_int = 6;
pub const EAGAIN: ::core::ffi::c_int = 11;
pub const EACCES: ::core::ffi::c_int = 13;
pub const EBUSY: ::core::ffi::c_int = 16;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const EISDIR: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const ENOSPC: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34;
pub const __O_LARGEFILE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_GETLK64: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const F_SETLK64: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const F_GETLK: ::core::ffi::c_int = F_GETLK64;
pub const F_SETLK: ::core::ffi::c_int = F_SETLK64;
pub const O_LARGEFILE: ::core::ffi::c_int = __O_LARGEFILE;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const F_RDLCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_WRLCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_UNLCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const W_OK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_SHARED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAP_FAILED: *mut ::core::ffi::c_void = -1 as ::core::ffi::c_int
    as *mut ::core::ffi::c_void;
pub const MREMAP_MAYMOVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn local_ioerr() {
    unsafe {
        sqlite3_io_error_hit += 1;
        if sqlite3_io_error_benign == 0 {
            sqlite3_io_error_hardhit += 1;
        }
    }
}
pub const RTLD_NOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const RTLD_GLOBAL: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_FILE_PERMISSIONS: ::core::ffi::c_int = 0o644
    as ::core::ffi::c_int;
pub const MAX_PATHNAME: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
static mut randomnessPid: pid_t = 0 as pid_t;
pub const UNIXFILE_EXCL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const UNIXFILE_RDONLY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const UNIXFILE_PERSIST_WAL: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const UNIXFILE_DIRSYNC: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const UNIXFILE_PSOW: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const UNIXFILE_DELETE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const UNIXFILE_URI: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const UNIXFILE_NOLOCK: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn posixOpen(
    mut zFile: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return open(zFile, flags, mode);
    }
}
static mut aSyscall: [unix_syscall; 29] = unsafe {
    [
        unix_syscall {
            zName: b"open\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    posixOpen
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"close\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
                sqlite3_syscall_ptr,
            >(
                Some(
                    close
                        as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"access\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    access
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"getcwd\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        size_t,
                    ) -> *mut ::core::ffi::c_char,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    getcwd
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_char,
                            size_t,
                        ) -> *mut ::core::ffi::c_char,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"stat\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    stat
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *mut stat,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"fstat\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    fstat
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *mut stat,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"ftruncate\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        __off64_t,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ftruncate
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            __off64_t,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"fcntl\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ...
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    fcntl
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ...
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"read\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> ssize_t,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    read
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_void,
                            size_t,
                        ) -> ssize_t,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"pread\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: None,
            pDefault: None

        },
        unix_syscall {
            zName: b"pread64\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_void,
                        size_t,
                        __off64_t,
                    ) -> ssize_t,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    pread64
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_void,
                            size_t,
                            __off64_t,
                        ) -> ssize_t,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"write\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        size_t,
                    ) -> ssize_t,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    write
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                            size_t,
                        ) -> ssize_t,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"pwrite\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: None,
            pDefault: None

        },
        unix_syscall {
            zName: b"pwrite64\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        size_t,
                        __off64_t,
                    ) -> ssize_t,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    pwrite64
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                            size_t,
                            __off64_t,
                        ) -> ssize_t,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"fchmod\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        __mode_t,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    fchmod
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            __mode_t,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"fallocate\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: None,
            pDefault: None

        },
        unix_syscall {
            zName: b"unlink\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    unlink
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"openDirectory\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    openDirectory
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"mkdir\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        __mode_t,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    mkdir
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            __mode_t,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"rmdir\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    rmdir
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"fchown\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        __uid_t,
                        __gid_t,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    fchown
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            __uid_t,
                            __gid_t,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"geteuid\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> __uid_t>,
                sqlite3_syscall_ptr,
            >(Some(geteuid as unsafe extern "C" fn() -> __uid_t)),
            pDefault: None

        },
        unix_syscall {
            zName: b"mmap\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        __off64_t,
                    ) -> *mut ::core::ffi::c_void,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    mmap
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            size_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            __off64_t,
                        ) -> *mut ::core::ffi::c_void,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"munmap\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    munmap
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            size_t,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"mremap\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        size_t,
                        ::core::ffi::c_int,
                        ...
                    ) -> *mut ::core::ffi::c_void,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    mremap
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            size_t,
                            size_t,
                            ::core::ffi::c_int,
                            ...
                        ) -> *mut ::core::ffi::c_void,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"getpagesize\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
                sqlite3_syscall_ptr,
            >(Some(unixGetpagesize as unsafe extern "C" fn() -> ::core::ffi::c_int)),
            pDefault: None

        },
        unix_syscall {
            zName: b"readlink\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                        size_t,
                    ) -> ssize_t,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    readlink
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *mut ::core::ffi::c_char,
                            size_t,
                        ) -> ssize_t,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"lstat\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    lstat
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *mut stat,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            pDefault: None

        },
        unix_syscall {
            zName: b"ioctl\0".as_ptr() as *const ::core::ffi::c_char,
            pCurrent: None,
            pDefault: None

        },
    ]
};
unsafe extern "C" fn robustFchown(
    mut fd: ::core::ffi::c_int,
    mut uid: uid_t,
    mut gid: gid_t,
) -> ::core::ffi::c_int {
    unsafe {
        return if ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<unsafe extern "C" fn() -> uid_t>,
        >(aSyscall[21 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")() != 0
        {
            0 as ::core::ffi::c_int
        } else {
            ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        uid_t,
                        gid_t,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[20 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")(fd, uid, gid)
        };
    }
}
unsafe extern "C" fn unixSetSystemCall(
    mut pNotUsed: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pNewFunc: sqlite3_syscall_ptr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_NOTFOUND;
        if zName.is_null() {
            rc = SQLITE_OK;
            i = 0 as ::core::ffi::c_uint;
            while (i as usize)
                < (::core::mem::size_of::<[unix_syscall; 29]>() as usize)
                    .wrapping_div(::core::mem::size_of::<unix_syscall>() as usize)
            {
                if aSyscall[i as usize].pDefault.is_some() {
                    aSyscall[i as usize].pCurrent = aSyscall[i as usize].pDefault;
                }
                i = i.wrapping_add(1);
            }
        } else {
            i = 0 as ::core::ffi::c_uint;
            while (i as usize)
                < (::core::mem::size_of::<[unix_syscall; 29]>() as usize)
                    .wrapping_div(::core::mem::size_of::<unix_syscall>() as usize)
            {
                if strcmp(zName, aSyscall[i as usize].zName) == 0 as ::core::ffi::c_int {
                    if aSyscall[i as usize].pDefault.is_none() {
                        aSyscall[i as usize].pDefault = aSyscall[i as usize].pCurrent;
                    }
                    rc = SQLITE_OK;
                    if pNewFunc.is_none() {
                        pNewFunc = aSyscall[i as usize].pDefault;
                    }
                    aSyscall[i as usize].pCurrent = pNewFunc;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn unixGetSystemCall(
    mut pNotUsed: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
) -> sqlite3_syscall_ptr {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while (i as usize)
            < (::core::mem::size_of::<[unix_syscall; 29]>() as usize)
                .wrapping_div(::core::mem::size_of::<unix_syscall>() as usize)
        {
            if strcmp(zName, aSyscall[i as usize].zName) == 0 as ::core::ffi::c_int {
                return aSyscall[i as usize].pCurrent;
            }
            i = i.wrapping_add(1);
        }
        return None;
    }
}
unsafe extern "C" fn unixNextSystemCall(
    mut p: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut i: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        if !zName.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i
                < (::core::mem::size_of::<[unix_syscall; 29]>() as usize)
                    .wrapping_div(::core::mem::size_of::<unix_syscall>() as usize)
                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            {
                if strcmp(zName, aSyscall[i as usize].zName) == 0 as ::core::ffi::c_int {
                    break;
                }
                i += 1;
            }
        }
        i += 1;
        while i
            < (::core::mem::size_of::<[unix_syscall; 29]>() as usize)
                .wrapping_div(::core::mem::size_of::<unix_syscall>() as usize)
                as ::core::ffi::c_int
        {
            if aSyscall[i as usize].pCurrent.is_some() {
                return aSyscall[i as usize].zName;
            }
            i += 1;
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
pub const SQLITE_MINIMUM_FILE_DESCRIPTOR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
unsafe extern "C" fn robust_open(
    mut z: *const ::core::ffi::c_char,
    mut f: ::core::ffi::c_int,
    mut m: mode_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut fd: ::core::ffi::c_int = 0;
        let mut m2: mode_t = if m != 0 {
            m
        } else {
            SQLITE_DEFAULT_FILE_PERMISSIONS as mode_t
        };
        loop {
            fd = ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[0 as ::core::ffi::c_int as usize].pCurrent)
                .expect(
                    "non-null function pointer",
                )(z, f | O_CLOEXEC, m2 as ::core::ffi::c_int);
            if fd < 0 as ::core::ffi::c_int {
                if !(*__errno_location() == EINTR) {
                    break;
                }
            } else {
                if fd >= SQLITE_MINIMUM_FILE_DESCRIPTOR {
                    break;
                }
                if f & (O_EXCL | O_CREAT) == O_EXCL | O_CREAT {
                    ::core::mem::transmute::<
                        sqlite3_syscall_ptr,
                        Option<
                            unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                    >(aSyscall[16 as ::core::ffi::c_int as usize].pCurrent)
                        .expect("non-null function pointer")(z);
                }
                ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[1 as ::core::ffi::c_int as usize].pCurrent)
                    .expect("non-null function pointer")(fd);
                sqlite3_log(
                    SQLITE_WARNING,
                    b"attempt to open \"%s\" as file descriptor %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    z,
                    fd,
                );
                fd = -1 as ::core::ffi::c_int;
                if ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[0 as ::core::ffi::c_int as usize].pCurrent)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
                    O_RDONLY,
                    m as ::core::ffi::c_int,
                ) < 0 as ::core::ffi::c_int
                {
                    break;
                }
            }
        }
        if fd >= 0 as ::core::ffi::c_int {
            if m != 0 as ::core::ffi::c_uint {
                let mut statbuf: stat = stat {
                    st_dev: 0,
                    st_ino: 0,
                    st_nlink: 0,
                    st_mode: 0,
                    st_uid: 0,
                    st_gid: 0,
                    __pad0: 0,
                    st_rdev: 0,
                    st_size: 0,
                    st_blksize: 0,
                    st_blocks: 0,
                    st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                    st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                    st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                    __glibc_reserved: [0; 3]

                };
                if ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *mut stat,
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
                    .expect("non-null function pointer")(fd, &raw mut statbuf)
                    == 0 as ::core::ffi::c_int
                    && statbuf.st_size == 0 as ::core::ffi::c_long
                    && statbuf.st_mode as ::core::ffi::c_uint
                        & 0o777 as ::core::ffi::c_uint != m
                {
                    ::core::mem::transmute::<
                        sqlite3_syscall_ptr,
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_int,
                                mode_t,
                            ) -> ::core::ffi::c_int,
                        >,
                    >(aSyscall[14 as ::core::ffi::c_int as usize].pCurrent)
                        .expect("non-null function pointer")(fd, m);
                }
            }
        }
        return fd;
    }
}
static mut unixBigLock: *mut sqlite3_mutex = ::core::ptr::null::<sqlite3_mutex>()
    as *mut sqlite3_mutex;
unsafe extern "C" fn unixEnterMutex() {
    unsafe {
        sqlite3_mutex_enter(unixBigLock);
    }
}
unsafe extern "C" fn unixLeaveMutex() {
    unsafe {
        sqlite3_mutex_leave(unixBigLock);
    }
}
unsafe extern "C" fn azFileLock(
    mut eFileLock: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        match eFileLock {
            NO_LOCK => return b"NONE\0".as_ptr() as *const ::core::ffi::c_char,
            SHARED_LOCK => return b"SHARED\0".as_ptr() as *const ::core::ffi::c_char,
            RESERVED_LOCK => return b"RESERVED\0".as_ptr() as *const ::core::ffi::c_char,
            PENDING_LOCK => return b"PENDING\0".as_ptr() as *const ::core::ffi::c_char,
            EXCLUSIVE_LOCK => {
                return b"EXCLUSIVE\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        return b"ERROR\0".as_ptr() as *const ::core::ffi::c_char;
    }
}
unsafe extern "C" fn robust_ftruncate(
    mut h: ::core::ffi::c_int,
    mut sz: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        loop {
            rc = ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(::core::ffi::c_int, off_t) -> ::core::ffi::c_int,
                >,
            >(aSyscall[6 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")(h, sz as off_t);
            if !(rc < 0 as ::core::ffi::c_int && *__errno_location() == EINTR) {
                break;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn sqliteErrorFromPosixError(
    mut posixError: ::core::ffi::c_int,
    mut sqliteIOErr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        match posixError {
            EACCES | EAGAIN | ETIMEDOUT | EBUSY | EINTR | ENOLCK => return SQLITE_BUSY,
            EPERM => return SQLITE_PERM,
            _ => return sqliteIOErr

        };
    }
}
static mut inodeList: *mut unixInodeInfo = ::core::ptr::null::<unixInodeInfo>()
    as *mut unixInodeInfo;
unsafe extern "C" fn unixLogErrorAtLine(
    mut errcode: ::core::ffi::c_int,
    mut zFunc: *const ::core::ffi::c_char,
    mut zPath: *const ::core::ffi::c_char,
    mut iLine: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut iErrno: ::core::ffi::c_int = *__errno_location();
        zErr = b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        if zPath.is_null() {
            zPath = b"\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sqlite3_log(
            errcode,
            b"os_unix.c:%d: (%d) %s(%s) - %s\0".as_ptr() as *const ::core::ffi::c_char,
            iLine,
            iErrno,
            zFunc,
            zPath,
            zErr,
        );
        return errcode;
    }
}
unsafe extern "C" fn robust_close(
    mut pFile: *mut unixFile,
    mut h: ::core::ffi::c_int,
    mut lineno: ::core::ffi::c_int,
) {
    unsafe {
        if ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
        >(aSyscall[1 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(h) != 0
        {
            unixLogErrorAtLine(
                SQLITE_IOERR_CLOSE,
                b"close\0".as_ptr() as *const ::core::ffi::c_char,
                if !pFile.is_null() {
                    (*pFile).zPath
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                lineno,
            );
        }
    }
}
unsafe extern "C" fn storeLastErrno(
    mut pFile: *mut unixFile,
    mut error: ::core::ffi::c_int,
) {
    unsafe {
        (*pFile).lastErrno = error;
    }
}
unsafe extern "C" fn closePendingFds(mut pFile: *mut unixFile) {
    unsafe {
        let mut pInode: *mut unixInodeInfo = (*pFile).pInode;
        let mut p: *mut UnixUnusedFd = ::core::ptr::null_mut::<UnixUnusedFd>();
        let mut pNext: *mut UnixUnusedFd = ::core::ptr::null_mut::<UnixUnusedFd>();
        p = (*pInode).pUnused;
        while !p.is_null() {
            pNext = (*p).pNext;
            robust_close(pFile, (*p).fd, 1478 as ::core::ffi::c_int);
            sqlite3_free(p as *mut ::core::ffi::c_void);
            p = pNext;
        }
        (*pInode).pUnused = ::core::ptr::null_mut::<UnixUnusedFd>();
    }
}
unsafe extern "C" fn releaseInodeInfo(mut pFile: *mut unixFile) {
    unsafe {
        let mut pInode: *mut unixInodeInfo = (*pFile).pInode;
        if !pInode.is_null() {
            (*pInode).nRef -= 1;
            if (*pInode).nRef == 0 as ::core::ffi::c_int {
                sqlite3_mutex_enter((*pInode).pLockMutex);
                closePendingFds(pFile);
                sqlite3_mutex_leave((*pInode).pLockMutex);
                if !(*pInode).pPrev.is_null() {
                    (*(*pInode).pPrev).pNext = (*pInode).pNext;
                } else {
                    inodeList = (*pInode).pNext;
                }
                if !(*pInode).pNext.is_null() {
                    (*(*pInode).pNext).pPrev = (*pInode).pPrev;
                }
                sqlite3_mutex_free((*pInode).pLockMutex);
                sqlite3_free(pInode as *mut ::core::ffi::c_void);
            }
        }
    }
}
unsafe extern "C" fn findInodeInfo(
    mut pFile: *mut unixFile,
    mut ppInode: *mut *mut unixInodeInfo,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut fd: ::core::ffi::c_int = 0;
        let mut fileId: unixFileId = unixFileId { dev: 0, ino: 0 };
        let mut statbuf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3]

        };
        let mut pInode: *mut unixInodeInfo = ::core::ptr::null_mut::<unixInodeInfo>();
        fd = (*pFile).h;
        rc = ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(::core::ffi::c_int, *mut stat) -> ::core::ffi::c_int,
            >,
        >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(fd, &raw mut statbuf);
        if rc != 0 as ::core::ffi::c_int {
            storeLastErrno(pFile, *__errno_location());
            return SQLITE_IOERR;
        }
        memset(
            &raw mut fileId as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<unixFileId>() as size_t,
        );
        fileId.dev = statbuf.st_dev as dev_t;
        fileId.ino = statbuf.st_ino as u64_0;
        pInode = inodeList;
        while !pInode.is_null()
            && memcmp(
                &raw mut fileId as *const ::core::ffi::c_void,
                &raw mut (*pInode).fileId as *const ::core::ffi::c_void,
                ::core::mem::size_of::<unixFileId>() as size_t,
            ) != 0
        {
            pInode = (*pInode).pNext;
        }
        if pInode.is_null() {
            pInode = sqlite3_malloc64(
                ::core::mem::size_of::<unixInodeInfo>() as sqlite3_uint64,
            ) as *mut unixInodeInfo;
            if pInode.is_null() {
                return SQLITE_NOMEM_BKPT;
            }
            memset(
                pInode as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<unixInodeInfo>() as size_t,
            );
            memcpy(
                &raw mut (*pInode).fileId as *mut ::core::ffi::c_void,
                &raw mut fileId as *const ::core::ffi::c_void,
                ::core::mem::size_of::<unixFileId>() as size_t,
            );
            if sqlite3Config.bCoreMutex != 0 {
                (*pInode).pLockMutex = sqlite3_mutex_alloc(SQLITE_MUTEX_FAST);
                if (*pInode).pLockMutex.is_null() {
                    sqlite3_free(pInode as *mut ::core::ffi::c_void);
                    return SQLITE_NOMEM_BKPT;
                }
            }
            (*pInode).nRef = 1 as ::core::ffi::c_int;
            (*pInode).pNext = inodeList;
            (*pInode).pPrev = ::core::ptr::null_mut::<unixInodeInfo>();
            if !inodeList.is_null() {
                (*inodeList).pPrev = pInode;
            }
            inodeList = pInode;
        } else {
            (*pInode).nRef += 1;
        }
        *ppInode = pInode;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fileHasMoved(mut pFile: *mut unixFile) -> ::core::ffi::c_int {
    unsafe {
        let mut buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3]

        };
        return (!(*pFile).pInode.is_null()
            && (::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[4 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")((*pFile).zPath, &raw mut buf)
                != 0 as ::core::ffi::c_int
                || buf.st_ino as u64_0 != (*(*pFile).pInode).fileId.ino))
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn verifyDbFile(mut pFile: *mut unixFile) {
    unsafe {
        let mut buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3]

        };
        let mut rc: ::core::ffi::c_int = 0;
        if (*pFile).ctrlFlags as ::core::ffi::c_int & UNIXFILE_NOLOCK != 0 {
            return;
        }
        rc = ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(::core::ffi::c_int, *mut stat) -> ::core::ffi::c_int,
            >,
        >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")((*pFile).h, &raw mut buf);
        if rc != 0 as ::core::ffi::c_int {
            sqlite3_log(
                SQLITE_WARNING,
                b"cannot fstat db file %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*pFile).zPath,
            );
            return;
        }
        if buf.st_nlink == 0 as ::core::ffi::c_ulong {
            sqlite3_log(
                SQLITE_WARNING,
                b"file unlinked while open: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*pFile).zPath,
            );
            return;
        }
        if buf.st_nlink > 1 as ::core::ffi::c_ulong {
            sqlite3_log(
                SQLITE_WARNING,
                b"multiple links to file: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*pFile).zPath,
            );
            return;
        }
        if fileHasMoved(pFile) != 0 {
            sqlite3_log(
                SQLITE_WARNING,
                b"file renamed while open: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*pFile).zPath,
            );
            return;
        }
    }
}
unsafe extern "C" fn unixCheckReservedLock(
    mut id: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut reserved: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pFile: *mut unixFile = id as *mut unixFile;
        if sqlite3_io_error_persist != 0 && sqlite3_io_error_hit != 0
            || {
                let c2rust_fresh19 = sqlite3_io_error_pending;
                sqlite3_io_error_pending = sqlite3_io_error_pending - 1;
                c2rust_fresh19 == 1 as ::core::ffi::c_int
            }
        {
            local_ioerr();
            return 10 as ::core::ffi::c_int
                | (14 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
        }
        sqlite3_mutex_enter((*(*pFile).pInode).pLockMutex);
        if (*(*pFile).pInode).eFileLock as ::core::ffi::c_int > SHARED_LOCK {
            reserved = 1 as ::core::ffi::c_int;
        }
        if reserved == 0 && (*(*pFile).pInode).bProcessLock == 0 {
            let mut lock: flock = flock {
                l_type: 0,
                l_whence: 0,
                l_start: 0,
                l_len: 0,
                l_pid: 0

            };
            lock.l_whence = SEEK_SET as ::core::ffi::c_short;
            lock.l_start = (sqlite3PendingByte + 1 as ::core::ffi::c_int) as __off64_t;
            lock.l_len = 1 as __off64_t;
            lock.l_type = F_WRLCK as ::core::ffi::c_short;
            if ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ...
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")((*pFile).h, F_GETLK, &raw mut lock)
                != 0
            {
                rc = SQLITE_IOERR_CHECKRESERVEDLOCK;
                storeLastErrno(pFile, *__errno_location());
            } else if lock.l_type as ::core::ffi::c_int != F_UNLCK {
                reserved = 1 as ::core::ffi::c_int;
            }
        }
        sqlite3_mutex_leave((*(*pFile).pInode).pLockMutex);
        if sqlite3OSTrace != 0 {
            sqlite3DebugPrintf(
                b"TEST WR-LOCK %d %d %d (unix)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pFile).h,
                rc,
                reserved,
            );
        }
        *pResOut = reserved;
        return rc;
    }
}
unsafe extern "C" fn unixFileLock(
    mut pFile: *mut unixFile,
    mut pLock: *mut flock,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pInode: *mut unixInodeInfo = (*pFile).pInode;
        if (*pFile).ctrlFlags as ::core::ffi::c_int & (UNIXFILE_EXCL | UNIXFILE_RDONLY)
            == UNIXFILE_EXCL
        {
            if (*pInode).bProcessLock as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let mut lock: flock = flock {
                    l_type: 0,
                    l_whence: 0,
                    l_start: 0,
                    l_len: 0,
                    l_pid: 0

                };
                lock.l_whence = SEEK_SET as ::core::ffi::c_short;
                lock.l_start = (sqlite3PendingByte + 2 as ::core::ffi::c_int)
                    as __off64_t;
                lock.l_len = SHARED_SIZE as __off64_t;
                lock.l_type = F_WRLCK as ::core::ffi::c_short;
                rc = ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ...
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
                    .expect(
                        "non-null function pointer",
                    )((*pFile).h, F_SETLK, &raw mut lock);
                if rc < 0 as ::core::ffi::c_int {
                    return rc;
                }
                (*pInode).bProcessLock = 1 as ::core::ffi::c_uchar;
                (*pInode).nLock += 1;
            } else {
                rc = 0 as ::core::ffi::c_int;
            }
        } else {
            rc = ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ...
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")((*pFile).h, F_SETLK, pLock);
        }
        return rc;
    }
}
unsafe extern "C" fn unixLock(
    mut id: *mut sqlite3_file,
    mut eFileLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pFile: *mut unixFile = id as *mut unixFile;
        let mut pInode: *mut unixInodeInfo = ::core::ptr::null_mut::<unixInodeInfo>();
        let mut lock: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0

        };
        let mut tErrno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if sqlite3OSTrace != 0 {
            sqlite3DebugPrintf(
                b"LOCK    %d %s was %s(%s,%d) pid=%d (unix)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pFile).h,
                azFileLock(eFileLock),
                azFileLock((*pFile).eFileLock as ::core::ffi::c_int),
                azFileLock((*(*pFile).pInode).eFileLock as ::core::ffi::c_int),
                (*(*pFile).pInode).nShared,
                getpid(),
            );
        }
        if (*pFile).eFileLock as ::core::ffi::c_int >= eFileLock {
            if sqlite3OSTrace != 0 {
                sqlite3DebugPrintf(
                    b"LOCK    %d %s ok (already held) (unix)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*pFile).h,
                    azFileLock(eFileLock),
                );
            }
            return SQLITE_OK;
        }
        pInode = (*pFile).pInode;
        sqlite3_mutex_enter((*pInode).pLockMutex);
        if (*pFile).eFileLock as ::core::ffi::c_int
            != (*pInode).eFileLock as ::core::ffi::c_int
            && ((*pInode).eFileLock as ::core::ffi::c_int >= PENDING_LOCK
                || eFileLock > SHARED_LOCK)
        {
            rc = SQLITE_BUSY;
        } else if eFileLock == SHARED_LOCK
            && ((*pInode).eFileLock as ::core::ffi::c_int == SHARED_LOCK
                || (*pInode).eFileLock as ::core::ffi::c_int == RESERVED_LOCK)
        {
            (*pFile).eFileLock = SHARED_LOCK as ::core::ffi::c_uchar;
            (*pInode).nShared += 1;
            (*pInode).nLock += 1;
        } else {
            lock.l_len = 1 as ::core::ffi::c_long as __off64_t;
            lock.l_whence = SEEK_SET as ::core::ffi::c_short;
            if eFileLock == SHARED_LOCK
                || eFileLock == EXCLUSIVE_LOCK
                    && (*pFile).eFileLock as ::core::ffi::c_int == RESERVED_LOCK
            {
                lock.l_type = (if eFileLock == SHARED_LOCK { F_RDLCK } else { F_WRLCK })
                    as ::core::ffi::c_short;
                lock.l_start = sqlite3PendingByte as __off64_t;
                if unixFileLock(pFile, &raw mut lock) != 0 {
                    tErrno = *__errno_location();
                    rc = sqliteErrorFromPosixError(tErrno, SQLITE_IOERR_LOCK);
                    if rc != SQLITE_BUSY {
                        storeLastErrno(pFile, tErrno);
                    }
                    c2rust_current_block = 12996110020734099755;
                } else {
                    if eFileLock == EXCLUSIVE_LOCK {
                        (*pFile).eFileLock = PENDING_LOCK as ::core::ffi::c_uchar;
                        (*pInode).eFileLock = PENDING_LOCK as ::core::ffi::c_uchar;
                    }
                    c2rust_current_block = 4488286894823169796;
                }
            } else {
                c2rust_current_block = 4488286894823169796;
            }
            match c2rust_current_block {
                12996110020734099755 => {}
                _ => {
                    if eFileLock == SHARED_LOCK {
                        lock.l_start = (sqlite3PendingByte + 2 as ::core::ffi::c_int)
                            as __off64_t;
                        lock.l_len = SHARED_SIZE as __off64_t;
                        if unixFileLock(pFile, &raw mut lock) != 0 {
                            tErrno = *__errno_location();
                            rc = sqliteErrorFromPosixError(tErrno, SQLITE_IOERR_LOCK);
                        }
                        lock.l_start = sqlite3PendingByte as __off64_t;
                        lock.l_len = 1 as ::core::ffi::c_long as __off64_t;
                        lock.l_type = F_UNLCK as ::core::ffi::c_short;
                        if unixFileLock(pFile, &raw mut lock) != 0 && rc == SQLITE_OK {
                            tErrno = *__errno_location();
                            rc = SQLITE_IOERR_UNLOCK;
                        }
                        if rc != 0 {
                            if rc != SQLITE_BUSY {
                                storeLastErrno(pFile, tErrno);
                            }
                            c2rust_current_block = 12996110020734099755;
                        } else {
                            (*pFile).eFileLock = SHARED_LOCK as ::core::ffi::c_uchar;
                            (*pInode).nLock += 1;
                            (*pInode).nShared = 1 as ::core::ffi::c_int;
                            c2rust_current_block = 12758904613967585247;
                        }
                    } else {
                        if eFileLock == EXCLUSIVE_LOCK
                            && (*pInode).nShared > 1 as ::core::ffi::c_int
                        {
                            rc = SQLITE_BUSY;
                        } else if unixIsSharingShmNode(pFile) != 0 {
                            rc = SQLITE_BUSY;
                        } else {
                            lock.l_type = F_WRLCK as ::core::ffi::c_short;
                            if eFileLock == RESERVED_LOCK {
                                lock.l_start = (sqlite3PendingByte
                                    + 1 as ::core::ffi::c_int) as __off64_t;
                                lock.l_len = 1 as ::core::ffi::c_long as __off64_t;
                            } else {
                                lock.l_start = (sqlite3PendingByte
                                    + 2 as ::core::ffi::c_int) as __off64_t;
                                lock.l_len = SHARED_SIZE as __off64_t;
                            }
                            if unixFileLock(pFile, &raw mut lock) != 0 {
                                tErrno = *__errno_location();
                                rc = sqliteErrorFromPosixError(tErrno, SQLITE_IOERR_LOCK);
                                if rc != SQLITE_BUSY {
                                    storeLastErrno(pFile, tErrno);
                                }
                            }
                        }
                        c2rust_current_block = 12758904613967585247;
                    }
                    match c2rust_current_block {
                        12996110020734099755 => {}
                        _ => {
                            if rc == SQLITE_OK {
                                (*pFile).eFileLock = eFileLock as ::core::ffi::c_uchar;
                                (*pInode).eFileLock = eFileLock as ::core::ffi::c_uchar;
                            }
                        }
                    }
                }
            }
        }
        sqlite3_mutex_leave((*pInode).pLockMutex);
        if sqlite3OSTrace != 0 {
            sqlite3DebugPrintf(
                b"LOCK    %d %s %s (unix)\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*pFile).h,
                azFileLock(eFileLock),
                if rc == 0 as ::core::ffi::c_int {
                    b"ok\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"failed\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        return rc;
    }
}
unsafe extern "C" fn setPendingFd(mut pFile: *mut unixFile) {
    unsafe {
        let mut pInode: *mut unixInodeInfo = (*pFile).pInode;
        let mut p: *mut UnixUnusedFd = (*pFile).pPreallocatedUnused;
        (*p).pNext = (*pInode).pUnused;
        (*pInode).pUnused = p;
        (*pFile).h = -1 as ::core::ffi::c_int;
        (*pFile).pPreallocatedUnused = ::core::ptr::null_mut::<UnixUnusedFd>();
    }
}
unsafe extern "C" fn posixUnlock(
    mut id: *mut sqlite3_file,
    mut eFileLock: ::core::ffi::c_int,
    mut handleNFSUnlock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pFile: *mut unixFile = id as *mut unixFile;
        let mut pInode: *mut unixInodeInfo = ::core::ptr::null_mut::<unixInodeInfo>();
        let mut lock: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0

        };
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if sqlite3OSTrace != 0 {
            sqlite3DebugPrintf(
                b"UNLOCK  %d %d was %d(%d,%d) pid=%d (unix)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pFile).h,
                eFileLock,
                (*pFile).eFileLock as ::core::ffi::c_int,
                (*(*pFile).pInode).eFileLock as ::core::ffi::c_int,
                (*(*pFile).pInode).nShared,
                getpid(),
            );
        }
        if (*pFile).eFileLock as ::core::ffi::c_int <= eFileLock {
            return SQLITE_OK;
        }
        pInode = (*pFile).pInode;
        sqlite3_mutex_enter((*pInode).pLockMutex);
        if (*pFile).eFileLock as ::core::ffi::c_int > SHARED_LOCK {
            if eFileLock == SHARED_LOCK {
                lock.l_type = F_RDLCK as ::core::ffi::c_short;
                lock.l_whence = SEEK_SET as ::core::ffi::c_short;
                lock.l_start = (sqlite3PendingByte + 2 as ::core::ffi::c_int)
                    as __off64_t;
                lock.l_len = SHARED_SIZE as __off64_t;
                if unixFileLock(pFile, &raw mut lock) != 0 {
                    rc = SQLITE_IOERR_RDLOCK;
                    storeLastErrno(pFile, *__errno_location());
                    c2rust_current_block = 1641373882995071503;
                } else {
                    c2rust_current_block = 15652330335145281839;
                }
            } else {
                c2rust_current_block = 15652330335145281839;
            }
            match c2rust_current_block {
                1641373882995071503 => {}
                _ => {
                    lock.l_type = F_UNLCK as ::core::ffi::c_short;
                    lock.l_whence = SEEK_SET as ::core::ffi::c_short;
                    lock.l_start = sqlite3PendingByte as __off64_t;
                    lock.l_len = 2 as ::core::ffi::c_long as __off64_t;
                    if unixFileLock(pFile, &raw mut lock) == 0 as ::core::ffi::c_int {
                        (*pInode).eFileLock = SHARED_LOCK as ::core::ffi::c_uchar;
                        c2rust_current_block = 16203760046146113240;
                    } else {
                        rc = SQLITE_IOERR_UNLOCK;
                        storeLastErrno(pFile, *__errno_location());
                        c2rust_current_block = 1641373882995071503;
                    }
                }
            }
        } else {
            c2rust_current_block = 16203760046146113240;
        }
        match c2rust_current_block {
            16203760046146113240 => {
                if eFileLock == NO_LOCK {
                    (*pInode).nShared -= 1;
                    if (*pInode).nShared == 0 as ::core::ffi::c_int {
                        lock.l_type = F_UNLCK as ::core::ffi::c_short;
                        lock.l_whence = SEEK_SET as ::core::ffi::c_short;
                        lock.l_len = 0 as ::core::ffi::c_long as __off64_t;
                        lock.l_start = lock.l_len;
                        if unixFileLock(pFile, &raw mut lock) == 0 as ::core::ffi::c_int
                        {
                            (*pInode).eFileLock = NO_LOCK as ::core::ffi::c_uchar;
                        } else {
                            rc = SQLITE_IOERR_UNLOCK;
                            storeLastErrno(pFile, *__errno_location());
                            (*pInode).eFileLock = NO_LOCK as ::core::ffi::c_uchar;
                            (*pFile).eFileLock = NO_LOCK as ::core::ffi::c_uchar;
                        }
                    }
                    (*pInode).nLock -= 1;
                    if (*pInode).nLock == 0 as ::core::ffi::c_int {
                        closePendingFds(pFile);
                    }
                }
            }
            _ => {}
        }
        sqlite3_mutex_leave((*pInode).pLockMutex);
        if rc == SQLITE_OK {
            (*pFile).eFileLock = eFileLock as ::core::ffi::c_uchar;
        }
        return rc;
    }
}
unsafe extern "C" fn unixUnlock(
    mut id: *mut sqlite3_file,
    mut eFileLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return posixUnlock(id, eFileLock, 0 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn closeUnixFile(mut id: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut pFile: *mut unixFile = id as *mut unixFile;
        unixUnmapfile(pFile);
        if (*pFile).h >= 0 as ::core::ffi::c_int {
            robust_close(pFile, (*pFile).h, 2312 as ::core::ffi::c_int);
            (*pFile).h = -1 as ::core::ffi::c_int;
        }
        if sqlite3OSTrace != 0 {
            sqlite3DebugPrintf(
                b"CLOSE   %-3d\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*pFile).h,
            );
        }
        sqlite3_open_file_count += -1 as ::core::ffi::c_int;
        sqlite3_free((*pFile).pPreallocatedUnused as *mut ::core::ffi::c_void);
        memset(
            pFile as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<unixFile>() as size_t,
        );
        return SQLITE_OK;
    }
}
unsafe extern "C" fn unixClose(mut id: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pFile: *mut unixFile = id as *mut unixFile;
        let mut pInode: *mut unixInodeInfo = (*pFile).pInode;
        verifyDbFile(pFile);
        unixUnlock(id, NO_LOCK);
        unixEnterMutex();
        sqlite3_mutex_enter((*pInode).pLockMutex);
        if (*pInode).nLock != 0 {
            setPendingFd(pFile);
        }
        sqlite3_mutex_leave((*pInode).pLockMutex);
        releaseInodeInfo(pFile);
        rc = closeUnixFile(id);
        unixLeaveMutex();
        return rc;
    }
}
unsafe extern "C" fn nolockCheckReservedLock(
    mut NotUsed: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        *pResOut = 0 as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn nolockLock(
    mut NotUsed: *mut sqlite3_file,
    mut NotUsed2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn nolockUnlock(
    mut NotUsed: *mut sqlite3_file,
    mut NotUsed2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn nolockClose(mut id: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        return closeUnixFile(id);
    }
}
unsafe extern "C" fn dotlockCheckReservedLock(
    mut id: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFile: *mut unixFile = id as *mut unixFile;
        if sqlite3_io_error_persist != 0 && sqlite3_io_error_hit != 0
            || {
                let c2rust_fresh12 = sqlite3_io_error_pending;
                sqlite3_io_error_pending = sqlite3_io_error_pending - 1;
                c2rust_fresh12 == 1 as ::core::ffi::c_int
            }
        {
            local_ioerr();
            return 10 as ::core::ffi::c_int
                | (14 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
        }
        if (*pFile).eFileLock as ::core::ffi::c_int >= SHARED_LOCK {
            *pResOut = 0 as ::core::ffi::c_int;
        } else {
            *pResOut = (::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[2 as ::core::ffi::c_int as usize].pCurrent)
                .expect(
                    "non-null function pointer",
                )(
                (*pFile).lockingContext as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
        if sqlite3OSTrace != 0 {
            sqlite3DebugPrintf(
                b"TEST WR-LOCK %d %d %d (dotlock)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pFile).h,
                0 as ::core::ffi::c_int,
                *pResOut,
            );
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn dotlockLock(
    mut id: *mut sqlite3_file,
    mut eFileLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFile: *mut unixFile = id as *mut unixFile;
        let mut zLockFile: *mut ::core::ffi::c_char = (*pFile).lockingContext
            as *mut ::core::ffi::c_char;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFile).eFileLock as ::core::ffi::c_int > NO_LOCK {
            (*pFile).eFileLock = eFileLock as ::core::ffi::c_uchar;
            utime(zLockFile, ::core::ptr::null::<utimbuf>());
            return SQLITE_OK;
        }
        rc = ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    mode_t,
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[18 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(zLockFile, 0o777 as mode_t);
        if rc < 0 as ::core::ffi::c_int {
            let mut tErrno: ::core::ffi::c_int = *__errno_location();
            if EEXIST == tErrno {
                rc = SQLITE_BUSY;
            } else {
                rc = sqliteErrorFromPosixError(tErrno, SQLITE_IOERR_LOCK);
                if rc != SQLITE_BUSY {
                    storeLastErrno(pFile, tErrno);
                }
            }
            return rc;
        }
        (*pFile).eFileLock = eFileLock as ::core::ffi::c_uchar;
        return rc;
    }
}
unsafe extern "C" fn dotlockUnlock(
    mut id: *mut sqlite3_file,
    mut eFileLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFile: *mut unixFile = id as *mut unixFile;
        let mut zLockFile: *mut ::core::ffi::c_char = (*pFile).lockingContext
            as *mut ::core::ffi::c_char;
        let mut rc: ::core::ffi::c_int = 0;
        if sqlite3OSTrace != 0 {
            sqlite3DebugPrintf(
                b"UNLOCK  %d %d was %d pid=%d (dotlock)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pFile).h,
                eFileLock,
                (*pFile).eFileLock as ::core::ffi::c_int,
                getpid(),
            );
        }
        if (*pFile).eFileLock as ::core::ffi::c_int == eFileLock {
            return SQLITE_OK;
        }
        if eFileLock == SHARED_LOCK {
            (*pFile).eFileLock = SHARED_LOCK as ::core::ffi::c_uchar;
            return SQLITE_OK;
        }
        rc = ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            >,
        >(aSyscall[19 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(zLockFile);
        if rc < 0 as ::core::ffi::c_int {
            let mut tErrno: ::core::ffi::c_int = *__errno_location();
            if tErrno == ENOENT {
                rc = SQLITE_OK;
            } else {
                rc = SQLITE_IOERR_UNLOCK;
                storeLastErrno(pFile, tErrno);
            }
            return rc;
        }
        (*pFile).eFileLock = NO_LOCK as ::core::ffi::c_uchar;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn dotlockClose(mut id: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut pFile: *mut unixFile = id as *mut unixFile;
        dotlockUnlock(id, NO_LOCK);
        sqlite3_free((*pFile).lockingContext);
        return closeUnixFile(id);
    }
}
unsafe extern "C" fn seekAndRead(
    mut id: *mut unixFile,
    mut offset: sqlite3_int64,
    mut pBuf: *mut ::core::ffi::c_void,
    mut cnt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut got: ::core::ffi::c_int = 0;
        let mut prior: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            got = ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_void,
                        size_t,
                        off64_t,
                    ) -> ssize_t,
                >,
            >(aSyscall[10 as ::core::ffi::c_int as usize].pCurrent)
                .expect(
                    "non-null function pointer",
                )((*id).h, pBuf, cnt as size_t, offset as off64_t) as ::core::ffi::c_int;
            if sqlite3_io_error_persist != 0 && sqlite3_io_error_hit != 0
                || {
                    let c2rust_fresh17 = sqlite3_io_error_pending;
                    sqlite3_io_error_pending = sqlite3_io_error_pending - 1;
                    c2rust_fresh17 == 1 as ::core::ffi::c_int
                }
            {
                local_ioerr();
                got = -1 as ::core::ffi::c_int;
            }
            if got == cnt {
                break;
            }
            if got < 0 as ::core::ffi::c_int {
                if *__errno_location() == EINTR {
                    got = 1 as ::core::ffi::c_int;
                } else {
                    prior = 0 as ::core::ffi::c_int;
                    storeLastErrno(id, *__errno_location());
                    break;
                }
            } else if got > 0 as ::core::ffi::c_int {
                cnt -= got;
                offset += got as ::core::ffi::c_longlong;
                prior += got;
                pBuf = (pBuf as *mut ::core::ffi::c_char).offset(got as isize)
                    as *mut ::core::ffi::c_void;
            }
            if !(got > 0 as ::core::ffi::c_int) {
                break;
            }
        }
        if sqlite3OSTrace != 0 {
            sqlite3DebugPrintf(
                b"READ    %-3d %5d %7lld %llu\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*id).h,
                got + prior,
                offset as ::core::ffi::c_longlong - prior as ::core::ffi::c_longlong,
                0 as ::core::ffi::c_int as sqlite_uint64,
            );
        }
        return got + prior;
    }
}
unsafe extern "C" fn unixRead(
    mut id: *mut sqlite3_file,
    mut pBuf: *mut ::core::ffi::c_void,
    mut amt: ::core::ffi::c_int,
    mut offset: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFile: *mut unixFile = id as *mut unixFile;
        let mut got: ::core::ffi::c_int = 0;
        if offset < (*pFile).mmapSize {
            if offset as ::core::ffi::c_longlong + amt as ::core::ffi::c_longlong
                <= (*pFile).mmapSize
            {
                memcpy(
                    pBuf,
                    ((*pFile).pMapRegion as *mut u8_0).offset(offset as isize)
                        as *mut u8_0 as *const ::core::ffi::c_void,
                    amt as size_t,
                );
                return SQLITE_OK;
            } else {
                let mut nCopy: ::core::ffi::c_int = ((*pFile).mmapSize - offset)
                    as ::core::ffi::c_int;
                memcpy(
                    pBuf,
                    ((*pFile).pMapRegion as *mut u8_0).offset(offset as isize)
                        as *mut u8_0 as *const ::core::ffi::c_void,
                    nCopy as size_t,
                );
                pBuf = (pBuf as *mut u8_0).offset(nCopy as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void;
                amt -= nCopy;
                offset += nCopy as ::core::ffi::c_longlong;
            }
        }
        got = seekAndRead(pFile, offset, pBuf, amt);
        if got == amt {
            return SQLITE_OK
        } else if got < 0 as ::core::ffi::c_int {
            match (*pFile).lastErrno {
                ERANGE | EIO | ENXIO => return SQLITE_IOERR_CORRUPTFS,
                _ => {}
            }
            return SQLITE_IOERR_READ;
        } else {
            storeLastErrno(pFile, 0 as ::core::ffi::c_int);
            memset(
                (pBuf as *mut ::core::ffi::c_char).offset(got as isize)
                    as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (amt - got) as size_t,
            );
            return SQLITE_IOERR_SHORT_READ;
        };
    }
}
unsafe extern "C" fn seekAndWriteFd(
    mut fd: ::core::ffi::c_int,
    mut iOff: i64_0,
    mut pBuf: *const ::core::ffi::c_void,
    mut nBuf: ::core::ffi::c_int,
    mut piErrno: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        nBuf &= 0x1ffff as ::core::ffi::c_int;
        loop {
            rc = ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        size_t,
                        off64_t,
                    ) -> ssize_t,
                >,
            >(aSyscall[13 as ::core::ffi::c_int as usize].pCurrent)
                .expect(
                    "non-null function pointer",
                )(fd, pBuf, nBuf as size_t, iOff as off64_t) as ::core::ffi::c_int;
            if !(rc < 0 as ::core::ffi::c_int && *__errno_location() == EINTR) {
                break;
            }
        }
        if sqlite3OSTrace != 0 {
            sqlite3DebugPrintf(
                b"WRITE   %-3d %5d %7lld %llu\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                fd,
                rc,
                iOff,
                0 as ::core::ffi::c_int as sqlite_uint64,
            );
        }
        if rc < 0 as ::core::ffi::c_int {
            *piErrno = *__errno_location();
        }
        return rc;
    }
}
unsafe extern "C" fn seekAndWrite(
    mut id: *mut unixFile,
    mut offset: i64_0,
    mut pBuf: *const ::core::ffi::c_void,
    mut cnt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return seekAndWriteFd((*id).h, offset, pBuf, cnt, &raw mut (*id).lastErrno);
    }
}
unsafe extern "C" fn unixWrite(
    mut id: *mut sqlite3_file,
    mut pBuf: *const ::core::ffi::c_void,
    mut amt: ::core::ffi::c_int,
    mut offset: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFile: *mut unixFile = id as *mut unixFile;
        let mut wrote: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            wrote = seekAndWrite(pFile, offset as i64_0, pBuf, amt);
            if !(wrote < amt && wrote > 0 as ::core::ffi::c_int) {
                break;
            }
            amt -= wrote;
            offset += wrote as ::core::ffi::c_longlong;
            pBuf = (pBuf as *mut ::core::ffi::c_char).offset(wrote as isize)
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_void;
        }
        if sqlite3_io_error_persist != 0 && sqlite3_io_error_hit != 0
            || {
                let c2rust_fresh16 = sqlite3_io_error_pending;
                sqlite3_io_error_pending = sqlite3_io_error_pending - 1;
                c2rust_fresh16 == 1 as ::core::ffi::c_int
            }
        {
            local_ioerr();
            wrote = -1 as ::core::ffi::c_int;
            amt = 1 as ::core::ffi::c_int;
        }
        if sqlite3_diskfull_pending != 0 {
            if sqlite3_diskfull_pending == 1 as ::core::ffi::c_int {
                local_ioerr();
                sqlite3_diskfull = 1 as ::core::ffi::c_int;
                sqlite3_io_error_hit = 1 as ::core::ffi::c_int;
                wrote = 0 as ::core::ffi::c_int;
                amt = 1 as ::core::ffi::c_int;
            } else {
                sqlite3_diskfull_pending -= 1;
            }
        }
        if amt > wrote {
            if wrote < 0 as ::core::ffi::c_int && (*pFile).lastErrno != ENOSPC {
                return SQLITE_IOERR_WRITE
            } else {
                storeLastErrno(pFile, 0 as ::core::ffi::c_int);
                return SQLITE_FULL;
            }
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub static mut sqlite3_sync_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_fullsync_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn full_fsync(
    mut fd: ::core::ffi::c_int,
    mut fullSync: ::core::ffi::c_int,
    mut dataOnly: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if fullSync != 0 {
            sqlite3_fullsync_count += 1;
        }
        sqlite3_sync_count += 1;
        let mut buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3]

        };
        rc = ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(::core::ffi::c_int, *mut stat) -> ::core::ffi::c_int,
            >,
        >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(fd, &raw mut buf);
        if OS_VXWORKS != 0 && rc != -1 as ::core::ffi::c_int {
            rc = 0 as ::core::ffi::c_int;
        }
        return rc;
    }
}
unsafe extern "C" fn openDirectory(
    mut zFilename: *const ::core::ffi::c_char,
    mut pFd: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        let mut fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut zDirname: [::core::ffi::c_char; 513] = [0; 513];
        sqlite3_snprintf(
            MAX_PATHNAME,
            &raw mut zDirname as *mut ::core::ffi::c_char,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            zFilename,
        );
        ii = strlen(&raw mut zDirname as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
        while ii > 0 as ::core::ffi::c_int
            && zDirname[ii as usize] as ::core::ffi::c_int != '/' as i32
        {
            ii -= 1;
        }
        if ii > 0 as ::core::ffi::c_int {
            zDirname[ii as usize] = '\0' as i32 as ::core::ffi::c_char;
        } else {
            if zDirname[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                != '/' as i32
            {
                zDirname[0 as ::core::ffi::c_int as usize] = '.' as i32
                    as ::core::ffi::c_char;
            }
            zDirname[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        }
        fd = robust_open(
            &raw mut zDirname as *mut ::core::ffi::c_char,
            O_RDONLY | O_BINARY,
            0 as mode_t,
        );
        if fd >= 0 as ::core::ffi::c_int {
            if sqlite3OSTrace != 0 {
                sqlite3DebugPrintf(
                    b"OPENDIR %-3d %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    fd,
                    &raw mut zDirname as *mut ::core::ffi::c_char,
                );
            }
        }
        *pFd = fd;
        if fd >= 0 as ::core::ffi::c_int {
            return SQLITE_OK;
        }
        return unixLogErrorAtLine(
            sqlite3CantopenError(3893 as ::core::ffi::c_int),
            b"openDirectory\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut zDirname as *mut ::core::ffi::c_char,
            3893 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn unixSync(
    mut id: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pFile: *mut unixFile = id as *mut unixFile;
        let mut isDataOnly: ::core::ffi::c_int = flags & SQLITE_SYNC_DATAONLY;
        let mut isFullsync: ::core::ffi::c_int = (flags & 0xf as ::core::ffi::c_int
            == SQLITE_SYNC_FULL) as ::core::ffi::c_int;
        if sqlite3_diskfull_pending != 0 {
            if sqlite3_diskfull_pending == 1 as ::core::ffi::c_int {
                local_ioerr();
                sqlite3_diskfull = 1 as ::core::ffi::c_int;
                sqlite3_io_error_hit = 1 as ::core::ffi::c_int;
                return 13 as ::core::ffi::c_int;
            } else {
                sqlite3_diskfull_pending -= 1;
            }
        }
        if sqlite3OSTrace != 0 {
            sqlite3DebugPrintf(
                b"SYNC    %-3d\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*pFile).h,
            );
        }
        rc = full_fsync((*pFile).h, isFullsync, isDataOnly);
        if sqlite3_io_error_persist != 0 && sqlite3_io_error_hit != 0
            || {
                let c2rust_fresh14 = sqlite3_io_error_pending;
                sqlite3_io_error_pending = sqlite3_io_error_pending - 1;
                c2rust_fresh14 == 1 as ::core::ffi::c_int
            }
        {
            local_ioerr();
            rc = 1 as ::core::ffi::c_int;
        }
        if rc != 0 {
            storeLastErrno(pFile, *__errno_location());
            return unixLogErrorAtLine(
                10 as ::core::ffi::c_int
                    | (4 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                b"full_fsync\0".as_ptr() as *const ::core::ffi::c_char,
                (*pFile).zPath,
                3934 as ::core::ffi::c_int,
            );
        }
        if (*pFile).ctrlFlags as ::core::ffi::c_int & UNIXFILE_DIRSYNC != 0 {
            let mut dirfd: ::core::ffi::c_int = 0;
            if sqlite3OSTrace != 0 {
                sqlite3DebugPrintf(
                    b"DIRSYNC %s (have_fullfsync=%d fullsync=%d)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*pFile).zPath,
                    0 as ::core::ffi::c_int,
                    isFullsync,
                );
            }
            rc = ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[17 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")((*pFile).zPath, &raw mut dirfd);
            if rc == SQLITE_OK {
                full_fsync(dirfd, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
                robust_close(pFile, dirfd, 3948 as ::core::ffi::c_int);
            } else {
                rc = SQLITE_OK;
            }
            (*pFile).ctrlFlags = ((*pFile).ctrlFlags as ::core::ffi::c_int
                & !UNIXFILE_DIRSYNC) as ::core::ffi::c_ushort;
        }
        return rc;
    }
}
unsafe extern "C" fn unixTruncate(
    mut id: *mut sqlite3_file,
    mut nByte: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFile: *mut unixFile = id as *mut unixFile;
        let mut rc: ::core::ffi::c_int = 0;
        if sqlite3_io_error_persist != 0 && sqlite3_io_error_hit != 0
            || {
                let c2rust_fresh15 = sqlite3_io_error_pending;
                sqlite3_io_error_pending = sqlite3_io_error_pending - 1;
                c2rust_fresh15 == 1 as ::core::ffi::c_int
            }
        {
            local_ioerr();
            return 10 as ::core::ffi::c_int
                | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
        }
        if (*pFile).szChunk > 0 as ::core::ffi::c_int {
            nByte = ((nByte as ::core::ffi::c_longlong
                + (*pFile).szChunk as ::core::ffi::c_longlong
                - 1 as ::core::ffi::c_longlong)
                / (*pFile).szChunk as ::core::ffi::c_longlong
                * (*pFile).szChunk as ::core::ffi::c_longlong) as i64_0;
        }
        rc = robust_ftruncate((*pFile).h, nByte as sqlite3_int64);
        if rc != 0 {
            storeLastErrno(pFile, *__errno_location());
            return unixLogErrorAtLine(
                10 as ::core::ffi::c_int
                    | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                b"ftruncate\0".as_ptr() as *const ::core::ffi::c_char,
                (*pFile).zPath,
                3979 as ::core::ffi::c_int,
            );
        } else {
            if nByte < (*pFile).mmapSize {
                (*pFile).mmapSize = nByte as sqlite3_int64;
            }
            return SQLITE_OK;
        };
    }
}
unsafe extern "C" fn unixFileSize(
    mut id: *mut sqlite3_file,
    mut pSize: *mut i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3]

        };
        rc = ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(::core::ffi::c_int, *mut stat) -> ::core::ffi::c_int,
            >,
        >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
            .expect(
                "non-null function pointer",
            )((*(id as *mut unixFile)).h, &raw mut buf);
        if sqlite3_io_error_persist != 0 && sqlite3_io_error_hit != 0
            || {
                let c2rust_fresh13 = sqlite3_io_error_pending;
                sqlite3_io_error_pending = sqlite3_io_error_pending - 1;
                c2rust_fresh13 == 1 as ::core::ffi::c_int
            }
        {
            local_ioerr();
            rc = 1 as ::core::ffi::c_int;
        }
        if rc != 0 as ::core::ffi::c_int {
            storeLastErrno(id as *mut unixFile, *__errno_location());
            return SQLITE_IOERR_FSTAT;
        }
        *pSize = buf.st_size as i64_0;
        if *pSize == 1 as ::core::ffi::c_longlong {
            *pSize = 0 as i64_0;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fcntlSizeHint(
    mut pFile: *mut unixFile,
    mut nByte: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pFile).szChunk > 0 as ::core::ffi::c_int {
            let mut nSize: i64_0 = 0;
            let mut buf: stat = stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                __glibc_reserved: [0; 3]

            };
            if ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")((*pFile).h, &raw mut buf) != 0
            {
                return SQLITE_IOERR_FSTAT;
            }
            nSize = ((nByte as ::core::ffi::c_longlong
                + (*pFile).szChunk as ::core::ffi::c_longlong
                - 1 as ::core::ffi::c_longlong)
                / (*pFile).szChunk as ::core::ffi::c_longlong
                * (*pFile).szChunk as ::core::ffi::c_longlong) as i64_0;
            if nSize > buf.st_size as i64_0 {
                let mut nBlk: ::core::ffi::c_int = buf.st_blksize as ::core::ffi::c_int;
                let mut nWrite: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut iWrite: i64_0 = 0;
                iWrite = (buf.st_size as ::core::ffi::c_long
                    / nBlk as ::core::ffi::c_long * nBlk as ::core::ffi::c_long
                    + nBlk as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as i64_0;
                while iWrite
                    < nSize as ::core::ffi::c_longlong + nBlk as ::core::ffi::c_longlong
                        - 1 as ::core::ffi::c_longlong
                {
                    if iWrite >= nSize {
                        iWrite = (nSize as ::core::ffi::c_longlong
                            - 1 as ::core::ffi::c_longlong) as i64_0;
                    }
                    nWrite = seekAndWrite(
                        pFile,
                        iWrite,
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        1 as ::core::ffi::c_int,
                    );
                    if nWrite != 1 as ::core::ffi::c_int {
                        return SQLITE_IOERR_WRITE;
                    }
                    iWrite += nBlk as ::core::ffi::c_longlong;
                }
            }
        }
        if (*pFile).mmapSizeMax > 0 as ::core::ffi::c_longlong
            && nByte > (*pFile).mmapSize
        {
            let mut rc: ::core::ffi::c_int = 0;
            if (*pFile).szChunk <= 0 as ::core::ffi::c_int {
                if robust_ftruncate((*pFile).h, nByte as sqlite3_int64) != 0 {
                    storeLastErrno(pFile, *__errno_location());
                    return unixLogErrorAtLine(
                        10 as ::core::ffi::c_int
                            | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                        b"ftruncate\0".as_ptr() as *const ::core::ffi::c_char,
                        (*pFile).zPath,
                        4100 as ::core::ffi::c_int,
                    );
                }
            }
            rc = unixMapfile(pFile, nByte);
            return rc;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn unixModeBit(
    mut pFile: *mut unixFile,
    mut mask: ::core::ffi::c_uchar,
    mut pArg: *mut ::core::ffi::c_int,
) {
    unsafe {
        if *pArg < 0 as ::core::ffi::c_int {
            *pArg = ((*pFile).ctrlFlags as ::core::ffi::c_int
                & mask as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        } else if *pArg == 0 as ::core::ffi::c_int {
            (*pFile).ctrlFlags = ((*pFile).ctrlFlags as ::core::ffi::c_int
                & !(mask as ::core::ffi::c_int)) as ::core::ffi::c_ushort;
        } else {
            (*pFile).ctrlFlags = ((*pFile).ctrlFlags as ::core::ffi::c_int
                | mask as ::core::ffi::c_int) as ::core::ffi::c_ushort;
        };
    }
}
unsafe extern "C" fn unixFileControl(
    mut id: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFile: *mut unixFile = id as *mut unixFile;
        match op {
            SQLITE_FCNTL_NULL_IO => {
                ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[1 as ::core::ffi::c_int as usize].pCurrent)
                    .expect("non-null function pointer")((*pFile).h);
                (*pFile).h = -1 as ::core::ffi::c_int;
                return SQLITE_OK;
            }
            SQLITE_FCNTL_LOCKSTATE => {
                *(pArg as *mut ::core::ffi::c_int) = (*pFile).eFileLock
                    as ::core::ffi::c_int;
                return SQLITE_OK;
            }
            SQLITE_FCNTL_LAST_ERRNO => {
                *(pArg as *mut ::core::ffi::c_int) = (*pFile).lastErrno;
                return SQLITE_OK;
            }
            SQLITE_FCNTL_CHUNK_SIZE => {
                (*pFile).szChunk = *(pArg as *mut ::core::ffi::c_int);
                return SQLITE_OK;
            }
            SQLITE_FCNTL_SIZE_HINT => {
                let mut rc: ::core::ffi::c_int = 0;
                sqlite3_io_error_benign = 1 as ::core::ffi::c_int;
                rc = fcntlSizeHint(pFile, *(pArg as *mut i64_0));
                sqlite3_io_error_benign = 0 as ::core::ffi::c_int;
                return rc;
            }
            SQLITE_FCNTL_PERSIST_WAL => {
                unixModeBit(
                    pFile,
                    UNIXFILE_PERSIST_WAL as ::core::ffi::c_uchar,
                    pArg as *mut ::core::ffi::c_int,
                );
                return SQLITE_OK;
            }
            SQLITE_FCNTL_POWERSAFE_OVERWRITE => {
                unixModeBit(
                    pFile,
                    UNIXFILE_PSOW as ::core::ffi::c_uchar,
                    pArg as *mut ::core::ffi::c_int,
                );
                return SQLITE_OK;
            }
            SQLITE_FCNTL_VFSNAME => {
                let ref mut c2rust_fresh7 = *(pArg as *mut *mut ::core::ffi::c_char);
                *c2rust_fresh7 = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*(*pFile).pVfs).zName,
                );
                return SQLITE_OK;
            }
            SQLITE_FCNTL_TEMPFILENAME => {
                let mut zTFile: *mut ::core::ffi::c_char = sqlite3_malloc64(
                    (*(*pFile).pVfs).mxPathname as sqlite3_uint64,
                ) as *mut ::core::ffi::c_char;
                if !zTFile.is_null() {
                    unixGetTempname((*(*pFile).pVfs).mxPathname, zTFile);
                    let ref mut c2rust_fresh8 = *(pArg as *mut *mut ::core::ffi::c_char);
                    *c2rust_fresh8 = zTFile;
                }
                return SQLITE_OK;
            }
            SQLITE_FCNTL_HAS_MOVED => {
                *(pArg as *mut ::core::ffi::c_int) = fileHasMoved(pFile);
                return SQLITE_OK;
            }
            SQLITE_FCNTL_MMAP_SIZE => {
                let mut newLimit: i64_0 = *(pArg as *mut i64_0);
                let mut rc_0: ::core::ffi::c_int = SQLITE_OK;
                if newLimit > sqlite3Config.mxMmap {
                    newLimit = sqlite3Config.mxMmap as i64_0;
                }
                if newLimit > 0 as ::core::ffi::c_longlong
                    && (::core::mem::size_of::<size_t>() as usize) < 8 as usize
                {
                    newLimit = (newLimit as ::core::ffi::c_longlong
                        & 0x7fffffff as ::core::ffi::c_longlong) as i64_0;
                }
                *(pArg as *mut i64_0) = (*pFile).mmapSizeMax as i64_0;
                if newLimit >= 0 as ::core::ffi::c_longlong
                    && newLimit != (*pFile).mmapSizeMax
                    && (*pFile).nFetchOut == 0 as ::core::ffi::c_int
                {
                    (*pFile).mmapSizeMax = newLimit as sqlite3_int64;
                    if (*pFile).mmapSize > 0 as ::core::ffi::c_longlong {
                        unixUnmapfile(pFile);
                        rc_0 = unixMapfile(pFile, -1 as ::core::ffi::c_int as i64_0);
                    }
                }
                return rc_0;
            }
            SQLITE_FCNTL_EXTERNAL_READER => {
                return unixFcntlExternalReader(
                    id as *mut unixFile,
                    pArg as *mut ::core::ffi::c_int,
                );
            }
            _ => {}
        }
        return SQLITE_NOTFOUND;
    }
}
unsafe extern "C" fn setDeviceCharacteristics(mut pFd: *mut unixFile) {
    unsafe {
        if (*pFd).sectorSize == 0 as ::core::ffi::c_int {
            if (*pFd).ctrlFlags as ::core::ffi::c_int & UNIXFILE_PSOW != 0 {
                (*pFd).deviceCharacteristics |= SQLITE_IOCAP_POWERSAFE_OVERWRITE;
            }
            (*pFd).deviceCharacteristics |= SQLITE_IOCAP_SUBPAGE_READ;
            (*pFd).sectorSize = SQLITE_DEFAULT_SECTOR_SIZE;
        }
    }
}
unsafe extern "C" fn unixSectorSize(mut id: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut pFd: *mut unixFile = id as *mut unixFile;
        setDeviceCharacteristics(pFd);
        return (*pFd).sectorSize;
    }
}
unsafe extern "C" fn unixDeviceCharacteristics(
    mut id: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFd: *mut unixFile = id as *mut unixFile;
        setDeviceCharacteristics(pFd);
        return (*pFd).deviceCharacteristics;
    }
}
unsafe extern "C" fn unixGetpagesize() -> ::core::ffi::c_int {
    unsafe {
        return sysconf(_SC_PAGESIZE as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
pub const UNIX_SHM_BASE: ::core::ffi::c_int = (22 as ::core::ffi::c_int
    + SQLITE_SHM_NLOCK) * 4 as ::core::ffi::c_int;
pub const UNIX_SHM_DMS: ::core::ffi::c_int = UNIX_SHM_BASE + SQLITE_SHM_NLOCK;
unsafe extern "C" fn unixFcntlExternalReader(
    mut pFile: *mut unixFile,
    mut piOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        *piOut = 0 as ::core::ffi::c_int;
        if !(*pFile).pShm.is_null() {
            let mut pShmNode: *mut unixShmNode = (*(*pFile).pShm).pShmNode;
            let mut f: flock = flock {
                l_type: 0,
                l_whence: 0,
                l_start: 0,
                l_len: 0,
                l_pid: 0

            };
            memset(
                &raw mut f as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<flock>() as size_t,
            );
            f.l_type = F_WRLCK as ::core::ffi::c_short;
            f.l_whence = SEEK_SET as ::core::ffi::c_short;
            f.l_start = (UNIX_SHM_BASE + 3 as ::core::ffi::c_int) as __off64_t;
            f.l_len = (SQLITE_SHM_NLOCK - 3 as ::core::ffi::c_int) as __off64_t;
            sqlite3_mutex_enter((*pShmNode).pShmMutex);
            if ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ...
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
                .expect(
                    "non-null function pointer",
                )((*pShmNode).hShm, F_GETLK, &raw mut f) < 0 as ::core::ffi::c_int
            {
                rc = SQLITE_IOERR_LOCK;
            } else {
                *piOut = (f.l_type as ::core::ffi::c_int != F_UNLCK)
                    as ::core::ffi::c_int;
            }
            sqlite3_mutex_leave((*pShmNode).pShmMutex);
        }
        return rc;
    }
}
unsafe extern "C" fn unixIsSharingShmNode(
    mut pFile: *mut unixFile,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pShmNode: *mut unixShmNode = ::core::ptr::null_mut::<unixShmNode>();
        let mut lock: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0

        };
        if (*pFile).pShm.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*pFile).ctrlFlags as ::core::ffi::c_int & UNIXFILE_EXCL != 0 {
            return 0 as ::core::ffi::c_int;
        }
        pShmNode = (*(*pFile).pShm).pShmNode;
        memset(
            &raw mut lock as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<flock>() as size_t,
        );
        lock.l_whence = SEEK_SET as ::core::ffi::c_short;
        lock.l_start = UNIX_SHM_DMS as __off64_t;
        lock.l_len = 1 as __off64_t;
        lock.l_type = F_WRLCK as ::core::ffi::c_short;
        ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ...
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
            .expect(
                "non-null function pointer",
            )((*pShmNode).hShm, F_GETLK, &raw mut lock);
        return (lock.l_type as ::core::ffi::c_int != F_UNLCK) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn unixShmSystemLock(
    mut pFile: *mut unixFile,
    mut lockType: ::core::ffi::c_int,
    mut ofst: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pShmNode: *mut unixShmNode = ::core::ptr::null_mut::<unixShmNode>();
        let mut f: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0

        };
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        pShmNode = (*(*pFile).pInode).pShmNode;
        ofst == UNIX_SHM_DMS;
        if (*pShmNode).hShm >= 0 as ::core::ffi::c_int {
            let mut res: ::core::ffi::c_int = 0;
            f.l_type = lockType as ::core::ffi::c_short;
            f.l_whence = SEEK_SET as ::core::ffi::c_short;
            f.l_start = ofst as __off64_t;
            f.l_len = n as __off64_t;
            res = ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ...
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
                .expect(
                    "non-null function pointer",
                )((*pShmNode).hShm, F_SETLK, &raw mut f);
            if res == -1 as ::core::ffi::c_int {
                rc = SQLITE_BUSY;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn unixShmRegionPerMap() -> ::core::ffi::c_int {
    unsafe {
        let mut shmsz: ::core::ffi::c_int = 32 as ::core::ffi::c_int
            * 1024 as ::core::ffi::c_int;
        let mut pgsz: ::core::ffi::c_int = ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(aSyscall[25 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")();
        if pgsz < shmsz {
            return 1 as ::core::ffi::c_int;
        }
        return pgsz / shmsz;
    }
}
unsafe extern "C" fn unixShmPurge(mut pFd: *mut unixFile) {
    unsafe {
        let mut p: *mut unixShmNode = (*(*pFd).pInode).pShmNode;
        if !p.is_null() && (*p).nRef == 0 as ::core::ffi::c_int {
            let mut nShmPerMap: ::core::ffi::c_int = unixShmRegionPerMap();
            let mut i: ::core::ffi::c_int = 0;
            sqlite3_mutex_free((*p).pShmMutex);
            i = 0 as ::core::ffi::c_int;
            while i < (*p).nRegion as ::core::ffi::c_int {
                if (*p).hShm >= 0 as ::core::ffi::c_int {
                    ::core::mem::transmute::<
                        sqlite3_syscall_ptr,
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                size_t,
                            ) -> ::core::ffi::c_int,
                        >,
                    >(aSyscall[23 as ::core::ffi::c_int as usize].pCurrent)
                        .expect(
                            "non-null function pointer",
                        )(
                        *(*p).apRegion.offset(i as isize) as *mut ::core::ffi::c_void,
                        (*p).szRegion as size_t,
                    );
                } else {
                    sqlite3_free(
                        *(*p).apRegion.offset(i as isize) as *mut ::core::ffi::c_void,
                    );
                }
                i += nShmPerMap;
            }
            sqlite3_free((*p).apRegion as *mut ::core::ffi::c_void);
            if (*p).hShm >= 0 as ::core::ffi::c_int {
                robust_close(pFd, (*p).hShm, 4833 as ::core::ffi::c_int);
                (*p).hShm = -1 as ::core::ffi::c_int;
            }
            (*(*p).pInode).pShmNode = ::core::ptr::null_mut::<unixShmNode>();
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn unixLockSharedMemory(
    mut pDbFd: *mut unixFile,
    mut pShmNode: *mut unixShmNode,
) -> ::core::ffi::c_int {
    unsafe {
        let mut lock: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0

        };
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        lock.l_whence = SEEK_SET as ::core::ffi::c_short;
        lock.l_start = UNIX_SHM_DMS as __off64_t;
        lock.l_len = 1 as __off64_t;
        lock.l_type = F_WRLCK as ::core::ffi::c_short;
        if ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ...
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
            .expect(
                "non-null function pointer",
            )((*pShmNode).hShm, F_GETLK, &raw mut lock) != 0 as ::core::ffi::c_int
        {
            rc = SQLITE_IOERR_LOCK;
        } else if lock.l_type as ::core::ffi::c_int == F_UNLCK {
            if (*pShmNode).isReadonly != 0 {
                (*pShmNode).isUnlocked = 1 as u8_0;
                rc = SQLITE_READONLY_CANTINIT;
            } else {
                rc = unixShmSystemLock(
                    pDbFd,
                    F_WRLCK,
                    UNIX_SHM_DMS,
                    1 as ::core::ffi::c_int,
                );
                if rc == SQLITE_OK
                    && robust_ftruncate((*pShmNode).hShm, 3 as sqlite3_int64) != 0
                {
                    rc = unixLogErrorAtLine(
                        10 as ::core::ffi::c_int
                            | (18 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                        b"ftruncate\0".as_ptr() as *const ::core::ffi::c_char,
                        (*pShmNode).zFilename,
                        4903 as ::core::ffi::c_int,
                    );
                }
            }
        } else if lock.l_type as ::core::ffi::c_int == F_WRLCK {
            rc = SQLITE_BUSY;
        }
        if rc == SQLITE_OK {
            rc = unixShmSystemLock(
                pDbFd,
                F_RDLCK,
                UNIX_SHM_DMS,
                1 as ::core::ffi::c_int,
            );
        }
        return rc;
    }
}
unsafe extern "C" fn unixOpenSharedMemory(
    mut pDbFd: *mut unixFile,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut p: *mut unixShm = ::core::ptr::null_mut::<unixShm>();
        let mut pShmNode: *mut unixShmNode = ::core::ptr::null_mut::<unixShmNode>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pInode: *mut unixInodeInfo = ::core::ptr::null_mut::<unixInodeInfo>();
        let mut zShm: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nShmFilename: ::core::ffi::c_int = 0;
        p = sqlite3_malloc64(::core::mem::size_of::<unixShm>() as sqlite3_uint64)
            as *mut unixShm;
        if p.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<unixShm>() as size_t,
        );
        unixEnterMutex();
        pInode = (*pDbFd).pInode;
        pShmNode = (*pInode).pShmNode as *mut unixShmNode;
        if pShmNode.is_null() {
            let mut sStat: stat = stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                __glibc_reserved: [0; 3]

            };
            let mut zBasePath: *const ::core::ffi::c_char = (*pDbFd).zPath;
            if ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")((*pDbFd).h, &raw mut sStat) != 0
            {
                rc = SQLITE_IOERR_FSTAT;
                c2rust_current_block = 1647493064508935385;
            } else {
                nShmFilename = 6 as ::core::ffi::c_int
                    + strlen(zBasePath) as ::core::ffi::c_int;
                pShmNode = sqlite3_malloc64(
                    (::core::mem::size_of::<unixShmNode>() as usize)
                        .wrapping_add(nShmFilename as usize) as sqlite3_uint64,
                ) as *mut unixShmNode;
                if pShmNode.is_null() {
                    rc = SQLITE_NOMEM_BKPT;
                    c2rust_current_block = 1647493064508935385;
                } else {
                    memset(
                        pShmNode as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (::core::mem::size_of::<unixShmNode>() as size_t)
                            .wrapping_add(nShmFilename as size_t),
                    );
                    (*pShmNode).zFilename = pShmNode
                        .offset(1 as ::core::ffi::c_int as isize) as *mut unixShmNode
                        as *mut ::core::ffi::c_char;
                    zShm = (*pShmNode).zFilename;
                    sqlite3_snprintf(
                        nShmFilename,
                        zShm,
                        b"%s-shm\0".as_ptr() as *const ::core::ffi::c_char,
                        zBasePath,
                    );
                    (*pShmNode).hShm = -1 as ::core::ffi::c_int;
                    (*(*pDbFd).pInode).pShmNode = pShmNode as *mut unixShmNode;
                    (*pShmNode).pInode = (*pDbFd).pInode;
                    if sqlite3Config.bCoreMutex != 0 {
                        (*pShmNode).pShmMutex = sqlite3_mutex_alloc(SQLITE_MUTEX_FAST);
                        if (*pShmNode).pShmMutex.is_null() {
                            rc = SQLITE_NOMEM_BKPT;
                            c2rust_current_block = 1647493064508935385;
                        } else {
                            c2rust_current_block = 14576567515993809846;
                        }
                    } else {
                        c2rust_current_block = 14576567515993809846;
                    }
                    match c2rust_current_block {
                        1647493064508935385 => {}
                        _ => {
                            if (*pInode).bProcessLock as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                if 0 as ::core::ffi::c_int
                                    == sqlite3_uri_boolean(
                                        (*pDbFd).zPath as sqlite3_filename,
                                        b"readonly_shm\0".as_ptr() as *const ::core::ffi::c_char,
                                        0 as ::core::ffi::c_int,
                                    )
                                {
                                    (*pShmNode).hShm = robust_open(
                                        zShm,
                                        O_RDWR | O_CREAT | O_NOFOLLOW,
                                        sStat.st_mode as mode_t & 0o777 as mode_t,
                                    );
                                }
                                if (*pShmNode).hShm < 0 as ::core::ffi::c_int {
                                    (*pShmNode).hShm = robust_open(
                                        zShm,
                                        O_RDONLY | O_NOFOLLOW,
                                        sStat.st_mode as mode_t & 0o777 as mode_t,
                                    );
                                    if (*pShmNode).hShm < 0 as ::core::ffi::c_int {
                                        rc = unixLogErrorAtLine(
                                            sqlite3CantopenError(5040 as ::core::ffi::c_int),
                                            b"open\0".as_ptr() as *const ::core::ffi::c_char,
                                            zShm,
                                            5040 as ::core::ffi::c_int,
                                        );
                                        c2rust_current_block = 1647493064508935385;
                                    } else {
                                        (*pShmNode).isReadonly = 1 as u8_0;
                                        c2rust_current_block = 11932355480408055363;
                                    }
                                } else {
                                    c2rust_current_block = 11932355480408055363;
                                }
                                match c2rust_current_block {
                                    1647493064508935385 => {}
                                    _ => {
                                        robustFchown(
                                            (*pShmNode).hShm,
                                            sStat.st_uid as uid_t,
                                            sStat.st_gid as gid_t,
                                        );
                                        rc = unixLockSharedMemory(
                                            pDbFd,
                                            pShmNode as *mut unixShmNode,
                                        );
                                        if rc != SQLITE_OK && rc != SQLITE_READONLY_CANTINIT {
                                            c2rust_current_block = 1647493064508935385;
                                        } else {
                                            c2rust_current_block = 14434620278749266018;
                                        }
                                    }
                                }
                            } else {
                                c2rust_current_block = 14434620278749266018;
                            }
                        }
                    }
                }
            }
            match c2rust_current_block {
                14434620278749266018 => {}
                _ => {
                    unixShmPurge(pDbFd);
                    sqlite3_free(p as *mut ::core::ffi::c_void);
                    unixLeaveMutex();
                    return rc;
                }
            }
        }
        (*p).pShmNode = pShmNode as *mut unixShmNode;
        (*pShmNode).nRef += 1;
        (*pDbFd).pShm = p as *mut unixShm;
        unixLeaveMutex();
        sqlite3_mutex_enter((*pShmNode).pShmMutex);
        (*p).pNext = (*pShmNode).pFirst;
        (*pShmNode).pFirst = p as *mut unixShm;
        sqlite3_mutex_leave((*pShmNode).pShmMutex);
        return rc;
    }
}
unsafe extern "C" fn unixShmMap(
    mut fd: *mut sqlite3_file,
    mut iRegion: ::core::ffi::c_int,
    mut szRegion: ::core::ffi::c_int,
    mut bExtend: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pDbFd: *mut unixFile = fd as *mut unixFile;
        let mut p: *mut unixShm = ::core::ptr::null_mut::<unixShm>();
        let mut pShmNode: *mut unixShmNode = ::core::ptr::null_mut::<unixShmNode>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nShmPerMap: ::core::ffi::c_int = unixShmRegionPerMap();
        let mut nReqRegion: ::core::ffi::c_int = 0;
        if (*pDbFd).pShm.is_null() {
            rc = unixOpenSharedMemory(pDbFd);
            if rc != SQLITE_OK {
                return rc;
            }
        }
        p = (*pDbFd).pShm;
        pShmNode = (*p).pShmNode;
        sqlite3_mutex_enter((*pShmNode).pShmMutex);
        if (*pShmNode).isUnlocked != 0 {
            rc = unixLockSharedMemory(pDbFd, pShmNode);
            if rc != SQLITE_OK {
                c2rust_current_block = 11659155799710607607;
            } else {
                (*pShmNode).isUnlocked = 0 as u8_0;
                c2rust_current_block = 4166486009154926805;
            }
        } else {
            c2rust_current_block = 4166486009154926805;
        }
        match c2rust_current_block {
            4166486009154926805 => {
                nReqRegion = (iRegion + nShmPerMap) / nShmPerMap * nShmPerMap;
                if ((*pShmNode).nRegion as ::core::ffi::c_int) < nReqRegion {
                    let mut apNew: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        *mut ::core::ffi::c_char,
                    >();
                    let mut nByte: ::core::ffi::c_int = nReqRegion * szRegion;
                    let mut sStat: stat = stat {
                        st_dev: 0,
                        st_ino: 0,
                        st_nlink: 0,
                        st_mode: 0,
                        st_uid: 0,
                        st_gid: 0,
                        __pad0: 0,
                        st_rdev: 0,
                        st_size: 0,
                        st_blksize: 0,
                        st_blocks: 0,
                        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                        __glibc_reserved: [0; 3]

                    };
                    (*pShmNode).szRegion = szRegion;
                    if (*pShmNode).hShm >= 0 as ::core::ffi::c_int {
                        if ::core::mem::transmute::<
                            sqlite3_syscall_ptr,
                            Option<
                                unsafe extern "C" fn(
                                    ::core::ffi::c_int,
                                    *mut stat,
                                ) -> ::core::ffi::c_int,
                            >,
                        >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
                            .expect(
                                "non-null function pointer",
                            )((*pShmNode).hShm, &raw mut sStat) != 0
                        {
                            rc = SQLITE_IOERR_SHMSIZE;
                            c2rust_current_block = 11659155799710607607;
                        } else if sStat.st_size < nByte as ::core::ffi::c_long {
                            if bExtend == 0 {
                                c2rust_current_block = 11659155799710607607;
                            } else {
                                static mut pgsz: ::core::ffi::c_int = 4096
                                    as ::core::ffi::c_int;
                                let mut iPg: ::core::ffi::c_int = 0;
                                iPg = (sStat.st_size as ::core::ffi::c_long
                                    / pgsz as ::core::ffi::c_long) as ::core::ffi::c_int;
                                loop {
                                    if !(iPg < nByte / pgsz) {
                                        c2rust_current_block = 8693738493027456495;
                                        break;
                                    }
                                    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    if seekAndWriteFd(
                                        (*pShmNode).hShm,
                                        (iPg * pgsz + pgsz - 1 as ::core::ffi::c_int) as i64_0,
                                        b"\0".as_ptr() as *const ::core::ffi::c_char
                                            as *const ::core::ffi::c_void,
                                        1 as ::core::ffi::c_int,
                                        &raw mut x,
                                    ) != 1 as ::core::ffi::c_int
                                    {
                                        let mut zFile: *const ::core::ffi::c_char = (*pShmNode)
                                            .zFilename;
                                        rc = unixLogErrorAtLine(
                                            10 as ::core::ffi::c_int
                                                | (19 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                                            b"write\0".as_ptr() as *const ::core::ffi::c_char,
                                            zFile,
                                            5184 as ::core::ffi::c_int,
                                        );
                                        c2rust_current_block = 11659155799710607607;
                                        break;
                                    } else {
                                        iPg += 1;
                                    }
                                }
                            }
                        } else {
                            c2rust_current_block = 8693738493027456495;
                        }
                    } else {
                        c2rust_current_block = 8693738493027456495;
                    }
                    match c2rust_current_block {
                        11659155799710607607 => {}
                        _ => {
                            apNew = sqlite3_realloc(
                                (*pShmNode).apRegion as *mut ::core::ffi::c_void,
                                (nReqRegion as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                                    ) as ::core::ffi::c_int,
                            ) as *mut *mut ::core::ffi::c_char;
                            if apNew.is_null() {
                                rc = SQLITE_IOERR_NOMEM_BKPT;
                            } else {
                                (*pShmNode).apRegion = apNew;
                                while ((*pShmNode).nRegion as ::core::ffi::c_int)
                                    < nReqRegion
                                {
                                    let mut nMap: ::core::ffi::c_int = szRegion * nShmPerMap;
                                    let mut i: ::core::ffi::c_int = 0;
                                    let mut pMem: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                                        ::core::ffi::c_void,
                                    >();
                                    if (*pShmNode).hShm >= 0 as ::core::ffi::c_int {
                                        pMem = ::core::mem::transmute::<
                                            sqlite3_syscall_ptr,
                                            Option<
                                                unsafe extern "C" fn(
                                                    *mut ::core::ffi::c_void,
                                                    size_t,
                                                    ::core::ffi::c_int,
                                                    ::core::ffi::c_int,
                                                    ::core::ffi::c_int,
                                                    off_t,
                                                ) -> *mut ::core::ffi::c_void,
                                            >,
                                        >(aSyscall[22 as ::core::ffi::c_int as usize].pCurrent)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                            nMap as size_t,
                                            if (*pShmNode).isReadonly as ::core::ffi::c_int != 0 {
                                                PROT_READ
                                            } else {
                                                PROT_READ | PROT_WRITE
                                            },
                                            MAP_SHARED,
                                            (*pShmNode).hShm,
                                            (szRegion as i64_0 * (*pShmNode).nRegion as i64_0) as off_t,
                                        );
                                        if pMem == MAP_FAILED {
                                            rc = unixLogErrorAtLine(
                                                10 as ::core::ffi::c_int
                                                    | (21 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                                                b"mmap\0".as_ptr() as *const ::core::ffi::c_char,
                                                (*pShmNode).zFilename,
                                                5211 as ::core::ffi::c_int,
                                            );
                                            break;
                                        }
                                    } else {
                                        pMem = sqlite3_malloc64(nMap as sqlite3_uint64);
                                        if pMem.is_null() {
                                            rc = SQLITE_NOMEM_BKPT;
                                            break;
                                        } else {
                                            memset(pMem, 0 as ::core::ffi::c_int, nMap as size_t);
                                        }
                                    }
                                    i = 0 as ::core::ffi::c_int;
                                    while i < nShmPerMap {
                                        let ref mut c2rust_fresh18 = *(*pShmNode)
                                            .apRegion
                                            .offset(
                                                ((*pShmNode).nRegion as ::core::ffi::c_int + i) as isize,
                                            );
                                        *c2rust_fresh18 = (pMem as *mut ::core::ffi::c_char)
                                            .offset((szRegion * i) as isize)
                                            as *mut ::core::ffi::c_char;
                                        i += 1;
                                    }
                                    (*pShmNode).nRegion = ((*pShmNode).nRegion
                                        as ::core::ffi::c_int + nShmPerMap) as u16_0;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        if (*pShmNode).nRegion as ::core::ffi::c_int > iRegion {
            *pp = *(*pShmNode).apRegion.offset(iRegion as isize)
                as *mut ::core::ffi::c_void;
        } else {
            *pp = ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        if (*pShmNode).isReadonly as ::core::ffi::c_int != 0 && rc == SQLITE_OK {
            rc = SQLITE_READONLY;
        }
        sqlite3_mutex_leave((*pShmNode).pShmMutex);
        return rc;
    }
}
unsafe extern "C" fn unixShmLock(
    mut fd: *mut sqlite3_file,
    mut ofst: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pDbFd: *mut unixFile = fd as *mut unixFile;
        let mut p: *mut unixShm = ::core::ptr::null_mut::<unixShm>();
        let mut pShmNode: *mut unixShmNode = ::core::ptr::null_mut::<unixShmNode>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut mask: u16_0 = (((1 as ::core::ffi::c_int) << ofst + n)
            - ((1 as ::core::ffi::c_int) << ofst)) as u16_0;
        let mut aLock: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        p = (*pDbFd).pShm;
        if p.is_null() {
            return SQLITE_IOERR_SHMLOCK;
        }
        pShmNode = (*p).pShmNode;
        if pShmNode.is_null() {
            return SQLITE_IOERR_SHMLOCK;
        }
        aLock = &raw mut (*pShmNode).aLock as *mut ::core::ffi::c_int;
        if flags & SQLITE_SHM_UNLOCK != 0
            && ((*p).exclMask as ::core::ffi::c_int
                | (*p).sharedMask as ::core::ffi::c_int) & mask as ::core::ffi::c_int
                != 0
            || flags == SQLITE_SHM_SHARED | SQLITE_SHM_LOCK
                && 0 as ::core::ffi::c_int
                    == (*p).sharedMask as ::core::ffi::c_int & mask as ::core::ffi::c_int
            || flags == SQLITE_SHM_EXCLUSIVE | SQLITE_SHM_LOCK
        {
            sqlite3_mutex_enter((*pShmNode).pShmMutex);
            if rc == 0 as ::core::ffi::c_int {
                if flags & SQLITE_SHM_UNLOCK != 0 {
                    let mut bUnlock: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    if flags & SQLITE_SHM_SHARED != 0 {
                        if *aLock.offset(ofst as isize) > 1 as ::core::ffi::c_int {
                            bUnlock = 0 as ::core::ffi::c_int;
                            let ref mut c2rust_fresh5 = *aLock.offset(ofst as isize);
                            *c2rust_fresh5 -= 1;
                            (*p).sharedMask = ((*p).sharedMask as ::core::ffi::c_int
                                & !(mask as ::core::ffi::c_int)) as u16_0;
                        }
                    }
                    if bUnlock != 0 {
                        rc = unixShmSystemLock(pDbFd, F_UNLCK, ofst + UNIX_SHM_BASE, n);
                        if rc == SQLITE_OK {
                            memset(
                                aLock.offset(ofst as isize) as *mut ::core::ffi::c_int
                                    as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
                                    .wrapping_mul(n as size_t),
                            );
                            (*p).sharedMask = ((*p).sharedMask as ::core::ffi::c_int
                                & !(mask as ::core::ffi::c_int)) as u16_0;
                            (*p).exclMask = ((*p).exclMask as ::core::ffi::c_int
                                & !(mask as ::core::ffi::c_int)) as u16_0;
                        }
                    }
                } else if flags & SQLITE_SHM_SHARED != 0 {
                    if *aLock.offset(ofst as isize) < 0 as ::core::ffi::c_int {
                        rc = SQLITE_BUSY;
                    } else if *aLock.offset(ofst as isize) == 0 as ::core::ffi::c_int {
                        rc = unixShmSystemLock(pDbFd, F_RDLCK, ofst + UNIX_SHM_BASE, n);
                    }
                    if rc == SQLITE_OK {
                        (*p).sharedMask = ((*p).sharedMask as ::core::ffi::c_int
                            | mask as ::core::ffi::c_int) as u16_0;
                        let ref mut c2rust_fresh6 = *aLock.offset(ofst as isize);
                        *c2rust_fresh6 += 1;
                    }
                } else {
                    let mut ii: ::core::ffi::c_int = 0;
                    ii = ofst;
                    while ii < ofst + n {
                        if *aLock.offset(ii as isize) != 0 {
                            rc = SQLITE_BUSY;
                            break;
                        } else {
                            ii += 1;
                        }
                    }
                    if rc == SQLITE_OK {
                        rc = unixShmSystemLock(pDbFd, F_WRLCK, ofst + UNIX_SHM_BASE, n);
                        if rc == SQLITE_OK {
                            (*p).exclMask = ((*p).exclMask as ::core::ffi::c_int
                                | mask as ::core::ffi::c_int) as u16_0;
                            ii = ofst;
                            while ii < ofst + n {
                                *aLock.offset(ii as isize) = -1 as ::core::ffi::c_int;
                                ii += 1;
                            }
                        }
                    }
                }
            }
            sqlite3_mutex_leave((*pShmNode).pShmMutex);
        }
        if sqlite3OSTrace != 0 {
            sqlite3DebugPrintf(
                b"SHM-LOCK shmid-%d, pid-%d got %03x,%03x\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).id as ::core::ffi::c_int,
                getpid(),
                (*p).sharedMask as ::core::ffi::c_int,
                (*p).exclMask as ::core::ffi::c_int,
            );
        }
        return rc;
    }
}
unsafe extern "C" fn unixShmBarrier(mut fd: *mut sqlite3_file) {
    unsafe {
        sqlite3MemoryBarrier();
        unixEnterMutex();
        unixLeaveMutex();
    }
}
unsafe extern "C" fn unixShmUnmap(
    mut fd: *mut sqlite3_file,
    mut deleteFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut unixShm = ::core::ptr::null_mut::<unixShm>();
        let mut pShmNode: *mut unixShmNode = ::core::ptr::null_mut::<unixShmNode>();
        let mut pp: *mut *mut unixShm = ::core::ptr::null_mut::<*mut unixShm>();
        let mut pDbFd: *mut unixFile = ::core::ptr::null_mut::<unixFile>();
        pDbFd = fd as *mut unixFile;
        p = (*pDbFd).pShm;
        if p.is_null() {
            return SQLITE_OK;
        }
        pShmNode = (*p).pShmNode;
        sqlite3_mutex_enter((*pShmNode).pShmMutex);
        pp = &raw mut (*pShmNode).pFirst;
        while *pp != p {
            pp = &raw mut (**pp).pNext;
        }
        *pp = (*p).pNext;
        sqlite3_free(p as *mut ::core::ffi::c_void);
        (*pDbFd).pShm = ::core::ptr::null_mut::<unixShm>();
        sqlite3_mutex_leave((*pShmNode).pShmMutex);
        unixEnterMutex();
        (*pShmNode).nRef -= 1;
        if (*pShmNode).nRef == 0 as ::core::ffi::c_int {
            if deleteFlag != 0 && (*pShmNode).hShm >= 0 as ::core::ffi::c_int {
                ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[16 as ::core::ffi::c_int as usize].pCurrent)
                    .expect("non-null function pointer")((*pShmNode).zFilename);
            }
            unixShmPurge(pDbFd);
        }
        unixLeaveMutex();
        return SQLITE_OK;
    }
}
unsafe extern "C" fn unixUnmapfile(mut pFd: *mut unixFile) {
    unsafe {
        if !(*pFd).pMapRegion.is_null() {
            ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[23 as ::core::ffi::c_int as usize].pCurrent)
                .expect(
                    "non-null function pointer",
                )((*pFd).pMapRegion, (*pFd).mmapSizeActual as size_t);
            (*pFd).pMapRegion = ::core::ptr::null_mut::<::core::ffi::c_void>();
            (*pFd).mmapSize = 0 as sqlite3_int64;
            (*pFd).mmapSizeActual = 0 as sqlite3_int64;
        }
    }
}
unsafe extern "C" fn unixRemapfile(mut pFd: *mut unixFile, mut nNew: i64_0) {
    unsafe {
        let mut zErr: *const ::core::ffi::c_char = b"mmap\0".as_ptr()
            as *const ::core::ffi::c_char;
        let mut h: ::core::ffi::c_int = (*pFd).h;
        let mut pOrig: *mut u8_0 = (*pFd).pMapRegion as *mut u8_0;
        let mut nOrig: i64_0 = (*pFd).mmapSizeActual as i64_0;
        let mut pNew: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut flags: ::core::ffi::c_int = PROT_READ;
        if !pOrig.is_null() {
            let mut nReuse: i64_0 = (*pFd).mmapSize as i64_0;
            let mut pReq: *mut u8_0 = pOrig.offset(nReuse as isize) as *mut u8_0;
            if nReuse != nOrig {
                ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            size_t,
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[23 as ::core::ffi::c_int as usize].pCurrent)
                    .expect(
                        "non-null function pointer",
                    )(pReq as *mut ::core::ffi::c_void, (nOrig - nReuse) as size_t);
            }
            pNew = ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        size_t,
                        ::core::ffi::c_int,
                        ...
                    ) -> *mut ::core::ffi::c_void,
                >,
            >(aSyscall[24 as ::core::ffi::c_int as usize].pCurrent)
                .expect(
                    "non-null function pointer",
                )(
                pOrig as *mut ::core::ffi::c_void,
                nReuse as size_t,
                nNew as size_t,
                MREMAP_MAYMOVE,
            ) as *mut u8_0;
            zErr = b"mremap\0".as_ptr() as *const ::core::ffi::c_char;
            if pNew == MAP_FAILED as *mut u8_0 || pNew.is_null() {
                ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            size_t,
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[23 as ::core::ffi::c_int as usize].pCurrent)
                    .expect(
                        "non-null function pointer",
                    )(pOrig as *mut ::core::ffi::c_void, nReuse as size_t);
            }
        }
        if pNew.is_null() {
            pNew = ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        off_t,
                    ) -> *mut ::core::ffi::c_void,
                >,
            >(aSyscall[22 as ::core::ffi::c_int as usize].pCurrent)
                .expect(
                    "non-null function pointer",
                )(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                nNew as size_t,
                flags,
                MAP_SHARED,
                h,
                0 as off_t,
            ) as *mut u8_0;
        }
        if pNew == MAP_FAILED as *mut u8_0 {
            pNew = ::core::ptr::null_mut::<u8_0>();
            nNew = 0 as i64_0;
            unixLogErrorAtLine(
                0 as ::core::ffi::c_int,
                zErr,
                (*pFd).zPath,
                5650 as ::core::ffi::c_int,
            );
            (*pFd).mmapSizeMax = 0 as sqlite3_int64;
        }
        (*pFd).pMapRegion = pNew as *mut ::core::ffi::c_void;
        (*pFd).mmapSizeActual = nNew as sqlite3_int64;
        (*pFd).mmapSize = (*pFd).mmapSizeActual;
    }
}
unsafe extern "C" fn unixMapfile(
    mut pFd: *mut unixFile,
    mut nMap: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pFd).nFetchOut > 0 as ::core::ffi::c_int {
            return SQLITE_OK;
        }
        if nMap < 0 as ::core::ffi::c_longlong {
            let mut statbuf: stat = stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                __glibc_reserved: [0; 3]

            };
            if ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")((*pFd).h, &raw mut statbuf) != 0
            {
                return SQLITE_IOERR_FSTAT;
            }
            nMap = statbuf.st_size as i64_0;
        }
        if nMap > (*pFd).mmapSizeMax {
            nMap = (*pFd).mmapSizeMax as i64_0;
        }
        if nMap != (*pFd).mmapSize {
            unixRemapfile(pFd, nMap);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn unixFetch(
    mut fd: *mut sqlite3_file,
    mut iOff: i64_0,
    mut nAmt: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFd: *mut unixFile = fd as *mut unixFile;
        *pp = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if (*pFd).mmapSizeMax > 0 as ::core::ffi::c_longlong {
            let nEofBuffer: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
            if (*pFd).pMapRegion.is_null() {
                let mut rc: ::core::ffi::c_int = unixMapfile(
                    pFd,
                    -1 as ::core::ffi::c_int as i64_0,
                );
                if rc != SQLITE_OK {
                    return rc;
                }
            }
            if (*pFd).mmapSize
                >= iOff as ::core::ffi::c_longlong + nAmt as ::core::ffi::c_longlong
                    + nEofBuffer as ::core::ffi::c_longlong
            {
                *pp = ((*pFd).pMapRegion as *mut u8_0).offset(iOff as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void;
                (*pFd).nFetchOut += 1;
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn unixUnfetch(
    mut fd: *mut sqlite3_file,
    mut iOff: i64_0,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFd: *mut unixFile = fd as *mut unixFile;
        if !p.is_null() {
            (*pFd).nFetchOut -= 1;
        } else {
            unixUnmapfile(pFd);
        }
        return SQLITE_OK;
    }
}
static mut posixIoFinder: Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        *mut unixFile,
    ) -> *const sqlite3_io_methods
> = unsafe {
    Some(
        posixIoFinderImpl
            as unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *mut unixFile,
            ) -> *const sqlite3_io_methods,
    )
};
static mut posixIoMethods: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 3 as ::core::ffi::c_int,
        xClose: Some(
            unixClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            unixRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            unixWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            unixTruncate
                as unsafe extern "C" fn(*mut sqlite3_file, i64_0) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            unixSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            unixFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut i64_0,
                ) -> ::core::ffi::c_int,
        ),
        xLock: Some(
            unixLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xUnlock: Some(
            unixUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCheckReservedLock: Some(
            unixCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileControl: Some(
            unixFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xSectorSize: Some(
            unixSectorSize
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xDeviceCharacteristics: Some(
            unixDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xShmMap: Some(
            unixShmMap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xShmLock: Some(
            unixShmLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xShmBarrier: Some(
            unixShmBarrier as unsafe extern "C" fn(*mut sqlite3_file) -> (),
        ),
        xShmUnmap: Some(
            unixShmUnmap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFetch: Some(
            unixFetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    i64_0,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xUnfetch: Some(
            unixUnfetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    i64_0,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )

    }
};
unsafe extern "C" fn posixIoFinderImpl(
    mut z: *const ::core::ffi::c_char,
    mut p: *mut unixFile,
) -> *const sqlite3_io_methods {
    unsafe {
        return &raw const posixIoMethods;
    }
}
unsafe extern "C" fn nolockIoFinderImpl(
    mut z: *const ::core::ffi::c_char,
    mut p: *mut unixFile,
) -> *const sqlite3_io_methods {
    unsafe {
        return &raw const nolockIoMethods;
    }
}
static mut nolockIoMethods: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 3 as ::core::ffi::c_int,
        xClose: Some(
            nolockClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            unixRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            unixWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            unixTruncate
                as unsafe extern "C" fn(*mut sqlite3_file, i64_0) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            unixSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            unixFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut i64_0,
                ) -> ::core::ffi::c_int,
        ),
        xLock: Some(
            nolockLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xUnlock: Some(
            nolockUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCheckReservedLock: Some(
            nolockCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileControl: Some(
            unixFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xSectorSize: Some(
            unixSectorSize
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xDeviceCharacteristics: Some(
            unixDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xShmMap: None,
        xShmLock: Some(
            unixShmLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xShmBarrier: Some(
            unixShmBarrier as unsafe extern "C" fn(*mut sqlite3_file) -> (),
        ),
        xShmUnmap: Some(
            unixShmUnmap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFetch: Some(
            unixFetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    i64_0,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xUnfetch: Some(
            unixUnfetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    i64_0,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )

    }
};
static mut nolockIoFinder: Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        *mut unixFile,
    ) -> *const sqlite3_io_methods
> = unsafe {
    Some(
        nolockIoFinderImpl
            as unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *mut unixFile,
            ) -> *const sqlite3_io_methods,
    )
};
static mut dotlockIoMethods: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 1 as ::core::ffi::c_int,
        xClose: Some(
            dotlockClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            unixRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            unixWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            unixTruncate
                as unsafe extern "C" fn(*mut sqlite3_file, i64_0) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            unixSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            unixFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut i64_0,
                ) -> ::core::ffi::c_int,
        ),
        xLock: Some(
            dotlockLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xUnlock: Some(
            dotlockUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCheckReservedLock: Some(
            dotlockCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileControl: Some(
            unixFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xSectorSize: Some(
            unixSectorSize
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xDeviceCharacteristics: Some(
            unixDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xShmMap: None,
        xShmLock: Some(
            unixShmLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xShmBarrier: Some(
            unixShmBarrier as unsafe extern "C" fn(*mut sqlite3_file) -> (),
        ),
        xShmUnmap: Some(
            unixShmUnmap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFetch: Some(
            unixFetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    i64_0,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xUnfetch: Some(
            unixUnfetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    i64_0,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )

    }
};
static mut dotlockIoFinder: Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        *mut unixFile,
    ) -> *const sqlite3_io_methods
> = unsafe {
    Some(
        dotlockIoFinderImpl
            as unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *mut unixFile,
            ) -> *const sqlite3_io_methods,
    )
};
unsafe extern "C" fn dotlockIoFinderImpl(
    mut z: *const ::core::ffi::c_char,
    mut p: *mut unixFile,
) -> *const sqlite3_io_methods {
    unsafe {
        return &raw const dotlockIoMethods;
    }
}
unsafe extern "C" fn fillInUnixFile(
    mut pVfs: *mut sqlite3_vfs,
    mut h: ::core::ffi::c_int,
    mut pId: *mut sqlite3_file,
    mut zFilename: *const ::core::ffi::c_char,
    mut ctrlFlags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pLockingStyle: *const sqlite3_io_methods = ::core::ptr::null::<
            sqlite3_io_methods,
        >();
        let mut pNew: *mut unixFile = pId as *mut unixFile;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if sqlite3OSTrace != 0 {
            sqlite3DebugPrintf(
                b"OPEN    %-3d %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                h,
                zFilename,
            );
        }
        (*pNew).h = h;
        (*pNew).pVfs = pVfs;
        (*pNew).zPath = zFilename;
        (*pNew).ctrlFlags = ctrlFlags as u8_0 as ::core::ffi::c_ushort;
        (*pNew).mmapSizeMax = sqlite3Config.szMmap;
        if sqlite3_uri_boolean(
            if ctrlFlags & UNIXFILE_URI != 0 {
                zFilename as sqlite3_filename
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            },
            b"psow\0".as_ptr() as *const ::core::ffi::c_char,
            SQLITE_POWERSAFE_OVERWRITE,
        ) != 0
        {
            (*pNew).ctrlFlags = ((*pNew).ctrlFlags as ::core::ffi::c_int | UNIXFILE_PSOW)
                as ::core::ffi::c_ushort;
        }
        if strcmp((*pVfs).zName, b"unix-excl\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*pNew).ctrlFlags = ((*pNew).ctrlFlags as ::core::ffi::c_int | UNIXFILE_EXCL)
                as ::core::ffi::c_ushort;
        }
        if ctrlFlags & UNIXFILE_NOLOCK != 0 {
            pLockingStyle = &raw const nolockIoMethods;
        } else {
            pLockingStyle = Some(
                    (*((*pVfs).pAppData as *mut finder_type))
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(zFilename, pNew);
        }
        if pLockingStyle == &raw const posixIoMethods {
            unixEnterMutex();
            rc = findInodeInfo(pNew, &raw mut (*pNew).pInode);
            if rc != SQLITE_OK {
                robust_close(pNew, h, 6158 as ::core::ffi::c_int);
                h = -1 as ::core::ffi::c_int;
            }
            unixLeaveMutex();
        } else if pLockingStyle == &raw const dotlockIoMethods {
            let mut zLockFile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut nFilename: ::core::ffi::c_int = 0;
            nFilename = strlen(zFilename) as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int;
            zLockFile = sqlite3_malloc64(nFilename as sqlite3_uint64)
                as *mut ::core::ffi::c_char;
            if zLockFile.is_null() {
                rc = SQLITE_NOMEM_BKPT;
            } else {
                sqlite3_snprintf(
                    nFilename,
                    zLockFile,
                    b"%s.lock\0".as_ptr() as *const ::core::ffi::c_char,
                    zFilename,
                );
            }
            (*pNew).lockingContext = zLockFile as *mut ::core::ffi::c_void;
        }
        storeLastErrno(pNew, 0 as ::core::ffi::c_int);
        if rc != SQLITE_OK {
            if h >= 0 as ::core::ffi::c_int {
                robust_close(pNew, h, 6250 as ::core::ffi::c_int);
            }
        } else {
            (*pId).pMethods = pLockingStyle as *const sqlite3_io_methods;
            sqlite3_open_file_count += 1 as ::core::ffi::c_int;
            verifyDbFile(pNew);
        }
        return rc;
    }
}
static mut azTempDirs: [*const ::core::ffi::c_char; 6] = [
    ::core::ptr::null::<::core::ffi::c_char>(),
    ::core::ptr::null::<::core::ffi::c_char>(),
    b"/var/tmp\0".as_ptr() as *const ::core::ffi::c_char,
    b"/usr/tmp\0".as_ptr() as *const ::core::ffi::c_char,
    b"/tmp\0".as_ptr() as *const ::core::ffi::c_char,
    b".\0".as_ptr() as *const ::core::ffi::c_char,
];
unsafe extern "C" fn unixTempFileInit() {
    unsafe {
        azTempDirs[0 as ::core::ffi::c_int as usize] = getenv(
            b"SQLITE_TMPDIR\0".as_ptr() as *const ::core::ffi::c_char,
        );
        azTempDirs[1 as ::core::ffi::c_int as usize] = getenv(
            b"TMPDIR\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn unixTempFileDir() -> *const ::core::ffi::c_char {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3]

        };
        let mut zDir: *const ::core::ffi::c_char = sqlite3_temp_directory;
        loop {
            if !zDir.is_null()
                && ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *mut stat,
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[4 as ::core::ffi::c_int as usize].pCurrent)
                    .expect("non-null function pointer")(zDir, &raw mut buf)
                    == 0 as ::core::ffi::c_int
                && buf.st_mode as ::core::ffi::c_uint & __S_IFMT as ::core::ffi::c_uint
                    == 0o40000 as ::core::ffi::c_uint
                && ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[2 as ::core::ffi::c_int as usize].pCurrent)
                    .expect("non-null function pointer")(zDir, 0o3 as ::core::ffi::c_int)
                    == 0 as ::core::ffi::c_int
            {
                return zDir;
            }
            if i as usize
                >= (::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>() as usize)
                    .wrapping_div(
                        ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize,
                    )
            {
                break;
            }
            let c2rust_fresh11 = i;
            i = i.wrapping_add(1);
            zDir = azTempDirs[c2rust_fresh11 as usize];
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn unixGetTempname(
    mut nBuf: ::core::ffi::c_int,
    mut zBuf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zDir: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut iLimit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        *zBuf.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
        if sqlite3_io_error_persist != 0 && sqlite3_io_error_hit != 0
            || {
                let c2rust_fresh9 = sqlite3_io_error_pending;
                sqlite3_io_error_pending = sqlite3_io_error_pending - 1;
                c2rust_fresh9 == 1 as ::core::ffi::c_int
            }
        {
            local_ioerr();
            return 10 as ::core::ffi::c_int;
        }
        sqlite3_mutex_enter(sqlite3MutexAlloc(SQLITE_MUTEX_STATIC_TEMPDIR));
        zDir = unixTempFileDir();
        if zDir.is_null() {
            rc = SQLITE_IOERR_GETTEMPPATH;
        } else {
            loop {
                let mut r: u64_0 = 0;
                sqlite3_randomness(
                    ::core::mem::size_of::<u64_0>() as ::core::ffi::c_int,
                    &raw mut r as *mut ::core::ffi::c_void,
                );
                *zBuf.offset((nBuf - 2 as ::core::ffi::c_int) as isize) = 0
                    as ::core::ffi::c_char;
                sqlite3_snprintf(
                    nBuf,
                    zBuf,
                    b"%s/etilqs_%llx%c\0".as_ptr() as *const ::core::ffi::c_char,
                    zDir,
                    r,
                    0 as ::core::ffi::c_int,
                );
                if *zBuf.offset((nBuf - 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    || {
                        let c2rust_fresh10 = iLimit;
                        iLimit = iLimit + 1;
                        c2rust_fresh10 > 10 as ::core::ffi::c_int
                    }
                {
                    rc = SQLITE_ERROR;
                    break;
                } else if !(::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[2 as ::core::ffi::c_int as usize].pCurrent)
                    .expect("non-null function pointer")(zBuf, 0 as ::core::ffi::c_int)
                    == 0 as ::core::ffi::c_int)
                {
                    break;
                }
            }
        }
        sqlite3_mutex_leave(sqlite3MutexAlloc(SQLITE_MUTEX_STATIC_TEMPDIR));
        return rc;
    }
}
unsafe extern "C" fn findReusableFd(
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> *mut UnixUnusedFd {
    unsafe {
        let mut pUnused: *mut UnixUnusedFd = ::core::ptr::null_mut::<UnixUnusedFd>();
        let mut sStat: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3]

        };
        unixEnterMutex();
        if !inodeList.is_null()
            && 0 as ::core::ffi::c_int
                == ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *mut stat,
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[4 as ::core::ffi::c_int as usize].pCurrent)
                    .expect("non-null function pointer")(zPath, &raw mut sStat)
        {
            let mut pInode: *mut unixInodeInfo = ::core::ptr::null_mut::<
                unixInodeInfo,
            >();
            pInode = inodeList;
            while !pInode.is_null()
                && ((*pInode).fileId.dev != sStat.st_dev
                    || (*pInode).fileId.ino != sStat.st_ino as u64_0)
            {
                pInode = (*pInode).pNext;
            }
            if !pInode.is_null() {
                let mut pp: *mut *mut UnixUnusedFd = ::core::ptr::null_mut::<
                    *mut UnixUnusedFd,
                >();
                sqlite3_mutex_enter((*pInode).pLockMutex);
                flags &= SQLITE_OPEN_READONLY | SQLITE_OPEN_READWRITE;
                pp = &raw mut (*pInode).pUnused;
                while !(*pp).is_null() && (**pp).flags != flags {
                    pp = &raw mut (**pp).pNext;
                }
                pUnused = *pp;
                if !pUnused.is_null() {
                    *pp = (*pUnused).pNext;
                }
                sqlite3_mutex_leave((*pInode).pLockMutex);
            }
        }
        unixLeaveMutex();
        return pUnused;
    }
}
unsafe extern "C" fn getFileMode(
    mut zFile: *const ::core::ffi::c_char,
    mut pMode: *mut mode_t,
    mut pUid: *mut uid_t,
    mut pGid: *mut gid_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sStat: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3]

        };
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if 0 as ::core::ffi::c_int
            == ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[4 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")(zFile, &raw mut sStat)
        {
            *pMode = (sStat.st_mode as ::core::ffi::c_uint
                & 0o777 as ::core::ffi::c_uint) as mode_t;
            *pUid = sStat.st_uid as uid_t;
            *pGid = sStat.st_gid as gid_t;
        } else {
            rc = SQLITE_IOERR_FSTAT;
        }
        return rc;
    }
}
unsafe extern "C" fn findCreateFileMode(
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pMode: *mut mode_t,
    mut pUid: *mut uid_t,
    mut pGid: *mut gid_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        *pMode = 0 as mode_t;
        *pUid = 0 as uid_t;
        *pGid = 0 as gid_t;
        if flags & (SQLITE_OPEN_WAL | SQLITE_OPEN_MAIN_JOURNAL) != 0 {
            let mut zDb: [::core::ffi::c_char; 513] = [0; 513];
            let mut nDb: ::core::ffi::c_int = 0;
            nDb = sqlite3Strlen30(zPath) - 1 as ::core::ffi::c_int;
            while nDb > 0 as ::core::ffi::c_int
                && *zPath.offset(nDb as isize) as ::core::ffi::c_int != '.' as i32
            {
                if *zPath.offset(nDb as isize) as ::core::ffi::c_int == '-' as i32 {
                    memcpy(
                        &raw mut zDb as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        zPath as *const ::core::ffi::c_void,
                        nDb as size_t,
                    );
                    zDb[nDb as usize] = '\0' as i32 as ::core::ffi::c_char;
                    rc = getFileMode(
                        &raw mut zDb as *mut ::core::ffi::c_char,
                        pMode,
                        pUid,
                        pGid,
                    );
                    break;
                } else {
                    nDb -= 1;
                }
            }
        } else if flags & SQLITE_OPEN_DELETEONCLOSE != 0 {
            *pMode = 0o600 as mode_t;
        } else if flags & SQLITE_OPEN_URI != 0 {
            let mut z: *const ::core::ffi::c_char = sqlite3_uri_parameter(
                zPath as sqlite3_filename,
                b"modeof\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !z.is_null() {
                rc = getFileMode(z, pMode, pUid, pGid);
            }
        }
        return rc;
    }
}
unsafe extern "C" fn unixOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut p: *mut unixFile = pFile as *mut unixFile;
        let mut fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut openFlags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut eType: ::core::ffi::c_int = flags & 0xfff00 as ::core::ffi::c_int;
        let mut noLock: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut ctrlFlags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut isExclusive: ::core::ffi::c_int = flags & SQLITE_OPEN_EXCLUSIVE;
        let mut isDelete: ::core::ffi::c_int = flags & SQLITE_OPEN_DELETEONCLOSE;
        let mut isCreate: ::core::ffi::c_int = flags & SQLITE_OPEN_CREATE;
        let mut isReadonly: ::core::ffi::c_int = flags & SQLITE_OPEN_READONLY;
        let mut isReadWrite: ::core::ffi::c_int = flags & SQLITE_OPEN_READWRITE;
        let mut isNewJrnl: ::core::ffi::c_int = (isCreate != 0
            && (eType == SQLITE_OPEN_SUPER_JOURNAL || eType == SQLITE_OPEN_MAIN_JOURNAL
                || eType == SQLITE_OPEN_WAL)) as ::core::ffi::c_int;
        let mut zTmpname: [::core::ffi::c_char; 514] = [0; 514];
        let mut zName: *const ::core::ffi::c_char = zPath;
        if randomnessPid != getpid() {
            randomnessPid = getpid();
            sqlite3_randomness(
                0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<unixFile>() as size_t,
        );
        if eType == SQLITE_OPEN_MAIN_DB {
            let mut pUnused: *mut UnixUnusedFd = ::core::ptr::null_mut::<UnixUnusedFd>();
            pUnused = findReusableFd(zName, flags);
            if !pUnused.is_null() {
                fd = (*pUnused).fd;
            } else {
                pUnused = sqlite3_malloc64(
                    ::core::mem::size_of::<UnixUnusedFd>() as sqlite3_uint64,
                ) as *mut UnixUnusedFd;
                if pUnused.is_null() {
                    return SQLITE_NOMEM_BKPT;
                }
            }
            (*p).pPreallocatedUnused = pUnused;
        } else if zName.is_null() {
            rc = unixGetTempname(
                (*pVfs).mxPathname,
                &raw mut zTmpname as *mut ::core::ffi::c_char,
            );
            if rc != SQLITE_OK {
                return rc;
            }
            zName = &raw mut zTmpname as *mut ::core::ffi::c_char;
        }
        if isReadonly != 0 {
            openFlags |= O_RDONLY;
        }
        if isReadWrite != 0 {
            openFlags |= O_RDWR;
        }
        if isCreate != 0 {
            openFlags |= O_CREAT;
        }
        if isExclusive != 0 {
            openFlags |= O_EXCL | O_NOFOLLOW;
        }
        openFlags |= O_LARGEFILE | O_BINARY | O_NOFOLLOW;
        if fd < 0 as ::core::ffi::c_int {
            let mut openMode: mode_t = 0;
            let mut uid: uid_t = 0;
            let mut gid: gid_t = 0;
            rc = findCreateFileMode(
                zName,
                flags,
                &raw mut openMode,
                &raw mut uid,
                &raw mut gid,
            );
            if rc != SQLITE_OK {
                return rc;
            }
            fd = robust_open(zName, openFlags, openMode);
            if sqlite3OSTrace != 0 {
                sqlite3DebugPrintf(
                    b"OPENX   %-3d %s 0%o\n\0".as_ptr() as *const ::core::ffi::c_char,
                    fd,
                    zName,
                    openFlags,
                );
            }
            if fd < 0 as ::core::ffi::c_int {
                if isNewJrnl != 0 && *__errno_location() == EACCES
                    && ::core::mem::transmute::<
                        sqlite3_syscall_ptr,
                        Option<
                            unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                        >,
                    >(aSyscall[2 as ::core::ffi::c_int as usize].pCurrent)
                        .expect("non-null function pointer")(zName, F_OK) != 0
                {
                    rc = SQLITE_READONLY_DIRECTORY;
                } else if *__errno_location() != EISDIR && isReadWrite != 0 {
                    let mut pReadonly: *mut UnixUnusedFd = ::core::ptr::null_mut::<
                        UnixUnusedFd,
                    >();
                    flags &= !(SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE);
                    openFlags &= !(O_RDWR | O_CREAT);
                    flags |= SQLITE_OPEN_READONLY;
                    openFlags |= O_RDONLY;
                    isReadonly = 1 as ::core::ffi::c_int;
                    pReadonly = findReusableFd(zName, flags);
                    if !pReadonly.is_null() {
                        fd = (*pReadonly).fd;
                        sqlite3_free(pReadonly as *mut ::core::ffi::c_void);
                    } else {
                        fd = robust_open(zName, openFlags, openMode);
                    }
                }
            }
            if fd < 0 as ::core::ffi::c_int {
                let mut rc2: ::core::ffi::c_int = unixLogErrorAtLine(
                    sqlite3CantopenError(6707 as ::core::ffi::c_int),
                    b"open\0".as_ptr() as *const ::core::ffi::c_char,
                    zName,
                    6707 as ::core::ffi::c_int,
                );
                if rc == SQLITE_OK {
                    rc = rc2;
                }
                c2rust_current_block = 9931747325399668865;
            } else {
                if openMode != 0
                    && flags & (SQLITE_OPEN_WAL | SQLITE_OPEN_MAIN_JOURNAL)
                        != 0 as ::core::ffi::c_int
                {
                    robustFchown(fd, uid, gid);
                }
                c2rust_current_block = 6281126495347172768;
            }
        } else {
            c2rust_current_block = 6281126495347172768;
        }
        match c2rust_current_block {
            6281126495347172768 => {
                if !pOutFlags.is_null() {
                    *pOutFlags = flags;
                }
                if !(*p).pPreallocatedUnused.is_null() {
                    (*(*p).pPreallocatedUnused).fd = fd;
                    (*(*p).pPreallocatedUnused).flags = flags
                        & (SQLITE_OPEN_READONLY | SQLITE_OPEN_READWRITE);
                }
                if isDelete != 0 {
                    ::core::mem::transmute::<
                        sqlite3_syscall_ptr,
                        Option<
                            unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                    >(aSyscall[16 as ::core::ffi::c_int as usize].pCurrent)
                        .expect("non-null function pointer")(zName);
                }
                if isDelete != 0 {
                    ctrlFlags |= UNIXFILE_DELETE;
                }
                if isReadonly != 0 {
                    ctrlFlags |= UNIXFILE_RDONLY;
                }
                noLock = (eType != SQLITE_OPEN_MAIN_DB) as ::core::ffi::c_int;
                if noLock != 0 {
                    ctrlFlags |= UNIXFILE_NOLOCK;
                }
                if isNewJrnl != 0 {
                    ctrlFlags |= UNIXFILE_DIRSYNC;
                }
                if flags & SQLITE_OPEN_URI != 0 {
                    ctrlFlags |= UNIXFILE_URI;
                }
                rc = fillInUnixFile(pVfs, fd, pFile, zPath, ctrlFlags);
            }
            _ => {}
        }
        if rc != SQLITE_OK {
            sqlite3_free((*p).pPreallocatedUnused as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn unixDelete(
    mut NotUsed: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut dirSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if sqlite3_io_error_persist != 0 && sqlite3_io_error_hit != 0
            || {
                let c2rust_fresh4 = sqlite3_io_error_pending;
                sqlite3_io_error_pending = sqlite3_io_error_pending - 1;
                c2rust_fresh4 == 1 as ::core::ffi::c_int
            }
        {
            local_ioerr();
            return 10 as ::core::ffi::c_int
                | (10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
        }
        if ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            >,
        >(aSyscall[16 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(zPath) == -1 as ::core::ffi::c_int
        {
            if *__errno_location() == ENOENT {
                rc = SQLITE_IOERR_DELETE_NOENT;
            } else {
                rc = unixLogErrorAtLine(
                    10 as ::core::ffi::c_int
                        | (10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                    b"unlink\0".as_ptr() as *const ::core::ffi::c_char,
                    zPath,
                    6849 as ::core::ffi::c_int,
                );
            }
            return rc;
        }
        if dirSync & 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            let mut fd: ::core::ffi::c_int = 0;
            rc = ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[17 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")(zPath, &raw mut fd);
            if rc == SQLITE_OK {
                if full_fsync(fd, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int) != 0
                {
                    rc = unixLogErrorAtLine(
                        10 as ::core::ffi::c_int
                            | (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                        b"fsync\0".as_ptr() as *const ::core::ffi::c_char,
                        zPath,
                        6859 as ::core::ffi::c_int,
                    );
                }
                robust_close(
                    ::core::ptr::null_mut::<unixFile>(),
                    fd,
                    6861 as ::core::ffi::c_int,
                );
            } else {
                rc = SQLITE_OK;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn unixAccess(
    mut NotUsed: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_io_error_persist != 0 && sqlite3_io_error_hit != 0
            || {
                let c2rust_fresh3 = sqlite3_io_error_pending;
                sqlite3_io_error_pending = sqlite3_io_error_pending - 1;
                c2rust_fresh3 == 1 as ::core::ffi::c_int
            }
        {
            local_ioerr();
            return 10 as ::core::ffi::c_int
                | (13 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
        }
        if flags == SQLITE_ACCESS_EXISTS {
            let mut buf: stat = stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                __glibc_reserved: [0; 3]

            };
            *pResOut = (0 as ::core::ffi::c_int
                == ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *mut stat,
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[4 as ::core::ffi::c_int as usize].pCurrent)
                    .expect("non-null function pointer")(zPath, &raw mut buf)
                && (!(buf.st_mode as ::core::ffi::c_uint
                    & __S_IFMT as ::core::ffi::c_uint == 0o100000 as ::core::ffi::c_uint)
                    || buf.st_size > 0 as ::core::ffi::c_long)) as ::core::ffi::c_int;
        } else {
            *pResOut = (::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[2 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")(zPath, W_OK | R_OK)
                == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn appendOnePathElement(
    mut pPath: *mut DbPath,
    mut zName: *const ::core::ffi::c_char,
    mut nName: ::core::ffi::c_int,
) {
    unsafe {
        if *zName.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '.' as i32
        {
            if nName == 1 as ::core::ffi::c_int {
                return;
            }
            if *zName.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '.' as i32 && nName == 2 as ::core::ffi::c_int
            {
                if (*pPath).nUsed > 1 as ::core::ffi::c_int {
                    loop {
                        (*pPath).nUsed -= 1;
                        if !(*(*pPath).zOut.offset((*pPath).nUsed as isize)
                            as ::core::ffi::c_int != '/' as i32)
                        {
                            break;
                        }
                    }
                }
                return;
            }
        }
        if (*pPath).nUsed + nName + 2 as ::core::ffi::c_int >= (*pPath).nOut {
            (*pPath).rc = SQLITE_ERROR;
            return;
        }
        let c2rust_fresh1 = (*pPath).nUsed;
        (*pPath).nUsed = (*pPath).nUsed + 1;
        *(*pPath).zOut.offset(c2rust_fresh1 as isize) = '/' as i32
            as ::core::ffi::c_char;
        memcpy(
            (*pPath).zOut.offset((*pPath).nUsed as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            zName as *const ::core::ffi::c_void,
            nName as size_t,
        );
        (*pPath).nUsed += nName;
        if (*pPath).rc == SQLITE_OK {
            let mut zIn: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut buf: stat = stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                __glibc_reserved: [0; 3]

            };
            *(*pPath).zOut.offset((*pPath).nUsed as isize) = 0 as ::core::ffi::c_char;
            zIn = (*pPath).zOut;
            if ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[27 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")(zIn, &raw mut buf)
                != 0 as ::core::ffi::c_int
            {
                if *__errno_location() != ENOENT {
                    (*pPath).rc = unixLogErrorAtLine(
                        sqlite3CantopenError(6955 as ::core::ffi::c_int),
                        b"lstat\0".as_ptr() as *const ::core::ffi::c_char,
                        zIn,
                        6955 as ::core::ffi::c_int,
                    );
                }
            } else if buf.st_mode as ::core::ffi::c_uint
                & __S_IFMT as ::core::ffi::c_uint == 0o120000 as ::core::ffi::c_uint
            {
                let mut got: ssize_t = 0;
                let mut zLnk: [::core::ffi::c_char; 4098] = [0; 4098];
                let c2rust_fresh2 = (*pPath).nSymlink;
                (*pPath).nSymlink = (*pPath).nSymlink + 1;
                if c2rust_fresh2 > SQLITE_MAX_SYMLINK {
                    (*pPath).rc = sqlite3CantopenError(6961 as ::core::ffi::c_int);
                    return;
                }
                got = ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *mut ::core::ffi::c_char,
                            size_t,
                        ) -> ssize_t,
                    >,
                >(aSyscall[26 as ::core::ffi::c_int as usize].pCurrent)
                    .expect(
                        "non-null function pointer",
                    )(
                    zIn,
                    &raw mut zLnk as *mut ::core::ffi::c_char,
                    (::core::mem::size_of::<[::core::ffi::c_char; 4098]>() as size_t)
                        .wrapping_sub(2 as size_t),
                );
                if got <= 0 as ::core::ffi::c_long
                    || got
                        >= ::core::mem::size_of::<[::core::ffi::c_char; 4098]>()
                            as ::core::ffi::c_long - 2 as ::core::ffi::c_long
                {
                    (*pPath).rc = unixLogErrorAtLine(
                        sqlite3CantopenError(6966 as ::core::ffi::c_int),
                        b"readlink\0".as_ptr() as *const ::core::ffi::c_char,
                        zIn,
                        6966 as ::core::ffi::c_int,
                    );
                    return;
                }
                zLnk[got as usize] = 0 as ::core::ffi::c_char;
                if zLnk[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == '/' as i32
                {
                    (*pPath).nUsed = 0 as ::core::ffi::c_int;
                } else {
                    (*pPath).nUsed -= nName + 1 as ::core::ffi::c_int;
                }
                appendAllPathElements(pPath, &raw mut zLnk as *mut ::core::ffi::c_char);
            }
        }
    }
}
unsafe extern "C" fn appendAllPathElements(
    mut pPath: *mut DbPath,
    mut zPath: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            while *zPath.offset(i as isize) as ::core::ffi::c_int != 0
                && *zPath.offset(i as isize) as ::core::ffi::c_int != '/' as i32
            {
                i += 1;
            }
            if i > j {
                appendOnePathElement(
                    pPath,
                    zPath.offset(j as isize) as *const ::core::ffi::c_char,
                    i - j,
                );
            }
            j = i + 1 as ::core::ffi::c_int;
            let c2rust_fresh0 = i;
            i = i + 1;
            if !(*zPath.offset(c2rust_fresh0 as isize) != 0) {
                break;
            }
        };
    }
}
unsafe extern "C" fn unixFullPathname(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut nOut: ::core::ffi::c_int,
    mut zOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut path: DbPath = DbPath {
            rc: 0,
            nSymlink: 0,
            zOut: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nOut: 0,
            nUsed: 0

        };
        path.rc = 0 as ::core::ffi::c_int;
        path.nUsed = 0 as ::core::ffi::c_int;
        path.nSymlink = 0 as ::core::ffi::c_int;
        path.nOut = nOut;
        path.zOut = zOut;
        if *zPath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '/' as i32
        {
            let mut zPwd: [::core::ffi::c_char; 4098] = [0; 4098];
            if ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        size_t,
                    ) -> *mut ::core::ffi::c_char,
                >,
            >(aSyscall[3 as ::core::ffi::c_int as usize].pCurrent)
                .expect(
                    "non-null function pointer",
                )(
                    &raw mut zPwd as *mut ::core::ffi::c_char,
                    (::core::mem::size_of::<[::core::ffi::c_char; 4098]>() as size_t)
                        .wrapping_sub(2 as size_t),
                )
                .is_null()
            {
                return unixLogErrorAtLine(
                    sqlite3CantopenError(7024 as ::core::ffi::c_int),
                    b"getcwd\0".as_ptr() as *const ::core::ffi::c_char,
                    zPath,
                    7024 as ::core::ffi::c_int,
                );
            }
            appendAllPathElements(
                &raw mut path,
                &raw mut zPwd as *mut ::core::ffi::c_char,
            );
        }
        appendAllPathElements(&raw mut path, zPath);
        *zOut.offset(path.nUsed as isize) = 0 as ::core::ffi::c_char;
        if path.rc != 0 || path.nUsed < 2 as ::core::ffi::c_int {
            return sqlite3CantopenError(7030 as ::core::ffi::c_int);
        }
        if path.nSymlink != 0 {
            return SQLITE_OK_SYMLINK;
        }
        return SQLITE_OK;
    }
}
pub const OS_VXWORKS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn unixDlOpen(
    mut NotUsed: *mut sqlite3_vfs,
    mut zFilename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return dlopen(zFilename, RTLD_NOW | RTLD_GLOBAL);
    }
}
unsafe extern "C" fn unixDlError(
    mut NotUsed: *mut sqlite3_vfs,
    mut nBuf: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        unixEnterMutex();
        zErr = dlerror();
        if !zErr.is_null() {
            sqlite3_snprintf(
                nBuf,
                zBufOut,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                zErr,
            );
        }
        unixLeaveMutex();
    }
}
unsafe extern "C" fn unixDlSym(
    mut NotUsed: *mut sqlite3_vfs,
    mut p: *mut ::core::ffi::c_void,
    mut zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    unsafe {
        let mut x: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> Option<unsafe extern "C" fn() -> ()>
        > = None;
        x = ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_void,
            >,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> Option<unsafe extern "C" fn() -> ()>,
            >,
        >(
            Some(
                dlsym
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                    ) -> *mut ::core::ffi::c_void,
            ),
        );
        return Some(x.expect("non-null function pointer"))
            .expect("non-null function pointer")(p, zSym);
    }
}
unsafe extern "C" fn unixDlClose(
    mut NotUsed: *mut sqlite3_vfs,
    mut pHandle: *mut ::core::ffi::c_void,
) {
    unsafe {
        dlclose(pHandle);
    }
}
unsafe extern "C" fn unixRandomness(
    mut NotUsed: *mut sqlite3_vfs,
    mut nBuf: ::core::ffi::c_int,
    mut zBuf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        memset(
            zBuf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            nBuf as size_t,
        );
        randomnessPid = getpid();
        return nBuf;
    }
}
unsafe extern "C" fn unixSleep(
    mut NotUsed: *mut sqlite3_vfs,
    mut microseconds: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sp: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        sp.tv_sec = (microseconds / 1000000 as ::core::ffi::c_int) as __time_t;
        sp.tv_nsec = (microseconds % 1000000 as ::core::ffi::c_int
            * 1000 as ::core::ffi::c_int) as __syscall_slong_t;
        nanosleep(&raw mut sp, ::core::ptr::null_mut::<timespec>());
        return microseconds;
    }
}
#[no_mangle]
pub static mut sqlite3_current_time: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn unixCurrentTimeInt64(
    mut NotUsed: *mut sqlite3_vfs,
    mut piNow: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        static mut unixEpoch: sqlite3_int64 = 24405875 as sqlite3_int64
            * 8640000 as ::core::ffi::c_int as sqlite3_int64;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut sNow: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        gettimeofday(&raw mut sNow, ::core::ptr::null_mut::<::core::ffi::c_void>());
        *piNow = (unixEpoch as ::core::ffi::c_longlong
            + 1000 as ::core::ffi::c_longlong * sNow.tv_sec as ::core::ffi::c_longlong
            + (sNow.tv_usec as ::core::ffi::c_long / 1000 as ::core::ffi::c_long)
                as ::core::ffi::c_longlong) as sqlite3_int64;
        if sqlite3_current_time != 0 {
            *piNow = 1000 as sqlite3_int64 * sqlite3_current_time as sqlite3_int64
                + unixEpoch;
        }
        return rc;
    }
}
unsafe extern "C" fn unixCurrentTime(
    mut NotUsed: *mut sqlite3_vfs,
    mut prNow: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: sqlite3_int64 = 0 as sqlite3_int64;
        let mut rc: ::core::ffi::c_int = 0;
        rc = unixCurrentTimeInt64(::core::ptr::null_mut::<sqlite3_vfs>(), &raw mut i);
        *prNow = i as ::core::ffi::c_double / 86400000.0f64;
        return rc;
    }
}
unsafe extern "C" fn unixGetLastError(
    mut NotUsed: *mut sqlite3_vfs,
    mut NotUsed2: ::core::ffi::c_int,
    mut NotUsed3: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return *__errno_location();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_os_init() -> ::core::ffi::c_int {
    unsafe {
        static mut aVfs: [sqlite3_vfs; 4] = unsafe {
            [
                sqlite3_vfs {
                    iVersion: 3 as ::core::ffi::c_int,
                    szOsFile: ::core::mem::size_of::<unixFile>() as ::core::ffi::c_int,
                    mxPathname: MAX_PATHNAME,
                    pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
                    zName: b"unix\0".as_ptr() as *const ::core::ffi::c_char,
                    pAppData: &raw const posixIoFinder as *mut ::core::ffi::c_void,
                    xOpen: Some(
                        unixOpen
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                *mut sqlite3_file,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xDelete: Some(
                        unixDelete
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xAccess: Some(
                        unixAccess
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xFullPathname: Some(
                        unixFullPathname
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    xDlOpen: Some(
                        unixDlOpen
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                            ) -> *mut ::core::ffi::c_void,
                    ),
                    xDlError: Some(
                        unixDlError
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> (),
                    ),
                    xDlSym: Some(
                        unixDlSym
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut ::core::ffi::c_void,
                                *const ::core::ffi::c_char,
                            ) -> Option<unsafe extern "C" fn() -> ()>,
                    ),
                    xDlClose: Some(
                        unixDlClose
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    xRandomness: Some(
                        unixRandomness
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    xSleep: Some(
                        unixSleep
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xCurrentTime: Some(
                        unixCurrentTime
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut ::core::ffi::c_double,
                            ) -> ::core::ffi::c_int,
                    ),
                    xGetLastError: Some(
                        unixGetLastError
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    xCurrentTimeInt64: Some(
                        unixCurrentTimeInt64
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut sqlite3_int64,
                            ) -> ::core::ffi::c_int,
                    ),
                    xSetSystemCall: Some(
                        unixSetSystemCall
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                sqlite3_syscall_ptr,
                            ) -> ::core::ffi::c_int,
                    ),
                    xGetSystemCall: Some(
                        unixGetSystemCall
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                            ) -> sqlite3_syscall_ptr,
                    ),
                    xNextSystemCall: Some(
                        unixNextSystemCall
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                            ) -> *const ::core::ffi::c_char,
                    )

                },
                sqlite3_vfs {
                    iVersion: 3 as ::core::ffi::c_int,
                    szOsFile: ::core::mem::size_of::<unixFile>() as ::core::ffi::c_int,
                    mxPathname: MAX_PATHNAME,
                    pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
                    zName: b"unix-none\0".as_ptr() as *const ::core::ffi::c_char,
                    pAppData: &raw const nolockIoFinder as *mut ::core::ffi::c_void,
                    xOpen: Some(
                        unixOpen
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                *mut sqlite3_file,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xDelete: Some(
                        unixDelete
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xAccess: Some(
                        unixAccess
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xFullPathname: Some(
                        unixFullPathname
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    xDlOpen: Some(
                        unixDlOpen
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                            ) -> *mut ::core::ffi::c_void,
                    ),
                    xDlError: Some(
                        unixDlError
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> (),
                    ),
                    xDlSym: Some(
                        unixDlSym
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut ::core::ffi::c_void,
                                *const ::core::ffi::c_char,
                            ) -> Option<unsafe extern "C" fn() -> ()>,
                    ),
                    xDlClose: Some(
                        unixDlClose
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    xRandomness: Some(
                        unixRandomness
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    xSleep: Some(
                        unixSleep
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xCurrentTime: Some(
                        unixCurrentTime
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut ::core::ffi::c_double,
                            ) -> ::core::ffi::c_int,
                    ),
                    xGetLastError: Some(
                        unixGetLastError
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    xCurrentTimeInt64: Some(
                        unixCurrentTimeInt64
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut sqlite3_int64,
                            ) -> ::core::ffi::c_int,
                    ),
                    xSetSystemCall: Some(
                        unixSetSystemCall
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                sqlite3_syscall_ptr,
                            ) -> ::core::ffi::c_int,
                    ),
                    xGetSystemCall: Some(
                        unixGetSystemCall
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                            ) -> sqlite3_syscall_ptr,
                    ),
                    xNextSystemCall: Some(
                        unixNextSystemCall
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                            ) -> *const ::core::ffi::c_char,
                    )

                },
                sqlite3_vfs {
                    iVersion: 3 as ::core::ffi::c_int,
                    szOsFile: ::core::mem::size_of::<unixFile>() as ::core::ffi::c_int,
                    mxPathname: MAX_PATHNAME,
                    pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
                    zName: b"unix-dotfile\0".as_ptr() as *const ::core::ffi::c_char,
                    pAppData: &raw const dotlockIoFinder as *mut ::core::ffi::c_void,
                    xOpen: Some(
                        unixOpen
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                *mut sqlite3_file,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xDelete: Some(
                        unixDelete
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xAccess: Some(
                        unixAccess
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xFullPathname: Some(
                        unixFullPathname
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    xDlOpen: Some(
                        unixDlOpen
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                            ) -> *mut ::core::ffi::c_void,
                    ),
                    xDlError: Some(
                        unixDlError
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> (),
                    ),
                    xDlSym: Some(
                        unixDlSym
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut ::core::ffi::c_void,
                                *const ::core::ffi::c_char,
                            ) -> Option<unsafe extern "C" fn() -> ()>,
                    ),
                    xDlClose: Some(
                        unixDlClose
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    xRandomness: Some(
                        unixRandomness
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    xSleep: Some(
                        unixSleep
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xCurrentTime: Some(
                        unixCurrentTime
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut ::core::ffi::c_double,
                            ) -> ::core::ffi::c_int,
                    ),
                    xGetLastError: Some(
                        unixGetLastError
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    xCurrentTimeInt64: Some(
                        unixCurrentTimeInt64
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut sqlite3_int64,
                            ) -> ::core::ffi::c_int,
                    ),
                    xSetSystemCall: Some(
                        unixSetSystemCall
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                sqlite3_syscall_ptr,
                            ) -> ::core::ffi::c_int,
                    ),
                    xGetSystemCall: Some(
                        unixGetSystemCall
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                            ) -> sqlite3_syscall_ptr,
                    ),
                    xNextSystemCall: Some(
                        unixNextSystemCall
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                            ) -> *const ::core::ffi::c_char,
                    )

                },
                sqlite3_vfs {
                    iVersion: 3 as ::core::ffi::c_int,
                    szOsFile: ::core::mem::size_of::<unixFile>() as ::core::ffi::c_int,
                    mxPathname: MAX_PATHNAME,
                    pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
                    zName: b"unix-excl\0".as_ptr() as *const ::core::ffi::c_char,
                    pAppData: &raw const posixIoFinder as *mut ::core::ffi::c_void,
                    xOpen: Some(
                        unixOpen
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                *mut sqlite3_file,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xDelete: Some(
                        unixDelete
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xAccess: Some(
                        unixAccess
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xFullPathname: Some(
                        unixFullPathname
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    xDlOpen: Some(
                        unixDlOpen
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                            ) -> *mut ::core::ffi::c_void,
                    ),
                    xDlError: Some(
                        unixDlError
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> (),
                    ),
                    xDlSym: Some(
                        unixDlSym
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut ::core::ffi::c_void,
                                *const ::core::ffi::c_char,
                            ) -> Option<unsafe extern "C" fn() -> ()>,
                    ),
                    xDlClose: Some(
                        unixDlClose
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    xRandomness: Some(
                        unixRandomness
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    xSleep: Some(
                        unixSleep
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    xCurrentTime: Some(
                        unixCurrentTime
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut ::core::ffi::c_double,
                            ) -> ::core::ffi::c_int,
                    ),
                    xGetLastError: Some(
                        unixGetLastError
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                ::core::ffi::c_int,
                                *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    xCurrentTimeInt64: Some(
                        unixCurrentTimeInt64
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *mut sqlite3_int64,
                            ) -> ::core::ffi::c_int,
                    ),
                    xSetSystemCall: Some(
                        unixSetSystemCall
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                                sqlite3_syscall_ptr,
                            ) -> ::core::ffi::c_int,
                    ),
                    xGetSystemCall: Some(
                        unixGetSystemCall
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                            ) -> sqlite3_syscall_ptr,
                    ),
                    xNextSystemCall: Some(
                        unixNextSystemCall
                            as unsafe extern "C" fn(
                                *mut sqlite3_vfs,
                                *const ::core::ffi::c_char,
                            ) -> *const ::core::ffi::c_char,
                    )

                },
            ]
        };
        let mut i: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while (i as usize)
            < (::core::mem::size_of::<[sqlite3_vfs; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<sqlite3_vfs>() as usize)
        {
            sqlite3_vfs_register(
                (&raw mut aVfs as *mut sqlite3_vfs).offset(i as isize)
                    as *mut sqlite3_vfs,
                (i == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        unixBigLock = sqlite3MutexAlloc(SQLITE_MUTEX_STATIC_VFS1);
        unixTempFileInit();
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_os_end() -> ::core::ffi::c_int {
    unsafe {
        unixBigLock = ::core::ptr::null_mut::<sqlite3_mutex>();
        return SQLITE_OK;
    }
}
