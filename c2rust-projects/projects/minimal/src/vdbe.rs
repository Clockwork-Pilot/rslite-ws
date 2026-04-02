use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type VdbeSorter;
    pub type BtCursor;
    pub type Btree;
    pub type RenameToken;
    pub type VtabCtx;
    pub type sqlite3_mutex;
    pub type sqlite3_pcache;
    pub type RowSet;
    pub type Pager;
    fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
        errmsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_log(
        iErrCode: ::core::ffi::c_int,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
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
    fn sqlite3PagerSetJournalMode(
        _: *mut Pager,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerGetJournalMode(_: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerOkToChangeJournalMode(_: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerWalSupported(pPager: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerCloseWal(pPager: *mut Pager, _: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3PagerFilename(
        _: *const Pager,
        _: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3BtreeOpen(
        pVfs: *mut sqlite3_vfs,
        zFilename: *const ::core::ffi::c_char,
        db: *mut sqlite3,
        ppBtree: *mut *mut Btree,
        flags: ::core::ffi::c_int,
        vfsFlags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeClose(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeMaxPageCount(_: *mut Btree, _: Pgno) -> Pgno;
    fn sqlite3BtreeLastPage(_: *mut Btree) -> Pgno;
    fn sqlite3BtreeBeginTrans(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeBeginStmt(_: *mut Btree, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3BtreeCreateTable(
        _: *mut Btree,
        _: *mut Pgno,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeSavepoint(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeIncrVacuum(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeDropTable(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeClearTable(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: *mut i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeClearTableOfCursor(_: *mut BtCursor) -> ::core::ffi::c_int;
    fn sqlite3BtreeTripAllCursors(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeGetMeta(
        pBtree: *mut Btree,
        idx: ::core::ffi::c_int,
        pValue: *mut u32_0,
    );
    fn sqlite3BtreeUpdateMeta(
        _: *mut Btree,
        idx: ::core::ffi::c_int,
        value: u32_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeCursor(
        _: *mut Btree,
        iTable: Pgno,
        wrFlag: ::core::ffi::c_int,
        _: *mut KeyInfo,
        pCursor: *mut BtCursor,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeFakeValidCursor() -> *mut BtCursor;
    fn sqlite3BtreeCursorSize() -> ::core::ffi::c_int;
    fn sqlite3BtreeCursorZero(_: *mut BtCursor);
    fn sqlite3BtreeCursorHintFlags(_: *mut BtCursor, _: ::core::ffi::c_uint);
    fn sqlite3BtreeTableMoveto(
        _: *mut BtCursor,
        intKey: i64_0,
        bias: ::core::ffi::c_int,
        pRes: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeIndexMoveto(
        _: *mut BtCursor,
        pUnKey: *mut UnpackedRecord,
        pRes: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeCursorHasMoved(_: *mut BtCursor) -> ::core::ffi::c_int;
    fn sqlite3BtreeDelete(_: *mut BtCursor, flags: u8_0) -> ::core::ffi::c_int;
    fn sqlite3BtreeInsert(
        _: *mut BtCursor,
        pPayload: *const BtreePayload,
        flags: ::core::ffi::c_int,
        seekResult: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeFirst(
        _: *mut BtCursor,
        pRes: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeIsEmpty(
        pCur: *mut BtCursor,
        pRes: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeLast(
        _: *mut BtCursor,
        pRes: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeNext(
        _: *mut BtCursor,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeEof(_: *mut BtCursor) -> ::core::ffi::c_int;
    fn sqlite3BtreePrevious(
        _: *mut BtCursor,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeIntegerKey(_: *mut BtCursor) -> i64_0;
    fn sqlite3BtreeCursorPin(_: *mut BtCursor);
    fn sqlite3BtreeCursorUnpin(_: *mut BtCursor);
    fn sqlite3BtreeOffset(_: *mut BtCursor) -> i64_0;
    fn sqlite3BtreePayload(
        _: *mut BtCursor,
        offset: u32_0,
        amt: u32_0,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreePayloadFetch(
        _: *mut BtCursor,
        pAmt: *mut u32_0,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3BtreePayloadSize(_: *mut BtCursor) -> u32_0;
    fn sqlite3BtreeIntegrityCheck(
        db: *mut sqlite3,
        p: *mut Btree,
        aRoot: *mut Pgno,
        aCnt: *mut sqlite3_value,
        nRoot: ::core::ffi::c_int,
        mxErr: ::core::ffi::c_int,
        pnErr: *mut ::core::ffi::c_int,
        pzOut: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeRowCountEst(_: *mut BtCursor) -> i64_0;
    fn sqlite3BtreeClearCursor(_: *mut BtCursor);
    fn sqlite3BtreeSetVersion(
        pBt: *mut Btree,
        iVersion: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeCursorHasHint(
        _: *mut BtCursor,
        mask: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeCursorIsValidNN(_: *mut BtCursor) -> ::core::ffi::c_int;
    fn sqlite3BtreeCount(
        _: *mut sqlite3,
        _: *mut BtCursor,
        _: *mut i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeTransferRow(
        _: *mut BtCursor,
        _: *mut BtCursor,
        _: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3MemCompare(
        _: *const Mem,
        _: *const Mem,
        _: *const CollSeq,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeRecordUnpack(
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: *mut UnpackedRecord,
    );
    fn sqlite3VdbeRecordCompareWithSkip(
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: *mut UnpackedRecord,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAllocUnpackedRecord(_: *mut KeyInfo) -> *mut UnpackedRecord;
    fn sqlite3ReportError(
        iErr: ::core::ffi::c_int,
        lineno: ::core::ffi::c_int,
        zType: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3CorruptError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRaw(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3IsNaN(_: ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3InitCallback(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3InitOne(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        _: u32_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3ResetAllSchemasOfConnection(_: *mut sqlite3);
    fn sqlite3ResetOneSchema(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3RowSetInsert(_: *mut RowSet, _: i64_0);
    fn sqlite3RowSetTest(
        _: *mut RowSet,
        iBatch: ::core::ffi::c_int,
        _: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3RowSetNext(_: *mut RowSet, _: *mut i64_0) -> ::core::ffi::c_int;
    fn sqlite3UnlinkAndDeleteTable(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3UnlinkAndDeleteIndex(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3RunVacuum(
        _: *mut *mut ::core::ffi::c_char,
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3RollbackAll(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3CloseSavepoints(_: *mut sqlite3);
    fn sqlite3UnlinkAndDeleteTrigger(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3RealSameAsInt(
        _: ::core::ffi::c_double,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3RealToI64(_: ::core::ffi::c_double) -> i64_0;
    fn sqlite3AtoF(
        z: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_double,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3LogEst(_: u64_0) -> LogEst;
    fn sqlite3PutVarint(_: *mut ::core::ffi::c_uchar, _: u64_0) -> ::core::ffi::c_int;
    fn sqlite3GetVarint32(_: *const ::core::ffi::c_uchar, _: *mut u32_0) -> u8_0;
    fn sqlite3VarintLen(v: u64_0) -> ::core::ffi::c_int;
    fn sqlite3Atoi64(
        _: *const ::core::ffi::c_char,
        _: *mut i64_0,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3SystemError(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3ErrStr(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3WritableSchema(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3VdbeSetChanges(_: *mut sqlite3, _: i64_0);
    fn sqlite3AddInt64(_: *mut i64_0, _: i64_0) -> ::core::ffi::c_int;
    fn sqlite3SubInt64(_: *mut i64_0, _: i64_0) -> ::core::ffi::c_int;
    fn sqlite3MulInt64(_: *mut i64_0, _: i64_0) -> ::core::ffi::c_int;
    fn sqlite3ValueText(_: *mut sqlite3_value, _: u8_0) -> *const ::core::ffi::c_void;
    static mut sqlite3StdType: [*const ::core::ffi::c_char; 0];
    static mut sqlite3aLTb: *const ::core::ffi::c_uchar;
    static mut sqlite3aEQb: *const ::core::ffi::c_uchar;
    static mut sqlite3aGTb: *const ::core::ffi::c_uchar;
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3RootPageMoved(_: *mut sqlite3, _: ::core::ffi::c_int, _: Pgno, _: Pgno);
    fn sqlite3ExpirePreparedStatements(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3AnalysisLoad(
        _: *mut sqlite3,
        iDB: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3SchemaClear(_: *mut ::core::ffi::c_void);
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3RCStrRef(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sqlite3RCStrUnref(_: *mut ::core::ffi::c_void);
    fn sqlite3RCStrNew(_: u64_0) -> *mut ::core::ffi::c_char;
    fn sqlite3VtabLock(_: *mut VTable);
    fn sqlite3VtabUnlock(_: *mut VTable);
    fn sqlite3VtabSavepoint(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VtabImportErrmsg(_: *mut Vdbe, _: *mut sqlite3_vtab);
    fn sqlite3VtabCallCreate(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3VtabCallDestroy(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3VtabBegin(_: *mut sqlite3, _: *mut VTable) -> ::core::ffi::c_int;
    fn sqlite3JournalModename(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3Checkpoint(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3FkClearTriggerCache(_: *mut sqlite3, _: ::core::ffi::c_int);
    static sqlite3SmallTypeSizes: [u8_0; 0];
    fn sqlite3VdbeError(_: *mut Vdbe, _: *const ::core::ffi::c_char, ...);
    fn sqlite3VdbeFreeCursor(_: *mut Vdbe, _: *mut VdbeCursor);
    fn sqlite3VdbeFreeCursorNN(_: *mut Vdbe, _: *mut VdbeCursor);
    fn sqlite3VdbeHandleMovedCursor(p: *mut VdbeCursor) -> ::core::ffi::c_int;
    fn sqlite3VdbeFinishMoveto(_: *mut VdbeCursor) -> ::core::ffi::c_int;
    fn sqlite3VdbeCursorRestore(_: *mut VdbeCursor) -> ::core::ffi::c_int;
    fn sqlite3VdbeSerialTypeLen(_: u32_0) -> u32_0;
    fn sqlite3VdbeOneByteSerialTypeLen(_: u8_0) -> u8_0;
    fn sqlite3VdbeSerialGet(_: *const ::core::ffi::c_uchar, _: u32_0, _: *mut Mem);
    fn sqlite3VdbeDeleteAuxData(
        _: *mut sqlite3,
        _: *mut *mut AuxData,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3VdbeIdxKeyCompare(
        _: *mut sqlite3,
        _: *mut VdbeCursor,
        _: *mut UnpackedRecord,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeIdxRowid(
        _: *mut sqlite3,
        _: *mut BtCursor,
        _: *mut i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeHalt(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeChangeEncoding(
        _: *mut Mem,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemTooBig(_: *mut Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemCopy(_: *mut Mem, _: *const Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemShallowCopy(_: *mut Mem, _: *const Mem, _: ::core::ffi::c_int);
    fn sqlite3VdbeMemMove(_: *mut Mem, _: *mut Mem);
    fn sqlite3VdbeMemSetStr(
        _: *mut Mem,
        _: *const ::core::ffi::c_char,
        _: i64_0,
        _: u8_0,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemSetInt64(_: *mut Mem, _: i64_0);
    fn sqlite3VdbeMemSetPointer(
        _: *mut Mem,
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_char,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3VdbeMemInit(_: *mut Mem, _: *mut sqlite3, _: u16_0);
    fn sqlite3VdbeMemSetNull(_: *mut Mem);
    fn sqlite3VdbeMemSetZeroBlob(_: *mut Mem, _: ::core::ffi::c_int);
    fn sqlite3VdbeMemSetRowSet(_: *mut Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemMakeWriteable(_: *mut Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemStringify(_: *mut Mem, _: u8_0, _: u8_0) -> ::core::ffi::c_int;
    fn sqlite3IntFloatCompare(_: i64_0, _: ::core::ffi::c_double) -> ::core::ffi::c_int;
    fn sqlite3VdbeIntValue(_: *const Mem) -> i64_0;
    fn sqlite3VdbeMemIntegerify(_: *mut Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeRealValue(_: *mut Mem) -> ::core::ffi::c_double;
    fn sqlite3VdbeBooleanValue(
        _: *mut Mem,
        ifNull: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeIntegerAffinity(_: *mut Mem);
    fn sqlite3VdbeMemRealify(_: *mut Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemCast(_: *mut Mem, _: u8_0, _: u8_0) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemFromBtree(
        _: *mut BtCursor,
        _: u32_0,
        _: u32_0,
        _: *mut Mem,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemFromBtreeZeroOffset(
        _: *mut BtCursor,
        _: u32_0,
        _: *mut Mem,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemRelease(p: *mut Mem);
    fn sqlite3VdbeMemReleaseMalloc(p: *mut Mem);
    fn sqlite3VdbeMemFinalize(_: *mut Mem, _: *mut FuncDef) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemAggValue(
        _: *mut Mem,
        _: *mut Mem,
        _: *mut FuncDef,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemGrow(
        pMem: *mut Mem,
        n: ::core::ffi::c_int,
        preserve: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemClearAndResize(
        pMem: *mut Mem,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeFrameMemDel(_: *mut ::core::ffi::c_void);
    fn sqlite3VdbeFrameRestore(_: *mut VdbeFrame) -> ::core::ffi::c_int;
    fn sqlite3VdbeSorterInit(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *mut VdbeCursor,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeSorterReset(_: *mut sqlite3, _: *mut VdbeSorter);
    fn sqlite3VdbeSorterRowkey(_: *const VdbeCursor, _: *mut Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeSorterNext(
        _: *mut sqlite3,
        _: *const VdbeCursor,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeSorterRewind(
        _: *const VdbeCursor,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeSorterWrite(_: *const VdbeCursor, _: *mut Mem) -> ::core::ffi::c_int;
    fn sqlite3VdbeSorterCompare(
        _: *const VdbeCursor,
        _: *mut Mem,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeValueListFree(_: *mut ::core::ffi::c_void);
    fn sqlite3VdbeCheckFkImmediate(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeCheckFkDeferred(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeMemExpandBlob(_: *mut Mem) -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3 {
    pub pVfs: *mut sqlite3_vfs,
    pub pVdbe: *mut Vdbe,
    pub pDfltColl: *mut CollSeq,
    pub mutex: *mut sqlite3_mutex,
    pub aDb: *mut Db,
    pub nDb: ::core::ffi::c_int,
    pub mDbFlags: u32_0,
    pub flags: u64_0,
    pub lastRowid: i64_0,
    pub szMmap: i64_0,
    pub nSchemaLock: u32_0,
    pub openFlags: ::core::ffi::c_uint,
    pub errCode: ::core::ffi::c_int,
    pub errByteOffset: ::core::ffi::c_int,
    pub errMask: ::core::ffi::c_int,
    pub iSysErrno: ::core::ffi::c_int,
    pub dbOptFlags: u32_0,
    pub enc: u8_0,
    pub autoCommit: u8_0,
    pub temp_store: u8_0,
    pub mallocFailed: u8_0,
    pub bBenignMalloc: u8_0,
    pub dfltLockMode: u8_0,
    pub nextAutovac: ::core::ffi::c_schar,
    pub suppressErr: u8_0,
    pub vtabOnConflict: u8_0,
    pub isTransactionSavepoint: u8_0,
    pub mTrace: u8_0,
    pub noSharedCache: u8_0,
    pub nSqlExec: u8_0,
    pub eOpenState: u8_0,
    pub nextPagesize: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub nTotalChange: i64_0,
    pub aLimit: [::core::ffi::c_int; 12],
    pub nMaxSorterMmap: ::core::ffi::c_int,
    pub init: sqlite3InitInfo,
    pub nVdbeActive: ::core::ffi::c_int,
    pub nVdbeRead: ::core::ffi::c_int,
    pub nVdbeWrite: ::core::ffi::c_int,
    pub nVdbeExec: ::core::ffi::c_int,
    pub nVDestroy: ::core::ffi::c_int,
    pub nExtension: ::core::ffi::c_int,
    pub aExtension: *mut *mut ::core::ffi::c_void,
    pub trace: C2Rust_Unnamed_23,
    pub pTraceArg: *mut ::core::ffi::c_void,
    pub pCommitArg: *mut ::core::ffi::c_void,
    pub xCommitCallback: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub pRollbackArg: *mut ::core::ffi::c_void,
    pub xRollbackCallback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pUpdateArg: *mut ::core::ffi::c_void,
    pub xUpdateCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            sqlite_int64,
        ) -> (),
    >,
    pub pAutovacPagesArg: *mut ::core::ffi::c_void,
    pub xAutovacDestr: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub xAutovacPages: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            u32_0,
            u32_0,
            u32_0,
        ) -> ::core::ffi::c_uint,
    >,
    pub pParse: *mut Parse,
    pub xWalCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pWalArg: *mut ::core::ffi::c_void,
    pub xCollNeeded: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub xCollNeeded16: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> (),
    >,
    pub pCollNeededArg: *mut ::core::ffi::c_void,
    pub pErr: *mut sqlite3_value,
    pub u1: C2Rust_Unnamed_22,
    pub lookaside: Lookaside,
    pub xAuth: sqlite3_xauth,
    pub pAuthArg: *mut ::core::ffi::c_void,
    pub xProgress: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub pProgressArg: *mut ::core::ffi::c_void,
    pub nProgressOps: ::core::ffi::c_uint,
    pub nVTrans: ::core::ffi::c_int,
    pub aModule: Hash,
    pub pVtabCtx: *mut VtabCtx,
    pub aVTrans: *mut *mut VTable,
    pub pDisconnect: *mut VTable,
    pub aFunc: Hash,
    pub aCollSeq: Hash,
    pub busyHandler: BusyHandler,
    pub aDbStatic: [Db; 2],
    pub pSavepoint: *mut Savepoint,
    pub nAnalysisLimit: ::core::ffi::c_int,
    pub busyTimeout: ::core::ffi::c_int,
    pub nSavepoint: ::core::ffi::c_int,
    pub nStatement: ::core::ffi::c_int,
    pub nDeferredCons: i64_0,
    pub nDeferredImmCons: i64_0,
    pub pnBytesFreed: *mut ::core::ffi::c_int,
    pub pDbData: *mut DbClientData,
    pub nSpill: u64_0

}
pub type u64_0 = sqlite_uint64;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbClientData {
    pub pNext: *mut DbClientData,
    pub pData: *mut ::core::ffi::c_void,
    pub xDestructor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub zName: [::core::ffi::c_char; 0]

}
pub type i64_0 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Savepoint {
    pub zName: *mut ::core::ffi::c_char,
    pub nDeferredCons: i64_0,
    pub nDeferredImmCons: i64_0,
    pub pNext: *mut Savepoint

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Db {
    pub zDbSName: *mut ::core::ffi::c_char,
    pub pBt: *mut Btree,
    pub safety_level: u8_0,
    pub bSyncSet: u8_0,
    pub pSchema: *mut Schema

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Schema {
    pub schema_cookie: ::core::ffi::c_int,
    pub iGeneration: ::core::ffi::c_int,
    pub tblHash: Hash,
    pub idxHash: Hash,
    pub trigHash: Hash,
    pub fkeyHash: Hash,
    pub pSeqTab: *mut Table,
    pub file_format: u8_0,
    pub enc: u8_0,
    pub schemaFlags: u16_0,
    pub cache_size: ::core::ffi::c_int

}
pub type u16_0 = ::core::ffi::c_ushort;
pub type u8_0 = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub zName: *mut ::core::ffi::c_char,
    pub aCol: *mut Column,
    pub pIndex: *mut Index,
    pub zColAff: *mut ::core::ffi::c_char,
    pub pCheck: *mut ExprList,
    pub tnum: Pgno,
    pub nTabRef: u32_0,
    pub tabFlags: u32_0,
    pub iPKey: i16_0,
    pub nCol: i16_0,
    pub nNVCol: i16_0,
    pub nRowLogEst: LogEst,
    pub szTabRow: LogEst,
    pub keyConf: u8_0,
    pub eTabType: u8_0,
    pub u: C2Rust_Unnamed_18,
    pub pTrigger: *mut Trigger,
    pub pSchema: *mut Schema,
    pub aHx: [u8_0; 16]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Trigger {
    pub zName: *mut ::core::ffi::c_char,
    pub table: *mut ::core::ffi::c_char,
    pub op: u8_0,
    pub tr_tm: u8_0,
    pub bReturning: u8_0,
    pub pWhen: *mut Expr,
    pub pColumns: *mut IdList,
    pub pSchema: *mut Schema,
    pub pTabSchema: *mut Schema,
    pub step_list: *mut TriggerStep,
    pub pNext: *mut Trigger

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerStep {
    pub op: u8_0,
    pub orconf: u8_0,
    pub pTrig: *mut Trigger,
    pub pSelect: *mut Select,
    pub zTarget: *mut ::core::ffi::c_char,
    pub pFrom: *mut SrcList,
    pub pWhere: *mut Expr,
    pub pExprList: *mut ExprList,
    pub pIdList: *mut IdList,
    pub pUpsert: *mut Upsert,
    pub zSpan: *mut ::core::ffi::c_char,
    pub pNext: *mut TriggerStep,
    pub pLast: *mut TriggerStep

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upsert {
    pub pUpsertTarget: *mut ExprList,
    pub pUpsertTargetWhere: *mut Expr,
    pub pUpsertSet: *mut ExprList,
    pub pUpsertWhere: *mut Expr,
    pub pNextUpsert: *mut Upsert,
    pub isDoUpdate: u8_0,
    pub isDup: u8_0,
    pub pToFree: *mut ::core::ffi::c_void,
    pub pUpsertIdx: *mut Index,
    pub pUpsertSrc: *mut SrcList,
    pub regData: ::core::ffi::c_int,
    pub iDataCur: ::core::ffi::c_int,
    pub iIdxCur: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SrcList {
    pub nSrc: ::core::ffi::c_int,
    pub nAlloc: u32_0,
    pub a: [SrcItem; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SrcItem {
    pub zName: *mut ::core::ffi::c_char,
    pub zAlias: *mut ::core::ffi::c_char,
    pub pSTab: *mut Table,
    pub fg: C2Rust_Unnamed_17,
    pub iCursor: ::core::ffi::c_int,
    pub colUsed: Bitmask,
    pub u1: C2Rust_Unnamed_16,
    pub u2: C2Rust_Unnamed_15,
    pub u3: C2Rust_Unnamed_14,
    pub u4: C2Rust_Unnamed

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub pSchema: *mut Schema,
    pub zDatabase: *mut ::core::ffi::c_char,
    pub pSubq: *mut Subquery

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Subquery {
    pub pSelect: *mut Select,
    pub addrFillSub: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int,
    pub regResult: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Select {
    pub op: u8_0,
    pub nSelectRow: LogEst,
    pub selFlags: u32_0,
    pub iLimit: ::core::ffi::c_int,
    pub iOffset: ::core::ffi::c_int,
    pub selId: u32_0,
    pub addrOpenEphm: [::core::ffi::c_int; 2],
    pub pEList: *mut ExprList,
    pub pSrc: *mut SrcList,
    pub pWhere: *mut Expr,
    pub pGroupBy: *mut ExprList,
    pub pHaving: *mut Expr,
    pub pOrderBy: *mut ExprList,
    pub pPrior: *mut Select,
    pub pNext: *mut Select,
    pub pLimit: *mut Expr,
    pub pWith: *mut With,
    pub pWin: *mut Window,
    pub pWinDefn: *mut Window

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Window {
    pub zName: *mut ::core::ffi::c_char,
    pub zBase: *mut ::core::ffi::c_char,
    pub pPartition: *mut ExprList,
    pub pOrderBy: *mut ExprList,
    pub eFrmType: u8_0,
    pub eStart: u8_0,
    pub eEnd: u8_0,
    pub bImplicitFrame: u8_0,
    pub eExclude: u8_0,
    pub pStart: *mut Expr,
    pub pEnd: *mut Expr,
    pub ppThis: *mut *mut Window,
    pub pNextWin: *mut Window,
    pub pFilter: *mut Expr,
    pub pWFunc: *mut FuncDef,
    pub iEphCsr: ::core::ffi::c_int,
    pub regAccum: ::core::ffi::c_int,
    pub regResult: ::core::ffi::c_int,
    pub csrApp: ::core::ffi::c_int,
    pub regApp: ::core::ffi::c_int,
    pub regPart: ::core::ffi::c_int,
    pub pOwner: *mut Expr,
    pub nBufferCol: ::core::ffi::c_int,
    pub iArgCol: ::core::ffi::c_int,
    pub regOne: ::core::ffi::c_int,
    pub regStartRowid: ::core::ffi::c_int,
    pub regEndRowid: ::core::ffi::c_int,
    pub bExprArgs: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Expr {
    pub op: u8_0,
    pub affExpr: ::core::ffi::c_char,
    pub op2: u8_0,
    pub flags: u32_0,
    pub u: C2Rust_Unnamed_13,
    pub pLeft: *mut Expr,
    pub pRight: *mut Expr,
    pub x: C2Rust_Unnamed_12,
    pub nHeight: ::core::ffi::c_int,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ynVar,
    pub iAgg: i16_0,
    pub w: C2Rust_Unnamed_11,
    pub pAggInfo: *mut AggInfo,
    pub y: C2Rust_Unnamed_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub pTab: *mut Table,
    pub pWin: *mut Window,
    pub nReg: ::core::ffi::c_int,
    pub sub: C2Rust_Unnamed_1

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub iAddr: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AggInfo {
    pub directMode: u8_0,
    pub useSortingIdx: u8_0,
    pub nSortingColumn: u32_0,
    pub sortingIdx: ::core::ffi::c_int,
    pub sortingIdxPTab: ::core::ffi::c_int,
    pub iFirstReg: ::core::ffi::c_int,
    pub pGroupBy: *mut ExprList,
    pub aCol: *mut AggInfo_col,
    pub nColumn: ::core::ffi::c_int,
    pub nAccumulator: ::core::ffi::c_int,
    pub aFunc: *mut AggInfo_func,
    pub nFunc: ::core::ffi::c_int,
    pub selId: u32_0

}
pub type u32_0 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AggInfo_func {
    pub pFExpr: *mut Expr,
    pub pFunc: *mut FuncDef,
    pub iDistinct: ::core::ffi::c_int,
    pub iDistAddr: ::core::ffi::c_int,
    pub iOBTab: ::core::ffi::c_int,
    pub bOBPayload: u8_0,
    pub bOBUnique: u8_0,
    pub bUseSubtype: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncDef {
    pub nArg: i16_0,
    pub funcFlags: u32_0,
    pub pUserData: *mut ::core::ffi::c_void,
    pub pNext: *mut FuncDef,
    pub xSFunc: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    pub xFinalize: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    pub xValue: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    pub xInverse: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    pub zName: *const ::core::ffi::c_char,
    pub u: C2Rust_Unnamed_2

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub pHash: *mut FuncDef,
    pub pDestructor: *mut FuncDestructor

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncDestructor {
    pub nRef: ::core::ffi::c_int,
    pub xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pUserData: *mut ::core::ffi::c_void

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_value {
    pub u: MemValue,
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub flags: u16_0,
    pub enc: u8_0,
    pub eSubtype: u8_0,
    pub db: *mut sqlite3,
    pub szMalloc: ::core::ffi::c_int,
    pub uTemp: u32_0,
    pub zMalloc: *mut ::core::ffi::c_char,
    pub xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union MemValue {
    pub r: ::core::ffi::c_double,
    pub i: i64_0,
    pub nZero: ::core::ffi::c_int,
    pub zPType: *const ::core::ffi::c_char,
    pub pDef: *mut FuncDef

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_context {
    pub pOut: *mut Mem,
    pub pFunc: *mut FuncDef,
    pub pMem: *mut Mem,
    pub pVdbe: *mut Vdbe,
    pub iOp: ::core::ffi::c_int,
    pub isError: ::core::ffi::c_int,
    pub enc: u8_0,
    pub skipFlag: u8_0,
    pub argc: u16_0,
    pub argv: [*mut sqlite3_value; 0]

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Vdbe {
    pub db: *mut sqlite3,
    pub ppVPrev: *mut *mut Vdbe,
    pub pVNext: *mut Vdbe,
    pub pParse: *mut Parse,
    pub nVar: ynVar,
    pub nMem: ::core::ffi::c_int,
    pub nCursor: ::core::ffi::c_int,
    pub cacheCtr: u32_0,
    pub pc: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub iStatement: ::core::ffi::c_int,
    pub iCurrentTime: i64_0,
    pub nFkConstraint: i64_0,
    pub nStmtDefCons: i64_0,
    pub nStmtDefImmCons: i64_0,
    pub aMem: *mut Mem,
    pub apArg: *mut *mut Mem,
    pub apCsr: *mut *mut VdbeCursor,
    pub aVar: *mut Mem,
    pub aOp: *mut Op,
    pub nOp: ::core::ffi::c_int,
    pub nOpAlloc: ::core::ffi::c_int,
    pub aColName: *mut Mem,
    pub pResultRow: *mut Mem,
    pub zErrMsg: *mut ::core::ffi::c_char,
    pub pVList: *mut VList,
    pub startTime: i64_0,
    pub nResColumn: u16_0,
    pub nResAlloc: u16_0,
    pub errorAction: u8_0,
    pub minWriteFileFormat: u8_0,
    pub prepFlags: u8_0,
    pub eVdbeState: u8_0,
    #[bitfield(name = "expired", ty = "bft", bits = "0..=1")]
    #[bitfield(name = "explain", ty = "bft", bits = "2..=3")]
    #[bitfield(name = "changeCntOn", ty = "bft", bits = "4..=4")]
    #[bitfield(name = "usesStmtJournal", ty = "bft", bits = "5..=5")]
    #[bitfield(name = "readOnly", ty = "bft", bits = "6..=6")]
    #[bitfield(name = "bIsReader", ty = "bft", bits = "7..=7")]
    #[bitfield(name = "haveEqpOps", ty = "bft", bits = "8..=8")]
    pub expired_explain_changeCntOn_usesStmtJournal_readOnly_bIsReader_haveEqpOps: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub btreeMask: yDbMask,
    pub lockMask: yDbMask,
    pub aCounter: [u32_0; 9],
    pub zSql: *mut ::core::ffi::c_char,
    pub pFree: *mut ::core::ffi::c_void,
    pub pFrame: *mut VdbeFrame,
    pub pDelFrame: *mut VdbeFrame,
    pub nFrame: ::core::ffi::c_int,
    pub expmask: u32_0,
    pub pProgram: *mut SubProgram,
    pub pAuxData: *mut AuxData

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AuxData {
    pub iAuxOp: ::core::ffi::c_int,
    pub iAuxArg: ::core::ffi::c_int,
    pub pAux: *mut ::core::ffi::c_void,
    pub xDeleteAux: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pNextAux: *mut AuxData

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubProgram {
    pub aOp: *mut VdbeOp,
    pub nOp: ::core::ffi::c_int,
    pub nMem: ::core::ffi::c_int,
    pub nCsr: ::core::ffi::c_int,
    pub aOnce: *mut u8_0,
    pub token: *mut ::core::ffi::c_void,
    pub pNext: *mut SubProgram

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeOp {
    pub opcode: u8_0,
    pub p4type: ::core::ffi::c_schar,
    pub p5: u16_0,
    pub p1: ::core::ffi::c_int,
    pub p2: ::core::ffi::c_int,
    pub p3: ::core::ffi::c_int,
    pub p4: p4union

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union p4union {
    pub i: ::core::ffi::c_int,
    pub p: *mut ::core::ffi::c_void,
    pub z: *mut ::core::ffi::c_char,
    pub pI64: *mut i64_0,
    pub pReal: *mut ::core::ffi::c_double,
    pub pFunc: *mut FuncDef,
    pub pCtx: *mut sqlite3_context,
    pub pColl: *mut CollSeq,
    pub pMem: *mut Mem,
    pub pVtab: *mut VTable,
    pub pKeyInfo: *mut KeyInfo,
    pub ai: *mut u32_0,
    pub pProgram: *mut SubProgram,
    pub pTab: *mut Table,
    pub pSubrtnSig: *mut SubrtnSig

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubrtnSig {
    pub selId: ::core::ffi::c_int,
    pub bComplete: u8_0,
    pub zAff: *mut ::core::ffi::c_char,
    pub iTable: ::core::ffi::c_int,
    pub iAddr: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyInfo {
    pub nRef: u32_0,
    pub enc: u8_0,
    pub nKeyField: u16_0,
    pub nAllField: u16_0,
    pub db: *mut sqlite3,
    pub aSortFlags: *mut u8_0,
    pub aColl: [*mut CollSeq; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollSeq {
    pub zName: *mut ::core::ffi::c_char,
    pub enc: u8_0,
    pub pUser: *mut ::core::ffi::c_void,
    pub xCmp: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VTable {
    pub db: *mut sqlite3,
    pub pMod: *mut Module,
    pub pVtab: *mut sqlite3_vtab,
    pub nRef: ::core::ffi::c_int,
    pub bConstraint: u8_0,
    pub bAllSchemas: u8_0,
    pub eVtabRisk: u8_0,
    pub iSavepoint: ::core::ffi::c_int,
    pub pNext: *mut VTable

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab {
    pub pModule: *const sqlite3_module,
    pub nRef: ::core::ffi::c_int,
    pub zErrMsg: *mut ::core::ffi::c_char

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_module {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xConnect: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xBestIndex: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut sqlite3_index_info,
        ) -> ::core::ffi::c_int,
    >,
    pub xDisconnect: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut *mut sqlite3_vtab_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xFilter: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub xNext: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xEof: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xColumn: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_context,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRowid: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xUpdate: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xBegin: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xSync: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xCommit: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xRollback: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xFindFunction: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *mut Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xRename: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xSavepoint: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xRelease: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xRollbackTo: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xShadowName: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    pub xIntegrity: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >

}
pub type sqlite3_int64 = sqlite_int64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_info {
    pub nConstraint: ::core::ffi::c_int,
    pub aConstraint: *mut sqlite3_index_constraint,
    pub nOrderBy: ::core::ffi::c_int,
    pub aOrderBy: *mut sqlite3_index_orderby,
    pub aConstraintUsage: *mut sqlite3_index_constraint_usage,
    pub idxNum: ::core::ffi::c_int,
    pub idxStr: *mut ::core::ffi::c_char,
    pub needToFreeIdxStr: ::core::ffi::c_int,
    pub orderByConsumed: ::core::ffi::c_int,
    pub estimatedCost: ::core::ffi::c_double,
    pub estimatedRows: sqlite3_int64,
    pub idxFlags: ::core::ffi::c_int,
    pub colUsed: sqlite3_uint64

}
pub type sqlite3_uint64 = sqlite_uint64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint_usage {
    pub argvIndex: ::core::ffi::c_int,
    pub omit: ::core::ffi::c_uchar

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_orderby {
    pub iColumn: ::core::ffi::c_int,
    pub desc: ::core::ffi::c_uchar

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint {
    pub iColumn: ::core::ffi::c_int,
    pub op: ::core::ffi::c_uchar,
    pub usable: ::core::ffi::c_uchar,
    pub iTermOffset: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Module {
    pub pModule: *const sqlite3_module,
    pub zName: *const ::core::ffi::c_char,
    pub nRefModule: ::core::ffi::c_int,
    pub pAux: *mut ::core::ffi::c_void,
    pub xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pEpoTab: *mut Table

}
pub type Mem = sqlite3_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeFrame {
    pub v: *mut Vdbe,
    pub pParent: *mut VdbeFrame,
    pub aOp: *mut Op,
    pub aMem: *mut Mem,
    pub apCsr: *mut *mut VdbeCursor,
    pub aOnce: *mut u8_0,
    pub token: *mut ::core::ffi::c_void,
    pub lastRowid: i64_0,
    pub pAuxData: *mut AuxData,
    pub nCursor: ::core::ffi::c_int,
    pub pc: ::core::ffi::c_int,
    pub nOp: ::core::ffi::c_int,
    pub nMem: ::core::ffi::c_int,
    pub nChildMem: ::core::ffi::c_int,
    pub nChildCsr: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub nDbChange: i64_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VdbeCursor {
    pub eCurType: u8_0,
    pub iDb: i8_0,
    pub nullRow: u8_0,
    pub deferredMoveto: u8_0,
    pub isTable: u8_0,
    #[bitfield(name = "isEphemeral", ty = "Bool", bits = "0..=0")]
    #[bitfield(name = "useRandomRowid", ty = "Bool", bits = "1..=1")]
    #[bitfield(name = "isOrdered", ty = "Bool", bits = "2..=2")]
    #[bitfield(name = "noReuse", ty = "Bool", bits = "3..=3")]
    #[bitfield(name = "colCache", ty = "Bool", bits = "4..=4")]
    pub isEphemeral_useRandomRowid_isOrdered_noReuse_colCache: [u8; 1],
    pub seekHit: u16_0,
    pub ub: C2Rust_Unnamed_4,
    pub seqCount: i64_0,
    pub cacheStatus: u32_0,
    pub seekResult: ::core::ffi::c_int,
    pub pAltCursor: *mut VdbeCursor,
    pub uc: C2Rust_Unnamed_3,
    pub pKeyInfo: *mut KeyInfo,
    pub iHdrOffset: u32_0,
    pub pgnoRoot: Pgno,
    pub nField: i16_0,
    pub nHdrParsed: u16_0,
    pub movetoTarget: i64_0,
    pub aOffset: *mut u32_0,
    pub aRow: *const u8_0,
    pub payloadSize: u32_0,
    pub szRow: u32_0,
    pub pCache: *mut VdbeTxtBlbCache,
    pub aType: [u32_0; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeTxtBlbCache {
    pub pCValue: *mut ::core::ffi::c_char,
    pub iOffset: i64_0,
    pub iCol: ::core::ffi::c_int,
    pub cacheStatus: u32_0,
    pub colCacheCtr: u32_0

}
pub type i16_0 = ::core::ffi::c_short;
pub type Pgno = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub pCursor: *mut BtCursor,
    pub pVCur: *mut sqlite3_vtab_cursor,
    pub pSorter: *mut VdbeSorter

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub pBtx: *mut Btree,
    pub aAltMap: *mut u32_0

}
pub type Bool = ::core::ffi::c_uint;
pub type i8_0 = ::core::ffi::c_schar;
pub type Op = VdbeOp;
pub type yDbMask = ::core::ffi::c_uint;
pub type bft = ::core::ffi::c_uint;
pub type VList = ::core::ffi::c_int;
pub type ynVar = i16_0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Parse {
    pub db: *mut sqlite3,
    pub zErrMsg: *mut ::core::ffi::c_char,
    pub pVdbe: *mut Vdbe,
    pub rc: ::core::ffi::c_int,
    pub nQueryLoop: LogEst,
    pub nested: u8_0,
    pub nTempReg: u8_0,
    pub isMultiWrite: u8_0,
    pub mayAbort: u8_0,
    pub hasCompound: u8_0,
    pub disableLookaside: u8_0,
    pub prepFlags: u8_0,
    pub withinRJSubrtn: u8_0,
    pub bHasExists: u8_0,
    pub mSubrtnSig: u8_0,
    pub eTriggerOp: u8_0,
    pub bReturning: u8_0,
    pub eOrconf: u8_0,
    pub disableTriggers: u8_0,
    #[bitfield(name = "colNamesSet", ty = "bft", bits = "0..=0")]
    #[bitfield(name = "bHasWith", ty = "bft", bits = "1..=1")]
    #[bitfield(name = "okConstFactor", ty = "bft", bits = "2..=2")]
    #[bitfield(name = "checkSchema", ty = "bft", bits = "3..=3")]
    pub colNamesSet_bHasWith_okConstFactor_checkSchema: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub nRangeReg: ::core::ffi::c_int,
    pub iRangeReg: ::core::ffi::c_int,
    pub nErr: ::core::ffi::c_int,
    pub nTab: ::core::ffi::c_int,
    pub nMem: ::core::ffi::c_int,
    pub szOpAlloc: ::core::ffi::c_int,
    pub iSelfTab: ::core::ffi::c_int,
    pub nLabel: ::core::ffi::c_int,
    pub nLabelAlloc: ::core::ffi::c_int,
    pub aLabel: *mut ::core::ffi::c_int,
    pub pConstExpr: *mut ExprList,
    pub pIdxEpr: *mut IndexedExpr,
    pub pIdxPartExpr: *mut IndexedExpr,
    pub writeMask: yDbMask,
    pub cookieMask: yDbMask,
    pub nMaxArg: ::core::ffi::c_int,
    pub nSelect: ::core::ffi::c_int,
    pub nProgressSteps: u32_0,
    pub pAinc: *mut AutoincInfo,
    pub pToplevel: *mut Parse,
    pub pTriggerTab: *mut Table,
    pub pTriggerPrg: *mut TriggerPrg,
    pub pCleanup: *mut ParseCleanup,
    pub aTempReg: [::core::ffi::c_int; 8],
    pub pOuterParse: *mut Parse,
    pub sNameToken: Token,
    pub oldmask: u32_0,
    pub newmask: u32_0,
    pub u1: C2Rust_Unnamed_8,
    pub sLastToken: Token,
    pub nVar: ynVar,
    pub iPkSortOrder: u8_0,
    pub explain: u8_0,
    pub eParseMode: u8_0,
    pub nVtabLock: ::core::ffi::c_int,
    pub nHeight: ::core::ffi::c_int,
    pub addrExplain: ::core::ffi::c_int,
    pub pVList: *mut VList,
    pub pReprepare: *mut Vdbe,
    pub zTail: *const ::core::ffi::c_char,
    pub pNewTable: *mut Table,
    pub pNewIndex: *mut Index,
    pub pNewTrigger: *mut Trigger,
    pub zAuthContext: *const ::core::ffi::c_char,
    pub sArg: Token,
    pub apVtabLock: *mut *mut Table,
    pub pWith: *mut With,
    pub pRename: *mut RenameToken

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct With {
    pub nCte: ::core::ffi::c_int,
    pub bView: ::core::ffi::c_int,
    pub pOuter: *mut With,
    pub a: [Cte; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cte {
    pub zName: *mut ::core::ffi::c_char,
    pub pCols: *mut ExprList,
    pub pSelect: *mut Select,
    pub zCteErr: *const ::core::ffi::c_char,
    pub pUse: *mut CteUse,
    pub eM10d: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CteUse {
    pub nUse: ::core::ffi::c_int,
    pub addrM9e: ::core::ffi::c_int,
    pub regRtn: ::core::ffi::c_int,
    pub iCur: ::core::ffi::c_int,
    pub nRowEst: LogEst,
    pub eM10d: u8_0

}
pub type LogEst = ::core::ffi::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprList {
    pub nExpr: ::core::ffi::c_int,
    pub nAlloc: ::core::ffi::c_int,
    pub a: [ExprList_item; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprList_item {
    pub pExpr: *mut Expr,
    pub zEName: *mut ::core::ffi::c_char,
    pub fg: C2Rust_Unnamed_7,
    pub u: C2Rust_Unnamed_5

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub x: C2Rust_Unnamed_6,
    pub iConstExprReg: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub iOrderByCol: u16_0,
    pub iAlias: u16_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub sortFlags: u8_0,
    #[bitfield(name = "eEName", ty = "::core::ffi::c_uint", bits = "0..=1")]
    #[bitfield(name = "done", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "reusable", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "bSorterRef", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "bNulls", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "bUsed", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "bUsingTerm", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "bNoExpand", ty = "::core::ffi::c_uint", bits = "8..=8")]
    pub eEName_done_reusable_bSorterRef_bNulls_bUsed_bUsingTerm_bNoExpand: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub z: *const ::core::ffi::c_char,
    pub n: ::core::ffi::c_uint

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Index {
    pub zName: *mut ::core::ffi::c_char,
    pub aiColumn: *mut i16_0,
    pub aiRowLogEst: *mut LogEst,
    pub pTable: *mut Table,
    pub zColAff: *mut ::core::ffi::c_char,
    pub pNext: *mut Index,
    pub pSchema: *mut Schema,
    pub aSortOrder: *mut u8_0,
    pub azColl: *mut *const ::core::ffi::c_char,
    pub pPartIdxWhere: *mut Expr,
    pub aColExpr: *mut ExprList,
    pub tnum: Pgno,
    pub szIdxRow: LogEst,
    pub nKeyCol: u16_0,
    pub nColumn: u16_0,
    pub onError: u8_0,
    #[bitfield(name = "idxType", ty = "::core::ffi::c_uint", bits = "0..=1")]
    #[bitfield(name = "bUnordered", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "uniqNotNull", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "isResized", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "isCovering", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "noSkipScan", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "hasStat1", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "bNoQuery", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "bAscKeyBug", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "bHasVCol", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(name = "bHasExpr", ty = "::core::ffi::c_uint", bits = "11..=11")]
    pub idxType_bUnordered_uniqNotNull_isResized_isCovering_noSkipScan_hasStat1_bNoQuery_bAscKeyBug_bHasVCol_bHasExpr: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub colNotIdxed: Bitmask

}
pub type Bitmask = u64_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub cr: C2Rust_Unnamed_10,
    pub d: C2Rust_Unnamed_9

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub pReturning: *mut Returning

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Returning {
    pub pParse: *mut Parse,
    pub pReturnEL: *mut ExprList,
    pub retTrig: Trigger,
    pub retTStep: TriggerStep,
    pub iRetCur: ::core::ffi::c_int,
    pub nRetCol: ::core::ffi::c_int,
    pub iRetReg: ::core::ffi::c_int,
    pub zName: [::core::ffi::c_char; 40]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub addrCrTab: ::core::ffi::c_int,
    pub regRowid: ::core::ffi::c_int,
    pub regRoot: ::core::ffi::c_int,
    pub constraintName: Token

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseCleanup {
    pub pNext: *mut ParseCleanup,
    pub pPtr: *mut ::core::ffi::c_void,
    pub xCleanup: Option<
        unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> (),
    >

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerPrg {
    pub pTrigger: *mut Trigger,
    pub pNext: *mut TriggerPrg,
    pub pProgram: *mut SubProgram,
    pub orconf: ::core::ffi::c_int,
    pub aColmask: [u32_0; 2]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AutoincInfo {
    pub pNext: *mut AutoincInfo,
    pub pTab: *mut Table,
    pub iDb: ::core::ffi::c_int,
    pub regCtr: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexedExpr {
    pub pExpr: *mut Expr,
    pub iDataCur: ::core::ffi::c_int,
    pub iIdxCur: ::core::ffi::c_int,
    pub iIdxCol: ::core::ffi::c_int,
    pub bMaybeNullRow: u8_0,
    pub aff: u8_0,
    pub pIENext: *mut IndexedExpr

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AggInfo_col {
    pub pTab: *mut Table,
    pub pCExpr: *mut Expr,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ::core::ffi::c_int,
    pub iSorterColumn: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub iJoin: ::core::ffi::c_int,
    pub iOfst: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub pList: *mut ExprList,
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_13 {
    pub zToken: *mut ::core::ffi::c_char,
    pub iValue: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_14 {
    pub pOn: *mut Expr,
    pub pUsing: *mut IdList

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdList {
    pub nId: ::core::ffi::c_int,
    pub a: [IdList_item; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdList_item {
    pub zName: *mut ::core::ffi::c_char

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_15 {
    pub pIBIndex: *mut Index,
    pub pCteUse: *mut CteUse

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_16 {
    pub zIndexedBy: *mut ::core::ffi::c_char,
    pub pFuncArg: *mut ExprList,
    pub nRow: u32_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub jointype: u8_0,
    #[bitfield(name = "notIndexed", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "isIndexedBy", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "isSubquery", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "isTabFunc", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "isCorrelated", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "isMaterialized", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "viaCoroutine", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "isRecursive", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "fromDDL", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "isCte", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "notCte", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(name = "isUsing", ty = "::core::ffi::c_uint", bits = "11..=11")]
    #[bitfield(name = "isOn", ty = "::core::ffi::c_uint", bits = "12..=12")]
    #[bitfield(name = "isSynthUsing", ty = "::core::ffi::c_uint", bits = "13..=13")]
    #[bitfield(name = "isNestedFrom", ty = "::core::ffi::c_uint", bits = "14..=14")]
    #[bitfield(name = "rowidUsed", ty = "::core::ffi::c_uint", bits = "15..=15")]
    #[bitfield(name = "fixedSchema", ty = "::core::ffi::c_uint", bits = "16..=16")]
    #[bitfield(name = "hadSchema", ty = "::core::ffi::c_uint", bits = "17..=17")]
    #[bitfield(name = "fromExists", ty = "::core::ffi::c_uint", bits = "18..=18")]
    pub notIndexed_isIndexedBy_isSubquery_isTabFunc_isCorrelated_isMaterialized_viaCoroutine_isRecursive_fromDDL_isCte_notCte_isUsing_isOn_isSynthUsing_isNestedFrom_rowidUsed_fixedSchema_hadSchema_fromExists: [u8; 3]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_18 {
    pub tab: C2Rust_Unnamed_21,
    pub view: C2Rust_Unnamed_20,
    pub vtab: C2Rust_Unnamed_19

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub nArg: ::core::ffi::c_int,
    pub azArg: *mut *mut ::core::ffi::c_char,
    pub p: *mut VTable

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub addColOffset: ::core::ffi::c_int,
    pub pFKey: *mut FKey,
    pub pDfltList: *mut ExprList

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FKey {
    pub pFrom: *mut Table,
    pub pNextFrom: *mut FKey,
    pub zTo: *mut ::core::ffi::c_char,
    pub pNextTo: *mut FKey,
    pub pPrevTo: *mut FKey,
    pub nCol: ::core::ffi::c_int,
    pub isDeferred: u8_0,
    pub aAction: [u8_0; 2],
    pub apTrigger: [*mut Trigger; 2],
    pub aCol: [sColMap; 0]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sColMap {
    pub iFrom: ::core::ffi::c_int,
    pub zCol: *mut ::core::ffi::c_char

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Column {
    pub zCnName: *mut ::core::ffi::c_char,
    #[bitfield(name = "notNull", ty = "::core::ffi::c_uint", bits = "0..=3")]
    #[bitfield(name = "eCType", ty = "::core::ffi::c_uint", bits = "4..=7")]
    pub notNull_eCType: [u8; 1],
    pub affinity: ::core::ffi::c_char,
    pub szEst: u8_0,
    pub hName: u8_0,
    pub iDflt: u16_0,
    pub colFlags: u16_0

}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BusyHandler {
    pub xBusyHandler: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pBusyArg: *mut ::core::ffi::c_void,
    pub nBusy: ::core::ffi::c_int

}
pub type sqlite3_xauth = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lookaside {
    pub bDisable: u32_0,
    pub sz: u16_0,
    pub szTrue: u16_0,
    pub bMalloced: u8_0,
    pub nSlot: u32_0,
    pub anStat: [u32_0; 3],
    pub pInit: *mut LookasideSlot,
    pub pFree: *mut LookasideSlot,
    pub pSmallInit: *mut LookasideSlot,
    pub pSmallFree: *mut LookasideSlot,
    pub pMiddle: *mut ::core::ffi::c_void,
    pub pStart: *mut ::core::ffi::c_void,
    pub pEnd: *mut ::core::ffi::c_void,
    pub pTrueEnd: *mut ::core::ffi::c_void

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookasideSlot {
    pub pNext: *mut LookasideSlot

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_22 {
    pub isInterrupted: ::core::ffi::c_int,
    pub notUsed1: ::core::ffi::c_double

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_23 {
    pub xLegacy: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> (),
    >,
    pub xV2: Option<
        unsafe extern "C" fn(
            u32_0,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct sqlite3InitInfo {
    pub newTnum: Pgno,
    pub iDb: u8_0,
    pub busy: u8_0,
    #[bitfield(name = "orphanTrigger", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "imposterTable", ty = "::core::ffi::c_uint", bits = "1..=2")]
    #[bitfield(name = "reopenMemdb", ty = "::core::ffi::c_uint", bits = "3..=3")]
    pub orphanTrigger_imposterTable_reopenMemdb: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub azInit: *mut *const ::core::ffi::c_char

}
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
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnpackedRecord {
    pub pKeyInfo: *mut KeyInfo,
    pub aMem: *mut Mem,
    pub u: C2Rust_Unnamed_24,
    pub n: ::core::ffi::c_int,
    pub nField: u16_0,
    pub default_rc: i8_0,
    pub errCode: u8_0,
    pub r1: i8_0,
    pub r2: i8_0,
    pub eqSeen: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_24 {
    pub z: *mut ::core::ffi::c_char,
    pub i: i64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BtreePayload {
    pub pKey: *const ::core::ffi::c_void,
    pub nKey: sqlite3_int64,
    pub pData: *const ::core::ffi::c_void,
    pub aMem: *mut sqlite3_value,
    pub nMem: u16_0,
    pub nData: ::core::ffi::c_int,
    pub nZero: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InitData {
    pub db: *mut sqlite3,
    pub pzErrMsg: *mut *mut ::core::ffi::c_char,
    pub iDb: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub mInitFlags: u32_0,
    pub nInitRow: u32_0,
    pub mxPage: Pgno

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
pub struct ValueList {
    pub pCsr: *mut BtCursor,
    pub pOut: *mut sqlite3_value

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_ABORT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_LOCKED: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_READONLY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_INTERRUPT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_FULL: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SQLITE_SCHEMA: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_MISMATCH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_CORRUPTFS: ::core::ffi::c_int = SQLITE_IOERR
    | (33 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT_INDEX: ::core::ffi::c_int = SQLITE_CORRUPT
    | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_ABORT_ROLLBACK: ::core::ffi::c_int = SQLITE_ABORT
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_DATATYPE: ::core::ffi::c_int = SQLITE_CONSTRAINT
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_DELETEONCLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_EXCLUSIVE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_OPEN_TRANSIENT_DB: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SQLITE_DELETE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_INSERT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_UPDATE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const SQLITE_TRACE_STMT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_TRACE_ROW: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LENGTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_TRIGGER_DEPTH: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_RESULT_SUBTYPE: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_STMTSTATUS_SORT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_VM_STEP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_RUN: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_FILTER_MISS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_FILTER_HIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const OP_Savepoint: ::core::ffi::c_int = 0;
pub const OP_AutoCommit: ::core::ffi::c_int = 1;
pub const OP_Transaction: ::core::ffi::c_int = 2;
pub const OP_Checkpoint: ::core::ffi::c_int = 3;
pub const OP_JournalMode: ::core::ffi::c_int = 4;
pub const OP_Vacuum: ::core::ffi::c_int = 5;
pub const OP_VFilter: ::core::ffi::c_int = 6;
pub const OP_VUpdate: ::core::ffi::c_int = 7;
pub const OP_Init: ::core::ffi::c_int = 8;
pub const OP_Goto: ::core::ffi::c_int = 9;
pub const OP_Gosub: ::core::ffi::c_int = 10;
pub const OP_InitCoroutine: ::core::ffi::c_int = 11;
pub const OP_Yield: ::core::ffi::c_int = 12;
pub const OP_MustBeInt: ::core::ffi::c_int = 13;
pub const OP_Jump: ::core::ffi::c_int = 14;
pub const OP_Once: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const OP_If: ::core::ffi::c_int = 16;
pub const OP_IfNot: ::core::ffi::c_int = 17;
pub const OP_IsType: ::core::ffi::c_int = 18;
pub const OP_Not: ::core::ffi::c_int = 19;
pub const OP_IfNullRow: ::core::ffi::c_int = 20;
pub const OP_SeekLT: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const OP_SeekLE: ::core::ffi::c_int = 22;
pub const OP_SeekGE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const OP_SeekGT: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const OP_IfNotOpen: ::core::ffi::c_int = 25;
pub const OP_IfNoHope: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const OP_NoConflict: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const OP_NotFound: ::core::ffi::c_int = 28;
pub const OP_Found: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const OP_SeekRowid: ::core::ffi::c_int = 30;
pub const OP_NotExists: ::core::ffi::c_int = 31;
pub const OP_Last: ::core::ffi::c_int = 32;
pub const OP_IfSizeBetween: ::core::ffi::c_int = 33;
pub const OP_SorterSort: ::core::ffi::c_int = 34;
pub const OP_Sort: ::core::ffi::c_int = 35;
pub const OP_Rewind: ::core::ffi::c_int = 36;
pub const OP_IfEmpty: ::core::ffi::c_int = 37;
pub const OP_SorterNext: ::core::ffi::c_int = 38;
pub const OP_Prev: ::core::ffi::c_int = 39;
pub const OP_Next: ::core::ffi::c_int = 40;
pub const OP_IdxLE: ::core::ffi::c_int = 41;
pub const OP_IdxGT: ::core::ffi::c_int = 42;
pub const OP_Or: ::core::ffi::c_int = 43;
pub const OP_And: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const OP_IdxLT: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const OP_IdxGE: ::core::ffi::c_int = 46;
pub const OP_RowSetRead: ::core::ffi::c_int = 47;
pub const OP_RowSetTest: ::core::ffi::c_int = 48;
pub const OP_Program: ::core::ffi::c_int = 49;
pub const OP_FkIfZero: ::core::ffi::c_int = 50;
pub const OP_IsNull: ::core::ffi::c_int = 51;
pub const OP_NotNull: ::core::ffi::c_int = 52;
pub const OP_Ne: ::core::ffi::c_int = 53;
pub const OP_Eq: ::core::ffi::c_int = 54;
pub const OP_Gt: ::core::ffi::c_int = 55;
pub const OP_Le: ::core::ffi::c_int = 56;
pub const OP_Lt: ::core::ffi::c_int = 57;
pub const OP_Ge: ::core::ffi::c_int = 58;
pub const OP_ElseEq: ::core::ffi::c_int = 59;
pub const OP_IfPos: ::core::ffi::c_int = 60;
pub const OP_IfNotZero: ::core::ffi::c_int = 61;
pub const OP_DecrJumpZero: ::core::ffi::c_int = 62;
pub const OP_IncrVacuum: ::core::ffi::c_int = 63;
pub const OP_VNext: ::core::ffi::c_int = 64;
pub const OP_Filter: ::core::ffi::c_int = 65;
pub const OP_PureFunc: ::core::ffi::c_int = 66;
pub const OP_Function: ::core::ffi::c_int = 67;
pub const OP_Return: ::core::ffi::c_int = 68;
pub const OP_EndCoroutine: ::core::ffi::c_int = 69;
pub const OP_HaltIfNull: ::core::ffi::c_int = 70;
pub const OP_Halt: ::core::ffi::c_int = 71;
pub const OP_Integer: ::core::ffi::c_int = 72;
pub const OP_Int64: ::core::ffi::c_int = 73;
pub const OP_String: ::core::ffi::c_int = 74;
pub const OP_BeginSubrtn: ::core::ffi::c_int = 75;
pub const OP_Null: ::core::ffi::c_int = 76;
pub const OP_SoftNull: ::core::ffi::c_int = 77;
pub const OP_Blob: ::core::ffi::c_int = 78;
pub const OP_Variable: ::core::ffi::c_int = 79;
pub const OP_Move: ::core::ffi::c_int = 80;
pub const OP_Copy: ::core::ffi::c_int = 81;
pub const OP_SCopy: ::core::ffi::c_int = 82;
pub const OP_IntCopy: ::core::ffi::c_int = 83;
pub const OP_FkCheck: ::core::ffi::c_int = 84;
pub const OP_ResultRow: ::core::ffi::c_int = 85;
pub const OP_CollSeq: ::core::ffi::c_int = 86;
pub const OP_AddImm: ::core::ffi::c_int = 87;
pub const OP_RealAffinity: ::core::ffi::c_int = 88;
pub const OP_Cast: ::core::ffi::c_int = 89;
pub const OP_Permutation: ::core::ffi::c_int = 90;
pub const OP_Compare: ::core::ffi::c_int = 91;
pub const OP_IsTrue: ::core::ffi::c_int = 92;
pub const OP_ZeroOrNull: ::core::ffi::c_int = 93;
pub const OP_Column: ::core::ffi::c_int = 95;
pub const OP_TypeCheck: ::core::ffi::c_int = 96;
pub const OP_Affinity: ::core::ffi::c_int = 97;
pub const OP_MakeRecord: ::core::ffi::c_int = 98;
pub const OP_Count: ::core::ffi::c_int = 99;
pub const OP_ReadCookie: ::core::ffi::c_int = 100;
pub const OP_SetCookie: ::core::ffi::c_int = 101;
pub const OP_ReopenIdx: ::core::ffi::c_int = 102;
pub const OP_BitAnd: ::core::ffi::c_int = 103 as ::core::ffi::c_int;
pub const OP_BitOr: ::core::ffi::c_int = 104 as ::core::ffi::c_int;
pub const OP_ShiftLeft: ::core::ffi::c_int = 105 as ::core::ffi::c_int;
pub const OP_ShiftRight: ::core::ffi::c_int = 106;
pub const OP_Add: ::core::ffi::c_int = 107;
pub const OP_Subtract: ::core::ffi::c_int = 108;
pub const OP_Multiply: ::core::ffi::c_int = 109;
pub const OP_Divide: ::core::ffi::c_int = 110;
pub const OP_Remainder: ::core::ffi::c_int = 111;
pub const OP_Concat: ::core::ffi::c_int = 112;
pub const OP_OpenRead: ::core::ffi::c_int = 113;
pub const OP_OpenWrite: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const OP_BitNot: ::core::ffi::c_int = 115;
pub const OP_OpenDup: ::core::ffi::c_int = 116;
pub const OP_OpenAutoindex: ::core::ffi::c_int = 117;
pub const OP_String8: ::core::ffi::c_int = 118;
pub const OP_OpenEphemeral: ::core::ffi::c_int = 119;
pub const OP_SorterOpen: ::core::ffi::c_int = 120;
pub const OP_SequenceTest: ::core::ffi::c_int = 121;
pub const OP_OpenPseudo: ::core::ffi::c_int = 122;
pub const OP_Close: ::core::ffi::c_int = 123;
pub const OP_SeekScan: ::core::ffi::c_int = 125;
pub const OP_SeekHit: ::core::ffi::c_int = 126;
pub const OP_Sequence: ::core::ffi::c_int = 127;
pub const OP_NewRowid: ::core::ffi::c_int = 128;
pub const OP_Insert: ::core::ffi::c_int = 129;
pub const OP_RowCell: ::core::ffi::c_int = 130;
pub const OP_Delete: ::core::ffi::c_int = 131;
pub const OP_ResetCount: ::core::ffi::c_int = 132;
pub const OP_SorterCompare: ::core::ffi::c_int = 133;
pub const OP_SorterData: ::core::ffi::c_int = 134;
pub const OP_RowData: ::core::ffi::c_int = 135;
pub const OP_Rowid: ::core::ffi::c_int = 136;
pub const OP_NullRow: ::core::ffi::c_int = 137;
pub const OP_SeekEnd: ::core::ffi::c_int = 138 as ::core::ffi::c_int;
pub const OP_IdxInsert: ::core::ffi::c_int = 139;
pub const OP_SorterInsert: ::core::ffi::c_int = 140;
pub const OP_IdxDelete: ::core::ffi::c_int = 141;
pub const OP_DeferredSeek: ::core::ffi::c_int = 142 as ::core::ffi::c_int;
pub const OP_IdxRowid: ::core::ffi::c_int = 143;
pub const OP_FinishSeek: ::core::ffi::c_int = 144;
pub const OP_Destroy: ::core::ffi::c_int = 145;
pub const OP_Clear: ::core::ffi::c_int = 146;
pub const OP_ResetSorter: ::core::ffi::c_int = 147;
pub const OP_CreateBtree: ::core::ffi::c_int = 148;
pub const OP_SqlExec: ::core::ffi::c_int = 149;
pub const OP_ParseSchema: ::core::ffi::c_int = 150;
pub const OP_LoadAnalysis: ::core::ffi::c_int = 151;
pub const OP_DropTable: ::core::ffi::c_int = 152;
pub const OP_DropIndex: ::core::ffi::c_int = 153;
pub const OP_Real: ::core::ffi::c_int = 154;
pub const OP_DropTrigger: ::core::ffi::c_int = 155;
pub const OP_IntegrityCk: ::core::ffi::c_int = 156;
pub const OP_RowSetAdd: ::core::ffi::c_int = 157;
pub const OP_Param: ::core::ffi::c_int = 158;
pub const OP_FkCounter: ::core::ffi::c_int = 159;
pub const OP_MemMax: ::core::ffi::c_int = 160;
pub const OP_OffsetLimit: ::core::ffi::c_int = 161;
pub const OP_AggInverse: ::core::ffi::c_int = 162;
pub const OP_AggStep: ::core::ffi::c_int = 163;
pub const OP_AggStep1: ::core::ffi::c_int = 164;
pub const OP_AggValue: ::core::ffi::c_int = 165;
pub const OP_AggFinal: ::core::ffi::c_int = 166;
pub const OP_Expire: ::core::ffi::c_int = 167;
pub const OP_CursorLock: ::core::ffi::c_int = 168;
pub const OP_CursorUnlock: ::core::ffi::c_int = 169;
pub const OP_VBegin: ::core::ffi::c_int = 171;
pub const OP_VCreate: ::core::ffi::c_int = 172;
pub const OP_VDestroy: ::core::ffi::c_int = 173;
pub const OP_VOpen: ::core::ffi::c_int = 174;
pub const OP_VCheck: ::core::ffi::c_int = 175;
pub const OP_VInitIn: ::core::ffi::c_int = 176;
pub const OP_VColumn: ::core::ffi::c_int = 177;
pub const OP_VRename: ::core::ffi::c_int = 178;
pub const OP_Pagecount: ::core::ffi::c_int = 179;
pub const OP_MaxPgcnt: ::core::ffi::c_int = 180;
pub const OP_ClrSubtype: ::core::ffi::c_int = 181;
pub const OP_GetSubtype: ::core::ffi::c_int = 182;
pub const OP_SetSubtype: ::core::ffi::c_int = 183;
pub const OP_FilterAdd: ::core::ffi::c_int = 184;
pub const OP_Trace: ::core::ffi::c_int = 185 as ::core::ffi::c_int;
pub const LARGEST_INT64: i64_0 = 0xffffffff as i64_0
    | (0x7fffffff as ::core::ffi::c_int as i64_0) << 32 as ::core::ffi::c_int;
pub const LARGEST_UINT64: u64_0 = 0xffffffff as u64_0
    | (0xffffffff as ::core::ffi::c_uint as u64_0) << 32 as ::core::ffi::c_int;
pub const SMALLEST_INT64: i64_0 = -1 as ::core::ffi::c_int as i64_0 - LARGEST_INT64;
pub const LEGACY_SCHEMA_TABLE: [::core::ffi::c_char; 14] = unsafe {
    ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"sqlite_master\0")
};
pub const SCHEMA_ROOT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_QUERY: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_OFF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_MEMORY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_WAL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const BTREE_OMIT_JOURNAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTREE_SINGLE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BTREE_UNORDERED: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const BTREE_BLOBKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BTREE_SCHEMA_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTREE_FILE_FORMAT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BTREE_SEEK_EQ: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BTREE_WRCSR: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const BTREE_AUXDELETE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const P4_NOTUSED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const P4_INT32: ::core::ffi::c_int = -3 as ::core::ffi::c_int;
pub const P4_TABLE: ::core::ffi::c_int = -5 as ::core::ffi::c_int;
pub const P4_DYNAMIC: ::core::ffi::c_int = -6 as ::core::ffi::c_int;
pub const P4_KEYINFO: ::core::ffi::c_int = -8 as ::core::ffi::c_int;
pub const P4_MEM: ::core::ffi::c_int = -10 as ::core::ffi::c_int;
pub const P4_FUNCCTX: ::core::ffi::c_int = -15 as ::core::ffi::c_int;
pub const SQLITE_TRACE_LEGACY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_DeferFKs: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_QueryOnly: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const SQLITE_LegacyAlter: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const SQLITE_CorruptRdOnly: u64_0 = (0x2 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const DBFLAG_SchemaChange: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const DBFLAG_SchemaKnownOk: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SAVEPOINT_BEGIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SAVEPOINT_RELEASE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SAVEPOINT_ROLLBACK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const COLTYPE_BLOB: ::core::ffi::c_int = 2;
pub const COLTYPE_INT: ::core::ffi::c_int = 3;
pub const COLTYPE_INTEGER: ::core::ffi::c_int = 4;
pub const COLTYPE_REAL: ::core::ffi::c_int = 5;
pub const COLTYPE_TEXT: ::core::ffi::c_int = 6;
pub const COLFLAG_VIRTUAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const COLFLAG_GENERATED: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const SQLITE_AFF_TEXT: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const SQLITE_AFF_REAL: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;
pub const SQLITE_AFF_MASK: ::core::ffi::c_int = 0x47 as ::core::ffi::c_int;
pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_NULLEQ: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Ignore: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const OE_Replace: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_DESC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const KEYINFO_ORDER_BIGNULL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPFLAG_NCHANGE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPFLAG_NOCHNG: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPFLAG_LASTROWID: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const OPFLAG_ISUPDATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const OPFLAG_APPEND: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPFLAG_USESEEKRESULT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const OPFLAG_TYPEOFARG: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const OPFLAG_BYTELENARG: ::core::ffi::c_int = 0xc0 as ::core::ffi::c_int;
pub const OPFLAG_BULKCSR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPFLAG_SEEKEQ: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPFLAG_FORDELETE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPFLAG_P2ISREG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const OPFLAG_PERMUTE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPFLAG_SAVEPOSITION: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPFLAG_PREFORMAT: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const CURTYPE_BTREE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CURTYPE_SORTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CURTYPE_VTAB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CURTYPE_PSEUDO: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CACHE_STALE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MEMCELLSIZE: ::core::ffi::c_ulong = 24 as ::core::ffi::c_ulong;
pub const MEM_Undefined: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MEM_Null: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MEM_Str: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MEM_Int: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MEM_Real: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MEM_Blob: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MEM_IntReal: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MEM_AffMask: ::core::ffi::c_int = 0x3f as ::core::ffi::c_int;
pub const MEM_FromBind: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const MEM_Cleared: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const MEM_Term: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const MEM_Zero: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const MEM_Subtype: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const MEM_TypeMask: ::core::ffi::c_int = 0xdbf as ::core::ffi::c_int;
pub const MEM_Dyn: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const MEM_Static: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const MEM_Ephem: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const MEM_Agg: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const VDBE_RUN_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const VDBE_HALT_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
unsafe extern "C" fn allocateCursor(
    mut p: *mut Vdbe,
    mut iCur: ::core::ffi::c_int,
    mut nField: ::core::ffi::c_int,
    mut eCurType: u8_0,
) -> *mut VdbeCursor {
    unsafe {
        let mut pMem: *mut Mem = if iCur > 0 as ::core::ffi::c_int {
            (*p).aMem.offset(((*p).nMem - iCur) as isize) as *mut Mem
        } else {
            (*p).aMem
        };
        let mut nByte: i64_0 = 0;
        let mut pCx: *mut VdbeCursor = ::core::ptr::null_mut::<VdbeCursor>();
        nByte = ((112 as usize).wrapping_add(7 as usize)
            & !(7 as ::core::ffi::c_int) as usize)
            .wrapping_add(
                ((nField + 1 as ::core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<u64_0>() as usize),
            ) as i64_0;
        if eCurType as ::core::ffi::c_int == CURTYPE_BTREE {
            nByte += sqlite3BtreeCursorSize() as ::core::ffi::c_longlong;
        }
        if !(*(*p).apCsr.offset(iCur as isize)).is_null() {
            sqlite3VdbeFreeCursorNN(p, *(*p).apCsr.offset(iCur as isize));
            let ref mut c2rust_fresh20 = *(*p).apCsr.offset(iCur as isize);
            *c2rust_fresh20 = ::core::ptr::null_mut::<VdbeCursor>();
        }
        if ((*pMem).szMalloc as ::core::ffi::c_longlong) < nByte {
            if (*pMem).szMalloc > 0 as ::core::ffi::c_int {
                sqlite3DbFreeNN((*pMem).db, (*pMem).zMalloc as *mut ::core::ffi::c_void);
            }
            (*pMem).zMalloc = sqlite3DbMallocRaw((*pMem).db, nByte as u64_0)
                as *mut ::core::ffi::c_char;
            (*pMem).z = (*pMem).zMalloc;
            if (*pMem).zMalloc.is_null() {
                (*pMem).szMalloc = 0 as ::core::ffi::c_int;
                return ::core::ptr::null_mut::<VdbeCursor>();
            }
            (*pMem).szMalloc = nByte as ::core::ffi::c_int;
        }
        pCx = (*pMem).zMalloc as *mut VdbeCursor;
        let ref mut c2rust_fresh21 = *(*p).apCsr.offset(iCur as isize);
        *c2rust_fresh21 = pCx;
        memset(pCx as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, 32 as size_t);
        (*pCx).eCurType = eCurType;
        (*pCx).nField = nField as i16_0;
        (*pCx).aOffset = (&raw mut (*pCx).aType as *mut u32_0).offset(nField as isize)
            as *mut u32_0;
        if eCurType as ::core::ffi::c_int == CURTYPE_BTREE {
            (*pCx).uc.pCursor = (*pMem)
                .z
                .offset(
                    ((112 as usize).wrapping_add(7 as usize)
                        & !(7 as ::core::ffi::c_int) as usize)
                        .wrapping_add(
                            ((nField + 1 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<u64_0>() as usize),
                        ) as isize,
                ) as *mut ::core::ffi::c_char as *mut BtCursor;
            sqlite3BtreeCursorZero((*pCx).uc.pCursor);
        }
        return pCx;
    }
}
unsafe extern "C" fn alsoAnInt(
    mut pRec: *mut Mem,
    mut rValue: ::core::ffi::c_double,
    mut piValue: *mut i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iValue: i64_0 = 0;
        iValue = sqlite3RealToI64(rValue);
        if sqlite3RealSameAsInt(rValue, iValue as sqlite3_int64) != 0 {
            *piValue = iValue;
            return 1 as ::core::ffi::c_int;
        }
        return (0 as ::core::ffi::c_int
            == sqlite3Atoi64((*pRec).z, piValue, (*pRec).n, (*pRec).enc))
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn applyNumericAffinity(
    mut pRec: *mut Mem,
    mut bTryForInt: ::core::ffi::c_int,
) {
    unsafe {
        let mut rValue: ::core::ffi::c_double = 0.;
        let mut enc: u8_0 = (*pRec).enc;
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3AtoF((*pRec).z, &raw mut rValue, (*pRec).n, enc);
        if rc <= 0 as ::core::ffi::c_int {
            return;
        }
        if rc == 1 as ::core::ffi::c_int
            && alsoAnInt(pRec, rValue, &raw mut (*pRec).u.i) != 0
        {
            (*pRec).flags = ((*pRec).flags as ::core::ffi::c_int | MEM_Int) as u16_0;
        } else {
            (*pRec).u.r = rValue;
            (*pRec).flags = ((*pRec).flags as ::core::ffi::c_int | MEM_Real) as u16_0;
            if bTryForInt != 0 {
                sqlite3VdbeIntegerAffinity(pRec);
            }
        }
        (*pRec).flags = ((*pRec).flags as ::core::ffi::c_int & !MEM_Str) as u16_0;
    }
}
unsafe extern "C" fn applyAffinity(
    mut pRec: *mut Mem,
    mut affinity: ::core::ffi::c_char,
    mut enc: u8_0,
) {
    unsafe {
        if affinity as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC {
            if (*pRec).flags as ::core::ffi::c_int & MEM_Int == 0 as ::core::ffi::c_int {
                if (*pRec).flags as ::core::ffi::c_int & (MEM_Real | MEM_IntReal)
                    == 0 as ::core::ffi::c_int
                {
                    if (*pRec).flags as ::core::ffi::c_int & MEM_Str != 0 {
                        applyNumericAffinity(pRec, 1 as ::core::ffi::c_int);
                    }
                } else if affinity as ::core::ffi::c_int <= SQLITE_AFF_REAL {
                    sqlite3VdbeIntegerAffinity(pRec);
                }
            }
        } else if affinity as ::core::ffi::c_int == SQLITE_AFF_TEXT {
            if 0 as ::core::ffi::c_int == (*pRec).flags as ::core::ffi::c_int & MEM_Str {
                if (*pRec).flags as ::core::ffi::c_int
                    & (MEM_Real | MEM_Int | MEM_IntReal) != 0
                {
                    sqlite3VdbeMemStringify(pRec, enc, 1 as u8_0);
                }
            }
            (*pRec).flags = ((*pRec).flags as ::core::ffi::c_int
                & !(MEM_Real | MEM_Int | MEM_IntReal)) as u16_0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3_value_numeric_type(
    mut pVal: *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eType: ::core::ffi::c_int = sqlite3_value_type(pVal);
        if eType == SQLITE_TEXT {
            let mut pMem: *mut Mem = pVal as *mut Mem;
            applyNumericAffinity(pMem, 0 as ::core::ffi::c_int);
            eType = sqlite3_value_type(pVal);
        }
        return eType;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ValueApplyAffinity(
    mut pVal: *mut sqlite3_value,
    mut affinity: u8_0,
    mut enc: u8_0,
) {
    unsafe {
        applyAffinity(pVal as *mut Mem, affinity as ::core::ffi::c_char, enc);
    }
}
#[inline(never)]
unsafe extern "C" fn computeNumericType(mut pMem: *mut Mem) -> u16_0 {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut ix: sqlite3_int64 = 0;
        if if (*pMem).flags as ::core::ffi::c_int & MEM_Zero != 0 {
            sqlite3VdbeMemExpandBlob(pMem)
        } else {
            0 as ::core::ffi::c_int
        } != 0
        {
            (*pMem).u.i = 0 as i64_0;
            return MEM_Int as u16_0;
        }
        rc = sqlite3AtoF((*pMem).z, &raw mut (*pMem).u.r, (*pMem).n, (*pMem).enc);
        if rc <= 0 as ::core::ffi::c_int {
            if rc == 0 as ::core::ffi::c_int
                && sqlite3Atoi64((*pMem).z, &raw mut ix, (*pMem).n, (*pMem).enc)
                    <= 1 as ::core::ffi::c_int
            {
                (*pMem).u.i = ix as i64_0;
                return MEM_Int as u16_0;
            } else {
                return MEM_Real as u16_0
            }
        } else if rc == 1 as ::core::ffi::c_int
            && sqlite3Atoi64((*pMem).z, &raw mut ix, (*pMem).n, (*pMem).enc)
                == 0 as ::core::ffi::c_int
        {
            (*pMem).u.i = ix as i64_0;
            return MEM_Int as u16_0;
        }
        return MEM_Real as u16_0;
    }
}
unsafe extern "C" fn numericType(mut pMem: *mut Mem) -> u16_0 {
    unsafe {
        if (*pMem).flags as ::core::ffi::c_int
            & (MEM_Int | MEM_Real | MEM_IntReal | MEM_Null) != 0
        {
            return ((*pMem).flags as ::core::ffi::c_int
                & (MEM_Int | MEM_Real | MEM_IntReal | MEM_Null)) as u16_0;
        }
        return computeNumericType(pMem);
    }
}
#[inline(never)]
unsafe extern "C" fn out2PrereleaseWithClear(mut pOut: *mut Mem) -> *mut Mem {
    unsafe {
        sqlite3VdbeMemSetNull(pOut);
        (*pOut).flags = MEM_Int as u16_0;
        return pOut;
    }
}
unsafe extern "C" fn out2Prerelease(mut p: *mut Vdbe, mut pOp: *mut VdbeOp) -> *mut Mem {
    unsafe {
        let mut pOut: *mut Mem = ::core::ptr::null_mut::<Mem>();
        pOut = (*p).aMem.offset((*pOp).p2 as isize) as *mut Mem;
        if (*pOut).flags as ::core::ffi::c_int & (MEM_Agg | MEM_Dyn)
            != 0 as ::core::ffi::c_int
        {
            return out2PrereleaseWithClear(pOut)
        } else {
            (*pOut).flags = MEM_Int as u16_0;
            return pOut;
        };
    }
}
unsafe extern "C" fn filterHash(mut aMem: *const Mem, mut pOp: *const Op) -> u64_0 {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut mx: ::core::ffi::c_int = 0;
        let mut h: u64_0 = 0 as u64_0;
        i = (*pOp).p3;
        mx = i + (*pOp).p4.i;
        while i < mx {
            let mut p: *const Mem = aMem.offset(i as isize) as *const Mem;
            if (*p).flags as ::core::ffi::c_int & (MEM_Int | MEM_IntReal) != 0 {
                h = (h as ::core::ffi::c_ulonglong)
                    .wrapping_add((*p).u.i as ::core::ffi::c_ulonglong) as u64_0
                    as u64_0;
            } else if (*p).flags as ::core::ffi::c_int & MEM_Real != 0 {
                h = (h as ::core::ffi::c_ulonglong)
                    .wrapping_add(sqlite3VdbeIntValue(p) as ::core::ffi::c_ulonglong)
                    as u64_0 as u64_0;
            } else if (*p).flags as ::core::ffi::c_int & (MEM_Str | MEM_Blob) != 0 {
                h = (h as ::core::ffi::c_ulonglong)
                    .wrapping_add(
                        (4093 as ::core::ffi::c_int
                            + ((*p).flags as ::core::ffi::c_int & (MEM_Str | MEM_Blob)))
                            as ::core::ffi::c_ulonglong,
                    ) as u64_0 as u64_0;
            }
            i += 1;
        }
        return h;
    }
}
#[inline(never)]
unsafe extern "C" fn vdbeColumnFromOverflow(
    mut pC: *mut VdbeCursor,
    mut iCol: ::core::ffi::c_int,
    mut t: u32_0,
    mut iOffset: i64_0,
    mut cacheStatus: u32_0,
    mut colCacheCtr: u32_0,
    mut pDest: *mut Mem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pDest).db;
        let mut encoding: ::core::ffi::c_int = (*pDest).enc as ::core::ffi::c_int;
        let mut len: ::core::ffi::c_int = sqlite3VdbeSerialTypeLen(t)
            as ::core::ffi::c_int;
        if len > (*db).aLimit[SQLITE_LIMIT_LENGTH as usize] {
            return SQLITE_TOOBIG;
        }
        if len > 4000 as ::core::ffi::c_int && (*pC).pKeyInfo.is_null() {
            let mut pCache: *mut VdbeTxtBlbCache = ::core::ptr::null_mut::<
                VdbeTxtBlbCache,
            >();
            let mut pBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            if (*pC).colCache() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*pC).pCache = sqlite3DbMallocZero(
                    db,
                    ::core::mem::size_of::<VdbeTxtBlbCache>() as u64_0,
                ) as *mut VdbeTxtBlbCache;
                if (*pC).pCache.is_null() {
                    return SQLITE_NOMEM;
                }
                (*pC).set_colCache(1 as Bool as Bool);
            }
            pCache = (*pC).pCache;
            if (*pCache).pCValue.is_null() || (*pCache).iCol != iCol
                || (*pCache).cacheStatus != cacheStatus
                || (*pCache).colCacheCtr != colCacheCtr
                || (*pCache).iOffset != sqlite3BtreeOffset((*pC).uc.pCursor)
            {
                if !(*pCache).pCValue.is_null() {
                    sqlite3RCStrUnref((*pCache).pCValue as *mut ::core::ffi::c_void);
                }
                (*pCache).pCValue = sqlite3RCStrNew(
                    (len + 3 as ::core::ffi::c_int) as u64_0,
                );
                pBuf = (*pCache).pCValue;
                if pBuf.is_null() {
                    return SQLITE_NOMEM;
                }
                rc = sqlite3BtreePayload(
                    (*pC).uc.pCursor,
                    iOffset as u32_0,
                    len as u32_0,
                    pBuf as *mut ::core::ffi::c_void,
                );
                if rc != 0 {
                    return rc;
                }
                *pBuf.offset(len as isize) = 0 as ::core::ffi::c_char;
                *pBuf.offset((len + 1 as ::core::ffi::c_int) as isize) = 0
                    as ::core::ffi::c_char;
                *pBuf.offset((len + 2 as ::core::ffi::c_int) as isize) = 0
                    as ::core::ffi::c_char;
                (*pCache).iCol = iCol;
                (*pCache).cacheStatus = cacheStatus;
                (*pCache).colCacheCtr = colCacheCtr;
                (*pCache).iOffset = sqlite3BtreeOffset((*pC).uc.pCursor);
            } else {
                pBuf = (*pCache).pCValue;
            }
            sqlite3RCStrRef(pBuf);
            if t as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint != 0 {
                rc = sqlite3VdbeMemSetStr(
                    pDest,
                    pBuf,
                    len as i64_0,
                    encoding as u8_0,
                    Some(
                        sqlite3RCStrUnref
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
                (*pDest).flags = ((*pDest).flags as ::core::ffi::c_int | MEM_Term)
                    as u16_0;
            } else {
                rc = sqlite3VdbeMemSetStr(
                    pDest,
                    pBuf,
                    len as i64_0,
                    0 as u8_0,
                    Some(
                        sqlite3RCStrUnref
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
            }
        } else {
            rc = sqlite3VdbeMemFromBtree(
                (*pC).uc.pCursor,
                iOffset as u32_0,
                len as u32_0,
                pDest,
            );
            if rc != 0 {
                return rc;
            }
            sqlite3VdbeSerialGet((*pDest).z as *const ::core::ffi::c_uchar, t, pDest);
            if t as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint && encoding == SQLITE_UTF8
            {
                *(*pDest).z.offset(len as isize) = 0 as ::core::ffi::c_char;
                (*pDest).flags = ((*pDest).flags as ::core::ffi::c_int | MEM_Term)
                    as u16_0;
            }
        }
        (*pDest).flags = ((*pDest).flags as ::core::ffi::c_int & !MEM_Ephem) as u16_0;
        return rc;
    }
}
#[inline(never)]
unsafe extern "C" fn sqlite3VdbeLogAbort(
    mut p: *mut Vdbe,
    mut rc: ::core::ffi::c_int,
    mut pOp: *mut Op,
    mut aOp: *mut Op,
) {
    unsafe {
        let mut zSql: *const ::core::ffi::c_char = (*p).zSql;
        let mut zPrefix: *const ::core::ffi::c_char = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        let mut pc: ::core::ffi::c_int = 0;
        let mut zXtra: [::core::ffi::c_char; 100] = [0; 100];
        if !(*p).pFrame.is_null() {
            if !(*aOp.offset(0 as ::core::ffi::c_int as isize)).p4.z.is_null() {
                sqlite3_snprintf(
                    ::core::mem::size_of::<[::core::ffi::c_char; 100]>()
                        as ::core::ffi::c_int,
                    &raw mut zXtra as *mut ::core::ffi::c_char,
                    b"/* %s */ \0".as_ptr() as *const ::core::ffi::c_char,
                    (*aOp.offset(0 as ::core::ffi::c_int as isize))
                        .p4
                        .z
                        .offset(3 as ::core::ffi::c_int as isize),
                );
                zPrefix = &raw mut zXtra as *mut ::core::ffi::c_char;
            } else {
                zPrefix = b"/* unknown trigger */ \0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
        }
        pc = pOp.offset_from(aOp) as ::core::ffi::c_long as ::core::ffi::c_int;
        sqlite3_log(
            rc,
            b"statement aborts at %d: %s; [%s%s]\0".as_ptr()
                as *const ::core::ffi::c_char,
            pc,
            (*p).zErrMsg,
            zPrefix,
            zSql,
        );
    }
}
unsafe extern "C" fn vdbeMemTypeName(mut pMem: *mut Mem) -> *const ::core::ffi::c_char {
    unsafe {
        static mut azTypes: [*const ::core::ffi::c_char; 5] = [
            b"INT\0".as_ptr() as *const ::core::ffi::c_char,
            b"REAL\0".as_ptr() as *const ::core::ffi::c_char,
            b"TEXT\0".as_ptr() as *const ::core::ffi::c_char,
            b"BLOB\0".as_ptr() as *const ::core::ffi::c_char,
            b"NULL\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        return azTypes[(sqlite3_value_type(pMem as *mut sqlite3_value)
            - 1 as ::core::ffi::c_int) as usize];
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3VdbeExec(mut p: *mut Vdbe) -> ::core::ffi::c_int {
    unsafe {
        let mut pC_21: *mut VdbeCursor = ::core::ptr::null_mut::<VdbeCursor>();
        let mut pC_8: *mut VdbeCursor = ::core::ptr::null_mut::<VdbeCursor>();
        let mut pCrsr_1: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        let mut res_2: ::core::ffi::c_int = 0;
        let mut iKey_0: u64_0 = 0;
        let mut nField_0: ::core::ffi::c_int = 0;
        let mut pKeyInfo_0: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        let mut p2_2: u32_0 = 0;
        let mut iDb_0: ::core::ffi::c_int = 0;
        let mut wrFlag: ::core::ffi::c_int = 0;
        let mut pX: *mut Btree = ::core::ptr::null_mut::<Btree>();
        let mut pCur: *mut VdbeCursor = ::core::ptr::null_mut::<VdbeCursor>();
        let mut pDb_1: *mut Db = ::core::ptr::null_mut::<Db>();
        let mut c2rust_current_block: u64;
        let mut aOp: *mut Op = (*p).aOp;
        let mut pOp: *mut Op = aOp;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut db: *mut sqlite3 = (*p).db;
        let mut resetSchemaOnFault: u8_0 = 0 as u8_0;
        let mut encoding: u8_0 = (*db).enc;
        let mut iCompare: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nVmStep: u64_0 = 0 as u64_0;
        let mut nProgressLimit: u64_0 = 0;
        let mut aMem: *mut Mem = (*p).aMem;
        let mut pIn1: *mut Mem = ::core::ptr::null_mut::<Mem>();
        let mut pIn2: *mut Mem = ::core::ptr::null_mut::<Mem>();
        let mut pIn3: *mut Mem = ::core::ptr::null_mut::<Mem>();
        let mut pOut: *mut Mem = ::core::ptr::null_mut::<Mem>();
        let mut colCacheCtr: u32_0 = 0 as u32_0;
        (*p).lockMask != 0 as ::core::ffi::c_uint;
        if (*db).xProgress.is_some() {
            let mut iPrior: u32_0 = (*p).aCounter[SQLITE_STMTSTATUS_VM_STEP as usize];
            nProgressLimit = (*db)
                .nProgressOps
                .wrapping_sub(
                    (iPrior as ::core::ffi::c_uint).wrapping_rem((*db).nProgressOps),
                ) as u64_0;
        } else {
            nProgressLimit = LARGEST_UINT64;
        }
        if (*p).rc == SQLITE_NOMEM {
            c2rust_current_block = 9955115899106739073;
        } else {
            (*p).rc = SQLITE_OK;
            (*p).iCurrentTime = 0 as i64_0;
            (*db).busyHandler.nBusy = 0 as ::core::ffi::c_int;
            if (*db).u1.isInterrupted
                != 0
            {
                c2rust_current_block = 4286470523181410568;
            } else {
                pOp = aOp.offset((*p).pc as isize) as *mut Op;
                's_109: loop {
                    nVmStep = nVmStep.wrapping_add(1);
                    match (*pOp).opcode as ::core::ffi::c_int {
                        OP_Goto => {
                            c2rust_current_block = 13898972957494642055;
                        }
                        OP_Gosub => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            (*pIn1).flags = MEM_Int as u16_0;
                            (*pIn1).u.i = pOp.offset_from(aOp) as ::core::ffi::c_long
                                as ::core::ffi::c_int as i64_0;
                            c2rust_current_block = 13898972957494642055;
                        }
                        OP_Return => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Int != 0 {
                                (*pOp).p3 != 0;
                                pOp = aOp.offset((*pIn1).u.i as isize) as *mut Op;
                            } else {
                                (*pOp).p3 != 0;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_InitCoroutine => {
                            pOut = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            (*pOut).u.i = ((*pOp).p3 - 1 as ::core::ffi::c_int) as i64_0;
                            (*pOut).flags = MEM_Int as u16_0;
                            if (*pOp).p2 == 0 as ::core::ffi::c_int {
                                c2rust_current_block = 5783071609795492627;
                            } else {
                                c2rust_current_block = 17233182392562552756;
                            }
                        }
                        OP_EndCoroutine => {
                            let mut pCaller: *mut VdbeOp = ::core::ptr::null_mut::<
                                VdbeOp,
                            >();
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pCaller = aOp.offset((*pIn1).u.i as isize) as *mut Op
                                as *mut VdbeOp;
                            (*pIn1).u.i = (pOp.offset_from((*p).aOp)
                                as ::core::ffi::c_long as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as i64_0;
                            pOp = aOp
                                .offset(((*pCaller).p2 - 1 as ::core::ffi::c_int) as isize)
                                as *mut Op;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Yield => {
                            let mut pcDest: ::core::ffi::c_int = 0;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            (*pIn1).flags = MEM_Int as u16_0;
                            pcDest = (*pIn1).u.i as ::core::ffi::c_int;
                            (*pIn1).u.i = pOp.offset_from(aOp) as ::core::ffi::c_long
                                as ::core::ffi::c_int as i64_0;
                            pOp = aOp.offset(pcDest as isize) as *mut Op;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_HaltIfNull => {
                            pIn3 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            if (*pIn3).flags as ::core::ffi::c_int & MEM_Null
                                == 0 as ::core::ffi::c_int
                            {
                                c2rust_current_block = 5783071609795492627;
                            } else {
                                c2rust_current_block = 7419121793134201633;
                            }
                        }
                        OP_Halt => {
                            c2rust_current_block = 7419121793134201633;
                        }
                        OP_Integer => {
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            (*pOut).u.i = (*pOp).p1 as i64_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Int64 => {
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            (*pOut).u.i = *(*pOp).p4.pI64;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Real => {
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            (*pOut).flags = MEM_Real as u16_0;
                            (*pOut).u.r = *(*pOp).p4.pReal;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_String8 => {
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            (*pOp).p1 = sqlite3Strlen30((*pOp).p4.z);
                            if encoding as ::core::ffi::c_int != SQLITE_UTF8 {
                                rc = sqlite3VdbeMemSetStr(
                                    pOut,
                                    (*pOp).p4.z,
                                    -1 as ::core::ffi::c_int as i64_0,
                                    SQLITE_UTF8 as u8_0,
                                    SQLITE_STATIC,
                                );
                                if rc != 0 {
                                    c2rust_current_block = 13536115470629238340;
                                    break;
                                }
                                if SQLITE_OK
                                    != sqlite3VdbeChangeEncoding(
                                        pOut,
                                        encoding as ::core::ffi::c_int,
                                    )
                                {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                }
                                (*pOut).szMalloc = 0 as ::core::ffi::c_int;
                                (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                    | MEM_Static) as u16_0;
                                if (*pOp).p4type as ::core::ffi::c_int == P4_DYNAMIC {
                                    sqlite3DbFree(db, (*pOp).p4.z as *mut ::core::ffi::c_void);
                                }
                                (*pOp).p4type = P4_DYNAMIC as ::core::ffi::c_schar;
                                (*pOp).p4.z = (*pOut).z;
                                (*pOp).p1 = (*pOut).n;
                            }
                            if (*pOp).p1 > (*db).aLimit[SQLITE_LIMIT_LENGTH as usize] {
                                c2rust_current_block = 13536115470629238340;
                                break;
                            }
                            (*pOp).opcode = OP_String as u8_0;
                            c2rust_current_block = 15460309861373144675;
                        }
                        OP_String => {
                            c2rust_current_block = 15460309861373144675;
                        }
                        OP_BeginSubrtn | OP_Null => {
                            let mut cnt: ::core::ffi::c_int = 0;
                            let mut nullFlag: u16_0 = 0;
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            cnt = (*pOp).p3 - (*pOp).p2;
                            nullFlag = (if (*pOp).p1 != 0 {
                                MEM_Null | MEM_Cleared
                            } else {
                                MEM_Null
                            }) as u16_0;
                            (*pOut).flags = nullFlag;
                            (*pOut).n = 0 as ::core::ffi::c_int;
                            while cnt > 0 as ::core::ffi::c_int {
                                pOut = pOut.offset(1);
                                sqlite3VdbeMemSetNull(pOut);
                                (*pOut).flags = nullFlag;
                                (*pOut).n = 0 as ::core::ffi::c_int;
                                cnt -= 1;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_SoftNull => {
                            pOut = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                & !(MEM_Undefined | MEM_AffMask) | MEM_Null) as u16_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Blob => {
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            if (*pOp).p4.z.is_null() {
                                sqlite3VdbeMemSetZeroBlob(pOut, (*pOp).p1);
                                if sqlite3VdbeMemExpandBlob(pOut) != 0 {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                }
                            } else {
                                sqlite3VdbeMemSetStr(
                                    pOut,
                                    (*pOp).p4.z,
                                    (*pOp).p1 as i64_0,
                                    0 as u8_0,
                                    None,
                                );
                            }
                            (*pOut).enc = encoding;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Variable => {
                            let mut pVar: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            pVar = (*p)
                                .aVar
                                .offset(((*pOp).p1 - 1 as ::core::ffi::c_int) as isize)
                                as *mut Mem;
                            if sqlite3VdbeMemTooBig(pVar) != 0 {
                                c2rust_current_block = 13536115470629238340;
                                break;
                            }
                            pOut = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            if (*pOut).flags as ::core::ffi::c_int & (MEM_Agg | MEM_Dyn)
                                != 0 as ::core::ffi::c_int
                            {
                                sqlite3VdbeMemSetNull(pOut);
                            }
                            memcpy(
                                pOut as *mut ::core::ffi::c_void,
                                pVar as *const ::core::ffi::c_void,
                                MEMCELLSIZE as size_t,
                            );
                            (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                & !(MEM_Dyn | MEM_Ephem)) as u16_0;
                            (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                | (MEM_Static | MEM_FromBind)) as u16_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Move => {
                            let mut n: ::core::ffi::c_int = 0;
                            let mut p1: ::core::ffi::c_int = 0;
                            let mut p2: ::core::ffi::c_int = 0;
                            n = (*pOp).p3;
                            p1 = (*pOp).p1;
                            p2 = (*pOp).p2;
                            pIn1 = aMem.offset(p1 as isize) as *mut Mem;
                            pOut = aMem.offset(p2 as isize) as *mut Mem;
                            loop {
                                sqlite3VdbeMemMove(pOut, pIn1);
                                if (*pOut).flags as ::core::ffi::c_int & MEM_Ephem
                                    != 0 as ::core::ffi::c_int
                                    && sqlite3VdbeMemMakeWriteable(pOut) != 0
                                {
                                    c2rust_current_block = 9955115899106739073;
                                    break 's_109;
                                }
                                pIn1 = pIn1.offset(1);
                                pOut = pOut.offset(1);
                                n -= 1;
                                if !(n != 0) {
                                    break;
                                }
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Copy => {
                            let mut n_0: ::core::ffi::c_int = 0;
                            n_0 = (*pOp).p3;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pOut = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            loop {
                                sqlite3VdbeMemShallowCopy(pOut, pIn1, MEM_Ephem);
                                if (*pOut).flags as ::core::ffi::c_int & MEM_Ephem
                                    != 0 as ::core::ffi::c_int
                                    && sqlite3VdbeMemMakeWriteable(pOut) != 0
                                {
                                    c2rust_current_block = 9955115899106739073;
                                    break 's_109;
                                }
                                if (*pOut).flags as ::core::ffi::c_int & MEM_Subtype
                                    != 0 as ::core::ffi::c_int
                                    && (*pOp).p5 as ::core::ffi::c_int
                                        & 0x2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                {
                                    (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                        & !MEM_Subtype) as u16_0;
                                }
                                let c2rust_fresh0 = n_0;
                                n_0 = n_0 - 1;
                                if c2rust_fresh0 == 0 as ::core::ffi::c_int {
                                    break;
                                }
                                pOut = pOut.offset(1);
                                pIn1 = pIn1.offset(1);
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_SCopy => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pOut = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            sqlite3VdbeMemShallowCopy(pOut, pIn1, MEM_Ephem);
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_IntCopy => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pOut = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            sqlite3VdbeMemSetInt64(pOut, (*pIn1).u.i);
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_FkCheck => {
                            rc = sqlite3VdbeCheckFkImmediate(p);
                            if rc != SQLITE_OK {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_ResultRow => {
                            (*p).cacheCtr = (((*p).cacheCtr as ::core::ffi::c_uint)
                                .wrapping_add(2 as ::core::ffi::c_uint)
                                | 1 as ::core::ffi::c_uint) as u32_0;
                            (*p).pResultRow = aMem.offset((*pOp).p1 as isize)
                                as *mut Mem;
                            if (*db).mallocFailed != 0 {
                                c2rust_current_block = 9955115899106739073;
                                break;
                            }
                            if (*db).mTrace as ::core::ffi::c_int & SQLITE_TRACE_ROW != 0
                            {
                                (*db)
                                    .trace
                                    .xV2
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    SQLITE_TRACE_ROW as u32_0,
                                    (*db).pTraceArg,
                                    p as *mut ::core::ffi::c_void,
                                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                );
                            }
                            (*p).pc = pOp.offset_from(aOp) as ::core::ffi::c_long
                                as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                            rc = SQLITE_ROW;
                            c2rust_current_block = 4288082686256521985;
                            break;
                        }
                        OP_Concat => {
                            let mut nByte: i64_0 = 0;
                            let mut flags1: u16_0 = 0;
                            let mut flags2: u16_0 = 0;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pIn2 = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            pOut = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            flags1 = (*pIn1).flags;
                            if (flags1 as ::core::ffi::c_int
                                | (*pIn2).flags as ::core::ffi::c_int) & MEM_Null != 0
                            {
                                sqlite3VdbeMemSetNull(pOut);
                            } else {
                                if flags1 as ::core::ffi::c_int & (MEM_Str | MEM_Blob)
                                    == 0 as ::core::ffi::c_int
                                {
                                    if sqlite3VdbeMemStringify(pIn1, encoding, 0 as u8_0) != 0 {
                                        c2rust_current_block = 9955115899106739073;
                                        break;
                                    }
                                    flags1 = ((*pIn1).flags as ::core::ffi::c_int & !MEM_Str)
                                        as u16_0;
                                } else if flags1 as ::core::ffi::c_int & MEM_Zero
                                    != 0 as ::core::ffi::c_int
                                {
                                    if sqlite3VdbeMemExpandBlob(pIn1) != 0 {
                                        c2rust_current_block = 9955115899106739073;
                                        break;
                                    }
                                    flags1 = ((*pIn1).flags as ::core::ffi::c_int & !MEM_Str)
                                        as u16_0;
                                }
                                flags2 = (*pIn2).flags;
                                if flags2 as ::core::ffi::c_int & (MEM_Str | MEM_Blob)
                                    == 0 as ::core::ffi::c_int
                                {
                                    if sqlite3VdbeMemStringify(pIn2, encoding, 0 as u8_0) != 0 {
                                        c2rust_current_block = 9955115899106739073;
                                        break;
                                    }
                                    flags2 = ((*pIn2).flags as ::core::ffi::c_int & !MEM_Str)
                                        as u16_0;
                                } else if flags2 as ::core::ffi::c_int & MEM_Zero
                                    != 0 as ::core::ffi::c_int
                                {
                                    if sqlite3VdbeMemExpandBlob(pIn2) != 0 {
                                        c2rust_current_block = 9955115899106739073;
                                        break;
                                    }
                                    flags2 = ((*pIn2).flags as ::core::ffi::c_int & !MEM_Str)
                                        as u16_0;
                                }
                                nByte = (*pIn1).n as i64_0;
                                nByte += (*pIn2).n as ::core::ffi::c_longlong;
                                if nByte
                                    > (*db).aLimit[SQLITE_LIMIT_LENGTH as usize]
                                        as ::core::ffi::c_longlong
                                {
                                    c2rust_current_block = 13536115470629238340;
                                    break;
                                }
                                if sqlite3VdbeMemGrow(
                                    pOut,
                                    nByte as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
                                    (pOut == pIn2) as ::core::ffi::c_int,
                                ) != 0
                                {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                }
                                (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                    & !(MEM_TypeMask | MEM_Zero) | 0x2 as ::core::ffi::c_int)
                                    as u16_0;
                                if pOut != pIn2 {
                                    memcpy(
                                        (*pOut).z as *mut ::core::ffi::c_void,
                                        (*pIn2).z as *const ::core::ffi::c_void,
                                        (*pIn2).n as size_t,
                                    );
                                    (*pIn2).flags = flags2;
                                }
                                memcpy(
                                    (*pOut).z.offset((*pIn2).n as isize)
                                        as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                                    (*pIn1).z as *const ::core::ffi::c_void,
                                    (*pIn1).n as size_t,
                                );
                                (*pIn1).flags = flags1;
                                if encoding as ::core::ffi::c_int > SQLITE_UTF8 {
                                    nByte
                                        &= !(1 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
                                }
                                *(*pOut).z.offset(nByte as isize) = 0
                                    as ::core::ffi::c_char;
                                *(*pOut)
                                    .z
                                    .offset(
                                        (nByte as ::core::ffi::c_longlong
                                            + 1 as ::core::ffi::c_longlong) as isize,
                                    ) = 0 as ::core::ffi::c_char;
                                (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                    | MEM_Term) as u16_0;
                                (*pOut).n = nByte as ::core::ffi::c_int;
                                (*pOut).enc = encoding;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Add => {
                            c2rust_current_block = 8767272321317040172;
                        }
                        OP_Subtract => {
                            c2rust_current_block = 8767272321317040172;
                        }
                        OP_Multiply => {
                            c2rust_current_block = 16624893707949258064;
                        }
                        OP_Divide => {
                            c2rust_current_block = 7300422940758305934;
                        }
                        OP_Remainder => {
                            c2rust_current_block = 9028266288740425872;
                        }
                        OP_CollSeq => {
                            if (*pOp).p1 != 0 {
                                sqlite3VdbeMemSetInt64(
                                    aMem.offset((*pOp).p1 as isize) as *mut Mem,
                                    0 as i64_0,
                                );
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_BitAnd => {
                            c2rust_current_block = 4315148737281477633;
                        }
                        OP_BitOr => {
                            c2rust_current_block = 4315148737281477633;
                        }
                        OP_ShiftLeft => {
                            c2rust_current_block = 4688408507276652532;
                        }
                        OP_ShiftRight => {
                            c2rust_current_block = 13567963862951993727;
                        }
                        OP_AddImm => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            sqlite3VdbeMemIntegerify(pIn1);
                            let ref mut c2rust_fresh1 = *(&raw mut (*pIn1).u.i
                                as *mut u64_0);
                            *c2rust_fresh1 = (*c2rust_fresh1 as ::core::ffi::c_ulonglong)
                                .wrapping_add(
                                    (*pOp).p2 as u64_0 as ::core::ffi::c_ulonglong,
                                ) as u64_0 as u64_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_MustBeInt => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Int
                                == 0 as ::core::ffi::c_int
                            {
                                applyAffinity(
                                    pIn1,
                                    SQLITE_AFF_NUMERIC as ::core::ffi::c_char,
                                    encoding,
                                );
                                if (*pIn1).flags as ::core::ffi::c_int & MEM_Int
                                    == 0 as ::core::ffi::c_int
                                {
                                    if (*pOp).p2 == 0 as ::core::ffi::c_int {
                                        rc = SQLITE_MISMATCH;
                                        c2rust_current_block = 8086965459351668542;
                                        break;
                                    } else {
                                        c2rust_current_block = 17233182392562552756;
                                    }
                                } else {
                                    c2rust_current_block = 11404795741823508192;
                                }
                            } else {
                                c2rust_current_block = 11404795741823508192;
                            }
                            match c2rust_current_block {
                                17233182392562552756 => {}
                                _ => {
                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                        & !(MEM_TypeMask | MEM_Zero) | 0x4 as ::core::ffi::c_int)
                                        as u16_0;
                                    c2rust_current_block = 5783071609795492627;
                                }
                            }
                        }
                        OP_RealAffinity => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            if (*pIn1).flags as ::core::ffi::c_int
                                & (MEM_Int | MEM_IntReal) != 0
                            {
                                sqlite3VdbeMemRealify(pIn1);
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Cast => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            rc = if (*pIn1).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                                sqlite3VdbeMemExpandBlob(pIn1)
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            rc = sqlite3VdbeMemCast(pIn1, (*pOp).p2 as u8_0, encoding);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Eq => {
                            c2rust_current_block = 4401829632234223291;
                        }
                        OP_Ne => {
                            c2rust_current_block = 4401829632234223291;
                        }
                        OP_Lt => {
                            c2rust_current_block = 2535740953232313464;
                        }
                        OP_Le => {
                            c2rust_current_block = 7659955564255721022;
                        }
                        OP_Gt => {
                            c2rust_current_block = 16318049377487887596;
                        }
                        OP_Ge => {
                            c2rust_current_block = 18434696934714153308;
                        }
                        OP_ElseEq => {
                            if iCompare == 0 as ::core::ffi::c_int {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_Compare => {
                            let mut n_1: ::core::ffi::c_int = 0;
                            let mut i: ::core::ffi::c_int = 0;
                            let mut p1_0: ::core::ffi::c_int = 0;
                            let mut p2_0: ::core::ffi::c_int = 0;
                            let mut pKeyInfo: *const KeyInfo = ::core::ptr::null::<
                                KeyInfo,
                            >();
                            let mut idx: u32_0 = 0;
                            let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<
                                CollSeq,
                            >();
                            let mut bRev: ::core::ffi::c_int = 0;
                            let mut aPermute: *mut u32_0 = ::core::ptr::null_mut::<
                                u32_0,
                            >();
                            if (*pOp).p5 as ::core::ffi::c_int & OPFLAG_PERMUTE
                                == 0 as ::core::ffi::c_int
                            {
                                aPermute = ::core::ptr::null_mut::<u32_0>();
                            } else {
                                aPermute = (*pOp.offset(-1 as ::core::ffi::c_int as isize))
                                    .p4
                                    .ai
                                    .offset(1 as ::core::ffi::c_int as isize);
                            }
                            n_1 = (*pOp).p3;
                            pKeyInfo = (*pOp).p4.pKeyInfo;
                            p1_0 = (*pOp).p1;
                            p2_0 = (*pOp).p2;
                            i = 0 as ::core::ffi::c_int;
                            while i < n_1 {
                                idx = (if !aPermute.is_null() {
                                    *aPermute.offset(i as isize) as ::core::ffi::c_uint
                                } else {
                                    i as ::core::ffi::c_uint
                                }) as u32_0;
                                pColl = *(&raw const (*pKeyInfo).aColl
                                    as *const *mut CollSeq)
                                    .offset(i as isize);
                                bRev = *(*pKeyInfo).aSortFlags.offset(i as isize)
                                    as ::core::ffi::c_int & KEYINFO_ORDER_DESC;
                                iCompare = sqlite3MemCompare(
                                    aMem.offset((p1_0 as u32_0).wrapping_add(idx) as isize)
                                        as *mut Mem,
                                    aMem.offset((p2_0 as u32_0).wrapping_add(idx) as isize)
                                        as *mut Mem,
                                    pColl,
                                );
                                if iCompare != 0 {
                                    if *(*pKeyInfo).aSortFlags.offset(i as isize)
                                        as ::core::ffi::c_int & KEYINFO_ORDER_BIGNULL != 0
                                        && ((*aMem
                                            .offset((p1_0 as u32_0).wrapping_add(idx) as isize))
                                            .flags as ::core::ffi::c_int & MEM_Null != 0
                                            || (*aMem
                                                .offset((p2_0 as u32_0).wrapping_add(idx) as isize))
                                                .flags as ::core::ffi::c_int & MEM_Null != 0)
                                    {
                                        iCompare = -iCompare;
                                    }
                                    if bRev != 0 {
                                        iCompare = -iCompare;
                                    }
                                    break;
                                } else {
                                    i += 1;
                                }
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Jump => {
                            if iCompare < 0 as ::core::ffi::c_int {
                                pOp = aOp
                                    .offset(((*pOp).p1 - 1 as ::core::ffi::c_int) as isize)
                                    as *mut Op;
                            } else if iCompare == 0 as ::core::ffi::c_int {
                                pOp = aOp
                                    .offset(((*pOp).p2 - 1 as ::core::ffi::c_int) as isize)
                                    as *mut Op;
                            } else {
                                pOp = aOp
                                    .offset(((*pOp).p3 - 1 as ::core::ffi::c_int) as isize)
                                    as *mut Op;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_And => {
                            c2rust_current_block = 3325916876629046855;
                        }
                        OP_Or => {
                            c2rust_current_block = 3325916876629046855;
                        }
                        OP_IsTrue => {
                            sqlite3VdbeMemSetInt64(
                                aMem.offset((*pOp).p2 as isize) as *mut Mem,
                                (sqlite3VdbeBooleanValue(
                                    aMem.offset((*pOp).p1 as isize) as *mut Mem,
                                    (*pOp).p3,
                                ) ^ (*pOp).p4.i) as i64_0,
                            );
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Not => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pOut = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Null
                                == 0 as ::core::ffi::c_int
                            {
                                sqlite3VdbeMemSetInt64(
                                    pOut,
                                    (sqlite3VdbeBooleanValue(pIn1, 0 as ::core::ffi::c_int)
                                        == 0) as ::core::ffi::c_int as i64_0,
                                );
                            } else {
                                sqlite3VdbeMemSetNull(pOut);
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_BitNot => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pOut = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            sqlite3VdbeMemSetNull(pOut);
                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Null
                                == 0 as ::core::ffi::c_int
                            {
                                (*pOut).flags = MEM_Int as u16_0;
                                (*pOut).u.i = !sqlite3VdbeIntValue(pIn1);
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Once => {
                            let mut iAddr: u32_0 = 0;
                            if !(*p).pFrame.is_null() {
                                iAddr = pOp.offset_from((*p).aOp) as ::core::ffi::c_long
                                    as ::core::ffi::c_int as u32_0;
                                if *(*(*p).pFrame)
                                    .aOnce
                                    .offset(
                                        (iAddr as ::core::ffi::c_uint)
                                            .wrapping_div(8 as ::core::ffi::c_uint) as isize,
                                    ) as ::core::ffi::c_int
                                    & (1 as ::core::ffi::c_int)
                                        << (iAddr as ::core::ffi::c_uint & 7 as ::core::ffi::c_uint)
                                    != 0 as ::core::ffi::c_int
                                {
                                    c2rust_current_block = 17233182392562552756;
                                } else {
                                    let ref mut c2rust_fresh2 = *(*(*p).pFrame)
                                        .aOnce
                                        .offset(
                                            (iAddr as ::core::ffi::c_uint)
                                                .wrapping_div(8 as ::core::ffi::c_uint) as isize,
                                        );
                                    *c2rust_fresh2 = (*c2rust_fresh2 as ::core::ffi::c_int
                                        | (1 as ::core::ffi::c_int)
                                            << (iAddr as ::core::ffi::c_uint
                                                & 7 as ::core::ffi::c_uint)) as u8_0;
                                    c2rust_current_block = 3355927352699379568;
                                }
                            } else if (*(*p)
                                .aOp
                                .offset(0 as ::core::ffi::c_int as isize))
                                .p1 == (*pOp).p1
                            {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 3355927352699379568;
                            }
                            match c2rust_current_block {
                                17233182392562552756 => {}
                                _ => {
                                    (*pOp).p1 = (*(*p)
                                        .aOp
                                        .offset(0 as ::core::ffi::c_int as isize))
                                        .p1;
                                    c2rust_current_block = 5783071609795492627;
                                }
                            }
                        }
                        OP_If => {
                            let mut c: ::core::ffi::c_int = 0;
                            c = sqlite3VdbeBooleanValue(
                                aMem.offset((*pOp).p1 as isize) as *mut Mem,
                                (*pOp).p3,
                            );
                            if c != 0 {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_IfNot => {
                            let mut c_0: ::core::ffi::c_int = 0;
                            c_0 = (sqlite3VdbeBooleanValue(
                                aMem.offset((*pOp).p1 as isize) as *mut Mem,
                                ((*pOp).p3 == 0) as ::core::ffi::c_int,
                            ) == 0) as ::core::ffi::c_int;
                            if c_0 != 0 {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_IsNull => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Null
                                != 0 as ::core::ffi::c_int
                            {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_IsType => {
                            let mut pC: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut typeMask: u16_0 = 0;
                            let mut serialType: u32_0 = 0;
                            if (*pOp).p1 >= 0 as ::core::ffi::c_int {
                                pC = *(*p).apCsr.offset((*pOp).p1 as isize);
                                if (*pOp).p3 < (*pC).nHdrParsed as ::core::ffi::c_int {
                                    serialType = *(&raw mut (*pC).aType as *mut u32_0)
                                        .offset((*pOp).p3 as isize);
                                    if serialType >= 12 as ::core::ffi::c_uint {
                                        if serialType as ::core::ffi::c_uint
                                            & 1 as ::core::ffi::c_uint != 0
                                        {
                                            typeMask = 0x4 as u16_0;
                                        } else {
                                            typeMask = 0x8 as u16_0;
                                        }
                                    } else {
                                        static mut aMask: [::core::ffi::c_uchar; 12] = [
                                            0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                            0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                            0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                            0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                            0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                            0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                            0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                            0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                            0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                            0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                            0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                            0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                        ];
                                        typeMask = aMask[serialType as usize] as u16_0;
                                    }
                                } else {
                                    typeMask = ((1 as ::core::ffi::c_int)
                                        << (*pOp).p4.i - 1 as ::core::ffi::c_int) as u16_0;
                                }
                            } else {
                                typeMask = ((1 as ::core::ffi::c_int)
                                    << sqlite3_value_type(
                                        aMem.offset((*pOp).p3 as isize) as *mut Mem
                                            as *mut sqlite3_value,
                                    ) - 1 as ::core::ffi::c_int) as u16_0;
                            }
                            if typeMask as ::core::ffi::c_int
                                & (*pOp).p5 as ::core::ffi::c_int != 0
                            {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_ZeroOrNull => {
                            if (*aMem.offset((*pOp).p1 as isize)).flags
                                as ::core::ffi::c_int & MEM_Null != 0 as ::core::ffi::c_int
                                || (*aMem.offset((*pOp).p3 as isize)).flags
                                    as ::core::ffi::c_int & MEM_Null != 0 as ::core::ffi::c_int
                            {
                                sqlite3VdbeMemSetNull(aMem.offset((*pOp).p2 as isize));
                            } else {
                                sqlite3VdbeMemSetInt64(
                                    aMem.offset((*pOp).p2 as isize),
                                    0 as i64_0,
                                );
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_NotNull => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Null
                                == 0 as ::core::ffi::c_int
                            {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_IfNullRow => {
                            let mut pC_0: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pC_0 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if !pC_0.is_null()
                                && (*pC_0).nullRow as ::core::ffi::c_int != 0
                            {
                                sqlite3VdbeMemSetNull(aMem.offset((*pOp).p3 as isize));
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_Column => {
                            let mut p2_1: u32_0 = 0;
                            let mut pC_1: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pCrsr: *mut BtCursor = ::core::ptr::null_mut::<
                                BtCursor,
                            >();
                            let mut aOffset: *mut u32_0 = ::core::ptr::null_mut::<
                                u32_0,
                            >();
                            let mut len: ::core::ffi::c_int = 0;
                            let mut i_0: ::core::ffi::c_int = 0;
                            let mut pDest: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut sMem: Mem = Mem {
                                u: MemValue { r: 0. },
                                z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                n: 0,
                                flags: 0,
                                enc: 0,
                                eSubtype: 0,
                                db: ::core::ptr::null_mut::<sqlite3>(),
                                szMalloc: 0,
                                uTemp: 0,
                                zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                xDel: None

                            };
                            let mut zData: *const u8_0 = ::core::ptr::null::<u8_0>();
                            let mut zHdr: *const u8_0 = ::core::ptr::null::<u8_0>();
                            let mut zEndHdr: *const u8_0 = ::core::ptr::null::<u8_0>();
                            let mut offset64: u64_0 = 0;
                            let mut t: u32_0 = 0;
                            let mut pReg: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            pC_1 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            p2_1 = (*pOp).p2 as u32_0;
                            loop {
                                aOffset = (*pC_1).aOffset;
                                if (*pC_1).cacheStatus != (*p).cacheCtr {
                                    if (*pC_1).nullRow != 0 {
                                        if (*pC_1).eCurType as ::core::ffi::c_int == CURTYPE_PSEUDO
                                            && (*pC_1).seekResult > 0 as ::core::ffi::c_int
                                        {
                                            c2rust_current_block = 3471494006171716190;
                                            break;
                                        } else {
                                            c2rust_current_block = 830712628114968639;
                                            break;
                                        }
                                    } else {
                                        pCrsr = (*pC_1).uc.pCursor;
                                        if (*pC_1).deferredMoveto != 0 {
                                            let mut iMap: u32_0 = 0;
                                            if !(*pC_1).ub.aAltMap.is_null()
                                                && {
                                                    iMap = *(*pC_1)
                                                        .ub
                                                        .aAltMap
                                                        .offset((1 as u32_0).wrapping_add(p2_1) as isize);
                                                    iMap > 0 as ::core::ffi::c_uint
                                                }
                                            {
                                                pC_1 = (*pC_1).pAltCursor;
                                                p2_1 = (iMap as ::core::ffi::c_uint)
                                                    .wrapping_sub(1 as ::core::ffi::c_uint) as u32_0;
                                            } else {
                                                rc = sqlite3VdbeFinishMoveto(pC_1);
                                                if rc != 0 {
                                                    c2rust_current_block = 8086965459351668542;
                                                    break 's_109;
                                                } else {
                                                    c2rust_current_block = 13807608635351685354;
                                                    break;
                                                }
                                            }
                                        } else {
                                            if !(sqlite3BtreeCursorHasMoved(pCrsr) != 0) {
                                                c2rust_current_block = 13807608635351685354;
                                                break;
                                            }
                                            rc = sqlite3VdbeHandleMovedCursor(pC_1);
                                            if rc != 0 {
                                                c2rust_current_block = 8086965459351668542;
                                                break 's_109;
                                            }
                                        }
                                    }
                                } else {
                                    if !(sqlite3BtreeCursorHasMoved((*pC_1).uc.pCursor) != 0) {
                                        c2rust_current_block = 5316308194266942939;
                                        break;
                                    }
                                    rc = sqlite3VdbeHandleMovedCursor(pC_1);
                                    if rc != 0 {
                                        c2rust_current_block = 8086965459351668542;
                                        break 's_109;
                                    }
                                }
                            }
                            match c2rust_current_block {
                                13807608635351685354 => {
                                    (*pC_1).payloadSize = sqlite3BtreePayloadSize(pCrsr);
                                    (*pC_1).aRow = sqlite3BtreePayloadFetch(
                                        pCrsr,
                                        &raw mut (*pC_1).szRow,
                                    ) as *const u8_0;
                                    c2rust_current_block = 4364832782951812655;
                                }
                                830712628114968639 => {
                                    pDest = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                                    sqlite3VdbeMemSetNull(pDest);
                                    c2rust_current_block = 5783071609795492627;
                                }
                                3471494006171716190 => {
                                    pReg = aMem.offset((*pC_1).seekResult as isize) as *mut Mem;
                                    (*pC_1).szRow = (*pReg).n as u32_0;
                                    (*pC_1).payloadSize = (*pC_1).szRow;
                                    (*pC_1).aRow = (*pReg).z as *mut u8_0;
                                    c2rust_current_block = 4364832782951812655;
                                }
                                _ => {}
                            }
                            match c2rust_current_block {
                                5783071609795492627 => {}
                                _ => {
                                    match c2rust_current_block {
                                        4364832782951812655 => {
                                            (*pC_1).cacheStatus = (*p).cacheCtr;
                                            let ref mut c2rust_fresh3 = *aOffset
                                                .offset(0 as ::core::ffi::c_int as isize);
                                            *c2rust_fresh3 = *(*pC_1)
                                                .aRow
                                                .offset(0 as ::core::ffi::c_int as isize) as u32_0;
                                            if *c2rust_fresh3 < 0x80 as ::core::ffi::c_uint {
                                                (*pC_1).iHdrOffset = 1 as u32_0;
                                            } else {
                                                (*pC_1).iHdrOffset = sqlite3GetVarint32(
                                                    (*pC_1).aRow as *const ::core::ffi::c_uchar,
                                                    aOffset,
                                                ) as u32_0;
                                            }
                                            (*pC_1).nHdrParsed = 0 as u16_0;
                                            if (*pC_1).szRow
                                                < *aOffset.offset(0 as ::core::ffi::c_int as isize)
                                            {
                                                (*pC_1).aRow = ::core::ptr::null::<u8_0>();
                                                (*pC_1).szRow = 0 as u32_0;
                                                if *aOffset.offset(0 as ::core::ffi::c_int as isize)
                                                    > 98307 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                    || *aOffset.offset(0 as ::core::ffi::c_int as isize)
                                                        > (*pC_1).payloadSize
                                                {
                                                    c2rust_current_block = 12869880844849433763;
                                                } else {
                                                    c2rust_current_block = 5316308194266942939;
                                                }
                                            } else {
                                                zData = (*pC_1).aRow;
                                                c2rust_current_block = 10859987898703693782;
                                            }
                                        }
                                        _ => {}
                                    }
                                    match c2rust_current_block {
                                        5316308194266942939 => {
                                            if (*pC_1).nHdrParsed as ::core::ffi::c_uint <= p2_1 {
                                                if (*pC_1).iHdrOffset
                                                    < *aOffset.offset(0 as ::core::ffi::c_int as isize)
                                                {
                                                    if (*pC_1).aRow.is_null() {
                                                        memset(
                                                            &raw mut sMem as *mut ::core::ffi::c_void,
                                                            0 as ::core::ffi::c_int,
                                                            ::core::mem::size_of::<Mem>() as size_t,
                                                        );
                                                        rc = sqlite3VdbeMemFromBtreeZeroOffset(
                                                            (*pC_1).uc.pCursor,
                                                            *aOffset.offset(0 as ::core::ffi::c_int as isize),
                                                            &raw mut sMem,
                                                        );
                                                        if rc != SQLITE_OK {
                                                            c2rust_current_block = 8086965459351668542;
                                                            break;
                                                        }
                                                        zData = sMem.z as *mut u8_0;
                                                    } else {
                                                        zData = (*pC_1).aRow;
                                                    }
                                                    c2rust_current_block = 10859987898703693782;
                                                } else {
                                                    t = 0 as u32_0;
                                                    c2rust_current_block = 2793942463428672759;
                                                }
                                            } else {
                                                t = *(&raw mut (*pC_1).aType as *mut u32_0)
                                                    .offset(p2_1 as isize);
                                                c2rust_current_block = 11498438067958447343;
                                            }
                                        }
                                        _ => {}
                                    }
                                    match c2rust_current_block {
                                        10859987898703693782 => {
                                            i_0 = (*pC_1).nHdrParsed as ::core::ffi::c_int;
                                            offset64 = *aOffset.offset(i_0 as isize) as u64_0;
                                            zHdr = zData.offset((*pC_1).iHdrOffset as isize);
                                            zEndHdr = zData
                                                .offset(
                                                    *aOffset.offset(0 as ::core::ffi::c_int as isize) as isize,
                                                );
                                            loop {
                                                t = *zHdr.offset(0 as ::core::ffi::c_int as isize) as u32_0;
                                                let ref mut c2rust_fresh4 = *(&raw mut (*pC_1).aType
                                                    as *mut u32_0)
                                                    .offset(i_0 as isize);
                                                *c2rust_fresh4 = t;
                                                if *c2rust_fresh4 < 0x80 as ::core::ffi::c_uint {
                                                    zHdr = zHdr.offset(1);
                                                    offset64 = (offset64 as ::core::ffi::c_ulonglong)
                                                        .wrapping_add(
                                                            sqlite3VdbeOneByteSerialTypeLen(t as u8_0)
                                                                as ::core::ffi::c_ulonglong,
                                                        ) as u64_0 as u64_0;
                                                } else {
                                                    zHdr = zHdr
                                                        .offset(
                                                            sqlite3GetVarint32(
                                                                zHdr as *const ::core::ffi::c_uchar,
                                                                &raw mut t,
                                                            ) as ::core::ffi::c_int as isize,
                                                        );
                                                    *(&raw mut (*pC_1).aType as *mut u32_0)
                                                        .offset(i_0 as isize) = t;
                                                    offset64 = (offset64 as ::core::ffi::c_ulonglong)
                                                        .wrapping_add(
                                                            sqlite3VdbeSerialTypeLen(t) as ::core::ffi::c_ulonglong,
                                                        ) as u64_0 as u64_0;
                                                }
                                                i_0 += 1;
                                                *aOffset.offset(i_0 as isize) = (offset64
                                                    as ::core::ffi::c_ulonglong
                                                    & 0xffffffff as ::core::ffi::c_ulonglong) as u32_0;
                                                if !(i_0 as u32_0 <= p2_1 && zHdr < zEndHdr) {
                                                    break;
                                                }
                                            }
                                            if zHdr >= zEndHdr
                                                && (zHdr > zEndHdr
                                                    || offset64
                                                        != (*pC_1).payloadSize as ::core::ffi::c_ulonglong)
                                                || offset64
                                                    > (*pC_1).payloadSize as ::core::ffi::c_ulonglong
                                            {
                                                if *aOffset.offset(0 as ::core::ffi::c_int as isize)
                                                    == 0 as ::core::ffi::c_uint
                                                {
                                                    i_0 = 0 as ::core::ffi::c_int;
                                                    zHdr = zEndHdr;
                                                    c2rust_current_block = 15745833042277525878;
                                                } else {
                                                    if (*pC_1).aRow.is_null() {
                                                        sqlite3VdbeMemRelease(&raw mut sMem);
                                                    }
                                                    c2rust_current_block = 12869880844849433763;
                                                }
                                            } else {
                                                c2rust_current_block = 15745833042277525878;
                                            }
                                            match c2rust_current_block {
                                                12869880844849433763 => {}
                                                _ => {
                                                    (*pC_1).nHdrParsed = i_0 as u16_0;
                                                    (*pC_1).iHdrOffset = zHdr.offset_from(zData)
                                                        as ::core::ffi::c_long as u32_0;
                                                    if (*pC_1).aRow.is_null() {
                                                        sqlite3VdbeMemRelease(&raw mut sMem);
                                                    }
                                                    c2rust_current_block = 2793942463428672759;
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                    match c2rust_current_block {
                                        2793942463428672759 => {
                                            if (*pC_1).nHdrParsed as ::core::ffi::c_uint <= p2_1 {
                                                pDest = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                                                if (*pOp).p4type as ::core::ffi::c_int == P4_MEM {
                                                    sqlite3VdbeMemShallowCopy(
                                                        pDest,
                                                        (*pOp).p4.pMem,
                                                        MEM_Static,
                                                    );
                                                } else {
                                                    sqlite3VdbeMemSetNull(pDest);
                                                }
                                                c2rust_current_block = 5783071609795492627;
                                            } else {
                                                c2rust_current_block = 11498438067958447343;
                                            }
                                        }
                                        12869880844849433763 => {
                                            if (*aOp.offset(0 as ::core::ffi::c_int as isize)).p3
                                                > 0 as ::core::ffi::c_int
                                            {
                                                pOp = aOp
                                                    .offset(
                                                        ((*aOp.offset(0 as ::core::ffi::c_int as isize)).p3
                                                            - 1 as ::core::ffi::c_int) as isize,
                                                    ) as *mut Op;
                                            } else {
                                                rc = sqlite3CorruptError(3263 as ::core::ffi::c_int);
                                                c2rust_current_block = 8086965459351668542;
                                                break;
                                            }
                                            c2rust_current_block = 5783071609795492627;
                                        }
                                        _ => {}
                                    }
                                    match c2rust_current_block {
                                        5783071609795492627 => {}
                                        _ => {
                                            pDest = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                                            if (*pDest).flags as ::core::ffi::c_int
                                                & (MEM_Agg | MEM_Dyn) != 0 as ::core::ffi::c_int
                                            {
                                                sqlite3VdbeMemSetNull(pDest);
                                            }
                                            if (*pC_1).szRow
                                                >= *aOffset
                                                    .offset(
                                                        (p2_1 as ::core::ffi::c_uint)
                                                            .wrapping_add(1 as ::core::ffi::c_uint) as isize,
                                                    )
                                            {
                                                zData = (*pC_1)
                                                    .aRow
                                                    .offset(*aOffset.offset(p2_1 as isize) as isize);
                                                if t < 12 as ::core::ffi::c_uint {
                                                    sqlite3VdbeSerialGet(
                                                        zData as *const ::core::ffi::c_uchar,
                                                        t,
                                                        pDest,
                                                    );
                                                } else {
                                                    static mut aFlag: [u16_0; 2] = [
                                                        MEM_Blob as u16_0,
                                                        (MEM_Str | MEM_Term) as u16_0,
                                                    ];
                                                    len = (t as ::core::ffi::c_uint)
                                                        .wrapping_sub(12 as ::core::ffi::c_uint)
                                                        .wrapping_div(2 as ::core::ffi::c_uint)
                                                        as ::core::ffi::c_int;
                                                    (*pDest).n = len;
                                                    (*pDest).enc = encoding;
                                                    if (*pDest).szMalloc < len + 2 as ::core::ffi::c_int {
                                                        if len > (*db).aLimit[SQLITE_LIMIT_LENGTH as usize] {
                                                            c2rust_current_block = 13536115470629238340;
                                                            break;
                                                        }
                                                        (*pDest).flags = MEM_Null as u16_0;
                                                        if sqlite3VdbeMemGrow(
                                                            pDest,
                                                            len + 2 as ::core::ffi::c_int,
                                                            0 as ::core::ffi::c_int,
                                                        ) != 0
                                                        {
                                                            c2rust_current_block = 9955115899106739073;
                                                            break;
                                                        }
                                                    } else {
                                                        (*pDest).z = (*pDest).zMalloc;
                                                    }
                                                    memcpy(
                                                        (*pDest).z as *mut ::core::ffi::c_void,
                                                        zData as *const ::core::ffi::c_void,
                                                        len as size_t,
                                                    );
                                                    *(*pDest).z.offset(len as isize) = 0 as ::core::ffi::c_char;
                                                    *(*pDest)
                                                        .z
                                                        .offset((len + 1 as ::core::ffi::c_int) as isize) = 0
                                                        as ::core::ffi::c_char;
                                                    (*pDest).flags = aFlag[(t as ::core::ffi::c_uint
                                                        & 1 as ::core::ffi::c_uint) as usize];
                                                }
                                            } else {
                                                let mut p5: u8_0 = 0;
                                                (*pDest).enc = encoding;
                                                p5 = ((*pOp).p5 as ::core::ffi::c_int & OPFLAG_BYTELENARG)
                                                    as u8_0;
                                                if p5 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                                    && (p5 as ::core::ffi::c_int == OPFLAG_TYPEOFARG
                                                        || t >= 12 as ::core::ffi::c_uint
                                                            && (t as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint
                                                                == 0 as ::core::ffi::c_uint
                                                                || p5 as ::core::ffi::c_int == OPFLAG_BYTELENARG))
                                                    || sqlite3VdbeSerialTypeLen(t) == 0 as ::core::ffi::c_uint
                                                {
                                                    sqlite3VdbeSerialGet(
                                                        &raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar
                                                            as *mut u8_0,
                                                        t,
                                                        pDest,
                                                    );
                                                } else {
                                                    rc = vdbeColumnFromOverflow(
                                                        pC_1,
                                                        p2_1 as ::core::ffi::c_int,
                                                        t,
                                                        *aOffset.offset(p2_1 as isize) as i64_0,
                                                        (*p).cacheCtr,
                                                        colCacheCtr,
                                                        pDest,
                                                    );
                                                    if rc != 0 {
                                                        if rc == SQLITE_NOMEM {
                                                            c2rust_current_block = 9955115899106739073;
                                                            break;
                                                        }
                                                        if rc == SQLITE_TOOBIG {
                                                            c2rust_current_block = 13536115470629238340;
                                                            break;
                                                        } else {
                                                            c2rust_current_block = 8086965459351668542;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                            c2rust_current_block = 5783071609795492627;
                                        }
                                    }
                                }
                            }
                        }
                        OP_TypeCheck => {
                            let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
                            let mut aCol: *mut Column = ::core::ptr::null_mut::<
                                Column,
                            >();
                            let mut i_1: ::core::ffi::c_int = 0;
                            let mut nCol: ::core::ffi::c_int = 0;
                            pTab = (*pOp).p4.pTab;
                            aCol = (*pTab).aCol;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            if (*pOp).p3 < 2 as ::core::ffi::c_int {
                                i_1 = 0 as ::core::ffi::c_int;
                                nCol = (*pTab).nCol as ::core::ffi::c_int;
                            } else {
                                i_1 = (*pOp).p3 - 2 as ::core::ffi::c_int;
                                nCol = i_1 + 1 as ::core::ffi::c_int;
                            }
                            loop {
                                if !(i_1 < nCol) {
                                    c2rust_current_block = 5783071609795492627;
                                    break;
                                }
                                if (*aCol.offset(i_1 as isize)).colFlags
                                    as ::core::ffi::c_int & COLFLAG_GENERATED
                                    != 0 as ::core::ffi::c_int
                                    && (*pOp).p3 < 2 as ::core::ffi::c_int
                                {
                                    if (*aCol.offset(i_1 as isize)).colFlags
                                        as ::core::ffi::c_int & COLFLAG_VIRTUAL
                                        != 0 as ::core::ffi::c_int
                                    {
                                        c2rust_current_block = 11193982772511216995;
                                    } else if (*pOp).p3 != 0 {
                                        pIn1 = pIn1.offset(1);
                                        c2rust_current_block = 11193982772511216995;
                                    } else {
                                        c2rust_current_block = 5841212014800822749;
                                    }
                                } else {
                                    c2rust_current_block = 5841212014800822749;
                                }
                                match c2rust_current_block {
                                    5841212014800822749 => {
                                        applyAffinity(
                                            pIn1,
                                            (*aCol.offset(i_1 as isize)).affinity,
                                            encoding,
                                        );
                                        if (*pIn1).flags as ::core::ffi::c_int & MEM_Null
                                            == 0 as ::core::ffi::c_int
                                        {
                                            match (*aCol.offset(i_1 as isize)).eCType()
                                                as ::core::ffi::c_int
                                            {
                                                COLTYPE_BLOB => {
                                                    c2rust_current_block = 4046737816052703705;
                                                    match c2rust_current_block {
                                                        2304935808676644642 => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Str
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                        4046737816052703705 => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Blob
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                        3089798912646860745 => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Int
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                        _ => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Int != 0 {
                                                                if (*pIn1).u.i <= 140737488355327 as ::core::ffi::c_longlong
                                                                    && (*pIn1).u.i
                                                                        >= -140737488355328 as ::core::ffi::c_longlong
                                                                {
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        | MEM_IntReal) as u16_0;
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        & !MEM_Int) as u16_0;
                                                                } else {
                                                                    (*pIn1).u.r = (*pIn1).u.i as ::core::ffi::c_double;
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        | MEM_Real) as u16_0;
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        & !MEM_Int) as u16_0;
                                                                }
                                                            } else if (*pIn1).flags as ::core::ffi::c_int
                                                                & (MEM_Real | MEM_IntReal) == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                                COLTYPE_INTEGER | COLTYPE_INT => {
                                                    c2rust_current_block = 3089798912646860745;
                                                    match c2rust_current_block {
                                                        2304935808676644642 => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Str
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                        4046737816052703705 => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Blob
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                        3089798912646860745 => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Int
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                        _ => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Int != 0 {
                                                                if (*pIn1).u.i <= 140737488355327 as ::core::ffi::c_longlong
                                                                    && (*pIn1).u.i
                                                                        >= -140737488355328 as ::core::ffi::c_longlong
                                                                {
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        | MEM_IntReal) as u16_0;
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        & !MEM_Int) as u16_0;
                                                                } else {
                                                                    (*pIn1).u.r = (*pIn1).u.i as ::core::ffi::c_double;
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        | MEM_Real) as u16_0;
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        & !MEM_Int) as u16_0;
                                                                }
                                                            } else if (*pIn1).flags as ::core::ffi::c_int
                                                                & (MEM_Real | MEM_IntReal) == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                                COLTYPE_TEXT => {
                                                    c2rust_current_block = 2304935808676644642;
                                                    match c2rust_current_block {
                                                        2304935808676644642 => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Str
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                        4046737816052703705 => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Blob
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                        3089798912646860745 => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Int
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                        _ => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Int != 0 {
                                                                if (*pIn1).u.i <= 140737488355327 as ::core::ffi::c_longlong
                                                                    && (*pIn1).u.i
                                                                        >= -140737488355328 as ::core::ffi::c_longlong
                                                                {
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        | MEM_IntReal) as u16_0;
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        & !MEM_Int) as u16_0;
                                                                } else {
                                                                    (*pIn1).u.r = (*pIn1).u.i as ::core::ffi::c_double;
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        | MEM_Real) as u16_0;
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        & !MEM_Int) as u16_0;
                                                                }
                                                            } else if (*pIn1).flags as ::core::ffi::c_int
                                                                & (MEM_Real | MEM_IntReal) == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                                COLTYPE_REAL => {
                                                    c2rust_current_block = 17590250397297081215;
                                                    match c2rust_current_block {
                                                        2304935808676644642 => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Str
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                        4046737816052703705 => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Blob
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                        3089798912646860745 => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Int
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                        _ => {
                                                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Int != 0 {
                                                                if (*pIn1).u.i <= 140737488355327 as ::core::ffi::c_longlong
                                                                    && (*pIn1).u.i
                                                                        >= -140737488355328 as ::core::ffi::c_longlong
                                                                {
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        | MEM_IntReal) as u16_0;
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        & !MEM_Int) as u16_0;
                                                                } else {
                                                                    (*pIn1).u.r = (*pIn1).u.i as ::core::ffi::c_double;
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        | MEM_Real) as u16_0;
                                                                    (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                                        & !MEM_Int) as u16_0;
                                                                }
                                                            } else if (*pIn1).flags as ::core::ffi::c_int
                                                                & (MEM_Real | MEM_IntReal) == 0 as ::core::ffi::c_int
                                                            {
                                                                c2rust_current_block = 9839710560781051809;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        pIn1 = pIn1.offset(1);
                                    }
                                    _ => {}
                                }
                                i_1 += 1;
                            }
                            match c2rust_current_block {
                                5783071609795492627 => {}
                                _ => {
                                    sqlite3VdbeError(
                                        p,
                                        b"cannot store %s value in %s column %s.%s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        vdbeMemTypeName(pIn1),
                                        *(&raw mut sqlite3StdType
                                            as *mut *const ::core::ffi::c_char)
                                            .offset(
                                                ((*aCol.offset(i_1 as isize)).eCType() as ::core::ffi::c_int
                                                    - 1 as ::core::ffi::c_int) as isize,
                                            ),
                                        (*pTab).zName,
                                        (*aCol.offset(i_1 as isize)).zCnName,
                                    );
                                    rc = SQLITE_CONSTRAINT_DATATYPE;
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                            }
                        }
                        OP_Affinity => {
                            let mut zAffinity: *const ::core::ffi::c_char = ::core::ptr::null::<
                                ::core::ffi::c_char,
                            >();
                            zAffinity = (*pOp).p4.z;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            loop {
                                applyAffinity(
                                    pIn1,
                                    *zAffinity.offset(0 as ::core::ffi::c_int as isize),
                                    encoding,
                                );
                                if *zAffinity.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int == SQLITE_AFF_REAL
                                    && (*pIn1).flags as ::core::ffi::c_int & MEM_Int
                                        != 0 as ::core::ffi::c_int
                                {
                                    if (*pIn1).u.i <= 140737488355327 as ::core::ffi::c_longlong
                                        && (*pIn1).u.i
                                            >= -140737488355328 as ::core::ffi::c_longlong
                                    {
                                        (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                            | MEM_IntReal) as u16_0;
                                        (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                            & !MEM_Int) as u16_0;
                                    } else {
                                        (*pIn1).u.r = (*pIn1).u.i as ::core::ffi::c_double;
                                        (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                            | MEM_Real) as u16_0;
                                        (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                            & !(MEM_Int | MEM_Str)) as u16_0;
                                    }
                                }
                                zAffinity = zAffinity.offset(1);
                                if *zAffinity.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                                pIn1 = pIn1.offset(1);
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_MakeRecord => {
                            let mut pRec: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut nData: u64_0 = 0;
                            let mut nHdr: ::core::ffi::c_int = 0;
                            let mut nByte_0: i64_0 = 0;
                            let mut nZero: i64_0 = 0;
                            let mut nVarint: ::core::ffi::c_int = 0;
                            let mut serial_type: u32_0 = 0;
                            let mut pData0: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut pLast: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut nField: ::core::ffi::c_int = 0;
                            let mut zAffinity_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                ::core::ffi::c_char,
                            >();
                            let mut len_0: u32_0 = 0;
                            let mut zHdr_0: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                            let mut zPayload: *mut u8_0 = ::core::ptr::null_mut::<
                                u8_0,
                            >();
                            nData = 0 as u64_0;
                            nHdr = 0 as ::core::ffi::c_int;
                            nZero = 0 as i64_0;
                            nField = (*pOp).p1;
                            zAffinity_0 = (*pOp).p4.z;
                            pData0 = aMem.offset(nField as isize) as *mut Mem;
                            nField = (*pOp).p2;
                            pLast = pData0
                                .offset((nField - 1 as ::core::ffi::c_int) as isize)
                                as *mut Mem;
                            pOut = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            if !zAffinity_0.is_null() {
                                pRec = pData0;
                                loop {
                                    applyAffinity(
                                        pRec,
                                        *zAffinity_0.offset(0 as ::core::ffi::c_int as isize),
                                        encoding,
                                    );
                                    if *zAffinity_0.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int == SQLITE_AFF_REAL
                                        && (*pRec).flags as ::core::ffi::c_int & MEM_Int != 0
                                    {
                                        (*pRec).flags = ((*pRec).flags as ::core::ffi::c_int
                                            | MEM_IntReal) as u16_0;
                                        (*pRec).flags = ((*pRec).flags as ::core::ffi::c_int
                                            & !(0x4 as ::core::ffi::c_int)) as u16_0;
                                    }
                                    zAffinity_0 = zAffinity_0.offset(1);
                                    pRec = pRec.offset(1);
                                    if !(*zAffinity_0.offset(0 as ::core::ffi::c_int as isize)
                                        != 0)
                                    {
                                        break;
                                    }
                                }
                            }
                            pRec = pLast;
                            loop {
                                if (*pRec).flags as ::core::ffi::c_int & MEM_Null != 0 {
                                    if (*pRec).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                                        (*pRec).uTemp = 10 as u32_0;
                                    } else {
                                        (*pRec).uTemp = 0 as u32_0;
                                    }
                                    nHdr += 1;
                                } else if (*pRec).flags as ::core::ffi::c_int
                                    & (MEM_Int | MEM_IntReal) != 0
                                {
                                    let mut i_2: i64_0 = (*pRec).u.i;
                                    let mut uu: u64_0 = 0;
                                    if i_2 < 0 as ::core::ffi::c_longlong {
                                        uu = !i_2 as u64_0;
                                    } else {
                                        uu = i_2 as u64_0;
                                    }
                                    nHdr += 1;
                                    if uu <= 127 as ::core::ffi::c_ulonglong {
                                        if i_2 as ::core::ffi::c_longlong
                                            & 1 as ::core::ffi::c_longlong == i_2
                                            && (*p).minWriteFileFormat as ::core::ffi::c_int
                                                >= 4 as ::core::ffi::c_int
                                        {
                                            (*pRec).uTemp = (8 as u32_0).wrapping_add(uu as u32_0);
                                        } else {
                                            nData = nData.wrapping_add(1);
                                            (*pRec).uTemp = 1 as u32_0;
                                        }
                                    } else if uu <= 32767 as ::core::ffi::c_ulonglong {
                                        nData = (nData as ::core::ffi::c_ulonglong)
                                            .wrapping_add(2 as ::core::ffi::c_ulonglong) as u64_0
                                            as u64_0;
                                        (*pRec).uTemp = 2 as u32_0;
                                    } else if uu <= 8388607 as ::core::ffi::c_ulonglong {
                                        nData = (nData as ::core::ffi::c_ulonglong)
                                            .wrapping_add(3 as ::core::ffi::c_ulonglong) as u64_0
                                            as u64_0;
                                        (*pRec).uTemp = 3 as u32_0;
                                    } else if uu <= 2147483647 as ::core::ffi::c_ulonglong {
                                        nData = (nData as ::core::ffi::c_ulonglong)
                                            .wrapping_add(4 as ::core::ffi::c_ulonglong) as u64_0
                                            as u64_0;
                                        (*pRec).uTemp = 4 as u32_0;
                                    } else if uu <= 140737488355327 as ::core::ffi::c_ulonglong
                                    {
                                        nData = (nData as ::core::ffi::c_ulonglong)
                                            .wrapping_add(6 as ::core::ffi::c_ulonglong) as u64_0
                                            as u64_0;
                                        (*pRec).uTemp = 5 as u32_0;
                                    } else {
                                        nData = (nData as ::core::ffi::c_ulonglong)
                                            .wrapping_add(8 as ::core::ffi::c_ulonglong) as u64_0
                                            as u64_0;
                                        if (*pRec).flags as ::core::ffi::c_int & MEM_IntReal != 0 {
                                            (*pRec).u.r = (*pRec).u.i as ::core::ffi::c_double;
                                            (*pRec).flags = ((*pRec).flags as ::core::ffi::c_int
                                                & !MEM_IntReal) as u16_0;
                                            (*pRec).flags = ((*pRec).flags as ::core::ffi::c_int
                                                | MEM_Real) as u16_0;
                                            (*pRec).uTemp = 7 as u32_0;
                                        } else {
                                            (*pRec).uTemp = 6 as u32_0;
                                        }
                                    }
                                } else if (*pRec).flags as ::core::ffi::c_int & MEM_Real
                                    != 0
                                {
                                    nHdr += 1;
                                    nData = (nData as ::core::ffi::c_ulonglong)
                                        .wrapping_add(8 as ::core::ffi::c_ulonglong) as u64_0
                                        as u64_0;
                                    (*pRec).uTemp = 7 as u32_0;
                                } else {
                                    len_0 = (*pRec).n as u32_0;
                                    serial_type = (len_0 as ::core::ffi::c_uint)
                                        .wrapping_mul(2 as ::core::ffi::c_uint)
                                        .wrapping_add(12 as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            ((*pRec).flags as ::core::ffi::c_int & MEM_Str
                                                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                                                as ::core::ffi::c_uint,
                                        ) as u32_0;
                                    if (*pRec).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                                        serial_type = (serial_type as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                ((*pRec).u.nZero as ::core::ffi::c_uint)
                                                    .wrapping_mul(2 as ::core::ffi::c_uint),
                                            ) as u32_0 as u32_0;
                                        if nData != 0 {
                                            if sqlite3VdbeMemExpandBlob(pRec) != 0 {
                                                c2rust_current_block = 9955115899106739073;
                                                break 's_109;
                                            }
                                            len_0 = (len_0 as ::core::ffi::c_uint)
                                                .wrapping_add((*pRec).u.nZero as ::core::ffi::c_uint)
                                                as u32_0 as u32_0;
                                        } else {
                                            nZero += (*pRec).u.nZero as ::core::ffi::c_longlong;
                                        }
                                    }
                                    nData = (nData as ::core::ffi::c_ulonglong)
                                        .wrapping_add(len_0 as ::core::ffi::c_ulonglong) as u64_0
                                        as u64_0;
                                    nHdr += sqlite3VarintLen(serial_type as u64_0);
                                    (*pRec).uTemp = serial_type;
                                }
                                if pRec == pData0 {
                                    break;
                                }
                                pRec = pRec.offset(-1);
                            }
                            if nHdr <= 126 as ::core::ffi::c_int {
                                nHdr += 1 as ::core::ffi::c_int;
                            } else {
                                nVarint = sqlite3VarintLen(nHdr as u64_0);
                                nHdr += nVarint;
                                if nVarint < sqlite3VarintLen(nHdr as u64_0) {
                                    nHdr += 1;
                                }
                            }
                            nByte_0 = (nHdr as u64_0).wrapping_add(nData) as i64_0;
                            if nByte_0 + nZero
                                <= (*pOut).szMalloc as ::core::ffi::c_longlong
                            {
                                (*pOut).z = (*pOut).zMalloc;
                            } else {
                                if nByte_0 + nZero
                                    > (*db).aLimit[SQLITE_LIMIT_LENGTH as usize]
                                        as ::core::ffi::c_longlong
                                {
                                    c2rust_current_block = 13536115470629238340;
                                    break;
                                }
                                if sqlite3VdbeMemClearAndResize(
                                    pOut,
                                    nByte_0 as ::core::ffi::c_int,
                                ) != 0
                                {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                }
                            }
                            (*pOut).n = nByte_0 as ::core::ffi::c_int;
                            (*pOut).flags = MEM_Blob as u16_0;
                            if nZero != 0 {
                                (*pOut).u.nZero = nZero as ::core::ffi::c_int;
                                (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                    | MEM_Zero) as u16_0;
                            }
                            zHdr_0 = (*pOut).z as *mut u8_0;
                            zPayload = zHdr_0.offset(nHdr as isize);
                            if nHdr < 0x80 as ::core::ffi::c_int {
                                let c2rust_fresh5 = zHdr_0;
                                zHdr_0 = zHdr_0.offset(1);
                                *c2rust_fresh5 = nHdr as u8_0;
                            } else {
                                zHdr_0 = zHdr_0
                                    .offset(
                                        sqlite3PutVarint(
                                            zHdr_0 as *mut ::core::ffi::c_uchar,
                                            nHdr as u64_0,
                                        ) as isize,
                                    );
                            }
                            pRec = pData0;
                            loop {
                                serial_type = (*pRec).uTemp;
                                if serial_type <= 7 as ::core::ffi::c_uint {
                                    let c2rust_fresh6 = zHdr_0;
                                    zHdr_0 = zHdr_0.offset(1);
                                    *c2rust_fresh6 = serial_type as u8_0;
                                    if !(serial_type == 0 as ::core::ffi::c_uint) {
                                        let mut v: u64_0 = 0;
                                        if serial_type == 7 as ::core::ffi::c_uint {
                                            memcpy(
                                                &raw mut v as *mut ::core::ffi::c_void,
                                                &raw mut (*pRec).u.r as *const ::core::ffi::c_void,
                                                ::core::mem::size_of::<u64_0>() as size_t,
                                            );
                                        } else {
                                            v = (*pRec).u.i as u64_0;
                                        }
                                        len_0 = *(&raw const sqlite3SmallTypeSizes as *const u8_0)
                                            .offset(serial_type as isize) as u32_0;
                                        let mut c2rust_current_block_953: u64;
                                        match len_0 {
                                            6 => {
                                                c2rust_current_block_953 = 15638369786160630947;
                                            }
                                            4 => {
                                                c2rust_current_block_953 = 15991945663577458498;
                                            }
                                            3 => {
                                                c2rust_current_block_953 = 4236070176017418430;
                                            }
                                            2 => {
                                                c2rust_current_block_953 = 4845812691790365806;
                                            }
                                            1 => {
                                                c2rust_current_block_953 = 7870975959482292763;
                                            }
                                            _ => {
                                                *zPayload.offset(7 as ::core::ffi::c_int as isize) = (v
                                                    as ::core::ffi::c_ulonglong
                                                    & 0xff as ::core::ffi::c_ulonglong) as u8_0;
                                                v >>= 8 as ::core::ffi::c_int;
                                                *zPayload.offset(6 as ::core::ffi::c_int as isize) = (v
                                                    as ::core::ffi::c_ulonglong
                                                    & 0xff as ::core::ffi::c_ulonglong) as u8_0;
                                                v >>= 8 as ::core::ffi::c_int;
                                                c2rust_current_block_953 = 15638369786160630947;
                                            }
                                        }
                                        match c2rust_current_block_953 {
                                            15638369786160630947 => {
                                                *zPayload.offset(5 as ::core::ffi::c_int as isize) = (v
                                                    as ::core::ffi::c_ulonglong
                                                    & 0xff as ::core::ffi::c_ulonglong) as u8_0;
                                                v >>= 8 as ::core::ffi::c_int;
                                                *zPayload.offset(4 as ::core::ffi::c_int as isize) = (v
                                                    as ::core::ffi::c_ulonglong
                                                    & 0xff as ::core::ffi::c_ulonglong) as u8_0;
                                                v >>= 8 as ::core::ffi::c_int;
                                                c2rust_current_block_953 = 15991945663577458498;
                                            }
                                            _ => {}
                                        }
                                        match c2rust_current_block_953 {
                                            15991945663577458498 => {
                                                *zPayload.offset(3 as ::core::ffi::c_int as isize) = (v
                                                    as ::core::ffi::c_ulonglong
                                                    & 0xff as ::core::ffi::c_ulonglong) as u8_0;
                                                v >>= 8 as ::core::ffi::c_int;
                                                c2rust_current_block_953 = 4236070176017418430;
                                            }
                                            _ => {}
                                        }
                                        match c2rust_current_block_953 {
                                            4236070176017418430 => {
                                                *zPayload.offset(2 as ::core::ffi::c_int as isize) = (v
                                                    as ::core::ffi::c_ulonglong
                                                    & 0xff as ::core::ffi::c_ulonglong) as u8_0;
                                                v >>= 8 as ::core::ffi::c_int;
                                                c2rust_current_block_953 = 4845812691790365806;
                                            }
                                            _ => {}
                                        }
                                        match c2rust_current_block_953 {
                                            4845812691790365806 => {
                                                *zPayload.offset(1 as ::core::ffi::c_int as isize) = (v
                                                    as ::core::ffi::c_ulonglong
                                                    & 0xff as ::core::ffi::c_ulonglong) as u8_0;
                                                v >>= 8 as ::core::ffi::c_int;
                                            }
                                            _ => {}
                                        }
                                        *zPayload.offset(0 as ::core::ffi::c_int as isize) = (v
                                            as ::core::ffi::c_ulonglong
                                            & 0xff as ::core::ffi::c_ulonglong) as u8_0;
                                        zPayload = zPayload.offset(len_0 as isize);
                                    }
                                } else if serial_type < 0x80 as ::core::ffi::c_uint {
                                    let c2rust_fresh7 = zHdr_0;
                                    zHdr_0 = zHdr_0.offset(1);
                                    *c2rust_fresh7 = serial_type as u8_0;
                                    if serial_type >= 14 as ::core::ffi::c_uint
                                        && (*pRec).n > 0 as ::core::ffi::c_int
                                    {
                                        memcpy(
                                            zPayload as *mut ::core::ffi::c_void,
                                            (*pRec).z as *const ::core::ffi::c_void,
                                            (*pRec).n as size_t,
                                        );
                                        zPayload = zPayload.offset((*pRec).n as isize);
                                    }
                                } else {
                                    zHdr_0 = zHdr_0
                                        .offset(
                                            sqlite3PutVarint(
                                                zHdr_0 as *mut ::core::ffi::c_uchar,
                                                serial_type as u64_0,
                                            ) as isize,
                                        );
                                    if (*pRec).n != 0 {
                                        memcpy(
                                            zPayload as *mut ::core::ffi::c_void,
                                            (*pRec).z as *const ::core::ffi::c_void,
                                            (*pRec).n as size_t,
                                        );
                                        zPayload = zPayload.offset((*pRec).n as isize);
                                    }
                                }
                                if pRec == pLast {
                                    break;
                                }
                                pRec = pRec.offset(1);
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Count => {
                            let mut nEntry: i64_0 = 0;
                            let mut pCrsr_0: *mut BtCursor = ::core::ptr::null_mut::<
                                BtCursor,
                            >();
                            pCrsr_0 = (**(*p).apCsr.offset((*pOp).p1 as isize))
                                .uc
                                .pCursor;
                            if (*pOp).p3 != 0 {
                                nEntry = sqlite3BtreeRowCountEst(pCrsr_0);
                            } else {
                                nEntry = 0 as i64_0;
                                rc = sqlite3BtreeCount(db, pCrsr_0, &raw mut nEntry);
                                if rc != 0 {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                            }
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            (*pOut).u.i = nEntry;
                            c2rust_current_block = 6498258645234546759;
                        }
                        OP_Savepoint => {
                            let mut p1_1: ::core::ffi::c_int = 0;
                            let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                ::core::ffi::c_char,
                            >();
                            let mut nName: ::core::ffi::c_int = 0;
                            let mut pNew: *mut Savepoint = ::core::ptr::null_mut::<
                                Savepoint,
                            >();
                            let mut pSavepoint: *mut Savepoint = ::core::ptr::null_mut::<
                                Savepoint,
                            >();
                            let mut pTmp: *mut Savepoint = ::core::ptr::null_mut::<
                                Savepoint,
                            >();
                            let mut iSavepoint: ::core::ffi::c_int = 0;
                            let mut ii: ::core::ffi::c_int = 0;
                            p1_1 = (*pOp).p1;
                            zName = (*pOp).p4.z;
                            if p1_1 == SAVEPOINT_BEGIN {
                                if (*db).nVdbeWrite > 0 as ::core::ffi::c_int {
                                    sqlite3VdbeError(
                                        p,
                                        b"cannot open savepoint - SQL statements in progress\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    rc = SQLITE_BUSY;
                                } else {
                                    nName = sqlite3Strlen30(zName);
                                    rc = sqlite3VtabSavepoint(
                                        db,
                                        SAVEPOINT_BEGIN,
                                        (*db).nStatement + (*db).nSavepoint,
                                    );
                                    if rc != SQLITE_OK {
                                        c2rust_current_block = 8086965459351668542;
                                        break;
                                    }
                                    pNew = sqlite3DbMallocRawNN(
                                        db,
                                        (::core::mem::size_of::<Savepoint>() as usize)
                                            .wrapping_add(nName as usize)
                                            .wrapping_add(1 as usize) as u64_0,
                                    ) as *mut Savepoint;
                                    if !pNew.is_null() {
                                        (*pNew).zName = pNew
                                            .offset(1 as ::core::ffi::c_int as isize) as *mut Savepoint
                                            as *mut ::core::ffi::c_char;
                                        memcpy(
                                            (*pNew).zName as *mut ::core::ffi::c_void,
                                            zName as *const ::core::ffi::c_void,
                                            (nName + 1 as ::core::ffi::c_int) as size_t,
                                        );
                                        if (*db).autoCommit != 0 {
                                            (*db).autoCommit = 0 as u8_0;
                                            (*db).isTransactionSavepoint = 1 as u8_0;
                                        } else {
                                            (*db).nSavepoint += 1;
                                        }
                                        (*pNew).pNext = (*db).pSavepoint;
                                        (*db).pSavepoint = pNew;
                                        (*pNew).nDeferredCons = (*db).nDeferredCons;
                                        (*pNew).nDeferredImmCons = (*db).nDeferredImmCons;
                                    }
                                }
                            } else {
                                iSavepoint = 0 as ::core::ffi::c_int;
                                pSavepoint = (*db).pSavepoint;
                                while !pSavepoint.is_null()
                                    && sqlite3StrICmp((*pSavepoint).zName, zName) != 0
                                {
                                    iSavepoint += 1;
                                    pSavepoint = (*pSavepoint).pNext;
                                }
                                if pSavepoint.is_null() {
                                    sqlite3VdbeError(
                                        p,
                                        b"no such savepoint: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        zName,
                                    );
                                    rc = SQLITE_ERROR;
                                } else if (*db).nVdbeWrite > 0 as ::core::ffi::c_int
                                    && p1_1 == SAVEPOINT_RELEASE
                                {
                                    sqlite3VdbeError(
                                        p,
                                        b"cannot release savepoint - SQL statements in progress\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    rc = SQLITE_BUSY;
                                } else {
                                    let mut isTransaction: ::core::ffi::c_int = ((*pSavepoint)
                                        .pNext
                                        .is_null()
                                        && (*db).isTransactionSavepoint as ::core::ffi::c_int != 0)
                                        as ::core::ffi::c_int;
                                    if isTransaction != 0 && p1_1 == SAVEPOINT_RELEASE {
                                        rc = sqlite3VdbeCheckFkDeferred(p);
                                        if rc != SQLITE_OK {
                                            c2rust_current_block = 4288082686256521985;
                                            break;
                                        }
                                        (*db).autoCommit = 1 as u8_0;
                                        if sqlite3VdbeHalt(p) == SQLITE_BUSY {
                                            (*p).pc = pOp.offset_from(aOp) as ::core::ffi::c_long
                                                as ::core::ffi::c_int;
                                            (*db).autoCommit = 0 as u8_0;
                                            rc = SQLITE_BUSY;
                                            (*p).rc = rc;
                                            c2rust_current_block = 4288082686256521985;
                                            break;
                                        } else {
                                            rc = (*p).rc;
                                            if rc != 0 {
                                                (*db).autoCommit = 0 as u8_0;
                                            } else {
                                                (*db).isTransactionSavepoint = 0 as u8_0;
                                            }
                                        }
                                    } else {
                                        let mut isSchemaChange: ::core::ffi::c_int = 0;
                                        iSavepoint = (*db).nSavepoint - iSavepoint
                                            - 1 as ::core::ffi::c_int;
                                        if p1_1 == SAVEPOINT_ROLLBACK {
                                            isSchemaChange = ((*db).mDbFlags as ::core::ffi::c_uint
                                                & DBFLAG_SchemaChange as ::core::ffi::c_uint
                                                != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                                            ii = 0 as ::core::ffi::c_int;
                                            while ii < (*db).nDb {
                                                rc = sqlite3BtreeTripAllCursors(
                                                    (*(*db).aDb.offset(ii as isize)).pBt,
                                                    SQLITE_ABORT_ROLLBACK,
                                                    (isSchemaChange == 0 as ::core::ffi::c_int)
                                                        as ::core::ffi::c_int,
                                                );
                                                if rc != SQLITE_OK {
                                                    c2rust_current_block = 8086965459351668542;
                                                    break 's_109;
                                                }
                                                ii += 1;
                                            }
                                        } else {
                                            isSchemaChange = 0 as ::core::ffi::c_int;
                                        }
                                        ii = 0 as ::core::ffi::c_int;
                                        while ii < (*db).nDb {
                                            rc = sqlite3BtreeSavepoint(
                                                (*(*db).aDb.offset(ii as isize)).pBt,
                                                p1_1,
                                                iSavepoint,
                                            );
                                            if rc != SQLITE_OK {
                                                c2rust_current_block = 8086965459351668542;
                                                break 's_109;
                                            }
                                            ii += 1;
                                        }
                                        if isSchemaChange != 0 {
                                            sqlite3ExpirePreparedStatements(
                                                db,
                                                0 as ::core::ffi::c_int,
                                            );
                                            sqlite3ResetAllSchemasOfConnection(db);
                                            (*db).mDbFlags
                                                |= DBFLAG_SchemaChange as ::core::ffi::c_uint;
                                        }
                                    }
                                    if rc != 0 {
                                        c2rust_current_block = 8086965459351668542;
                                        break;
                                    }
                                    while (*db).pSavepoint != pSavepoint {
                                        pTmp = (*db).pSavepoint;
                                        (*db).pSavepoint = (*pTmp).pNext;
                                        sqlite3DbFree(db, pTmp as *mut ::core::ffi::c_void);
                                        (*db).nSavepoint -= 1;
                                    }
                                    if p1_1 == SAVEPOINT_RELEASE {
                                        (*db).pSavepoint = (*pSavepoint).pNext;
                                        sqlite3DbFree(db, pSavepoint as *mut ::core::ffi::c_void);
                                        if isTransaction == 0 {
                                            (*db).nSavepoint -= 1;
                                        }
                                    } else {
                                        (*db).nDeferredCons = (*pSavepoint).nDeferredCons;
                                        (*db).nDeferredImmCons = (*pSavepoint).nDeferredImmCons;
                                    }
                                    if isTransaction == 0 || p1_1 == SAVEPOINT_ROLLBACK {
                                        rc = sqlite3VtabSavepoint(db, p1_1, iSavepoint);
                                        if rc != SQLITE_OK {
                                            c2rust_current_block = 8086965459351668542;
                                            break;
                                        }
                                    }
                                }
                            }
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            if (*p).eVdbeState as ::core::ffi::c_int == VDBE_HALT_STATE {
                                rc = SQLITE_DONE;
                                c2rust_current_block = 4288082686256521985;
                                break;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_AutoCommit => {
                            let mut desiredAutoCommit: ::core::ffi::c_int = 0;
                            let mut iRollback: ::core::ffi::c_int = 0;
                            desiredAutoCommit = (*pOp).p1;
                            iRollback = (*pOp).p2;
                            if desiredAutoCommit
                                != (*db).autoCommit as ::core::ffi::c_int
                            {
                                if iRollback != 0 {
                                    sqlite3RollbackAll(db, SQLITE_ABORT_ROLLBACK);
                                    (*db).autoCommit = 1 as u8_0;
                                } else if desiredAutoCommit != 0
                                    && (*db).nVdbeWrite > 0 as ::core::ffi::c_int
                                {
                                    sqlite3VdbeError(
                                        p,
                                        b"cannot commit transaction - SQL statements in progress\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    rc = SQLITE_BUSY;
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                } else {
                                    rc = sqlite3VdbeCheckFkDeferred(p);
                                    if rc != SQLITE_OK {
                                        c2rust_current_block = 4288082686256521985;
                                        break;
                                    }
                                    (*db).autoCommit = desiredAutoCommit as u8_0;
                                }
                                if sqlite3VdbeHalt(p) == SQLITE_BUSY {
                                    (*p).pc = pOp.offset_from(aOp) as ::core::ffi::c_long
                                        as ::core::ffi::c_int;
                                    (*db).autoCommit = (1 as ::core::ffi::c_int
                                        - desiredAutoCommit) as u8_0;
                                    rc = SQLITE_BUSY;
                                    (*p).rc = rc;
                                    c2rust_current_block = 4288082686256521985;
                                    break;
                                } else {
                                    sqlite3CloseSavepoints(db);
                                    if (*p).rc == SQLITE_OK {
                                        rc = SQLITE_DONE;
                                    } else {
                                        rc = SQLITE_ERROR;
                                    }
                                    c2rust_current_block = 4288082686256521985;
                                    break;
                                }
                            } else {
                                sqlite3VdbeError(
                                    p,
                                    if desiredAutoCommit == 0 {
                                        b"cannot start a transaction within a transaction\0"
                                            .as_ptr() as *const ::core::ffi::c_char
                                    } else if iRollback != 0 {
                                        b"cannot rollback - no transaction is active\0".as_ptr()
                                            as *const ::core::ffi::c_char
                                    } else {
                                        b"cannot commit - no transaction is active\0".as_ptr()
                                            as *const ::core::ffi::c_char
                                    },
                                );
                                rc = SQLITE_ERROR;
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                        }
                        OP_Transaction => {
                            let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
                            let mut pDb: *mut Db = ::core::ptr::null_mut::<Db>();
                            let mut iMeta: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            if (*pOp).p2 != 0
                                && (*db).flags
                                    & (SQLITE_QueryOnly as u64_0 | SQLITE_CorruptRdOnly)
                                    != 0 as ::core::ffi::c_ulonglong
                            {
                                if (*db).flags as ::core::ffi::c_ulonglong
                                    & SQLITE_QueryOnly as ::core::ffi::c_ulonglong != 0
                                {
                                    rc = SQLITE_READONLY;
                                } else {
                                    rc = SQLITE_CORRUPT;
                                }
                                c2rust_current_block = 8086965459351668542;
                                break;
                            } else {
                                pDb = (*db).aDb.offset((*pOp).p1 as isize) as *mut Db;
                                pBt = (*pDb).pBt;
                                if !pBt.is_null() {
                                    rc = sqlite3BtreeBeginTrans(pBt, (*pOp).p2, &raw mut iMeta);
                                    if rc != SQLITE_OK {
                                        if !(rc & 0xff as ::core::ffi::c_int == SQLITE_BUSY) {
                                            c2rust_current_block = 8086965459351668542;
                                            break;
                                        }
                                        (*p).pc = pOp.offset_from(aOp) as ::core::ffi::c_long
                                            as ::core::ffi::c_int;
                                        (*p).rc = rc;
                                        c2rust_current_block = 4288082686256521985;
                                        break;
                                    } else if (*p).usesStmtJournal() as ::core::ffi::c_int != 0
                                        && (*pOp).p2 != 0
                                        && ((*db).autoCommit as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                            || (*db).nVdbeRead > 1 as ::core::ffi::c_int)
                                    {
                                        if (*p).iStatement == 0 as ::core::ffi::c_int {
                                            (*db).nStatement += 1;
                                            (*p).iStatement = (*db).nSavepoint + (*db).nStatement;
                                        }
                                        rc = sqlite3VtabSavepoint(
                                            db,
                                            SAVEPOINT_BEGIN,
                                            (*p).iStatement - 1 as ::core::ffi::c_int,
                                        );
                                        if rc == SQLITE_OK {
                                            rc = sqlite3BtreeBeginStmt(pBt, (*p).iStatement);
                                        }
                                        (*p).nStmtDefCons = (*db).nDeferredCons;
                                        (*p).nStmtDefImmCons = (*db).nDeferredImmCons;
                                    }
                                }
                                if rc == SQLITE_OK && (*pOp).p5 as ::core::ffi::c_int != 0
                                    && (iMeta != (*pOp).p3
                                        || (*(*pDb).pSchema).iGeneration != (*pOp).p4.i)
                                {
                                    sqlite3DbFree(db, (*p).zErrMsg as *mut ::core::ffi::c_void);
                                    (*p).zErrMsg = sqlite3DbStrDup(
                                        db,
                                        b"database schema has changed\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                    if (*(*(*db).aDb.offset((*pOp).p1 as isize)).pSchema)
                                        .schema_cookie != iMeta
                                    {
                                        sqlite3ResetOneSchema(db, (*pOp).p1);
                                    }
                                    (*p).set_expired(1 as bft as bft);
                                    rc = SQLITE_SCHEMA;
                                    (*p).set_changeCntOn(0 as bft as bft);
                                }
                                if rc != 0 {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_ReadCookie => {
                            let mut iMeta_0: ::core::ffi::c_int = 0;
                            let mut iDb: ::core::ffi::c_int = 0;
                            let mut iCookie: ::core::ffi::c_int = 0;
                            iDb = (*pOp).p1;
                            iCookie = (*pOp).p3;
                            sqlite3BtreeGetMeta(
                                (*(*db).aDb.offset(iDb as isize)).pBt,
                                iCookie,
                                &raw mut iMeta_0 as *mut u32_0,
                            );
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            (*pOut).u.i = iMeta_0 as i64_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_SetCookie => {
                            let mut pDb_0: *mut Db = ::core::ptr::null_mut::<Db>();
                            pDb_0 = (*db).aDb.offset((*pOp).p1 as isize) as *mut Db;
                            rc = sqlite3BtreeUpdateMeta(
                                (*pDb_0).pBt,
                                (*pOp).p2,
                                (*pOp).p3 as u32_0,
                            );
                            if (*pOp).p2 == BTREE_SCHEMA_VERSION {
                                *(&raw mut (*(*pDb_0).pSchema).schema_cookie
                                    as *mut u32_0) = (*(&raw mut (*pOp).p3 as *mut u32_0))
                                    .wrapping_sub((*pOp).p5 as ::core::ffi::c_uint) as u32_0;
                                (*db).mDbFlags
                                    |= DBFLAG_SchemaChange as ::core::ffi::c_uint;
                                sqlite3FkClearTriggerCache(db, (*pOp).p1);
                            } else if (*pOp).p2 == BTREE_FILE_FORMAT {
                                (*(*pDb_0).pSchema).file_format = (*pOp).p3 as u8_0;
                            }
                            if (*pOp).p1 == 1 as ::core::ffi::c_int {
                                sqlite3ExpirePreparedStatements(
                                    db,
                                    0 as ::core::ffi::c_int,
                                );
                                (*p).set_expired(0 as bft as bft);
                            }
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_ReopenIdx => {
                            nField_0 = 0;
                            pKeyInfo_0 = ::core::ptr::null_mut::<KeyInfo>();
                            p2_2 = 0;
                            iDb_0 = 0;
                            wrFlag = 0;
                            pX = ::core::ptr::null_mut::<Btree>();
                            pCur = ::core::ptr::null_mut::<VdbeCursor>();
                            pDb_1 = ::core::ptr::null_mut::<Db>();
                            pCur = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if !pCur.is_null() && (*pCur).pgnoRoot == (*pOp).p2 as u32_0
                            {
                                sqlite3BtreeClearCursor((*pCur).uc.pCursor);
                                c2rust_current_block = 7045491780073535011;
                            } else {
                                c2rust_current_block = 16007193389016785609;
                            }
                        }
                        OP_OpenRead | OP_OpenWrite => {
                            c2rust_current_block = 16007193389016785609;
                        }
                        OP_OpenDup => {
                            let mut pOrig: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pCx: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pOrig = *(*p).apCsr.offset((*pOp).p2 as isize);
                            pCx = allocateCursor(
                                p,
                                (*pOp).p1,
                                (*pOrig).nField as ::core::ffi::c_int,
                                CURTYPE_BTREE as u8_0,
                            );
                            if pCx.is_null() {
                                c2rust_current_block = 9955115899106739073;
                                break;
                            }
                            (*pCx).nullRow = 1 as u8_0;
                            (*pCx).set_isEphemeral(1 as Bool as Bool);
                            (*pCx).pKeyInfo = (*pOrig).pKeyInfo;
                            (*pCx).isTable = (*pOrig).isTable;
                            (*pCx).pgnoRoot = (*pOrig).pgnoRoot;
                            (*pCx).set_isOrdered((*pOrig).isOrdered() as Bool);
                            (*pCx).ub.pBtx = (*pOrig).ub.pBtx;
                            (*pCx).set_noReuse(1 as Bool as Bool);
                            (*pOrig).set_noReuse(1 as Bool as Bool);
                            rc = sqlite3BtreeCursor(
                                (*pCx).ub.pBtx,
                                (*pCx).pgnoRoot,
                                BTREE_WRCSR,
                                (*pCx).pKeyInfo as *mut KeyInfo,
                                (*pCx).uc.pCursor,
                            );
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_OpenAutoindex => {
                            c2rust_current_block = 12138815981076939541;
                        }
                        OP_OpenEphemeral => {
                            c2rust_current_block = 12138815981076939541;
                        }
                        OP_SorterOpen => {
                            let mut pCx_1: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pCx_1 = allocateCursor(
                                p,
                                (*pOp).p1,
                                (*pOp).p2,
                                CURTYPE_SORTER as u8_0,
                            );
                            if pCx_1.is_null() {
                                c2rust_current_block = 9955115899106739073;
                                break;
                            }
                            (*pCx_1).pKeyInfo = (*pOp).p4.pKeyInfo;
                            rc = sqlite3VdbeSorterInit(db, (*pOp).p3, pCx_1);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_SequenceTest => {
                            let mut pC_2: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pC_2 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            let c2rust_fresh10 = (*pC_2).seqCount;
                            (*pC_2).seqCount = (*pC_2).seqCount + 1;
                            if c2rust_fresh10 == 0 as ::core::ffi::c_longlong {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_OpenPseudo => {
                            let mut pCx_2: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pCx_2 = allocateCursor(
                                p,
                                (*pOp).p1,
                                (*pOp).p3,
                                CURTYPE_PSEUDO as u8_0,
                            );
                            if pCx_2.is_null() {
                                c2rust_current_block = 9955115899106739073;
                                break;
                            }
                            (*pCx_2).nullRow = 1 as u8_0;
                            (*pCx_2).seekResult = (*pOp).p2;
                            (*pCx_2).isTable = 1 as u8_0;
                            (*pCx_2).uc.pCursor = sqlite3BtreeFakeValidCursor();
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Close => {
                            sqlite3VdbeFreeCursor(
                                p,
                                *(*p).apCsr.offset((*pOp).p1 as isize),
                            );
                            let ref mut c2rust_fresh11 = *(*p)
                                .apCsr
                                .offset((*pOp).p1 as isize);
                            *c2rust_fresh11 = ::core::ptr::null_mut::<VdbeCursor>();
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_SeekLT => {
                            c2rust_current_block = 11769942314722711362;
                        }
                        OP_SeekLE => {
                            c2rust_current_block = 11769942314722711362;
                        }
                        OP_SeekGE => {
                            c2rust_current_block = 4606120822943820531;
                        }
                        OP_SeekGT => {
                            c2rust_current_block = 3833908538611729861;
                        }
                        OP_SeekScan => {
                            let mut pC_4: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut res_1: ::core::ffi::c_int = 0;
                            let mut nStep: ::core::ffi::c_int = 0;
                            let mut r_0: UnpackedRecord = UnpackedRecord {
                                pKeyInfo: ::core::ptr::null_mut::<KeyInfo>(),
                                aMem: ::core::ptr::null_mut::<Mem>(),
                                u: C2Rust_Unnamed_24 {
                                    z: ::core::ptr::null_mut::<::core::ffi::c_char>()

                                },
                                n: 0,
                                nField: 0,
                                default_rc: 0,
                                errCode: 0,
                                r1: 0,
                                r2: 0,
                                eqSeen: 0

                            };
                            pC_4 = *(*p)
                                .apCsr
                                .offset(
                                    (*pOp.offset(1 as ::core::ffi::c_int as isize)).p1 as isize,
                                );
                            if sqlite3BtreeCursorIsValidNN((*pC_4).uc.pCursor) == 0 {
                                c2rust_current_block = 5783071609795492627;
                            } else {
                                nStep = (*pOp).p1;
                                r_0.pKeyInfo = (*pC_4).pKeyInfo;
                                r_0.nField = (*pOp.offset(1 as ::core::ffi::c_int as isize))
                                    .p4
                                    .i as u16_0;
                                r_0.default_rc = 0 as i8_0;
                                r_0.aMem = aMem
                                    .offset(
                                        (*pOp.offset(1 as ::core::ffi::c_int as isize)).p3 as isize,
                                    ) as *mut Mem;
                                res_1 = 0 as ::core::ffi::c_int;
                                loop {
                                    rc = sqlite3VdbeIdxKeyCompare(
                                        db,
                                        pC_4,
                                        &raw mut r_0,
                                        &raw mut res_1,
                                    );
                                    if rc != 0 {
                                        c2rust_current_block = 8086965459351668542;
                                        break 's_109;
                                    }
                                    if !(res_1 > 0 as ::core::ffi::c_int
                                        && (*pOp).p5 as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int)
                                    {
                                        if res_1 >= 0 as ::core::ffi::c_int {
                                            c2rust_current_block = 17233182392562552756;
                                            break;
                                        } else {
                                            if nStep <= 0 as ::core::ffi::c_int {
                                                c2rust_current_block = 5783071609795492627;
                                                break;
                                            }
                                            nStep -= 1;
                                            (*pC_4).cacheStatus = CACHE_STALE as u32_0;
                                            rc = sqlite3BtreeNext(
                                                (*pC_4).uc.pCursor,
                                                0 as ::core::ffi::c_int,
                                            );
                                            if !(rc != 0) {
                                                continue;
                                            }
                                            if !(rc == SQLITE_DONE) {
                                                c2rust_current_block = 8086965459351668542;
                                                break 's_109;
                                            }
                                            rc = SQLITE_OK;
                                        }
                                    }
                                    pOp = pOp.offset(1);
                                    c2rust_current_block = 17233182392562552756;
                                    break;
                                }
                            }
                        }
                        OP_SeekHit => {
                            let mut pC_5: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pC_5 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if ((*pC_5).seekHit as ::core::ffi::c_int) < (*pOp).p2 {
                                (*pC_5).seekHit = (*pOp).p2 as u16_0;
                            } else if (*pC_5).seekHit as ::core::ffi::c_int > (*pOp).p3 {
                                (*pC_5).seekHit = (*pOp).p3 as u16_0;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_IfNotOpen => {
                            let mut pCur_0: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pCur_0 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if pCur_0.is_null()
                                || (*pCur_0).nullRow as ::core::ffi::c_int != 0
                            {
                                c2rust_current_block = 13898972957494642055;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_IfNoHope => {
                            let mut pC_6: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pC_6 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if (*pC_6).seekHit as ::core::ffi::c_int >= (*pOp).p4.i {
                                c2rust_current_block = 5783071609795492627;
                            } else {
                                c2rust_current_block = 10118086935549511662;
                            }
                        }
                        OP_NoConflict => {
                            c2rust_current_block = 10118086935549511662;
                        }
                        OP_NotFound => {
                            c2rust_current_block = 12239147378838813631;
                        }
                        OP_Found => {
                            c2rust_current_block = 682219075816228721;
                        }
                        OP_SeekRowid => {
                            pC_8 = ::core::ptr::null_mut::<VdbeCursor>();
                            pCrsr_1 = ::core::ptr::null_mut::<BtCursor>();
                            res_2 = 0;
                            iKey_0 = 0;
                            pIn3 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            if (*pIn3).flags as ::core::ffi::c_int
                                & (MEM_Int | MEM_IntReal) == 0 as ::core::ffi::c_int
                            {
                                let mut x: Mem = *pIn3
                                    .offset(0 as ::core::ffi::c_int as isize);
                                applyAffinity(
                                    &raw mut x,
                                    SQLITE_AFF_NUMERIC as ::core::ffi::c_char,
                                    encoding,
                                );
                                if x.flags as ::core::ffi::c_int & MEM_Int
                                    == 0 as ::core::ffi::c_int
                                {
                                    c2rust_current_block = 17233182392562552756;
                                } else {
                                    iKey_0 = x.u.i as u64_0;
                                    c2rust_current_block = 11230649915035842278;
                                }
                            } else {
                                c2rust_current_block = 14309557416411021540;
                            }
                        }
                        OP_NotExists => {
                            c2rust_current_block = 14309557416411021540;
                        }
                        OP_Sequence => {
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            let ref mut c2rust_fresh12 = (**(*p)
                                .apCsr
                                .offset((*pOp).p1 as isize))
                                .seqCount;
                            let c2rust_fresh13 = *c2rust_fresh12;
                            *c2rust_fresh12 = *c2rust_fresh12 + 1;
                            (*pOut).u.i = c2rust_fresh13;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_NewRowid => {
                            let mut v_0: i64_0 = 0;
                            let mut pC_9: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut res_3: ::core::ffi::c_int = 0;
                            let mut cnt_0: ::core::ffi::c_int = 0;
                            let mut pMem: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut pFrame_0: *mut VdbeFrame = ::core::ptr::null_mut::<
                                VdbeFrame,
                            >();
                            v_0 = 0 as i64_0;
                            res_3 = 0 as ::core::ffi::c_int;
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            pC_9 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if (*pC_9).useRandomRowid() == 0 {
                                rc = sqlite3BtreeLast((*pC_9).uc.pCursor, &raw mut res_3);
                                if rc != SQLITE_OK {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                if res_3 != 0 {
                                    v_0 = 1 as i64_0;
                                } else {
                                    v_0 = sqlite3BtreeIntegerKey((*pC_9).uc.pCursor);
                                    if v_0 >= MAX_ROWID {
                                        (*pC_9).set_useRandomRowid(1 as Bool as Bool);
                                    } else {
                                        v_0 += 1;
                                    }
                                }
                            }
                            if (*pOp).p3 != 0 {
                                if !(*p).pFrame.is_null() {
                                    pFrame_0 = (*p).pFrame;
                                    while !(*pFrame_0).pParent.is_null() {
                                        pFrame_0 = (*pFrame_0).pParent;
                                    }
                                    pMem = (*pFrame_0).aMem.offset((*pOp).p3 as isize)
                                        as *mut Mem;
                                } else {
                                    pMem = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                                }
                                sqlite3VdbeMemIntegerify(pMem);
                                if (*pMem).u.i == MAX_ROWID
                                    || (*pC_9).useRandomRowid() as ::core::ffi::c_int != 0
                                {
                                    rc = SQLITE_FULL;
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                } else {
                                    if v_0
                                        < (*pMem).u.i as ::core::ffi::c_longlong
                                            + 1 as ::core::ffi::c_longlong
                                    {
                                        v_0 = ((*pMem).u.i as ::core::ffi::c_longlong
                                            + 1 as ::core::ffi::c_longlong) as i64_0;
                                    }
                                    (*pMem).u.i = v_0;
                                }
                            }
                            if (*pC_9).useRandomRowid() != 0 {
                                cnt_0 = 0 as ::core::ffi::c_int;
                                loop {
                                    sqlite3_randomness(
                                        ::core::mem::size_of::<i64_0>() as ::core::ffi::c_int,
                                        &raw mut v_0 as *mut ::core::ffi::c_void,
                                    );
                                    v_0
                                        &= (MAX_ROWID >> 1 as ::core::ffi::c_int)
                                            as ::core::ffi::c_longlong;
                                    v_0 += 1;
                                    rc = sqlite3BtreeTableMoveto(
                                        (*pC_9).uc.pCursor,
                                        v_0 as u64_0 as i64_0,
                                        0 as ::core::ffi::c_int,
                                        &raw mut res_3,
                                    );
                                    if !(rc == SQLITE_OK && res_3 == 0 as ::core::ffi::c_int
                                        && {
                                            cnt_0 += 1;
                                            cnt_0 < 100 as ::core::ffi::c_int
                                        })
                                    {
                                        break;
                                    }
                                }
                                if rc != 0 {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                if res_3 == 0 as ::core::ffi::c_int {
                                    rc = SQLITE_FULL;
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                            }
                            (*pC_9).deferredMoveto = 0 as u8_0;
                            (*pC_9).cacheStatus = CACHE_STALE as u32_0;
                            (*pOut).u.i = v_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Insert => {
                            let mut pData: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut pKey: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut pC_10: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut seekResult: ::core::ffi::c_int = 0;
                            let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
                                ::core::ffi::c_char,
                            >();
                            let mut pTab_0: *mut Table = ::core::ptr::null_mut::<
                                Table,
                            >();
                            let mut x_0: BtreePayload = BtreePayload {
                                pKey: ::core::ptr::null::<::core::ffi::c_void>(),
                                nKey: 0,
                                pData: ::core::ptr::null::<::core::ffi::c_void>(),
                                aMem: ::core::ptr::null_mut::<sqlite3_value>(),
                                nMem: 0,
                                nData: 0,
                                nZero: 0

                            };
                            pData = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            pC_10 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pKey = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            x_0.nKey = (*pKey).u.i as sqlite3_int64;
                            if (*pOp).p4type as ::core::ffi::c_int == P4_TABLE
                                && (*db).xUpdateCallback.is_some()
                            {
                                zDb = (*(*db).aDb.offset((*pC_10).iDb as isize)).zDbSName;
                                pTab_0 = (*pOp).p4.pTab;
                            } else {
                                pTab_0 = ::core::ptr::null_mut::<Table>();
                                zDb = ::core::ptr::null::<::core::ffi::c_char>();
                            }
                            if (*pOp).p5 as ::core::ffi::c_int & OPFLAG_NCHANGE != 0 {
                                (*p).nChange += 1;
                                if (*pOp).p5 as ::core::ffi::c_int & OPFLAG_LASTROWID != 0 {
                                    (*db).lastRowid = x_0.nKey as i64_0;
                                }
                            }
                            x_0.pData = (*pData).z as *const ::core::ffi::c_void;
                            x_0.nData = (*pData).n;
                            seekResult = if (*pOp).p5 as ::core::ffi::c_int
                                & OPFLAG_USESEEKRESULT != 0
                            {
                                (*pC_10).seekResult
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            if (*pData).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                                x_0.nZero = (*pData).u.nZero;
                            } else {
                                x_0.nZero = 0 as ::core::ffi::c_int;
                            }
                            x_0.pKey = ::core::ptr::null::<::core::ffi::c_void>();
                            rc = sqlite3BtreeInsert(
                                (*pC_10).uc.pCursor,
                                &raw mut x_0,
                                (*pOp).p5 as ::core::ffi::c_int
                                    & (OPFLAG_APPEND | OPFLAG_SAVEPOSITION | OPFLAG_PREFORMAT),
                                seekResult,
                            );
                            (*pC_10).deferredMoveto = 0 as u8_0;
                            (*pC_10).cacheStatus = CACHE_STALE as u32_0;
                            colCacheCtr = colCacheCtr.wrapping_add(1);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            if !pTab_0.is_null() {
                                (*db)
                                    .xUpdateCallback
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*db).pUpdateArg,
                                    if (*pOp).p5 as ::core::ffi::c_int & OPFLAG_ISUPDATE != 0 {
                                        SQLITE_UPDATE
                                    } else {
                                        SQLITE_INSERT
                                    },
                                    zDb,
                                    (*pTab_0).zName,
                                    x_0.nKey as sqlite_int64,
                                );
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_RowCell => {
                            let mut pDest_0: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pSrc: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut iKey_1: i64_0 = 0;
                            pDest_0 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pSrc = *(*p).apCsr.offset((*pOp).p2 as isize);
                            iKey_1 = (if (*pOp).p3 != 0 {
                                (*aMem.offset((*pOp).p3 as isize)).u.i
                                    as ::core::ffi::c_longlong
                            } else {
                                0 as ::core::ffi::c_longlong
                            }) as i64_0;
                            rc = sqlite3BtreeTransferRow(
                                (*pDest_0).uc.pCursor,
                                (*pSrc).uc.pCursor,
                                iKey_1,
                            );
                            if rc != SQLITE_OK {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Delete => {
                            let mut pC_11: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut zDb_0: *const ::core::ffi::c_char = ::core::ptr::null::<
                                ::core::ffi::c_char,
                            >();
                            let mut pTab_1: *mut Table = ::core::ptr::null_mut::<
                                Table,
                            >();
                            let mut opflags: ::core::ffi::c_int = 0;
                            opflags = (*pOp).p2;
                            pC_11 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if (*pOp).p4type as ::core::ffi::c_int == P4_TABLE
                                && (*db).xUpdateCallback.is_some()
                            {
                                zDb_0 = (*(*db).aDb.offset((*pC_11).iDb as isize)).zDbSName;
                                pTab_1 = (*pOp).p4.pTab;
                                if (*pOp).p5 as ::core::ffi::c_int & OPFLAG_SAVEPOSITION
                                    != 0 as ::core::ffi::c_int
                                    && (*pC_11).isTable as ::core::ffi::c_int != 0
                                {
                                    (*pC_11).movetoTarget = sqlite3BtreeIntegerKey(
                                        (*pC_11).uc.pCursor,
                                    );
                                }
                            } else {
                                zDb_0 = ::core::ptr::null::<::core::ffi::c_char>();
                                pTab_1 = ::core::ptr::null_mut::<Table>();
                            }
                            rc = sqlite3BtreeDelete(
                                (*pC_11).uc.pCursor,
                                (*pOp).p5 as u8_0,
                            );
                            (*pC_11).cacheStatus = CACHE_STALE as u32_0;
                            colCacheCtr = colCacheCtr.wrapping_add(1);
                            (*pC_11).seekResult = 0 as ::core::ffi::c_int;
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            if opflags & OPFLAG_NCHANGE != 0 {
                                (*p).nChange += 1;
                                if (*db).xUpdateCallback.is_some() && !pTab_1.is_null()
                                    && (*pTab_1).tabFlags as ::core::ffi::c_uint
                                        & TF_WithoutRowid as ::core::ffi::c_uint
                                        == 0 as ::core::ffi::c_uint
                                {
                                    (*db)
                                        .xUpdateCallback
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        (*db).pUpdateArg,
                                        SQLITE_DELETE,
                                        zDb_0,
                                        (*pTab_1).zName,
                                        (*pC_11).movetoTarget as sqlite_int64,
                                    );
                                }
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_ResetCount => {
                            sqlite3VdbeSetChanges(db, (*p).nChange);
                            (*p).nChange = 0 as i64_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_SorterCompare => {
                            let mut pC_12: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut res_4: ::core::ffi::c_int = 0;
                            let mut nKeyCol: ::core::ffi::c_int = 0;
                            pC_12 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pIn3 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            nKeyCol = (*pOp).p4.i;
                            res_4 = 0 as ::core::ffi::c_int;
                            rc = sqlite3VdbeSorterCompare(
                                pC_12,
                                pIn3,
                                nKeyCol,
                                &raw mut res_4,
                            );
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            if res_4 != 0 {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_SorterData => {
                            let mut pC_13: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pOut = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            pC_13 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            rc = sqlite3VdbeSorterRowkey(pC_13, pOut);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            (**(*p).apCsr.offset((*pOp).p3 as isize)).cacheStatus = CACHE_STALE
                                as u32_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_RowData => {
                            let mut pC_14: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pCrsr_2: *mut BtCursor = ::core::ptr::null_mut::<
                                BtCursor,
                            >();
                            let mut n_2: u32_0 = 0;
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            pC_14 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pCrsr_2 = (*pC_14).uc.pCursor;
                            n_2 = sqlite3BtreePayloadSize(pCrsr_2);
                            if n_2 > (*db).aLimit[SQLITE_LIMIT_LENGTH as usize] as u32_0
                            {
                                c2rust_current_block = 13536115470629238340;
                                break;
                            }
                            rc = sqlite3VdbeMemFromBtreeZeroOffset(pCrsr_2, n_2, pOut);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            if (*pOp).p3 == 0 {
                                if (*pOut).flags as ::core::ffi::c_int & MEM_Ephem
                                    != 0 as ::core::ffi::c_int
                                    && sqlite3VdbeMemMakeWriteable(pOut) != 0
                                {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                }
                                c2rust_current_block = 5783071609795492627;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_Rowid => {
                            let mut pC_15: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut v_1: i64_0 = 0;
                            let mut pVtab: *mut sqlite3_vtab = ::core::ptr::null_mut::<
                                sqlite3_vtab,
                            >();
                            let mut pModule: *const sqlite3_module = ::core::ptr::null::<
                                sqlite3_module,
                            >();
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            pC_15 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if (*pC_15).nullRow != 0 {
                                (*pOut).flags = MEM_Null as u16_0;
                            } else {
                                if (*pC_15).deferredMoveto != 0 {
                                    v_1 = (*pC_15).movetoTarget;
                                    c2rust_current_block = 12306302469606455720;
                                } else if (*pC_15).eCurType as ::core::ffi::c_int
                                    == CURTYPE_VTAB
                                {
                                    pVtab = (*(*pC_15).uc.pVCur).pVtab;
                                    pModule = (*pVtab).pModule;
                                    rc = (*pModule)
                                        .xRowid
                                        .expect(
                                            "non-null function pointer",
                                        )((*pC_15).uc.pVCur, &raw mut v_1);
                                    sqlite3VtabImportErrmsg(p, pVtab);
                                    if rc != 0 {
                                        c2rust_current_block = 8086965459351668542;
                                        break;
                                    }
                                    c2rust_current_block = 12306302469606455720;
                                } else {
                                    rc = sqlite3VdbeCursorRestore(pC_15);
                                    if rc != 0 {
                                        c2rust_current_block = 8086965459351668542;
                                        break;
                                    }
                                    if (*pC_15).nullRow != 0 {
                                        (*pOut).flags = MEM_Null as u16_0;
                                        c2rust_current_block = 5783071609795492627;
                                    } else {
                                        v_1 = sqlite3BtreeIntegerKey((*pC_15).uc.pCursor);
                                        c2rust_current_block = 12306302469606455720;
                                    }
                                }
                                match c2rust_current_block {
                                    5783071609795492627 => {}
                                    _ => {
                                        (*pOut).u.i = v_1;
                                    }
                                }
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_NullRow => {
                            let mut pC_16: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pC_16 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if pC_16.is_null() {
                                pC_16 = allocateCursor(
                                    p,
                                    (*pOp).p1,
                                    1 as ::core::ffi::c_int,
                                    CURTYPE_PSEUDO as u8_0,
                                );
                                if pC_16.is_null() {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                }
                                (*pC_16).seekResult = 0 as ::core::ffi::c_int;
                                (*pC_16).isTable = 1 as u8_0;
                                (*pC_16).set_noReuse(1 as Bool as Bool);
                                (*pC_16).uc.pCursor = sqlite3BtreeFakeValidCursor();
                            }
                            (*pC_16).nullRow = 1 as u8_0;
                            (*pC_16).cacheStatus = CACHE_STALE as u32_0;
                            if (*pC_16).eCurType as ::core::ffi::c_int == CURTYPE_BTREE {
                                sqlite3BtreeClearCursor((*pC_16).uc.pCursor);
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_SeekEnd => {
                            c2rust_current_block = 12752338440521374055;
                        }
                        OP_Last => {
                            c2rust_current_block = 12752338440521374055;
                        }
                        OP_IfSizeBetween => {
                            let mut pC_18: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pCrsr_4: *mut BtCursor = ::core::ptr::null_mut::<
                                BtCursor,
                            >();
                            let mut res_6: ::core::ffi::c_int = 0;
                            let mut sz: i64_0 = 0;
                            pC_18 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pCrsr_4 = (*pC_18).uc.pCursor;
                            rc = sqlite3BtreeFirst(pCrsr_4, &raw mut res_6);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            if res_6 != 0 as ::core::ffi::c_int {
                                sz = -1 as ::core::ffi::c_int as i64_0;
                            } else {
                                sz = sqlite3BtreeRowCountEst(pCrsr_4);
                                sz = sqlite3LogEst(sz as u64_0) as i64_0;
                            }
                            res_6 = (sz >= (*pOp).p3 as ::core::ffi::c_longlong
                                && sz <= (*pOp).p4.i as ::core::ffi::c_longlong)
                                as ::core::ffi::c_int;
                            if res_6 != 0 {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_SorterSort | OP_Sort => {
                            (*p).aCounter[SQLITE_STMTSTATUS_SORT as usize] = (*p)
                                .aCounter[SQLITE_STMTSTATUS_SORT as usize]
                                .wrapping_add(1);
                            c2rust_current_block = 15206327760975505511;
                        }
                        OP_Rewind => {
                            c2rust_current_block = 15206327760975505511;
                        }
                        OP_IfEmpty => {
                            let mut pC_20: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pCrsr_6: *mut BtCursor = ::core::ptr::null_mut::<
                                BtCursor,
                            >();
                            let mut res_8: ::core::ffi::c_int = 0;
                            pC_20 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pCrsr_6 = (*pC_20).uc.pCursor;
                            rc = sqlite3BtreeIsEmpty(pCrsr_6, &raw mut res_8);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            if res_8 != 0 {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_SorterNext => {
                            pC_21 = ::core::ptr::null_mut::<VdbeCursor>();
                            pC_21 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            rc = sqlite3VdbeSorterNext(db, pC_21);
                            c2rust_current_block = 2917012134985143631;
                        }
                        OP_Prev => {
                            pC_21 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            rc = sqlite3BtreePrevious((*pC_21).uc.pCursor, (*pOp).p3);
                            c2rust_current_block = 2917012134985143631;
                        }
                        OP_Next => {
                            pC_21 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            rc = sqlite3BtreeNext((*pC_21).uc.pCursor, (*pOp).p3);
                            c2rust_current_block = 2917012134985143631;
                        }
                        OP_IdxInsert => {
                            let mut pC_22: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut x_1: BtreePayload = BtreePayload {
                                pKey: ::core::ptr::null::<::core::ffi::c_void>(),
                                nKey: 0,
                                pData: ::core::ptr::null::<::core::ffi::c_void>(),
                                aMem: ::core::ptr::null_mut::<sqlite3_value>(),
                                nMem: 0,
                                nData: 0,
                                nZero: 0

                            };
                            pC_22 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pIn2 = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            if (*pOp).p5 as ::core::ffi::c_int & OPFLAG_NCHANGE != 0 {
                                (*p).nChange += 1;
                            }
                            rc = if (*pIn2).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                                sqlite3VdbeMemExpandBlob(pIn2)
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            x_1.nKey = (*pIn2).n as sqlite3_int64;
                            x_1.pKey = (*pIn2).z as *const ::core::ffi::c_void;
                            x_1.aMem = aMem.offset((*pOp).p3 as isize)
                                as *mut sqlite3_value;
                            x_1.nMem = (*pOp).p4.i as u16_0;
                            rc = sqlite3BtreeInsert(
                                (*pC_22).uc.pCursor,
                                &raw mut x_1,
                                (*pOp).p5 as ::core::ffi::c_int
                                    & (OPFLAG_APPEND | OPFLAG_SAVEPOSITION | OPFLAG_PREFORMAT),
                                if (*pOp).p5 as ::core::ffi::c_int & OPFLAG_USESEEKRESULT
                                    != 0
                                {
                                    (*pC_22).seekResult
                                } else {
                                    0 as ::core::ffi::c_int
                                },
                            );
                            (*pC_22).cacheStatus = CACHE_STALE as u32_0;
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_SorterInsert => {
                            let mut pC_23: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pC_23 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pIn2 = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            rc = if (*pIn2).flags as ::core::ffi::c_int & MEM_Zero != 0 {
                                sqlite3VdbeMemExpandBlob(pIn2)
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            rc = sqlite3VdbeSorterWrite(pC_23, pIn2);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_IdxDelete => {
                            let mut pC_24: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pCrsr_7: *mut BtCursor = ::core::ptr::null_mut::<
                                BtCursor,
                            >();
                            let mut res_9: ::core::ffi::c_int = 0;
                            let mut r_2: UnpackedRecord = UnpackedRecord {
                                pKeyInfo: ::core::ptr::null_mut::<KeyInfo>(),
                                aMem: ::core::ptr::null_mut::<Mem>(),
                                u: C2Rust_Unnamed_24 {
                                    z: ::core::ptr::null_mut::<::core::ffi::c_char>()

                                },
                                n: 0,
                                nField: 0,
                                default_rc: 0,
                                errCode: 0,
                                r1: 0,
                                r2: 0,
                                eqSeen: 0

                            };
                            pC_24 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pCrsr_7 = (*pC_24).uc.pCursor;
                            r_2.pKeyInfo = (*pC_24).pKeyInfo;
                            r_2.nField = (*pOp).p3 as u16_0;
                            r_2.default_rc = 0 as i8_0;
                            r_2.aMem = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            rc = sqlite3BtreeIndexMoveto(
                                pCrsr_7,
                                &raw mut r_2,
                                &raw mut res_9,
                            );
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            if res_9 == 0 as ::core::ffi::c_int {
                                rc = sqlite3BtreeDelete(pCrsr_7, BTREE_AUXDELETE as u8_0);
                                if rc != 0 {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                            } else if (*pOp).p5 as ::core::ffi::c_int != 0
                                && sqlite3WritableSchema(db) == 0
                            {
                                rc = sqlite3ReportError(
                                    SQLITE_CORRUPT_INDEX,
                                    6663 as ::core::ffi::c_int,
                                    b"index corruption\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            (*pC_24).cacheStatus = CACHE_STALE as u32_0;
                            (*pC_24).seekResult = 0 as ::core::ffi::c_int;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_DeferredSeek => {
                            c2rust_current_block = 17265526906670415574;
                        }
                        OP_IdxRowid => {
                            c2rust_current_block = 17265526906670415574;
                        }
                        OP_FinishSeek => {
                            let mut pC_26: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pC_26 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if (*pC_26).deferredMoveto != 0 {
                                rc = sqlite3VdbeFinishMoveto(pC_26);
                                if rc != 0 {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                c2rust_current_block = 5783071609795492627;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_IdxLE => {
                            c2rust_current_block = 13858715045951221004;
                        }
                        OP_IdxGT => {
                            c2rust_current_block = 13858715045951221004;
                        }
                        OP_IdxLT => {
                            c2rust_current_block = 11000743977270914936;
                        }
                        OP_IdxGE => {
                            c2rust_current_block = 6548257557770401308;
                        }
                        OP_Destroy => {
                            let mut iMoved: ::core::ffi::c_int = 0;
                            let mut iDb_1: ::core::ffi::c_int = 0;
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            (*pOut).flags = MEM_Null as u16_0;
                            if (*db).nVdbeRead
                                > (*db).nVDestroy + 1 as ::core::ffi::c_int
                            {
                                rc = SQLITE_LOCKED;
                                (*p).errorAction = OE_Abort as u8_0;
                                c2rust_current_block = 8086965459351668542;
                                break;
                            } else {
                                iDb_1 = (*pOp).p3;
                                iMoved = 0 as ::core::ffi::c_int;
                                rc = sqlite3BtreeDropTable(
                                    (*(*db).aDb.offset(iDb_1 as isize)).pBt,
                                    (*pOp).p1,
                                    &raw mut iMoved,
                                );
                                (*pOut).flags = MEM_Int as u16_0;
                                (*pOut).u.i = iMoved as i64_0;
                                if rc != 0 {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                if iMoved != 0 as ::core::ffi::c_int {
                                    sqlite3RootPageMoved(
                                        db,
                                        iDb_1,
                                        iMoved as Pgno,
                                        (*pOp).p1 as Pgno,
                                    );
                                    resetSchemaOnFault = (iDb_1 + 1 as ::core::ffi::c_int)
                                        as u8_0;
                                }
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Clear => {
                            let mut nChange: i64_0 = 0;
                            nChange = 0 as i64_0;
                            rc = sqlite3BtreeClearTable(
                                (*(*db).aDb.offset((*pOp).p2 as isize)).pBt,
                                (*pOp).p1 as u32_0 as ::core::ffi::c_int,
                                &raw mut nChange,
                            );
                            if (*pOp).p3 != 0 {
                                (*p).nChange += nChange as ::core::ffi::c_longlong;
                                if (*pOp).p3 > 0 as ::core::ffi::c_int {
                                    let ref mut c2rust_fresh14 = (*aMem
                                        .offset((*pOp).p3 as isize))
                                        .u
                                        .i;
                                    *c2rust_fresh14 += nChange as ::core::ffi::c_longlong;
                                }
                            }
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_ResetSorter => {
                            let mut pC_28: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pC_28 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if (*pC_28).eCurType as ::core::ffi::c_int == CURTYPE_SORTER
                            {
                                sqlite3VdbeSorterReset(db, (*pC_28).uc.pSorter);
                            } else {
                                rc = sqlite3BtreeClearTableOfCursor((*pC_28).uc.pCursor);
                                if rc != 0 {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_CreateBtree => {
                            let mut pgno: Pgno = 0;
                            let mut pDb_2: *mut Db = ::core::ptr::null_mut::<Db>();
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            pgno = 0 as Pgno;
                            pDb_2 = (*db).aDb.offset((*pOp).p1 as isize) as *mut Db;
                            rc = sqlite3BtreeCreateTable(
                                (*pDb_2).pBt,
                                &raw mut pgno,
                                (*pOp).p3,
                            );
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            (*pOut).u.i = pgno as i64_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_SqlExec => {
                            let mut zErr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                ::core::ffi::c_char,
                            >();
                            let mut xAuth: sqlite3_xauth = None;
                            let mut mTrace: u8_0 = 0;
                            let mut savedAnalysisLimit: ::core::ffi::c_int = 0;
                            (*db).nSqlExec = (*db).nSqlExec.wrapping_add(1);
                            zErr_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            xAuth = (*db).xAuth;
                            mTrace = (*db).mTrace;
                            savedAnalysisLimit = (*db).nAnalysisLimit;
                            if (*pOp).p1 & 0x1 as ::core::ffi::c_int != 0 {
                                (*db).xAuth = None;
                                (*db).mTrace = 0 as u8_0;
                            }
                            if (*pOp).p1 & 0x2 as ::core::ffi::c_int != 0 {
                                (*db).nAnalysisLimit = (*pOp).p2;
                            }
                            rc = sqlite3_exec(
                                db,
                                (*pOp).p4.z,
                                None,
                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                &raw mut zErr_0,
                            );
                            (*db).nSqlExec = (*db).nSqlExec.wrapping_sub(1);
                            (*db).xAuth = xAuth;
                            (*db).mTrace = mTrace;
                            (*db).nAnalysisLimit = savedAnalysisLimit;
                            if !zErr_0.is_null() || rc != 0 {
                                sqlite3VdbeError(
                                    p,
                                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                    zErr_0,
                                );
                                sqlite3_free(zErr_0 as *mut ::core::ffi::c_void);
                                if rc == SQLITE_NOMEM {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                } else {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_ParseSchema => {
                            let mut iDb_2: ::core::ffi::c_int = 0;
                            let mut zSchema: *const ::core::ffi::c_char = ::core::ptr::null::<
                                ::core::ffi::c_char,
                            >();
                            let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                ::core::ffi::c_char,
                            >();
                            let mut initData: InitData = InitData {
                                db: ::core::ptr::null_mut::<sqlite3>(),
                                pzErrMsg: ::core::ptr::null_mut::<
                                    *mut ::core::ffi::c_char,
                                >(),
                                iDb: 0,
                                rc: 0,
                                mInitFlags: 0,
                                nInitRow: 0,
                                mxPage: 0

                            };
                            iDb_2 = (*pOp).p1;
                            if (*pOp).p4.z.is_null() {
                                sqlite3SchemaClear(
                                    (*(*db).aDb.offset(iDb_2 as isize)).pSchema
                                        as *mut ::core::ffi::c_void,
                                );
                                (*db).mDbFlags
                                    &= !DBFLAG_SchemaKnownOk as ::core::ffi::c_uint;
                                rc = sqlite3InitOne(
                                    db,
                                    iDb_2,
                                    &raw mut (*p).zErrMsg,
                                    (*pOp).p5 as u32_0,
                                );
                                (*db).mDbFlags
                                    |= DBFLAG_SchemaChange as ::core::ffi::c_uint;
                                (*p).set_expired(0 as bft as bft);
                            } else {
                                zSchema = LEGACY_SCHEMA_TABLE.as_ptr();
                                initData.db = db;
                                initData.iDb = iDb_2;
                                initData.pzErrMsg = &raw mut (*p).zErrMsg;
                                initData.mInitFlags = 0 as u32_0;
                                initData.mxPage = sqlite3BtreeLastPage(
                                    (*(*db).aDb.offset(iDb_2 as isize)).pBt,
                                );
                                zSql = sqlite3MPrintf(
                                    db,
                                    b"SELECT*FROM\"%w\".%s WHERE %s ORDER BY rowid\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*(*db).aDb.offset(iDb_2 as isize)).zDbSName,
                                    zSchema,
                                    (*pOp).p4.z,
                                );
                                if zSql.is_null() {
                                    rc = SQLITE_NOMEM_BKPT;
                                } else {
                                    (*db).init.busy = 1 as u8_0;
                                    initData.rc = SQLITE_OK;
                                    initData.nInitRow = 0 as u32_0;
                                    rc = sqlite3_exec(
                                        db,
                                        zSql,
                                        Some(
                                            sqlite3InitCallback
                                                as unsafe extern "C" fn(
                                                    *mut ::core::ffi::c_void,
                                                    ::core::ffi::c_int,
                                                    *mut *mut ::core::ffi::c_char,
                                                    *mut *mut ::core::ffi::c_char,
                                                ) -> ::core::ffi::c_int,
                                        ),
                                        &raw mut initData as *mut ::core::ffi::c_void,
                                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                    );
                                    if rc == SQLITE_OK {
                                        rc = initData.rc;
                                    }
                                    if rc == SQLITE_OK
                                        && initData.nInitRow == 0 as ::core::ffi::c_uint
                                    {
                                        rc = sqlite3CorruptError(7161 as ::core::ffi::c_int);
                                    }
                                    sqlite3DbFreeNN(db, zSql as *mut ::core::ffi::c_void);
                                    (*db).init.busy = 0 as u8_0;
                                }
                            }
                            if rc != 0 {
                                sqlite3ResetAllSchemasOfConnection(db);
                                if rc == SQLITE_NOMEM {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                } else {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_LoadAnalysis => {
                            rc = sqlite3AnalysisLoad(db, (*pOp).p1);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_DropTable => {
                            sqlite3UnlinkAndDeleteTable(db, (*pOp).p1, (*pOp).p4.z);
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_DropIndex => {
                            sqlite3UnlinkAndDeleteIndex(db, (*pOp).p1, (*pOp).p4.z);
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_DropTrigger => {
                            sqlite3UnlinkAndDeleteTrigger(db, (*pOp).p1, (*pOp).p4.z);
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_IntegrityCk => {
                            let mut nRoot: ::core::ffi::c_int = 0;
                            let mut aRoot: *mut Pgno = ::core::ptr::null_mut::<Pgno>();
                            let mut nErr: ::core::ffi::c_int = 0;
                            let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                ::core::ffi::c_char,
                            >();
                            let mut pnErr: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            nRoot = (*pOp).p2;
                            aRoot = (*pOp).p4.ai as *mut Pgno;
                            pnErr = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pIn1 = aMem
                                .offset(((*pOp).p1 + 1 as ::core::ffi::c_int) as isize)
                                as *mut Mem;
                            rc = sqlite3BtreeIntegrityCheck(
                                db,
                                (*(*db).aDb.offset((*pOp).p5 as isize)).pBt,
                                aRoot.offset(1 as ::core::ffi::c_int as isize) as *mut Pgno,
                                aMem.offset((*pOp).p3 as isize) as *mut sqlite3_value,
                                nRoot,
                                (*pnErr).u.i as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int,
                                &raw mut nErr,
                                &raw mut z,
                            );
                            sqlite3VdbeMemSetNull(pIn1);
                            if !(nErr == 0 as ::core::ffi::c_int) {
                                if rc != 0 {
                                    sqlite3_free(z as *mut ::core::ffi::c_void);
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                } else {
                                    (*pnErr).u.i
                                        -= (nErr - 1 as ::core::ffi::c_int)
                                            as ::core::ffi::c_longlong;
                                    sqlite3VdbeMemSetStr(
                                        pIn1,
                                        z,
                                        -1 as ::core::ffi::c_int as i64_0,
                                        SQLITE_UTF8 as u8_0,
                                        Some(
                                            sqlite3_free
                                                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                                        ),
                                    );
                                }
                            }
                            sqlite3VdbeChangeEncoding(
                                pIn1,
                                encoding as ::core::ffi::c_int,
                            );
                            c2rust_current_block = 6498258645234546759;
                        }
                        OP_RowSetAdd => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pIn2 = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Blob
                                == 0 as ::core::ffi::c_int
                            {
                                if sqlite3VdbeMemSetRowSet(pIn1) != 0 {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                }
                            }
                            sqlite3RowSetInsert((*pIn1).z as *mut RowSet, (*pIn2).u.i);
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_RowSetRead => {
                            let mut val: i64_0 = 0;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Blob
                                == 0 as ::core::ffi::c_int
                                || sqlite3RowSetNext((*pIn1).z as *mut RowSet, &raw mut val)
                                    == 0 as ::core::ffi::c_int
                            {
                                sqlite3VdbeMemSetNull(pIn1);
                                c2rust_current_block = 13898972957494642055;
                            } else {
                                sqlite3VdbeMemSetInt64(
                                    aMem.offset((*pOp).p3 as isize) as *mut Mem,
                                    val,
                                );
                                c2rust_current_block = 6498258645234546759;
                            }
                        }
                        OP_RowSetTest => {
                            let mut iSet: ::core::ffi::c_int = 0;
                            let mut exists: ::core::ffi::c_int = 0;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pIn3 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            iSet = (*pOp).p4.i;
                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Blob
                                == 0 as ::core::ffi::c_int
                            {
                                if sqlite3VdbeMemSetRowSet(pIn1) != 0 {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                }
                            }
                            if iSet != 0 {
                                exists = sqlite3RowSetTest(
                                    (*pIn1).z as *mut RowSet,
                                    iSet,
                                    (*pIn3).u.i,
                                );
                                if exists != 0 {
                                    c2rust_current_block = 17233182392562552756;
                                } else {
                                    c2rust_current_block = 6625593353261917340;
                                }
                            } else {
                                c2rust_current_block = 6625593353261917340;
                            }
                            match c2rust_current_block {
                                17233182392562552756 => {}
                                _ => {
                                    if iSet >= 0 as ::core::ffi::c_int {
                                        sqlite3RowSetInsert((*pIn1).z as *mut RowSet, (*pIn3).u.i);
                                    }
                                    c2rust_current_block = 5783071609795492627;
                                }
                            }
                        }
                        OP_Program => {
                            let mut nMem: ::core::ffi::c_int = 0;
                            let mut nByte_1: i64_0 = 0;
                            let mut pRt: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut pMem_0: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut pEnd: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut pFrame_1: *mut VdbeFrame = ::core::ptr::null_mut::<
                                VdbeFrame,
                            >();
                            let mut pProgram: *mut SubProgram = ::core::ptr::null_mut::<
                                SubProgram,
                            >();
                            let mut t_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                                ::core::ffi::c_void,
                            >();
                            pProgram = (*pOp).p4.pProgram;
                            pRt = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            if (*pOp).p5 != 0 {
                                t_0 = (*pProgram).token;
                                pFrame_1 = (*p).pFrame;
                                while !pFrame_1.is_null() && (*pFrame_1).token != t_0 {
                                    pFrame_1 = (*pFrame_1).pParent;
                                }
                                if !pFrame_1.is_null() {
                                    c2rust_current_block = 5783071609795492627;
                                } else {
                                    c2rust_current_block = 8217459546479625078;
                                }
                            } else {
                                c2rust_current_block = 8217459546479625078;
                            }
                            match c2rust_current_block {
                                5783071609795492627 => {}
                                _ => {
                                    if (*p).nFrame
                                        >= (*db).aLimit[SQLITE_LIMIT_TRIGGER_DEPTH as usize]
                                    {
                                        rc = SQLITE_ERROR;
                                        sqlite3VdbeError(
                                            p,
                                            b"too many levels of trigger recursion\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        c2rust_current_block = 8086965459351668542;
                                        break;
                                    } else {
                                        if (*pRt).flags as ::core::ffi::c_int & MEM_Blob
                                            == 0 as ::core::ffi::c_int
                                        {
                                            nMem = (*pProgram).nMem + (*pProgram).nCsr;
                                            if (*pProgram).nCsr == 0 as ::core::ffi::c_int {
                                                nMem += 1;
                                            }
                                            nByte_1 = (((::core::mem::size_of::<VdbeFrame>() as usize)
                                                .wrapping_add(7 as usize)
                                                & !(7 as ::core::ffi::c_int) as usize)
                                                .wrapping_add(
                                                    (nMem as usize)
                                                        .wrapping_mul(::core::mem::size_of::<Mem>() as usize),
                                                )
                                                .wrapping_add(
                                                    ((*pProgram).nCsr as usize)
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<*mut VdbeCursor>() as usize,
                                                        ),
                                                ) as ::core::ffi::c_ulonglong)
                                                .wrapping_add(
                                                    ((7 as ::core::ffi::c_longlong
                                                        + (*pProgram).nOp as ::core::ffi::c_longlong)
                                                        / 8 as ::core::ffi::c_longlong) as ::core::ffi::c_ulonglong,
                                                ) as i64_0;
                                            pFrame_1 = sqlite3DbMallocZero(db, nByte_1 as u64_0)
                                                as *mut VdbeFrame;
                                            if pFrame_1.is_null() {
                                                c2rust_current_block = 9955115899106739073;
                                                break;
                                            }
                                            sqlite3VdbeMemRelease(pRt);
                                            (*pRt).flags = (MEM_Blob | MEM_Dyn) as u16_0;
                                            (*pRt).z = pFrame_1 as *mut ::core::ffi::c_char;
                                            (*pRt).n = nByte_1 as ::core::ffi::c_int;
                                            (*pRt).xDel = Some(
                                                sqlite3VdbeFrameMemDel
                                                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                                            )
                                                as Option<
                                                    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                                                >;
                                            (*pFrame_1).v = p;
                                            (*pFrame_1).nChildMem = nMem;
                                            (*pFrame_1).nChildCsr = (*pProgram).nCsr;
                                            (*pFrame_1).pc = pOp.offset_from(aOp) as ::core::ffi::c_long
                                                as ::core::ffi::c_int;
                                            (*pFrame_1).aMem = (*p).aMem;
                                            (*pFrame_1).nMem = (*p).nMem;
                                            (*pFrame_1).apCsr = (*p).apCsr;
                                            (*pFrame_1).nCursor = (*p).nCursor;
                                            (*pFrame_1).aOp = (*p).aOp;
                                            (*pFrame_1).nOp = (*p).nOp;
                                            (*pFrame_1).token = (*pProgram).token;
                                            pEnd = ((pFrame_1 as *mut u8_0)
                                                .offset(
                                                    ((::core::mem::size_of::<VdbeFrame>() as usize)
                                                        .wrapping_add(7 as usize)
                                                        & !(7 as ::core::ffi::c_int) as usize) as isize,
                                                ) as *mut u8_0 as *mut Mem)
                                                .offset((*pFrame_1).nChildMem as isize) as *mut Mem;
                                            pMem_0 = (pFrame_1 as *mut u8_0)
                                                .offset(
                                                    ((::core::mem::size_of::<VdbeFrame>() as usize)
                                                        .wrapping_add(7 as usize)
                                                        & !(7 as ::core::ffi::c_int) as usize) as isize,
                                                ) as *mut u8_0 as *mut Mem;
                                            while pMem_0 != pEnd {
                                                (*pMem_0).flags = MEM_Undefined as u16_0;
                                                (*pMem_0).db = db;
                                                pMem_0 = pMem_0.offset(1);
                                            }
                                        } else {
                                            pFrame_1 = (*pRt).z as *mut VdbeFrame;
                                        }
                                        (*p).nFrame += 1;
                                        (*pFrame_1).pParent = (*p).pFrame;
                                        (*pFrame_1).lastRowid = (*db).lastRowid;
                                        (*pFrame_1).nChange = (*p).nChange;
                                        (*pFrame_1).nDbChange = (*(*p).db).nChange;
                                        (*pFrame_1).pAuxData = (*p).pAuxData;
                                        (*p).pAuxData = ::core::ptr::null_mut::<AuxData>();
                                        (*p).nChange = 0 as i64_0;
                                        (*p).pFrame = pFrame_1;
                                        aMem = (pFrame_1 as *mut u8_0)
                                            .offset(
                                                ((::core::mem::size_of::<VdbeFrame>() as usize)
                                                    .wrapping_add(7 as usize)
                                                    & !(7 as ::core::ffi::c_int) as usize) as isize,
                                            ) as *mut u8_0 as *mut Mem;
                                        (*p).aMem = aMem;
                                        (*p).nMem = (*pFrame_1).nChildMem;
                                        (*p).nCursor = (*pFrame_1).nChildCsr as u16_0
                                            as ::core::ffi::c_int;
                                        (*p).apCsr = aMem.offset((*p).nMem as isize) as *mut Mem
                                            as *mut *mut VdbeCursor;
                                        (*pFrame_1).aOnce = (*p)
                                            .apCsr
                                            .offset((*pProgram).nCsr as isize) as *mut *mut VdbeCursor
                                            as *mut u8_0;
                                        memset(
                                            (*pFrame_1).aOnce as *mut ::core::ffi::c_void,
                                            0 as ::core::ffi::c_int,
                                            (((*pProgram).nOp + 7 as ::core::ffi::c_int)
                                                / 8 as ::core::ffi::c_int) as size_t,
                                        );
                                        aOp = (*pProgram).aOp as *mut Op;
                                        (*p).aOp = aOp;
                                        (*p).nOp = (*pProgram).nOp;
                                        pOp = aOp.offset(-1 as ::core::ffi::c_int as isize)
                                            as *mut Op;
                                    }
                                    c2rust_current_block = 6498258645234546759;
                                }
                            }
                        }
                        OP_Param => {
                            let mut pFrame_2: *mut VdbeFrame = ::core::ptr::null_mut::<
                                VdbeFrame,
                            >();
                            let mut pIn: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            pFrame_2 = (*p).pFrame;
                            pIn = (*pFrame_2)
                                .aMem
                                .offset(
                                    ((*pOp).p1
                                        + (*(*pFrame_2).aOp.offset((*pFrame_2).pc as isize)).p1)
                                        as isize,
                                ) as *mut Mem;
                            sqlite3VdbeMemShallowCopy(pOut, pIn, MEM_Ephem);
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_FkCounter => {
                            if (*pOp).p1 != 0 {
                                (*db).nDeferredCons += (*pOp).p2 as ::core::ffi::c_longlong;
                            } else if (*db).flags as ::core::ffi::c_ulonglong
                                & SQLITE_DeferFKs as ::core::ffi::c_ulonglong != 0
                            {
                                (*db).nDeferredImmCons
                                    += (*pOp).p2 as ::core::ffi::c_longlong;
                            } else {
                                (*p).nFkConstraint += (*pOp).p2 as ::core::ffi::c_longlong;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_FkIfZero => {
                            if (*pOp).p1 != 0 {
                                if (*db).nDeferredCons == 0 as ::core::ffi::c_longlong
                                    && (*db).nDeferredImmCons == 0 as ::core::ffi::c_longlong
                                {
                                    c2rust_current_block = 17233182392562552756;
                                } else {
                                    c2rust_current_block = 5783071609795492627;
                                }
                            } else if (*p).nFkConstraint == 0 as ::core::ffi::c_longlong
                                && (*db).nDeferredImmCons == 0 as ::core::ffi::c_longlong
                            {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_MemMax => {
                            let mut pFrame_3: *mut VdbeFrame = ::core::ptr::null_mut::<
                                VdbeFrame,
                            >();
                            if !(*p).pFrame.is_null() {
                                pFrame_3 = (*p).pFrame;
                                while !(*pFrame_3).pParent.is_null() {
                                    pFrame_3 = (*pFrame_3).pParent;
                                }
                                pIn1 = (*pFrame_3).aMem.offset((*pOp).p1 as isize)
                                    as *mut Mem;
                            } else {
                                pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            }
                            sqlite3VdbeMemIntegerify(pIn1);
                            pIn2 = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            sqlite3VdbeMemIntegerify(pIn2);
                            if (*pIn1).u.i < (*pIn2).u.i {
                                (*pIn1).u.i = (*pIn2).u.i;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_IfPos => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            if (*pIn1).u.i > 0 as ::core::ffi::c_longlong {
                                (*pIn1).u.i -= (*pOp).p3 as ::core::ffi::c_longlong;
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_OffsetLimit => {
                            let mut x_2: i64_0 = 0;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pIn3 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            x_2 = (*pIn1).u.i;
                            if x_2 <= 0 as ::core::ffi::c_longlong
                                || sqlite3AddInt64(
                                    &raw mut x_2,
                                    (if (*pIn3).u.i > 0 as ::core::ffi::c_longlong {
                                        (*pIn3).u.i
                                    } else {
                                        0 as i64_0
                                    }),
                                ) != 0
                            {
                                (*pOut).u.i = -1 as ::core::ffi::c_int as i64_0;
                            } else {
                                (*pOut).u.i = x_2;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_IfNotZero => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            if (*pIn1).u.i != 0 {
                                if (*pIn1).u.i > 0 as ::core::ffi::c_longlong {
                                    (*pIn1).u.i -= 1;
                                }
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_DecrJumpZero => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            if (*pIn1).u.i > SMALLEST_INT64 {
                                (*pIn1).u.i -= 1;
                            }
                            if (*pIn1).u.i == 0 as ::core::ffi::c_longlong {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_AggInverse | OP_AggStep => {
                            let mut n_3: ::core::ffi::c_int = 0;
                            let mut pCtx: *mut sqlite3_context = ::core::ptr::null_mut::<
                                sqlite3_context,
                            >();
                            let mut nAlloc: u64_0 = 0;
                            n_3 = (*pOp).p5 as ::core::ffi::c_int;
                            nAlloc = (48 as usize)
                                .wrapping_add(
                                    (n_3 as usize)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<*mut sqlite3_value>() as usize,
                                        ),
                                ) as u64_0;
                            pCtx = sqlite3DbMallocRawNN(
                                db,
                                nAlloc.wrapping_add(::core::mem::size_of::<Mem>() as u64_0),
                            ) as *mut sqlite3_context;
                            if pCtx.is_null() {
                                c2rust_current_block = 9955115899106739073;
                                break;
                            }
                            (*pCtx).pOut = (pCtx as *mut u8_0).offset(nAlloc as isize)
                                as *mut Mem;
                            sqlite3VdbeMemInit((*pCtx).pOut, db, MEM_Null as u16_0);
                            (*pCtx).pMem = ::core::ptr::null_mut::<Mem>();
                            (*pCtx).pFunc = (*pOp).p4.pFunc;
                            (*pCtx).iOp = pOp.offset_from(aOp) as ::core::ffi::c_long
                                as ::core::ffi::c_int;
                            (*pCtx).pVdbe = p;
                            (*pCtx).skipFlag = 0 as u8_0;
                            (*pCtx).isError = 0 as ::core::ffi::c_int;
                            (*pCtx).enc = encoding;
                            (*pCtx).argc = n_3 as u16_0;
                            (*pOp).p4type = P4_FUNCCTX as ::core::ffi::c_schar;
                            (*pOp).p4.pCtx = pCtx;
                            (*pOp).opcode = OP_AggStep1 as u8_0;
                            c2rust_current_block = 12597090803267581175;
                        }
                        OP_AggStep1 => {
                            c2rust_current_block = 12597090803267581175;
                        }
                        OP_AggValue | OP_AggFinal => {
                            let mut pMem_2: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            pMem_2 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            if (*pOp).p3 != 0 {
                                rc = sqlite3VdbeMemAggValue(
                                    pMem_2,
                                    aMem.offset((*pOp).p3 as isize) as *mut Mem,
                                    (*pOp).p4.pFunc,
                                );
                                pMem_2 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            } else {
                                rc = sqlite3VdbeMemFinalize(pMem_2, (*pOp).p4.pFunc);
                            }
                            if rc != 0 {
                                sqlite3VdbeError(
                                    p,
                                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                    sqlite3_value_text(pMem_2 as *mut sqlite3_value),
                                );
                                c2rust_current_block = 8086965459351668542;
                                break;
                            } else {
                                sqlite3VdbeChangeEncoding(
                                    pMem_2,
                                    encoding as ::core::ffi::c_int,
                                );
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Checkpoint => {
                            let mut i_4: ::core::ffi::c_int = 0;
                            let mut aRes: [::core::ffi::c_int; 3] = [0; 3];
                            let mut pMem_3: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            aRes[0 as ::core::ffi::c_int as usize] = 0
                                as ::core::ffi::c_int;
                            aRes[2 as ::core::ffi::c_int as usize] = -1
                                as ::core::ffi::c_int;
                            aRes[1 as ::core::ffi::c_int as usize] = aRes[2
                                as ::core::ffi::c_int as usize];
                            rc = sqlite3Checkpoint(
                                db,
                                (*pOp).p1,
                                (*pOp).p2,
                                (&raw mut aRes as *mut ::core::ffi::c_int)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_int,
                                (&raw mut aRes as *mut ::core::ffi::c_int)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_int,
                            );
                            if rc != 0 {
                                if rc != SQLITE_BUSY {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                rc = SQLITE_OK;
                                aRes[0 as ::core::ffi::c_int as usize] = 1
                                    as ::core::ffi::c_int;
                            }
                            i_4 = 0 as ::core::ffi::c_int;
                            pMem_3 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            while i_4 < 3 as ::core::ffi::c_int {
                                sqlite3VdbeMemSetInt64(pMem_3, aRes[i_4 as usize] as i64_0);
                                i_4 += 1;
                                pMem_3 = pMem_3.offset(1);
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_JournalMode => {
                            let mut pBt_0: *mut Btree = ::core::ptr::null_mut::<Btree>();
                            let mut pPager: *mut Pager = ::core::ptr::null_mut::<
                                Pager,
                            >();
                            let mut eNew: ::core::ffi::c_int = 0;
                            let mut eOld: ::core::ffi::c_int = 0;
                            let mut zFilename: *const ::core::ffi::c_char = ::core::ptr::null::<
                                ::core::ffi::c_char,
                            >();
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            eNew = (*pOp).p3;
                            pBt_0 = (*(*db).aDb.offset((*pOp).p1 as isize)).pBt;
                            pPager = sqlite3BtreePager(pBt_0) as *mut Pager;
                            eOld = sqlite3PagerGetJournalMode(pPager);
                            if eNew == PAGER_JOURNALMODE_QUERY {
                                eNew = eOld;
                            }
                            if sqlite3PagerOkToChangeJournalMode(pPager) == 0 {
                                eNew = eOld;
                            }
                            zFilename = sqlite3PagerFilename(
                                pPager,
                                1 as ::core::ffi::c_int,
                            );
                            if eNew == PAGER_JOURNALMODE_WAL
                                && (sqlite3Strlen30(zFilename) == 0 as ::core::ffi::c_int
                                    || sqlite3PagerWalSupported(pPager) == 0)
                            {
                                eNew = eOld;
                            }
                            if eNew != eOld
                                && (eOld == PAGER_JOURNALMODE_WAL
                                    || eNew == PAGER_JOURNALMODE_WAL)
                            {
                                if (*db).autoCommit == 0
                                    || (*db).nVdbeRead > 1 as ::core::ffi::c_int
                                {
                                    rc = SQLITE_ERROR;
                                    sqlite3VdbeError(
                                        p,
                                        b"cannot change %s wal mode from within a transaction\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        if eNew == PAGER_JOURNALMODE_WAL {
                                            b"into\0".as_ptr() as *const ::core::ffi::c_char
                                        } else {
                                            b"out of\0".as_ptr() as *const ::core::ffi::c_char
                                        },
                                    );
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                } else {
                                    if eOld == PAGER_JOURNALMODE_WAL {
                                        rc = sqlite3PagerCloseWal(pPager, db);
                                        if rc == SQLITE_OK {
                                            sqlite3PagerSetJournalMode(pPager, eNew);
                                        }
                                    } else if eOld == PAGER_JOURNALMODE_MEMORY {
                                        sqlite3PagerSetJournalMode(pPager, PAGER_JOURNALMODE_OFF);
                                    }
                                    if rc == SQLITE_OK {
                                        rc = sqlite3BtreeSetVersion(
                                            pBt_0,
                                            if eNew == PAGER_JOURNALMODE_WAL {
                                                2 as ::core::ffi::c_int
                                            } else {
                                                1 as ::core::ffi::c_int
                                            },
                                        );
                                    }
                                }
                            }
                            if rc != 0 {
                                eNew = eOld;
                            }
                            eNew = sqlite3PagerSetJournalMode(pPager, eNew);
                            (*pOut).flags = (MEM_Str | MEM_Static | MEM_Term) as u16_0;
                            (*pOut).z = sqlite3JournalModename(eNew)
                                as *mut ::core::ffi::c_char;
                            (*pOut).n = sqlite3Strlen30((*pOut).z);
                            (*pOut).enc = SQLITE_UTF8 as u8_0;
                            sqlite3VdbeChangeEncoding(
                                pOut,
                                encoding as ::core::ffi::c_int,
                            );
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Vacuum => {
                            rc = sqlite3RunVacuum(
                                &raw mut (*p).zErrMsg,
                                db,
                                (*pOp).p1,
                                if (*pOp).p2 != 0 {
                                    aMem.offset((*pOp).p2 as isize) as *mut sqlite3_value
                                } else {
                                    ::core::ptr::null_mut::<sqlite3_value>()
                                },
                            );
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_IncrVacuum => {
                            let mut pBt_1: *mut Btree = ::core::ptr::null_mut::<Btree>();
                            pBt_1 = (*(*db).aDb.offset((*pOp).p1 as isize)).pBt;
                            rc = sqlite3BtreeIncrVacuum(pBt_1);
                            if rc != 0 {
                                if rc != SQLITE_DONE {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                rc = SQLITE_OK;
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_Expire => {
                            if (*pOp).p1 == 0 {
                                sqlite3ExpirePreparedStatements(db, (*pOp).p2);
                            } else {
                                (*p)
                                    .set_expired(
                                        ((*pOp).p2 + 1 as ::core::ffi::c_int) as bft as bft,
                                    );
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_CursorLock => {
                            let mut pC_29: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pC_29 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            sqlite3BtreeCursorPin((*pC_29).uc.pCursor);
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_CursorUnlock => {
                            let mut pC_30: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pC_30 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            sqlite3BtreeCursorUnpin((*pC_30).uc.pCursor);
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_VBegin => {
                            let mut pVTab: *mut VTable = ::core::ptr::null_mut::<
                                VTable,
                            >();
                            pVTab = (*pOp).p4.pVtab;
                            rc = sqlite3VtabBegin(db, pVTab);
                            if !pVTab.is_null() {
                                sqlite3VtabImportErrmsg(p, (*pVTab).pVtab);
                            }
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_VCreate => {
                            let mut sMem_0: Mem = Mem {
                                u: MemValue { r: 0. },
                                z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                n: 0,
                                flags: 0,
                                enc: 0,
                                eSubtype: 0,
                                db: ::core::ptr::null_mut::<sqlite3>(),
                                szMalloc: 0,
                                uTemp: 0,
                                zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                xDel: None

                            };
                            let mut zTab: *const ::core::ffi::c_char = ::core::ptr::null::<
                                ::core::ffi::c_char,
                            >();
                            memset(
                                &raw mut sMem_0 as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<Mem>() as size_t,
                            );
                            sMem_0.db = db;
                            rc = sqlite3VdbeMemCopy(
                                &raw mut sMem_0,
                                aMem.offset((*pOp).p2 as isize) as *mut Mem,
                            );
                            zTab = sqlite3_value_text(&raw mut sMem_0)
                                as *const ::core::ffi::c_char;
                            if !zTab.is_null() {
                                rc = sqlite3VtabCallCreate(
                                    db,
                                    (*pOp).p1,
                                    zTab,
                                    &raw mut (*p).zErrMsg,
                                );
                            }
                            sqlite3VdbeMemRelease(&raw mut sMem_0);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_VDestroy => {
                            (*db).nVDestroy += 1;
                            rc = sqlite3VtabCallDestroy(db, (*pOp).p1, (*pOp).p4.z);
                            (*db).nVDestroy -= 1;
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_VOpen => {
                            let mut pCur_2: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pVCur: *mut sqlite3_vtab_cursor = ::core::ptr::null_mut::<
                                sqlite3_vtab_cursor,
                            >();
                            let mut pVtab_0: *mut sqlite3_vtab = ::core::ptr::null_mut::<
                                sqlite3_vtab,
                            >();
                            let mut pModule_0: *const sqlite3_module = ::core::ptr::null::<
                                sqlite3_module,
                            >();
                            pCur_2 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if !(!pCur_2.is_null()
                                && (*pCur_2).eCurType as ::core::ffi::c_int
                                    == 2 as ::core::ffi::c_int
                                && (*(*pCur_2).uc.pVCur).pVtab == (*(*pOp).p4.pVtab).pVtab)
                            {
                                pVCur = ::core::ptr::null_mut::<sqlite3_vtab_cursor>();
                                pVtab_0 = (*(*pOp).p4.pVtab).pVtab;
                                if pVtab_0.is_null() || (*pVtab_0).pModule.is_null() {
                                    rc = SQLITE_LOCKED;
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                } else {
                                    pModule_0 = (*pVtab_0).pModule;
                                    rc = (*pModule_0)
                                        .xOpen
                                        .expect(
                                            "non-null function pointer",
                                        )(pVtab_0, &raw mut pVCur);
                                    sqlite3VtabImportErrmsg(p, pVtab_0);
                                    if rc != 0 {
                                        c2rust_current_block = 8086965459351668542;
                                        break;
                                    }
                                    (*pVCur).pVtab = pVtab_0;
                                    pCur_2 = allocateCursor(
                                        p,
                                        (*pOp).p1,
                                        0 as ::core::ffi::c_int,
                                        CURTYPE_VTAB as u8_0,
                                    );
                                    if !pCur_2.is_null() {
                                        (*pCur_2).uc.pVCur = pVCur;
                                        (*pVtab_0).nRef += 1;
                                    } else {
                                        (*pModule_0)
                                            .xClose
                                            .expect("non-null function pointer")(pVCur);
                                        c2rust_current_block = 9955115899106739073;
                                        break;
                                    }
                                }
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_VCheck => {
                            let mut pTab_2: *mut Table = ::core::ptr::null_mut::<
                                Table,
                            >();
                            let mut pVtab_1: *mut sqlite3_vtab = ::core::ptr::null_mut::<
                                sqlite3_vtab,
                            >();
                            let mut pModule_1: *const sqlite3_module = ::core::ptr::null::<
                                sqlite3_module,
                            >();
                            let mut zErr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                ::core::ffi::c_char,
                            >();
                            pOut = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            sqlite3VdbeMemSetNull(pOut);
                            pTab_2 = (*pOp).p4.pTab;
                            if (*pTab_2).u.vtab.p.is_null() {
                                c2rust_current_block = 5783071609795492627;
                            } else {
                                pVtab_1 = (*(*pTab_2).u.vtab.p).pVtab;
                                pModule_1 = (*pVtab_1).pModule;
                                sqlite3VtabLock((*pTab_2).u.vtab.p);
                                rc = (*pModule_1)
                                    .xIntegrity
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    pVtab_1,
                                    (*(*db).aDb.offset((*pOp).p1 as isize)).zDbSName,
                                    (*pTab_2).zName,
                                    (*pOp).p3,
                                    &raw mut zErr_1,
                                );
                                sqlite3VtabUnlock((*pTab_2).u.vtab.p);
                                if rc != 0 {
                                    sqlite3_free(zErr_1 as *mut ::core::ffi::c_void);
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                } else if !zErr_1.is_null() {
                                    sqlite3VdbeMemSetStr(
                                        pOut,
                                        zErr_1,
                                        -1 as ::core::ffi::c_int as i64_0,
                                        SQLITE_UTF8 as u8_0,
                                        Some(
                                            sqlite3_free
                                                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                                        ),
                                    );
                                }
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_VInitIn => {
                            let mut pC_31: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pRhs: *mut ValueList = ::core::ptr::null_mut::<
                                ValueList,
                            >();
                            pC_31 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pRhs = sqlite3_malloc64(
                                ::core::mem::size_of::<ValueList>() as sqlite3_uint64,
                            ) as *mut ValueList;
                            if pRhs.is_null() {
                                c2rust_current_block = 9955115899106739073;
                                break;
                            }
                            (*pRhs).pCsr = (*pC_31).uc.pCursor;
                            (*pRhs).pOut = aMem.offset((*pOp).p3 as isize) as *mut Mem
                                as *mut sqlite3_value;
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            (*pOut).flags = MEM_Null as u16_0;
                            sqlite3VdbeMemSetPointer(
                                pOut,
                                pRhs as *mut ::core::ffi::c_void,
                                b"ValueList\0".as_ptr() as *const ::core::ffi::c_char,
                                Some(
                                    sqlite3VdbeValueListFree
                                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                                ),
                            );
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_VFilter => {
                            let mut nArg: ::core::ffi::c_int = 0;
                            let mut iQuery: ::core::ffi::c_int = 0;
                            let mut pModule_2: *const sqlite3_module = ::core::ptr::null::<
                                sqlite3_module,
                            >();
                            let mut pQuery: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut pArgc: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut pVCur_0: *mut sqlite3_vtab_cursor = ::core::ptr::null_mut::<
                                sqlite3_vtab_cursor,
                            >();
                            let mut pVtab_2: *mut sqlite3_vtab = ::core::ptr::null_mut::<
                                sqlite3_vtab,
                            >();
                            let mut pCur_3: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut res_11: ::core::ffi::c_int = 0;
                            let mut i_5: ::core::ffi::c_int = 0;
                            let mut apArg: *mut *mut Mem = ::core::ptr::null_mut::<
                                *mut Mem,
                            >();
                            pQuery = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            pArgc = pQuery.offset(1 as ::core::ffi::c_int as isize)
                                as *mut Mem;
                            pCur_3 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pVCur_0 = (*pCur_3).uc.pVCur;
                            pVtab_2 = (*pVCur_0).pVtab;
                            pModule_2 = (*pVtab_2).pModule;
                            nArg = (*pArgc).u.i as ::core::ffi::c_int;
                            iQuery = (*pQuery).u.i as ::core::ffi::c_int;
                            apArg = (*p).apArg;
                            i_5 = 0 as ::core::ffi::c_int;
                            while i_5 < nArg {
                                let ref mut c2rust_fresh16 = *apArg.offset(i_5 as isize);
                                *c2rust_fresh16 = pArgc
                                    .offset((i_5 + 1 as ::core::ffi::c_int) as isize)
                                    as *mut Mem;
                                i_5 += 1;
                            }
                            rc = (*pModule_2)
                                .xFilter
                                .expect(
                                    "non-null function pointer",
                                )(
                                pVCur_0,
                                iQuery,
                                (*pOp).p4.z,
                                nArg,
                                apArg as *mut *mut sqlite3_value,
                            );
                            sqlite3VtabImportErrmsg(p, pVtab_2);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            res_11 = (*pModule_2)
                                .xEof
                                .expect("non-null function pointer")(pVCur_0);
                            (*pCur_3).nullRow = 0 as u8_0;
                            if res_11 != 0 {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_VColumn => {
                            let mut pVtab_3: *mut sqlite3_vtab = ::core::ptr::null_mut::<
                                sqlite3_vtab,
                            >();
                            let mut pModule_3: *const sqlite3_module = ::core::ptr::null::<
                                sqlite3_module,
                            >();
                            let mut pDest_1: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut sContext: sqlite3_context = sqlite3_context {
                                pOut: ::core::ptr::null_mut::<Mem>(),
                                pFunc: ::core::ptr::null_mut::<FuncDef>(),
                                pMem: ::core::ptr::null_mut::<Mem>(),
                                pVdbe: ::core::ptr::null_mut::<Vdbe>(),
                                iOp: 0,
                                isError: 0,
                                enc: 0,
                                skipFlag: 0,
                                argc: 0,
                                argv: []

                            };
                            let mut nullFunc: FuncDef = FuncDef {
                                nArg: 0,
                                funcFlags: 0,
                                pUserData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                pNext: ::core::ptr::null_mut::<FuncDef>(),
                                xSFunc: None,
                                xFinalize: None,
                                xValue: None,
                                xInverse: None,
                                zName: ::core::ptr::null::<::core::ffi::c_char>(),
                                u: C2Rust_Unnamed_2 {
                                    pHash: ::core::ptr::null_mut::<FuncDef>()

                                }

                            };
                            let mut pCur_4: *mut VdbeCursor = *(*p)
                                .apCsr
                                .offset((*pOp).p1 as isize);
                            pDest_1 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            if (*pCur_4).nullRow != 0 {
                                sqlite3VdbeMemSetNull(pDest_1);
                            } else {
                                pVtab_3 = (*(*pCur_4).uc.pVCur).pVtab;
                                pModule_3 = (*pVtab_3).pModule;
                                memset(
                                    &raw mut sContext as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    ::core::mem::size_of::<sqlite3_context>() as size_t,
                                );
                                sContext.pOut = pDest_1;
                                sContext.enc = encoding;
                                nullFunc.pUserData = ::core::ptr::null_mut::<
                                    ::core::ffi::c_void,
                                >();
                                nullFunc.funcFlags = SQLITE_RESULT_SUBTYPE as u32_0;
                                sContext.pFunc = &raw mut nullFunc;
                                if (*pOp).p5 as ::core::ffi::c_int & OPFLAG_NOCHNG != 0 {
                                    sqlite3VdbeMemSetNull(pDest_1);
                                    (*pDest_1).flags = (MEM_Null | MEM_Zero) as u16_0;
                                    (*pDest_1).u.nZero = 0 as ::core::ffi::c_int;
                                } else {
                                    (*pDest_1).flags = ((*pDest_1).flags as ::core::ffi::c_int
                                        & !(MEM_TypeMask | MEM_Zero) | 0x1 as ::core::ffi::c_int)
                                        as u16_0;
                                }
                                rc = (*pModule_3)
                                    .xColumn
                                    .expect(
                                        "non-null function pointer",
                                    )((*pCur_4).uc.pVCur, &raw mut sContext, (*pOp).p2);
                                sqlite3VtabImportErrmsg(p, pVtab_3);
                                if sContext.isError > 0 as ::core::ffi::c_int {
                                    sqlite3VdbeError(
                                        p,
                                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                        sqlite3_value_text(pDest_1 as *mut sqlite3_value),
                                    );
                                    rc = sContext.isError;
                                }
                                sqlite3VdbeChangeEncoding(
                                    pDest_1,
                                    encoding as ::core::ffi::c_int,
                                );
                                if rc != 0 {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_VNext => {
                            let mut pVtab_4: *mut sqlite3_vtab = ::core::ptr::null_mut::<
                                sqlite3_vtab,
                            >();
                            let mut pModule_4: *const sqlite3_module = ::core::ptr::null::<
                                sqlite3_module,
                            >();
                            let mut res_12: ::core::ffi::c_int = 0;
                            let mut pCur_5: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            pCur_5 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if (*pCur_5).nullRow != 0 {
                                c2rust_current_block = 5783071609795492627;
                            } else {
                                pVtab_4 = (*(*pCur_5).uc.pVCur).pVtab;
                                pModule_4 = (*pVtab_4).pModule;
                                rc = (*pModule_4)
                                    .xNext
                                    .expect("non-null function pointer")((*pCur_5).uc.pVCur);
                                sqlite3VtabImportErrmsg(p, pVtab_4);
                                if rc != 0 {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                res_12 = (*pModule_4)
                                    .xEof
                                    .expect("non-null function pointer")((*pCur_5).uc.pVCur);
                                if res_12 == 0 {
                                    c2rust_current_block = 13898972957494642055;
                                } else {
                                    c2rust_current_block = 6498258645234546759;
                                }
                            }
                        }
                        OP_VRename => {
                            let mut pVtab_5: *mut sqlite3_vtab = ::core::ptr::null_mut::<
                                sqlite3_vtab,
                            >();
                            let mut pName: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            let mut isLegacy: ::core::ffi::c_int = 0;
                            isLegacy = ((*db).flags as ::core::ffi::c_ulonglong
                                & SQLITE_LegacyAlter as ::core::ffi::c_ulonglong)
                                as ::core::ffi::c_int;
                            (*db).flags
                                |= SQLITE_LegacyAlter as ::core::ffi::c_ulonglong;
                            pVtab_5 = (*(*pOp).p4.pVtab).pVtab;
                            pName = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            rc = sqlite3VdbeChangeEncoding(pName, SQLITE_UTF8);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            rc = (*(*pVtab_5).pModule)
                                .xRename
                                .expect("non-null function pointer")(pVtab_5, (*pName).z);
                            if isLegacy == 0 as ::core::ffi::c_int {
                                (*db).flags
                                    &= !(SQLITE_LegacyAlter as u64_0)
                                        as ::core::ffi::c_ulonglong;
                            }
                            sqlite3VtabImportErrmsg(p, pVtab_5);
                            (*p).set_expired(0 as bft as bft);
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_VUpdate => {
                            let mut pVtab_6: *mut sqlite3_vtab = ::core::ptr::null_mut::<
                                sqlite3_vtab,
                            >();
                            let mut pModule_5: *const sqlite3_module = ::core::ptr::null::<
                                sqlite3_module,
                            >();
                            let mut nArg_0: ::core::ffi::c_int = 0;
                            let mut i_6: ::core::ffi::c_int = 0;
                            let mut rowid_0: sqlite_int64 = 0 as sqlite_int64;
                            let mut apArg_0: *mut *mut Mem = ::core::ptr::null_mut::<
                                *mut Mem,
                            >();
                            let mut pX_0: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            if (*db).mallocFailed != 0 {
                                c2rust_current_block = 9955115899106739073;
                                break;
                            }
                            pVtab_6 = (*(*pOp).p4.pVtab).pVtab;
                            if pVtab_6.is_null() || (*pVtab_6).pModule.is_null() {
                                rc = SQLITE_LOCKED;
                                c2rust_current_block = 8086965459351668542;
                                break;
                            } else {
                                pModule_5 = (*pVtab_6).pModule;
                                nArg_0 = (*pOp).p2;
                                if (*pModule_5).xUpdate.is_some() {
                                    let mut vtabOnConflict: u8_0 = (*db).vtabOnConflict;
                                    apArg_0 = (*p).apArg;
                                    pX_0 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                                    i_6 = 0 as ::core::ffi::c_int;
                                    while i_6 < nArg_0 {
                                        let ref mut c2rust_fresh17 = *apArg_0.offset(i_6 as isize);
                                        *c2rust_fresh17 = pX_0;
                                        pX_0 = pX_0.offset(1);
                                        i_6 += 1;
                                    }
                                    (*db).vtabOnConflict = (*pOp).p5 as u8_0;
                                    rc = (*pModule_5)
                                        .xUpdate
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        pVtab_6,
                                        nArg_0,
                                        apArg_0 as *mut *mut sqlite3_value,
                                        &raw mut rowid_0,
                                    );
                                    (*db).vtabOnConflict = vtabOnConflict;
                                    sqlite3VtabImportErrmsg(p, pVtab_6);
                                    if rc == SQLITE_OK && (*pOp).p1 != 0 {
                                        (*db).lastRowid = rowid_0 as i64_0;
                                    }
                                    if rc & 0xff as ::core::ffi::c_int == SQLITE_CONSTRAINT
                                        && (*(*pOp).p4.pVtab).bConstraint as ::core::ffi::c_int != 0
                                    {
                                        if (*pOp).p5 as ::core::ffi::c_int == OE_Ignore {
                                            rc = SQLITE_OK;
                                        } else {
                                            (*p).errorAction = (if (*pOp).p5 as ::core::ffi::c_int
                                                == OE_Replace
                                            {
                                                OE_Abort
                                            } else {
                                                (*pOp).p5 as ::core::ffi::c_int
                                            }) as u8_0;
                                        }
                                    } else {
                                        (*p).nChange += 1;
                                    }
                                    if rc != 0 {
                                        c2rust_current_block = 8086965459351668542;
                                        break;
                                    }
                                }
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Pagecount => {
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            (*pOut).u.i = sqlite3BtreeLastPage(
                                (*(*db).aDb.offset((*pOp).p1 as isize)).pBt,
                            ) as i64_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_MaxPgcnt => {
                            let mut newMax: ::core::ffi::c_uint = 0;
                            let mut pBt_2: *mut Btree = ::core::ptr::null_mut::<Btree>();
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            pBt_2 = (*(*db).aDb.offset((*pOp).p1 as isize)).pBt;
                            newMax = 0 as ::core::ffi::c_uint;
                            if (*pOp).p3 != 0 {
                                newMax = sqlite3BtreeLastPage(pBt_2) as ::core::ffi::c_uint;
                                if newMax < (*pOp).p3 as ::core::ffi::c_uint {
                                    newMax = (*pOp).p3 as ::core::ffi::c_uint;
                                }
                            }
                            (*pOut).u.i = sqlite3BtreeMaxPageCount(pBt_2, newMax as Pgno)
                                as i64_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_PureFunc => {
                            c2rust_current_block = 14375524894201648466;
                        }
                        OP_Function => {
                            c2rust_current_block = 14375524894201648466;
                        }
                        OP_ClrSubtype => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                & !MEM_Subtype) as u16_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_GetSubtype => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pOut = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Subtype != 0 {
                                sqlite3VdbeMemSetInt64(pOut, (*pIn1).eSubtype as i64_0);
                            } else {
                                sqlite3VdbeMemSetNull(pOut);
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_SetSubtype => {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pOut = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            if (*pIn1).flags as ::core::ffi::c_int & MEM_Null != 0 {
                                (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                    & !MEM_Subtype) as u16_0;
                            } else {
                                (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                    | MEM_Subtype) as u16_0;
                                (*pOut).eSubtype = ((*pIn1).u.i as ::core::ffi::c_longlong
                                    & 0xff as ::core::ffi::c_longlong) as u8_0;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_FilterAdd => {
                            let mut h: u64_0 = 0;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            h = filterHash(aMem, pOp);
                            h = (h as ::core::ffi::c_ulonglong)
                                .wrapping_rem(
                                    ((*pIn1).n * 8 as ::core::ffi::c_int)
                                        as ::core::ffi::c_ulonglong,
                                ) as u64_0 as u64_0;
                            let ref mut c2rust_fresh19 = *(*pIn1)
                                .z
                                .offset(
                                    (h as ::core::ffi::c_ulonglong)
                                        .wrapping_div(8 as ::core::ffi::c_ulonglong) as isize,
                                );
                            *c2rust_fresh19 = (*c2rust_fresh19 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int)
                                    << (h as ::core::ffi::c_ulonglong
                                        & 7 as ::core::ffi::c_ulonglong)) as ::core::ffi::c_char;
                            c2rust_current_block = 5783071609795492627;
                        }
                        OP_Filter => {
                            let mut h_0: u64_0 = 0;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            h_0 = filterHash(aMem, pOp);
                            h_0 = (h_0 as ::core::ffi::c_ulonglong)
                                .wrapping_rem(
                                    ((*pIn1).n * 8 as ::core::ffi::c_int)
                                        as ::core::ffi::c_ulonglong,
                                ) as u64_0 as u64_0;
                            if *(*pIn1)
                                .z
                                .offset(
                                    (h_0 as ::core::ffi::c_ulonglong)
                                        .wrapping_div(8 as ::core::ffi::c_ulonglong) as isize,
                                ) as ::core::ffi::c_int
                                & (1 as ::core::ffi::c_int)
                                    << (h_0 as ::core::ffi::c_ulonglong
                                        & 7 as ::core::ffi::c_ulonglong) == 0 as ::core::ffi::c_int
                            {
                                (*p).aCounter[SQLITE_STMTSTATUS_FILTER_HIT as usize] = (*p)
                                    .aCounter[SQLITE_STMTSTATUS_FILTER_HIT as usize]
                                    .wrapping_add(1);
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                (*p).aCounter[SQLITE_STMTSTATUS_FILTER_MISS as usize] = (*p)
                                    .aCounter[SQLITE_STMTSTATUS_FILTER_MISS as usize]
                                    .wrapping_add(1);
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        OP_Trace | OP_Init => {
                            let mut i_8: ::core::ffi::c_int = 0;
                            let mut zTrace: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                ::core::ffi::c_char,
                            >();
                            if (*db).mTrace as ::core::ffi::c_int
                                & (SQLITE_TRACE_STMT | SQLITE_TRACE_LEGACY)
                                != 0 as ::core::ffi::c_int
                                && (*p).minWriteFileFormat as ::core::ffi::c_int
                                    != 254 as ::core::ffi::c_int
                                && {
                                    zTrace = (if !(*pOp).p4.z.is_null() {
                                        (*pOp).p4.z
                                    } else {
                                        (*p).zSql
                                    });
                                    !zTrace.is_null()
                                }
                            {
                                if (*db).nVdbeExec > 1 as ::core::ffi::c_int {
                                    let mut z_0: *mut ::core::ffi::c_char = sqlite3MPrintf(
                                        db,
                                        b"-- %s\0".as_ptr() as *const ::core::ffi::c_char,
                                        zTrace,
                                    );
                                    (*db)
                                        .trace
                                        .xV2
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        SQLITE_TRACE_STMT as u32_0,
                                        (*db).pTraceArg,
                                        p as *mut ::core::ffi::c_void,
                                        z_0 as *mut ::core::ffi::c_void,
                                    );
                                    sqlite3DbFree(db, z_0 as *mut ::core::ffi::c_void);
                                } else {
                                    (*db)
                                        .trace
                                        .xV2
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        SQLITE_TRACE_STMT as u32_0,
                                        (*db).pTraceArg,
                                        p as *mut ::core::ffi::c_void,
                                        zTrace as *mut ::core::ffi::c_void,
                                    );
                                }
                            }
                            if (*pOp).p1 >= sqlite3Config.iOnceResetThreshold {
                                if (*pOp).opcode as ::core::ffi::c_int == OP_Trace {
                                    c2rust_current_block = 5783071609795492627;
                                } else {
                                    i_8 = 1 as ::core::ffi::c_int;
                                    while i_8 < (*p).nOp {
                                        if (*(*p).aOp.offset(i_8 as isize)).opcode
                                            as ::core::ffi::c_int == OP_Once
                                        {
                                            (*(*p).aOp.offset(i_8 as isize)).p1 = 0
                                                as ::core::ffi::c_int;
                                        }
                                        i_8 += 1;
                                    }
                                    (*pOp).p1 = 0 as ::core::ffi::c_int;
                                    c2rust_current_block = 8618027029035688756;
                                }
                            } else {
                                c2rust_current_block = 8618027029035688756;
                            }
                            match c2rust_current_block {
                                5783071609795492627 => {}
                                _ => {
                                    (*pOp).p1 += 1;
                                    (*p).aCounter[SQLITE_STMTSTATUS_RUN as usize] = (*p)
                                        .aCounter[SQLITE_STMTSTATUS_RUN as usize]
                                        .wrapping_add(1);
                                    c2rust_current_block = 17233182392562552756;
                                }
                            }
                        }
                        OP_Permutation | _ => {
                            c2rust_current_block = 5783071609795492627;
                        }
                    }
                    match c2rust_current_block {
                        7419121793134201633 => {
                            let mut pFrame: *mut VdbeFrame = ::core::ptr::null_mut::<
                                VdbeFrame,
                            >();
                            let mut pcx: ::core::ffi::c_int = 0;
                            if !(*p).pFrame.is_null() && (*pOp).p1 == SQLITE_OK {
                                pFrame = (*p).pFrame;
                                (*p).pFrame = (*pFrame).pParent;
                                (*p).nFrame -= 1;
                                sqlite3VdbeSetChanges(db, (*p).nChange);
                                pcx = sqlite3VdbeFrameRestore(pFrame);
                                if (*pOp).p2 == OE_Ignore {
                                    pcx = (*(*p).aOp.offset(pcx as isize)).p2
                                        - 1 as ::core::ffi::c_int;
                                }
                                aOp = (*p).aOp;
                                aMem = (*p).aMem;
                                pOp = aOp.offset(pcx as isize) as *mut Op;
                            } else {
                                (*p).rc = (*pOp).p1;
                                (*p).errorAction = (*pOp).p2 as u8_0;
                                if (*p).rc != 0 {
                                    if (*pOp).p3 > 0 as ::core::ffi::c_int
                                        && (*pOp).p4type as ::core::ffi::c_int == P4_NOTUSED
                                    {
                                        let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
                                            ::core::ffi::c_char,
                                        >();
                                        zErr = sqlite3ValueText(
                                            aMem.offset((*pOp).p3 as isize) as *mut sqlite3_value,
                                            SQLITE_UTF8 as u8_0,
                                        ) as *const ::core::ffi::c_char;
                                        sqlite3VdbeError(
                                            p,
                                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                            zErr,
                                        );
                                    } else if (*pOp).p5 != 0 {
                                        static mut azType: [*const ::core::ffi::c_char; 4] = [
                                            b"NOT NULL\0".as_ptr() as *const ::core::ffi::c_char,
                                            b"UNIQUE\0".as_ptr() as *const ::core::ffi::c_char,
                                            b"CHECK\0".as_ptr() as *const ::core::ffi::c_char,
                                            b"FOREIGN KEY\0".as_ptr() as *const ::core::ffi::c_char,
                                        ];
                                        sqlite3VdbeError(
                                            p,
                                            b"%s constraint failed\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            azType[((*pOp).p5 as ::core::ffi::c_int
                                                - 1 as ::core::ffi::c_int) as usize],
                                        );
                                        if !(*pOp).p4.z.is_null() {
                                            (*p).zErrMsg = sqlite3MPrintf(
                                                db,
                                                b"%z: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                                (*p).zErrMsg,
                                                (*pOp).p4.z,
                                            );
                                        }
                                    } else {
                                        sqlite3VdbeError(
                                            p,
                                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                            (*pOp).p4.z,
                                        );
                                    }
                                    sqlite3VdbeLogAbort(p, (*pOp).p1, pOp, aOp);
                                }
                                rc = sqlite3VdbeHalt(p);
                                if rc == SQLITE_BUSY {
                                    (*p).rc = SQLITE_BUSY;
                                } else {
                                    rc = if (*p).rc != 0 { SQLITE_ERROR } else { SQLITE_DONE };
                                }
                                c2rust_current_block = 4288082686256521985;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        14375524894201648466 => {
                            let mut i_7: ::core::ffi::c_int = 0;
                            let mut pCtx_1: *mut sqlite3_context = ::core::ptr::null_mut::<
                                sqlite3_context,
                            >();
                            pCtx_1 = (*pOp).p4.pCtx;
                            pOut = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            if (*pCtx_1).pOut != pOut {
                                (*pCtx_1).pVdbe = p;
                                (*pCtx_1).pOut = pOut;
                                (*pCtx_1).enc = encoding;
                                i_7 = (*pCtx_1).argc as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int;
                                while i_7 >= 0 as ::core::ffi::c_int {
                                    let ref mut c2rust_fresh18 = *(&raw mut (*pCtx_1).argv
                                        as *mut *mut sqlite3_value)
                                        .offset(i_7 as isize);
                                    *c2rust_fresh18 = aMem.offset(((*pOp).p2 + i_7) as isize)
                                        as *mut Mem as *mut sqlite3_value;
                                    i_7 -= 1;
                                }
                            }
                            (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                & !(MEM_TypeMask | MEM_Zero) | 0x1 as ::core::ffi::c_int)
                                as u16_0;
                            Some(
                                    (*(*pCtx_1).pFunc)
                                        .xSFunc
                                        .expect("non-null function pointer"),
                                )
                                .expect(
                                    "non-null function pointer",
                                )(
                                pCtx_1,
                                (*pCtx_1).argc as ::core::ffi::c_int,
                                &raw mut (*pCtx_1).argv as *mut *mut sqlite3_value,
                            );
                            if (*pCtx_1).isError != 0 {
                                if (*pCtx_1).isError > 0 as ::core::ffi::c_int {
                                    sqlite3VdbeError(
                                        p,
                                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                        sqlite3_value_text(pOut as *mut sqlite3_value),
                                    );
                                    rc = (*pCtx_1).isError;
                                }
                                sqlite3VdbeDeleteAuxData(
                                    db,
                                    &raw mut (*p).pAuxData,
                                    (*pCtx_1).iOp,
                                    (*pOp).p1,
                                );
                                (*pCtx_1).isError = 0 as ::core::ffi::c_int;
                                if rc != 0 {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                c2rust_current_block = 5783071609795492627;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        12597090803267581175 => {
                            let mut i_3: ::core::ffi::c_int = 0;
                            let mut pCtx_0: *mut sqlite3_context = ::core::ptr::null_mut::<
                                sqlite3_context,
                            >();
                            let mut pMem_1: *mut Mem = ::core::ptr::null_mut::<Mem>();
                            pCtx_0 = (*pOp).p4.pCtx;
                            pMem_1 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            if (*pCtx_0).pMem != pMem_1 {
                                (*pCtx_0).pMem = pMem_1;
                                i_3 = (*pCtx_0).argc as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int;
                                while i_3 >= 0 as ::core::ffi::c_int {
                                    let ref mut c2rust_fresh15 = *(&raw mut (*pCtx_0).argv
                                        as *mut *mut sqlite3_value)
                                        .offset(i_3 as isize);
                                    *c2rust_fresh15 = aMem.offset(((*pOp).p2 + i_3) as isize)
                                        as *mut Mem as *mut sqlite3_value;
                                    i_3 -= 1;
                                }
                            }
                            (*pMem_1).n += 1;
                            if (*pOp).p1 != 0 {
                                (*(*pCtx_0).pFunc)
                                    .xInverse
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    pCtx_0,
                                    (*pCtx_0).argc as ::core::ffi::c_int,
                                    &raw mut (*pCtx_0).argv as *mut *mut sqlite3_value,
                                );
                            } else {
                                (*(*pCtx_0).pFunc)
                                    .xSFunc
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    pCtx_0,
                                    (*pCtx_0).argc as ::core::ffi::c_int,
                                    &raw mut (*pCtx_0).argv as *mut *mut sqlite3_value,
                                );
                            }
                            if (*pCtx_0).isError != 0 {
                                if (*pCtx_0).isError > 0 as ::core::ffi::c_int {
                                    sqlite3VdbeError(
                                        p,
                                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                        sqlite3_value_text((*pCtx_0).pOut as *mut sqlite3_value),
                                    );
                                    rc = (*pCtx_0).isError;
                                }
                                if (*pCtx_0).skipFlag != 0 {
                                    i_3 = (*pOp.offset(-1 as ::core::ffi::c_int as isize)).p1;
                                    if i_3 != 0 {
                                        sqlite3VdbeMemSetInt64(
                                            aMem.offset(i_3 as isize) as *mut Mem,
                                            1 as i64_0,
                                        );
                                    }
                                    (*pCtx_0).skipFlag = 0 as u8_0;
                                }
                                sqlite3VdbeMemRelease((*pCtx_0).pOut);
                                (*(*pCtx_0).pOut).flags = MEM_Null as u16_0;
                                (*pCtx_0).isError = 0 as ::core::ffi::c_int;
                                if rc != 0 {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                c2rust_current_block = 5783071609795492627;
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        17265526906670415574 => {
                            let mut pC_25: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pTabCur: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut rowid: i64_0 = 0;
                            pC_25 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            rc = sqlite3VdbeCursorRestore(pC_25);
                            if rc != SQLITE_OK {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            if (*pC_25).nullRow == 0 {
                                rowid = 0 as i64_0;
                                rc = sqlite3VdbeIdxRowid(
                                    db,
                                    (*pC_25).uc.pCursor,
                                    &raw mut rowid,
                                );
                                if rc != SQLITE_OK {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                if (*pOp).opcode as ::core::ffi::c_int == OP_DeferredSeek {
                                    pTabCur = *(*p).apCsr.offset((*pOp).p3 as isize);
                                    (*pTabCur).nullRow = 0 as u8_0;
                                    (*pTabCur).movetoTarget = rowid;
                                    (*pTabCur).deferredMoveto = 1 as u8_0;
                                    (*pTabCur).cacheStatus = CACHE_STALE as u32_0;
                                    (*pTabCur).ub.aAltMap = (*pOp).p4.ai;
                                    (*pTabCur).pAltCursor = pC_25;
                                } else {
                                    pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                                    (*pOut).u.i = rowid;
                                }
                            } else {
                                sqlite3VdbeMemSetNull(
                                    aMem.offset((*pOp).p2 as isize) as *mut Mem,
                                );
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        2917012134985143631 => {
                            (*pC_21).cacheStatus = CACHE_STALE as u32_0;
                            if rc == SQLITE_OK {
                                (*pC_21).nullRow = 0 as u8_0;
                                (*p).aCounter[(*pOp).p5 as usize] = (*p)
                                    .aCounter[(*pOp).p5 as usize]
                                    .wrapping_add(1);
                                c2rust_current_block = 13898972957494642055;
                            } else {
                                if rc != SQLITE_DONE {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                rc = SQLITE_OK;
                                (*pC_21).nullRow = 1 as u8_0;
                                c2rust_current_block = 6498258645234546759;
                            }
                        }
                        15206327760975505511 => {
                            let mut pC_19: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pCrsr_5: *mut BtCursor = ::core::ptr::null_mut::<
                                BtCursor,
                            >();
                            let mut res_7: ::core::ffi::c_int = 0;
                            pC_19 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            res_7 = 1 as ::core::ffi::c_int;
                            if (*pC_19).eCurType as ::core::ffi::c_int == CURTYPE_SORTER
                            {
                                rc = sqlite3VdbeSorterRewind(pC_19, &raw mut res_7);
                            } else {
                                pCrsr_5 = (*pC_19).uc.pCursor;
                                rc = sqlite3BtreeFirst(pCrsr_5, &raw mut res_7);
                                (*pC_19).deferredMoveto = 0 as u8_0;
                                (*pC_19).cacheStatus = CACHE_STALE as u32_0;
                            }
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            (*pC_19).nullRow = res_7 as u8_0;
                            if (*pOp).p2 > 0 as ::core::ffi::c_int {
                                if res_7 != 0 {
                                    c2rust_current_block = 17233182392562552756;
                                } else {
                                    c2rust_current_block = 5783071609795492627;
                                }
                            } else {
                                c2rust_current_block = 5783071609795492627;
                            }
                        }
                        12752338440521374055 => {
                            let mut pC_17: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pCrsr_3: *mut BtCursor = ::core::ptr::null_mut::<
                                BtCursor,
                            >();
                            let mut res_5: ::core::ffi::c_int = 0;
                            pC_17 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pCrsr_3 = (*pC_17).uc.pCursor;
                            res_5 = 0 as ::core::ffi::c_int;
                            if (*pOp).opcode as ::core::ffi::c_int == OP_SeekEnd {
                                (*pC_17).seekResult = -1 as ::core::ffi::c_int;
                                if sqlite3BtreeCursorIsValidNN(pCrsr_3) != 0 {
                                    c2rust_current_block = 5783071609795492627;
                                } else {
                                    c2rust_current_block = 278792255694607253;
                                }
                            } else {
                                c2rust_current_block = 278792255694607253;
                            }
                            match c2rust_current_block {
                                5783071609795492627 => {}
                                _ => {
                                    rc = sqlite3BtreeLast(pCrsr_3, &raw mut res_5);
                                    (*pC_17).nullRow = res_5 as u8_0;
                                    (*pC_17).deferredMoveto = 0 as u8_0;
                                    (*pC_17).cacheStatus = CACHE_STALE as u32_0;
                                    if rc != 0 {
                                        c2rust_current_block = 8086965459351668542;
                                        break;
                                    }
                                    if (*pOp).p2 > 0 as ::core::ffi::c_int {
                                        if res_5 != 0 {
                                            c2rust_current_block = 17233182392562552756;
                                        } else {
                                            c2rust_current_block = 5783071609795492627;
                                        }
                                    } else {
                                        c2rust_current_block = 5783071609795492627;
                                    }
                                }
                            }
                        }
                        14309557416411021540 => {
                            pIn3 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            iKey_0 = (*pIn3).u.i as u64_0;
                            c2rust_current_block = 11230649915035842278;
                        }
                        12138815981076939541 => {
                            let mut pCx_0: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pKeyInfo_1: *mut KeyInfo = ::core::ptr::null_mut::<
                                KeyInfo,
                            >();
                            static mut vfsFlags: ::core::ffi::c_int = SQLITE_OPEN_READWRITE
                                | SQLITE_OPEN_CREATE | SQLITE_OPEN_EXCLUSIVE
                                | SQLITE_OPEN_DELETEONCLOSE | SQLITE_OPEN_TRANSIENT_DB;
                            if (*pOp).p3 > 0 as ::core::ffi::c_int {
                                (*aMem.offset((*pOp).p3 as isize)).n = 0
                                    as ::core::ffi::c_int;
                                let ref mut c2rust_fresh8 = (*aMem
                                    .offset((*pOp).p3 as isize))
                                    .z;
                                *c2rust_fresh8 = b"\0".as_ptr()
                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                            }
                            pCx_0 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            if !pCx_0.is_null() && (*pCx_0).noReuse() == 0
                                && (*pOp).p2 <= (*pCx_0).nField as ::core::ffi::c_int
                            {
                                (*pCx_0).seqCount = 0 as i64_0;
                                (*pCx_0).cacheStatus = CACHE_STALE as u32_0;
                                rc = sqlite3BtreeClearTable(
                                    (*pCx_0).ub.pBtx,
                                    (*pCx_0).pgnoRoot as ::core::ffi::c_int,
                                    ::core::ptr::null_mut::<i64_0>(),
                                );
                            } else {
                                pCx_0 = allocateCursor(
                                    p,
                                    (*pOp).p1,
                                    (*pOp).p2,
                                    CURTYPE_BTREE as u8_0,
                                );
                                if pCx_0.is_null() {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                }
                                (*pCx_0).set_isEphemeral(1 as Bool as Bool);
                                rc = sqlite3BtreeOpen(
                                    (*db).pVfs,
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    db,
                                    &raw mut (*pCx_0).ub.pBtx,
                                    BTREE_OMIT_JOURNAL | BTREE_SINGLE
                                        | (*pOp).p5 as ::core::ffi::c_int,
                                    vfsFlags,
                                );
                                if rc == SQLITE_OK {
                                    rc = sqlite3BtreeBeginTrans(
                                        (*pCx_0).ub.pBtx,
                                        1 as ::core::ffi::c_int,
                                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                    );
                                    if rc == SQLITE_OK {
                                        pKeyInfo_1 = (*pOp).p4.pKeyInfo;
                                        (*pCx_0).pKeyInfo = pKeyInfo_1;
                                        if !(*pCx_0).pKeyInfo.is_null() {
                                            rc = sqlite3BtreeCreateTable(
                                                (*pCx_0).ub.pBtx,
                                                &raw mut (*pCx_0).pgnoRoot,
                                                BTREE_BLOBKEY | (*pOp).p5 as ::core::ffi::c_int,
                                            );
                                            if rc == SQLITE_OK {
                                                rc = sqlite3BtreeCursor(
                                                    (*pCx_0).ub.pBtx,
                                                    (*pCx_0).pgnoRoot,
                                                    BTREE_WRCSR,
                                                    pKeyInfo_1 as *mut KeyInfo,
                                                    (*pCx_0).uc.pCursor,
                                                );
                                            }
                                            (*pCx_0).isTable = 0 as u8_0;
                                        } else {
                                            (*pCx_0).pgnoRoot = SCHEMA_ROOT as Pgno;
                                            rc = sqlite3BtreeCursor(
                                                (*pCx_0).ub.pBtx,
                                                SCHEMA_ROOT as Pgno,
                                                BTREE_WRCSR,
                                                ::core::ptr::null_mut::<KeyInfo>(),
                                                (*pCx_0).uc.pCursor,
                                            );
                                            (*pCx_0).isTable = 1 as u8_0;
                                        }
                                    }
                                    (*pCx_0)
                                        .set_isOrdered(
                                            ((*pOp).p5 as ::core::ffi::c_int != BTREE_UNORDERED)
                                                as ::core::ffi::c_int as Bool as Bool,
                                        );
                                    if rc != 0 {
                                        sqlite3BtreeClose((*pCx_0).ub.pBtx);
                                        let ref mut c2rust_fresh9 = *(*p)
                                            .apCsr
                                            .offset((*pOp).p1 as isize);
                                        *c2rust_fresh9 = ::core::ptr::null_mut::<VdbeCursor>();
                                    }
                                }
                            }
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            (*pCx_0).nullRow = 1 as u8_0;
                            c2rust_current_block = 5783071609795492627;
                        }
                        16007193389016785609 => {
                            if (*p).expired() as ::core::ffi::c_int
                                == 1 as ::core::ffi::c_int
                            {
                                rc = SQLITE_ABORT_ROLLBACK;
                                c2rust_current_block = 8086965459351668542;
                                break;
                            } else {
                                nField_0 = 0 as ::core::ffi::c_int;
                                pKeyInfo_0 = ::core::ptr::null_mut::<KeyInfo>();
                                p2_2 = (*pOp).p2 as u32_0;
                                iDb_0 = (*pOp).p3;
                                pDb_1 = (*db).aDb.offset(iDb_0 as isize) as *mut Db;
                                pX = (*pDb_1).pBt;
                                if (*pOp).opcode as ::core::ffi::c_int == OP_OpenWrite {
                                    wrFlag = BTREE_WRCSR
                                        | (*pOp).p5 as ::core::ffi::c_int & OPFLAG_FORDELETE;
                                    if ((*(*pDb_1).pSchema).file_format as ::core::ffi::c_int)
                                        < (*p).minWriteFileFormat as ::core::ffi::c_int
                                    {
                                        (*p).minWriteFileFormat = (*(*pDb_1).pSchema).file_format;
                                    }
                                    if (*pOp).p5 as ::core::ffi::c_int & OPFLAG_P2ISREG != 0 {
                                        pIn2 = aMem.offset(p2_2 as isize) as *mut Mem;
                                        sqlite3VdbeMemIntegerify(pIn2);
                                        p2_2 = (*pIn2).u.i as ::core::ffi::c_int as u32_0;
                                    }
                                } else {
                                    wrFlag = 0 as ::core::ffi::c_int;
                                }
                                if (*pOp).p4type as ::core::ffi::c_int == P4_KEYINFO {
                                    pKeyInfo_0 = (*pOp).p4.pKeyInfo;
                                    nField_0 = (*pKeyInfo_0).nAllField as ::core::ffi::c_int;
                                } else if (*pOp).p4type as ::core::ffi::c_int == P4_INT32 {
                                    nField_0 = (*pOp).p4.i;
                                }
                                pCur = allocateCursor(
                                    p,
                                    (*pOp).p1,
                                    nField_0,
                                    CURTYPE_BTREE as u8_0,
                                );
                                if pCur.is_null() {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                }
                                (*pCur).iDb = iDb_0 as i8_0;
                                (*pCur).nullRow = 1 as u8_0;
                                (*pCur).set_isOrdered(1 as Bool as Bool);
                                (*pCur).pgnoRoot = p2_2 as Pgno;
                                rc = sqlite3BtreeCursor(
                                    pX,
                                    p2_2 as Pgno,
                                    wrFlag,
                                    pKeyInfo_0 as *mut KeyInfo,
                                    (*pCur).uc.pCursor,
                                );
                                (*pCur).pKeyInfo = pKeyInfo_0;
                                (*pCur).isTable = ((*pOp).p4type as ::core::ffi::c_int
                                    != P4_KEYINFO) as ::core::ffi::c_int as u8_0;
                            }
                            c2rust_current_block = 7045491780073535011;
                        }
                        3325916876629046855 => {
                            let mut v1: ::core::ffi::c_int = 0;
                            let mut v2: ::core::ffi::c_int = 0;
                            v1 = sqlite3VdbeBooleanValue(
                                aMem.offset((*pOp).p1 as isize) as *mut Mem,
                                2 as ::core::ffi::c_int,
                            );
                            v2 = sqlite3VdbeBooleanValue(
                                aMem.offset((*pOp).p2 as isize) as *mut Mem,
                                2 as ::core::ffi::c_int,
                            );
                            if (*pOp).opcode as ::core::ffi::c_int == OP_And {
                                static mut and_logic: [::core::ffi::c_uchar; 9] = [
                                    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                ];
                                v1 = and_logic[(v1 * 3 as ::core::ffi::c_int + v2) as usize]
                                    as ::core::ffi::c_int;
                            } else {
                                static mut or_logic: [::core::ffi::c_uchar; 9] = [
                                    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                ];
                                v1 = or_logic[(v1 * 3 as ::core::ffi::c_int + v2) as usize]
                                    as ::core::ffi::c_int;
                            }
                            pOut = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            if v1 == 2 as ::core::ffi::c_int {
                                (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                    & !(MEM_TypeMask | MEM_Zero) | 0x1 as ::core::ffi::c_int)
                                    as u16_0;
                            } else {
                                (*pOut).u.i = v1 as i64_0;
                                (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                    & !(MEM_TypeMask | MEM_Zero) | 0x4 as ::core::ffi::c_int)
                                    as u16_0;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        15460309861373144675 => {
                            pOut = out2Prerelease(p, pOp as *mut VdbeOp);
                            (*pOut).flags = (MEM_Str | MEM_Static | MEM_Term) as u16_0;
                            (*pOut).z = (*pOp).p4.z;
                            (*pOut).n = (*pOp).p1;
                            (*pOut).enc = encoding;
                            if (*pOp).p3 > 0 as ::core::ffi::c_int {
                                pIn3 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                                if (*pIn3).u.i == (*pOp).p5 as ::core::ffi::c_longlong {
                                    (*pOut).flags = (MEM_Blob | MEM_Static | MEM_Term) as u16_0;
                                }
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        8767272321317040172 => {
                            c2rust_current_block = 16624893707949258064;
                        }
                        4315148737281477633 => {
                            c2rust_current_block = 4688408507276652532;
                        }
                        4401829632234223291 => {
                            c2rust_current_block = 2535740953232313464;
                        }
                        11769942314722711362 => {
                            c2rust_current_block = 4606120822943820531;
                        }
                        10118086935549511662 => {
                            c2rust_current_block = 12239147378838813631;
                        }
                        13858715045951221004 => {
                            c2rust_current_block = 11000743977270914936;
                        }
                        _ => {}
                    }
                    match c2rust_current_block {
                        11230649915035842278 => {
                            pC_8 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            pCrsr_1 = (*pC_8).uc.pCursor;
                            res_2 = 0 as ::core::ffi::c_int;
                            rc = sqlite3BtreeTableMoveto(
                                pCrsr_1,
                                iKey_0 as i64_0,
                                0 as ::core::ffi::c_int,
                                &raw mut res_2,
                            );
                            (*pC_8).movetoTarget = iKey_0 as i64_0;
                            (*pC_8).nullRow = 0 as u8_0;
                            (*pC_8).cacheStatus = CACHE_STALE as u32_0;
                            (*pC_8).deferredMoveto = 0 as u8_0;
                            (*pC_8).seekResult = res_2;
                            if res_2 != 0 as ::core::ffi::c_int {
                                if (*pOp).p2 == 0 as ::core::ffi::c_int {
                                    rc = sqlite3CorruptError(5545 as ::core::ffi::c_int);
                                    c2rust_current_block = 18379064606459669838;
                                } else {
                                    c2rust_current_block = 17233182392562552756;
                                }
                            } else {
                                c2rust_current_block = 18379064606459669838;
                            }
                            match c2rust_current_block {
                                17233182392562552756 => {}
                                _ => {
                                    if rc != 0 {
                                        c2rust_current_block = 8086965459351668542;
                                        break;
                                    }
                                    c2rust_current_block = 5783071609795492627;
                                }
                            }
                        }
                        7045491780073535011 => {
                            sqlite3BtreeCursorHintFlags(
                                (*pCur).uc.pCursor,
                                ((*pOp).p5 as ::core::ffi::c_int
                                    & (OPFLAG_BULKCSR | OPFLAG_SEEKEQ)) as ::core::ffi::c_uint,
                            );
                            if rc != 0 {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        16624893707949258064 => {
                            c2rust_current_block = 7300422940758305934;
                        }
                        4688408507276652532 => {
                            c2rust_current_block = 13567963862951993727;
                        }
                        2535740953232313464 => {
                            c2rust_current_block = 7659955564255721022;
                        }
                        4606120822943820531 => {
                            c2rust_current_block = 3833908538611729861;
                        }
                        12239147378838813631 => {
                            c2rust_current_block = 682219075816228721;
                        }
                        11000743977270914936 => {
                            c2rust_current_block = 6548257557770401308;
                        }
                        13898972957494642055 => {
                            pOp = aOp
                                .offset(((*pOp).p2 - 1 as ::core::ffi::c_int) as isize)
                                as *mut Op;
                            c2rust_current_block = 6498258645234546759;
                        }
                        _ => {}
                    }
                    match c2rust_current_block {
                        6498258645234546759 => {
                            if (*db).u1.isInterrupted != 0
                            {
                                c2rust_current_block = 4286470523181410568;
                                break;
                            }
                            while nVmStep >= nProgressLimit && (*db).xProgress.is_some()
                            {
                                nProgressLimit = (nProgressLimit
                                    as ::core::ffi::c_ulonglong)
                                    .wrapping_add(
                                        (*db).nProgressOps as ::core::ffi::c_ulonglong,
                                    ) as u64_0 as u64_0;
                                if !((*db)
                                    .xProgress
                                    .expect("non-null function pointer")((*db).pProgressArg)
                                    != 0)
                                {
                                    continue;
                                }
                                nProgressLimit = LARGEST_UINT64;
                                rc = SQLITE_INTERRUPT;
                                c2rust_current_block = 8086965459351668542;
                                break 's_109;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        6548257557770401308 => {
                            let mut pC_27: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut res_10: ::core::ffi::c_int = 0;
                            let mut r_3: UnpackedRecord = UnpackedRecord {
                                pKeyInfo: ::core::ptr::null_mut::<KeyInfo>(),
                                aMem: ::core::ptr::null_mut::<Mem>(),
                                u: C2Rust_Unnamed_24 {
                                    z: ::core::ptr::null_mut::<::core::ffi::c_char>()

                                },
                                n: 0,
                                nField: 0,
                                default_rc: 0,
                                errCode: 0,
                                r1: 0,
                                r2: 0,
                                eqSeen: 0

                            };
                            pC_27 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            r_3.pKeyInfo = (*pC_27).pKeyInfo;
                            r_3.nField = (*pOp).p4.i as u16_0;
                            if ((*pOp).opcode as ::core::ffi::c_int) < OP_IdxLT {
                                r_3.default_rc = -1 as ::core::ffi::c_int as i8_0;
                            } else {
                                r_3.default_rc = 0 as i8_0;
                            }
                            r_3.aMem = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            let mut nCellKey: i64_0 = 0 as i64_0;
                            let mut pCur_1: *mut BtCursor = ::core::ptr::null_mut::<
                                BtCursor,
                            >();
                            let mut m: Mem = Mem {
                                u: MemValue { r: 0. },
                                z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                n: 0,
                                flags: 0,
                                enc: 0,
                                eSubtype: 0,
                                db: ::core::ptr::null_mut::<sqlite3>(),
                                szMalloc: 0,
                                uTemp: 0,
                                zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                xDel: None

                            };
                            pCur_1 = (*pC_27).uc.pCursor;
                            nCellKey = sqlite3BtreePayloadSize(pCur_1) as i64_0;
                            if nCellKey <= 0 as ::core::ffi::c_longlong
                                || nCellKey > 0x7fffffff as ::core::ffi::c_longlong
                            {
                                rc = sqlite3CorruptError(6868 as ::core::ffi::c_int);
                                c2rust_current_block = 8086965459351668542;
                                break;
                            } else {
                                sqlite3VdbeMemInit(&raw mut m, db, 0 as u16_0);
                                rc = sqlite3VdbeMemFromBtreeZeroOffset(
                                    pCur_1,
                                    nCellKey as u32_0,
                                    &raw mut m,
                                );
                                if rc != 0 {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                res_10 = sqlite3VdbeRecordCompareWithSkip(
                                    m.n,
                                    m.z as *const ::core::ffi::c_void,
                                    &raw mut r_3,
                                    0 as ::core::ffi::c_int,
                                );
                                sqlite3VdbeMemReleaseMalloc(&raw mut m);
                                if (*pOp).opcode as ::core::ffi::c_int
                                    & 1 as ::core::ffi::c_int
                                    == OP_IdxLT & 1 as ::core::ffi::c_int
                                {
                                    res_10 = -res_10;
                                } else {
                                    res_10 += 1;
                                }
                                if res_10 > 0 as ::core::ffi::c_int {
                                    c2rust_current_block = 17233182392562552756;
                                } else {
                                    c2rust_current_block = 5783071609795492627;
                                }
                            }
                        }
                        682219075816228721 => {
                            let mut alreadyExists: ::core::ffi::c_int = 0;
                            let mut ii_0: ::core::ffi::c_int = 0;
                            let mut pC_7: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut pIdxKey: *mut UnpackedRecord = ::core::ptr::null_mut::<
                                UnpackedRecord,
                            >();
                            let mut r_1: UnpackedRecord = UnpackedRecord {
                                pKeyInfo: ::core::ptr::null_mut::<KeyInfo>(),
                                aMem: ::core::ptr::null_mut::<Mem>(),
                                u: C2Rust_Unnamed_24 {
                                    z: ::core::ptr::null_mut::<::core::ffi::c_char>()

                                },
                                n: 0,
                                nField: 0,
                                default_rc: 0,
                                errCode: 0,
                                r1: 0,
                                r2: 0,
                                eqSeen: 0

                            };
                            pC_7 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            r_1.aMem = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            r_1.nField = (*pOp).p4.i as u16_0;
                            if r_1.nField as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                            {
                                r_1.pKeyInfo = (*pC_7).pKeyInfo;
                                r_1.default_rc = 0 as i8_0;
                                rc = sqlite3BtreeIndexMoveto(
                                    (*pC_7).uc.pCursor,
                                    &raw mut r_1,
                                    &raw mut (*pC_7).seekResult,
                                );
                            } else {
                                rc = if (*r_1.aMem).flags as ::core::ffi::c_int & MEM_Zero
                                    != 0
                                {
                                    sqlite3VdbeMemExpandBlob(r_1.aMem)
                                } else {
                                    0 as ::core::ffi::c_int
                                };
                                if rc != 0 {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                }
                                pIdxKey = sqlite3VdbeAllocUnpackedRecord((*pC_7).pKeyInfo);
                                if pIdxKey.is_null() {
                                    c2rust_current_block = 9955115899106739073;
                                    break;
                                }
                                sqlite3VdbeRecordUnpack(
                                    (*r_1.aMem).n,
                                    (*r_1.aMem).z as *const ::core::ffi::c_void,
                                    pIdxKey,
                                );
                                (*pIdxKey).default_rc = 0 as i8_0;
                                rc = sqlite3BtreeIndexMoveto(
                                    (*pC_7).uc.pCursor,
                                    pIdxKey,
                                    &raw mut (*pC_7).seekResult,
                                );
                                sqlite3DbFreeNN(db, pIdxKey as *mut ::core::ffi::c_void);
                            }
                            if rc != SQLITE_OK {
                                c2rust_current_block = 8086965459351668542;
                                break;
                            }
                            alreadyExists = ((*pC_7).seekResult
                                == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                            (*pC_7).nullRow = (1 as ::core::ffi::c_int - alreadyExists)
                                as u8_0;
                            (*pC_7).deferredMoveto = 0 as u8_0;
                            (*pC_7).cacheStatus = CACHE_STALE as u32_0;
                            if (*pOp).opcode as ::core::ffi::c_int == OP_Found {
                                if alreadyExists != 0 {
                                    c2rust_current_block = 17233182392562552756;
                                } else {
                                    c2rust_current_block = 5783071609795492627;
                                }
                            } else if alreadyExists == 0 {
                                c2rust_current_block = 17233182392562552756;
                            } else {
                                if (*pOp).opcode as ::core::ffi::c_int == OP_NoConflict {
                                    ii_0 = 0 as ::core::ffi::c_int;
                                    loop {
                                        if !(ii_0 < r_1.nField as ::core::ffi::c_int) {
                                            c2rust_current_block = 8371078721871921775;
                                            break;
                                        }
                                        if (*r_1.aMem.offset(ii_0 as isize)).flags
                                            as ::core::ffi::c_int & MEM_Null != 0
                                        {
                                            c2rust_current_block = 17233182392562552756;
                                            break;
                                        }
                                        ii_0 += 1;
                                    }
                                } else {
                                    c2rust_current_block = 8371078721871921775;
                                }
                                match c2rust_current_block {
                                    17233182392562552756 => {}
                                    _ => {
                                        if (*pOp).opcode as ::core::ffi::c_int == OP_IfNoHope {
                                            (*pC_7).seekHit = (*pOp).p4.i as u16_0;
                                        }
                                        c2rust_current_block = 5783071609795492627;
                                    }
                                }
                            }
                        }
                        3833908538611729861 => {
                            let mut res_0: ::core::ffi::c_int = 0;
                            let mut oc: ::core::ffi::c_int = 0;
                            let mut pC_3: *mut VdbeCursor = ::core::ptr::null_mut::<
                                VdbeCursor,
                            >();
                            let mut r: UnpackedRecord = UnpackedRecord {
                                pKeyInfo: ::core::ptr::null_mut::<KeyInfo>(),
                                aMem: ::core::ptr::null_mut::<Mem>(),
                                u: C2Rust_Unnamed_24 {
                                    z: ::core::ptr::null_mut::<::core::ffi::c_char>()

                                },
                                n: 0,
                                nField: 0,
                                default_rc: 0,
                                errCode: 0,
                                r1: 0,
                                r2: 0,
                                eqSeen: 0

                            };
                            let mut nField_1: ::core::ffi::c_int = 0;
                            let mut iKey: i64_0 = 0;
                            let mut eqOnly: ::core::ffi::c_int = 0;
                            pC_3 = *(*p).apCsr.offset((*pOp).p1 as isize);
                            oc = (*pOp).opcode as ::core::ffi::c_int;
                            eqOnly = 0 as ::core::ffi::c_int;
                            (*pC_3).nullRow = 0 as u8_0;
                            (*pC_3).deferredMoveto = 0 as u8_0;
                            (*pC_3).cacheStatus = CACHE_STALE as u32_0;
                            if (*pC_3).isTable != 0 {
                                let mut flags3_0: u16_0 = 0;
                                let mut newType: u16_0 = 0;
                                pIn3 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                                flags3_0 = (*pIn3).flags;
                                if flags3_0 as ::core::ffi::c_int
                                    & (MEM_Int | MEM_Real | MEM_IntReal | MEM_Str) == MEM_Str
                                {
                                    applyNumericAffinity(pIn3, 0 as ::core::ffi::c_int);
                                }
                                iKey = sqlite3VdbeIntValue(pIn3);
                                newType = (*pIn3).flags;
                                (*pIn3).flags = flags3_0;
                                if newType as ::core::ffi::c_int & (MEM_Int | MEM_IntReal)
                                    == 0 as ::core::ffi::c_int
                                {
                                    let mut c_1: ::core::ffi::c_int = 0;
                                    if newType as ::core::ffi::c_int & MEM_Real
                                        == 0 as ::core::ffi::c_int
                                    {
                                        if newType as ::core::ffi::c_int & MEM_Null != 0
                                            || oc >= OP_SeekGE
                                        {
                                            c2rust_current_block = 17233182392562552756;
                                        } else {
                                            rc = sqlite3BtreeLast((*pC_3).uc.pCursor, &raw mut res_0);
                                            if rc != SQLITE_OK {
                                                c2rust_current_block = 8086965459351668542;
                                                break;
                                            }
                                            c2rust_current_block = 10641450158969160003;
                                        }
                                    } else {
                                        c_1 = sqlite3IntFloatCompare(iKey, (*pIn3).u.r);
                                        if c_1 > 0 as ::core::ffi::c_int {
                                            if oc & 0x1 as ::core::ffi::c_int
                                                == OP_SeekGT & 0x1 as ::core::ffi::c_int
                                            {
                                                oc -= 1;
                                            }
                                        } else if c_1 < 0 as ::core::ffi::c_int {
                                            if oc & 0x1 as ::core::ffi::c_int
                                                == OP_SeekLT & 0x1 as ::core::ffi::c_int
                                            {
                                                oc += 1;
                                            }
                                        }
                                        c2rust_current_block = 12822409024687027075;
                                    }
                                } else {
                                    c2rust_current_block = 12822409024687027075;
                                }
                                match c2rust_current_block {
                                    10641450158969160003 => {}
                                    17233182392562552756 => {}
                                    _ => {
                                        rc = sqlite3BtreeTableMoveto(
                                            (*pC_3).uc.pCursor,
                                            iKey as u64_0 as i64_0,
                                            0 as ::core::ffi::c_int,
                                            &raw mut res_0,
                                        );
                                        (*pC_3).movetoTarget = iKey;
                                        if rc != SQLITE_OK {
                                            c2rust_current_block = 8086965459351668542;
                                            break;
                                        }
                                        c2rust_current_block = 12769553129898204157;
                                    }
                                }
                            } else {
                                if sqlite3BtreeCursorHasHint(
                                    (*pC_3).uc.pCursor,
                                    BTREE_SEEK_EQ as ::core::ffi::c_uint,
                                ) != 0
                                {
                                    eqOnly = 1 as ::core::ffi::c_int;
                                }
                                nField_1 = (*pOp).p4.i;
                                r.pKeyInfo = (*pC_3).pKeyInfo;
                                r.nField = nField_1 as u16_0;
                                r.default_rc = (if 1 as ::core::ffi::c_int & oc - OP_SeekLT
                                    != 0
                                {
                                    -1 as ::core::ffi::c_int
                                } else {
                                    1 as ::core::ffi::c_int
                                }) as i8_0;
                                r.aMem = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                                r.eqSeen = 0 as u8_0;
                                rc = sqlite3BtreeIndexMoveto(
                                    (*pC_3).uc.pCursor,
                                    &raw mut r,
                                    &raw mut res_0,
                                );
                                if rc != SQLITE_OK {
                                    c2rust_current_block = 8086965459351668542;
                                    break;
                                }
                                if eqOnly != 0
                                    && r.eqSeen as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                {
                                    c2rust_current_block = 10641450158969160003;
                                } else {
                                    c2rust_current_block = 12769553129898204157;
                                }
                            }
                            match c2rust_current_block {
                                17233182392562552756 => {}
                                _ => {
                                    match c2rust_current_block {
                                        12769553129898204157 => {
                                            if oc >= OP_SeekGE {
                                                if res_0 < 0 as ::core::ffi::c_int
                                                    || res_0 == 0 as ::core::ffi::c_int && oc == OP_SeekGT
                                                {
                                                    res_0 = 0 as ::core::ffi::c_int;
                                                    rc = sqlite3BtreeNext(
                                                        (*pC_3).uc.pCursor,
                                                        0 as ::core::ffi::c_int,
                                                    );
                                                    if rc != SQLITE_OK {
                                                        if !(rc == SQLITE_DONE) {
                                                            c2rust_current_block = 8086965459351668542;
                                                            break;
                                                        }
                                                        rc = SQLITE_OK;
                                                        res_0 = 1 as ::core::ffi::c_int;
                                                    }
                                                } else {
                                                    res_0 = 0 as ::core::ffi::c_int;
                                                }
                                            } else if res_0 > 0 as ::core::ffi::c_int
                                                || res_0 == 0 as ::core::ffi::c_int && oc == OP_SeekLT
                                            {
                                                res_0 = 0 as ::core::ffi::c_int;
                                                rc = sqlite3BtreePrevious(
                                                    (*pC_3).uc.pCursor,
                                                    0 as ::core::ffi::c_int,
                                                );
                                                if rc != SQLITE_OK {
                                                    if !(rc == SQLITE_DONE) {
                                                        c2rust_current_block = 8086965459351668542;
                                                        break;
                                                    }
                                                    rc = SQLITE_OK;
                                                    res_0 = 1 as ::core::ffi::c_int;
                                                }
                                            } else {
                                                res_0 = sqlite3BtreeEof((*pC_3).uc.pCursor);
                                            }
                                        }
                                        _ => {}
                                    }
                                    if res_0 != 0 {
                                        c2rust_current_block = 17233182392562552756;
                                    } else {
                                        if eqOnly != 0 {
                                            pOp = pOp.offset(1);
                                        }
                                        c2rust_current_block = 5783071609795492627;
                                    }
                                }
                            }
                        }
                        13567963862951993727 => {
                            let mut iA_0: i64_0 = 0;
                            let mut uA: u64_0 = 0;
                            let mut iB_0: i64_0 = 0;
                            let mut op: u8_0 = 0;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pIn2 = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            pOut = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            if ((*pIn1).flags as ::core::ffi::c_int
                                | (*pIn2).flags as ::core::ffi::c_int) & MEM_Null != 0
                            {
                                sqlite3VdbeMemSetNull(pOut);
                            } else {
                                iA_0 = sqlite3VdbeIntValue(pIn2);
                                iB_0 = sqlite3VdbeIntValue(pIn1);
                                op = (*pOp).opcode;
                                if op as ::core::ffi::c_int == OP_BitAnd {
                                    iA_0 &= iB_0 as ::core::ffi::c_longlong;
                                } else if op as ::core::ffi::c_int == OP_BitOr {
                                    iA_0 |= iB_0 as ::core::ffi::c_longlong;
                                } else if iB_0 != 0 as ::core::ffi::c_longlong {
                                    if iB_0 < 0 as ::core::ffi::c_longlong {
                                        op = (2 as ::core::ffi::c_int * OP_ShiftLeft
                                            + 1 as ::core::ffi::c_int - op as ::core::ffi::c_int)
                                            as u8_0;
                                        iB_0 = (if iB_0
                                            > -64 as ::core::ffi::c_int as ::core::ffi::c_longlong
                                        {
                                            -(iB_0 as ::core::ffi::c_longlong)
                                        } else {
                                            64 as ::core::ffi::c_longlong
                                        }) as i64_0;
                                    }
                                    if iB_0 >= 64 as ::core::ffi::c_longlong {
                                        iA_0 = (if iA_0 >= 0 as ::core::ffi::c_longlong
                                            || op as ::core::ffi::c_int == OP_ShiftLeft
                                        {
                                            0 as ::core::ffi::c_int
                                        } else {
                                            -1 as ::core::ffi::c_int
                                        }) as i64_0;
                                    } else {
                                        memcpy(
                                            &raw mut uA as *mut ::core::ffi::c_void,
                                            &raw mut iA_0 as *const ::core::ffi::c_void,
                                            ::core::mem::size_of::<u64_0>() as size_t,
                                        );
                                        if op as ::core::ffi::c_int == OP_ShiftLeft {
                                            uA <<= iB_0;
                                        } else {
                                            uA >>= iB_0;
                                            if iA_0 < 0 as ::core::ffi::c_longlong {
                                                uA
                                                    |= ((0xffffffff as ::core::ffi::c_uint
                                                        as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int
                                                        | 0xffffffff as ::core::ffi::c_ulonglong)
                                                        << 64 as i64_0 - iB_0;
                                            }
                                        }
                                        memcpy(
                                            &raw mut iA_0 as *mut ::core::ffi::c_void,
                                            &raw mut uA as *const ::core::ffi::c_void,
                                            ::core::mem::size_of::<i64_0>() as size_t,
                                        );
                                    }
                                }
                                (*pOut).u.i = iA_0;
                                (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                    & !(MEM_TypeMask | MEM_Zero) | 0x4 as ::core::ffi::c_int)
                                    as u16_0;
                            }
                            c2rust_current_block = 5783071609795492627;
                        }
                        7300422940758305934 => {
                            c2rust_current_block = 9028266288740425872;
                        }
                        7659955564255721022 => {
                            c2rust_current_block = 16318049377487887596;
                        }
                        _ => {}
                    }
                    match c2rust_current_block {
                        9028266288740425872 => {
                            let mut type1: u16_0 = 0;
                            let mut type2: u16_0 = 0;
                            let mut iA: i64_0 = 0;
                            let mut iB: i64_0 = 0;
                            let mut rA: ::core::ffi::c_double = 0.;
                            let mut rB: ::core::ffi::c_double = 0.;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            type1 = (*pIn1).flags;
                            pIn2 = aMem.offset((*pOp).p2 as isize) as *mut Mem;
                            type2 = (*pIn2).flags;
                            pOut = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            if type1 as ::core::ffi::c_int & type2 as ::core::ffi::c_int
                                & MEM_Int != 0 as ::core::ffi::c_int
                            {
                                c2rust_current_block = 3801854829799466053;
                            } else if (type1 as ::core::ffi::c_int
                                | type2 as ::core::ffi::c_int) & MEM_Null
                                != 0 as ::core::ffi::c_int
                            {
                                c2rust_current_block = 230263296376300617;
                            } else {
                                type1 = numericType(pIn1);
                                type2 = numericType(pIn2);
                                if type1 as ::core::ffi::c_int & type2 as ::core::ffi::c_int
                                    & MEM_Int != 0 as ::core::ffi::c_int
                                {
                                    c2rust_current_block = 3801854829799466053;
                                } else {
                                    c2rust_current_block = 17141096483388064781;
                                }
                            }
                            match c2rust_current_block {
                                3801854829799466053 => {
                                    iA = (*pIn1).u.i;
                                    iB = (*pIn2).u.i;
                                    match (*pOp).opcode as ::core::ffi::c_int {
                                        OP_Add => {
                                            if sqlite3AddInt64(&raw mut iB, iA) != 0 {
                                                c2rust_current_block = 17141096483388064781;
                                            } else {
                                                c2rust_current_block = 9914851455145855695;
                                            }
                                        }
                                        OP_Subtract => {
                                            if sqlite3SubInt64(&raw mut iB, iA) != 0 {
                                                c2rust_current_block = 17141096483388064781;
                                            } else {
                                                c2rust_current_block = 9914851455145855695;
                                            }
                                        }
                                        OP_Multiply => {
                                            if sqlite3MulInt64(&raw mut iB, iA) != 0 {
                                                c2rust_current_block = 17141096483388064781;
                                            } else {
                                                c2rust_current_block = 9914851455145855695;
                                            }
                                        }
                                        OP_Divide => {
                                            if iA == 0 as ::core::ffi::c_longlong {
                                                c2rust_current_block = 230263296376300617;
                                            } else if iA
                                                == -1 as ::core::ffi::c_int as ::core::ffi::c_longlong
                                                && iB == SMALLEST_INT64
                                            {
                                                c2rust_current_block = 17141096483388064781;
                                            } else {
                                                iB /= iA as ::core::ffi::c_longlong;
                                                c2rust_current_block = 9914851455145855695;
                                            }
                                        }
                                        _ => {
                                            if iA == 0 as ::core::ffi::c_longlong {
                                                c2rust_current_block = 230263296376300617;
                                            } else {
                                                if iA == -1 as ::core::ffi::c_int as ::core::ffi::c_longlong
                                                {
                                                    iA = 1 as i64_0;
                                                }
                                                iB %= iA as ::core::ffi::c_longlong;
                                                c2rust_current_block = 9914851455145855695;
                                            }
                                        }
                                    }
                                    match c2rust_current_block {
                                        230263296376300617 => {}
                                        17141096483388064781 => {}
                                        _ => {
                                            (*pOut).u.i = iB;
                                            (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                                & !(MEM_TypeMask | MEM_Zero) | 0x4 as ::core::ffi::c_int)
                                                as u16_0;
                                            c2rust_current_block = 5783071609795492627;
                                        }
                                    }
                                }
                                _ => {}
                            }
                            match c2rust_current_block {
                                5783071609795492627 => {}
                                _ => {
                                    match c2rust_current_block {
                                        17141096483388064781 => {
                                            rA = sqlite3VdbeRealValue(pIn1);
                                            rB = sqlite3VdbeRealValue(pIn2);
                                            match (*pOp).opcode as ::core::ffi::c_int {
                                                OP_Add => {
                                                    rB += rA;
                                                    c2rust_current_block = 6950939092385495874;
                                                }
                                                OP_Subtract => {
                                                    rB -= rA;
                                                    c2rust_current_block = 6950939092385495874;
                                                }
                                                OP_Multiply => {
                                                    rB *= rA;
                                                    c2rust_current_block = 6950939092385495874;
                                                }
                                                OP_Divide => {
                                                    if rA == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                                        c2rust_current_block = 230263296376300617;
                                                    } else {
                                                        rB /= rA;
                                                        c2rust_current_block = 6950939092385495874;
                                                    }
                                                }
                                                _ => {
                                                    iA = sqlite3VdbeIntValue(pIn1);
                                                    iB = sqlite3VdbeIntValue(pIn2);
                                                    if iA == 0 as ::core::ffi::c_longlong {
                                                        c2rust_current_block = 230263296376300617;
                                                    } else {
                                                        if iA == -1 as ::core::ffi::c_int as ::core::ffi::c_longlong
                                                        {
                                                            iA = 1 as i64_0;
                                                        }
                                                        rB = (iB % iA) as ::core::ffi::c_double;
                                                        c2rust_current_block = 6950939092385495874;
                                                    }
                                                }
                                            }
                                            match c2rust_current_block {
                                                230263296376300617 => {}
                                                _ => {
                                                    if sqlite3IsNaN(rB) != 0 {
                                                        c2rust_current_block = 230263296376300617;
                                                    } else {
                                                        (*pOut).u.r = rB;
                                                        (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                                            & !(MEM_TypeMask | MEM_Zero) | 0x8 as ::core::ffi::c_int)
                                                            as u16_0;
                                                        c2rust_current_block = 5783071609795492627;
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                    match c2rust_current_block {
                                        5783071609795492627 => {}
                                        _ => {
                                            sqlite3VdbeMemSetNull(pOut);
                                            c2rust_current_block = 5783071609795492627;
                                        }
                                    }
                                }
                            }
                        }
                        16318049377487887596 => {
                            c2rust_current_block = 18434696934714153308;
                        }
                        _ => {}
                    }
                    match c2rust_current_block {
                        18434696934714153308 => {
                            let mut res: ::core::ffi::c_int = 0;
                            let mut res2: ::core::ffi::c_int = 0;
                            let mut affinity: ::core::ffi::c_char = 0;
                            let mut flags1_0: u16_0 = 0;
                            let mut flags3: u16_0 = 0;
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut Mem;
                            pIn3 = aMem.offset((*pOp).p3 as isize) as *mut Mem;
                            flags1_0 = (*pIn1).flags;
                            flags3 = (*pIn3).flags;
                            if flags1_0 as ::core::ffi::c_int
                                & flags3 as ::core::ffi::c_int & MEM_Int
                                != 0 as ::core::ffi::c_int
                            {
                                if (*pIn3).u.i > (*pIn1).u.i {
                                    if *sqlite3aGTb.offset((*pOp).opcode as isize) != 0 {
                                        c2rust_current_block = 17233182392562552756;
                                    } else {
                                        iCompare = 1 as ::core::ffi::c_int;
                                        c2rust_current_block = 5783071609795492627;
                                    }
                                } else if (*pIn3).u.i < (*pIn1).u.i {
                                    if *sqlite3aLTb.offset((*pOp).opcode as isize) != 0 {
                                        c2rust_current_block = 17233182392562552756;
                                    } else {
                                        iCompare = -1 as ::core::ffi::c_int;
                                        c2rust_current_block = 5783071609795492627;
                                    }
                                } else if *sqlite3aEQb.offset((*pOp).opcode as isize) != 0 {
                                    c2rust_current_block = 17233182392562552756;
                                } else {
                                    iCompare = 0 as ::core::ffi::c_int;
                                    c2rust_current_block = 5783071609795492627;
                                }
                            } else {
                                if (flags1_0 as ::core::ffi::c_int
                                    | flags3 as ::core::ffi::c_int) & MEM_Null != 0
                                {
                                    if (*pOp).p5 as ::core::ffi::c_int & SQLITE_NULLEQ != 0 {
                                        if flags1_0 as ::core::ffi::c_int
                                            & flags3 as ::core::ffi::c_int & MEM_Null
                                            != 0 as ::core::ffi::c_int
                                            && flags3 as ::core::ffi::c_int & MEM_Cleared
                                                == 0 as ::core::ffi::c_int
                                        {
                                            res = 0 as ::core::ffi::c_int;
                                        } else {
                                            res = if flags3 as ::core::ffi::c_int & MEM_Null != 0 {
                                                -1 as ::core::ffi::c_int
                                            } else {
                                                1 as ::core::ffi::c_int
                                            };
                                        }
                                        c2rust_current_block = 5769507501690292314;
                                    } else if (*pOp).p5 as ::core::ffi::c_int
                                        & SQLITE_JUMPIFNULL != 0
                                    {
                                        c2rust_current_block = 17233182392562552756;
                                    } else {
                                        iCompare = 1 as ::core::ffi::c_int;
                                        c2rust_current_block = 5783071609795492627;
                                    }
                                } else {
                                    affinity = ((*pOp).p5 as ::core::ffi::c_int
                                        & SQLITE_AFF_MASK) as ::core::ffi::c_char;
                                    if affinity as ::core::ffi::c_int >= SQLITE_AFF_NUMERIC {
                                        if (flags1_0 as ::core::ffi::c_int
                                            | flags3 as ::core::ffi::c_int) & MEM_Str != 0
                                        {
                                            if flags1_0 as ::core::ffi::c_int
                                                & (MEM_Int | MEM_IntReal | MEM_Real | MEM_Str) == MEM_Str
                                            {
                                                applyNumericAffinity(pIn1, 0 as ::core::ffi::c_int);
                                                flags3 = (*pIn3).flags;
                                            }
                                            if flags3 as ::core::ffi::c_int
                                                & (MEM_Int | MEM_IntReal | MEM_Real | MEM_Str) == MEM_Str
                                            {
                                                applyNumericAffinity(pIn3, 0 as ::core::ffi::c_int);
                                            }
                                        }
                                    } else if affinity as ::core::ffi::c_int == SQLITE_AFF_TEXT
                                        && (flags1_0 as ::core::ffi::c_int
                                            | flags3 as ::core::ffi::c_int) & MEM_Str
                                            != 0 as ::core::ffi::c_int
                                    {
                                        if flags1_0 as ::core::ffi::c_int & MEM_Str
                                            != 0 as ::core::ffi::c_int
                                        {
                                            (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                                & !(MEM_Int | MEM_Real | MEM_IntReal)) as u16_0;
                                        } else if flags1_0 as ::core::ffi::c_int
                                            & (MEM_Int | MEM_Real | MEM_IntReal)
                                            != 0 as ::core::ffi::c_int
                                        {
                                            sqlite3VdbeMemStringify(pIn1, encoding, 1 as u8_0);
                                            flags1_0 = ((*pIn1).flags as ::core::ffi::c_int
                                                & !MEM_TypeMask
                                                | flags1_0 as ::core::ffi::c_int & MEM_TypeMask) as u16_0;
                                            if pIn1 == pIn3 {
                                                flags3 = (flags1_0 as ::core::ffi::c_int | MEM_Str)
                                                    as u16_0;
                                            }
                                        }
                                        if flags3 as ::core::ffi::c_int & MEM_Str
                                            != 0 as ::core::ffi::c_int
                                        {
                                            (*pIn3).flags = ((*pIn3).flags as ::core::ffi::c_int
                                                & !(MEM_Int | MEM_Real | MEM_IntReal)) as u16_0;
                                        } else if flags3 as ::core::ffi::c_int
                                            & (MEM_Int | MEM_Real | MEM_IntReal)
                                            != 0 as ::core::ffi::c_int
                                        {
                                            sqlite3VdbeMemStringify(pIn3, encoding, 1 as u8_0);
                                            flags3 = ((*pIn3).flags as ::core::ffi::c_int
                                                & !MEM_TypeMask
                                                | flags3 as ::core::ffi::c_int & MEM_TypeMask) as u16_0;
                                        }
                                    }
                                    res = sqlite3MemCompare(pIn3, pIn1, (*pOp).p4.pColl);
                                    c2rust_current_block = 5769507501690292314;
                                }
                                match c2rust_current_block {
                                    5783071609795492627 => {}
                                    17233182392562552756 => {}
                                    _ => {
                                        if res < 0 as ::core::ffi::c_int {
                                            res2 = *sqlite3aLTb.offset((*pOp).opcode as isize)
                                                as ::core::ffi::c_int;
                                        } else if res == 0 as ::core::ffi::c_int {
                                            res2 = *sqlite3aEQb.offset((*pOp).opcode as isize)
                                                as ::core::ffi::c_int;
                                        } else {
                                            res2 = *sqlite3aGTb.offset((*pOp).opcode as isize)
                                                as ::core::ffi::c_int;
                                        }
                                        iCompare = res;
                                        (*pIn3).flags = flags3;
                                        (*pIn1).flags = flags1_0;
                                        if res2 != 0 {
                                            c2rust_current_block = 17233182392562552756;
                                        } else {
                                            c2rust_current_block = 5783071609795492627;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    match c2rust_current_block {
                        17233182392562552756 => {
                            pOp = aOp
                                .offset(((*pOp).p2 - 1 as ::core::ffi::c_int) as isize)
                                as *mut Op;
                        }
                        _ => {}
                    }
                    pOp = pOp.offset(1);
                }
                match c2rust_current_block {
                    8086965459351668542 => {}
                    4288082686256521985 => {}
                    4286470523181410568 => {}
                    9955115899106739073 => {}
                    _ => {
                        sqlite3VdbeError(
                            p,
                            b"string or blob too big\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        rc = SQLITE_TOOBIG;
                        c2rust_current_block = 8086965459351668542;
                    }
                }
            }
            match c2rust_current_block {
                8086965459351668542 => {}
                4288082686256521985 => {}
                9955115899106739073 => {}
                _ => {
                    rc = SQLITE_INTERRUPT;
                    c2rust_current_block = 8086965459351668542;
                }
            }
        }
        match c2rust_current_block {
            9955115899106739073 => {
                sqlite3OomFault(db);
                sqlite3VdbeError(
                    p,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
                rc = SQLITE_NOMEM_BKPT;
                c2rust_current_block = 8086965459351668542;
            }
            _ => {}
        }
        loop {
            match c2rust_current_block {
                4288082686256521985 => {
                    if !(nVmStep >= nProgressLimit && (*db).xProgress.is_some()) {
                        break;
                    }
                    nProgressLimit = (nProgressLimit as ::core::ffi::c_ulonglong)
                        .wrapping_add((*db).nProgressOps as ::core::ffi::c_ulonglong)
                        as u64_0 as u64_0;
                    if !((*db)
                        .xProgress
                        .expect("non-null function pointer")((*db).pProgressArg) != 0)
                    {
                        c2rust_current_block = 4288082686256521985;
                        continue;
                    }
                    nProgressLimit = LARGEST_UINT64;
                    rc = SQLITE_INTERRUPT;
                    c2rust_current_block = 8086965459351668542;
                }
                _ => {
                    if (*db).mallocFailed != 0 {
                        rc = SQLITE_NOMEM_BKPT;
                    } else if rc == SQLITE_IOERR_CORRUPTFS {
                        rc = sqlite3CorruptError(9231 as ::core::ffi::c_int);
                    }
                    if (*p).zErrMsg.is_null() && rc != SQLITE_IOERR_NOMEM {
                        sqlite3VdbeError(
                            p,
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            sqlite3ErrStr(rc),
                        );
                    }
                    (*p).rc = rc;
                    sqlite3SystemError(db, rc);
                    sqlite3VdbeLogAbort(p, rc, pOp, aOp);
                    if (*p).eVdbeState as ::core::ffi::c_int == VDBE_RUN_STATE {
                        sqlite3VdbeHalt(p);
                    }
                    if rc == SQLITE_IOERR_NOMEM {
                        sqlite3OomFault(db);
                    }
                    if rc == SQLITE_CORRUPT
                        && (*db).autoCommit as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        (*db).flags |= SQLITE_CorruptRdOnly as ::core::ffi::c_ulonglong;
                    }
                    rc = SQLITE_ERROR;
                    if resetSchemaOnFault as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    {
                        sqlite3ResetOneSchema(
                            db,
                            resetSchemaOnFault as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int,
                        );
                    }
                    c2rust_current_block = 4288082686256521985;
                }
            }
        }
        (*p).aCounter[SQLITE_STMTSTATUS_VM_STEP as usize] = ((*p)
            .aCounter[SQLITE_STMTSTATUS_VM_STEP as usize] as ::core::ffi::c_uint)
            .wrapping_add(nVmStep as ::core::ffi::c_int as ::core::ffi::c_uint) as u32_0
            as u32_0;
        (*p).lockMask != 0 as ::core::ffi::c_uint;
        return rc;
    }
}
pub const MAX_ROWID: i64_0 = ((0x7fffffff as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int | 0xffffffff as ::core::ffi::c_uint as u64_0) as i64_0;
pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
