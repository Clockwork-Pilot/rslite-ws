extern "C" {
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
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
    fn sqlite3OsClose(_: *mut sqlite3_file);
    fn sqlite3OsWrite(
        _: *mut sqlite3_file,
        _: *const ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsOpen(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type i64_0 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type u8_0 = ::core::ffi::c_uchar;
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemJournal {
    pub pMethod: *const sqlite3_io_methods,
    pub nChunkSize: ::core::ffi::c_int,
    pub nSpill: ::core::ffi::c_int,
    pub pFirst: *mut FileChunk,
    pub endpoint: FilePoint,
    pub readpoint: FilePoint,
    pub flags: ::core::ffi::c_int,
    pub pVfs: *mut sqlite3_vfs,
    pub zJournal: *const ::core::ffi::c_char

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FilePoint {
    pub iOffset: sqlite3_int64,
    pub pChunk: *mut FileChunk

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileChunk {
    pub pNext: *mut FileChunk,
    pub zChunk: [u8_0; 8]

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_IOERR_SHORT_READ: ::core::ffi::c_int = SQLITE_IOERR
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const MEMJOURNAL_DFLT_FILECHUNKSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
unsafe extern "C" fn memjrnlRead(
    mut pJfd: *mut sqlite3_file,
    mut zBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemJournal = pJfd as *mut MemJournal;
        let mut zOut: *mut u8_0 = zBuf as *mut u8_0;
        let mut nRead: ::core::ffi::c_int = iAmt;
        let mut iChunkOffset: ::core::ffi::c_int = 0;
        let mut pChunk: *mut FileChunk = ::core::ptr::null_mut::<FileChunk>();
        if iAmt as sqlite_int64 + iOfst > (*p).endpoint.iOffset {
            return SQLITE_IOERR_SHORT_READ;
        }
        if (*p).readpoint.iOffset != iOfst || iOfst == 0 as ::core::ffi::c_longlong {
            let mut iOff: sqlite3_int64 = 0 as sqlite3_int64;
            pChunk = (*p).pFirst;
            while !pChunk.is_null()
                && iOff as ::core::ffi::c_longlong
                    + (*p).nChunkSize as ::core::ffi::c_longlong <= iOfst
            {
                iOff += (*p).nChunkSize as ::core::ffi::c_longlong;
                pChunk = (*pChunk).pNext;
            }
        } else {
            pChunk = (*p).readpoint.pChunk;
        }
        iChunkOffset = (iOfst as ::core::ffi::c_longlong
            % (*p).nChunkSize as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        loop {
            let mut iSpace: ::core::ffi::c_int = (*p).nChunkSize - iChunkOffset;
            let mut nCopy: ::core::ffi::c_int = if nRead < (*p).nChunkSize - iChunkOffset
            {
                nRead
            } else {
                (*p).nChunkSize - iChunkOffset
            };
            memcpy(
                zOut as *mut ::core::ffi::c_void,
                (&raw mut (*pChunk).zChunk as *mut u8_0).offset(iChunkOffset as isize)
                    as *const ::core::ffi::c_void,
                nCopy as size_t,
            );
            zOut = zOut.offset(nCopy as isize);
            nRead -= iSpace;
            iChunkOffset = 0 as ::core::ffi::c_int;
            if !(nRead >= 0 as ::core::ffi::c_int
                && {
                    pChunk = (*pChunk).pNext;
                    !pChunk.is_null()
                } && nRead > 0 as ::core::ffi::c_int)
            {
                break;
            }
        }
        (*p).readpoint.iOffset = (if !pChunk.is_null() {
            iOfst as ::core::ffi::c_longlong + iAmt as ::core::ffi::c_longlong
        } else {
            0 as ::core::ffi::c_longlong
        }) as sqlite3_int64;
        (*p).readpoint.pChunk = pChunk;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memjrnlFreeChunks(mut pFirst: *mut FileChunk) {
    unsafe {
        let mut pIter: *mut FileChunk = ::core::ptr::null_mut::<FileChunk>();
        let mut pNext: *mut FileChunk = ::core::ptr::null_mut::<FileChunk>();
        pIter = pFirst;
        while !pIter.is_null() {
            pNext = (*pIter).pNext;
            sqlite3_free(pIter as *mut ::core::ffi::c_void);
            pIter = pNext;
        }
    }
}
unsafe extern "C" fn memjrnlCreateFile(mut p: *mut MemJournal) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pReal: *mut sqlite3_file = p as *mut sqlite3_file;
        let mut copy: MemJournal = *p;
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<MemJournal>() as size_t,
        );
        rc = sqlite3OsOpen(
            copy.pVfs,
            copy.zJournal,
            pReal,
            copy.flags,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if rc == SQLITE_OK {
            let mut nChunk: ::core::ffi::c_int = copy.nChunkSize;
            let mut iOff: i64_0 = 0 as i64_0;
            let mut pIter: *mut FileChunk = ::core::ptr::null_mut::<FileChunk>();
            pIter = copy.pFirst;
            while !pIter.is_null() {
                if iOff as ::core::ffi::c_longlong + nChunk as ::core::ffi::c_longlong
                    > copy.endpoint.iOffset
                {
                    nChunk = (copy.endpoint.iOffset as i64_0 - iOff)
                        as ::core::ffi::c_int;
                }
                rc = sqlite3OsWrite(
                    pReal,
                    &raw mut (*pIter).zChunk as *mut u8_0 as *const ::core::ffi::c_void,
                    nChunk,
                    iOff,
                );
                if rc != 0 {
                    break;
                }
                iOff += nChunk as ::core::ffi::c_longlong;
                pIter = (*pIter).pNext;
            }
            if rc == SQLITE_OK {
                memjrnlFreeChunks(copy.pFirst);
            }
        }
        if rc != SQLITE_OK {
            sqlite3OsClose(pReal);
            *p = copy;
        }
        return rc;
    }
}
unsafe extern "C" fn memjrnlWrite(
    mut pJfd: *mut sqlite3_file,
    mut zBuf: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemJournal = pJfd as *mut MemJournal;
        let mut nWrite: ::core::ffi::c_int = iAmt;
        let mut zWrite: *mut u8_0 = zBuf as *mut u8_0;
        if (*p).nSpill > 0 as ::core::ffi::c_int
            && iAmt as sqlite_int64 + iOfst > (*p).nSpill as ::core::ffi::c_longlong
        {
            let mut rc: ::core::ffi::c_int = memjrnlCreateFile(p);
            if rc == SQLITE_OK {
                rc = sqlite3OsWrite(pJfd, zBuf, iAmt, iOfst as i64_0);
            }
            return rc;
        } else {
            if iOfst > 0 as ::core::ffi::c_longlong && iOfst != (*p).endpoint.iOffset {
                memjrnlTruncate(pJfd, iOfst);
            }
            if iOfst == 0 as ::core::ffi::c_longlong && !(*p).pFirst.is_null() {
                memcpy(
                    &raw mut (*(*p).pFirst).zChunk as *mut u8_0
                        as *mut ::core::ffi::c_void,
                    zBuf,
                    iAmt as size_t,
                );
            } else {
                while nWrite > 0 as ::core::ffi::c_int {
                    let mut pChunk: *mut FileChunk = (*p).endpoint.pChunk;
                    let mut iChunkOffset: ::core::ffi::c_int = ((*p).endpoint.iOffset
                        as ::core::ffi::c_longlong
                        % (*p).nChunkSize as ::core::ffi::c_longlong)
                        as ::core::ffi::c_int;
                    let mut iSpace: ::core::ffi::c_int = if nWrite
                        < (*p).nChunkSize - iChunkOffset
                    {
                        nWrite
                    } else {
                        (*p).nChunkSize - iChunkOffset
                    };
                    if iChunkOffset == 0 as ::core::ffi::c_int {
                        let mut pNew: *mut FileChunk = sqlite3_malloc(
                            (::core::mem::size_of::<FileChunk>() as usize)
                                .wrapping_add(
                                    ((*p).nChunkSize - 8 as ::core::ffi::c_int) as usize,
                                ) as ::core::ffi::c_int,
                        ) as *mut FileChunk;
                        if pNew.is_null() {
                            return SQLITE_IOERR_NOMEM_BKPT;
                        }
                        (*pNew).pNext = ::core::ptr::null_mut::<FileChunk>();
                        if !pChunk.is_null() {
                            (*pChunk).pNext = pNew;
                        } else {
                            (*p).pFirst = pNew;
                        }
                        (*p).endpoint.pChunk = pNew;
                        pChunk = (*p).endpoint.pChunk;
                    }
                    memcpy(
                        (&raw mut (*pChunk).zChunk as *mut u8_0)
                            .offset(iChunkOffset as isize) as *mut ::core::ffi::c_void,
                        zWrite as *const ::core::ffi::c_void,
                        iSpace as size_t,
                    );
                    zWrite = zWrite.offset(iSpace as isize);
                    nWrite -= iSpace;
                    (*p).endpoint.iOffset += iSpace as ::core::ffi::c_longlong;
                }
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memjrnlTruncate(
    mut pJfd: *mut sqlite3_file,
    mut size: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemJournal = pJfd as *mut MemJournal;
        if size < (*p).endpoint.iOffset {
            let mut pIter: *mut FileChunk = ::core::ptr::null_mut::<FileChunk>();
            if size == 0 as ::core::ffi::c_longlong {
                memjrnlFreeChunks((*p).pFirst);
                (*p).pFirst = ::core::ptr::null_mut::<FileChunk>();
            } else {
                let mut iOff: i64_0 = (*p).nChunkSize as i64_0;
                pIter = (*p).pFirst;
                while !pIter.is_null() && iOff < size {
                    iOff += (*p).nChunkSize as ::core::ffi::c_longlong;
                    pIter = (*pIter).pNext;
                }
                if !pIter.is_null() {
                    memjrnlFreeChunks((*pIter).pNext);
                    (*pIter).pNext = ::core::ptr::null_mut::<FileChunk>();
                }
            }
            (*p).endpoint.pChunk = pIter;
            (*p).endpoint.iOffset = size as sqlite3_int64;
            (*p).readpoint.pChunk = ::core::ptr::null_mut::<FileChunk>();
            (*p).readpoint.iOffset = 0 as sqlite3_int64;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memjrnlClose(mut pJfd: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemJournal = pJfd as *mut MemJournal;
        memjrnlFreeChunks((*p).pFirst);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memjrnlSync(
    mut pJfd: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn memjrnlFileSize(
    mut pJfd: *mut sqlite3_file,
    mut pSize: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemJournal = pJfd as *mut MemJournal;
        *pSize = (*p).endpoint.iOffset;
        return SQLITE_OK;
    }
}
static mut MemJournalMethods: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 1 as ::core::ffi::c_int,
        xClose: Some(
            memjrnlClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            memjrnlRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            memjrnlWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            memjrnlTruncate
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            memjrnlSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            memjrnlFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xLock: None,
        xUnlock: None,
        xCheckReservedLock: None,
        xFileControl: None,
        xSectorSize: None,
        xDeviceCharacteristics: None,
        xShmMap: None,
        xShmLock: None,
        xShmBarrier: None,
        xShmUnmap: None,
        xFetch: None,
        xUnfetch: None

    }
};
#[no_mangle]
pub unsafe extern "C" fn sqlite3JournalOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pJfd: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut nSpill: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut MemJournal = pJfd as *mut MemJournal;
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<MemJournal>() as size_t,
        );
        if nSpill == 0 as ::core::ffi::c_int {
            return sqlite3OsOpen(
                pVfs,
                zName,
                pJfd,
                flags,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        }
        if nSpill > 0 as ::core::ffi::c_int {
            (*p).nChunkSize = nSpill;
        } else {
            (*p).nChunkSize = ((8 as ::core::ffi::c_int + MEMJOURNAL_DFLT_FILECHUNKSIZE)
                as usize)
                .wrapping_sub(::core::mem::size_of::<FileChunk>() as usize)
                as ::core::ffi::c_int;
        }
        (*pJfd).pMethods = &raw const MemJournalMethods as *const sqlite3_io_methods
            as *const sqlite3_io_methods;
        (*p).nSpill = nSpill;
        (*p).flags = flags;
        (*p).zJournal = zName;
        (*p).pVfs = pVfs;
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MemJournalOpen(mut pJfd: *mut sqlite3_file) {
    unsafe {
        sqlite3JournalOpen(
            ::core::ptr::null_mut::<sqlite3_vfs>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            pJfd,
            0 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3JournalIsInMemory(
    mut p: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        return ((*p).pMethods == &raw const MemJournalMethods) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3JournalSize(
    mut pVfs: *mut sqlite3_vfs,
) -> ::core::ffi::c_int {
    unsafe {
        return if (*pVfs).szOsFile
            > ::core::mem::size_of::<MemJournal>() as ::core::ffi::c_int
        {
            (*pVfs).szOsFile
        } else {
            ::core::mem::size_of::<MemJournal>() as ::core::ffi::c_int
        };
    }
}
pub const SQLITE_IOERR_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_IOERR_NOMEM;
