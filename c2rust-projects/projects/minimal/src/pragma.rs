use c2rust_bitfields::BitfieldStruct;
use ::libc;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type RenameToken;
    pub type Vdbe;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type sqlite3_pcache;
    pub type Pager;
    fn sqlite3_compileoption_get(N: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3_busy_timeout(
        _: *mut sqlite3,
        ms: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_limit(
        _: *mut sqlite3,
        id: ::core::ffi::c_int,
        newVal: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_prepare_v3(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        prepFlags: ::core::ffi::c_uint,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_value(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *mut sqlite3_value;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value);
    static mut sqlite3_temp_directory: *mut ::core::ffi::c_char;
    fn sqlite3_db_release_memory(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_soft_heap_limit64(N: sqlite3_int64) -> sqlite3_int64;
    fn sqlite3_hard_heap_limit64(N: sqlite3_int64) -> sqlite3_int64;
    fn sqlite3_declare_vtab(
        _: *mut sqlite3,
        zSQL: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn sqlite3_file_control(
        _: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
        op: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_str_appendf(
        _: *mut sqlite3_str,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3_str_append(
        _: *mut sqlite3_str,
        zIn: *const ::core::ffi::c_char,
        N: ::core::ffi::c_int,
    );
    fn sqlite3_str_appendall(_: *mut sqlite3_str, zIn: *const ::core::ffi::c_char);
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_wal_autocheckpoint(
        db: *mut sqlite3,
        N: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3HashFind(
        _: *const Hash,
        pKey: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3OsAccess(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        pResOut: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerLockingMode(
        _: *mut Pager,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerJournalSizeLimit(_: *mut Pager, _: i64_0) -> i64_0;
    fn sqlite3BtreeClose(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeSetCacheSize(
        _: *mut Btree,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeSetSpillSize(
        _: *mut Btree,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeSetMmapLimit(_: *mut Btree, _: sqlite3_int64) -> ::core::ffi::c_int;
    fn sqlite3BtreeSetPagerFlags(
        _: *mut Btree,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeSetPageSize(
        p: *mut Btree,
        nPagesize: ::core::ffi::c_int,
        nReserve: ::core::ffi::c_int,
        eFix: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeGetPageSize(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeSecureDelete(
        _: *mut Btree,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeSetAutoVacuum(
        _: *mut Btree,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeGetAutoVacuum(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeTxnState(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeGetFilename(_: *mut Btree) -> *const ::core::ffi::c_char;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3VdbeAddOp0(_: *mut Vdbe, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddOp1(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddOp2(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeGoto(_: *mut Vdbe, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3VdbeLoadString(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeMultiLoad(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3VdbeAddOp3(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddOp4(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        zP4: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddOp4Dup8(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const u8_0,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddOp4Int(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeAddOpList(
        _: *mut Vdbe,
        nOp: ::core::ffi::c_int,
        aOp: *const VdbeOpList,
        iLineno: ::core::ffi::c_int,
    ) -> *mut VdbeOp;
    fn sqlite3VdbeChangeP3(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
        P3: ::core::ffi::c_int,
    );
    fn sqlite3VdbeChangeP5(_: *mut Vdbe, P5: u16_0);
    fn sqlite3VdbeTypeofColumn(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeJumpHere(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeAppendP4(
        _: *mut Vdbe,
        pP4: *mut ::core::ffi::c_void,
        p4type: ::core::ffi::c_int,
    );
    fn sqlite3VdbeSetP4KeyInfo(_: *mut Parse, _: *mut Index);
    fn sqlite3VdbeUsesBtree(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeGetOp(_: *mut Vdbe, _: ::core::ffi::c_int) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3VdbeRunOnlyOnce(_: *mut Vdbe);
    fn sqlite3VdbeReusable(_: *mut Vdbe);
    fn sqlite3VdbeResolveLabel(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeCurrentAddr(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3VdbeSetNumCols(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeSetColName(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3ColumnType(
        _: *mut Column,
        _: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MutexAlloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3GetTempReg(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3GetTempRange(_: *mut Parse, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempRange(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ClearTempRegCache(_: *mut Parse);
    fn sqlite3TouchRegister(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3ExprListDelete(_: *mut sqlite3, _: *mut ExprList);
    fn sqlite3ResetAllSchemasOfConnection(_: *mut sqlite3);
    fn sqlite3ColumnExpr(_: *mut Table, _: *mut Column) -> *mut Expr;
    fn sqlite3PrimaryKeyIndex(_: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(
        _: *mut Index,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3TableColumnToStorage(_: *mut Table, _: i16_0) -> i16_0;
    fn sqlite3ViewGetColumnNames(_: *mut Parse, _: *mut Table) -> ::core::ffi::c_int;
    fn sqlite3OpenTable(
        _: *mut Parse,
        iCur: ::core::ffi::c_int,
        iDb: ::core::ffi::c_int,
        _: *mut Table,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprCodeLoadIndexColumn(
        _: *mut Parse,
        _: *mut Index,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprCodeGetColumnOfTable(
        _: *mut Vdbe,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprIfTrue(
        _: *mut Parse,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprIfFalse(
        _: *mut Parse,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3FindTable(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut Table;
    fn sqlite3LocateTable(
        _: *mut Parse,
        flags: u32_0,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut Table;
    fn sqlite3PreferredTableName(
        _: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3FindIndex(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut Index;
    fn sqlite3NameFromToken(
        _: *mut sqlite3,
        _: *const Token,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3GetVdbe(_: *mut Parse) -> *mut Vdbe;
    fn sqlite3CodeVerifySchema(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3CodeVerifyNamedSchema(_: *mut Parse, zDb: *const ::core::ffi::c_char);
    fn sqlite3GenerateIndexKey(
        _: *mut Parse,
        _: *mut Index,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut Index,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ResolvePartIdxLabel(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3OpenTableAndIndices(
        _: *mut Parse,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: u8_0,
        _: ::core::ffi::c_int,
        _: *mut u8_0,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BeginWriteOperation(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprListDup(
        _: *mut sqlite3,
        _: *const ExprList,
        _: ::core::ffi::c_int,
    ) -> *mut ExprList;
    fn sqlite3AuthCheck(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3GetInt32(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3IndexAffinityStr(
        _: *mut sqlite3,
        _: *mut Index,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3DecOrHexToI64(
        _: *const ::core::ffi::c_char,
        _: *mut i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3TwoPartName(
        _: *mut Parse,
        _: *mut Token,
        _: *mut Token,
        _: *mut *mut Token,
    ) -> ::core::ffi::c_int;
    fn sqlite3ErrStr(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3ReadSchema(pParse: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3SetTextEncoding(db: *mut sqlite3, _: u8_0);
    fn sqlite3ValueFree(_: *mut sqlite3_value);
    fn sqlite3ValueFromExpr(
        _: *mut sqlite3,
        _: *const Expr,
        _: u8_0,
        _: u8_0,
        _: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    static sqlite3StrBINARY: [::core::ffi::c_char; 0];
    static mut sqlite3StdType: [*const ::core::ffi::c_char; 0];
    static sqlite3UpperToLower: [::core::ffi::c_uchar; 0];
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3BuiltinFunctions: FuncDefHash;
    fn sqlite3ColumnDefault(
        _: *mut Vdbe,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3RegisterLikeFunctions(_: *mut sqlite3, _: ::core::ffi::c_int);
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3OpenTempDatabase(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3StrAccumInit(
        _: *mut StrAccum,
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3StrAccumFinish(_: *mut StrAccum) -> *mut ::core::ffi::c_char;
    fn sqlite3VtabCreateModule(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: *const sqlite3_module,
        _: *mut ::core::ffi::c_void,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> *mut Module;
    fn sqlite3WalDefaultHook(
        _: *mut ::core::ffi::c_void,
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3FkLocateIndex(
        _: *mut Parse,
        _: *mut Table,
        _: *mut FKey,
        _: *mut *mut Index,
        _: *mut *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    pub trace: C2Rust_Unnamed_21,
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
    pub u1: C2Rust_Unnamed_17,
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
    pub u: C2Rust_Unnamed_13,
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
    pub fg: C2Rust_Unnamed_12,
    pub iCursor: ::core::ffi::c_int,
    pub colUsed: Bitmask,
    pub u1: C2Rust_Unnamed_11,
    pub u2: C2Rust_Unnamed_10,
    pub u3: C2Rust_Unnamed_9,
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
    pub u: C2Rust_Unnamed_8,
    pub pLeft: *mut Expr,
    pub pRight: *mut Expr,
    pub x: C2Rust_Unnamed_7,
    pub nHeight: ::core::ffi::c_int,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ynVar,
    pub iAgg: i16_0,
    pub w: C2Rust_Unnamed_6,
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
pub type i16_0 = ::core::ffi::c_short;
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
    pub fg: C2Rust_Unnamed_5,
    pub u: C2Rust_Unnamed_3

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub x: C2Rust_Unnamed_4,
    pub iConstExprReg: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub iOrderByCol: u16_0,
    pub iAlias: u16_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
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
pub union C2Rust_Unnamed_6 {
    pub iJoin: ::core::ffi::c_int,
    pub iOfst: ::core::ffi::c_int

}
pub type ynVar = i16_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub pList: *mut ExprList,
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub zToken: *mut ::core::ffi::c_char,
    pub iValue: ::core::ffi::c_int

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
pub union C2Rust_Unnamed_9 {
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
pub union C2Rust_Unnamed_10 {
    pub pIBIndex: *mut Index,
    pub pCteUse: *mut CteUse

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
pub type Pgno = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub zIndexedBy: *mut ::core::ffi::c_char,
    pub pFuncArg: *mut ExprList,
    pub nRow: u32_0

}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
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
pub union C2Rust_Unnamed_13 {
    pub tab: C2Rust_Unnamed_16,
    pub view: C2Rust_Unnamed_15,
    pub vtab: C2Rust_Unnamed_14

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub nArg: ::core::ffi::c_int,
    pub azArg: *mut *mut ::core::ffi::c_char,
    pub p: *mut VTable

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub pSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
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
pub union C2Rust_Unnamed_17 {
    pub isInterrupted: ::core::ffi::c_int,
    pub notUsed1: ::core::ffi::c_double

}
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
    pub u1: C2Rust_Unnamed_18,
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
pub struct Token {
    pub z: *const ::core::ffi::c_char,
    pub n: ::core::ffi::c_uint

}
pub type VList = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_18 {
    pub cr: C2Rust_Unnamed_20,
    pub d: C2Rust_Unnamed_19

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
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
pub struct C2Rust_Unnamed_20 {
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
pub type Mem = sqlite3_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AutoincInfo {
    pub pNext: *mut AutoincInfo,
    pub pTab: *mut Table,
    pub iDb: ::core::ffi::c_int,
    pub regCtr: ::core::ffi::c_int

}
pub type yDbMask = ::core::ffi::c_uint;
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
pub type bft = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_21 {
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
pub struct sqlite3_str {
    pub db: *mut sqlite3,
    pub zText: *mut ::core::ffi::c_char,
    pub nAlloc: u32_0,
    pub mxAlloc: u32_0,
    pub nChar: u32_0,
    pub accError: u8_0,
    pub printfFlags: u8_0

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
pub struct FuncDefHash {
    pub a: [*mut FuncDef; 23]

}
pub type StrAccum = sqlite3_str;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeOpList {
    pub opcode: u8_0,
    pub p1: ::core::ffi::c_schar,
    pub p2: ::core::ffi::c_schar,
    pub p3: ::core::ffi::c_schar

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
pub struct PragmaName {
    pub zName: *const ::core::ffi::c_char,
    pub ePragTyp: u8_0,
    pub mPragFlg: u8_0,
    pub iPragCName: u8_0,
    pub nPragCName: u8_0,
    pub iArg: u64_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EncName {
    pub zName: *mut ::core::ffi::c_char,
    pub enc: u8_0

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PragmaVtabCursor {
    pub base: sqlite3_vtab_cursor,
    pub pPragma: *mut sqlite3_stmt,
    pub iRowid: sqlite_int64,
    pub azArg: [*mut ::core::ffi::c_char; 2]

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PragmaVtab {
    pub base: sqlite3_vtab,
    pub db: *mut sqlite3,
    pub pName: *const PragmaName,
    pub nHidden: u8_0,
    pub iHidden: u8_0

}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_PRAGMA: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_MMAP_SIZE: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_ACCESS_READWRITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_PRAGMA: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_SQL_LENGTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_WORKER_THREADS: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_PREPARE_DONT_LOG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_UTF16LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_UTF16BE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_DIRECTONLY: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_SUBTYPE: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const SQLITE_INNOCUOUS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_TXN_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_STATIC_VFS1: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_CHECKPOINT_NOOP: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const SQLITE_CHECKPOINT_PASSIVE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CHECKPOINT_FULL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_CHECKPOINT_RESTART: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_CHECKPOINT_TRUNCATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_STATIC_TEMPDIR: ::core::ffi::c_int = SQLITE_MUTEX_STATIC_VFS1;
pub const OP_Transaction: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OP_Checkpoint: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const OP_JournalMode: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const OP_Goto: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OP_If: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const OP_IsType: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const OP_Found: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const OP_SeekRowid: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const OP_IfSizeBetween: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const OP_IdxGT: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const OP_IsNull: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const OP_NotNull: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const OP_Ne: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const OP_Eq: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const OP_IfPos: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const OP_IfNotZero: ::core::ffi::c_int = 61 as ::core::ffi::c_int;
pub const OP_IncrVacuum: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
pub const OP_Halt: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const OP_Int64: ::core::ffi::c_int = 73 as ::core::ffi::c_int;
pub const OP_Null: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const OP_ResultRow: ::core::ffi::c_int = 85 as ::core::ffi::c_int;
pub const OP_AddImm: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_Affinity: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
pub const OP_ReadCookie: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const OP_SetCookie: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const OP_Concat: ::core::ffi::c_int = 112 as ::core::ffi::c_int;
pub const OP_OpenRead: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const OP_String8: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const OP_Rowid: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const OP_IdxRowid: ::core::ffi::c_int = 143 as ::core::ffi::c_int;
pub const OP_SqlExec: ::core::ffi::c_int = 149 as ::core::ffi::c_int;
pub const OP_IntegrityCk: ::core::ffi::c_int = 156 as ::core::ffi::c_int;
pub const OP_Expire: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const OP_VCheck: ::core::ffi::c_int = 175 as ::core::ffi::c_int;
pub const OP_Pagecount: ::core::ffi::c_int = 179 as ::core::ffi::c_int;
pub const OP_MaxPgcnt: ::core::ffi::c_int = 180 as ::core::ffi::c_int;
pub const OP_Noop: ::core::ffi::c_int = 188 as ::core::ffi::c_int;
pub const OMIT_TEMPDB: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_TEMP_STORE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_UTF16NATIVE: ::core::ffi::c_int = SQLITE_UTF16LE;
pub const PAGER_LOCKINGMODE_QUERY: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const PAGER_LOCKINGMODE_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PAGER_LOCKINGMODE_EXCLUSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_QUERY: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_OFF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PAGER_SYNCHRONOUS_MASK: ::core::ffi::c_int = 0x7 as ::core::ffi::c_int;
pub const PAGER_FLAGS_MASK: ::core::ffi::c_int = 0x38 as ::core::ffi::c_int;
pub const BTREE_AUTOVACUUM_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BTREE_AUTOVACUUM_FULL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTREE_AUTOVACUUM_INCR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BTREE_FREE_PAGE_COUNT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BTREE_SCHEMA_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTREE_LARGEST_ROOT_PAGE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BTREE_USER_VERSION: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const BTREE_INCR_VACUUM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const BTREE_APPLICATION_ID: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const BTREE_DATA_VERSION: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const P4_STATIC: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const P4_DYNAMIC: ::core::ffi::c_int = -6 as ::core::ffi::c_int;
pub const P4_INT64: ::core::ffi::c_int = -13 as ::core::ffi::c_int;
pub const P4_INTARRAY: ::core::ffi::c_int = -14 as ::core::ffi::c_int;
pub const P4_TABLEREF: ::core::ffi::c_int = -16 as ::core::ffi::c_int;
pub const COLNAME_NAME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_OPTIMIZE_LIMIT: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
unsafe extern "C" fn getSafetyLevel(
    mut z: *const ::core::ffi::c_char,
    mut omitFull: ::core::ffi::c_int,
    mut dflt: u8_0,
) -> u8_0 {
    unsafe {
        static mut zText: [::core::ffi::c_char; 25] = unsafe {
            ::core::mem::transmute::<
                [u8; 25],
                [::core::ffi::c_char; 25],
            >(*b"onoffalseyestruextrafull\0")
        };
        static mut iOffset: [u8_0; 8] = [
            0 as ::core::ffi::c_int as u8_0,
            1 as ::core::ffi::c_int as u8_0,
            2 as ::core::ffi::c_int as u8_0,
            4 as ::core::ffi::c_int as u8_0,
            9 as ::core::ffi::c_int as u8_0,
            12 as ::core::ffi::c_int as u8_0,
            15 as ::core::ffi::c_int as u8_0,
            20 as ::core::ffi::c_int as u8_0,
        ];
        static mut iLength: [u8_0; 8] = [
            2 as ::core::ffi::c_int as u8_0,
            2 as ::core::ffi::c_int as u8_0,
            3 as ::core::ffi::c_int as u8_0,
            5 as ::core::ffi::c_int as u8_0,
            3 as ::core::ffi::c_int as u8_0,
            4 as ::core::ffi::c_int as u8_0,
            5 as ::core::ffi::c_int as u8_0,
            4 as ::core::ffi::c_int as u8_0,
        ];
        static mut iValue: [u8_0; 8] = [
            1 as ::core::ffi::c_int as u8_0,
            0 as ::core::ffi::c_int as u8_0,
            0 as ::core::ffi::c_int as u8_0,
            0 as ::core::ffi::c_int as u8_0,
            1 as ::core::ffi::c_int as u8_0,
            1 as ::core::ffi::c_int as u8_0,
            3 as ::core::ffi::c_int as u8_0,
            2 as ::core::ffi::c_int as u8_0,
        ];
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(*z as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & 0x4 as ::core::ffi::c_int != 0
        {
            return sqlite3Atoi(z) as u8_0;
        }
        n = sqlite3Strlen30(z);
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[u8_0; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<u8_0>() as usize)
                as ::core::ffi::c_int
        {
            if iLength[i as usize] as ::core::ffi::c_int == n
                && sqlite3_strnicmp(
                    (&raw const zText as *const ::core::ffi::c_char)
                        .offset(
                            *(&raw const iOffset as *const u8_0).offset(i as isize)
                                as isize,
                        ) as *const ::core::ffi::c_char,
                    z,
                    n,
                ) == 0 as ::core::ffi::c_int
                && (omitFull == 0
                    || iValue[i as usize] as ::core::ffi::c_int
                        <= 1 as ::core::ffi::c_int)
            {
                return iValue[i as usize];
            }
            i += 1;
        }
        return dflt;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3GetBoolean(
    mut z: *const ::core::ffi::c_char,
    mut dflt: u8_0,
) -> u8_0 {
    unsafe {
        return (getSafetyLevel(z, 1 as ::core::ffi::c_int, dflt) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as u8_0;
    }
}
unsafe extern "C" fn getLockingMode(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if !z.is_null() {
            if 0 as ::core::ffi::c_int
                == sqlite3StrICmp(
                    z,
                    b"exclusive\0".as_ptr() as *const ::core::ffi::c_char,
                )
            {
                return PAGER_LOCKINGMODE_EXCLUSIVE;
            }
            if 0 as ::core::ffi::c_int
                == sqlite3StrICmp(z, b"normal\0".as_ptr() as *const ::core::ffi::c_char)
            {
                return PAGER_LOCKINGMODE_NORMAL;
            }
        }
        return PAGER_LOCKINGMODE_QUERY;
    }
}
unsafe extern "C" fn getAutoVacuum(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if 0 as ::core::ffi::c_int
            == sqlite3StrICmp(z, b"none\0".as_ptr() as *const ::core::ffi::c_char)
        {
            return BTREE_AUTOVACUUM_NONE;
        }
        if 0 as ::core::ffi::c_int
            == sqlite3StrICmp(z, b"full\0".as_ptr() as *const ::core::ffi::c_char)
        {
            return BTREE_AUTOVACUUM_FULL;
        }
        if 0 as ::core::ffi::c_int
            == sqlite3StrICmp(z, b"incremental\0".as_ptr() as *const ::core::ffi::c_char)
        {
            return BTREE_AUTOVACUUM_INCR;
        }
        i = sqlite3Atoi(z);
        return (if i >= 0 as ::core::ffi::c_int && i <= 2 as ::core::ffi::c_int {
            i
        } else {
            0 as ::core::ffi::c_int
        }) as u8_0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn getTempStore(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >= '0' as i32
            && *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                <= '2' as i32
        {
            return *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - '0' as i32
        } else if sqlite3StrICmp(z, b"file\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int
        } else if sqlite3StrICmp(z, b"memory\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return 2 as ::core::ffi::c_int
        } else {
            return 0 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn invalidateTempStorage(
    mut pParse: *mut Parse,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        if !(*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pBt.is_null() {
            if (*db).autoCommit == 0
                || sqlite3BtreeTxnState(
                    (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pBt,
                ) != SQLITE_TXN_NONE
            {
                sqlite3ErrorMsg(
                    pParse,
                    b"temporary storage cannot be changed from within a transaction\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                return SQLITE_ERROR;
            }
            sqlite3BtreeClose((*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pBt);
            let ref mut c2rust_fresh8 = (*(*db)
                .aDb
                .offset(1 as ::core::ffi::c_int as isize))
                .pBt;
            *c2rust_fresh8 = ::core::ptr::null_mut::<Btree>();
            sqlite3ResetAllSchemasOfConnection(db);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn changeTempStorage(
    mut pParse: *mut Parse,
    mut zStorageType: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ts: ::core::ffi::c_int = getTempStore(zStorageType);
        let mut db: *mut sqlite3 = (*pParse).db;
        if (*db).temp_store as ::core::ffi::c_int == ts {
            return SQLITE_OK;
        }
        if invalidateTempStorage(pParse) != SQLITE_OK {
            return SQLITE_ERROR;
        }
        (*db).temp_store = ts as u8_0;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn setPragmaResultColumnNames(
    mut v: *mut Vdbe,
    mut pPragma: *const PragmaName,
) {
    unsafe {
        let mut n: u8_0 = (*pPragma).nPragCName;
        sqlite3VdbeSetNumCols(
            v,
            if n as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                n as ::core::ffi::c_int
            },
        );
        if n as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            sqlite3VdbeSetColName(
                v,
                0 as ::core::ffi::c_int,
                COLNAME_NAME,
                (*pPragma).zName,
                SQLITE_STATIC,
            );
        } else {
            let mut i: ::core::ffi::c_int = 0;
            let mut j: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            j = (*pPragma).iPragCName as ::core::ffi::c_int;
            while i < n as ::core::ffi::c_int {
                sqlite3VdbeSetColName(
                    v,
                    i,
                    COLNAME_NAME,
                    pragCName[j as usize],
                    SQLITE_STATIC,
                );
                i += 1;
                j += 1;
            }
        };
    }
}
unsafe extern "C" fn returnSingleInt(mut v: *mut Vdbe, mut value: i64_0) {
    unsafe {
        sqlite3VdbeAddOp4Dup8(
            v,
            OP_Int64,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw mut value as *const u8_0,
            P4_INT64,
        );
        sqlite3VdbeAddOp2(
            v,
            OP_ResultRow,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn returnSingleText(
    mut v: *mut Vdbe,
    mut zValue: *const ::core::ffi::c_char,
) {
    unsafe {
        if !zValue.is_null() {
            sqlite3VdbeLoadString(v, 1 as ::core::ffi::c_int, zValue);
            sqlite3VdbeAddOp2(
                v,
                OP_ResultRow,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
        }
    }
}
unsafe extern "C" fn setAllPagerFlags(mut db: *mut sqlite3) {
    unsafe {
        if (*db).autoCommit != 0 {
            let mut pDb: *mut Db = (*db).aDb;
            let mut n: ::core::ffi::c_int = (*db).nDb;
            loop {
                let c2rust_fresh7 = n;
                n = n - 1;
                if !(c2rust_fresh7 > 0 as ::core::ffi::c_int) {
                    break;
                }
                if !(*pDb).pBt.is_null() {
                    sqlite3BtreeSetPagerFlags(
                        (*pDb).pBt,
                        ((*pDb).safety_level as ::core::ffi::c_ulonglong
                            | (*db).flags as ::core::ffi::c_ulonglong
                                & PAGER_FLAGS_MASK as ::core::ffi::c_ulonglong)
                            as ::core::ffi::c_uint,
                    );
                }
                pDb = pDb.offset(1);
            }
        }
    }
}
unsafe extern "C" fn actionName(mut action: u8_0) -> *const ::core::ffi::c_char {
    unsafe {
        let mut zName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        match action as ::core::ffi::c_int {
            OE_SetNull => {
                zName = b"SET NULL\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OE_SetDflt => {
                zName = b"SET DEFAULT\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OE_Cascade => {
                zName = b"CASCADE\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OE_Restrict => {
                zName = b"RESTRICT\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {
                zName = b"NO ACTION\0".as_ptr() as *const ::core::ffi::c_char;
            }
        }
        return zName;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3JournalModename(
    mut eMode: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        static mut azModeName: [*mut ::core::ffi::c_char; 6] = [
            b"delete\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"persist\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"off\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"truncate\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"memory\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"wal\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ];
        if eMode
            == (::core::mem::size_of::<[*mut ::core::ffi::c_char; 6]>() as usize)
                .wrapping_div(
                    ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                ) as ::core::ffi::c_int
        {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        return azModeName[eMode as usize];
    }
}
unsafe extern "C" fn pragmaLocate(
    mut zName: *const ::core::ffi::c_char,
) -> *const PragmaName {
    unsafe {
        let mut upr: ::core::ffi::c_int = 0;
        let mut lwr: ::core::ffi::c_int = 0;
        let mut mid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = 0;
        lwr = 0 as ::core::ffi::c_int;
        upr = (::core::mem::size_of::<[PragmaName; 65]>() as usize)
            .wrapping_div(::core::mem::size_of::<PragmaName>() as usize)
            as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while lwr <= upr {
            mid = (lwr + upr) / 2 as ::core::ffi::c_int;
            rc = sqlite3_stricmp(zName, aPragmaName[mid as usize].zName);
            if rc == 0 as ::core::ffi::c_int {
                break;
            }
            if rc < 0 as ::core::ffi::c_int {
                upr = mid - 1 as ::core::ffi::c_int;
            } else {
                lwr = mid + 1 as ::core::ffi::c_int;
            }
        }
        return if lwr > upr {
            ::core::ptr::null::<PragmaName>()
        } else {
            (&raw const aPragmaName as *const PragmaName).offset(mid as isize)
                as *const PragmaName
        };
    }
}
unsafe extern "C" fn pragmaFunclistLine(
    mut v: *mut Vdbe,
    mut p: *mut FuncDef,
    mut isBuiltin: ::core::ffi::c_int,
    mut showInternFuncs: ::core::ffi::c_int,
) {
    unsafe {
        let mut mask: u32_0 = (SQLITE_DETERMINISTIC | SQLITE_DIRECTONLY | SQLITE_SUBTYPE
            | SQLITE_INNOCUOUS | SQLITE_FUNC_INTERNAL) as u32_0;
        if showInternFuncs != 0 {
            mask = 0xffffffff as ::core::ffi::c_uint as u32_0;
        }
        while !p.is_null() {
            let mut zType: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            static mut azEnc: [*const ::core::ffi::c_char; 4] = [
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"utf8\0".as_ptr() as *const ::core::ffi::c_char,
                b"utf16le\0".as_ptr() as *const ::core::ffi::c_char,
                b"utf16be\0".as_ptr() as *const ::core::ffi::c_char,
            ];
            if !(*p).xSFunc.is_none() {
                if !((*p).funcFlags as ::core::ffi::c_uint
                    & SQLITE_FUNC_INTERNAL as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                    && showInternFuncs == 0 as ::core::ffi::c_int)
                {
                    if (*p).xValue.is_some() {
                        zType = b"w\0".as_ptr() as *const ::core::ffi::c_char;
                    } else if (*p).xFinalize.is_some() {
                        zType = b"a\0".as_ptr() as *const ::core::ffi::c_char;
                    } else {
                        zType = b"s\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                    sqlite3VdbeMultiLoad(
                        v,
                        1 as ::core::ffi::c_int,
                        b"sissii\0".as_ptr() as *const ::core::ffi::c_char,
                        (*p).zName,
                        isBuiltin,
                        zType,
                        azEnc[((*p).funcFlags as ::core::ffi::c_uint
                            & SQLITE_FUNC_ENCMASK as ::core::ffi::c_uint) as usize],
                        (*p).nArg as ::core::ffi::c_int,
                        (*p).funcFlags as ::core::ffi::c_uint
                            & mask as ::core::ffi::c_uint
                            ^ SQLITE_INNOCUOUS as ::core::ffi::c_uint,
                    );
                }
            }
            p = (*p).pNext;
        }
    }
}
unsafe extern "C" fn integrityCheckResultRow(mut v: *mut Vdbe) -> ::core::ffi::c_int {
    unsafe {
        let mut addr: ::core::ffi::c_int = 0;
        sqlite3VdbeAddOp2(
            v,
            OP_ResultRow,
            3 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        addr = sqlite3VdbeAddOp3(
            v,
            OP_IfPos,
            1 as ::core::ffi::c_int,
            sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        sqlite3VdbeAddOp0(v, OP_Halt);
        return addr;
    }
}
unsafe extern "C" fn tableSkipIntegrityCheck(
    mut pTab: *const Table,
    mut pObjTab: *const Table,
) -> ::core::ffi::c_int {
    unsafe {
        if !pObjTab.is_null() {
            return (pTab != pObjTab) as ::core::ffi::c_int
        } else {
            return ((*pTab).tabFlags as ::core::ffi::c_uint
                & TF_Imposter as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint)
                as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Pragma(
    mut pParse: *mut Parse,
    mut pId1: *mut Token,
    mut pId2: *mut Token,
    mut pValue: *mut Token,
    mut minusFlag: ::core::ffi::c_int,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut zLeft: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zRight: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pId: *mut Token = ::core::ptr::null_mut::<Token>();
        let mut aFcntl: [*mut ::core::ffi::c_char; 4] = [::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >(); 4];
        let mut iDb: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pDb: *mut Db = ::core::ptr::null_mut::<Db>();
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        let mut pPragma: *const PragmaName = ::core::ptr::null::<PragmaName>();
        if v.is_null() {
            return;
        }
        sqlite3VdbeRunOnlyOnce(v);
        (*pParse).nMem = 2 as ::core::ffi::c_int;
        iDb = sqlite3TwoPartName(pParse, pId1, pId2, &raw mut pId);
        if iDb < 0 as ::core::ffi::c_int {
            return;
        }
        pDb = (*db).aDb.offset(iDb as isize) as *mut Db;
        if iDb == 1 as ::core::ffi::c_int && sqlite3OpenTempDatabase(pParse) != 0 {
            return;
        }
        zLeft = sqlite3NameFromToken(db, pId);
        if zLeft.is_null() {
            return;
        }
        if minusFlag != 0 {
            zRight = sqlite3MPrintf(
                db,
                b"-%T\0".as_ptr() as *const ::core::ffi::c_char,
                pValue,
            );
        } else {
            zRight = sqlite3NameFromToken(db, pValue);
        }
        zDb = if (*pId2).n > 0 as ::core::ffi::c_uint {
            (*pDb).zDbSName
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        if !(sqlite3AuthCheck(pParse, SQLITE_PRAGMA, zLeft, zRight, zDb) != 0) {
            aFcntl[0 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            aFcntl[1 as ::core::ffi::c_int as usize] = zLeft;
            aFcntl[2 as ::core::ffi::c_int as usize] = zRight;
            aFcntl[3 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            (*db).busyHandler.nBusy = 0 as ::core::ffi::c_int;
            rc = sqlite3_file_control(
                db,
                zDb,
                SQLITE_FCNTL_PRAGMA,
                &raw mut aFcntl as *mut *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
            );
            if rc == SQLITE_OK {
                sqlite3VdbeSetNumCols(v, 1 as ::core::ffi::c_int);
                sqlite3VdbeSetColName(
                    v,
                    0 as ::core::ffi::c_int,
                    COLNAME_NAME,
                    aFcntl[0 as ::core::ffi::c_int as usize],
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                returnSingleText(v, aFcntl[0 as ::core::ffi::c_int as usize]);
                sqlite3_free(
                    aFcntl[0 as ::core::ffi::c_int as usize] as *mut ::core::ffi::c_void,
                );
            } else if rc != SQLITE_NOTFOUND {
                if !aFcntl[0 as ::core::ffi::c_int as usize].is_null() {
                    sqlite3ErrorMsg(
                        pParse,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        aFcntl[0 as ::core::ffi::c_int as usize],
                    );
                    sqlite3_free(
                        aFcntl[0 as ::core::ffi::c_int as usize]
                            as *mut ::core::ffi::c_void,
                    );
                }
                (*pParse).nErr += 1;
                (*pParse).rc = rc;
            } else {
                pPragma = pragmaLocate(zLeft);
                if !pPragma.is_null() {
                    if (*pPragma).mPragFlg as ::core::ffi::c_int & PragFlg_NeedSchema
                        != 0 as ::core::ffi::c_int
                    {
                        if sqlite3ReadSchema(pParse) != 0 {
                            c2rust_current_block = 4749248788619550567;
                        } else {
                            c2rust_current_block = 8845338526596852646;
                        }
                    } else {
                        c2rust_current_block = 8845338526596852646;
                    }
                    match c2rust_current_block {
                        4749248788619550567 => {}
                        _ => {
                            if (*pPragma).mPragFlg as ::core::ffi::c_int
                                & PragFlg_NoColumns == 0 as ::core::ffi::c_int
                                && ((*pPragma).mPragFlg as ::core::ffi::c_int
                                    & PragFlg_NoColumns1 == 0 as ::core::ffi::c_int
                                    || zRight.is_null())
                            {
                                setPragmaResultColumnNames(v, pPragma);
                            }
                            match (*pPragma).ePragTyp as ::core::ffi::c_int {
                                PragTyp_PAGE_SIZE => {
                                    let mut pBt: *mut Btree = (*pDb).pBt;
                                    if zRight.is_null() {
                                        let mut size: ::core::ffi::c_int = if !pBt.is_null() {
                                            sqlite3BtreeGetPageSize(pBt)
                                        } else {
                                            0 as ::core::ffi::c_int
                                        };
                                        returnSingleInt(v, size as i64_0);
                                    } else {
                                        (*db).nextPagesize = sqlite3Atoi(zRight);
                                        if SQLITE_NOMEM
                                            == sqlite3BtreeSetPageSize(
                                                pBt,
                                                (*db).nextPagesize,
                                                0 as ::core::ffi::c_int,
                                                0 as ::core::ffi::c_int,
                                            )
                                        {
                                            sqlite3OomFault(db);
                                        }
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_SECURE_DELETE => {
                                    let mut pBt_0: *mut Btree = (*pDb).pBt;
                                    let mut b: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                                    if !zRight.is_null() {
                                        if sqlite3_stricmp(
                                            zRight,
                                            b"fast\0".as_ptr() as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            b = 2 as ::core::ffi::c_int;
                                        } else {
                                            b = sqlite3GetBoolean(zRight, 0 as u8_0)
                                                as ::core::ffi::c_int;
                                        }
                                    }
                                    if (*pId2).n == 0 as ::core::ffi::c_uint
                                        && b >= 0 as ::core::ffi::c_int
                                    {
                                        let mut ii: ::core::ffi::c_int = 0;
                                        ii = 0 as ::core::ffi::c_int;
                                        while ii < (*db).nDb {
                                            sqlite3BtreeSecureDelete(
                                                (*(*db).aDb.offset(ii as isize)).pBt,
                                                b,
                                            );
                                            ii += 1;
                                        }
                                    }
                                    b = sqlite3BtreeSecureDelete(pBt_0, b);
                                    returnSingleInt(v, b as i64_0);
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_PAGE_COUNT => {
                                    let mut iReg: ::core::ffi::c_int = 0;
                                    let mut x: i64_0 = 0 as i64_0;
                                    sqlite3CodeVerifySchema(pParse, iDb);
                                    (*pParse).nMem += 1;
                                    iReg = (*pParse).nMem;
                                    if *(&raw const sqlite3UpperToLower
                                        as *const ::core::ffi::c_uchar)
                                        .offset(
                                            *zLeft.offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uchar as isize,
                                        ) as ::core::ffi::c_int == 'p' as i32
                                    {
                                        sqlite3VdbeAddOp2(v, OP_Pagecount, iDb, iReg);
                                    } else {
                                        if !zRight.is_null()
                                            && sqlite3DecOrHexToI64(zRight, &raw mut x)
                                                == 0 as ::core::ffi::c_int
                                        {
                                            if x < 0 as ::core::ffi::c_longlong {
                                                x = 0 as i64_0;
                                            } else if x > 0xfffffffe as ::core::ffi::c_longlong {
                                                x = 0xfffffffe as i64_0;
                                            }
                                        } else {
                                            x = 0 as i64_0;
                                        }
                                        sqlite3VdbeAddOp3(
                                            v,
                                            OP_MaxPgcnt,
                                            iDb,
                                            iReg,
                                            x as ::core::ffi::c_int,
                                        );
                                    }
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_ResultRow,
                                        iReg,
                                        1 as ::core::ffi::c_int,
                                    );
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_LOCKING_MODE => {
                                    let mut zRet: *const ::core::ffi::c_char = b"normal\0"
                                        .as_ptr() as *const ::core::ffi::c_char;
                                    let mut eMode: ::core::ffi::c_int = getLockingMode(zRight);
                                    if (*pId2).n == 0 as ::core::ffi::c_uint
                                        && eMode == PAGER_LOCKINGMODE_QUERY
                                    {
                                        eMode = (*db).dfltLockMode as ::core::ffi::c_int;
                                    } else {
                                        let mut pPager: *mut Pager = ::core::ptr::null_mut::<
                                            Pager,
                                        >();
                                        if (*pId2).n == 0 as ::core::ffi::c_uint {
                                            let mut ii_0: ::core::ffi::c_int = 0;
                                            ii_0 = 2 as ::core::ffi::c_int;
                                            while ii_0 < (*db).nDb {
                                                pPager = sqlite3BtreePager(
                                                    (*(*db).aDb.offset(ii_0 as isize)).pBt,
                                                ) as *mut Pager;
                                                sqlite3PagerLockingMode(pPager, eMode);
                                                ii_0 += 1;
                                            }
                                            (*db).dfltLockMode = eMode as u8_0;
                                        }
                                        pPager = sqlite3BtreePager((*pDb).pBt) as *mut Pager;
                                        eMode = sqlite3PagerLockingMode(pPager, eMode);
                                    }
                                    if eMode == PAGER_LOCKINGMODE_EXCLUSIVE {
                                        zRet = b"exclusive\0".as_ptr()
                                            as *const ::core::ffi::c_char;
                                    }
                                    returnSingleText(v, zRet);
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_JOURNAL_MODE => {
                                    let mut eMode_0: ::core::ffi::c_int = 0;
                                    let mut ii_1: ::core::ffi::c_int = 0;
                                    if zRight.is_null() {
                                        eMode_0 = PAGER_JOURNALMODE_QUERY;
                                    } else {
                                        let mut zMode: *const ::core::ffi::c_char = ::core::ptr::null::<
                                            ::core::ffi::c_char,
                                        >();
                                        let mut n: ::core::ffi::c_int = sqlite3Strlen30(zRight);
                                        eMode_0 = 0 as ::core::ffi::c_int;
                                        loop {
                                            zMode = sqlite3JournalModename(eMode_0);
                                            if zMode.is_null() {
                                                break;
                                            }
                                            if sqlite3_strnicmp(zRight, zMode, n)
                                                == 0 as ::core::ffi::c_int
                                            {
                                                break;
                                            }
                                            eMode_0 += 1;
                                        }
                                        if zMode.is_null() {
                                            eMode_0 = PAGER_JOURNALMODE_QUERY;
                                        }
                                        if eMode_0 == PAGER_JOURNALMODE_OFF
                                            && (*db).flags as ::core::ffi::c_ulonglong
                                                & SQLITE_Defensive as ::core::ffi::c_ulonglong
                                                != 0 as ::core::ffi::c_ulonglong
                                        {
                                            eMode_0 = PAGER_JOURNALMODE_QUERY;
                                        }
                                    }
                                    if eMode_0 == PAGER_JOURNALMODE_QUERY
                                        && (*pId2).n == 0 as ::core::ffi::c_uint
                                    {
                                        iDb = 0 as ::core::ffi::c_int;
                                        (*pId2).n = 1 as ::core::ffi::c_uint;
                                    }
                                    ii_1 = (*db).nDb - 1 as ::core::ffi::c_int;
                                    while ii_1 >= 0 as ::core::ffi::c_int {
                                        if !(*(*db).aDb.offset(ii_1 as isize)).pBt.is_null()
                                            && (ii_1 == iDb || (*pId2).n == 0 as ::core::ffi::c_uint)
                                        {
                                            sqlite3VdbeUsesBtree(v, ii_1);
                                            sqlite3VdbeAddOp3(
                                                v,
                                                OP_JournalMode,
                                                ii_1,
                                                1 as ::core::ffi::c_int,
                                                eMode_0,
                                            );
                                        }
                                        ii_1 -= 1;
                                    }
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_ResultRow,
                                        1 as ::core::ffi::c_int,
                                        1 as ::core::ffi::c_int,
                                    );
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_JOURNAL_SIZE_LIMIT => {
                                    let mut pPager_0: *mut Pager = sqlite3BtreePager((*pDb).pBt)
                                        as *mut Pager;
                                    let mut iLimit: i64_0 = -2 as ::core::ffi::c_int as i64_0;
                                    if !zRight.is_null() {
                                        sqlite3DecOrHexToI64(zRight, &raw mut iLimit);
                                        if iLimit
                                            < -1 as ::core::ffi::c_int as ::core::ffi::c_longlong
                                        {
                                            iLimit = -1 as ::core::ffi::c_int as i64_0;
                                        }
                                    }
                                    iLimit = sqlite3PagerJournalSizeLimit(pPager_0, iLimit);
                                    returnSingleInt(v, iLimit);
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_AUTO_VACUUM => {
                                    let mut pBt_1: *mut Btree = (*pDb).pBt;
                                    if zRight.is_null() {
                                        returnSingleInt(
                                            v,
                                            sqlite3BtreeGetAutoVacuum(pBt_1) as i64_0,
                                        );
                                    } else {
                                        let mut eAuto: ::core::ffi::c_int = getAutoVacuum(zRight);
                                        (*db).nextAutovac = eAuto as u8_0 as ::core::ffi::c_schar;
                                        rc = sqlite3BtreeSetAutoVacuum(pBt_1, eAuto);
                                        if rc == SQLITE_OK
                                            && (eAuto == 1 as ::core::ffi::c_int
                                                || eAuto == 2 as ::core::ffi::c_int)
                                        {
                                            static mut iLn: ::core::ffi::c_int = 0
                                                as ::core::ffi::c_int;
                                            static mut setMeta6: [VdbeOpList; 5] = [
                                                VdbeOpList {
                                                    opcode: OP_Transaction as u8_0,
                                                    p1: 0 as ::core::ffi::c_schar,
                                                    p2: 1 as ::core::ffi::c_schar,
                                                    p3: 0 as ::core::ffi::c_schar

                                                },
                                                VdbeOpList {
                                                    opcode: OP_ReadCookie as u8_0,
                                                    p1: 0 as ::core::ffi::c_schar,
                                                    p2: 1 as ::core::ffi::c_schar,
                                                    p3: BTREE_LARGEST_ROOT_PAGE as ::core::ffi::c_schar

                                                },
                                                VdbeOpList {
                                                    opcode: OP_If as u8_0,
                                                    p1: 1 as ::core::ffi::c_schar,
                                                    p2: 0 as ::core::ffi::c_schar,
                                                    p3: 0 as ::core::ffi::c_schar

                                                },
                                                VdbeOpList {
                                                    opcode: OP_Halt as u8_0,
                                                    p1: SQLITE_OK as ::core::ffi::c_schar,
                                                    p2: OE_Abort as ::core::ffi::c_schar,
                                                    p3: 0 as ::core::ffi::c_schar

                                                },
                                                VdbeOpList {
                                                    opcode: OP_SetCookie as u8_0,
                                                    p1: 0 as ::core::ffi::c_schar,
                                                    p2: BTREE_INCR_VACUUM as ::core::ffi::c_schar,
                                                    p3: 0 as ::core::ffi::c_schar

                                                },
                                            ];
                                            let mut aOp: *mut VdbeOp = ::core::ptr::null_mut::<
                                                VdbeOp,
                                            >();
                                            let mut iAddr: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(
                                                v,
                                            );
                                            aOp = sqlite3VdbeAddOpList(
                                                v,
                                                (::core::mem::size_of::<[VdbeOpList; 5]>() as usize)
                                                    .wrapping_div(::core::mem::size_of::<VdbeOpList>() as usize)
                                                    as ::core::ffi::c_int,
                                                &raw const setMeta6 as *const VdbeOpList,
                                                iLn,
                                            );
                                            (*aOp.offset(0 as ::core::ffi::c_int as isize)).p1 = iDb;
                                            (*aOp.offset(1 as ::core::ffi::c_int as isize)).p1 = iDb;
                                            (*aOp.offset(2 as ::core::ffi::c_int as isize)).p2 = iAddr
                                                + 4 as ::core::ffi::c_int;
                                            (*aOp.offset(4 as ::core::ffi::c_int as isize)).p1 = iDb;
                                            (*aOp.offset(4 as ::core::ffi::c_int as isize)).p3 = eAuto
                                                - 1 as ::core::ffi::c_int;
                                            sqlite3VdbeUsesBtree(v, iDb);
                                        }
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_INCREMENTAL_VACUUM => {
                                    let mut iLimit_0: ::core::ffi::c_int = 0
                                        as ::core::ffi::c_int;
                                    let mut addr: ::core::ffi::c_int = 0;
                                    if zRight.is_null()
                                        || sqlite3GetInt32(zRight, &raw mut iLimit_0) == 0
                                        || iLimit_0 <= 0 as ::core::ffi::c_int
                                    {
                                        iLimit_0 = 0x7fffffff as ::core::ffi::c_int;
                                    }
                                    sqlite3BeginWriteOperation(
                                        pParse,
                                        0 as ::core::ffi::c_int,
                                        iDb,
                                    );
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_Integer,
                                        iLimit_0,
                                        1 as ::core::ffi::c_int,
                                    );
                                    addr = sqlite3VdbeAddOp1(v, OP_IncrVacuum, iDb);
                                    sqlite3VdbeAddOp1(v, OP_ResultRow, 1 as ::core::ffi::c_int);
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_AddImm,
                                        1 as ::core::ffi::c_int,
                                        -1 as ::core::ffi::c_int,
                                    );
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_IfPos,
                                        1 as ::core::ffi::c_int,
                                        addr,
                                    );
                                    sqlite3VdbeJumpHere(v, addr);
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_CACHE_SIZE => {
                                    if zRight.is_null() {
                                        returnSingleInt(v, (*(*pDb).pSchema).cache_size as i64_0);
                                    } else {
                                        let mut size_0: ::core::ffi::c_int = sqlite3Atoi(zRight);
                                        (*(*pDb).pSchema).cache_size = size_0;
                                        sqlite3BtreeSetCacheSize(
                                            (*pDb).pBt,
                                            (*(*pDb).pSchema).cache_size,
                                        );
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_CACHE_SPILL => {
                                    if zRight.is_null() {
                                        returnSingleInt(
                                            v,
                                            (if (*db).flags as ::core::ffi::c_ulonglong
                                                & SQLITE_CacheSpill as ::core::ffi::c_ulonglong
                                                == 0 as ::core::ffi::c_ulonglong
                                            {
                                                0 as ::core::ffi::c_int
                                            } else {
                                                sqlite3BtreeSetSpillSize(
                                                    (*pDb).pBt,
                                                    0 as ::core::ffi::c_int,
                                                )
                                            }) as i64_0,
                                        );
                                    } else {
                                        let mut size_1: ::core::ffi::c_int = 1
                                            as ::core::ffi::c_int;
                                        if sqlite3GetInt32(zRight, &raw mut size_1) != 0 {
                                            sqlite3BtreeSetSpillSize((*pDb).pBt, size_1);
                                        }
                                        if sqlite3GetBoolean(
                                            zRight,
                                            (size_1 != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                                                as u8_0,
                                        ) != 0
                                        {
                                            (*db).flags
                                                |= SQLITE_CacheSpill as ::core::ffi::c_ulonglong;
                                        } else {
                                            (*db).flags
                                                &= !(SQLITE_CacheSpill as u64_0)
                                                    as ::core::ffi::c_ulonglong;
                                        }
                                        setAllPagerFlags(db);
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_MMAP_SIZE => {
                                    let mut sz: sqlite3_int64 = 0;
                                    if !zRight.is_null() {
                                        let mut ii_2: ::core::ffi::c_int = 0;
                                        sqlite3DecOrHexToI64(zRight, &raw mut sz);
                                        if sz < 0 as ::core::ffi::c_longlong {
                                            sz = sqlite3Config.szMmap;
                                        }
                                        if (*pId2).n == 0 as ::core::ffi::c_uint {
                                            (*db).szMmap = sz as i64_0;
                                        }
                                        ii_2 = (*db).nDb - 1 as ::core::ffi::c_int;
                                        while ii_2 >= 0 as ::core::ffi::c_int {
                                            if !(*(*db).aDb.offset(ii_2 as isize)).pBt.is_null()
                                                && (ii_2 == iDb || (*pId2).n == 0 as ::core::ffi::c_uint)
                                            {
                                                sqlite3BtreeSetMmapLimit(
                                                    (*(*db).aDb.offset(ii_2 as isize)).pBt,
                                                    sz,
                                                );
                                            }
                                            ii_2 -= 1;
                                        }
                                    }
                                    sz = -1 as ::core::ffi::c_int as sqlite3_int64;
                                    rc = sqlite3_file_control(
                                        db,
                                        zDb,
                                        SQLITE_FCNTL_MMAP_SIZE,
                                        &raw mut sz as *mut ::core::ffi::c_void,
                                    );
                                    if rc == SQLITE_OK {
                                        returnSingleInt(v, sz as i64_0);
                                    } else if rc != SQLITE_NOTFOUND {
                                        (*pParse).nErr += 1;
                                        (*pParse).rc = rc;
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_TEMP_STORE => {
                                    if zRight.is_null() {
                                        returnSingleInt(v, (*db).temp_store as i64_0);
                                    } else {
                                        changeTempStorage(pParse, zRight);
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_TEMP_STORE_DIRECTORY => {
                                    sqlite3_mutex_enter(
                                        sqlite3MutexAlloc(SQLITE_MUTEX_STATIC_TEMPDIR),
                                    );
                                    if zRight.is_null() {
                                        returnSingleText(v, sqlite3_temp_directory);
                                        c2rust_current_block = 7192749559210104566;
                                    } else {
                                        if *zRight.offset(0 as ::core::ffi::c_int as isize) != 0 {
                                            let mut res: ::core::ffi::c_int = 0;
                                            rc = sqlite3OsAccess(
                                                (*db).pVfs,
                                                zRight,
                                                SQLITE_ACCESS_READWRITE,
                                                &raw mut res,
                                            );
                                            if rc != SQLITE_OK || res == 0 as ::core::ffi::c_int {
                                                sqlite3ErrorMsg(
                                                    pParse,
                                                    b"not a writable directory\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                sqlite3_mutex_leave(
                                                    sqlite3MutexAlloc(SQLITE_MUTEX_STATIC_TEMPDIR),
                                                );
                                                c2rust_current_block = 4749248788619550567;
                                            } else {
                                                c2rust_current_block = 7868909573868156133;
                                            }
                                        } else {
                                            c2rust_current_block = 7868909573868156133;
                                        }
                                        match c2rust_current_block {
                                            4749248788619550567 => {}
                                            _ => {
                                                if SQLITE_TEMP_STORE == 0 as ::core::ffi::c_int
                                                    || SQLITE_TEMP_STORE == 1 as ::core::ffi::c_int
                                                        && (*db).temp_store as ::core::ffi::c_int
                                                            <= 1 as ::core::ffi::c_int
                                                    || SQLITE_TEMP_STORE == 2 as ::core::ffi::c_int
                                                        && (*db).temp_store as ::core::ffi::c_int
                                                            == 1 as ::core::ffi::c_int
                                                {
                                                    invalidateTempStorage(pParse);
                                                }
                                                sqlite3_free(
                                                    sqlite3_temp_directory as *mut ::core::ffi::c_void,
                                                );
                                                if *zRight.offset(0 as ::core::ffi::c_int as isize) != 0 {
                                                    sqlite3_temp_directory = sqlite3_mprintf(
                                                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                                        zRight,
                                                    );
                                                } else {
                                                    sqlite3_temp_directory = ::core::ptr::null_mut::<
                                                        ::core::ffi::c_char,
                                                    >();
                                                }
                                                c2rust_current_block = 7192749559210104566;
                                            }
                                        }
                                    }
                                    match c2rust_current_block {
                                        4749248788619550567 => {}
                                        _ => {
                                            sqlite3_mutex_leave(
                                                sqlite3MutexAlloc(SQLITE_MUTEX_STATIC_TEMPDIR),
                                            );
                                            c2rust_current_block = 10387754024080658218;
                                        }
                                    }
                                }
                                PragTyp_SYNCHRONOUS => {
                                    if zRight.is_null() {
                                        returnSingleInt(
                                            v,
                                            ((*pDb).safety_level as ::core::ffi::c_int
                                                - 1 as ::core::ffi::c_int) as i64_0,
                                        );
                                    } else if (*db).autoCommit == 0 {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            b"Safety level may not be changed inside a transaction\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                        );
                                    } else if iDb != 1 as ::core::ffi::c_int {
                                        let mut iLevel: ::core::ffi::c_int = getSafetyLevel(
                                            zRight,
                                            0 as ::core::ffi::c_int,
                                            1 as u8_0,
                                        ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                            & PAGER_SYNCHRONOUS_MASK;
                                        if iLevel == 0 as ::core::ffi::c_int {
                                            iLevel = 1 as ::core::ffi::c_int;
                                        }
                                        (*pDb).safety_level = iLevel as u8_0;
                                        (*pDb).bSyncSet = 1 as u8_0;
                                        setAllPagerFlags(db);
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_FLAG => {
                                    if zRight.is_null() {
                                        setPragmaResultColumnNames(v, pPragma);
                                        returnSingleInt(
                                            v,
                                            ((*db).flags & (*pPragma).iArg
                                                != 0 as ::core::ffi::c_ulonglong) as ::core::ffi::c_int
                                                as i64_0,
                                        );
                                    } else {
                                        let mut mask: u64_0 = (*pPragma).iArg;
                                        if (*db).autoCommit as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                        {
                                            mask
                                                &= !(0x4000 as ::core::ffi::c_int)
                                                    as ::core::ffi::c_ulonglong;
                                        }
                                        if sqlite3GetBoolean(zRight, 0 as u8_0) != 0 {
                                            if mask as ::core::ffi::c_ulonglong
                                                & SQLITE_WriteSchema as ::core::ffi::c_ulonglong
                                                == 0 as ::core::ffi::c_ulonglong
                                                || (*db).flags as ::core::ffi::c_ulonglong
                                                    & SQLITE_Defensive as ::core::ffi::c_ulonglong
                                                    == 0 as ::core::ffi::c_ulonglong
                                            {
                                                (*db).flags |= mask as ::core::ffi::c_ulonglong;
                                            }
                                        } else {
                                            (*db).flags &= !mask as ::core::ffi::c_ulonglong;
                                            if mask == SQLITE_DeferFKs as ::core::ffi::c_ulonglong {
                                                (*db).nDeferredImmCons = 0 as i64_0;
                                                (*db).nDeferredCons = 0 as i64_0;
                                            }
                                            if mask as ::core::ffi::c_ulonglong
                                                & SQLITE_WriteSchema as ::core::ffi::c_ulonglong
                                                != 0 as ::core::ffi::c_ulonglong
                                                && sqlite3_stricmp(
                                                    zRight,
                                                    b"reset\0".as_ptr() as *const ::core::ffi::c_char,
                                                ) == 0 as ::core::ffi::c_int
                                            {
                                                sqlite3ResetAllSchemasOfConnection(db);
                                            }
                                        }
                                        sqlite3VdbeAddOp0(v, OP_Expire);
                                        setAllPagerFlags(db);
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_TABLE_INFO => {
                                    if !zRight.is_null() {
                                        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
                                        sqlite3CodeVerifyNamedSchema(pParse, zDb);
                                        pTab = sqlite3LocateTable(
                                            pParse,
                                            LOCATE_NOERR as u32_0,
                                            zRight,
                                            zDb,
                                        );
                                        if !pTab.is_null() {
                                            let mut i: ::core::ffi::c_int = 0;
                                            let mut k: ::core::ffi::c_int = 0;
                                            let mut nHidden: ::core::ffi::c_int = 0
                                                as ::core::ffi::c_int;
                                            let mut pCol: *mut Column = ::core::ptr::null_mut::<
                                                Column,
                                            >();
                                            let mut pPk: *mut Index = sqlite3PrimaryKeyIndex(pTab);
                                            (*pParse).nMem = 7 as ::core::ffi::c_int;
                                            sqlite3ViewGetColumnNames(pParse, pTab);
                                            let mut c2rust_current_block_290: u64;
                                            i = 0 as ::core::ffi::c_int;
                                            pCol = (*pTab).aCol;
                                            while i < (*pTab).nCol as ::core::ffi::c_int {
                                                let mut isHidden: ::core::ffi::c_int = 0
                                                    as ::core::ffi::c_int;
                                                let mut pColExpr: *const Expr = ::core::ptr::null::<Expr>();
                                                if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_NOINSERT
                                                    != 0
                                                {
                                                    if (*pPragma).iArg == 0 as ::core::ffi::c_ulonglong {
                                                        nHidden += 1;
                                                        c2rust_current_block_290 = 11591941514213818729;
                                                    } else {
                                                        if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_VIRTUAL
                                                            != 0
                                                        {
                                                            isHidden = 2 as ::core::ffi::c_int;
                                                        } else if (*pCol).colFlags as ::core::ffi::c_int
                                                            & COLFLAG_STORED != 0
                                                        {
                                                            isHidden = 3 as ::core::ffi::c_int;
                                                        } else {
                                                            isHidden = 1 as ::core::ffi::c_int;
                                                        }
                                                        c2rust_current_block_290 = 17222848433855212006;
                                                    }
                                                } else {
                                                    c2rust_current_block_290 = 17222848433855212006;
                                                }
                                                match c2rust_current_block_290 {
                                                    17222848433855212006 => {
                                                        if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_PRIMKEY
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            k = 0 as ::core::ffi::c_int;
                                                        } else if pPk.is_null() {
                                                            k = 1 as ::core::ffi::c_int;
                                                        } else {
                                                            k = 1 as ::core::ffi::c_int;
                                                            while k <= (*pTab).nCol as ::core::ffi::c_int
                                                                && *(*pPk)
                                                                    .aiColumn
                                                                    .offset((k - 1 as ::core::ffi::c_int) as isize)
                                                                    as ::core::ffi::c_int != i
                                                            {
                                                                k += 1;
                                                            }
                                                        }
                                                        pColExpr = sqlite3ColumnExpr(pTab, pCol);
                                                        sqlite3VdbeMultiLoad(
                                                            v,
                                                            1 as ::core::ffi::c_int,
                                                            if (*pPragma).iArg != 0 {
                                                                b"issisii\0".as_ptr() as *const ::core::ffi::c_char
                                                            } else {
                                                                b"issisi\0".as_ptr() as *const ::core::ffi::c_char
                                                            },
                                                            i - nHidden,
                                                            (*pCol).zCnName,
                                                            sqlite3ColumnType(
                                                                pCol,
                                                                b"\0".as_ptr() as *const ::core::ffi::c_char
                                                                    as *mut ::core::ffi::c_char,
                                                            ),
                                                            if (*pCol).notNull() as ::core::ffi::c_int != 0 {
                                                                1 as ::core::ffi::c_int
                                                            } else {
                                                                0 as ::core::ffi::c_int
                                                            },
                                                            if isHidden >= 2 as ::core::ffi::c_int || pColExpr.is_null()
                                                            {
                                                                ::core::ptr::null_mut::<::core::ffi::c_char>()
                                                            } else {
                                                                (*pColExpr).u.zToken
                                                            },
                                                            k,
                                                            isHidden,
                                                        );
                                                    }
                                                    _ => {}
                                                }
                                                i += 1;
                                                pCol = pCol.offset(1);
                                            }
                                        }
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_TABLE_LIST => {
                                    let mut ii_3: ::core::ffi::c_int = 0;
                                    (*pParse).nMem = 6 as ::core::ffi::c_int;
                                    sqlite3CodeVerifyNamedSchema(pParse, zDb);
                                    ii_3 = 0 as ::core::ffi::c_int;
                                    while ii_3 < (*db).nDb {
                                        let mut k_0: *mut HashElem = ::core::ptr::null_mut::<
                                            HashElem,
                                        >();
                                        let mut pHash: *mut Hash = ::core::ptr::null_mut::<Hash>();
                                        let mut initNCol: ::core::ffi::c_int = 0;
                                        if !(!zDb.is_null()
                                            && sqlite3_stricmp(
                                                zDb,
                                                (*(*db).aDb.offset(ii_3 as isize)).zDbSName,
                                            ) != 0 as ::core::ffi::c_int)
                                        {
                                            pHash = &raw mut (*(*(*db).aDb.offset(ii_3 as isize))
                                                .pSchema)
                                                .tblHash;
                                            initNCol = (*pHash).count as ::core::ffi::c_int;
                                            loop {
                                                let c2rust_fresh0 = initNCol;
                                                initNCol = initNCol - 1;
                                                if !(c2rust_fresh0 != 0) {
                                                    break;
                                                }
                                                k_0 = (*pHash).first;
                                                loop {
                                                    let mut pTab_0: *mut Table = ::core::ptr::null_mut::<
                                                        Table,
                                                    >();
                                                    if k_0.is_null() {
                                                        initNCol = 0 as ::core::ffi::c_int;
                                                        break;
                                                    } else {
                                                        pTab_0 = (*k_0).data as *mut Table;
                                                        if (*pTab_0).nCol as ::core::ffi::c_int
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            let mut zSql: *mut ::core::ffi::c_char = sqlite3MPrintf(
                                                                db,
                                                                b"SELECT*FROM\"%w\"\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                (*pTab_0).zName,
                                                            );
                                                            if !zSql.is_null() {
                                                                let mut pDummy: *mut sqlite3_stmt = ::core::ptr::null_mut::<
                                                                    sqlite3_stmt,
                                                                >();
                                                                sqlite3_prepare_v3(
                                                                    db,
                                                                    zSql,
                                                                    -1 as ::core::ffi::c_int,
                                                                    SQLITE_PREPARE_DONT_LOG as ::core::ffi::c_uint,
                                                                    &raw mut pDummy,
                                                                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                                                                );
                                                                sqlite3_finalize(pDummy);
                                                                sqlite3DbFree(db, zSql as *mut ::core::ffi::c_void);
                                                            }
                                                            if (*db).mallocFailed != 0 {
                                                                sqlite3ErrorMsg(
                                                                    (*db).pParse,
                                                                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                                                                );
                                                                (*(*db).pParse).rc = SQLITE_NOMEM_BKPT;
                                                            }
                                                            pHash = &raw mut (*(*(*db).aDb.offset(ii_3 as isize))
                                                                .pSchema)
                                                                .tblHash;
                                                            break;
                                                        } else {
                                                            k_0 = (*k_0).next;
                                                        }
                                                    }
                                                }
                                            }
                                            k_0 = (*pHash).first;
                                            while !k_0.is_null() {
                                                let mut pTab_1: *mut Table = (*k_0).data as *mut Table;
                                                let mut zType: *const ::core::ffi::c_char = ::core::ptr::null::<
                                                    ::core::ffi::c_char,
                                                >();
                                                if !(!zRight.is_null()
                                                    && sqlite3_stricmp(zRight, (*pTab_1).zName)
                                                        != 0 as ::core::ffi::c_int)
                                                {
                                                    if (*pTab_1).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
                                                        zType = b"view\0".as_ptr() as *const ::core::ffi::c_char;
                                                    } else if (*pTab_1).eTabType as ::core::ffi::c_int
                                                        == TABTYP_VTAB
                                                    {
                                                        zType = b"virtual\0".as_ptr() as *const ::core::ffi::c_char;
                                                    } else if (*pTab_1).tabFlags as ::core::ffi::c_uint
                                                        & TF_Shadow as ::core::ffi::c_uint != 0
                                                    {
                                                        zType = b"shadow\0".as_ptr() as *const ::core::ffi::c_char;
                                                    } else {
                                                        zType = b"table\0".as_ptr() as *const ::core::ffi::c_char;
                                                    }
                                                    sqlite3VdbeMultiLoad(
                                                        v,
                                                        1 as ::core::ffi::c_int,
                                                        b"sssiii\0".as_ptr() as *const ::core::ffi::c_char,
                                                        (*(*db).aDb.offset(ii_3 as isize)).zDbSName,
                                                        sqlite3PreferredTableName((*pTab_1).zName),
                                                        zType,
                                                        (*pTab_1).nCol as ::core::ffi::c_int,
                                                        ((*pTab_1).tabFlags as ::core::ffi::c_uint
                                                            & TF_WithoutRowid as ::core::ffi::c_uint
                                                            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                                                        ((*pTab_1).tabFlags as ::core::ffi::c_uint
                                                            & TF_Strict as ::core::ffi::c_uint
                                                            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                                                    );
                                                }
                                                k_0 = (*k_0).next;
                                            }
                                        }
                                        ii_3 += 1;
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_INDEX_INFO => {
                                    if !zRight.is_null() {
                                        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
                                        let mut pTab_2: *mut Table = ::core::ptr::null_mut::<
                                            Table,
                                        >();
                                        pIdx = sqlite3FindIndex(db, zRight, zDb);
                                        if pIdx.is_null() {
                                            pTab_2 = sqlite3LocateTable(
                                                pParse,
                                                LOCATE_NOERR as u32_0,
                                                zRight,
                                                zDb,
                                            );
                                            if !pTab_2.is_null()
                                                && !((*pTab_2).tabFlags as ::core::ffi::c_uint
                                                    & TF_WithoutRowid as ::core::ffi::c_uint
                                                    == 0 as ::core::ffi::c_uint)
                                            {
                                                pIdx = sqlite3PrimaryKeyIndex(pTab_2);
                                            }
                                        }
                                        if !pIdx.is_null() {
                                            let mut iIdxDb: ::core::ffi::c_int = sqlite3SchemaToIndex(
                                                db,
                                                (*pIdx).pSchema,
                                            );
                                            let mut i_0: ::core::ffi::c_int = 0;
                                            let mut mx: ::core::ffi::c_int = 0;
                                            if (*pPragma).iArg != 0 {
                                                mx = (*pIdx).nColumn as ::core::ffi::c_int;
                                                (*pParse).nMem = 6 as ::core::ffi::c_int;
                                            } else {
                                                mx = (*pIdx).nKeyCol as ::core::ffi::c_int;
                                                (*pParse).nMem = 3 as ::core::ffi::c_int;
                                            }
                                            pTab_2 = (*pIdx).pTable;
                                            sqlite3CodeVerifySchema(pParse, iIdxDb);
                                            i_0 = 0 as ::core::ffi::c_int;
                                            while i_0 < mx {
                                                let mut cnum: i16_0 = *(*pIdx)
                                                    .aiColumn
                                                    .offset(i_0 as isize);
                                                sqlite3VdbeMultiLoad(
                                                    v,
                                                    1 as ::core::ffi::c_int,
                                                    b"iisX\0".as_ptr() as *const ::core::ffi::c_char,
                                                    i_0,
                                                    cnum as ::core::ffi::c_int,
                                                    if (cnum as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                                                    } else {
                                                        (*(*pTab_2).aCol.offset(cnum as isize)).zCnName
                                                    },
                                                );
                                                if (*pPragma).iArg != 0 {
                                                    sqlite3VdbeMultiLoad(
                                                        v,
                                                        4 as ::core::ffi::c_int,
                                                        b"isiX\0".as_ptr() as *const ::core::ffi::c_char,
                                                        *(*pIdx).aSortOrder.offset(i_0 as isize)
                                                            as ::core::ffi::c_int,
                                                        *(*pIdx).azColl.offset(i_0 as isize),
                                                        (i_0 < (*pIdx).nKeyCol as ::core::ffi::c_int)
                                                            as ::core::ffi::c_int,
                                                    );
                                                }
                                                sqlite3VdbeAddOp2(
                                                    v,
                                                    OP_ResultRow,
                                                    1 as ::core::ffi::c_int,
                                                    (*pParse).nMem,
                                                );
                                                i_0 += 1;
                                            }
                                        }
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_INDEX_LIST => {
                                    if !zRight.is_null() {
                                        let mut pIdx_0: *mut Index = ::core::ptr::null_mut::<
                                            Index,
                                        >();
                                        let mut pTab_3: *mut Table = ::core::ptr::null_mut::<
                                            Table,
                                        >();
                                        let mut i_1: ::core::ffi::c_int = 0;
                                        pTab_3 = sqlite3FindTable(db, zRight, zDb);
                                        if !pTab_3.is_null() {
                                            let mut iTabDb: ::core::ffi::c_int = sqlite3SchemaToIndex(
                                                db,
                                                (*pTab_3).pSchema,
                                            );
                                            (*pParse).nMem = 5 as ::core::ffi::c_int;
                                            sqlite3CodeVerifySchema(pParse, iTabDb);
                                            pIdx_0 = (*pTab_3).pIndex;
                                            i_1 = 0 as ::core::ffi::c_int;
                                            while !pIdx_0.is_null() {
                                                let mut azOrigin: [*const ::core::ffi::c_char; 3] = [
                                                    b"c\0".as_ptr() as *const ::core::ffi::c_char,
                                                    b"u\0".as_ptr() as *const ::core::ffi::c_char,
                                                    b"pk\0".as_ptr() as *const ::core::ffi::c_char,
                                                ];
                                                sqlite3VdbeMultiLoad(
                                                    v,
                                                    1 as ::core::ffi::c_int,
                                                    b"isisi\0".as_ptr() as *const ::core::ffi::c_char,
                                                    i_1,
                                                    (*pIdx_0).zName,
                                                    ((*pIdx_0).onError as ::core::ffi::c_int != OE_None)
                                                        as ::core::ffi::c_int,
                                                    azOrigin[(*pIdx_0).idxType() as usize],
                                                    !(*pIdx_0).pPartIdxWhere.is_null() as ::core::ffi::c_int,
                                                );
                                                pIdx_0 = (*pIdx_0).pNext;
                                                i_1 += 1;
                                            }
                                        }
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_DATABASE_LIST => {
                                    let mut i_2: ::core::ffi::c_int = 0;
                                    (*pParse).nMem = 3 as ::core::ffi::c_int;
                                    i_2 = 0 as ::core::ffi::c_int;
                                    while i_2 < (*db).nDb {
                                        if !(*(*db).aDb.offset(i_2 as isize)).pBt.is_null() {
                                            sqlite3VdbeMultiLoad(
                                                v,
                                                1 as ::core::ffi::c_int,
                                                b"iss\0".as_ptr() as *const ::core::ffi::c_char,
                                                i_2,
                                                (*(*db).aDb.offset(i_2 as isize)).zDbSName,
                                                sqlite3BtreeGetFilename(
                                                    (*(*db).aDb.offset(i_2 as isize)).pBt,
                                                ),
                                            );
                                        }
                                        i_2 += 1;
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_COLLATION_LIST => {
                                    let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    let mut p: *mut HashElem = ::core::ptr::null_mut::<
                                        HashElem,
                                    >();
                                    (*pParse).nMem = 2 as ::core::ffi::c_int;
                                    p = (*db).aCollSeq.first;
                                    while !p.is_null() {
                                        let mut pColl: *mut CollSeq = (*p).data as *mut CollSeq;
                                        let c2rust_fresh1 = i_3;
                                        i_3 = i_3 + 1;
                                        sqlite3VdbeMultiLoad(
                                            v,
                                            1 as ::core::ffi::c_int,
                                            b"is\0".as_ptr() as *const ::core::ffi::c_char,
                                            c2rust_fresh1,
                                            (*pColl).zName,
                                        );
                                        p = (*p).next;
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_FUNCTION_LIST => {
                                    let mut i_4: ::core::ffi::c_int = 0;
                                    let mut j: *mut HashElem = ::core::ptr::null_mut::<
                                        HashElem,
                                    >();
                                    let mut p_0: *mut FuncDef = ::core::ptr::null_mut::<
                                        FuncDef,
                                    >();
                                    let mut showInternFunc: ::core::ffi::c_int = ((*db).mDbFlags
                                        as ::core::ffi::c_uint
                                        & DBFLAG_InternalFunc as ::core::ffi::c_uint
                                        != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                                    (*pParse).nMem = 6 as ::core::ffi::c_int;
                                    i_4 = 0 as ::core::ffi::c_int;
                                    while i_4 < SQLITE_FUNC_HASH_SZ {
                                        p_0 = sqlite3BuiltinFunctions.a[i_4 as usize];
                                        while !p_0.is_null() {
                                            pragmaFunclistLine(
                                                v,
                                                p_0,
                                                1 as ::core::ffi::c_int,
                                                showInternFunc,
                                            );
                                            p_0 = (*p_0).u.pHash;
                                        }
                                        i_4 += 1;
                                    }
                                    j = (*db).aFunc.first;
                                    while !j.is_null() {
                                        p_0 = (*j).data as *mut FuncDef;
                                        pragmaFunclistLine(
                                            v,
                                            p_0,
                                            0 as ::core::ffi::c_int,
                                            showInternFunc,
                                        );
                                        j = (*j).next;
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_MODULE_LIST => {
                                    let mut j_0: *mut HashElem = ::core::ptr::null_mut::<
                                        HashElem,
                                    >();
                                    (*pParse).nMem = 1 as ::core::ffi::c_int;
                                    j_0 = (*db).aModule.first;
                                    while !j_0.is_null() {
                                        let mut pMod: *mut Module = (*j_0).data as *mut Module;
                                        sqlite3VdbeMultiLoad(
                                            v,
                                            1 as ::core::ffi::c_int,
                                            b"s\0".as_ptr() as *const ::core::ffi::c_char,
                                            (*pMod).zName,
                                        );
                                        j_0 = (*j_0).next;
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_PRAGMA_LIST => {
                                    let mut i_5: ::core::ffi::c_int = 0;
                                    i_5 = 0 as ::core::ffi::c_int;
                                    while i_5
                                        < (::core::mem::size_of::<[PragmaName; 65]>() as usize)
                                            .wrapping_div(::core::mem::size_of::<PragmaName>() as usize)
                                            as ::core::ffi::c_int
                                    {
                                        sqlite3VdbeMultiLoad(
                                            v,
                                            1 as ::core::ffi::c_int,
                                            b"s\0".as_ptr() as *const ::core::ffi::c_char,
                                            aPragmaName[i_5 as usize].zName,
                                        );
                                        i_5 += 1;
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_FOREIGN_KEY_LIST => {
                                    if !zRight.is_null() {
                                        let mut pFK: *mut FKey = ::core::ptr::null_mut::<FKey>();
                                        let mut pTab_4: *mut Table = ::core::ptr::null_mut::<
                                            Table,
                                        >();
                                        pTab_4 = sqlite3FindTable(db, zRight, zDb);
                                        if !pTab_4.is_null()
                                            && (*pTab_4).eTabType as ::core::ffi::c_int == TABTYP_NORM
                                        {
                                            pFK = (*pTab_4).u.tab.pFKey;
                                            if !pFK.is_null() {
                                                let mut iTabDb_0: ::core::ffi::c_int = sqlite3SchemaToIndex(
                                                    db,
                                                    (*pTab_4).pSchema,
                                                );
                                                let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                (*pParse).nMem = 8 as ::core::ffi::c_int;
                                                sqlite3CodeVerifySchema(pParse, iTabDb_0);
                                                while !pFK.is_null() {
                                                    let mut j_1: ::core::ffi::c_int = 0;
                                                    j_1 = 0 as ::core::ffi::c_int;
                                                    while j_1 < (*pFK).nCol {
                                                        sqlite3VdbeMultiLoad(
                                                            v,
                                                            1 as ::core::ffi::c_int,
                                                            b"iissssss\0".as_ptr() as *const ::core::ffi::c_char,
                                                            i_6,
                                                            j_1,
                                                            (*pFK).zTo,
                                                            (*(*pTab_4)
                                                                .aCol
                                                                .offset(
                                                                    (*(&raw mut (*pFK).aCol as *mut sColMap)
                                                                        .offset(j_1 as isize))
                                                                        .iFrom as isize,
                                                                ))
                                                                .zCnName,
                                                            (*(&raw mut (*pFK).aCol as *mut sColMap)
                                                                .offset(j_1 as isize))
                                                                .zCol,
                                                            actionName(
                                                                (*pFK).aAction[1 as ::core::ffi::c_int as usize],
                                                            ),
                                                            actionName(
                                                                (*pFK).aAction[0 as ::core::ffi::c_int as usize],
                                                            ),
                                                            b"NONE\0".as_ptr() as *const ::core::ffi::c_char,
                                                        );
                                                        j_1 += 1;
                                                    }
                                                    i_6 += 1;
                                                    pFK = (*pFK).pNextFrom;
                                                }
                                            }
                                        }
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_FOREIGN_KEY_CHECK => {
                                    let mut pFK_0: *mut FKey = ::core::ptr::null_mut::<FKey>();
                                    let mut pTab_5: *mut Table = ::core::ptr::null_mut::<
                                        Table,
                                    >();
                                    let mut pParent: *mut Table = ::core::ptr::null_mut::<
                                        Table,
                                    >();
                                    let mut pIdx_1: *mut Index = ::core::ptr::null_mut::<
                                        Index,
                                    >();
                                    let mut i_7: ::core::ffi::c_int = 0;
                                    let mut j_2: ::core::ffi::c_int = 0;
                                    let mut k_1: *mut HashElem = ::core::ptr::null_mut::<
                                        HashElem,
                                    >();
                                    let mut x_0: ::core::ffi::c_int = 0;
                                    let mut regResult: ::core::ffi::c_int = 0;
                                    let mut regRow: ::core::ffi::c_int = 0;
                                    let mut addrTop: ::core::ffi::c_int = 0;
                                    let mut addrOk: ::core::ffi::c_int = 0;
                                    let mut aiCols: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
                                        ::core::ffi::c_int,
                                    >();
                                    regResult = (*pParse).nMem + 1 as ::core::ffi::c_int;
                                    (*pParse).nMem += 4 as ::core::ffi::c_int;
                                    (*pParse).nMem += 1;
                                    regRow = (*pParse).nMem;
                                    k_1 = (*(*(*db).aDb.offset(iDb as isize)).pSchema)
                                        .tblHash
                                        .first;
                                    while !k_1.is_null() {
                                        if !zRight.is_null() {
                                            pTab_5 = sqlite3LocateTable(
                                                pParse,
                                                0 as u32_0,
                                                zRight,
                                                zDb,
                                            );
                                            k_1 = ::core::ptr::null_mut::<HashElem>();
                                        } else {
                                            pTab_5 = (*k_1).data as *mut Table;
                                            k_1 = (*k_1).next;
                                        }
                                        if pTab_5.is_null()
                                            || !((*pTab_5).eTabType as ::core::ffi::c_int
                                                == TABTYP_NORM) || (*pTab_5).u.tab.pFKey.is_null()
                                        {
                                            continue;
                                        }
                                        iDb = sqlite3SchemaToIndex(db, (*pTab_5).pSchema);
                                        zDb = (*(*db).aDb.offset(iDb as isize)).zDbSName;
                                        sqlite3CodeVerifySchema(pParse, iDb);
                                        sqlite3TouchRegister(
                                            pParse,
                                            (*pTab_5).nCol as ::core::ffi::c_int + regRow,
                                        );
                                        sqlite3OpenTable(
                                            pParse,
                                            0 as ::core::ffi::c_int,
                                            iDb,
                                            pTab_5,
                                            OP_OpenRead,
                                        );
                                        sqlite3VdbeLoadString(v, regResult, (*pTab_5).zName);
                                        i_7 = 1 as ::core::ffi::c_int;
                                        pFK_0 = (*pTab_5).u.tab.pFKey;
                                        while !pFK_0.is_null() {
                                            pParent = sqlite3FindTable(db, (*pFK_0).zTo, zDb);
                                            if !pParent.is_null() {
                                                pIdx_1 = ::core::ptr::null_mut::<Index>();
                                                x_0 = sqlite3FkLocateIndex(
                                                    pParse,
                                                    pParent,
                                                    pFK_0,
                                                    &raw mut pIdx_1,
                                                    ::core::ptr::null_mut::<*mut ::core::ffi::c_int>(),
                                                );
                                                if x_0 == 0 as ::core::ffi::c_int {
                                                    if pIdx_1.is_null() {
                                                        sqlite3OpenTable(pParse, i_7, iDb, pParent, OP_OpenRead);
                                                    } else {
                                                        sqlite3VdbeAddOp3(
                                                            v,
                                                            OP_OpenRead,
                                                            i_7,
                                                            (*pIdx_1).tnum as ::core::ffi::c_int,
                                                            iDb,
                                                        );
                                                        sqlite3VdbeSetP4KeyInfo(pParse, pIdx_1);
                                                    }
                                                } else {
                                                    k_1 = ::core::ptr::null_mut::<HashElem>();
                                                    break;
                                                }
                                            }
                                            i_7 += 1;
                                            pFK_0 = (*pFK_0).pNextFrom;
                                        }
                                        if !pFK_0.is_null() {
                                            break;
                                        }
                                        if (*pParse).nTab < i_7 {
                                            (*pParse).nTab = i_7;
                                        }
                                        addrTop = sqlite3VdbeAddOp1(
                                            v,
                                            OP_Rewind,
                                            0 as ::core::ffi::c_int,
                                        );
                                        i_7 = 1 as ::core::ffi::c_int;
                                        pFK_0 = (*pTab_5).u.tab.pFKey;
                                        while !pFK_0.is_null() {
                                            pParent = sqlite3FindTable(db, (*pFK_0).zTo, zDb);
                                            pIdx_1 = ::core::ptr::null_mut::<Index>();
                                            aiCols = ::core::ptr::null_mut::<::core::ffi::c_int>();
                                            if !pParent.is_null() {
                                                x_0 = sqlite3FkLocateIndex(
                                                    pParse,
                                                    pParent,
                                                    pFK_0,
                                                    &raw mut pIdx_1,
                                                    &raw mut aiCols,
                                                );
                                            }
                                            addrOk = sqlite3VdbeMakeLabel(pParse);
                                            sqlite3TouchRegister(pParse, regRow + (*pFK_0).nCol);
                                            j_2 = 0 as ::core::ffi::c_int;
                                            while j_2 < (*pFK_0).nCol {
                                                let mut iCol: ::core::ffi::c_int = if !aiCols.is_null() {
                                                    *aiCols.offset(j_2 as isize)
                                                } else {
                                                    (*(&raw mut (*pFK_0).aCol as *mut sColMap)
                                                        .offset(j_2 as isize))
                                                        .iFrom
                                                };
                                                sqlite3ExprCodeGetColumnOfTable(
                                                    v,
                                                    pTab_5,
                                                    0 as ::core::ffi::c_int,
                                                    iCol,
                                                    regRow + j_2,
                                                );
                                                sqlite3VdbeAddOp2(v, OP_IsNull, regRow + j_2, addrOk);
                                                j_2 += 1;
                                            }
                                            if !pIdx_1.is_null() {
                                                sqlite3VdbeAddOp4(
                                                    v,
                                                    OP_Affinity,
                                                    regRow,
                                                    (*pFK_0).nCol,
                                                    0 as ::core::ffi::c_int,
                                                    sqlite3IndexAffinityStr(db, pIdx_1),
                                                    (*pFK_0).nCol,
                                                );
                                                sqlite3VdbeAddOp4Int(
                                                    v,
                                                    OP_Found,
                                                    i_7,
                                                    addrOk,
                                                    regRow,
                                                    (*pFK_0).nCol,
                                                );
                                            } else if !pParent.is_null() {
                                                let mut jmp: ::core::ffi::c_int = sqlite3VdbeCurrentAddr(v)
                                                    + 2 as ::core::ffi::c_int;
                                                sqlite3VdbeAddOp3(v, OP_SeekRowid, i_7, jmp, regRow);
                                                sqlite3VdbeGoto(v, addrOk);
                                            }
                                            if (*pTab_5).tabFlags as ::core::ffi::c_uint
                                                & TF_WithoutRowid as ::core::ffi::c_uint
                                                == 0 as ::core::ffi::c_uint
                                            {
                                                sqlite3VdbeAddOp2(
                                                    v,
                                                    OP_Rowid,
                                                    0 as ::core::ffi::c_int,
                                                    regResult + 1 as ::core::ffi::c_int,
                                                );
                                            } else {
                                                sqlite3VdbeAddOp2(
                                                    v,
                                                    OP_Null,
                                                    0 as ::core::ffi::c_int,
                                                    regResult + 1 as ::core::ffi::c_int,
                                                );
                                            }
                                            sqlite3VdbeMultiLoad(
                                                v,
                                                regResult + 2 as ::core::ffi::c_int,
                                                b"siX\0".as_ptr() as *const ::core::ffi::c_char,
                                                (*pFK_0).zTo,
                                                i_7 - 1 as ::core::ffi::c_int,
                                            );
                                            sqlite3VdbeAddOp2(
                                                v,
                                                OP_ResultRow,
                                                regResult,
                                                4 as ::core::ffi::c_int,
                                            );
                                            sqlite3VdbeResolveLabel(v, addrOk);
                                            sqlite3DbFree(db, aiCols as *mut ::core::ffi::c_void);
                                            i_7 += 1;
                                            pFK_0 = (*pFK_0).pNextFrom;
                                        }
                                        sqlite3VdbeAddOp2(
                                            v,
                                            OP_Next,
                                            0 as ::core::ffi::c_int,
                                            addrTop + 1 as ::core::ffi::c_int,
                                        );
                                        sqlite3VdbeJumpHere(v, addrTop);
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_CASE_SENSITIVE_LIKE => {
                                    if !zRight.is_null() {
                                        sqlite3RegisterLikeFunctions(
                                            db,
                                            sqlite3GetBoolean(zRight, 0 as u8_0) as ::core::ffi::c_int,
                                        );
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_INTEGRITY_CHECK => {
                                    let mut i_8: ::core::ffi::c_int = 0;
                                    let mut j_3: ::core::ffi::c_int = 0;
                                    let mut addr_0: ::core::ffi::c_int = 0;
                                    let mut mxErr: ::core::ffi::c_int = 0;
                                    let mut pObjTab: *mut Table = ::core::ptr::null_mut::<
                                        Table,
                                    >();
                                    let mut isQuick: ::core::ffi::c_int = (*(&raw const sqlite3UpperToLower
                                        as *const ::core::ffi::c_uchar)
                                        .offset(
                                            *zLeft.offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_uchar as isize,
                                        ) as ::core::ffi::c_int == 'q' as i32)
                                        as ::core::ffi::c_int;
                                    if (*pId2).z.is_null() {
                                        iDb = -1 as ::core::ffi::c_int;
                                    }
                                    (*pParse).nMem = 6 as ::core::ffi::c_int;
                                    mxErr = SQLITE_INTEGRITY_CHECK_ERROR_MAX;
                                    if !zRight.is_null() {
                                        if sqlite3GetInt32((*pValue).z, &raw mut mxErr) != 0 {
                                            if mxErr <= 0 as ::core::ffi::c_int {
                                                mxErr = SQLITE_INTEGRITY_CHECK_ERROR_MAX;
                                            }
                                        } else {
                                            pObjTab = sqlite3LocateTable(
                                                pParse,
                                                0 as u32_0,
                                                zRight,
                                                if iDb >= 0 as ::core::ffi::c_int {
                                                    (*(*db).aDb.offset(iDb as isize)).zDbSName
                                                } else {
                                                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                                                },
                                            );
                                        }
                                    }
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_Integer,
                                        mxErr - 1 as ::core::ffi::c_int,
                                        1 as ::core::ffi::c_int,
                                    );
                                    i_8 = 0 as ::core::ffi::c_int;
                                    while i_8 < (*db).nDb {
                                        let mut x_1: *mut HashElem = ::core::ptr::null_mut::<
                                            HashElem,
                                        >();
                                        let mut pTbls: *mut Hash = ::core::ptr::null_mut::<Hash>();
                                        let mut aRoot: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
                                            ::core::ffi::c_int,
                                        >();
                                        let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                        if !(OMIT_TEMPDB != 0 && i_8 == 1 as ::core::ffi::c_int) {
                                            if !(iDb >= 0 as ::core::ffi::c_int && i_8 != iDb) {
                                                sqlite3CodeVerifySchema(pParse, i_8);
                                                (*pParse).set_okConstFactor(0 as bft as bft);
                                                pTbls = &raw mut (*(*(*db).aDb.offset(i_8 as isize))
                                                    .pSchema)
                                                    .tblHash;
                                                cnt = 0 as ::core::ffi::c_int;
                                                x_1 = (*pTbls).first;
                                                while !x_1.is_null() {
                                                    let mut pTab_6: *mut Table = (*x_1).data as *mut Table;
                                                    let mut pIdx_2: *mut Index = ::core::ptr::null_mut::<
                                                        Index,
                                                    >();
                                                    let mut nIdx: ::core::ffi::c_int = 0;
                                                    if !(tableSkipIntegrityCheck(pTab_6, pObjTab) != 0) {
                                                        if (*pTab_6).tabFlags as ::core::ffi::c_uint
                                                            & TF_WithoutRowid as ::core::ffi::c_uint
                                                            == 0 as ::core::ffi::c_uint
                                                        {
                                                            cnt += 1;
                                                        }
                                                        nIdx = 0 as ::core::ffi::c_int;
                                                        pIdx_2 = (*pTab_6).pIndex;
                                                        while !pIdx_2.is_null() {
                                                            cnt += 1;
                                                            pIdx_2 = (*pIdx_2).pNext;
                                                            nIdx += 1;
                                                        }
                                                    }
                                                    x_1 = (*x_1).next;
                                                }
                                                if !(cnt == 0 as ::core::ffi::c_int) {
                                                    if !pObjTab.is_null() {
                                                        cnt += 1;
                                                    }
                                                    aRoot = sqlite3DbMallocRawNN(
                                                        db,
                                                        (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                                            .wrapping_mul((cnt + 1 as ::core::ffi::c_int) as usize)
                                                            as u64_0,
                                                    ) as *mut ::core::ffi::c_int;
                                                    if aRoot.is_null() {
                                                        break;
                                                    }
                                                    cnt = 0 as ::core::ffi::c_int;
                                                    if !pObjTab.is_null() {
                                                        cnt += 1;
                                                        *aRoot.offset(cnt as isize) = 0 as ::core::ffi::c_int;
                                                    }
                                                    x_1 = (*pTbls).first;
                                                    while !x_1.is_null() {
                                                        let mut pTab_7: *mut Table = (*x_1).data as *mut Table;
                                                        let mut pIdx_3: *mut Index = ::core::ptr::null_mut::<
                                                            Index,
                                                        >();
                                                        if !(tableSkipIntegrityCheck(pTab_7, pObjTab) != 0) {
                                                            if (*pTab_7).tabFlags as ::core::ffi::c_uint
                                                                & TF_WithoutRowid as ::core::ffi::c_uint
                                                                == 0 as ::core::ffi::c_uint
                                                            {
                                                                cnt += 1;
                                                                *aRoot.offset(cnt as isize) = (*pTab_7).tnum
                                                                    as ::core::ffi::c_int;
                                                            }
                                                            pIdx_3 = (*pTab_7).pIndex;
                                                            while !pIdx_3.is_null() {
                                                                cnt += 1;
                                                                *aRoot.offset(cnt as isize) = (*pIdx_3).tnum
                                                                    as ::core::ffi::c_int;
                                                                pIdx_3 = (*pIdx_3).pNext;
                                                            }
                                                        }
                                                        x_1 = (*x_1).next;
                                                    }
                                                    *aRoot.offset(0 as ::core::ffi::c_int as isize) = cnt;
                                                    sqlite3TouchRegister(pParse, 8 as ::core::ffi::c_int + cnt);
                                                    sqlite3VdbeAddOp3(
                                                        v,
                                                        OP_Null,
                                                        0 as ::core::ffi::c_int,
                                                        8 as ::core::ffi::c_int,
                                                        8 as ::core::ffi::c_int + cnt,
                                                    );
                                                    sqlite3ClearTempRegCache(pParse);
                                                    sqlite3VdbeAddOp4(
                                                        v,
                                                        OP_IntegrityCk,
                                                        1 as ::core::ffi::c_int,
                                                        cnt,
                                                        8 as ::core::ffi::c_int,
                                                        aRoot as *mut ::core::ffi::c_char,
                                                        P4_INTARRAY,
                                                    );
                                                    sqlite3VdbeChangeP5(v, i_8 as u16_0);
                                                    addr_0 = sqlite3VdbeAddOp1(
                                                        v,
                                                        OP_IsNull,
                                                        2 as ::core::ffi::c_int,
                                                    );
                                                    sqlite3VdbeAddOp4(
                                                        v,
                                                        OP_String8,
                                                        0 as ::core::ffi::c_int,
                                                        3 as ::core::ffi::c_int,
                                                        0 as ::core::ffi::c_int,
                                                        sqlite3MPrintf(
                                                            db,
                                                            b"*** in database %s ***\n\0".as_ptr()
                                                                as *const ::core::ffi::c_char,
                                                            (*(*db).aDb.offset(i_8 as isize)).zDbSName,
                                                        ),
                                                        P4_DYNAMIC,
                                                    );
                                                    sqlite3VdbeAddOp3(
                                                        v,
                                                        OP_Concat,
                                                        2 as ::core::ffi::c_int,
                                                        3 as ::core::ffi::c_int,
                                                        3 as ::core::ffi::c_int,
                                                    );
                                                    integrityCheckResultRow(v);
                                                    sqlite3VdbeJumpHere(v, addr_0);
                                                    cnt = if !pObjTab.is_null() {
                                                        1 as ::core::ffi::c_int
                                                    } else {
                                                        0 as ::core::ffi::c_int
                                                    };
                                                    sqlite3VdbeLoadString(
                                                        v,
                                                        2 as ::core::ffi::c_int,
                                                        b"wrong # of entries in index \0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    x_1 = (*pTbls).first;
                                                    while !x_1.is_null() {
                                                        let mut iTab: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                        let mut pTab_8: *mut Table = (*x_1).data as *mut Table;
                                                        let mut pIdx_4: *mut Index = ::core::ptr::null_mut::<
                                                            Index,
                                                        >();
                                                        if !(tableSkipIntegrityCheck(pTab_8, pObjTab) != 0) {
                                                            if (*pTab_8).tabFlags as ::core::ffi::c_uint
                                                                & TF_WithoutRowid as ::core::ffi::c_uint
                                                                == 0 as ::core::ffi::c_uint
                                                            {
                                                                let c2rust_fresh2 = cnt;
                                                                cnt = cnt + 1;
                                                                iTab = c2rust_fresh2;
                                                            } else {
                                                                iTab = cnt;
                                                                pIdx_4 = (*pTab_8).pIndex;
                                                                while !pIdx_4.is_null() {
                                                                    if (*pIdx_4).idxType() as ::core::ffi::c_int
                                                                        == SQLITE_IDXTYPE_PRIMARYKEY
                                                                    {
                                                                        break;
                                                                    }
                                                                    iTab += 1;
                                                                    pIdx_4 = (*pIdx_4).pNext;
                                                                }
                                                            }
                                                            pIdx_4 = (*pTab_8).pIndex;
                                                            while !pIdx_4.is_null() {
                                                                if (*pIdx_4).pPartIdxWhere.is_null() {
                                                                    addr_0 = sqlite3VdbeAddOp3(
                                                                        v,
                                                                        OP_Eq,
                                                                        8 as ::core::ffi::c_int + cnt,
                                                                        0 as ::core::ffi::c_int,
                                                                        8 as ::core::ffi::c_int + iTab,
                                                                    );
                                                                    sqlite3VdbeLoadString(
                                                                        v,
                                                                        4 as ::core::ffi::c_int,
                                                                        (*pIdx_4).zName,
                                                                    );
                                                                    sqlite3VdbeAddOp3(
                                                                        v,
                                                                        OP_Concat,
                                                                        4 as ::core::ffi::c_int,
                                                                        2 as ::core::ffi::c_int,
                                                                        3 as ::core::ffi::c_int,
                                                                    );
                                                                    integrityCheckResultRow(v);
                                                                    sqlite3VdbeJumpHere(v, addr_0);
                                                                }
                                                                cnt += 1;
                                                                pIdx_4 = (*pIdx_4).pNext;
                                                            }
                                                        }
                                                        x_1 = (*x_1).next;
                                                    }
                                                    x_1 = (*pTbls).first;
                                                    while !x_1.is_null() {
                                                        let mut pTab_9: *mut Table = (*x_1).data as *mut Table;
                                                        let mut pIdx_5: *mut Index = ::core::ptr::null_mut::<
                                                            Index,
                                                        >();
                                                        let mut pPk_0: *mut Index = ::core::ptr::null_mut::<
                                                            Index,
                                                        >();
                                                        let mut pPrior: *mut Index = ::core::ptr::null_mut::<
                                                            Index,
                                                        >();
                                                        let mut loopTop: ::core::ffi::c_int = 0;
                                                        let mut iDataCur: ::core::ffi::c_int = 0;
                                                        let mut iIdxCur: ::core::ffi::c_int = 0;
                                                        let mut r1: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                                                        let mut bStrict: ::core::ffi::c_int = 0;
                                                        let mut r2: ::core::ffi::c_int = 0;
                                                        let mut mxCol: ::core::ffi::c_int = 0;
                                                        if !(tableSkipIntegrityCheck(pTab_9, pObjTab) != 0) {
                                                            if (*pTab_9).eTabType as ::core::ffi::c_int == TABTYP_NORM {
                                                                if isQuick != 0
                                                                    || (*pTab_9).tabFlags as ::core::ffi::c_uint
                                                                        & TF_WithoutRowid as ::core::ffi::c_uint
                                                                        == 0 as ::core::ffi::c_uint
                                                                {
                                                                    pPk_0 = ::core::ptr::null_mut::<Index>();
                                                                    r2 = 0 as ::core::ffi::c_int;
                                                                } else {
                                                                    pPk_0 = sqlite3PrimaryKeyIndex(pTab_9);
                                                                    r2 = sqlite3GetTempRange(
                                                                        pParse,
                                                                        (*pPk_0).nKeyCol as ::core::ffi::c_int,
                                                                    );
                                                                    sqlite3VdbeAddOp3(
                                                                        v,
                                                                        OP_Null,
                                                                        1 as ::core::ffi::c_int,
                                                                        r2,
                                                                        r2 + (*pPk_0).nKeyCol as ::core::ffi::c_int
                                                                            - 1 as ::core::ffi::c_int,
                                                                    );
                                                                }
                                                                sqlite3OpenTableAndIndices(
                                                                    pParse,
                                                                    pTab_9,
                                                                    OP_OpenRead,
                                                                    0 as u8_0,
                                                                    1 as ::core::ffi::c_int,
                                                                    ::core::ptr::null_mut::<u8_0>(),
                                                                    &raw mut iDataCur,
                                                                    &raw mut iIdxCur,
                                                                );
                                                                sqlite3VdbeAddOp2(
                                                                    v,
                                                                    OP_Integer,
                                                                    0 as ::core::ffi::c_int,
                                                                    7 as ::core::ffi::c_int,
                                                                );
                                                                j_3 = 0 as ::core::ffi::c_int;
                                                                pIdx_5 = (*pTab_9).pIndex;
                                                                while !pIdx_5.is_null() {
                                                                    sqlite3VdbeAddOp2(
                                                                        v,
                                                                        OP_Integer,
                                                                        0 as ::core::ffi::c_int,
                                                                        8 as ::core::ffi::c_int + j_3,
                                                                    );
                                                                    pIdx_5 = (*pIdx_5).pNext;
                                                                    j_3 += 1;
                                                                }
                                                                sqlite3VdbeAddOp2(
                                                                    v,
                                                                    OP_Rewind,
                                                                    iDataCur,
                                                                    0 as ::core::ffi::c_int,
                                                                );
                                                                loopTop = sqlite3VdbeAddOp2(
                                                                    v,
                                                                    OP_AddImm,
                                                                    7 as ::core::ffi::c_int,
                                                                    1 as ::core::ffi::c_int,
                                                                );
                                                                if (*pTab_9).tabFlags as ::core::ffi::c_uint
                                                                    & TF_WithoutRowid as ::core::ffi::c_uint
                                                                    == 0 as ::core::ffi::c_uint
                                                                {
                                                                    mxCol = -1 as ::core::ffi::c_int;
                                                                    j_3 = 0 as ::core::ffi::c_int;
                                                                    while j_3 < (*pTab_9).nCol as ::core::ffi::c_int {
                                                                        if (*(*pTab_9).aCol.offset(j_3 as isize)).colFlags
                                                                            as ::core::ffi::c_int & COLFLAG_VIRTUAL
                                                                            == 0 as ::core::ffi::c_int
                                                                        {
                                                                            mxCol += 1;
                                                                        }
                                                                        j_3 += 1;
                                                                    }
                                                                    if mxCol == (*pTab_9).iPKey as ::core::ffi::c_int {
                                                                        mxCol -= 1;
                                                                    }
                                                                } else {
                                                                    mxCol = (*sqlite3PrimaryKeyIndex(pTab_9)).nColumn
                                                                        as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
                                                                }
                                                                if mxCol >= 0 as ::core::ffi::c_int {
                                                                    sqlite3VdbeAddOp3(
                                                                        v,
                                                                        OP_Column,
                                                                        iDataCur,
                                                                        mxCol,
                                                                        3 as ::core::ffi::c_int,
                                                                    );
                                                                    sqlite3VdbeTypeofColumn(v, 3 as ::core::ffi::c_int);
                                                                }
                                                                if isQuick == 0 {
                                                                    if !pPk_0.is_null() {
                                                                        let mut a1: ::core::ffi::c_int = 0;
                                                                        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                                                            ::core::ffi::c_char,
                                                                        >();
                                                                        a1 = sqlite3VdbeAddOp4Int(
                                                                            v,
                                                                            OP_IdxGT,
                                                                            iDataCur,
                                                                            0 as ::core::ffi::c_int,
                                                                            r2,
                                                                            (*pPk_0).nKeyCol as ::core::ffi::c_int,
                                                                        );
                                                                        sqlite3VdbeAddOp1(v, OP_IsNull, r2);
                                                                        zErr = sqlite3MPrintf(
                                                                            db,
                                                                            b"row not in PRIMARY KEY order for %s\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            (*pTab_9).zName,
                                                                        );
                                                                        sqlite3VdbeAddOp4(
                                                                            v,
                                                                            OP_String8,
                                                                            0 as ::core::ffi::c_int,
                                                                            3 as ::core::ffi::c_int,
                                                                            0 as ::core::ffi::c_int,
                                                                            zErr,
                                                                            P4_DYNAMIC,
                                                                        );
                                                                        integrityCheckResultRow(v);
                                                                        sqlite3VdbeJumpHere(v, a1);
                                                                        sqlite3VdbeJumpHere(v, a1 + 1 as ::core::ffi::c_int);
                                                                        j_3 = 0 as ::core::ffi::c_int;
                                                                        while j_3 < (*pPk_0).nKeyCol as ::core::ffi::c_int {
                                                                            sqlite3ExprCodeLoadIndexColumn(
                                                                                pParse,
                                                                                pPk_0,
                                                                                iDataCur,
                                                                                j_3,
                                                                                r2 + j_3,
                                                                            );
                                                                            j_3 += 1;
                                                                        }
                                                                    }
                                                                }
                                                                bStrict = ((*pTab_9).tabFlags as ::core::ffi::c_uint
                                                                    & TF_Strict as ::core::ffi::c_uint
                                                                    != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                                                                j_3 = 0 as ::core::ffi::c_int;
                                                                while j_3 < (*pTab_9).nCol as ::core::ffi::c_int {
                                                                    let mut zErr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                                                        ::core::ffi::c_char,
                                                                    >();
                                                                    let mut pCol_0: *mut Column = (*pTab_9)
                                                                        .aCol
                                                                        .offset(j_3 as isize);
                                                                    let mut labelError: ::core::ffi::c_int = 0;
                                                                    let mut labelOk: ::core::ffi::c_int = 0;
                                                                    let mut p1: ::core::ffi::c_int = 0;
                                                                    let mut p3: ::core::ffi::c_int = 0;
                                                                    let mut p4: ::core::ffi::c_int = 0;
                                                                    let mut doTypeCheck: ::core::ffi::c_int = 0;
                                                                    if !(j_3 == (*pTab_9).iPKey as ::core::ffi::c_int) {
                                                                        if bStrict != 0 {
                                                                            doTypeCheck = ((*pCol_0).eCType() as ::core::ffi::c_int
                                                                                > COLTYPE_ANY) as ::core::ffi::c_int;
                                                                        } else {
                                                                            doTypeCheck = ((*pCol_0).affinity as ::core::ffi::c_int
                                                                                > SQLITE_AFF_BLOB) as ::core::ffi::c_int;
                                                                        }
                                                                        if !((*pCol_0).notNull() as ::core::ffi::c_int
                                                                            == 0 as ::core::ffi::c_int && doTypeCheck == 0)
                                                                        {
                                                                            p4 = SQLITE_NULL;
                                                                            if (*pCol_0).colFlags as ::core::ffi::c_int
                                                                                & COLFLAG_VIRTUAL != 0
                                                                            {
                                                                                sqlite3ExprCodeGetColumnOfTable(
                                                                                    v,
                                                                                    pTab_9,
                                                                                    iDataCur,
                                                                                    j_3,
                                                                                    3 as ::core::ffi::c_int,
                                                                                );
                                                                                p1 = -1 as ::core::ffi::c_int;
                                                                                p3 = 3 as ::core::ffi::c_int;
                                                                            } else {
                                                                                if (*pCol_0).iDflt != 0 {
                                                                                    let mut pDfltValue: *mut sqlite3_value = ::core::ptr::null_mut::<
                                                                                        sqlite3_value,
                                                                                    >();
                                                                                    sqlite3ValueFromExpr(
                                                                                        db,
                                                                                        sqlite3ColumnExpr(pTab_9, pCol_0),
                                                                                        (*db).enc,
                                                                                        (*pCol_0).affinity as u8_0,
                                                                                        &raw mut pDfltValue,
                                                                                    );
                                                                                    if !pDfltValue.is_null() {
                                                                                        p4 = sqlite3_value_type(pDfltValue);
                                                                                        sqlite3ValueFree(pDfltValue);
                                                                                    }
                                                                                }
                                                                                p1 = iDataCur;
                                                                                if !((*pTab_9).tabFlags as ::core::ffi::c_uint
                                                                                    & TF_WithoutRowid as ::core::ffi::c_uint
                                                                                    == 0 as ::core::ffi::c_uint)
                                                                                {
                                                                                    p3 = sqlite3TableColumnToIndex(
                                                                                        sqlite3PrimaryKeyIndex(pTab_9),
                                                                                        j_3,
                                                                                    );
                                                                                } else {
                                                                                    p3 = sqlite3TableColumnToStorage(pTab_9, j_3 as i16_0)
                                                                                        as ::core::ffi::c_int;
                                                                                }
                                                                            }
                                                                            labelError = sqlite3VdbeMakeLabel(pParse);
                                                                            labelOk = sqlite3VdbeMakeLabel(pParse);
                                                                            if (*pCol_0).notNull() != 0 {
                                                                                let mut jmp3: ::core::ffi::c_int = 0;
                                                                                let mut jmp2: ::core::ffi::c_int = sqlite3VdbeAddOp4Int(
                                                                                    v,
                                                                                    OP_IsType,
                                                                                    p1,
                                                                                    labelOk,
                                                                                    p3,
                                                                                    p4,
                                                                                );
                                                                                if p1 < 0 as ::core::ffi::c_int {
                                                                                    sqlite3VdbeChangeP5(v, 0xf as u16_0);
                                                                                    jmp3 = jmp2;
                                                                                } else {
                                                                                    sqlite3VdbeChangeP5(v, 0xd as u16_0);
                                                                                    sqlite3VdbeAddOp3(
                                                                                        v,
                                                                                        OP_Column,
                                                                                        p1,
                                                                                        p3,
                                                                                        3 as ::core::ffi::c_int,
                                                                                    );
                                                                                    sqlite3ColumnDefault(
                                                                                        v,
                                                                                        pTab_9,
                                                                                        j_3,
                                                                                        3 as ::core::ffi::c_int,
                                                                                    );
                                                                                    jmp3 = sqlite3VdbeAddOp2(
                                                                                        v,
                                                                                        OP_NotNull,
                                                                                        3 as ::core::ffi::c_int,
                                                                                        labelOk,
                                                                                    );
                                                                                }
                                                                                zErr_0 = sqlite3MPrintf(
                                                                                    db,
                                                                                    b"NULL value in %s.%s\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                    (*pTab_9).zName,
                                                                                    (*pCol_0).zCnName,
                                                                                );
                                                                                sqlite3VdbeAddOp4(
                                                                                    v,
                                                                                    OP_String8,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    zErr_0,
                                                                                    P4_DYNAMIC,
                                                                                );
                                                                                if doTypeCheck != 0 {
                                                                                    sqlite3VdbeGoto(v, labelError);
                                                                                    sqlite3VdbeJumpHere(v, jmp2);
                                                                                    sqlite3VdbeJumpHere(v, jmp3);
                                                                                }
                                                                            }
                                                                            if bStrict != 0 && doTypeCheck != 0 {
                                                                                static mut aStdTypeMask: [::core::ffi::c_uchar; 6] = [
                                                                                    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                                                    0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                                                    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                                                    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                                                    0x13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                                                    0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                                                ];
                                                                                sqlite3VdbeAddOp4Int(v, OP_IsType, p1, labelOk, p3, p4);
                                                                                sqlite3VdbeChangeP5(
                                                                                    v,
                                                                                    aStdTypeMask[((*pCol_0).eCType() as ::core::ffi::c_int
                                                                                        - 1 as ::core::ffi::c_int) as usize] as u16_0,
                                                                                );
                                                                                zErr_0 = sqlite3MPrintf(
                                                                                    db,
                                                                                    b"non-%s value in %s.%s\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                    *(&raw mut sqlite3StdType
                                                                                        as *mut *const ::core::ffi::c_char)
                                                                                        .offset(
                                                                                            ((*pCol_0).eCType() as ::core::ffi::c_int
                                                                                                - 1 as ::core::ffi::c_int) as isize,
                                                                                        ),
                                                                                    (*pTab_9).zName,
                                                                                    (*(*pTab_9).aCol.offset(j_3 as isize)).zCnName,
                                                                                );
                                                                                sqlite3VdbeAddOp4(
                                                                                    v,
                                                                                    OP_String8,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    zErr_0,
                                                                                    P4_DYNAMIC,
                                                                                );
                                                                            } else if bStrict == 0
                                                                                && (*pCol_0).affinity as ::core::ffi::c_int
                                                                                    == SQLITE_AFF_TEXT
                                                                            {
                                                                                sqlite3VdbeAddOp4Int(v, OP_IsType, p1, labelOk, p3, p4);
                                                                                sqlite3VdbeChangeP5(v, 0x1c as u16_0);
                                                                                zErr_0 = sqlite3MPrintf(
                                                                                    db,
                                                                                    b"NUMERIC value in %s.%s\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                    (*pTab_9).zName,
                                                                                    (*(*pTab_9).aCol.offset(j_3 as isize)).zCnName,
                                                                                );
                                                                                sqlite3VdbeAddOp4(
                                                                                    v,
                                                                                    OP_String8,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    zErr_0,
                                                                                    P4_DYNAMIC,
                                                                                );
                                                                            } else if bStrict == 0
                                                                                && (*pCol_0).affinity as ::core::ffi::c_int
                                                                                    >= SQLITE_AFF_NUMERIC
                                                                            {
                                                                                sqlite3VdbeAddOp4Int(v, OP_IsType, p1, labelOk, p3, p4);
                                                                                sqlite3VdbeChangeP5(v, 0x1b as u16_0);
                                                                                if p1 >= 0 as ::core::ffi::c_int {
                                                                                    sqlite3ExprCodeGetColumnOfTable(
                                                                                        v,
                                                                                        pTab_9,
                                                                                        iDataCur,
                                                                                        j_3,
                                                                                        3 as ::core::ffi::c_int,
                                                                                    );
                                                                                }
                                                                                sqlite3VdbeAddOp4(
                                                                                    v,
                                                                                    OP_Affinity,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    1 as ::core::ffi::c_int,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    b"C\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                    P4_STATIC,
                                                                                );
                                                                                sqlite3VdbeAddOp4Int(
                                                                                    v,
                                                                                    OP_IsType,
                                                                                    -1 as ::core::ffi::c_int,
                                                                                    labelOk,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    p4,
                                                                                );
                                                                                sqlite3VdbeChangeP5(v, 0x1c as u16_0);
                                                                                zErr_0 = sqlite3MPrintf(
                                                                                    db,
                                                                                    b"TEXT value in %s.%s\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                    (*pTab_9).zName,
                                                                                    (*(*pTab_9).aCol.offset(j_3 as isize)).zCnName,
                                                                                );
                                                                                sqlite3VdbeAddOp4(
                                                                                    v,
                                                                                    OP_String8,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    zErr_0,
                                                                                    P4_DYNAMIC,
                                                                                );
                                                                            }
                                                                            sqlite3VdbeResolveLabel(v, labelError);
                                                                            integrityCheckResultRow(v);
                                                                            sqlite3VdbeResolveLabel(v, labelOk);
                                                                        }
                                                                    }
                                                                    j_3 += 1;
                                                                }
                                                                if !(*pTab_9).pCheck.is_null()
                                                                    && (*db).flags as ::core::ffi::c_ulonglong
                                                                        & SQLITE_IgnoreChecks as ::core::ffi::c_ulonglong
                                                                        == 0 as ::core::ffi::c_ulonglong
                                                                {
                                                                    let mut pCheck: *mut ExprList = sqlite3ExprListDup(
                                                                        db,
                                                                        (*pTab_9).pCheck,
                                                                        0 as ::core::ffi::c_int,
                                                                    );
                                                                    if (*db).mallocFailed as ::core::ffi::c_int
                                                                        == 0 as ::core::ffi::c_int
                                                                    {
                                                                        let mut addrCkFault: ::core::ffi::c_int = sqlite3VdbeMakeLabel(
                                                                            pParse,
                                                                        );
                                                                        let mut addrCkOk: ::core::ffi::c_int = sqlite3VdbeMakeLabel(
                                                                            pParse,
                                                                        );
                                                                        let mut zErr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                                                            ::core::ffi::c_char,
                                                                        >();
                                                                        let mut k_2: ::core::ffi::c_int = 0;
                                                                        (*pParse).iSelfTab = iDataCur + 1 as ::core::ffi::c_int;
                                                                        k_2 = (*pCheck).nExpr - 1 as ::core::ffi::c_int;
                                                                        while k_2 > 0 as ::core::ffi::c_int {
                                                                            sqlite3ExprIfFalse(
                                                                                pParse,
                                                                                (*(&raw mut (*pCheck).a as *mut ExprList_item)
                                                                                    .offset(k_2 as isize))
                                                                                    .pExpr,
                                                                                addrCkFault,
                                                                                0 as ::core::ffi::c_int,
                                                                            );
                                                                            k_2 -= 1;
                                                                        }
                                                                        sqlite3ExprIfTrue(
                                                                            pParse,
                                                                            (*(&raw mut (*pCheck).a as *mut ExprList_item)
                                                                                .offset(0 as ::core::ffi::c_int as isize))
                                                                                .pExpr,
                                                                            addrCkOk,
                                                                            SQLITE_JUMPIFNULL,
                                                                        );
                                                                        sqlite3VdbeResolveLabel(v, addrCkFault);
                                                                        (*pParse).iSelfTab = 0 as ::core::ffi::c_int;
                                                                        zErr_1 = sqlite3MPrintf(
                                                                            db,
                                                                            b"CHECK constraint failed in %s\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            (*pTab_9).zName,
                                                                        );
                                                                        sqlite3VdbeAddOp4(
                                                                            v,
                                                                            OP_String8,
                                                                            0 as ::core::ffi::c_int,
                                                                            3 as ::core::ffi::c_int,
                                                                            0 as ::core::ffi::c_int,
                                                                            zErr_1,
                                                                            P4_DYNAMIC,
                                                                        );
                                                                        integrityCheckResultRow(v);
                                                                        sqlite3VdbeResolveLabel(v, addrCkOk);
                                                                    }
                                                                    sqlite3ExprListDelete(db, pCheck);
                                                                }
                                                                if isQuick == 0 {
                                                                    j_3 = 0 as ::core::ffi::c_int;
                                                                    pIdx_5 = (*pTab_9).pIndex;
                                                                    while !pIdx_5.is_null() {
                                                                        let mut jmp2_0: ::core::ffi::c_int = 0;
                                                                        let mut jmp3_0: ::core::ffi::c_int = 0;
                                                                        let mut jmp4: ::core::ffi::c_int = 0;
                                                                        let mut jmp5: ::core::ffi::c_int = 0;
                                                                        let mut label6: ::core::ffi::c_int = 0;
                                                                        let mut kk: ::core::ffi::c_int = 0;
                                                                        let mut ckUniq: ::core::ffi::c_int = sqlite3VdbeMakeLabel(
                                                                            pParse,
                                                                        );
                                                                        if !(pPk_0 == pIdx_5) {
                                                                            r1 = sqlite3GenerateIndexKey(
                                                                                pParse,
                                                                                pIdx_5,
                                                                                iDataCur,
                                                                                0 as ::core::ffi::c_int,
                                                                                0 as ::core::ffi::c_int,
                                                                                &raw mut jmp3_0,
                                                                                pPrior,
                                                                                r1,
                                                                            );
                                                                            pPrior = pIdx_5;
                                                                            sqlite3VdbeAddOp2(
                                                                                v,
                                                                                OP_AddImm,
                                                                                8 as ::core::ffi::c_int + j_3,
                                                                                1 as ::core::ffi::c_int,
                                                                            );
                                                                            jmp2_0 = sqlite3VdbeAddOp4Int(
                                                                                v,
                                                                                OP_Found,
                                                                                iIdxCur + j_3,
                                                                                ckUniq,
                                                                                r1,
                                                                                (*pIdx_5).nColumn as ::core::ffi::c_int,
                                                                            );
                                                                            sqlite3VdbeLoadString(
                                                                                v,
                                                                                3 as ::core::ffi::c_int,
                                                                                b"row \0".as_ptr() as *const ::core::ffi::c_char,
                                                                            );
                                                                            sqlite3VdbeAddOp3(
                                                                                v,
                                                                                OP_Concat,
                                                                                7 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                            );
                                                                            sqlite3VdbeLoadString(
                                                                                v,
                                                                                4 as ::core::ffi::c_int,
                                                                                b" missing from index \0".as_ptr()
                                                                                    as *const ::core::ffi::c_char,
                                                                            );
                                                                            sqlite3VdbeAddOp3(
                                                                                v,
                                                                                OP_Concat,
                                                                                4 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                            );
                                                                            jmp5 = sqlite3VdbeLoadString(
                                                                                v,
                                                                                4 as ::core::ffi::c_int,
                                                                                (*pIdx_5).zName,
                                                                            );
                                                                            sqlite3VdbeAddOp3(
                                                                                v,
                                                                                OP_Concat,
                                                                                4 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                            );
                                                                            jmp4 = integrityCheckResultRow(v);
                                                                            sqlite3VdbeJumpHere(v, jmp2_0);
                                                                            if (*pTab_9).tabFlags as ::core::ffi::c_uint
                                                                                & TF_WithoutRowid as ::core::ffi::c_uint
                                                                                == 0 as ::core::ffi::c_uint
                                                                            {
                                                                                let mut jmp7: ::core::ffi::c_int = 0;
                                                                                sqlite3VdbeAddOp2(
                                                                                    v,
                                                                                    OP_IdxRowid,
                                                                                    iIdxCur + j_3,
                                                                                    3 as ::core::ffi::c_int,
                                                                                );
                                                                                jmp7 = sqlite3VdbeAddOp3(
                                                                                    v,
                                                                                    OP_Eq,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    r1 + (*pIdx_5).nColumn as ::core::ffi::c_int
                                                                                        - 1 as ::core::ffi::c_int,
                                                                                );
                                                                                sqlite3VdbeLoadString(
                                                                                    v,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    b"rowid not at end-of-record for row \0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                );
                                                                                sqlite3VdbeAddOp3(
                                                                                    v,
                                                                                    OP_Concat,
                                                                                    7 as ::core::ffi::c_int,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    3 as ::core::ffi::c_int,
                                                                                );
                                                                                sqlite3VdbeLoadString(
                                                                                    v,
                                                                                    4 as ::core::ffi::c_int,
                                                                                    b" of index \0".as_ptr() as *const ::core::ffi::c_char,
                                                                                );
                                                                                sqlite3VdbeGoto(v, jmp5 - 1 as ::core::ffi::c_int);
                                                                                sqlite3VdbeJumpHere(v, jmp7);
                                                                            }
                                                                            label6 = 0 as ::core::ffi::c_int;
                                                                            kk = 0 as ::core::ffi::c_int;
                                                                            while kk < (*pIdx_5).nKeyCol as ::core::ffi::c_int {
                                                                                if !(*(*pIdx_5).azColl.offset(kk as isize)
                                                                                    == &raw const sqlite3StrBINARY
                                                                                        as *const ::core::ffi::c_char)
                                                                                {
                                                                                    if label6 == 0 as ::core::ffi::c_int {
                                                                                        label6 = sqlite3VdbeMakeLabel(pParse);
                                                                                    }
                                                                                    sqlite3VdbeAddOp3(
                                                                                        v,
                                                                                        OP_Column,
                                                                                        iIdxCur + j_3,
                                                                                        kk,
                                                                                        3 as ::core::ffi::c_int,
                                                                                    );
                                                                                    sqlite3VdbeAddOp3(
                                                                                        v,
                                                                                        OP_Ne,
                                                                                        3 as ::core::ffi::c_int,
                                                                                        label6,
                                                                                        r1 + kk,
                                                                                    );
                                                                                }
                                                                                kk += 1;
                                                                            }
                                                                            if label6 != 0 {
                                                                                let mut jmp6: ::core::ffi::c_int = sqlite3VdbeAddOp0(
                                                                                    v,
                                                                                    OP_Goto,
                                                                                );
                                                                                sqlite3VdbeResolveLabel(v, label6);
                                                                                sqlite3VdbeLoadString(
                                                                                    v,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    b"row \0".as_ptr() as *const ::core::ffi::c_char,
                                                                                );
                                                                                sqlite3VdbeAddOp3(
                                                                                    v,
                                                                                    OP_Concat,
                                                                                    7 as ::core::ffi::c_int,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    3 as ::core::ffi::c_int,
                                                                                );
                                                                                sqlite3VdbeLoadString(
                                                                                    v,
                                                                                    4 as ::core::ffi::c_int,
                                                                                    b" values differ from index \0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                );
                                                                                sqlite3VdbeGoto(v, jmp5 - 1 as ::core::ffi::c_int);
                                                                                sqlite3VdbeJumpHere(v, jmp6);
                                                                            }
                                                                            if (*pIdx_5).onError as ::core::ffi::c_int != OE_None {
                                                                                let mut uniqOk: ::core::ffi::c_int = sqlite3VdbeMakeLabel(
                                                                                    pParse,
                                                                                );
                                                                                let mut jmp6_0: ::core::ffi::c_int = 0;
                                                                                kk = 0 as ::core::ffi::c_int;
                                                                                while kk < (*pIdx_5).nKeyCol as ::core::ffi::c_int {
                                                                                    let mut iCol_0: ::core::ffi::c_int = *(*pIdx_5)
                                                                                        .aiColumn
                                                                                        .offset(kk as isize) as ::core::ffi::c_int;
                                                                                    if !(iCol_0 >= 0 as ::core::ffi::c_int
                                                                                        && (*(*pTab_9).aCol.offset(iCol_0 as isize)).notNull()
                                                                                            as ::core::ffi::c_int != 0)
                                                                                    {
                                                                                        sqlite3VdbeAddOp2(v, OP_IsNull, r1 + kk, uniqOk);
                                                                                    }
                                                                                    kk += 1;
                                                                                }
                                                                                jmp6_0 = sqlite3VdbeAddOp1(v, OP_Next, iIdxCur + j_3);
                                                                                sqlite3VdbeGoto(v, uniqOk);
                                                                                sqlite3VdbeJumpHere(v, jmp6_0);
                                                                                sqlite3VdbeAddOp4Int(
                                                                                    v,
                                                                                    OP_IdxGT,
                                                                                    iIdxCur + j_3,
                                                                                    uniqOk,
                                                                                    r1,
                                                                                    (*pIdx_5).nKeyCol as ::core::ffi::c_int,
                                                                                );
                                                                                sqlite3VdbeLoadString(
                                                                                    v,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    b"non-unique entry in index \0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                );
                                                                                sqlite3VdbeGoto(v, jmp5);
                                                                                sqlite3VdbeResolveLabel(v, uniqOk);
                                                                            }
                                                                            sqlite3VdbeJumpHere(v, jmp4);
                                                                            sqlite3ResolvePartIdxLabel(pParse, jmp3_0);
                                                                        }
                                                                        pIdx_5 = (*pIdx_5).pNext;
                                                                        j_3 += 1;
                                                                    }
                                                                }
                                                                sqlite3VdbeAddOp2(v, OP_Next, iDataCur, loopTop);
                                                                sqlite3VdbeJumpHere(v, loopTop - 1 as ::core::ffi::c_int);
                                                                if !pPk_0.is_null() {
                                                                    sqlite3ReleaseTempRange(
                                                                        pParse,
                                                                        r2,
                                                                        (*pPk_0).nKeyCol as ::core::ffi::c_int,
                                                                    );
                                                                }
                                                            }
                                                        }
                                                        x_1 = (*x_1).next;
                                                    }
                                                    let mut c2rust_current_block_829: u64;
                                                    x_1 = (*pTbls).first;
                                                    while !x_1.is_null() {
                                                        let mut pTab_10: *mut Table = (*x_1).data as *mut Table;
                                                        let mut pVTab: *mut sqlite3_vtab = ::core::ptr::null_mut::<
                                                            sqlite3_vtab,
                                                        >();
                                                        let mut a1_0: ::core::ffi::c_int = 0;
                                                        if !(tableSkipIntegrityCheck(pTab_10, pObjTab) != 0) {
                                                            if !((*pTab_10).eTabType as ::core::ffi::c_int
                                                                == TABTYP_NORM)
                                                            {
                                                                if (*pTab_10).eTabType as ::core::ffi::c_int == TABTYP_VTAB
                                                                {
                                                                    if (*pTab_10).nCol as ::core::ffi::c_int
                                                                        <= 0 as ::core::ffi::c_int
                                                                    {
                                                                        let mut zMod: *const ::core::ffi::c_char = *(*pTab_10)
                                                                            .u
                                                                            .vtab
                                                                            .azArg
                                                                            .offset(0 as ::core::ffi::c_int as isize);
                                                                        if sqlite3HashFind(&raw mut (*db).aModule, zMod).is_null() {
                                                                            c2rust_current_block_829 = 5603807564562889783;
                                                                        } else {
                                                                            c2rust_current_block_829 = 3336758898438370709;
                                                                        }
                                                                    } else {
                                                                        c2rust_current_block_829 = 3336758898438370709;
                                                                    }
                                                                    match c2rust_current_block_829 {
                                                                        5603807564562889783 => {}
                                                                        _ => {
                                                                            sqlite3ViewGetColumnNames(pParse, pTab_10);
                                                                            if !(*pTab_10).u.vtab.p.is_null() {
                                                                                pVTab = (*(*pTab_10).u.vtab.p).pVtab;
                                                                                if !pVTab.is_null() {
                                                                                    if !(*pVTab).pModule.is_null() {
                                                                                        if !((*(*pVTab).pModule).iVersion < 4 as ::core::ffi::c_int)
                                                                                        {
                                                                                            if !(*(*pVTab).pModule).xIntegrity.is_none() {
                                                                                                sqlite3VdbeAddOp3(
                                                                                                    v,
                                                                                                    OP_VCheck,
                                                                                                    i_8,
                                                                                                    3 as ::core::ffi::c_int,
                                                                                                    isQuick,
                                                                                                );
                                                                                                (*pTab_10).nTabRef = (*pTab_10).nTabRef.wrapping_add(1);
                                                                                                sqlite3VdbeAppendP4(
                                                                                                    v,
                                                                                                    pTab_10 as *mut ::core::ffi::c_void,
                                                                                                    P4_TABLEREF,
                                                                                                );
                                                                                                a1_0 = sqlite3VdbeAddOp1(
                                                                                                    v,
                                                                                                    OP_IsNull,
                                                                                                    3 as ::core::ffi::c_int,
                                                                                                );
                                                                                                integrityCheckResultRow(v);
                                                                                                sqlite3VdbeJumpHere(v, a1_0);
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        x_1 = (*x_1).next;
                                                    }
                                                }
                                            }
                                        }
                                        i_8 += 1;
                                    }
                                    static mut iLn_0: ::core::ffi::c_int = 0
                                        as ::core::ffi::c_int;
                                    static mut endCode: [VdbeOpList; 7] = [
                                        VdbeOpList {
                                            opcode: OP_AddImm as u8_0,
                                            p1: 1 as ::core::ffi::c_schar,
                                            p2: 0 as ::core::ffi::c_schar,
                                            p3: 0 as ::core::ffi::c_schar

                                        },
                                        VdbeOpList {
                                            opcode: OP_IfNotZero as u8_0,
                                            p1: 1 as ::core::ffi::c_schar,
                                            p2: 4 as ::core::ffi::c_schar,
                                            p3: 0 as ::core::ffi::c_schar

                                        },
                                        VdbeOpList {
                                            opcode: OP_String8 as u8_0,
                                            p1: 0 as ::core::ffi::c_schar,
                                            p2: 3 as ::core::ffi::c_schar,
                                            p3: 0 as ::core::ffi::c_schar

                                        },
                                        VdbeOpList {
                                            opcode: OP_ResultRow as u8_0,
                                            p1: 3 as ::core::ffi::c_schar,
                                            p2: 1 as ::core::ffi::c_schar,
                                            p3: 0 as ::core::ffi::c_schar

                                        },
                                        VdbeOpList {
                                            opcode: OP_Halt as u8_0,
                                            p1: 0 as ::core::ffi::c_schar,
                                            p2: 0 as ::core::ffi::c_schar,
                                            p3: 0 as ::core::ffi::c_schar

                                        },
                                        VdbeOpList {
                                            opcode: OP_String8 as u8_0,
                                            p1: 0 as ::core::ffi::c_schar,
                                            p2: 3 as ::core::ffi::c_schar,
                                            p3: 0 as ::core::ffi::c_schar

                                        },
                                        VdbeOpList {
                                            opcode: OP_Goto as u8_0,
                                            p1: 0 as ::core::ffi::c_schar,
                                            p2: 3 as ::core::ffi::c_schar,
                                            p3: 0 as ::core::ffi::c_schar

                                        },
                                    ];
                                    let mut aOp_0: *mut VdbeOp = ::core::ptr::null_mut::<
                                        VdbeOp,
                                    >();
                                    aOp_0 = sqlite3VdbeAddOpList(
                                        v,
                                        (::core::mem::size_of::<[VdbeOpList; 7]>() as usize)
                                            .wrapping_div(::core::mem::size_of::<VdbeOpList>() as usize)
                                            as ::core::ffi::c_int,
                                        &raw const endCode as *const VdbeOpList,
                                        iLn_0,
                                    );
                                    if !aOp_0.is_null() {
                                        (*aOp_0.offset(0 as ::core::ffi::c_int as isize)).p2 = 1
                                            as ::core::ffi::c_int - mxErr;
                                        (*aOp_0.offset(2 as ::core::ffi::c_int as isize)).p4type = P4_STATIC
                                            as ::core::ffi::c_schar;
                                        let ref mut c2rust_fresh3 = (*aOp_0
                                            .offset(2 as ::core::ffi::c_int as isize))
                                            .p4
                                            .z;
                                        *c2rust_fresh3 = b"ok\0".as_ptr()
                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                        (*aOp_0.offset(5 as ::core::ffi::c_int as isize)).p4type = P4_STATIC
                                            as ::core::ffi::c_schar;
                                        let ref mut c2rust_fresh4 = (*aOp_0
                                            .offset(5 as ::core::ffi::c_int as isize))
                                            .p4
                                            .z;
                                        *c2rust_fresh4 = sqlite3ErrStr(SQLITE_CORRUPT)
                                            as *mut ::core::ffi::c_char;
                                    }
                                    sqlite3VdbeChangeP3(
                                        v,
                                        0 as ::core::ffi::c_int,
                                        sqlite3VdbeCurrentAddr(v) - 2 as ::core::ffi::c_int,
                                    );
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_ENCODING => {
                                    static mut encnames: [EncName; 9] = [
                                        EncName {
                                            zName: b"UTF8\0".as_ptr() as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                            enc: SQLITE_UTF8 as u8_0

                                        },
                                        EncName {
                                            zName: b"UTF-8\0".as_ptr() as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                            enc: SQLITE_UTF8 as u8_0

                                        },
                                        EncName {
                                            zName: b"UTF-16le\0".as_ptr() as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                            enc: SQLITE_UTF16LE as u8_0

                                        },
                                        EncName {
                                            zName: b"UTF-16be\0".as_ptr() as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                            enc: SQLITE_UTF16BE as u8_0

                                        },
                                        EncName {
                                            zName: b"UTF16le\0".as_ptr() as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                            enc: SQLITE_UTF16LE as u8_0

                                        },
                                        EncName {
                                            zName: b"UTF16be\0".as_ptr() as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                            enc: SQLITE_UTF16BE as u8_0

                                        },
                                        EncName {
                                            zName: b"UTF-16\0".as_ptr() as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                            enc: 0 as u8_0

                                        },
                                        EncName {
                                            zName: b"UTF16\0".as_ptr() as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                            enc: 0 as u8_0

                                        },
                                        EncName {
                                            zName: ::core::ptr::null::<::core::ffi::c_char>()
                                                as *mut ::core::ffi::c_char,
                                            enc: 0 as u8_0

                                        },
                                    ];
                                    let mut pEnc: *const EncName = ::core::ptr::null::<
                                        EncName,
                                    >();
                                    if zRight.is_null() {
                                        if sqlite3ReadSchema(pParse) != 0 {
                                            c2rust_current_block = 4749248788619550567;
                                        } else {
                                            returnSingleText(
                                                v,
                                                encnames[(*(*pParse).db).enc as usize].zName,
                                            );
                                            c2rust_current_block = 10387754024080658218;
                                        }
                                    } else {
                                        if (*db).mDbFlags as ::core::ffi::c_uint
                                            & DBFLAG_EncodingFixed as ::core::ffi::c_uint
                                            == 0 as ::core::ffi::c_uint
                                        {
                                            pEnc = (&raw const encnames as *const EncName)
                                                .offset(0 as ::core::ffi::c_int as isize) as *const EncName
                                                as *const EncName;
                                            while !(*pEnc).zName.is_null() {
                                                if 0 as ::core::ffi::c_int
                                                    == sqlite3StrICmp(zRight, (*pEnc).zName)
                                                {
                                                    let mut enc: u8_0 = (if (*pEnc).enc as ::core::ffi::c_int
                                                        != 0
                                                    {
                                                        (*pEnc).enc as ::core::ffi::c_int
                                                    } else {
                                                        SQLITE_UTF16NATIVE
                                                    }) as u8_0;
                                                    (*(*(*db).aDb.offset(0 as ::core::ffi::c_int as isize))
                                                        .pSchema)
                                                        .enc = enc;
                                                    sqlite3SetTextEncoding(db, enc);
                                                    break;
                                                } else {
                                                    pEnc = pEnc.offset(1);
                                                }
                                            }
                                            if (*pEnc).zName.is_null() {
                                                sqlite3ErrorMsg(
                                                    pParse,
                                                    b"unsupported encoding: %s\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    zRight,
                                                );
                                            }
                                        }
                                        c2rust_current_block = 10387754024080658218;
                                    }
                                }
                                PragTyp_HEADER_VALUE => {
                                    let mut iCookie: ::core::ffi::c_int = (*pPragma).iArg
                                        as ::core::ffi::c_int;
                                    sqlite3VdbeUsesBtree(v, iDb);
                                    if !zRight.is_null()
                                        && (*pPragma).mPragFlg as ::core::ffi::c_int
                                            & PragFlg_ReadOnly == 0 as ::core::ffi::c_int
                                    {
                                        static mut setCookie: [VdbeOpList; 2] = [
                                            VdbeOpList {
                                                opcode: OP_Transaction as u8_0,
                                                p1: 0 as ::core::ffi::c_schar,
                                                p2: 1 as ::core::ffi::c_schar,
                                                p3: 0 as ::core::ffi::c_schar

                                            },
                                            VdbeOpList {
                                                opcode: OP_SetCookie as u8_0,
                                                p1: 0 as ::core::ffi::c_schar,
                                                p2: 0 as ::core::ffi::c_schar,
                                                p3: 0 as ::core::ffi::c_schar

                                            },
                                        ];
                                        let mut aOp_1: *mut VdbeOp = ::core::ptr::null_mut::<
                                            VdbeOp,
                                        >();
                                        aOp_1 = sqlite3VdbeAddOpList(
                                            v,
                                            (::core::mem::size_of::<[VdbeOpList; 2]>() as usize)
                                                .wrapping_div(::core::mem::size_of::<VdbeOpList>() as usize)
                                                as ::core::ffi::c_int,
                                            &raw const setCookie as *const VdbeOpList,
                                            0 as ::core::ffi::c_int,
                                        );
                                        (*aOp_1.offset(0 as ::core::ffi::c_int as isize)).p1 = iDb;
                                        (*aOp_1.offset(1 as ::core::ffi::c_int as isize)).p1 = iDb;
                                        (*aOp_1.offset(1 as ::core::ffi::c_int as isize)).p2 = iCookie;
                                        (*aOp_1.offset(1 as ::core::ffi::c_int as isize)).p3 = sqlite3Atoi(
                                            zRight,
                                        );
                                        (*aOp_1.offset(1 as ::core::ffi::c_int as isize)).p5 = 1
                                            as u16_0;
                                        if iCookie == BTREE_SCHEMA_VERSION
                                            && (*db).flags as ::core::ffi::c_ulonglong
                                                & SQLITE_Defensive as ::core::ffi::c_ulonglong
                                                != 0 as ::core::ffi::c_ulonglong
                                        {
                                            (*aOp_1.offset(1 as ::core::ffi::c_int as isize)).opcode = OP_Noop
                                                as u8_0;
                                        }
                                    } else {
                                        static mut readCookie: [VdbeOpList; 3] = [
                                            VdbeOpList {
                                                opcode: OP_Transaction as u8_0,
                                                p1: 0 as ::core::ffi::c_schar,
                                                p2: 0 as ::core::ffi::c_schar,
                                                p3: 0 as ::core::ffi::c_schar

                                            },
                                            VdbeOpList {
                                                opcode: OP_ReadCookie as u8_0,
                                                p1: 0 as ::core::ffi::c_schar,
                                                p2: 1 as ::core::ffi::c_schar,
                                                p3: 0 as ::core::ffi::c_schar

                                            },
                                            VdbeOpList {
                                                opcode: OP_ResultRow as u8_0,
                                                p1: 1 as ::core::ffi::c_schar,
                                                p2: 1 as ::core::ffi::c_schar,
                                                p3: 0 as ::core::ffi::c_schar

                                            },
                                        ];
                                        let mut aOp_2: *mut VdbeOp = ::core::ptr::null_mut::<
                                            VdbeOp,
                                        >();
                                        aOp_2 = sqlite3VdbeAddOpList(
                                            v,
                                            (::core::mem::size_of::<[VdbeOpList; 3]>() as usize)
                                                .wrapping_div(::core::mem::size_of::<VdbeOpList>() as usize)
                                                as ::core::ffi::c_int,
                                            &raw const readCookie as *const VdbeOpList,
                                            0 as ::core::ffi::c_int,
                                        );
                                        (*aOp_2.offset(0 as ::core::ffi::c_int as isize)).p1 = iDb;
                                        (*aOp_2.offset(1 as ::core::ffi::c_int as isize)).p1 = iDb;
                                        (*aOp_2.offset(1 as ::core::ffi::c_int as isize)).p3 = iCookie;
                                        sqlite3VdbeReusable(v);
                                    }
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_COMPILE_OPTIONS => {
                                    let mut i_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    let mut zOpt: *const ::core::ffi::c_char = ::core::ptr::null::<
                                        ::core::ffi::c_char,
                                    >();
                                    (*pParse).nMem = 1 as ::core::ffi::c_int;
                                    loop {
                                        let c2rust_fresh5 = i_9;
                                        i_9 = i_9 + 1;
                                        zOpt = sqlite3_compileoption_get(c2rust_fresh5);
                                        if zOpt.is_null() {
                                            break;
                                        }
                                        sqlite3VdbeLoadString(v, 1 as ::core::ffi::c_int, zOpt);
                                        sqlite3VdbeAddOp2(
                                            v,
                                            OP_ResultRow,
                                            1 as ::core::ffi::c_int,
                                            1 as ::core::ffi::c_int,
                                        );
                                    }
                                    sqlite3VdbeReusable(v);
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_WAL_CHECKPOINT => {
                                    let mut iBt: ::core::ffi::c_int = if !(*pId2).z.is_null() {
                                        iDb
                                    } else {
                                        SQLITE_MAX_DB
                                    };
                                    let mut eMode_1: ::core::ffi::c_int = SQLITE_CHECKPOINT_PASSIVE;
                                    if !zRight.is_null() {
                                        if sqlite3StrICmp(
                                            zRight,
                                            b"full\0".as_ptr() as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            eMode_1 = SQLITE_CHECKPOINT_FULL;
                                        } else if sqlite3StrICmp(
                                            zRight,
                                            b"restart\0".as_ptr() as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            eMode_1 = SQLITE_CHECKPOINT_RESTART;
                                        } else if sqlite3StrICmp(
                                            zRight,
                                            b"truncate\0".as_ptr() as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            eMode_1 = SQLITE_CHECKPOINT_TRUNCATE;
                                        } else if sqlite3StrICmp(
                                            zRight,
                                            b"noop\0".as_ptr() as *const ::core::ffi::c_char,
                                        ) == 0 as ::core::ffi::c_int
                                        {
                                            eMode_1 = SQLITE_CHECKPOINT_NOOP;
                                        }
                                    }
                                    (*pParse).nMem = 3 as ::core::ffi::c_int;
                                    sqlite3VdbeAddOp3(
                                        v,
                                        OP_Checkpoint,
                                        iBt,
                                        eMode_1,
                                        1 as ::core::ffi::c_int,
                                    );
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_ResultRow,
                                        1 as ::core::ffi::c_int,
                                        3 as ::core::ffi::c_int,
                                    );
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_WAL_AUTOCHECKPOINT => {
                                    if !zRight.is_null() {
                                        sqlite3_wal_autocheckpoint(db, sqlite3Atoi(zRight));
                                    }
                                    returnSingleInt(
                                        v,
                                        (if (*db).xWalCallback
                                            == Some(
                                                sqlite3WalDefaultHook
                                                    as unsafe extern "C" fn(
                                                        *mut ::core::ffi::c_void,
                                                        *mut sqlite3,
                                                        *const ::core::ffi::c_char,
                                                        ::core::ffi::c_int,
                                                    ) -> ::core::ffi::c_int,
                                            )
                                        {
                                            (*db).pWalArg as ::core::ffi::c_long as ::core::ffi::c_int
                                        } else {
                                            0 as ::core::ffi::c_int
                                        }) as i64_0,
                                    );
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_SHRINK_MEMORY => {
                                    sqlite3_db_release_memory(db);
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_OPTIMIZE => {
                                    let mut iDbLast: ::core::ffi::c_int = 0;
                                    let mut iTabCur: ::core::ffi::c_int = 0;
                                    let mut k_3: *mut HashElem = ::core::ptr::null_mut::<
                                        HashElem,
                                    >();
                                    let mut pSchema: *mut Schema = ::core::ptr::null_mut::<
                                        Schema,
                                    >();
                                    let mut pTab_11: *mut Table = ::core::ptr::null_mut::<
                                        Table,
                                    >();
                                    let mut pIdx_6: *mut Index = ::core::ptr::null_mut::<
                                        Index,
                                    >();
                                    let mut szThreshold: LogEst = 0;
                                    let mut zSubSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                        ::core::ffi::c_char,
                                    >();
                                    let mut opMask: u32_0 = 0;
                                    let mut nLimit: ::core::ffi::c_int = 0;
                                    let mut nCheck: ::core::ffi::c_int = 0
                                        as ::core::ffi::c_int;
                                    let mut nBtree: ::core::ffi::c_int = 0
                                        as ::core::ffi::c_int;
                                    let mut nIndex: ::core::ffi::c_int = 0;
                                    if !zRight.is_null() {
                                        opMask = sqlite3Atoi(zRight) as u32_0;
                                        if opMask as ::core::ffi::c_uint
                                            & 0x2 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                                        {
                                            c2rust_current_block = 10387754024080658218;
                                        } else {
                                            c2rust_current_block = 5496625480571732597;
                                        }
                                    } else {
                                        opMask = 0xfffe as u32_0;
                                        c2rust_current_block = 5496625480571732597;
                                    }
                                    match c2rust_current_block {
                                        10387754024080658218 => {}
                                        _ => {
                                            if opMask as ::core::ffi::c_uint
                                                & 0x10 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                                            {
                                                nLimit = 0 as ::core::ffi::c_int;
                                            } else if (*db).nAnalysisLimit > 0 as ::core::ffi::c_int
                                                && (*db).nAnalysisLimit < SQLITE_DEFAULT_OPTIMIZE_LIMIT
                                            {
                                                nLimit = 0 as ::core::ffi::c_int;
                                            } else {
                                                nLimit = SQLITE_DEFAULT_OPTIMIZE_LIMIT;
                                            }
                                            let c2rust_fresh6 = (*pParse).nTab;
                                            (*pParse).nTab = (*pParse).nTab + 1;
                                            iTabCur = c2rust_fresh6;
                                            iDbLast = if !zDb.is_null() {
                                                iDb
                                            } else {
                                                (*db).nDb - 1 as ::core::ffi::c_int
                                            };
                                            while iDb <= iDbLast {
                                                if !(iDb == 1 as ::core::ffi::c_int) {
                                                    sqlite3CodeVerifySchema(pParse, iDb);
                                                    pSchema = (*(*db).aDb.offset(iDb as isize)).pSchema;
                                                    let mut c2rust_current_block_953: u64;
                                                    k_3 = (*pSchema).tblHash.first;
                                                    while !k_3.is_null() {
                                                        pTab_11 = (*k_3).data as *mut Table;
                                                        if (*pTab_11).eTabType as ::core::ffi::c_int == TABTYP_NORM
                                                        {
                                                            if !(0 as ::core::ffi::c_int
                                                                == sqlite3_strnicmp(
                                                                    (*pTab_11).zName,
                                                                    b"sqlite_\0".as_ptr() as *const ::core::ffi::c_char,
                                                                    7 as ::core::ffi::c_int,
                                                                ))
                                                            {
                                                                szThreshold = (*pTab_11).nRowLogEst;
                                                                nIndex = 0 as ::core::ffi::c_int;
                                                                pIdx_6 = (*pTab_11).pIndex;
                                                                while !pIdx_6.is_null() {
                                                                    nIndex += 1;
                                                                    if (*pIdx_6).hasStat1() == 0 {
                                                                        szThreshold = -1 as ::core::ffi::c_int as LogEst;
                                                                    }
                                                                    pIdx_6 = (*pIdx_6).pNext;
                                                                }
                                                                if (*pTab_11).tabFlags as ::core::ffi::c_uint
                                                                    & TF_MaybeReanalyze as ::core::ffi::c_uint
                                                                    != 0 as ::core::ffi::c_uint
                                                                {
                                                                    c2rust_current_block_953 = 16110670689437439712;
                                                                } else if opMask as ::core::ffi::c_uint
                                                                    & 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                                                                {
                                                                    c2rust_current_block_953 = 16110670689437439712;
                                                                } else if !(*pTab_11).pIndex.is_null()
                                                                    && (szThreshold as ::core::ffi::c_int)
                                                                        < 0 as ::core::ffi::c_int
                                                                {
                                                                    c2rust_current_block_953 = 16110670689437439712;
                                                                } else {
                                                                    c2rust_current_block_953 = 15611964311717037170;
                                                                }
                                                                match c2rust_current_block_953 {
                                                                    15611964311717037170 => {}
                                                                    _ => {
                                                                        nCheck += 1;
                                                                        if nCheck == 2 as ::core::ffi::c_int {
                                                                            sqlite3BeginWriteOperation(
                                                                                pParse,
                                                                                0 as ::core::ffi::c_int,
                                                                                iDb,
                                                                            );
                                                                        }
                                                                        nBtree += nIndex + 1 as ::core::ffi::c_int;
                                                                        sqlite3OpenTable(
                                                                            pParse,
                                                                            iTabCur,
                                                                            iDb,
                                                                            pTab_11,
                                                                            OP_OpenRead,
                                                                        );
                                                                        if szThreshold as ::core::ffi::c_int
                                                                            >= 0 as ::core::ffi::c_int
                                                                        {
                                                                            let iRange: LogEst = 33 as LogEst;
                                                                            sqlite3VdbeAddOp4Int(
                                                                                v,
                                                                                OP_IfSizeBetween,
                                                                                iTabCur,
                                                                                ((sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int)
                                                                                    as ::core::ffi::c_uint)
                                                                                    .wrapping_add(
                                                                                        opMask as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint,
                                                                                    ) as ::core::ffi::c_int,
                                                                                if szThreshold as ::core::ffi::c_int
                                                                                    >= iRange as ::core::ffi::c_int
                                                                                {
                                                                                    szThreshold as ::core::ffi::c_int
                                                                                        - iRange as ::core::ffi::c_int
                                                                                } else {
                                                                                    -1 as ::core::ffi::c_int
                                                                                },
                                                                                szThreshold as ::core::ffi::c_int
                                                                                    + iRange as ::core::ffi::c_int,
                                                                            );
                                                                        } else {
                                                                            sqlite3VdbeAddOp2(
                                                                                v,
                                                                                OP_Rewind,
                                                                                iTabCur,
                                                                                ((sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int)
                                                                                    as ::core::ffi::c_uint)
                                                                                    .wrapping_add(
                                                                                        opMask as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint,
                                                                                    ) as ::core::ffi::c_int,
                                                                            );
                                                                        }
                                                                        zSubSql = sqlite3MPrintf(
                                                                            db,
                                                                            b"ANALYZE \"%w\".\"%w\"\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            (*(*db).aDb.offset(iDb as isize)).zDbSName,
                                                                            (*pTab_11).zName,
                                                                        );
                                                                        if opMask as ::core::ffi::c_uint
                                                                            & 0x1 as ::core::ffi::c_uint != 0
                                                                        {
                                                                            let mut r1_0: ::core::ffi::c_int = sqlite3GetTempReg(
                                                                                pParse,
                                                                            );
                                                                            sqlite3VdbeAddOp4(
                                                                                v,
                                                                                OP_String8,
                                                                                0 as ::core::ffi::c_int,
                                                                                r1_0,
                                                                                0 as ::core::ffi::c_int,
                                                                                zSubSql,
                                                                                P4_DYNAMIC,
                                                                            );
                                                                            sqlite3VdbeAddOp2(
                                                                                v,
                                                                                OP_ResultRow,
                                                                                r1_0,
                                                                                1 as ::core::ffi::c_int,
                                                                            );
                                                                        } else {
                                                                            sqlite3VdbeAddOp4(
                                                                                v,
                                                                                OP_SqlExec,
                                                                                if nLimit != 0 {
                                                                                    0x2 as ::core::ffi::c_int
                                                                                } else {
                                                                                    0 as ::core::ffi::c_int
                                                                                },
                                                                                nLimit,
                                                                                0 as ::core::ffi::c_int,
                                                                                zSubSql,
                                                                                P4_DYNAMIC,
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        k_3 = (*k_3).next;
                                                    }
                                                }
                                                iDb += 1;
                                            }
                                            sqlite3VdbeAddOp0(v, OP_Expire);
                                            if (*db).mallocFailed == 0
                                                && nLimit > 0 as ::core::ffi::c_int
                                                && nBtree > 100 as ::core::ffi::c_int
                                            {
                                                let mut iAddr_0: ::core::ffi::c_int = 0;
                                                let mut iEnd: ::core::ffi::c_int = 0;
                                                let mut aOp_3: *mut VdbeOp = ::core::ptr::null_mut::<
                                                    VdbeOp,
                                                >();
                                                nLimit = 100 as ::core::ffi::c_int * nLimit / nBtree;
                                                if nLimit < 100 as ::core::ffi::c_int {
                                                    nLimit = 100 as ::core::ffi::c_int;
                                                }
                                                aOp_3 = sqlite3VdbeGetOp(v, 0 as ::core::ffi::c_int);
                                                iEnd = sqlite3VdbeCurrentAddr(v);
                                                iAddr_0 = 0 as ::core::ffi::c_int;
                                                while iAddr_0 < iEnd {
                                                    if (*aOp_3.offset(iAddr_0 as isize)).opcode
                                                        as ::core::ffi::c_int == OP_SqlExec
                                                    {
                                                        (*aOp_3.offset(iAddr_0 as isize)).p2 = nLimit;
                                                    }
                                                    iAddr_0 += 1;
                                                }
                                            }
                                            c2rust_current_block = 10387754024080658218;
                                        }
                                    }
                                }
                                PragTyp_SOFT_HEAP_LIMIT => {
                                    let mut N: sqlite3_int64 = 0;
                                    if !zRight.is_null()
                                        && sqlite3DecOrHexToI64(zRight, &raw mut N) == SQLITE_OK
                                    {
                                        sqlite3_soft_heap_limit64(N);
                                    }
                                    returnSingleInt(
                                        v,
                                        sqlite3_soft_heap_limit64(
                                            -1 as ::core::ffi::c_int as sqlite3_int64,
                                        ) as i64_0,
                                    );
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_HARD_HEAP_LIMIT => {
                                    let mut N_0: sqlite3_int64 = 0;
                                    if !zRight.is_null()
                                        && sqlite3DecOrHexToI64(zRight, &raw mut N_0) == SQLITE_OK
                                    {
                                        let mut iPrior: sqlite3_int64 = sqlite3_hard_heap_limit64(
                                            -1 as ::core::ffi::c_int as sqlite3_int64,
                                        );
                                        if N_0 > 0 as ::core::ffi::c_longlong
                                            && (iPrior == 0 as ::core::ffi::c_longlong || iPrior > N_0)
                                        {
                                            sqlite3_hard_heap_limit64(N_0);
                                        }
                                    }
                                    returnSingleInt(
                                        v,
                                        sqlite3_hard_heap_limit64(
                                            -1 as ::core::ffi::c_int as sqlite3_int64,
                                        ) as i64_0,
                                    );
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_THREADS => {
                                    let mut N_1: sqlite3_int64 = 0;
                                    if !zRight.is_null()
                                        && sqlite3DecOrHexToI64(zRight, &raw mut N_1) == SQLITE_OK
                                        && N_1 >= 0 as ::core::ffi::c_longlong
                                    {
                                        sqlite3_limit(
                                            db,
                                            SQLITE_LIMIT_WORKER_THREADS,
                                            (N_1 as ::core::ffi::c_longlong
                                                & 0x7fffffff as ::core::ffi::c_longlong)
                                                as ::core::ffi::c_int,
                                        );
                                    }
                                    returnSingleInt(
                                        v,
                                        sqlite3_limit(
                                            db,
                                            SQLITE_LIMIT_WORKER_THREADS,
                                            -1 as ::core::ffi::c_int,
                                        ) as i64_0,
                                    );
                                    c2rust_current_block = 10387754024080658218;
                                }
                                PragTyp_ANALYSIS_LIMIT => {
                                    let mut N_2: sqlite3_int64 = 0;
                                    if !zRight.is_null()
                                        && sqlite3DecOrHexToI64(zRight, &raw mut N_2) == SQLITE_OK
                                        && N_2 >= 0 as ::core::ffi::c_longlong
                                    {
                                        (*db).nAnalysisLimit = (N_2 as ::core::ffi::c_longlong
                                            & 0x7fffffff as ::core::ffi::c_longlong)
                                            as ::core::ffi::c_int;
                                    }
                                    returnSingleInt(v, (*db).nAnalysisLimit as i64_0);
                                    c2rust_current_block = 10387754024080658218;
                                }
                                _ => {
                                    if !zRight.is_null() {
                                        sqlite3_busy_timeout(db, sqlite3Atoi(zRight));
                                    }
                                    returnSingleInt(v, (*db).busyTimeout as i64_0);
                                    c2rust_current_block = 10387754024080658218;
                                }
                            }
                            match c2rust_current_block {
                                4749248788619550567 => {}
                                _ => {
                                    (*pPragma).mPragFlg as ::core::ffi::c_int
                                        & PragFlg_NoColumns1 != 0 && !zRight.is_null();
                                }
                            }
                        }
                    }
                }
            }
        }
        sqlite3DbFree(db, zLeft as *mut ::core::ffi::c_void);
        sqlite3DbFree(db, zRight as *mut ::core::ffi::c_void);
    }
}
pub const SQLITE_INTEGRITY_CHECK_ERROR_MAX: ::core::ffi::c_int = 100
    as ::core::ffi::c_int;
unsafe extern "C" fn pragmaVtabConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPragma: *const PragmaName = pAux as *const PragmaName;
        let mut pTab: *mut PragmaVtab = ::core::ptr::null_mut::<PragmaVtab>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut cSep: ::core::ffi::c_char = '(' as i32 as ::core::ffi::c_char;
        let mut acc: StrAccum = StrAccum {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        let mut zBuf: [::core::ffi::c_char; 200] = [0; 200];
        sqlite3StrAccumInit(
            &raw mut acc,
            ::core::ptr::null_mut::<sqlite3>(),
            &raw mut zBuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        sqlite3_str_appendall(
            &raw mut acc,
            b"CREATE TABLE x\0".as_ptr() as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        j = (*pPragma).iPragCName as ::core::ffi::c_int;
        while i < (*pPragma).nPragCName as ::core::ffi::c_int {
            sqlite3_str_appendf(
                &raw mut acc,
                b"%c\"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                cSep as ::core::ffi::c_int,
                pragCName[j as usize],
            );
            cSep = ',' as i32 as ::core::ffi::c_char;
            i += 1;
            j += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            sqlite3_str_appendf(
                &raw mut acc,
                b"(\"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                (*pPragma).zName,
            );
            i += 1;
        }
        j = 0 as ::core::ffi::c_int;
        if (*pPragma).mPragFlg as ::core::ffi::c_int & PragFlg_Result1 != 0 {
            sqlite3_str_appendall(
                &raw mut acc,
                b",arg HIDDEN\0".as_ptr() as *const ::core::ffi::c_char,
            );
            j += 1;
        }
        if (*pPragma).mPragFlg as ::core::ffi::c_int
            & (PragFlg_SchemaOpt | PragFlg_SchemaReq) != 0
        {
            sqlite3_str_appendall(
                &raw mut acc,
                b",schema HIDDEN\0".as_ptr() as *const ::core::ffi::c_char,
            );
            j += 1;
        }
        sqlite3_str_append(
            &raw mut acc,
            b")\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        sqlite3StrAccumFinish(&raw mut acc);
        rc = sqlite3_declare_vtab(db, &raw mut zBuf as *mut ::core::ffi::c_char);
        if rc == SQLITE_OK {
            pTab = sqlite3_malloc(
                ::core::mem::size_of::<PragmaVtab>() as ::core::ffi::c_int,
            ) as *mut PragmaVtab;
            if pTab.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                memset(
                    pTab as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<PragmaVtab>() as size_t,
                );
                (*pTab).pName = pPragma;
                (*pTab).db = db;
                (*pTab).iHidden = i as u8_0;
                (*pTab).nHidden = j as u8_0;
            }
        } else {
            *pzErr = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
        }
        *ppVtab = pTab as *mut sqlite3_vtab;
        return rc;
    }
}
unsafe extern "C" fn pragmaVtabDisconnect(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut PragmaVtab = pVtab as *mut PragmaVtab;
        sqlite3_free(pTab as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pragmaVtabBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut PragmaVtab = tab as *mut PragmaVtab;
        let mut pConstraint: *const sqlite3_index_constraint = ::core::ptr::null::<
            sqlite3_index_constraint,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut seen: [::core::ffi::c_int; 2] = [0; 2];
        (*pIdxInfo).estimatedCost = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        if (*pTab).nHidden as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return SQLITE_OK;
        }
        pConstraint = (*pIdxInfo).aConstraint;
        seen[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
        seen[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdxInfo).nConstraint {
            if !((*pConstraint).iColumn < (*pTab).iHidden as ::core::ffi::c_int) {
                if !((*pConstraint).op as ::core::ffi::c_int
                    != SQLITE_INDEX_CONSTRAINT_EQ)
                {
                    if (*pConstraint).usable as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        return SQLITE_CONSTRAINT;
                    }
                    j = (*pConstraint).iColumn - (*pTab).iHidden as ::core::ffi::c_int;
                    seen[j as usize] = i + 1 as ::core::ffi::c_int;
                }
            }
            i += 1;
            pConstraint = pConstraint.offset(1);
        }
        if seen[0 as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
            (*pIdxInfo).estimatedCost = 2147483647 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            (*pIdxInfo).estimatedRows = 2147483647 as sqlite3_int64;
            return SQLITE_OK;
        }
        j = seen[0 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int;
        (*(*pIdxInfo).aConstraintUsage.offset(j as isize)).argvIndex = 1
            as ::core::ffi::c_int;
        (*(*pIdxInfo).aConstraintUsage.offset(j as isize)).omit = 1
            as ::core::ffi::c_uchar;
        (*pIdxInfo).estimatedCost = 20 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*pIdxInfo).estimatedRows = 20 as sqlite3_int64;
        if seen[1 as ::core::ffi::c_int as usize] != 0 {
            j = seen[1 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int;
            (*(*pIdxInfo).aConstraintUsage.offset(j as isize)).argvIndex = 2
                as ::core::ffi::c_int;
            (*(*pIdxInfo).aConstraintUsage.offset(j as isize)).omit = 1
                as ::core::ffi::c_uchar;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pragmaVtabOpen(
    mut pVtab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut PragmaVtabCursor = ::core::ptr::null_mut::<
            PragmaVtabCursor,
        >();
        pCsr = sqlite3_malloc(
            ::core::mem::size_of::<PragmaVtabCursor>() as ::core::ffi::c_int,
        ) as *mut PragmaVtabCursor;
        if pCsr.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pCsr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<PragmaVtabCursor>() as size_t,
        );
        (*pCsr).base.pVtab = pVtab;
        *ppCursor = &raw mut (*pCsr).base;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pragmaVtabCursorClear(mut pCsr: *mut PragmaVtabCursor) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        sqlite3_finalize((*pCsr).pPragma);
        (*pCsr).pPragma = ::core::ptr::null_mut::<sqlite3_stmt>();
        (*pCsr).iRowid = 0 as sqlite_int64;
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 2]>() as usize)
                .wrapping_div(
                    ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                ) as ::core::ffi::c_int
        {
            sqlite3_free((*pCsr).azArg[i as usize] as *mut ::core::ffi::c_void);
            (*pCsr).azArg[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
            i += 1;
        }
    }
}
unsafe extern "C" fn pragmaVtabClose(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut PragmaVtabCursor = cur as *mut PragmaVtabCursor;
        pragmaVtabCursorClear(pCsr);
        sqlite3_free(pCsr as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pragmaVtabNext(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        (*pCsr).iRowid += 1;
        if SQLITE_ROW != sqlite3_step((*pCsr).pPragma) {
            rc = sqlite3_finalize((*pCsr).pPragma);
            (*pCsr).pPragma = ::core::ptr::null_mut::<sqlite3_stmt>();
            pragmaVtabCursorClear(pCsr);
        }
        return rc;
    }
}
unsafe extern "C" fn pragmaVtabFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
        let mut pTab: *mut PragmaVtab = (*pVtabCursor).pVtab as *mut PragmaVtab;
        let mut rc: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut acc: StrAccum = StrAccum {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        pragmaVtabCursorClear(pCsr);
        j = if (*(*pTab).pName).mPragFlg as ::core::ffi::c_int & PragFlg_Result1
            != 0 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            let mut zText: *const ::core::ffi::c_char = sqlite3_value_text(
                *argv.offset(i as isize),
            ) as *const ::core::ffi::c_char;
            if !zText.is_null() {
                (*pCsr).azArg[j as usize] = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    zText,
                );
                if (*pCsr).azArg[j as usize].is_null() {
                    return SQLITE_NOMEM;
                }
            }
            i += 1;
            j += 1;
        }
        sqlite3StrAccumInit(
            &raw mut acc,
            ::core::ptr::null_mut::<sqlite3>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            (*(*pTab).db).aLimit[SQLITE_LIMIT_SQL_LENGTH as usize],
        );
        sqlite3_str_appendall(
            &raw mut acc,
            b"PRAGMA \0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !(*pCsr).azArg[1 as ::core::ffi::c_int as usize].is_null() {
            sqlite3_str_appendf(
                &raw mut acc,
                b"%Q.\0".as_ptr() as *const ::core::ffi::c_char,
                (*pCsr).azArg[1 as ::core::ffi::c_int as usize],
            );
        }
        sqlite3_str_appendall(&raw mut acc, (*(*pTab).pName).zName);
        if !(*pCsr).azArg[0 as ::core::ffi::c_int as usize].is_null() {
            sqlite3_str_appendf(
                &raw mut acc,
                b"=%Q\0".as_ptr() as *const ::core::ffi::c_char,
                (*pCsr).azArg[0 as ::core::ffi::c_int as usize],
            );
        }
        zSql = sqlite3StrAccumFinish(&raw mut acc);
        if zSql.is_null() {
            return SQLITE_NOMEM;
        }
        rc = sqlite3_prepare_v2(
            (*pTab).db,
            zSql,
            -1 as ::core::ffi::c_int,
            &raw mut (*pCsr).pPragma,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        if rc != SQLITE_OK {
            (*pTab).base.zErrMsg = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg((*pTab).db),
            );
            return rc;
        }
        return pragmaVtabNext(pVtabCursor);
    }
}
unsafe extern "C" fn pragmaVtabEof(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
        return (*pCsr).pPragma.is_null() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn pragmaVtabColumn(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
        let mut pTab: *mut PragmaVtab = (*pVtabCursor).pVtab as *mut PragmaVtab;
        if i < (*pTab).iHidden as ::core::ffi::c_int {
            sqlite3_result_value(ctx, sqlite3_column_value((*pCsr).pPragma, i));
        } else {
            sqlite3_result_text(
                ctx,
                (*pCsr).azArg[(i - (*pTab).iHidden as ::core::ffi::c_int) as usize],
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn pragmaVtabRowid(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut p: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
        *p = (*pCsr).iRowid;
        return SQLITE_OK;
    }
}
static mut pragmaVtabModule: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: None,
        xConnect: Some(
            pragmaVtabConnect
                as unsafe extern "C" fn(
                    *mut sqlite3,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const *const ::core::ffi::c_char,
                    *mut *mut sqlite3_vtab,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xBestIndex: Some(
            pragmaVtabBestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            pragmaVtabDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: None,
        xOpen: Some(
            pragmaVtabOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            pragmaVtabClose
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            pragmaVtabFilter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            pragmaVtabNext
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            pragmaVtabEof
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            pragmaVtabColumn
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            pragmaVtabRowid
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xUpdate: None,
        xBegin: None,
        xSync: None,
        xCommit: None,
        xRollback: None,
        xFindFunction: None,
        xRename: None,
        xSavepoint: None,
        xRelease: None,
        xRollbackTo: None,
        xShadowName: None,
        xIntegrity: None

    }
};
#[no_mangle]
pub unsafe extern "C" fn sqlite3PragmaVtabRegister(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
) -> *mut Module {
    unsafe {
        let mut pName: *const PragmaName = ::core::ptr::null::<PragmaName>();
        pName = pragmaLocate(zName.offset(7 as ::core::ffi::c_int as isize));
        if pName.is_null() {
            return ::core::ptr::null_mut::<Module>();
        }
        if (*pName).mPragFlg as ::core::ffi::c_int & (PragFlg_Result0 | PragFlg_Result1)
            == 0 as ::core::ffi::c_int
        {
            return ::core::ptr::null_mut::<Module>();
        }
        return sqlite3VtabCreateModule(
            db,
            zName,
            &raw const pragmaVtabModule,
            pName as *mut ::core::ffi::c_void,
            None,
        );
    }
}
pub const PragTyp_ANALYSIS_LIMIT: ::core::ffi::c_int = 1;
pub const PragTyp_HEADER_VALUE: ::core::ffi::c_int = 2;
pub const PragTyp_AUTO_VACUUM: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PragTyp_FLAG: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PragTyp_BUSY_TIMEOUT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PragTyp_CACHE_SIZE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PragTyp_CACHE_SPILL: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const PragTyp_CASE_SENSITIVE_LIKE: ::core::ffi::c_int = 8;
pub const PragTyp_COLLATION_LIST: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const PragTyp_COMPILE_OPTIONS: ::core::ffi::c_int = 10;
pub const PragTyp_DATABASE_LIST: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const PragTyp_ENCODING: ::core::ffi::c_int = 14;
pub const PragTyp_FOREIGN_KEY_CHECK: ::core::ffi::c_int = 15;
pub const PragTyp_FOREIGN_KEY_LIST: ::core::ffi::c_int = 16;
pub const PragTyp_FUNCTION_LIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const PragTyp_HARD_HEAP_LIMIT: ::core::ffi::c_int = 18;
pub const PragTyp_INCREMENTAL_VACUUM: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const PragTyp_INDEX_INFO: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const PragTyp_INDEX_LIST: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const PragTyp_INTEGRITY_CHECK: ::core::ffi::c_int = 22;
pub const PragTyp_JOURNAL_MODE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const PragTyp_JOURNAL_SIZE_LIMIT: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const PragTyp_LOCKING_MODE: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const PragTyp_PAGE_COUNT: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const PragTyp_MMAP_SIZE: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const PragTyp_MODULE_LIST: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const PragTyp_OPTIMIZE: ::core::ffi::c_int = 30;
pub const PragTyp_PAGE_SIZE: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const PragTyp_PRAGMA_LIST: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PragTyp_SECURE_DELETE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const PragTyp_SHRINK_MEMORY: ::core::ffi::c_int = 34;
pub const PragTyp_SOFT_HEAP_LIMIT: ::core::ffi::c_int = 35;
pub const PragTyp_SYNCHRONOUS: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const PragTyp_TABLE_INFO: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const PragTyp_TABLE_LIST: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const PragTyp_TEMP_STORE: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const PragTyp_TEMP_STORE_DIRECTORY: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const PragTyp_THREADS: ::core::ffi::c_int = 41;
pub const PragTyp_WAL_AUTOCHECKPOINT: ::core::ffi::c_int = 42;
pub const PragTyp_WAL_CHECKPOINT: ::core::ffi::c_int = 43;
pub const PragFlg_NeedSchema: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PragFlg_NoColumns: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PragFlg_NoColumns1: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const PragFlg_ReadOnly: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const PragFlg_Result0: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const PragFlg_Result1: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const PragFlg_SchemaOpt: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const PragFlg_SchemaReq: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
static mut pragCName: [*const ::core::ffi::c_char; 57] = [
    b"id\0".as_ptr() as *const ::core::ffi::c_char,
    b"seq\0".as_ptr() as *const ::core::ffi::c_char,
    b"table\0".as_ptr() as *const ::core::ffi::c_char,
    b"from\0".as_ptr() as *const ::core::ffi::c_char,
    b"to\0".as_ptr() as *const ::core::ffi::c_char,
    b"on_update\0".as_ptr() as *const ::core::ffi::c_char,
    b"on_delete\0".as_ptr() as *const ::core::ffi::c_char,
    b"match\0".as_ptr() as *const ::core::ffi::c_char,
    b"cid\0".as_ptr() as *const ::core::ffi::c_char,
    b"name\0".as_ptr() as *const ::core::ffi::c_char,
    b"type\0".as_ptr() as *const ::core::ffi::c_char,
    b"notnull\0".as_ptr() as *const ::core::ffi::c_char,
    b"dflt_value\0".as_ptr() as *const ::core::ffi::c_char,
    b"pk\0".as_ptr() as *const ::core::ffi::c_char,
    b"hidden\0".as_ptr() as *const ::core::ffi::c_char,
    b"name\0".as_ptr() as *const ::core::ffi::c_char,
    b"builtin\0".as_ptr() as *const ::core::ffi::c_char,
    b"type\0".as_ptr() as *const ::core::ffi::c_char,
    b"enc\0".as_ptr() as *const ::core::ffi::c_char,
    b"narg\0".as_ptr() as *const ::core::ffi::c_char,
    b"flags\0".as_ptr() as *const ::core::ffi::c_char,
    b"schema\0".as_ptr() as *const ::core::ffi::c_char,
    b"name\0".as_ptr() as *const ::core::ffi::c_char,
    b"type\0".as_ptr() as *const ::core::ffi::c_char,
    b"ncol\0".as_ptr() as *const ::core::ffi::c_char,
    b"wr\0".as_ptr() as *const ::core::ffi::c_char,
    b"strict\0".as_ptr() as *const ::core::ffi::c_char,
    b"seqno\0".as_ptr() as *const ::core::ffi::c_char,
    b"cid\0".as_ptr() as *const ::core::ffi::c_char,
    b"name\0".as_ptr() as *const ::core::ffi::c_char,
    b"desc\0".as_ptr() as *const ::core::ffi::c_char,
    b"coll\0".as_ptr() as *const ::core::ffi::c_char,
    b"key\0".as_ptr() as *const ::core::ffi::c_char,
    b"seq\0".as_ptr() as *const ::core::ffi::c_char,
    b"name\0".as_ptr() as *const ::core::ffi::c_char,
    b"unique\0".as_ptr() as *const ::core::ffi::c_char,
    b"origin\0".as_ptr() as *const ::core::ffi::c_char,
    b"partial\0".as_ptr() as *const ::core::ffi::c_char,
    b"tbl\0".as_ptr() as *const ::core::ffi::c_char,
    b"idx\0".as_ptr() as *const ::core::ffi::c_char,
    b"wdth\0".as_ptr() as *const ::core::ffi::c_char,
    b"hght\0".as_ptr() as *const ::core::ffi::c_char,
    b"flgs\0".as_ptr() as *const ::core::ffi::c_char,
    b"table\0".as_ptr() as *const ::core::ffi::c_char,
    b"rowid\0".as_ptr() as *const ::core::ffi::c_char,
    b"parent\0".as_ptr() as *const ::core::ffi::c_char,
    b"fkid\0".as_ptr() as *const ::core::ffi::c_char,
    b"busy\0".as_ptr() as *const ::core::ffi::c_char,
    b"log\0".as_ptr() as *const ::core::ffi::c_char,
    b"checkpointed\0".as_ptr() as *const ::core::ffi::c_char,
    b"seq\0".as_ptr() as *const ::core::ffi::c_char,
    b"name\0".as_ptr() as *const ::core::ffi::c_char,
    b"file\0".as_ptr() as *const ::core::ffi::c_char,
    b"database\0".as_ptr() as *const ::core::ffi::c_char,
    b"status\0".as_ptr() as *const ::core::ffi::c_char,
    b"cache_size\0".as_ptr() as *const ::core::ffi::c_char,
    b"timeout\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut aPragmaName: [PragmaName; 65] = [
    PragmaName {
        zName: b"analysis_limit\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_ANALYSIS_LIMIT as u8_0,
        mPragFlg: PragFlg_Result0 as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"application_id\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_HEADER_VALUE as u8_0,
        mPragFlg: (PragFlg_NoColumns1 | PragFlg_Result0) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: BTREE_APPLICATION_ID as u64_0

    },
    PragmaName {
        zName: b"auto_vacuum\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_AUTO_VACUUM as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result0 | PragFlg_SchemaReq
            | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"automatic_index\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_AutoIndex as u64_0

    },
    PragmaName {
        zName: b"busy_timeout\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_BUSY_TIMEOUT as u8_0,
        mPragFlg: PragFlg_Result0 as u8_0,
        iPragCName: 56 as u8_0,
        nPragCName: 1 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"cache_size\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_CACHE_SIZE as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result0 | PragFlg_SchemaReq
            | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"cache_spill\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_CACHE_SPILL as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_SchemaReq | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"case_sensitive_like\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_CASE_SENSITIVE_LIKE as u8_0,
        mPragFlg: PragFlg_NoColumns as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"cell_size_check\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_CellSizeCk as u64_0

    },
    PragmaName {
        zName: b"checkpoint_fullfsync\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_CkptFullFSync as u64_0

    },
    PragmaName {
        zName: b"collation_list\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_COLLATION_LIST as u8_0,
        mPragFlg: PragFlg_Result0 as u8_0,
        iPragCName: 33 as u8_0,
        nPragCName: 2 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"compile_options\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_COMPILE_OPTIONS as u8_0,
        mPragFlg: PragFlg_Result0 as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"count_changes\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_CountRows

    },
    PragmaName {
        zName: b"data_version\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_HEADER_VALUE as u8_0,
        mPragFlg: (PragFlg_ReadOnly | PragFlg_Result0) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: BTREE_DATA_VERSION as u64_0

    },
    PragmaName {
        zName: b"database_list\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_DATABASE_LIST as u8_0,
        mPragFlg: PragFlg_Result0 as u8_0,
        iPragCName: 50 as u8_0,
        nPragCName: 3 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"defer_foreign_keys\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_DeferFKs as u64_0

    },
    PragmaName {
        zName: b"empty_result_callbacks\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_NullCallback as u64_0

    },
    PragmaName {
        zName: b"encoding\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_ENCODING as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"foreign_key_check\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FOREIGN_KEY_CHECK as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result0 | PragFlg_Result1
            | PragFlg_SchemaOpt) as u8_0,
        iPragCName: 43 as u8_0,
        nPragCName: 4 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"foreign_key_list\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FOREIGN_KEY_LIST as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result1 | PragFlg_SchemaOpt) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 8 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"foreign_keys\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_ForeignKeys as u64_0

    },
    PragmaName {
        zName: b"freelist_count\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_HEADER_VALUE as u8_0,
        mPragFlg: (PragFlg_ReadOnly | PragFlg_Result0) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: BTREE_FREE_PAGE_COUNT as u64_0

    },
    PragmaName {
        zName: b"full_column_names\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_FullColNames as u64_0

    },
    PragmaName {
        zName: b"fullfsync\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_FullFSync as u64_0

    },
    PragmaName {
        zName: b"function_list\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FUNCTION_LIST as u8_0,
        mPragFlg: PragFlg_Result0 as u8_0,
        iPragCName: 15 as u8_0,
        nPragCName: 6 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"hard_heap_limit\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_HARD_HEAP_LIMIT as u8_0,
        mPragFlg: PragFlg_Result0 as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"ignore_check_constraints\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_IgnoreChecks as u64_0

    },
    PragmaName {
        zName: b"incremental_vacuum\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_INCREMENTAL_VACUUM as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_NoColumns) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"index_info\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_INDEX_INFO as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result1 | PragFlg_SchemaOpt) as u8_0,
        iPragCName: 27 as u8_0,
        nPragCName: 3 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"index_list\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_INDEX_LIST as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result1 | PragFlg_SchemaOpt) as u8_0,
        iPragCName: 33 as u8_0,
        nPragCName: 5 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"index_xinfo\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_INDEX_INFO as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result1 | PragFlg_SchemaOpt) as u8_0,
        iPragCName: 27 as u8_0,
        nPragCName: 6 as u8_0,
        iArg: 1 as u64_0

    },
    PragmaName {
        zName: b"integrity_check\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_INTEGRITY_CHECK as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result0 | PragFlg_Result1
            | PragFlg_SchemaOpt) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"journal_mode\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_JOURNAL_MODE as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result0 | PragFlg_SchemaReq) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"journal_size_limit\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_JOURNAL_SIZE_LIMIT as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_SchemaReq) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"legacy_alter_table\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_LegacyAlter as u64_0

    },
    PragmaName {
        zName: b"locking_mode\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_LOCKING_MODE as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_SchemaReq) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"max_page_count\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_PAGE_COUNT as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result0 | PragFlg_SchemaReq) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"mmap_size\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_MMAP_SIZE as u8_0,
        mPragFlg: 0 as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"module_list\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_MODULE_LIST as u8_0,
        mPragFlg: PragFlg_Result0 as u8_0,
        iPragCName: 9 as u8_0,
        nPragCName: 1 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"optimize\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_OPTIMIZE as u8_0,
        mPragFlg: (PragFlg_Result1 | PragFlg_NeedSchema) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"page_count\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_PAGE_COUNT as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result0 | PragFlg_SchemaReq) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"page_size\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_PAGE_SIZE as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_SchemaReq | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"pragma_list\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_PRAGMA_LIST as u8_0,
        mPragFlg: PragFlg_Result0 as u8_0,
        iPragCName: 9 as u8_0,
        nPragCName: 1 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"query_only\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_QueryOnly as u64_0

    },
    PragmaName {
        zName: b"quick_check\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_INTEGRITY_CHECK as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result0 | PragFlg_Result1
            | PragFlg_SchemaOpt) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"read_uncommitted\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_ReadUncommit

    },
    PragmaName {
        zName: b"recursive_triggers\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_RecTriggers as u64_0

    },
    PragmaName {
        zName: b"reverse_unordered_selects\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_ReverseOrder as u64_0

    },
    PragmaName {
        zName: b"schema_version\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_HEADER_VALUE as u8_0,
        mPragFlg: (PragFlg_NoColumns1 | PragFlg_Result0) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: BTREE_SCHEMA_VERSION as u64_0

    },
    PragmaName {
        zName: b"secure_delete\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_SECURE_DELETE as u8_0,
        mPragFlg: PragFlg_Result0 as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"short_column_names\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_ShortColNames as u64_0

    },
    PragmaName {
        zName: b"shrink_memory\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_SHRINK_MEMORY as u8_0,
        mPragFlg: PragFlg_NoColumns as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"soft_heap_limit\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_SOFT_HEAP_LIMIT as u8_0,
        mPragFlg: PragFlg_Result0 as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"synchronous\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_SYNCHRONOUS as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result0 | PragFlg_SchemaReq
            | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"table_info\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_TABLE_INFO as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result1 | PragFlg_SchemaOpt) as u8_0,
        iPragCName: 8 as u8_0,
        nPragCName: 6 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"table_list\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_TABLE_LIST as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result1) as u8_0,
        iPragCName: 21 as u8_0,
        nPragCName: 6 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"table_xinfo\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_TABLE_INFO as u8_0,
        mPragFlg: (PragFlg_NeedSchema | PragFlg_Result1 | PragFlg_SchemaOpt) as u8_0,
        iPragCName: 8 as u8_0,
        nPragCName: 7 as u8_0,
        iArg: 1 as u64_0

    },
    PragmaName {
        zName: b"temp_store\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_TEMP_STORE as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"temp_store_directory\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_TEMP_STORE_DIRECTORY as u8_0,
        mPragFlg: PragFlg_NoColumns1 as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"threads\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_THREADS as u8_0,
        mPragFlg: PragFlg_Result0 as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"trusted_schema\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: SQLITE_TrustedSchema as u64_0

    },
    PragmaName {
        zName: b"user_version\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_HEADER_VALUE as u8_0,
        mPragFlg: (PragFlg_NoColumns1 | PragFlg_Result0) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: BTREE_USER_VERSION as u64_0

    },
    PragmaName {
        zName: b"wal_autocheckpoint\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_WAL_AUTOCHECKPOINT as u8_0,
        mPragFlg: 0 as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"wal_checkpoint\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_WAL_CHECKPOINT as u8_0,
        mPragFlg: PragFlg_NeedSchema as u8_0,
        iPragCName: 47 as u8_0,
        nPragCName: 3 as u8_0,
        iArg: 0 as u64_0

    },
    PragmaName {
        zName: b"writable_schema\0".as_ptr() as *const ::core::ffi::c_char,
        ePragTyp: PragTyp_FLAG as u8_0,
        mPragFlg: (PragFlg_Result0 | PragFlg_NoColumns1) as u8_0,
        iPragCName: 0 as u8_0,
        nPragCName: 0 as u8_0,
        iArg: (SQLITE_WriteSchema | SQLITE_NoSchemaError) as u64_0

    },
];
pub const SQLITE_FUNC_HASH_SZ: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const SQLITE_MAX_DB: ::core::ffi::c_int = SQLITE_MAX_ATTACHED
    + 2 as ::core::ffi::c_int;
pub const SQLITE_WriteSchema: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_FullColNames: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_FullFSync: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_CkptFullFSync: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_CacheSpill: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_ShortColNames: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_TrustedSchema: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_NullCallback: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_IgnoreChecks: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SQLITE_ReverseOrder: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const SQLITE_RecTriggers: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SQLITE_ForeignKeys: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_AutoIndex: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_DeferFKs: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_QueryOnly: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const SQLITE_CellSizeCk: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SQLITE_LegacyAlter: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const SQLITE_NoSchemaError: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const SQLITE_Defensive: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SQLITE_CountRows: u64_0 = (0x1 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const SQLITE_ReadUncommit: u64_0 = (0x4 as ::core::ffi::c_int as u64_0)
    << 32 as ::core::ffi::c_int;
pub const DBFLAG_InternalFunc: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const DBFLAG_EncodingFixed: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_FUNC_ENCMASK: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
pub const SQLITE_FUNC_INTERNAL: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const COLTYPE_ANY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const COLFLAG_PRIMKEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const COLFLAG_VIRTUAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const COLFLAG_STORED: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const COLFLAG_NOINSERT: ::core::ffi::c_int = 0x62 as ::core::ffi::c_int;
pub const SQLITE_AFF_BLOB: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const SQLITE_AFF_TEXT: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_MaybeReanalyze: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const TF_Shadow: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const TF_Strict: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const TF_Imposter: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const TABTYP_NORM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TABTYP_VIEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_None: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Restrict: ::core::ffi::c_int = 7;
pub const OE_SetNull: ::core::ffi::c_int = 8;
pub const OE_SetDflt: ::core::ffi::c_int = 9;
pub const OE_Cascade: ::core::ffi::c_int = 10;
pub const SQLITE_IDXTYPE_PRIMARYKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const LOCATE_NOERR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_MAX_ATTACHED: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
