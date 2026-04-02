extern "C" {
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Malloc(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3MallocSize(_: *const ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
}
pub type u64_0 = sqlite_uint64;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hash {
    pub htsize: ::core::ffi::c_uint,
    pub count: ::core::ffi::c_uint,
    pub first: *mut HashElem,
    pub ht: *mut _ht

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ht {
    pub count: ::core::ffi::c_uint,
    pub chain: *mut HashElem

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashElem {
    pub next: *mut HashElem,
    pub prev: *mut HashElem,
    pub data: *mut ::core::ffi::c_void,
    pub pKey: *const ::core::ffi::c_char,
    pub h: ::core::ffi::c_uint

}
pub type size_t = usize;
pub const SQLITE_MALLOC_SOFT_LIMIT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3HashInit(mut pNew: *mut Hash) {
    unsafe {
        (*pNew).first = ::core::ptr::null_mut::<HashElem>();
        (*pNew).count = 0 as ::core::ffi::c_uint;
        (*pNew).htsize = 0 as ::core::ffi::c_uint;
        (*pNew).ht = ::core::ptr::null_mut::<_ht>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3HashClear(mut pH: *mut Hash) {
    unsafe {
        let mut elem: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        elem = (*pH).first;
        (*pH).first = ::core::ptr::null_mut::<HashElem>();
        sqlite3_free((*pH).ht as *mut ::core::ffi::c_void);
        (*pH).ht = ::core::ptr::null_mut::<_ht>();
        (*pH).htsize = 0 as ::core::ffi::c_uint;
        while !elem.is_null() {
            let mut next_elem: *mut HashElem = (*elem).next;
            sqlite3_free(elem as *mut ::core::ffi::c_void);
            elem = next_elem;
        }
        (*pH).count = 0 as ::core::ffi::c_uint;
    }
}
unsafe extern "C" fn strHash(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_uint {
    unsafe {
        let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while *z.offset(0 as ::core::ffi::c_int as isize) != 0 {
            let c2rust_fresh0 = z;
            z = z.offset(1);
            h = h
                .wrapping_add(
                    (0xdf as ::core::ffi::c_int
                        & *c2rust_fresh0 as ::core::ffi::c_uchar as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                );
            h = h.wrapping_mul(0x9e3779b1 as ::core::ffi::c_uint);
        }
        return h;
    }
}
unsafe extern "C" fn insertElement(
    mut pH: *mut Hash,
    mut pEntry: *mut _ht,
    mut pNew: *mut HashElem,
) {
    unsafe {
        let mut pHead: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        if !pEntry.is_null() {
            pHead = if (*pEntry).count != 0 {
                (*pEntry).chain
            } else {
                ::core::ptr::null_mut::<HashElem>()
            };
            (*pEntry).count = (*pEntry).count.wrapping_add(1);
            (*pEntry).chain = pNew;
        } else {
            pHead = ::core::ptr::null_mut::<HashElem>();
        }
        if !pHead.is_null() {
            (*pNew).next = pHead;
            (*pNew).prev = (*pHead).prev;
            if !(*pHead).prev.is_null() {
                (*(*pHead).prev).next = pNew;
            } else {
                (*pH).first = pNew;
            }
            (*pHead).prev = pNew;
        } else {
            (*pNew).next = (*pH).first;
            if !(*pH).first.is_null() {
                (*(*pH).first).prev = pNew;
            }
            (*pNew).prev = ::core::ptr::null_mut::<HashElem>();
            (*pH).first = pNew;
        };
    }
}
unsafe extern "C" fn rehash(
    mut pH: *mut Hash,
    mut new_size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut new_ht: *mut _ht = ::core::ptr::null_mut::<_ht>();
        let mut elem: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut next_elem: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        if (new_size as usize).wrapping_mul(::core::mem::size_of::<_ht>() as usize)
            > SQLITE_MALLOC_SOFT_LIMIT as usize
        {
            new_size = (SQLITE_MALLOC_SOFT_LIMIT as usize)
                .wrapping_div(::core::mem::size_of::<_ht>() as usize)
                as ::core::ffi::c_uint;
        }
        if new_size == (*pH).htsize {
            return 0 as ::core::ffi::c_int;
        }
        sqlite3BeginBenignMalloc();
        new_ht = sqlite3Malloc(
            (new_size as usize).wrapping_mul(::core::mem::size_of::<_ht>() as usize)
                as u64_0,
        ) as *mut _ht;
        sqlite3EndBenignMalloc();
        if new_ht.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        sqlite3_free((*pH).ht as *mut ::core::ffi::c_void);
        (*pH).ht = new_ht as *mut _ht;
        new_size = (sqlite3MallocSize(new_ht as *const ::core::ffi::c_void) as usize)
            .wrapping_div(::core::mem::size_of::<_ht>() as usize) as ::core::ffi::c_uint;
        (*pH).htsize = new_size;
        memset(
            new_ht as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (new_size as size_t).wrapping_mul(::core::mem::size_of::<_ht>() as size_t),
        );
        elem = (*pH).first;
        (*pH).first = ::core::ptr::null_mut::<HashElem>();
        while !elem.is_null() {
            next_elem = (*elem).next;
            insertElement(
                pH,
                new_ht.offset((*elem).h.wrapping_rem(new_size) as isize) as *mut _ht,
                elem,
            );
            elem = next_elem;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn findElementWithHash(
    mut pH: *const Hash,
    mut pKey: *const ::core::ffi::c_char,
    mut pHash: *mut ::core::ffi::c_uint,
) -> *mut HashElem {
    unsafe {
        let mut elem: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut count: ::core::ffi::c_uint = 0;
        let mut h: ::core::ffi::c_uint = 0;
        static mut nullElement: HashElem = HashElem {
            next: ::core::ptr::null::<HashElem>() as *mut HashElem,
            prev: ::core::ptr::null::<HashElem>() as *mut HashElem,
            data: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            pKey: ::core::ptr::null::<::core::ffi::c_char>(),
            h: 0 as ::core::ffi::c_uint

        };
        h = strHash(pKey);
        if !(*pH).ht.is_null() {
            let mut pEntry: *mut _ht = ::core::ptr::null_mut::<_ht>();
            pEntry = (*pH).ht.offset(h.wrapping_rem((*pH).htsize) as isize) as *mut _ht
                as *mut _ht;
            elem = (*pEntry).chain;
            count = (*pEntry).count;
        } else {
            elem = (*pH).first;
            count = (*pH).count;
        }
        if !pHash.is_null() {
            *pHash = h;
        }
        while count != 0 {
            if h == (*elem).h
                && sqlite3StrICmp((*elem).pKey, pKey) == 0 as ::core::ffi::c_int
            {
                return elem;
            }
            elem = (*elem).next;
            count = count.wrapping_sub(1);
        }
        return &raw mut nullElement;
    }
}
unsafe extern "C" fn removeElement(mut pH: *mut Hash, mut elem: *mut HashElem) {
    unsafe {
        let mut pEntry: *mut _ht = ::core::ptr::null_mut::<_ht>();
        if !(*elem).prev.is_null() {
            (*(*elem).prev).next = (*elem).next;
        } else {
            (*pH).first = (*elem).next;
        }
        if !(*elem).next.is_null() {
            (*(*elem).next).prev = (*elem).prev;
        }
        if !(*pH).ht.is_null() {
            pEntry = (*pH).ht.offset((*elem).h.wrapping_rem((*pH).htsize) as isize)
                as *mut _ht as *mut _ht;
            if (*pEntry).chain == elem {
                (*pEntry).chain = (*elem).next;
            }
            (*pEntry).count = (*pEntry).count.wrapping_sub(1);
        }
        sqlite3_free(elem as *mut ::core::ffi::c_void);
        (*pH).count = (*pH).count.wrapping_sub(1);
        if (*pH).count == 0 as ::core::ffi::c_uint {
            sqlite3HashClear(pH);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3HashFind(
    mut pH: *const Hash,
    mut pKey: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*findElementWithHash(
            pH,
            pKey,
            ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        ))
            .data;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3HashInsert(
    mut pH: *mut Hash,
    mut pKey: *const ::core::ffi::c_char,
    mut data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut h: ::core::ffi::c_uint = 0;
        let mut elem: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut new_elem: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        elem = findElementWithHash(pH, pKey, &raw mut h);
        if !(*elem).data.is_null() {
            let mut old_data: *mut ::core::ffi::c_void = (*elem).data;
            if data.is_null() {
                removeElement(pH, elem);
            } else {
                (*elem).data = data;
                (*elem).pKey = pKey;
            }
            return old_data;
        }
        if data.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        new_elem = sqlite3Malloc(::core::mem::size_of::<HashElem>() as u64_0)
            as *mut HashElem;
        if new_elem.is_null() {
            return data;
        }
        (*new_elem).pKey = pKey;
        (*new_elem).h = h;
        (*new_elem).data = data;
        (*pH).count = (*pH).count.wrapping_add(1);
        if (*pH).count >= 5 as ::core::ffi::c_uint
            && (*pH).count > (2 as ::core::ffi::c_uint).wrapping_mul((*pH).htsize)
        {
            rehash(pH, (*pH).count.wrapping_mul(3 as ::core::ffi::c_uint));
        }
        insertElement(
            pH,
            if !(*pH).ht.is_null() {
                (*pH).ht.offset((*new_elem).h.wrapping_rem((*pH).htsize) as isize)
                    as *mut _ht
            } else {
                ::core::ptr::null_mut::<_ht>()
            },
            new_elem,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
}
