use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type PreUpdate;
    pub type RenameToken;
    pub type Vdbe;
    pub type sqlite3_mutex;
    pub type sqlite3_pcache;
    pub type CheckOnCtx;
    pub type CoveringIndexCheck;
    pub type RenameCtx;
    pub type WhereConst;
    pub type WindowRewrite;
    pub type IdxCover;
    pub type RefSrcList;
    pub type CCurHint;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
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
    fn sqlite3HashInsert(
        _: *mut Hash,
        pKey: *const ::core::ffi::c_char,
        pData: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3HashFind(
        _: *const Hash,
        pKey: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3BtreeOpen(
        pVfs: *mut sqlite3_vfs,
        zFilename: *const ::core::ffi::c_char,
        db: *mut sqlite3,
        ppBtree: *mut *mut Btree,
        flags: ::core::ffi::c_int,
        vfsFlags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeSetPageSize(
        p: *mut Btree,
        nPagesize: ::core::ffi::c_int,
        nReserve: ::core::ffi::c_int,
        eFix: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeIsReadonly(pBt: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeEnterAll(_: *mut sqlite3);
    fn sqlite3BtreeSharable(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeLeaveAll(_: *mut sqlite3);
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
    fn sqlite3VdbeAddOp4Int(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3VdbeEndCoroutine(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeAddParseSchemaOp(
        _: *mut Vdbe,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: u16_0,
    );
    fn sqlite3VdbeChangeOpcode(_: *mut Vdbe, addr: ::core::ffi::c_int, _: u8_0);
    fn sqlite3VdbeChangeP3(
        _: *mut Vdbe,
        addr: ::core::ffi::c_int,
        P3: ::core::ffi::c_int,
    );
    fn sqlite3VdbeChangeP5(_: *mut Vdbe, P5: u16_0);
    fn sqlite3VdbeJumpHere(_: *mut Vdbe, addr: ::core::ffi::c_int);
    fn sqlite3VdbeUsesBtree(_: *mut Vdbe, _: ::core::ffi::c_int);
    fn sqlite3VdbeMakeReady(_: *mut Vdbe, _: *mut Parse);
    fn sqlite3VdbeCurrentAddr(_: *mut Vdbe) -> ::core::ffi::c_int;
    fn sqlite3CorruptError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3ColumnType(
        _: *mut Column,
        _: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRaw(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbStrNDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: u64_0,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbSpanDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3DbReallocOrFree(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: u64_0,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3DbRealloc(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: u64_0,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3DbNNFreeNN(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3MPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3VMPrintf(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3Dequote(_: *mut ::core::ffi::c_char);
    fn sqlite3DequoteToken(_: *mut Token);
    fn sqlite3TokenInit(_: *mut Token, _: *mut ::core::ffi::c_char);
    fn sqlite3KeywordCode(
        _: *const ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3RunParser(
        _: *mut Parse,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3GetTempReg(_: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3ReleaseTempReg(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3ExprAlloc(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const Token,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3PExpr(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *mut Expr,
        _: *mut Expr,
    ) -> *mut Expr;
    fn sqlite3ExprDelete(_: *mut sqlite3, _: *mut Expr);
    fn sqlite3ExprListAppend(
        _: *mut Parse,
        _: *mut ExprList,
        _: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListSetSortOrder(
        _: *mut ExprList,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprListSetName(
        _: *mut Parse,
        _: *mut ExprList,
        _: *const Token,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprListDelete(_: *mut sqlite3, _: *mut ExprList);
    fn sqlite3IndexHasDuplicateRootPage(_: *mut Index) -> ::core::ffi::c_int;
    fn sqlite3PragmaVtabRegister(
        _: *mut sqlite3,
        zName: *const ::core::ffi::c_char,
    ) -> *mut Module;
    fn sqlite3ColumnsFromExprList(
        _: *mut Parse,
        _: *mut ExprList,
        _: *mut i16_0,
        _: *mut *mut Column,
    ) -> ::core::ffi::c_int;
    fn sqlite3SubqueryColumnTypes(
        _: *mut Parse,
        _: *mut Table,
        _: *mut Select,
        _: ::core::ffi::c_char,
    );
    fn sqlite3ResultSetOfSelect(
        _: *mut Parse,
        _: *mut Select,
        _: ::core::ffi::c_char,
    ) -> *mut Table;
    fn sqlite3AutoincrementBegin(pParse: *mut Parse);
    fn sqlite3ClearOnOrUsing(_: *mut sqlite3, _: *mut OnOrUsing);
    fn sqlite3Select(
        _: *mut Parse,
        _: *mut Select,
        _: *mut SelectDest,
    ) -> ::core::ffi::c_int;
    fn sqlite3SelectDelete(_: *mut sqlite3, _: *mut Select);
    fn sqlite3SrcListLookup(_: *mut Parse, _: *mut SrcList) -> *mut Table;
    fn sqlite3OpenTable(
        _: *mut Parse,
        iCur: ::core::ffi::c_int,
        iDb: ::core::ffi::c_int,
        _: *mut Table,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprCode(_: *mut Parse, _: *mut Expr, _: ::core::ffi::c_int);
    fn sqlite3GetVdbe(_: *mut Parse) -> *mut Vdbe;
    fn sqlite3ExprIsConstantOrFunction(_: *mut Expr, _: u8_0) -> ::core::ffi::c_int;
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
    fn sqlite3ExprDup(
        _: *mut sqlite3,
        _: *const Expr,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3ExprListDup(
        _: *mut sqlite3,
        _: *const ExprList,
        _: ::core::ffi::c_int,
    ) -> *mut ExprList;
    fn sqlite3SelectDup(
        _: *mut sqlite3,
        _: *const Select,
        _: ::core::ffi::c_int,
    ) -> *mut Select;
    fn sqlite3JsonVtabRegister(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
    ) -> *mut Module;
    fn sqlite3CarrayRegister(_: *mut sqlite3) -> *mut Module;
    fn sqlite3DropTriggerPtr(_: *mut Parse, _: *mut Trigger);
    fn sqlite3TriggerList(_: *mut Parse, _: *mut Table) -> *mut Trigger;
    fn sqlite3ColumnIndex(
        pTab: *mut Table,
        zCol: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3AuthCheck(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3DbIsNamed(
        db: *mut sqlite3,
        iDb: ::core::ffi::c_int,
        zName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3FixInit(
        _: *mut DbFixer,
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const Token,
    );
    fn sqlite3FixSrcList(_: *mut DbFixer, _: *mut SrcList) -> ::core::ffi::c_int;
    fn sqlite3FixSelect(_: *mut DbFixer, _: *mut Select) -> ::core::ffi::c_int;
    fn sqlite3GetInt32(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3LogEst(_: u64_0) -> LogEst;
    fn sqlite3TableAffinity(_: *mut Vdbe, _: *mut Table, _: ::core::ffi::c_int);
    fn sqlite3ReadSchema(pParse: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3FindCollSeq(
        _: *mut sqlite3,
        enc: u8_0,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut CollSeq;
    fn sqlite3LocateCollSeq(
        pParse: *mut Parse,
        zName: *const ::core::ffi::c_char,
    ) -> *mut CollSeq;
    fn sqlite3ExprSkipCollate(_: *mut Expr) -> *mut Expr;
    static sqlite3StrBINARY: [::core::ffi::c_char; 0];
    static sqlite3StdTypeLen: [::core::ffi::c_uchar; 0];
    static sqlite3StdTypeAffinity: [::core::ffi::c_char; 0];
    static mut sqlite3StdType: [*const ::core::ffi::c_char; 0];
    static sqlite3UpperToLower: [::core::ffi::c_uchar; 0];
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3StrIHash(_: *const ::core::ffi::c_char) -> u8_0;
    fn sqlite3ResolveSelfReference(
        _: *mut Parse,
        _: *mut Table,
        _: ::core::ffi::c_int,
        _: *mut Expr,
        _: *mut ExprList,
    ) -> ::core::ffi::c_int;
    fn sqlite3RenameTokenMap(
        _: *mut Parse,
        _: *const ::core::ffi::c_void,
        _: *const Token,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3RenameTokenRemap(
        _: *mut Parse,
        pTo: *const ::core::ffi::c_void,
        pFrom: *const ::core::ffi::c_void,
    );
    fn sqlite3RenameExprUnmap(_: *mut Parse, _: *mut Expr);
    fn sqlite3RenameExprlistUnmap(_: *mut Parse, _: *mut ExprList);
    fn sqlite3DeleteIndexSamples(_: *mut sqlite3, _: *mut Index);
    fn sqlite3SchemaClear(_: *mut ::core::ffi::c_void);
    fn sqlite3SchemaToIndex(db: *mut sqlite3, _: *mut Schema) -> ::core::ffi::c_int;
    fn sqlite3KeyInfoAlloc(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut KeyInfo;
    fn sqlite3KeyInfoUnref(_: *mut KeyInfo);
    fn sqlite3KeyInfoRef(_: *mut KeyInfo) -> *mut KeyInfo;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3StrAccumInit(
        _: *mut StrAccum,
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3StrAccumFinish(_: *mut StrAccum) -> *mut ::core::ffi::c_char;
    fn sqlite3SelectDestInit(
        _: *mut SelectDest,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3VtabClear(db: *mut sqlite3, _: *mut Table);
    fn sqlite3VtabUnlockList(_: *mut sqlite3);
    fn sqlite3GetVTable(_: *mut sqlite3, _: *mut Table) -> *mut VTable;
    fn sqlite3VtabEponymousTableInit(
        _: *mut Parse,
        _: *mut Module,
    ) -> ::core::ffi::c_int;
    fn sqlite3VtabCallConnect(_: *mut Parse, _: *mut Table) -> ::core::ffi::c_int;
    fn sqlite3ParserAddCleanup(
        _: *mut Parse,
        _: Option<unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> ()>,
        _: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3ExprListCheckLength(
        _: *mut Parse,
        _: *mut ExprList,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3FkDropTable(_: *mut Parse, _: *mut SrcList, _: *mut Table);
    fn sqlite3FkDelete(_: *mut sqlite3, _: *mut Table);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void

}
pub type va_list = __builtin_va_list;
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
    pub xProfile: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            u64_0,
        ) -> (),
    >,
    pub pProfileArg: *mut ::core::ffi::c_void,
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
    pub pPreUpdateArg: *mut ::core::ffi::c_void,
    pub xPreUpdateCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            sqlite3_int64,
            sqlite3_int64,
        ) -> (),
    >,
    pub pPreUpdate: *mut PreUpdate,
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
pub type u16_0 = uint16_t;
pub type uint16_t = __uint16_t;
pub type __uint16_t = u16;
pub type u8_0 = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
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
pub type u32_0 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
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
pub type i16_0 = int16_t;
pub type int16_t = __int16_t;
pub type __int16_t = i16;
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
pub type LogEst = int16_t;
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
        ) -> ::core::ffi::c_int
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
    pub nTableLock: ::core::ffi::c_int,
    pub aTableLock: *mut TableLock,
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
        unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> ()
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableLock {
    pub iDb: ::core::ffi::c_int,
    pub iTab: Pgno,
    pub isWriteLock: u8_0,
    pub zLockName: *const ::core::ffi::c_char

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
        ) -> ::core::ffi::c_int
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
        ) -> *const ::core::ffi::c_char
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
        ) -> ::core::ffi::c_int
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
pub type __int8_t = i8;
pub type int8_t = __int8_t;
pub type size_t = usize;
pub type i8_0 = int8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbFixer {
    pub pParse: *mut Parse,
    pub w: Walker,
    pub pSchema: *mut Schema,
    pub bTemp: u8_0,
    pub zDb: *const ::core::ffi::c_char,
    pub zType: *const ::core::ffi::c_char,
    pub pName: *const Token

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Walker {
    pub pParse: *mut Parse,
    pub xExprCallback: Option<
        unsafe extern "C" fn(*mut Walker, *mut Expr) -> ::core::ffi::c_int,
    >,
    pub xSelectCallback: Option<
        unsafe extern "C" fn(*mut Walker, *mut Select) -> ::core::ffi::c_int,
    >,
    pub xSelectCallback2: Option<unsafe extern "C" fn(*mut Walker, *mut Select) -> ()>,
    pub walkerDepth: ::core::ffi::c_int,
    pub eCode: u16_0,
    pub mWFlags: u16_0,
    pub u: C2Rust_Unnamed_22

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_22 {
    pub pNC: *mut NameContext,
    pub n: ::core::ffi::c_int,
    pub iCur: ::core::ffi::c_int,
    pub pSrcList: *mut SrcList,
    pub pCCurHint: *mut CCurHint,
    pub pRefSrcList: *mut RefSrcList,
    pub aiCol: *mut ::core::ffi::c_int,
    pub pIdxCover: *mut IdxCover,
    pub pGroupBy: *mut ExprList,
    pub pSelect: *mut Select,
    pub pRewrite: *mut WindowRewrite,
    pub pConst: *mut WhereConst,
    pub pRename: *mut RenameCtx,
    pub pTab: *mut Table,
    pub pCovIdxCk: *mut CoveringIndexCheck,
    pub pSrcItem: *mut SrcItem,
    pub pFix: *mut DbFixer,
    pub aMem: *mut Mem,
    pub pCheckOnCtx: *mut CheckOnCtx

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameContext {
    pub pParse: *mut Parse,
    pub pSrcList: *mut SrcList,
    pub uNC: C2Rust_Unnamed_23,
    pub pNext: *mut NameContext,
    pub nRef: ::core::ffi::c_int,
    pub nNcErr: ::core::ffi::c_int,
    pub ncFlags: ::core::ffi::c_int,
    pub nNestedSelect: u32_0,
    pub pWinSelect: *mut Select

}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_23 {
    pub pEList: *mut ExprList,
    pub pAggInfo: *mut AggInfo,
    pub pUpsert: *mut Upsert,
    pub iBaseReg: ::core::ffi::c_int

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnOrUsing {
    pub pOn: *mut Expr,
    pub pUsing: *mut IdList

}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SelectDest {
    pub eDest: u8_0,
    pub iSDParm: ::core::ffi::c_int,
    pub iSDParm2: ::core::ffi::c_int,
    pub iSdst: ::core::ffi::c_int,
    pub nSdst: ::core::ffi::c_int,
    pub zAffSdst: *mut ::core::ffi::c_char,
    pub pOrderBy: *mut ExprList

}
pub type StrAccum = sqlite3_str;
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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_ERROR_RETRY: ::core::ffi::c_int = SQLITE_ERROR
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_PRIMARYKEY: ::core::ffi::c_int = SQLITE_CONSTRAINT
    | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_UNIQUE: ::core::ffi::c_int = SQLITE_CONSTRAINT
    | (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_ROWID: ::core::ffi::c_int = SQLITE_CONSTRAINT
    | (10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_DELETEONCLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_EXCLUSIVE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_OPEN_TEMP_DB: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SQLITE_CREATE_INDEX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_CREATE_TABLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_CREATE_TEMP_INDEX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_CREATE_TEMP_TABLE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_CREATE_TEMP_VIEW: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_CREATE_VIEW: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_DELETE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_DROP_INDEX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_DROP_TABLE: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_DROP_TEMP_INDEX: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_DROP_TEMP_TABLE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SQLITE_DROP_TEMP_VIEW: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SQLITE_DROP_VIEW: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SQLITE_INSERT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_TRANSACTION: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const SQLITE_REINDEX: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const SQLITE_DROP_VTABLE: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const SQLITE_SAVEPOINT: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LENGTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_COLUMN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_PREPARE_NO_VTAB: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const OP_Savepoint: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OP_AutoCommit: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OP_Transaction: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OP_JournalMode: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const OP_Goto: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OP_InitCoroutine: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const OP_Yield: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const OP_If: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const OP_SorterSort: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const OP_Rewind: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const OP_SorterNext: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const OP_Next: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const OP_Halt: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
pub const OP_Integer: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const OP_Blob: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
pub const OP_FkCheck: ::core::ffi::c_int = 84 as ::core::ffi::c_int;
pub const OP_ResultRow: ::core::ffi::c_int = 85 as ::core::ffi::c_int;
pub const OP_Column: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const OP_MakeRecord: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const OP_ReadCookie: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const OP_SetCookie: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const OP_OpenRead: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const OP_OpenWrite: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const OP_OpenEphemeral: ::core::ffi::c_int = 119 as ::core::ffi::c_int;
pub const OP_SorterOpen: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
pub const OP_Close: ::core::ffi::c_int = 123 as ::core::ffi::c_int;
pub const OP_NewRowid: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const OP_Insert: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const OP_SorterCompare: ::core::ffi::c_int = 133 as ::core::ffi::c_int;
pub const OP_SorterData: ::core::ffi::c_int = 134 as ::core::ffi::c_int;
pub const OP_SeekEnd: ::core::ffi::c_int = 138 as ::core::ffi::c_int;
pub const OP_IdxInsert: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const OP_SorterInsert: ::core::ffi::c_int = 140 as ::core::ffi::c_int;
pub const OP_Destroy: ::core::ffi::c_int = 145 as ::core::ffi::c_int;
pub const OP_Clear: ::core::ffi::c_int = 146 as ::core::ffi::c_int;
pub const OP_CreateBtree: ::core::ffi::c_int = 148 as ::core::ffi::c_int;
pub const OP_SqlExec: ::core::ffi::c_int = 149 as ::core::ffi::c_int;
pub const OP_DropTable: ::core::ffi::c_int = 152 as ::core::ffi::c_int;
pub const OP_DropIndex: ::core::ffi::c_int = 153 as ::core::ffi::c_int;
pub const OP_Expire: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const OP_TableLock: ::core::ffi::c_int = 170 as ::core::ffi::c_int;
pub const OP_VBegin: ::core::ffi::c_int = 171 as ::core::ffi::c_int;
pub const OP_VDestroy: ::core::ffi::c_int = 173 as ::core::ffi::c_int;
pub const OP_Noop: ::core::ffi::c_int = 188 as ::core::ffi::c_int;
pub const OMIT_TEMPDB: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_MAX_FILE_FORMAT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LEGACY_SCHEMA_TABLE: [::core::ffi::c_char; 14] = unsafe {
    ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"sqlite_master\0")
};
pub const LEGACY_TEMP_SCHEMA_TABLE: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<
        [u8; 19],
        [::core::ffi::c_char; 19],
    >(*b"sqlite_temp_master\0")
};
pub const PREFERRED_SCHEMA_TABLE: [::core::ffi::c_char; 14] = unsafe {
    ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"sqlite_schema\0")
};
pub const PREFERRED_TEMP_SCHEMA_TABLE: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<
        [u8; 19],
        [::core::ffi::c_char; 19],
    >(*b"sqlite_temp_schema\0")
};
pub const SCHEMA_ROOT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BMS: ::core::ffi::c_int = (::core::mem::size_of::<Bitmask>() as usize)
    .wrapping_mul(8 as usize) as ::core::ffi::c_int;
pub const PAGER_JOURNALMODE_QUERY: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const BTREE_INTKEY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTREE_BLOBKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BTREE_SCHEMA_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTREE_FILE_FORMAT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BTREE_TEXT_ENCODING: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const P4_STATIC: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const P4_DYNAMIC: ::core::ffi::c_int = -6 as ::core::ffi::c_int;
pub const P4_KEYINFO: ::core::ffi::c_int = -8 as ::core::ffi::c_int;
pub const P4_VTAB: ::core::ffi::c_int = -11 as ::core::ffi::c_int;
pub const P5_ConstraintUnique: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TK_DEFERRED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const TK_EXCLUSIVE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const TK_ROLLBACK: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const TK_ID: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const TK_RAISE: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const TK_COLLATE: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const TK_STRING: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_RETURNING: ::core::ffi::c_int = 151 as ::core::ffi::c_int;
pub const TK_COLUMN: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
pub const TK_UPLUS: ::core::ffi::c_int = 173 as ::core::ffi::c_int;
pub const TK_SPAN: ::core::ffi::c_int = 181 as ::core::ffi::c_int;
pub const DB_UnresetViews: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_WriteSchema: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_LegacyFileFmt: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_Defensive: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const DBFLAG_SchemaChange: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const DBFLAG_PreferBuiltin: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const DBFLAG_SchemaKnownOk: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const COLTYPE_CUSTOM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const COLTYPE_ANY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const COLTYPE_INTEGER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_N_STDTYPE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const COLFLAG_PRIMKEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const COLFLAG_HASTYPE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const COLFLAG_UNIQUE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const COLFLAG_VIRTUAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const COLFLAG_STORED: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const COLFLAG_HASCOLL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const COLFLAG_GENERATED: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const COLFLAG_NOINSERT: ::core::ffi::c_int = 0x62 as ::core::ffi::c_int;
pub const SQLITE_SO_DESC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_SO_UNDEFINED: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const SQLITE_AFF_NONE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_AFF_BLOB: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const SQLITE_AFF_TEXT: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const SQLITE_AFF_INTEGER: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;
pub const SQLITE_AFF_REAL: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;
pub const TF_Readonly: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const TF_HasPrimaryKey: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const TF_Autoincrement: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const TF_HasVirtual: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const TF_HasGenerated: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_NoVisibleRowid: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const TF_HasNotNull: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const TF_Shadow: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const TF_Eponymous: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const TF_Strict: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const TF_Imposter: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const TABTYP_NORM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TABTYP_VIEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_None: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Replace: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const OE_Default: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_IDXTYPE_APPDEF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_IDXTYPE_PRIMARYKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XN_ROWID: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const XN_EXPR: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const EP_Skip: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const EXPRDUP_REDUCE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const JT_RIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const JT_LTORJ: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const NC_PartIdx: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const NC_IsCheck: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const NC_GenCol: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const NC_IdxExpr: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SF_NestedFrom: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SF_View: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SRT_Coroutine: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const PARSE_MODE_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PARSE_MODE_DECLARE_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PARSE_MODE_RENAME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PARSE_RECURSE_SZ: ::core::ffi::c_ulong = 288 as ::core::ffi::c_ulong;
pub const PARSE_TAIL_SZ: usize = (::core::mem::size_of::<Parse>() as usize)
    .wrapping_sub(PARSE_RECURSE_SZ as usize);
pub const OPFLAG_APPEND: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPFLAG_USESEEKRESULT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const OPFLAG_BULKCSR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPFLAG_P2ISREG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const TRIGGER_AFTER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const LOCATE_VIEW: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const LOCATE_NOERR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
#[inline(never)]
unsafe extern "C" fn lockTable(
    mut pParse: *mut Parse,
    mut iDb: ::core::ffi::c_int,
    mut iTab: Pgno,
    mut isWriteLock: u8_0,
    mut zName: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut pToplevel: *mut Parse = ::core::ptr::null_mut::<Parse>();
        let mut i: ::core::ffi::c_int = 0;
        let mut nBytes: ::core::ffi::c_int = 0;
        let mut p: *mut TableLock = ::core::ptr::null_mut::<TableLock>();
        pToplevel = if !(*pParse).pToplevel.is_null() {
            (*pParse).pToplevel
        } else {
            pParse
        };
        i = 0 as ::core::ffi::c_int;
        while i < (*pToplevel).nTableLock {
            p = (*pToplevel).aTableLock.offset(i as isize) as *mut TableLock;
            if (*p).iDb == iDb && (*p).iTab == iTab {
                (*p).isWriteLock = ((*p).isWriteLock as ::core::ffi::c_int != 0
                    || isWriteLock as ::core::ffi::c_int != 0) as ::core::ffi::c_int
                    as u8_0;
                return;
            }
            i += 1;
        }
        nBytes = (::core::mem::size_of::<TableLock>() as usize)
            .wrapping_mul(((*pToplevel).nTableLock + 1 as ::core::ffi::c_int) as usize)
            as ::core::ffi::c_int;
        (*pToplevel).aTableLock = sqlite3DbReallocOrFree(
            (*pToplevel).db,
            (*pToplevel).aTableLock as *mut ::core::ffi::c_void,
            nBytes as u64_0,
        ) as *mut TableLock;
        if !(*pToplevel).aTableLock.is_null() {
            let c2rust_fresh4 = (*pToplevel).nTableLock;
            (*pToplevel).nTableLock = (*pToplevel).nTableLock + 1;
            p = (*pToplevel).aTableLock.offset(c2rust_fresh4 as isize) as *mut TableLock;
            (*p).iDb = iDb;
            (*p).iTab = iTab;
            (*p).isWriteLock = isWriteLock;
            (*p).zLockName = zName;
        } else {
            (*pToplevel).nTableLock = 0 as ::core::ffi::c_int;
            sqlite3OomFault((*pToplevel).db);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TableLock(
    mut pParse: *mut Parse,
    mut iDb: ::core::ffi::c_int,
    mut iTab: Pgno,
    mut isWriteLock: u8_0,
    mut zName: *const ::core::ffi::c_char,
) {
    unsafe {
        if iDb == 1 as ::core::ffi::c_int {
            return;
        }
        if sqlite3BtreeSharable((*(*(*pParse).db).aDb.offset(iDb as isize)).pBt) == 0 {
            return;
        }
        lockTable(pParse, iDb, iTab, isWriteLock, zName);
    }
}
unsafe extern "C" fn codeTableLocks(mut pParse: *mut Parse) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pVdbe: *mut Vdbe = (*pParse).pVdbe;
        i = 0 as ::core::ffi::c_int;
        while i < (*pParse).nTableLock {
            let mut p: *mut TableLock = (*pParse).aTableLock.offset(i as isize)
                as *mut TableLock;
            let mut p1: ::core::ffi::c_int = (*p).iDb;
            sqlite3VdbeAddOp4(
                pVdbe,
                OP_TableLock,
                p1,
                (*p).iTab as ::core::ffi::c_int,
                (*p).isWriteLock as ::core::ffi::c_int,
                (*p).zLockName,
                P4_STATIC,
            );
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FinishCoding(mut pParse: *mut Parse) {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut iDb: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        db = (*pParse).db;
        if (*pParse).nested != 0 {
            return;
        }
        if (*pParse).nErr != 0 {
            if (*db).mallocFailed != 0 {
                (*pParse).rc = SQLITE_NOMEM;
            }
            return;
        }
        v = (*pParse).pVdbe;
        if v.is_null() {
            if (*db).init.busy != 0 {
                (*pParse).rc = SQLITE_DONE;
                return;
            }
            v = sqlite3GetVdbe(pParse);
            if v.is_null() {
                (*pParse).rc = SQLITE_ERROR;
            }
        }
        if !v.is_null() {
            if (*pParse).bReturning != 0 {
                let mut pReturning: *mut Returning = ::core::ptr::null_mut::<
                    Returning,
                >();
                let mut addrRewind: ::core::ffi::c_int = 0;
                let mut reg: ::core::ffi::c_int = 0;
                pReturning = (*pParse).u1.d.pReturning;
                if (*pReturning).nRetCol != 0 {
                    sqlite3VdbeAddOp0(v, OP_FkCheck);
                    addrRewind = sqlite3VdbeAddOp1(v, OP_Rewind, (*pReturning).iRetCur);
                    reg = (*pReturning).iRetReg;
                    i = 0 as ::core::ffi::c_int;
                    while i < (*pReturning).nRetCol {
                        sqlite3VdbeAddOp3(
                            v,
                            OP_Column,
                            (*pReturning).iRetCur,
                            i,
                            reg + i,
                        );
                        i += 1;
                    }
                    sqlite3VdbeAddOp2(v, OP_ResultRow, reg, i);
                    sqlite3VdbeAddOp2(
                        v,
                        OP_Next,
                        (*pReturning).iRetCur,
                        addrRewind + 1 as ::core::ffi::c_int,
                    );
                    sqlite3VdbeJumpHere(v, addrRewind);
                }
            }
            sqlite3VdbeAddOp0(v, OP_Halt);
            sqlite3VdbeJumpHere(v, 0 as ::core::ffi::c_int);
            iDb = 0 as ::core::ffi::c_int;
            loop {
                let mut pSchema: *mut Schema = ::core::ptr::null_mut::<Schema>();
                if !(((*pParse).cookieMask & (1 as ::core::ffi::c_int as yDbMask) << iDb
                    != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int)
                {
                    sqlite3VdbeUsesBtree(v, iDb);
                    pSchema = (*(*db).aDb.offset(iDb as isize)).pSchema;
                    sqlite3VdbeAddOp4Int(
                        v,
                        OP_Transaction,
                        iDb,
                        ((*pParse).writeMask
                            & (1 as ::core::ffi::c_int as yDbMask) << iDb
                            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                        (*pSchema).schema_cookie,
                        (*pSchema).iGeneration,
                    );
                    if (*db).init.busy as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        sqlite3VdbeChangeP5(v, 1 as u16_0);
                    }
                }
                iDb += 1;
                if !(iDb < (*db).nDb) {
                    break;
                }
            }
            i = 0 as ::core::ffi::c_int;
            while i < (*pParse).nVtabLock {
                let mut vtab: *mut ::core::ffi::c_char = sqlite3GetVTable(
                    db,
                    *(*pParse).apVtabLock.offset(i as isize),
                ) as *mut ::core::ffi::c_char;
                sqlite3VdbeAddOp4(
                    v,
                    OP_VBegin,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    vtab,
                    P4_VTAB,
                );
                i += 1;
            }
            (*pParse).nVtabLock = 0 as ::core::ffi::c_int;
            if (*pParse).nTableLock != 0 {
                codeTableLocks(pParse);
            }
            if !(*pParse).pAinc.is_null() {
                sqlite3AutoincrementBegin(pParse);
            }
            if !(*pParse).pConstExpr.is_null() {
                let mut pEL: *mut ExprList = (*pParse).pConstExpr;
                (*pParse).set_okConstFactor(0 as bft as bft);
                i = 0 as ::core::ffi::c_int;
                while i < (*pEL).nExpr {
                    sqlite3ExprCode(
                        pParse,
                        (*(&raw mut (*pEL).a as *mut ExprList_item).offset(i as isize))
                            .pExpr,
                        (*(&raw mut (*pEL).a as *mut ExprList_item).offset(i as isize))
                            .u
                            .iConstExprReg,
                    );
                    i += 1;
                }
            }
            if (*pParse).bReturning != 0 {
                let mut pRet: *mut Returning = ::core::ptr::null_mut::<Returning>();
                pRet = (*pParse).u1.d.pReturning;
                if (*pRet).nRetCol != 0 {
                    sqlite3VdbeAddOp2(
                        v,
                        OP_OpenEphemeral,
                        (*pRet).iRetCur,
                        (*pRet).nRetCol,
                    );
                }
            }
            sqlite3VdbeGoto(v, 1 as ::core::ffi::c_int);
        }
        if (*pParse).nErr == 0 as ::core::ffi::c_int {
            sqlite3VdbeMakeReady(v, pParse);
            (*pParse).rc = SQLITE_DONE;
        } else {
            (*pParse).rc = SQLITE_ERROR;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3NestedParse(
    mut pParse: *mut Parse,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut savedDbFlags: u32_0 = (*db).mDbFlags;
        let mut saveBuf: [::core::ffi::c_char; 136] = [0; 136];
        if (*pParse).nErr != 0 {
            return;
        }
        if (*pParse).eParseMode != 0 {
            return;
        }
        let mut ap = c2rust_args;
        zSql = sqlite3VMPrintf(db, zFormat, ap);
        if zSql.is_null() {
            if (*db).mallocFailed == 0 {
                (*pParse).rc = SQLITE_TOOBIG;
            }
            (*pParse).nErr += 1;
            return;
        }
        (*pParse).nested = (*pParse).nested.wrapping_add(1);
        memcpy(
            &raw mut saveBuf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            (pParse as *mut ::core::ffi::c_char).offset(PARSE_RECURSE_SZ as isize)
                as *const ::core::ffi::c_void,
            PARSE_TAIL_SZ,
        );
        memset(
            (pParse as *mut ::core::ffi::c_char).offset(PARSE_RECURSE_SZ as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            PARSE_TAIL_SZ,
        );
        (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
            | DBFLAG_PreferBuiltin as ::core::ffi::c_uint) as u32_0;
        sqlite3RunParser(pParse, zSql);
        (*db).mDbFlags = savedDbFlags;
        sqlite3DbFree(db, zSql as *mut ::core::ffi::c_void);
        memcpy(
            (pParse as *mut ::core::ffi::c_char).offset(PARSE_RECURSE_SZ as isize)
                as *mut ::core::ffi::c_void,
            &raw mut saveBuf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            PARSE_TAIL_SZ,
        );
        (*pParse).nested = (*pParse).nested.wrapping_sub(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FindTable(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut zDatabase: *const ::core::ffi::c_char,
) -> *mut Table {
    unsafe {
        let mut p: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut i: ::core::ffi::c_int = 0;
        if !zDatabase.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*db).nDb {
                if sqlite3StrICmp(zDatabase, (*(*db).aDb.offset(i as isize)).zDbSName)
                    == 0 as ::core::ffi::c_int
                {
                    break;
                }
                i += 1;
            }
            if i >= (*db).nDb {
                if sqlite3StrICmp(
                    zDatabase,
                    b"main\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    i = 0 as ::core::ffi::c_int;
                } else {
                    return ::core::ptr::null_mut::<Table>()
                }
            }
            p = sqlite3HashFind(
                &raw mut (*(*(*db).aDb.offset(i as isize)).pSchema).tblHash,
                zName,
            ) as *mut Table;
            if p.is_null()
                && sqlite3_strnicmp(
                    zName,
                    b"sqlite_\0".as_ptr() as *const ::core::ffi::c_char,
                    7 as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
            {
                if i == 1 as ::core::ffi::c_int {
                    if sqlite3StrICmp(
                        zName.offset(7 as ::core::ffi::c_int as isize),
                        PREFERRED_TEMP_SCHEMA_TABLE
                            .as_ptr()
                            .offset(7 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                        || sqlite3StrICmp(
                            zName.offset(7 as ::core::ffi::c_int as isize),
                            PREFERRED_SCHEMA_TABLE
                                .as_ptr()
                                .offset(7 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        || sqlite3StrICmp(
                            zName.offset(7 as ::core::ffi::c_int as isize),
                            LEGACY_SCHEMA_TABLE
                                .as_ptr()
                                .offset(7 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        p = sqlite3HashFind(
                            &raw mut (*(*(*db)
                                .aDb
                                .offset(1 as ::core::ffi::c_int as isize))
                                .pSchema)
                                .tblHash,
                            LEGACY_TEMP_SCHEMA_TABLE.as_ptr(),
                        ) as *mut Table;
                    }
                } else if sqlite3StrICmp(
                    zName.offset(7 as ::core::ffi::c_int as isize),
                    PREFERRED_SCHEMA_TABLE
                        .as_ptr()
                        .offset(7 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    p = sqlite3HashFind(
                        &raw mut (*(*(*db).aDb.offset(i as isize)).pSchema).tblHash,
                        LEGACY_SCHEMA_TABLE.as_ptr(),
                    ) as *mut Table;
                }
            }
        } else {
            p = sqlite3HashFind(
                &raw mut (*(*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pSchema)
                    .tblHash,
                zName,
            ) as *mut Table;
            if !p.is_null() {
                return p;
            }
            p = sqlite3HashFind(
                &raw mut (*(*(*db).aDb.offset(0 as ::core::ffi::c_int as isize)).pSchema)
                    .tblHash,
                zName,
            ) as *mut Table;
            if !p.is_null() {
                return p;
            }
            i = 2 as ::core::ffi::c_int;
            while i < (*db).nDb {
                p = sqlite3HashFind(
                    &raw mut (*(*(*db).aDb.offset(i as isize)).pSchema).tblHash,
                    zName,
                ) as *mut Table;
                if !p.is_null() {
                    break;
                }
                i += 1;
            }
            if p.is_null()
                && sqlite3_strnicmp(
                    zName,
                    b"sqlite_\0".as_ptr() as *const ::core::ffi::c_char,
                    7 as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
            {
                if sqlite3StrICmp(
                    zName.offset(7 as ::core::ffi::c_int as isize),
                    PREFERRED_SCHEMA_TABLE
                        .as_ptr()
                        .offset(7 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    p = sqlite3HashFind(
                        &raw mut (*(*(*db).aDb.offset(0 as ::core::ffi::c_int as isize))
                            .pSchema)
                            .tblHash,
                        LEGACY_SCHEMA_TABLE.as_ptr(),
                    ) as *mut Table;
                } else if sqlite3StrICmp(
                    zName.offset(7 as ::core::ffi::c_int as isize),
                    PREFERRED_TEMP_SCHEMA_TABLE
                        .as_ptr()
                        .offset(7 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    p = sqlite3HashFind(
                        &raw mut (*(*(*db).aDb.offset(1 as ::core::ffi::c_int as isize))
                            .pSchema)
                            .tblHash,
                        LEGACY_TEMP_SCHEMA_TABLE.as_ptr(),
                    ) as *mut Table;
                }
            }
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3LocateTable(
    mut pParse: *mut Parse,
    mut flags: u32_0,
    mut zName: *const ::core::ffi::c_char,
    mut zDbase: *const ::core::ffi::c_char,
) -> *mut Table {
    unsafe {
        let mut p: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut db: *mut sqlite3 = (*pParse).db;
        if (*db).mDbFlags & DBFLAG_SchemaKnownOk as u32_0 == 0 as u32_0
            && SQLITE_OK != sqlite3ReadSchema(pParse)
        {
            return ::core::ptr::null_mut::<Table>();
        }
        p = sqlite3FindTable(db, zName, zDbase);
        if p.is_null() {
            if (*pParse).prepFlags as ::core::ffi::c_int & SQLITE_PREPARE_NO_VTAB
                == 0 as ::core::ffi::c_int
                && (*db).init.busy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                let mut pMod: *mut Module = sqlite3HashFind(
                    &raw mut (*db).aModule,
                    zName,
                ) as *mut Module;
                if pMod.is_null()
                    && sqlite3_strnicmp(
                        zName,
                        b"pragma_\0".as_ptr() as *const ::core::ffi::c_char,
                        7 as ::core::ffi::c_int,
                    ) == 0 as ::core::ffi::c_int
                {
                    pMod = sqlite3PragmaVtabRegister(db, zName);
                }
                if pMod.is_null()
                    && sqlite3_strnicmp(
                        zName,
                        b"json\0".as_ptr() as *const ::core::ffi::c_char,
                        4 as ::core::ffi::c_int,
                    ) == 0 as ::core::ffi::c_int
                {
                    pMod = sqlite3JsonVtabRegister(db, zName);
                }
                if pMod.is_null()
                    && sqlite3_stricmp(
                        zName,
                        b"carray\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    pMod = sqlite3CarrayRegister(db);
                }
                if !pMod.is_null() && sqlite3VtabEponymousTableInit(pParse, pMod) != 0 {
                    return (*pMod).pEpoTab;
                }
            }
            if flags & LOCATE_NOERR as u32_0 != 0 {
                return ::core::ptr::null_mut::<Table>();
            }
            (*pParse).set_checkSchema(1 as bft as bft);
        } else if (*p).eTabType as ::core::ffi::c_int == TABTYP_VTAB
            && (*pParse).prepFlags as ::core::ffi::c_int & SQLITE_PREPARE_NO_VTAB
                != 0 as ::core::ffi::c_int
        {
            p = ::core::ptr::null_mut::<Table>();
        }
        if p.is_null() {
            let mut zMsg: *const ::core::ffi::c_char = if flags & LOCATE_VIEW as u32_0
                != 0
            {
                b"no such view\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"no such table\0".as_ptr() as *const ::core::ffi::c_char
            };
            if !zDbase.is_null() {
                sqlite3ErrorMsg(
                    pParse,
                    b"%s: %s.%s\0".as_ptr() as *const ::core::ffi::c_char,
                    zMsg,
                    zDbase,
                    zName,
                );
            } else {
                sqlite3ErrorMsg(
                    pParse,
                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    zMsg,
                    zName,
                );
            }
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3LocateTableItem(
    mut pParse: *mut Parse,
    mut flags: u32_0,
    mut p: *mut SrcItem,
) -> *mut Table {
    unsafe {
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if (*p).fg.fixedSchema() != 0 {
            let mut iDb: ::core::ffi::c_int = sqlite3SchemaToIndex(
                (*pParse).db,
                (*p).u4.pSchema,
            );
            zDb = (*(*(*pParse).db).aDb.offset(iDb as isize)).zDbSName;
        } else {
            zDb = (*p).u4.zDatabase;
        }
        return sqlite3LocateTable(pParse, flags, (*p).zName, zDb);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PreferredTableName(
    mut zName: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        if sqlite3_strnicmp(
            zName,
            b"sqlite_\0".as_ptr() as *const ::core::ffi::c_char,
            7 as ::core::ffi::c_int,
        ) == 0 as ::core::ffi::c_int
        {
            if sqlite3StrICmp(
                zName.offset(7 as ::core::ffi::c_int as isize),
                LEGACY_SCHEMA_TABLE.as_ptr().offset(7 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                return PREFERRED_SCHEMA_TABLE.as_ptr();
            }
            if sqlite3StrICmp(
                zName.offset(7 as ::core::ffi::c_int as isize),
                LEGACY_TEMP_SCHEMA_TABLE
                    .as_ptr()
                    .offset(7 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                return PREFERRED_TEMP_SCHEMA_TABLE.as_ptr();
            }
        }
        return zName;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FindIndex(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
    mut zDb: *const ::core::ffi::c_char,
) -> *mut Index {
    unsafe {
        let mut p: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut i: ::core::ffi::c_int = 0;
        i = OMIT_TEMPDB;
        while i < (*db).nDb {
            let mut j: ::core::ffi::c_int = if i < 2 as ::core::ffi::c_int {
                i ^ 1 as ::core::ffi::c_int
            } else {
                i
            };
            let mut pSchema: *mut Schema = (*(*db).aDb.offset(j as isize)).pSchema;
            if !(!zDb.is_null()
                && sqlite3DbIsNamed(db, j, zDb) == 0 as ::core::ffi::c_int)
            {
                p = sqlite3HashFind(&raw mut (*pSchema).idxHash, zName) as *mut Index;
                if !p.is_null() {
                    break;
                }
            }
            i += 1;
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FreeIndex(mut db: *mut sqlite3, mut p: *mut Index) {
    unsafe {
        sqlite3DeleteIndexSamples(db, p);
        sqlite3ExprDelete(db, (*p).pPartIdxWhere);
        sqlite3ExprListDelete(db, (*p).aColExpr);
        sqlite3DbFree(db, (*p).zColAff as *mut ::core::ffi::c_void);
        if (*p).isResized() != 0 {
            sqlite3DbFree(db, (*p).azColl as *mut ::core::ffi::c_void);
        }
        sqlite3DbFree(db, p as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3UnlinkAndDeleteIndex(
    mut db: *mut sqlite3,
    mut iDb: ::core::ffi::c_int,
    mut zIdxName: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut pIndex: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut pHash: *mut Hash = ::core::ptr::null_mut::<Hash>();
        pHash = &raw mut (*(*(*db).aDb.offset(iDb as isize)).pSchema).idxHash;
        pIndex = sqlite3HashInsert(
            pHash,
            zIdxName,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut Index;
        if !pIndex.is_null() {
            if (*(*pIndex).pTable).pIndex == pIndex {
                (*(*pIndex).pTable).pIndex = (*pIndex).pNext;
            } else {
                let mut p: *mut Index = ::core::ptr::null_mut::<Index>();
                p = (*(*pIndex).pTable).pIndex;
                while !p.is_null() && (*p).pNext != pIndex {
                    p = (*p).pNext;
                }
                if !p.is_null() && (*p).pNext == pIndex {
                    (*p).pNext = (*pIndex).pNext;
                }
            }
            sqlite3FreeIndex(db, pIndex);
        }
        (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
            | DBFLAG_SchemaChange as ::core::ffi::c_uint) as u32_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CollapseDatabaseArray(mut db: *mut sqlite3) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        j = 2 as ::core::ffi::c_int;
        i = j;
        while i < (*db).nDb {
            let mut pDb: *mut Db = (*db).aDb.offset(i as isize) as *mut Db;
            if (*pDb).pBt.is_null() {
                sqlite3DbFree(db, (*pDb).zDbSName as *mut ::core::ffi::c_void);
                (*pDb).zDbSName = ::core::ptr::null_mut::<::core::ffi::c_char>();
            } else {
                if j < i {
                    *(*db).aDb.offset(j as isize) = *(*db).aDb.offset(i as isize);
                }
                j += 1;
            }
            i += 1;
        }
        (*db).nDb = j;
        if (*db).nDb <= 2 as ::core::ffi::c_int
            && (*db).aDb != &raw mut (*db).aDbStatic as *mut Db
        {
            memcpy(
                &raw mut (*db).aDbStatic as *mut Db as *mut ::core::ffi::c_void,
                (*db).aDb as *const ::core::ffi::c_void,
                (2 as size_t).wrapping_mul(::core::mem::size_of::<Db>() as size_t),
            );
            sqlite3DbFree(db, (*db).aDb as *mut ::core::ffi::c_void);
            (*db).aDb = &raw mut (*db).aDbStatic as *mut Db;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ResetOneSchema(
    mut db: *mut sqlite3,
    mut iDb: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if iDb >= 0 as ::core::ffi::c_int {
            let ref mut c2rust_fresh1 = (*(*(*db).aDb.offset(iDb as isize)).pSchema)
                .schemaFlags;
            *c2rust_fresh1 = (*c2rust_fresh1 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int) as u16_0;
            let ref mut c2rust_fresh2 = (*(*(*db)
                .aDb
                .offset(1 as ::core::ffi::c_int as isize))
                .pSchema)
                .schemaFlags;
            *c2rust_fresh2 = (*c2rust_fresh2 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int) as u16_0;
            (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
                & !DBFLAG_SchemaKnownOk as ::core::ffi::c_uint) as u32_0;
        }
        if (*db).nSchemaLock == 0 as u32_0 {
            i = 0 as ::core::ffi::c_int;
            while i < (*db).nDb {
                if (*(*(*db).aDb.offset(i as isize)).pSchema).schemaFlags
                    as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int
                    == 0x8 as ::core::ffi::c_int
                {
                    sqlite3SchemaClear(
                        (*(*db).aDb.offset(i as isize)).pSchema
                            as *mut ::core::ffi::c_void,
                    );
                }
                i += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ResetAllSchemasOfConnection(mut db: *mut sqlite3) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        sqlite3BtreeEnterAll(db);
        i = 0 as ::core::ffi::c_int;
        while i < (*db).nDb {
            let mut pDb: *mut Db = (*db).aDb.offset(i as isize) as *mut Db;
            if !(*pDb).pSchema.is_null() {
                if (*db).nSchemaLock == 0 as u32_0 {
                    sqlite3SchemaClear((*pDb).pSchema as *mut ::core::ffi::c_void);
                } else {
                    let ref mut c2rust_fresh0 = (*(*(*db).aDb.offset(i as isize))
                        .pSchema)
                        .schemaFlags;
                    *c2rust_fresh0 = (*c2rust_fresh0 as ::core::ffi::c_int
                        | 0x8 as ::core::ffi::c_int) as u16_0;
                }
            }
            i += 1;
        }
        (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
            & !(DBFLAG_SchemaChange | DBFLAG_SchemaKnownOk) as ::core::ffi::c_uint)
            as u32_0;
        sqlite3VtabUnlockList(db);
        sqlite3BtreeLeaveAll(db);
        if (*db).nSchemaLock == 0 as u32_0 {
            sqlite3CollapseDatabaseArray(db);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CommitInternalChanges(mut db: *mut sqlite3) {
    unsafe {
        (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
            & !DBFLAG_SchemaChange as ::core::ffi::c_uint) as u32_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ColumnSetExpr(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pCol: *mut Column,
    mut pExpr: *mut Expr,
) {
    unsafe {
        let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
        pList = (*pTab).u.tab.pDfltList;
        if (*pCol).iDflt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || pList.is_null() || (*pList).nExpr < (*pCol).iDflt as ::core::ffi::c_int
        {
            (*pCol).iDflt = (if pList.is_null() {
                1 as ::core::ffi::c_int
            } else {
                (*pList).nExpr + 1 as ::core::ffi::c_int
            }) as u16_0;
            (*pTab).u.tab.pDfltList = sqlite3ExprListAppend(pParse, pList, pExpr);
        } else {
            sqlite3ExprDelete(
                (*pParse).db,
                (*(&raw mut (*pList).a as *mut ExprList_item)
                    .offset(
                        ((*pCol).iDflt as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as isize,
                    ))
                    .pExpr,
            );
            let ref mut c2rust_fresh3 = (*(&raw mut (*pList).a as *mut ExprList_item)
                .offset(
                    ((*pCol).iDflt as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as isize,
                ))
                .pExpr;
            *c2rust_fresh3 = pExpr;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ColumnExpr(
    mut pTab: *mut Table,
    mut pCol: *mut Column,
) -> *mut Expr {
    unsafe {
        if (*pCol).iDflt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<Expr>();
        }
        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_NORM) {
            return ::core::ptr::null_mut::<Expr>();
        }
        if (*pTab).u.tab.pDfltList.is_null() {
            return ::core::ptr::null_mut::<Expr>();
        }
        if (*(*pTab).u.tab.pDfltList).nExpr < (*pCol).iDflt as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<Expr>();
        }
        return (*(&raw mut (*(*pTab).u.tab.pDfltList).a as *mut ExprList_item)
            .offset(
                ((*pCol).iDflt as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
            ))
            .pExpr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ColumnSetColl(
    mut db: *mut sqlite3,
    mut pCol: *mut Column,
    mut zColl: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut nColl: i64_0 = 0;
        let mut n: i64_0 = 0;
        let mut zNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        n = (sqlite3Strlen30((*pCol).zCnName) + 1 as ::core::ffi::c_int) as i64_0;
        if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_HASTYPE != 0 {
            n
                += (sqlite3Strlen30((*pCol).zCnName.offset(n as isize))
                    + 1 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
        }
        nColl = (sqlite3Strlen30(zColl) + 1 as ::core::ffi::c_int) as i64_0;
        zNew = sqlite3DbRealloc(
            db,
            (*pCol).zCnName as *mut ::core::ffi::c_void,
            (nColl + n) as u64_0,
        ) as *mut ::core::ffi::c_char;
        if !zNew.is_null() {
            (*pCol).zCnName = zNew;
            memcpy(
                (*pCol).zCnName.offset(n as isize) as *mut ::core::ffi::c_void,
                zColl as *const ::core::ffi::c_void,
                nColl as size_t,
            );
            (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int | COLFLAG_HASCOLL)
                as u16_0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ColumnColl(
    mut pCol: *mut Column,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_HASCOLL
            == 0 as ::core::ffi::c_int
        {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        z = (*pCol).zCnName;
        while *z != 0 {
            z = z.offset(1);
        }
        if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_HASTYPE != 0 {
            loop {
                z = z.offset(1);
                if !(*z != 0) {
                    break;
                }
            }
        }
        return z.offset(1 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DeleteColumnNames(
    mut db: *mut sqlite3,
    mut pTable: *mut Table,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        pCol = (*pTable).aCol;
        if !pCol.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*pTable).nCol as ::core::ffi::c_int {
                sqlite3DbFree(db, (*pCol).zCnName as *mut ::core::ffi::c_void);
                i += 1;
                pCol = pCol.offset(1);
            }
            sqlite3DbNNFreeNN(db, (*pTable).aCol as *mut ::core::ffi::c_void);
            if (*pTable).eTabType as ::core::ffi::c_int == TABTYP_NORM {
                sqlite3ExprListDelete(db, (*pTable).u.tab.pDfltList);
            }
            if (*db).pnBytesFreed.is_null() {
                (*pTable).aCol = ::core::ptr::null_mut::<Column>();
                (*pTable).nCol = 0 as i16_0;
                if (*pTable).eTabType as ::core::ffi::c_int == TABTYP_NORM {
                    (*pTable).u.tab.pDfltList = ::core::ptr::null_mut::<ExprList>();
                }
            }
        }
    }
}
#[inline(never)]
unsafe extern "C" fn deleteTable(mut db: *mut sqlite3, mut pTable: *mut Table) {
    unsafe {
        let mut pIndex: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut pNext: *mut Index = ::core::ptr::null_mut::<Index>();
        pIndex = (*pTable).pIndex;
        while !pIndex.is_null() {
            pNext = (*pIndex).pNext;
            if (*db).pnBytesFreed.is_null()
                && !((*pTable).eTabType as ::core::ffi::c_int == TABTYP_VTAB)
            {
                let mut zName: *mut ::core::ffi::c_char = (*pIndex).zName;
                sqlite3HashInsert(
                    &raw mut (*(*pIndex).pSchema).idxHash,
                    zName,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
            }
            sqlite3FreeIndex(db, pIndex);
            pIndex = pNext;
        }
        if (*pTable).eTabType as ::core::ffi::c_int == TABTYP_NORM {
            sqlite3FkDelete(db, pTable);
        } else if (*pTable).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
            sqlite3VtabClear(db, pTable);
        } else {
            sqlite3SelectDelete(db, (*pTable).u.view.pSelect);
        }
        sqlite3DeleteColumnNames(db, pTable);
        sqlite3DbFree(db, (*pTable).zName as *mut ::core::ffi::c_void);
        sqlite3DbFree(db, (*pTable).zColAff as *mut ::core::ffi::c_void);
        sqlite3ExprListDelete(db, (*pTable).pCheck);
        sqlite3DbFree(db, pTable as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DeleteTable(
    mut db: *mut sqlite3,
    mut pTable: *mut Table,
) {
    unsafe {
        if pTable.is_null() {
            return;
        }
        if (*db).pnBytesFreed.is_null()
            && {
                (*pTable).nTabRef = (*pTable).nTabRef.wrapping_sub(1);
                (*pTable).nTabRef > 0 as u32_0
            }
        {
            return;
        }
        deleteTable(db, pTable);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DeleteTableGeneric(
    mut db: *mut sqlite3,
    mut pTable: *mut ::core::ffi::c_void,
) {
    unsafe {
        sqlite3DeleteTable(db, pTable as *mut Table);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3UnlinkAndDeleteTable(
    mut db: *mut sqlite3,
    mut iDb: ::core::ffi::c_int,
    mut zTabName: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut p: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pDb: *mut Db = ::core::ptr::null_mut::<Db>();
        pDb = (*db).aDb.offset(iDb as isize) as *mut Db;
        p = sqlite3HashInsert(
            &raw mut (*(*pDb).pSchema).tblHash,
            zTabName,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) as *mut Table;
        sqlite3DeleteTable(db, p);
        (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
            | DBFLAG_SchemaChange as ::core::ffi::c_uint) as u32_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3NameFromToken(
    mut db: *mut sqlite3,
    mut pName: *const Token,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if !pName.is_null() {
            zName = sqlite3DbStrNDup(db, (*pName).z, (*pName).n as u64_0);
            sqlite3Dequote(zName);
        } else {
            zName = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return zName;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OpenSchemaTable(
    mut p: *mut Parse,
    mut iDb: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = sqlite3GetVdbe(p);
        sqlite3TableLock(
            p,
            iDb,
            SCHEMA_ROOT as Pgno,
            1 as u8_0,
            LEGACY_SCHEMA_TABLE.as_ptr(),
        );
        sqlite3VdbeAddOp4Int(
            v,
            OP_OpenWrite,
            0 as ::core::ffi::c_int,
            SCHEMA_ROOT,
            iDb,
            5 as ::core::ffi::c_int,
        );
        if (*p).nTab == 0 as ::core::ffi::c_int {
            (*p).nTab = 1 as ::core::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FindDbName(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        if !zName.is_null() {
            let mut pDb: *mut Db = ::core::ptr::null_mut::<Db>();
            i = (*db).nDb - 1 as ::core::ffi::c_int;
            pDb = (*db).aDb.offset(i as isize) as *mut Db;
            while i >= 0 as ::core::ffi::c_int {
                if 0 as ::core::ffi::c_int == sqlite3_stricmp((*pDb).zDbSName, zName) {
                    break;
                }
                if i == 0 as ::core::ffi::c_int
                    && 0 as ::core::ffi::c_int
                        == sqlite3_stricmp(
                            b"main\0".as_ptr() as *const ::core::ffi::c_char,
                            zName,
                        )
                {
                    break;
                }
                i -= 1;
                pDb = pDb.offset(-1);
            }
        }
        return i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3FindDb(
    mut db: *mut sqlite3,
    mut pName: *mut Token,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        zName = sqlite3NameFromToken(db, pName);
        i = sqlite3FindDbName(db, zName);
        sqlite3DbFree(db, zName as *mut ::core::ffi::c_void);
        return i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TwoPartName(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
    mut pUnqual: *mut *mut Token,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iDb: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        if (*pName2).n > 0 as ::core::ffi::c_uint {
            if (*db).init.busy != 0 {
                sqlite3ErrorMsg(
                    pParse,
                    b"corrupt database\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            *pUnqual = pName2;
            iDb = sqlite3FindDb(db, pName1);
            if iDb < 0 as ::core::ffi::c_int {
                sqlite3ErrorMsg(
                    pParse,
                    b"unknown database %T\0".as_ptr() as *const ::core::ffi::c_char,
                    pName1,
                );
                return -1 as ::core::ffi::c_int;
            }
        } else {
            iDb = (*db).init.iDb as ::core::ffi::c_int;
            *pUnqual = pName1;
        }
        return iDb;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WritableSchema(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        return ((*db).flags as ::core::ffi::c_ulonglong
            & (SQLITE_WriteSchema | SQLITE_Defensive) as ::core::ffi::c_ulonglong
            == SQLITE_WriteSchema as ::core::ffi::c_ulonglong) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CheckObjectName(
    mut pParse: *mut Parse,
    mut zName: *const ::core::ffi::c_char,
    mut zType: *const ::core::ffi::c_char,
    mut zTblName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        if sqlite3WritableSchema(db) != 0
            || (*db).init.imposterTable() as ::core::ffi::c_int != 0
            || sqlite3Config.bExtraSchemaChecks == 0
        {
            return SQLITE_OK;
        }
        if (*db).init.busy != 0 {
            if sqlite3_stricmp(
                zType,
                *(*db).init.azInit.offset(0 as ::core::ffi::c_int as isize),
            ) != 0
                || sqlite3_stricmp(
                    zName,
                    *(*db).init.azInit.offset(1 as ::core::ffi::c_int as isize),
                ) != 0
                || sqlite3_stricmp(
                    zTblName,
                    *(*db).init.azInit.offset(2 as ::core::ffi::c_int as isize),
                ) != 0
            {
                sqlite3ErrorMsg(pParse, b"\0".as_ptr() as *const ::core::ffi::c_char);
                return SQLITE_ERROR;
            }
        } else if (*pParse).nested as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && 0 as ::core::ffi::c_int
                == sqlite3_strnicmp(
                    zName,
                    b"sqlite_\0".as_ptr() as *const ::core::ffi::c_char,
                    7 as ::core::ffi::c_int,
                )
            || sqlite3ReadOnlyShadowTables(db) != 0
                && sqlite3ShadowTableName(db, zName) != 0
        {
            sqlite3ErrorMsg(
                pParse,
                b"object name reserved for internal use: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                zName,
            );
            return SQLITE_ERROR;
        }
        return SQLITE_OK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3PrimaryKeyIndex(mut pTab: *mut Table) -> *mut Index {
    unsafe {
        let mut p: *mut Index = ::core::ptr::null_mut::<Index>();
        p = (*pTab).pIndex;
        while !p.is_null()
            && !((*p).idxType() as ::core::ffi::c_int == SQLITE_IDXTYPE_PRIMARYKEY)
        {
            p = (*p).pNext;
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TableColumnToIndex(
    mut pIdx: *mut Index,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut iCol16: i16_0 = 0;
        iCol16 = iCol as i16_0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdx).nColumn as ::core::ffi::c_int {
            if iCol16 as ::core::ffi::c_int
                == *(*pIdx).aiColumn.offset(i as isize) as ::core::ffi::c_int
            {
                return i;
            }
            i += 1;
        }
        return -1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3StorageColumnToTable(
    mut pTab: *mut Table,
    mut iCol: i16_0,
) -> i16_0 {
    unsafe {
        if (*pTab).tabFlags & TF_HasVirtual as u32_0 != 0 {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i <= iCol as ::core::ffi::c_int {
                if (*(*pTab).aCol.offset(i as isize)).colFlags as ::core::ffi::c_int
                    & COLFLAG_VIRTUAL != 0
                {
                    iCol += 1;
                }
                i += 1;
            }
        }
        return iCol;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3TableColumnToStorage(
    mut pTab: *mut Table,
    mut iCol: i16_0,
) -> i16_0 {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: i16_0 = 0;
        if (*pTab).tabFlags & TF_HasVirtual as u32_0 == 0 as u32_0
            || (iCol as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        {
            return iCol;
        }
        i = 0 as ::core::ffi::c_int;
        n = 0 as i16_0;
        while i < iCol as ::core::ffi::c_int {
            if (*(*pTab).aCol.offset(i as isize)).colFlags as ::core::ffi::c_int
                & COLFLAG_VIRTUAL == 0 as ::core::ffi::c_int
            {
                n += 1;
            }
            i += 1;
        }
        if (*(*pTab).aCol.offset(i as isize)).colFlags as ::core::ffi::c_int
            & COLFLAG_VIRTUAL != 0
        {
            return ((*pTab).nNVCol as ::core::ffi::c_int + i - n as ::core::ffi::c_int)
                as i16_0
        } else {
            return n
        };
    }
}
unsafe extern "C" fn sqlite3ForceNotReadOnly(mut pParse: *mut Parse) {
    unsafe {
        (*pParse).nMem += 1;
        let mut iReg: ::core::ffi::c_int = (*pParse).nMem;
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        if !v.is_null() {
            sqlite3VdbeAddOp3(
                v,
                OP_JournalMode,
                0 as ::core::ffi::c_int,
                iReg,
                PAGER_JOURNALMODE_QUERY,
            );
            sqlite3VdbeUsesBtree(v, 0 as ::core::ffi::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3StartTable(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
    mut isTemp: ::core::ffi::c_int,
    mut isView: ::core::ffi::c_int,
    mut isVirtual: ::core::ffi::c_int,
    mut noErr: ::core::ffi::c_int,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pTable: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut iDb: ::core::ffi::c_int = 0;
        let mut pName: *mut Token = ::core::ptr::null_mut::<Token>();
        if (*db).init.busy as ::core::ffi::c_int != 0 && (*db).init.newTnum == 1 as Pgno
        {
            iDb = (*db).init.iDb as ::core::ffi::c_int;
            zName = sqlite3DbStrDup(
                db,
                if OMIT_TEMPDB == 0 && iDb == 1 as ::core::ffi::c_int {
                    LEGACY_TEMP_SCHEMA_TABLE.as_ptr()
                } else {
                    LEGACY_SCHEMA_TABLE.as_ptr()
                },
            );
            pName = pName1;
        } else {
            iDb = sqlite3TwoPartName(pParse, pName1, pName2, &raw mut pName);
            if iDb < 0 as ::core::ffi::c_int {
                return;
            }
            if OMIT_TEMPDB == 0 && isTemp != 0 && (*pName2).n > 0 as ::core::ffi::c_uint
                && iDb != 1 as ::core::ffi::c_int
            {
                sqlite3ErrorMsg(
                    pParse,
                    b"temporary table name must be unqualified\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return;
            }
            if OMIT_TEMPDB == 0 && isTemp != 0 {
                iDb = 1 as ::core::ffi::c_int;
            }
            zName = sqlite3NameFromToken(db, pName);
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                sqlite3RenameTokenMap(pParse, zName as *mut ::core::ffi::c_void, pName);
            }
        }
        (*pParse).sNameToken = *pName;
        if zName.is_null() {
            return;
        }
        if !(sqlite3CheckObjectName(
            pParse,
            zName,
            if isView != 0 {
                b"view\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"table\0".as_ptr() as *const ::core::ffi::c_char
            },
            zName,
        ) != 0)
        {
            if (*db).init.iDb as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                isTemp = 1 as ::core::ffi::c_int;
            }
            static mut aCode: [u8_0; 4] = [
                SQLITE_CREATE_TABLE as u8_0,
                SQLITE_CREATE_TEMP_TABLE as u8_0,
                SQLITE_CREATE_VIEW as u8_0,
                SQLITE_CREATE_TEMP_VIEW as u8_0,
            ];
            let mut zDb: *mut ::core::ffi::c_char = (*(*db).aDb.offset(iDb as isize))
                .zDbSName;
            if !(sqlite3AuthCheck(
                pParse,
                SQLITE_INSERT,
                if OMIT_TEMPDB == 0 && isTemp == 1 as ::core::ffi::c_int {
                    LEGACY_TEMP_SCHEMA_TABLE.as_ptr()
                } else {
                    LEGACY_SCHEMA_TABLE.as_ptr()
                },
                ::core::ptr::null::<::core::ffi::c_char>(),
                zDb,
            ) != 0)
            {
                if !(isVirtual == 0
                    && sqlite3AuthCheck(
                        pParse,
                        aCode[(isTemp + 2 as ::core::ffi::c_int * isView) as usize]
                            as ::core::ffi::c_int,
                        zName,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        zDb,
                    ) != 0)
                {
                    if !((*pParse).eParseMode as ::core::ffi::c_int != PARSE_MODE_NORMAL)
                    {
                        let mut zDb_0: *mut ::core::ffi::c_char = (*(*db)
                            .aDb
                            .offset(iDb as isize))
                            .zDbSName;
                        if SQLITE_OK != sqlite3ReadSchema(pParse) {
                            c2rust_current_block = 6400536489589497761;
                        } else {
                            pTable = sqlite3FindTable(db, zName, zDb_0);
                            if !pTable.is_null() {
                                if noErr == 0 {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        b"%s %T already exists\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        if (*pTable).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
                                            b"view\0".as_ptr() as *const ::core::ffi::c_char
                                        } else {
                                            b"table\0".as_ptr() as *const ::core::ffi::c_char
                                        },
                                        pName,
                                    );
                                } else {
                                    sqlite3CodeVerifySchema(pParse, iDb);
                                    sqlite3ForceNotReadOnly(pParse);
                                }
                                c2rust_current_block = 6400536489589497761;
                            } else if !sqlite3FindIndex(db, zName, zDb_0).is_null() {
                                sqlite3ErrorMsg(
                                    pParse,
                                    b"there is already an index named %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    zName,
                                );
                                c2rust_current_block = 6400536489589497761;
                            } else {
                                c2rust_current_block = 7746103178988627676;
                            }
                        }
                    } else {
                        c2rust_current_block = 7746103178988627676;
                    }
                    match c2rust_current_block {
                        6400536489589497761 => {}
                        _ => {
                            pTable = sqlite3DbMallocZero(
                                db,
                                ::core::mem::size_of::<Table>() as u64_0,
                            ) as *mut Table;
                            if pTable.is_null() {
                                (*pParse).rc = SQLITE_NOMEM_BKPT;
                                (*pParse).nErr += 1;
                            } else {
                                (*pTable).zName = zName;
                                (*pTable).iPKey = -1 as ::core::ffi::c_int as i16_0;
                                (*pTable).pSchema = (*(*db).aDb.offset(iDb as isize))
                                    .pSchema;
                                (*pTable).nTabRef = 1 as u32_0;
                                (*pTable).nRowLogEst = 200 as LogEst;
                                (*pParse).pNewTable = pTable;
                                if (*db).init.busy == 0
                                    && {
                                        v = sqlite3GetVdbe(pParse);
                                        !v.is_null()
                                    }
                                {
                                    let mut addr1: ::core::ffi::c_int = 0;
                                    let mut fileFormat: ::core::ffi::c_int = 0;
                                    let mut reg1: ::core::ffi::c_int = 0;
                                    let mut reg2: ::core::ffi::c_int = 0;
                                    let mut reg3: ::core::ffi::c_int = 0;
                                    static mut nullRow: [::core::ffi::c_char; 6] = [
                                        6 as ::core::ffi::c_int as ::core::ffi::c_char,
                                        0 as ::core::ffi::c_int as ::core::ffi::c_char,
                                        0 as ::core::ffi::c_int as ::core::ffi::c_char,
                                        0 as ::core::ffi::c_int as ::core::ffi::c_char,
                                        0 as ::core::ffi::c_int as ::core::ffi::c_char,
                                        0 as ::core::ffi::c_int as ::core::ffi::c_char,
                                    ];
                                    sqlite3BeginWriteOperation(
                                        pParse,
                                        1 as ::core::ffi::c_int,
                                        iDb,
                                    );
                                    if isVirtual != 0 {
                                        sqlite3VdbeAddOp0(v, OP_VBegin);
                                    }
                                    (*pParse).nMem += 1;
                                    (*pParse).u1.cr.regRowid = (*pParse).nMem;
                                    reg1 = (*pParse).u1.cr.regRowid;
                                    (*pParse).nMem += 1;
                                    (*pParse).u1.cr.regRoot = (*pParse).nMem;
                                    reg2 = (*pParse).u1.cr.regRoot;
                                    (*pParse).nMem += 1;
                                    reg3 = (*pParse).nMem;
                                    sqlite3VdbeAddOp3(
                                        v,
                                        OP_ReadCookie,
                                        iDb,
                                        reg3,
                                        BTREE_FILE_FORMAT,
                                    );
                                    sqlite3VdbeUsesBtree(v, iDb);
                                    addr1 = sqlite3VdbeAddOp1(v, OP_If, reg3);
                                    fileFormat = if (*db).flags as ::core::ffi::c_ulonglong
                                        & SQLITE_LegacyFileFmt as ::core::ffi::c_ulonglong
                                        != 0 as ::core::ffi::c_ulonglong
                                    {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        SQLITE_MAX_FILE_FORMAT
                                    };
                                    sqlite3VdbeAddOp3(
                                        v,
                                        OP_SetCookie,
                                        iDb,
                                        BTREE_FILE_FORMAT,
                                        fileFormat,
                                    );
                                    sqlite3VdbeAddOp3(
                                        v,
                                        OP_SetCookie,
                                        iDb,
                                        BTREE_TEXT_ENCODING,
                                        (*db).enc as ::core::ffi::c_int,
                                    );
                                    sqlite3VdbeJumpHere(v, addr1);
                                    if isView != 0 || isVirtual != 0 {
                                        sqlite3VdbeAddOp2(
                                            v,
                                            OP_Integer,
                                            0 as ::core::ffi::c_int,
                                            reg2,
                                        );
                                    } else {
                                        (*pParse).u1.cr.addrCrTab = sqlite3VdbeAddOp3(
                                            v,
                                            OP_CreateBtree,
                                            iDb,
                                            reg2,
                                            BTREE_INTKEY,
                                        );
                                    }
                                    sqlite3OpenSchemaTable(pParse, iDb);
                                    sqlite3VdbeAddOp2(
                                        v,
                                        OP_NewRowid,
                                        0 as ::core::ffi::c_int,
                                        reg1,
                                    );
                                    sqlite3VdbeAddOp4(
                                        v,
                                        OP_Blob,
                                        6 as ::core::ffi::c_int,
                                        reg3,
                                        0 as ::core::ffi::c_int,
                                        &raw const nullRow as *const ::core::ffi::c_char,
                                        P4_STATIC,
                                    );
                                    sqlite3VdbeAddOp3(
                                        v,
                                        OP_Insert,
                                        0 as ::core::ffi::c_int,
                                        reg3,
                                        reg1,
                                    );
                                    sqlite3VdbeChangeP5(v, OPFLAG_APPEND as u16_0);
                                    sqlite3VdbeAddOp0(v, OP_Close);
                                } else if (*db).init.imposterTable() != 0 {
                                    (*pTable).tabFlags = ((*pTable).tabFlags
                                        as ::core::ffi::c_uint | TF_Imposter as ::core::ffi::c_uint)
                                        as u32_0;
                                    if (*db).init.imposterTable() as ::core::ffi::c_int
                                        >= 2 as ::core::ffi::c_int
                                    {
                                        (*pTable).tabFlags = ((*pTable).tabFlags
                                            as ::core::ffi::c_uint | TF_Readonly as ::core::ffi::c_uint)
                                            as u32_0;
                                    }
                                }
                                return;
                            }
                        }
                    }
                }
            }
        }
        (*pParse).set_checkSchema(1 as bft as bft);
        sqlite3DbFree(db, zName as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn sqlite3DeleteReturning(
    mut db: *mut sqlite3,
    mut pArg: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut pRet: *mut Returning = pArg as *mut Returning;
        let mut pHash: *mut Hash = ::core::ptr::null_mut::<Hash>();
        pHash = &raw mut (*(*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pSchema)
            .trigHash;
        sqlite3HashInsert(
            pHash,
            &raw mut (*pRet).zName as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        sqlite3ExprListDelete(db, (*pRet).pReturnEL);
        sqlite3DbFree(db, pRet as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AddReturning(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
) {
    unsafe {
        let mut pRet: *mut Returning = ::core::ptr::null_mut::<Returning>();
        let mut pHash: *mut Hash = ::core::ptr::null_mut::<Hash>();
        let mut db: *mut sqlite3 = (*pParse).db;
        if !(*pParse).pNewTrigger.is_null() {
            sqlite3ErrorMsg(
                pParse,
                b"cannot use RETURNING in a trigger\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        (*pParse).bReturning = 1 as u8_0;
        pRet = sqlite3DbMallocZero(db, ::core::mem::size_of::<Returning>() as u64_0)
            as *mut Returning;
        if pRet.is_null() {
            sqlite3ExprListDelete(db, pList);
            return;
        }
        (*pParse).u1.d.pReturning = pRet;
        (*pRet).pParse = pParse;
        (*pRet).pReturnEL = pList;
        sqlite3ParserAddCleanup(
            pParse,
            Some(
                sqlite3DeleteReturning
                    as unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> (),
            ),
            pRet as *mut ::core::ffi::c_void,
        );
        if (*db).mallocFailed != 0 {
            return;
        }
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 40]>() as ::core::ffi::c_int,
            &raw mut (*pRet).zName as *mut ::core::ffi::c_char,
            b"sqlite_returning_%p\0".as_ptr() as *const ::core::ffi::c_char,
            pParse,
        );
        (*pRet).retTrig.zName = &raw mut (*pRet).zName as *mut ::core::ffi::c_char;
        (*pRet).retTrig.op = TK_RETURNING as u8_0;
        (*pRet).retTrig.tr_tm = TRIGGER_AFTER as u8_0;
        (*pRet).retTrig.bReturning = 1 as u8_0;
        (*pRet).retTrig.pSchema = (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize))
            .pSchema;
        (*pRet).retTrig.pTabSchema = (*(*db)
            .aDb
            .offset(1 as ::core::ffi::c_int as isize))
            .pSchema;
        (*pRet).retTrig.step_list = &raw mut (*pRet).retTStep;
        (*pRet).retTStep.op = TK_RETURNING as u8_0;
        (*pRet).retTStep.pTrig = &raw mut (*pRet).retTrig;
        (*pRet).retTStep.pExprList = pList;
        pHash = &raw mut (*(*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pSchema)
            .trigHash;
        if sqlite3HashInsert(
            pHash,
            &raw mut (*pRet).zName as *mut ::core::ffi::c_char,
            &raw mut (*pRet).retTrig as *mut ::core::ffi::c_void,
        ) == &raw mut (*pRet).retTrig as *mut ::core::ffi::c_void
        {
            sqlite3OomFault(db);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AddColumn(
    mut pParse: *mut Parse,
    mut sName: Token,
    mut sType: Token,
) {
    unsafe {
        let mut p: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut i: ::core::ffi::c_int = 0;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zType: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut aNew: *mut Column = ::core::ptr::null_mut::<Column>();
        let mut eType: u8_0 = COLTYPE_CUSTOM as u8_0;
        let mut szEst: u8_0 = 1 as u8_0;
        let mut affinity: ::core::ffi::c_char = SQLITE_AFF_BLOB as ::core::ffi::c_char;
        p = (*pParse).pNewTable;
        if p.is_null() {
            return;
        }
        if (*p).nCol as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            > (*db).aLimit[SQLITE_LIMIT_COLUMN as usize]
        {
            sqlite3ErrorMsg(
                pParse,
                b"too many columns on %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).zName,
            );
            return;
        }
        if !((*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME) {
            sqlite3DequoteToken(&raw mut sName);
        }
        if sType.n >= 16 as ::core::ffi::c_uint
            && sqlite3_strnicmp(
                sType.z.offset(sType.n.wrapping_sub(6 as ::core::ffi::c_uint) as isize),
                b"always\0".as_ptr() as *const ::core::ffi::c_char,
                6 as ::core::ffi::c_int,
            ) == 0 as ::core::ffi::c_int
        {
            sType.n = sType.n.wrapping_sub(6 as ::core::ffi::c_uint);
            while sType.n > 0 as ::core::ffi::c_uint
                && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(
                        *sType
                            .z
                            .offset(
                                sType.n.wrapping_sub(1 as ::core::ffi::c_uint) as isize,
                            ) as ::core::ffi::c_uchar as isize,
                    ) as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
            {
                sType.n = sType.n.wrapping_sub(1);
            }
            if sType.n >= 9 as ::core::ffi::c_uint
                && sqlite3_strnicmp(
                    sType
                        .z
                        .offset(sType.n.wrapping_sub(9 as ::core::ffi::c_uint) as isize),
                    b"generated\0".as_ptr() as *const ::core::ffi::c_char,
                    9 as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
            {
                sType.n = sType.n.wrapping_sub(9 as ::core::ffi::c_uint);
                while sType.n > 0 as ::core::ffi::c_uint
                    && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                        .offset(
                            *sType
                                .z
                                .offset(
                                    sType.n.wrapping_sub(1 as ::core::ffi::c_uint) as isize,
                                ) as ::core::ffi::c_uchar as isize,
                        ) as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
                {
                    sType.n = sType.n.wrapping_sub(1);
                }
            }
        }
        if sType.n >= 3 as ::core::ffi::c_uint {
            sqlite3DequoteToken(&raw mut sType);
            i = 0 as ::core::ffi::c_int;
            while i < SQLITE_N_STDTYPE {
                if sType.n
                    == *(&raw const sqlite3StdTypeLen as *const ::core::ffi::c_uchar)
                        .offset(i as isize) as ::core::ffi::c_uint
                    && sqlite3_strnicmp(
                        sType.z,
                        *(&raw mut sqlite3StdType as *mut *const ::core::ffi::c_char)
                            .offset(i as isize),
                        sType.n as ::core::ffi::c_int,
                    ) == 0 as ::core::ffi::c_int
                {
                    sType.n = 0 as ::core::ffi::c_uint;
                    eType = (i + 1 as ::core::ffi::c_int) as u8_0;
                    affinity = *(&raw const sqlite3StdTypeAffinity
                        as *const ::core::ffi::c_char)
                        .offset(i as isize);
                    if affinity as ::core::ffi::c_int <= SQLITE_AFF_TEXT {
                        szEst = 5 as u8_0;
                    }
                    break;
                } else {
                    i += 1;
                }
            }
        }
        z = sqlite3DbMallocRaw(
            db,
            (sName.n as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong
                + sType.n as ::core::ffi::c_longlong
                + (sType.n > 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
                    as ::core::ffi::c_longlong) as u64_0,
        ) as *mut ::core::ffi::c_char;
        if z.is_null() {
            return;
        }
        if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
            sqlite3RenameTokenMap(pParse, z as *mut ::core::ffi::c_void, &raw mut sName);
        }
        memcpy(
            z as *mut ::core::ffi::c_void,
            sName.z as *const ::core::ffi::c_void,
            sName.n as size_t,
        );
        *z.offset(sName.n as isize) = 0 as ::core::ffi::c_char;
        sqlite3Dequote(z);
        if (*p).nCol as ::core::ffi::c_int != 0
            && sqlite3ColumnIndex(p, z) >= 0 as ::core::ffi::c_int
        {
            sqlite3ErrorMsg(
                pParse,
                b"duplicate column name: %s\0".as_ptr() as *const ::core::ffi::c_char,
                z,
            );
            sqlite3DbFree(db, z as *mut ::core::ffi::c_void);
            return;
        }
        aNew = sqlite3DbRealloc(
            db,
            (*p).aCol as *mut ::core::ffi::c_void,
            (((*p).nCol as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                as u64_0)
                .wrapping_mul(::core::mem::size_of::<Column>() as u64_0),
        ) as *mut Column;
        if aNew.is_null() {
            sqlite3DbFree(db, z as *mut ::core::ffi::c_void);
            return;
        }
        (*p).aCol = aNew;
        pCol = (*p).aCol.offset((*p).nCol as isize) as *mut Column;
        memset(
            pCol as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Column>() as size_t,
        );
        (*pCol).zCnName = z;
        (*pCol).hName = sqlite3StrIHash(z);
        if sType.n == 0 as ::core::ffi::c_uint {
            (*pCol).affinity = affinity;
            (*pCol).set_eCType(eType as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*pCol).szEst = szEst;
        } else {
            zType = z
                .offset(sqlite3Strlen30(z) as isize)
                .offset(1 as ::core::ffi::c_int as isize);
            memcpy(
                zType as *mut ::core::ffi::c_void,
                sType.z as *const ::core::ffi::c_void,
                sType.n as size_t,
            );
            *zType.offset(sType.n as isize) = 0 as ::core::ffi::c_char;
            sqlite3Dequote(zType);
            (*pCol).affinity = sqlite3AffinityType(zType, pCol);
            (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int | COLFLAG_HASTYPE)
                as u16_0;
        }
        if (*p).nCol as ::core::ffi::c_int <= 0xff as ::core::ffi::c_int {
            let mut h: u8_0 = ((*pCol).hName as usize)
                .wrapping_rem(::core::mem::size_of::<[u8_0; 16]>() as usize) as u8_0;
            (*p).aHx[h as usize] = (*p).nCol as u8_0;
        }
        (*p).nCol += 1;
        (*p).nNVCol += 1;
        (*pParse).u1.cr.constraintName.n = 0 as ::core::ffi::c_uint;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AddNotNull(
    mut pParse: *mut Parse,
    mut onError: ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        p = (*pParse).pNewTable;
        if p.is_null() || ((*p).nCol as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
            return;
        }
        pCol = (*p)
            .aCol
            .offset(((*p).nCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
            as *mut Column;
        (*pCol)
            .set_notNull(onError as u8_0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).tabFlags = ((*p).tabFlags as ::core::ffi::c_uint
            | TF_HasNotNull as ::core::ffi::c_uint) as u32_0;
        if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_UNIQUE != 0 {
            let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
            pIdx = (*p).pIndex;
            while !pIdx.is_null() {
                if *(*pIdx).aiColumn.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == (*p).nCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    (*pIdx)
                        .set_uniqNotNull(
                            1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                        );
                }
                pIdx = (*pIdx).pNext;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AffinityType(
    mut zIn: *const ::core::ffi::c_char,
    mut pCol: *mut Column,
) -> ::core::ffi::c_char {
    unsafe {
        let mut h: u32_0 = 0 as u32_0;
        let mut aff: ::core::ffi::c_char = SQLITE_AFF_NUMERIC as ::core::ffi::c_char;
        let mut zChar: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        while *zIn.offset(0 as ::core::ffi::c_int as isize) != 0 {
            let mut x: u8_0 = *(zIn as *mut u8_0);
            h = (h << 8 as ::core::ffi::c_int)
                .wrapping_add(
                    *(&raw const sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                        .offset(x as isize) as u32_0,
                );
            zIn = zIn.offset(1);
            if h
                == ((('c' as i32) << 24 as ::core::ffi::c_int)
                    + (('h' as i32) << 16 as ::core::ffi::c_int)
                    + (('a' as i32) << 8 as ::core::ffi::c_int) + 'r' as i32) as u32_0
            {
                aff = SQLITE_AFF_TEXT as ::core::ffi::c_char;
                zChar = zIn;
            } else if h
                == ((('c' as i32) << 24 as ::core::ffi::c_int)
                    + (('l' as i32) << 16 as ::core::ffi::c_int)
                    + (('o' as i32) << 8 as ::core::ffi::c_int) + 'b' as i32) as u32_0
            {
                aff = SQLITE_AFF_TEXT as ::core::ffi::c_char;
            } else if h
                == ((('t' as i32) << 24 as ::core::ffi::c_int)
                    + (('e' as i32) << 16 as ::core::ffi::c_int)
                    + (('x' as i32) << 8 as ::core::ffi::c_int) + 't' as i32) as u32_0
            {
                aff = SQLITE_AFF_TEXT as ::core::ffi::c_char;
            } else if h
                == ((('b' as i32) << 24 as ::core::ffi::c_int)
                    + (('l' as i32) << 16 as ::core::ffi::c_int)
                    + (('o' as i32) << 8 as ::core::ffi::c_int) + 'b' as i32) as u32_0
                && (aff as ::core::ffi::c_int == SQLITE_AFF_NUMERIC
                    || aff as ::core::ffi::c_int == SQLITE_AFF_REAL)
            {
                aff = SQLITE_AFF_BLOB as ::core::ffi::c_char;
                if *zIn.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '(' as i32
                {
                    zChar = zIn;
                }
            } else if h
                == ((('r' as i32) << 24 as ::core::ffi::c_int)
                    + (('e' as i32) << 16 as ::core::ffi::c_int)
                    + (('a' as i32) << 8 as ::core::ffi::c_int) + 'l' as i32) as u32_0
                && aff as ::core::ffi::c_int == SQLITE_AFF_NUMERIC
            {
                aff = SQLITE_AFF_REAL as ::core::ffi::c_char;
            } else if h
                == ((('f' as i32) << 24 as ::core::ffi::c_int)
                    + (('l' as i32) << 16 as ::core::ffi::c_int)
                    + (('o' as i32) << 8 as ::core::ffi::c_int) + 'a' as i32) as u32_0
                && aff as ::core::ffi::c_int == SQLITE_AFF_NUMERIC
            {
                aff = SQLITE_AFF_REAL as ::core::ffi::c_char;
            } else if h
                == ((('d' as i32) << 24 as ::core::ffi::c_int)
                    + (('o' as i32) << 16 as ::core::ffi::c_int)
                    + (('u' as i32) << 8 as ::core::ffi::c_int) + 'b' as i32) as u32_0
                && aff as ::core::ffi::c_int == SQLITE_AFF_NUMERIC
            {
                aff = SQLITE_AFF_REAL as ::core::ffi::c_char;
            } else {
                if !(h & 0xffffff as u32_0
                    == ((('i' as i32) << 16 as ::core::ffi::c_int)
                        + (('n' as i32) << 8 as ::core::ffi::c_int) + 't' as i32)
                        as u32_0)
                {
                    continue;
                }
                aff = SQLITE_AFF_INTEGER as ::core::ffi::c_char;
                break;
            }
        }
        if !pCol.is_null() {
            let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if (aff as ::core::ffi::c_int) < SQLITE_AFF_NUMERIC {
                if !zChar.is_null() {
                    while *zChar.offset(0 as ::core::ffi::c_int as isize) != 0 {
                        if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                            .offset(
                                *zChar.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                        {
                            sqlite3GetInt32(zChar, &raw mut v);
                            break;
                        } else {
                            zChar = zChar.offset(1);
                        }
                    }
                } else {
                    v = 16 as ::core::ffi::c_int;
                }
            }
            v = v / 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
            if v > 255 as ::core::ffi::c_int {
                v = 255 as ::core::ffi::c_int;
            }
            (*pCol).szEst = v as u8_0;
        }
        return aff;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AddDefaultValue(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut zStart: *const ::core::ffi::c_char,
    mut zEnd: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut p: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        let mut db: *mut sqlite3 = (*pParse).db;
        p = (*pParse).pNewTable;
        if !p.is_null() {
            let mut isInit: ::core::ffi::c_int = ((*db).init.busy as ::core::ffi::c_int
                != 0 && (*db).init.iDb as ::core::ffi::c_int != 1 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            pCol = (*p)
                .aCol
                .offset(
                    ((*p).nCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) as *mut Column;
            if sqlite3ExprIsConstantOrFunction(pExpr, isInit as u8_0) == 0 {
                sqlite3ErrorMsg(
                    pParse,
                    b"default value of column [%s] is not constant\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*pCol).zCnName,
                );
            } else if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_GENERATED != 0 {
                sqlite3ErrorMsg(
                    pParse,
                    b"cannot use DEFAULT on a generated column\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                let mut x: Expr = Expr {
                    op: 0,
                    affExpr: 0,
                    op2: 0,
                    flags: 0,
                    u: C2Rust_Unnamed_8 {
                        zToken: ::core::ptr::null_mut::<::core::ffi::c_char>()

                    },
                    pLeft: ::core::ptr::null_mut::<Expr>(),
                    pRight: ::core::ptr::null_mut::<Expr>(),
                    x: C2Rust_Unnamed_7 {
                        pList: ::core::ptr::null_mut::<ExprList>()

                    },
                    nHeight: 0,
                    iTable: 0,
                    iColumn: 0,
                    iAgg: 0,
                    w: C2Rust_Unnamed_6 { iJoin: 0 },
                    pAggInfo: ::core::ptr::null_mut::<AggInfo>(),
                    y: C2Rust_Unnamed_0 {
                        pTab: ::core::ptr::null_mut::<Table>()

                    }

                };
                let mut pDfltExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
                memset(
                    &raw mut x as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<Expr>() as size_t,
                );
                x.op = TK_SPAN as u8_0;
                x.u.zToken = sqlite3DbSpanDup(db, zStart, zEnd);
                x.pLeft = pExpr;
                x.flags = EP_Skip as u32_0;
                pDfltExpr = sqlite3ExprDup(db, &raw mut x, EXPRDUP_REDUCE);
                sqlite3DbFree(db, x.u.zToken as *mut ::core::ffi::c_void);
                sqlite3ColumnSetExpr(pParse, p, pCol, pDfltExpr);
            }
        }
        if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
            sqlite3RenameExprUnmap(pParse, pExpr);
        }
        sqlite3ExprDelete(db, pExpr);
    }
}
unsafe extern "C" fn sqlite3StringToId(mut p: *mut Expr) {
    unsafe {
        if (*p).op as ::core::ffi::c_int == TK_STRING {
            (*p).op = TK_ID as u8_0;
        } else if (*p).op as ::core::ffi::c_int == TK_COLLATE
            && (*(*p).pLeft).op as ::core::ffi::c_int == TK_STRING
        {
            (*(*p).pLeft).op = TK_ID as u8_0;
        }
    }
}
unsafe extern "C" fn makeColumnPartOfPrimaryKey(
    mut pParse: *mut Parse,
    mut pCol: *mut Column,
) {
    unsafe {
        (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int | COLFLAG_PRIMKEY)
            as u16_0;
        if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_GENERATED != 0 {
            sqlite3ErrorMsg(
                pParse,
                b"generated columns cannot be part of the PRIMARY KEY\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AddPrimaryKey(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut onError: ::core::ffi::c_int,
    mut autoInc: ::core::ffi::c_int,
    mut sortOrder: ::core::ffi::c_int,
) {
    unsafe {
        let mut pTab: *mut Table = (*pParse).pNewTable;
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        let mut iCol: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut nTerm: ::core::ffi::c_int = 0;
        if !pTab.is_null() {
            if (*pTab).tabFlags & TF_HasPrimaryKey as u32_0 != 0 {
                sqlite3ErrorMsg(
                    pParse,
                    b"table \"%s\" has more than one primary key\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*pTab).zName,
                );
            } else {
                (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                    | TF_HasPrimaryKey as ::core::ffi::c_uint) as u32_0;
                if pList.is_null() {
                    iCol = (*pTab).nCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
                    pCol = (*pTab).aCol.offset(iCol as isize) as *mut Column;
                    makeColumnPartOfPrimaryKey(pParse, pCol);
                    nTerm = 1 as ::core::ffi::c_int;
                } else {
                    nTerm = (*pList).nExpr;
                    i = 0 as ::core::ffi::c_int;
                    while i < nTerm {
                        let mut pCExpr: *mut Expr = sqlite3ExprSkipCollate(
                            (*(&raw mut (*pList).a as *mut ExprList_item)
                                .offset(i as isize))
                                .pExpr,
                        );
                        sqlite3StringToId(pCExpr);
                        if (*pCExpr).op as ::core::ffi::c_int == TK_ID {
                            iCol = sqlite3ColumnIndex(pTab, (*pCExpr).u.zToken);
                            if iCol >= 0 as ::core::ffi::c_int {
                                pCol = (*pTab).aCol.offset(iCol as isize) as *mut Column;
                                makeColumnPartOfPrimaryKey(pParse, pCol);
                            }
                        }
                        i += 1;
                    }
                }
                if nTerm == 1 as ::core::ffi::c_int && !pCol.is_null()
                    && (*pCol).eCType() as ::core::ffi::c_int == COLTYPE_INTEGER
                    && sortOrder != SQLITE_SO_DESC
                {
                    if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME
                        && !pList.is_null()
                    {
                        let mut pCExpr_0: *mut Expr = sqlite3ExprSkipCollate(
                            (*(&raw mut (*pList).a as *mut ExprList_item)
                                .offset(0 as ::core::ffi::c_int as isize))
                                .pExpr,
                        );
                        sqlite3RenameTokenRemap(
                            pParse,
                            &raw mut (*pTab).iPKey as *const ::core::ffi::c_void,
                            pCExpr_0 as *const ::core::ffi::c_void,
                        );
                    }
                    (*pTab).iPKey = iCol as i16_0;
                    (*pTab).keyConf = onError as u8_0;
                    (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                        | (autoInc * TF_Autoincrement) as ::core::ffi::c_uint) as u32_0;
                    if !pList.is_null() {
                        (*pParse).iPkSortOrder = (*(&raw mut (*pList).a
                            as *mut ExprList_item)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .fg
                            .sortFlags;
                    }
                    sqlite3HasExplicitNulls(pParse, pList);
                } else if autoInc != 0 {
                    sqlite3ErrorMsg(
                        pParse,
                        b"AUTOINCREMENT is only allowed on an INTEGER PRIMARY KEY\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    sqlite3CreateIndex(
                        pParse,
                        ::core::ptr::null_mut::<Token>(),
                        ::core::ptr::null_mut::<Token>(),
                        ::core::ptr::null_mut::<SrcList>(),
                        pList,
                        onError,
                        ::core::ptr::null_mut::<Token>(),
                        ::core::ptr::null_mut::<Expr>(),
                        sortOrder,
                        0 as ::core::ffi::c_int,
                        SQLITE_IDXTYPE_PRIMARYKEY as u8_0,
                    );
                    pList = ::core::ptr::null_mut::<ExprList>();
                }
            }
        }
        sqlite3ExprListDelete((*pParse).db, pList);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AddCheckConstraint(
    mut pParse: *mut Parse,
    mut pCheckExpr: *mut Expr,
    mut zStart: *const ::core::ffi::c_char,
    mut zEnd: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut pTab: *mut Table = (*pParse).pNewTable;
        let mut db: *mut sqlite3 = (*pParse).db;
        if !pTab.is_null()
            && !((*pParse).eParseMode as ::core::ffi::c_int == PARSE_MODE_DECLARE_VTAB)
            && sqlite3BtreeIsReadonly((*(*db).aDb.offset((*db).init.iDb as isize)).pBt)
                == 0
        {
            (*pTab).pCheck = sqlite3ExprListAppend(pParse, (*pTab).pCheck, pCheckExpr);
            if (*pParse).u1.cr.constraintName.n != 0 {
                sqlite3ExprListSetName(
                    pParse,
                    (*pTab).pCheck,
                    &raw mut (*pParse).u1.cr.constraintName,
                    1 as ::core::ffi::c_int,
                );
            } else {
                let mut t: Token = Token {
                    z: ::core::ptr::null::<::core::ffi::c_char>(),
                    n: 0

                };
                zStart = zStart.offset(1);
                while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(
                        *zStart.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar as isize,
                    ) as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
                {
                    zStart = zStart.offset(1);
                }
                while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(
                        *zEnd.offset(-1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar as isize,
                    ) as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
                {
                    zEnd = zEnd.offset(-1);
                }
                t.z = zStart;
                t.n = zEnd.offset_from(t.z) as ::core::ffi::c_long as ::core::ffi::c_int
                    as ::core::ffi::c_uint;
                sqlite3ExprListSetName(
                    pParse,
                    (*pTab).pCheck,
                    &raw mut t,
                    1 as ::core::ffi::c_int,
                );
            }
        } else {
            sqlite3ExprDelete((*pParse).db, pCheckExpr);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AddCollateType(
    mut pParse: *mut Parse,
    mut pToken: *mut Token,
) {
    unsafe {
        let mut p: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut i: ::core::ffi::c_int = 0;
        let mut zColl: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        p = (*pParse).pNewTable;
        if p.is_null() || (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME
        {
            return;
        }
        i = (*p).nCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        db = (*pParse).db;
        zColl = sqlite3NameFromToken(db, pToken);
        if zColl.is_null() {
            return;
        }
        if !sqlite3LocateCollSeq(pParse, zColl).is_null() {
            let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
            sqlite3ColumnSetColl(db, (*p).aCol.offset(i as isize) as *mut Column, zColl);
            pIdx = (*p).pIndex;
            while !pIdx.is_null() {
                if *(*pIdx).aiColumn.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == i
                {
                    let ref mut c2rust_fresh14 = *(*pIdx)
                        .azColl
                        .offset(0 as ::core::ffi::c_int as isize);
                    *c2rust_fresh14 = sqlite3ColumnColl(
                        (*p).aCol.offset(i as isize) as *mut Column,
                    );
                }
                pIdx = (*pIdx).pNext;
            }
        }
        sqlite3DbFree(db, zColl as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AddGenerated(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pType: *mut Token,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut eType: u8_0 = COLFLAG_VIRTUAL as u8_0;
        let mut pTab: *mut Table = (*pParse).pNewTable;
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        if !pTab.is_null() {
            pCol = (*pTab)
                .aCol
                .offset(
                    ((*pTab).nCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as isize,
                ) as *mut Column;
            if (*pParse).eParseMode as ::core::ffi::c_int == PARSE_MODE_DECLARE_VTAB {
                sqlite3ErrorMsg(
                    pParse,
                    b"virtual tables cannot use computed columns\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                if (*pCol).iDflt as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    c2rust_current_block = 11877844450409184960;
                } else {
                    if !pType.is_null() {
                        if (*pType).n == 7 as ::core::ffi::c_uint
                            && sqlite3_strnicmp(
                                b"virtual\0".as_ptr() as *const ::core::ffi::c_char,
                                (*pType).z,
                                7 as ::core::ffi::c_int,
                            ) == 0 as ::core::ffi::c_int
                        {
                            c2rust_current_block = 5399440093318478209;
                        } else if (*pType).n == 6 as ::core::ffi::c_uint
                            && sqlite3_strnicmp(
                                b"stored\0".as_ptr() as *const ::core::ffi::c_char,
                                (*pType).z,
                                6 as ::core::ffi::c_int,
                            ) == 0 as ::core::ffi::c_int
                        {
                            eType = COLFLAG_STORED as u8_0;
                            c2rust_current_block = 5399440093318478209;
                        } else {
                            c2rust_current_block = 11877844450409184960;
                        }
                    } else {
                        c2rust_current_block = 5399440093318478209;
                    }
                    match c2rust_current_block {
                        11877844450409184960 => {}
                        _ => {
                            if eType as ::core::ffi::c_int == COLFLAG_VIRTUAL {
                                (*pTab).nNVCol -= 1;
                            }
                            (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                                | eType as ::core::ffi::c_int) as u16_0;
                            (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                                | eType as ::core::ffi::c_uint) as u32_0;
                            if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_PRIMKEY
                                != 0
                            {
                                makeColumnPartOfPrimaryKey(pParse, pCol);
                            }
                            if !pExpr.is_null()
                                && (*pExpr).op as ::core::ffi::c_int == TK_ID
                            {
                                pExpr = sqlite3PExpr(
                                    pParse,
                                    TK_UPLUS,
                                    pExpr,
                                    ::core::ptr::null_mut::<Expr>(),
                                );
                            }
                            if !pExpr.is_null()
                                && (*pExpr).op as ::core::ffi::c_int != TK_RAISE
                            {
                                (*pExpr).affExpr = (*pCol).affinity;
                            }
                            sqlite3ColumnSetExpr(pParse, pTab, pCol, pExpr);
                            pExpr = ::core::ptr::null_mut::<Expr>();
                            c2rust_current_block = 10976852346319169975;
                        }
                    }
                }
                match c2rust_current_block {
                    10976852346319169975 => {}
                    _ => {
                        sqlite3ErrorMsg(
                            pParse,
                            b"error in generated column \"%s\"\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*pCol).zCnName,
                        );
                    }
                }
            }
        }
        sqlite3ExprDelete((*pParse).db, pExpr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ChangeCookie(
    mut pParse: *mut Parse,
    mut iDb: ::core::ffi::c_int,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        sqlite3VdbeAddOp3(
            v,
            OP_SetCookie,
            iDb,
            BTREE_SCHEMA_VERSION,
            (1 as ::core::ffi::c_uint)
                .wrapping_add(
                    (*(*(*db).aDb.offset(iDb as isize)).pSchema).schema_cookie
                        as ::core::ffi::c_uint,
                ) as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn identLength(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        n = 0 as ::core::ffi::c_int;
        while *z != 0 {
            if *z as ::core::ffi::c_int == '"' as i32 {
                n += 1;
            }
            n += 1;
            z = z.offset(1);
        }
        return n + 2 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn identPut(
    mut z: *mut ::core::ffi::c_char,
    mut pIdx: *mut ::core::ffi::c_int,
    mut zSignedIdent: *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut zIdent: *mut ::core::ffi::c_uchar = zSignedIdent
            as *mut ::core::ffi::c_uchar;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut needQuote: ::core::ffi::c_int = 0;
        i = *pIdx;
        j = 0 as ::core::ffi::c_int;
        while *zIdent.offset(j as isize) != 0 {
            if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(*zIdent.offset(j as isize) as isize) as ::core::ffi::c_int
                & 0x6 as ::core::ffi::c_int == 0
                && *zIdent.offset(j as isize) as ::core::ffi::c_int != '_' as i32
            {
                break;
            }
            j += 1;
        }
        needQuote = (*(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(*zIdent.offset(0 as ::core::ffi::c_int as isize) as isize)
            as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
            || sqlite3KeywordCode(zIdent, j) != TK_ID
            || *zIdent.offset(j as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int || j == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        if needQuote != 0 {
            let c2rust_fresh17 = i;
            i = i + 1;
            *z.offset(c2rust_fresh17 as isize) = '"' as i32 as ::core::ffi::c_char;
        }
        j = 0 as ::core::ffi::c_int;
        while *zIdent.offset(j as isize) != 0 {
            let c2rust_fresh18 = i;
            i = i + 1;
            *z.offset(c2rust_fresh18 as isize) = *zIdent.offset(j as isize)
                as ::core::ffi::c_char;
            if *zIdent.offset(j as isize) as ::core::ffi::c_int == '"' as i32 {
                let c2rust_fresh19 = i;
                i = i + 1;
                *z.offset(c2rust_fresh19 as isize) = '"' as i32 as ::core::ffi::c_char;
            }
            j += 1;
        }
        if needQuote != 0 {
            let c2rust_fresh20 = i;
            i = i + 1;
            *z.offset(c2rust_fresh20 as isize) = '"' as i32 as ::core::ffi::c_char;
        }
        *z.offset(i as isize) = 0 as ::core::ffi::c_char;
        *pIdx = i;
    }
}
unsafe extern "C" fn createTableStmt(
    mut db: *mut sqlite3,
    mut p: *mut Table,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut len: ::core::ffi::c_int = 0;
        let mut n: i64_0 = 0;
        let mut zStmt: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zSep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zSep2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zEnd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pCol: *mut Column = ::core::ptr::null_mut::<Column>();
        n = 0 as i64_0;
        pCol = (*p).aCol;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nCol as ::core::ffi::c_int {
            n
                += (identLength((*pCol).zCnName) + 5 as ::core::ffi::c_int)
                    as ::core::ffi::c_longlong;
            i += 1;
            pCol = pCol.offset(1);
        }
        n += identLength((*p).zName) as ::core::ffi::c_longlong;
        if n < 50 as ::core::ffi::c_longlong {
            zSep = b"\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            zSep2 = b",\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            zEnd = b")\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        } else {
            zSep = b"\n  \0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            zSep2 = b",\n  \0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            zEnd = b"\n)\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        n
            += (35 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * (*p).nCol as ::core::ffi::c_int)
                as ::core::ffi::c_longlong;
        zStmt = sqlite3DbMallocRaw(::core::ptr::null_mut::<sqlite3>(), n as u64_0)
            as *mut ::core::ffi::c_char;
        if zStmt.is_null() {
            sqlite3OomFault(db);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        memcpy(
            zStmt as *mut ::core::ffi::c_void,
            b"CREATE TABLE \0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            13 as size_t,
        );
        k = 13 as ::core::ffi::c_int;
        identPut(zStmt, &raw mut k, (*p).zName);
        let c2rust_fresh16 = k;
        k = k + 1;
        *zStmt.offset(c2rust_fresh16 as isize) = '(' as i32 as ::core::ffi::c_char;
        pCol = (*p).aCol;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nCol as ::core::ffi::c_int {
            static mut azType: [*const ::core::ffi::c_char; 6] = [
                b"\0".as_ptr() as *const ::core::ffi::c_char,
                b" TEXT\0".as_ptr() as *const ::core::ffi::c_char,
                b" NUM\0".as_ptr() as *const ::core::ffi::c_char,
                b" INT\0".as_ptr() as *const ::core::ffi::c_char,
                b" REAL\0".as_ptr() as *const ::core::ffi::c_char,
                b" NUM\0".as_ptr() as *const ::core::ffi::c_char,
            ];
            let mut zType: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            len = sqlite3Strlen30(zSep);
            memcpy(
                zStmt.offset(k as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                zSep as *const ::core::ffi::c_void,
                len as size_t,
            );
            k += len;
            zSep = zSep2;
            identPut(zStmt, &raw mut k, (*pCol).zCnName);
            zType = azType[((*pCol).affinity as ::core::ffi::c_int - SQLITE_AFF_BLOB)
                as usize];
            len = sqlite3Strlen30(zType);
            memcpy(
                zStmt.offset(k as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                zType as *const ::core::ffi::c_void,
                len as size_t,
            );
            k += len;
            i += 1;
            pCol = pCol.offset(1);
        }
        len = sqlite3Strlen30(zEnd);
        memcpy(
            zStmt.offset(k as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            zEnd as *const ::core::ffi::c_void,
            (len + 1 as ::core::ffi::c_int) as size_t,
        );
        return zStmt;
    }
}
unsafe extern "C" fn resizeIndexObject(
    mut pParse: *mut Parse,
    mut pIdx: *mut Index,
    mut N: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zExtra: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nByte: u64_0 = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if (*pIdx).nColumn as ::core::ffi::c_int >= N {
            return SQLITE_OK;
        }
        db = (*pParse).db;
        nByte = ((::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            .wrapping_add(::core::mem::size_of::<LogEst>() as usize)
            .wrapping_add(::core::mem::size_of::<i16_0>() as usize)
            .wrapping_add(1 as usize) as u64_0)
            .wrapping_mul(N as u64_0);
        zExtra = sqlite3DbMallocZero(db, nByte) as *mut ::core::ffi::c_char;
        if zExtra.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        memcpy(
            zExtra as *mut ::core::ffi::c_void,
            (*pIdx).azColl as *const ::core::ffi::c_void,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul((*pIdx).nColumn as size_t),
        );
        (*pIdx).azColl = zExtra as *mut *const ::core::ffi::c_char;
        zExtra = zExtra
            .offset(
                (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                    .wrapping_mul(N as usize) as isize,
            );
        memcpy(
            zExtra as *mut ::core::ffi::c_void,
            (*pIdx).aiRowLogEst as *const ::core::ffi::c_void,
            (::core::mem::size_of::<LogEst>() as size_t)
                .wrapping_mul(
                    ((*pIdx).nKeyCol as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as size_t,
                ),
        );
        (*pIdx).aiRowLogEst = zExtra as *mut LogEst;
        zExtra = zExtra
            .offset(
                (::core::mem::size_of::<LogEst>() as usize).wrapping_mul(N as usize)
                    as isize,
            );
        memcpy(
            zExtra as *mut ::core::ffi::c_void,
            (*pIdx).aiColumn as *const ::core::ffi::c_void,
            (::core::mem::size_of::<i16_0>() as size_t)
                .wrapping_mul((*pIdx).nColumn as size_t),
        );
        (*pIdx).aiColumn = zExtra as *mut i16_0;
        zExtra = zExtra
            .offset(
                (::core::mem::size_of::<i16_0>() as usize).wrapping_mul(N as usize)
                    as isize,
            );
        memcpy(
            zExtra as *mut ::core::ffi::c_void,
            (*pIdx).aSortOrder as *const ::core::ffi::c_void,
            (*pIdx).nColumn as size_t,
        );
        (*pIdx).aSortOrder = zExtra as *mut u8_0;
        (*pIdx).nColumn = N as u16_0;
        (*pIdx).set_isResized(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn estimateTableWidth(mut pTab: *mut Table) {
    unsafe {
        let mut wTable: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut pTabCol: *const Column = ::core::ptr::null::<Column>();
        let mut i: ::core::ffi::c_int = 0;
        i = (*pTab).nCol as ::core::ffi::c_int;
        pTabCol = (*pTab).aCol;
        while i > 0 as ::core::ffi::c_int {
            wTable = wTable.wrapping_add((*pTabCol).szEst as ::core::ffi::c_uint);
            i -= 1;
            pTabCol = pTabCol.offset(1);
        }
        if ((*pTab).iPKey as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            wTable = wTable.wrapping_add(1);
        }
        (*pTab).szTabRow = sqlite3LogEst(
            wTable.wrapping_mul(4 as ::core::ffi::c_uint) as u64_0,
        );
    }
}
unsafe extern "C" fn estimateIndexWidth(mut pIdx: *mut Index) {
    unsafe {
        let mut wIndex: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut i: ::core::ffi::c_int = 0;
        let mut aCol: *const Column = (*(*pIdx).pTable).aCol;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdx).nColumn as ::core::ffi::c_int {
            let mut x: i16_0 = *(*pIdx).aiColumn.offset(i as isize);
            wIndex = wIndex
                .wrapping_add(
                    (if (x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                        1 as ::core::ffi::c_int
                    } else {
                        (*aCol.offset(x as isize)).szEst as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                );
            i += 1;
        }
        (*pIdx).szIdxRow = sqlite3LogEst(
            wIndex.wrapping_mul(4 as ::core::ffi::c_uint) as u64_0,
        );
    }
}
unsafe extern "C" fn hasColumn(
    mut aiCol: *const i16_0,
    mut nCol: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        loop {
            let c2rust_fresh26 = nCol;
            nCol = nCol - 1;
            if !(c2rust_fresh26 > 0 as ::core::ffi::c_int) {
                break;
            }
            let c2rust_fresh27 = aiCol;
            aiCol = aiCol.offset(1);
            if x == *c2rust_fresh27 as ::core::ffi::c_int {
                return 1 as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn isDupColumn(
    mut pIdx: *mut Index,
    mut nKey: ::core::ffi::c_int,
    mut pPk: *mut Index,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        j = *(*pPk).aiColumn.offset(iCol as isize) as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < nKey {
            if *(*pIdx).aiColumn.offset(i as isize) as ::core::ffi::c_int == j
                && sqlite3StrICmp(
                    *(*pIdx).azColl.offset(i as isize),
                    *(*pPk).azColl.offset(iCol as isize),
                ) == 0 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn recomputeColumnsNotIndexed(mut pIdx: *mut Index) {
    unsafe {
        let mut m: Bitmask = 0 as Bitmask;
        let mut j: ::core::ffi::c_int = 0;
        let mut pTab: *mut Table = (*pIdx).pTable;
        j = (*pIdx).nColumn as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while j >= 0 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = *(*pIdx).aiColumn.offset(j as isize)
                as ::core::ffi::c_int;
            if x >= 0 as ::core::ffi::c_int
                && (*(*pTab).aCol.offset(x as isize)).colFlags as ::core::ffi::c_int
                    & COLFLAG_VIRTUAL == 0 as ::core::ffi::c_int
            {
                if x < BMS - 1 as ::core::ffi::c_int {
                    m
                        |= ((1 as ::core::ffi::c_int as Bitmask) << x)
                            as ::core::ffi::c_ulonglong;
                }
            }
            j -= 1;
        }
        (*pIdx).colNotIdxed = !m;
    }
}
unsafe extern "C" fn convertToWithoutRowidTable(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
) {
    unsafe {
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut pPk: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut nPk: ::core::ffi::c_int = 0;
        let mut nExtra: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut v: *mut Vdbe = (*pParse).pVdbe;
        if (*db).init.imposterTable() == 0 {
            i = 0 as ::core::ffi::c_int;
            while i < (*pTab).nCol as ::core::ffi::c_int {
                if (*(*pTab).aCol.offset(i as isize)).colFlags as ::core::ffi::c_int
                    & COLFLAG_PRIMKEY != 0 as ::core::ffi::c_int
                    && (*(*pTab).aCol.offset(i as isize)).notNull() as ::core::ffi::c_int
                        == OE_None
                {
                    let ref mut c2rust_fresh21 = *(*pTab).aCol.offset(i as isize);
                    (*c2rust_fresh21)
                        .set_notNull(
                            OE_Abort as ::core::ffi::c_uint as ::core::ffi::c_uint,
                        );
                }
                i += 1;
            }
            (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                | TF_HasNotNull as ::core::ffi::c_uint) as u32_0;
        }
        if (*pParse).u1.cr.addrCrTab != 0 {
            sqlite3VdbeChangeP3(v, (*pParse).u1.cr.addrCrTab, BTREE_BLOBKEY);
        }
        if (*pTab).iPKey as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            let mut pList: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            let mut ipkToken: Token = Token {
                z: ::core::ptr::null::<::core::ffi::c_char>(),
                n: 0

            };
            sqlite3TokenInit(
                &raw mut ipkToken,
                (*(*pTab).aCol.offset((*pTab).iPKey as isize)).zCnName,
            );
            pList = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                sqlite3ExprAlloc(db, TK_ID, &raw mut ipkToken, 0 as ::core::ffi::c_int),
            );
            if pList.is_null() {
                (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                    & !TF_WithoutRowid as ::core::ffi::c_uint) as u32_0;
                return;
            }
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                sqlite3RenameTokenRemap(
                    pParse,
                    (*(&raw mut (*pList).a as *mut ExprList_item)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .pExpr as *const ::core::ffi::c_void,
                    &raw mut (*pTab).iPKey as *const ::core::ffi::c_void,
                );
            }
            (*(&raw mut (*pList).a as *mut ExprList_item)
                .offset(0 as ::core::ffi::c_int as isize))
                .fg
                .sortFlags = (*pParse).iPkSortOrder;
            (*pTab).iPKey = -1 as ::core::ffi::c_int as i16_0;
            sqlite3CreateIndex(
                pParse,
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<SrcList>(),
                pList,
                (*pTab).keyConf as ::core::ffi::c_int,
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<Expr>(),
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                SQLITE_IDXTYPE_PRIMARYKEY as u8_0,
            );
            if (*pParse).nErr != 0 {
                (*pTab).tabFlags = ((*pTab).tabFlags as ::core::ffi::c_uint
                    & !TF_WithoutRowid as ::core::ffi::c_uint) as u32_0;
                return;
            }
            pPk = sqlite3PrimaryKeyIndex(pTab);
        } else {
            pPk = sqlite3PrimaryKeyIndex(pTab);
            j = 1 as ::core::ffi::c_int;
            i = j;
            while i < (*pPk).nKeyCol as ::core::ffi::c_int {
                if isDupColumn(pPk, j, pPk, i) != 0 {
                    (*pPk).nColumn = (*pPk).nColumn.wrapping_sub(1);
                } else {
                    let ref mut c2rust_fresh22 = *(*pPk).azColl.offset(j as isize);
                    *c2rust_fresh22 = *(*pPk).azColl.offset(i as isize);
                    *(*pPk).aSortOrder.offset(j as isize) = *(*pPk)
                        .aSortOrder
                        .offset(i as isize);
                    let c2rust_fresh23 = j;
                    j = j + 1;
                    *(*pPk).aiColumn.offset(c2rust_fresh23 as isize) = *(*pPk)
                        .aiColumn
                        .offset(i as isize);
                }
                i += 1;
            }
            (*pPk).nKeyCol = j as u16_0;
        }
        (*pPk).set_isCovering(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if (*db).init.imposterTable() == 0 {
            (*pPk).set_uniqNotNull(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        (*pPk).nColumn = (*pPk).nKeyCol;
        nPk = (*pPk).nColumn as ::core::ffi::c_int;
        if !v.is_null() && (*pPk).tnum > 0 as Pgno {
            sqlite3VdbeChangeOpcode(
                v,
                (*pPk).tnum as ::core::ffi::c_int,
                OP_Goto as u8_0,
            );
        }
        (*pPk).tnum = (*pTab).tnum;
        pIdx = (*pTab).pIndex;
        while !pIdx.is_null() {
            let mut n: ::core::ffi::c_int = 0;
            if !((*pIdx).idxType() as ::core::ffi::c_int == SQLITE_IDXTYPE_PRIMARYKEY) {
                n = 0 as ::core::ffi::c_int;
                i = n;
                while i < nPk {
                    if isDupColumn(pIdx, (*pIdx).nKeyCol as ::core::ffi::c_int, pPk, i)
                        == 0
                    {
                        n += 1;
                    }
                    i += 1;
                }
                if n == 0 as ::core::ffi::c_int {
                    (*pIdx).nColumn = (*pIdx).nKeyCol;
                } else {
                    if resizeIndexObject(
                        pParse,
                        pIdx,
                        (*pIdx).nKeyCol as ::core::ffi::c_int + n,
                    ) != 0
                    {
                        return;
                    }
                    i = 0 as ::core::ffi::c_int;
                    j = (*pIdx).nKeyCol as ::core::ffi::c_int;
                    while i < nPk {
                        if isDupColumn(
                            pIdx,
                            (*pIdx).nKeyCol as ::core::ffi::c_int,
                            pPk,
                            i,
                        ) == 0
                        {
                            *(*pIdx).aiColumn.offset(j as isize) = *(*pPk)
                                .aiColumn
                                .offset(i as isize);
                            let ref mut c2rust_fresh24 = *(*pIdx)
                                .azColl
                                .offset(j as isize);
                            *c2rust_fresh24 = *(*pPk).azColl.offset(i as isize);
                            if *(*pPk).aSortOrder.offset(i as isize) != 0 {
                                (*pIdx)
                                    .set_bAscKeyBug(
                                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                    );
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                }
            }
            pIdx = (*pIdx).pNext;
        }
        nExtra = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*pTab).nCol as ::core::ffi::c_int {
            if hasColumn((*pPk).aiColumn, nPk, i) == 0
                && (*(*pTab).aCol.offset(i as isize)).colFlags as ::core::ffi::c_int
                    & COLFLAG_VIRTUAL == 0 as ::core::ffi::c_int
            {
                nExtra += 1;
            }
            i += 1;
        }
        if resizeIndexObject(pParse, pPk, nPk + nExtra) != 0 {
            return;
        }
        i = 0 as ::core::ffi::c_int;
        j = nPk;
        while i < (*pTab).nCol as ::core::ffi::c_int {
            if hasColumn((*pPk).aiColumn, j, i) == 0
                && (*(*pTab).aCol.offset(i as isize)).colFlags as ::core::ffi::c_int
                    & COLFLAG_VIRTUAL == 0 as ::core::ffi::c_int
            {
                *(*pPk).aiColumn.offset(j as isize) = i as i16_0;
                let ref mut c2rust_fresh25 = *(*pPk).azColl.offset(j as isize);
                *c2rust_fresh25 = &raw const sqlite3StrBINARY
                    as *const ::core::ffi::c_char;
                j += 1;
            }
            i += 1;
        }
        recomputeColumnsNotIndexed(pPk);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IsShadowTableOf(
    mut db: *mut sqlite3,
    mut pTab: *mut Table,
    mut zName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nName: ::core::ffi::c_int = 0;
        let mut pMod: *mut Module = ::core::ptr::null_mut::<Module>();
        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB) {
            return 0 as ::core::ffi::c_int;
        }
        nName = sqlite3Strlen30((*pTab).zName);
        if sqlite3_strnicmp(zName, (*pTab).zName, nName) != 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if *zName.offset(nName as isize) as ::core::ffi::c_int != '_' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        pMod = sqlite3HashFind(
            &raw mut (*db).aModule,
            *(*pTab).u.vtab.azArg.offset(0 as ::core::ffi::c_int as isize),
        ) as *mut Module;
        if pMod.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*pMod).pModule).iVersion < 3 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*pMod).pModule).xShadowName.is_none() {
            return 0 as ::core::ffi::c_int;
        }
        return (*(*pMod).pModule)
            .xShadowName
            .expect(
                "non-null function pointer",
            )(zName.offset(nName as isize).offset(1 as ::core::ffi::c_int as isize));
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MarkAllShadowTablesOf(
    mut db: *mut sqlite3,
    mut pTab: *mut Table,
) {
    unsafe {
        let mut nName: ::core::ffi::c_int = 0;
        let mut pMod: *mut Module = ::core::ptr::null_mut::<Module>();
        let mut k: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        pMod = sqlite3HashFind(
            &raw mut (*db).aModule,
            *(*pTab).u.vtab.azArg.offset(0 as ::core::ffi::c_int as isize),
        ) as *mut Module;
        if pMod.is_null() {
            return;
        }
        if (*pMod).pModule.is_null() {
            return;
        }
        if (*(*pMod).pModule).iVersion < 3 as ::core::ffi::c_int {
            return;
        }
        if (*(*pMod).pModule).xShadowName.is_none() {
            return;
        }
        nName = sqlite3Strlen30((*pTab).zName);
        k = (*(*pTab).pSchema).tblHash.first;
        while !k.is_null() {
            let mut pOther: *mut Table = (*k).data as *mut Table;
            if (*pOther).eTabType as ::core::ffi::c_int == TABTYP_NORM {
                if !((*pOther).tabFlags & TF_Shadow as u32_0 != 0) {
                    if sqlite3_strnicmp((*pOther).zName, (*pTab).zName, nName)
                        == 0 as ::core::ffi::c_int
                        && *(*pOther).zName.offset(nName as isize) as ::core::ffi::c_int
                            == '_' as i32
                        && (*(*pMod).pModule)
                            .xShadowName
                            .expect(
                                "non-null function pointer",
                            )(
                            (*pOther)
                                .zName
                                .offset(nName as isize)
                                .offset(1 as ::core::ffi::c_int as isize),
                        ) != 0
                    {
                        (*pOther).tabFlags = ((*pOther).tabFlags as ::core::ffi::c_uint
                            | TF_Shadow as ::core::ffi::c_uint) as u32_0;
                    }
                }
            }
            k = (*k).next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ShadowTableName(
    mut db: *mut sqlite3,
    mut zName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zTail: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        zTail = strrchr(zName, '_' as i32);
        if zTail.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        *zTail = 0 as ::core::ffi::c_char;
        pTab = sqlite3FindTable(db, zName, ::core::ptr::null::<::core::ffi::c_char>());
        *zTail = '_' as i32 as ::core::ffi::c_char;
        if pTab.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB) {
            return 0 as ::core::ffi::c_int;
        }
        return sqlite3IsShadowTableOf(db, pTab, zName);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3EndTable(
    mut pParse: *mut Parse,
    mut pCons: *mut Token,
    mut pEnd: *mut Token,
    mut tabOpts: u32_0,
    mut pSelect: *mut Select,
) {
    unsafe {
        let mut p: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut iDb: ::core::ffi::c_int = 0;
        let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
        if pEnd.is_null() && pSelect.is_null() {
            return;
        }
        p = (*pParse).pNewTable;
        if p.is_null() {
            return;
        }
        if pSelect.is_null() && sqlite3ShadowTableName(db, (*p).zName) != 0 {
            (*p).tabFlags = ((*p).tabFlags as ::core::ffi::c_uint
                | TF_Shadow as ::core::ffi::c_uint) as u32_0;
        }
        if (*db).init.busy != 0 {
            if !pSelect.is_null()
                || !((*p).eTabType as ::core::ffi::c_int == TABTYP_NORM)
                    && (*db).init.newTnum != 0
            {
                sqlite3ErrorMsg(pParse, b"\0".as_ptr() as *const ::core::ffi::c_char);
                return;
            }
            (*p).tnum = (*db).init.newTnum;
            if (*p).tnum == 1 as Pgno {
                (*p).tabFlags = ((*p).tabFlags as ::core::ffi::c_uint
                    | TF_Readonly as ::core::ffi::c_uint) as u32_0;
            }
        }
        if tabOpts & TF_Strict as u32_0 != 0 {
            let mut ii: ::core::ffi::c_int = 0;
            (*p).tabFlags = ((*p).tabFlags as ::core::ffi::c_uint
                | TF_Strict as ::core::ffi::c_uint) as u32_0;
            ii = 0 as ::core::ffi::c_int;
            while ii < (*p).nCol as ::core::ffi::c_int {
                let mut pCol: *mut Column = (*p).aCol.offset(ii as isize) as *mut Column;
                if (*pCol).eCType() as ::core::ffi::c_int == COLTYPE_CUSTOM {
                    if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_HASTYPE != 0 {
                        sqlite3ErrorMsg(
                            pParse,
                            b"unknown datatype for %s.%s: \"%s\"\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*p).zName,
                            (*pCol).zCnName,
                            sqlite3ColumnType(
                                pCol,
                                b"\0".as_ptr() as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            ),
                        );
                    } else {
                        sqlite3ErrorMsg(
                            pParse,
                            b"missing datatype for %s.%s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*p).zName,
                            (*pCol).zCnName,
                        );
                    }
                    return;
                } else if (*pCol).eCType() as ::core::ffi::c_int == COLTYPE_ANY {
                    (*pCol).affinity = SQLITE_AFF_BLOB as ::core::ffi::c_char;
                }
                if (*pCol).colFlags as ::core::ffi::c_int & COLFLAG_PRIMKEY
                    != 0 as ::core::ffi::c_int && (*p).iPKey as ::core::ffi::c_int != ii
                    && (*pCol).notNull() as ::core::ffi::c_int == OE_None
                {
                    (*pCol)
                        .set_notNull(
                            OE_Abort as ::core::ffi::c_uint as ::core::ffi::c_uint,
                        );
                    (*p).tabFlags = ((*p).tabFlags as ::core::ffi::c_uint
                        | TF_HasNotNull as ::core::ffi::c_uint) as u32_0;
                }
                ii += 1;
            }
        }
        if tabOpts & TF_WithoutRowid as u32_0 != 0 {
            if (*p).tabFlags & TF_Autoincrement as u32_0 != 0 {
                sqlite3ErrorMsg(
                    pParse,
                    b"AUTOINCREMENT not allowed on WITHOUT ROWID tables\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return;
            }
            if (*p).tabFlags & TF_HasPrimaryKey as u32_0 == 0 as u32_0 {
                sqlite3ErrorMsg(
                    pParse,
                    b"PRIMARY KEY missing on table %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*p).zName,
                );
                return;
            }
            (*p).tabFlags = ((*p).tabFlags as ::core::ffi::c_uint
                | (TF_WithoutRowid | TF_NoVisibleRowid) as ::core::ffi::c_uint) as u32_0;
            convertToWithoutRowidTable(pParse, p);
        }
        iDb = sqlite3SchemaToIndex(db, (*p).pSchema);
        if !(*p).pCheck.is_null() {
            sqlite3ResolveSelfReference(
                pParse,
                p,
                NC_IsCheck,
                ::core::ptr::null_mut::<Expr>(),
                (*p).pCheck,
            );
            if (*pParse).nErr != 0 {
                sqlite3ExprListDelete(db, (*p).pCheck);
                (*p).pCheck = ::core::ptr::null_mut::<ExprList>();
            }
        }
        if (*p).tabFlags & TF_HasGenerated as u32_0 != 0 {
            let mut ii_0: ::core::ffi::c_int = 0;
            let mut nNG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            ii_0 = 0 as ::core::ffi::c_int;
            while ii_0 < (*p).nCol as ::core::ffi::c_int {
                let mut colFlags: u32_0 = (*(*p).aCol.offset(ii_0 as isize)).colFlags
                    as u32_0;
                if colFlags & COLFLAG_GENERATED as u32_0 != 0 as u32_0 {
                    let mut pX: *mut Expr = sqlite3ColumnExpr(
                        p,
                        (*p).aCol.offset(ii_0 as isize) as *mut Column,
                    );
                    if sqlite3ResolveSelfReference(
                        pParse,
                        p,
                        NC_GenCol,
                        pX,
                        ::core::ptr::null_mut::<ExprList>(),
                    ) != 0
                    {
                        sqlite3ColumnSetExpr(
                            pParse,
                            p,
                            (*p).aCol.offset(ii_0 as isize) as *mut Column,
                            sqlite3ExprAlloc(
                                db,
                                TK_NULL,
                                ::core::ptr::null::<Token>(),
                                0 as ::core::ffi::c_int,
                            ),
                        );
                    }
                } else {
                    nNG += 1;
                }
                ii_0 += 1;
            }
            if nNG == 0 as ::core::ffi::c_int {
                sqlite3ErrorMsg(
                    pParse,
                    b"must have at least one non-generated column\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return;
            }
        }
        estimateTableWidth(p);
        pIdx = (*p).pIndex;
        while !pIdx.is_null() {
            estimateIndexWidth(pIdx);
            pIdx = (*pIdx).pNext;
        }
        if (*db).init.busy == 0 {
            let mut n: ::core::ffi::c_int = 0;
            let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
            let mut zType: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut zType2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut zStmt: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            v = sqlite3GetVdbe(pParse);
            if v.is_null() {
                return;
            }
            sqlite3VdbeAddOp1(v, OP_Close, 0 as ::core::ffi::c_int);
            if (*p).eTabType as ::core::ffi::c_int == TABTYP_NORM {
                zType = b"table\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                zType2 = b"TABLE\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            } else {
                zType = b"view\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                zType2 = b"VIEW\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            if !pSelect.is_null() {
                let mut dest: SelectDest = SelectDest {
                    eDest: 0,
                    iSDParm: 0,
                    iSDParm2: 0,
                    iSdst: 0,
                    nSdst: 0,
                    zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    pOrderBy: ::core::ptr::null_mut::<ExprList>()

                };
                let mut regYield: ::core::ffi::c_int = 0;
                let mut addrTop: ::core::ffi::c_int = 0;
                let mut regRec: ::core::ffi::c_int = 0;
                let mut regRowid: ::core::ffi::c_int = 0;
                let mut addrInsLoop: ::core::ffi::c_int = 0;
                let mut pSelTab: *mut Table = ::core::ptr::null_mut::<Table>();
                let mut iCsr: ::core::ffi::c_int = 0;
                if (*pParse).eParseMode as ::core::ffi::c_int != PARSE_MODE_NORMAL {
                    (*pParse).rc = SQLITE_ERROR;
                    (*pParse).nErr += 1;
                    return;
                }
                let c2rust_fresh15 = (*pParse).nTab;
                (*pParse).nTab = (*pParse).nTab + 1;
                iCsr = c2rust_fresh15;
                (*pParse).nMem += 1;
                regYield = (*pParse).nMem;
                (*pParse).nMem += 1;
                regRec = (*pParse).nMem;
                (*pParse).nMem += 1;
                regRowid = (*pParse).nMem;
                sqlite3MayAbort(pParse);
                sqlite3VdbeAddOp3(v, OP_OpenWrite, iCsr, (*pParse).u1.cr.regRoot, iDb);
                sqlite3VdbeChangeP5(v, OPFLAG_P2ISREG as u16_0);
                addrTop = sqlite3VdbeCurrentAddr(v) + 1 as ::core::ffi::c_int;
                sqlite3VdbeAddOp3(
                    v,
                    OP_InitCoroutine,
                    regYield,
                    0 as ::core::ffi::c_int,
                    addrTop,
                );
                if (*pParse).nErr != 0 {
                    return;
                }
                pSelTab = sqlite3ResultSetOfSelect(
                    pParse,
                    pSelect,
                    SQLITE_AFF_BLOB as ::core::ffi::c_char,
                );
                if pSelTab.is_null() {
                    return;
                }
                (*p).nNVCol = (*pSelTab).nCol;
                (*p).nCol = (*p).nNVCol;
                (*p).aCol = (*pSelTab).aCol;
                (*pSelTab).nCol = 0 as i16_0;
                (*pSelTab).aCol = ::core::ptr::null_mut::<Column>();
                sqlite3DeleteTable(db, pSelTab);
                sqlite3SelectDestInit(&raw mut dest, SRT_Coroutine, regYield);
                sqlite3Select(pParse, pSelect, &raw mut dest);
                if (*pParse).nErr != 0 {
                    return;
                }
                sqlite3VdbeEndCoroutine(v, regYield);
                sqlite3VdbeJumpHere(v, addrTop - 1 as ::core::ffi::c_int);
                addrInsLoop = sqlite3VdbeAddOp1(v, OP_Yield, dest.iSDParm);
                sqlite3VdbeAddOp3(v, OP_MakeRecord, dest.iSdst, dest.nSdst, regRec);
                sqlite3TableAffinity(v, p, 0 as ::core::ffi::c_int);
                sqlite3VdbeAddOp2(v, OP_NewRowid, iCsr, regRowid);
                sqlite3VdbeAddOp3(v, OP_Insert, iCsr, regRec, regRowid);
                sqlite3VdbeGoto(v, addrInsLoop);
                sqlite3VdbeJumpHere(v, addrInsLoop);
                sqlite3VdbeAddOp1(v, OP_Close, iCsr);
            }
            if !pSelect.is_null() {
                zStmt = createTableStmt(db, p);
            } else {
                let mut pEnd2: *mut Token = if tabOpts != 0 {
                    &raw mut (*pParse).sLastToken
                } else {
                    pEnd
                };
                n = (*pEnd2).z.offset_from((*pParse).sNameToken.z) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
                if *(*pEnd2).z.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int != ';' as i32
                {
                    n = (n as ::core::ffi::c_uint).wrapping_add((*pEnd2).n)
                        as ::core::ffi::c_int as ::core::ffi::c_int;
                }
                zStmt = sqlite3MPrintf(
                    db,
                    b"CREATE %s %.*s\0".as_ptr() as *const ::core::ffi::c_char,
                    zType2,
                    n,
                    (*pParse).sNameToken.z,
                );
            }
            sqlite3NestedParse(
                pParse,
                b"UPDATE %Q.sqlite_master SET type='%s', name=%Q, tbl_name=%Q, rootpage=#%d, sql=%Q WHERE rowid=#%d\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*(*db).aDb.offset(iDb as isize)).zDbSName,
                zType,
                (*p).zName,
                (*p).zName,
                (*pParse).u1.cr.regRoot,
                zStmt,
                (*pParse).u1.cr.regRowid,
            );
            sqlite3DbFree(db, zStmt as *mut ::core::ffi::c_void);
            sqlite3ChangeCookie(pParse, iDb);
            if (*p).tabFlags & TF_Autoincrement as u32_0 != 0 as u32_0
                && !((*pParse).eParseMode as ::core::ffi::c_int != PARSE_MODE_NORMAL)
            {
                let mut pDb: *mut Db = (*db).aDb.offset(iDb as isize) as *mut Db;
                if (*(*pDb).pSchema).pSeqTab.is_null() {
                    sqlite3NestedParse(
                        pParse,
                        b"CREATE TABLE %Q.sqlite_sequence(name,seq)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*pDb).zDbSName,
                    );
                }
            }
            sqlite3VdbeAddParseSchemaOp(
                v,
                iDb,
                sqlite3MPrintf(
                    db,
                    b"tbl_name='%q' AND type!='trigger'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*p).zName,
                ),
                0 as u16_0,
            );
            if (*p).tabFlags & TF_HasGenerated as u32_0 != 0 {
                sqlite3VdbeAddOp4(
                    v,
                    OP_SqlExec,
                    0x1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    sqlite3MPrintf(
                        db,
                        b"SELECT*FROM\"%w\".\"%w\"\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*(*db).aDb.offset(iDb as isize)).zDbSName,
                        (*p).zName,
                    ),
                    P4_DYNAMIC,
                );
            }
        }
        if (*db).init.busy != 0 {
            let mut pOld: *mut Table = ::core::ptr::null_mut::<Table>();
            let mut pSchema: *mut Schema = (*p).pSchema;
            pOld = sqlite3HashInsert(
                &raw mut (*pSchema).tblHash,
                (*p).zName,
                p as *mut ::core::ffi::c_void,
            ) as *mut Table;
            if !pOld.is_null() {
                sqlite3OomFault(db);
                return;
            }
            (*pParse).pNewTable = ::core::ptr::null_mut::<Table>();
            (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
                | DBFLAG_SchemaChange as ::core::ffi::c_uint) as u32_0;
            if strcmp(
                (*p).zName,
                b"sqlite_sequence\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*(*p).pSchema).pSeqTab = p;
            }
        }
        if pSelect.is_null() && (*p).eTabType as ::core::ffi::c_int == TABTYP_NORM {
            if (*pCons).z.is_null() {
                pCons = pEnd;
            }
            (*p).u.tab.addColOffset = 13 as ::core::ffi::c_int
                + (*pCons).z.offset_from((*pParse).sNameToken.z) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CreateView(
    mut pParse: *mut Parse,
    mut pBegin: *mut Token,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
    mut pCNames: *mut ExprList,
    mut pSelect: *mut Select,
    mut isTemp: ::core::ffi::c_int,
    mut noErr: ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut n: ::core::ffi::c_int = 0;
        let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut sEnd: Token = Token {
            z: ::core::ptr::null::<::core::ffi::c_char>(),
            n: 0

        };
        let mut sFix: DbFixer = DbFixer {
            pParse: ::core::ptr::null_mut::<Parse>(),
            w: Walker {
                pParse: ::core::ptr::null_mut::<Parse>(),
                xExprCallback: None,
                xSelectCallback: None,
                xSelectCallback2: None,
                walkerDepth: 0,
                eCode: 0,
                mWFlags: 0,
                u: C2Rust_Unnamed_22 {
                    pNC: ::core::ptr::null_mut::<NameContext>()

                }

            },
            pSchema: ::core::ptr::null_mut::<Schema>(),
            bTemp: 0,
            zDb: ::core::ptr::null::<::core::ffi::c_char>(),
            zType: ::core::ptr::null::<::core::ffi::c_char>(),
            pName: ::core::ptr::null::<Token>()

        };
        let mut pName: *mut Token = ::core::ptr::null_mut::<Token>();
        let mut iDb: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        if (*pParse).nVar as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            sqlite3ErrorMsg(
                pParse,
                b"parameters are not allowed in views\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            sqlite3StartTable(
                pParse,
                pName1,
                pName2,
                isTemp,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                noErr,
            );
            p = (*pParse).pNewTable;
            if !(p.is_null() || (*pParse).nErr != 0) {
                (*p).tabFlags = ((*p).tabFlags as ::core::ffi::c_uint
                    | TF_NoVisibleRowid as ::core::ffi::c_uint) as u32_0;
                sqlite3TwoPartName(pParse, pName1, pName2, &raw mut pName);
                iDb = sqlite3SchemaToIndex(db, (*p).pSchema);
                sqlite3FixInit(
                    &raw mut sFix,
                    pParse,
                    iDb,
                    b"view\0".as_ptr() as *const ::core::ffi::c_char,
                    pName,
                );
                if !(sqlite3FixSelect(&raw mut sFix, pSelect) != 0) {
                    (*pSelect).selFlags = ((*pSelect).selFlags as ::core::ffi::c_uint
                        | SF_View as ::core::ffi::c_uint) as u32_0;
                    if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                        (*p).u.view.pSelect = pSelect;
                        pSelect = ::core::ptr::null_mut::<Select>();
                    } else {
                        (*p).u.view.pSelect = sqlite3SelectDup(
                            db,
                            pSelect,
                            EXPRDUP_REDUCE,
                        );
                    }
                    (*p).pCheck = sqlite3ExprListDup(db, pCNames, EXPRDUP_REDUCE);
                    (*p).eTabType = TABTYP_VIEW as u8_0;
                    if !((*db).mallocFailed != 0) {
                        sEnd = (*pParse).sLastToken;
                        if *sEnd.z.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int != ';' as i32
                        {
                            sEnd.z = sEnd.z.offset(sEnd.n as isize);
                        }
                        sEnd.n = 0 as ::core::ffi::c_uint;
                        n = sEnd.z.offset_from((*pBegin).z) as ::core::ffi::c_long
                            as ::core::ffi::c_int;
                        z = (*pBegin).z;
                        while *(&raw const sqlite3CtypeMap
                            as *const ::core::ffi::c_uchar)
                            .offset(
                                *z.offset((n - 1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
                        {
                            n -= 1;
                        }
                        sEnd.z = z.offset((n - 1 as ::core::ffi::c_int) as isize)
                            as *const ::core::ffi::c_char;
                        sEnd.n = 1 as ::core::ffi::c_uint;
                        sqlite3EndTable(
                            pParse,
                            ::core::ptr::null_mut::<Token>(),
                            &raw mut sEnd,
                            0 as u32_0,
                            ::core::ptr::null_mut::<Select>(),
                        );
                    }
                }
            }
        }
        sqlite3SelectDelete(db, pSelect);
        if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
            sqlite3RenameExprlistUnmap(pParse, pCNames);
        }
        sqlite3ExprListDelete(db, pCNames);
    }
}
#[inline(never)]
unsafe extern "C" fn viewGetColumnNames(
    mut pParse: *mut Parse,
    mut pTable: *mut Table,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSelTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pSel: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut nErr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut rc: ::core::ffi::c_int = 0;
        let mut xAuth: sqlite3_xauth = None;
        if (*pTable).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
            (*db).nSchemaLock = (*db).nSchemaLock.wrapping_add(1);
            rc = sqlite3VtabCallConnect(pParse, pTable);
            (*db).nSchemaLock = (*db).nSchemaLock.wrapping_sub(1);
            return rc;
        }
        if ((*pTable).nCol as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            sqlite3ErrorMsg(
                pParse,
                b"view %s is circularly defined\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pTable).zName,
            );
            return 1 as ::core::ffi::c_int;
        }
        pSel = sqlite3SelectDup(db, (*pTable).u.view.pSelect, 0 as ::core::ffi::c_int);
        if !pSel.is_null() {
            let mut eParseMode: u8_0 = (*pParse).eParseMode;
            let mut nTab: ::core::ffi::c_int = (*pParse).nTab;
            let mut nSelect: ::core::ffi::c_int = (*pParse).nSelect;
            (*pParse).eParseMode = PARSE_MODE_NORMAL as u8_0;
            sqlite3SrcListAssignCursors(pParse, (*pSel).pSrc);
            (*pTable).nCol = -1 as ::core::ffi::c_int as i16_0;
            (*db).lookaside.bDisable = (*db).lookaside.bDisable.wrapping_add(1);
            (*db).lookaside.sz = 0 as u16_0;
            xAuth = (*db).xAuth;
            (*db).xAuth = None;
            pSelTab = sqlite3ResultSetOfSelect(
                pParse,
                pSel,
                SQLITE_AFF_NONE as ::core::ffi::c_char,
            );
            (*db).xAuth = xAuth;
            (*pParse).nTab = nTab;
            (*pParse).nSelect = nSelect;
            if pSelTab.is_null() {
                (*pTable).nCol = 0 as i16_0;
                nErr += 1;
            } else if !(*pTable).pCheck.is_null() {
                sqlite3ColumnsFromExprList(
                    pParse,
                    (*pTable).pCheck,
                    &raw mut (*pTable).nCol,
                    &raw mut (*pTable).aCol,
                );
                if (*pParse).nErr == 0 as ::core::ffi::c_int
                    && (*pTable).nCol as ::core::ffi::c_int == (*(*pSel).pEList).nExpr
                {
                    sqlite3SubqueryColumnTypes(
                        pParse,
                        pTable,
                        pSel,
                        SQLITE_AFF_NONE as ::core::ffi::c_char,
                    );
                }
            } else {
                (*pTable).nCol = (*pSelTab).nCol;
                (*pTable).aCol = (*pSelTab).aCol;
                (*pTable).tabFlags = ((*pTable).tabFlags as ::core::ffi::c_uint
                    | ((*pSelTab).tabFlags & COLFLAG_NOINSERT as u32_0)
                        as ::core::ffi::c_uint) as u32_0;
                (*pSelTab).nCol = 0 as i16_0;
                (*pSelTab).aCol = ::core::ptr::null_mut::<Column>();
            }
            (*pTable).nNVCol = (*pTable).nCol;
            sqlite3DeleteTable(db, pSelTab);
            sqlite3SelectDelete(db, pSel);
            (*db).lookaside.bDisable = (*db).lookaside.bDisable.wrapping_sub(1);
            (*db).lookaside.sz = (if (*db).lookaside.bDisable != 0 {
                0 as ::core::ffi::c_int
            } else {
                (*db).lookaside.szTrue as ::core::ffi::c_int
            }) as u16_0;
            (*pParse).eParseMode = eParseMode;
        } else {
            nErr += 1;
        }
        (*(*pTable).pSchema).schemaFlags = ((*(*pTable).pSchema).schemaFlags
            as ::core::ffi::c_int | DB_UnresetViews) as u16_0;
        if (*db).mallocFailed != 0 {
            sqlite3DeleteColumnNames(db, pTable);
        }
        return nErr + (*pParse).nErr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ViewGetColumnNames(
    mut pParse: *mut Parse,
    mut pTable: *mut Table,
) -> ::core::ffi::c_int {
    unsafe {
        if !((*pTable).eTabType as ::core::ffi::c_int == TABTYP_VTAB)
            && (*pTable).nCol as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        return viewGetColumnNames(pParse, pTable);
    }
}
unsafe extern "C" fn sqliteViewResetAll(
    mut db: *mut sqlite3,
    mut idx: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        if !((*(*(*db).aDb.offset(idx as isize)).pSchema).schemaFlags
            as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
            == 0x2 as ::core::ffi::c_int)
        {
            return;
        }
        i = (*(*(*db).aDb.offset(idx as isize)).pSchema).tblHash.first;
        while !i.is_null() {
            let mut pTab: *mut Table = (*i).data as *mut Table;
            if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW {
                sqlite3DeleteColumnNames(db, pTab);
            }
            i = (*i).next;
        }
        let ref mut c2rust_fresh29 = (*(*(*db).aDb.offset(idx as isize)).pSchema)
            .schemaFlags;
        *c2rust_fresh29 = (*c2rust_fresh29 as ::core::ffi::c_int
            & !(0x2 as ::core::ffi::c_int)) as u16_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RootPageMoved(
    mut db: *mut sqlite3,
    mut iDb: ::core::ffi::c_int,
    mut iFrom: Pgno,
    mut iTo: Pgno,
) {
    unsafe {
        let mut pElem: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut pHash: *mut Hash = ::core::ptr::null_mut::<Hash>();
        let mut pDb: *mut Db = ::core::ptr::null_mut::<Db>();
        pDb = (*db).aDb.offset(iDb as isize) as *mut Db;
        pHash = &raw mut (*(*pDb).pSchema).tblHash;
        pElem = (*pHash).first;
        while !pElem.is_null() {
            let mut pTab: *mut Table = (*pElem).data as *mut Table;
            if (*pTab).tnum == iFrom {
                (*pTab).tnum = iTo;
            }
            pElem = (*pElem).next;
        }
        pHash = &raw mut (*(*pDb).pSchema).idxHash;
        pElem = (*pHash).first;
        while !pElem.is_null() {
            let mut pIdx: *mut Index = (*pElem).data as *mut Index;
            if (*pIdx).tnum == iFrom {
                (*pIdx).tnum = iTo;
            }
            pElem = (*pElem).next;
        }
    }
}
unsafe extern "C" fn destroyRootPage(
    mut pParse: *mut Parse,
    mut iTable: ::core::ffi::c_int,
    mut iDb: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
        let mut r1: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
        if iTable < 2 as ::core::ffi::c_int {
            sqlite3ErrorMsg(
                pParse,
                b"corrupt schema\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        sqlite3VdbeAddOp3(v, OP_Destroy, iTable, r1, iDb);
        sqlite3MayAbort(pParse);
        sqlite3NestedParse(
            pParse,
            b"UPDATE %Q.sqlite_master SET rootpage=%d WHERE #%d AND rootpage=#%d\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*(*(*pParse).db).aDb.offset(iDb as isize)).zDbSName,
            iTable,
            r1,
            r1,
        );
        sqlite3ReleaseTempReg(pParse, r1);
    }
}
unsafe extern "C" fn destroyTable(mut pParse: *mut Parse, mut pTab: *mut Table) {
    unsafe {
        let mut iTab: Pgno = (*pTab).tnum;
        let mut iDestroyed: Pgno = 0 as Pgno;
        loop {
            let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
            let mut iLargest: Pgno = 0 as Pgno;
            if iDestroyed == 0 as Pgno || iTab < iDestroyed {
                iLargest = iTab;
            }
            pIdx = (*pTab).pIndex;
            while !pIdx.is_null() {
                let mut iIdx: Pgno = (*pIdx).tnum;
                if (iDestroyed == 0 as Pgno || iIdx < iDestroyed) && iIdx > iLargest {
                    iLargest = iIdx;
                }
                pIdx = (*pIdx).pNext;
            }
            if iLargest == 0 as Pgno {
                return
            } else {
                let mut iDb: ::core::ffi::c_int = sqlite3SchemaToIndex(
                    (*pParse).db,
                    (*pTab).pSchema,
                );
                destroyRootPage(pParse, iLargest as ::core::ffi::c_int, iDb);
                iDestroyed = iLargest;
            }
        };
    }
}
unsafe extern "C" fn sqlite3ClearStatTables(
    mut pParse: *mut Parse,
    mut iDb: ::core::ffi::c_int,
    mut zType: *const ::core::ffi::c_char,
    mut zName: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut zDbName: *const ::core::ffi::c_char = (*(*(*pParse).db)
            .aDb
            .offset(iDb as isize))
            .zDbSName;
        i = 1 as ::core::ffi::c_int;
        while i <= 4 as ::core::ffi::c_int {
            let mut zTab: [::core::ffi::c_char; 24] = [0; 24];
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 24]>()
                    as ::core::ffi::c_int,
                &raw mut zTab as *mut ::core::ffi::c_char,
                b"sqlite_stat%d\0".as_ptr() as *const ::core::ffi::c_char,
                i,
            );
            if !sqlite3FindTable(
                    (*pParse).db,
                    &raw mut zTab as *mut ::core::ffi::c_char,
                    zDbName,
                )
                .is_null()
            {
                sqlite3NestedParse(
                    pParse,
                    b"DELETE FROM %Q.%s WHERE %s=%Q\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    zDbName,
                    &raw mut zTab as *mut ::core::ffi::c_char,
                    zType,
                    zName,
                );
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CodeDropTable(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut iDb: ::core::ffi::c_int,
    mut isView: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pTrigger: *mut Trigger = ::core::ptr::null_mut::<Trigger>();
        let mut pDb: *mut Db = (*db).aDb.offset(iDb as isize) as *mut Db;
        v = sqlite3GetVdbe(pParse);
        sqlite3BeginWriteOperation(pParse, 1 as ::core::ffi::c_int, iDb);
        if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
            sqlite3VdbeAddOp0(v, OP_VBegin);
        }
        pTrigger = sqlite3TriggerList(pParse, pTab);
        while !pTrigger.is_null() {
            sqlite3DropTriggerPtr(pParse, pTrigger);
            pTrigger = (*pTrigger).pNext;
        }
        if (*pTab).tabFlags & TF_Autoincrement as u32_0 != 0 {
            sqlite3NestedParse(
                pParse,
                b"DELETE FROM %Q.sqlite_sequence WHERE name=%Q\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pDb).zDbSName,
                (*pTab).zName,
            );
        }
        sqlite3NestedParse(
            pParse,
            b"DELETE FROM %Q.sqlite_master WHERE tbl_name=%Q and type!='trigger'\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*pDb).zDbSName,
            (*pTab).zName,
        );
        if isView == 0 && !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB) {
            destroyTable(pParse, pTab);
        }
        if (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB {
            sqlite3VdbeAddOp4(
                v,
                OP_VDestroy,
                iDb,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                (*pTab).zName,
                0 as ::core::ffi::c_int,
            );
            sqlite3MayAbort(pParse);
        }
        sqlite3VdbeAddOp4(
            v,
            OP_DropTable,
            iDb,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*pTab).zName,
            0 as ::core::ffi::c_int,
        );
        sqlite3ChangeCookie(pParse, iDb);
        sqliteViewResetAll(db, iDb);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ReadOnlyShadowTables(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        if (*db).flags as ::core::ffi::c_ulonglong
            & SQLITE_Defensive as ::core::ffi::c_ulonglong
            != 0 as ::core::ffi::c_ulonglong && (*db).pVtabCtx.is_null()
            && (*db).nVdbeExec == 0 as ::core::ffi::c_int
            && !((*db).nVTrans > 0 as ::core::ffi::c_int && (*db).aVTrans.is_null())
        {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn tableMayNotBeDropped(
    mut db: *mut sqlite3,
    mut pTab: *mut Table,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_strnicmp(
            (*pTab).zName,
            b"sqlite_\0".as_ptr() as *const ::core::ffi::c_char,
            7 as ::core::ffi::c_int,
        ) == 0 as ::core::ffi::c_int
        {
            if sqlite3_strnicmp(
                (*pTab).zName.offset(7 as ::core::ffi::c_int as isize),
                b"stat\0".as_ptr() as *const ::core::ffi::c_char,
                4 as ::core::ffi::c_int,
            ) == 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            if sqlite3_strnicmp(
                (*pTab).zName.offset(7 as ::core::ffi::c_int as isize),
                b"parameters\0".as_ptr() as *const ::core::ffi::c_char,
                10 as ::core::ffi::c_int,
            ) == 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            return 1 as ::core::ffi::c_int;
        }
        if (*pTab).tabFlags & TF_Shadow as u32_0 != 0 as u32_0
            && sqlite3ReadOnlyShadowTables(db) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        if (*pTab).tabFlags & TF_Eponymous as u32_0 != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DropTable(
    mut pParse: *mut Parse,
    mut pName: *mut SrcList,
    mut isView: ::core::ffi::c_int,
    mut noErr: ::core::ffi::c_int,
) {
    unsafe {
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut iDb: ::core::ffi::c_int = 0;
        if !((*db).mallocFailed != 0) {
            if !(sqlite3ReadSchema(pParse) != 0) {
                if noErr != 0 {
                    (*db).suppressErr = (*db).suppressErr.wrapping_add(1);
                }
                pTab = sqlite3LocateTableItem(
                    pParse,
                    isView as u32_0,
                    (&raw mut (*pName).a as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem,
                );
                if noErr != 0 {
                    (*db).suppressErr = (*db).suppressErr.wrapping_sub(1);
                }
                if pTab.is_null() {
                    if noErr != 0 {
                        sqlite3CodeVerifyNamedSchema(
                            pParse,
                            (*(&raw mut (*pName).a as *mut SrcItem)
                                .offset(0 as ::core::ffi::c_int as isize))
                                .u4
                                .zDatabase,
                        );
                        sqlite3ForceNotReadOnly(pParse);
                    }
                } else {
                    iDb = sqlite3SchemaToIndex(db, (*pTab).pSchema);
                    if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB
                        && sqlite3ViewGetColumnNames(pParse, pTab) != 0)
                    {
                        let mut code: ::core::ffi::c_int = 0;
                        let mut zTab: *const ::core::ffi::c_char = if OMIT_TEMPDB == 0
                            && iDb == 1 as ::core::ffi::c_int
                        {
                            LEGACY_TEMP_SCHEMA_TABLE.as_ptr()
                        } else {
                            LEGACY_SCHEMA_TABLE.as_ptr()
                        };
                        let mut zDb: *const ::core::ffi::c_char = (*(*db)
                            .aDb
                            .offset(iDb as isize))
                            .zDbSName;
                        let mut zArg2: *const ::core::ffi::c_char = ::core::ptr::null::<
                            ::core::ffi::c_char,
                        >();
                        if !(sqlite3AuthCheck(
                            pParse,
                            SQLITE_DELETE,
                            zTab,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            zDb,
                        ) != 0)
                        {
                            if isView != 0 {
                                if OMIT_TEMPDB == 0 && iDb == 1 as ::core::ffi::c_int {
                                    code = SQLITE_DROP_TEMP_VIEW;
                                } else {
                                    code = SQLITE_DROP_VIEW;
                                }
                            } else if (*pTab).eTabType as ::core::ffi::c_int
                                == TABTYP_VTAB
                            {
                                code = SQLITE_DROP_VTABLE;
                                zArg2 = (*(*sqlite3GetVTable(db, pTab)).pMod).zName;
                            } else if OMIT_TEMPDB == 0 && iDb == 1 as ::core::ffi::c_int
                            {
                                code = SQLITE_DROP_TEMP_TABLE;
                            } else {
                                code = SQLITE_DROP_TABLE;
                            }
                            if !(sqlite3AuthCheck(
                                pParse,
                                code,
                                (*pTab).zName,
                                zArg2,
                                zDb,
                            ) != 0)
                            {
                                if !(sqlite3AuthCheck(
                                    pParse,
                                    SQLITE_DELETE,
                                    (*pTab).zName,
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    zDb,
                                ) != 0)
                                {
                                    if tableMayNotBeDropped(db, pTab) != 0 {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            b"table %s may not be dropped\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            (*pTab).zName,
                                        );
                                    } else if isView != 0
                                        && !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW)
                                    {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            b"use DROP TABLE to delete table %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            (*pTab).zName,
                                        );
                                    } else if isView == 0
                                        && (*pTab).eTabType as ::core::ffi::c_int == TABTYP_VIEW
                                    {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            b"use DROP VIEW to delete view %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            (*pTab).zName,
                                        );
                                    } else {
                                        v = sqlite3GetVdbe(pParse);
                                        if !v.is_null() {
                                            sqlite3BeginWriteOperation(
                                                pParse,
                                                1 as ::core::ffi::c_int,
                                                iDb,
                                            );
                                            if isView == 0 {
                                                sqlite3ClearStatTables(
                                                    pParse,
                                                    iDb,
                                                    b"tbl\0".as_ptr() as *const ::core::ffi::c_char,
                                                    (*pTab).zName,
                                                );
                                                sqlite3FkDropTable(pParse, pName, pTab);
                                            }
                                            sqlite3CodeDropTable(pParse, pTab, iDb, isView);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        sqlite3SrcListDelete(db, pName);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CreateForeignKey(
    mut pParse: *mut Parse,
    mut pFromCol: *mut ExprList,
    mut pTo: *mut Token,
    mut pToCol: *mut ExprList,
    mut flags: ::core::ffi::c_int,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pFKey: *mut FKey = ::core::ptr::null_mut::<FKey>();
        let mut pNextTo: *mut FKey = ::core::ptr::null_mut::<FKey>();
        let mut p: *mut Table = (*pParse).pNewTable;
        let mut nByte: i64_0 = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut nCol: ::core::ffi::c_int = 0;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if !(p.is_null()
            || (*pParse).eParseMode as ::core::ffi::c_int == PARSE_MODE_DECLARE_VTAB)
        {
            if pFromCol.is_null() {
                let mut iCol: ::core::ffi::c_int = (*p).nCol as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int;
                if iCol < 0 as ::core::ffi::c_int {
                    c2rust_current_block = 11266026347216730296;
                } else if !pToCol.is_null() && (*pToCol).nExpr != 1 as ::core::ffi::c_int
                {
                    sqlite3ErrorMsg(
                        pParse,
                        b"foreign key on %s should reference only one column of table %T\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*(*p).aCol.offset(iCol as isize)).zCnName,
                        pTo,
                    );
                    c2rust_current_block = 11266026347216730296;
                } else {
                    nCol = 1 as ::core::ffi::c_int;
                    c2rust_current_block = 4166486009154926805;
                }
            } else if !pToCol.is_null() && (*pToCol).nExpr != (*pFromCol).nExpr {
                sqlite3ErrorMsg(
                    pParse,
                    b"number of columns in foreign key does not match the number of columns in the referenced table\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                c2rust_current_block = 11266026347216730296;
            } else {
                nCol = (*pFromCol).nExpr;
                c2rust_current_block = 4166486009154926805;
            }
            match c2rust_current_block {
                11266026347216730296 => {}
                _ => {
                    nByte = (64 as usize)
                        .wrapping_add(
                            (nCol as usize)
                                .wrapping_mul(::core::mem::size_of::<sColMap>() as usize),
                        )
                        .wrapping_add((*pTo).n as usize)
                        .wrapping_add(1 as usize) as i64_0;
                    if !pToCol.is_null() {
                        i = 0 as ::core::ffi::c_int;
                        while i < (*pToCol).nExpr {
                            nByte
                                += (sqlite3Strlen30(
                                    (*(&raw mut (*pToCol).a as *mut ExprList_item)
                                        .offset(i as isize))
                                        .zEName,
                                ) + 1 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
                            i += 1;
                        }
                    }
                    pFKey = sqlite3DbMallocZero(db, nByte as u64_0) as *mut FKey;
                    if !pFKey.is_null() {
                        (*pFKey).pFrom = p;
                        (*pFKey).pNextFrom = (*p).u.tab.pFKey;
                        z = (&raw mut (*pFKey).aCol as *mut sColMap)
                            .offset(nCol as isize) as *mut sColMap
                            as *mut ::core::ffi::c_char;
                        (*pFKey).zTo = z;
                        if (*pParse).eParseMode as ::core::ffi::c_int
                            >= PARSE_MODE_RENAME
                        {
                            sqlite3RenameTokenMap(
                                pParse,
                                z as *mut ::core::ffi::c_void,
                                pTo,
                            );
                        }
                        memcpy(
                            z as *mut ::core::ffi::c_void,
                            (*pTo).z as *const ::core::ffi::c_void,
                            (*pTo).n as size_t,
                        );
                        *z.offset((*pTo).n as isize) = 0 as ::core::ffi::c_char;
                        sqlite3Dequote(z);
                        z = z
                            .offset(
                                (*pTo).n.wrapping_add(1 as ::core::ffi::c_uint) as isize,
                            );
                        (*pFKey).nCol = nCol;
                        if pFromCol.is_null() {
                            (*(&raw mut (*pFKey).aCol as *mut sColMap)
                                .offset(0 as ::core::ffi::c_int as isize))
                                .iFrom = (*p).nCol as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int;
                            c2rust_current_block = 16799951812150840583;
                        } else {
                            i = 0 as ::core::ffi::c_int;
                            loop {
                                if !(i < nCol) {
                                    c2rust_current_block = 16799951812150840583;
                                    break;
                                }
                                let mut j: ::core::ffi::c_int = 0;
                                j = 0 as ::core::ffi::c_int;
                                while j < (*p).nCol as ::core::ffi::c_int {
                                    if sqlite3StrICmp(
                                        (*(*p).aCol.offset(j as isize)).zCnName,
                                        (*(&raw mut (*pFromCol).a as *mut ExprList_item)
                                            .offset(i as isize))
                                            .zEName,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        (*(&raw mut (*pFKey).aCol as *mut sColMap)
                                            .offset(i as isize))
                                            .iFrom = j;
                                        break;
                                    } else {
                                        j += 1;
                                    }
                                }
                                if j >= (*p).nCol as ::core::ffi::c_int {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        b"unknown column \"%s\" in foreign key definition\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        (*(&raw mut (*pFromCol).a as *mut ExprList_item)
                                            .offset(i as isize))
                                            .zEName,
                                    );
                                    c2rust_current_block = 11266026347216730296;
                                    break;
                                } else {
                                    if (*pParse).eParseMode as ::core::ffi::c_int
                                        >= PARSE_MODE_RENAME
                                    {
                                        sqlite3RenameTokenRemap(
                                            pParse,
                                            (&raw mut (*pFKey).aCol as *mut sColMap).offset(i as isize)
                                                as *mut sColMap as *const ::core::ffi::c_void,
                                            (*(&raw mut (*pFromCol).a as *mut ExprList_item)
                                                .offset(i as isize))
                                                .zEName as *const ::core::ffi::c_void,
                                        );
                                    }
                                    i += 1;
                                }
                            }
                        }
                        match c2rust_current_block {
                            11266026347216730296 => {}
                            _ => {
                                if !pToCol.is_null() {
                                    i = 0 as ::core::ffi::c_int;
                                    while i < nCol {
                                        let mut n: ::core::ffi::c_int = sqlite3Strlen30(
                                            (*(&raw mut (*pToCol).a as *mut ExprList_item)
                                                .offset(i as isize))
                                                .zEName,
                                        );
                                        let ref mut c2rust_fresh35 = (*(&raw mut (*pFKey).aCol
                                            as *mut sColMap)
                                            .offset(i as isize))
                                            .zCol;
                                        *c2rust_fresh35 = z;
                                        if (*pParse).eParseMode as ::core::ffi::c_int
                                            >= PARSE_MODE_RENAME
                                        {
                                            sqlite3RenameTokenRemap(
                                                pParse,
                                                z as *const ::core::ffi::c_void,
                                                (*(&raw mut (*pToCol).a as *mut ExprList_item)
                                                    .offset(i as isize))
                                                    .zEName as *const ::core::ffi::c_void,
                                            );
                                        }
                                        memcpy(
                                            z as *mut ::core::ffi::c_void,
                                            (*(&raw mut (*pToCol).a as *mut ExprList_item)
                                                .offset(i as isize))
                                                .zEName as *const ::core::ffi::c_void,
                                            n as size_t,
                                        );
                                        *z.offset(n as isize) = 0 as ::core::ffi::c_char;
                                        z = z.offset((n + 1 as ::core::ffi::c_int) as isize);
                                        i += 1;
                                    }
                                }
                                (*pFKey).isDeferred = 0 as u8_0;
                                (*pFKey).aAction[0 as ::core::ffi::c_int as usize] = (flags
                                    & 0xff as ::core::ffi::c_int) as u8_0;
                                (*pFKey).aAction[1 as ::core::ffi::c_int as usize] = (flags
                                    >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                                    as u8_0;
                                pNextTo = sqlite3HashInsert(
                                    &raw mut (*(*p).pSchema).fkeyHash,
                                    (*pFKey).zTo,
                                    pFKey as *mut ::core::ffi::c_void,
                                ) as *mut FKey;
                                if pNextTo == pFKey {
                                    sqlite3OomFault(db);
                                } else {
                                    if !pNextTo.is_null() {
                                        (*pFKey).pNextTo = pNextTo;
                                        (*pNextTo).pPrevTo = pFKey;
                                    }
                                    (*p).u.tab.pFKey = pFKey;
                                    pFKey = ::core::ptr::null_mut::<FKey>();
                                }
                            }
                        }
                    }
                }
            }
        }
        sqlite3DbFree(db, pFKey as *mut ::core::ffi::c_void);
        sqlite3ExprListDelete(db, pFromCol);
        sqlite3ExprListDelete(db, pToCol);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DeferForeignKey(
    mut pParse: *mut Parse,
    mut isDeferred: ::core::ffi::c_int,
) {
    unsafe {
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pFKey: *mut FKey = ::core::ptr::null_mut::<FKey>();
        pTab = (*pParse).pNewTable;
        if pTab.is_null() {
            return;
        }
        if !((*pTab).eTabType as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
            return;
        }
        pFKey = (*pTab).u.tab.pFKey;
        if pFKey.is_null() {
            return;
        }
        (*pFKey).isDeferred = isDeferred as u8_0;
    }
}
unsafe extern "C" fn sqlite3RefillIndex(
    mut pParse: *mut Parse,
    mut pIndex: *mut Index,
    mut memRootPage: ::core::ffi::c_int,
) {
    unsafe {
        let mut pTab: *mut Table = (*pIndex).pTable;
        let c2rust_fresh10 = (*pParse).nTab;
        (*pParse).nTab = (*pParse).nTab + 1;
        let mut iTab: ::core::ffi::c_int = c2rust_fresh10;
        let c2rust_fresh11 = (*pParse).nTab;
        (*pParse).nTab = (*pParse).nTab + 1;
        let mut iIdx: ::core::ffi::c_int = c2rust_fresh11;
        let mut iSorter: ::core::ffi::c_int = 0;
        let mut addr1: ::core::ffi::c_int = 0;
        let mut addr2: ::core::ffi::c_int = 0;
        let mut tnum: Pgno = 0;
        let mut iPartIdxLabel: ::core::ffi::c_int = 0;
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut pKey: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        let mut regRecord: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut iDb: ::core::ffi::c_int = sqlite3SchemaToIndex(db, (*pIndex).pSchema);
        if sqlite3AuthCheck(
            pParse,
            SQLITE_REINDEX,
            (*pIndex).zName,
            ::core::ptr::null::<::core::ffi::c_char>(),
            (*(*db).aDb.offset(iDb as isize)).zDbSName,
        ) != 0
        {
            return;
        }
        sqlite3TableLock(pParse, iDb, (*pTab).tnum, 1 as u8_0, (*pTab).zName);
        v = sqlite3GetVdbe(pParse);
        if v.is_null() {
            return;
        }
        if memRootPage >= 0 as ::core::ffi::c_int {
            tnum = memRootPage as Pgno;
        } else {
            tnum = (*pIndex).tnum;
        }
        pKey = sqlite3KeyInfoOfIndex(pParse, pIndex);
        let c2rust_fresh12 = (*pParse).nTab;
        (*pParse).nTab = (*pParse).nTab + 1;
        iSorter = c2rust_fresh12;
        sqlite3VdbeAddOp4(
            v,
            OP_SorterOpen,
            iSorter,
            0 as ::core::ffi::c_int,
            (*pIndex).nKeyCol as ::core::ffi::c_int,
            sqlite3KeyInfoRef(pKey) as *mut ::core::ffi::c_char,
            P4_KEYINFO,
        );
        sqlite3OpenTable(pParse, iTab, iDb, pTab, OP_OpenRead);
        addr1 = sqlite3VdbeAddOp2(v, OP_Rewind, iTab, 0 as ::core::ffi::c_int);
        regRecord = sqlite3GetTempReg(pParse);
        sqlite3MultiWrite(pParse);
        sqlite3GenerateIndexKey(
            pParse,
            pIndex,
            iTab,
            regRecord,
            0 as ::core::ffi::c_int,
            &raw mut iPartIdxLabel,
            ::core::ptr::null_mut::<Index>(),
            0 as ::core::ffi::c_int,
        );
        sqlite3VdbeAddOp2(v, OP_SorterInsert, iSorter, regRecord);
        sqlite3ResolvePartIdxLabel(pParse, iPartIdxLabel);
        sqlite3VdbeAddOp2(v, OP_Next, iTab, addr1 + 1 as ::core::ffi::c_int);
        sqlite3VdbeJumpHere(v, addr1);
        if memRootPage < 0 as ::core::ffi::c_int {
            sqlite3VdbeAddOp2(v, OP_Clear, tnum as ::core::ffi::c_int, iDb);
        }
        sqlite3VdbeAddOp4(
            v,
            OP_OpenWrite,
            iIdx,
            tnum as ::core::ffi::c_int,
            iDb,
            pKey as *mut ::core::ffi::c_char,
            P4_KEYINFO,
        );
        sqlite3VdbeChangeP5(
            v,
            (OPFLAG_BULKCSR
                | (if memRootPage >= 0 as ::core::ffi::c_int {
                    OPFLAG_P2ISREG
                } else {
                    0 as ::core::ffi::c_int
                })) as u16_0,
        );
        addr1 = sqlite3VdbeAddOp2(v, OP_SorterSort, iSorter, 0 as ::core::ffi::c_int);
        if (*pIndex).onError as ::core::ffi::c_int != OE_None {
            let mut j2: ::core::ffi::c_int = sqlite3VdbeGoto(v, 1 as ::core::ffi::c_int);
            addr2 = sqlite3VdbeCurrentAddr(v);
            sqlite3VdbeAddOp4Int(
                v,
                OP_SorterCompare,
                iSorter,
                j2,
                regRecord,
                (*pIndex).nKeyCol as ::core::ffi::c_int,
            );
            sqlite3UniqueConstraint(pParse, OE_Abort, pIndex);
            sqlite3VdbeJumpHere(v, j2);
        } else {
            sqlite3MayAbort(pParse);
            addr2 = sqlite3VdbeCurrentAddr(v);
        }
        sqlite3VdbeAddOp3(v, OP_SorterData, iSorter, regRecord, iIdx);
        if (*pIndex).bAscKeyBug() == 0 {
            sqlite3VdbeAddOp1(v, OP_SeekEnd, iIdx);
        }
        sqlite3VdbeAddOp2(v, OP_IdxInsert, iIdx, regRecord);
        sqlite3VdbeChangeP5(v, OPFLAG_USESEEKRESULT as u16_0);
        sqlite3ReleaseTempReg(pParse, regRecord);
        sqlite3VdbeAddOp2(v, OP_SorterNext, iSorter, addr2);
        sqlite3VdbeJumpHere(v, addr1);
        sqlite3VdbeAddOp1(v, OP_Close, iTab);
        sqlite3VdbeAddOp1(v, OP_Close, iIdx);
        sqlite3VdbeAddOp1(v, OP_Close, iSorter);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3AllocateIndexObject(
    mut db: *mut sqlite3,
    mut nCol: ::core::ffi::c_int,
    mut nExtra: ::core::ffi::c_int,
    mut ppExtra: *mut *mut ::core::ffi::c_char,
) -> *mut Index {
    unsafe {
        let mut p: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut nByte: i64_0 = 0;
        nByte = ((::core::mem::size_of::<Index>() as usize).wrapping_add(7 as usize)
            & !(7 as ::core::ffi::c_int) as usize)
            .wrapping_add(
                (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                    .wrapping_mul(nCol as usize)
                    .wrapping_add(7 as usize) & !(7 as ::core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (::core::mem::size_of::<LogEst>() as usize)
                    .wrapping_mul((nCol + 1 as ::core::ffi::c_int) as usize)
                    .wrapping_add(
                        (::core::mem::size_of::<i16_0>() as usize)
                            .wrapping_mul(nCol as usize),
                    )
                    .wrapping_add(
                        (::core::mem::size_of::<u8_0>() as usize)
                            .wrapping_mul(nCol as usize),
                    )
                    .wrapping_add(7 as usize) & !(7 as ::core::ffi::c_int) as usize,
            ) as i64_0;
        p = sqlite3DbMallocZero(
            db,
            (nByte as ::core::ffi::c_longlong + nExtra as ::core::ffi::c_longlong)
                as u64_0,
        ) as *mut Index;
        if !p.is_null() {
            let mut pExtra: *mut ::core::ffi::c_char = (p as *mut ::core::ffi::c_char)
                .offset(
                    ((::core::mem::size_of::<Index>() as usize).wrapping_add(7 as usize)
                        & !(7 as ::core::ffi::c_int) as usize) as isize,
                );
            (*p).azColl = pExtra as *mut *const ::core::ffi::c_char;
            pExtra = pExtra
                .offset(
                    ((::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        .wrapping_mul(nCol as usize)
                        .wrapping_add(7 as usize) & !(7 as ::core::ffi::c_int) as usize)
                        as isize,
                );
            (*p).aiRowLogEst = pExtra as *mut LogEst;
            pExtra = pExtra
                .offset(
                    (::core::mem::size_of::<LogEst>() as usize)
                        .wrapping_mul((nCol + 1 as ::core::ffi::c_int) as usize) as isize,
                );
            (*p).aiColumn = pExtra as *mut i16_0;
            pExtra = pExtra
                .offset(
                    (::core::mem::size_of::<i16_0>() as usize)
                        .wrapping_mul(nCol as usize) as isize,
                );
            (*p).aSortOrder = pExtra as *mut u8_0;
            (*p).nColumn = nCol as u16_0;
            (*p).nKeyCol = (nCol - 1 as ::core::ffi::c_int) as u16_0;
            *ppExtra = (p as *mut ::core::ffi::c_char).offset(nByte as isize);
        }
        return p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3HasExplicitNulls(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
) -> ::core::ffi::c_int {
    unsafe {
        if !pList.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pList).nExpr {
                if (*(&raw mut (*pList).a as *mut ExprList_item).offset(i as isize))
                    .fg
                    .bNulls() != 0
                {
                    let mut sf: u8_0 = (*(&raw mut (*pList).a as *mut ExprList_item)
                        .offset(i as isize))
                        .fg
                        .sortFlags;
                    sqlite3ErrorMsg(
                        pParse,
                        b"unsupported use of NULLS %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        if sf as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            || sf as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                        {
                            b"FIRST\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"LAST\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                    return 1 as ::core::ffi::c_int;
                }
                i += 1;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CreateIndex(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
    mut pTblName: *mut SrcList,
    mut pList: *mut ExprList,
    mut onError: ::core::ffi::c_int,
    mut pStart: *mut Token,
    mut pPIWhere: *mut Expr,
    mut sortOrder: ::core::ffi::c_int,
    mut ifNotExist: ::core::ffi::c_int,
    mut idxType: u8_0,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pIndex: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nName: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut sFix: DbFixer = DbFixer {
            pParse: ::core::ptr::null_mut::<Parse>(),
            w: Walker {
                pParse: ::core::ptr::null_mut::<Parse>(),
                xExprCallback: None,
                xSelectCallback: None,
                xSelectCallback2: None,
                walkerDepth: 0,
                eCode: 0,
                mWFlags: 0,
                u: C2Rust_Unnamed_22 {
                    pNC: ::core::ptr::null_mut::<NameContext>()

                }

            },
            pSchema: ::core::ptr::null_mut::<Schema>(),
            bTemp: 0,
            zDb: ::core::ptr::null::<::core::ffi::c_char>(),
            zType: ::core::ptr::null::<::core::ffi::c_char>(),
            pName: ::core::ptr::null::<Token>()

        };
        let mut sortOrderMask: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pDb: *mut Db = ::core::ptr::null_mut::<Db>();
        let mut iDb: ::core::ffi::c_int = 0;
        let mut pName: *mut Token = ::core::ptr::null_mut::<Token>();
        let mut pListItem: *mut ExprList_item = ::core::ptr::null_mut::<ExprList_item>();
        let mut nExtra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nExtraCol: ::core::ffi::c_int = 0;
        let mut zExtra: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pPk: *mut Index = ::core::ptr::null_mut::<Index>();
        if !((*pParse).nErr != 0) {
            if !((*pParse).eParseMode as ::core::ffi::c_int == PARSE_MODE_DECLARE_VTAB
                && idxType as ::core::ffi::c_int != SQLITE_IDXTYPE_PRIMARYKEY)
            {
                if !(SQLITE_OK != sqlite3ReadSchema(pParse)) {
                    if !(sqlite3HasExplicitNulls(pParse, pList) != 0) {
                        if !pTblName.is_null() {
                            iDb = sqlite3TwoPartName(
                                pParse,
                                pName1,
                                pName2,
                                &raw mut pName,
                            );
                            if iDb < 0 as ::core::ffi::c_int {
                                c2rust_current_block = 17595362031169221896;
                            } else {
                                if (*db).init.busy == 0 {
                                    pTab = sqlite3SrcListLookup(pParse, pTblName);
                                    if (*pName2).n == 0 as ::core::ffi::c_uint
                                        && !pTab.is_null()
                                        && (*pTab).pSchema
                                            == (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize))
                                                .pSchema
                                    {
                                        iDb = 1 as ::core::ffi::c_int;
                                    }
                                }
                                sqlite3FixInit(
                                    &raw mut sFix,
                                    pParse,
                                    iDb,
                                    b"index\0".as_ptr() as *const ::core::ffi::c_char,
                                    pName,
                                );
                                sqlite3FixSrcList(&raw mut sFix, pTblName) != 0;
                                pTab = sqlite3LocateTableItem(
                                    pParse,
                                    0 as u32_0,
                                    (&raw mut (*pTblName).a as *mut SrcItem)
                                        .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem,
                                );
                                if pTab.is_null() {
                                    c2rust_current_block = 17595362031169221896;
                                } else if iDb == 1 as ::core::ffi::c_int
                                    && (*(*db).aDb.offset(iDb as isize)).pSchema
                                        != (*pTab).pSchema
                                {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        b"cannot create a TEMP index on non-TEMP table \"%s\"\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        (*pTab).zName,
                                    );
                                    c2rust_current_block = 17595362031169221896;
                                } else {
                                    if !((*pTab).tabFlags & TF_WithoutRowid as u32_0
                                        == 0 as u32_0)
                                    {
                                        pPk = sqlite3PrimaryKeyIndex(pTab);
                                    }
                                    c2rust_current_block = 7333393191927787629;
                                }
                            }
                        } else {
                            pTab = (*pParse).pNewTable;
                            if pTab.is_null() {
                                c2rust_current_block = 17595362031169221896;
                            } else {
                                iDb = sqlite3SchemaToIndex(db, (*pTab).pSchema);
                                c2rust_current_block = 7333393191927787629;
                            }
                        }
                        match c2rust_current_block {
                            17595362031169221896 => {}
                            _ => {
                                pDb = (*db).aDb.offset(iDb as isize) as *mut Db;
                                if sqlite3_strnicmp(
                                    (*pTab).zName,
                                    b"sqlite_\0".as_ptr() as *const ::core::ffi::c_char,
                                    7 as ::core::ffi::c_int,
                                ) == 0 as ::core::ffi::c_int
                                    && (*db).init.busy as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int && !pTblName.is_null()
                                {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        b"table %s may not be indexed\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        (*pTab).zName,
                                    );
                                } else if (*pTab).eTabType as ::core::ffi::c_int
                                    == TABTYP_VIEW
                                {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        b"views may not be indexed\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                } else if (*pTab).eTabType as ::core::ffi::c_int
                                    == TABTYP_VTAB
                                {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        b"virtual tables may not be indexed\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                } else {
                                    if !pName.is_null() {
                                        zName = sqlite3NameFromToken(db, pName);
                                        if zName.is_null() {
                                            c2rust_current_block = 17595362031169221896;
                                        } else if SQLITE_OK
                                            != sqlite3CheckObjectName(
                                                pParse,
                                                zName,
                                                b"index\0".as_ptr() as *const ::core::ffi::c_char,
                                                (*pTab).zName,
                                            )
                                        {
                                            c2rust_current_block = 17595362031169221896;
                                        } else if !((*pParse).eParseMode as ::core::ffi::c_int
                                            >= PARSE_MODE_RENAME)
                                        {
                                            if (*db).init.busy == 0 {
                                                if !sqlite3FindTable(db, zName, (*pDb).zDbSName).is_null() {
                                                    sqlite3ErrorMsg(
                                                        pParse,
                                                        b"there is already a table named %s\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        zName,
                                                    );
                                                    c2rust_current_block = 17595362031169221896;
                                                } else {
                                                    c2rust_current_block = 5891011138178424807;
                                                }
                                            } else {
                                                c2rust_current_block = 5891011138178424807;
                                            }
                                            match c2rust_current_block {
                                                17595362031169221896 => {}
                                                _ => {
                                                    if !sqlite3FindIndex(db, zName, (*pDb).zDbSName).is_null() {
                                                        if ifNotExist == 0 {
                                                            sqlite3ErrorMsg(
                                                                pParse,
                                                                b"index %s already exists\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                zName,
                                                            );
                                                        } else {
                                                            sqlite3CodeVerifySchema(pParse, iDb);
                                                            sqlite3ForceNotReadOnly(pParse);
                                                        }
                                                        c2rust_current_block = 17595362031169221896;
                                                    } else {
                                                        c2rust_current_block = 15970011996474399071;
                                                    }
                                                }
                                            }
                                        } else {
                                            c2rust_current_block = 15970011996474399071;
                                        }
                                    } else {
                                        let mut n: ::core::ffi::c_int = 0;
                                        let mut pLoop: *mut Index = ::core::ptr::null_mut::<
                                            Index,
                                        >();
                                        pLoop = (*pTab).pIndex;
                                        n = 1 as ::core::ffi::c_int;
                                        while !pLoop.is_null() {
                                            pLoop = (*pLoop).pNext;
                                            n += 1;
                                        }
                                        zName = sqlite3MPrintf(
                                            db,
                                            b"sqlite_autoindex_%s_%d\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            (*pTab).zName,
                                            n,
                                        );
                                        if zName.is_null() {
                                            c2rust_current_block = 17595362031169221896;
                                        } else {
                                            if (*pParse).eParseMode as ::core::ffi::c_int
                                                != PARSE_MODE_NORMAL
                                            {
                                                let ref mut c2rust_fresh6 = *zName
                                                    .offset(7 as ::core::ffi::c_int as isize);
                                                *c2rust_fresh6 += 1;
                                            }
                                            c2rust_current_block = 15970011996474399071;
                                        }
                                    }
                                    match c2rust_current_block {
                                        17595362031169221896 => {}
                                        _ => {
                                            if !((*pParse).eParseMode as ::core::ffi::c_int
                                                >= PARSE_MODE_RENAME)
                                            {
                                                let mut zDb: *const ::core::ffi::c_char = (*pDb).zDbSName;
                                                if sqlite3AuthCheck(
                                                    pParse,
                                                    SQLITE_INSERT,
                                                    if OMIT_TEMPDB == 0 && iDb == 1 as ::core::ffi::c_int {
                                                        LEGACY_TEMP_SCHEMA_TABLE.as_ptr()
                                                    } else {
                                                        LEGACY_SCHEMA_TABLE.as_ptr()
                                                    },
                                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                                    zDb,
                                                ) != 0
                                                {
                                                    c2rust_current_block = 17595362031169221896;
                                                } else {
                                                    i = SQLITE_CREATE_INDEX;
                                                    if OMIT_TEMPDB == 0 && iDb == 1 as ::core::ffi::c_int {
                                                        i = SQLITE_CREATE_TEMP_INDEX;
                                                    }
                                                    if sqlite3AuthCheck(pParse, i, zName, (*pTab).zName, zDb)
                                                        != 0
                                                    {
                                                        c2rust_current_block = 17595362031169221896;
                                                    } else {
                                                        c2rust_current_block = 7385833325316299293;
                                                    }
                                                }
                                            } else {
                                                c2rust_current_block = 7385833325316299293;
                                            }
                                            match c2rust_current_block {
                                                17595362031169221896 => {}
                                                _ => {
                                                    if pList.is_null() {
                                                        let mut prevCol: Token = Token {
                                                            z: ::core::ptr::null::<::core::ffi::c_char>(),
                                                            n: 0

                                                        };
                                                        let mut pCol: *mut Column = (*pTab)
                                                            .aCol
                                                            .offset(
                                                                ((*pTab).nCol as ::core::ffi::c_int
                                                                    - 1 as ::core::ffi::c_int) as isize,
                                                            ) as *mut Column;
                                                        (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                                                            | COLFLAG_UNIQUE) as u16_0;
                                                        sqlite3TokenInit(&raw mut prevCol, (*pCol).zCnName);
                                                        pList = sqlite3ExprListAppend(
                                                            pParse,
                                                            ::core::ptr::null_mut::<ExprList>(),
                                                            sqlite3ExprAlloc(
                                                                db,
                                                                TK_ID,
                                                                &raw mut prevCol,
                                                                0 as ::core::ffi::c_int,
                                                            ),
                                                        );
                                                        if pList.is_null() {
                                                            c2rust_current_block = 17595362031169221896;
                                                        } else {
                                                            sqlite3ExprListSetSortOrder(
                                                                pList,
                                                                sortOrder,
                                                                SQLITE_SO_UNDEFINED,
                                                            );
                                                            c2rust_current_block = 17167606947040001567;
                                                        }
                                                    } else {
                                                        sqlite3ExprListCheckLength(
                                                            pParse,
                                                            pList,
                                                            b"index\0".as_ptr() as *const ::core::ffi::c_char,
                                                        );
                                                        if (*pParse).nErr != 0 {
                                                            c2rust_current_block = 17595362031169221896;
                                                        } else {
                                                            c2rust_current_block = 17167606947040001567;
                                                        }
                                                    }
                                                    match c2rust_current_block {
                                                        17595362031169221896 => {}
                                                        _ => {
                                                            i = 0 as ::core::ffi::c_int;
                                                            while i < (*pList).nExpr {
                                                                let mut pExpr: *mut Expr = (*(&raw mut (*pList).a
                                                                    as *mut ExprList_item)
                                                                    .offset(i as isize))
                                                                    .pExpr;
                                                                if (*pExpr).op as ::core::ffi::c_int == TK_COLLATE {
                                                                    nExtra
                                                                        += 1 as ::core::ffi::c_int
                                                                            + sqlite3Strlen30((*pExpr).u.zToken);
                                                                }
                                                                i += 1;
                                                            }
                                                            nName = sqlite3Strlen30(zName);
                                                            nExtraCol = if !pPk.is_null() {
                                                                (*pPk).nKeyCol as ::core::ffi::c_int
                                                            } else {
                                                                1 as ::core::ffi::c_int
                                                            };
                                                            pIndex = sqlite3AllocateIndexObject(
                                                                db,
                                                                (*pList).nExpr + nExtraCol,
                                                                nName + nExtra + 1 as ::core::ffi::c_int,
                                                                &raw mut zExtra,
                                                            );
                                                            if !((*db).mallocFailed != 0) {
                                                                (*pIndex).zName = zExtra;
                                                                zExtra = zExtra
                                                                    .offset((nName + 1 as ::core::ffi::c_int) as isize);
                                                                memcpy(
                                                                    (*pIndex).zName as *mut ::core::ffi::c_void,
                                                                    zName as *const ::core::ffi::c_void,
                                                                    (nName + 1 as ::core::ffi::c_int) as size_t,
                                                                );
                                                                (*pIndex).pTable = pTab;
                                                                (*pIndex).onError = onError as u8_0;
                                                                (*pIndex)
                                                                    .set_uniqNotNull(
                                                                        (onError != OE_None) as ::core::ffi::c_int
                                                                            as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                    );
                                                                (*pIndex)
                                                                    .set_idxType(
                                                                        idxType as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                    );
                                                                (*pIndex).pSchema = (*(*db).aDb.offset(iDb as isize))
                                                                    .pSchema;
                                                                (*pIndex).nKeyCol = (*pList).nExpr as u16_0;
                                                                if !pPIWhere.is_null() {
                                                                    sqlite3ResolveSelfReference(
                                                                        pParse,
                                                                        pTab,
                                                                        NC_PartIdx,
                                                                        pPIWhere,
                                                                        ::core::ptr::null_mut::<ExprList>(),
                                                                    );
                                                                    (*pIndex).pPartIdxWhere = pPIWhere;
                                                                    pPIWhere = ::core::ptr::null_mut::<Expr>();
                                                                }
                                                                if (*(*pDb).pSchema).file_format as ::core::ffi::c_int
                                                                    >= 4 as ::core::ffi::c_int
                                                                {
                                                                    sortOrderMask = -1 as ::core::ffi::c_int;
                                                                } else {
                                                                    sortOrderMask = 0 as ::core::ffi::c_int;
                                                                }
                                                                pListItem = &raw mut (*pList).a as *mut ExprList_item
                                                                    as *mut ExprList_item;
                                                                if (*pParse).eParseMode as ::core::ffi::c_int
                                                                    >= PARSE_MODE_RENAME
                                                                {
                                                                    (*pIndex).aColExpr = pList;
                                                                    pList = ::core::ptr::null_mut::<ExprList>();
                                                                }
                                                                i = 0 as ::core::ffi::c_int;
                                                                loop {
                                                                    if !(i < (*pIndex).nKeyCol as ::core::ffi::c_int) {
                                                                        c2rust_current_block = 2838755337219234678;
                                                                        break;
                                                                    }
                                                                    let mut pCExpr: *mut Expr = ::core::ptr::null_mut::<Expr>();
                                                                    let mut requestedSortOrder: ::core::ffi::c_int = 0;
                                                                    let mut zColl: *const ::core::ffi::c_char = ::core::ptr::null::<
                                                                        ::core::ffi::c_char,
                                                                    >();
                                                                    sqlite3StringToId((*pListItem).pExpr);
                                                                    sqlite3ResolveSelfReference(
                                                                        pParse,
                                                                        pTab,
                                                                        NC_IdxExpr,
                                                                        (*pListItem).pExpr,
                                                                        ::core::ptr::null_mut::<ExprList>(),
                                                                    );
                                                                    if (*pParse).nErr != 0 {
                                                                        c2rust_current_block = 17595362031169221896;
                                                                        break;
                                                                    }
                                                                    pCExpr = sqlite3ExprSkipCollate((*pListItem).pExpr);
                                                                    if (*pCExpr).op as ::core::ffi::c_int != TK_COLUMN {
                                                                        if pTab == (*pParse).pNewTable {
                                                                            sqlite3ErrorMsg(
                                                                                pParse,
                                                                                b"expressions prohibited in PRIMARY KEY and UNIQUE constraints\0"
                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                            );
                                                                            c2rust_current_block = 17595362031169221896;
                                                                            break;
                                                                        } else {
                                                                            if (*pIndex).aColExpr.is_null() {
                                                                                (*pIndex).aColExpr = pList;
                                                                                pList = ::core::ptr::null_mut::<ExprList>();
                                                                            }
                                                                            j = XN_EXPR;
                                                                            *(*pIndex).aiColumn.offset(i as isize) = XN_EXPR as i16_0;
                                                                            (*pIndex)
                                                                                .set_uniqNotNull(
                                                                                    0 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                                );
                                                                            (*pIndex)
                                                                                .set_bHasExpr(
                                                                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                                );
                                                                        }
                                                                    } else {
                                                                        j = (*pCExpr).iColumn as ::core::ffi::c_int;
                                                                        if j < 0 as ::core::ffi::c_int {
                                                                            j = (*pTab).iPKey as ::core::ffi::c_int;
                                                                        } else {
                                                                            if (*(*pTab).aCol.offset(j as isize)).notNull()
                                                                                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                                                            {
                                                                                (*pIndex)
                                                                                    .set_uniqNotNull(
                                                                                        0 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                                    );
                                                                            }
                                                                            if (*(*pTab).aCol.offset(j as isize)).colFlags
                                                                                as ::core::ffi::c_int & COLFLAG_VIRTUAL != 0
                                                                            {
                                                                                (*pIndex)
                                                                                    .set_bHasVCol(
                                                                                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                                    );
                                                                                (*pIndex)
                                                                                    .set_bHasExpr(
                                                                                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                                    );
                                                                            }
                                                                        }
                                                                        *(*pIndex).aiColumn.offset(i as isize) = j as i16_0;
                                                                    }
                                                                    zColl = ::core::ptr::null::<::core::ffi::c_char>();
                                                                    if (*(*pListItem).pExpr).op as ::core::ffi::c_int
                                                                        == TK_COLLATE
                                                                    {
                                                                        let mut nColl: ::core::ffi::c_int = 0;
                                                                        zColl = (*(*pListItem).pExpr).u.zToken;
                                                                        nColl = sqlite3Strlen30(zColl) + 1 as ::core::ffi::c_int;
                                                                        memcpy(
                                                                            zExtra as *mut ::core::ffi::c_void,
                                                                            zColl as *const ::core::ffi::c_void,
                                                                            nColl as size_t,
                                                                        );
                                                                        zColl = zExtra;
                                                                        zExtra = zExtra.offset(nColl as isize);
                                                                        nExtra -= nColl;
                                                                    } else if j >= 0 as ::core::ffi::c_int {
                                                                        zColl = sqlite3ColumnColl(
                                                                            (*pTab).aCol.offset(j as isize) as *mut Column,
                                                                        );
                                                                    }
                                                                    if zColl.is_null() {
                                                                        zColl = &raw const sqlite3StrBINARY
                                                                            as *const ::core::ffi::c_char;
                                                                    }
                                                                    if (*db).init.busy == 0
                                                                        && sqlite3LocateCollSeq(pParse, zColl).is_null()
                                                                    {
                                                                        c2rust_current_block = 17595362031169221896;
                                                                        break;
                                                                    }
                                                                    let ref mut c2rust_fresh7 = *(*pIndex)
                                                                        .azColl
                                                                        .offset(i as isize);
                                                                    *c2rust_fresh7 = zColl;
                                                                    requestedSortOrder = (*pListItem).fg.sortFlags
                                                                        as ::core::ffi::c_int & sortOrderMask;
                                                                    *(*pIndex).aSortOrder.offset(i as isize) = requestedSortOrder
                                                                        as u8_0;
                                                                    i += 1;
                                                                    pListItem = pListItem.offset(1);
                                                                }
                                                                match c2rust_current_block {
                                                                    17595362031169221896 => {}
                                                                    _ => {
                                                                        if !pPk.is_null() {
                                                                            j = 0 as ::core::ffi::c_int;
                                                                            while j < (*pPk).nKeyCol as ::core::ffi::c_int {
                                                                                let mut x: ::core::ffi::c_int = *(*pPk)
                                                                                    .aiColumn
                                                                                    .offset(j as isize) as ::core::ffi::c_int;
                                                                                if isDupColumn(
                                                                                    pIndex,
                                                                                    (*pIndex).nKeyCol as ::core::ffi::c_int,
                                                                                    pPk,
                                                                                    j,
                                                                                ) != 0
                                                                                {
                                                                                    (*pIndex).nColumn = (*pIndex).nColumn.wrapping_sub(1);
                                                                                } else {
                                                                                    *(*pIndex).aiColumn.offset(i as isize) = x as i16_0;
                                                                                    let ref mut c2rust_fresh8 = *(*pIndex)
                                                                                        .azColl
                                                                                        .offset(i as isize);
                                                                                    *c2rust_fresh8 = *(*pPk).azColl.offset(j as isize);
                                                                                    *(*pIndex).aSortOrder.offset(i as isize) = *(*pPk)
                                                                                        .aSortOrder
                                                                                        .offset(j as isize);
                                                                                    i += 1;
                                                                                }
                                                                                j += 1;
                                                                            }
                                                                        } else {
                                                                            *(*pIndex).aiColumn.offset(i as isize) = XN_ROWID as i16_0;
                                                                            let ref mut c2rust_fresh9 = *(*pIndex)
                                                                                .azColl
                                                                                .offset(i as isize);
                                                                            *c2rust_fresh9 = &raw const sqlite3StrBINARY
                                                                                as *const ::core::ffi::c_char;
                                                                        }
                                                                        sqlite3DefaultRowEst(pIndex);
                                                                        if (*pParse).pNewTable.is_null() {
                                                                            estimateIndexWidth(pIndex);
                                                                        }
                                                                        recomputeColumnsNotIndexed(pIndex);
                                                                        if !pTblName.is_null()
                                                                            && (*pIndex).nColumn as ::core::ffi::c_int
                                                                                >= (*pTab).nCol as ::core::ffi::c_int
                                                                        {
                                                                            (*pIndex)
                                                                                .set_isCovering(
                                                                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                                );
                                                                            j = 0 as ::core::ffi::c_int;
                                                                            while j < (*pTab).nCol as ::core::ffi::c_int {
                                                                                if !(j == (*pTab).iPKey as ::core::ffi::c_int) {
                                                                                    if !(sqlite3TableColumnToIndex(pIndex, j)
                                                                                        >= 0 as ::core::ffi::c_int)
                                                                                    {
                                                                                        (*pIndex)
                                                                                            .set_isCovering(
                                                                                                0 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                                            );
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                j += 1;
                                                                            }
                                                                        }
                                                                        if pTab == (*pParse).pNewTable {
                                                                            let mut pIdx: *mut Index = ::core::ptr::null_mut::<Index>();
                                                                            pIdx = (*pTab).pIndex;
                                                                            loop {
                                                                                if pIdx.is_null() {
                                                                                    c2rust_current_block = 6091595930016798176;
                                                                                    break;
                                                                                }
                                                                                let mut k: ::core::ffi::c_int = 0;
                                                                                if !((*pIdx).nKeyCol as ::core::ffi::c_int
                                                                                    != (*pIndex).nKeyCol as ::core::ffi::c_int)
                                                                                {
                                                                                    k = 0 as ::core::ffi::c_int;
                                                                                    while k < (*pIdx).nKeyCol as ::core::ffi::c_int {
                                                                                        let mut z1: *const ::core::ffi::c_char = ::core::ptr::null::<
                                                                                            ::core::ffi::c_char,
                                                                                        >();
                                                                                        let mut z2: *const ::core::ffi::c_char = ::core::ptr::null::<
                                                                                            ::core::ffi::c_char,
                                                                                        >();
                                                                                        if *(*pIdx).aiColumn.offset(k as isize)
                                                                                            as ::core::ffi::c_int
                                                                                            != *(*pIndex).aiColumn.offset(k as isize)
                                                                                                as ::core::ffi::c_int
                                                                                        {
                                                                                            break;
                                                                                        }
                                                                                        z1 = *(*pIdx).azColl.offset(k as isize);
                                                                                        z2 = *(*pIndex).azColl.offset(k as isize);
                                                                                        if sqlite3StrICmp(z1, z2) != 0 {
                                                                                            break;
                                                                                        }
                                                                                        k += 1;
                                                                                    }
                                                                                    if k == (*pIdx).nKeyCol as ::core::ffi::c_int {
                                                                                        if (*pIdx).onError as ::core::ffi::c_int
                                                                                            != (*pIndex).onError as ::core::ffi::c_int
                                                                                        {
                                                                                            if !((*pIdx).onError as ::core::ffi::c_int == OE_Default
                                                                                                || (*pIndex).onError as ::core::ffi::c_int == OE_Default)
                                                                                            {
                                                                                                sqlite3ErrorMsg(
                                                                                                    pParse,
                                                                                                    b"conflicting ON CONFLICT clauses specified\0".as_ptr()
                                                                                                        as *const ::core::ffi::c_char,
                                                                                                    0 as ::core::ffi::c_int,
                                                                                                );
                                                                                            }
                                                                                            if (*pIdx).onError as ::core::ffi::c_int == OE_Default {
                                                                                                (*pIdx).onError = (*pIndex).onError;
                                                                                            }
                                                                                        }
                                                                                        if idxType as ::core::ffi::c_int
                                                                                            == SQLITE_IDXTYPE_PRIMARYKEY
                                                                                        {
                                                                                            (*pIdx)
                                                                                                .set_idxType(
                                                                                                    idxType as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                                                                                );
                                                                                        }
                                                                                        if (*pParse).eParseMode as ::core::ffi::c_int
                                                                                            >= PARSE_MODE_RENAME
                                                                                        {
                                                                                            (*pIndex).pNext = (*pParse).pNewIndex;
                                                                                            (*pParse).pNewIndex = pIndex;
                                                                                            pIndex = ::core::ptr::null_mut::<Index>();
                                                                                        }
                                                                                        c2rust_current_block = 17595362031169221896;
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                pIdx = (*pIdx).pNext;
                                                                            }
                                                                        } else {
                                                                            c2rust_current_block = 6091595930016798176;
                                                                        }
                                                                        match c2rust_current_block {
                                                                            17595362031169221896 => {}
                                                                            _ => {
                                                                                if !((*pParse).eParseMode as ::core::ffi::c_int
                                                                                    >= PARSE_MODE_RENAME)
                                                                                {
                                                                                    if (*db).init.busy != 0 {
                                                                                        let mut p: *mut Index = ::core::ptr::null_mut::<Index>();
                                                                                        if !pTblName.is_null() {
                                                                                            (*pIndex).tnum = (*db).init.newTnum;
                                                                                            if sqlite3IndexHasDuplicateRootPage(pIndex) != 0 {
                                                                                                sqlite3ErrorMsg(
                                                                                                    pParse,
                                                                                                    b"invalid rootpage\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                );
                                                                                                (*pParse).rc = sqlite3CorruptError(
                                                                                                    4391 as ::core::ffi::c_int,
                                                                                                );
                                                                                                c2rust_current_block = 17595362031169221896;
                                                                                            } else {
                                                                                                c2rust_current_block = 10490607306284298299;
                                                                                            }
                                                                                        } else {
                                                                                            c2rust_current_block = 10490607306284298299;
                                                                                        }
                                                                                        match c2rust_current_block {
                                                                                            17595362031169221896 => {}
                                                                                            _ => {
                                                                                                p = sqlite3HashInsert(
                                                                                                    &raw mut (*(*pIndex).pSchema).idxHash,
                                                                                                    (*pIndex).zName,
                                                                                                    pIndex as *mut ::core::ffi::c_void,
                                                                                                ) as *mut Index;
                                                                                                if !p.is_null() {
                                                                                                    sqlite3OomFault(db);
                                                                                                    c2rust_current_block = 17595362031169221896;
                                                                                                } else {
                                                                                                    (*db).mDbFlags = ((*db).mDbFlags as ::core::ffi::c_uint
                                                                                                        | DBFLAG_SchemaChange as ::core::ffi::c_uint) as u32_0;
                                                                                                    c2rust_current_block = 2942740671786505091;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else if (*pTab).tabFlags & TF_WithoutRowid as u32_0
                                                                                        == 0 as u32_0 || !pTblName.is_null()
                                                                                    {
                                                                                        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
                                                                                        let mut zStmt: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                                                                            ::core::ffi::c_char,
                                                                                        >();
                                                                                        (*pParse).nMem += 1;
                                                                                        let mut iMem: ::core::ffi::c_int = (*pParse).nMem;
                                                                                        v = sqlite3GetVdbe(pParse);
                                                                                        if v.is_null() {
                                                                                            c2rust_current_block = 17595362031169221896;
                                                                                        } else {
                                                                                            sqlite3BeginWriteOperation(
                                                                                                pParse,
                                                                                                1 as ::core::ffi::c_int,
                                                                                                iDb,
                                                                                            );
                                                                                            (*pIndex).tnum = sqlite3VdbeAddOp0(v, OP_Noop) as Pgno;
                                                                                            sqlite3VdbeAddOp3(
                                                                                                v,
                                                                                                OP_CreateBtree,
                                                                                                iDb,
                                                                                                iMem,
                                                                                                BTREE_BLOBKEY,
                                                                                            );
                                                                                            if !pStart.is_null() {
                                                                                                let mut n_0: ::core::ffi::c_int = ((*pParse)
                                                                                                    .sLastToken
                                                                                                    .z
                                                                                                    .offset_from((*pName).z) as ::core::ffi::c_long
                                                                                                    as ::core::ffi::c_int as ::core::ffi::c_uint)
                                                                                                    .wrapping_add((*pParse).sLastToken.n) as ::core::ffi::c_int;
                                                                                                if *(*pName)
                                                                                                    .z
                                                                                                    .offset((n_0 - 1 as ::core::ffi::c_int) as isize)
                                                                                                    as ::core::ffi::c_int == ';' as i32
                                                                                                {
                                                                                                    n_0 -= 1;
                                                                                                }
                                                                                                zStmt = sqlite3MPrintf(
                                                                                                    db,
                                                                                                    b"CREATE%s INDEX %.*s\0".as_ptr()
                                                                                                        as *const ::core::ffi::c_char,
                                                                                                    if onError == OE_None {
                                                                                                        b"\0".as_ptr() as *const ::core::ffi::c_char
                                                                                                    } else {
                                                                                                        b" UNIQUE\0".as_ptr() as *const ::core::ffi::c_char
                                                                                                    },
                                                                                                    n_0,
                                                                                                    (*pName).z,
                                                                                                );
                                                                                            } else {
                                                                                                zStmt = ::core::ptr::null_mut::<::core::ffi::c_char>();
                                                                                            }
                                                                                            sqlite3NestedParse(
                                                                                                pParse,
                                                                                                b"INSERT INTO %Q.sqlite_master VALUES('index',%Q,%Q,#%d,%Q);\0"
                                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                                (*(*db).aDb.offset(iDb as isize)).zDbSName,
                                                                                                (*pIndex).zName,
                                                                                                (*pTab).zName,
                                                                                                iMem,
                                                                                                zStmt,
                                                                                            );
                                                                                            sqlite3DbFree(db, zStmt as *mut ::core::ffi::c_void);
                                                                                            if !pTblName.is_null() {
                                                                                                sqlite3RefillIndex(pParse, pIndex, iMem);
                                                                                                sqlite3ChangeCookie(pParse, iDb);
                                                                                                sqlite3VdbeAddParseSchemaOp(
                                                                                                    v,
                                                                                                    iDb,
                                                                                                    sqlite3MPrintf(
                                                                                                        db,
                                                                                                        b"name='%q' AND type='index'\0".as_ptr()
                                                                                                            as *const ::core::ffi::c_char,
                                                                                                        (*pIndex).zName,
                                                                                                    ),
                                                                                                    0 as u16_0,
                                                                                                );
                                                                                                sqlite3VdbeAddOp2(
                                                                                                    v,
                                                                                                    OP_Expire,
                                                                                                    0 as ::core::ffi::c_int,
                                                                                                    1 as ::core::ffi::c_int,
                                                                                                );
                                                                                            }
                                                                                            sqlite3VdbeJumpHere(
                                                                                                v,
                                                                                                (*pIndex).tnum as ::core::ffi::c_int,
                                                                                            );
                                                                                            c2rust_current_block = 2942740671786505091;
                                                                                        }
                                                                                    } else {
                                                                                        c2rust_current_block = 2942740671786505091;
                                                                                    }
                                                                                } else {
                                                                                    c2rust_current_block = 2942740671786505091;
                                                                                }
                                                                                match c2rust_current_block {
                                                                                    17595362031169221896 => {}
                                                                                    _ => {
                                                                                        if (*db).init.busy as ::core::ffi::c_int != 0
                                                                                            || pTblName.is_null()
                                                                                        {
                                                                                            (*pIndex).pNext = (*pTab).pIndex;
                                                                                            (*pTab).pIndex = pIndex;
                                                                                            pIndex = ::core::ptr::null_mut::<Index>();
                                                                                        } else if (*pParse).eParseMode as ::core::ffi::c_int
                                                                                            >= PARSE_MODE_RENAME
                                                                                        {
                                                                                            (*pParse).pNewIndex = pIndex;
                                                                                            pIndex = ::core::ptr::null_mut::<Index>();
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
        }
        if !pIndex.is_null() {
            sqlite3FreeIndex(db, pIndex);
        }
        if !pTab.is_null() {
            let mut ppFrom: *mut *mut Index = ::core::ptr::null_mut::<*mut Index>();
            let mut pThis: *mut Index = ::core::ptr::null_mut::<Index>();
            ppFrom = &raw mut (*pTab).pIndex;
            loop {
                pThis = *ppFrom;
                if pThis.is_null() {
                    break;
                }
                let mut pNext: *mut Index = ::core::ptr::null_mut::<Index>();
                if (*pThis).onError as ::core::ffi::c_int != OE_Replace {
                    ppFrom = &raw mut (*pThis).pNext;
                } else {
                    loop {
                        pNext = (*pThis).pNext;
                        if !(!pNext.is_null()
                            && (*pNext).onError as ::core::ffi::c_int != OE_Replace)
                        {
                            break;
                        }
                        *ppFrom = pNext;
                        (*pThis).pNext = (*pNext).pNext;
                        (*pNext).pNext = pThis;
                        ppFrom = &raw mut (*pNext).pNext;
                    }
                    break;
                }
            }
        }
        sqlite3ExprDelete(db, pPIWhere);
        sqlite3ExprListDelete(db, pList);
        sqlite3SrcListDelete(db, pTblName);
        sqlite3DbFree(db, zName as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DefaultRowEst(mut pIdx: *mut Index) {
    unsafe {
        static mut aVal: [LogEst; 5] = [
            33 as ::core::ffi::c_int as LogEst,
            32 as ::core::ffi::c_int as LogEst,
            30 as ::core::ffi::c_int as LogEst,
            28 as ::core::ffi::c_int as LogEst,
            26 as ::core::ffi::c_int as LogEst,
        ];
        let mut a: *mut LogEst = (*pIdx).aiRowLogEst;
        let mut x: LogEst = 0;
        let mut nCopy: ::core::ffi::c_int = if ((::core::mem::size_of::<[LogEst; 5]>()
            as usize)
            .wrapping_div(::core::mem::size_of::<LogEst>() as usize)
            as ::core::ffi::c_int) < (*pIdx).nKeyCol as ::core::ffi::c_int
        {
            (::core::mem::size_of::<[LogEst; 5]>() as usize)
                .wrapping_div(::core::mem::size_of::<LogEst>() as usize)
                as ::core::ffi::c_int
        } else {
            (*pIdx).nKeyCol as ::core::ffi::c_int
        };
        let mut i: ::core::ffi::c_int = 0;
        x = (*(*pIdx).pTable).nRowLogEst;
        if (x as ::core::ffi::c_int) < 99 as ::core::ffi::c_int {
            x = 99 as LogEst;
            (*(*pIdx).pTable).nRowLogEst = x;
        }
        if !(*pIdx).pPartIdxWhere.is_null() {
            x = (x as ::core::ffi::c_int - 10 as ::core::ffi::c_int) as LogEst;
        }
        *a.offset(0 as ::core::ffi::c_int as isize) = x;
        memcpy(
            a.offset(1 as ::core::ffi::c_int as isize) as *mut LogEst
                as *mut ::core::ffi::c_void,
            &raw const aVal as *const LogEst as *const ::core::ffi::c_void,
            (nCopy as size_t).wrapping_mul(::core::mem::size_of::<LogEst>() as size_t),
        );
        i = nCopy + 1 as ::core::ffi::c_int;
        while i <= (*pIdx).nKeyCol as ::core::ffi::c_int {
            *a.offset(i as isize) = 23 as LogEst;
            i += 1;
        }
        if (*pIdx).onError as ::core::ffi::c_int != OE_None {
            *a.offset((*pIdx).nKeyCol as isize) = 0 as LogEst;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DropIndex(
    mut pParse: *mut Parse,
    mut pName: *mut SrcList,
    mut ifExists: ::core::ffi::c_int,
) {
    unsafe {
        let mut pIndex: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut iDb: ::core::ffi::c_int = 0;
        if !((*db).mallocFailed != 0) {
            if !(SQLITE_OK != sqlite3ReadSchema(pParse)) {
                pIndex = sqlite3FindIndex(
                    db,
                    (*(&raw mut (*pName).a as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .zName,
                    (*(&raw mut (*pName).a as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .u4
                        .zDatabase,
                );
                if pIndex.is_null() {
                    if ifExists == 0 {
                        sqlite3ErrorMsg(
                            pParse,
                            b"no such index: %S\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*pName).a as *mut SrcItem,
                        );
                    } else {
                        sqlite3CodeVerifyNamedSchema(
                            pParse,
                            (*(&raw mut (*pName).a as *mut SrcItem)
                                .offset(0 as ::core::ffi::c_int as isize))
                                .u4
                                .zDatabase,
                        );
                        sqlite3ForceNotReadOnly(pParse);
                    }
                    (*pParse).set_checkSchema(1 as bft as bft);
                } else if (*pIndex).idxType() as ::core::ffi::c_int
                    != SQLITE_IDXTYPE_APPDEF
                {
                    sqlite3ErrorMsg(
                        pParse,
                        b"index associated with UNIQUE or PRIMARY KEY constraint cannot be dropped\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        0 as ::core::ffi::c_int,
                    );
                } else {
                    iDb = sqlite3SchemaToIndex(db, (*pIndex).pSchema);
                    let mut code: ::core::ffi::c_int = SQLITE_DROP_INDEX;
                    let mut pTab: *mut Table = (*pIndex).pTable;
                    let mut zDb: *const ::core::ffi::c_char = (*(*db)
                        .aDb
                        .offset(iDb as isize))
                        .zDbSName;
                    let mut zTab: *const ::core::ffi::c_char = if OMIT_TEMPDB == 0
                        && iDb == 1 as ::core::ffi::c_int
                    {
                        LEGACY_TEMP_SCHEMA_TABLE.as_ptr()
                    } else {
                        LEGACY_SCHEMA_TABLE.as_ptr()
                    };
                    if !(sqlite3AuthCheck(
                        pParse,
                        SQLITE_DELETE,
                        zTab,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        zDb,
                    ) != 0)
                    {
                        if OMIT_TEMPDB == 0 && iDb == 1 as ::core::ffi::c_int {
                            code = SQLITE_DROP_TEMP_INDEX;
                        }
                        if !(sqlite3AuthCheck(
                            pParse,
                            code,
                            (*pIndex).zName,
                            (*pTab).zName,
                            zDb,
                        ) != 0)
                        {
                            v = sqlite3GetVdbe(pParse);
                            if !v.is_null() {
                                sqlite3BeginWriteOperation(
                                    pParse,
                                    1 as ::core::ffi::c_int,
                                    iDb,
                                );
                                sqlite3NestedParse(
                                    pParse,
                                    b"DELETE FROM %Q.sqlite_master WHERE name=%Q AND type='index'\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*(*db).aDb.offset(iDb as isize)).zDbSName,
                                    (*pIndex).zName,
                                );
                                sqlite3ClearStatTables(
                                    pParse,
                                    iDb,
                                    b"idx\0".as_ptr() as *const ::core::ffi::c_char,
                                    (*pIndex).zName,
                                );
                                sqlite3ChangeCookie(pParse, iDb);
                                destroyRootPage(
                                    pParse,
                                    (*pIndex).tnum as ::core::ffi::c_int,
                                    iDb,
                                );
                                sqlite3VdbeAddOp4(
                                    v,
                                    OP_DropIndex,
                                    iDb,
                                    0 as ::core::ffi::c_int,
                                    0 as ::core::ffi::c_int,
                                    (*pIndex).zName,
                                    0 as ::core::ffi::c_int,
                                );
                            }
                        }
                    }
                }
            }
        }
        sqlite3SrcListDelete(db, pName);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ArrayAllocate(
    mut db: *mut sqlite3,
    mut pArray: *mut ::core::ffi::c_void,
    mut szEntry: ::core::ffi::c_int,
    mut pnEntry: *mut ::core::ffi::c_int,
    mut pIdx: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        *pIdx = *pnEntry;
        let mut n: sqlite3_int64 = *pIdx as sqlite3_int64;
        if n as ::core::ffi::c_longlong
            & n as ::core::ffi::c_longlong - 1 as ::core::ffi::c_longlong
            == 0 as ::core::ffi::c_longlong
        {
            let mut sz: sqlite3_int64 = if n == 0 as ::core::ffi::c_longlong {
                1 as sqlite3_int64
            } else {
                2 as sqlite3_int64 * n
            };
            let mut pNew: *mut ::core::ffi::c_void = sqlite3DbRealloc(
                db,
                pArray,
                (sz as ::core::ffi::c_longlong * szEntry as ::core::ffi::c_longlong)
                    as u64_0,
            );
            if pNew.is_null() {
                *pIdx = -1 as ::core::ffi::c_int;
                return pArray;
            }
            pArray = pNew;
        }
        z = pArray as *mut ::core::ffi::c_char;
        memset(
            z
                .offset(
                    (n as ::core::ffi::c_longlong * szEntry as ::core::ffi::c_longlong)
                        as isize,
                ) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            szEntry as size_t,
        );
        *pnEntry += 1;
        return pArray;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IdListAppend(
    mut pParse: *mut Parse,
    mut pList: *mut IdList,
    mut pToken: *mut Token,
) -> *mut IdList {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut i: ::core::ffi::c_int = 0;
        if pList.is_null() {
            pList = sqlite3DbMallocZero(
                db,
                (8 as usize)
                    .wrapping_add(
                        (1 as usize)
                            .wrapping_mul(::core::mem::size_of::<IdList_item>() as usize),
                    ) as u64_0,
            ) as *mut IdList;
            if pList.is_null() {
                return ::core::ptr::null_mut::<IdList>();
            }
        } else {
            let mut pNew: *mut IdList = ::core::ptr::null_mut::<IdList>();
            pNew = sqlite3DbRealloc(
                db,
                pList as *mut ::core::ffi::c_void,
                (8 as usize)
                    .wrapping_add(
                        (((*pList).nId + 1 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<IdList_item>() as usize),
                    ) as u64_0,
            ) as *mut IdList;
            if pNew.is_null() {
                sqlite3IdListDelete(db, pList);
                return ::core::ptr::null_mut::<IdList>();
            }
            pList = pNew;
        }
        let c2rust_fresh30 = (*pList).nId;
        (*pList).nId = (*pList).nId + 1;
        i = c2rust_fresh30;
        let ref mut c2rust_fresh31 = (*(&raw mut (*pList).a as *mut IdList_item)
            .offset(i as isize))
            .zName;
        *c2rust_fresh31 = sqlite3NameFromToken(db, pToken);
        if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME
            && !(*(&raw mut (*pList).a as *mut IdList_item).offset(i as isize))
                .zName
                .is_null()
        {
            sqlite3RenameTokenMap(
                pParse,
                (*(&raw mut (*pList).a as *mut IdList_item).offset(i as isize)).zName
                    as *mut ::core::ffi::c_void,
                pToken,
            );
        }
        return pList;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IdListDelete(
    mut db: *mut sqlite3,
    mut pList: *mut IdList,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if pList.is_null() {
            return;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pList).nId {
            sqlite3DbFree(
                db,
                (*(&raw mut (*pList).a as *mut IdList_item).offset(i as isize)).zName
                    as *mut ::core::ffi::c_void,
            );
            i += 1;
        }
        sqlite3DbNNFreeNN(db, pList as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3IdListIndex(
    mut pList: *mut IdList,
    mut zName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pList).nId {
            if sqlite3StrICmp(
                (*(&raw mut (*pList).a as *mut IdList_item).offset(i as isize)).zName,
                zName,
            ) == 0 as ::core::ffi::c_int
            {
                return i;
            }
            i += 1;
        }
        return -1 as ::core::ffi::c_int;
    }
}
pub const SQLITE_MAX_SRCLIST: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcListEnlarge(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut nExtra: ::core::ffi::c_int,
    mut iStart: ::core::ffi::c_int,
) -> *mut SrcList {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if ((*pSrc).nSrc as u32_0).wrapping_add(nExtra as u32_0) > (*pSrc).nAlloc {
            let mut pNew: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
            let mut nAlloc: sqlite3_int64 = 2 as sqlite3_int64
                * (*pSrc).nSrc as sqlite3_int64 + nExtra as sqlite3_int64;
            let mut db: *mut sqlite3 = (*pParse).db;
            if (*pSrc).nSrc + nExtra >= SQLITE_MAX_SRCLIST {
                sqlite3ErrorMsg(
                    pParse,
                    b"too many FROM clause terms, max: %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    SQLITE_MAX_SRCLIST,
                );
                return ::core::ptr::null_mut::<SrcList>();
            }
            if nAlloc > SQLITE_MAX_SRCLIST as ::core::ffi::c_longlong {
                nAlloc = SQLITE_MAX_SRCLIST as sqlite3_int64;
            }
            pNew = sqlite3DbRealloc(
                db,
                pSrc as *mut ::core::ffi::c_void,
                (8 as ::core::ffi::c_ulong as u64_0)
                    .wrapping_add(
                        (nAlloc as u64_0)
                            .wrapping_mul(::core::mem::size_of::<SrcItem>() as u64_0),
                    ),
            ) as *mut SrcList;
            if pNew.is_null() {
                return ::core::ptr::null_mut::<SrcList>();
            }
            pSrc = pNew;
            (*pSrc).nAlloc = nAlloc as u32_0;
        }
        i = (*pSrc).nSrc - 1 as ::core::ffi::c_int;
        while i >= iStart {
            *(&raw mut (*pSrc).a as *mut SrcItem).offset((i + nExtra) as isize) = *(&raw mut (*pSrc)
                .a as *mut SrcItem)
                .offset(i as isize);
            i -= 1;
        }
        (*pSrc).nSrc += nExtra;
        memset(
            (&raw mut (*pSrc).a as *mut SrcItem).offset(iStart as isize) as *mut SrcItem
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<SrcItem>() as size_t).wrapping_mul(nExtra as size_t),
        );
        i = iStart;
        while i < iStart + nExtra {
            (*(&raw mut (*pSrc).a as *mut SrcItem).offset(i as isize)).iCursor = -1
                as ::core::ffi::c_int;
            i += 1;
        }
        return pSrc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcListAppend(
    mut pParse: *mut Parse,
    mut pList: *mut SrcList,
    mut pTable: *mut Token,
    mut pDatabase: *mut Token,
) -> *mut SrcList {
    unsafe {
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        db = (*pParse).db;
        if pList.is_null() {
            pList = sqlite3DbMallocRawNN(
                (*pParse).db,
                (8 as usize)
                    .wrapping_add(
                        (1 as usize)
                            .wrapping_mul(::core::mem::size_of::<SrcItem>() as usize),
                    ) as u64_0,
            ) as *mut SrcList;
            if pList.is_null() {
                return ::core::ptr::null_mut::<SrcList>();
            }
            (*pList).nAlloc = 1 as u32_0;
            (*pList).nSrc = 1 as ::core::ffi::c_int;
            memset(
                (&raw mut (*pList).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut SrcItem
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<SrcItem>() as size_t,
            );
            (*(&raw mut (*pList).a as *mut SrcItem)
                .offset(0 as ::core::ffi::c_int as isize))
                .iCursor = -1 as ::core::ffi::c_int;
        } else {
            let mut pNew: *mut SrcList = sqlite3SrcListEnlarge(
                pParse,
                pList,
                1 as ::core::ffi::c_int,
                (*pList).nSrc,
            );
            if pNew.is_null() {
                sqlite3SrcListDelete(db, pList);
                return ::core::ptr::null_mut::<SrcList>();
            } else {
                pList = pNew;
            }
        }
        pItem = (&raw mut (*pList).a as *mut SrcItem)
            .offset(((*pList).nSrc - 1 as ::core::ffi::c_int) as isize) as *mut SrcItem;
        if !pDatabase.is_null() && (*pDatabase).z.is_null() {
            pDatabase = ::core::ptr::null_mut::<Token>();
        }
        if !pDatabase.is_null() {
            (*pItem).zName = sqlite3NameFromToken(db, pDatabase);
            (*pItem).u4.zDatabase = sqlite3NameFromToken(db, pTable);
        } else {
            (*pItem).zName = sqlite3NameFromToken(db, pTable);
            (*pItem).u4.zDatabase = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return pList;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcListAssignCursors(
    mut pParse: *mut Parse,
    mut pList: *mut SrcList,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        if !pList.is_null() {
            i = 0 as ::core::ffi::c_int;
            pItem = &raw mut (*pList).a as *mut SrcItem;
            while i < (*pList).nSrc {
                if !((*pItem).iCursor >= 0 as ::core::ffi::c_int) {
                    let c2rust_fresh28 = (*pParse).nTab;
                    (*pParse).nTab = (*pParse).nTab + 1;
                    (*pItem).iCursor = c2rust_fresh28;
                    if (*pItem).fg.isSubquery() != 0 {
                        sqlite3SrcListAssignCursors(
                            pParse,
                            (*(*(*pItem).u4.pSubq).pSelect).pSrc,
                        );
                    }
                }
                i += 1;
                pItem = pItem.offset(1);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SubqueryDelete(
    mut db: *mut sqlite3,
    mut pSubq: *mut Subquery,
) {
    unsafe {
        sqlite3SelectDelete(db, (*pSubq).pSelect);
        sqlite3DbFree(db, pSubq as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SubqueryDetach(
    mut db: *mut sqlite3,
    mut pItem: *mut SrcItem,
) -> *mut Select {
    unsafe {
        let mut pSel: *mut Select = ::core::ptr::null_mut::<Select>();
        pSel = (*(*pItem).u4.pSubq).pSelect;
        sqlite3DbFree(db, (*pItem).u4.pSubq as *mut ::core::ffi::c_void);
        (*pItem).u4.pSubq = ::core::ptr::null_mut::<Subquery>();
        (*pItem).fg.set_isSubquery(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        return pSel;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcListDelete(
    mut db: *mut sqlite3,
    mut pList: *mut SrcList,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        if pList.is_null() {
            return;
        }
        pItem = &raw mut (*pList).a as *mut SrcItem;
        i = 0 as ::core::ffi::c_int;
        while i < (*pList).nSrc {
            if !(*pItem).zName.is_null() {
                sqlite3DbNNFreeNN(db, (*pItem).zName as *mut ::core::ffi::c_void);
            }
            if !(*pItem).zAlias.is_null() {
                sqlite3DbNNFreeNN(db, (*pItem).zAlias as *mut ::core::ffi::c_void);
            }
            if (*pItem).fg.isSubquery() != 0 {
                sqlite3SubqueryDelete(db, (*pItem).u4.pSubq);
            } else if (*pItem).fg.fixedSchema() as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int && !(*pItem).u4.zDatabase.is_null()
            {
                sqlite3DbNNFreeNN(db, (*pItem).u4.zDatabase as *mut ::core::ffi::c_void);
            }
            if (*pItem).fg.isIndexedBy() != 0 {
                sqlite3DbFree(db, (*pItem).u1.zIndexedBy as *mut ::core::ffi::c_void);
            }
            if (*pItem).fg.isTabFunc() != 0 {
                sqlite3ExprListDelete(db, (*pItem).u1.pFuncArg);
            }
            sqlite3DeleteTable(db, (*pItem).pSTab);
            if (*pItem).fg.isUsing() != 0 {
                sqlite3IdListDelete(db, (*pItem).u3.pUsing);
            } else if !(*pItem).u3.pOn.is_null() {
                sqlite3ExprDelete(db, (*pItem).u3.pOn);
            }
            i += 1;
            pItem = pItem.offset(1);
        }
        sqlite3DbNNFreeNN(db, pList as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcItemAttachSubquery(
    mut pParse: *mut Parse,
    mut pItem: *mut SrcItem,
    mut pSelect: *mut Select,
    mut dupSelect: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Subquery = ::core::ptr::null_mut::<Subquery>();
        if (*pItem).fg.fixedSchema() != 0 {
            (*pItem).u4.pSchema = ::core::ptr::null_mut::<Schema>();
            (*pItem).fg.set_fixedSchema(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else if !(*pItem).u4.zDatabase.is_null() {
            sqlite3DbFree(
                (*pParse).db,
                (*pItem).u4.zDatabase as *mut ::core::ffi::c_void,
            );
            (*pItem).u4.zDatabase = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if dupSelect != 0 {
            pSelect = sqlite3SelectDup((*pParse).db, pSelect, 0 as ::core::ffi::c_int);
            if pSelect.is_null() {
                return 0 as ::core::ffi::c_int;
            }
        }
        (*pItem).u4.pSubq = sqlite3DbMallocRawNN(
            (*pParse).db,
            ::core::mem::size_of::<Subquery>() as u64_0,
        ) as *mut Subquery;
        p = (*pItem).u4.pSubq;
        if p.is_null() {
            sqlite3SelectDelete((*pParse).db, pSelect);
            return 0 as ::core::ffi::c_int;
        }
        (*pItem).fg.set_isSubquery(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).pSelect = pSelect;
        memset(
            (p as *mut ::core::ffi::c_char)
                .offset(::core::mem::size_of::<*mut Select>() as usize as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<Subquery>() as size_t)
                .wrapping_sub(::core::mem::size_of::<*mut Select>() as size_t),
        );
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcListAppendFromTerm(
    mut pParse: *mut Parse,
    mut p: *mut SrcList,
    mut pTable: *mut Token,
    mut pDatabase: *mut Token,
    mut pAlias: *mut Token,
    mut pSubquery: *mut Select,
    mut pOnUsing: *mut OnOrUsing,
) -> *mut SrcList {
    unsafe {
        let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
        let mut db: *mut sqlite3 = (*pParse).db;
        if p.is_null() && !pOnUsing.is_null()
            && (!(*pOnUsing).pOn.is_null() || !(*pOnUsing).pUsing.is_null())
        {
            sqlite3ErrorMsg(
                pParse,
                b"a JOIN clause is required before %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if !(*pOnUsing).pOn.is_null() {
                    b"ON\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"USING\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        } else {
            p = sqlite3SrcListAppend(pParse, p, pTable, pDatabase);
            if !p.is_null() {
                pItem = (&raw mut (*p).a as *mut SrcItem)
                    .offset(((*p).nSrc - 1 as ::core::ffi::c_int) as isize)
                    as *mut SrcItem;
                if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME
                    && !(*pItem).zName.is_null()
                {
                    let mut pToken: *mut Token = if !pDatabase.is_null()
                        && !(*pDatabase).z.is_null()
                    {
                        pDatabase
                    } else {
                        pTable
                    };
                    sqlite3RenameTokenMap(
                        pParse,
                        (*pItem).zName as *const ::core::ffi::c_void,
                        pToken,
                    );
                }
                if (*pAlias).n != 0 {
                    (*pItem).zAlias = sqlite3NameFromToken(db, pAlias);
                }
                if !pSubquery.is_null() {
                    if sqlite3SrcItemAttachSubquery(
                        pParse,
                        pItem,
                        pSubquery,
                        0 as ::core::ffi::c_int,
                    ) != 0
                    {
                        if (*pSubquery).selFlags & SF_NestedFrom as u32_0 != 0 {
                            (*pItem)
                                .fg
                                .set_isNestedFrom(
                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                        }
                    }
                }
                if pOnUsing.is_null() {
                    (*pItem).u3.pOn = ::core::ptr::null_mut::<Expr>();
                } else if !(*pOnUsing).pUsing.is_null() {
                    (*pItem)
                        .fg
                        .set_isUsing(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*pItem).u3.pUsing = (*pOnUsing).pUsing;
                } else {
                    (*pItem).u3.pOn = (*pOnUsing).pOn;
                }
                return p;
            }
        }
        sqlite3ClearOnOrUsing(db, pOnUsing);
        sqlite3SelectDelete(db, pSubquery);
        return ::core::ptr::null_mut::<SrcList>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcListIndexedBy(
    mut pParse: *mut Parse,
    mut p: *mut SrcList,
    mut pIndexedBy: *mut Token,
) {
    unsafe {
        if !p.is_null() && (*pIndexedBy).n > 0 as ::core::ffi::c_uint {
            let mut pItem: *mut SrcItem = ::core::ptr::null_mut::<SrcItem>();
            pItem = (&raw mut (*p).a as *mut SrcItem)
                .offset(((*p).nSrc - 1 as ::core::ffi::c_int) as isize) as *mut SrcItem;
            if (*pIndexedBy).n == 1 as ::core::ffi::c_uint && (*pIndexedBy).z.is_null() {
                (*pItem)
                    .fg
                    .set_notIndexed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else {
                (*pItem).u1.zIndexedBy = sqlite3NameFromToken((*pParse).db, pIndexedBy);
                (*pItem)
                    .fg
                    .set_isIndexedBy(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcListAppendList(
    mut pParse: *mut Parse,
    mut p1: *mut SrcList,
    mut p2: *mut SrcList,
) -> *mut SrcList {
    unsafe {
        if !p2.is_null() {
            let mut nOld: ::core::ffi::c_int = (*p1).nSrc;
            let mut pNew: *mut SrcList = sqlite3SrcListEnlarge(
                pParse,
                p1,
                (*p2).nSrc,
                nOld,
            );
            if pNew.is_null() {
                sqlite3SrcListDelete((*pParse).db, p2);
            } else {
                p1 = pNew;
                memcpy(
                    (&raw mut (*p1).a as *mut SrcItem).offset(nOld as isize)
                        as *mut SrcItem as *mut ::core::ffi::c_void,
                    &raw mut (*p2).a as *mut SrcItem as *const ::core::ffi::c_void,
                    ((*p2).nSrc as size_t)
                        .wrapping_mul(::core::mem::size_of::<SrcItem>() as size_t),
                );
                let ref mut c2rust_fresh32 = (*(&raw mut (*p1).a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .fg
                    .jointype;
                *c2rust_fresh32 = (*c2rust_fresh32 as ::core::ffi::c_int
                    | JT_LTORJ
                        & (*(&raw mut (*p2).a as *mut SrcItem)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .fg
                            .jointype as ::core::ffi::c_int) as u8_0;
                sqlite3DbFree((*pParse).db, p2 as *mut ::core::ffi::c_void);
            }
        }
        return p1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcListFuncArgs(
    mut pParse: *mut Parse,
    mut p: *mut SrcList,
    mut pList: *mut ExprList,
) {
    unsafe {
        if !p.is_null() {
            let mut pItem: *mut SrcItem = (&raw mut (*p).a as *mut SrcItem)
                .offset(((*p).nSrc - 1 as ::core::ffi::c_int) as isize) as *mut SrcItem;
            (*pItem).u1.pFuncArg = pList;
            (*pItem).fg.set_isTabFunc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else {
            sqlite3ExprListDelete((*pParse).db, pList);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3SrcListShiftJoinType(
    mut pParse: *mut Parse,
    mut p: *mut SrcList,
) {
    unsafe {
        if !p.is_null() && (*p).nSrc > 1 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = (*p).nSrc - 1 as ::core::ffi::c_int;
            let mut allFlags: u8_0 = 0 as u8_0;
            loop {
                let ref mut c2rust_fresh33 = (*(&raw mut (*p).a as *mut SrcItem)
                    .offset(i as isize))
                    .fg
                    .jointype;
                *c2rust_fresh33 = (*(&raw mut (*p).a as *mut SrcItem)
                    .offset((i - 1 as ::core::ffi::c_int) as isize))
                    .fg
                    .jointype;
                allFlags = (allFlags as ::core::ffi::c_int
                    | *c2rust_fresh33 as ::core::ffi::c_int) as u8_0;
                i -= 1;
                if !(i > 0 as ::core::ffi::c_int) {
                    break;
                }
            }
            (*(&raw mut (*p).a as *mut SrcItem).offset(0 as ::core::ffi::c_int as isize))
                .fg
                .jointype = 0 as u8_0;
            if allFlags as ::core::ffi::c_int & JT_RIGHT != 0 {
                i = (*p).nSrc - 1 as ::core::ffi::c_int;
                while i > 0 as ::core::ffi::c_int
                    && (*(&raw mut (*p).a as *mut SrcItem).offset(i as isize))
                        .fg
                        .jointype as ::core::ffi::c_int & JT_RIGHT
                        == 0 as ::core::ffi::c_int
                {
                    i -= 1;
                }
                i -= 1;
                loop {
                    let ref mut c2rust_fresh34 = (*(&raw mut (*p).a as *mut SrcItem)
                        .offset(i as isize))
                        .fg
                        .jointype;
                    *c2rust_fresh34 = (*c2rust_fresh34 as ::core::ffi::c_int | JT_LTORJ)
                        as u8_0;
                    i -= 1;
                    if !(i >= 0 as ::core::ffi::c_int) {
                        break;
                    }
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BeginTransaction(
    mut pParse: *mut Parse,
    mut type_0: ::core::ffi::c_int,
) {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut i: ::core::ffi::c_int = 0;
        db = (*pParse).db;
        if sqlite3AuthCheck(
            pParse,
            SQLITE_TRANSACTION,
            b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) != 0
        {
            return;
        }
        v = sqlite3GetVdbe(pParse);
        if v.is_null() {
            return;
        }
        if type_0 != TK_DEFERRED {
            i = 0 as ::core::ffi::c_int;
            while i < (*db).nDb {
                let mut eTxnType: ::core::ffi::c_int = 0;
                let mut pBt: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
                if !pBt.is_null() && sqlite3BtreeIsReadonly(pBt) != 0 {
                    eTxnType = 0 as ::core::ffi::c_int;
                } else if type_0 == TK_EXCLUSIVE {
                    eTxnType = 2 as ::core::ffi::c_int;
                } else {
                    eTxnType = 1 as ::core::ffi::c_int;
                }
                sqlite3VdbeAddOp2(v, OP_Transaction, i, eTxnType);
                sqlite3VdbeUsesBtree(v, i);
                i += 1;
            }
        }
        sqlite3VdbeAddOp0(v, OP_AutoCommit);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3EndTransaction(
    mut pParse: *mut Parse,
    mut eType: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        let mut isRollback: ::core::ffi::c_int = 0;
        isRollback = (eType == TK_ROLLBACK) as ::core::ffi::c_int;
        if sqlite3AuthCheck(
            pParse,
            SQLITE_TRANSACTION,
            if isRollback != 0 {
                b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) != 0
        {
            return;
        }
        v = sqlite3GetVdbe(pParse);
        if !v.is_null() {
            sqlite3VdbeAddOp2(v, OP_AutoCommit, 1 as ::core::ffi::c_int, isRollback);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Savepoint(
    mut pParse: *mut Parse,
    mut op: ::core::ffi::c_int,
    mut pName: *mut Token,
) {
    unsafe {
        let mut zName: *mut ::core::ffi::c_char = sqlite3NameFromToken(
            (*pParse).db,
            pName,
        );
        if !zName.is_null() {
            let mut v: *mut Vdbe = sqlite3GetVdbe(pParse);
            static mut az: [*const ::core::ffi::c_char; 3] = [
                b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char,
                b"RELEASE\0".as_ptr() as *const ::core::ffi::c_char,
                b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char,
            ];
            if v.is_null()
                || sqlite3AuthCheck(
                    pParse,
                    SQLITE_SAVEPOINT,
                    az[op as usize],
                    zName,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                ) != 0
            {
                sqlite3DbFree((*pParse).db, zName as *mut ::core::ffi::c_void);
                return;
            }
            sqlite3VdbeAddOp4(
                v,
                OP_Savepoint,
                op,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                zName,
                P4_DYNAMIC,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3OpenTempDatabase(
    mut pParse: *mut Parse,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        if (*(*db).aDb.offset(1 as ::core::ffi::c_int as isize)).pBt.is_null()
            && (*pParse).explain == 0
        {
            let mut rc: ::core::ffi::c_int = 0;
            let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
            static mut flags: ::core::ffi::c_int = SQLITE_OPEN_READWRITE
                | SQLITE_OPEN_CREATE | SQLITE_OPEN_EXCLUSIVE | SQLITE_OPEN_DELETEONCLOSE
                | SQLITE_OPEN_TEMP_DB;
            rc = sqlite3BtreeOpen(
                (*db).pVfs,
                ::core::ptr::null::<::core::ffi::c_char>(),
                db,
                &raw mut pBt,
                0 as ::core::ffi::c_int,
                flags,
            );
            if rc != SQLITE_OK {
                sqlite3ErrorMsg(
                    pParse,
                    b"unable to open a temporary database file for storing temporary tables\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                (*pParse).rc = rc;
                return 1 as ::core::ffi::c_int;
            }
            let ref mut c2rust_fresh5 = (*(*db)
                .aDb
                .offset(1 as ::core::ffi::c_int as isize))
                .pBt;
            *c2rust_fresh5 = pBt;
            if SQLITE_NOMEM
                == sqlite3BtreeSetPageSize(
                    pBt,
                    (*db).nextPagesize,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                )
            {
                sqlite3OomFault(db);
                return 1 as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn sqlite3CodeVerifySchemaAtToplevel(
    mut pToplevel: *mut Parse,
    mut iDb: ::core::ffi::c_int,
) {
    unsafe {
        if ((*pToplevel).cookieMask & (1 as ::core::ffi::c_int as yDbMask) << iDb
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*pToplevel).cookieMask
                |= ((1 as ::core::ffi::c_int as yDbMask) << iDb) as ::core::ffi::c_uint;
            if OMIT_TEMPDB == 0 && iDb == 1 as ::core::ffi::c_int {
                sqlite3OpenTempDatabase(pToplevel);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CodeVerifySchema(
    mut pParse: *mut Parse,
    mut iDb: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3CodeVerifySchemaAtToplevel(
            if !(*pParse).pToplevel.is_null() { (*pParse).pToplevel } else { pParse },
            iDb,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CodeVerifyNamedSchema(
    mut pParse: *mut Parse,
    mut zDb: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*db).nDb {
            let mut pDb: *mut Db = (*db).aDb.offset(i as isize) as *mut Db;
            if !(*pDb).pBt.is_null()
                && (zDb.is_null()
                    || 0 as ::core::ffi::c_int == sqlite3StrICmp(zDb, (*pDb).zDbSName))
            {
                sqlite3CodeVerifySchema(pParse, i);
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3BeginWriteOperation(
    mut pParse: *mut Parse,
    mut setStatement: ::core::ffi::c_int,
    mut iDb: ::core::ffi::c_int,
) {
    unsafe {
        let mut pToplevel: *mut Parse = if !(*pParse).pToplevel.is_null() {
            (*pParse).pToplevel
        } else {
            pParse
        };
        sqlite3CodeVerifySchemaAtToplevel(pToplevel, iDb);
        (*pToplevel).writeMask
            |= ((1 as ::core::ffi::c_int as yDbMask) << iDb) as ::core::ffi::c_uint;
        (*pToplevel).isMultiWrite = ((*pToplevel).isMultiWrite as ::core::ffi::c_int
            | setStatement) as u8_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MultiWrite(mut pParse: *mut Parse) {
    unsafe {
        let mut pToplevel: *mut Parse = if !(*pParse).pToplevel.is_null() {
            (*pParse).pToplevel
        } else {
            pParse
        };
        (*pToplevel).isMultiWrite = 1 as u8_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3MayAbort(mut pParse: *mut Parse) {
    unsafe {
        let mut pToplevel: *mut Parse = if !(*pParse).pToplevel.is_null() {
            (*pParse).pToplevel
        } else {
            pParse
        };
        (*pToplevel).mayAbort = 1 as u8_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3HaltConstraint(
    mut pParse: *mut Parse,
    mut errCode: ::core::ffi::c_int,
    mut onError: ::core::ffi::c_int,
    mut p4: *mut ::core::ffi::c_char,
    mut p4type: i8_0,
    mut p5Errmsg: u8_0,
) {
    unsafe {
        let mut v: *mut Vdbe = ::core::ptr::null_mut::<Vdbe>();
        v = sqlite3GetVdbe(pParse);
        if onError == OE_Abort {
            sqlite3MayAbort(pParse);
        }
        sqlite3VdbeAddOp4(
            v,
            OP_Halt,
            errCode,
            onError,
            0 as ::core::ffi::c_int,
            p4,
            p4type as ::core::ffi::c_int,
        );
        sqlite3VdbeChangeP5(v, p5Errmsg as u16_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3UniqueConstraint(
    mut pParse: *mut Parse,
    mut onError: ::core::ffi::c_int,
    mut pIdx: *mut Index,
) {
    unsafe {
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut j: ::core::ffi::c_int = 0;
        let mut errMsg: StrAccum = StrAccum {
            db: ::core::ptr::null_mut::<sqlite3>(),
            zText: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nAlloc: 0,
            mxAlloc: 0,
            nChar: 0,
            accError: 0,
            printfFlags: 0

        };
        let mut pTab: *mut Table = (*pIdx).pTable;
        sqlite3StrAccumInit(
            &raw mut errMsg,
            (*pParse).db,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            (*(*pParse).db).aLimit[SQLITE_LIMIT_LENGTH as usize],
        );
        if !(*pIdx).aColExpr.is_null() {
            sqlite3_str_appendf(
                &raw mut errMsg,
                b"index '%q'\0".as_ptr() as *const ::core::ffi::c_char,
                (*pIdx).zName,
            );
        } else {
            j = 0 as ::core::ffi::c_int;
            while j < (*pIdx).nKeyCol as ::core::ffi::c_int {
                let mut zCol: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                zCol = (*(*pTab)
                    .aCol
                    .offset(*(*pIdx).aiColumn.offset(j as isize) as isize))
                    .zCnName;
                if j != 0 {
                    sqlite3_str_append(
                        &raw mut errMsg,
                        b", \0".as_ptr() as *const ::core::ffi::c_char,
                        2 as ::core::ffi::c_int,
                    );
                }
                sqlite3_str_appendall(&raw mut errMsg, (*pTab).zName);
                sqlite3_str_append(
                    &raw mut errMsg,
                    b".\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                );
                sqlite3_str_appendall(&raw mut errMsg, zCol);
                j += 1;
            }
        }
        zErr = sqlite3StrAccumFinish(&raw mut errMsg);
        sqlite3HaltConstraint(
            pParse,
            if (*pIdx).idxType() as ::core::ffi::c_int == SQLITE_IDXTYPE_PRIMARYKEY {
                SQLITE_CONSTRAINT_PRIMARYKEY
            } else {
                SQLITE_CONSTRAINT_UNIQUE
            },
            onError,
            zErr,
            P4_DYNAMIC as i8_0,
            P5_ConstraintUnique as u8_0,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3RowidConstraint(
    mut pParse: *mut Parse,
    mut onError: ::core::ffi::c_int,
    mut pTab: *mut Table,
) {
    unsafe {
        let mut zMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        if (*pTab).iPKey as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            zMsg = sqlite3MPrintf(
                (*pParse).db,
                b"%s.%s\0".as_ptr() as *const ::core::ffi::c_char,
                (*pTab).zName,
                (*(*pTab).aCol.offset((*pTab).iPKey as isize)).zCnName,
            );
            rc = SQLITE_CONSTRAINT_PRIMARYKEY;
        } else {
            zMsg = sqlite3MPrintf(
                (*pParse).db,
                b"%s.rowid\0".as_ptr() as *const ::core::ffi::c_char,
                (*pTab).zName,
            );
            rc = SQLITE_CONSTRAINT_ROWID;
        }
        sqlite3HaltConstraint(
            pParse,
            rc,
            onError,
            zMsg,
            P4_DYNAMIC as i8_0,
            P5_ConstraintUnique as u8_0,
        );
    }
}
unsafe extern "C" fn collationMatch(
    mut zColl: *const ::core::ffi::c_char,
    mut pIndex: *mut Index,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIndex).nColumn as ::core::ffi::c_int {
            let mut z: *const ::core::ffi::c_char = *(*pIndex).azColl.offset(i as isize);
            if *(*pIndex).aiColumn.offset(i as isize) as ::core::ffi::c_int
                >= 0 as ::core::ffi::c_int
                && 0 as ::core::ffi::c_int == sqlite3StrICmp(z, zColl)
            {
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn reindexTable(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut zColl: *const ::core::ffi::c_char,
) {
    unsafe {
        if !((*pTab).eTabType as ::core::ffi::c_int == TABTYP_VTAB) {
            let mut pIndex: *mut Index = ::core::ptr::null_mut::<Index>();
            pIndex = (*pTab).pIndex;
            while !pIndex.is_null() {
                if zColl.is_null() || collationMatch(zColl, pIndex) != 0 {
                    let mut iDb: ::core::ffi::c_int = sqlite3SchemaToIndex(
                        (*pParse).db,
                        (*pTab).pSchema,
                    );
                    sqlite3BeginWriteOperation(pParse, 0 as ::core::ffi::c_int, iDb);
                    sqlite3RefillIndex(pParse, pIndex, -1 as ::core::ffi::c_int);
                }
                pIndex = (*pIndex).pNext;
            }
        }
    }
}
unsafe extern "C" fn reindexDatabases(
    mut pParse: *mut Parse,
    mut zColl: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut pDb: *mut Db = ::core::ptr::null_mut::<Db>();
        let mut iDb: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut k: *mut HashElem = ::core::ptr::null_mut::<HashElem>();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        iDb = 0 as ::core::ffi::c_int;
        pDb = (*db).aDb;
        while iDb < (*db).nDb {
            k = (*(*pDb).pSchema).tblHash.first;
            while !k.is_null() {
                pTab = (*k).data as *mut Table;
                reindexTable(pParse, pTab, zColl);
                k = (*k).next;
            }
            iDb += 1;
            pDb = pDb.offset(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Reindex(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
) {
    unsafe {
        let mut pColl: *mut CollSeq = ::core::ptr::null_mut::<CollSeq>();
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pTab: *mut Table = ::core::ptr::null_mut::<Table>();
        let mut pIndex: *mut Index = ::core::ptr::null_mut::<Index>();
        let mut iDb: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pObjName: *mut Token = ::core::ptr::null_mut::<Token>();
        if SQLITE_OK != sqlite3ReadSchema(pParse) {
            return;
        }
        if pName1.is_null() {
            reindexDatabases(pParse, ::core::ptr::null::<::core::ffi::c_char>());
            return;
        } else if pName2.is_null() || (*pName2).z.is_null() {
            let mut zColl: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            zColl = sqlite3NameFromToken((*pParse).db, pName1);
            if zColl.is_null() {
                return;
            }
            pColl = sqlite3FindCollSeq(db, (*db).enc, zColl, 0 as ::core::ffi::c_int);
            if !pColl.is_null() {
                reindexDatabases(pParse, zColl);
                sqlite3DbFree(db, zColl as *mut ::core::ffi::c_void);
                return;
            }
            sqlite3DbFree(db, zColl as *mut ::core::ffi::c_void);
        }
        iDb = sqlite3TwoPartName(pParse, pName1, pName2, &raw mut pObjName);
        if iDb < 0 as ::core::ffi::c_int {
            return;
        }
        z = sqlite3NameFromToken(db, pObjName);
        if z.is_null() {
            return;
        }
        zDb = if (*pName2).n != 0 {
            (*(*db).aDb.offset(iDb as isize)).zDbSName
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        pTab = sqlite3FindTable(db, z, zDb);
        if !pTab.is_null() {
            reindexTable(pParse, pTab, ::core::ptr::null::<::core::ffi::c_char>());
            sqlite3DbFree(db, z as *mut ::core::ffi::c_void);
            return;
        }
        pIndex = sqlite3FindIndex(db, z, zDb);
        sqlite3DbFree(db, z as *mut ::core::ffi::c_void);
        if !pIndex.is_null() {
            iDb = sqlite3SchemaToIndex(db, (*(*pIndex).pTable).pSchema);
            sqlite3BeginWriteOperation(pParse, 0 as ::core::ffi::c_int, iDb);
            sqlite3RefillIndex(pParse, pIndex, -1 as ::core::ffi::c_int);
            return;
        }
        sqlite3ErrorMsg(
            pParse,
            b"unable to identify the object to be reindexed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3KeyInfoOfIndex(
    mut pParse: *mut Parse,
    mut pIdx: *mut Index,
) -> *mut KeyInfo {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut nCol: ::core::ffi::c_int = (*pIdx).nColumn as ::core::ffi::c_int;
        let mut nKey: ::core::ffi::c_int = (*pIdx).nKeyCol as ::core::ffi::c_int;
        let mut pKey: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        if (*pParse).nErr != 0 {
            return ::core::ptr::null_mut::<KeyInfo>();
        }
        if (*pIdx).uniqNotNull() != 0 {
            pKey = sqlite3KeyInfoAlloc((*pParse).db, nKey, nCol - nKey);
        } else {
            pKey = sqlite3KeyInfoAlloc((*pParse).db, nCol, 0 as ::core::ffi::c_int);
        }
        if !pKey.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < nCol {
                let mut zColl: *const ::core::ffi::c_char = *(*pIdx)
                    .azColl
                    .offset(i as isize);
                let ref mut c2rust_fresh13 = *(&raw mut (*pKey).aColl
                    as *mut *mut CollSeq)
                    .offset(i as isize);
                *c2rust_fresh13 = if zColl
                    == &raw const sqlite3StrBINARY as *const ::core::ffi::c_char
                {
                    ::core::ptr::null_mut::<CollSeq>()
                } else {
                    sqlite3LocateCollSeq(pParse, zColl)
                };
                *(*pKey).aSortFlags.offset(i as isize) = *(*pIdx)
                    .aSortOrder
                    .offset(i as isize);
                i += 1;
            }
            if (*pParse).nErr != 0 {
                if (*pIdx).bNoQuery() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && !sqlite3HashFind(
                            &raw mut (*(*pIdx).pSchema).idxHash,
                            (*pIdx).zName,
                        )
                        .is_null()
                {
                    (*pIdx)
                        .set_bNoQuery(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*pParse).rc = SQLITE_ERROR_RETRY;
                }
                sqlite3KeyInfoUnref(pKey);
                pKey = ::core::ptr::null_mut::<KeyInfo>();
            }
        }
        return pKey;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CteNew(
    mut pParse: *mut Parse,
    mut pName: *mut Token,
    mut pArglist: *mut ExprList,
    mut pQuery: *mut Select,
    mut eM10d: u8_0,
) -> *mut Cte {
    unsafe {
        let mut pNew: *mut Cte = ::core::ptr::null_mut::<Cte>();
        let mut db: *mut sqlite3 = (*pParse).db;
        pNew = sqlite3DbMallocZero(db, ::core::mem::size_of::<Cte>() as u64_0)
            as *mut Cte;
        if (*db).mallocFailed != 0 {
            sqlite3ExprListDelete(db, pArglist);
            sqlite3SelectDelete(db, pQuery);
        } else {
            (*pNew).pSelect = pQuery;
            (*pNew).pCols = pArglist;
            (*pNew).zName = sqlite3NameFromToken((*pParse).db, pName);
            (*pNew).eM10d = eM10d;
        }
        return pNew;
    }
}
unsafe extern "C" fn cteClear(mut db: *mut sqlite3, mut pCte: *mut Cte) {
    unsafe {
        sqlite3ExprListDelete(db, (*pCte).pCols);
        sqlite3SelectDelete(db, (*pCte).pSelect);
        sqlite3DbFree(db, (*pCte).zName as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3CteDelete(mut db: *mut sqlite3, mut pCte: *mut Cte) {
    unsafe {
        cteClear(db, pCte);
        sqlite3DbFree(db, pCte as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WithAdd(
    mut pParse: *mut Parse,
    mut pWith: *mut With,
    mut pCte: *mut Cte,
) -> *mut With {
    unsafe {
        let mut db: *mut sqlite3 = (*pParse).db;
        let mut pNew: *mut With = ::core::ptr::null_mut::<With>();
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if pCte.is_null() {
            return pWith;
        }
        zName = (*pCte).zName;
        if !zName.is_null() && !pWith.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pWith).nCte {
                if sqlite3StrICmp(
                    zName,
                    (*(&raw mut (*pWith).a as *mut Cte).offset(i as isize)).zName,
                ) == 0 as ::core::ffi::c_int
                {
                    sqlite3ErrorMsg(
                        pParse,
                        b"duplicate WITH table name: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        zName,
                    );
                }
                i += 1;
            }
        }
        if !pWith.is_null() {
            pNew = sqlite3DbRealloc(
                db,
                pWith as *mut ::core::ffi::c_void,
                (16 as usize)
                    .wrapping_add(
                        (((*pWith).nCte + 1 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<Cte>() as usize),
                    ) as u64_0,
            ) as *mut With;
        } else {
            pNew = sqlite3DbMallocZero(
                db,
                (16 as usize)
                    .wrapping_add(
                        (1 as usize).wrapping_mul(::core::mem::size_of::<Cte>() as usize),
                    ) as u64_0,
            ) as *mut With;
        }
        if (*db).mallocFailed != 0 {
            sqlite3CteDelete(db, pCte);
            pNew = pWith;
        } else {
            let c2rust_fresh36 = (*pNew).nCte;
            (*pNew).nCte = (*pNew).nCte + 1;
            *(&raw mut (*pNew).a as *mut Cte).offset(c2rust_fresh36 as isize) = *pCte;
            sqlite3DbFree(db, pCte as *mut ::core::ffi::c_void);
        }
        return pNew;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WithDelete(mut db: *mut sqlite3, mut pWith: *mut With) {
    unsafe {
        if !pWith.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pWith).nCte {
                cteClear(
                    db,
                    (&raw mut (*pWith).a as *mut Cte).offset(i as isize) as *mut Cte,
                );
                i += 1;
            }
            sqlite3DbFree(db, pWith as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3WithDeleteGeneric(
    mut db: *mut sqlite3,
    mut pWith: *mut ::core::ffi::c_void,
) {
    unsafe {
        sqlite3WithDelete(db, pWith as *mut With);
    }
}
